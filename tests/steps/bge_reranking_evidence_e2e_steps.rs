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

#[when("the end-to-end user explores the draft through the givn CLI")]
fn e2e_explore_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E2E permanent project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
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
