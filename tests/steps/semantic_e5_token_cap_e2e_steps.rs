//! E2E step definitions for E5 passage-cap evidence.

use std::fs;

use cucumber::{given, then, when};
use serde_json::Value;

use super::GivnWorld;
use crate::common;

const FITTING_TITLE: &str = "Fitting E5 scenario";
const CAPPED_TITLE: &str = "Over-cap E5 scenario";

fn index(world: &GivnWorld) -> Value {
    let env = world.env.as_ref().expect("E5 index project is missing");
    let bytes = fs::read(env.path("givn/.spec-index.json")).expect("read permanent E5 index");
    serde_json::from_slice(&bytes).expect("valid permanent E5 index JSON")
}

#[given(
    "an end-to-end permanent corpus with one fitting scenario and one complete E5 passage above 512 tokens"
)]
fn e2e_partial_permanent_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    fs::create_dir_all(specs.join("e5-cap")).expect("create E5 cap specs");
    fs::write(
        env.path("givn/specs/e5-cap/e5-cap.feature"),
        "Feature: E5 cap fixture\n\n  Scenario: Fitting E5 scenario\n    Given a fitting context\n    When the fitting action runs\n    Then the fitting result is complete\n\n  Scenario: Over-cap E5 scenario\n    Given an over-cap context\n    When the over-cap action runs\n    Then the over-cap result is complete\n",
    )
    .expect("write E5 cap corpus");
    world.env = Some(env);
}

#[given(
    "deterministic E5 token-budget fixtures identify the over-cap scenario as requiring 550 tokens"
)]
fn e2e_over_cap_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the E5 cap corpus is missing");
    world.retrieval_e5_token_budget = Some("partial-512-550".to_string());
}

#[when("the end-to-end user runs `givn spec index` through the givn CLI")]
fn e2e_run_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 cap corpus is missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "index"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_E5_TOKEN_BUDGET",
                world
                    .retrieval_e5_token_budget
                    .as_deref()
                    .expect("E5 token fixture"),
            ),
        ],
    ));
}

#[then("the fitting scenario has a vector in the permanent index")]
fn fitting_scenario_has_vector(world: &mut GivnWorld) {
    let document = index(world);
    let entries = document["entries"].as_array().expect("index entries");
    let entry = entries
        .iter()
        .find(|entry| entry["title"] == FITTING_TITLE)
        .expect("fitting index entry");
    assert!(
        entry["embedding"]
            .as_array()
            .is_some_and(|values| !values.is_empty())
    );
}

#[then("the over-cap scenario has no truncated vector in the permanent index")]
fn over_cap_scenario_has_no_vector(world: &mut GivnWorld) {
    let document = index(world);
    let entries = document["entries"].as_array().expect("index entries");
    assert!(!entries.iter().any(|entry| entry["title"] == CAPPED_TITLE));
}

#[then(
    "the permanent index has one additive `e5_token_cap_candidates` record for the over-cap endpoint"
)]
fn index_has_one_e5_cap_record(world: &mut GivnWorld) {
    let document = index(world);
    let caps = document["e5_token_caps"]
        .as_array()
        .expect("index E5 token-cap metadata");
    assert_eq!(caps.len(), 1);
    assert_eq!(caps[0]["status"], "E5_TOKEN_CAP");
    assert_eq!(caps[0]["title"], CAPPED_TITLE);
}

#[then(
    "that cap record has its file, title, complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and guidance"
)]
fn index_cap_record_is_complete(world: &mut GivnWorld) {
    let cap = &index(world)["e5_token_caps"][0];
    let file = cap["file"].as_str().expect("cap file");
    assert_eq!(file, "givn/specs/e5-cap/e5-cap.feature");
    assert_eq!(cap["title"], CAPPED_TITLE);
    let hash = cap["content_hash"].as_str().expect("cap content hash");
    assert_eq!(hash.len(), 16);
    assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
    assert_eq!(cap["token_count"], 550);
    assert_eq!(cap["limit"], 512);
    assert_eq!(cap["model"], "intfloat/multilingual-e5-large");
    assert_eq!(
        cap["tokenizer"],
        "intfloat/multilingual-e5-large/tokenizer.json"
    );
    assert_eq!(cap["refactoring_required"], true);
    assert!(cap["guidance"].as_str().is_some_and(|guidance| {
        guidance.contains("shorten") && guidance.contains("split") && guidance.contains("rerun")
    }));
}

#[then("stdout identifies the over-cap scenario as an E5 token-cap refactoring finding")]
fn stdout_identifies_e5_cap(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(stdout.contains("E5_TOKEN_CAP"));
    assert!(stdout.contains(CAPPED_TITLE));
    assert!(stdout.contains("refactoring required: true"));
}
