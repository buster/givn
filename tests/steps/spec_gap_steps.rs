//! Step definitions for the `spec-gap` capability.
//!
//! The scenarios request the served proposal or review guidance on an
//! `ise`-enabled project; none reads the asset file.

use cucumber::{then, when};

use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps as serving;

fn request(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_output = Some(serving::request_change_guidance(env, id));
}

fn served_guidance(world: &GivnWorld) -> String {
    crate::steps::stdout(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[when("the consumer requests the proposal guidance for an active change")]
fn consumer_requests_proposal_guidance(world: &mut GivnWorld) {
    request(world, "proposal");
}

#[when("the consumer requests the review guidance for an active change")]
fn consumer_requests_review_guidance_for_an_active_change(world: &mut GivnWorld) {
    request(world, "review");
}

#[when("the consumer requests the review guidance")]
fn consumer_requests_review_guidance(world: &mut GivnWorld) {
    request(world, "review");
}

#[then("the served proposal guidance names the record and its outcome for a corrective change")]
fn proposal_names_the_record_and_outcome(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("`Spec gap` record and its single outcome"),
        "the served proposal guidance omits the record and its outcome: {content}"
    );
}

#[then(
    "the served proposal guidance sends \"implementation defect\" back to the agent with no corpus edit"
)]
fn proposal_returns_implementation_defect(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains(
            "`implementation defect` returns to the agent inside the Ise Test with no corpus edit"
        ),
        "the served proposal guidance omits the implementation-defect route: {content}"
    );
}

#[then(
    "the served proposal guidance carries the corpus or E2E suite edit for \"specification defect\" and \"E2E test defect\""
)]
fn proposal_carries_the_corpus_edit(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains(
            "`specification defect` or `E2E test defect` carries the corpus or E2E suite edit"
        ),
        "the served proposal guidance omits the corpus-edit route: {content}"
    );
}

#[then("the served proposal guidance records \"acceptable undocumented change\" in the corpus")]
fn proposal_records_acceptable_undocumented_change(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("`acceptable undocumented change` is recorded in the corpus"),
        "the served proposal guidance omits the corpus record: {content}"
    );
}

#[then(
    "the served proposal guidance requires the record to name the wrong artifact, not who found it"
)]
fn proposal_names_the_artifact_not_the_finder(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("names the artifact that is wrong, not who found it"),
        "the served proposal guidance does not name the artifact: {content}"
    );
}

#[then(
    "the served review guidance stops an Ise Test on \"E2E test defect\" and restarts it after the test is corrected"
)]
fn review_stops_the_trial_on_an_e2e_test_defect(world: &mut GivnWorld) {
    let content = served_guidance(world);
    for marker in [
        "`E2E test defect` stops the Ise Test",
        "restart from the corrected suite",
    ] {
        assert!(
            content.contains(marker),
            "the served review guidance omits {marker:?}: {content}"
        );
    }
}

#[then(
    "the served review guidance states that the corrected test is a Specification correction, not an exception to the freeze"
)]
fn review_calls_the_correction_a_specification_correction(world: &mut GivnWorld) {
    let content = served_guidance(world);
    assert!(
        content.contains("Specification correction, not an exception to the freeze"),
        "the served review guidance does not classify the corrected test: {content}"
    );
}
