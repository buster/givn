//! Fixture builders for the declared `e2e` boundary scenarios.
//!
//! Every fixture is a real initialized Givn project with one committed change
//! whose `config-lock.yaml` records `ise: true`. The scenario drives the built
//! binary with `givn check tasks`; no step imports a Givn module.

use crate::common;
use crate::steps::GivnWorld;

pub(crate) const CHANGE_ID: &str = "ise-boundary";

/// What the scenario's Given wrote, for the assertion steps to name.
#[derive(Debug, Default)]
pub(crate) struct FixtureState {
    pub(crate) declared_entry: Option<String>,
    pub(crate) source_roots: Vec<String>,
    pub(crate) import_target: Option<String>,
    pub(crate) runner_command: Option<String>,
    /// Resolved `e2e` package directories the receipt must bind.
    pub(crate) resolved_packages: Vec<String>,
}

pub(crate) fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a fixture project is required first")
}

/// The fixture's recorded state; every Given installs one.
pub(crate) fn state(world: &GivnWorld) -> &FixtureState {
    world
        .e2e_boundary
        .as_ref()
        .expect("an ise boundary fixture is required first")
}

/// An initialized Git project with one change whose lock has `ise` effective,
/// a proposal, and a delta feature with no `@wip` scenario, so the change
/// check reaches its boundary preflight.
pub(crate) fn ise_change(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let change = env.path(&format!("givn/changes/{CHANGE_ID}"));
    std::fs::create_dir_all(change.join("specs/verify-change")).expect("create the delta specs");
    std::fs::write(
        change.join("config-lock.yaml"),
        "version: 1\naddons:\n  ise: true\noverrides: []\n",
    )
    .expect("write the change lock");
    std::fs::write(
        change.join("proposal.md"),
        "# Proposal: ise boundary fixture\n\n## Behavior change:\n\n- The boundary is checked.\n",
    )
    .expect("write the proposal");
    std::fs::write(
        change.join("specs/verify-change/givn-workflow-enforcement.feature"),
        "@givn.delta @gvn-workflow-enforcement\nFeature: givn Workflow Enforcement\n\n  \
         @givn.added\n  Scenario: A fixture delta scenario passes\n    Given a fixture precondition\n    \
         Then a fixture result\n",
    )
    .expect("write the delta feature");
    world.env = Some(env);
    world.e2e_boundary = Some(FixtureState::default());
}

