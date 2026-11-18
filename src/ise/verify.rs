//! `givn ise verify-trial`: the component-tier execution gate.
//!
//! Comparison proves frozen-input equality. This module runs the generated
//! verification contract and is the only path that can report a whole-system
//! trial result. It requires fresh runner results, exact declared
//! execution-unit counts, zero failed and zero skipped counts, and coverage
//! when the contract carries it.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::config::overlay::load_effective_at;
use crate::proof::RunnerResult;
use crate::scm::EntryKind;

use super::compare::{CompareRequest, require_recorded_copy};
use super::contract::{Contract, CoverageContract};
use super::manifest::{self, Manifest};
use super::require_addons;

/// `givn ise verify-trial`'s parsed request.
#[derive(Debug, Clone)]
pub struct VerifyRequest {
    /// The manifest the preparation wrote.
    pub manifest: PathBuf,
    /// The spec copy the manifest describes.
    pub copy: PathBuf,
    /// Where the `ise-trial-result/v1` record is written.
    pub result: PathBuf,
    /// An explicit not-run decision; accepted only when the contract declares
    /// no component execution units.
    pub component_not_run: Option<String>,
}

/// One tier's recorded result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierReport {
    /// `passed`, `failed`, `not_run`, or `not_applicable`.
    pub status: &'static str,
    pub command: String,
    pub features: String,
    pub declared: usize,
    pub executed: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<CoverageNumbers>,
}

/// One tier's line and branch coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoverageNumbers {
    /// `measured` or `subprocess-uninstrumented`.
    pub status: &'static str,
    pub lines_covered: u64,
    pub lines_valid: u64,
    pub branches_covered: u64,
    pub branches_valid: u64,
}

/// The trial's two measured tiers and their coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrialReport {
    pub scope: &'static str,
    pub revision: String,
    pub copy_directory: String,
    pub component: TierReport,
    pub end_to_end: TierReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<TrialCoverage>,
    /// True only when both tiers ran their declared units and coverage passed.
    pub clean: bool,
}

/// The two tier reports' coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TrialCoverage {
    pub component: CoverageNumbers,
    pub end_to_end: CoverageNumbers,
}

/// Verify one trial through its generated contract.
pub fn verify_trial(project_root: &Path, request: &VerifyRequest) -> Result<TrialReport> {
    let effective = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&effective)?;

    // Input equality is a precondition, never the trial result.
    let comparison = super::compare(
        project_root,
        &CompareRequest {
            manifest: request.manifest.clone(),
            copy: request.copy.clone(),
        },
    )?;
    if !comparison.is_clean() {
        bail!(
            "the frozen Specification or End-to-end inputs differ; resolve `givn ise compare` before claiming a trial result"
        );
    }

    let recorded = Manifest::read(&request.manifest)?;
    let copy = request
        .copy
        .canonicalize()
        .with_context(|| format!("resolve the spec copy '{}'", request.copy.display()))?;
    require_recorded_copy(&request.manifest, &recorded, &copy)?;
    let (contract, _) = read_contract(&recorded, &copy)?;

    let result = super::resolved_target(&request.result, project_root)?;
    if std::fs::symlink_metadata(&result).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        bail!(
            "the trial result '{}' is a symlink; the result is evidence written to a regular path",
            result.display()
        );
    }
    if result.starts_with(&copy) {
        bail!(
            "the trial result '{}' is inside the spec copy '{}'; the result is evidence outside the copy",
            result.display(),
            copy.display()
        );
    }

    let component_result = sibling(&result, "component");
    let e2e_result = sibling(&result, "e2e");

    let component = match run_component(
        &contract,
        &copy,
        &component_result,
        request.component_not_run.as_deref(),
    ) {
        Ok(component) => component,
        Err(failure) => {
            let report = TrialReport {
                scope: "ise-trial-result/v1",
                revision: recorded.revision.clone(),
                copy_directory: recorded.copy_directory.clone(),
                component: *failure.report,
                end_to_end: not_reached(&contract.end_to_end),
                coverage: None,
                clean: false,
            };
            write_result(&result, &report)?;
            return Err(failure.error);
        }
    };

    let end_to_end = match run_end_to_end(&contract, project_root, &e2e_result) {
        Ok(end_to_end) => end_to_end,
        Err(failure) => {
            let report = TrialReport {
                scope: "ise-trial-result/v1",
                revision: recorded.revision.clone(),
                copy_directory: recorded.copy_directory.clone(),
                component: component.clone(),
                end_to_end: *failure.report,
                coverage: None,
                clean: false,
            };
            write_result(&result, &report)?;
            return Err(failure.error);
        }
    };

    let coverage = match &contract.coverage {
        Some(coverage) => match run_coverage(coverage, &copy, &contract) {
            Ok(coverage) => Some(coverage),
            Err(error) => {
                let report = TrialReport {
                    scope: "ise-trial-result/v1",
                    revision: recorded.revision.clone(),
                    copy_directory: recorded.copy_directory.clone(),
                    component: component.clone(),
                    end_to_end: end_to_end.clone(),
                    coverage: None,
                    clean: false,
                };
                write_result(&result, &report)?;
                return Err(error);
            }
        },
        None => None,
    };

    let report = TrialReport {
        scope: "ise-trial-result/v1",
        revision: recorded.revision.clone(),
        copy_directory: recorded.copy_directory.clone(),
        component,
        end_to_end,
        coverage,
        clean: true,
    };
    write_result(&result, &report)?;
    Ok(report)
}

