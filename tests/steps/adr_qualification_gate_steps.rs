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

#[when("an author reads the lower-level artifact rules")]
fn author_reads_lower_level_artifact_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "lower-level artifact guidance is empty"
    );
}

#[then("the guidance distinguishes a feature specification from technical design")]
fn guidance_distinguishes_feature_from_design(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("observable behavior with no architectural trade-off"));
    assert!(content.contains("Gherkin specification"));
    assert!(content.contains("local implementation approach or algorithm detail"));
    assert!(content.contains("`design.md` and code"));
}

#[then("the guidance distinguishes Arc42 documentation from implementation code")]
fn guidance_distinguishes_arc42_from_code(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("current component/runtime/deployment fact"));
    assert!(content.contains("corresponding Arc42 chapter"));
}

#[then("the guidance distinguishes process guidance from architecture rationale")]
fn guidance_distinguishes_process_from_architecture(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("prompt, workflow, review, test, coverage, licensing, roadmap"));
    assert!(content.contains("owning process or project document"));
}

#[then("the guidance requires exactly one canonical target for a non-ADR decision")]
fn guidance_requires_one_canonical_target(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`canonical_artifact` naming exactly one destination"));
}

#[when("an author reads the duplicate and relationship rules")]
fn author_reads_duplicate_relationship_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "relationship guidance is empty"
    );
}

#[then("the guidance requires searching existing ADRs before drafting")]
fn guidance_requires_searching_existing_adrs(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("active and archived registers have been searched"));
    assert!(content.contains("before proposing a new ID"));
}

#[then("the guidance requires an explicit new amend supersede or canonical-artifact route")]
fn guidance_requires_explicit_route(world: &mut GivnWorld) {
    let content = guidance(world);
    for route in [
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(content.contains(route), "guidance missing route {route}");
    }
}

#[then("the guidance rejects a new ADR when the existing-ADR check is incomplete")]
fn guidance_rejects_incomplete_existing_check(world: &mut GivnWorld) {
    assert!(guidance(world).contains("An incomplete existing-ADR check fails"));
}

#[when("an author reads the structured verdict contract")]
fn author_reads_structured_verdict_contract(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "structured verdict guidance is empty"
    );
}

#[then("the verdict contract names all five mandatory gate fields")]
fn verdict_names_mandatory_gate_fields(world: &mut GivnWorld) {
    let content = guidance(world);
    for field in [
        "alternatives",
        "architectural_impact",
        "durable_consequence",
        "lower_level_artifact",
        "existing_adr_check",
    ] {
        assert!(content.contains(field), "verdict contract missing {field}");
    }
}

#[then("the verdict contract names the supporting-only `must_be_shared` field")]
fn verdict_names_supporting_field(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("must_be_shared"));
    assert!(content.contains("SUPPORTING"));
}

#[then("the verdict contract names `qualification` and `routing`")]
fn verdict_names_qualification_and_routing(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("qualification"));
    assert!(content.contains("routing"));
}

#[then("the verdict contract names `canonical_artifact`, `target_adr`, and `replacement_adr`")]
fn verdict_names_route_targets(world: &mut GivnWorld) {
    let content = guidance(world);
    for field in ["canonical_artifact", "target_adr", "replacement_adr"] {
        assert!(content.contains(field), "verdict contract missing {field}");
    }
}

#[then("the guidance defines the allowed qualification gate and routing values")]
fn guidance_defines_verdict_values(world: &mut GivnWorld) {
    let content = guidance(world);
    for value in [
        "QUALIFIED",
        "NOT_QUALIFIED",
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(
            content.contains(value),
            "guidance missing verdict value {value}"
        );
    }
}

#[when("an author reads the route-target rules")]
fn author_reads_route_target_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "route-target guidance is empty"
    );
}

#[then("`CANONICAL_ARTIFACT` requires a `canonical_artifact` target")]
fn canonical_artifact_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`CANONICAL_ARTIFACT` for a non-qualifying candidate"));
    assert!(content.contains("`canonical_artifact` naming exactly one destination"));
}

#[then("`AMEND_ADR` requires a `target_adr` target")]
fn amend_adr_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`AMEND_ADR` for a qualified refinement"));
    assert!(content.contains("`target_adr` naming the existing"));
}

#[then("`SUPERSEDE_ADR` requires a `replacement_adr` target")]
fn supersede_adr_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`SUPERSEDE_ADR` for a qualified replacement"));
    assert!(content.contains("`replacement_adr` naming"));
}

#[then("unused route-target fields are required to remain null")]
fn unused_route_targets_are_null(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("For `CANONICAL_ARTIFACT`"));
    assert!(content.contains("For\n`AMEND_ADR`") || content.contains("For `AMEND_ADR`"));
    assert!(content.contains("For\n`SUPERSEDE_ADR`") || content.contains("For `SUPERSEDE_ADR`"));
    assert!(content.matches("are `null`").count() >= 3);
    assert!(content.contains("`canonical_artifact` and `replacement_adr`"));
    assert!(content.contains("`canonical_artifact` and `target_adr`"));
}

#[when("an author reads the evidence rules")]
fn author_reads_evidence_rules(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "evidence guidance is empty");
}

#[then("the guidance requires evidence supplied by the author or source artifacts")]
fn guidance_requires_supplied_evidence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Evidence arrays contain only facts and"));
    assert!(content.contains("alternatives supplied by the author or source artifacts"));
}

#[then("the guidance says missing evidence remains empty")]
fn guidance_keeps_missing_evidence_empty(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Missing evidence stays"));
    assert!(content.contains("empty and causes its gate to fail"));
}

#[then("the guidance forbids inventing alternatives impact consequences or participants")]
fn guidance_forbids_invented_evidence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("must not invent alternatives"));
    assert!(content.contains("impact, consequences, participants, or provenance"));
}

#[when("an author reads the refinement rules")]
fn author_reads_refinement_rules(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "refinement guidance is empty");
}

#[then("the guidance routes a refinement to `AMEND_ADR`")]
fn guidance_routes_refinement_to_amend(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`AMEND_ADR` for a qualified refinement"));
}

#[then("the guidance requires the target ADR to be named")]
fn guidance_requires_target_adr(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`target_adr` naming the existing"));
}

#[then("the guidance forbids a parallel ADR for the same decision boundary")]
fn guidance_forbids_parallel_adr(world: &mut GivnWorld) {
    assert!(guidance(world).contains("must not create a parallel ADR for"));
}
