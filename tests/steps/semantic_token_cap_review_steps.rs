//! Regular black-box steps for complete semantic token-cap evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

const FOUR_SCENARIO_CORPUS: &str = r#"Feature: token budget corpus

  Scenario: Aster semantic candidate
    Given alpha context
    When alpha action runs
    Then alpha result is complete

  Scenario: Beryl semantic candidate
    Given beta context
    When beta action runs
    Then beta result is complete

  Scenario: Cobalt semantic candidate
    Given gamma context
    When gamma action runs
    Then gamma result is complete

  Scenario: Dune semantic candidate
    Given delta context
    When delta action runs
    Then delta result is complete
"#;

const FOUR_SCENARIO_SCORE_OVERRIDES: &str = "givn/specs/token-budget/token-budget.feature|Aster semantic candidate::givn/specs/token-budget/token-budget.feature|Beryl semantic candidate=0.9600;givn/specs/token-budget/token-budget.feature|Cobalt semantic candidate::givn/specs/token-budget/token-budget.feature|Dune semantic candidate=0.9600;givn/specs/token-budget/token-budget.feature|Aster semantic candidate::givn/specs/token-budget/token-budget.feature|Cobalt semantic candidate=0.0000;givn/specs/token-budget/token-budget.feature|Aster semantic candidate::givn/specs/token-budget/token-budget.feature|Dune semantic candidate=0.0000;givn/specs/token-budget/token-budget.feature|Beryl semantic candidate::givn/specs/token-budget/token-budget.feature|Cobalt semantic candidate=0.0000;givn/specs/token-budget/token-budget.feature|Beryl semantic candidate::givn/specs/token-budget/token-budget.feature|Dune semantic candidate=0.0000";

fn four_scenario_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    fs::create_dir_all(specs.join("token-budget")).expect("create token-budget specs");
    fs::write(
        env.path("givn/specs/token-budget/token-budget.feature"),
        FOUR_SCENARIO_CORPUS,
    )
    .expect("write token-budget corpus");
    world.env = Some(env);
    world.retrieval_duplicate_scores = Some(FOUR_SCENARIO_SCORE_OVERRIDES.to_string());
}

#[given("a permanent corpus with two E5 semantic candidates")]
fn permanent_two_semantic_candidates(world: &mut GivnWorld) {
    four_scenario_project(world);
}

#[given("one complete serialized pair fits both BGE and NLI")]
fn one_pair_fits_bge_and_nli(world: &mut GivnWorld) {
    assert!(
        world.env.is_some(),
        "the permanent corpus must be initialized first"
    );
    world.retrieval_bge_token_budget = Some("all-512".to_string());
    world.retrieval_nli_token_budget = Some("partial-512-513".to_string());
}

#[given("another complete serialized pair fits BGE but encodes to 513 NLI tokens")]
fn another_pair_reaches_nli_cap(world: &mut GivnWorld) {
    assert!(
        world.env.is_some(),
        "the permanent corpus must be initialized first"
    );
    assert_eq!(world.retrieval_bge_token_budget.as_deref(), Some("all-512"));
    assert_eq!(
        world.retrieval_nli_token_budget.as_deref(),
        Some("partial-512-513")
    );
}

#[given("local BGE and NLI evidence are enabled with deterministic token-budget fixtures")]
fn local_bge_nli_token_budget_fixtures(world: &mut GivnWorld) {
    assert!(
        world.env.is_some(),
        "the permanent corpus must be initialized first"
    );
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[when("the user runs `givn spec duplicates --format json` through the givn CLI")]
fn run_regular_duplicate_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("token-budget corpus is missing");
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
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            ("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores),
            (
                "GIVN_SPEC_BGE_TOKEN_BUDGET",
                world
                    .retrieval_bge_token_budget
                    .as_deref()
                    .expect("BGE token fixture"),
            ),
            (
                "GIVN_SPEC_NLI_TOKEN_BUDGET",
                world
                    .retrieval_nli_token_budget
                    .as_deref()
                    .expect("NLI token fixture"),
            ),
        ],
    ));
}

fn json(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON")
}

fn nli_cap_candidate(document: &serde_json::Value) -> &serde_json::Value {
    document["candidates"]
        .as_array()
        .expect("candidate collection")
        .iter()
        .find(|candidate| candidate["evidence"]["nli"]["status"] == "NLI_TOKEN_CAP")
        .expect("NLI cap candidate")
}

#[then("the JSON response preserves a normal BGE evidence object for the 513-token NLI pair")]
fn normal_bge_evidence_for_nli_cap(world: &mut GivnWorld) {
    let document = json(world);
    let candidate = nli_cap_candidate(&document);
    assert!(candidate["evidence"]["bge"].is_object());
    assert_ne!(candidate["evidence"]["bge"]["status"], "BGE_TOKEN_CAP");
    assert!(candidate["evidence"]["bge"]["symmetric_raw"].is_number());
}

#[then("that candidate's JSON `evidence.nli.status` is \"NLI_TOKEN_CAP\"")]
fn nli_candidate_has_token_cap_status(world: &mut GivnWorld) {
    assert_eq!(
        nli_cap_candidate(&json(world))["evidence"]["nli"]["status"],
        "NLI_TOKEN_CAP"
    );
}

