//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use serde_json::Value;

/// State for the development-record scenarios: the entry the last reading
/// step identified, so a following step can assert on "that entry".
// Skeleton (tasks.md S3): the slot exists before the capability's first step
// reads it; that scenario removes this allowance.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordState {
    pub(crate) last_entry: Option<Value>,
}
