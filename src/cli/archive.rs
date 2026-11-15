use crate::config::overlay::load_effective;
use crate::gherkin::parse::{collect_feature_files, has_e2e_scenarios};
use crate::proof::{ArchiveProof, SCOPE_E2E, SCOPE_REGULAR};
#[cfg(test)]
use crate::spec::merge_plan::resolve_permanent_dir;
use crate::spec::merge_plan::{
    MergeEntry, build_merge_plan, collect_narrative_documents, delta_capability, narrative_target,
    write_feature, write_feature_retry,
};
use crate::spec::projection::{
    MergeProgress, interaction_findings_for_plan, permanent_interaction_findings,
};
use crate::state::{ArtifactStatus, ChangeState};
use anyhow::{Context, Result, bail};
use gherkin::GherkinEnv;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Which dirty-path message the witness uses.
#[derive(Clone, Copy)]
enum Witness {
    /// Before publication: the project must be committed.
    Project,
    /// Before finalization: only evidence may be dirty.
    Tested,
}

/// Archive a completed change (ADR-070, arc42 ch.6.7).
///
/// Forward-only publication:
/// 1. Selected-backend prerequisite and clean-state witness (evidence paths
///    are exempt so a failed attempt stays resumable).
/// 2. Fresh or interrupted move: structural gates, write the pending receipt,
///    and move the change into the archive location with one rename.
/// 3. Merge the archived delta into the permanent corpus, recording per-file
///    progress in the receipt.
/// 4. Publish the candidate revision.
/// 5. Run each verification scope once against the committed candidate.
/// 6. Run the optional coverage gate.
/// 7. Guard the tested commit, write the proven receipt, and publish the
///    evidence-only finalization.
///
/// A failure after the candidate commit leaves the candidate unproven; the
/// developer repairs the committed state and reruns the command.
///
/// State invariant: at every successful step the project has either the
/// active change (with a pending receipt when the move was interrupted), a
/// pending archive candidate, or a proven archive. A conflicting
/// active/pending pair is refused before any mutation.
pub fn run(change: &str, verbose: bool) -> Result<()> {
    crate::subprocess::set_verbose(verbose);
    let project_root = std::env::current_dir()?;
    let manifest = load_effective()?;
    let change_dir = crate::state::resolve_change_dir(&project_root, change)?;
    let archive_target = project_root.join("givn/archive").join(change);
    let specs_dir = project_root.join("givn/specs");

    if change_dir.exists() {
        crate::proof::validate_no_behavior_change(&change_dir)?;
    }

    let backend = crate::scm::select(&project_root)?;

    let mut fresh = false;
    let mut recovered_duplicate = false;
    match crate::proof::read_status(&project_root, change) {
        ArchiveProof::Corrupt {
            both_dirs: true,
            pending_conflict: true,
        } => bail!(
            "archive blocked: project state is corrupt — the pending archive candidate \
             'givn/archive/{change}' conflicts with the active change 'givn/changes/{change}'. \
             Repair or remove one side; rerunning archive cannot choose between them."
        ),
        ArchiveProof::Corrupt {
            both_dirs: true,
            pending_conflict: false,
        } => bail!(
            "archive blocked: project state is corrupt — 'givn/changes/{change}' and \
             'givn/archive/{change}' both exist"
        ),
        ArchiveProof::Corrupt {
            both_dirs: false, ..
        } => bail!(
            "archive blocked: project state is corrupt — the archive receipt for \
             'givn/archive/{change}' is unreadable"
        ),
        ArchiveProof::Proven => {
            if recover_proven_finalization(
                &backend,
                &project_root,
                &manifest,
                change,
                &archive_target,
            )? {
                return Ok(());
            }
        }
        ArchiveProof::Legacy => bail!("change '{change}' is already archived"),
        ArchiveProof::Pending => {
            println!(
                "[{}] Resuming unproven archive candidate '{change}'...",
                crate::subprocess::local_timestamp()
            );
        }
        ArchiveProof::PendingWithActive => {
            println!(
                "[{}] Resuming interrupted archive move for '{change}'...",
                crate::subprocess::local_timestamp()
            );
            std::fs::remove_dir_all(&change_dir).with_context(|| {
                format!(
                    "archive blocked: could not remove the duplicate active change '{}'",
                    change_dir.display()
                )
            })?;
            backend.stage_paths(
                &[std::path::PathBuf::from(format!("givn/changes/{change}"))],
                false,
            )?;
            recovered_duplicate = true;
        }
        ArchiveProof::None => {
            if !change_dir.exists() {
                bail!("change '{}' not found under givn/changes/", change);
            }
            fresh = true;
        }
    }

    if fresh {
        ensure_clean(&backend, &project_root, change, &manifest, Witness::Project)?;
        ensure_change_receipt(&backend, &manifest, &project_root, &change_dir, change)?;
        // A declared retained path must survive the move. The declaration is
        // read through the one strict loader and only while `ise` is
        // effective; nothing is written before this refusal.
        ensure_declared_paths_survive_archive(&project_root, &backend, &manifest, change)?;
        println!(
            "[{}] Archiving change '{change}'...",
            crate::subprocess::local_timestamp()
        );

        // 1. Every apply.requires artifact must be Done (or Skipped).
        ensure_artifact_gates(&change_dir, &manifest)?;
        // 1b. The planning-layer gate is mandatory in the default graph even
        //     when a custom apply.requires omits it.
        ensure_design_review_gate(&change_dir, &manifest)?;

        let features_path = change_dir.join("specs").to_string_lossy().into_owned();

        // 1a. Delta step definitions must already bind in the permanent test
        //     tree; archive merges features and narratives only.
        ensure_step_definitions_merged(&project_root, &change_dir, StepGatePhase::Fresh)?;
        validate_delta_publication(&change_dir.join("specs"))?;

        // 2. Configured archive hooks.
        println!(
            "[{}] Running archive gate hooks...",
            crate::subprocess::local_timestamp()
        );
        let mut hooks_manifest = manifest.clone();
        crate::cli::check::remove_verification_hooks(&mut hooks_manifest);
        crate::hooks::run_archive_gate_hooks(&hooks_manifest, &features_path)?;
        println!(
            "[{}] ✓ Gate hooks passed",
            crate::subprocess::local_timestamp()
        );

        // 3. Run declaration gate, before the README gate.
        ensure_run_declaration(&manifest)?;

        // 4. README completeness gate.
        if manifest.addons.readme {
            ensure_readme(&project_root)?;
        }

        // 4b. Re-run the static review gates. No test suite runs here; lint
        //     stays review-only.
        run_static_review_gates(&project_root, change, &features_path, &manifest)?;

        // 5. Merge preconditions: use-case base revisions and delta parsing.
        let source_specs = change_dir.join("specs");
        validate_usecase_documents(&project_root, &source_specs)?;
        let plan = build_merge_plan(&project_root, &change_dir, change, &source_specs, true)?;

        // 5b. The projected interaction inventory is a merge precondition:
        //     a delta that leaves the merged corpus inconsistent is refused
        //     before any mutation.
        let projected = interaction_findings_for_plan(
            &project_root,
            &source_specs,
            &plan,
            MergeProgress::empty(),
        )?;
        ensure_interaction_inventory(projected)?;

        // 6. E2E runner configuration is a configuration gap, not a
        //    verification result: block before any mutation.
        let merged_has_e2e = has_e2e_scenarios(&source_specs)
            || crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;

        // 7. Write the pending receipt into the active change, then move the
        //    whole directory with one rename. The merge runs against the
        //    archived candidate.
        let started_at_unix = crate::proof::unix_now();
        let (merge_plan, merge_targets) = receipt_plan(&plan);
        crate::proof::Receipt::write_pending_progress(
            &change_dir,
            change,
            started_at_unix,
            false,
            Vec::new(),
            None,
            merge_plan,
            merge_targets,
        )?;
        cleanup_stale_temporaries(&project_root, &change_dir, &archive_target)?;
        move_change_to_archive(&change_dir, &archive_target)?;

        let receipt = crate::proof::read_receipt(&archive_target)
            .context("archive blocked: pending receipt missing after the move")?;
        apply_merge_plan(&project_root, &archive_target, change, &plan, &receipt)?;
        publish_candidate(&backend, change)?;
        println!(
            "[{}] ✓ Candidate committed",
            crate::subprocess::local_timestamp()
        );
    } else {
        let receipt = crate::proof::read_receipt(&archive_target).ok_or_else(|| {
            anyhow::anyhow!(
                "archive blocked: pending receipt missing at {}",
                archive_target.join(crate::proof::RECEIPT_FILE).display()
            )
        })?;

        let merge_needed = !receipt.merge_completed;
        if merge_needed {
            let mut allowed = receipt_targets(&project_root, &receipt);
            allowed.push(project_root.join("givn/specs/.retired.yaml"));
            if recovered_duplicate {
                allowed.push(project_root.join(format!("givn/changes/{change}")));
            }
            ensure_clean_allowed(
                &backend,
                &project_root,
                change,
                &manifest,
                Witness::Project,
                &allowed,
            )?;
        } else if recovered_duplicate {
            ensure_clean_allowed(
                &backend,
                &project_root,
                change,
                &manifest,
                Witness::Project,
                &[project_root.join(format!("givn/changes/{change}"))],
            )?;
        } else {
            ensure_clean(&backend, &project_root, change, &manifest, Witness::Project)?;
        }

        // Re-check the step-definition precondition before the candidate is
        // merged or verified again.
        ensure_step_definitions_merged(&project_root, &archive_target, StepGatePhase::Resume)?;
        validate_delta_publication(&archive_target.join("specs"))?;

        // Re-check the e2e precondition against the permanent corpus before
        // re-running.
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;

        let mut merge_applied = false;
        if merge_needed {
            let plan = plan_from_receipt(&project_root, &archive_target, change, &receipt)?;
            // Project the remaining merge before touching the corpus. A
            // pending receipt written before merge progress existed cannot
            // say which files were applied, so mirror the writer's legacy
            // retry mode.
            let legacy_unrecorded_merge = receipt.merge_plan.is_empty()
                && receipt.merged_deltas.is_empty()
                && receipt.merge_in_progress.is_none();
            let progress = MergeProgress {
                merged: &receipt.merged_deltas,
                in_progress: receipt.merge_in_progress.as_deref(),
                retry_unrecorded: legacy_unrecorded_merge,
            };
            let projected = interaction_findings_for_plan(
                &project_root,
                &archive_target.join("specs"),
                &plan,
                progress,
            )?;
            ensure_interaction_inventory(projected)?;
            cleanup_stale_temporaries(&project_root, &change_dir, &archive_target)?;
            merge_applied =
                apply_merge_plan(&project_root, &archive_target, change, &plan, &receipt)?;
        }

        let receipt_in_head = backend.revision_contains(&format!(
            "givn/archive/{change}/{}",
            crate::proof::RECEIPT_FILE
        ))?;
        let duplicate_needs_publication = recovered_duplicate && backend.has_staged_changes()?;
        if !receipt_in_head || merge_applied || duplicate_needs_publication {
            publish_candidate(&backend, change)?;
            println!(
                "[{}] ✓ Candidate committed",
                crate::subprocess::local_timestamp()
            );
        }
    }

    // 7b. The merged permanent corpus must satisfy the same interaction gate
    //     before any verification scope runs.
    ensure_interaction_inventory(permanent_interaction_findings(&project_root)?)?;

    // 8. Verification scopes run once against the committed candidate.
    let tested_revision = backend.revision()?;
    let started_at_unix = crate::proof::unix_now();
    let mut scopes = std::collections::BTreeMap::new();

    println!(
        "Archive proof excludes the entire archive directory; executable changes inside exempt paths may escape cheap input checks."
    );
    let regular = verify_archive_scope(
        &project_root,
        &backend,
        &manifest,
        change,
        &archive_target,
        SCOPE_REGULAR,
        &manifest.verify.command,
    )?;
    scopes.insert(SCOPE_REGULAR.to_string(), regular);

    let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
    if merged_has_e2e {
        let e2e = verify_archive_scope(
            &project_root,
            &backend,
            &manifest,
            change,
            &archive_target,
            SCOPE_E2E,
            &manifest.verify.e2e_command,
        )?;
        scopes.insert(SCOPE_E2E.to_string(), e2e);
    } else {
        scopes.insert(
            SCOPE_E2E.to_string(),
            crate::proof::ScopeOutcome::not_applicable(),
        );
    }

    // 9. Coverage runs once against the candidate, without rollback.
    let coverage = if manifest.addons.coverage {
        Some(verify_archive_coverage(
            &project_root,
            &backend,
            &manifest,
            change,
            &archive_target,
        )?)
    } else {
        None
    };

    // 10. Finalization: refresh managed blocks, guard the tested commit,
    //     then write and commit the proven receipt.
    if manifest.addons.readme {
        refresh_managed_blocks(&project_root)?;
    }

    let head = backend.revision()?;
    if head != tested_revision {
        bail!(
            "archive blocked: the tested tree changed — the current revision is {head} but {tested_revision} was \
             verified. Repair the committed state and rerun `givn archive --change {change}`."
        );
    }
    ensure_clean(&backend, &project_root, change, &manifest, Witness::Tested)?;

    crate::proof::Receipt::write_proven(
        &archive_target,
        change,
        &tested_revision,
        scopes,
        coverage,
        started_at_unix,
        crate::proof::unix_now(),
    )?;
    if let Err(error) = backend.publish(
        &finalization_evidence_paths(&project_root, &manifest, change),
        &format!("chore({change}): prove archive"),
    ) {
        crate::proof::Receipt::resume_pending(&archive_target)?;
        return Err(error);
    }

    print_archived(change, &archive_target);
    Ok(())
}

