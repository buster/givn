//! Step-definition skeleton for the domain-skill-trigger capability.
//!
//! Regular steps inspect the `givn-domain-modeling` skill template asset and
//! drive the CLI (addon-off and JSON variants) via `crate::harness::run_givn`.

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
    world.env.as_ref().expect("fixture project is missing")
}

const FIXTURE_CHANGE_ID: &str = "domain-skill-trigger";

#[given("a Givn project with the domain model addon enabled and a change awaiting specs")]
fn addon_enabled_project_with_change(world: &mut GivnWorld) {
    let env = crate::harness::make_init_root();
    let enable = crate::harness::run_givn(&env, &["addons", "enable", "domain_modeling"]);
    assert!(
        enable.status.success(),
        "addons enable domain_modeling failed"
    );
    let created = crate::harness::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("the agent requests the specs instructions through the Givn CLI")]
fn agent_requests_specs_instructions(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::harness::run_givn(
            env,
            &["instructions", "specs", "--change", FIXTURE_CHANGE_ID],
        )
    };
    world.last_output = Some(output);
}

#[then("the guidance says to load the domain-modeling skill now")]
fn guidance_says_load_domain_skill(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("Skill to load now: `givn-domain-modeling`"),
        "guidance does not say to load the domain-modeling skill now: {output}"
    );
}
