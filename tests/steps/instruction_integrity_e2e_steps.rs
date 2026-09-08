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

#[then("the CLI output contains the deterministic gate dispositions guidance")]
fn cli_contains_dispositions_guidance(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("## Deterministic gate dispositions"),
        "review guidance missing deterministic gate dispositions section"
    );
    assert!(
        output.contains("Overlap") && output.contains("dispositions"),
        "review guidance missing overlap dispositions table requirement"
    );
    assert!(
        output.contains("split-or-keep"),
        "review guidance missing split-or-keep decision requirement"
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
