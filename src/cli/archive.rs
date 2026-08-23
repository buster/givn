use crate::config::overlay::load_effective;
use crate::gherkin::merge::merge_file;
use crate::gherkin::parse::collect_feature_files;
use crate::state::{ArtifactStatus, ChangeState};
use anyhow::{Result, bail};
use gherkin::GherkinEnv;

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
pub fn run(change: &str) -> Result<()> {
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

    println!("Archiving change '{change}'...");

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
    println!("Running archive gate hooks...");
    crate::hooks::run_archive_gate_hooks(&manifest, &features_path)?;
    println!("✓ Gate hooks passed");

    // Re-run the strict semantic gate immediately before touching the
    // permanent tree. A REVIEW: PASS marker alone must not bypass it.
    run_similarity_gate(&project_root, change)?;
    println!("✓ Similarity gate passed");

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

    // === Merge delta features ===

    let delta_feature_dir = change_dir.join("specs");
    if delta_feature_dir.exists() {
        let delta_files = collect_feature_files(&delta_feature_dir);
        println!("Merging {} delta feature file(s)...", delta_files.len());

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

            let cap = crate::gherkin::delta::capability_tag(&delta_feature)
                .map(|t| t.to_string())
                .unwrap_or_else(|| {
                    delta_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string()
                });

            let permanent_dir = project_root.join("givn/specs").join(&cap);
            let permanent_path = permanent_dir.join(format!("{cap}.feature"));

            merge_file(&permanent_path, &delta_feature)?;
            println!("  merged → {}", permanent_path.display());
        }
    }

    // === Post-merge verify gate ===

    let verify_command = &manifest.verify.command;
    println!("Running post-merge verify: {verify_command}");
    let verify_status = std::process::Command::new("sh")
        .arg("-c")
        .arg(verify_command)
        .env("GIVN_FEATURES", "givn/specs")
        .status()?;

    if !verify_status.success() {
        rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
        bail!(
            "archive blocked: merged permanent specs failed verify (exit code {}). \
             Specs have been rolled back to their pre-archive state.",
            verify_status.code().unwrap_or(-1)
        );
    }
    println!("✓ Post-merge verify passed");

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
            println!("Running post-merge verify-e2e: {e2e_command}");
            let e2e_status = std::process::Command::new("sh")
                .arg("-c")
                .arg(e2e_command)
                .env("GIVN_FEATURES", "givn/specs")
                .status()?;

            if !e2e_status.success() {
                rollback_specs(&specs_dir, &backup_specs, &backup_dir)?;
                bail!(
                    "archive blocked: merged permanent specs failed verify-e2e (exit code {}). \
                     Specs have been rolled back to their pre-archive state.",
                    e2e_status.code().unwrap_or(-1)
                );
            }
            println!("✓ Post-merge verify-e2e passed");
        }
    }

    // === Coverage gate (archive-level, only if addon enabled) ===

    if manifest.addons.coverage {
        crate::cli::coverage_gate::run_coverage_gate(&project_root, &manifest, &backup_dir)?;
    }

    // Clean up backup (all gates passed).
    if backup_dir.exists() {
        std::fs::remove_dir_all(&backup_dir)?;
        println!("  removed backup: {}", backup_dir.display());
    }

    // === Move change to archive ===

    std::fs::create_dir_all(&archive_target)?;
    copy_dir_all(&change_dir, &archive_target)?;
    std::fs::remove_dir_all(&change_dir)?;

    println!("✓ Archived '{change}' to {}", archive_target.display());
    println!("\nChange archived successfully.");
    println!("The delta specs have been merged into givn/specs/.");

    refresh_similarity_index(&project_root);

    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn run_similarity_gate(project_root: &std::path::Path, change_id: &str) -> Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    if crate::state::is_skipped("specs", &change_dir) {
        return Ok(());
    }
    crate::spec::retrieval::review(project_root, change_id)
}

#[cfg(not(feature = "spec-retrieval"))]
fn run_similarity_gate(_project_root: &std::path::Path, _change_id: &str) -> Result<()> {
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn refresh_similarity_index(project_root: &std::path::Path) {
    if let Err(error) = crate::spec::retrieval::index(project_root) {
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
