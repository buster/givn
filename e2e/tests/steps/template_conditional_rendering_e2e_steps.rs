//! End-to-end step definitions for the template-conditional-rendering delta.
//! These scenarios drive the real `givn` CLI toggles (`addons enable/disable`)
//! as subprocesses and then read the regenerated guidance. The fixture and
//! reading steps live in `template_conditional_rendering_steps` / the shared
//! support module.

use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::template_conditional_rendering_support as support;
use cucumber::{then, when};

#[when("a consumer enables the arc42 and domain-modeling addons through the Givn CLI")]
fn consumer_enable_domain_modeling_and_arc42(world: &mut GivnWorld) {
    for addon in ["arc42", "domain_modeling"] {
        let out = common::run_givn(support::env(world), &["addons", "enable", addon]);
        world.last_output = Some(out);
    }
}

#[then("the served review instruction omits the Ubiquitous language conformance section")]
fn served_review_instruction_omits_ubiquitous_language(world: &mut GivnWorld) {
    let content = support::served_artifact_instruction(world, "review");
    assert!(
        !content.contains("Ubiquitous language conformance"),
        "the served review instruction carries the ubiquitous-language section:\n{content}"
    );
}

#[then("the generated review skill carries no Ubiquitous language conformance section")]
fn generated_review_skill_carries_no_ubiquitous_language(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in &texts {
        assert!(
            !text.contains("Ubiquitous language conformance"),
            "generated review skill unexpectedly contains the ubiquitous-language section:\n{text}",
        );
    }
}

#[then("the served review instruction contains the Arc42 implementation conformance section")]
fn served_review_instruction_contains_arc42(world: &mut GivnWorld) {
    let content = support::served_artifact_instruction(world, "review");
    assert!(
        content.contains("Arc42 implementation conformance"),
        "the served review instruction is missing the Arc42 section:\n{content}"
    );
}

#[then("the served review instruction contains the Ubiquitous language conformance section")]
fn served_review_instruction_contains_ubiquitous_language(world: &mut GivnWorld) {
    let content = support::served_artifact_instruction(world, "review");
    assert!(
        content.contains("Ubiquitous language conformance"),
        "the served review instruction is missing the ubiquitous-language section:\n{content}"
    );
}

#[then(
    "the served design-review instruction contains the Architecture documentation (arc42) branch"
)]
fn served_design_review_instruction_contains_arc42(world: &mut GivnWorld) {
    let content = support::served_artifact_instruction(world, "design-review");
    assert!(
        content.contains("Architecture documentation (arc42)"),
        "the served design-review instruction is missing the arc42 branch:\n{content}"
    );
}
