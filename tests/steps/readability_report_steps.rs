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

#[given("a well-formed production source unit")]
fn well_formed_unit(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_small_unit(env);
}

#[given("a malformed production source file")]
fn malformed_source(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_source(
        env,
        "src/malformed.rs",
        "pub struct Broken {\n",
    );
}

#[then("the report names the malformed file")]
fn report_names_malformed(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("malformed.rs") && stdout.contains("parse-failed"),
        "expected the malformed file and parse failure, got: {stdout}"
    );
}

#[then("the report still classifies the well-formed unit")]
fn report_still_classifies(world: &mut GivnWorld) {
    let block = unit_block(world, "Small");
    assert!(
        block.contains("within-limits"),
        "expected the well-formed unit to be classified, got: {block}"
    );
}

#[given("no production source files")]
fn no_production_sources(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        !env.path("src").exists(),
        "the fixture unexpectedly has a src/ tree"
    );
}

#[then("the report says there are no production sources")]
fn report_says_no_sources(world: &mut GivnWorld) {
    let stdout = report_text(world).to_lowercase();
    assert!(
        stdout.contains("no production sources"),
        "expected the empty-scope message, got: {stdout}"
    );
}

#[then("the command exits with code 0")]
fn command_exits_with_code_0(world: &mut GivnWorld) {
    let status = crate::steps::output(world).status.code();
    assert_eq!(status, Some(0), "stderr: {}", crate::steps::stderr(world));
}

#[given("the readability fixture corpus")]
fn fixture_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::readability_report_support::enable_readability(&env);
    crate::steps::readability_report_support::write_fixture_corpus(&env);
    world.env = Some(env);
}

#[when("the readability report assesses every unit")]
fn assess_every_unit(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let (units, classified, actions) =
        crate::steps::readability_report_support::assessment_counts(env);
    world.additional_output = Some(format!("{units},{classified},{actions}"));
}

#[then("every unit has exactly one classification and one action")]
fn every_unit_has_one(world: &mut GivnWorld) {
    let output = world
        .additional_output
        .as_deref()
        .expect("assessment counts");
    let values: Vec<usize> = output
        .split(',')
        .map(|value| value.parse().expect("count"))
        .collect();
    assert_eq!(values.len(), 3, "expected three counts");
    assert!(values[0] > 0, "no units assessed");
    assert_eq!(values[1], values[0], "some unit has no classification");
    assert_eq!(values[2], values[0], "some unit has no action");
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

#[then("the command exits with code 1")]
fn command_exits_with_code_1(world: &mut GivnWorld) {
    let status = crate::steps::output(world).status.code();
    assert_eq!(status, Some(1), "stderr: {}", crate::steps::stderr(world));
}

#[then(regex = r#"^the report is written to "([^"]+)"$"#)]
fn report_written_to_file(world: &mut GivnWorld, relative: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = std::fs::read_to_string(env.path(&relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"));
    assert!(
        content.contains("readability report"),
        "written report missing header: {content}"
    );
}

#[then("stdout is empty")]
fn stdout_is_empty(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(stdout.is_empty(), "expected empty stdout, got: {stdout}");
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

#[given("a production source unit supported by two analysis engines")]
fn two_engines_fixture(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_small_unit(&env);
    world.env = Some(env);
}

#[when("the readability report is produced twice")]
fn report_produced_twice(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let first = crate::steps::readability_report_support::assess_report(env);
    let second = crate::steps::readability_report_support::assess_report(env);
    world.additional_output = Some(format!("{first}\u{0}{second}"));
}

#[when("the readability report is rendered from each engine")]
fn render_from_each_engine(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let (syn, fake) = crate::steps::readability_report_support::engine_reports(env);
    world.additional_output = Some(format!("{syn}\u{0}{fake}"));
}

#[then("the two reports are identical")]
fn two_reports_identical(world: &mut GivnWorld) {
    let combined = world
        .additional_output
        .as_deref()
        .expect("two reports were produced");
    let (syn, fake) = combined.split_once('\u{0}').expect("two reports");
    assert_eq!(syn, fake, "analysis engines produced different reports");
}

#[given(
    regex = r#"^a workspace member "([^"]+)" with a production source unit that is over the readable size$"#
)]
fn workspace_member(world: &mut GivnWorld, member: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let relative = format!("{member}/src/engine.rs");
    crate::steps::readability_report_support::write_god_type_at(env, "Engine", &relative);
}

#[then("the report classifies the unit as a god type")]
fn report_classifies_god_type(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("god-type"),
        "expected a god-type classification, got: {stdout}"
    );
}

#[then(regex = r#"^the report names the file "([^"]+)"$"#)]
fn report_names_file(world: &mut GivnWorld, relative: String) {
    let stdout = report_text(world);
    assert!(
        stdout.contains(&relative),
        "expected the report to name {relative}, got: {stdout}"
    );
}

#[given("a production source unit and a test source unit")]
fn production_and_test_sources(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_small_unit(env);
    crate::steps::readability_report_support::write_god_type_at(env, "TestGod", "tests/big.rs");
}

#[then("the report does not name the test source")]
fn report_does_not_name_test_source(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        !stdout.contains("tests/big.rs"),
        "the test source must not be scanned: {stdout}"
    );
}