/// A tier failure with the partial report the result file records.
struct TierFailure {
    error: anyhow::Error,
    report: Box<TierReport>,
}

impl TierFailure {
    fn new(error: anyhow::Error, report: TierReport) -> Self {
        Self {
            error,
            report: Box::new(report),
        }
    }
}

/// Read the generated contract and require the manifest's checksum.
fn read_contract(recorded: &Manifest, copy: &Path) -> Result<(Contract, String)> {
    let Some(verification) = &recorded.verification else {
        bail!(
            "the manifest records no verification contract; rebuild the trial with this Givn version before claiming a whole-system result"
        );
    };
    if verification.path != super::contract::CONTRACT_PATH {
        bail!(
            "the manifest records the verification contract at '{}'; the only contract path is '{}'",
            verification.path,
            super::contract::CONTRACT_PATH
        );
    }
    let path = copy.join(&verification.path);
    let bytes = std::fs::read(&path).with_context(|| {
        format!(
            "read the verification contract '{}' recorded by the manifest",
            path.display()
        )
    })?;
    let checksum = manifest::path_checksum(&verification.path, EntryKind::Regular, false, &bytes);
    if checksum != verification.checksum {
        bail!(
            "the verification contract '{}' changed after the manifest was written",
            verification.path
        );
    }
    Ok((Contract::read(&path)?, checksum))
}

/// Run the component tier inside the spec copy.
fn run_component(
    contract: &Contract,
    copy: &Path,
    result_file: &Path,
    component_not_run: Option<&str>,
) -> std::result::Result<TierReport, TierFailure> {
    let tier = &contract.component;
    let declared = tier.execution_units;
    if let Some(reason) = component_not_run {
        if declared > 0 {
            return Err(TierFailure::new(
                anyhow::anyhow!(
                    "the contract declares {declared} component execution units; a not-run decision cannot replace their execution"
                ),
                not_run(tier, reason),
            ));
        }
        return Ok(not_run(tier, reason));
    }
    if declared == 0 {
        return Ok(not_applicable(tier));
    }
    if not_runnable(&tier.command) {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the verification contract has no component command; component_executed=0/{declared}"
            ),
            failed(tier),
        ));
    }

    remove_previous(result_file);
    let status = crate::subprocess::sh_streamed(
        &tier.command,
        &[
            ("GIVN_FEATURES", &tier.features),
            ("GIVN_RESULT_FILE", &absolute_text(result_file)),
            ("GIVN_RESULT_SCOPE", "component"),
        ],
        Some(copy),
        "component verification",
    );
    let status = match status {
        Ok(status) => status,
        Err(error) => {
            return Err(TierFailure::new(error.into(), failed(tier)));
        }
    };
    if !result_file.exists() {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the component verification produced no runner result at '{}'; component_executed=0/{declared}; the frozen End-to-end tests alone do not establish a whole-system rebuild",
                result_file.display()
            ),
            failed(tier),
        ));
    }
    let counts = match read_counts(result_file, "component", "component") {
        Ok(counts) => counts,
        Err(error) => return Err(TierFailure::new(error, failed(tier))),
    };
    if !status.success() {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the component verification exited {}; component_executed={}/{declared}",
                status.code().unwrap_or(-1),
                counts.total
            ),
            with_counts(tier, &counts, "failed"),
        ));
    }
    if counts.total != declared as u64 {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the component verification executed {} of {declared} declared execution units; component_executed={}/{declared}",
                counts.total,
                counts.total
            ),
            with_counts(tier, &counts, "failed"),
        ));
    }
    Ok(measured(tier, &counts))
}

