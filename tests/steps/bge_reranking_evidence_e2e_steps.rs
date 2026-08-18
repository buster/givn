//! Real compiled-CLI E2E steps for bounded BGE evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("an end-to-end project with a permanent semantic candidate for second-stage evidence")]
fn bge_e2e_permanent_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated E2E specs");
    fs::create_dir_all(&specs).expect("create E2E permanent specs");
    fs::write(
        specs.join("provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write E2E permanent corpus");
    world.env = Some(env);
}

#[given("an end-to-end project with permanent and active-change semantic candidates")]
fn bge_e2e_change_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated change-aware specs");
    fs::create_dir_all(&specs).expect("create change-aware permanent specs");
    fs::write(
        specs.join("provider.feature"),
        "Feature: provider\n\n  Scenario: Permanent provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Permanent provider warning\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write change-aware permanent corpus");
    let active = env.path("givn/changes/current-change/specs/current.feature");
    fs::create_dir_all(active.parent().expect("change-aware delta parent"))
        .expect("create change-aware delta");
    fs::write(
        &active,
        "@givn.delta @current-change\nFeature: current change\n\n  @givn.added\n  Scenario: Current provider warning\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write change-aware active corpus");
    fs::write(
        env.path("givn/changes/current-change/specs/query.feature"),
        "@givn.delta @current-change\nFeature: current query\n\n  @givn.added\n  Scenario: Current provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write change-aware query corpus");
    world.env = Some(env);
}

#[given("an end-to-end permanent corpus with deterministic overlaps and many semantic candidates")]
fn bge_e2e_duplicate_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated duplicate specs");
    fs::create_dir_all(&specs).expect("create duplicate specs");
    fs::write(
        specs.join("exact.feature"),
        "Feature: exact\n\n  Scenario: Exact left\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Exact right\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write E2E exact overlaps");
    fs::write(
        specs.join("structural.feature"),
        "Feature: structural\n\n  Scenario: Structural left\n    Given a configured provider catalog\n    When the provider command is run normally\n    Then the command exits 0 and output contains ready\n\n  Scenario: Structural right\n    Given a configured provider catalog\n    When the provider command is run in TTY mode\n    Then the command exits 2 and output does not contain ready\n",
    )
    .expect("write E2E structural overlaps");
    fs::write(
        specs.join("semantic.feature"),
        "Feature: semantic\n\n  Scenario: Token is required\n    Given a client request\n    When the client sends a request without credentials\n    Then the server responds with unauthorized status\n\n  Scenario: Authentication needs a token\n    Given an identity service\n    When the client authenticates without a token\n    Then access is denied with token required\n",
    )
    .expect("write E2E semantic candidates");
    world.env = Some(env);
    world.retrieval_duplicate_scores = Some(
        "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600".to_string(),
    );
    world.retrieval_bge_scores = Some(
        "givn/specs/semantic.feature|Token is required->givn/specs/semantic.feature|Authentication needs a token=1.0000;givn/specs/semantic.feature|Authentication needs a token->givn/specs/semantic.feature|Token is required=1.0000".to_string(),
    );
}

#[when("the end-to-end user explores the draft through the givn CLI")]
fn e2e_explore_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E permanent project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")],
    ));
}

#[when(
    "the end-to-end user explores the draft against change \"current-change\" through the givn CLI"
)]
fn e2e_explore_change_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E change-aware project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--change",
            "current-change",
            "--file",
            "givn/changes/current-change/specs/query.feature",
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")],
    ));
}

#[when(
    "the end-to-end user runs advisory change review for \"running-change\" through the givn CLI"
)]
fn e2e_run_advisory_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E advisory project");
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

#[when("the end-to-end user runs the permanent duplicate audit through the givn CLI")]
fn e2e_run_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E duplicate project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_duplicate_scores.as_deref() {
        environments.push(("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates"],
        &environments,
    ));
}

#[then("stdout identifies the E5 candidate generator")]
fn e2e_stdout_identifies_e5(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE candidate generator: E5"));
}

#[then("stdout identifies the second-stage model")]
fn e2e_stdout_identifies_bge(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE model: rozgo/bge-reranker-v2-m3"));
}

#[then("stdout reports a symmetric raw score and elapsed runtime")]
fn e2e_stdout_reports_score_and_runtime(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE symmetric raw score"));
    assert!(output.contains("elapsed runtime ms"));
}

#[then("stdout reports that the second-stage candidate pool is bounded")]
fn e2e_stdout_reports_bounded_pool(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("bounded pool"));
}

#[then("stdout contains both a permanent and a current-change candidate")]
fn e2e_change_contains_both_scopes(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("scope permanent"), "stdout: {output}");
    assert!(output.contains("scope current-change"), "stdout: {output}");
}

