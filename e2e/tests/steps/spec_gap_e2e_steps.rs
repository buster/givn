//! E2E step definitions for the `spec-gap` capability.
//!
//! The scenario requests the served review guidance on a synchronized
//! `ise`-enabled project (the shared Given from `change_strategy_e2e_steps`)
//! and asserts the spec-gap block; it never reads the asset.

use crate::steps::GivnWorld;
use cucumber::then;

fn served_guidance(world: &GivnWorld) -> String {
    crate::steps::stdout(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[then("the served review guidance requires exactly one spec gap outcome before any fix")]
fn served_review_requires_one_spec_gap_outcome(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("One `Spec gap` outcome before any fix")
            && content.contains("Record exactly one `Spec gap` outcome before any fix"),
        "the served review guidance does not require one spec gap outcome before any fix: {content}"
    );
}

#[then(
    regex = r#"^the served review guidance names "([^"]+)", "([^"]+)", "([^"]+)", and "([^"]+)"$"#
)]
fn served_review_names_the_four_outcomes(
    world: &mut GivnWorld,
    e2e_defect: String,
    implementation_defect: String,
    specification_defect: String,
    acceptable: String,
) {
    let content = served_guidance(world);
    for outcome in [
        e2e_defect,
        implementation_defect,
        specification_defect,
        acceptable,
    ] {
        assert!(
            content.contains(&format!("`{outcome}`")),
            "the served review guidance does not name the outcome `{outcome}`: {content}"
        );
    }
}

#[then(
    regex = r#"^the served review guidance states that only "([^"]+)" is a gap in the Specification$"#
)]
fn served_review_states_the_only_specification_gap(world: &mut GivnWorld, outcome: String) {
    let content = served_guidance(world);
    assert!(
        content.contains(&format!("Only `{outcome}` is a gap in the Specification")),
        "the served review guidance does not name `{outcome}` as the only Specification gap: {content}"
    );
}
