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

impl Receipt {
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
                "archive blocked: {label} runner result declares {} failed scenario(s)",
                result.failed
            );
        }
        if result.skipped != 0 {
            bail!(
                "archive blocked: {label} runner result declares {} skipped scenario(s)",
                result.skipped
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
    pub runner_exit: i32,
    pub result: RunnerResult,
    pub finished_at_unix: u64,
}

/// Path of the change-scoped verification receipt.
pub const CHANGE_RECEIPT_FILE: &str = "verification/tasks-verification.json";
/// Path of the raw change-scoped runner result.
pub const CHANGE_RESULT_FILE: &str = "verification/tasks-result.json";

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

    /// Whether this receipt still proves the same revision, delta features,
    /// and verify command.
    pub fn is_current(
        &self,
        change_dir: &Path,
        head: &str,
        command: &str,
    ) -> Result<bool> {
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

/// Write a change-scoped receipt atomically.
pub fn write_change_receipt(
    project_root: &Path,
    change_dir: &Path,
    change: &str,
    head: &str,
    command: &str,
    result: RunnerResult,
    finished_at_unix: u64,
) -> Result<()> {
    let features = change_dir.join("specs");
    let features_path = features
        .strip_prefix(project_root)
        .unwrap_or(&features)
        .to_string_lossy()
        .replace('\\', "/");
    let receipt = ChangeReceipt {
        schema: 1,
        change: change.to_string(),
        status: "passed".to_string(),
        head: head.to_string(),
        features_path,
        features_digest: digest_features(&features)?,
        command_digest: digest_text(command),
        command: Some(command.to_string()),
        runner_exit: 0,
        result,
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
            receipt.merge_targets.get("cap/file.feature").map(String::as_str),
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
            RunnerResult {
                scope: None,
                total: 1,
                passed: 1,
                failed: 0,
                skipped: 0,
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
        assert!(err.to_string().contains("verification receipt missing"), "{err}");
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
            RunnerResult {
                scope: None,
                total: 2,
                passed: 2,
                failed: 0,
                skipped: 0,
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
