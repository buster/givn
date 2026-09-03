use crate::steps::GivnWorld;
use cucumber::{given, then};

use crate::common;

#[given("a fresh givn project")]
fn fresh_project(world: &mut GivnWorld) {
    world.env = Some(crate::common::make_bare_root());
}

#[given("a givn project with script-based coverage commands")]
fn script_based_project(world: &mut GivnWorld) {
    world.env = Some(crate::common::make_init_root());
    let env = world.env.as_ref().expect("environment is set");
    write_measure_script(env);
    let merge = r##"#!/bin/sh
set -eu
mkdir -p "$(dirname "$GIVN_COVERAGE_OUTPUT_PATH")"
printf '%s\n' '<?xml version="1.0"?><coverage line-rate="0.95" branch-rate="0.95" lines-covered="95" lines-valid="100" branches-covered="19" branches-valid="20"><packages /></coverage>' > "$GIVN_COVERAGE_OUTPUT_PATH"
"##;
    std::fs::write(env.path("merge-coverages.sh"), merge).expect("write merge script");
    make_executable(&env.path("merge-coverages.sh"));
    let yaml = "verify:\n  command: \"true\"\n  e2e_command: \"\"\ncoverage:\n  measure_command: \"./measure-coverage.sh\"\n  merge_command: \"./merge-coverages.sh\"\n  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"\n  e2e_path: \"coverage/e2e-cobertura.xml\"\n  output_path: \"coverage/cobertura-coverage.xml\"\naddons:\n  coverage: true\n";
    common::write_commands_yaml(&env.path("givn/commands.yaml"), yaml);
}

#[then("givn/commands.yaml contains `measure_command: \"./measure-coverage.sh\"`")]
fn commands_name_measure_script(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let commands =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert!(commands.contains("measure_command: \"./measure-coverage.sh\""));
}

#[then("givn/commands.yaml contains `merge_command: \"./merge-coverages.sh\"`")]
fn commands_name_merge_script(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let commands =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert!(commands.contains("merge_command: \"./merge-coverages.sh\""));
}

#[then(
    "the README documents `./run-tests.sh`, `./measure-coverage.sh`, and `./merge-coverages.sh`"
)]
fn readme_documents_scripts(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    for script in [
        "./run-tests.sh",
        "./measure-coverage.sh",
        "./merge-coverages.sh",
    ] {
        assert!(readme.contains(script), "README missing {script}");
    }
}

#[then("the archive succeeds")]
fn archive_succeeds(world: &mut GivnWorld) {
    assert!(
        crate::steps::output(world).status.success(),
        "archive failed: {}",
        crate::steps::stderr(world)
    );
}

#[then("the merged Cobertura report contains line coverage `95/100` and branch coverage `19/20`")]
fn merged_report_has_expected_counters(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let report = std::fs::read_to_string(env.path("coverage/cobertura-coverage.xml"))
        .expect("read merged report");
    assert!(report.contains("lines-covered=\"95\""));
    assert!(report.contains("lines-valid=\"100\""));
    assert!(report.contains("branches-covered=\"19\""));
    assert!(report.contains("branches-valid=\"20\""));
}

#[then("the README places line and branch badges directly below its title")]
fn readme_badges_are_at_top(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    let title_end = readme.find('\n').expect("README has a title");
    let badges = readme
        .find("<!-- givn:begin:coverage-badge -->")
        .expect("README has badge marker");
    assert!(readme[..title_end].starts_with("# "));
    assert!(readme[title_end..badges].trim().is_empty());
    assert!(readme.contains("Line Coverage: 95%"));
    assert!(readme.contains("Branch Coverage: 95%"));
}

#[then(
    "the README coverage summary contains line coverage `95% (95/100)` and branch coverage `95% (19/20)`"
)]
fn readme_summary_has_expected_counters(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(readme.contains("Line coverage: 95% (95/100)"));
    assert!(readme.contains("Branch coverage: 95% (19/20)"));
}

#[then("the README coverage summary links the merged report and names the configured scripts")]
fn readme_summary_links_report_and_scripts(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(readme.contains("[coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)"));
    assert!(readme.contains("./measure-coverage.sh"));
    assert!(readme.contains("./merge-coverages.sh"));
}

fn write_measure_script(env: &crate::common::TestEnv) {
    let script = r##"#!/bin/sh
set -eu
mkdir -p "$(dirname "$GIVN_COVERAGE_NON_E2E_PATH")" "$(dirname "$GIVN_COVERAGE_E2E_PATH")"
printf '%s\n' '<?xml version="1.0"?><coverage line-rate="0.5" branch-rate="0.5" lines-covered="50" lines-valid="100" branches-covered="10" branches-valid="20"><packages /></coverage>' > "$GIVN_COVERAGE_NON_E2E_PATH"
printf '%s\n' '<?xml version="1.0"?><coverage line-rate="0.6" branch-rate="0.6" lines-covered="60" lines-valid="100" branches-covered="12" branches-valid="20"><packages /></coverage>' > "$GIVN_COVERAGE_E2E_PATH"
"##;
    std::fs::write(env.path("measure-coverage.sh"), script).expect("write measure script");
    make_executable(&env.path("measure-coverage.sh"));
}

fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .expect("make script executable");
}
