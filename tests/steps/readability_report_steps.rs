//! Non-E2E step definitions for the readability report capability.
//!
//! CLI scenarios drive the real `givn` binary; pipeline scenarios call the
//! public `givn::readability` API in process.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a Givn project with the readability experiment disabled")]
fn readability_disabled_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[when(regex = r#"^the developer runs "([^"]+)"$"#)]
fn developer_runs(world: &mut GivnWorld, cmd: String) {
    crate::steps::user_runs_quoted(world, cmd);
}

#[then("the readability addon is listed as disabled")]
fn readability_addon_is_listed_as_disabled(world: &mut GivnWorld) {
    readability_addon_is_disabled(world);
}

#[given("a Givn project with the readability experiment enabled")]
fn readability_enabled_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::readability_report_support::enable_readability(&env);
    world.env = Some(env);
}

#[given("a production source unit with a data-only enum over the readable size")]
fn data_only_enum(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_data_only_enum(env, "BigEnum", 520);
}

#[then("the report classifies the unit as data-only")]
fn report_classifies_data_only(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("data-only"),
        "expected a data-only classification, got: {stdout}"
    );
}

#[then("the report recommends an advisory action")]
fn report_recommends_advisory(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("ADVISORY"),
        "expected an advisory action, got: {stdout}"
    );
}

#[then("the report recommends no plan for the unit")]
fn report_recommends_no_plan(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    let block = stdout
        .split("\n\n")
        .find(|block| block.contains("BigEnum"))
        .expect("the data-only unit block");
    assert!(
        !block.to_lowercase().contains("plan"),
        "the data-only unit must not recommend a plan: {block}"
    );
}

#[then("the readability addon is disabled")]
fn readability_addon_is_disabled(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["addons", "list"]);
    assert!(
        out.status.success(),
        "givn addons list failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[disabled]  readability"),
        "readability should be disabled, got: {stdout}"
    );
}
