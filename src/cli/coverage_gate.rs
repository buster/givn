use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

const COVERAGE_SENTINEL: &str = "givn missing-coverage";

/// Run the archive-level coverage pipeline: measure, merge, validate, and
/// publish the merged result in the README.
///
/// A failure leaves the archive candidate unproven in place; there is no
/// backup or rollback in the forward-only publication model.
pub(crate) fn run_coverage_gate(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
) -> Result<crate::proof::CoverageOutcome> {
    let merged = run_coverage_gate_inner(project_root, manifest)?;
    Ok(crate::proof::CoverageOutcome::passed(&merged))
}

fn run_coverage_gate_inner(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
) -> Result<crate::cobertura::CoberturaReport> {
    let cov = &manifest.coverage;

    println!("Running coverage gate...");

    let legacy_commands =
        if cov.measure_command == COVERAGE_SENTINEL && cov.merge_command == COVERAGE_SENTINEL {
            cov.non_e2e_command
                .as_deref()
                .zip(cov.e2e_command.as_deref())
        } else {
            None
        };
    if (cov.measure_command == COVERAGE_SENTINEL || cov.merge_command == COVERAGE_SENTINEL)
        && legacy_commands.is_none()
    {
        bail!(
            "archive blocked: coverage commands are not configured. Set coverage.measure_command \
             and coverage.merge_command in givn/commands.yaml."
        );
    }

    let paths = CoveragePaths::new(project_root, cov)?;

    if let Some((non_e2e_command, e2e_command)) = legacy_commands {
        println!("  Running legacy non-e2e coverage: {non_e2e_command}");
        run_command(
            project_root,
            non_e2e_command,
            &paths,
            "legacy non-e2e coverage",
        )?;
        println!("  Running legacy e2e coverage: {e2e_command}");
        run_command(project_root, e2e_command, &paths, "legacy e2e coverage")?;
    } else {
        println!("  Running coverage measurement: {}", cov.measure_command);
        run_command(
            project_root,
            &cov.measure_command,
            &paths,
            "coverage measurement",
        )?;
    }

    let non_e2e_report = parse_and_validate(&paths.non_e2e, "non-e2e")?;
    let e2e_report = parse_and_validate(&paths.e2e, "e2e")?;
    crate::cobertura::assert_distinct(&non_e2e_report, &e2e_report)
        .map_err(|e| anyhow::anyhow!("archive blocked: {e}"))?;

    if paths.output.exists() {
        std::fs::remove_file(&paths.output).with_context(|| {
            format!(
                "archive blocked: failed to remove stale merged coverage report {}",
                paths.output.display()
            )
        })?;
    }

    if legacy_commands.is_some() {
        write_legacy_merged_report(&paths.output, &non_e2e_report, &e2e_report)?;
    } else {
        println!("  Running coverage merge: {}", cov.merge_command);
        run_command(project_root, &cov.merge_command, &paths, "coverage merge")?;
    }

    if !paths.output.exists() {
        bail!(
            "archive blocked: coverage merge command succeeded but did not create {}",
            cov.output_path
        );
    }

    let merged = parse_and_validate(&paths.output, "merged")?;
    let readme_path = project_root.join("README.md");
    remove_legacy_coverage_badges(&readme_path)?;
    let badges = crate::cobertura::render_badges(&merged, &cov.output_path);
    crate::cli::agents_block::apply_after_title(&readme_path, "coverage-badge", &badges)?;
    let summary = crate::cobertura::render_readme_summary(
        &merged,
        &cov.output_path,
        cov.non_e2e_command
            .as_deref()
            .unwrap_or(&cov.measure_command),
        cov.e2e_command.as_deref().unwrap_or(&cov.merge_command),
    );
    crate::cli::agents_block::apply(&readme_path, "coverage", &summary)?;

    println!(
        "  published merged coverage: {}",
        crate::cobertura::render_markdown(&merged)
    );
    println!("  updated README.md with merged line and branch coverage");
    println!("✓ Coverage gate passed");
    Ok(merged)
}

