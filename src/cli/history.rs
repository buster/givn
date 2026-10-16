//! `givn history`: the read view, the one intake (`givn history ingest`), and
//! the process-per-hook harness mapper (`givn history harness-hook`).

use anyhow::Result;

/// One reported event from the intake payload, with its per-kind validation.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct IngestedEvent {
    /// The event kind (`turn`, `tool`, `subagent`, `question`, `usage`, …).
    pub kind: String,
    /// RFC 3339 UTC end timestamp of the event.
    pub at: String,
}

/// One raw hook payload from a process-per-hook harness, with the fields the
/// mapper reads.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct HarnessPayload {
    /// The harness's own session id (the pending-state file key).
    pub session_id: String,
}

/// `givn history` — read one scope's record.
pub fn run(_change: Option<&str>, _topic: Option<&str>, _json: bool) -> Result<()> {
    todo!("TODO: development-timeline")
}

/// `givn history ingest` — read one JSON payload from stdin and store its
/// accepted events.
pub fn ingest() -> Result<()> {
    todo!("TODO: development-timeline")
}

/// `givn history harness-hook --harness <name>` — map one raw harness payload
/// from stdin into the record (the process-per-hook entry point).
pub fn harness_hook(_harness: &str) -> Result<()> {
    todo!("TODO: development-timeline")
}

/// Raw harness payload → record events plus pending-state updates; the Claude
/// mapper's only entry point, also usable by a future shell-based harness.
pub fn map_harness_event(_payload: &HarnessPayload) -> Result<Vec<IngestedEvent>> {
    todo!("TODO: development-timeline")
}
