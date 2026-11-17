//! E2E step definitions for the ADR qualification guidance capability.
//!
//! E2E steps invoke the compiled Givn CLI as a real subprocess.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn project(world: &GivnWorld) -> &crate::harness::TestEnv {
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
    let env = crate::harness::make_init_root();
    let output = crate::harness::run_givn(&env, &["new", "adr-qualification-gate"]);
    assert!(
        output.status.success(),
        "create ADR qualification change failed"
    );
    world.env = Some(env);
}

#[when("the author requests the Arc42 ADR guidance through the Givn CLI")]
fn author_requests_arc42_adr_guidance(world: &mut GivnWorld) {
    let output = crate::harness::run_givn(
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
