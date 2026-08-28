//! E2E step-definition skeleton for the domain-skill-trigger capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess: enable the
//! addon, create a change, synchronize skills, and read the generated skill.

use crate::steps::GivnWorld;
use cucumber::{then, when};

const FIXTURE_CHANGE_ID: &str = "domain-skill-trigger";

fn active_project(world: &GivnWorld) -> &crate::common::TestEnv {
    world.env.as_ref().expect("E2E project is missing")
}

fn generated_skill(world: &GivnWorld) -> String {
    let env = active_project(world);
    let path = env.path(".opencode/skills/givn-domain-modeling/SKILL.md");
    std::fs::read_to_string(&path).expect("generated domain skill file must exist after sync")
}

#[when("the agent synchronizes skills through the Givn CLI")]
fn agent_synchronizes_skills(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(env, &["skills", "sync"])
    };
    assert!(output.status.success(), "skills sync failed");
    world.last_output = Some(output);
}

#[when("the agent requests the next-step guidance through the Givn CLI")]
fn agent_requests_next_step_guidance(world: &mut GivnWorld) {
    unimplemented!("agent-requests-next-step-guidance");
    let _ = world;
}

#[when("the agent requests the design instructions through the Givn CLI")]
fn agent_requests_design_instructions(world: &mut GivnWorld) {
    unimplemented!("agent-requests-design-instructions");
    let _ = world;
}

#[then("the generated domain skill names the domain-modeling skill")]
fn generated_skill_names_domain_skill(world: &mut GivnWorld) {
    let content = generated_skill(world);
    assert!(
        content.contains("name: givn-domain-modeling"),
        "generated skill is not the domain-modeling skill"
    );
}

#[then("the generated skill loads before writing feature specs or the design")]
fn generated_skill_loads_before_specs_or_design(world: &mut GivnWorld) {
    let content = generated_skill(world);
    assert!(
        content.contains("before writing .feature specs or design.md"),
        "generated skill trigger does not cover the specs and design phases"
    );
}

#[then("the generated skill does not describe the givn change lifecycle")]
fn generated_skill_omits_givn_lifecycle(world: &mut GivnWorld) {
    let content = generated_skill(world);
    assert!(
        !content.contains("created -> planned -> implemented -> archived"),
        "generated skill still describes the givn change lifecycle"
    );
}
