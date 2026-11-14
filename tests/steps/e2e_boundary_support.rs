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

/// Point both verify commands at fake scoped runners: the regular command
/// writes one passing scenario, the E2E command names the declared root.
pub(crate) fn write_scoped_runners(world: &mut GivnWorld) {
    let script = "#!/bin/sh\n\
                  printf '{\"scope\":\"%s\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}\\n' \
                  \"${GIVN_RESULT_SCOPE:-regular}\" > \"$GIVN_RESULT_FILE\"\n";
    write_executable(world, "emit-change-result.sh", script);
    common::write_commands_yaml(
        &env(world).path("givn/commands.yaml"),
        "verify:\n  command: \"./emit-change-result.sh\"\n  \
         e2e_command: \"./emit-change-result.sh tests/e2e\"\n",
    );
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
