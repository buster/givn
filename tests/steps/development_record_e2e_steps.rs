//! `@e2e` step definitions for the `development-record` capability (Task E2,
//! Task E3).
//!
//! The scenarios under this file drive the real CLI as a subprocess in a
//! temporary project root (`common::run_givn`) and assert on what it printed
//! and on the files it wrote — never on the store read behind the command's
//! back. The capability's unit/integration steps live in
//! [`crate::steps::development_record_steps`]; this file holds only the steps
//! that need the real consumer interface.

use serde_json::Value;

/// State for the `@e2e` development-record scenarios: the machine-readable
/// record the scenario's own read printed
/// (`givn history --change … --json`), parsed once, so every Then step asserts
/// against the same parse its When read.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordE2eState {
    /// The entries the last machine-readable read printed.
    #[allow(dead_code)] // Task E3 reads it.
    pub(crate) entries: Vec<Value>,
}
