use crate::config::overlay::load_effective;
use crate::gherkin::merge::merge_file;
use crate::gherkin::parse::collect_feature_files;
use crate::state::{ArtifactStatus, ChangeState};
use anyhow::{Result, bail};
use gherkin::GherkinEnv;
use sha2::{Digest, Sha256};

/// Archive a completed change.
///
/// Gate (ADR-028, arc42 ch.8.6):
/// 1. Check apply.requires: all required artifacts must exist.
/// 2. Check tasks complete (all [x]).
/// 3. Check review PASS (if review is in apply.requires).
/// 4. Run hooks of every apply.requires artifact.
/// 5. Merge delta .feature files into givn/specs/.
/// 6. Run coverage gate (if addons.coverage: true) — measures full-project
///    coverage, updates README with a fenced coverage block. Auto-rollback
///    if any step in the coverage pipeline fails.
/// 7. Move change to givn/archive/<id>/.
///
/// All-or-nothing: if any step fails, abort and roll back.
pub fn run(change: &str, verbose: bool) -> Result<()> {
    crate::subprocess::set_verbose(verbose);
    let project_root = std::env::current_dir()?;
    let change_dir = crate::state::resolve_change_dir(&project_root, change)?;

    if !change_dir.exists() {
        bail!("change '{}' not found under givn/changes/", change);
    }

    // Check not already archived.
    let archive_target = project_root.join("givn/archive").join(change);
    if archive_target.exists() {
        bail!("change '{}' is already archived", change);
    }

    let manifest = load_effective()?;
    let state = ChangeState::from_manifest(&change_dir, &manifest);

    println!(
        "[{}] Archiving change '{change}'...",
        crate::subprocess::local_timestamp()
    );

    // === Gate checks (manifest-aware) ===

    // 1. Check all apply.requires artifacts are Done (or Skipped).
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

    // 4. Run hooks of all apply.requires artifacts.
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

    // Re-run the strict semantic gate immediately before touching the
    // permanent tree. A REVIEW: PASS marker alone must not bypass it.
    run_similarity_gate(&project_root, change)?;
    println!(
        "[{}] ✓ Similarity gate passed",
        crate::subprocess::local_timestamp()
    );

    // Run declaration gate: the project must declare a complete, configured
    // run block in givn/commands.yaml. Runs before the README gate so the
    // finding order matches `givn check review`.
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

    // README completeness gate (readme addon enabled): no open placeholders,
    // no remnants, no managed-block damage, an existing managed README. Runs
    // before any mutation, so a blocked archive needs no rollback.
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
                "archive blocked: finish the README before archiving — see the README check report above"
            );
        }
        println!(
            "[{}] ✓ README completeness gate passed",
            crate::subprocess::local_timestamp()
        );
    }

    // Backup permanent specs and README before merge/coverage.
    // Always create a backup marker even if specs_dir doesn't exist yet,
    // so rollback can distinguish "nothing to restore" from "merge created
    // specs_dir and then failed".
    let specs_dir = project_root.join("givn/specs");
    let readme_path = project_root.join("README.md");
    let backup_id = format!(".archive-backup-{change}");
    let backup_dir = project_root.join("givn").join(&backup_id);
    let backup_specs = backup_dir.join("specs");
    let backup_marker = backup_dir.join(".backup-state");
    std::fs::create_dir_all(&backup_dir)?;

    if specs_dir.exists() {
        std::fs::create_dir_all(&backup_specs)?;
        copy_dir_all(&specs_dir, &backup_specs)?;
        std::fs::write(&backup_marker, "present")?;
        println!(
            "  backed up {} → {}",
            specs_dir.display(),
            backup_specs.display()
        );
    } else {
        std::fs::write(&backup_marker, "absent")?;
    }
    if readme_path.exists() {
        let backup_readme = backup_dir.join("README.md");
        std::fs::copy(&readme_path, &backup_readme)?;
    }

    // Validate complete use-case replacements against their base revisions
    // before changing either narratives or feature files.
    validate_usecase_documents(&project_root, change)?;
    merge_usecase_documents(&project_root, change)?;

    // === Merge delta features ===

    let delta_feature_dir = change_dir.join("specs");
    if delta_feature_dir.exists() {
        let delta_files = collect_feature_files(&delta_feature_dir);
        println!("Merging {} delta feature file(s)...", delta_files.len());

        // Prevalidation pass: parse, WIP-check, and resolve every merge
        // target before writing anything, so a blocked archive leaves no
        // partially merged permanent specs behind.
        let mut parsed: Vec<(&std::path::Path, gherkin::Feature)> = Vec::new();
        for delta_path in &delta_files {
            let env = GherkinEnv::default();
            let delta_feature = gherkin::Feature::parse_path(delta_path, env).map_err(|e| {
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
        for (delta_path, delta_feature) in &parsed {
            process_delta_file(&project_root, change, delta_path, delta_feature)?;
        }
    }

    // === Post-merge verify gate ===

    let verify_command = &manifest.verify.command;
    println!(
        "[{}] Running post-merge verify: {verify_command}",
        crate::subprocess::local_timestamp()
    );
    let verify_status = crate::subprocess::sh_streamed(
        verify_command,
        &[("GIVN_FEATURES", "givn/specs")],
        None,
        "post-merge verify",
    )?;

    if !verify_status.success() {
        rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
        bail!(
            "archive blocked: merged permanent specs failed verify (exit code {}). \
             Specs have been rolled back to their pre-archive state.",
            verify_status.code().unwrap_or(-1)
        );
    }
    println!(
        "[{}] ✓ Post-merge verify passed",
        crate::subprocess::local_timestamp()
    );

    // === Post-merge verify-e2e gate ===

    let e2e_command = &manifest.verify.e2e_command;
    if e2e_command.is_empty() || e2e_command == "givn missing-e2e-runner" {
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        if merged_has_e2e {
            rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
            bail!(
                "'{}' contains @e2e-tagged scenario(s) but verify.e2e_command is not \
                 configured in givn/config.yaml. An @e2e scenario without a configured \
                 e2e runner is an unenforced gap, not an optional one — set verify.e2e_command.",
                specs_dir.display()
            );
        }
    } else {
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        if merged_has_e2e {
            println!(
                "[{}] Running post-merge verify-e2e: {e2e_command}",
                crate::subprocess::local_timestamp()
            );
            let e2e_status = crate::subprocess::sh_streamed(
                e2e_command,
                &[("GIVN_FEATURES", "givn/specs")],
                None,
                "post-merge verify-e2e",
            )?;

            if !e2e_status.success() {
                rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
                bail!(
                    "archive blocked: merged permanent specs failed verify-e2e (exit code {}). \
                     Specs have been rolled back to their pre-archive state.",
                    e2e_status.code().unwrap_or(-1)
                );
            }
            println!(
                "[{}] ✓ Post-merge verify-e2e passed",
                crate::subprocess::local_timestamp()
            );
        }
    }

    // Development managed block refresh (readme addon enabled): the block
    // content is rewritten from the effective command file after the
    // post-merge gates and before the coverage gate. A failure restores the
    // README from the pre-archive backup.
    if manifest.addons.readme {
        match crate::cli::readme::refresh_development_block(&project_root) {
            Ok(true) => println!(
                "[{}] ✓ development managed block refreshed",
                crate::subprocess::local_timestamp()
            ),
            Ok(false) => {}
            Err(e) => {
                let _ = crate::cli::coverage_gate::rollback_readme(&project_root, &backup_dir);
                bail!(
                    "archive blocked: development managed block refresh failed: {e:#}. \
                     README has been restored to its pre-archive state."
                );
            }
        }
        match crate::cli::readme::refresh_run_block(&project_root) {
            Ok(true) => println!(
                "[{}] ✓ run managed block refreshed",
                crate::subprocess::local_timestamp()
            ),
            Ok(false) => {}
            Err(e) => {
                let _ = crate::cli::coverage_gate::rollback_readme(&project_root, &backup_dir);
                bail!(
                    "archive blocked: run managed block refresh failed: {e:#}. \
                     README has been restored to its pre-archive state."
                );
            }
        }
    }

    // === Coverage gate (archive-level, only if addon enabled) ===

    if manifest.addons.coverage {
        crate::cli::coverage_gate::run_coverage_gate(&project_root, &manifest, &backup_dir)?;
    }

    // Clean up backup (all gates passed).
    if backup_dir.exists() {
        std::fs::remove_dir_all(&backup_dir)?;
        println!(
            "[{}]   removed backup: {}",
            crate::subprocess::local_timestamp(),
            backup_dir.display()
        );
    }

    // === Move change to archive ===

    std::fs::create_dir_all(&archive_target)?;
    copy_dir_all(&change_dir, &archive_target)?;
    std::fs::remove_dir_all(&change_dir)?;

    println!(
        "[{}] ✓ Archived '{change}' to {}",
        crate::subprocess::local_timestamp(),
        archive_target.display()
    );
    println!("\nChange archived successfully.");
    println!("The delta specs have been merged into givn/specs/.");

    refresh_similarity_index(&project_root);

    Ok(())
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

#[cfg(feature = "spec-retrieval")]
fn run_similarity_gate(project_root: &std::path::Path, change_id: &str) -> Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    if crate::state::is_skipped("specs", &change_dir) {
        return Ok(());
    }
    crate::spec::retrieval::semantic_review_archive_gate(project_root, change_id)
}
#[cfg(not(feature = "spec-retrieval"))]
fn run_similarity_gate(_project_root: &std::path::Path, _change_id: &str) -> Result<()> {
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn refresh_similarity_index(project_root: &std::path::Path) {
    if let Err(error) = crate::spec::retrieval::index_without_download(project_root) {
        eprintln!("warning: retrieval index refresh after archive failed: {error:#}");
    }
}

#[cfg(not(feature = "spec-retrieval"))]
fn refresh_similarity_index(_project_root: &std::path::Path) {}

/// Roll back permanent specs from backup (if it exists).
/// Uses a copy-to-temp-then-rename approach so that if the copy fails,
/// the original specs_dir is never left deleted.
pub(crate) fn rollback_specs(
    specs_dir: &std::path::Path,
    backup_specs: &std::path::Path,
    backup_dir: &std::path::Path,
) -> Result<()> {
    let marker = backup_dir.join(".backup-state");
    let was_present =
        marker.exists() && std::fs::read_to_string(&marker).ok().as_deref() == Some("present");

    if !was_present {
        // Specs did not exist pre-archive — remove whatever merge left behind.
        if specs_dir.exists() {
            std::fs::remove_dir_all(specs_dir)?;
        }
        return Ok(());
    }

    if !backup_specs.exists() {
        bail!(
            "backup specs directory '{}' is missing (was present pre-archive)",
            backup_specs.display()
        );
    }

    // Copy backup to a temp location first, then swap.
    let parent = specs_dir.parent().unwrap();
    let tmp = parent.join(format!(".rollback-tmp-{}", std::process::id()));
    if tmp.exists() {
        std::fs::remove_dir_all(&tmp)?;
    }
    copy_dir_all(backup_specs, &tmp)?;

    if specs_dir.exists() {
        std::fs::remove_dir_all(specs_dir)?;
    }
    std::fs::rename(&tmp, specs_dir)?;

    eprintln!("[archive] rolled back specs from backup");
    Ok(())
}

/// Re-export the checker's finding labels for the archive gate report.
use crate::cli::readme::finding_label;

#[derive(serde::Serialize, serde::Deserialize)]
struct RetiredEntry {
    capability: String,
    retired_date: String,
    retired_by_change: String,
}

/// Append a retirement record to `givn/specs/.retired.yaml`. This file
/// lives *inside* `givn/specs`, so it is covered by the same
/// backup/rollback as everything else the merge loop touches — a failed
/// archive after this call rolls the tombstone back along with the
/// deleted directory.
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
    fn rollback_restores_retired_capability_and_clears_tombstone() {
        let (_root, project, backup_dir) = temp_project();
        let specs_dir = project.join("givn/specs");
        let backup_specs = backup_dir.join("specs");
        std::fs::create_dir_all(&backup_specs).unwrap();
        copy_dir_all(&specs_dir, &backup_specs).unwrap();
        std::fs::write(backup_dir.join(".backup-state"), "present").unwrap();

        // Retire dead-cap (simulating the merge loop running to completion).
        let delta_path = project.join("delta.feature");
        let delta = parse_delta("@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n");
        process_delta_file(&project, "retire-change", &delta_path, &delta).unwrap();
        assert!(
            !project
                .join("givn/specs/dead-cap/dead-cap.feature")
                .exists()
        );
        assert!(specs_dir.join(".retired.yaml").exists());

        // Post-merge verify fails → rollback. The backup was taken before
        // the merge loop, so it still holds dead-cap and no tombstone.
        rollback_specs(&specs_dir, &backup_specs, &backup_dir).unwrap();

        assert!(
            project
                .join("givn/specs/dead-cap/dead-cap.feature")
                .exists(),
            "rollback must restore the retired capability"
        );
        assert!(
            !specs_dir.join(".retired.yaml").exists(),
            "rollback must clear the tombstone written after the backup"
        );
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