#[then("stdout reports one bounded second-stage pool and a reduced recommended list for the query")]
fn e2e_change_reports_reduced_recommendations(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let accounting = output
        .lines()
        .find(|line| line.contains("BGE candidate generator:"))
        .expect("BGE candidate accounting");
    let raw = accounting
        .split("raw ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("raw candidate count");
    let recommended = accounting
        .split("recommended ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("recommended candidate count");
    assert!(
        raw > recommended,
        "expected reduced recommendations: {output}"
    );
    assert!(output.contains("bounded pool"), "stdout: {output}");
}

#[then("stdout excludes the query's own delta and projected target identities")]
fn e2e_change_excludes_query_identity(world: &mut GivnWorld) {
    assert!(
        !super::stdout(world)
            .lines()
            .filter(|line| line.starts_with("rank "))
            .any(|line| line.contains("Current provider request"))
    );
}

#[then("stdout identifies the second-stage model, score, and elapsed runtime")]
fn e2e_advisory_reports_bge_evidence(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE: model"), "stdout: {output}");
    assert!(output.contains("symmetric"), "stdout: {output}");
    assert!(output.contains("runtime ms"), "stdout: {output}");
}

#[then("stdout reports fewer recommended candidates than E5 candidates")]
fn e2e_advisory_reduces_recommendations(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let raw = output
        .lines()
        .find(|line| line.starts_with("BGE: model"))
        .and_then(|line| line.split("E5 raw ").nth(1))
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("E5 raw count");
    let recommended = output
        .lines()
        .find(|line| line.starts_with("BGE candidate accounting"))
        .and_then(|line| line.split("recommended ").nth(1))
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("recommended count");
    assert!(raw > recommended, "stdout: {output}");
}

#[then("stdout says that the evidence is advisory")]
fn e2e_advisory_is_advisory(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("evidence is advisory"));
}

#[then("stdout contains no automatic Givn classification")]
fn e2e_advisory_has_no_classification(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for classification in ["DUPLIKAT", "VARIANTE", "VALID-BOUNDARY", "UNCERTAIN"] {
        assert!(!output.contains(classification), "stdout: {output}");
    }
}

#[then("the audit reports the E5 candidate count and bounded second-stage pair count")]
fn e2e_duplicate_reports_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE E5 raw count:"));
    assert!(output.contains("BGE unique pair count:"));
}

#[then("the audit reports the reduced recommended candidate count and filtered candidate count")]
fn e2e_duplicate_reports_admission_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE recommended count:"));
    assert!(output.contains("BGE filtered count:"));
}

#[then("the audit reports the second-stage model identity, symmetric score, and elapsed runtime")]
fn e2e_duplicate_reports_model_score_runtime(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE model: rozgo/bge-reranker-v2-m3"));
    assert!(output.contains("bge_symmetric_raw="));
    assert!(output.contains("BGE runtime ms:"));
}

#[then("deterministic exact and structural overlaps are absent from second-stage scoring")]
fn e2e_duplicate_excludes_deterministic_bge(world: &mut GivnWorld) {
    for line in super::stdout(world).lines() {
        if line.contains("kind=exact_duplicate") || line.contains("kind=structural_overlap") {
            assert!(!line.contains("bge_status="), "deterministic line: {line}");
        }
    }
}

#[then("the second-stage pair count is no greater than the documented endpoint-neighbour bound")]
fn e2e_duplicate_respects_endpoint_bound(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let scenarios = output
        .lines()
        .find_map(|line| line.strip_prefix("Scanned scenarios: "))
        .and_then(|value| value.parse::<usize>().ok())
        .expect("scanned scenario count");
    let top_k = output
        .lines()
        .find_map(|line| line.strip_prefix("BGE top_k: "))
        .and_then(|value| value.parse::<usize>().ok())
        .expect("BGE top-k");
    let pairs = output
        .lines()
        .find_map(|line| line.strip_prefix("BGE unique pair count: "))
        .and_then(|value| value.parse::<usize>().ok())
        .expect("BGE pair count");
    assert!(pairs <= scenarios * top_k, "stdout: {output}");
}

#[then("the command exits non-zero because the candidate is unresolved")]
fn e2e_blocking_review_is_unresolved(world: &mut GivnWorld) {
    assert!(!super::output(world).status.success());
}

fn e2e_semantic_report(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("E2E semantic project");
    fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read E2E semantic report")
}

#[then("the semantic review report contains the second-stage model identity")]
fn e2e_report_contains_bge_model(world: &mut GivnWorld) {
    assert!(e2e_semantic_report(world).contains("BGE model: rozgo/bge-reranker-v2-m3"));
}

#[then("the semantic review report contains the symmetric raw score and elapsed runtime")]
fn e2e_report_contains_bge_score_and_runtime(world: &mut GivnWorld) {
    let report = e2e_semantic_report(world);
    assert!(report.contains("bge_symmetric_raw:"));
    assert!(report.contains("BGE runtime ms:"));
}

#[then("the semantic review report records E5 candidates filtered from the recommended worklist")]
fn e2e_report_records_filtered_candidates(world: &mut GivnWorld) {
    let report = e2e_semantic_report(world);
    assert!(report.contains("BGE filtered count:"));
    assert!(
        report.contains("## BGE Filtered Candidate"),
        "report:\n{report}"
    );
}

#[then("the semantic review report keeps the candidate classification unresolved")]
fn e2e_report_keeps_unresolved_classification(world: &mut GivnWorld) {
    assert!(e2e_semantic_report(world).contains("classification: UNCERTAIN"));
}

#[then("the active and permanent specifications remain unchanged")]
fn e2e_specs_remain_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E semantic project");
    for (path, before) in world
        .permanent_spec_snapshot
        .as_ref()
        .expect("permanent snapshot")
        .iter()
        .chain(
            world
                .active_change_snapshot
                .as_ref()
                .expect("active snapshot")
                .iter(),
        )
    {
        assert_eq!(&fs::read(env.path(path)).unwrap(), before, "changed {path}");
    }
}
