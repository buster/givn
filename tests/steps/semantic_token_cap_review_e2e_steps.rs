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
    if let Some(fixture) = world.retrieval_e5_token_budget.as_deref() {
        world.last_output = Some(common::run_givn_with_envs(
            env,
            &["spec", "duplicates", "--format", "json"],
            &[
                ("GIVN_SPEC_EMBEDDER", "stub"),
                ("GIVN_SPEC_E5_TOKEN_BUDGET", fixture),
            ],
        ));
        return;
    }
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
fn e2e_unresolved_semantic_candidate(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    std::fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    std::fs::create_dir_all(specs.join("permanent")).expect("create permanent specs");
    std::fs::write(
        env.path("givn/specs/permanent/permanent.feature"),
        "Feature: permanent token budget\n\n  Scenario: Permanent semantic candidate\n    Given permanent alpha context\n    When permanent alpha action runs\n    Then permanent alpha result is complete\n",
    )
    .expect("write permanent semantic candidate");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    std::fs::create_dir_all(delta.parent().expect("running change specs parent"))
        .expect("create running change specs");
    std::fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running token budget\n\n  @givn.added\n  Scenario: Unresolved active semantic candidate\n    Given active alpha context\n    When active alpha action runs\n    Then active alpha result is complete\n",
    )
    .expect("write active semantic candidate");
    std::fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unresolved active semantic candidate` | `Permanent semantic candidate` | related |\n",
    )
    .expect("write running change review");
    std::fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write deterministic E2E commands");
    world.env = Some(env);
    world.retrieval_reranker = Some("nli-stub".to_string());
    world.retrieval_stub_scores = Some(
        "Unresolved active semantic candidate|Permanent semantic candidate=0.9600".to_string(),
    );
}

#[given("its complete serialized pair encodes to 513 BGE tokens and 513 NLI tokens")]
fn e2e_bge_nli_over_cap_pair(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the blocking E2E project is missing");
    world.retrieval_bge_token_budget = Some("overflow-513".to_string());
    world.retrieval_nli_token_budget = Some("overflow-513".to_string());
}

fn semantic_report(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("blocking E2E project is missing");
    std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report")
}

#[then("the command exits non-zero because the semantic candidate remains unresolved")]
fn e2e_blocking_review_fails_unresolved(world: &mut GivnWorld) {
    assert!(!super::output(world).status.success());
}

#[then(
    "the semantic review report records BGE status \"BGE_UNAVAILABLE\" with reason \"token-cap\" because no BGE pair was scoreable"
)]
fn e2e_report_bge_token_cap_unavailable(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("BGE status: BGE_UNAVAILABLE"));
    assert!(report.contains("BGE reason: token-cap"));
    assert!(report.contains("BGE unique pair count: 0"));
    assert!(report.contains("BGE model load attempts: 0"));
}

#[then(
    "the semantic review report records NLI status \"NLI_UNAVAILABLE\" with reason \"token-cap\" because no NLI pair was scoreable"
)]
fn e2e_report_nli_token_cap_unavailable(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("NLI status: NLI_UNAVAILABLE"));
    assert!(report.contains("NLI reason: token-cap"));
    assert!(report.contains("NLI unique pair count: 0"));
    assert!(report.contains("NLI model load attempts: 0"));
}

#[then("the candidate records BGE admission \"BGE_TOKEN_CAP\" and NLI admission \"NLI_TOKEN_CAP\"")]
fn e2e_report_records_bge_nli_cap_admissions(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("bge_admission: BGE_TOKEN_CAP"));
    assert!(report.contains("nli_admission: NLI_TOKEN_CAP"));
}

#[then(
    "the semantic review report records both scenario token counts, each pair count of 513, and each limit of 512"
)]
fn e2e_report_records_pair_budgets(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("bge_left_tokens: 257"));
    assert!(report.contains("bge_right_tokens: 255"));
    assert!(report.contains("bge_pair_tokens: 513"));
    assert!(report.contains("bge_limit: 512"));
    assert!(report.contains("nli_left_tokens: 257"));
    assert!(report.contains("nli_right_tokens: 255"));
    assert!(report.contains("nli_pair_tokens: 513"));
    assert!(report.contains("nli_limit: 512"));
}

#[then(
    "the semantic review report tells the agent to shorten or split the scenarios before rerunning review"
)]
fn e2e_report_has_refactoring_guidance(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("refactoring required: shorten or split"));
    assert!(report.contains("rerun semantic review"));
}

#[then("the candidate remains `UNCERTAIN` and has no automatic Givn classification")]
fn e2e_candidate_remains_uncertain(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("classification: UNCERTAIN"));
    assert!(!report.contains("classification: BGE_TOKEN_CAP"));
    assert!(!report.contains("classification: NLI_TOKEN_CAP"));
}