fn receipt_plan(plan: &[MergeEntry]) -> (Vec<String>, BTreeMap<String, String>) {
    let mut order = Vec::new();
    let mut targets = BTreeMap::new();
    for entry in plan {
        order.push(entry.relative.clone());
        let relative = entry.target_rel.to_string_lossy().replace('\\', "/");
        targets.insert(entry.relative.clone(), relative);
    }
    (order, targets)
}

fn receipt_targets(project_root: &Path, receipt: &crate::proof::Receipt) -> Vec<PathBuf> {
    receipt
        .merge_targets
        .values()
        .map(|value| project_root.join(value))
        .collect()
}

/// Which archive phase runs the step-definition preflight. The phase only
/// changes the refusal lead-in; the comparison is identical.
#[derive(Clone, Copy)]
enum StepGatePhase {
    Fresh,
    Resume,
}

/// Directory pruning for the step-definition walks. `components` names are
/// pruned at any depth; `prefixes` are project-relative paths pruned only on
/// the permanent walk so the change/archive trees cannot satisfy their own
/// phrases.
#[derive(Clone, Copy)]
struct PruneRules {
    components: &'static [&'static str],
    prefixes: &'static [&'static str],
}

const STEP_COMPONENT_PRUNES: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "target",
    "node_modules",
    ".venv",
    "venv",
    "coverage",
    "dist",
    "build",
    "vendor",
    "__pycache__",
];

const PERMANENT_PREFIX_PRUNES: &[&str] = &["givn/changes", "givn/archive", "givn/history"];

const DELTA_PRUNE: PruneRules = PruneRules {
    components: STEP_COMPONENT_PRUNES,
    prefixes: &[],
};

const PERMANENT_PRUNE: PruneRules = PruneRules {
    components: STEP_COMPONENT_PRUNES,
    prefixes: PERMANENT_PREFIX_PRUNES,
};

/// Caps the phrases named in one refusal. The refusal still reports the exact
/// remainder so a large gap is visible without an unbounded error.
const MAX_REPORTED_PHRASES: usize = 20;

/// Binding tokens that introduce a step phrase. They are matched anywhere in
/// a step-definition file and consumed as an expression, so multi-line
/// bindings and raw-string regexes are read whole.
const BINDING_TOKENS: &[&str] = &[
    "#[given(", "#[when(", "#[then(", "@given(", "@when(", "@then(", "@step(", "@Given(", "@When(",
    "@Then(", "@And(", "@But(", "@Step(", "Given(", "When(", "Then(", "And(", "But(", "Step(",
    ".Given(", ".When(", ".Then(", ".Step(",
];

impl PruneRules {
    fn prunes_dir(&self, relative: &Path) -> bool {
        if self.components.iter().any(|name| {
            relative
                .components()
                .any(|component| component.as_os_str().to_string_lossy() == *name)
        }) {
            return true;
        }
        self.prefixes
            .iter()
            .any(|prefix| relative.starts_with(prefix))
    }
}

/// The documented path rule: a file under a directory named `steps` or ending
/// in `_steps`, or a file whose name ends in `_steps` (with or without an
/// extension).
fn is_step_definition_path(path: &Path) -> bool {
    if path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| stem.ends_with("_steps"))
    {
        return true;
    }
    path.parent().is_some_and(|parent| {
        parent.components().any(|component| {
            let name = component.as_os_str().to_string_lossy();
            name == "steps" || name.ends_with("_steps")
        })
    })
}

/// Return each supported binding expression's text between its `(` and the
/// matching `)`. A token that starts inside an already-consumed expression is
/// skipped, so `@Given(` is not parsed twice as `Given(`.
fn step_binding_expressions(text: &str) -> Vec<&str> {
    let mut matches: Vec<(usize, &'static str)> = Vec::new();
    for token in BINDING_TOKENS {
        for (index, _) in text.match_indices(token) {
            if text[..index]
                .chars()
                .next_back()
                .is_some_and(|before| before.is_alphanumeric() || before == '_')
            {
                continue;
            }
            matches.push((index, token));
        }
    }
    matches.sort_unstable();
    let mut expressions = Vec::new();
    let mut consumed = 0;
    for (index, token) in matches {
        if index < consumed {
            continue;
        }
        let open = index + token.len() - 1;
        let Some(close) = matching_paren(text, open) else {
            continue;
        };
        expressions.push(&text[open + 1..close]);
        consumed = close + 1;
    }
    expressions
}

/// Find the `)` matching the `(` at `open`, skipping escaped strings, raw
/// strings, regex literals, and comments.
fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(open) != Some(&b'(') {
        return None;
    }
    let mut depth = 1usize;
    let mut index = open + 1;
    while index < text.len() {
        if let Some((_, end)) = parse_literal(text, index) {
            index = end;
            continue;
        }
        if let Some((_, end)) = parse_regex_literal(text, index) {
            index = end;
            continue;
        }
        if let Some(end) = skip_comment(text, index) {
            index = end;
            continue;
        }
        match bytes[index] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += char_len(text, index);
    }
    None
}

fn char_len(text: &str, index: usize) -> usize {
    text[index..]
        .chars()
        .next()
        .map(char::len_utf8)
        .unwrap_or(1)
}

/// Skip a line or block comment starting at `index`. `#` is a Python comment
/// only when it starts a line's content; inside an expression it is rare.
fn skip_comment(text: &str, index: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(index) == Some(&b'/') && bytes.get(index + 1) == Some(&b'/') {
        return text[index..].find('\n').map(|offset| index + offset + 1);
    }
    if bytes.get(index) == Some(&b'/') && bytes.get(index + 1) == Some(&b'*') {
        return text[index + 2..]
            .find("*/")
            .map(|offset| index + 2 + offset + 2);
    }
    if bytes.get(index) == Some(&b'#') {
        return text[index..].find('\n').map(|offset| index + offset + 1);
    }
    None
}

/// Parse a quoted literal, raw string, or template literal at `index`.
/// Returns the literal content and the byte index after it.
fn parse_literal(text: &str, index: usize) -> Option<(String, usize)> {
    let bytes = text.as_bytes();
    let first = *bytes.get(index)?;
    if first == b'r' || first == b'R' {
        let mut hashes = 0;
        let mut cursor = index + 1;
        while bytes.get(cursor) == Some(&b'#') {
            hashes += 1;
            cursor += 1;
        }
        let quote = *bytes.get(cursor)?;
        if quote == b'"' || quote == b'\'' {
            let content_start = cursor + 1;
            let mut end = content_start;
            while end < text.len() {
                if bytes[end] == quote {
                    let mut close = end + 1;
                    let mut seen = 0;
                    while seen < hashes && bytes.get(close) == Some(&b'#') {
                        close += 1;
                        seen += 1;
                    }
                    if seen == hashes {
                        return Some((text[content_start..end].to_string(), close));
                    }
                }
                end += 1;
            }
            return None;
        }
    }
    if !matches!(first, b'"' | b'\'' | b'`') {
        return None;
    }
    let mut escaped = false;
    let mut cursor = index + 1;
    while cursor < text.len() {
        let byte = bytes[cursor];
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == first {
            return Some((text[index + 1..cursor].to_string(), cursor + 1));
        }
        cursor += 1;
    }
    None
}

/// Parse a `/regex/` literal when the slash starts a regex position. Returns
/// the pattern and the index after the closing slash.
fn parse_regex_literal(text: &str, index: usize) -> Option<(String, usize)> {
    if text.as_bytes().get(index) != Some(&b'/') || !is_regex_start(text, index) {
        return None;
    }
    let bytes = text.as_bytes();
    let mut escaped = false;
    let mut in_class = false;
    let mut cursor = index + 1;
    while cursor < text.len() {
        let byte = bytes[cursor];
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'[' {
            in_class = true;
        } else if byte == b']' {
            in_class = false;
        } else if byte == b'/' && !in_class {
            return Some((text[index + 1..cursor].to_string(), cursor + 1));
        }
        cursor += 1;
    }
    None
}

