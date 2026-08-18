//! Regular black-box steps for bounded BGE evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("a project with a permanent semantic candidate for second-stage evidence")]
fn bge_project(world: &mut GivnWorld) {
    super::spec_explore_permanent_steps::prepare_exploration_project(world);
    world.retrieval_reranker = Some("disabled".to_string());
}

#[given("the local second-stage evidence is disabled")]
fn bge_disabled(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("disabled".to_string());
}

#[given("a complete draft scenario is available for permanent exploration")]
fn complete_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft provider request\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write complete draft scenario");
}

#[when("the user explores the draft through the givn CLI")]
fn explore_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "disabled")],
    ));
}

#[when("the user explores the draft with JSON output")]
fn explore_permanent_draft_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature", "--format", "json"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "disabled")],
    ));
}

#[then("the result set contains the E5 candidates")]
fn result_set_contains_e5_candidates(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("rank 1:"),
        "expected an E5-ranked result: {}",
        super::stdout(world)
    );
}

#[then("stdout identifies the E5-only configuration")]
fn stdout_identifies_e5_only(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("model: intfloat/multilingual-e5-large"),
        "stdout did not identify E5: {}",
        super::stdout(world)
    );
}

#[then("stdout contains no participating second-stage model, score, or runtime")]
fn stdout_has_no_bge_participation(world: &mut GivnWorld) {
    let output = super::stdout(world).to_ascii_lowercase();
    assert!(!output.contains("bge"), "stdout unexpectedly contains BGE evidence: {output}");
}

#[then("the JSON retains the existing E5 query, index, and result contract")]
fn json_retains_e5_contract(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid explore JSON");
    assert!(document.get("query").is_some());
    assert!(document.get("index").is_some());
    assert!(document.get("results").is_some());
}

#[then("the JSON contains no BGE evidence object or participating model")]
fn json_has_no_bge_evidence(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid explore JSON");
    assert!(document.get("evidence").is_none());
    assert!(!super::stdout(world).to_ascii_lowercase().contains("bge"));
}
