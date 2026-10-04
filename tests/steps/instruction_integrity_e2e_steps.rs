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

#[then("the CLI output no longer advertises retrieval commands")]
fn cli_no_longer_advertises_retrieval(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        !output.contains("givn spec index"),
        "specs instruction still advertises `givn spec index` after semantic-gate retirement"
    );
    assert!(
        !output.contains("retrieval-unavailable"),
        "specs instruction still describes retrieval-unavailable behaviour"
    );
}

// Step kept for the @givn.removed placeholder in the removal delta (the
// scenario itself was removed with the semantic gate).

#[then("the CLI output contains semantic classification and remediation checks")]
fn cli_contains_remediation_checks(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        !output.contains("Semantic remediation verification"),
        "semantic remediation guidance must be gone"
    );
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