/// Run the frozen End-to-end tier in the checkout.
fn run_end_to_end(
    contract: &Contract,
    project_root: &Path,
    result_file: &Path,
) -> std::result::Result<TierReport, TierFailure> {
    let tier = &contract.end_to_end;
    let declared = tier.execution_units;
    if declared == 0 {
        return Ok(not_applicable(tier));
    }
    if not_runnable(&tier.command) {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the verification contract has no End-to-end command; e2e_executed=0/{declared}"
            ),
            failed(tier),
        ));
    }
    remove_previous(result_file);
    let status = match crate::subprocess::sh_streamed(
        &tier.command,
        &[
            ("GIVN_FEATURES", &tier.features),
            ("GIVN_RESULT_FILE", &absolute_text(result_file)),
            ("GIVN_RESULT_SCOPE", "e2e"),
        ],
        Some(project_root),
        "End-to-end verification",
    ) {
        Ok(status) => status,
        Err(error) => {
            return Err(TierFailure::new(
                anyhow::Error::from(error).context("start the End-to-end verification"),
                failed(tier),
            ));
        }
    };
    if !result_file.exists() {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the End-to-end verification produced no runner result at '{}'; e2e_executed=0/{declared}",
                result_file.display()
            ),
            failed(tier),
        ));
    }
    let counts = match read_counts(result_file, "End-to-end", "e2e") {
        Ok(counts) => counts,
        Err(error) => return Err(TierFailure::new(error, failed(tier))),
    };
    if !status.success() {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the End-to-end verification exited {}; e2e_executed={}/{declared}",
                status.code().unwrap_or(-1),
                counts.total
            ),
            with_counts(tier, &counts, "failed"),
        ));
    }
    if counts.total != declared as u64 {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the End-to-end verification executed {} of {declared} declared execution units; e2e_executed={}/{declared}",
                counts.total,
                counts.total
            ),
            with_counts(tier, &counts, "failed"),
        ));
    }
    Ok(measured(tier, &counts))
}

