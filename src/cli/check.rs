use crate::config::overlay::load_effective;
use anyhow::{Result, bail};
use std::collections::HashMap;

/// Run an artifact's post hooks (e.g. verify.command, lint).
///
/// `givn check <artifact>` — runs that artifact's hooks.
/// `givn check` (no arg) — prints non-executing guidance.
pub fn run(artifact: Option<&str>, change: Option<&str>) -> Result<()> {
    if artifact.is_none() {
        println!(
            "No checks were executed. Select an explicit command:\n  givn check tasks --change <change> — executes changed-scenario verification\n  givn check review --change <change> — checks evidence without running verification suites\n  givn check <artifact> --change <change> — runs that artifact's configured hooks\n  givn check migration --change <change> — checks preservation\n  givn check rules — checks rule cleanliness"
        );
        return Ok(());
    }
    let project_root = std::env::current_dir()?;
    if artifact == Some("migration") {
        let migration_id = change.ok_or_else(|| {
            anyhow::anyhow!("givn check migration requires --change <migration-id>")
        })?;
        return crate::spec::coverage::check_migration(&project_root, migration_id);
    }
    if artifact == Some("rules") {
        let report = crate::skills::rules::cleanliness(&project_root)?;
        crate::skills::rules::print_report("check rules", &report)?;
        if report.policy == Some(crate::config::overlay::RulesPolicy::Versioned) {
            println!("  check rules: the rules are versioned by policy; nothing to purge");
            return Ok(());
        }
        if !report.is_clean() {
            bail!(
                "rules check failed: tracked or reachable-history rules exist; clean them before publishing"
            );
        }
        return Ok(());
    }
    let manifest = load_effective()?;

    // Determine features_path: active change's features, or default specs.
    let features_path = if let Some(id) = change {
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        change_dir.join("specs").to_string_lossy().into_owned()
    } else {
        manifest.verify.features_path.clone()
    };

    if let Some(artifact_id) = artifact {
        // Validate against manifest before running checks.
        if !manifest.artifacts.iter().any(|a| a.id == artifact_id) {
            let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
            bail!(
                "unknown artifact '{}'. Known artifacts: {}",
                artifact_id,
                known.join(", ")
            );
        }
        // Run mandatory checks first, then custom hooks. The mandatory-check
        // boundary is one `phase` entry, parented to this command's entry.
        let phase = crate::history::current().map(|recorder| {
            recorder
                .span("mandatory checks")
                .kind("phase")
                .label(artifact_id)
        });
        let checks = run_mandatory_checks(
            artifact_id,
            &manifest,
            &features_path,
            &project_root,
            change,
        );
        if let Some(phase) = phase {
            phase.mark(if checks.is_ok() { "ok" } else { "failed" });
        }
        checks?;
        let mut hooks_manifest = manifest.clone();
        if artifact_id == "tasks" || artifact_id == "review" {
            remove_verification_hooks(&mut hooks_manifest);
        }
        crate::hooks::run_artifact_hooks(artifact_id, &hooks_manifest, &features_path)
    } else {
        unreachable!()
    }
}

