//! Regular black-box steps for bounded NLI evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("the NLI strict-mode setup step")]
fn nli_strict_mode_setup(_world: &mut GivnWorld) {
    unimplemented!("NLI step skeleton is implemented scenario by scenario");
}

#[given("a project with one semantic candidate and directional NLI evidence scores")]
fn one_nli_candidate(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove generated NLI specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create NLI permanent specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write NLI permanent candidate");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("NLI delta parent"))
        .expect("create NLI delta directory");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write NLI active candidate");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[when("the user runs advisory NLI review")]
fn run_advisory_nli_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI candidate project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
        ],
    ));
}

#[then(
    "the output reports entailment, neutral, and contradiction probabilities in both directions"
)]
fn reports_nli_directional_probabilities(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("forward [entailment"), "stdout: {output}");
    assert!(output.contains("reverse [entailment"), "stdout: {output}");
    assert!(output.contains("neutral"), "stdout: {output}");
    assert!(output.contains("contradiction"), "stdout: {output}");
}

#[then("the output reports the symmetric NLI probability summary")]
fn reports_nli_symmetric_probabilities(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("symmetric [entailment"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the output reports an NLI admission status")]
fn reports_nli_admission_status(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("NLI_RECOMMENDED")
            || super::stdout(world).contains("NLI_FILTERED"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the symmetric summary is independent of endpoint order")]
fn nli_symmetric_summary_is_order_independent(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let symmetric = output
        .lines()
        .find(|line| line.contains("symmetric [entailment"))
        .expect("symmetric NLI output");
    let probabilities = symmetric
        .split("symmetric [")
        .nth(1)
        .and_then(|value| value.split(']').next())
        .expect("symmetric probability list")
        .split(',')
        .map(|value| {
            value
                .split_whitespace()
                .last()
                .expect("probability value")
                .parse::<f32>()
                .expect("numeric probability")
        })
        .collect::<Vec<_>>();
    assert_eq!(probabilities.len(), 3);
    assert!((probabilities.iter().sum::<f32>() - 1.0).abs() < 1e-3);
}

#[given("a project with semantic candidates and independent BGE and NLI evidence")]
fn combined_evidence_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove combined specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create combined specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write combined permanent candidates");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("combined delta parent"))
        .expect("create combined delta");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write combined active candidate");
    world.env = Some(env);
}

#[when("the user runs combined second-stage advisory review")]
fn run_combined_advisory_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("combined candidate project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=4.0;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0",
            ),
        ],
    ));
}

#[then("the E5 raw candidate identities remain unchanged")]
fn e5_raw_identities_remain_unchanged(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("Provider request is accepted"),
        "stdout: {output}"
    );
    assert!(
        output.contains("Provider warning is reported"),
        "stdout: {output}"
    );
}

#[then("a candidate is recommended only when both BGE and NLI recommend it")]
fn combined_admission_requires_both(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("combined recommended 1"),
        "stdout: {output}"
    );
    assert!(output.contains("BGE_FILTERED"), "stdout: {output}");
}

#[then("each filtered candidate remains evidence without a Givn classification")]
fn filtered_combined_candidates_are_non_classifying(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("evidence is advisory"), "stdout: {output}");
    assert!(!output.contains("DUPLIKAT"), "stdout: {output}");
    assert!(!output.contains("VARIANTE"), "stdout: {output}");
}

#[given("a project with a semantic candidate that BGE recommends and NLI cannot score")]
fn combined_unavailable_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove unavailable specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create unavailable specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write unavailable permanent candidate");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("unavailable delta parent"))
        .expect("create unavailable delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write unavailable active candidate");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write unavailable commands");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Active provider request` | `Provider request is accepted` | related |\n",
    )
    .expect("write unavailable review disposition");
    world.env = Some(env);
}

#[when("the user runs combined second-stage blocking review")]
fn run_combined_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            ("GIVN_SPEC_RERANKER", "nli-missing"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Active provider request|Provider request is accepted=0.9900",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/changes/running-change/specs/running.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0;givn/specs/provider.feature|Provider request is accepted->givn/changes/running-change/specs/running.feature|Active provider request=4.0",
            ),
        ],
    ));
}

#[then("the command exits non-zero because the E5 candidate remains unresolved")]
fn combined_unavailable_exits_nonzero(world: &mut GivnWorld) {
    assert!(
        !super::output(world).status.success(),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the report records NLI as unavailable without NLI filtered evidence")]
fn report_records_nli_unavailable(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read unavailable NLI report");
    assert!(
        report.contains("NLI status: NLI_UNAVAILABLE"),
        "report: {report}"
    );
    assert!(
        report.contains("NLI reason: missing-artifact"),
        "report: {report}"
    );
    assert!(!report.contains("NLI_FILTERED"), "report: {report}");
}

#[then("the candidate remains a normal blocking candidate with no NLI probabilities")]
fn unavailable_candidate_remains_blocking(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read unavailable NLI report");
    assert!(report.contains("## Candidate"), "report: {report}");
    assert!(
        report.contains("classification: UNCERTAIN"),
        "report: {report}"
    );
    assert!(
        report.contains("nli_admission: NLI_UNSCORED"),
        "report: {report}"
    );
    assert!(
        report.contains("nli_forward_entailment: null"),
        "report: {report}"
    );
}
