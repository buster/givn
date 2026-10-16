//! `givn timeline`: the development report — the model the report screen
//! renders and the self-contained HTML document it is rendered into.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// Everything the report screen renders: scope, bands, entries, summary.
#[derive(Debug, Default, serde::Serialize)]
pub struct ReportModel {}

/// Record lines → `ReportModel` (phase derivation, ranking, totals).
pub fn build_report_model() -> ReportModel {
    todo!("TODO: development-timeline")
}

/// `ReportModel` → the HTML document (data-slot substitution, escaping).
pub fn render_report(_model: &ReportModel) -> String {
    todo!("TODO: development-timeline")
}

/// `givn timeline` — write the report for one scope and print its path.
pub fn run(_change: Option<&str>, _topic: Option<&str>, _out: Option<&Path>) -> Result<PathBuf> {
    todo!("TODO: development-timeline")
}