/// Run mandatory built-in checks for a given artifact.
///
/// These are not user-configurable post hooks — they are required gates
/// baked into the binary. For `tasks`: verify (test suite). For `review`:
/// integrity. Scenario execution is proven once per archive by the
/// verification receipt; review does not re-run the suite. All other
/// artifacts have no mandatory checks; their behavior comes from post hooks
/// only.
fn run_mandatory_checks(
    artifact_id: &str,
    manifest: &crate::config::manifest::Manifest,
    features_path: &str,
    project_root: &std::path::Path,
    change: Option<&str>,
) -> anyhow::Result<()> {
    match artifact_id {
        "tasks" => {
            println!("Mandatory checks for 'tasks':");
            // When running against delta specs (under givn/changes/<id>/specs/),
            // block if any non-removed scenario is still tagged @wip.
            if features_path.contains("givn/changes/") {
                let specs_path = std::path::Path::new(features_path);
                for path in crate::gherkin::parse::collect_feature_files(specs_path) {
                    let env = gherkin::GherkinEnv::default();
                    let feature = gherkin::Feature::parse_path(&path, env).map_err(|e| {
                        anyhow::anyhow!("failed to parse '{}': {}", path.display(), e)
                    })?;
                    if let Some(msg) = crate::gherkin::delta::find_wip_in_feature(&feature) {
                        anyhow::bail!(
                            "{} in '{}' — remove @wip before marking tasks complete",
                            msg,
                            path.display(),
                        );
                    }
                }
            }
            if let Some(id) = change {
                run_change_verification(manifest, project_root, id)?;
            } else {
                crate::hooks::run_verify_hook(manifest, features_path)?;
            }
            println!("  ✓ verify passed");
        }
        "review" => {
            println!("Mandatory checks for 'review':");
            let mut failures = Vec::new();
            match crate::integrity_hook::run_integrity_hook(
                features_path,
                manifest.addons.arc42,
                manifest.addons.coverage,
                manifest.addons.readme,
            ) {
                Ok(()) => println!("  ✓ integrity checks passed"),
                Err(error) => failures.push(error.to_string()),
            }
            let run_findings = crate::cli::run_block::check_run(manifest);
            if !run_findings.is_empty() {
                for f in &run_findings {
                    println!("  [{}] {}", crate::cli::run_block::label(f.kind), f.message);
                }
                failures.push(
                    "run declaration incomplete — configure `run:` in givn/commands.yaml"
                        .to_owned(),
                );
            }
            if run_findings.is_empty() {
                println!("  ✓ run declaration passed");
            }
            if let Some(id) = change {
                let lint = crate::cli::lint::run_scoped(project_root, id);
                let lint_ok = lint.is_ok();
                if let Err(error) = lint {
                    failures.push(error.to_string());
                }
                if lint_ok {
                    for result in [
                        crate::overlap::analysis::run_review_overlap_gate(project_root, id),
                        check_retirement_disclosed(project_root, id),
                        crate::spec::routing_table::check_routing_consistency(project_root, id),
                    ] {
                        if let Err(error) = result {
                            failures.push(error.to_string());
                        }
                    }
                } else {
                    println!(
                        "  capability checks requiring valid specification identity: not performed"
                    );
                }
                if let Err(error) = check_change_receipt(manifest, project_root, id) {
                    failures.push(error.to_string());
                }
                if let Err(error) = crate::config::lock::check(project_root, id) {
                    failures.push(error.to_string());
                }
                let change_dir = crate::state::resolve_change_dir(project_root, id)?;
                let backend = crate::scm::select(project_root)?;
                if let Err(error) = check_scenario_commits(&backend, &change_dir) {
                    failures.push(error.to_string());
                }
            } else {
                println!(
                    "  ⚠ no --change given — overlap, retirement, routing, and receipt checks were skipped \
                     (they require a specific change to scope to)"
                );
            }
            if !failures.is_empty() {
                bail!("review checks failed:\n  {}", failures.join("\n  "));
            }
        }
        _ => {}
    }
    Ok(())
}

struct ScenarioCommit {
    title: String,
    checked: bool,
    hash: Option<String>,
    batch: Option<String>,
}

fn scenario_title(line: &str) -> Option<(bool, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("- [") {
        return None;
    }
    let checked = trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]");
    let index = trimmed.find("Scenario:")?;
    let title = trimmed[index + "Scenario:".len()..]
        .trim()
        .trim_matches('*')
        .trim_matches('`')
        .trim();
    if title.is_empty() {
        return None;
    }
    Some((checked, title.to_string()))
}

fn commit_line(line: &str) -> Option<(bool, Option<String>, Option<String>)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("- [") || !trimmed.contains("COMMIT:") {
        return None;
    }
    let checked = trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]");
    let rest = trimmed
        .split_once("COMMIT:")
        .map(|(_, rest)| rest)
        .unwrap_or("");
    let (hash_part, label_part) = rest.split_once("--").unwrap_or((rest, ""));
    let hash = hash_part.trim().trim_matches('`');
    let batch = label_part
        .find("Batch:")
        .or_else(|| label_part.find("batch:"))
        .map(|index| label_part[index + "Batch:".len()..].trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.trim_matches('`').to_string());
    Some((checked, (!hash.is_empty()).then(|| hash.to_string()), batch))
}

