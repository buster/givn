//! Step-definition skeleton for the instruction-integrity capability.
//!
//! RED scenarios intentionally remain undefined until each scenario enters its
//! own RED -> GREEN -> REFACTOR cycle.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn policy(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("canonical specs guidance was not loaded")
}

#[given("the canonical specs guidance is requested")]
fn canonical_specs_guidance_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string("assets/instructions/specs.md")
        .expect("read canonical specs instruction");
    world.additional_output = Some(content);
}

#[when("an author reads the interaction and E2E policy")]
fn author_reads_interaction_policy(world: &mut GivnWorld) {
    assert!(!policy(world).is_empty(), "canonical policy must not be empty");
}

#[then("the guidance requires primary assertions on the named real interface")]
fn guidance_requires_primary_interface_assertions(world: &mut GivnWorld) {
    let content = policy(world);
    assert!(
        content.contains("primary assertion is on what")
            && content.contains("the named real interface"),
        "canonical policy does not require primary interface assertions"
    );
}

#[then("the guidance rejects repository-only evidence for a higher-level interface")]
fn guidance_rejects_repository_only_evidence(world: &mut GivnWorld) {
    assert!(
        policy(world).contains("Repository/database checks are secondary"),
        "canonical policy does not reject repository-only evidence"
    );
}

#[then("the guidance requires a real browser for browser UI interactions")]
fn guidance_requires_real_browser(world: &mut GivnWorld) {
    let content = policy(world);
    assert!(
        content.contains("for a browser-UI capability, drive the actual")
            && content.contains("browser through a real browser driver"),
        "canonical policy does not require a real browser"
    );
}

#[then("the guidance treats an unresolved interface obstacle as a design gap")]
fn guidance_treats_obstacle_as_design_gap(world: &mut GivnWorld) {
    assert!(
        policy(world).contains("an interface obstacle is a design gap"),
        "canonical policy does not define the interface obstacle as a design gap"
    );
}

fn design_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("design guidance was not loaded")
}

#[given("the design instructions are requested")]
fn design_instructions_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string("assets/instructions/design.md")
        .expect("read design instruction");
    world.additional_output = Some(content);
}

#[when("an author prepares the technical design")]
fn author_prepares_design(world: &mut GivnWorld) {
    assert!(!design_guidance(world).is_empty(), "design guidance must not be empty");
}

#[then("the guidance requires the E2E runner and its tag isolation")]
fn design_requires_e2e_runner(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("verify.e2e_command"));
    assert!(content.contains("filtered to"));
    assert!(content.contains("`@e2e`"));
}

#[then("the guidance requires the step-definition location and driving mechanism")]
fn design_requires_step_location(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("E2E step definition location"));
    assert!(content.contains("driving mechanism"));
}

#[then("the guidance requires local infrastructure and framework decisions")]
fn design_requires_infrastructure(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("Local test infrastructure"));
    assert!(content.contains("E2E framework choice and justification"));
}

#[then("the guidance requires the interaction coverage matrix")]
fn design_requires_coverage_matrix(world: &mut GivnWorld) {
    assert!(design_guidance(world).contains("Interaction Coverage Matrix"));
}
