//! Regular black-box steps for current-change scenario exploration.

use std::fs;

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

const PERMANENT_CORPUS: &str = "Feature: provider behavior

  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Authentication token is missing
    Given a configured provider catalog
    When the user authenticates without a token
    Then authentication is rejected
";

#[given("a project prepared for current-change exploration with a removed scenario")]
fn removed_scenario_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_project_path(&env, "givn/specs/provider.feature", PERMANENT_CORPUS);
    write_project_path(
        &env,
        "givn/changes/current-change/specs/current-change/current-change.feature",
        "@givn.delta @current-change
Feature: current provider behavior

  @givn.removed
  Scenario: Removed provider behavior
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("a project prepared for current-change exploration with a modified scenario")]
fn modified_scenario_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_project_path(
        &env,
        "givn/specs/current-change/current-change.feature",
        "Feature: current provider behavior

  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Authentication token is missing
    Given a configured provider catalog
    When the user authenticates without a token
    Then authentication is rejected
",
    );
    write_project_path(
        &env,
        "givn/changes/current-change/specs/current-change/current-change.feature",
        "@givn.delta @current-change
Feature: current provider behavior

  @givn.modified
  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider from the updated catalog
    Then the command rejects the request
",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[when(regex = r#"^the user explores "([^"]+)" against change "([^"]+)"$"#)]
fn explore_against_change(world: &mut GivnWorld, path: String, change: String) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--change", &change, "--file", &path],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

#[then(regex = r#"^no ranked result is titled "([^"]+)"$"#)]
fn no_ranked_result(world: &mut GivnWorld, title: String) {
    assert!(
        !stdout(world)
            .lines()
            .filter(|line| line.trim_start().starts_with("rank "))
            .any(|line| line.contains(&title)),
        "ranked result unexpectedly contains {title:?}: {}",
        stdout(world)
    );
}

#[then("another eligible candidate remains visible")]
fn another_eligible_candidate(world: &mut GivnWorld) {
    assert!(
        stdout(world)
            .lines()
            .any(|line| line.trim_start().starts_with("rank ")),
        "expected another ranked candidate: {}",
        stdout(world)
    );
}

fn clear_specs(env: &common::TestEnv) {
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
}

fn write_project_path(env: &common::TestEnv, path: &str, content: &str) {
    let file = env.path(path);
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).expect("create project file parent");
    }
    fs::write(file, content).expect("write project file");
}
