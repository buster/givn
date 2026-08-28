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
    unimplemented!("agent-synchronizes-skills");
    let _ = world;
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
    unimplemented!("generated-skill-names-domain-skill");
    let _ = world;
}

#[then("the generated skill loads before writing feature specs or the design")]
fn generated_skill_loads_before_specs_or_design(world: &mut GivnWorld) {
    unimplemented!("generated-skill-loads-before-specs-or-design");
    let _ = world;
}

#[then("the generated skill does not describe the givn change lifecycle")]
fn generated_skill_omits_givn_lifecycle(world: &mut GivnWorld) {
    unimplemented!("generated-skill-omits-givn-lifecycle");
    let _ = world;
}
