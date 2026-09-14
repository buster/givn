use crate::config::overlay::load_effective;
use crate::gherkin::merge::merge_file;
use crate::gherkin::parse::collect_feature_files;
use crate::proof::{ArchiveProof, SCOPE_E2E, SCOPE_REGULAR};
use crate::state::{ArtifactStatus, ChangeState};
use anyhow::{Context, Result, bail};
use gherkin::GherkinEnv;
use sha2::{Digest, Sha256};
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
/// 2. Fresh only: structural gates, merge, publish the candidate, and commit
///    it with a pending receipt.
/// 3. Run each verification scope once against the committed candidate and
///    validate its fail-closed runner result.
/// 4. Run the optional coverage gate.
/// 5. Guard the tested commit, write the proven receipt, and commit the
///    evidence-only finalization.
///
/// A failure after the candidate commit leaves the candidate unproven; the
/// developer repairs the committed state and reruns the command.
pub fn run(change: &str, verbose: bool) -> Result<()> {
    crate::subprocess::set_verbose(verbose);
    let project_root = std::env::current_dir()?;
    let manifest = load_effective()?;
    let change_dir = crate::state::resolve_change_dir(&project_root, change)?;
    let archive_target = project_root.join("givn/archive").join(change);
    let specs_dir = project_root.join("givn/specs");

    let backend = crate::scm::select(&project_root)?;

    let fresh = match crate::proof::read_status(&project_root, change) {
        ArchiveProof::Corrupt => bail!(
            "archive blocked: project state is corrupt — 'givn/changes/{change}' and \
             'givn/archive/{change}' both exist"
        ),
        ArchiveProof::Proven => bail!("archive blocked: '{change}' is already proven"),
        ArchiveProof::Legacy => bail!("change '{change}' is already archived"),
        ArchiveProof::Pending => {
            println!(
                "[{}] Resuming unproven archive candidate '{change}'...",
                crate::subprocess::local_timestamp()
            );
            false
        }
        ArchiveProof::None => {
            if !change_dir.exists() {
                bail!("change '{}' not found under givn/changes/", change);
            }
            true
        }
    };

    // Clean-state witness: evidence paths left by a failed attempt stay
    // resumable; anything else is uncommitted work.
    ensure_clean(&backend, &project_root, change, &manifest, Witness::Project)?;

    if fresh {
        println!(
            "[{}] Archiving change '{change}'...",
            crate::subprocess::local_timestamp()
        );

        // 1. Every apply.requires artifact must be Done (or Skipped).
        let state = ChangeState::from_manifest(&change_dir, &manifest);
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
                    change
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

        // 2. Configured archive hooks.
        let features_path = if crate::state::is_skipped("specs", &change_dir) {
            manifest.verify.features_path.clone()
        } else {
            change_dir.join("specs").to_string_lossy().into_owned()
        };
        println!(
            "[{}] Running archive gate hooks...",
            crate::subprocess::local_timestamp()
        );
        crate::hooks::run_archive_gate_hooks(&manifest, &features_path)?;
        println!(
            "[{}] ✓ Gate hooks passed",
            crate::subprocess::local_timestamp()
        );

        // 3. Run declaration gate, before the README gate.
        {
            let findings = crate::cli::run_block::check_run(&manifest);
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
        }

        // 4. README completeness gate.
        if manifest.addons.readme {
            let findings = crate::cli::readme::check_readme(&project_root);
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
        }

        // 5. Merge preconditions: use-case base revisions and delta parsing.
        validate_usecase_documents(&project_root, change)?;

        let delta_feature_dir = change_dir.join("specs");
        let mut parsed: Vec<(PathBuf, gherkin::Feature)> = Vec::new();
        if delta_feature_dir.exists() {
            for delta_path in collect_feature_files(&delta_feature_dir) {
                let env = GherkinEnv::default();
                let delta_feature =
                    gherkin::Feature::parse_path(&delta_path, env).map_err(|e| {
                        anyhow::anyhow!(
                            "failed to parse delta feature '{}': {}",
                            delta_path.display(),
                            e
                        )
                    })?;
                if let Some(wip_msg) = crate::gherkin::delta::find_wip_in_feature(&delta_feature) {
                    bail!(
                        "archive blocked: {} in '{}' — remove @wip from completed scenarios",
                        wip_msg,
                        delta_path.display(),
                    );
                }
                parsed.push((delta_path, delta_feature));
            }
            for (delta_path, delta_feature) in &parsed {
                if crate::gherkin::delta::is_retired_feature(delta_feature) {
                    continue;
                }
                let cap = delta_capability(delta_path, delta_feature)?;
                resolve_permanent_dir(&project_root, change, &cap)?;
            }
        }

        // 6. E2E runner configuration is a configuration gap, not a
        //    verification result: block before any mutation.
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&delta_feature_dir)
            || crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;

        // 7. Merge the accepted narratives and deltas into the permanent
        //    corpus, then publish the candidate.
        merge_usecase_documents(&project_root, change)?;
        if delta_feature_dir.exists() {
            println!("Merging {} delta feature file(s)...", parsed.len());
            for (delta_path, delta_feature) in &parsed {
                process_delta_file(&project_root, change, delta_path, delta_feature)?;
            }
        }

        std::fs::create_dir_all(&archive_target)?;
        copy_dir_all(&change_dir, &archive_target)?;
        crate::proof::Receipt::write_pending(&archive_target, change, crate::proof::unix_now())?;
        std::fs::remove_dir_all(&change_dir)?;

        let candidate_paths = vec![
            "givn/specs".to_string(),
            format!("givn/archive/{change}"),
            format!("givn/changes/{change}"),
        ];
        backend
            .publish(
                &candidate_paths,
                &format!("chore({change}): publish archive candidate"),
            )
            .with_context(|| {
                format!(
                    "archive blocked: the candidate revision was rejected. {}. The archive is \
                     not proven. Resolve the error and rerun `givn archive --change {change}`.",
                    rejection_state(&backend)
                )
            })?;
        println!(
            "[{}] ✓ Candidate committed",
            crate::subprocess::local_timestamp()
        );
    } else {
        // Resume: the merge is already committed. Re-check the e2e
        // precondition against the permanent corpus before re-running.
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;
    }

    // 8. Verification scopes run once against the committed candidate.
    let tested_revision = backend.revision()?;
    let started_at_unix = crate::proof::unix_now();
    let mut scopes = std::collections::BTreeMap::new();

    let regular_path = crate::proof::result_path(&archive_target, SCOPE_REGULAR);
    let (regular, regular_seconds) = run_scope(
        &project_root,
        &manifest.verify.command,
        SCOPE_REGULAR,
        &regular_path,
    )?;
    scopes.insert(
        SCOPE_REGULAR.to_string(),
        crate::proof::ScopeOutcome::passed(&regular, regular_seconds),
    );

    let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
    if merged_has_e2e {
        let e2e_path = crate::proof::result_path(&archive_target, SCOPE_E2E);
        let (e2e, e2e_seconds) = run_scope(
            &project_root,
            &manifest.verify.e2e_command,
            SCOPE_E2E,
            &e2e_path,
        )?;
        scopes.insert(
            SCOPE_E2E.to_string(),
            crate::proof::ScopeOutcome::passed(&e2e, e2e_seconds),
        );
    } else {
        scopes.insert(
            SCOPE_E2E.to_string(),
            crate::proof::ScopeOutcome::not_applicable(),
        );
    }

    // 9. Coverage runs once against the candidate, without rollback.
    let coverage = if manifest.addons.coverage {
        Some(crate::cli::coverage_gate::run_coverage_gate(
            &project_root,
            &manifest,
        )?)
    } else {
        None
    };

    // 10. Finalization: refresh managed blocks, guard the tested commit,
    //     then write and commit the proven receipt.
    if manifest.addons.readme {
        match crate::cli::readme::refresh_development_block(&project_root) {
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
        match crate::cli::readme::refresh_run_block(&project_root) {
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
    backend.publish(&evidence_paths, &format!("chore({change}): prove archive"))?;

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

    Ok(())
}

/// The backend-specific sentence describing a rejected candidate's
/// intermediate state.
fn rejection_state(backend: &crate::scm::Backend) -> &'static str {
    match backend {
        crate::scm::Backend::Git(_) => "The merged state is staged but not committed",
        crate::scm::Backend::Fossil(_) => "The merged state stays in the checkout uncommitted",
    }
}

/// Block unless the dirty worktree contains only evidence paths.
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
             configured in givn/config.yaml. An @e2e scenario without a configured \
             e2e runner is an unenforced gap, not an optional one — set verify.e2e_command.",
            specs_dir.display()
        );
    }
    Ok(())
}

