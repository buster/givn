//! Archive proof: provenance, the runner-result contract, and the
//! verification receipt.
//!
//! `verification.json` is the durable record of an archive's state. Pending
//! receipts are committed with the candidate; proven receipts are committed
//! as the evidence-only finalization after every scope passed.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Versioned scope of the tested-content digest and documentation allowlist.
pub const CONTENT_SCOPE: &str = "change-proof/v1";

/// File name of the verification receipt inside `givn/archive/<id>/`.
pub const RECEIPT_FILE: &str = "verification.json";
/// Directory (relative to the archive directory) holding raw scope results.
pub const RESULTS_DIR: &str = "results";
/// Receipt status before every verification scope passed.
pub const STATUS_PENDING: &str = "pending";
/// Receipt status after every verification scope passed.
pub const STATUS_PROVEN: &str = "proven";
/// Verification scope covering the regular (non-e2e-filtered) suite.
pub const SCOPE_REGULAR: &str = "regular";
/// Verification scope covering `@e2e` scenarios.
pub const SCOPE_E2E: &str = "e2e";

/// Archive provenance derived from the receipt file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveProof {
    /// No archive directory exists.
    None,
    /// An archive directory without a receipt (pre-receipt publication).
    Legacy,
    /// A committed candidate that has not passed verification.
    Pending,
    /// An interrupted transition left an identical active change beside the
    /// pending candidate. The active duplicate is safe to remove.
    PendingWithActive,
    /// A verified and finalized archive.
    Proven,
    /// The archive state cannot be trusted. `both_dirs` tells the caller
    /// whether the active and archive directories both exist;
    /// `pending_conflict` tells it whether both exist with a pending receipt
    /// whose active artifacts do not match the candidate.
    Corrupt {
        both_dirs: bool,
        pending_conflict: bool,
    },
}

/// Read the archive receipt, if it exists and parses.
pub fn read_receipt(archive_dir: &Path) -> Option<Receipt> {
    let text = std::fs::read_to_string(archive_dir.join(RECEIPT_FILE)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Classify an archive by its receipt and, for a both-directory state, by
/// whether the active change's artifacts match the pending candidate.
pub fn read_status(project_root: &Path, id: &str) -> ArchiveProof {
    let active = project_root.join("givn/changes").join(id);
    let archive = project_root.join("givn/archive").join(id);
    if active.exists() && archive.exists() {
        return match read_receipt(&archive) {
            Some(receipt) if receipt.status == STATUS_PENDING => {
                if candidate_matches_active(project_root, id) {
                    ArchiveProof::PendingWithActive
                } else {
                    ArchiveProof::Corrupt {
                        both_dirs: true,
                        pending_conflict: true,
                    }
                }
            }
            _ => ArchiveProof::Corrupt {
                both_dirs: true,
                pending_conflict: false,
            },
        };
    }
    if !archive.exists() {
        return ArchiveProof::None;
    }
    match read_receipt(&archive) {
        Some(receipt) if receipt.status == STATUS_PENDING => ArchiveProof::Pending,
        Some(receipt) if receipt.status == STATUS_PROVEN => ArchiveProof::Proven,
        None if !archive.join(RECEIPT_FILE).exists() => ArchiveProof::Legacy,
        _ => ArchiveProof::Corrupt {
            both_dirs: false,
            pending_conflict: false,
        },
    }
}

/// Whether every active artifact exists byte-identically in the pending
/// archive candidate. Evidence paths are outputs and are ignored on both
/// sides; archive-only artifacts are allowed because the candidate is
/// authoritative and may carry evidence the active copy lost.
pub fn candidate_matches_active(project_root: &Path, id: &str) -> bool {
    let active = project_root.join("givn/changes").join(id);
    let archive = project_root.join("givn/archive").join(id);
    let mut stack = vec![(active.clone(), String::new())];
    while let Some((dir, prefix)) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return false;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let relative = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            if path.is_dir() {
                stack.push((path, relative));
                continue;
            }
            if is_candidate_evidence(&relative) {
                continue;
            }
            let counterpart = archive.join(&relative);
            let (Ok(left), Ok(right)) = (std::fs::read(&path), std::fs::read(&counterpart)) else {
                return false;
            };
            if left != right {
                return false;
            }
        }
    }
    true
}

fn is_candidate_evidence(relative: &str) -> bool {
    relative == RECEIPT_FILE
        || relative.starts_with("results/")
        || relative.starts_with("evidence/")
        || relative.starts_with("verification/")
}

/// A verification receipt (`verification.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub schema: u32,
    pub change: String,
    pub status: String,
    #[serde(default, alias = "candidate_commit")]
    pub candidate_revision: Option<String>,
    #[serde(default, alias = "tested_commit")]
    pub tested_revision: Option<String>,
    #[serde(default)]
    pub scopes: BTreeMap<String, ScopeOutcome>,
    #[serde(default)]
    pub coverage: Option<CoverageOutcome>,
    #[serde(default)]
    pub scope_evidence: BTreeMap<String, ScopeEvidence>,
    #[serde(default)]
    pub coverage_inputs: Option<VerificationInputs>,
    #[serde(default)]
    pub merge_completed: bool,
    #[serde(default)]
    pub merged_deltas: Vec<String>,
    #[serde(default)]
    pub merge_in_progress: Option<String>,
    #[serde(default)]
    pub merge_plan: Vec<String>,
    #[serde(default)]
    pub merge_targets: BTreeMap<String, String>,
    #[serde(default)]
    pub started_at_unix: Option<u64>,
    #[serde(default)]
    pub finished_at_unix: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeEvidence {
    pub inputs: VerificationInputs,
    pub result_digest: String,
}