/// Run the coverage measurement and validate both tier reports.
fn run_coverage(
    coverage: &CoverageContract,
    copy: &Path,
    contract: &Contract,
) -> Result<TrialCoverage> {
    if not_runnable(&coverage.measure_command) {
        bail!("the coverage addon is effective but coverage.measure_command is not configured");
    }
    let non_e2e = copy.join(&coverage.non_e2e_path);
    let e2e = copy.join(&coverage.e2e_path);
    remove_previous(&non_e2e);
    remove_previous(&e2e);
    let status = crate::subprocess::sh_streamed(
        &coverage.measure_command,
        &[
            ("GIVN_FEATURES", &contract.component.features),
            ("GIVN_COVERAGE_NON_E2E_PATH", &coverage.non_e2e_path),
            ("GIVN_COVERAGE_E2E_PATH", &coverage.e2e_path),
        ],
        Some(copy),
        "trial coverage measurement",
    )
    .context("start the trial coverage measurement")?;
    if !status.success() {
        bail!(
            "the trial coverage measurement exited {}",
            status.code().unwrap_or(-1)
        );
    }

    let component_report = crate::cobertura::parse(&non_e2e)
        .with_context(|| format!("read the component coverage '{}'", non_e2e.display()))?;
    crate::cobertura::validate(&component_report)
        .context("the component coverage report is invalid")?;
    if component_report.lines_covered == 0 {
        bail!(
            "the component coverage report measured no executed line; the component corpus was not exercised"
        );
    }
    let component = coverage_numbers(&component_report, "measured");

    let end_to_end_report = crate::cobertura::parse(&e2e)
        .with_context(|| format!("read the End-to-end coverage '{}'", e2e.display()))?;
    let end_to_end = if end_to_end_report.lines_valid == 0 {
        if !coverage.e2e_subprocess {
            bail!(
                "the End-to-end coverage report measured zero lines; declare `coverage.e2e_subprocess: true` only when the suite runs the product as an uninstrumented subprocess"
            );
        }
        coverage_numbers(&end_to_end_report, "subprocess-uninstrumented")
    } else {
        crate::cobertura::validate(&end_to_end_report)
            .context("the End-to-end coverage report is invalid")?;
        if end_to_end_report.lines_covered == 0 {
            bail!("the End-to-end coverage report measured no executed line");
        }
        coverage_numbers(&end_to_end_report, "measured")
    };

    if component_report.lines_valid > 0 && end_to_end_report.lines_valid > 0 {
        crate::cobertura::assert_distinct(&component_report, &end_to_end_report)
            .context("the two coverage reports are not independent")?;
    }

    Ok(TrialCoverage {
        component,
        end_to_end,
    })
}

/// One Cobertura report's numbers.
fn coverage_numbers(
    report: &crate::cobertura::CoberturaReport,
    status: &'static str,
) -> CoverageNumbers {
    CoverageNumbers {
        status,
        lines_covered: report.lines_covered,
        lines_valid: report.lines_valid,
        branches_covered: report.branches_covered,
        branches_valid: report.branches_valid,
    }
}

/// Read and validate one fresh runner result.
fn read_counts(path: &Path, label: &str, expected_scope: &str) -> Result<RunnerResult> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read the {label} runner result '{}'", path.display()))?;
    let result: RunnerResult = serde_json::from_str(&text).map_err(|error| {
        anyhow::anyhow!(
            "the {label} runner result '{}' is malformed: {error}",
            path.display()
        )
    })?;
    if let Some(scope) = result.scope.as_deref()
        && scope != expected_scope
    {
        bail!(
            "the {label} runner result '{}' declares scope '{scope}' but '{expected_scope}' was expected",
            path.display()
        );
    }
    if result.total == 0 {
        bail!(
            "the {label} runner result '{}' declares zero scenarios",
            path.display()
        );
    }
    if result.failed != 0 {
        bail!("the {label} runner result reports {} failed", result.failed);
    }
    if result.skipped != 0 {
        bail!(
            "the {label} runner result reports {} skipped",
            result.skipped
        );
    }
    if result.passed + result.failed + result.skipped != result.total {
        bail!(
            "the {label} runner result counts do not reconcile (passed {} + failed {} + skipped {} != total {})",
            result.passed,
            result.failed,
            result.skipped,
            result.total
        );
    }
    Ok(result)
}

/// A passing tier report from its counts.
fn measured(tier: &super::contract::Tier, counts: &RunnerResult) -> TierReport {
    with_counts(tier, counts, "passed")
}

/// A tier report with its measured counts and an explicit status.
fn with_counts(
    tier: &super::contract::Tier,
    counts: &RunnerResult,
    status: &'static str,
) -> TierReport {
    TierReport {
        status,
        command: tier.command.clone(),
        features: tier.features.clone(),
        declared: tier.execution_units,
        executed: counts.total,
        passed: counts.passed,
        failed: counts.failed,
        skipped: counts.skipped,
        reason: None,
        coverage: None,
    }
}

/// A failed tier report with no counts.
fn failed(tier: &super::contract::Tier) -> TierReport {
    TierReport {
        status: "failed",
        command: tier.command.clone(),
        features: tier.features.clone(),
        declared: tier.execution_units,
        executed: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        reason: None,
        coverage: None,
    }
}

