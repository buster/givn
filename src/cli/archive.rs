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
/// 6. Move change to givn/archive/<id>/.
///
/// All-or-nothing: if any step fails, abort with a clear error.
pub fn run(change: &str) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let change_dir = project_root.join("givn/changes").join(change);

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
            Some(a) if a.status == ArtifactStatus::Skipped => {
                // Intentionally skipped (e.g. reverse-engineering flow).
                // A required artifact should not normally be skipped; if it is,
                // the skip is intentional and the gate allows it.
            }
            Some(a) if a.status == ArtifactStatus::Pending => bail!(
                "archive blocked: required artifact '{}' does not exist for change '{}'",
                req,
                change
            ),
            Some(a) if a.status == ArtifactStatus::Partial => {
                // For tasks: incomplete checkboxes. For review: missing PASS.
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
    let features_path = change_dir.join("specs").to_string_lossy().into_owned();
    println!("Running archive gate hooks...");
    crate::hooks::run_archive_gate_hooks(&manifest, &features_path)?;
    println!("✓ Gate hooks passed");

    // Backup permanent specs before merge.
    let specs_dir = project_root.join("givn/specs");
    let backup_id = format!(".archive-backup-{change}");
    let backup_dir = project_root.join("givn").join(&backup_id);
    let backup_specs = backup_dir.join("specs");

    if specs_dir.exists() {
        std::fs::create_dir_all(&backup_specs)?;
        copy_dir_all(&specs_dir, &backup_specs)?;
        println!(
            "  backed up {} → {}",
            specs_dir.display(),
            backup_specs.display()
        );
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
        // Verify failed — print error, detect new capabilities, keep backup.
        eprintln!(
            "[archive] post-merge verify failed (exit code {}) against merged permanent specs at {}",
            verify_status.code().unwrap_or(-1),
            specs_dir.display()
        );
        eprintln!(
            "[archive] backup of pre-merge permanent specs is at {}",
            backup_specs.display()
        );

        // Detect newly-created permanent spec files (exist in specs_dir but not in backup).
        if backup_specs.exists() && specs_dir.exists() {
            let mut new_dirs = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&specs_dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let backup_path = backup_specs.join(&name);
                    if !backup_path.exists() {
                        new_dirs.push(format!("  givn/specs/{}", name.to_string_lossy()));
                    }
                }
            }
            if !new_dirs.is_empty() {
                eprintln!(
                    "[archive] the following permanent spec directories were newly created and must be deleted to restore the pre-archive state:"
                );
                for d in &new_dirs {
                    eprintln!("{d}");
                }
            }
        }

        bail!(
            "archive blocked: merged permanent specs failed verify. Pre-merge specs backed up at {}. \
             Fix the verify failure and re-run archive.",
            backup_specs.display()
        );
    }

    // Verify passed — clean up backup.
    println!("✓ Post-merge verify passed");

    // === Post-merge verify-e2e gate ===

    let e2e_command = &manifest.verify.e2e_command;
    if !e2e_command.is_empty() {
        let merged_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
        if merged_has_e2e {
            println!("Running post-merge verify-e2e: {e2e_command}");
            let e2e_status = std::process::Command::new("sh")
                .arg("-c")
                .arg(e2e_command)
                .env("GIVN_FEATURES", "givn/specs")
                .status()?;

            if !e2e_status.success() {
                eprintln!(
                    "[archive] post-merge verify-e2e failed (exit code {}) \
                     against merged permanent specs at {}",
                    e2e_status.code().unwrap_or(-1),
                    specs_dir.display()
                );
                eprintln!(
                    "[archive] backup of pre-merge permanent specs is at {}",
                    backup_specs.display()
                );
                bail!(
                    "archive blocked: merged permanent specs failed verify-e2e. \
                     Pre-merge specs backed up at {}. \
                     Fix the verify-e2e failure and re-run archive.",
                    backup_specs.display()
                );
            }
            println!("✓ Post-merge verify-e2e passed");
        }
    }

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
