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
    let features_path = change_dir.join("specs").to_string_lossy().into_owned();
    println!("Running archive gate hooks...");
    crate::hooks::run_archive_gate_hooks(&manifest, &features_path)?;
    println!("✓ Gate hooks passed");

    // Backup permanent specs and README before merge/coverage.
    let specs_dir = project_root.join("givn/specs");
    let readme_path = project_root.join("README.md");
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
    if readme_path.exists() {
        std::fs::create_dir_all(&backup_dir)?;
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
        rollback_specs(&specs_dir, &backup_specs)?;
        bail!(
            "archive blocked: merged permanent specs failed verify (exit code {}). \
             Specs have been rolled back to their pre-archive state.",
            verify_status.code().unwrap_or(-1)
        );
    }
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
                rollback_specs(&specs_dir, &backup_specs)?;
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
        run_coverage_gate(&project_root, &manifest, &backup_dir)?;
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

    Ok(())
}

/// Run the archive-level coverage pipeline: run both coverage commands,
/// merge Cobertura XML, validate, update README fence.
///
/// Auto-rollback: if any step fails, specs and README are restored from backup.
fn run_coverage_gate(
    project_root: &std::path::Path,
    manifest: &crate::config::manifest::Manifest,
    backup_dir: &std::path::Path,
) -> Result<()> {
    let cov = &manifest.coverage;
    let project_root_str = project_root.to_string_lossy();

    println!("Running coverage gate...");

    // Validate commands are configured.
    let sentinel = "givn missing-coverage";
    if cov.non_e2e_command == sentinel || cov.e2e_command == sentinel {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: coverage commands are not configured. \
             Set coverage.non_e2e_command and coverage.e2e_command in givn/config.yaml. \
             Specs and README have been rolled back."
        );
    }

    // Run non-e2e coverage command.
    println!("  Running non-e2e coverage: {}", cov.non_e2e_command);
    let non_e2e_status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cov.non_e2e_command)
        .env("GIVN_FEATURES", &*project_root_str)
        .status()?;
    if !non_e2e_status.success() {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: non-e2e coverage command failed (exit code {}). \
             Specs and README have been rolled back.",
            non_e2e_status.code().unwrap_or(-1)
        );
    }

    // Run e2e coverage command.
    println!("  Running e2e coverage: {}", cov.e2e_command);
    let e2e_status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cov.e2e_command)
        .env("GIVN_FEATURES", &*project_root_str)
        .status()?;
    if !e2e_status.success() {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: e2e coverage command failed (exit code {}). \
             Specs and README have been rolled back.",
            e2e_status.code().unwrap_or(-1)
        );
    }

    // Parse Cobertura XML files.
    let non_e2e_path = project_root.join(&cov.non_e2e_path);
    let e2e_path = project_root.join(&cov.e2e_path);

    let non_e2e_report = match crate::cobertura::parse(&non_e2e_path) {
        Ok(r) => r,
        Err(e) => {
            let backup_specs = backup_dir.join("specs");
            rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
            rollback_readme(project_root, backup_dir)?;
            bail!(
                "archive blocked: failed to parse non-e2e Cobertura XML: {e}. \
                 Specs and README have been rolled back."
            );
        }
    };
    let e2e_report = match crate::cobertura::parse(&e2e_path) {
        Ok(r) => r,
        Err(e) => {
            let backup_specs = backup_dir.join("specs");
            rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
            rollback_readme(project_root, backup_dir)?;
            bail!(
                "archive blocked: failed to parse e2e Cobertura XML: {e}. \
                 Specs and README have been rolled back."
            );
        }
    };

    // Validate individual reports.
    if let Err(e) = crate::cobertura::validate(&non_e2e_report) {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: non-e2e coverage validation failed: {e}. \
             Specs and README have been rolled back."
        );
    }
    if let Err(e) = crate::cobertura::validate(&e2e_report) {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: e2e coverage validation failed: {e}. \
             Specs and README have been rolled back."
        );
    }

    // Ensure the two reports are structurally distinct (not the same coverage
    // data copied to both paths). E2e coverage must come from an instrumented
    // server run — duplicating the non-e2e output is a configuration error.
    if let Err(e) = crate::cobertura::assert_distinct(&non_e2e_report, &e2e_report) {
        let backup_specs = backup_dir.join("specs");
        rollback_specs(&project_root.join("givn/specs"), &backup_specs)?;
        rollback_readme(project_root, backup_dir)?;
        bail!(
            "archive blocked: {e} \
             Specs and README have been rolled back."
        );
    }

    // Merge reports.
    let merged = crate::cobertura::merge(&non_e2e_report, &e2e_report);

    // Write merged Cobertura XML.
    let output_path = project_root.join(&cov.output_path);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let merged_xml = format!(
        r#"<?xml version="1.0"?>
<coverage line-rate="{lr}" branch-rate="{br}" lines-covered="{lc}" lines-valid="{lv}" branches-covered="{bc}" branches-valid="{bv}" timestamp="{ts}" version="1">
  <packages />
</coverage>"#,
        lr = merged.line_rate,
        br = merged.branch_rate,
        lc = merged.lines_covered,
        lv = merged.lines_valid,
        bc = merged.branches_covered,
        bv = merged.branches_valid,
        ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    );
    std::fs::write(&output_path, merged_xml)?;
    println!("  wrote merged Cobertura → {}", output_path.display());

    // Update README with coverage fence.
    let readme_path = project_root.join("README.md");
    let summary = crate::cobertura::render_markdown(&merged);
    crate::cli::agents_block::apply(&readme_path, "coverage", &summary)?;
    println!("  updated README.md with coverage summary");

    println!("✓ Coverage gate passed");
    Ok(())
}

/// Roll back permanent specs from backup (if it exists).
fn rollback_specs(specs_dir: &std::path::Path, backup_specs: &std::path::Path) -> Result<()> {
    if backup_specs.exists() {
        if specs_dir.exists() {
            std::fs::remove_dir_all(specs_dir)?;
        }
        copy_dir_all(backup_specs, specs_dir)?;
        eprintln!("[archive] rolled back specs from backup");
    }
    Ok(())
}

/// Roll back README.md from backup (if it exists).
fn rollback_readme(project_root: &std::path::Path, backup_dir: &std::path::Path) -> Result<()> {
    let backup_readme = backup_dir.join("README.md");
    let readme_path = project_root.join("README.md");
    if backup_readme.exists() {
        if readme_path.exists() {
            std::fs::remove_file(&readme_path)?;
        }
        std::fs::copy(&backup_readme, &readme_path)?;
        eprintln!("[archive] rolled back README.md from backup");
    } else if readme_path.exists() {
        // No backup means README was created during this archive — remove the
        // coverage block we just wrote (it was the last thing we did).
        let current = std::fs::read_to_string(&readme_path).unwrap_or_default();
        let updated = crate::cli::agents_block::remove_block(&current, "coverage");
        if updated.trim().is_empty() {
            std::fs::remove_file(&readme_path)?;
        } else {
            std::fs::write(&readme_path, updated)?;
        }
    }
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