/// A tier the earlier failure prevented from running.
fn not_reached(tier: &super::contract::Tier) -> TierReport {
    TierReport {
        status: "not_reached",
        command: tier.command.clone(),
        features: tier.features.clone(),
        declared: tier.execution_units,
        executed: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        reason: None,
        coverage: None,
    }
}

/// A tier with no declared execution units.
fn not_applicable(tier: &super::contract::Tier) -> TierReport {
    TierReport {
        status: "not_applicable",
        command: tier.command.clone(),
        features: tier.features.clone(),
        declared: 0,
        executed: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        reason: Some("the specification declares no execution units for this tier".to_string()),
        coverage: None,
    }
}

/// An explicit not-run decision with its reason.
fn not_run(tier: &super::contract::Tier, reason: &str) -> TierReport {
    TierReport {
        status: "not_run",
        command: tier.command.clone(),
        features: tier.features.clone(),
        declared: tier.execution_units,
        executed: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        reason: Some(reason.to_string()),
        coverage: None,
    }
}

/// Write the result record, creating its parent directory when needed.
fn write_result(path: &Path, report: &TrialReport) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create the result directory '{}'", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(report).context("serialize the trial result")?;
    std::fs::write(path, format!("{text}\n"))
        .with_context(|| format!("write the trial result '{}'", path.display()))
}

/// A raw runner-result sibling of the result file.
fn sibling(result: &Path, suffix: &str) -> PathBuf {
    let name = result
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "trial-result.json".to_string());
    result.with_file_name(format!("{name}.{suffix}"))
}

/// A path rendered for an environment variable.
fn absolute_text(path: &Path) -> std::borrow::Cow<'_, str> {
    path.to_string_lossy()
}

/// Remove a previous raw result so a stale file cannot satisfy the gate.
fn remove_previous(path: &Path) {
    if let Err(error) = std::fs::remove_file(path)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        eprintln!(
            "warning: could not remove the previous result '{}': {error}",
            path.display()
        );
    }
}

