use crate::config::overlay::load_effective;
use crate::gherkin::merge::{apply_deltas, merge_file, merge_file_retry};
use crate::gherkin::parse::{collect_feature_files, has_e2e_scenarios};
use crate::proof::{ArchiveProof, SCOPE_E2E, SCOPE_REGULAR};
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

    cleanup_stale_temporaries(&project_root, &change_dir, &archive_target)?;

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
        println!(
            "[{}] Archiving change '{change}'...",
            crate::subprocess::local_timestamp()
        );

        // 1. Every apply.requires artifact must be Done (or Skipped).
        ensure_artifact_gates(&change_dir, &manifest)?;

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
        ensure_run_declaration(&manifest)?;

        // 4. README completeness gate.
        if manifest.addons.readme {
            ensure_readme(&project_root)?;
        }

        // 5. Merge preconditions: use-case base revisions and delta parsing.
        let source_specs = change_dir.join("specs");
        validate_usecase_documents(&project_root, &source_specs)?;
        let plan = build_merge_plan(&project_root, &change_dir, change, &source_specs, true)?;

        // 6. E2E runner configuration is a configuration gap, not a
        //    verification result: block before any mutation.
        let merged_has_e2e = has_e2e_scenarios(&source_specs)
            || crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;

        // 7. Write the pending receipt into the active change, then move the
        //    whole directory with one rename. The merge runs against the
        //    archived candidate.
        let started_at_unix = crate::proof::unix_now();
        let (merge_plan, merge_targets) = receipt_plan(&project_root, &plan);
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

        // Re-check the e2e precondition against the permanent corpus before
        // re-running.
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        ensure_e2e_configured(&manifest, merged_has_e2e, &specs_dir)?;

        let mut merge_applied = false;
        if merge_needed {
            let plan = plan_from_receipt(&project_root, &archive_target, change, &receipt)?;
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
    backend.publish(
        &finalization_evidence_paths(&project_root, &manifest, change),
        &format!("chore({change}): prove archive"),
    )?;

    print_archived(change, &archive_target);
    Ok(())
}

struct MergeEntry {
    relative: String,
    feature: gherkin::Feature,
    target: PathBuf,
    retire: bool,
    cap: String,
}

