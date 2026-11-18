//! Step definitions for the `ise-rebuild-decision` capability.
//!
//! The scenario requests the served design-review guidance on an `ise`-enabled
//! project and asserts the Ise rebuild decision block; it never reads the asset.

use cucumber::{then, when};

use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps as serving;

fn served_guidance(world: &GivnWorld) -> String {
    crate::steps::stdout(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[when("the consumer requests the design-review guidance")]
fn consumer_requests_design_review_guidance(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_output = Some(serving::request_change_guidance(env, "design-review"));
}

#[when("the consumer requests the design guidance for an active change")]
fn consumer_requests_design_guidance(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_output = Some(serving::request_change_guidance(env, "design"));
    world.last_command = Some("instructions".to_string());
}

#[then(
    "the served design guidance states that a Single-Component Rebuild with `Rebuilt code: ship` follows the working-tree sequence"
)]
fn served_design_names_the_working_tree_sequence(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains(
            "Single-Component Rebuild` with `Rebuilt code: ship` follows the working-tree sequence"
        ),
        "the served design guidance does not name the working-tree ship practice: {content}"
    );
}

#[then("the served design guidance names the `ise` instruction as the owner of that sequence")]
fn served_design_points_at_the_ise_instruction(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("working-tree sequence in `givn instructions ise`"),
        "the served design guidance does not name the ise instruction as the sequence owner: {content}"
    );
}

#[then(
    "the served design-review guidance keeps a rebuilding change with no recorded Rebuilt code line a finding"
)]
fn rebuilding_change_without_a_mode_is_a_finding(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("rebuilding change with no recorded `Rebuilt code` line"),
        "the served design-review guidance omits the missing-`Rebuilt code` requirement: {content}"
    );
    assert!(
        content.contains("is a finding"),
        "the served design-review guidance does not classify the missing mode: {content}"
    );
}

#[then("the served design-review guidance keeps an unresolved Ise rebuild decision a finding")]
fn unresolved_change_strategy_is_a_finding(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("unresolved or unrecorded decision"),
        "the served design-review guidance omits the unresolved decision: {content}"
    );
    assert!(
        content.contains("is a finding"),
        "the served design-review guidance does not classify the unresolved strategy: {content}"
    );
}
