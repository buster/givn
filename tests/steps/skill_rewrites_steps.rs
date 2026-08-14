use cucumber::when;

use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("a project root is required first")
}

#[when("the user reads the design and steps instructions for skill-rewrites")]
fn read_design_and_steps_instructions(world: &mut GivnWorld) {
    let design = common::run_givn(
        env(world),
        &["instructions", "design", "--change", "skill-rewrites"],
    );
    let steps = common::run_givn(
        env(world),
        &["instructions", "tasks", "--change", "skill-rewrites"],
    );
    assert!(design.status.success(), "design instructions failed: {:?}", design);
    assert!(steps.status.success(), "tasks instructions failed: {:?}", steps);
    let mut combined = String::from_utf8_lossy(&design.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&steps.stdout));
    world.additional_output = Some(combined);
}
