//! Real compiled-CLI E2E steps for bounded NLI evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("the NLI E2E strict-mode setup step")]
fn nli_e2e_strict_mode_setup(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step skeleton is implemented scenario by scenario");
}

#[given("an end-to-end project with two permanent semantic candidates for NLI evidence")]
fn e2e_nli_permanent_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove E2E NLI specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create E2E NLI specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write E2E NLI permanent candidates");
    world.env = Some(env);
}

#[given("NLI evidence is enabled while BGE evidence is disabled")]
fn e2e_nli_enabled(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("a complete draft scenario is available for NLI exploration")]
fn e2e_nli_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E NLI permanent project");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write E2E NLI draft");
}

#[when("the end-to-end user explores the draft with NLI through the givn CLI")]
fn e2e_nli_explore_permanent(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E NLI permanent project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
        ],
    ));
}

#[then("stdout identifies E5 as the raw candidate generator")]
fn e2e_nli_identifies_e5(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("NLI candidate generator: E5"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("stdout identifies the NLI model and both directional probability sets")]
fn e2e_nli_reports_directional(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("NLI model: MoritzLaurer"),
        "stdout: {output}"
    );
    assert!(
        output.contains("NLI forward probabilities"),
        "stdout: {output}"
    );
    assert!(
        output.contains("NLI reverse probabilities"),
        "stdout: {output}"
    );
}

#[then("stdout reports fewer NLI recommended candidates than E5 candidates")]
fn e2e_nli_reduces_recommendations(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let line = output
        .lines()
        .find(|line| line.contains("NLI candidate generator:"))
        .expect("NLI candidate accounting");
    let raw = line
        .split("raw ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("NLI raw count");
    let recommended = line
        .split("recommended ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse::<usize>().ok())
        .expect("NLI recommended count");
    assert!(raw > recommended, "expected reduction: {output}");
}

#[then("stdout keeps the E5 raw candidate identities visible")]
fn e2e_nli_keeps_raw_candidates(world: &mut GivnWorld) {
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

#[given(
    "an end-to-end project with permanent and active-change semantic candidates for NLI evidence"
)]
fn e2e_nli_change_project(_world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove E2E NLI change specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create E2E NLI change specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Permanent provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Audit storage quota is enforced\n    Given an isolated archive ledger\n    When the ledger reaches capacity\n    Then the retention policy rejects the write\n",
    )
    .expect("write E2E NLI permanent change candidates");
    fs::create_dir_all(env.path("givn/changes/current-change/specs"))
        .expect("create E2E NLI change directory");
    fs::write(
        env.path("givn/changes/current-change/specs/current.feature"),
        "@givn.delta @current-change\nFeature: current change\n\n  @givn.added\n  Scenario: Current audit storage quota\n    Given an isolated archive ledger\n    When the ledger reaches capacity\n    Then the retention policy rejects the write\n",
    )
    .expect("write E2E NLI active candidate");
    fs::write(
        env.path("givn/changes/current-change/specs/query.feature"),
        "@givn.delta @current-change\nFeature: current query\n\n  @givn.added\n  Scenario: Current provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write E2E NLI query delta");
    _world.env = Some(env);
}

#[given("a complete draft scenario is available for NLI change-aware exploration")]
fn e2e_nli_change_draft(_world: &mut GivnWorld) {
    assert!(
        _world
            .env
            .as_ref()
            .expect("E2E NLI change project")
            .path("givn/changes/current-change/specs/query.feature")
            .is_file()
    );
}

#[when(
    "the end-to-end user explores the draft against change \"current-change\" with NLI through the givn CLI"
)]
fn e2e_nli_explore_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E NLI change project");
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
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
        ],
    ));
}

#[then("stdout contains both a permanent and a current-change NLI candidate")]
fn e2e_nli_change_has_both_scopes(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("scope permanent"), "stdout: {output}");
    assert!(output.contains("scope current-change"), "stdout: {output}");
}

#[given("an end-to-end project with an active semantic review candidate for NLI evidence")]
fn e2e_nli_advisory_project(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[when("the end-to-end user runs advisory NLI review through the givn CLI")]
fn e2e_nli_advisory_review(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("stdout reports the NLI model and elapsed runtime")]
fn e2e_nli_advisory_runtime(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("stdout records the filtered candidate as NLI evidence")]
fn e2e_nli_advisory_filtered(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("stdout assigns no automatic Givn classification")]
fn e2e_nli_advisory_no_classification(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[given("an end-to-end project whose active semantic candidates are all filtered by NLI evidence")]
fn e2e_nli_blocking_project(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[when("the end-to-end user runs blocking NLI review through the givn CLI")]
fn e2e_nli_blocking_review(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("the semantic review report has NLI status enabled")]
fn e2e_nli_report_enabled(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("the semantic review report records the filtered candidates without a Givn classification")]
fn e2e_nli_report_filtered(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[given("an end-to-end permanent corpus with deterministic overlaps and NLI semantic candidates")]
fn e2e_nli_duplicate_project(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[when("the end-to-end user runs the permanent NLI duplicate audit through the givn CLI")]
fn e2e_nli_duplicate_audit(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("the audit reports the E5 candidate count and bounded NLI pair count")]
fn e2e_nli_duplicate_counts(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("the audit reports reduced recommended and filtered NLI candidate counts")]
fn e2e_nli_duplicate_reduced(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}

#[then("deterministic exact and structural overlaps have no NLI evidence")]
fn e2e_nli_duplicate_deterministic(_world: &mut GivnWorld) {
    unimplemented!("NLI E2E step is implemented scenario by scenario");
}
