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

    if cov.measure_command == COVERAGE_SENTINEL || cov.merge_command == COVERAGE_SENTINEL {
        bail!(
            "archive blocked: coverage commands are not configured. Set coverage.measure_command \
             and coverage.merge_command in givn/commands.yaml."
        );
    }

    let paths = CoveragePaths::new(project_root, cov)?;

    println!("  Running coverage measurement: {}", cov.measure_command);
    run_command(
        project_root,
        &cov.measure_command,
        &paths,
        "coverage measurement",
    )?;

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

    println!("  Running coverage merge: {}", cov.merge_command);
    run_command(project_root, &cov.merge_command, &paths, "coverage merge")?;

    if !paths.output.exists() {
        bail!(
            "archive blocked: coverage merge command succeeded but did not create {}",
            cov.output_path
        );
    }

    let merged = parse_and_validate(&paths.output, "merged")?;
    let readme_path = project_root.join("README.md");
    let badges = crate::cobertura::render_badges(&merged, &cov.output_path);
    crate::cli::agents_block::apply_after_title(&readme_path, "coverage-badge", &badges)?;
    let summary = crate::cobertura::render_readme_summary(
        &merged,
        &cov.output_path,
        &cov.measure_command,
        &cov.merge_command,
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
