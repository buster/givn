//! Non-E2E step definitions for the outline report capability.
//!
//! CLI scenarios drive the real `givn` binary; pipeline scenarios call the
//! public `givn::outline` API in process.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a Givn project with the outline experiment disabled")]
fn outline_disabled_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[when(regex = r#"^the developer runs "(givn outline report[^"]*)"$"#)]
fn developer_runs(world: &mut GivnWorld, cmd: String) {
    crate::steps::user_runs_quoted(world, cmd);
}

#[when(regex = r#"^the developer runs "(givn addons list)"$"#)]
fn developer_runs_addons_list(world: &mut GivnWorld, cmd: String) {
    crate::steps::user_runs_quoted(world, cmd);
}

#[then("the outline addon is listed as disabled")]
fn outline_addon_is_listed_as_disabled(world: &mut GivnWorld) {
    outline_addon_is_disabled(world);
}

#[given("a well-formed production source unit")]
fn well_formed_unit(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_small_unit(env);
}

#[given("a malformed production source file")]
fn malformed_source(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_source(
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

#[given("the outline fixture corpus")]
fn fixture_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_fixture_corpus(&env);
    world.env = Some(env);
}

#[when("the outline report assesses every unit")]
fn assess_every_unit(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let (units, classified, actions) = crate::steps::outline_report_support::assessment_counts(env);
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

#[given("a Givn project with the outline experiment enabled")]
fn outline_enabled_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    world.env = Some(env);
}

#[given("a production source unit with a data-only enum over the readable size")]
fn data_only_enum(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_data_only_enum(env, "BigEnum", 520);
}

#[given("a production source unit with a trait over the readable size")]
fn large_trait(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_large_trait(env, "BigTrait", 520);
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
        content.contains("outline report"),
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
    crate::steps::outline_report_support::write_god_type(env, "GodType");
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
    crate::steps::outline_report_support::write_small_unit(&env);
    world.env = Some(env);
}

#[when("the outline report is produced twice")]
fn report_produced_twice(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let first = crate::steps::outline_report_support::assess_report(env);
    let second = crate::steps::outline_report_support::assess_report(env);
    world.additional_output = Some(format!("{first}\u{0}{second}"));
}

#[when("the outline report is rendered from each engine")]
fn render_from_each_engine(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let (syn, fake) = crate::steps::outline_report_support::engine_reports(env);
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
    crate::steps::outline_report_support::write_god_type_at(env, "Engine", &relative);
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
    crate::steps::outline_report_support::write_small_unit(env);
    crate::steps::outline_report_support::write_god_type_at(env, "TestGod", "tests/big.rs");
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
    crate::steps::outline_report_support::write_long_method(&env);
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
    crate::steps::outline_report_support::write_large_module(&env);
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
    crate::steps::outline_report_support::write_inspect_band_type(&env);
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
    crate::steps::outline_report_support::write_many_public_methods(&env);
    world.env = Some(env);
}

#[given("a production source unit with platform-conditional methods")]
fn platform_conditional(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::outline_report_support::write_conditional_unit(&env);
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
    crate::steps::outline_report_support::write_boundary_type(&env, 500);
    world.env = Some(env);
}

#[when("the outline report assesses the unit")]
fn assess_unit_in_process(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.additional_output = Some(crate::steps::outline_report_support::assess_report(env));
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
    let marker = format!("::{name}");
    report
        .split("\n\n")
        .find(|block| block.contains(&marker))
        .unwrap_or(&report)
        .to_string()
}

#[given("a production source unit with 501 code lines and cohesive methods")]
fn boundary_501(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::outline_report_support::write_boundary_type(&env, 501);
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
    crate::steps::outline_report_support::write_cohesive_type(env, "Cohesive", 260);
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
    crate::steps::outline_report_support::write_small_unit(env);
}

#[then("the report labels the unit as a hint")]
fn report_labels_hint(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(report.contains("HINT"), "expected a HINT label: {report}");
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

#[then("the JSON report reports 2 hints")]
fn json_reports_two_hints(world: &mut GivnWorld) {
    let value = json_output(world);
    assert_eq!(
        value["summary"]["hints"], 2,
        "unexpected hints count: {value}"
    );
}

#[then("the JSON hints count matches the non-within-limits units")]
fn json_hints_count_matches(world: &mut GivnWorld) {
    let value = json_output(world);
    let units = value["units"].as_array().expect("units");
    let expected = units
        .iter()
        .filter(|unit| unit["classification"] != "within-limits")
        .count();
    assert_eq!(
        value["summary"]["hints"].as_u64(),
        Some(expected as u64),
        "hints count does not match classifications: {value}"
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
    let json = crate::steps::outline_report_support::assess_json(env);
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
        if parts.len() >= 5 && (parts[0] == "HINT" || parts[0] == "WITHIN-LIMITS") {
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
    crate::steps::outline_report_support::write_small_unit(env);
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

#[then("the JSON report has schema version 3")]
fn json_schema_version(world: &mut GivnWorld) {
    let value = json_output(world);
    assert_eq!(value["schema_version"], 3, "unexpected schema version");
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
    crate::steps::outline_report_support::write_item_macro(env);
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

#[given("the outline addon is disabled")]
fn outline_addon_is_disabled_given(world: &mut GivnWorld) {
    outline_addon_is_disabled(world);
}

#[then("the outline addon is disabled")]
fn outline_addon_is_disabled_then(world: &mut GivnWorld) {
    outline_addon_is_disabled(world);
}

fn outline_addon_is_disabled(world: &mut GivnWorld) {
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
        stdout.contains("[disabled]  outline"),
        "outline should be disabled, got: {stdout}"
    );
}

#[given("a Python module with a class and methods")]
fn python_class_module(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_python_class(env);
}

#[given("a Python module with top-level functions")]
fn python_function_module(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_python_module(env);
}

#[given("python3 is not on the PATH")]
fn python3_absent(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    std::fs::create_dir_all(env.path(".empty-path")).expect("create empty PATH dir");
}

#[given("a python3 recorder that records every invocation")]
fn python3_recorder(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::outline_report_support::write_python3_stub(env);
}

#[when("the developer runs the outline report")]
fn developer_runs_report_without_python3(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let empty = env.path(".empty-path");
    let empty_path = empty.to_string_lossy().into_owned();
    let out = common::run_givn_with_envs(
        env,
        &["outline", "report"],
        &[("PATH", empty_path.as_str())],
    );
    world.last_output = Some(out);
}

#[when("the developer runs the outline report with the recorder on the PATH")]
fn developer_runs_report_with_recorder(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let recorder = env.path(".python3-stub");
    let existing = std::env::var("PATH").unwrap_or_default();
    let value = format!("{}:{existing}", recorder.display());
    let out = common::run_givn_with_envs(env, &["outline", "report"], &[("PATH", value.as_str())]);
    world.last_output = Some(out);
}

#[then(regex = r#"^the JSON report names the Python class "([^"]+)"$"#)]
fn json_names_python_class(world: &mut GivnWorld, name: String) {
    let value = json_output(world);
    let units = value["units"].as_array().expect("units");
    assert!(
        units.iter().any(|unit| {
            unit["name"].as_str() == Some(name.as_str())
                && unit["language"].as_str() == Some("python")
                && unit["kind"].as_str() == Some("type")
                && unit["type_kind"].as_str() == Some("class")
        }),
        "no Python class {name}: {value}"
    );
}

#[then(regex = r#"^the JSON report lists the class methods "([^"]+)" and "([^"]+)"$"#)]
fn json_lists_class_methods(world: &mut GivnWorld, first: String, second: String) {
    let value = json_output(world);
    let unit = value["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|unit| {
            unit["language"].as_str() == Some("python") && unit["kind"].as_str() == Some("type")
        })
        .unwrap_or_else(|| panic!("no Python class unit: {value}"));
    let names: Vec<&str> = unit["methods"]
        .as_array()
        .expect("methods")
        .iter()
        .filter_map(|method| method["name"].as_str())
        .collect();
    assert!(
        names.contains(&first.as_str()) && names.contains(&second.as_str()),
        "expected methods {first} and {second}, got: {names:?}"
    );
}

#[then(regex = r#"^the JSON outline records the call from "([^"]+)" to "([^"]+)"$"#)]
fn json_outline_records_call(world: &mut GivnWorld, caller: String, callee: String) {
    let value = json_output(world);
    let items = value["units"]
        .as_array()
        .expect("units")
        .iter()
        .flat_map(|unit| unit["outline"].as_array().into_iter().flatten())
        .collect::<Vec<_>>();
    let item = items
        .iter()
        .find(|item| item["name"].as_str() == Some(caller.as_str()))
        .unwrap_or_else(|| panic!("no outline item {caller}: {value}"));
    let calls: Vec<&str> = item["calls"]
        .as_array()
        .expect("calls")
        .iter()
        .filter_map(|call| call["callee"].as_str())
        .collect();
    assert!(
        calls.contains(&callee.as_str()),
        "expected {caller} to call {callee}, got: {calls:?}"
    );
}

#[then(regex = r#"^the JSON report records the file language "([^"]+)"$"#)]
fn json_records_file_language(world: &mut GivnWorld, language: String) {
    let value = json_output(world);
    let files = value["files"].as_array().expect("files");
    assert!(
        files.iter().any(|file| {
            file["path"].as_str() == Some("src/tracker.py")
                && file["language"].as_str() == Some(language.as_str())
        }),
        "expected src/tracker.py to be {language}: {value}"
    );
}

#[then("the JSON report names one Python module unit")]
fn json_names_python_module(world: &mut GivnWorld) {
    let value = json_output(world);
    let modules: Vec<_> = value["units"]
        .as_array()
        .expect("units")
        .iter()
        .filter(|unit| {
            unit["language"].as_str() == Some("python") && unit["kind"].as_str() == Some("module")
        })
        .collect();
    assert_eq!(modules.len(), 1, "expected one Python module unit: {value}");
    assert_eq!(
        modules[0]["name"].as_str(),
        Some("src/store.py"),
        "unexpected module name: {value}"
    );
}

#[then(regex = r#"^the JSON report lists the module functions "([^"]+)" and "([^"]+)"$"#)]
fn json_lists_module_functions(world: &mut GivnWorld, first: String, second: String) {
    let value = json_output(world);
    let unit = value["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|unit| {
            unit["language"].as_str() == Some("python") && unit["kind"].as_str() == Some("module")
        })
        .unwrap_or_else(|| panic!("no Python module unit: {value}"));
    let names: Vec<&str> = unit["methods"]
        .as_array()
        .expect("methods")
        .iter()
        .filter_map(|method| method["name"].as_str())
        .collect();
    assert!(
        names.contains(&first.as_str()) && names.contains(&second.as_str()),
        "expected functions {first} and {second}, got: {names:?}"
    );
}

#[then("the report names the analyzer as unavailable")]
fn report_names_analyzer_unavailable(world: &mut GivnWorld) {
    let report = report_text(world);
    assert!(
        report.contains("analyzer-unavailable"),
        "expected an unavailable analyzer diagnostic, got: {report}"
    );
}

#[then("the python3 recorder records no invocation")]
fn recorder_records_no_invocation(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        !crate::steps::outline_report_support::python3_stub_invoked(env),
        "a Rust-only project must not invoke python3"
    );
}

#[then("the report names only the rust language")]
fn report_names_only_rust(world: &mut GivnWorld) {
    let report = report_text(world);
    let header = report.lines().next().unwrap_or_default();
    assert!(
        header.contains("outline report — rust,"),
        "expected a rust-only header, got: {header}"
    );
    assert!(
        !header.contains("python"),
        "the header must not name python: {header}"
    );
}

#[then(regex = r#"^the outline of the class "([^"]+)" shows "([^"]+)" calling "([^"]+)"$"#)]
fn outline_class_shows_call(world: &mut GivnWorld, class: String, caller: String, callee: String) {
    let out = world.last_output.as_ref().expect("report output");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let marker = format!("::{class}");
    let section = stdout.split(&marker).nth(1).expect("class component");
    let caller_marker = format!("def {caller}(");
    let caller_offset = section.find(&caller_marker).expect("caller method");
    let call_marker = format!("-> [public, {class}] {callee}");
    assert!(
        section[caller_offset..].contains(&call_marker),
        "expected {caller} to call {callee} in {class}: {section}"
    );
}

#[given("a production source unit whose method calls a module function")]
fn cross_component_source(world: &mut GivnWorld) {
    let env = match world.env.take() {
        Some(env) => env,
        None => {
            let env = common::make_init_root();
            crate::steps::outline_report_support::enable_outline(&env);
            env
        }
    };
    crate::steps::outline_report_support::write_cross_component_calls(&env);
    world.env = Some(env);
}

#[then("the type outline calls the module function without expanding it")]
fn type_outline_keeps_cross_component_calls_leaf(world: &mut GivnWorld) {
    let out = world.last_output.as_ref().expect("report output");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let engine = stdout
        .split("type:src/component.rs::Engine")
        .nth(1)
        .expect("Engine component");
    assert!(
        engine.contains("-> [public, module] normalize"),
        "expected the Engine outline to call normalize: {engine}"
    );
    assert!(
        !engine.contains("helper"),
        "a cross-component call must not expand the target's calls: {engine}"
    );
    let module = stdout
        .split("module:src/component.rs")
        .nth(1)
        .expect("module component");
    assert!(
        module.contains("-> [private, module] helper"),
        "expected normalize to call helper in the module component: {module}"
    );
}
#[given("a production source unit whose method calls a receiver of unknown type")]
fn unknown_receiver_call(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_unknown_receiver_call(&env);
    world.env = Some(env);
}

fn all_calls(value: &serde_json::Value) -> Vec<&serde_json::Value> {
    value["units"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|unit| unit["outline"].as_array().into_iter().flatten())
        .flat_map(|item| item["calls"].as_array().into_iter().flatten())
        .collect()
}

fn call_named<'a>(value: &'a serde_json::Value, callee: &str) -> &'a serde_json::Value {
    all_calls(value)
        .into_iter()
        .find(|call| call["callee"].as_str() == Some(callee))
        .unwrap_or_else(|| panic!("no call {callee} in {value}"))
}

#[then(regex = r#"^the JSON report records the call "([^"]+)" as "([^"]+)"$"#)]
fn json_call_resolution(world: &mut GivnWorld, callee: String, resolution: String) {
    let value = json_output(world);
    let call = call_named(&value, &callee);
    assert_eq!(
        call["resolution"].as_str(),
        Some(resolution.as_str()),
        "unexpected resolution for {callee}: {call}"
    );
}

#[then(regex = r#"^the JSON call names "([^"]+)" as its target$"#)]
fn json_call_names_target(world: &mut GivnWorld, target: String) {
    let value = json_output(world);
    assert!(
        all_calls(&value)
            .iter()
            .any(|call| call["target"].as_str() == Some(target.as_str())),
        "no call targets {target}: {value}"
    );
}

#[then(regex = r#"^the JSON call names "([^"]+)" as its candidate target$"#)]
fn json_call_names_candidate(world: &mut GivnWorld, target: String) {
    json_call_names_target(world, target);
}

#[then(regex = r#"^the JSON call "([^"]+)" names no target$"#)]
fn json_call_has_no_target(world: &mut GivnWorld, callee: String) {
    let value = json_output(world);
    let call = call_named(&value, &callee);
    assert!(call["target"].is_null(), "{callee} has a target: {call}");
}

#[then(regex = r#"^the report prints the call "([^"]+)" as "([^"]+)"$"#)]
fn human_call_label(world: &mut GivnWorld, callee: String, tag: String) {
    let report = report_text(world);
    assert!(
        report.contains(&tag),
        "missing {tag} for {callee}: {report}"
    );
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

#[then(regex = r#"^the name-only call "([^"]+)" is not expanded into a deeper-indented call line$"#)]
fn name_only_not_expanded(world: &mut GivnWorld, callee: String) {
    let report = report_text(world);
    let lines: Vec<&str> = report.lines().collect();
    let index = lines
        .iter()
        .position(|line| line.contains("-> [name_only") && line.trim_end().ends_with(&callee))
        .unwrap_or_else(|| panic!("no name-only call {callee}: {report}"));
    let next = lines[index + 1..]
        .iter()
        .find(|line| !line.trim().is_empty())
        .expect("a line after the name-only call");
    assert!(
        indent(next) <= indent(lines[index]),
        "{callee} was expanded: {report}"
    );
}

fn report_block(world: &GivnWorld, id: &str) -> String {
    let report = report_text(world);
    report
        .split("\n\n")
        .find(|block| block.contains(id))
        .unwrap_or_else(|| panic!("no report block for {id}: {report}"))
        .to_string()
}

#[then(regex = r#"^the report records "(LCOM4 \d+)" for the unit "([^"]+)"$"#)]
fn report_records_lcom4(world: &mut GivnWorld, expected: String, unit: String) {
    let block = report_block(world, &unit);
    assert!(block.contains(&expected), "missing {expected}: {block}");
}

#[given("a production source unit whose method calls another method on itself")]
fn self_receiver_call(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_self_receiver_call(&env);
    world.env = Some(env);
}

#[given("a production source unit whose method calls an associated function on a project type")]
fn named_receiver_call(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_named_receiver_call(&env);
    world.env = Some(env);
}

#[given("a production source unit whose method calls an unknown receiver whose name two project types declare")]
fn ambiguous_call(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_ambiguous_call(&env);
    world.env = Some(env);
}

#[given("a production source unit that calls \"Line::from\" while the project declares \"Error::from\"")]
fn contradictory_qualifier(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_contradictory_qualifier(&env);
    world.env = Some(env);
}

#[given("a production source unit over the readable size whose methods are connected only by calls")]
fn tcc_god_type(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::outline_report_support::write_tcc_god_type(&env);
    world.env = Some(env);
}

#[then(regex = r#"^the report records the evidence "([^"]+)"$"#)]
fn report_records_evidence(world: &mut GivnWorld, expected: String) {
    let block = report_block(world, &latest_unit_id(world, &expected));
    let (rule, rest) = expected
        .split_once(' ')
        .unwrap_or((expected.as_str(), ""));
    assert!(block.contains(rule), "missing evidence {rule}: {block}");
    if !rest.is_empty() {
        assert!(block.contains(rest), "missing evidence {rest}: {block}");
    }
}

#[then(regex = r#"^the report records no "([^"]+)" evidence$"#)]
fn report_records_no_evidence(world: &mut GivnWorld, rule: String) {
    let block = report_block(world, &latest_unit_id(world, &rule));
    assert!(!block.contains(&rule), "unexpected evidence {rule}: {block}");
}

fn latest_unit_id(world: &GivnWorld, _hint: &str) -> String {
    // Evidence scenarios write exactly one unit under test; use the report's
    // first HINT/WITHIN-LIMITS header so the assertion stays fixture-driven.
    let report = report_text(world);
    report
        .lines()
        .find(|line| line.starts_with("HINT") || line.starts_with("WITHIN-LIMITS"))
        .and_then(|line| line.split_whitespace().nth(3))
        .unwrap_or_else(|| panic!("no unit header: {report}"))
        .to_string()
}

#[given("a production source unit with a method over the complexity limit but within the length limit")]
fn complexity_method(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    crate::steps::outline_report_support::write_complexity_method(&env);
    world.env = Some(env);
}

#[given("a production Rust module with three declared types and one free function")]
fn module_with_types(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_module_with_types(&env);
    world.env = Some(env);
}

#[then(regex = r#"^the JSON module outline lists the types "([^"]+)", "([^"]+)", and "([^"]+)"$"#)]
fn json_module_lists_types(world: &mut GivnWorld, first: String, second: String, third: String) {
    let value = json_output(world);
    let names = module_outline_names(&value);
    for name in [first, second, third] {
        assert!(names.contains(&name), "module outline lacks {name}: {names:?}");
    }
}

fn module_outline_names(value: &serde_json::Value) -> Vec<String> {
    value["units"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|unit| unit["kind"].as_str() == Some("module"))
        .and_then(|unit| unit["outline"].as_array())
        .into_iter()
        .flatten()
        .filter(|item| item["kind"].as_str() == Some("type"))
        .filter_map(|item| item["name"].as_str().map(ToString::to_string))
        .collect()
}

#[given("a production Rust module with a free function, a type, a free function, and a type in that order")]
fn interleaved_module(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_interleaved_module(&env);
    world.env = Some(env);
}

#[then(regex = r#"^the JSON module outline lists "([^"]+)", "([^"]+)", "([^"]+)", and "([^"]+)" in source order$"#)]
fn json_module_lists_in_order(
    world: &mut GivnWorld,
    first: String,
    second: String,
    third: String,
    fourth: String,
) {
    let value = json_output(world);
    let names = module_outline_all_names(&value);
    assert_eq!(names, vec![first, second, third, fourth], "{value}");
}

fn module_outline_all_names(value: &serde_json::Value) -> Vec<String> {
    value["units"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|unit| unit["kind"].as_str() == Some("module"))
        .and_then(|unit| unit["outline"].as_array())
        .into_iter()
        .flatten()
        .filter_map(|item| item["name"].as_str().map(ToString::to_string))
        .collect()
}

#[given("a Python module with two classes and one function")]
fn python_classes_module(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_python_classes_module(&env);
    world.env = Some(env);
}

#[then(regex = r#"^the JSON module outline lists the classes "([^"]+)" and "([^"]+)"$"#)]
fn json_module_lists_classes(world: &mut GivnWorld, first: String, second: String) {
    let value = json_output(world);
    let names = module_outline_names(&value);
    for name in [first, second] {
        assert!(names.contains(&name), "module outline lacks {name}: {names:?}");
    }
}

#[then(regex = r#"^the JSON call "([^"]+)" names the target "([^"]+)"$"#)]
fn json_call_named_target(world: &mut GivnWorld, callee: String, target: String) {
    let value = json_output(world);
    let call = call_named(&value, &callee);
    assert_eq!(call["target"].as_str(), Some(target.as_str()), "{call}");
}

#[given("a production source unit declared in one file and implemented in another")]
fn cross_file_impl(world: &mut GivnWorld) {
    let env = common::make_init_root();
    crate::steps::outline_report_support::enable_outline(&env);
    crate::steps::outline_report_support::write_cross_file_impl(&env);
    world.env = Some(env);
}

#[then(regex = r#"^the call "([^"]+)" is expanded into a deeper-indented call line$"#)]
fn call_is_expanded(world: &mut GivnWorld, callee: String) {
    let report = report_text(world);
    let lines: Vec<&str> = report.lines().collect();
    let index = lines
        .iter()
        .position(|line| line.trim_end().ends_with(&callee) && line.contains("-> ["))
        .unwrap_or_else(|| panic!("no call {callee}: {report}"));
    let next = lines[index + 1..]
        .iter()
        .find(|line| !line.trim().is_empty())
        .expect("a line after the call");
    assert!(
        indent(next) > indent(lines[index]),
        "{callee} was not expanded: {report}"
    );
}

