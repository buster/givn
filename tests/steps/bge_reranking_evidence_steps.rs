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

#[given("the local second-stage evidence is enabled")]
fn bge_enabled(world: &mut GivnWorld) {
    world.retrieval_reranker = None;
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

#[when("the user explores the draft once with second-stage evidence disabled")]
fn explore_disabled_for_parity(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.previous_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "disabled")],
    ));
}

#[when("the user explores the same draft once with second-stage evidence enabled")]
fn explore_enabled_for_parity(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")],
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

#[given("a project with one permanent semantic pair and directional local evidence scores")]
fn directional_bge_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("pair.feature"),
        "Feature: pair\n\n  Scenario: Alpha request\n    Given an alpha context\n    When alpha runs\n    Then alpha succeeds\n\n  Scenario: Beta request\n    Given a beta context\n    When beta runs\n    Then beta succeeds\n",
    )
    .expect("write directional BGE fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/pair.feature|Alpha request::givn/specs/pair.feature|Beta request=0.9600"
            .to_string(),
    );
    world.retrieval_bge_scores = Some(
        "givn/specs/pair.feature|Alpha request->givn/specs/pair.feature|Beta request=2.0000;givn/specs/pair.feature|Beta request->givn/specs/pair.feature|Alpha request=4.0000".to_string(),
    );
}

#[when("the user runs the permanent duplicate audit with second-stage evidence enabled")]
fn run_directional_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("directional BGE project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_duplicate_scores.as_deref() {
        environments.push(("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &environments,
    ));
}

#[then("the audit reports both directional scores")]
fn audit_reports_directional_scores(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    let candidate = &document["candidates"][0];
    assert_eq!(candidate["evidence"]["bge"]["forward_raw"], serde_json::json!(2.0));
    assert_eq!(candidate["evidence"]["bge"]["reverse_raw"], serde_json::json!(4.0));
}

#[then("the reported raw score is their documented symmetric average")]
fn audit_reports_symmetric_score(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(document["candidates"][0]["evidence"]["bge"]["symmetric_raw"], serde_json::json!(3.0));
}

#[then("reversing endpoint order does not change the reported raw score")]
fn reversed_direction_is_symmetric(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("directional BGE project");
    let output = common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/pair.feature|Alpha request::givn/specs/pair.feature|Beta request=0.9600",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/pair.feature|Beta request->givn/specs/pair.feature|Alpha request=4.0000;givn/specs/pair.feature|Alpha request->givn/specs/pair.feature|Beta request=2.0000",
            ),
        ],
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(document["candidates"][0]["evidence"]["bge"]["symmetric_raw"], serde_json::json!(3.0));
}

#[given("a permanent corpus containing exact, structural, and semantic relationships")]
fn deterministic_exclusion_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("exact.feature"),
        "Feature: exact\n\n  Scenario: Exact left\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Exact right\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write exact fixture");
    fs::write(
        specs.join("structural.feature"),
        "Feature: structural\n\n  Scenario: Structural left\n    Given a configured provider catalog\n    When the provider command is run normally\n    Then the command exits 0 and output contains ready\n\n  @givn.done\n  Scenario: Structural right\n    Given a configured provider catalog\n    When the provider command is run in TTY mode\n    Then the command exits 2 and output does not contain ready\n",
    )
    .expect("write structural fixture");
    fs::write(
        specs.join("semantic.feature"),
        "Feature: semantic\n\n  Scenario: Token is required\n    Given a client request\n    When the client sends a request without credentials\n    Then the server responds with unauthorized status\n\n  Scenario: Authentication needs a token\n    Given an identity service\n    When the client authenticates without a token\n    Then access is denied with token required\n",
    )
    .expect("write semantic fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600".to_string(),
    );
    world.retrieval_bge_scores = Some(
        "givn/specs/semantic.feature|Token is required->givn/specs/semantic.feature|Authentication needs a token=4.0000;givn/specs/semantic.feature|Authentication needs a token->givn/specs/semantic.feature|Token is required=4.0000".to_string(),
    );
}