#[given("a production source unit with a long method")]
fn long_method(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_long_method(&env);
    world.env = Some(env);
}

#[then("the report classifies the unit as a method-length finding")]
fn report_classifies_method_length(world: &mut GivnWorld) {
    let block = unit_block(world, "Long");
    assert!(
        block.contains("method-length"),
        "expected method-length, got: {block}"
    );
}

#[then("the report recommends a watch action")]
fn report_recommends_watch(world: &mut GivnWorld) {
    let block = unit_block(world, "Long");
    assert!(block.contains("WATCH"), "expected WATCH, got: {block}");
}

#[given("a production source unit with a module over the plan limit")]
fn module_over_limit(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_large_module(&env);
    world.env = Some(env);
}

#[then("the report classifies the unit as a module")]
fn report_classifies_module(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        report.contains("module") && !report.contains("multi-type-module"),
        "expected the module classification, got: {report}"
    );
}

#[given("a production source unit with a large type in the inspect band")]
fn inspect_band_type(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_inspect_band_type(&env);
    world.env = Some(env);
}

#[then("the report recommends an exception action")]
fn report_recommends_exception(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        report.contains("EXCEPTION"),
        "expected an exception action, got: {report}"
    );
}

#[given("a production source unit with more than 50 public methods")]
fn many_public_methods(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_many_public_methods(&env);
    world.env = Some(env);
}

#[given("a production source unit with platform-conditional methods")]
fn platform_conditional(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::readability_report_support::write_conditional_unit(&env);
    world.env = Some(env);
}

#[then("the report counts both platform variants")]
fn report_counts_both_variants(world: &mut GivnWorld) {
    let block = unit_block(world, "Conditional");
    assert!(
        block.contains("methods 2"),
        "expected both variants to count, got: {block}"
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
    let report = report_text(world);
    report
        .split("\n\n")
        .find(|block| block.contains(name))
        .unwrap_or(&report)
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

#[given("a production source unit within the readable size")]
fn within_readable_size(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_small_unit(env);
}

#[then("the report labels the unit as a finding")]
fn report_labels_finding(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        report.contains("FINDING"),
        "expected a FINDING label: {report}"
    );
}

#[then("the report records no recommendation")]
fn report_records_no_recommendation(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        !report.to_lowercase().contains("recommendation"),
        "the report must not recommend an action: {report}"
    );
}

#[then("the report labels the unit as within limits")]
fn report_labels_within_limits(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        report.contains("WITHIN-LIMITS"),
        "expected a WITHIN-LIMITS label: {report}"
    );
}

#[then("the JSON report has no action or recommendation fields")]
fn json_has_no_action_fields(world: &mut GivnWorld) {
    let value = json_output(world);
    let text = value.to_string();
    assert!(!text.contains("\"action\""), "action field remains: {text}");
    assert!(
        !text.contains("\"recommendation\""),
        "recommendation field remains: {text}"
    );
}

#[then("the JSON report reports 2 findings")]
fn json_reports_two_findings(world: &mut GivnWorld) {
    let value = json_output(world);
    assert_eq!(
        value["summary"]["findings"], 2,
        "unexpected findings count: {value}"
    );
}

#[then("the JSON findings count matches the non-within-limits units")]
fn json_findings_count_matches(world: &mut GivnWorld) {
    let value = json_output(world);
    let units = value["units"].as_array().expect("units");
    let expected = units
        .iter()
        .filter(|unit| unit["classification"] != "within-limits")
        .count();
    assert_eq!(
        value["summary"]["findings"].as_u64(),
        Some(expected as u64),
        "findings count does not match classifications: {value}"
    );
}