impl Receipt {
    pub fn record_scope(
        archive_dir: &Path,
        scope: &str,
        outcome: ScopeOutcome,
        inputs: VerificationInputs,
        result_path: &Path,
    ) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).context("archive receipt missing")?;
        let result_digest = digest_text(&std::fs::read_to_string(result_path)?);
        receipt.scopes.insert(scope.to_owned(), outcome);
        receipt.scope_evidence.insert(
            scope.to_owned(),
            ScopeEvidence {
                inputs,
                result_digest,
            },
        );
        write_receipt(archive_dir, &receipt)
    }

    pub fn reusable_scope(
        &self,
        scope: &str,
        inputs: &VerificationInputs,
        result_path: &Path,
    ) -> bool {
        let Some(evidence) = self.scope_evidence.get(scope) else {
            return false;
        };
        self.scopes
            .get(scope)
            .is_some_and(|outcome| outcome.status == "passed")
            && evidence.inputs.reusable_for(inputs)
            && std::fs::read_to_string(result_path)
                .is_ok_and(|text| digest_text(&text) == evidence.result_digest)
            && RunnerResult::read(result_path, scope).is_ok()
    }

    pub fn record_coverage(
        archive_dir: &Path,
        outcome: CoverageOutcome,
        inputs: VerificationInputs,
    ) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).context("archive receipt missing")?;
        receipt.coverage = Some(outcome);
        receipt.coverage_inputs = Some(inputs);
        write_receipt(archive_dir, &receipt)
    }

    pub fn resume_pending(archive_dir: &Path) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).context("archive receipt missing")?;
        receipt.status = STATUS_PENDING.to_owned();
        write_receipt(archive_dir, &receipt)
    }
    /// Write the pending receipt committed with the candidate. It cannot name
    /// the candidate revision's own identifier, so both revision fields stay unset.
    #[cfg(test)]
    pub fn write_pending(archive_dir: &Path, change: &str, started_at_unix: u64) -> Result<()> {
        Self::write_pending_progress(
            archive_dir,
            change,
            started_at_unix,
            false,
            Vec::new(),
            None,
            Vec::new(),
            BTreeMap::new(),
        )
    }

    /// Write a pending receipt that carries the merge-progress state.
    #[allow(clippy::too_many_arguments)]
    pub fn write_pending_progress(
        archive_dir: &Path,
        change: &str,
        started_at_unix: u64,
        merge_completed: bool,
        merged_deltas: Vec<String>,
        merge_in_progress: Option<String>,
        merge_plan: Vec<String>,
        merge_targets: BTreeMap<String, String>,
    ) -> Result<()> {
        let receipt = Self {
            schema: 1,
            change: change.to_string(),
            status: STATUS_PENDING.to_string(),
            merge_completed,
            merged_deltas,
            merge_in_progress,
            merge_plan,
            merge_targets,
            candidate_revision: None,
            tested_revision: None,
            scopes: BTreeMap::new(),
            coverage: None,
            scope_evidence: BTreeMap::new(),
            coverage_inputs: None,
            started_at_unix: Some(started_at_unix),
            finished_at_unix: None,
        };
        write_receipt(archive_dir, &receipt)
    }

    /// Write the proven receipt. Callers must have already passed both
    /// finalization guards so a false `proven` state can never be written.
    #[allow(clippy::too_many_arguments)]
    pub fn write_proven(
        archive_dir: &Path,
        change: &str,
        tested_revision: &str,
        scopes: BTreeMap<String, ScopeOutcome>,
        coverage: Option<CoverageOutcome>,
        started_at_unix: u64,
        finished_at_unix: u64,
    ) -> Result<()> {
        let receipt = Self {
            schema: 1,
            change: change.to_string(),
            status: STATUS_PROVEN.to_string(),
            merge_completed: true,
            merged_deltas: Vec::new(),
            merge_in_progress: None,
            merge_plan: Vec::new(),
            merge_targets: BTreeMap::new(),
            candidate_revision: Some(tested_revision.to_string()),
            tested_revision: Some(tested_revision.to_string()),
            scopes,
            coverage,
            scope_evidence: read_receipt(archive_dir)
                .map(|r| r.scope_evidence)
                .unwrap_or_default(),
            coverage_inputs: read_receipt(archive_dir).and_then(|r| r.coverage_inputs),
            started_at_unix: Some(started_at_unix),
            finished_at_unix: Some(finished_at_unix),
        };
        write_receipt(archive_dir, &receipt)
    }

    /// Mark the delta file currently being applied. The receipt update is
    /// written before the permanent spec so a crash leaves a retry marker.
    pub fn mark_merge_started(archive_dir: &Path, relative: &str) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).ok_or_else(|| {
            anyhow::anyhow!(
                "archive blocked: pending receipt missing at {}",
                archive_dir.join(RECEIPT_FILE).display()
            )
        })?;
        if receipt.status != STATUS_PENDING {
            bail!("archive blocked: the archive receipt is not pending");
        }
        receipt.merge_in_progress = Some(relative.to_string());
        write_receipt(archive_dir, &receipt)
    }

    /// Record a completed delta file and clear the in-progress marker.
    pub fn mark_merge_file_done(archive_dir: &Path, relative: &str) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).ok_or_else(|| {
            anyhow::anyhow!(
                "archive blocked: pending receipt missing at {}",
                archive_dir.join(RECEIPT_FILE).display()
            )
        })?;
        if !receipt.merged_deltas.iter().any(|entry| entry == relative) {
            receipt.merged_deltas.push(relative.to_string());
        }
        receipt.merge_in_progress = None;
        write_receipt(archive_dir, &receipt)
    }

    /// Mark the delta merge complete.
    pub fn mark_merge_completed(archive_dir: &Path) -> Result<()> {
        let mut receipt = read_receipt(archive_dir).ok_or_else(|| {
            anyhow::anyhow!(
                "archive blocked: pending receipt missing at {}",
                archive_dir.join(RECEIPT_FILE).display()
            )
        })?;
        receipt.merge_completed = true;
        receipt.merge_in_progress = None;
        write_receipt(archive_dir, &receipt)
    }
}

fn write_receipt(archive_dir: &Path, receipt: &Receipt) -> Result<()> {
    std::fs::create_dir_all(archive_dir)
        .with_context(|| format!("create archive directory {}", archive_dir.display()))?;
    let path = archive_dir.join(RECEIPT_FILE);
    let temporary = archive_dir.join(format!(".{RECEIPT_FILE}.givn-tmp"));
    let json = serde_json::to_string_pretty(receipt)?;
    std::fs::write(&temporary, format!("{json}\n"))
        .with_context(|| format!("write verification receipt {}", temporary.display()))?;
    std::fs::rename(&temporary, &path)
        .with_context(|| format!("publish verification receipt {}", path.display()))
}

/// The outcome of one verification scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeOutcome {
    /// `passed` or `not_applicable`.
    pub status: String,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub passed: u64,
    #[serde(default)]
    pub failed: u64,
    #[serde(default)]
    pub skipped: u64,
    #[serde(default)]
    pub seconds: f64,
}

impl ScopeOutcome {
    pub fn passed(result: &RunnerResult, seconds: f64) -> Self {
        Self {
            status: "passed".to_string(),
            total: result.total,
            passed: result.passed,
            failed: result.failed,
            skipped: result.skipped,
            seconds,
        }
    }

    pub fn not_applicable() -> Self {
        Self {
            status: "not_applicable".to_string(),
            ..Self::default()
        }
    }

    /// Whether the scope records a green run with at least one scenario.
    pub fn is_green(&self) -> bool {
        self.status == "passed"
            && self.failed == 0
            && self.skipped == 0
            && self.total > 0
            && self.passed == self.total
            && self.passed + self.failed + self.skipped == self.total
    }
}

impl Default for ScopeOutcome {
    fn default() -> Self {
        Self {
            status: String::new(),
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            seconds: 0.0,
        }
    }
}

/// The coverage outcome recorded by a proven receipt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageOutcome {
    pub status: String,
    pub line_covered: u64,
    pub line_valid: u64,
    pub line_percent: u64,
}

impl CoverageOutcome {
    pub fn passed(report: &crate::cobertura::CoberturaReport) -> Self {
        let percent = (report.lines_covered * 100)
            .checked_div(report.lines_valid)
            .unwrap_or(0);
        Self {
            status: "passed".to_string(),
            line_covered: report.lines_covered,
            line_valid: report.lines_valid,
            line_percent: percent,
        }
    }
}

/// The machine-readable result a verification scope's runner writes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerResult {
    /// Optional: a runner that writes only counts is compatible. When
    /// present, archive validates it against the expected scope.
    #[serde(default)]
    pub scope: Option<String>,
    pub total: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
}

impl RunnerResult {
    /// Read and validate a scope result. Every failure is fail-closed and
    /// names the offending file.
    pub fn read(path: &Path, expected_scope: &str) -> Result<Self> {
        let result = Self::read_counts(path, &format!("the {expected_scope} scope"))?;
        if let Some(scope) = result.scope.as_deref()
            && scope != expected_scope
        {
            bail!(
                "archive blocked: the runner result at {} declares scope '{}' but '{}' was expected",
                path.display(),
                scope,
                expected_scope
            );
        }
        Ok(result)
    }

    /// Read a runner result without a required scope field.
    pub fn read_counts(path: &Path, label: &str) -> Result<Self> {
        if !path.exists() {
            bail!(
                "archive blocked: {label} produced no runner result at {}",
                path.display()
            );
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read runner result {}", path.display()))?;
        let result: Self = serde_json::from_str(&text).map_err(|error| {
            anyhow::anyhow!(
                "archive blocked: {label} runner result at {} is malformed: {error}",
                path.display()
            )
        })?;
        if result.total == 0 {
            bail!("archive blocked: {label} runner result declares zero scenarios");
        }
        if result.failed != 0 {
            bail!(
                "archive blocked: {label} runner result reports {} failed {}",
                result.failed,
                if result.failed == 1 { "test" } else { "tests" }
            );
        }
        if result.skipped != 0 {
            bail!(
                "archive blocked: {label} runner result reports {} skipped {}",
                result.skipped,
                if result.skipped == 1 { "test" } else { "tests" }
            );
        }
        if result.passed + result.failed + result.skipped != result.total {
            bail!(
                "archive blocked: {label} runner result counts do not reconcile \
                 (passed {} + failed {} + skipped {} != total {})",
                result.passed,
                result.failed,
                result.skipped,
                result.total
            );
        }
        Ok(result)
    }
}

/// The E2E identity a receipt binds: one digest per resolved `e2e` package,
/// the E2E runner command digest, and the Givn version that wrote it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct E2eIdentity {
    /// Package directory (or import-mode path) to the digest of its tracked
    /// tree.
    pub package_digests: BTreeMap<String, String>,
    pub runner_command_digest: String,
    pub givn_version: String,
}