/// A command that is empty or is Givn's unconfigured sentinel.
fn not_runnable(command: &str) -> bool {
    let command = command.trim();
    command.is_empty() || command.starts_with("givn missing-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_component_result_is_zero_of_declared() {
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("missing.json");
        let tier = super::super::contract::Tier {
            command: "true".to_string(),
            features: "givn/specs".to_string(),
            execution_units: 10,
        };
        let failure = run_component(
            &Contract {
                scope: "ise-verification/v1".to_string(),
                revision: "abc".to_string(),
                component: tier,
                end_to_end: super::super::contract::Tier {
                    command: "true".to_string(),
                    features: "givn/specs".to_string(),
                    execution_units: 1,
                },
                coverage: None,
            },
            directory.path(),
            &path,
            None,
        )
        .expect_err("a missing result refuses");
        assert!(
            failure
                .error
                .to_string()
                .contains("component_executed=0/10")
        );
        assert_eq!(failure.report.status, "failed");
    }

    #[test]
    fn a_mismatched_component_count_refuses() {
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("result.json");
        std::fs::write(
            &path,
            "{\"scope\":\"component\",\"total\":9,\"passed\":9,\"failed\":0,\"skipped\":0}\n",
        )
        .unwrap();
        let script = directory.path().join("verify.sh");
        std::fs::write(
            &script,
            "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":9,\"passed\":9,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
        )
        .unwrap();
        let tier = super::super::contract::Tier {
            command: format!("sh {}", script.display()),
            features: "givn/specs".to_string(),
            execution_units: 10,
        };
        let failure = run_component(
            &Contract {
                scope: "ise-verification/v1".to_string(),
                revision: "abc".to_string(),
                component: tier,
                end_to_end: super::super::contract::Tier {
                    command: "true".to_string(),
                    features: "givn/specs".to_string(),
                    execution_units: 1,
                },
                coverage: None,
            },
            directory.path(),
            &path,
            None,
        )
        .expect_err("a mismatch refuses");
        assert!(
            failure
                .error
                .to_string()
                .contains("component_executed=9/10")
        );
    }

    #[test]
    fn a_not_run_decision_refuses_when_units_are_declared() {
        let directory = tempfile::TempDir::new().unwrap();
        let contract = Contract {
            scope: "ise-verification/v1".to_string(),
            revision: "abc".to_string(),
            component: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 10,
            },
            end_to_end: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 1,
            },
            coverage: None,
        };
        let failure = run_component(
            &contract,
            directory.path(),
            &directory.path().join("result.json"),
            Some("out of scope"),
        )
        .expect_err("a not-run bypass refuses");
        assert!(failure.error.to_string().contains("cannot replace"));
    }

    fn coverage_contract(path: &Path, e2e_subprocess: bool) -> CoverageContract {
        CoverageContract {
            measure_command: format!("sh {}", path.display()),
            non_e2e_path: "coverage/non-e2e.xml".to_string(),
            e2e_path: "coverage/e2e.xml".to_string(),
            e2e_subprocess,
        }
    }

    fn contract_with_coverage(coverage: CoverageContract) -> Contract {
        Contract {
            scope: "ise-verification/v1".to_string(),
            revision: "abc".to_string(),
            component: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 10,
            },
            end_to_end: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 1,
            },
            coverage: Some(coverage),
        }
    }

    fn write_measure_script(path: &Path, component: &str, end_to_end: &str) {
        std::fs::write(
            path,
            format!(
                "#!/bin/sh\nmkdir -p coverage\nprintf '%s' '{component}' > \"$GIVN_COVERAGE_NON_E2E_PATH\"\nprintf '%s' '{end_to_end}' > \"$GIVN_COVERAGE_E2E_PATH\"\n"
            ),
        )
        .unwrap();
    }

    fn cobertura(lines_covered: u64, lines_valid: u64) -> String {
        format!(
            "<?xml version=\"1.0\"?><coverage line-rate=\"{rate}\" branch-rate=\"0\" lines-covered=\"{lines_covered}\" lines-valid=\"{lines_valid}\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\"><packages /></coverage>",
            rate = if lines_valid == 0 {
                0.0
            } else {
                lines_covered as f64 / lines_valid as f64
            }
        )
    }

    #[test]
    fn coverage_rejects_a_zero_line_component_report() {
        let directory = tempfile::TempDir::new().unwrap();
        let script = directory.path().join("measure.sh");
        write_measure_script(&script, &cobertura(0, 0), &cobertura(1, 10));
        let contract = contract_with_coverage(coverage_contract(&script, false));
        let error = run_coverage(
            contract.coverage.as_ref().unwrap(),
            directory.path(),
            &contract,
        )
        .expect_err("a zero-line component report refuses");
        assert!(format!("{error:#}").contains("zero lines"), "{error:#}");
    }

    #[test]
    fn coverage_rejects_a_zero_line_e2e_report_without_the_declaration() {
        let directory = tempfile::TempDir::new().unwrap();
        let script = directory.path().join("measure.sh");
        write_measure_script(&script, &cobertura(5, 10), &cobertura(0, 0));
        let contract = contract_with_coverage(coverage_contract(&script, false));
        let error = run_coverage(
            contract.coverage.as_ref().unwrap(),
            directory.path(),
            &contract,
        )
        .expect_err("a zero-line e2e report refuses without the declaration");
        assert!(format!("{error:#}").contains("e2e_subprocess"), "{error:#}");
    }

    #[test]
    fn coverage_accepts_a_declared_subprocess_e2e_report() {
        let directory = tempfile::TempDir::new().unwrap();
        let script = directory.path().join("measure.sh");
        write_measure_script(&script, &cobertura(5, 10), &cobertura(0, 0));
        let contract = contract_with_coverage(coverage_contract(&script, true));
        let coverage = run_coverage(
            contract.coverage.as_ref().unwrap(),
            directory.path(),
            &contract,
        )
        .expect("a declared subprocess e2e report passes");
        assert_eq!(coverage.end_to_end.status, "subprocess-uninstrumented");
        assert_eq!(coverage.component.lines_covered, 5);
    }
}
