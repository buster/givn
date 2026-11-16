//! Non-E2E step definitions for the outline report capability.
//!
//! CLI scenarios drive the real `givn` binary; pipeline scenarios call the
//! the built binary and reads what it printed.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[when(regex = r#"^the developer runs "(givn outline report[^"]*)"$"#)]
fn developer_runs(world: &mut GivnWorld, cmd: String) {
    crate::steps::user_runs_quoted(world, cmd);
}

#[then("the command exits with code 0")]
fn command_exits_with_code_0(world: &mut GivnWorld) {
    let status = crate::steps::output(world).status.code();
    assert_eq!(status, Some(0), "stderr: {}", crate::steps::stderr(world));
}

#[given("a Givn project with the outline experiment enabled")]
fn outline_enabled_project(world: &mut GivnWorld) {
    world.env = Some(outline_project());
}

#[given("a production source unit that is over the readable size")]
fn over_readable_size(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_god_type(env, "GodType");
}

#[then("the report classifies the unit as a god type")]
fn report_classifies_god_type(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("god-type"),
        "expected a god-type classification, got: {stdout}"
    );
}

fn report_text(world: &GivnWorld) -> String {
    if let Some(text) = &world.additional_output {
        return text.clone();
    }
    crate::steps::stdout(world)
}

#[then("the report records no recommendation")]
fn report_records_no_recommendation(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        !report.to_lowercase().contains("recommendation"),
        "the report must not recommend an action: {report}"
    );
}

fn outline_project() -> common::TestEnv {
    let env = common::make_bare_root();
    std::fs::create_dir_all(env.path("givn")).expect("create givn dir");
    std::fs::write(env.path("givn/config.yaml"), "addons:\n  outline: true\n")
        .expect("write config");
    env
}

#[then("the report labels the finding as advisory rather than a review or archive verdict")]
fn report_labels_finding_advisory(world: &mut GivnWorld) {
    let report = report_text(world).to_lowercase();
    assert!(
        report.contains("advisory"),
        "the report does not label the finding advisory: {report}"
    );
    assert!(
        !report.contains("review: pass")
            && !report.contains("review: fail")
            && !report.contains("archive: pass")
            && !report.contains("archive: fail"),
        "the report presents a review or archive verdict: {report}"
    );
}
