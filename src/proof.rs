//! Archive proof: provenance, the runner-result contract, and the
//! verification receipt.
//!
//! `verification.json` is the durable record of an archive's state. Pending
//! receipts are committed with the candidate; proven receipts are committed
//! as the evidence-only finalization after every scope passed.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
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
    /// A verified and finalized archive.
    Proven,
    /// Both active and archived directories exist, or the receipt is
    /// unreadable/unrecognized.
    Corrupt,
}

/// Classify an archive by its receipt.
pub fn read_status(project_root: &Path, id: &str) -> ArchiveProof {
    let active = project_root.join("givn/changes").join(id).exists();
    let archive = project_root.join("givn/archive").join(id);
    if active && archive.exists() {
        return ArchiveProof::Corrupt;
    }
    if !archive.exists() {
        return ArchiveProof::None;
    }
    let Ok(text) = std::fs::read_to_string(archive.join(RECEIPT_FILE)) else {
        return ArchiveProof::Legacy;
    };
    match serde_json::from_str::<Receipt>(&text) {
        Ok(receipt) if receipt.status == STATUS_PENDING => ArchiveProof::Pending,
        Ok(receipt) if receipt.status == STATUS_PROVEN => ArchiveProof::Proven,
        _ => ArchiveProof::Corrupt,
    }
}

/// A verification receipt (`verification.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub schema: u32,
    pub change: String,
    pub status: String,
    #[serde(default)]
    pub candidate_revision: Option<String>,
    #[serde(default)]
    pub tested_revision: Option<String>,
    #[serde(default)]
    pub scopes: BTreeMap<String, ScopeOutcome>,
    #[serde(default)]
    pub coverage: Option<CoverageOutcome>,
    #[serde(default)]
    pub started_at_unix: Option<u64>,
    #[serde(default)]
    pub finished_at_unix: Option<u64>,
}

impl Receipt {
    /// Write the pending receipt committed with the candidate. It cannot name
    /// the candidate revision's own identifier, so both revision fields stay unset.
    pub fn write_pending(archive_dir: &Path, change: &str, started_at_unix: u64) -> Result<()> {
        let receipt = Self {
            schema: 1,
            change: change.to_string(),
            status: STATUS_PENDING.to_string(),
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
            candidate_revision: Some(tested_revision.to_string()),
            tested_revision: Some(tested_revision.to_string()),
            scopes,
            coverage,
            started_at_unix: Some(started_at_unix),
            finished_at_unix: Some(finished_at_unix),
        };
        write_receipt(archive_dir, &receipt)
    }
}

fn write_receipt(archive_dir: &Path, receipt: &Receipt) -> Result<()> {
    std::fs::create_dir_all(archive_dir)
        .with_context(|| format!("create archive directory {}", archive_dir.display()))?;
    let path = archive_dir.join(RECEIPT_FILE);
    let json = serde_json::to_string_pretty(receipt)?;
    std::fs::write(&path, format!("{json}\n"))
        .with_context(|| format!("write verification receipt {}", path.display()))
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
    pub scope: String,
    pub total: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
}

impl RunnerResult {
    /// Read and validate a scope result. Every failure is fail-closed and
    /// names the offending file.
    pub fn read(path: &Path, expected_scope: &str) -> Result<Self> {
        if !path.exists() {
            bail!(
                "archive blocked: the {expected_scope} scope produced no runner result at {}",
                path.display()
            );
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read runner result {}", path.display()))?;
        let result: Self = serde_json::from_str(&text).map_err(|error| {
            anyhow::anyhow!(
                "archive blocked: the {expected_scope} scope runner result at {} is malformed: {error}",
                path.display()
            )
        })?;
        if result.scope != expected_scope {
            bail!(
                "archive blocked: the runner result at {} declares scope '{}' but '{}' was expected",
                path.display(),
                result.scope,
                expected_scope
            );
        }
        if result.total == 0 {
            bail!(
                "archive blocked: the {expected_scope} scope runner result declares zero scenarios"
            );
        }
        if result.failed != 0 {
            bail!(
                "archive blocked: the {expected_scope} scope runner result declares {} failed scenario(s)",
                result.failed
            );
        }
        if result.skipped != 0 {
            bail!(
                "archive blocked: the {expected_scope} scope runner result declares {} skipped scenario(s)",
                result.skipped
            );
        }
        if result.passed + result.failed + result.skipped != result.total {
            bail!(
                "archive blocked: the {expected_scope} scope runner result counts do not reconcile \
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

    if normalized.starts_with(&format!("givn/archive/{id}/")) {
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

        std::fs::create_dir_all(root.path().join("givn/changes/demo")).unwrap();
        assert_eq!(read_status(root.path(), id), ArchiveProof::Corrupt);
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
