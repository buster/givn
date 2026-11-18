//! E2E step definitions for the `ise-rebuild-decision` capability.
//!
//! The scenario requests the served design guidance on an `ise`-enabled
//! project with one active change and asserts the change-strategy block; it
//! never reads the asset.

use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps as serving;
use cucumber::{then, when};

fn served_guidance(world: &GivnWorld) -> String {
    crate::steps::stdout(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
    "the served design guidance requires exactly one Ise rebuild decision as a `Decides D` entry"
)]
fn served_design_requires_one_change_strategy(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("Record exactly one `Ise rebuild decision`")
            && content.contains("as a `Decides D<n>` entry"),
        "the served design guidance does not require one Ise rebuild decision in the Decisions ledger: {content}"
    );
}

#[then(
    "the served design guidance names No rebuild, Single-Component Rebuild, and Whole-system rebuild"
)]
fn served_design_names_the_three_decisions(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("`No rebuild`")
            && content.contains("`Single-Component Rebuild`")
            && content.contains("`Whole-system rebuild`"),
        "the served design guidance does not name the three Ise rebuild decisions: {content}"
    );
}

#[then(
    "the served design guidance records a plain Rebuilt code line with discard or ship for a rebuilding change"
)]
fn served_design_records_rebuilt_code(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("records a plain `Rebuilt code` line")
            && content.contains("`discard`")
            && content.contains("`ship`"),
        "the served design guidance does not record what happens to rebuilt code: {content}"
    );
}

#[then(
    "the served design guidance requires the affected consumers, the persisted state, and the rebuild boundary"
)]
fn served_design_requires_rebuild_facts(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("the affected consumers")
            && content.contains("the persisted state")
            && content.contains("the rebuild boundary"),
        "the served design guidance does not require the rebuild facts: {content}"
    );
}
