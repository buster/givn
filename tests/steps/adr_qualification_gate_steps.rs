//! Step definitions for the ADR qualification guidance capability.

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

fn guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("ADR qualification guidance was not loaded")
}

#[given("the ADR qualification guidance is available")]
fn adr_qualification_guidance_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string("assets/addons/arc42-docs/instructions.md")
        .expect("read canonical Arc42 instruction");
    world.additional_output = Some(content);
}

#[when("an author reads the qualification gate")]
fn author_reads_qualification_gate(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance requires materially different alternatives")]
fn guidance_requires_materially_different_alternatives(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Real alternatives"));
    assert!(content.contains("materially different options"));
}

#[then("the guidance requires architectural impact")]
fn guidance_requires_architectural_impact(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Architectural impact"));
    assert!(content.contains("component/ownership boundary"));
}

#[then("the guidance requires a durable consequence")]
fn guidance_requires_durable_consequence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Durable consequence"));
    assert!(content.contains("migration"));
}

#[then("the guidance requires correct lower-level artifact classification")]
fn guidance_requires_lower_level_classification(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Lower-level artifact classification"));
    assert!(content.contains("complete home"));
}

#[then("the guidance requires a completed existing-ADR or duplicate check")]
fn guidance_requires_existing_adr_check(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Existing-ADR check"));
    assert!(content.contains("NEW_ADR"));
}

#[then("the guidance defines qualification as Boolean AND across all mandatory gates")]
fn guidance_defines_boolean_and_gate(world: &mut GivnWorld) {
    let content = guidance(world);
    for clause in [
        "qualification = alternatives",
        "AND architectural_impact",
        "AND durable_consequence",
        "AND lower_level_artifact",
        "AND existing_adr_check",
    ] {
        assert!(
            content.contains(clause),
            "qualification guidance missing {clause}"
        );
    }
}

#[when("an author reads the qualification criteria")]
fn author_reads_qualification_criteria(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance says `Must be shared` is supporting evidence only")]
fn guidance_says_shared_is_supporting_only(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`Must be shared` is supporting evidence only"));
}

#[then("the guidance says shared evidence cannot qualify a candidate by itself")]
fn guidance_rejects_shared_only_qualification(world: &mut GivnWorld) {
    assert!(guidance(world).contains("It can never qualify a candidate by itself"));
}

#[then("the guidance routes a non-qualifying candidate to one canonical artifact")]
fn guidance_routes_non_qualifying_candidate(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("If any mandatory dimension fails, do not create an ADR"));
    assert!(content.contains("Route the decision once\nto its canonical artifact"));
}
