//! `@e2e` step definitions for the `agent-activity-reporting` capability
//! (Task E4).
//!
//! The scenario under this file installs the project (`givn init --agents
//! <target>`) and then drives that harness's *generated* reporter through its
//! own real entry point — the shared harness drivers in
//! [`crate::steps::development_timeline_support`] — and asserts on
//! `givn history --change demo --json`. The capability's unit/integration
//! steps live in [`crate::steps::agent_activity_reporting_steps`].

use std::process::Output;

/// State for the `@e2e` agent-activity-reporting scenarios: the output of the
/// last reporter run the scenario drove, so the assertions read what that
/// process produced rather than a re-derived expectation.
#[derive(Debug, Default)]
pub(crate) struct AgentActivityReportingE2eState {
    /// The reporter process's own output.
    #[allow(dead_code)] // Task E4 reads it.
    pub(crate) reporter_output: Option<Output>,
}
