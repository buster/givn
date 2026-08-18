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