fn is_regex_start(text: &str, index: usize) -> bool {
    text[..index]
        .chars()
        .rev()
        .find(|character| !character.is_whitespace())
        .is_none_or(|before| matches!(before, '(' | ',' | '=' | ':'))
}

/// Extract the first phrase literal from one binding expression. Python's
/// implicit adjacent literals are joined; internal whitespace is normalized.
/// A binding whose phrase is computed dynamically yields `None`.
fn binding_phrase(expression: &str) -> Option<String> {
    let mut cursor = 0;
    while cursor < expression.len() {
        if let Some((literal, end)) = parse_literal(expression, cursor) {
            let mut phrase = literal;
            let mut next = end;
            while let Some(joined) =
                expression[next..].find(|character: char| !character.is_whitespace())
            {
                let at = next + joined;
                let Some((adjacent, after)) = parse_literal(expression, at) else {
                    break;
                };
                phrase.push_str(&adjacent);
                next = after;
            }
            let normalized = phrase.split_whitespace().collect::<Vec<_>>().join(" ");
            return (!normalized.is_empty()).then_some(normalized);
        }
        if let Some((regex, _end)) = parse_regex_literal(expression, cursor) {
            let normalized = regex.split_whitespace().collect::<Vec<_>>().join(" ");
            return (!normalized.is_empty()).then_some(normalized);
        }
        if let Some(end) = skip_comment(expression, cursor) {
            cursor = end;
            continue;
        }
        if let Some(character) = expression[cursor..].chars().next() {
            cursor += character.len_utf8();
        } else {
            break;
        }
    }
    None
}

fn collect_step_phrases(path: &Path) -> BTreeSet<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return BTreeSet::new();
    };
    step_binding_expressions(&text)
        .into_iter()
        .filter_map(binding_phrase)
        .collect()
}

fn collect_step_definition_files(root: &Path, rules: PruneRules) -> Vec<PathBuf> {
    let mut files = Vec::new();
    walk_step_definition_files(root, root, rules, &mut files);
    files.sort();
    files
}

fn walk_step_definition_files(
    root: &Path,
    directory: &Path,
    rules: PruneRules,
    files: &mut Vec<PathBuf>,
) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            if !rules.prunes_dir(relative) {
                walk_step_definition_files(root, &path, rules, files);
            }
        } else if file_type.is_file() && is_step_definition_path(&path) {
            files.push(path);
        }
    }
}

fn collect_phrases_in_tree(root: &Path, rules: PruneRules) -> BTreeSet<String> {
    let mut phrases = BTreeSet::new();
    for file in collect_step_definition_files(root, rules) {
        phrases.extend(collect_step_phrases(&file));
    }
    phrases
}

/// Step definitions are durable project source, not delta artifacts. Archive
/// merges `.feature` files and use-case documents only, so a delta step phrase
/// with no binding in the permanent test tree would be buried in the archive
/// while the merged suite fails with an undefined step. Refuse before any
/// mutation and name the project-relative delta file and the missing phrases.
fn ensure_step_definitions_merged(
    project_root: &Path,
    change_root: &Path,
    phase: StepGatePhase,
) -> Result<()> {
    let delta_root = change_root.join("specs");
    if !delta_root.exists() {
        return Ok(());
    }
    let delta_files = collect_step_definition_files(&delta_root, DELTA_PRUNE);
    if delta_files.is_empty() {
        return Ok(());
    }
    let permanent = collect_phrases_in_tree(project_root, PERMANENT_PRUNE);
    let mut gaps: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    for file in delta_files {
        let missing: Vec<String> = collect_step_phrases(&file)
            .into_iter()
            .filter(|phrase| !permanent.contains(phrase))
            .collect();
        if !missing.is_empty() {
            gaps.insert(file, missing);
        }
    }
    if gaps.is_empty() {
        return Ok(());
    }

    let total: usize = gaps.values().map(Vec::len).sum();
    let mut listed = Vec::new();
    let mut shown = 0;
    for (file, phrases) in &gaps {
        let relative = file.strip_prefix(project_root).unwrap_or(file);
        for phrase in phrases {
            if shown == MAX_REPORTED_PHRASES {
                break;
            }
            listed.push(format!("  - \"{phrase}\" ({})", relative.display()));
            shown += 1;
        }
        if shown == MAX_REPORTED_PHRASES {
            break;
        }
    }
    let lead = match phase {
        StepGatePhase::Fresh => {
            "archive blocked: delta step definitions are not present in the permanent test tree:"
        }
        StepGatePhase::Resume => {
            "archive blocked: the unproven candidate's delta step definitions are not present in the permanent test tree:"
        }
    };
    let remainder = if total > shown {
        format!("\n  +{} more", total - shown)
    } else {
        String::new()
    };
    let change = change_root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("the change");
    bail!(
        "{lead}\n{}{remainder}\n\
         The archive merges feature files and use-case documents only. Merge each definition \
         into the permanent per-capability step file, keep the delta copy for the change suite, \
         and rerun `givn archive --change {change}`.",
        listed.join("\n")
    )
}

fn validate_delta_publication(specs: &Path) -> Result<()> {
    for path in collect_feature_files(specs) {
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read delta feature '{}'", path.display()))?;
        crate::gherkin::parse::parse_supported_source(&source, &path)?;
    }
    Ok(())
}

fn plan_from_receipt(
    project_root: &Path,
    archive_target: &Path,
    change: &str,
    receipt: &crate::proof::Receipt,
) -> Result<Vec<MergeEntry>> {
    let source_specs = archive_target.join("specs");
    if receipt.merge_plan.is_empty() {
        return Ok(build_merge_plan(
            project_root,
            archive_target,
            change,
            &source_specs,
            false,
        )?);
    }
    let mut plan = Vec::new();
    for rel in &receipt.merge_plan {
        let delta_path = source_specs.join(rel);
        let feature =
            gherkin::Feature::parse_path(&delta_path, GherkinEnv::default()).map_err(|e| {
                anyhow::anyhow!(
                    "failed to parse archived delta feature '{}': {}",
                    delta_path.display(),
                    e
                )
            })?;
        let cap = delta_capability(&delta_path, &feature)?;
        let value = receipt.merge_targets.get(rel).ok_or_else(|| {
            anyhow::anyhow!("archive blocked: the pending receipt has no merge target for '{rel}'")
        })?;
        let target_rel = PathBuf::from(value);
        if !target_rel.starts_with("givn/specs") {
            bail!("archive blocked: pending merge target '{value}' is outside givn/specs");
        }
        let target = project_root.join(&target_rel);
        plan.push(MergeEntry {
            relative: rel.clone(),
            retire: crate::gherkin::delta::is_retired_feature(&feature),
            feature,
            target,
            target_rel,
            cap,
        });
    }
    Ok(plan)
}

/// Apply the archived delta to the permanent corpus. The change-scoped suite
/// already ran under `givn check tasks`; only the merged regular/e2e scopes
/// run after the candidate is published.
fn apply_merge_plan(
    project_root: &Path,
    archive_target: &Path,
    change: &str,
    plan: &[MergeEntry],
    receipt: &crate::proof::Receipt,
) -> Result<bool> {
    merge_usecase_documents(project_root, &archive_target.join("specs"))?;
    // A pending receipt written before merge progress existed cannot say
    // which files were applied. Treat every file as a retry so an already
    // merged delta is a no-op instead of a duplicate-title error.
    let legacy_unrecorded_merge = receipt.merge_plan.is_empty()
        && receipt.merged_deltas.is_empty()
        && receipt.merge_in_progress.is_none();
    let mut applied = false;
    for entry in plan {
        if receipt
            .merged_deltas
            .iter()
            .any(|done| done == &entry.relative)
        {
            continue;
        }
        let retry = legacy_unrecorded_merge
            || receipt.merge_in_progress.as_deref() == Some(entry.relative.as_str());
        crate::proof::Receipt::mark_merge_started(archive_target, &entry.relative)?;
        if entry.retire {
            retire_capability(project_root, change, &entry.cap, &entry.target)?;
        } else if retry {
            write_feature_retry(&entry.target, &entry.feature)?;
        } else {
            write_feature(&entry.target, &entry.feature)?;
        }
        crate::proof::Receipt::mark_merge_file_done(archive_target, &entry.relative)?;
        applied = true;
    }
    crate::proof::Receipt::mark_merge_completed(archive_target)?;
    Ok(applied)
}

fn move_change_to_archive(change_dir: &Path, archive_target: &Path) -> Result<()> {
    if let Some(parent) = archive_target.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "archive blocked: could not create the archive location '{}'",
                archive_target.display()
            )
        })?;
    }
    std::fs::rename(change_dir, archive_target).with_context(|| {
        format!(
            "archive blocked: could not move '{}' to '{}'. The active change is in place \
             with its pending receipt; rerun `givn archive` to retry.",
            change_dir.display(),
            archive_target.display()
        )
    })
}

fn cleanup_stale_temporaries(
    project_root: &Path,
    change_dir: &Path,
    archive_target: &Path,
) -> Result<()> {
    for dir in [change_dir, archive_target] {
        let temporary = dir.join(format!(".{}.givn-tmp", crate::proof::RECEIPT_FILE));
        if temporary.exists() {
            std::fs::remove_file(&temporary)
                .with_context(|| format!("remove stale temporary {}", temporary.display()))?;
        }
    }
    remove_temporary_files(&project_root.join("givn/specs"))
}

fn remove_temporary_files(dir: &Path) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_temporary_files(&path)?;
        } else if path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().ends_with(".givn-tmp"))
        {
            std::fs::remove_file(&path)
                .with_context(|| format!("remove stale temporary {}", path.display()))?;
        }
    }
    Ok(())
}

fn ensure_artifact_gates(
    change_dir: &Path,
    manifest: &crate::config::manifest::Manifest,
) -> Result<()> {
    let state = ChangeState::from_manifest(change_dir, manifest);
    for req in &manifest.apply.requires {
        let artifact_state = state.artifacts.iter().find(|a| a.id == *req);
        match artifact_state {
            None => bail!(
                "archive blocked: required artifact '{}' not found in manifest",
                req
            ),
            Some(a) if a.status == ArtifactStatus::Skipped => {}
            Some(a) if a.status == ArtifactStatus::Pending => bail!(
                "archive blocked: required artifact '{}' does not exist for change '{}'",
                req,
                change_dir.file_name().unwrap_or_default().to_string_lossy()
            ),
            Some(a) if a.status == ArtifactStatus::Partial => {
                if let Some(ref p) = a.tasks_progress {
                    bail!(
                        "archive blocked: tasks not complete ({}/{} done)",
                        p.done,
                        p.total
                    );
                }
                bail!(
                    "archive blocked: artifact '{}' is incomplete. \
                     Ensure the sign-off marker is present \
                     (e.g. 'DESIGN-REVIEW: PASS' or 'REVIEW: PASS').",
                    req
                );
            }
            _ => {}
        }
    }
    Ok(())
}

