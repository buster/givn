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

// --- prepare-0-9-10: served concise qualification ---------------------------

#[then(
    "the output explains qualification by architectural commitment real alternatives lasting consequence and existing-ADR search"
)]
fn output_explains_concise_qualification(world: &mut GivnWorld) {
    let content = stdout(world);
    for marker in [
        "Architectural commitment",
        "Real alternatives",
        "Durable consequence",
        "search the active and archived ADR registers",
    ] {
        assert!(
            content.contains(marker),
            "served ADR guidance is missing {marker:?}: {content}"
        );
    }
}

#[then("the output routes the result to a new amended superseding or non-ADR artifact")]
fn output_routes_the_result(world: &mut GivnWorld) {
    let content = stdout(world);
    for route in ["NEW_ADR", "AMEND_ADR", "SUPERSEDE_ADR"] {
        assert!(
            content.contains(route),
            "served ADR guidance is missing route {route}: {content}"
        );
    }
    assert!(
        content.contains(
            "ordinary implementation choice belongs in its appropriate existing artifact"
        ),
        "served ADR guidance does not route non-ADR decisions: {content}"
    );
}

#[then("the output requires qualification once in the change assessment")]
fn output_requires_one_assessment(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(
        content.contains("Qualify a decision once, concisely, in the change assessment"),
        "served ADR guidance does not require one change-assessment qualification: {content}"
    );
    assert!(
        content.contains("Do not repeat a qualification worksheet"),
        "served ADR guidance permits repeated worksheets: {content}"
    );
}

#[then("the output directs the reviewer to the recorded qualification and existing-ADR search")]
fn output_directs_reviewer_to_recorded_qualification(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(
        content.contains("concise qualification recorded once"),
        "design-review guidance does not point to the recorded qualification: {content}"
    );
    assert!(
        content.contains("existing-decision search"),
        "design-review guidance does not require the existing-decision search: {content}"
    );
}

#[then("the output does not demand a duplicate qualification worksheet")]
fn output_does_not_demand_duplicate_worksheet(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(
        content.contains("Do not demand a duplicate qualification worksheet"),
        "design-review guidance does not reject a duplicate worksheet: {content}"
    );
    assert!(
        !content.contains("qualification = alternatives"),
        "design-review guidance still demands the old gate formula: {content}"
    );
}

#[then("the output distinguishes authoring policy from runtime enforcement")]
fn output_distinguishes_policy_from_runtime(world: &mut GivnWorld) {
    let content = stdout(world);
    assert!(
        content.contains("authoring guidance, not runtime enforcement")
            || content.contains("not runtime enforcement"),
        "design-review guidance does not distinguish policy from runtime enforcement: {content}"
    );
}