/// The verification outcome a change-scoped run records in its receipt.
#[derive(Debug, Clone)]
pub struct ChangeVerification {
    pub result: RunnerResult,
    /// Absent when the change lock has `ise` off.
    pub e2e_identity: Option<E2eIdentity>,
    /// The E2E scope outcome; absent when the change lock has `ise` off.
    pub e2e_scope: Option<ScopeOutcome>,
}

/// The receipt `givn check tasks` writes after a green change-scoped run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeReceipt {
    pub schema: u32,
    pub change: String,
    pub status: String,
    pub head: String,
    pub features_path: String,
    pub features_digest: String,
    pub command_digest: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub tested_content_digest: Option<String>,
    #[serde(default)]
    pub content_scope: Option<String>,
    pub runner_exit: i32,
    pub result: RunnerResult,
    /// The E2E identity; absent on a receipt written with `ise` off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e2e_identity: Option<E2eIdentity>,
    /// The E2E scope outcome; absent on a receipt written with `ise` off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e2e_scope: Option<ScopeOutcome>,
    pub finished_at_unix: u64,
}

/// The shared freshness result used by review and archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofStatus {
    Current,
    DocumentationOnly {
        tested_revision: String,
        current_revision: String,
    },
    Stale(Vec<StaleReason>),
}

/// One independently checkable reason a change verification receipt is stale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaleReason {
    NotGreen,
    FeaturesChanged {
        receipt: String,
        current: String,
    },
    CommandChanged {
        receipt: String,
        current: String,
    },
    TestedContentChanged {
        receipt: String,
        current: String,
    },
    TestedRevisionUnavailable {
        tested_revision: String,
    },
    LegacyReceiptWithoutContentBinding {
        tested_revision: String,
    },
    UnsupportedContentScope {
        scope: String,
    },
    UnsupportedReceiptSchema {
        schema: u32,
    },
    MissingE2eIdentity,
    E2eIdentityUnavailable {
        reason: String,
    },
    E2ePackageChanged {
        package: String,
        receipt: String,
        current: String,
    },
    E2eRunnerCommandChanged {
        receipt: String,
        current: String,
    },
    GivnVersionChanged {
        receipt: String,
        current: String,
    },
    E2eScopeMissing,
    E2eScopeUnexpected,
}

impl StaleReason {
    pub fn render(&self) -> String {
        match self {
            Self::NotGreen => {
                "the receipt does not record a green run with at least one scenario".to_string()
            }
            Self::FeaturesChanged { receipt, current } => {
                format!("delta features changed: receipt {receipt}, current {current}")
            }
            Self::CommandChanged { receipt, current } => {
                format!("verify command changed: receipt {receipt}, current {current}")
            }
            Self::TestedContentChanged { receipt, current } => {
                format!("tested content changed: receipt {receipt}, current {current}")
            }
            Self::TestedRevisionUnavailable { tested_revision } => format!(
                "the tested revision {tested_revision} is not available from the current revision"
            ),
            Self::LegacyReceiptWithoutContentBinding { tested_revision } => format!(
                "the receipt is a legacy receipt without tested-content binding (tested revision {tested_revision}); run `givn check tasks` once"
            ),
            Self::UnsupportedContentScope { scope } => {
                format!("unsupported receipt content scope '{scope}'")
            }
            Self::UnsupportedReceiptSchema { schema } => {
                format!("unsupported change verification receipt schema {schema}")
            }
            Self::MissingE2eIdentity => {
                "the receipt lacks the E2E identity; run `givn check tasks` once".to_string()
            }
            Self::E2eIdentityUnavailable { reason } => {
                format!("the E2E identity cannot be read: {reason}")
            }
            Self::E2ePackageChanged {
                package,
                receipt,
                current,
            } => format!(
                "the e2e package digest for '{package}' changed: receipt {receipt}, current {current}"
            ),
            Self::E2eRunnerCommandChanged { receipt, current } => {
                format!("E2E runner command changed: receipt {receipt}, current {current}")
            }
            Self::GivnVersionChanged { receipt, current } => {
                format!("Givn version changed: receipt {receipt}, current {current}")
            }
            Self::E2eScopeMissing => "the E2E scope is missing: the delta has an @e2e scenario \
                 but the receipt records no green E2E scope; run `givn check tasks` once"
                .to_string(),
            Self::E2eScopeUnexpected => "the E2E scope is not applicable to this delta: the \
                 receipt records a passed E2E scope but the delta has no @e2e scenario; run \
                 `givn check tasks` once"
                .to_string(),
        }
    }
}

/// Path of the change-scoped E2E runner result.
pub const CHANGE_E2E_RESULT_FILE: &str = "verification/e2e-result.json";

/// Read the change's E2E scope result. The refusal names the failing E2E run
/// and the count the result reported, so an unbound step (a failed test) and a
/// skipped step (a skipped test) never share one generic message.
pub fn read_change_e2e_result(change_id: &str, path: &Path) -> Result<RunnerResult> {
    RunnerResult::read(path, SCOPE_E2E).map_err(|error| {
        anyhow::anyhow!(
            "the change E2E run failed: {error}; repair the change and rerun \
             `givn check tasks --change {change_id}`"
        )
    })
}

/// Path of the change-scoped verification receipt.
pub const CHANGE_RECEIPT_FILE: &str = "verification/tasks-verification.json";
/// Path of the raw change-scoped runner result.
pub const CHANGE_RESULT_FILE: &str = "verification/tasks-result.json";

pub fn remove_previous_output(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("remove stale output {}", path.display())),
    }
}

fn no_behavior_change_declared(change_dir: &Path) -> bool {
    let proposal = std::fs::read_to_string(change_dir.join("proposal.md")).unwrap_or_default();
    crate::state::actual_entries(&proposal, "Behavior change:")
        .iter()
        .any(|value| value == "none")
}

pub fn validate_no_behavior_change(change_dir: &Path) -> Result<()> {
    if !no_behavior_change_declared(change_dir) {
        return Ok(());
    }
    if !crate::gherkin::parse::collect_feature_files(&change_dir.join("specs")).is_empty() {
        bail!(
            "executable deltas contradict no behavior change; remove the declaration or delta feature files"
        );
    }
    if !crate::state::is_skipped("specs", change_dir) {
        bail!("Behavior change: none requires an explicit specs skip in .givn-skip");
    }
    Ok(())
}