/// Run one verification scope and validate its fail-closed result. Returns
/// the parsed result and the wall-clock seconds the scope took.
fn run_scope(
    project_root: &Path,
    command: &str,
    scope: &str,
    result_path: &Path,
) -> Result<(crate::proof::RunnerResult, f64)> {
    if let Some(parent) = result_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let _ = std::fs::remove_file(result_path);
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

fn collect_narrative_documents(
    dir: &std::path::Path,
    out: &mut Vec<std::path::PathBuf>,
) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_narrative_documents(&path, out)?;
        } else if path.file_name().is_some_and(|name| name == "usecase.md")
            || (path.file_name().is_some_and(|name| name == "fragment.md")
                && path.parent().and_then(|parent| parent.file_name())
                    == Some(std::ffi::OsStr::new("fragments")))
        {
            out.push(path);
        }
    }
    Ok(())
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

fn narrative_target(
    specs_root: &std::path::Path,
    relative: &std::path::Path,
) -> Result<std::path::PathBuf> {
    if relative == std::path::Path::new("fragments/fragment.md") {
        return Ok(specs_root.join(relative));
    }
    if relative
        .file_name()
        .is_some_and(|name| name == "usecase.md")
    {
        let Some(usecase) = relative.parent().and_then(|parent| parent.file_name()) else {
            bail!("usecase.md must be under specs/<usecase-id>/");
        };
        return Ok(specs_root.join(usecase).join("usecase.md"));
    }
    bail!(
        "unsupported narrative path '{}'; expected usecase.md or fragments/fragment.md",
        relative.display()
    )
}