/// Re-run the static review gates at archive time. No test suite runs here:
/// scenario execution is proven by the change receipt and the archive scopes.
fn run_static_review_gates(
    project_root: &Path,
    change: &str,
    features_path: &str,
    manifest: &crate::config::manifest::Manifest,
) -> Result<()> {
    println!(
        "[{}] Running static review gates...",
        crate::subprocess::local_timestamp()
    );
    crate::review_context::check(&project_root.join("givn/changes").join(change))?;
    crate::integrity_hook::run_integrity_hook(
        features_path,
        manifest.addons.arc42,
        manifest.addons.coverage,
        manifest.addons.readme,
    )?;
    crate::config::lock::check(project_root, change)?;
    crate::overlap::analysis::run_review_overlap_gate(project_root, change)?;
    crate::cli::check::check_retirement_disclosed(project_root, change)?;
    crate::spec::routing_table::check_routing_consistency(project_root, change)?;
    println!(
        "[{}] ✓ Static review gates passed",
        crate::subprocess::local_timestamp()
    );
    Ok(())
}

/// Refuse archive when the planning-layer review is missing or unsigned.
///
/// `design-review` is a normal optional-skippable artifact: `.givn-skip`
/// satisfies it, and a manifest without the artifact is unaffected. The check
/// exists because `apply.requires` is configurable and a custom list must not
/// silently drop the planning gate.
fn ensure_design_review_gate(
    change_dir: &Path,
    manifest: &crate::config::manifest::Manifest,
) -> Result<()> {
    if !manifest
        .artifacts
        .iter()
        .any(|artifact| artifact.id == "design-review")
        || crate::state::is_skipped("design-review", change_dir)
    {
        return Ok(());
    }
    let state = ChangeState::from_manifest(change_dir, manifest);
    let Some(artifact) = state
        .artifacts
        .iter()
        .find(|artifact| artifact.id == "design-review")
    else {
        return Ok(());
    };
    match artifact.status {
        ArtifactStatus::Done | ArtifactStatus::Skipped => Ok(()),
        ArtifactStatus::Pending => bail!(
            "archive blocked: required artifact 'design-review' does not exist for change '{}'",
            change_dir.file_name().unwrap_or_default().to_string_lossy()
        ),
        ArtifactStatus::Partial => bail!(
            "archive blocked: artifact 'design-review' is incomplete. \
             Ensure the sign-off marker is present (e.g. 'DESIGN-REVIEW: PASS')."
        ),
    }
}

fn ensure_run_declaration(manifest: &crate::config::manifest::Manifest) -> Result<()> {
    let findings = crate::cli::run_block::check_run(manifest);
    if !findings.is_empty() {
        println!(
            "archive blocked: run declaration has {} finding(s):",
            findings.len()
        );
        for finding in &findings {
            println!(
                "  [{}] {}",
                crate::cli::run_block::label(finding.kind),
                finding.message
            );
        }
        bail!(
            "archive blocked: finish the run declaration in givn/commands.yaml — \
             see the findings above"
        );
    }
    println!(
        "[{}] ✓ run declaration passed",
        crate::subprocess::local_timestamp()
    );
    Ok(())
}

fn ensure_readme(project_root: &Path) -> Result<()> {
    let findings = crate::cli::readme::check_readme(project_root);
    if !findings.is_empty() {
        println!(
            "archive blocked: README check found {} finding(s):",
            findings.len()
        );
        for finding in &findings {
            println!("  [{}] {}", finding_label(finding.kind), finding.message);
        }
        bail!(
            "archive blocked: finish the README before archiving — see the README\
             check report above"
        );
    }
    println!(
        "[{}] ✓ README completeness gate passed",
        crate::subprocess::local_timestamp()
    );
    Ok(())
}

fn refresh_managed_blocks(project_root: &Path) -> Result<()> {
    match crate::cli::readme::refresh_development_block(project_root) {
        Ok(true) => println!(
            "[{}] ✓ development managed block refreshed",
            crate::subprocess::local_timestamp()
        ),
        Ok(false) => {}
        Err(e) => bail!(
            "archive blocked: development managed block refresh failed: {e:#}. The archive \
             candidate is unproven."
        ),
    }
    match crate::cli::readme::refresh_run_block(project_root) {
        Ok(true) => println!(
            "[{}] ✓ run managed block refreshed",
            crate::subprocess::local_timestamp()
        ),
        Ok(false) => {}
        Err(e) => bail!(
            "archive blocked: run managed block refresh failed: {e:#}. The archive candidate \
             is unproven."
        ),
    }
    Ok(())
}

fn finalization_evidence_paths(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
    change: &str,
) -> Vec<String> {
    let mut evidence_paths = vec![format!("givn/archive/{change}")];
    if manifest.addons.readme && project_root.join("README.md").exists() {
        evidence_paths.push("README.md".to_string());
    }
    if manifest.addons.coverage {
        for candidate in [
            &manifest.coverage.non_e2e_path,
            &manifest.coverage.e2e_path,
            &manifest.coverage.output_path,
        ] {
            if project_root.join(candidate).exists() {
                evidence_paths.push(candidate.trim_start_matches("./").to_string());
            }
        }
    }
    evidence_paths
}

fn print_archived(change: &str, archive_target: &Path) {
    println!(
        "[{}] ✓ Archived '{change}' to {}",
        crate::subprocess::local_timestamp(),
        archive_target.display()
    );
    println!(
        "Verification receipt: {}",
        archive_target.join(crate::proof::RECEIPT_FILE).display()
    );
    println!("\nChange archived successfully.");
    println!("The delta specs have been merged into givn/specs/.");
}

fn recover_proven_finalization(
    backend: &crate::scm::Backend,
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
    change: &str,
    archive_target: &Path,
) -> Result<bool> {
    let committed = backend.revision_file(&format!(
        "givn/archive/{change}/{}",
        crate::proof::RECEIPT_FILE
    ))?;
    let committed_proven = committed
        .and_then(|text| serde_json::from_str::<crate::proof::Receipt>(&text).ok())
        .is_some_and(|receipt| receipt.status == crate::proof::STATUS_PROVEN);
    if committed_proven {
        bail!("archive blocked: '{change}' is already proven");
    }
    let Some(receipt) = crate::proof::read_receipt(archive_target) else {
        bail!("archive blocked: '{change}' is already proven");
    };
    if receipt.status != crate::proof::STATUS_PROVEN {
        bail!("archive blocked: '{change}' is already proven");
    }
    ensure_clean(backend, project_root, change, manifest, Witness::Project)?;
    crate::proof::Receipt::resume_pending(archive_target)?;
    println!(
        "[{}] The proven receipt names an older revision; resuming verification...",
        crate::subprocess::local_timestamp()
    );
    Ok(false)
}

fn ensure_clean_allowed(
    backend: &crate::scm::Backend,
    project_root: &Path,
    change: &str,
    manifest: &crate::config::manifest::Manifest,
    witness: Witness,
    allowed: &[PathBuf],
) -> Result<()> {
    let dirty = backend.dirty_entries()?;
    let offending: Vec<String> = dirty
        .iter()
        .filter(|path| {
            !crate::proof::is_evidence_path(project_root, change, manifest, path)
                && !allowed
                    .iter()
                    .any(|permitted| path == &permitted || path.starts_with(permitted))
        })
        .map(|path| path.display().to_string())
        .collect();
    if offending.is_empty() {
        return Ok(());
    }
    match witness {
        Witness::Project => bail!(
            "archive blocked: the project has uncommitted work: {}",
            offending.join(", ")
        ),
        Witness::Tested => bail!(
            "archive blocked: the tested tree changed outside the declared evidence: {}",
            offending.join(", ")
        ),
    }
}

/// Publish the merged candidate revision, validating the exact paths and
/// reporting a rejection as an unproven, repairable state.
fn publish_candidate(backend: &crate::scm::Backend, change: &str) -> Result<()> {
    let change_path = format!("givn/changes/{change}");
    let mut candidate_paths = vec!["givn/specs".to_string(), format!("givn/archive/{change}")];
    // A staged-only or untracked restore can leave no change-path diff after
    // the duplicate is removed; Git rejects a pathspec that matches nothing.
    if backend
        .dirty_entries()?
        .iter()
        .any(|path| path.starts_with(&change_path))
    {
        candidate_paths.push(change_path);
    }
    backend
        .publish(
            &candidate_paths,
            &format!("chore({change}): publish archive candidate"),
        )
        .with_context(|| {
            format!(
                "archive blocked: the candidate revision was rejected. {}. The archive is \
                 not proven. Resolve the error and rerun `givn archive --change {change}`.",
                rejection_state(backend)
            )
        })?;
    Ok(())
}

/// The backend-specific sentence describing a rejected candidate's
/// intermediate state.
fn rejection_state(backend: &crate::scm::Backend) -> &'static str {
    match backend {
        crate::scm::Backend::Git(_) => "The candidate state is staged but not committed",
        crate::scm::Backend::Fossil(_) => "The candidate state stays in the checkout uncommitted",
    }
}

/// Refuse a fresh archive unless the change-scoped receipt is current.
/// The receipt is evidence; it must name the same revision, delta-feature
/// digest, and verify command as the tree the merge is about to consume.
fn ensure_change_receipt(
    backend: &crate::scm::Backend,
    manifest: &crate::config::manifest::Manifest,
    project_root: &Path,
    change_dir: &Path,
    change: &str,
) -> Result<()> {
    if crate::proof::change_verification_not_applicable(change_dir)? {
        println!(
            "  changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)"
        );
        return Ok(());
    }
    let receipt = crate::proof::ChangeReceipt::read(change_dir).map_err(|error| {
        anyhow::anyhow!(
            "archive blocked: {error}. Run `givn check tasks --change {change}` before archiving."
        )
    })?;
    match receipt.freshness(manifest, change_dir, backend, project_root)? {
        crate::proof::ProofStatus::Current => {
            println!(
                "[{}] ✓ change verification receipt passed ({} scenario(s))",
                crate::subprocess::local_timestamp(),
                receipt.result.total
            );
        }
        crate::proof::ProofStatus::DocumentationOnly {
            tested_revision,
            current_revision,
        } => {
            println!(
                "[{}] ✓ change verification receipt reused across documentation-only revision \
                 {current_revision} (tested at {tested_revision}, {} scenario(s))",
                crate::subprocess::local_timestamp(),
                receipt.result.total
            );
        }
        crate::proof::ProofStatus::Stale(reasons) => {
            let details = reasons
                .iter()
                .map(|reason| format!("  - {}", reason.render()))
                .collect::<Vec<_>>()
                .join("\n");
            bail!(
                "archive blocked: the change verification receipt is stale for '{change}':\n\
                 {details}\nRun `givn check tasks --change {change}` again."
            );
        }
    }
    Ok(())
}