#[then("the JSON report classifies the data-only unit")]
fn json_classifies_data_only(world: &mut GivnWorld) {
    let value = json_output(world);
    let units = value["units"].as_array().expect("units");
    assert!(
        units
            .iter()
            .any(|unit| unit["classification"] == "data-only"),
        "no data-only unit: {value}"
    );
}

#[then("every unit has exactly one classification")]
fn every_unit_has_one_classification(world: &mut GivnWorld) {
    let output = world
        .additional_output
        .as_deref()
        .expect("assessment counts");
    let values: Vec<usize> = output
        .split(',')
        .map(|value| value.parse().expect("count"))
        .collect();
    assert!(values[0] > 0, "no units assessed");
    assert_eq!(values[1], values[0], "some unit has no classification");
}

#[then("no unit carries an action or recommendation")]
fn no_unit_carries_action(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let json = crate::steps::readability_report_support::assess_json(env);
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    let text = value["units"].to_string();
    assert!(!text.contains("\"action\""), "action remains: {text}");
    assert!(
        !text.contains("\"recommendation\""),
        "recommendation remains: {text}"
    );
}

#[then("the report lists units by classification, then path, then unit id")]
fn report_sorted_by_classification(world: &mut GivnWorld) {
    let report = report_text(world);
    let mut keys = Vec::new();
    for line in report.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 && (parts[0] == "FINDING" || parts[0] == "WITHIN-LIMITS") {
            keys.push((
                parts[4].to_string(),
                parts[2].to_string(),
                parts[3].to_string(),
            ));
        }
    }
    assert!(
        keys.len() > 1,
        "need several units to check ordering: {report}"
    );
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(
        keys, sorted,
        "units are not sorted by classification/path/id"
    );
}

#[given("a production source unit")]
fn production_source_unit(world: &mut GivnWorld) {
    if world.env.is_none() {
        world.env = Some(common::make_bare_root());
    }
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_small_unit(env);
}

#[then("the JSON report names the supported constructs")]
fn json_supported_constructs(world: &mut GivnWorld) {
    let value = json_output(world);
    let constructs = value["capabilities"]["supported_constructs"]
        .as_array()
        .expect("supported_constructs array");
    assert!(!constructs.is_empty(), "no supported constructs in {value}");
}

#[then("the JSON report names the analyzer limitations")]
fn json_analyzer_limitations(world: &mut GivnWorld) {
    let value = json_output(world);
    let limitations = value["capabilities"]["limitations"]
        .as_array()
        .expect("limitations array");
    assert!(
        !limitations.is_empty(),
        "no analyzer limitations in {value}"
    );
}

#[then("the JSON report has schema version 2")]
fn json_schema_version(world: &mut GivnWorld) {
    let value = json_output(world);
    assert_eq!(value["schema_version"], 2, "unexpected schema version");
}

#[then("the JSON report has rules version 1")]
fn json_rules_version(world: &mut GivnWorld) {
    let value = json_output(world);
    assert_eq!(value["rules_version"], 1, "unexpected rules version");
}

fn json_output(world: &GivnWorld) -> serde_json::Value {
    let stdout = crate::steps::stdout(world);
    serde_json::from_str(&stdout).expect("valid JSON")
}

#[given("a production source file with an item-position macro")]
fn item_position_macro(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::readability_report_support::write_item_macro(env);
}

#[then("the report names the unsupported construct")]
fn report_names_unsupported(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        stdout.contains("item-macros") && stdout.contains("unsupported"),
        "expected an unsupported item-macros section, got: {stdout}"
    );
}

#[then("the report does not classify the unsupported unit")]
fn report_does_not_classify_unsupported(world: &mut GivnWorld) {
    let stdout = report_text(world);
    assert!(
        !stdout
            .lines()
            .any(|line| line.contains("type") && line.contains("make_items")),
        "the macro must not become a classified unit: {stdout}"
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

#[given("the readability addon is disabled")]
fn readability_addon_is_disabled_given(world: &mut GivnWorld) {
    readability_addon_is_disabled(world);
}

#[then("the readability addon is disabled")]
fn readability_addon_is_disabled_then(world: &mut GivnWorld) {
    readability_addon_is_disabled(world);
}

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