#[when("the user runs the permanent duplicate audit with JSON output")]
fn run_deterministic_exclusion_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("deterministic exclusion project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/semantic.feature|Token is required->givn/specs/semantic.feature|Authentication needs a token=4.0000;givn/specs/semantic.feature|Authentication needs a token->givn/specs/semantic.feature|Token is required=4.0000",
            ),
        ],
    ));
}

#[then("exact and structural relationships retain their deterministic evidence")]
fn deterministic_candidates_have_no_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    for candidate in document["candidates"].as_array().unwrap() {
        if candidate["kind"] == "exact_duplicate" || candidate["kind"] == "structural_overlap" {
            assert!(candidate["evidence"]["bge"].is_null());
        }
    }
}

#[then("only the eligible semantic neighbour union has second-stage evidence")]
fn only_semantic_candidates_have_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    for collection in ["candidates", "filtered_candidates", "unscored_candidates"] {
        if let Some(candidates) = document[collection].as_array() {
            for candidate in candidates {
                if !candidate["evidence"]["bge"].is_null() {
                    assert_eq!(candidate["kind"], "semantic_candidate");
                }
            }
        }
    }
}

#[then("no deterministic exclusion is counted as a second-stage pair")]
fn deterministic_pairs_excluded_from_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(1));
}

#[given("a project with two E5 candidates and one labeled false positive")]
fn advisory_bge_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write advisory permanent fixture");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().unwrap()).expect("create advisory delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write advisory delta");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[given("the local second-stage evidence gives the false positive a low relevance score")]
fn advisory_bge_scores(world: &mut GivnWorld) {
    world.retrieval_bge_scores = Some(
        "givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0000;givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=4.0000;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider warning is reported=1.0000;givn/specs/provider.feature|Provider warning is reported->givn/specs/running-change/running-change.feature|Active provider request=1.0000".to_string(),
    );
}

#[when("the user runs advisory change review with second-stage evidence enabled")]
fn run_advisory_bge_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("advisory BGE project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &environments,
    ));
}

#[then("the false positive is recorded as filtered from the recommended list")]
fn false_positive_is_filtered(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE_FILTERED"));
}

#[then("the false positive is not classified as `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`")]
fn filtered_candidate_has_no_classification(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for classification in ["DUPLIKAT", "VARIANTE", "VALID-BOUNDARY", "UNCERTAIN"] {
        assert!(!output.contains(classification), "filtered output contains {classification}");
    }
}

#[then("the remaining recommended candidate retains its E5 and second-stage evidence")]
fn recommended_candidate_retains_evidence(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE_RECOMMENDED"));
    assert!(output.contains("E5 "));
    assert!(output.contains("symmetric"));
}

#[then("both runs report the same E5 raw candidate identities in the same order")]
fn parity_raw_identities(world: &mut GivnWorld) {
    let before = world
        .previous_output
        .as_ref()
        .expect("disabled exploration output");
    let before = String::from_utf8_lossy(&before.stdout);
    let after = super::stdout(world);
    let ranks = |value: &str| {
        value
            .lines()
            .filter(|line| line.starts_with("rank "))
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    assert_eq!(ranks(&before), ranks(&after), "E5 raw identities changed");
}

#[then("the enabled run reports a reduced recommended candidate list when the evidence rejects a candidate")]
fn enabled_run_reduces_recommendations(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE filtered"), "missing BGE filtered accounting: {output}");
    assert!(output.contains("recommended 0"), "expected a reduced recommendation list: {output}");
}

#[then("only the enabled run reports participating second-stage evidence")]
fn only_enabled_reports_bge(world: &mut GivnWorld) {
    let before = world
        .previous_output
        .as_ref()
        .expect("disabled exploration output");
    assert!(!String::from_utf8_lossy(&before.stdout).contains("BGE"));
    assert!(super::stdout(world).contains("BGE"));
}
