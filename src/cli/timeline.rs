//! `givn timeline`: the development report — the model the report screen
//! renders and the self-contained HTML document it is rendered into.
//!
//! Like `givn history`, this command runs without a recorder: rendering a
//! report must never extend the record it renders.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::history::{HistoryEntry, Scope};

/// The asset's single data slot: the generator replaces it with the JSON
/// model, so the document stays one self-contained file.
const DATA_SLOT: &str = "/*__GIVN_REPORT_DATA__*/";

/// The report asset, embedded so the binary needs no data directory.
const REPORT_ASSET: &str = include_str!("../../assets/timeline/report.html");

/// Everything the report screen renders: scope, bands, entries, summary. The
/// bands, the phase derivation, the ranking, and the totals are built by the
/// report scenarios; every report already carries the scope it covers and the
/// record lines it is built from.
#[derive(Debug, Default, serde::Serialize)]
pub struct ReportModel {
    /// The named scope this report covers; `None` renders the project report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The record lines the report renders, in file order.
    pub entries: Vec<HistoryEntry>,
}

/// Record lines → `ReportModel` (phase derivation, ranking, totals).
pub fn build_report_model(scope: Option<Scope>, entries: Vec<HistoryEntry>) -> ReportModel {
    ReportModel { scope, entries }
}

/// `ReportModel` → the HTML document (data-slot substitution, escaping).
pub fn render_report(model: &ReportModel) -> String {
    let data = serde_json::to_string(model).unwrap_or_else(|_| "null".to_string());
    REPORT_ASSET.replacen(DATA_SLOT, &data, 1)
}

/// `givn timeline` — write the report for one scope and print its path. With
/// `--out` omitted the report goes to the project's own default path
/// `givn/history/reports/<scope>.html`.
pub fn run(change: Option<&str>, topic: Option<&str>, out: Option<&Path>) -> Result<PathBuf> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let scope = crate::cli::history::scope_for(&root, change, topic);
    let entries = crate::cli::history::entries_in_scope(&root, scope.as_ref())?;
    let model = build_report_model(scope.clone(), entries);
    let path = match out {
        Some(path) => path.to_path_buf(),
        None => default_report_path(&root, scope.as_ref()),
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create the report directory {}", parent.display()))?;
    }
    std::fs::write(&path, render_report(&model))
        .with_context(|| format!("write the report to {}", path.display()))?;
    println!("{}", path.display());
    Ok(path)
}

/// The report's default path: `givn/history/reports/<scope>.html`.
fn default_report_path(root: &Path, scope: Option<&Scope>) -> PathBuf {
    let id = match scope {
        Some(Scope::Change { id }) | Some(Scope::Ideation { id }) | Some(Scope::Project { id }) => {
            id.clone()
        }
        None => "project".to_string(),
    };
    root.join("givn")
        .join("history")
        .join("reports")
        .join(format!("{id}.html"))
}
