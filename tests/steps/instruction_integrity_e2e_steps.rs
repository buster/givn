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
