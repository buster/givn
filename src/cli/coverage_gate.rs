use anyhow::{Result, bail};
use std::path::Path;

/// Run the archive-level coverage pipeline: run both coverage commands,
/// merge Cobertura XML, validate, update README fence.
pub(crate) fn run_coverage_gate(
    project_root: &std::path::Path,
    manifest: &crate::config::manifest::Manifest,
    backup_dir: &std::path::Path,
) -> Result<()> {
    let cov = &manifest.coverage;

    println!("Running coverage gate...");

    // Validate commands are configured.
    let sentinel = "givn missing-coverage";
    if cov.non_e2e_command == sentinel || cov.e2e_command == sentinel {
        let _ = rollback_specs_and_readme(project_root, backup_dir);
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
        .status()?;
    if !non_e2e_status.success() {
        let _ = rollback_specs_and_readme(project_root, backup_dir);
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
        .status()?;
    if !e2e_status.success() {
        let _ = rollback_specs_and_readme(project_root, backup_dir);
        bail!(
            "archive blocked: e2e coverage command failed (exit code {}). \
             Specs and README have been rolled back.",
            e2e_status.code().unwrap_or(-1)
        );
    }

    // Parse Cobertura XML files.
    let non_e2e_path = project_root.join(&cov.non_e2e_path);
    let e2e_path = project_root.join(&cov.e2e_path);

    let non_e2e_report = crate::cobertura::parse(&non_e2e_path)
        .map_err(|e| anyhow::anyhow!("archive blocked: failed to parse non-e2e Cobertura XML: {e}"))?;
    let e2e_report = crate::cobertura::parse(&e2e_path)
        .map_err(|e| anyhow::anyhow!("archive blocked: failed to parse e2e Cobertura XML: {e}"))?;

    // Validate individual reports.
    crate::cobertura::validate(&non_e2e_report)
        .map_err(|e| anyhow::anyhow!("archive blocked: non-e2e coverage validation failed: {e}"))?;
    crate::cobertura::validate(&e2e_report)
        .map_err(|e| anyhow::anyhow!("archive blocked: e2e coverage validation failed: {e}"))?;

    // Ensure the two reports are structurally distinct.
    crate::cobertura::assert_distinct(&non_e2e_report, &e2e_report)
        .map_err(|e| anyhow::anyhow!("archive blocked: {e}"))?;

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

/// Roll back specs and README to pre-archive state.
///
/// Errors from the rollback itself are logged but swallowed — if the
/// rollback fails we are already in a fault handler and cannot recover.
fn rollback_specs_and_readme(project_root: &Path, backup_dir: &Path) -> Result<()> {
    let backup_specs = backup_dir.join("specs");
    if let Err(e) = crate::cli::archive::rollback_specs(
        &project_root.join("givn/specs"),
        &backup_specs,
        backup_dir,
    ) {
        eprintln!("  warning: spec rollback failed: {e}");
    }
    if let Err(e) = rollback_readme(project_root, backup_dir) {
        eprintln!("  warning: README rollback failed: {e}");
    }
    Ok(())
}

/// Roll back the coverage summary from README.
fn rollback_readme(project_root: &Path, _backup_dir: &Path) -> Result<()> {
    let readme_path = project_root.join("README.md");
    if !readme_path.exists() {
        return Ok(());
    }

    let readme_content = std::fs::read_to_string(&readme_path)?;
    let updated = crate::cli::agents_block::remove_block(&readme_content, "coverage");
    if updated != readme_content {
        std::fs::write(&readme_path, updated)?;
    }
    println!("  restored README.md (removed coverage fence)");
    Ok(())
}
