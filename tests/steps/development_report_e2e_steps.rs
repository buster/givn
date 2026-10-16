//! `@e2e` step definitions for the `development-report` capability (Tasks E5
//! to E9).
//!
//! The CLI scenario generates the report with the real `givn timeline` and
//! asserts on the file it wrote; the browser scenarios drive the real report
//! through [`crate::common::browser::BrowserDriver`] — one browser and one
//! context per scenario — and assert on what the browser rendered, never on
//! the file's raw bytes. The capability's unit/integration steps live in
//! [`crate::steps::development_report_steps`].

use std::path::PathBuf;

use crate::common::browser::BrowserDriver;

/// State for the `@e2e` development-report scenarios: the report the scenario
/// generated, and the browser a browser scenario drives for the scenario's
/// lifetime. The driver owns its browser and one context; dropping this state
/// closes both, so a failing step never leaves a browser behind.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentReportE2eState {
    /// The report the scenario's generation wrote.
    #[allow(dead_code)] // Task E5–E9 read it.
    pub(crate) report_path: Option<PathBuf>,
    /// The scenario's browser, started on the first browser step.
    #[allow(dead_code)] // Task E6–E9 drive it.
    pub(crate) browser: Option<BrowserDriver>,
}
