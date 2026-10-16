//! Step definitions for the `development-report` capability.
//!
//! The scenarios generate a report through `givn timeline` and assert on the
//! written file (and, in the browser tier, on the rendered page); the record
//! fixtures live in [`crate::steps::development_timeline_support`].

use std::path::PathBuf;

/// State for the development-report scenarios: the report path the last
/// generation step wrote, so a following step can read the rendered file.
// Skeleton (tasks.md S3): the slot exists before the capability's first step
// reads it; that scenario removes this allowance.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct DevelopmentReportState {
    pub(crate) report_path: Option<PathBuf>,
}
