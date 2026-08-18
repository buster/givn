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