pub fn change_verification_not_applicable(change_dir: &Path) -> Result<bool> {
    if no_behavior_change_declared(change_dir) {
        return Ok(true);
    }
    if crate::gherkin::parse::collect_feature_files(&change_dir.join("specs")).is_empty() {
        bail!("missing delta specs or explicit Behavior change: none declaration and specs skip");
    }
    Ok(false)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationInputs {
    pub revision: String,
    pub features_digest: String,
    pub command_digest: String,
    pub content_digest: String,
    pub scope: String,
}

impl VerificationInputs {
    pub fn reusable_for(&self, current: &Self) -> bool {
        self.features_digest == current.features_digest
            && self.command_digest == current.command_digest
            && self.content_digest == current.content_digest
            && self.scope == current.scope
    }

    pub fn capture_archive(
        project_root: &Path,
        backend: &crate::scm::Backend,
        change: &str,
        manifest: &crate::config::manifest::Manifest,
        command: &str,
        scope: &str,
    ) -> Result<Self> {
        Ok(Self {
            revision: backend.revision()?,
            features_digest: digest_features(&project_root.join("givn/specs"))?,
            command_digest: digest_text(command),
            content_digest: digest_content(project_root, backend, change, manifest, true)?,
            scope: scope.to_owned(),
        })
    }

    pub fn capture(
        project_root: &Path,
        backend: &crate::scm::Backend,
        change: &str,
        manifest: &crate::config::manifest::Manifest,
        features: &Path,
        command: &str,
        scope: &str,
    ) -> Result<Self> {
        Ok(Self {
            revision: backend.revision()?,
            features_digest: digest_features(features)?,
            command_digest: digest_text(command),
            content_digest: digest_tested_content(project_root, backend, change, manifest)?,
            scope: scope.to_owned(),
        })
    }

    pub fn ensure_unchanged(&self, after: &Self) -> Result<()> {
        if self != after {
            bail!(
                "tested inputs changed during execution — the tested tree changed outside the declared \
                 evidence; no passing verification evidence was issued"
            );
        }
        Ok(())
    }
}

pub fn digest_text(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Digest the selected feature files by sorted relative path and content.
pub fn digest_features(dir: &Path) -> Result<String> {
    let mut files = Vec::new();
    crate::gherkin::parse::collect_feature_files(dir)
        .into_iter()
        .for_each(|path| files.push(path));
    files.sort();
    let mut hasher = Sha256::new();
    for path in files {
        let relative = path.strip_prefix(dir).unwrap_or(&path);
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update(b"\0");
        hasher.update(std::fs::read(&path)?);
        hasher.update(b"\0");
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

impl ChangeReceipt {
    pub fn read(change_dir: &Path) -> Result<Self> {
        let path = change_dir.join(CHANGE_RECEIPT_FILE);
        let text = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "change verification receipt missing at {}; run `givn check tasks --change <id>`",
                path.display()
            )
        })?;
        serde_json::from_str(&text).with_context(|| {
            format!(
                "change verification receipt at {} is malformed; run `givn check tasks --change <id>`",
                path.display()
            )
        })
    }

    /// The shared review/archive freshness rule. Every independently
    /// checkable stale reason is collected.
    pub fn freshness(
        &self,
        manifest: &crate::config::manifest::Manifest,
        change_dir: &Path,
        backend: &crate::scm::Backend,
        project_root: &Path,
    ) -> Result<ProofStatus> {
        if self.schema != 1 && self.schema != 2 {
            return Ok(ProofStatus::Stale(vec![
                StaleReason::UnsupportedReceiptSchema {
                    schema: self.schema,
                },
            ]));
        }

        let current_revision = backend.revision()?;
        let mut reasons = Vec::new();
        let green = self.status == "passed"
            && self.runner_exit == 0
            && self.result.failed == 0
            && self.result.skipped == 0
            && self.result.total > 0
            && self.result.passed == self.result.total
            && self.result.passed + self.result.failed + self.result.skipped == self.result.total;
        if !green {
            reasons.push(StaleReason::NotGreen);
        }

        let current_features = digest_features(&change_dir.join("specs"))?;
        if self.features_digest != current_features {
            reasons.push(StaleReason::FeaturesChanged {
                receipt: self.features_digest.clone(),
                current: current_features,
            });
        }
        let current_command = digest_text(&manifest.verify.command);
        if self.command_digest != current_command {
            reasons.push(StaleReason::CommandChanged {
                receipt: self.command_digest.clone(),
                current: current_command,
            });
        }

        let mut documentation_only = false;
        match self.schema {
            1 => {
                if self.head != current_revision {
                    reasons.push(StaleReason::LegacyReceiptWithoutContentBinding {
                        tested_revision: self.head.clone(),
                    });
                }
            }
            2 => match self.content_scope.as_deref() {
                Some(scope) if scope != CONTENT_SCOPE => {
                    reasons.push(StaleReason::UnsupportedContentScope {
                        scope: scope.to_string(),
                    });
                }
                Some(CONTENT_SCOPE) => match &self.tested_content_digest {
                    None => {
                        if self.head != current_revision {
                            reasons.push(StaleReason::LegacyReceiptWithoutContentBinding {
                                tested_revision: self.head.clone(),
                            });
                        }
                    }
                    Some(receipt_digest) => {
                        let current_digest =
                            digest_tested_content(project_root, backend, &self.change, manifest)?;
                        if receipt_digest != &current_digest {
                            reasons.push(StaleReason::TestedContentChanged {
                                receipt: receipt_digest.clone(),
                                current: current_digest,
                            });
                        } else if self.head != current_revision {
                            if backend.is_ancestor(&self.head)? {
                                documentation_only = true;
                            } else {
                                reasons.push(StaleReason::TestedRevisionUnavailable {
                                    tested_revision: self.head.clone(),
                                });
                            }
                        }
                    }
                },
                _ => {
                    if self.head != current_revision {
                        reasons.push(StaleReason::LegacyReceiptWithoutContentBinding {
                            tested_revision: self.head.clone(),
                        });
                    }
                }
            },
            _ => unreachable!("schema was checked above"),
        }

        if crate::config::lock::ise_effective_for_change(change_dir)? {
            reasons.extend(self.e2e_stale_reasons(change_dir, project_root, manifest)?);
        }

        if !reasons.is_empty() {
            Ok(ProofStatus::Stale(reasons))
        } else if documentation_only {
            Ok(ProofStatus::DocumentationOnly {
                tested_revision: self.head.clone(),
                current_revision,
            })
        } else {
            Ok(ProofStatus::Current)
        }
    }

    /// The E2E identity and delta-aware scope reasons an `ise`-effective
    /// change adds to the shared freshness rule. Every independent reason is
    /// collected; only a receipt without the identity stops the comparison.
    fn e2e_stale_reasons(
        &self,
        change_dir: &Path,
        project_root: &Path,
        manifest: &crate::config::manifest::Manifest,
    ) -> Result<Vec<StaleReason>> {
        let mut reasons = Vec::new();

        let delta_has_e2e = crate::gherkin::parse::has_e2e_scenarios(&change_dir.join("specs"));
        match self.e2e_scope.as_ref() {
            Some(scope) if delta_has_e2e && !scope.is_green() => {
                reasons.push(StaleReason::E2eScopeMissing);
            }
            None if delta_has_e2e => reasons.push(StaleReason::E2eScopeMissing),
            Some(scope) if !delta_has_e2e && scope.status == "passed" => {
                reasons.push(StaleReason::E2eScopeUnexpected);
            }
            _ => {}
        }

        let Some(identity) = self.e2e_identity.as_ref() else {
            reasons.push(StaleReason::MissingE2eIdentity);
            return Ok(reasons);
        };

        let current_runner = digest_text(&manifest.verify.e2e_command);
        if identity.runner_command_digest != current_runner {
            reasons.push(StaleReason::E2eRunnerCommandChanged {
                receipt: identity.runner_command_digest.clone(),
                current: current_runner,
            });
        }
        let current_version = env!("CARGO_PKG_VERSION");
        if identity.givn_version != current_version {
            reasons.push(StaleReason::GivnVersionChanged {
                receipt: identity.givn_version.clone(),
                current: current_version.to_string(),
            });
        }

        let declaration =
            match crate::config::declaration::SpecificationDeclaration::load(project_root) {
                Ok(Some(declaration)) => declaration,
                Ok(None) => {
                    reasons.push(StaleReason::E2eIdentityUnavailable {
                        reason: format!(
                            "'{}' is absent",
                            crate::config::declaration::DECLARATION_PATH
                        ),
                    });
                    return Ok(reasons);
                }
                Err(error) => {
                    reasons.push(StaleReason::E2eIdentityUnavailable {
                        reason: error.to_string(),
                    });
                    return Ok(reasons);
                }
            };
        let current = crate::e2e_boundary::E2eBoundary::resolve(project_root, &declaration)
            .identity(project_root)?;
        for (package, recorded) in &identity.package_digests {
            match current.get(package) {
                Some(digest) if digest == recorded => {}
                Some(digest) => reasons.push(StaleReason::E2ePackageChanged {
                    package: package.clone(),
                    receipt: recorded.clone(),
                    current: digest.clone(),
                }),
                None => reasons.push(StaleReason::E2ePackageChanged {
                    package: package.clone(),
                    receipt: recorded.clone(),
                    current: "(absent)".to_string(),
                }),
            }
        }
        for (package, digest) in &current {
            if !identity.package_digests.contains_key(package) {
                reasons.push(StaleReason::E2ePackageChanged {
                    package: package.clone(),
                    receipt: "(absent)".to_string(),
                    current: digest.clone(),
                });
            }
        }

        Ok(reasons)
    }

    /// Strict revision equality used by older callers and tests.
    #[allow(dead_code)]
    pub fn is_current(&self, change_dir: &Path, head: &str, command: &str) -> Result<bool> {
        let features = change_dir.join("specs");
        Ok(self.status == "passed"
            && self.head == head
            && self.features_digest == digest_features(&features)?
            && self.command_digest == digest_text(command)
            && self.result.failed == 0
            && self.result.skipped == 0
            && self.result.total > 0)
    }
}

/// Whether a path is documentation or evidence for the tested-content
/// boundary. The change-scoped evidence rules mirror `is_evidence_path`, but
/// README is always documentation and configured coverage outputs are not.
pub fn is_documentation_or_evidence_path(
    project_root: &Path,
    change: &str,
    manifest: &crate::config::manifest::Manifest,
    path: &Path,
) -> bool {
    let relative = path.strip_prefix(project_root).unwrap_or(path);
    let normalized = relative.to_string_lossy().replace('\\', "/");
    let normalized = normalized.trim_start_matches("./");
    let change_root = format!("givn/changes/{change}");

    if let Some(name) = normalized.strip_prefix(&format!("{change_root}/"))
        && !name.contains('/')
        && name.ends_with(".md")
    {
        return true;
    }
    if normalized == format!("{change_root}/verification.json")
        || normalized.starts_with(&format!("{change_root}/evidence/"))
        || normalized.starts_with(&format!("{change_root}/verification/"))
        || normalized.starts_with(&format!("{change_root}/results/"))
    {
        return is_evidence_path(project_root, change, manifest, path);
    }
    if normalized.starts_with(&format!("{change_root}/specs/"))
        && (normalized.ends_with("/usecase.md") || normalized.ends_with("/fragment.md"))
    {
        return true;
    }
    if normalized == "givn/specs/fragments/fragment.md"
        || (normalized.starts_with("givn/specs/") && normalized.ends_with("/usecase.md"))
    {
        return true;
    }
    if normalized.starts_with("givn/personas/")
        || normalized.starts_with("givn/archive/")
        || normalized.starts_with("givn/history/")
    {
        return true;
    }
    if normalized == "givn/.spec-index.json"
        || (normalized.starts_with("givn/.spec-") && normalized.ends_with("-cache.json"))
    {
        return true;
    }
    if normalized.starts_with("docs/arc42/") || normalized == "README.md" {
        return true;
    }
    false
}

/// Digest every tested content path. Documentation/evidence paths are
/// excluded; the path, executable marker, and bytes (or gitlink id / symlink
/// target) are hashed in sorted order.
pub fn digest_tested_content(
    project_root: &Path,
    backend: &crate::scm::Backend,
    change: &str,
    manifest: &crate::config::manifest::Manifest,
) -> Result<String> {
    digest_content(project_root, backend, change, manifest, false)
}

fn digest_content(
    project_root: &Path,
    backend: &crate::scm::Backend,
    change: &str,
    manifest: &crate::config::manifest::Manifest,
    archive: bool,
) -> Result<String> {
    let mut entries = backend.content_paths()?;
    entries.retain(|entry| {
        !is_documentation_or_evidence_path(project_root, change, manifest, &entry.path)
            && !(archive
                && (entry.path == Path::new("givn/commands.yaml")
                    || is_evidence_path(project_root, change, manifest, &entry.path)))
    });
    entries.sort_by(|left, right| left.path.cmp(&right.path));

    let mut hasher = Sha256::new();
    if archive {
        let mut settings = serde_json::to_value(manifest)?;
        if let Some(verify) = settings.get_mut("verify").and_then(|v| v.as_object_mut()) {
            verify.remove("command");
            verify.remove("e2e_command");
        }
        hasher.update(serde_json::to_vec(&settings)?);
    }
    for entry in entries {
        let relative = entry.path.to_string_lossy().replace('\\', "/");
        crate::e2e_boundary::hash_entry(
            &mut hasher,
            project_root,
            &relative,
            entry.gitlink.as_deref(),
        )?;
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

/// Write a change-scoped receipt atomically.
#[allow(clippy::too_many_arguments)]
pub fn write_change_receipt(
    project_root: &Path,
    change_dir: &Path,
    change: &str,
    head: &str,
    command: &str,
    tested_content_digest: &str,
    verification: ChangeVerification,
    finished_at_unix: u64,
) -> Result<()> {
    let features = change_dir.join("specs");
    let features_path = features
        .strip_prefix(project_root)
        .unwrap_or(&features)
        .to_string_lossy()
        .replace('\\', "/");
    let receipt = ChangeReceipt {
        schema: 2,
        change: change.to_string(),
        status: "passed".to_string(),
        head: head.to_string(),
        features_path,
        features_digest: digest_features(&features)?,
        command_digest: digest_text(command),
        command: Some(command.to_string()),
        tested_content_digest: Some(tested_content_digest.to_string()),
        content_scope: Some(CONTENT_SCOPE.to_string()),
        runner_exit: 0,
        result: verification.result,
        e2e_identity: verification.e2e_identity,
        e2e_scope: verification.e2e_scope,
        finished_at_unix,
    };
    let dir = change_dir.join("verification");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("tasks-verification.json");
    let temporary = dir.join(format!(
        ".tasks-verification.json.{}.givn-tmp",
        std::process::id()
    ));
    let json = format!("{}\n", serde_json::to_string_pretty(&receipt)?);
    std::fs::write(&temporary, json)?;
    std::fs::rename(&temporary, &path)?;
    Ok(())
}

/// The single allowed-path predicate shared by the clean-state witness and
/// the finalization guard. Evidence paths are outputs, not inputs: they may
/// be dirty before and after verification without changing the tested tree.
pub fn is_evidence_path(
    project_root: &Path,
    id: &str,
    manifest: &crate::config::manifest::Manifest,
    path: &Path,
) -> bool {
    let relative = path.strip_prefix(project_root).unwrap_or(path);
    let normalized = relative.to_string_lossy().replace('\\', "/");
    let normalized = normalized.trim_start_matches("./");

    if normalized == format!("givn/archive/{id}")
        || normalized.starts_with(&format!("givn/archive/{id}/"))
    {
        return true;
    }
    // Evidence produced by another archive's verification is an output, not
    // an input to this change's tested tree. Browser screenshot replays can
    // rewrite it while the current archive runs.
    if let Some(rest) = normalized.strip_prefix("givn/archive/")
        && let Some((_other, tail)) = rest.split_once('/')
        && (tail == RECEIPT_FILE
            || tail.starts_with("results/")
            || tail.starts_with("evidence/")
            || tail.starts_with("verification/"))
    {
        return true;
    }
    let change_root = format!("givn/changes/{id}");
    if normalized == format!("{change_root}/verification.json")
        || normalized.starts_with(&format!("{change_root}/evidence/"))
        || normalized.starts_with(&format!("{change_root}/verification/"))
    {
        return true;
    }
    if manifest.addons.readme && normalized == "README.md" {
        return true;
    }
    if manifest.addons.coverage {
        for candidate in [
            &manifest.coverage.non_e2e_path,
            &manifest.coverage.e2e_path,
            &manifest.coverage.output_path,
        ] {
            if normalized == candidate.trim_start_matches("./") {
                return true;
            }
        }
    }
    false
}

/// Current Unix time in seconds.
pub fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

/// Path of a scope result file inside the archive directory.
pub fn result_path(archive_dir: &Path, scope: &str) -> PathBuf {
    archive_dir.join(RESULTS_DIR).join(format!("{scope}.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::manifest::Manifest;

    #[test]
    fn archive_exclusive_commands_and_shared_inputs_have_distinct_witnesses() {
        let (root, backend) = git_fixture();
        let mut manifest = Manifest::load_embedded().unwrap();
        std::fs::create_dir_all(root.path().join("givn")).unwrap();
        std::fs::write(root.path().join("givn/commands.yaml"), "first").unwrap();
        let capture = |manifest: &Manifest, scope: &str| {
            VerificationInputs::capture_archive(
                root.path(),
                &backend,
                "demo",
                manifest,
                if scope == "regular" {
                    &manifest.verify.command
                } else {
                    &manifest.verify.e2e_command
                },
                scope,
            )
            .unwrap()
        };
        let regular = capture(&manifest, "regular");
        let e2e = capture(&manifest, "e2e");
        manifest.verify.e2e_command = "repaired e2e runner".into();
        std::fs::write(root.path().join("givn/commands.yaml"), "reformatted").unwrap();
        assert!(regular.reusable_for(&capture(&manifest, "regular")));
        assert!(!e2e.reusable_for(&capture(&manifest, "e2e")));
        std::fs::write(root.path().join("src/lib.rs"), "changed source").unwrap();
        assert!(!regular.reusable_for(&capture(&manifest, "regular")));
        let before = capture(&manifest, "regular");
        manifest.verify.features_path = "other features".into();
        assert!(!before.reusable_for(&capture(&manifest, "regular")));
    }

    #[test]
    fn stable_witness_rejects_changes_in_each_input() {
        let (root, backend) = git_fixture();
        let mut manifest = Manifest::load_embedded().unwrap();
        let features = root.path().join("givn/changes/demo/specs");
        std::fs::create_dir_all(&features).unwrap();
        std::fs::write(features.join("demo.feature"), "Feature: Demo\n").unwrap();
        let capture = |m: &Manifest| {
            VerificationInputs::capture(
                root.path(),
                &backend,
                "demo",
                m,
                &features,
                &m.verify.command,
                "change",
            )
            .unwrap()
        };
        let before = capture(&manifest);
        assert!(before.ensure_unchanged(&before).is_ok());
        manifest.verify.command = "changed runner".into();
        assert!(before.ensure_unchanged(&capture(&manifest)).is_err());
        let before = capture(&manifest);
        std::fs::write(features.join("demo.feature"), "Feature: Changed\n").unwrap();
        assert!(before.ensure_unchanged(&capture(&manifest)).is_err());
        let before = capture(&manifest);
        std::fs::write(root.path().join("src/lib.rs"), "changed source").unwrap();
        assert!(before.ensure_unchanged(&capture(&manifest)).is_err());
        let before = capture(&manifest);
        commit_git(root.path(), "repair");
        assert!(before.ensure_unchanged(&capture(&manifest)).is_err());
    }

    #[test]
    fn no_behavior_change_requires_both_skip_and_absent_features() {
        let root = tempfile::TempDir::new().unwrap();
        assert!(change_verification_not_applicable(root.path()).is_err());
        std::fs::write(root.path().join("proposal.md"), "Behavior change: none").unwrap();
        assert!(change_verification_not_applicable(root.path()).unwrap());
        assert!(validate_no_behavior_change(root.path()).is_err());
        std::fs::write(root.path().join(".givn-skip"), "specs\n").unwrap();
        assert!(change_verification_not_applicable(root.path()).unwrap());
        assert!(validate_no_behavior_change(root.path()).is_ok());
        std::fs::create_dir_all(root.path().join("specs")).unwrap();
        std::fs::write(root.path().join("specs/demo.feature"), "Feature: Demo\n").unwrap();
        assert!(validate_no_behavior_change(root.path()).is_err());
    }

    #[test]
    fn archive_evidence_survives_finalization_and_checks_result_bytes() {
        let root = tempfile::TempDir::new().unwrap();
        Receipt::write_pending(root.path(), "demo", 1).unwrap();
        let path = result_path(root.path(), "regular");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            r#"{"scope":"regular","total":1,"passed":1,"failed":0,"skipped":0}"#,
        )
        .unwrap();
        let inputs = VerificationInputs {
            revision: "a".into(),
            features_digest: "f".into(),
            command_digest: "c".into(),
            content_digest: "s".into(),
            scope: "regular".into(),
        };
        Receipt::record_scope(
            root.path(),
            "regular",
            ScopeOutcome::passed(&RunnerResult::read(&path, "regular").unwrap(), 1.0),
            inputs.clone(),
            &path,
        )
        .unwrap();
        let receipt = read_receipt(root.path()).unwrap();
        assert!(receipt.reusable_scope("regular", &inputs, &path));
        let mut repaired = inputs.clone();
        repaired.revision = "b".into();
        assert!(receipt.reusable_scope("regular", &repaired, &path));
        repaired.content_digest = "changed".into();
        assert!(!receipt.reusable_scope("regular", &repaired, &path));
        Receipt::write_proven(root.path(), "demo", "a", receipt.scopes, None, 1, 2).unwrap();
        assert!(
            read_receipt(root.path())
                .unwrap()
                .reusable_scope("regular", &inputs, &path)
        );
        std::fs::write(&path, "tampered").unwrap();
        assert!(
            !read_receipt(root.path())
                .unwrap()
                .reusable_scope("regular", &inputs, &path)
        );
    }

    fn temp_receipt_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().unwrap()
    }

    #[test]
    fn read_status_classifies_provenance() {
        let root = tempfile::TempDir::new().unwrap();
        let id = "demo";
        assert_eq!(read_status(root.path(), id), ArchiveProof::None);

        let archive = root.path().join("givn/archive/demo");
        std::fs::create_dir_all(&archive).unwrap();
        assert_eq!(read_status(root.path(), id), ArchiveProof::Legacy);

        Receipt::write_pending(&archive, id, 1).unwrap();
        assert_eq!(read_status(root.path(), id), ArchiveProof::Pending);

        Receipt::write_proven(&archive, id, "abc", BTreeMap::new(), None, 1, 2).unwrap();
        assert_eq!(read_status(root.path(), id), ArchiveProof::Proven);

        let active = root.path().join("givn/changes/demo");
        std::fs::create_dir_all(&active).unwrap();
        std::fs::write(active.join("proposal.md"), "# proposal\n").unwrap();
        std::fs::remove_file(archive.join(RECEIPT_FILE)).unwrap();
        assert_eq!(
            read_status(root.path(), id),
            ArchiveProof::Corrupt {
                both_dirs: true,
                pending_conflict: false,
            }
        );

        Receipt::write_pending(&archive, id, 1).unwrap();
        assert_eq!(
            read_status(root.path(), id),
            ArchiveProof::Corrupt {
                both_dirs: true,
                pending_conflict: true,
            }
        );

        std::fs::write(archive.join("proposal.md"), "# proposal\n").unwrap();
        assert_eq!(
            read_status(root.path(), id),
            ArchiveProof::PendingWithActive
        );
    }

    #[test]
    fn candidate_comparison_is_an_active_subset_with_evidence_ignored() {
        let root = tempfile::TempDir::new().unwrap();
        let id = "demo";
        let active = root.path().join("givn/changes/demo");
        let archive = root.path().join("givn/archive/demo");
        std::fs::create_dir_all(active.join("evidence")).unwrap();
        std::fs::create_dir_all(archive.join("results")).unwrap();
        std::fs::write(active.join("proposal.md"), "same\n").unwrap();
        std::fs::write(archive.join("proposal.md"), "same\n").unwrap();
        std::fs::write(active.join("evidence/run.log"), "active-only\n").unwrap();
        std::fs::write(archive.join("verification.json"), "{}\n").unwrap();
        std::fs::write(archive.join("results/regular.json"), "{}\n").unwrap();
        assert!(candidate_matches_active(root.path(), id));

        std::fs::write(active.join("proposal.md"), "changed\n").unwrap();
        assert!(!candidate_matches_active(root.path(), id));
    }

    #[test]
    fn candidate_comparison_rejects_an_active_only_artifact() {
        let root = tempfile::TempDir::new().unwrap();
        let id = "demo";
        let active = root.path().join("givn/changes/demo");
        let archive = root.path().join("givn/archive/demo");
        std::fs::create_dir_all(&active).unwrap();
        std::fs::create_dir_all(&archive).unwrap();
        std::fs::write(active.join("proposal.md"), "same\n").unwrap();
        std::fs::write(archive.join("proposal.md"), "same\n").unwrap();
        std::fs::write(active.join("extra.md"), "active-only artifact\n").unwrap();
        assert!(!candidate_matches_active(root.path(), id));
    }

    #[test]
    fn candidate_comparison_ignores_receipts_on_both_sides() {
        let root = tempfile::TempDir::new().unwrap();
        let id = "demo";
        let active = root.path().join("givn/changes/demo");
        let archive = root.path().join("givn/archive/demo");
        std::fs::create_dir_all(&active).unwrap();
        std::fs::create_dir_all(&archive).unwrap();
        std::fs::write(active.join("proposal.md"), "same\n").unwrap();
        std::fs::write(archive.join("proposal.md"), "same\n").unwrap();
        std::fs::write(active.join("verification.json"), "active receipt\n").unwrap();
        std::fs::write(archive.join("verification.json"), "archive receipt\n").unwrap();
        assert!(candidate_matches_active(root.path(), id));
    }

    #[test]
    fn receipt_progress_roundtrip_preserves_plan_targets_and_in_progress() {
        let dir = tempfile::TempDir::new().unwrap();
        let archive = dir.path().join("archive");
        let mut targets = BTreeMap::new();
        targets.insert(
            "cap/file.feature".to_string(),
            "givn/specs/group/cap.feature".to_string(),
        );
        Receipt::write_pending_progress(
            &archive,
            "demo",
            7,
            false,
            Vec::new(),
            None,
            vec!["cap/file.feature".to_string()],
            targets,
        )
        .unwrap();
        let receipt = read_receipt(&archive).unwrap();
        assert_eq!(receipt.started_at_unix, Some(7));
        assert!(!receipt.merge_completed);
        assert_eq!(receipt.merge_plan, vec!["cap/file.feature"]);
        assert_eq!(
            receipt
                .merge_targets
                .get("cap/file.feature")
                .map(String::as_str),
            Some("givn/specs/group/cap.feature")
        );

        Receipt::mark_merge_started(&archive, "cap/file.feature").unwrap();
        assert_eq!(
            read_receipt(&archive).unwrap().merge_in_progress.as_deref(),
            Some("cap/file.feature")
        );
        Receipt::mark_merge_file_done(&archive, "cap/file.feature").unwrap();
        let receipt = read_receipt(&archive).unwrap();
        assert_eq!(receipt.merged_deltas, vec!["cap/file.feature"]);
        assert!(receipt.merge_in_progress.is_none());
        Receipt::mark_merge_completed(&archive).unwrap();
        assert!(read_receipt(&archive).unwrap().merge_completed);
    }

    #[test]
    fn evidence_paths_include_active_change_evidence_only_with_separator() {
        let manifest = Manifest::load_embedded().unwrap();
        let root = Path::new("/project");
        for path in [
            "/project/givn/changes/demo/verification.json",
            "givn/changes/demo/evidence/run.log",
            "givn/changes/demo/evidence/cli/session.txt",
            "givn/changes/demo/verification/notes.txt",
        ] {
            assert!(
                is_evidence_path(root, "demo", &manifest, Path::new(path)),
                "{path} should be evidence"
            );
        }
        assert!(!is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/changes/demo/evidence-other/run.log")
        ));
        assert!(!is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/changes/demo/proposal.md")
        ));
        assert!(is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/archive/demo-other/verification.json")
        ));
        assert!(is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/archive/demo-other/evidence/visual/run.png")
        ));
        assert!(!is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/archive/demo-other/design.md")
        ));
    }

    #[test]
    fn write_receipt_is_atomic_and_readable() {
        let dir = tempfile::TempDir::new().unwrap();
        let archive = dir.path().join("archive");
        Receipt::write_pending(&archive, "demo", 1).unwrap();
        assert!(read_receipt(&archive).is_some());
        assert!(!archive.join(format!(".{RECEIPT_FILE}.givn-tmp")).exists());
    }

    #[test]
    fn proven_receipt_names_the_tested_revision() {
        let dir = tempfile::TempDir::new().unwrap();
        let archive = dir.path().join("archive");
        Receipt::write_proven(&archive, "demo", "abc123", BTreeMap::new(), None, 1, 2).unwrap();
        let receipt = read_receipt(&archive).unwrap();
        assert_eq!(receipt.status, STATUS_PROVEN);
        assert_eq!(receipt.tested_revision.as_deref(), Some("abc123"));
        assert!(receipt.merge_completed);
    }

    #[test]
    fn change_receipt_rejects_a_changed_command() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();
        let change = root.join("givn/changes/demo");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        write_change_receipt(
            root,
            &change,
            "demo",
            "abc",
            "runner",
            "digest",
            ChangeVerification {
                result: RunnerResult {
                    scope: None,
                    total: 1,
                    passed: 1,
                    failed: 0,
                    skipped: 0,
                },
                e2e_identity: None,
                e2e_scope: None,
            },
            1,
        )
        .unwrap();
        let receipt = ChangeReceipt::read(&change).unwrap();
        assert!(!receipt.is_current(&change, "abc", "other-runner").unwrap());
    }

    #[test]
    fn change_receipt_read_names_the_missing_receipt() {
        let dir = tempfile::TempDir::new().unwrap();
        let err = ChangeReceipt::read(dir.path()).unwrap_err();
        assert!(
            err.to_string().contains("verification receipt missing"),
            "{err}"
        );
    }

    #[test]
    fn change_receipt_is_current_and_roundtrips() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();
        let change = root.join("givn/changes/demo");
        let specs = change.join("specs");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::write(specs.join("demo.feature"), "Feature: Demo\n").unwrap();
        write_change_receipt(
            root,
            &change,
            "demo",
            "abc",
            "runner",
            "digest",
            ChangeVerification {
                result: RunnerResult {
                    scope: None,
                    total: 2,
                    passed: 2,
                    failed: 0,
                    skipped: 0,
                },
                e2e_identity: None,
                e2e_scope: None,
            },
            1,
        )
        .unwrap();
        let receipt = ChangeReceipt::read(&change).unwrap();
        assert!(receipt.is_current(&change, "abc", "runner").unwrap());
        assert!(!receipt.is_current(&change, "def", "runner").unwrap());
        std::fs::write(specs.join("demo.feature"), "Feature: Changed\n").unwrap();
        assert!(!receipt.is_current(&change, "abc", "runner").unwrap());
    }

    fn git_fixture() -> (tempfile::TempDir, crate::scm::Backend) {
        use std::process::Command;
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();
        let run = |args: &[&str]| {
            let output = Command::new("git")
                .args(args)
                .current_dir(root)
                .env("GIT_AUTHOR_NAME", "fixture")
                .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
                .env("GIT_COMMITTER_NAME", "fixture")
                .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
                .output()
                .expect("run git");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        run(&["init", "-q"]);
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/lib.rs"), "pub fn fixture() -> u32 { 1 }\n").unwrap();
        std::fs::write(root.join("README.md"), "fixture readme\n").unwrap();
        run(&["add", "."]);
        run(&["commit", "-q", "-m", "fixture"]);
        let backend = crate::scm::select(root).expect("select the git fixture");
        (dir, backend)
    }

    fn commit_git(root: &Path, message: &str) {
        use std::process::Command;
        for args in [vec!["add", "-A"], vec!["commit", "-q", "-m", message]] {
            let output = Command::new("git")
                .args(&args)
                .current_dir(root)
                .env("GIT_AUTHOR_NAME", "fixture")
                .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
                .env("GIT_COMMITTER_NAME", "fixture")
                .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
                .output()
                .expect("run git");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn manifest() -> crate::config::manifest::Manifest {
        crate::config::manifest::Manifest::load_embedded().unwrap()
    }

    #[test]
    fn tested_content_digest_ignores_documentation_and_tracks_source() {
        let (dir, backend) = git_fixture();
        let root = dir.path();
        let manifest = manifest();
        let before = digest_tested_content(root, &backend, "demo", &manifest).unwrap();
        std::fs::write(root.join("README.md"), "changed readme\n").unwrap();
        let after_docs = digest_tested_content(root, &backend, "demo", &manifest).unwrap();
        assert_eq!(before, after_docs, "README must not change tested content");
        std::fs::write(root.join("src/lib.rs"), "pub fn fixture() -> u32 { 2 }\n").unwrap();
        let after_source = digest_tested_content(root, &backend, "demo", &manifest).unwrap();
        assert_ne!(before, after_source, "source must change tested content");
    }

    #[test]
    fn freshness_reuses_tested_content_across_a_documentation_only_commit() {
        let (dir, backend) = git_fixture();
        let root = dir.path();
        let change = root.join("givn/changes/demo");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        let manifest = manifest();
        let head = backend.revision().unwrap();
        let digest = digest_tested_content(root, &backend, "demo", &manifest).unwrap();
        write_change_receipt(
            root,
            &change,
            "demo",
            &head,
            &manifest.verify.command,
            &digest,
            ChangeVerification {
                result: RunnerResult {
                    scope: None,
                    total: 1,
                    passed: 1,
                    failed: 0,
                    skipped: 0,
                },
                e2e_identity: None,
                e2e_scope: None,
            },
            1,
        )
        .unwrap();
        std::fs::write(root.join("README.md"), "documentation-only change\n").unwrap();
        commit_git(root, "docs");
        let receipt = ChangeReceipt::read(&change).unwrap();
        let status = receipt
            .freshness(&manifest, &change, &backend, root)
            .unwrap();
        assert!(
            matches!(status, ProofStatus::DocumentationOnly { .. }),
            "expected documentation-only reuse, got {status:?}"
        );

        std::fs::write(root.join("src/lib.rs"), "pub fn fixture() -> u32 { 3 }\n").unwrap();
        commit_git(root, "source");
        let receipt = ChangeReceipt::read(&change).unwrap();
        let status = receipt
            .freshness(&manifest, &change, &backend, root)
            .unwrap();
        assert!(
            matches!(
                status,
                ProofStatus::Stale(ref reasons)
                    if reasons.iter().any(|reason| matches!(reason, StaleReason::TestedContentChanged { .. }))
            ),
            "expected a tested-content stale reason, got {status:?}"
        );
    }

    #[test]
    fn schema_1_and_unknown_schemas_keep_the_compatibility_boundary() {
        let (dir, backend) = git_fixture();
        let root = dir.path();
        let change = root.join("givn/changes/demo");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        let manifest = manifest();
        let write = |schema: u32, scope: Option<&str>| {
            let head = backend.revision().unwrap();
            let mut value = serde_json::json!({
                "schema": schema,
                "change": "demo",
                "status": "passed",
                "head": head,
                "features_path": "givn/changes/demo/specs",
                "features_digest": digest_features(&change.join("specs")).unwrap(),
                "command_digest": digest_text(&manifest.verify.command),
                "command": manifest.verify.command,
                "runner_exit": 0,
                "result": {"scope": null, "total": 1, "passed": 1, "failed": 0, "skipped": 0},
                "finished_at_unix": 1,
            });
            if schema == 2 {
                value["tested_content_digest"] = serde_json::json!(
                    digest_tested_content(root, &backend, "demo", &manifest).unwrap()
                );
                value["content_scope"] = serde_json::json!(scope.unwrap_or(CONTENT_SCOPE));
            }
            std::fs::create_dir_all(change.join("verification")).unwrap();
            std::fs::write(
                change.join(CHANGE_RECEIPT_FILE),
                serde_json::to_string_pretty(&value).unwrap(),
            )
            .unwrap();
            ChangeReceipt::read(&change).unwrap()
        };

        let schema_1 = write(1, None);
        let status = schema_1
            .freshness(&manifest, &change, &backend, root)
            .unwrap();
        assert_eq!(status, ProofStatus::Current);

        let unknown = write(99, None);
        assert_eq!(
            unknown
                .freshness(&manifest, &change, &backend, root)
                .unwrap(),
            ProofStatus::Stale(vec![StaleReason::UnsupportedReceiptSchema { schema: 99 }])
        );

        let future_scope = write(2, Some("change-proof/v2"));
        assert_eq!(
            future_scope
                .freshness(&manifest, &change, &backend, root)
                .unwrap(),
            ProofStatus::Stale(vec![StaleReason::UnsupportedContentScope {
                scope: "change-proof/v2".to_string()
            }])
        );
    }

    #[test]
    fn runner_result_reads_counts_without_a_scope() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("change.json");
        std::fs::write(&path, r#"{"total":2,"passed":2,"failed":0,"skipped":0}"#).unwrap();
        let result = RunnerResult::read_counts(&path, "the change verification").unwrap();
        assert_eq!(result.total, 2);
        assert!(result.scope.is_none());
    }

    #[test]
    fn runner_result_rejects_every_untrusted_shape() {
        let dir = temp_receipt_dir();
        let path = dir.path().join("regular.json");
        let write = |json: &str| std::fs::write(&path, json).unwrap();

        write(r#"{"scope":"regular","total":2,"passed":2,"failed":0,"skipped":0}"#);
        assert!(RunnerResult::read(&path, "regular").is_ok());

        write(r#"{"scope":"e2e","total":2,"passed":2,"failed":0,"skipped":0}"#);
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("declares scope")
        );

        write(r#"{"scope":"regular","total":0,"passed":0,"failed":0,"skipped":0}"#);
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("zero scenarios")
        );

        write(r#"{"scope":"regular","total":2,"passed":1,"failed":1,"skipped":0}"#);
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("failed")
        );

        write(r#"{"scope":"regular","total":2,"passed":1,"failed":0,"skipped":1}"#);
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("skipped")
        );

        write(r#"{"scope":"regular","total":3,"passed":2,"failed":0,"skipped":0}"#);
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("reconcile")
        );

        write("{ not json");
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("malformed")
        );

        std::fs::remove_file(&path).unwrap();
        assert!(
            RunnerResult::read(&path, "regular")
                .unwrap_err()
                .to_string()
                .contains("no runner result")
        );
    }

    #[test]
    fn evidence_paths_cover_archive_readme_and_coverage_outputs() {
        let manifest = Manifest::load_embedded().unwrap();
        let root = Path::new("/project");
        assert!(is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("/project/givn/archive/demo/results/regular.json")
        ));
        assert!(is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/archive/demo/verification.json")
        ));
        if manifest.addons.readme {
            assert!(is_evidence_path(
                root,
                "demo",
                &manifest,
                Path::new("README.md")
            ));
        }
        assert!(!is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("givn/specs/demo.feature")
        ));
        assert!(!is_evidence_path(
            root,
            "demo",
            &manifest,
            Path::new("src/lib.rs")
        ));
    }
}