fn receipt_plan(
    project_root: &Path,
    plan: &[MergeEntry],
) -> (Vec<String>, BTreeMap<String, String>) {
    let mut order = Vec::new();
    let mut targets = BTreeMap::new();
    for entry in plan {
        order.push(entry.relative.clone());
        let relative = entry
            .target
            .strip_prefix(project_root)
            .unwrap_or(&entry.target)
            .to_string_lossy()
            .replace('\\', "/");
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

fn build_merge_plan(
    project_root: &Path,
    change_root: &Path,
    change_id: &str,
    source_specs: &Path,
    validate: bool,
) -> Result<Vec<MergeEntry>> {
    let specs_root = project_root.join("givn/specs");
    let mut plan = Vec::new();
    let mut seen = BTreeSet::new();
    if !source_specs.exists() {
        return Ok(plan);
    }
    for delta_path in collect_feature_files(source_specs) {
        let env = GherkinEnv::default();
        let feature = gherkin::Feature::parse_path(&delta_path, env).map_err(|e| {
            anyhow::anyhow!(
                "failed to parse delta feature '{}': {}",
                delta_path.display(),
                e
            )
        })?;
        if validate && let Some(wip_msg) = crate::gherkin::delta::find_wip_in_feature(&feature) {
            bail!(
                "archive blocked: {} in '{}' — remove @wip from completed scenarios",
                wip_msg,
                delta_path.display(),
            );
        }
        let rel = delta_path
            .strip_prefix(source_specs)
            .unwrap_or(&delta_path)
            .to_string_lossy()
            .replace('\\', "/");
        let cap = delta_capability(&delta_path, &feature)?;
        if !seen.insert(cap.clone()) {
            bail!(
                "archive blocked: capability '{cap}' appears in more than one delta file; \
                 a change may merge each capability once"
            );
        }
        let retire = crate::gherkin::delta::is_retired_feature(&feature);
        if retire {
            if !feature.scenarios.is_empty() {
                bail!(
                    "archive blocked: '{}' is tagged @givn.retired but contains {} \
                     scenario(s) — a retirement delta must have zero scenarios",
                    delta_path.display(),
                    feature.scenarios.len()
                );
            }
            let Some(usecase_dir) = resolve_capability_usecase(&specs_root, &cap) else {
                if validate {
                    bail!(
                        "archive blocked: '{}' retires capability '{cap}' but no such \
                         capability exists in the active use-case tree",
                        delta_path.display()
                    );
                }
                bail!(
                    "archive blocked: the retirement target for '{cap}' is no longer \
                     present; rebuild the plan from the receipt"
                );
            };
            if validate && !usecase_dir.join(format!("{cap}.feature")).exists() {
                bail!(
                    "archive blocked: '{}' retires capability '{cap}' but '{}' does not exist",
                    delta_path.display(),
                    usecase_dir.join(format!("{cap}.feature")).display()
                );
            }
            plan.push(MergeEntry {
                relative: rel,
                feature,
                target: usecase_dir,
                retire: true,
                cap,
            });
            continue;
        }

        let permanent_dir = resolve_permanent_dir(project_root, change_root, change_id, &cap)?;
        let target = permanent_dir.join(format!("{cap}.feature"));
        if validate {
            let permanent_text = std::fs::read_to_string(&target).ok();
            let deltas = crate::gherkin::delta::delta_scenarios(&feature);
            apply_deltas(permanent_text.as_deref(), &deltas, &feature.name)?;
        }
        plan.push(MergeEntry {
            relative: rel,
            feature,
            target,
            retire: false,
            cap,
        });
    }
    Ok(plan)
}

fn plan_from_receipt(
    project_root: &Path,
    archive_target: &Path,
    change: &str,
    receipt: &crate::proof::Receipt,
) -> Result<Vec<MergeEntry>> {
    let source_specs = archive_target.join("specs");
    if receipt.merge_plan.is_empty() {
        return build_merge_plan(project_root, archive_target, change, &source_specs, false);
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
        let target = receipt
            .merge_targets
            .get(rel)
            .map(|value| project_root.join(value))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "archive blocked: the pending receipt has no merge target for '{rel}'"
                )
            })?;
        plan.push(MergeEntry {
            relative: rel.clone(),
            retire: crate::gherkin::delta::is_retired_feature(&feature),
            feature,
            target,
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
            merge_file_retry(&entry.target, &entry.feature)?;
        } else {
            merge_file(&entry.target, &entry.feature)?;
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
    let head = backend.revision()?;
    if receipt.tested_revision.as_deref() == Some(head.as_str()) {
        println!(
            "[{}] Finalization was not published; republishing the proven receipt...",
            crate::subprocess::local_timestamp()
        );
        backend.publish(
            &finalization_evidence_paths(project_root, manifest, change),
            &format!("chore({change}): prove archive"),
        )?;
        print_archived(change, archive_target);
        return Ok(true);
    }
    crate::proof::Receipt::write_pending_progress(
        archive_target,
        change,
        receipt
            .started_at_unix
            .unwrap_or_else(crate::proof::unix_now),
        receipt.merge_completed,
        receipt.merged_deltas,
        receipt.merge_in_progress,
        receipt.merge_plan,
        receipt.merge_targets,
    )?;
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
             configured in givn/commands.yaml. An @e2e scenario without a configured \
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

/// Resolve which use-case directory an existing capability lives in, by
/// scanning `givn/specs/*/<cap>.feature`. Returns None if the capability
/// doesn't exist yet anywhere (a genuinely new capability — the caller
/// then decides placement; see the `## Capability Routing` section in
/// proposal.md).
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
    change_root: &Path,
    change_id: &str,
    cap: &str,
) -> Result<std::path::PathBuf> {
    let specs_root = project_root.join("givn/specs");
    if let Some(dir) = resolve_capability_usecase(&specs_root, cap) {
        return Ok(dir);
    }
    let usecase = routing_usecase_for_new_capability(change_root, cap).ok_or_else(|| {
        anyhow::anyhow!(
            "archive blocked: capability '{cap}' exists nowhere in the \
             permanent corpus and its proposal records no 'NEW in <usecase-id>' \
             routing decision (change '{change_id}')"
        )
    })?;
    Ok(specs_root.join(usecase))
}

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
fn routing_usecase_for_new_capability(change_root: &Path, cap: &str) -> Option<String> {
    let proposal = std::fs::read_to_string(change_root.join("proposal.md")).ok()?;
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
