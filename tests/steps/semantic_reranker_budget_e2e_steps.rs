//! Compiled-CLI E2E steps for the configurable second-stage pair budget.

use cucumber::{then, when};

use super::GivnWorld;
use crate::common;

#[when(
    "the end-to-end user runs the permanent duplicate audit with JSON output through the givn CLI"
)]
fn e2e_runs_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E semantic budget project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_duplicate_scores.as_deref() {
        environments.push(("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &environments,
    ));
}

#[then("stdout reports a second-stage pair budget of 2048")]
fn e2e_reports_default_budget(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(
        document["bge_run"]["max_unique_pairs"],
        serde_json::json!(2048)
    );
}

#[then("stdout reports that all three E5 candidates were scored by BGE")]
fn e2e_reports_all_scored(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(3));
    assert_eq!(document["bge_run"]["unscored_count"], serde_json::json!(0));
}