fn validate_usecase_documents(project_root: &std::path::Path, change_id: &str) -> Result<()> {
    let change_specs = project_root
        .join("givn/changes")
        .join(change_id)
        .join("specs");
    let permanent_specs = project_root.join("givn/specs");
    let mut documents = Vec::new();
    collect_narrative_documents(&change_specs, &mut documents)?;
    for document in documents {
        let relative = document.strip_prefix(&change_specs)?;
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

fn merge_usecase_documents(project_root: &std::path::Path, change_id: &str) -> Result<()> {
    let change_specs = project_root
        .join("givn/changes")
        .join(change_id)
        .join("specs");
    let permanent_specs = project_root.join("givn/specs");
    let mut documents = Vec::new();
    collect_narrative_documents(&change_specs, &mut documents)?;
    for document in documents {
        let relative = document.strip_prefix(&change_specs)?;
        let target = narrative_target(&permanent_specs, relative)?;
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(target, std::fs::read_to_string(document)?)?;
    }
    Ok(())
}

/// Resolve which use-case directory an existing capability lives in, by
/// scanning `givn/specs/*/<cap>.feature`. Returns None if the capability
/// doesn't exist yet anywhere (a genuinely new capability — the caller
/// then decides placement; see the `## Capability Routing` section in
/// proposal.md, Phase 4).
fn resolve_capability_usecase(
    specs_root: &std::path::Path,
    cap: &str,
) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(specs_root).ok()?;
    for entry in entries.flatten() {
        let usecase_dir = entry.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        let candidate = usecase_dir.join(format!("{cap}.feature"));
        if candidate.exists() {
            return Some(usecase_dir);
        }
    }
    // Fragment capabilities live under givn/specs/fragments/ with
    // fragment.md as their narrative; retirements and merges must reach them.
    let fragments = specs_root.join("fragments");
    if fragments.join(format!("{cap}.feature")).exists() {
        return Some(fragments);
    }
    None
}

/// Handle one parsed delta feature: validate its capability tag, then
/// either retire the capability directory (`@givn.retired`) or merge the
/// delta into the permanent spec. Extracted from `run`'s merge loop so the
/// retirement path is testable without driving the full archive gate
/// pipeline (which needs a manifest, hooks, and a verify command).
fn delta_capability(
    delta_path: &std::path::Path,
    delta_feature: &gherkin::Feature,
) -> Result<String> {
    let cap = crate::gherkin::delta::capability_tag(delta_feature)
        .map(|t| t.to_string())
        .unwrap_or_else(|| {
            delta_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

    crate::gherkin::delta::validate_capability_tag(&cap)
        .map_err(|e| anyhow::anyhow!("archive blocked: {e} (from '{}')", delta_path.display()))?;
    Ok(cap)
}

/// Resolve the permanent directory a capability's spec lives in: the use-case
/// directory it already exists in, or — for a brand-new capability — the use
/// case its proposal's `NEW in <usecase-id>` routing decision declares.
fn resolve_permanent_dir(
    project_root: &std::path::Path,
    change_id: &str,
    cap: &str,
) -> Result<std::path::PathBuf> {
    let specs_root = project_root.join("givn/specs");
    if let Some(dir) = resolve_capability_usecase(&specs_root, cap) {
        return Ok(dir);
    }
    let usecase =
        routing_usecase_for_new_capability(project_root, change_id, cap).ok_or_else(|| {
            anyhow::anyhow!(
                "archive blocked: capability '{cap}' exists nowhere in the \
             permanent corpus and its proposal records no 'NEW in <usecase-id>' \
             routing decision"
            )
        })?;
    Ok(specs_root.join(usecase))
}

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
    let existing_dir = resolve_capability_usecase(&specs_root, &cap);

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
                let cap_file = usecase_dir.join(format!("{cap}.feature"));
                std::fs::remove_file(&cap_file)?;
                record_retirement(project_root, &cap, change_id)?;
                println!(
                    "[{}]   retired → {}",
                    crate::subprocess::local_timestamp(),
                    cap_file.display()
                );
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
        None => resolve_permanent_dir(project_root, change_id, &cap)?,
    };
    let permanent_path = permanent_dir.join(format!("{cap}.feature"));
    merge_file(&permanent_path, delta_feature)?;
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
fn routing_usecase_for_new_capability(
    project_root: &std::path::Path,
    change_id: &str,
    cap: &str,
) -> Option<String> {
    let proposal = std::fs::read_to_string(
        project_root
            .join("givn/changes")
            .join(change_id)
            .join("proposal.md"),
    )
    .ok()?;
    crate::spec::routing_table::parse_routing_table(&proposal)
        .into_iter()
        .find(|row| row.capability == cap)
        .and_then(|row| match row.decision {
            crate::spec::routing_table::Decision::NewIn(usecase) => Some(usecase),
            _ => None,
        })
}

/// Re-export the checker's finding labels for the archive gate report.
use crate::cli::readme::finding_label;

#[derive(serde::Serialize, serde::Deserialize)]
struct RetiredEntry {
    capability: String,
    retired_date: String,
    retired_by_change: String,
}

/// Append a retirement record to `givn/specs/.retired.yaml`. The tombstone
/// is part of the merged candidate commit; a failed attempt is repaired
/// forward, never rolled back.
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
    std::fs::write(&path, yaml)?;
    Ok(())
}

/// Recursively copy a directory tree.
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn retirement_deletes_capability_and_writes_tombstone() {
        let (_root, project, _backup) = temp_project();
        let cap_dir = project.join("givn/specs/dead-cap");
        assert!(cap_dir.exists());

        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n");
        process_delta_file(&project, "retire-change", &delta_path, &delta).unwrap();

        assert!(
            cap_dir.join("usecase.md").exists(),
            "use-case narrative remains"
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
        validate_usecase_documents(project, "new-usecase").unwrap();
        merge_usecase_documents(project, "new-usecase").unwrap();
        assert!(project.join("givn/specs/project-run/usecase.md").exists());
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
        let error = validate_usecase_documents(project, "replace-usecase").unwrap_err();
        assert!(error.to_string().contains("stale narrative replacement"));
        assert_eq!(
            std::fs::read_to_string(permanent.join("usecase.md")).unwrap(),
            original
        );
    }
}