/// The largest number of isolated retained paths the refusal lists before it
/// counts the remainder.
const MAX_REPORTED_RETAINED_PATHS: usize = 20;

/// One declared retained entry the archive move would isolate.
struct RetainedPathOffender {
    role: &'static str,
    entry: String,
    moved_path: String,
    advice: Option<String>,
}

/// Refuse a fresh archive while `ise` is effective and a declared retained
/// path would be isolated by the move (design.md "Guard Algorithm").
///
/// The declaration is project-owned: it is read through the one strict loader
/// and never written. An entry survives when it matches a content path outside
/// `givn/changes/<id>/` or the `givn/archive/<id>/` form of one inside it. A
/// project with `ise` off reads no declaration.
fn ensure_declared_paths_survive_archive(
    project_root: &Path,
    backend: &crate::scm::Backend,
    manifest: &crate::config::manifest::Manifest,
    change: &str,
) -> Result<()> {
    if !crate::config::declaration::SpecificationDeclaration::is_effective(manifest) {
        return Ok(());
    }
    let declaration = match crate::config::declaration::SpecificationDeclaration::load(project_root)
    {
        Ok(Some(declaration)) => declaration,
        Ok(None) => return Ok(()),
        Err(error) => {
            bail!("archive blocked: the declared retained paths cannot be read: {error}")
        }
    };
    let mut content_paths: Vec<String> = backend
        .content_paths()?
        .iter()
        .map(|entry| entry.path.to_string_lossy().replace('\\', "/"))
        .collect();
    content_paths.sort();
    let offenders = declared_retained_path_offenders(&declaration, change, &content_paths);
    if offenders.is_empty() {
        return Ok(());
    }
    bail!("{}", retained_path_refusal(&offenders, change))
}

/// The declared retained entries the move isolates, ordered by role
/// declaration order then entry text.
fn declared_retained_path_offenders(
    declaration: &crate::config::declaration::SpecificationDeclaration,
    change: &str,
    content_paths: &[String],
) -> Vec<RetainedPathOffender> {
    let (moved, surviving) = partition_content_paths(change, content_paths);
    let archived: Vec<String> = moved
        .iter()
        .map(|path| archived_counterpart(path, change))
        .collect();
    let mut offenders: Vec<(usize, RetainedPathOffender)> = Vec::new();
    for (role_order, (role, patterns)) in declaration.roles().into_iter().enumerate() {
        for entry in patterns {
            let glob = crate::e2e_boundary::Glob::compile(entry);
            let Some(first_matched) = moved
                .iter()
                .find(|path| crate::e2e_boundary::glob_matches_path_or_parent(&glob, path))
            else {
                continue;
            };
            let survives = surviving
                .iter()
                .map(|path| path.as_str())
                .chain(archived.iter().map(String::as_str))
                .any(|path| crate::e2e_boundary::glob_matches_path_or_parent(&glob, path));
            if survives {
                continue;
            }
            offenders.push((
                role_order,
                RetainedPathOffender {
                    role: role.as_str(),
                    entry: entry.clone(),
                    moved_path: reported_moved_path(&glob, change, first_matched.as_str()),
                    advice: archive_advice(entry),
                },
            ));
        }
    }
    offenders.sort_by(|(left_order, left), (right_order, right)| {
        left_order
            .cmp(right_order)
            .then_with(|| left.entry.cmp(&right.entry))
    });
    offenders
        .into_iter()
        .map(|(_, offender)| offender)
        .collect()
}

/// Split the content listing into the paths this archive moves and the paths
/// that stay.
fn partition_content_paths<'a>(
    change: &str,
    content_paths: &'a [String],
) -> (Vec<&'a String>, Vec<&'a String>) {
    let root = format!("givn/changes/{change}");
    let under = format!("{root}/");
    let mut moved = Vec::new();
    let mut surviving = Vec::new();
    for path in content_paths {
        if path == &root || path.starts_with(&under) {
            moved.push(path);
        } else {
            surviving.push(path);
        }
    }
    (moved, surviving)
}

/// The archive location of a path the move carries:
/// `givn/changes/<id><rest>` → `givn/archive/<id><rest>`.
fn archived_counterpart(path: &str, change: &str) -> String {
    match path.strip_prefix(&format!("givn/changes/{change}")) {
        Some(rest) => format!("givn/archive/{change}{rest}"),
        None => path.to_string(),
    }
}

/// The shallowest prefix of `matched`, at or below `givn/changes/<id>`, that
/// the pattern itself matches: the moved path the refusal names.
fn reported_moved_path(glob: &crate::e2e_boundary::Glob, change: &str, matched: &str) -> String {
    let root = format!("givn/changes/{change}");
    let under = format!("{root}/");
    for (index, _) in matched.match_indices('/') {
        let prefix = &matched[..index];
        if (prefix == root || prefix.starts_with(&under)) && glob.matches(prefix) {
            return prefix.to_string();
        }
    }
    matched.to_string()
}

/// The shape-preserving archived pattern to declare instead: the leading
/// `givn/changes` segments replaced by `givn/archive`. `None` when the entry
/// reaches the change tree through another prefix, so no narrower path is
/// invented.
fn archive_advice(entry: &str) -> Option<String> {
    let mut segments: Vec<&str> = entry.split('/').collect();
    if segments.len() < 2 || segments[0] != "givn" || segments[1] != "changes" {
        return None;
    }
    segments[1] = "archive";
    Some(segments.join("/"))
}

/// The refusal text: bounded offender lines, the remainder count, and the
/// repair sentence naming the declaration.
fn retained_path_refusal(offenders: &[RetainedPathOffender], change: &str) -> String {
    let mut lines = vec![String::from(
        "archive blocked: a declared retained path is moved by this archive:",
    )];
    for offender in offenders.iter().take(MAX_REPORTED_RETAINED_PATHS) {
        let advice = match &offender.advice {
            Some(advice) => format!("declare '{advice}'"),
            None => format!("the archived paths must still match '{}'", offender.entry),
        };
        lines.push(format!(
            "  - {} '{}' matches '{}'; {advice}",
            offender.role, offender.entry, offender.moved_path
        ));
    }
    if offenders.len() > MAX_REPORTED_RETAINED_PATHS {
        lines.push(format!(
            "  +{} more",
            offenders.len() - MAX_REPORTED_RETAINED_PATHS
        ));
    }
    lines.push(format!(
        "A retained role names a path that survives `givn archive`. Declare the archived or \
         another durable path in givn/specification.yaml and rerun `givn archive --change \
         {change}`."
    ));
    lines.join("\n")
}

/// Block unless the dirty worktree contains only evidence paths.
/// Refuse a projected or merged interaction-inventory mismatch.
fn ensure_interaction_inventory(findings: Vec<crate::overlap::analysis::Finding>) -> Result<()> {
    if findings.is_empty() {
        return Ok(());
    }
    let mut message = String::from("archive blocked: interaction inventory mismatch");
    for finding in findings {
        for title in finding.titles {
            message.push_str("\n  - ");
            message.push_str(&title);
        }
    }
    bail!("{message}")
}

fn ensure_clean(
    backend: &crate::scm::Backend,
    project_root: &Path,
    change: &str,
    manifest: &crate::config::manifest::Manifest,
    witness: Witness,
) -> Result<()> {
    let dirty = backend.dirty_entries()?;
    let offending: Vec<String> = dirty
        .iter()
        .filter(|path| !crate::proof::is_evidence_path(project_root, change, manifest, path))
        .map(|path| path.display().to_string())
        .collect();
    if offending.is_empty() {
        return Ok(());
    }
    match witness {
        Witness::Project => bail!(
            "archive blocked: the project has uncommitted work: {}",
            offending.join(", ")
        ),
        Witness::Tested => bail!(
            "archive blocked: the tested tree changed outside the declared evidence: {}",
            offending.join(", ")
        ),
    }
}

/// The E2E runner must be configured when the merged corpus has @e2e
/// scenarios; otherwise the archive blocks before any mutation.
fn ensure_e2e_configured(
    manifest: &crate::config::manifest::Manifest,
    merged_has_e2e: bool,
    specs_dir: &Path,
) -> Result<()> {
    let configured = !manifest.verify.e2e_command.is_empty()
        && manifest.verify.e2e_command != "givn missing-e2e-runner";
    if merged_has_e2e && !configured {
        bail!(
            "'{}' contains @e2e-tagged scenario(s) but verify.e2e_command is not \
             configured in givn/commands.yaml. An @e2e scenario without a configured \
             e2e runner is an unenforced gap, not an optional one — set verify.e2e_command.",
            specs_dir.display()
        );
    }
    Ok(())
}

/// Run one verification scope and validate its fail-closed result. Returns
/// the parsed result and the wall-clock seconds the scope took.
#[allow(clippy::too_many_arguments)]
fn verify_archive_scope(
    project_root: &Path,
    backend: &crate::scm::Backend,
    manifest: &crate::config::manifest::Manifest,
    change: &str,
    archive: &Path,
    scope: &str,
    command: &str,
) -> Result<crate::proof::ScopeOutcome> {
    let before = crate::proof::VerificationInputs::capture_archive(
        project_root,
        backend,
        change,
        manifest,
        command,
        scope,
    )?;
    let path = crate::proof::result_path(archive, scope);
    let receipt = crate::proof::read_receipt(archive).context("archive receipt missing")?;
    if receipt.reusable_scope(scope, &before, &path) {
        println!(
            "  reusing passed {scope} verification (tested at {})",
            receipt.scope_evidence[scope].inputs.revision
        );
        return Ok(receipt.scopes[scope].clone());
    }
    let (result, seconds) = run_scope(project_root, command, scope, &path)?;
    let current = load_effective()?;
    let current_command = if scope == SCOPE_REGULAR {
        &current.verify.command
    } else {
        &current.verify.e2e_command
    };
    let after = crate::proof::VerificationInputs::capture_archive(
        project_root,
        backend,
        change,
        &current,
        current_command,
        scope,
    )?;
    before.ensure_unchanged(&after)?;
    let outcome = crate::proof::ScopeOutcome::passed(&result, seconds);
    crate::proof::Receipt::record_scope(archive, scope, outcome.clone(), before, &path)?;
    Ok(outcome)
}

