//! Step definitions for the `agent-activity-reporting` capability.
//!
//! The scenarios hand a reporter payload to the intake (`givn history
//! ingest`) and observe the record it produced; the harness drivers live in
//! [`crate::steps::development_timeline_support`].

use serde_json::Value;

/// State for the agent-activity-reporting scenarios: the payload the last
/// intake invocation was fed, so a following step can assert on its events.
// Skeleton (tasks.md S3): the slot exists before the capability's first step
// reads it; that scenario removes this allowance.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct AgentActivityReportingState {
    pub(crate) last_payload: Option<Value>,
}