/// Write the declaration and record the entries the assertions name.
pub(crate) fn declare(world: &mut GivnWorld, e2e: &[&str], source: &[&str]) {
    let list = |values: &[&str]| {
        values
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    std::fs::write(
        env(world).path("givn/specification.yaml"),
        format!(
            "behavior: []\nstructure: []\ncontracts: []\nlimits: []\ne2e: [{}]\nsource: [{}]\n",
            list(e2e),
            list(source),
        ),
    )
    .expect("write the declaration");
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.declared_entry = e2e.first().map(|entry| (*entry).to_string());
    state.source_roots = source.iter().map(|root| (*root).to_string()).collect();
}

pub(crate) fn write_file(world: &GivnWorld, relative: &str, content: &str) {
    let path = env(world).path(relative);
    std::fs::create_dir_all(path.parent().expect("a fixture file has a parent"))
        .expect("create the fixture directory");
    std::fs::write(path, content).expect("write the fixture file");
}

pub(crate) fn write_manifest(world: &GivnWorld, relative: &str, content: &str) {
    write_file(world, relative, content);
}

/// Commit every file the Given wrote, so the fixture is the committed state a
/// developer runs the change check from.
pub(crate) fn commit(world: &GivnWorld) {
    common::commit_fixture(env(world));
}

/// Point `verify.e2e_command` at the scenario's command, mapping `(empty)` to
/// the explicit empty value the declaration carries.
pub(crate) fn set_e2e_command(world: &mut GivnWorld, command: &str) {
    let command = if command == "(empty)" {
        String::new()
    } else {
        command.to_string()
    };
    let yaml =
        format!("verify:\n  command: \"givn missing-testrunner\"\n  e2e_command: \"{command}\"\n");
    common::write_commands_yaml(&env(world).path("givn/commands.yaml"), &yaml);
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.runner_command = Some(command);
}

pub(crate) fn set_import_target(world: &mut GivnWorld, target: &str) {
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.import_target = Some(target.to_string());
}

/// The change directory of the fixture project.
pub(crate) fn change_path(world: &GivnWorld) -> std::path::PathBuf {
    env(world).path(&format!("givn/changes/{CHANGE_ID}"))
}

/// Write an executable fixture script at the project root.
pub(crate) fn write_executable(world: &GivnWorld, relative: &str, content: &str) {
    let path = env(world).path(relative);
    std::fs::write(&path, content).expect("write the fixture script");
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(&path)
        .expect("fixture script metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&path, permissions).expect("make the fixture script executable");
}

/// A Cargo package at `directory`, recorded as the fixture's resolved `e2e`
/// package so the receipt assertions name the same key.
pub(crate) fn write_package(world: &mut GivnWorld, directory: &str) {
    write_manifest(
        world,
        &format!("{directory}/Cargo.toml"),
        "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n",
    );
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.resolved_packages.push(directory.to_string());
}

/// Point both verify commands at the fixture runner and record the E2E
/// command, so the receipt assertion knows the text its digest binds.
fn write_verify_commands(world: &mut GivnWorld, command: &str, e2e_command: &str) {
    common::write_commands_yaml(
        &env(world).path("givn/commands.yaml"),
        &format!("verify:\n  command: \"{command}\"\n  e2e_command: \"{e2e_command}\"\n"),
    );
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.runner_command = Some(e2e_command.to_string());
}

/// A fixture runner that writes one passing scenario for whichever scope it
/// is asked to run.
const PASSING_RUNNER: &str = "#!/bin/sh\n\
     printf '{\"scope\":\"%s\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}\\n' \
     \"${GIVN_RESULT_SCOPE:-regular}\" > \"$GIVN_RESULT_FILE\"\n";

/// Point both verify commands at fake scoped runners: the regular command
/// writes one passing scenario, the E2E command names the declared root.
pub(crate) fn write_scoped_runners(world: &mut GivnWorld) {
    write_executable(world, "emit-change-result.sh", PASSING_RUNNER);
    write_verify_commands(
        world,
        "./emit-change-result.sh",
        "./emit-change-result.sh tests/e2e",
    );
}

/// An `ise`-effective change whose delta carries one `@e2e` scenario and whose
/// fixture E2E run reports `failed` or `skipped` and exits nonzero. The
/// regular scope stays green, so only the E2E result can block the receipt.
pub(crate) fn ise_change_with_failing_e2e_run(world: &mut GivnWorld, failure: &str) {
    let (failed, skipped) = match failure {
        "failed" => (1, 0),
        "skipped" => (0, 1),
        other => panic!("unknown E2E failure state '{other}'"),
    };
    ise_change(world);
    declare(world, &["tests/e2e"], &[]);
    write_package(world, "tests/e2e");
    add_delta_e2e_scenario(world);
    let script = format!(
        "#!/bin/sh\n\
         if [ \"${{GIVN_RESULT_SCOPE:-regular}}\" = \"e2e\" ]; then\n\
         \x20 printf '{{\"scope\":\"e2e\",\"total\":2,\"passed\":1,\"failed\":{failed},\"skipped\":{skipped}}}\\n' \
         > \"$GIVN_RESULT_FILE\"\n\
         \x20 exit 1\n\
         fi\n\
         printf '{{\"scope\":\"change\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}}\\n' \
         > \"$GIVN_RESULT_FILE\"\n"
    );
    write_executable(world, "emit-failing-result.sh", &script);
    write_verify_commands(
        world,
        "./emit-failing-result.sh",
        "./emit-failing-result.sh tests/e2e",
    );
    commit(world);
}

/// An `ise`-effective change ready for a passing E2E run over two resolved
/// packages, both named by the E2E runner command.
pub(crate) fn ise_change_with_passing_e2e_over_two_packages(world: &mut GivnWorld) {
    ise_change(world);
    declare(world, &["tests/e2e", "tests/e2e-support"], &[]);
    write_package(world, "tests/e2e");
    write_package(world, "tests/e2e-support");
    add_delta_e2e_scenario(world);
    write_executable(world, "emit-two-package-result.sh", PASSING_RUNNER);
    write_verify_commands(
        world,
        "./emit-two-package-result.sh",
        "./emit-two-package-result.sh tests/e2e tests/e2e-support",
    );
    commit(world);
}

/// The declaration, root manifest, and standalone package of an acceptance
/// package that is its own workspace root and excluded from the product
/// package.
const STANDALONE_WORKSPACE: &str = r#"#!/bin/sh
# The project runner the fixture declares: one passing scenario per scope.
if [ -n "${GIVN_RESULT_FILE:-}" ]; then
    scope=${GIVN_RESULT_SCOPE:-regular}
    printf '{"scope":"%s","total":1,"passed":1,"failed":0,"skipped":0}\n' "$scope" \
        > "$GIVN_RESULT_FILE"
fi
"#;

/// An `ise`-effective change whose declared `e2e` package declares its own
/// workspace, is excluded from the product package, and is named by the E2E
/// runner command through `--package`; the delta carries one `@e2e` scenario.
pub(crate) fn ise_change_with_standalone_workspace(world: &mut GivnWorld) {
    ise_change(world);
    declare(world, &["e2e"], &["src/**"]);
    write_manifest(
        world,
        "Cargo.toml",
        "[package]\nname = \"fixture-product\"\nversion = \"0.1.0\"\nexclude = [\"e2e/**\"]\n",
    );
    write_manifest(
        world,
        "e2e/Cargo.toml",
        "[package]\nname = \"fixture-e2e\"\nversion = \"0.0.0\"\npublish = false\n\n[workspace]\n",
    );
    write_manifest(world, "src/lib.rs", "pub fn fixture() {}\n");
    write_executable(world, "run-tests.sh", STANDALONE_WORKSPACE);
    write_verify_commands(
        world,
        "./run-tests.sh --non-e2e",
        "./run-tests.sh --e2e --package e2e",
    );
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.resolved_packages.push("e2e".to_string());
    add_delta_e2e_scenario(world);
}

/// An `ise`-effective change ready for change-scoped verification: a resolved
/// `e2e` package, a green regular run, and no `@e2e` scenario in the delta.
pub(crate) fn ise_change_ready_for_verification(world: &mut GivnWorld) {
    ise_change(world);
    declare(world, &["tests/e2e"], &[]);
    write_package(world, "tests/e2e");
    write_scoped_runners(world);
    commit(world);
}

/// An `ise`-effective change that declares no behavior change, explicitly skips
/// specs, and has no `givn/specification.yaml`, so only the boundary preflight
/// can refuse it.
pub(crate) fn ise_change_without_behavior_change(world: &mut GivnWorld) {
    ise_change(world);
    let change = change_path(world);
    std::fs::write(
        change.join("proposal.md"),
        "# Proposal: ise-boundary\n\nBehavior change: none\n",
    )
    .expect("write the no-behavior-change proposal");
    std::fs::write(change.join(".givn-skip"), "specs\n").expect("write the specs skip");
    std::fs::remove_file(change.join("specs/verify-change/givn-workflow-enforcement.feature"))
        .expect("remove the delta feature");
    commit(world);
}

/// The same ready change with the lock recording `ise: false`, so no
/// declaration is read and no boundary preflight runs.
pub(crate) fn change_ready_for_verification_without_ise(world: &mut GivnWorld) {
    ise_change(world);
    std::fs::write(
        change_path(world).join("config-lock.yaml"),
        "version: 1\naddons:\n  ise: false\noverrides: []\n",
    )
    .expect("write the ise-off lock");
    write_scoped_runners(world);
    commit(world);
}

/// The receipt the fixture's change check wrote.
pub(crate) fn receipt_path(world: &GivnWorld) -> std::path::PathBuf {
    change_path(world).join("verification/tasks-verification.json")
}

fn write_receipt_json(world: &GivnWorld, receipt: &serde_json::Value) {
    std::fs::write(
        receipt_path(world),
        format!(
            "{}\n",
            serde_json::to_string_pretty(receipt).expect("serialize the receipt")
        ),
    )
    .expect("write the fixture receipt");
    commit(world);
}

fn read_receipt_json(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_str(
        &std::fs::read_to_string(receipt_path(world)).expect("read the fixture receipt"),
    )
    .expect("parse the fixture receipt")
}

/// Run the change check once, so the fixture starts from a receipt written by
/// the real check flow rather than one assembled by the test.
pub(crate) fn write_current_receipt(world: &mut GivnWorld) {
    let out = common::run_givn(env(world), &["check", "tasks", "--change", CHANGE_ID]);
    assert!(
        out.status.success(),
        "the fixture change check failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some(CHANGE_ID.to_string());
    commit(world);
}

/// Record `ise: true` in the project configuration, so the change lock matches
/// the effective add-on set the review gate compares against.
fn enable_ise_addon(world: &GivnWorld) {
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read the fixture config");
    let updated = content.replacen("\naddons:\n", "\naddons:\n  ise: true\n", 1);
    assert_ne!(
        updated, content,
        "the fixture config carries no addons block to enable ise in"
    );
    std::fs::write(&path, updated).expect("write the fixture config");
}

/// An `ise`-effective change holding a receipt written by a real change check:
/// the identity is current and the scope is `not_applicable`.
pub(crate) fn ise_change_with_current_receipt(world: &mut GivnWorld) {
    ise_change_ready_for_verification(world);
    enable_ise_addon(world);
    commit(world);
    write_current_receipt(world);
}

/// Add a tracked file under the resolved `e2e` package and commit it.
pub(crate) fn change_e2e_package(world: &mut GivnWorld) {
    write_file(world, "tests/e2e/harness.txt", "fixture suite change\n");
    commit(world);
}

/// Change the recorded E2E runner command and commit it, keeping the resolved
/// root named.
pub(crate) fn change_e2e_command(world: &mut GivnWorld) {
    let path = env(world).path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read the fixture commands");
    let updated = content.replace(
        "e2e_command: \"./emit-change-result.sh tests/e2e\"",
        "e2e_command: \"./emit-change-result.sh tests/e2e --recorded\"",
    );
    assert_ne!(
        updated, content,
        "the fixture E2E runner command was not rewritten"
    );
    std::fs::write(&path, updated).expect("write the fixture commands");
    commit(world);
}

/// Rewrite the recorded Givn version and commit the receipt.
pub(crate) fn change_recorded_givn_version(world: &mut GivnWorld) {
    let mut receipt = read_receipt_json(world);
    receipt["e2e_identity"]["givn_version"] = serde_json::json!("0.0.0-fixture");
    write_receipt_json(world, &receipt);
}

/// Delete the recorded E2E identity and commit the receipt.
pub(crate) fn remove_recorded_identity(world: &mut GivnWorld) {
    let mut receipt = read_receipt_json(world);
    let object = receipt.as_object_mut().expect("a receipt object");
    assert!(
        object.remove("e2e_identity").is_some(),
        "the fixture receipt carries no e2e identity to remove"
    );
    write_receipt_json(world, &receipt);
}

/// Make the declaration unreadable and commit it.
pub(crate) fn corrupt_declaration(world: &mut GivnWorld) {
    write_file(world, "givn/specification.yaml", "e2e: [\"tests/e2e\"\n");
    commit(world);
}

/// Append one `@e2e` scenario to the delta feature and commit it.
pub(crate) fn add_delta_e2e_scenario(world: &mut GivnWorld) {
    let feature = change_path(world).join("specs/verify-change/givn-workflow-enforcement.feature");
    let mut content = std::fs::read_to_string(&feature).expect("read the fixture delta feature");
    content.push_str(
        "\n  @givn.added @e2e\n  Scenario: Fixture E2E scenario\n    \
         Given a fixture precondition\n    Then a fixture result\n",
    );
    std::fs::write(&feature, content).expect("write the fixture delta feature");
    commit(world);
}

/// Record the given E2E scope status in the receipt and commit it.
pub(crate) fn record_e2e_scope(world: &mut GivnWorld, scope: &str) {
    let mut receipt = read_receipt_json(world);
    receipt["e2e_scope"] = match scope {
        "not_applicable" => serde_json::json!({ "status": "not_applicable" }),
        "passed" => serde_json::json!({
            "status": "passed",
            "total": 1,
            "passed": 1,
            "failed": 0,
            "skipped": 0,
            "seconds": 0.0,
        }),
        other => panic!("unknown fixture E2E scope '{other}'"),
    };
    write_receipt_json(world, &receipt);
}