fn parse_scenario_commits(content: &str) -> Vec<ScenarioCommit> {
    let mut scenarios: Vec<ScenarioCommit> = Vec::new();
    for line in content.lines() {
        if let Some((checked, title)) = scenario_title(line) {
            scenarios.push(ScenarioCommit {
                title,
                checked,
                hash: None,
                batch: None,
            });
        } else if let Some((checked, hash, batch)) = commit_line(line)
            && let Some(current) = scenarios.last_mut()
            && current.hash.is_none()
        {
            current.hash = hash;
            current.batch = batch;
            if checked {
                current.checked = true;
            }
        }
    }
    scenarios
}

fn normalize_commit_text(text: &str) -> String {
    text.replace(['`', '*'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn placeholder_hash(hash: &str) -> bool {
    let lower = hash.to_lowercase();
    lower == "___"
        || lower == "---"
        || lower == "xxx"
        || lower.chars().all(|c| c == 'x')
        || lower == "tbd"
        || lower == "todo"
        || lower.starts_with('<')
}

/// Block unless every checked scenario has a real commit whose message names
/// either the scenario or its declared batch. A change with no checked
/// scenario has not started implementation; it is not blocked here.
fn check_scenario_commits(
    backend: &crate::scm::Backend,
    change_dir: &std::path::Path,
) -> Result<()> {
    let tasks = change_dir.join("tasks.md");
    let Ok(content) = std::fs::read_to_string(&tasks) else {
        return Ok(());
    };
    let scenarios = parse_scenario_commits(&content);
    if scenarios.is_empty() || !scenarios.iter().any(|scenario| scenario.checked) {
        return Ok(());
    }
    let mut errors = Vec::new();
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for scenario in &scenarios {
        if let Some(hash) = scenario.hash.as_deref() {
            *counts.entry(hash).or_default() += 1;
        }
    }
    for scenario in &scenarios {
        if !scenario.checked {
            errors.push(format!(
                "scenario '{}' is not checked; a scenario is done only when its commit is recorded",
                scenario.title
            ));
            continue;
        }
        let Some(hash) = scenario.hash.as_deref() else {
            errors.push(format!(
                "scenario '{}' has no `COMMIT:` line",
                scenario.title
            ));
            continue;
        };
        if placeholder_hash(hash) {
            errors.push(format!(
                "scenario '{}' uses a placeholder revision `{hash}`",
                scenario.title
            ));
            continue;
        }
        if counts.get(hash).copied().unwrap_or(0) > 1 && scenario.batch.is_none() {
            errors.push(format!(
                "scenario '{}' shares revision `{hash}` with another scenario but has no `Batch:` label",
                scenario.title
            ));
            continue;
        }
        if !backend.commit_exists(hash)? {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` which does not exist",
                scenario.title
            ));
            continue;
        }
        if !backend.is_ancestor(hash)? {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` which is not reachable from HEAD",
                scenario.title
            ));
            continue;
        }
        let Some(message) = backend.commit_message(hash)? else {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` with no readable message",
                scenario.title
            ));
            continue;
        };
        let required = scenario.batch.as_deref().unwrap_or(&scenario.title);
        if !normalize_commit_text(&message).contains(&normalize_commit_text(required)) {
            let subject = if scenario.batch.is_some() {
                "does not name the batch"
            } else {
                "does not name the scenario"
            };
            errors.push(format!(
                "scenario '{}' commit `{hash}` {subject} in its message",
                scenario.title
            ));
        }
    }
    if !errors.is_empty() {
        bail!("scenario commit check failed:\n  {}", errors.join("\n  "));
    }
    println!(
        "  ✓ scenario commits passed ({} scenario(s))",
        scenarios.len()
    );
    Ok(())
}

/// The no-behavior-change route still checks identities that do not depend on
/// delta scenarios: the proposal names this change and every delta use-case
/// document names its own directory.
fn check_change_identity(change_dir: &std::path::Path) -> Result<()> {
    let id = change_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if let Ok(proposal) = std::fs::read_to_string(change_dir.join("proposal.md"))
        && let Some(title) = proposal
            .lines()
            .map(str::trim)
            .find_map(|line| line.strip_prefix("# Proposal:"))
    {
        let declared = title.trim();
        if !declared.is_empty() && declared != id && !declared.contains("{{CHANGE_ID}}") {
            anyhow::bail!(
                "change identity mismatch: proposal.md declares '{declared}' but the change directory is '{id}'"
            );
        }
    }
    let specs = change_dir.join("specs");
    let Ok(entries) = std::fs::read_dir(&specs) else {
        return Ok(());
    };
    let mut documents: Vec<std::path::PathBuf> = entries
        .flatten()
        .map(|entry| entry.path().join("usecase.md"))
        .filter(|path| path.is_file())
        .collect();
    documents.sort();
    for path in documents {
        let text = std::fs::read_to_string(&path)?;
        if let Ok(doc) = crate::gherkin::usecase_md::parse_usecase_md(&text) {
            let directory = path
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            if doc.id != directory {
                anyhow::bail!(
                    "change identity mismatch: use-case document '{}' declares '{}' but lives under '{directory}'",
                    path.display(),
                    doc.id
                );
            }
        }
    }
    Ok(())
}

/// Report a missing README-impact decision before review. The review integrity
/// gate remains the blocking authority.
fn preflight_readme_impact(change_dir: &std::path::Path) {
    let review = change_dir.join("review.md");
    match std::fs::read_to_string(&review) {
        Ok(content) if content.contains("README-IMPACT:") => {}
        Ok(_) => println!(
            "  ⚠ task preflight: review.md has no README-IMPACT decision (none or updated - <sections>)"
        ),
        Err(_) => println!("  ⚠ task preflight: review.md is missing"),
    }
}

/// Run the change-scoped verification and persist its receipt.
/// The clean-tree requirement keeps the receipt bound to one revision; the
/// agent commits implementation and evidence before this command runs.
fn run_change_verification(
    manifest: &crate::config::manifest::Manifest,
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    preflight_readme_impact(&change_dir);
    let not_applicable = crate::proof::change_verification_not_applicable(&change_dir)?;
    let backend = crate::scm::select(project_root)?;
    if not_applicable {
        println!(
            "  changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)"
        );
        check_change_identity(&change_dir)?;
        check_scenario_commits(&backend, &change_dir)?;
        return Ok(());
    }
    preflight_e2e_boundary(project_root, &change_dir, manifest)?;
    check_scenario_commits(&backend, &change_dir)?;
    let offending: Vec<String> = backend
        .dirty_entries()?
        .iter()
        .filter(|path| !crate::proof::is_evidence_path(project_root, change_id, manifest, path))
        .map(|path| path.display().to_string())
        .collect();
    if !offending.is_empty() {
        anyhow::bail!(
            "change verification requires a clean committed tree: {}",
            offending.join(", ")
        );
    }
    let rules = project_root
        .join("givn/changes")
        .join(change_id)
        .join("specs");
    let features = rules.to_string_lossy().into_owned();
    let result_file = change_dir.join(crate::proof::CHANGE_RESULT_FILE);
    if let Some(parent) = result_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let result_path = result_file.to_string_lossy().into_owned();
    crate::proof::remove_previous_output(&result_file)?;
    let before = crate::proof::VerificationInputs::capture(
        project_root,
        &backend,
        change_id,
        manifest,
        &rules,
        &manifest.verify.command,
        "change",
    )?;
    let status = crate::subprocess::sh_streamed(
        &manifest.verify.command,
        &[
            ("GIVN_FEATURES", &features),
            ("GIVN_RESULT_FILE", &result_path),
            ("GIVN_RESULT_SCOPE", "change"),
        ],
        Some(project_root),
        "change verification",
    )?;
    if !status.success() {
        anyhow::bail!(
            "change verification failed (exit code {}); repair the change and rerun \
             `givn check tasks --change {change_id}`",
            status.code().unwrap_or(-1)
        );
    }
    let result = crate::proof::RunnerResult::read(&result_file, "change")?;
    let current_manifest = load_effective()?;
    let after = crate::proof::VerificationInputs::capture(
        project_root,
        &backend,
        change_id,
        &current_manifest,
        &rules,
        &current_manifest.verify.command,
        "change",
    )?;
    before.ensure_unchanged(&after)?;
    let head = before.revision;
    let tested_content_digest = before.content_digest;
    crate::proof::write_change_receipt(
        project_root,
        &change_dir,
        change_id,
        &head,
        &manifest.verify.command,
        &tested_content_digest,
        result.clone(),
        crate::proof::unix_now(),
    )?;
    println!(
        "  ✓ change verification receipt: {} scenario(s), 0 failed, 0 skipped at {}",
        result.total, head
    );
    Ok(())
}

/// Refuse an `ise`-effective change whose declared `e2e` boundary cannot be
/// accepted, before any runner executes. A change whose locked add-on set has
/// `ise` off reads no declaration and gains no condition.
fn preflight_e2e_boundary(
    project_root: &std::path::Path,
    change_dir: &std::path::Path,
    manifest: &crate::config::manifest::Manifest,
) -> anyhow::Result<()> {
    if !crate::config::lock::ise_effective_for_change(change_dir)? {
        return Ok(());
    }
    let declaration = match crate::config::declaration::SpecificationDeclaration::load(project_root)
    {
        Ok(Some(declaration)) => declaration,
        Ok(None) => anyhow::bail!(
            "the declared e2e boundary cannot be resolved: '{}' is absent",
            crate::config::declaration::DECLARATION_PATH
        ),
        Err(error) => anyhow::bail!("the declared e2e boundary cannot be resolved: {error}"),
    };
    let boundary = crate::e2e_boundary::E2eBoundary::resolve(project_root, &declaration);
    let conditions = boundary.conditions(&manifest.verify.e2e_command);
    if conditions.is_empty() && boundary.roots().next().is_none() {
        anyhow::bail!(
            "the declared e2e boundary cannot be accepted: the 'e2e' role lists no path; the \
             change check requires a resolved E2E package"
        );
    }
    for condition in conditions
        .iter()
        .filter(|condition| !condition.acceptance())
    {
        println!("  ⚠ e2e boundary: {}", condition.render());
    }
    let refused = conditions
        .iter()
        .filter(|condition| condition.acceptance())
        .map(|condition| format!("  - {}", condition.render()))
        .collect::<Vec<_>>()
        .join("\n");
    if refused.is_empty() {
        return Ok(());
    }
    anyhow::bail!("the declared e2e boundary cannot be accepted:\n{refused}");
}

/// Refuse review unless a current, green change-scoped receipt exists.
fn check_change_receipt(
    manifest: &crate::config::manifest::Manifest,
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    if crate::proof::change_verification_not_applicable(&change_dir)? {
        println!(
            "  changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)"
        );
        check_change_identity(&change_dir)?;
        let backend = crate::scm::select(project_root)?;
        check_scenario_commits(&backend, &change_dir)?;
        return Ok(());
    }
    let receipt = crate::proof::ChangeReceipt::read(&change_dir).map_err(|error| {
        anyhow::anyhow!("{error}. Run `givn check tasks --change {change_id}` before review.")
    })?;
    let backend = crate::scm::select(project_root)?;
    check_scenario_commits(&backend, &change_dir)?;
    match receipt.freshness(manifest, &change_dir, &backend, project_root)? {
        crate::proof::ProofStatus::Current => {
            println!(
                "  ✓ change verification receipt passed ({} scenario(s) at {})",
                receipt.result.total, receipt.head
            );
        }
        crate::proof::ProofStatus::DocumentationOnly {
            tested_revision,
            current_revision,
        } => {
            println!(
                "  ✓ change verification receipt reused across documentation-only revision \
                 {current_revision} (tested at {tested_revision}, {} scenario(s))",
                receipt.result.total
            );
        }
        crate::proof::ProofStatus::Stale(reasons) => {
            let details = reasons
                .iter()
                .map(|reason| format!("  - {}", reason.render()))
                .collect::<Vec<_>>()
                .join("\n");
            anyhow::bail!(
                "change verification receipt is stale for '{change_id}':\n{details}\n\
                 Run `givn check tasks --change {change_id}` again."
            );
        }
    }
    Ok(())
}

/// If this change's delta retires any capability (`@givn.retired`),
/// require review.md to contain a `RETIRE: <capability>` line for each
/// one. A plain substring check, not a table parser — deliberately
/// simple so there is no heading-name mismatch to get wrong (see the
/// disposition-table parser in overlap/disposition.rs for what that
/// class of bug looks like).
pub(crate) fn check_retirement_disclosed(
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let specs_dir = change_dir.join("specs");
    let retired_caps: Vec<String> = crate::gherkin::parse::collect_feature_files(&specs_dir)
        .into_iter()
        .filter_map(|path| {
            let feature =
                gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default()).ok()?;
            if crate::gherkin::delta::is_retired_feature(&feature) {
                crate::gherkin::delta::capability_tag(&feature).map(|c| c.to_string())
            } else {
                None
            }
        })
        .collect();
    if retired_caps.is_empty() {
        return Ok(());
    }
    let review_path = change_dir.join("review.md");
    let review_text = std::fs::read_to_string(&review_path).unwrap_or_default();
    let missing: Vec<&String> = retired_caps
        .iter()
        .filter(|cap| !review_text.contains(&format!("RETIRE: {cap}")))
        .collect();
    if !missing.is_empty() {
        anyhow::bail!(
            "review.md is missing a 'RETIRE: <capability>' line for: {}. \
             Retiring a capability must be disclosed and justified in review.md.",
            missing
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

pub(crate) fn remove_verification_hooks(manifest: &mut crate::config::manifest::Manifest) {
    for artifact in &mut manifest.artifacts {
        artifact.post.retain(|hook| {
            let command = match hook {
                crate::config::manifest::HookEntry::Short(command) => command,
                crate::config::manifest::HookEntry::Full(hook) => &hook.run,
            };
            command != "verify" && command != "verify-e2e"
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Temp project with a change whose delta retires `dead-cap`.
    fn temp_change(review_text: &str) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        let change = project.join("givn/changes/retire-thing");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        std::fs::write(
            change.join("specs/retire-thing.feature"),
            "@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n",
        )
        .unwrap();
        std::fs::write(change.join("review.md"), review_text).unwrap();
        root
    }

    #[test]
    fn retirement_without_disclosure_fails() {
        let root = temp_change("# Review\n\nREVIEW: PASS\n");
        let project = root.path().to_path_buf();
        let err = check_retirement_disclosed(&project, "retire-thing").unwrap_err();
        assert!(err.to_string().contains("RETIRE: <capability>"), "{err}");
        assert!(err.to_string().contains("dead-cap"), "{err}");
    }

    #[test]
    fn disclosed_retirement_passes() {
        let root = temp_change("# Review\n\nRETIRE: dead-cap — obsolete engine\n");
        let project = root.path().to_path_buf();
        check_retirement_disclosed(&project, "retire-thing").unwrap();
    }

    #[test]
    fn no_retirement_delta_means_no_requirement() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        let change = project.join("givn/changes/plain-change");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        std::fs::write(
            change.join("specs/plain-change.feature"),
            "@givn.delta @some-cap\nFeature: Some capability\n\n  Scenario: One\n    Given x\n    Then y\n",
        )
        .unwrap();
        check_retirement_disclosed(&project, "plain-change").unwrap();
    }
    #[test]
    fn parses_two_scenarios_with_the_same_batch_revision() {
        let content = "- [x] Scenario: First\n  - [x] COMMIT: abc -- Batch: first-batch\n\
                       - [x] Scenario: Second\n  - [x] COMMIT: abc -- Batch: first-batch\n";
        let scenarios = parse_scenario_commits(content);
        assert_eq!(scenarios.len(), 2);
        assert_eq!(scenarios[0].hash.as_deref(), Some("abc"));
        assert_eq!(scenarios[1].hash.as_deref(), Some("abc"));
        assert_eq!(scenarios[0].batch.as_deref(), Some("first-batch"));
        assert_eq!(scenarios[1].batch.as_deref(), Some("first-batch"));
    }

    #[test]
    fn normalizes_commit_messages_for_scenario_matching() {
        let message = normalize_commit_text("feat: **Scenario with a title**");
        assert!(message.contains(&normalize_commit_text("Scenario with a title")));
        assert!(!message.contains(&normalize_commit_text("Different scenario")));
    }

    #[test]
    fn parses_a_scenario_with_its_own_revision() {
        let content = "- [x] Scenario: One scenario\n  - [x] COMMIT: abc -- One scenario\n";
        let scenarios = parse_scenario_commits(content);
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].title, "One scenario");
        assert!(scenarios[0].checked);
        assert_eq!(scenarios[0].hash.as_deref(), Some("abc"));
    }
}
