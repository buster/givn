//! E2E black-box steps for complete semantic token-cap evidence.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("an end-to-end permanent corpus with two E5 semantic candidates")]
fn e2e_two_semantic_candidates(world: &mut GivnWorld) {
    super::semantic_token_cap_review_steps::four_scenario_project(world);
}

#[given("one complete serialized pair encodes to exactly 512 BGE tokens")]
fn e2e_exact_bge_pair(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the E2E permanent corpus is missing");
    world.retrieval_bge_token_budget = Some("partial-512-513".to_string());
}

#[given("another complete serialized pair encodes to 513 BGE tokens")]
fn e2e_over_cap_bge_pair(world: &mut GivnWorld) {
    assert_eq!(
        world.retrieval_bge_token_budget.as_deref(),
        Some("partial-512-513")
    );
}

#[given("local BGE evidence is enabled with deterministic token-budget fixtures")]
fn e2e_bge_token_budget_fixtures(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the E2E permanent corpus is missing");
    world.retrieval_reranker = Some("stub".to_string());
}

#[when("the end-to-end user runs `givn spec duplicates --format json` through the givn CLI")]
fn e2e_run_duplicate_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E duplicate corpus is missing");
    let scores = world
        .retrieval_duplicate_scores
        .as_deref()
        .expect("duplicate score fixture");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            ("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores),
            (
                "GIVN_SPEC_BGE_TOKEN_BUDGET",
                world
                    .retrieval_bge_token_budget
                    .as_deref()
                    .expect("BGE token fixture"),
            ),
        ],
    ));
}

fn json(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_slice(&super::output(world).stdout).expect("valid E2E duplicate JSON")
}

fn bge_cap_candidate(document: &serde_json::Value) -> &serde_json::Value {
    document["candidates"]
        .as_array()
        .expect("candidate collection")
        .iter()
        .find(|candidate| candidate["evidence"]["bge"]["status"] == "BGE_TOKEN_CAP")
        .expect("BGE cap candidate")
}

#[then("the JSON response includes the 513-token pair in the `candidates` collection")]
fn e2e_json_includes_over_cap_candidate(world: &mut GivnWorld) {
    let document = json(world);
    assert!(bge_cap_candidate(&document)["left"].is_object());
    assert_eq!(document["candidate_count"], 2);
}

#[then("that candidate's JSON `evidence.bge.status` is \"BGE_TOKEN_CAP\"")]
fn e2e_candidate_has_bge_cap_status(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(
        bge_cap_candidate(&document)["evidence"]["bge"]["status"],
        "BGE_TOKEN_CAP"
    );
}

#[then(
    "that candidate's JSON BGE evidence reports both scenario token counts, pair token count 513, and limit 512"
)]
fn e2e_candidate_reports_bge_pair_budget(world: &mut GivnWorld) {
    let document = json(world);
    let evidence = &bge_cap_candidate(&document)["evidence"]["bge"];
    assert_eq!(evidence["left_tokens"], 257);
    assert_eq!(evidence["right_tokens"], 255);
    assert_eq!(evidence["pair_tokens"], 513);
    assert_eq!(evidence["limit"], 512);
    assert_eq!(evidence["refactoring_required"], true);
}

#[then("the JSON response reports `bge_run.status` as \"BGE_ENABLED\"")]
fn e2e_bge_run_is_enabled(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(document["bge_run"]["status"], "BGE_ENABLED");
    assert_eq!(document["bge_run"]["e5_candidates"], 2);
    assert_eq!(document["bge_run"]["unique_pairs"], 1);
}

#[then("the candidate is absent from the BGE filtered collection")]
fn e2e_bge_cap_candidate_is_not_filtered(world: &mut GivnWorld) {
    let document = json(world);
    let candidate = bge_cap_candidate(&document);
    let left = candidate["left"].clone();
    let right = candidate["right"].clone();
    assert!(
        !document["filtered_candidates"]
            .as_array()
            .expect("BGE filtered collection")
            .iter()
            .any(|entry| entry["left"] == left && entry["right"] == right)
    );
}

#[then(
    "the JSON response marks refactoring as required and instructs the agent to shorten or split the scenarios and rerun the audit"
)]
fn e2e_json_has_refactoring_guidance(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(document["refactoring_required"], true);
    let guidance = document["guidance"].as_str().expect("token-cap guidance");
    assert!(guidance.contains("shorten or split"));
    assert!(guidance.contains("rerun"));
}

#[given("an end-to-end project with one unresolved E5 semantic candidate")]
fn e2e_unresolved_semantic_candidate(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("its complete serialized pair encodes to 513 BGE tokens and 513 NLI tokens")]
fn e2e_bge_nli_over_cap_pair(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when(
    "the end-to-end user runs blocking change review for \"running-change\" through the givn CLI"
)]
fn e2e_run_blocking_review(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the command exits non-zero because the semantic candidate remains unresolved")]
fn e2e_blocking_review_fails_unresolved(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(
    "the semantic review report records BGE status \"BGE_UNAVAILABLE\" with reason \"token-cap\" because no BGE pair was scoreable"
)]
fn e2e_report_bge_token_cap_unavailable(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(
    "the semantic review report records NLI status \"NLI_UNAVAILABLE\" with reason \"token-cap\" because no NLI pair was scoreable"
)]
fn e2e_report_nli_token_cap_unavailable(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the candidate records BGE admission \"BGE_TOKEN_CAP\" and NLI admission \"NLI_TOKEN_CAP\"")]
fn e2e_report_records_bge_nli_cap_admissions(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(
    "the semantic review report records both scenario token counts, each pair count of 513, and each limit of 512"
)]
fn e2e_report_records_pair_budgets(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(
    "the semantic review report tells the agent to shorten or split the scenarios before rerunning review"
)]
fn e2e_report_has_refactoring_guidance(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the candidate remains `UNCERTAIN` and has no automatic Givn classification")]
fn e2e_candidate_remains_uncertain(_world: &mut GivnWorld) {
    unimplemented!()
}
