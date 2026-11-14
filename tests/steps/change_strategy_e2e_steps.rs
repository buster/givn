//! E2E step definitions for the `change-strategy` capability.
//!
//! The scenario requests the served design guidance on an `ise`-enabled
//! project with one active change and asserts the change-strategy block; it
//! never reads the asset.

use cucumber::{then, when};

use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps as serving;

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

#[then("the served design guidance requires exactly one change strategy as a `Decides D` entry")]
fn served_design_requires_one_change_strategy(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("Record exactly one `change strategy`")
            && content.contains("as a `Decides D<n>` entry"),
        "the served design guidance does not require one change strategy in the Decisions ledger: {content}"
    );
}

#[then(
    regex = r#"^the served design guidance names "([^"]+)", "([^"]+)", and a whole-product rebuild$"#
)]
fn served_design_names_the_three_strategies(
    world: &mut GivnWorld,
    incremental: String,
    constant_ise: String,
) {
    let content = served_guidance(world);
    assert!(
        content.contains(&format!("`{incremental}`"))
            && content.contains(&format!("`{constant_ise}`"))
            && content.contains("whole-product rebuild"),
        "the served design guidance does not name the three change strategies: {content}"
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
