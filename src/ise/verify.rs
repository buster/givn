//! `givn ise verify-trial`: the component-tier execution gate.
//!
//! Comparison proves frozen-input equality. This module runs the generated
//! verification contract and is the only path that can report a whole-system
//! trial result. It requires fresh runner results, exact declared
//! execution-unit counts, zero failed and zero skipped counts, and coverage
//! when the contract carries it.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::overlay::load_effective_at;
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
    /// The scenario titles the runner reported.
    pub scenarios: Vec<String>,
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
    /// The rebuilt product the End-to-end tier drove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductReport>,
    /// True only when both tiers ran their declared units and coverage passed.
    pub clean: bool,
}

/// The rebuilt product's identity in the trial result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductReport {
    pub path: String,
    pub sha256: String,
}

/// The runner result's required evidence: counts, scenario titles, and the
/// product the End-to-end runner drove.
#[derive(Debug, Clone, Default, Deserialize)]
struct TrialRunnerResult {
    #[serde(default)]
    scope: Option<String>,
    total: u64,
    passed: u64,
    failed: u64,
    skipped: u64,
    #[serde(default)]
    scenarios: Vec<String>,
    #[serde(default)]
    product: Option<String>,
    #[serde(default)]
    product_sha256: Option<String>,
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

    let manifest_before = std::fs::read(&request.manifest)
        .with_context(|| format!("read the manifest '{}'", request.manifest.display()))?;
    let recorded = Manifest::read(&request.manifest)?;
    let copy = request
        .copy
        .canonicalize()
        .with_context(|| format!("resolve the spec copy '{}'", request.copy.display()))?;
    require_recorded_copy(&request.manifest, &recorded, &copy)?;
    let (contract, contract_before) = read_contract(&recorded, &copy)?;

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
                product: None,
                clean: false,
            };
            write_result(&result, &report)?;
            return Err(failure.error);
        }
    };

    let product = match bind_product(&contract, &copy) {
        Ok(product) => product,
        Err(error) => {
            let report = TrialReport {
                scope: "ise-trial-result/v1",
                revision: recorded.revision.clone(),
                copy_directory: recorded.copy_directory.clone(),
                component: component.clone(),
                end_to_end: not_reached(&contract.end_to_end),
                coverage: None,
                product: None,
                clean: false,
            };
            write_result(&result, &report)?;
            return Err(error);
        }
    };

    let end_to_end = match run_end_to_end(&contract, project_root, &e2e_result, product.as_ref()) {
        Ok(end_to_end) => end_to_end,
        Err(failure) => {
            let report = TrialReport {
                scope: "ise-trial-result/v1",
                revision: recorded.revision.clone(),
                copy_directory: recorded.copy_directory.clone(),
                component: component.clone(),
                end_to_end: *failure.report,
                coverage: None,
                product: product.as_ref().map(ProductBinding::report),
                clean: false,
            };
            write_result(&result, &report)?;
            return Err(failure.error);
        }
    };

    if let Some(product) = &product
        && let Err(error) = product.ensure_unchanged()
    {
        let report = TrialReport {
            scope: "ise-trial-result/v1",
            revision: recorded.revision.clone(),
            copy_directory: recorded.copy_directory.clone(),
            component: component.clone(),
            end_to_end: end_to_end.clone(),
            coverage: None,
            product: Some(product.report()),
            clean: false,
        };
        write_result(&result, &report)?;
        return Err(error);
    }

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
                    product: product.as_ref().map(ProductBinding::report),
                    clean: false,
                };
                write_result(&result, &report)?;
                return Err(error);
            }
        },
        None => None,
    };

    // Re-check frozen inputs after the scripts ran. This closes the window
    // The initial manifest and contract bytes are the authority: a changed
    // checkout, manifest, contract, or comparison error writes clean:false.
    let manifest_after = std::fs::read(&request.manifest);
    let contract_after = std::fs::read(copy.join(super::contract::CONTRACT_PATH));
    let inputs_changed = match (manifest_after, contract_after) {
        (Ok(manifest), Ok(contract)) => manifest != manifest_before || contract != contract_before,
        _ => true,
    };
    let product_changed = product
        .as_ref()
        .map(ProductBinding::ensure_unchanged)
        .transpose()
        .err();
    let comparison_clean = matches!(
        super::compare(
            project_root,
            &CompareRequest {
                manifest: request.manifest.clone(),
                copy: request.copy.clone(),
            },
        ),
        Ok(report) if report.is_clean()
    );
    if inputs_changed || product_changed.is_some() || !comparison_clean {
        let report = TrialReport {
            scope: "ise-trial-result/v1",
            revision: recorded.revision.clone(),
            copy_directory: recorded.copy_directory.clone(),
            component: component.clone(),
            end_to_end: end_to_end.clone(),
            coverage: coverage.clone(),
            product: product.as_ref().map(ProductBinding::report),
            clean: false,
        };
        write_result(&result, &report)?;
        bail!(
            "the frozen inputs or rebuilt product changed while the trial executed; the trial result is not clean"
        );
    }

    let report = TrialReport {
        scope: "ise-trial-result/v1",
        revision: recorded.revision.clone(),
        copy_directory: recorded.copy_directory.clone(),
        component,
        end_to_end,
        coverage,
        product: product.as_ref().map(ProductBinding::report),
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
fn read_contract(recorded: &Manifest, copy: &Path) -> Result<(Contract, Vec<u8>)> {
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
    Ok((Contract::from_bytes(&bytes, &path)?, bytes))
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

    if let Err(error) = remove_previous(result_file) {
        return Err(TierFailure::new(error, failed(tier)));
    }
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
    if let Err(error) = require_scenarios(tier, &counts) {
        return Err(TierFailure::new(
            error,
            with_counts(tier, &counts, "failed"),
        ));
    }
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

/// Resolve a contract-relative path through its deepest existing ancestor and
/// require the result to stay under the canonical copy.
fn under_copy(copy: &Path, relative: &str, what: &str) -> Result<PathBuf> {
    let target = copy.join(relative);
    let resolved = super::resolved_target(&target, copy)?;
    if !resolved.starts_with(copy) {
        bail!("{what} '{relative}' resolves outside the spec copy");
    }
    Ok(resolved)
}

/// Re-resolve a path after the measure command and require it still stays under
/// the copy; a builder-created parent symlink must not redirect a report.
fn require_still_under_copy(copy: &Path, path: &Path, what: &str) -> Result<()> {
    let resolved = std::fs::canonicalize(path)
        .with_context(|| format!("resolve {what} '{}'", path.display()))?;
    if !resolved.starts_with(copy) {
        bail!("{what} '{}' resolves outside the spec copy", path.display());
    }
    Ok(())
}

/// The rebuilt product's absolute path and content digest.
struct ProductBinding {
    path: PathBuf,
    sha256: String,
}

impl ProductBinding {
    fn ensure_unchanged(&self) -> Result<()> {
        let observed = digest_product(&self.path)?;
        if observed != self.sha256 {
            bail!(
                "the rebuilt product '{}' changed after the component command",
                self.path.display()
            );
        }
        Ok(())
    }

    fn report(&self) -> ProductReport {
        ProductReport {
            path: self.path.to_string_lossy().into_owned(),
            sha256: self.sha256.clone(),
        }
    }

    fn text(&self) -> std::borrow::Cow<'_, str> {
        self.path.to_string_lossy()
    }
}

/// Hash a product file's bytes or a product directory's regular-file tree.
/// Directory entries are sorted by relative path and include the executable
/// marker, so two binaries and their modes are part of one identity.
fn digest_product(path: &Path) -> Result<String> {
    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("read the rebuilt product '{}'", path.display()))?;
    let file_type = metadata.file_type();
    if file_type.is_symlink() {
        bail!("the rebuilt product '{}' is a symlink", path.display());
    }
    if file_type.is_file() {
        let bytes = std::fs::read(path)
            .with_context(|| format!("read the rebuilt product '{}'", path.display()))?;
        return Ok(Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect());
    }
    if !file_type.is_dir() {
        bail!(
            "the rebuilt product '{}' is neither a regular file nor a directory",
            path.display()
        );
    }
    let mut files: Vec<(String, bool, Vec<u8>)> = Vec::new();
    collect_product_files(path, path, &mut files)?;
    if files.is_empty() {
        bail!(
            "the rebuilt product directory '{}' contains no regular file",
            path.display()
        );
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hasher = Sha256::new();
    for (relative, executable, bytes) in files {
        hasher.update(relative.as_bytes());
        hasher.update(b"\0");
        hasher.update(if executable { b"1" } else { b"0" });
        hasher.update(b"\0");
        hasher.update(&bytes);
        hasher.update(b"\0");
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn collect_product_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<(String, bool, Vec<u8>)>,
) -> Result<()> {
    for entry in std::fs::read_dir(directory)
        .with_context(|| format!("read the rebuilt product '{}'", directory.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            bail!("the rebuilt product '{}' is a symlink", path.display());
        }
        if file_type.is_dir() {
            collect_product_files(root, &path, files)?;
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .expect("the walk starts at the product root")
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = std::fs::read(&path)?;
            files.push((relative, is_executable(&metadata), bytes));
        }
    }
    Ok(())
}

/// Bind the contract's declared product path to bytes inside the copy.
fn bind_product(contract: &Contract, copy: &Path) -> Result<Option<ProductBinding>> {
    if contract.component.execution_units == 0 && contract.end_to_end.execution_units == 0 {
        return Ok(None);
    }
    let Some(product) = &contract.product else {
        bail!(
            "the verification contract names no rebuilt product; set verify.product_path in givn/commands.yaml"
        );
    };
    let target = copy.join(&product.path);
    let metadata = std::fs::symlink_metadata(&target)
        .with_context(|| format!("read the rebuilt product '{}'", target.display()))?;
    if metadata.file_type().is_symlink()
        || !(metadata.file_type().is_file() || metadata.file_type().is_dir())
    {
        bail!(
            "the rebuilt product '{}' must be a regular file or directory inside the spec copy",
            product.path
        );
    }
    let resolved = std::fs::canonicalize(&target)
        .with_context(|| format!("resolve the rebuilt product '{}'", target.display()))?;
    if !resolved.starts_with(copy) {
        bail!(
            "the rebuilt product '{}' resolves outside the spec copy",
            product.path
        );
    }
    let sha256 = digest_product(&resolved)?;
    Ok(Some(ProductBinding {
        path: resolved,
        sha256,
    }))
}

/// Run the frozen End-to-end tier in the checkout, bound to the rebuilt product.
fn run_end_to_end(
    contract: &Contract,
    project_root: &Path,
    result_file: &Path,
    product: Option<&ProductBinding>,
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
    let Some(product) = product else {
        return Err(TierFailure::new(
            anyhow::anyhow!(
                "the End-to-end tier is declared but no rebuilt product is bound; set verify.product_path and run the component command"
            ),
            failed(tier),
        ));
    };
    let product_text = product.text();
    let result_text = absolute_text(result_file);
    let product_env = contract
        .product
        .as_ref()
        .map(|product| product.env.as_str())
        .unwrap_or("GIVN_BIN");
    if let Err(error) = remove_previous(result_file) {
        return Err(TierFailure::new(error, failed(tier)));
    }
    let status = match crate::subprocess::sh_streamed(
        &tier.command,
        &[
            ("GIVN_FEATURES", &tier.features),
            ("GIVN_RESULT_FILE", result_text.as_ref()),
            ("GIVN_RESULT_SCOPE", "e2e"),
            ("GIVN_TRIAL_PRODUCT", product_text.as_ref()),
            ("GIVN_TRIAL_PRODUCT_SHA256", product.sha256.as_str()),
            (product_env, product_text.as_ref()),
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
    if let Err(error) = require_scenarios(tier, &counts) {
        return Err(TierFailure::new(
            error,
            with_counts(tier, &counts, "failed"),
        ));
    }
    if let Err(error) = require_product(&counts, product) {
        return Err(TierFailure::new(
            error,
            with_counts(tier, &counts, "failed"),
        ));
    }
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
    let non_e2e = under_copy(copy, &coverage.non_e2e_path, "coverage.non_e2e_path")?;
    let e2e = under_copy(copy, &coverage.e2e_path, "coverage.e2e_path")?;
    let output = under_copy(copy, &coverage.output_path, "coverage.output_path")?;
    remove_previous(&non_e2e)?;
    remove_previous(&e2e)?;
    remove_previous(&output)?;
    let status = crate::subprocess::sh_streamed(
        &coverage.measure_command,
        &[
            ("GIVN_FEATURES", &contract.component.features),
            ("GIVN_COVERAGE_NON_E2E_PATH", &coverage.non_e2e_path),
            ("GIVN_COVERAGE_E2E_PATH", &coverage.e2e_path),
            ("GIVN_COVERAGE_OUTPUT_PATH", &coverage.output_path),
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
    require_still_under_copy(copy, &non_e2e, "coverage.non_e2e_path")?;
    if e2e.exists() {
        require_still_under_copy(copy, &e2e, "coverage.e2e_path")?;
    }
    if output.exists() {
        require_still_under_copy(copy, &output, "coverage.output_path")?;
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

    let end_to_end_report = if !e2e.exists() {
        if !coverage.e2e_subprocess {
            bail!(
                "the End-to-end coverage report '{}' is missing; declare `coverage.e2e_subprocess: true` only when the suite runs the product as an uninstrumented subprocess",
                coverage.e2e_path
            );
        }
        None
    } else {
        let report = crate::cobertura::parse(&e2e)
            .with_context(|| format!("read the End-to-end coverage '{}'", e2e.display()))?;
        if report.lines_valid == 0 {
            if !coverage.e2e_subprocess {
                bail!(
                    "the End-to-end coverage report measured zero lines; declare `coverage.e2e_subprocess: true` only when the suite runs the product as an uninstrumented subprocess"
                );
            }
            None
        } else {
            crate::cobertura::validate(&report)
                .context("the End-to-end coverage report is invalid")?;
            if report.lines_covered == 0 {
                bail!("the End-to-end coverage report measured no executed line");
            }
            Some(report)
        }
    };
    let end_to_end = match &end_to_end_report {
        Some(report) => coverage_numbers(report, "measured"),
        None => CoverageNumbers {
            status: "subprocess-uninstrumented",
            lines_covered: 0,
            lines_valid: 0,
            branches_covered: 0,
            branches_valid: 0,
        },
    };

    if let Some(report) = &end_to_end_report {
        crate::cobertura::assert_distinct(&component_report, report)
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
fn read_counts(path: &Path, label: &str, expected_scope: &str) -> Result<TrialRunnerResult> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read the {label} runner result '{}'", path.display()))?;
    let result: TrialRunnerResult = serde_json::from_str(&text).map_err(|error| {
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

/// Require the runner result to carry the declared scenario multiset exactly:
/// one identity per execution unit, duplicate titles preserved.
fn require_scenarios(tier: &super::contract::Tier, counts: &TrialRunnerResult) -> Result<()> {
    if tier.scenarios.is_empty() {
        return Ok(());
    }
    let mut declared_count: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for title in &tier.scenarios {
        *declared_count.entry(title.as_str()).or_default() += 1;
    }
    let mut observed_count: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for title in &counts.scenarios {
        *observed_count.entry(title.as_str()).or_default() += 1;
    }

    let mut missing: Vec<&str> = Vec::new();
    let mut extra: Vec<&str> = Vec::new();
    let titles: std::collections::BTreeSet<&str> = declared_count
        .keys()
        .chain(observed_count.keys())
        .copied()
        .collect();
    for title in titles {
        let declared = declared_count.get(title).copied().unwrap_or(0);
        let observed = observed_count.get(title).copied().unwrap_or(0);
        for _ in observed..declared {
            missing.push(title);
        }
        for _ in declared..observed {
            extra.push(title);
        }
    }
    if missing.is_empty() && extra.is_empty() {
        return Ok(());
    }
    let mut reasons = Vec::new();
    if !missing.is_empty() {
        let shown: Vec<&str> = missing.iter().take(10).copied().collect();
        let remainder = missing.len() - shown.len();
        let more = if remainder == 0 {
            String::new()
        } else {
            format!(" (and {remainder} more)")
        };
        reasons.push(format!(
            "does not name {} declared scenario unit(s): {}{more}",
            missing.len(),
            shown.join(", ")
        ));
    }
    if !extra.is_empty() {
        let shown: Vec<&str> = extra.iter().take(10).copied().collect();
        let remainder = extra.len() - shown.len();
        let more = if remainder == 0 {
            String::new()
        } else {
            format!(" (and {remainder} more)")
        };
        reasons.push(format!(
            "names {} title unit(s) outside the declared corpus: {}{more}",
            extra.len(),
            shown.join(", ")
        ));
    }
    bail!("the runner result {}", reasons.join("; "))
}

/// Require the End-to-end result to name the rebuilt product it drove.
fn require_product(counts: &TrialRunnerResult, product: &ProductBinding) -> Result<()> {
    let recorded = counts.product_sha256.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "the End-to-end runner result must carry product_sha256 for the rebuilt product"
        )
    })?;
    if recorded != product.sha256 {
        bail!(
            "the End-to-end runner result reports product_sha256 '{recorded}' but the rebuilt product is '{}'",
            product.sha256
        );
    }
    let recorded_path = counts.product.as_deref().ok_or_else(|| {
        anyhow::anyhow!("the End-to-end runner result must carry the product path it drove")
    })?;
    let expected = std::fs::canonicalize(&product.path).unwrap_or_else(|_| product.path.clone());
    let observed =
        std::fs::canonicalize(recorded_path).unwrap_or_else(|_| PathBuf::from(recorded_path));
    if observed != expected {
        bail!(
            "the End-to-end runner result drove '{}' but the rebuilt product is '{}'",
            observed.display(),
            expected.display()
        );
    }
    Ok(())
}

/// A passing tier report from its counts.
fn measured(tier: &super::contract::Tier, counts: &TrialRunnerResult) -> TierReport {
    with_counts(tier, counts, "passed")
}

/// A tier report with its measured counts and an explicit status.
fn with_counts(
    tier: &super::contract::Tier,
    counts: &TrialRunnerResult,
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
        scenarios: counts.scenarios.clone(),
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
        scenarios: Vec::new(),
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
        scenarios: Vec::new(),
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
        scenarios: Vec::new(),
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
        scenarios: Vec::new(),
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

/// Remove a previous raw result so a stale file cannot satisfy the gate. A
/// removal failure is itself a refusal; a warning is not fail-closed.
fn remove_previous(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(error).with_context(|| format!("remove the previous result '{}'", path.display()))
        }
    }
}

/// A command that is empty or is Givn's unconfigured sentinel.
fn not_runnable(command: &str) -> bool {
    let command = command.trim();
    command.is_empty() || command.starts_with("givn missing-")
}

#[cfg(unix)]
fn is_executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_metadata: &std::fs::Metadata) -> bool {
    false
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
            scenarios: vec![],
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
                    scenarios: vec![],
                },
                coverage: None,
                product: None,
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
    fn a_count_only_result_refuses_when_titles_are_declared() {
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("result.json");
        let script = directory.path().join("verify.sh");
        std::fs::write(
            &script,
            "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}\n' > \"$GIVN_RESULT_FILE\"\n",
        )
        .unwrap();
        let tier = super::super::contract::Tier {
            command: format!("sh {}", script.display()),
            features: "givn/specs".to_string(),
            execution_units: 1,
            scenarios: vec!["declared scenario".to_string()],
        };
        let failure = run_component(
            &Contract {
                scope: "ise-verification/v1".to_string(),
                revision: "abc".to_string(),
                component: tier,
                end_to_end: super::super::contract::Tier {
                    command: "true".to_string(),
                    features: "givn/specs".to_string(),
                    execution_units: 0,
                    scenarios: vec![],
                },
                coverage: None,
                product: None,
            },
            directory.path(),
            &path,
            None,
        )
        .expect_err("a count-only result refuses");
        assert!(
            failure.error.to_string().contains("does not name"),
            "{:?}",
            failure.error
        );
    }

    #[test]
    fn duplicate_and_outline_units_are_a_multiset() {
        let tier = super::super::contract::Tier {
            command: "true".to_string(),
            features: "givn/specs".to_string(),
            execution_units: 4,
            scenarios: vec![
                "dup".to_string(),
                "dup".to_string(),
                "outline".to_string(),
                "outline".to_string(),
            ],
        };
        let exact = TrialRunnerResult {
            total: 4,
            passed: 4,
            scenarios: vec![
                "dup".to_string(),
                "dup".to_string(),
                "outline".to_string(),
                "outline".to_string(),
            ],
            ..TrialRunnerResult::default()
        };
        require_scenarios(&tier, &exact).expect("the exact multiset passes");

        let collapsed = TrialRunnerResult {
            total: 2,
            passed: 2,
            scenarios: vec!["dup".to_string(), "outline".to_string()],
            ..TrialRunnerResult::default()
        };
        let error = require_scenarios(&tier, &collapsed).expect_err("a collapsed set refuses");
        assert!(
            error.to_string().contains("declared scenario unit"),
            "{error}"
        );
    }

    #[test]
    fn an_undeclared_extra_title_refuses() {
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("result.json");
        let script = directory.path().join("verify.sh");
        std::fs::write(
            &script,
            "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":2,\"passed\":2,\"failed\":0,\"skipped\":0,\"scenarios\":[\"declared\",\"foreign\"]}\n' > \"$GIVN_RESULT_FILE\"\n",
        )
        .unwrap();
        let tier = super::super::contract::Tier {
            command: format!("sh {}", script.display()),
            features: "givn/specs".to_string(),
            execution_units: 1,
            scenarios: vec!["declared".to_string()],
        };
        let failure = run_component(
            &Contract {
                scope: "ise-verification/v1".to_string(),
                revision: "abc".to_string(),
                component: tier,
                end_to_end: super::super::contract::Tier {
                    command: "true".to_string(),
                    features: "givn/specs".to_string(),
                    execution_units: 0,
                    scenarios: vec![],
                },
                coverage: None,
                product: None,
            },
            directory.path(),
            &path,
            None,
        )
        .expect_err("an extra title refuses");
        assert!(
            failure
                .error
                .to_string()
                .contains("outside the declared corpus"),
            "{:?}",
            failure.error
        );
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
            scenarios: vec![],
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
                    scenarios: vec![],
                },
                coverage: None,
                product: None,
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
                scenarios: vec![],
            },
            end_to_end: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 1,
                scenarios: vec![],
            },
            coverage: None,
            product: None,
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
            output_path: "coverage/output.xml".to_string(),
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
                scenarios: vec![],
            },
            end_to_end: super::super::contract::Tier {
                command: "true".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 1,
                scenarios: vec![],
            },
            coverage: Some(coverage),
            product: None,
        }
    }

    fn write_measure_script(path: &Path, component: &str, end_to_end: &str) {
        std::fs::write(
            path,
            format!(
                "#!/bin/sh\nset -eu\n: \"${{GIVN_COVERAGE_OUTPUT_PATH:?missing output path}}\"\nmkdir -p coverage\nprintf '%s' '{component}' > \"$GIVN_COVERAGE_NON_E2E_PATH\"\nprintf '%s' '{end_to_end}' > \"$GIVN_COVERAGE_E2E_PATH\"\n"
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
    fn coverage_accepts_a_missing_e2e_report_when_declared() {
        let directory = tempfile::TempDir::new().unwrap();
        let script = directory.path().join("measure.sh");
        std::fs::write(
            &script,
            "#!/bin/sh\nset -eu\n: \"${GIVN_COVERAGE_OUTPUT_PATH:?missing}\"\nmkdir -p coverage\nprintf '%s' '<?xml version=\"1.0\"?><coverage line-rate=\"0.5\" branch-rate=\"0\" lines-covered=\"5\" lines-valid=\"10\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\"><packages /></coverage>' > \"$GIVN_COVERAGE_NON_E2E_PATH\"\n",
        )
        .unwrap();
        let contract = contract_with_coverage(coverage_contract(&script, true));
        let coverage = run_coverage(
            contract.coverage.as_ref().unwrap(),
            directory.path(),
            &contract,
        )
        .expect("a missing e2e report is accepted with the declaration");
        assert_eq!(coverage.end_to_end.status, "subprocess-uninstrumented");
        assert!(!directory.path().join("coverage/e2e.xml").exists());
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