fn verify_archive_coverage(
    project_root: &Path,
    backend: &crate::scm::Backend,
    manifest: &crate::config::manifest::Manifest,
    change: &str,
    archive: &Path,
) -> Result<crate::proof::CoverageOutcome> {
    let command = serde_json::to_string(&manifest.coverage)?;
    let before = crate::proof::VerificationInputs::capture_archive(
        project_root,
        backend,
        change,
        manifest,
        &command,
        "coverage",
    )?;
    let receipt = crate::proof::read_receipt(archive).context("archive receipt missing")?;
    if receipt
        .coverage_inputs
        .as_ref()
        .is_some_and(|inputs| inputs.reusable_for(&before))
        && let Some(outcome) = receipt.coverage
    {
        println!("  reusing passed coverage measurement");
        return Ok(outcome);
    }
    let outcome = crate::cli::coverage_gate::run_coverage_gate(project_root, manifest)?;
    let current = load_effective()?;
    let command = serde_json::to_string(&current.coverage)?;
    let after = crate::proof::VerificationInputs::capture_archive(
        project_root,
        backend,
        change,
        &current,
        &command,
        "coverage",
    )?;
    before.ensure_unchanged(&after)?;
    crate::proof::Receipt::record_coverage(archive, outcome.clone(), before)?;
    Ok(outcome)
}

fn run_scope(
    project_root: &Path,
    command: &str,
    scope: &str,
    result_path: &Path,
) -> Result<(crate::proof::RunnerResult, f64)> {
    if let Some(parent) = result_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    crate::proof::remove_previous_output(result_path)?;
    let absolute = if result_path.is_absolute() {
        result_path.to_path_buf()
    } else {
        project_root.join(result_path)
    };
    let result_file = absolute.to_string_lossy().into_owned();

    println!(
        "[{}] Running {scope} verification: {command}",
        crate::subprocess::local_timestamp()
    );
    let started = std::time::Instant::now();
    let status = crate::subprocess::sh_streamed(
        command,
        &[
            ("GIVN_FEATURES", "givn/specs"),
            ("GIVN_RESULT_FILE", &result_file),
            ("GIVN_RESULT_SCOPE", scope),
        ],
        Some(project_root),
        &format!("{scope} verification"),
    )?;
    let seconds = started.elapsed().as_secs_f64();

    if !status.success() {
        bail!(
            "archive blocked: the {scope} verification failed (exit code {}). The archive \
             candidate is unproven; repair the committed state and rerun the archive.",
            status.code().unwrap_or(-1)
        );
    }
    let result = crate::proof::RunnerResult::read(&absolute, scope)?;
    Ok((result, seconds))
}

