//! Regular step definitions for E5 passage-cap evidence.

use cucumber::{given, then};
use serde_json::Value;

use super::GivnWorld;

#[given("a permanent corpus with fitting semantic scenarios")]
fn permanent_fitting_semantic_scenarios(world: &mut GivnWorld) {
    super::semantic_token_cap_review_steps::four_scenario_project(world);
    world.retrieval_e5_token_budget = Some("all-512".to_string());
}

#[given(
    "deterministic E5 token-budget fixtures assign exactly 512 tokens to every complete passage"
)]
fn exact_e5_token_budget_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the fitting E5 corpus is missing");
    world.retrieval_e5_token_budget = Some("all-512".to_string());
}

fn json(world: &GivnWorld) -> Value {
    serde_json::from_slice(&super::output(world).stdout).expect("valid boundary duplicate JSON")
}

#[then("the JSON contains no `E5_TOKEN_CAP` finding")]
fn boundary_json_has_no_e5_cap(world: &mut GivnWorld) {
    let output = String::from_utf8_lossy(&super::output(world).stdout);
    assert!(!output.contains("E5_TOKEN_CAP"));
}

#[then("the `e5_token_cap_candidates` collection is empty and `e5_token_cap_count` is 0")]
fn boundary_json_has_empty_e5_caps(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(document["e5_token_cap_count"], 0);
    assert!(
        document["e5_token_cap_candidates"]
            .as_array()
            .is_some_and(Vec::is_empty)
    );
}

#[then("the fitting scenarios retain the existing E5 candidate behaviour")]
fn boundary_json_retains_e5_candidates(world: &mut GivnWorld) {
    let document = json(world);
    let candidates = document["candidates"]
        .as_array()
        .expect("normal candidates");
    assert!(!candidates.is_empty());
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate["kind"] == "semantic_candidate")
    );
}
