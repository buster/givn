//! E2E step definitions for the ADR qualification guidance capability.
//!
//! E2E steps invoke the compiled Givn CLI as a real subprocess.

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

fn project(world: &GivnWorld) -> &crate::common::TestEnv {
    world
        .env
        .as_ref()
        .expect("ADR qualification E2E project is missing")
}

fn stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no ADR qualification CLI output was captured");
    assert!(output.status.success(), "CLI command failed");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[given("an initialized ADR qualification project with active change `adr-qualification-gate`")]
fn initialized_adr_qualification_project(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "adr-qualification-gate"]);
    assert!(
        output.status.success(),
        "create ADR qualification change failed"
    );
    world.env = Some(env);
}

#[when("the author requests the Arc42 ADR guidance through the Givn CLI")]
fn author_requests_arc42_adr_guidance(world: &mut GivnWorld) {
    let output = crate::common::run_givn(
        project(world),
        &[
            "instructions",
            "arc42-docs",
            "--change",
            "adr-qualification-gate",
        ],
    );
    world.last_output = Some(output);
}

#[then("the CLI exits with status 0")]
fn cli_exits_with_status_zero(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(output.status.success());
}

#[then("the CLI output contains the strict AND qualification gate")]
fn cli_output_contains_strict_gate(world: &mut GivnWorld) {
    assert!(stdout(world).contains("qualification = alternatives"));
}

#[then("the CLI output says that `Must be shared` is supporting evidence only")]
fn cli_output_says_shared_is_supporting(world: &mut GivnWorld) {
    assert!(stdout(world).contains("`Must be shared` is supporting evidence only"));
}

#[then(
    "the CLI output distinguishes `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, and `CANONICAL_ARTIFACT`"
)]
fn cli_output_distinguishes_routes(world: &mut GivnWorld) {
    let content = stdout(world);
    for route in [
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(content.contains(route), "CLI output missing {route}");
    }
}

#[when("the reviewer requests the `design-review` guidance through the Givn CLI")]
fn reviewer_requests_design_review_guidance(world: &mut GivnWorld) {
    let output = crate::common::run_givn(
        project(world),
        &[
            "instructions",
            "design-review",
            "--change",
            "adr-qualification-gate",
        ],
    );
    world.last_output = Some(output);
}

#[then("the CLI output requires every mandatory qualification dimension to pass")]
fn cli_output_requires_mandatory_dimensions(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(content.contains("mandatory gates"));
    assert!(content.contains("`PASS`"));
}

#[then("the CLI output requires the existing-ADR and duplicate check")]
fn cli_output_requires_existing_adr_check(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(content.contains("existing-ADR") || content.contains("duplicate"));
}

#[then("the CLI output says that qualification policy is guidance rather than runtime enforcement")]
fn cli_output_states_guidance_only(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(content.contains("not runtime"));
    assert!(content.contains("enforcement"));
}