#[then(
    "that candidate's JSON NLI evidence reports its pair token count as 513 and its limit as 512"
)]
fn nli_candidate_reports_pair_budget(world: &mut GivnWorld) {
    let document = json(world);
    let evidence = &nli_cap_candidate(&document)["evidence"]["nli"];
    assert_eq!(evidence["left_tokens"], 257);
    assert_eq!(evidence["right_tokens"], 255);
    assert_eq!(evidence["pair_tokens"], 513);
    assert_eq!(evidence["limit"], 512);
    assert_eq!(evidence["refactoring_required"], true);
}

#[then("the JSON response reports `nli_run.status` as \"NLI_ENABLED\"")]
fn nli_run_is_enabled(world: &mut GivnWorld) {
    assert_eq!(json(world)["nli_run"]["status"], "NLI_ENABLED");
}

#[then(
    "the candidate is absent from the NLI filtered collection and the combined recommended collection"
)]
fn nli_cap_candidate_is_not_filtered_or_combined(world: &mut GivnWorld) {
    let document = json(world);
    let candidate = nli_cap_candidate(&document);
    let left = candidate["left"].clone();
    let right = candidate["right"].clone();
    for collection in ["nli_filtered_candidates", "combined_recommended_candidates"] {
        let entries = document[collection]
            .as_array()
            .expect("candidate collection");
        assert!(
            !entries
                .iter()
                .any(|entry| entry["left"] == left && entry["right"] == right)
        );
    }
}

#[then("BGE and NLI evidence identify the same E5 candidate pool and endpoint pair")]
fn bge_nli_share_candidate_pool(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(document["bge_run"]["e5_candidates"], 2);
    assert_eq!(document["nli_run"]["e5_candidates"], 2);
    let candidate = nli_cap_candidate(&document);
    assert!(candidate["evidence"]["bge"].is_object());
    assert!(candidate["evidence"]["nli"].is_object());
}

#[then(
    "the JSON response instructs the agent to shorten or split the scenarios and rerun the audit"
)]
fn duplicate_json_has_token_cap_guidance(world: &mut GivnWorld) {
    let document = json(world);
    assert_eq!(document["refactoring_required"], true);
    assert!(
        document["guidance"]
            .as_str()
            .unwrap()
            .contains("shorten or split")
    );
    assert!(document["guidance"].as_str().unwrap().contains("rerun"));
}

#[given("a permanent corpus with one E5 semantic candidate")]
fn permanent_one_semantic_candidate(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    fs::create_dir_all(specs.join("token-budget")).expect("create token-budget specs");
    fs::write(
        env.path("givn/specs/token-budget/token-budget.feature"),
        "Feature: exact token budget\n\n  Scenario: Aster exact candidate\n    Given exact alpha context\n    When exact alpha action runs\n    Then exact alpha result is complete\n\n  Scenario: Beryl exact candidate\n    Given exact beta context\n    When exact beta action runs\n    Then exact beta result is complete\n",
    )
    .expect("write exact token-budget corpus");
    world.env = Some(env);
    world.retrieval_duplicate_scores = Some("givn/specs/token-budget/token-budget.feature|Aster exact candidate::givn/specs/token-budget/token-budget.feature|Beryl exact candidate=0.9600".to_string());
}

#[given(
    "its complete serialized pair encodes to exactly 512 BGE tokens and exactly 512 NLI tokens"
)]
fn exact_boundary_token_budget(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the exact-boundary corpus is missing");
    world.retrieval_bge_token_budget = Some("exact-512".to_string());
    world.retrieval_nli_token_budget = Some("exact-512".to_string());
}

#[then("the JSON response reports normal scored BGE and NLI evidence statuses for the candidate")]
fn exact_boundary_has_scored_evidence(world: &mut GivnWorld) {
    let candidate = &json(world)["candidates"][0];
    assert!(candidate["evidence"]["bge"].is_object());
    assert!(candidate["evidence"]["nli"].is_object());
    assert!(
        candidate["evidence"]["bge"]["status"]
            .as_str()
            .unwrap()
            .starts_with("BGE_")
    );
    assert!(
        candidate["evidence"]["nli"]["status"]
            .as_str()
            .unwrap()
            .starts_with("NLI_")
    );
}

#[then("the JSON response reports no BGE or NLI token-cap status or reason")]
fn exact_boundary_has_no_cap_status(world: &mut GivnWorld) {
    let output = String::from_utf8_lossy(&super::output(world).stdout);
    assert!(!output.contains("TOKEN_CAP"));
    assert!(!output.contains("token-cap"));
}

#[then("the candidate receives the normal recommended or filtered evidence status")]
fn exact_boundary_has_normal_admission(world: &mut GivnWorld) {
    let candidate = &json(world)["candidates"][0];
    assert!(matches!(
        candidate["evidence"]["bge"]["status"].as_str(),
        Some("BGE_RECOMMENDED") | Some("BGE_FILTERED")
    ));
    assert!(matches!(
        candidate["evidence"]["nli"]["status"].as_str(),
        Some("NLI_RECOMMENDED") | Some("NLI_FILTERED")
    ));
}
