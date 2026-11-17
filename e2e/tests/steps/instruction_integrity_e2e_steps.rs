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

fn active_project(world: &GivnWorld) -> &crate::harness::TestEnv {
    world.env.as_ref().expect("E2E project is missing")
}

#[given("a Givn project with an active change")]
pub(crate) fn givn_project_with_active_change(world: &mut GivnWorld) {
    let env = crate::harness::make_init_root();
    let output = crate::harness::run_givn(&env, &["new", "instruction-integrity"]);
    assert!(output.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[given("an initialized Givn project without an active change")]
fn initialized_project_without_change(world: &mut GivnWorld) {
    world.env = Some(crate::harness::make_init_root());
}

#[when("the author requests the normal Givn startup instructions through the Givn CLI")]
fn author_requests_startup_instructions(world: &mut GivnWorld) {
    let env = active_project(world);
    world.last_output = Some(crate::harness::run_givn(env, &["instructions"]));
}

#[then(
    "the startup guidance requires loading the effective dialog policy before the context's first user-facing question"
)]
fn startup_requires_policy_before_first_question(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("load it once per context before the first question"),
        "startup guidance does not require loading the policy before the first question: {output}"
    );
}

#[when("the agent requests the specs instructions as JSON through the Givn CLI")]
fn agent_requests_specs_json(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::harness::run_givn(
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

#[then("the bootstrap owns that obligation independently of optional addons and named skills")]
fn bootstrap_owns_dialog_obligation(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("optional addons disabled")
            && output.contains("outside a named workflow")
            && output.contains("outside any named skill"),
        "the bootstrap does not own the addon- and skill-independent obligation: {output}"
    );
}

#[then("the policy is loaded once per context and reloaded if lost")]
fn policy_load_once_and_reload(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("load it once per context") && output.contains("not loaded automatically"),
        "the bootstrap does not state the load-once and reload-if-lost obligation: {output}"
    );
}

#[then("the terminal startup output recommends givn-start and the direct change command")]
fn terminal_startup_recommends_start(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("the startup command must run first");
    assert!(output.status.success(), "startup command failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("givn-start")
            && stdout.contains("givn new <change-id>")
            && stdout.contains("direct alternative"),
        "terminal startup output does not recommend start and the direct change command: {stdout}"
    );
}
