//! Step definitions for the `change-strategy` capability.
//!
//! The scenario requests the served design-review guidance on an `ise`-enabled
//! project and asserts the change-strategy block; it never reads the asset.

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

#[then(
    "the served design-review guidance keeps a rebuilding change with no recorded Ise Test mode a finding"
)]
fn rebuilding_change_without_a_mode_is_a_finding(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("rebuilding change with no recorded `Ise Test` mode"),
        "the served design-review guidance omits the missing-mode requirement: {content}"
    );
    assert!(
        content.contains("is a finding"),
        "the served design-review guidance does not classify the missing mode: {content}"
    );
}

#[then("the served design-review guidance keeps an unresolved change strategy a finding")]
fn unresolved_change_strategy_is_a_finding(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("unresolved or unrecorded change strategy"),
        "the served design-review guidance omits the unresolved strategy: {content}"
    );
    assert!(
        content.contains("is a finding"),
        "the served design-review guidance does not classify the unresolved strategy: {content}"
    );
}