struct CoveragePaths {
    non_e2e: PathBuf,
    e2e: PathBuf,
    output: PathBuf,
    non_e2e_relative: String,
    e2e_relative: String,
    output_relative: String,
}

impl CoveragePaths {
    fn new(project_root: &Path, coverage: &crate::config::manifest::Coverage) -> Result<Self> {
        let non_e2e = project_path(project_root, &coverage.non_e2e_path)?;
        let e2e = project_path(project_root, &coverage.e2e_path)?;
        let output = project_path(project_root, &coverage.output_path)?;
        if non_e2e == e2e || non_e2e == output || e2e == output {
            bail!("coverage input and output paths must be distinct");
        }
        Ok(Self {
            non_e2e,
            e2e,
            output,
            non_e2e_relative: coverage.non_e2e_path.clone(),
            e2e_relative: coverage.e2e_path.clone(),
            output_relative: coverage.output_path.clone(),
        })
    }
}

fn remove_legacy_coverage_badges(readme_path: &Path) -> Result<()> {
    if !readme_path.exists() {
        return Ok(());
    }
    let existing = std::fs::read_to_string(readme_path)?;
    let updated: String = existing
        .lines()
        .filter(|line| {
            !(line.contains("img.shields.io/badge/coverage-")
                || line.contains("img.shields.io/badge/line%20coverage-")
                || line.contains("img.shields.io/badge/branch%20coverage-"))
        })
        .map(|line| format!("{line}\n"))
        .collect();
    if updated != existing {
        std::fs::write(readme_path, updated)?;
    }
    Ok(())
}

fn project_path(project_root: &Path, configured: &str) -> Result<PathBuf> {
    let path = Path::new(configured);
    if path.is_absolute()
        || path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
    {
        bail!(
            "coverage path '{}' must be relative to the project root and cannot contain '..'",
            configured
        );
    }
    Ok(project_root.join(path))
}

fn run_command(
    project_root: &Path,
    command: &str,
    paths: &CoveragePaths,
    label: &str,
) -> Result<()> {
    let status = crate::subprocess::sh_streamed(
        command,
        &[
            ("GIVN_FEATURES", "givn/specs"),
            ("GIVN_COVERAGE_NON_E2E_PATH", &paths.non_e2e_relative),
            ("GIVN_COVERAGE_E2E_PATH", &paths.e2e_relative),
            ("GIVN_COVERAGE_OUTPUT_PATH", &paths.output_relative),
        ],
        Some(project_root),
        label,
    )
    .with_context(|| format!("archive blocked: failed to start {label} command"))?;

    if !status.success() {
        bail!(
            "archive blocked: {label} command failed (exit code {}).",
            status.code().unwrap_or(-1)
        );
    }
    Ok(())
}

fn parse_and_validate(path: &Path, label: &str) -> Result<crate::cobertura::CoberturaReport> {
    let report = crate::cobertura::parse(path).map_err(|e| {
        anyhow::anyhow!("archive blocked: failed to parse {label} Cobertura XML: {e}")
    })?;
    crate::cobertura::validate(&report)
        .map_err(|e| anyhow::anyhow!("archive blocked: {label} coverage validation failed: {e}"))?;
    Ok(report)
}

fn write_legacy_merged_report(
    output: &Path,
    non_e2e: &crate::cobertura::CoberturaReport,
    e2e: &crate::cobertura::CoberturaReport,
) -> Result<()> {
    let merged = crate::cobertura::merge_legacy_reports(non_e2e, e2e);
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let xml = format!(
        r#"<?xml version="1.0"?>
<coverage line-rate="{line_rate}" branch-rate="{branch_rate}" lines-covered="{lines_covered}" lines-valid="{lines_valid}" branches-covered="{branches_covered}" branches-valid="{branches_valid}" timestamp="0" version="1">
  <packages />
</coverage>"#,
        line_rate = merged.line_rate,
        branch_rate = merged.branch_rate,
        lines_covered = merged.lines_covered,
        lines_valid = merged.lines_valid,
        branches_covered = merged.branches_covered,
        branches_valid = merged.branches_valid,
    );
    std::fs::write(output, xml)?;
    Ok(())
}
