//! Regular black-box steps for the semantic review report.

use std::fs;

use cucumber::{given, then};

use super::GivnWorld;
use crate::common;

const PERMANENT_CORPUS: &str = "Feature: retrieval fixtures

  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Authentication token is missing
    Given a configured provider catalog
    When the user authenticates without a token
    Then authentication is rejected

  Scenario: Provider configuration is valid
    Given a configured provider catalog
    When the user selects a known provider
    Then the command accepts the request
";

#[given("a project prepared for semantic review with only a removed delta")]
fn removed_semantic_delta_project(_world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(specs.join("retrieval.feature"), PERMANENT_CORPUS)
        .expect("write permanent semantic fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.removed\n  Scenario: Removed scenario\n    Given a removed scenario\n",
    )
    .expect("write removed delta");
    _world.env = Some(env);
    _world.retrieval_embedder = Some("stub".to_string());
    _world.retrieval_reranker = Some("stub".to_string());
}

#[then("the semantic review report exists")]
fn semantic_report_exists(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    assert!(
        env.path("givn/changes/running-change/semantic-review.md")
            .is_file()
    );
}

#[then(regex = r#"^the semantic review report status is "([^"]+)"$"#)]
fn semantic_report_status(_world: &mut GivnWorld, _status: String) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains(&format!("Status: {}", _status)));
}

#[then("the semantic review report candidate count is 0")]
fn semantic_report_has_no_candidates(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains("Candidate count: 0"));
}

#[then(regex = r#"^the semantic review report does not mention "([^"]+)"$"#)]
fn semantic_report_does_not_mention(_world: &mut GivnWorld, text: String) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(!report.contains(&text), "semantic report contains {text:?}");
}

#[then("the permanent index remains structurally permanent-only")]
fn permanent_index_is_structurally_permanent(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let index =
        fs::read_to_string(env.path("givn/.spec-index.json")).expect("read permanent index");
    assert!(!index.contains("givn/changes/"));
    assert!(!index.contains("Delta"));
    let value: serde_json::Value = serde_json::from_str(&index).expect("parse permanent index");
    for entry in value["entries"].as_array().expect("index entries") {
        assert!(
            entry["file"]
                .as_str()
                .is_some_and(|file| file.starts_with("givn/specs/"))
        );
    }
}
