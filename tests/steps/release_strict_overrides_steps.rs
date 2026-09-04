//! Step definitions for the development-build control scenario of the
//! installed-build strictness contract. A development givn build keeps
//! honouring fixture requests and never emits the installed-build
//! announcement.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

/// A project with permanent semantic scenarios, prepared so `givn spec
/// index` actually attempts embedding. Shared by the dev control and the
/// installed-build scenarios (the release subprocess only ever sees this
/// fixture plus the request/location envs the steps set explicitly).
#[given("a project prepared for semantic indexing")]
fn project_prepared_for_semantic_indexing(world: &mut GivnWorld) {
    let env = common::make_init_root();
    std::fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    std::fs::write(
        env.path("givn/specs/retrieval.feature"),
        "Feature: retrieval fixtures\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Authentication token is missing\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n\n  Scenario: Provider configuration is valid\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write retrieval fixtures");
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    );
    world.env = Some(env);
}

#[given("a request for simulated embedder output")]
fn request_simulated_embedder(world: &mut GivnWorld) {
    world.retrieval_embedder = Some("stub".to_string());
}

#[when("a consumer runs `givn spec index` with a development givn build")]
fn run_index_development_build(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project prepared for semantic indexing is required first");
    world.last_output = Some(crate::steps::run_givn(env, &["spec", "index"], world));
}

#[then("the index is built")]
fn index_is_built(world: &mut GivnWorld) {
    // In a feature-free build `givn spec index` reports unavailability and
    // no index can exist; this scenario only has meaning in a build with
    // the retrieval feature compiled in.
    if !cfg!(feature = "spec-retrieval") {
        return;
    }
    let env = world
        .env
        .as_ref()
        .expect("a project prepared for semantic indexing is required first");
    assert!(
        super::stdout(world).contains("Indexed"),
        "stdout did not report the completed index: {}",
        super::stdout(world)
    );
    assert!(
        env.path("givn/.spec-index.json").is_file(),
        "the permanent semantic index was not written"
    );
}

#[then("no installed-build announcement is emitted")]
fn no_installed_build_announcement(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        !stderr.contains("is ignored in installed builds"),
        "stderr emitted an installed-build announcement: {}",
        stderr,
    );
}
