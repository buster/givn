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

#[given("a production source unit with a trait over the readable size")]
fn large_trait(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_large_trait(env, "BigTrait", 520);
}

#[given("a production source unit that is over the readable size")]
fn over_readable_size(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_god_type(env, "GodType");
}

#[then("the JSON report classifies the unit")]
fn json_report_classifies(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let units = value["units"].as_array().expect("units array");
    assert!(
        units.iter().any(|unit| unit["classification"].is_string()),
        "no classified unit in {stdout}"
    );
}

#[given("a production source unit with exactly 500 code lines and cohesive methods")]
fn boundary_500(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_boundary_type(&env, 500);
    world.env = Some(env);
}

#[when("the readability report assesses the unit")]
fn assess_unit_in_process(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.additional_output = Some(crate::steps::readability_report_support::assess_report(env));
}

#[then("the report classifies the unit as within-limits")]
fn report_classifies_within_limits(world: &mut GivnWorld) {
    let block = unit_block(world, "Boundary");
    assert!(
        block.contains("within-limits"),
        "expected within-limits, got: {block}"
    );
}

#[then("the report recommends no action")]
fn report_recommends_no_action(world: &mut GivnWorld) {
    let block = unit_block(world, "Boundary");
    assert!(
        block.contains("NONE") || block.contains("recommendation: none"),
        "expected no action, got: {block}"
    );
}

fn unit_block(world: &GivnWorld, name: &str) -> String {
    let report = world
        .additional_output
        .as_deref()
        .expect("an in-process report was produced");
    report
        .split("\n\n")
        .find(|block| block.contains(name))
        .unwrap_or(report)
        .to_string()
}

#[given("a production source unit with 501 code lines and cohesive methods")]
fn boundary_501(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_boundary_type(&env, 501);
    world.env = Some(env);
}

#[then("the report classifies the unit as a cohesive large type")]
fn report_classifies_cohesive_large(world: &mut GivnWorld) {
    let block = unit_block(world, "Boundary");
    assert!(
        block.contains("cohesive-large-type"),
        "expected cohesive-large-type, got: {block}"
    );
}

#[then("the report records 501 code lines")]
fn report_records_501(world: &mut GivnWorld) {
    let block = unit_block(world, "Boundary");
    assert!(
        block.contains("501"),
        "expected 501 code lines, got: {block}"
    );
}

fn report_text(world: &GivnWorld) -> String {
    if let Some(text) = &world.additional_output {
        return text.clone();
    }
    crate::steps::stdout(world)
}

#[given("a production source unit with a cohesive type over the readable size")]
fn cohesive_type(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_cohesive_type(env, "Cohesive", 260);
}

#[then("the report classifies the unit as a cohesive-large-type")]
fn report_classifies_cohesive(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("cohesive-large-type"),
        "expected a cohesive-large-type classification, got: {stdout}"
    );
}

#[then("the report classifies the unit as an interface")]
fn report_classifies_interface(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("interface"),
        "expected an interface classification, got: {stdout}"
    );
}

#[then("the report recommends a trait-composition plan")]
fn report_recommends_trait_composition(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("trait-composition plan"),
        "expected a trait-composition plan, got: {stdout}"
    );
}

#[then("the report classifies the unit as data-only")]
fn report_classifies_data_only(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("data-only"),
        "expected a data-only classification, got: {stdout}"
    );
}

#[then("the report recommends an advisory action")]
fn report_recommends_advisory(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("ADVISORY"),
        "expected an advisory action, got: {stdout}"
    );
}

#[then("the report recommends no plan for the unit")]
fn report_recommends_no_plan(world: &mut GivnWorld) {
    let stdout = report_text(world);
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
