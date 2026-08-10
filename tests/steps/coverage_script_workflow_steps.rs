use crate::steps::{GivnWorld, stderr, stdout};
use cucumber::{given, then, when};

#[given("a givn project with a failing coverage merge command")]
fn project_with_failing_merge(world: &mut GivnWorld) {
    world.env = Some(crate::common::make_init_root());
    let env = world.env.as_ref().expect("environment is set");
    write_measure_script(env);
    std::fs::write(
        env.path("merge-coverages.sh"),
        "#!/bin/sh\nset -eu\nexit 7\n",
    )
    .expect("write failing merge script");
    make_executable(&env.path("merge-coverages.sh"));
    write_commands(env, "./measure-coverage.sh", "./merge-coverages.sh");
}

#[when("the user archives the change")]
fn archive_change(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .as_deref()
        .expect("a completed change is required");
    let env = world.env.as_ref().expect("environment is set");
    world.last_output = Some(crate::common::run_givn(env, &["archive", "--change", id]));
}

#[then("the archive fails because coverage merging failed")]
fn archive_fails_at_merge(world: &mut GivnWorld) {
    assert!(!crate::steps::output(world).status.success());
    let combined = format!("{}{}", stdout(world), stderr(world));
    assert!(
        combined.contains("coverage merge command failed"),
        "archive output did not identify merge failure: {combined}"
    );
}

#[then("the change remains under givn/changes/")]
fn change_remains_under_changes(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("environment is set");
    let id = world.last_change_id.as_deref().expect("change id is set");
    assert!(env.path(&format!("givn/changes/{id}")).is_dir());
}

fn write_commands(env: &crate::common::TestEnv, measure: &str, merge: &str) {
    let yaml = format!(
        "verify:\n  command: \"true\"\n  e2e_command: \"\"\ncoverage:\n  measure_command: \"{measure}\"\n  merge_command: \"{merge}\"\n  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"\n  e2e_path: \"coverage/e2e-cobertura.xml\"\n  output_path: \"coverage/cobertura-coverage.xml\"\naddons:\n  coverage: true\n"
    );
    std::fs::write(env.path("givn/commands.yaml"), yaml).expect("write commands.yaml");
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
