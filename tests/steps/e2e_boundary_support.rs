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

pub(crate) fn set_import_target(world: &mut GivnWorld, target: &str) {
    let state = world
        .e2e_boundary
        .as_mut()
        .expect("an ise boundary fixture is required first");
    state.import_target = Some(target.to_string());
}