fn content_hash(content: &str) -> String {
    Sha256::digest(content.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn base_hash(content: &str) -> Option<&str> {
    content.lines().find_map(|line| {
        line.trim()
            .strip_prefix("<!-- givn:base-sha256:")
            .and_then(|value| value.strip_suffix("-->"))
            .map(str::trim)
    })
}

fn validate_usecase_documents(project_root: &std::path::Path, change_specs: &Path) -> Result<()> {
    let permanent_specs = project_root.join("givn/specs");
    let mut documents = Vec::new();
    collect_narrative_documents(change_specs, &mut documents)?;
    for document in documents {
        let relative = document.strip_prefix(change_specs)?;
        let target = narrative_target(&permanent_specs, relative)?;
        let content = std::fs::read_to_string(&document)?;
        if relative == std::path::Path::new("fragments/fragment.md") {
            crate::gherkin::usecase_md::parse_fragment_md(&content).map_err(|error| {
                anyhow::anyhow!(
                    "invalid fragment document '{}': {error}",
                    document.display()
                )
            })?;
        } else {
            let doc = crate::gherkin::usecase_md::parse_usecase_md(&content).map_err(|error| {
                anyhow::anyhow!(
                    "invalid use-case document '{}': {error}",
                    document.display()
                )
            })?;
            let expected = relative
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            if doc.id != expected {
                bail!(
                    "use-case document '{}' declares ID '{}' but its path owns '{}',",
                    document.display(),
                    doc.id,
                    expected
                );
            }
        }
        if target.exists() {
            let Some(expected_hash) = base_hash(&content) else {
                bail!(
                    "archive blocked: '{}' replaces an existing narrative without a base hash",
                    document.display()
                );
            };
            let actual_hash = content_hash(&std::fs::read_to_string(&target)?);
            if expected_hash != actual_hash {
                bail!(
                    "archive blocked: stale narrative replacement '{}' — expected base {}, current {}",
                    document.display(),
                    expected_hash,
                    actual_hash
                );
            }
        }
    }
    Ok(())
}

fn merge_usecase_documents(project_root: &std::path::Path, change_specs: &Path) -> Result<()> {
    let permanent_specs = project_root.join("givn/specs");
    let mut documents = Vec::new();
    collect_narrative_documents(change_specs, &mut documents)?;
    for document in documents {
        let relative = document.strip_prefix(change_specs)?;
        let target = narrative_target(&permanent_specs, relative)?;
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(target, std::fs::read_to_string(document)?)?;
    }
    Ok(())
}

/// Whether a use-case directory still holds at least one capability
/// `.feature` file. A directory whose last capability retired leaves with
/// its narrative; one that still serves a sibling capability stays untouched.
fn holds_feature_files(dir: &std::path::Path) -> Result<bool> {
    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("failed to scan use-case directory '{}'", dir.display()))?;
    for entry in entries {
        let path = entry
            .with_context(|| format!("failed to read use-case directory '{}'", dir.display()))?
            .path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("feature"))
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Handle one parsed delta feature: validate its capability tag, then
/// either retire the capability directory (`@givn.retired`) or merge the
/// delta into the permanent spec. Extracted from `run`'s merge loop so the
/// retirement path is testable without driving the full archive gate
/// pipeline (which needs a manifest, hooks, and a verify command).
#[cfg(test)]
fn process_delta_file(
    project_root: &std::path::Path,
    change_id: &str,
    delta_path: &std::path::Path,
    delta_feature: &gherkin::Feature,
) -> Result<()> {
    let cap = delta_capability(delta_path, delta_feature)?;

    let specs_root = project_root.join("givn/specs");

    // An existing capability lives in the use-case directory that already
    // contains it; a brand-new capability is placed by its proposal's
    // routing decision (the archive blocks when none is recorded).
    let existing_dir = crate::spec::owner::resolve_capability_owner(&specs_root, &cap)?;

    if crate::gherkin::delta::is_retired_feature(delta_feature) {
        if !delta_feature.scenarios.is_empty() {
            bail!(
                "archive blocked: '{}' is tagged @givn.retired but contains {} \
                 scenario(s) — a retirement delta must have zero scenarios",
                delta_path.display(),
                delta_feature.scenarios.len()
            );
        }
        match existing_dir {
            Some(usecase_dir) => {
                retire_capability(project_root, change_id, &cap, &usecase_dir)?;
            }
            None => bail!(
                "archive blocked: '{}' retires capability '{cap}' but no such capability exists in the active use-case tree",
                delta_path.display()
            ),
        }
        return Ok(());
    }

    let permanent_dir = match existing_dir {
        Some(dir) => dir,
        None => resolve_permanent_dir(
            project_root,
            &project_root.join("givn/changes").join(change_id),
            change_id,
            &cap,
        )?,
    };
    let permanent_path = permanent_dir.join(format!("{cap}.feature"));
    write_feature(&permanent_path, delta_feature)?;
    println!(
        "[{}]   merged → {}",
        crate::subprocess::local_timestamp(),
        permanent_path.display()
    );
    Ok(())
}

/// The declared use case for a new capability, taken from the change's
/// recorded routing decision. Returns `None` when the proposal is absent,
/// unparseable, has no row for the capability, or does not declare
/// `NEW in <usecase-id>` — the archive blocks in those cases.
/// Re-export the checker's finding labels for the archive gate report.
use crate::cli::readme::finding_label;

#[derive(serde::Serialize, serde::Deserialize)]
struct RetiredEntry {
    capability: String,
    retired_date: String,
    retired_by_change: String,
}

/// Remove a retiring capability, clean up an emptied use-case directory, and
/// record the tombstone once. Safe to retry after a partial retirement.
fn retire_capability(
    project_root: &std::path::Path,
    change_id: &str,
    cap: &str,
    usecase_dir: &Path,
) -> Result<()> {
    let cap_file = usecase_dir.join(format!("{cap}.feature"));
    if cap_file.exists() {
        std::fs::remove_file(&cap_file)
            .with_context(|| format!("failed to remove '{}'", cap_file.display()))?;
    }
    // Last-capability retirement: a use-case directory whose retiring
    // capability was its only `.feature` leaves with its narrative. The
    // `fragments` directory is exempt — it is the shared fragment home, not a
    // use case. Already-removed files are a retry, not an error.
    let is_fragments = usecase_dir
        .file_name()
        .is_some_and(|name| name == "fragments");
    if usecase_dir.exists() && !is_fragments && !holds_feature_files(usecase_dir)? {
        let narrative = usecase_dir.join("usecase.md");
        if narrative.exists() {
            std::fs::remove_file(&narrative).with_context(|| {
                format!(
                    "failed to remove the retired use-case narrative '{}'",
                    narrative.display()
                )
            })?;
        }
        let empty = std::fs::read_dir(usecase_dir)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);
        if empty {
            std::fs::remove_dir(usecase_dir).with_context(|| {
                format!(
                    "failed to remove the emptied use-case directory '{}'",
                    usecase_dir.display()
                )
            })?;
        }
    }
    record_retirement(project_root, cap, change_id)?;
    println!(
        "[{}]   retired → {}",
        crate::subprocess::local_timestamp(),
        cap_file.display()
    );
    Ok(())
}

/// Append a retirement record to `givn/specs/.retired.yaml`. The tombstone
/// is part of the merged candidate commit; a failed attempt is repaired
/// forward, never rolled back. The write is idempotent and atomic.
fn record_retirement(
    project_root: &std::path::Path,
    capability: &str,
    change_id: &str,
) -> Result<()> {
    let path = project_root.join("givn/specs/.retired.yaml");
    let mut entries: Vec<RetiredEntry> = if path.exists() {
        let text = std::fs::read_to_string(&path)?;
        serde_yaml_ng::from_str(&text).unwrap_or_default()
    } else {
        Vec::new()
    };
    if entries
        .iter()
        .any(|entry| entry.capability == capability && entry.retired_by_change == change_id)
    {
        return Ok(());
    }
    let date_output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output();
    let retired_date = date_output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    entries.push(RetiredEntry {
        capability: capability.to_string(),
        retired_date,
        retired_by_change: change_id.to_string(),
    });
    let yaml = serde_yaml_ng::to_string(&entries)?;
    let temporary = project_root.join("givn/specs/.retired.yaml.givn-tmp");
    std::fs::write(&temporary, yaml)?;
    std::fs::rename(&temporary, &path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A declaration whose only populated role is `limits`, loaded through the
    /// real strict loader.
    fn declaration_with_limits(
        entries: &[&str],
    ) -> crate::config::declaration::SpecificationDeclaration {
        let root = tempfile::TempDir::new().unwrap();
        let mut text = String::new();
        for role in ["behavior", "structure", "contracts", "e2e"] {
            text.push_str(&format!("{role}: []\n"));
        }
        text.push_str("limits:\n");
        for entry in entries {
            text.push_str(&format!("  - \"{entry}\"\n"));
        }
        text.push_str("source:\n  - \"src/**\"\n");
        std::fs::create_dir_all(root.path().join("givn")).unwrap();
        std::fs::write(root.path().join("givn/specification.yaml"), text).unwrap();
        crate::config::declaration::SpecificationDeclaration::load(root.path())
            .expect("the fixture declaration loads")
            .expect("the fixture declaration exists")
    }

    fn listing(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|path| (*path).to_string()).collect()
    }

    #[test]
    fn a_literal_under_the_change_tree_is_refused_with_its_archived_path() {
        let declaration = declaration_with_limits(&["givn/changes/moved-limits/design.md"]);
        let content = listing(&["givn/changes/moved-limits/design.md", "givn/config.yaml"]);
        let offenders = declared_retained_path_offenders(&declaration, "moved-limits", &content);
        assert_eq!(offenders.len(), 1);
        assert_eq!(offenders[0].role, "limits");
        assert_eq!(
            offenders[0].moved_path,
            "givn/changes/moved-limits/design.md"
        );
        assert_eq!(
            offenders[0].advice.as_deref(),
            Some("givn/archive/moved-limits/design.md")
        );
    }

    #[test]
    fn an_entry_matching_only_the_archived_counterpart_survives() {
        let declaration = declaration_with_limits(&["givn/*/moved-limits/design.md"]);
        let content = listing(&["givn/changes/moved-limits/design.md", "givn/config.yaml"]);
        assert!(
            declared_retained_path_offenders(&declaration, "moved-limits", &content).is_empty()
        );
    }

    #[test]
    fn a_wildcard_spanning_the_change_tree_is_refused_with_its_archived_pattern() {
        let declaration = declaration_with_limits(&["givn/changes/**"]);
        let content = listing(&["givn/changes/moved-limits/design.md", "givn/config.yaml"]);
        let offenders = declared_retained_path_offenders(&declaration, "moved-limits", &content);
        assert_eq!(offenders.len(), 1);
        assert_eq!(offenders[0].moved_path, "givn/changes/moved-limits");
        assert_eq!(offenders[0].advice.as_deref(), Some("givn/archive/**"));
    }

    #[test]
    fn a_broad_root_covering_surviving_content_is_accepted() {
        let declaration = declaration_with_limits(&["givn/**"]);
        let content = listing(&["givn/changes/moved-limits/design.md", "givn/config.yaml"]);
        assert!(
            declared_retained_path_offenders(&declaration, "moved-limits", &content).is_empty()
        );
    }

    #[test]
    fn a_literal_for_another_active_change_is_unaffected() {
        let declaration = declaration_with_limits(&["givn/changes/other-change/design.md"]);
        let content = listing(&[
            "givn/changes/moved-limits/design.md",
            "givn/changes/other-change/design.md",
        ]);
        assert!(
            declared_retained_path_offenders(&declaration, "moved-limits", &content).is_empty()
        );
    }

    #[test]
    fn an_entry_matching_a_content_path_outside_the_move_is_accepted() {
        let declaration = declaration_with_limits(&["givn/changes/**"]);
        let content = listing(&[
            "givn/changes/moved-limits/design.md",
            "givn/changes/other-active/design.md",
        ]);
        assert!(
            declared_retained_path_offenders(&declaration, "moved-limits", &content).is_empty()
        );
    }

    #[test]
    fn the_refusal_lists_twenty_offenders_and_counts_the_rest() {
        let entries: Vec<String> = (1..=21)
            .map(|index| format!("givn/changes/many-limits/limit-{index:02}.md"))
            .collect();
        let entry_refs: Vec<&str> = entries.iter().map(String::as_str).collect();
        let declaration = declaration_with_limits(&entry_refs);
        let content: Vec<String> = entries.clone();
        let offenders = declared_retained_path_offenders(&declaration, "many-limits", &content);
        assert_eq!(offenders.len(), 21);
        let refusal = retained_path_refusal(&offenders, "many-limits");
        assert_eq!(refusal.matches("matches '").count(), 20, "{refusal}");
        assert!(refusal.contains("limit-20.md"), "{refusal}");
        assert!(!refusal.contains("limit-21.md"), "{refusal}");
        assert!(refusal.contains("  +1 more"), "{refusal}");
    }

    #[test]
    fn scope_runner_receives_its_declared_environment() {
        let root = tempfile::TempDir::new().unwrap();
        let result = root.path().join("result.json");
        let command = "printf '{\"scope\":\"%s\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}' \"$GIVN_RESULT_SCOPE\" > \"$GIVN_RESULT_FILE\"";
        run_scope(root.path(), command, "regular", &result).unwrap();
    }

    use crate::gherkin::parse::parse_str;

    const PERMANENT_SPEC: &str = r#"
Feature: Dead capability

  Scenario: It does something
    Given a thing
    Then the thing happens
"#;

    /// A temp project whose `givn/specs/` holds one capability dir with a
    /// real scenario. Returns (root, specs_dir, backup_dir).
    fn temp_project() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        let cap_dir = project.join("givn/specs/dead-cap");
        std::fs::create_dir_all(&cap_dir).unwrap();
        std::fs::write(cap_dir.join("dead-cap.feature"), PERMANENT_SPEC).unwrap();
        std::fs::write(
            cap_dir.join("usecase.md"),
            crate::gherkin::usecase_md::usecase_md_template()
                .replace("<usecase-id>", "dead-cap")
                .replace("<capability-id>", "dead-cap")
                .replace("<scenario title>", "It does something"),
        )
        .unwrap();
        let backup_dir = project.join("givn/.archive-backup-test");
        (root, project, backup_dir)
    }

    /// Parse a delta feature from gherkin text.
    fn parse_delta(text: &str) -> gherkin::Feature {
        parse_str(text).unwrap()
    }

    #[test]
    fn receipt_target_outside_specs_is_refused() {
        let root = tempfile::TempDir::new().unwrap();
        let archive = root.path().join("givn/archive/demo");
        let specs = archive.join("specs/cap");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::write(
            specs.join("cap.feature"),
            "@givn.delta @cap\nFeature: Cap\n",
        )
        .unwrap();
        let receipt: crate::proof::Receipt = serde_json::from_value(serde_json::json!({
            "schema": 1,
            "change": "demo",
            "status": "pending",
            "merge_plan": ["cap/cap.feature"],
            "merge_targets": {"cap/cap.feature": "../outside/cap.feature"}
        }))
        .unwrap();
        let error = plan_from_receipt(root.path(), &archive, "demo", &receipt).unwrap_err();
        assert!(error.to_string().contains("outside givn/specs"), "{error}");
    }

    #[test]
    fn retirement_deletes_capability_and_writes_tombstone() {
        let (_root, project, _backup) = temp_project();
        let cap_dir = project.join("givn/specs/dead-cap");
        assert!(cap_dir.exists());

        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n");
        process_delta_file(&project, "retire-change", &delta_path, &delta).unwrap();

        assert!(
            !cap_dir.exists(),
            "the last capability's use-case directory must leave with its narrative"
        );
        assert!(
            !cap_dir.join("usecase.md").exists(),
            "the use-case narrative must leave with the directory"
        );
        assert!(
            !cap_dir.join("dead-cap.feature").exists(),
            "capability file should be deleted"
        );

        let tombstone = std::fs::read_to_string(project.join("givn/specs/.retired.yaml")).unwrap();
        assert!(tombstone.contains("dead-cap"), "tombstone: {tombstone}");
        assert!(
            tombstone.contains("retire-change"),
            "tombstone: {tombstone}"
        );
    }

    #[test]
    fn retirement_of_shared_usecase_keeps_directory_and_narrative() {
        let (_root, project, _backup) = temp_project();
        let cap_dir = project.join("givn/specs/dead-cap");
        std::fs::write(
            cap_dir.join("sibling-cap.feature"),
            "Feature: Sibling capability\n\n  Scenario: It stays\n    Given a thing\n",
        )
        .unwrap();

        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n");
        process_delta_file(&project, "retire-change", &delta_path, &delta).unwrap();

        assert!(
            !cap_dir.join("dead-cap.feature").exists(),
            "the retiring capability's file should be deleted"
        );
        assert!(
            cap_dir.join("sibling-cap.feature").exists(),
            "a remaining sibling capability's file stays"
        );
        assert!(
            cap_dir.join("usecase.md").exists(),
            "a use case that still serves a capability keeps its narrative"
        );
    }

    #[test]
    fn retirement_delta_with_scenarios_is_blocked() {
        let (_root, project, _backup) = temp_project();
        let delta_path = project.join("delta.feature");
        let delta = parse_delta(
            "@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n\n  Scenario: Not empty\n    Given x\n",
        );
        let err = process_delta_file(&project, "c", &delta_path, &delta).unwrap_err();
        assert!(err.to_string().contains("zero scenarios"), "{err}");
        assert!(project.join("givn/specs/dead-cap").exists());
    }

    #[test]
    fn retirement_of_missing_capability_is_blocked() {
        let (_root, project, _backup) = temp_project();
        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @never-existed\nFeature: Ghost\n");
        let err = process_delta_file(&project, "c", &delta_path, &delta).unwrap_err();
        assert!(
            err.to_string().contains("no such capability exists"),
            "{err}"
        );
    }

    #[test]
    fn unsafe_capability_tag_is_blocked() {
        let (_root, project, _backup) = temp_project();
        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @../../etc\nFeature: Traversal\n");
        let err = process_delta_file(&project, "c", &delta_path, &delta).unwrap_err();
        assert!(err.to_string().contains("invalid capability tag"), "{err}");
        assert!(project.join("givn/specs/dead-cap").exists());
    }

    #[test]
    fn non_retirement_delta_still_merges() {
        let (_root, project, _backup) = temp_project();
        let delta_path = project.join("delta.feature");
        let delta = parse_delta(
            "@givn.delta @dead-cap\nFeature: Dead capability\n\n  @givn.added\n  Scenario: Extra step\n    Given another thing\n    Then it works\n",
        );
        process_delta_file(&project, "c", &delta_path, &delta).unwrap();
        let merged =
            std::fs::read_to_string(project.join("givn/specs/dead-cap/dead-cap.feature")).unwrap();
        assert!(merged.contains("Extra step"), "{merged}");
    }

    #[test]
    fn merge_targets_existing_group_directory() {
        let (_root, project, _backup) = temp_project();
        // Migrate the capability into another use-case directory.
        let grouped = project.join("givn/specs/some-group");
        std::fs::create_dir_all(&grouped).unwrap();
        std::fs::rename(
            project.join("givn/specs/dead-cap/dead-cap.feature"),
            grouped.join("dead-cap.feature"),
        )
        .unwrap();
        std::fs::rename(
            project.join("givn/specs/dead-cap/usecase.md"),
            grouped.join("usecase.md"),
        )
        .unwrap();
        std::fs::remove_dir(project.join("givn/specs/dead-cap")).unwrap();

        let delta_path = project.join("delta.feature");
        let delta = parse_delta(
            "@givn.delta @dead-cap\nFeature: Dead capability\n\n  @givn.added\n  Scenario: Extra step\n    Given another thing\n    Then it works\n",
        );
        process_delta_file(&project, "c", &delta_path, &delta).unwrap();

        let merged = std::fs::read_to_string(grouped.join("dead-cap.feature")).unwrap();
        assert!(merged.contains("Extra step"), "{merged}");
        assert!(
            !project.join("givn/specs/dead-cap").exists(),
            "merge must not create a stray flat directory for a grouped capability"
        );
    }

    #[test]
    fn retirement_targets_grouped_capability_directory() {
        let (_root, project, _backup) = temp_project();
        let grouped = project.join("givn/specs/some-group");
        std::fs::create_dir_all(&grouped).unwrap();
        std::fs::rename(
            project.join("givn/specs/dead-cap/dead-cap.feature"),
            grouped.join("dead-cap.feature"),
        )
        .unwrap();
        std::fs::rename(
            project.join("givn/specs/dead-cap/usecase.md"),
            grouped.join("usecase.md"),
        )
        .unwrap();
        std::fs::remove_dir(project.join("givn/specs/dead-cap")).unwrap();

        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n");
        process_delta_file(&project, "c", &delta_path, &delta).unwrap();

        assert!(
            !grouped.join("dead-cap.feature").exists(),
            "grouped capability file must be deleted with its directory"
        );
        assert!(
            !project.join("givn/specs/dead-cap").exists(),
            "no flat directory may appear for a grouped capability"
        );
    }

    #[test]
    fn stale_temporaries_are_removed_before_the_witness() {
        let (_root, project, _backup) = temp_project();
        let change = project.join("givn/changes/demo");
        let archive = project.join("givn/archive/demo");
        std::fs::create_dir_all(&change).unwrap();
        std::fs::create_dir_all(&archive).unwrap();
        let specs_temp = project.join("givn/specs/dead-cap/dead-cap.feature.givn-tmp");
        std::fs::write(&specs_temp, "torn").unwrap();
        let change_temp = change.join(".verification.json.givn-tmp");
        std::fs::write(&change_temp, "torn").unwrap();
        let archive_temp = archive.join(".verification.json.givn-tmp");
        std::fs::write(&archive_temp, "torn").unwrap();

        cleanup_stale_temporaries(&project, &change, &archive).unwrap();

        assert!(!specs_temp.exists());
        assert!(!change_temp.exists());
        assert!(!archive_temp.exists());
    }

    #[test]
    fn retirement_retry_is_idempotent_and_cleans_up() {
        let (_root, project, _backup) = temp_project();
        let usecase_dir = project.join("givn/specs/dead-cap");
        retire_capability(&project, "retire-change", "dead-cap", &usecase_dir).unwrap();
        assert!(!usecase_dir.exists());
        let tombstone = std::fs::read_to_string(project.join("givn/specs/.retired.yaml")).unwrap();
        assert_eq!(
            tombstone.matches("capability: dead-cap").count(),
            1,
            "{tombstone}"
        );

        // A retry after the capability file and narrative already left is a no-op.
        retire_capability(&project, "retire-change", "dead-cap", &usecase_dir).unwrap();
        let tombstone = std::fs::read_to_string(project.join("givn/specs/.retired.yaml")).unwrap();
        assert_eq!(
            tombstone.matches("capability: dead-cap").count(),
            1,
            "{tombstone}"
        );
    }

    #[test]
    fn new_usecase_document_is_installed_atomically() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path();
        let change_specs = project.join("givn/changes/new-usecase/specs/project-run");
        std::fs::create_dir_all(&change_specs).unwrap();
        let document = crate::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", "project-run")
            .replace("<capability-id>", "run-project")
            .replace("<scenario title>", "Consumer runs a project");
        std::fs::write(change_specs.join("usecase.md"), document).unwrap();
        validate_usecase_documents(project, &project.join("givn/changes/new-usecase/specs"))
            .unwrap();
        merge_usecase_documents(project, &project.join("givn/changes/new-usecase/specs")).unwrap();
        assert!(project.join("givn/specs/project-run/usecase.md").exists());
    }

    fn write(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    fn phrases_in(text: &str) -> BTreeSet<String> {
        step_binding_expressions(text)
            .into_iter()
            .filter_map(binding_phrase)
            .collect()
    }

    #[test]
    fn step_binding_expressions_read_multiline_raw_and_regex_literals() {
        let text = r##"
#[given(regex = r#"^a givn project with a completed change "([^"]+)" ready to archive$"#)]
@when(
    "a multi-line "
    "phrase"  # a comment with ) and (
)
@then("plain phrase")
Given(/^a regex step$/, () => {})
Step("a godog step")
@Step("a java step")
"##;
        let phrases = phrases_in(text);
        assert!(
            phrases
                .contains(r#"^a givn project with a completed change "([^"]+)" ready to archive$"#),
            "{phrases:?}"
        );
        assert!(phrases.contains("a multi-line phrase"), "{phrases:?}");
        assert!(phrases.contains("plain phrase"), "{phrases:?}");
        assert!(phrases.contains("^a regex step$"), "{phrases:?}");
        assert!(phrases.contains("a godog step"), "{phrases:?}");
        assert!(phrases.contains("a java step"), "{phrases:?}");
    }

    #[test]
    fn step_definition_paths_follow_the_documented_rule() {
        assert!(is_step_definition_path(Path::new("tests/steps/foo.rs")));
        assert!(is_step_definition_path(Path::new(
            "givn/specs/run/steps/foo.py"
        )));
        assert!(is_step_definition_path(Path::new(
            "givn/specs/run/e2e_steps/foo.py"
        )));
        assert!(is_step_definition_path(Path::new(
            "givn/specs/run/foo_steps.py"
        )));
        assert!(!is_step_definition_path(Path::new("givn/specs/run/foo.py")));
        assert!(!is_step_definition_path(Path::new(
            "givn/specs/run/steps_helper/foo.py"
        )));
    }

    #[test]
    fn archive_copies_cannot_satisfy_their_own_delta_phrases() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path();
        let change = project.join("givn/changes/demo");
        let archive = project.join("givn/archive/demo");
        let other = project.join("givn/changes/other");
        write(
            &project.join("tests/steps/keep.rs"),
            "#[given(\"permanent phrase\")]\nfn keep() {}\n",
        );
        for directory in [&change, &archive, &other] {
            write(
                &directory.join("specs/steps/demo_steps.py"),
                "@given(\"delta phrase\")\ndef step():\n    pass\n",
            );
        }

        let error = ensure_step_definitions_merged(project, &change, StepGatePhase::Fresh)
            .unwrap_err()
            .to_string();
        assert!(error.contains("delta phrase"), "{error}");
        assert!(
            error.contains("givn/changes/demo/specs/steps/demo_steps.py"),
            "{error}"
        );
        assert!(!error.contains("permanent phrase"), "{error}");

        let error = ensure_step_definitions_merged(project, &archive, StepGatePhase::Resume)
            .unwrap_err()
            .to_string();
        assert!(error.contains("delta phrase"), "{error}");

        write(
            &project.join("tests/steps/demo_steps.py"),
            "@given(\"delta phrase\")\ndef step():\n    pass\n",
        );
        ensure_step_definitions_merged(project, &change, StepGatePhase::Fresh).unwrap();
        ensure_step_definitions_merged(project, &archive, StepGatePhase::Resume).unwrap();
    }

    #[test]
    fn refusal_lists_twenty_phrases_and_the_exact_remainder() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path();
        let change = project.join("givn/changes/demo");
        let mut text = String::new();
        for index in 1..=21 {
            text.push_str(&format!(
                "@given(\"catalog phrase {index:02}\")\ndef step_{index}():\n    pass\n"
            ));
        }
        write(&change.join("specs/steps/demo_steps.py"), &text);

        let error = ensure_step_definitions_merged(project, &change, StepGatePhase::Fresh)
            .unwrap_err()
            .to_string();
        assert!(error.contains("catalog phrase 01"), "{error}");
        assert!(error.contains("catalog phrase 20"), "{error}");
        assert!(!error.contains("catalog phrase 21"), "{error}");
        assert!(error.contains("+1 more"), "{error}");
    }

    #[test]
    fn unreadable_step_cache_files_are_pruned() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path();
        let change = project.join("givn/changes/demo");
        write(
            &project.join("tests/steps/keep.rs"),
            "#[given(\"delta phrase\")]\nfn keep() {}\n",
        );
        write(
            &change.join("specs/steps/__pycache__/demo.cpython-312.pyc"),
            "not utf-8",
        );
        write(
            &change.join("specs/steps/demo_steps.py"),
            "@given(\"delta phrase\")\ndef step():\n    pass\n",
        );
        ensure_step_definitions_merged(project, &change, StepGatePhase::Fresh).unwrap();
    }

    #[test]
    fn stale_usecase_replacement_is_rejected_before_write() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path();
        let permanent = project.join("givn/specs/project-run");
        std::fs::create_dir_all(&permanent).unwrap();
        let original = crate::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", "project-run")
            .replace("<capability-id>", "run-project")
            .replace("<scenario title>", "Consumer runs a project");
        std::fs::write(permanent.join("usecase.md"), &original).unwrap();
        let change_specs = project.join("givn/changes/replace-usecase/specs/project-run");
        std::fs::create_dir_all(&change_specs).unwrap();
        let replacement = format!(
            "<!-- givn:base-sha256:{} -->\n{}",
            content_hash("different base"),
            original
        );
        std::fs::write(change_specs.join("usecase.md"), replacement).unwrap();
        let error = validate_usecase_documents(
            project,
            &project.join("givn/changes/replace-usecase/specs"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("stale narrative replacement"));
        assert_eq!(
            std::fs::read_to_string(permanent.join("usecase.md")).unwrap(),
            original
        );
    }
}
