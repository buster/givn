//! Regular black-box steps for the configurable second-stage pair budget.

use std::fs;

use cucumber::{given, then};

use super::GivnWorld;
use crate::common;

#[given("a permanent corpus with three ordered E5 semantic candidates")]
fn three_ordered_e5_candidates(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated semantic budget specs");
    fs::create_dir_all(&specs).expect("create semantic budget specs");
    fs::write(
        specs.join("ordered.feature"),
        "Feature: ordered candidates\n\n  Scenario: Candidate A\n    Given alpha context\n    When alpha action runs\n    Then alpha result appears\n\n  Scenario: Candidate B\n    Given beta context\n    When beta action runs\n    Then beta result appears\n\n  Scenario: Candidate C\n    Given gamma context\n    When gamma action runs\n    Then gamma result appears\n",
    )
    .expect("write ordered semantic budget fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/ordered.feature|Candidate A::givn/specs/ordered.feature|Candidate B=0.9900;givn/specs/ordered.feature|Candidate A::givn/specs/ordered.feature|Candidate C=0.9800;givn/specs/ordered.feature|Candidate B::givn/specs/ordered.feature|Candidate C=0.9700".to_string(),
    );
}

#[given("local BGE evidence is enabled")]
fn local_bge_enabled(world: &mut GivnWorld) {
    world.retrieval_bge = Some("stub".to_string());
}

#[given("local BGE and NLI evidence are enabled")]
fn local_bge_nli_enabled(world: &mut GivnWorld) {
    world.retrieval_bge = Some("stub".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("the second-stage pair budget is 2")]
fn pair_budget_two(world: &mut GivnWorld) {
    world.retrieval_max_rerank_pairs = Some("2".to_string());
}

#[given("the second-stage pair budget is \"not-a-number\"")]
fn pair_budget_malformed(world: &mut GivnWorld) {
    world.retrieval_max_rerank_pairs = Some("not-a-number".to_string());
}

#[given("the second-stage pair budget is \"0\"")]
fn pair_budget_zero(world: &mut GivnWorld) {
    world.retrieval_max_rerank_pairs = Some("0".to_string());
}

#[then("the JSON reports a second-stage pair budget of 2048")]
fn json_reports_default_budget(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(
        document["bge_run"]["max_unique_pairs"],
        serde_json::json!(2048)
    );
}

#[then("the JSON reports three scored BGE pairs and zero BGE unscored candidates")]
fn json_reports_all_bge_scored(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(3));
    assert_eq!(document["bge_run"]["unscored_count"], serde_json::json!(0));
}

#[then("the JSON reports a second-stage pair budget of 2")]
fn json_reports_configured_budget(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    let run = document
        .get("bge_run")
        .or_else(|| document.get("nli_run"))
        .expect("second-stage run");
    assert_eq!(run["max_unique_pairs"], serde_json::json!(2));
}

#[then("BGE scores the two highest E5 candidates")]
fn bge_scores_highest_e5(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(2));
    let scored = document["candidates"]
        .as_array()
        .expect("candidate array")
        .iter()
        .filter(|candidate| candidate["kind"] == "semantic_candidate")
        .filter(|candidate| candidate["evidence"]["bge"].is_object())
        .map(|candidate| candidate["evidence"]["score"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(scored, vec!["0.9900", "0.9800"]);
}

#[then("the lowest-E5 candidate is recorded as BGE_UNSCORED")]
fn bge_records_lowest_unscored(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unscored_count"], serde_json::json!(1));
    let unscored = document["unscored_candidates"]
        .as_array()
        .expect("BGE unscored candidates");
    assert_eq!(unscored.len(), 1);
    assert_eq!(
        unscored[0]["evidence"]["score"],
        serde_json::json!("0.9700")
    );
    assert!(unscored[0]["evidence"]["bge"].is_null());
}

#[then("the BGE run is enabled rather than unavailable for pair-cap")]
fn bge_is_enabled_not_pair_cap(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(
        document["bge_run"]["status"],
        serde_json::json!("BGE_ENABLED")
    );
    assert!(document["bge_run"]["reason"].is_null());
}

#[then("NLI scores the two highest E5 candidates")]
fn nli_scores_highest_e5(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["nli_run"]["unique_pairs"], serde_json::json!(2));
    let scored = document["candidates"]
        .as_array()
        .expect("candidate array")
        .iter()
        .filter(|candidate| candidate["kind"] == "semantic_candidate")
        .filter(|candidate| candidate["evidence"]["nli"].is_object())
        .map(|candidate| candidate["evidence"]["score"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(scored, vec!["0.9900", "0.9800"]);
}

#[then("the lowest-E5 candidate is recorded as NLI_UNSCORED")]
fn nli_records_lowest_unscored(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["nli_run"]["unscored_count"], serde_json::json!(1));
    let unscored = document["nli_unscored_candidates"]
        .as_array()
        .expect("NLI unscored candidates");
    assert_eq!(unscored.len(), 1);
    assert_eq!(
        unscored[0]["evidence"]["score"],
        serde_json::json!("0.9700")
    );
    assert!(unscored[0]["evidence"]["nli"].is_null());
}

#[then("the NLI run is enabled rather than unavailable for pair-cap")]
fn nli_is_enabled_not_pair_cap(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(
        document["nli_run"]["status"],
        serde_json::json!("NLI_ENABLED")
    );
    assert!(document["nli_run"]["reason"].is_null());
}

#[then("stderr identifies the invalid second-stage pair budget")]
fn stderr_identifies_invalid_budget(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("GIVN_SPEC_MAX_RERANK_PAIRS"),
        "stderr: {}",
        super::stderr(world)
    );
}

#[then("BGE and NLI each score the two highest E5 candidates")]
fn both_score_highest_e5(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(2));
    assert_eq!(document["nli_run"]["unique_pairs"], serde_json::json!(2));
    let mut bge_scores = Vec::new();
    let mut nli_scores = Vec::new();
    for candidate in document["candidates"].as_array().expect("candidate array") {
        if candidate["kind"] != "semantic_candidate" {
            continue;
        }
        if candidate["evidence"]["bge"].is_object() {
            bge_scores.push(candidate["evidence"]["score"].as_str().unwrap());
        }
        if candidate["evidence"]["nli"].is_object() {
            nli_scores.push(candidate["evidence"]["score"].as_str().unwrap());
        }
    }
    assert_eq!(bge_scores, vec!["0.9900", "0.9800"]);
    assert_eq!(nli_scores, vec!["0.9900", "0.9800"]);
}

#[then("BGE and NLI each record the lowest-E5 candidate as unscored")]
fn both_record_lowest_unscored(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["bge_run"]["unscored_count"], serde_json::json!(1));
    assert_eq!(document["nli_run"]["unscored_count"], serde_json::json!(1));
    assert_eq!(document["unscored_candidates"].as_array().unwrap().len(), 1);
    assert_eq!(
        document["nli_unscored_candidates"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[then("the combined recommendation uses the intersection of both model recommendations")]
fn combined_uses_intersection(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    let expected = document["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|candidate| {
            candidate["kind"] == "semantic_candidate"
                && candidate["evidence"]["bge"]["status"] == "BGE_RECOMMENDED"
                && candidate["evidence"]["nli"]["status"] == "NLI_RECOMMENDED"
        })
        .count();
    assert_eq!(
        document["combined_recommended_candidates"]
            .as_array()
            .unwrap()
            .len(),
        expected
    );
}
