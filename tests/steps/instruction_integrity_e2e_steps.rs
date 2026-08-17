//! E2E step-definition skeleton for the instruction-integrity capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(output.status.success(), "CLI command failed");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn active_project(world: &GivnWorld) -> &crate::common::TestEnv {
    world.env.as_ref().expect("E2E project is missing")
}

#[given("a Givn project with an active change")]
fn givn_project_with_active_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "instruction-integrity"]);
    assert!(output.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("the author requests the specs instructions through the Givn CLI")]
fn author_requests_specs_instructions(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(
            env,
            &["instructions", "specs", "--change", "instruction-integrity"],
        )
    };
    world.last_output = Some(output);
}

#[then("the CLI output contains the complete interaction and E2E policy")]
fn cli_contains_complete_policy(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(output.contains("Primary assertion rule"));
    assert!(output.contains("Real-interface driving rule"));
    assert!(output.contains("Interface obstacle rule"));
}

#[then("the CLI output distinguishes available and unavailable retrieval behavior")]
fn cli_distinguishes_retrieval_behavior(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(output.contains("givn spec index"));
    assert!(output.contains("retrieval-unavailable"));
}

#[when("the author requests the design instructions through the Givn CLI")]
fn author_requests_design_instructions(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(
            env,
            &[
                "instructions",
                "design",
                "--change",
                "instruction-integrity",
            ],
        )
    };
    world.last_output = Some(output);
}

#[then("the CLI output contains every required E2E design decision")]
fn cli_contains_e2e_design_decisions(world: &mut GivnWorld) {
    let output = stdout(world);
    for required in [
        "E2E smoke test infrastructure",
        "verify.e2e_command",
        "Local test infrastructure",
        "E2E framework choice and justification",
        "Interaction Coverage Matrix",
    ] {
        assert!(
            output.contains(required),
            "design guidance missing {required}"
        );
    }
}

#[when("the reviewer requests the review instructions through the Givn CLI")]
fn reviewer_requests_review_instructions(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(
            env,
            &[
                "instructions",
                "review",
                "--change",
                "instruction-integrity",
            ],
        )
    };
    world.last_output = Some(output);
}

#[then("the CLI output contains semantic classification and remediation checks")]
fn cli_contains_remediation_checks(world: &mut GivnWorld) {
    let output = stdout(world);
    for required in [
        "Semantic remediation verification",
        "DUPLIKAT",
        "VARIANTE",
        "VALID-BOUNDARY",
    ] {
        assert!(
            output.contains(required),
            "review guidance missing {required}"
        );
    }
    assert!(output.contains("semantic"));
    assert!(output.contains("remediation verified"));
}

#[when("the agent requests the specs instructions as JSON through the Givn CLI")]
fn agent_requests_specs_json(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(
            env,
            &[
                "instructions",
                "specs",
                "--change",
                "instruction-integrity",
                "--json",
            ],
        )
    };
    world.last_output = Some(output);
}

#[then("the response is valid JSON with artifact paths prerequisites and instructions")]
fn response_is_valid_instruction_json(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("JSON instruction command was not run");
    assert!(output.status.success());
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("instruction output is not JSON");
    assert_eq!(value["id"], "specs");
    assert!(value["generates"].is_array());
    assert!(value["requires"].is_array());
    assert!(value["instruction"].is_string());
}
