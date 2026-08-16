//! Regular black-box steps for current-change scenario exploration.

use std::fs;

use cucumber::{given, then, when};

use super::{GivnWorld, output, stdout};
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

#[given(
    "a project prepared for current-change exploration with a modified scenario and a separate eligible candidate"
)]
fn modified_target_scenario_project(world: &mut GivnWorld) {
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
",
    );
    write_project_path(
        &env,
        "givn/specs/other/other.feature",
        "Feature: other provider behavior

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

#[given("a project prepared for current-change exploration with an active added scenario")]
fn active_added_scenario_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_project_path(&env, "givn/specs/provider.feature", PERMANENT_CORPUS);
    write_project_path(
        &env,
        "givn/changes/current-change/specs/current-change/current-change.feature",
        "@givn.delta @current-change
Feature: current provider behavior

  @givn.added
  Scenario: Current change rejects an unknown provider
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given(
    "a project prepared for current-change exploration with an active added scenario in a one-scenario delta file"
)]
fn active_added_single_scenario_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_project_path(&env, "givn/specs/provider.feature", PERMANENT_CORPUS);
    write_project_path(
        &env,
        "givn/changes/current-change/specs/current-change/added-provider.feature",
        "@givn.delta @current-change
Feature: current provider behavior

  @givn.added
  Scenario: Current change rejects an unknown provider
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("the permanent exploration index is built")]
fn build_permanent_exploration_index(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    let output =
        common::run_givn_with_envs(env, &["spec", "index"], &[("GIVN_SPEC_EMBEDDER", "stub")]);
    assert!(
        output.status.success(),
        "permanent index build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[given("the permanent exploration index is snapshotted")]
fn snapshot_permanent_exploration_index(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    world.permanent_index_snapshot = Some(
        fs::read(env.path("givn/.spec-index.json")).expect("read permanent exploration index"),
    );
}

#[then("the permanent exploration index is unchanged")]
fn permanent_exploration_index_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    let before = world
        .permanent_index_snapshot
        .as_ref()
        .expect("permanent exploration index was not snapshotted");
    let after = fs::read(env.path("givn/.spec-index.json")).expect("read permanent index after");
    assert_eq!(after, *before, "exploration changed the permanent index");
}

#[when(
    "the user explores the modified delta scenario once by its relative path and once by its absolute project path against change \"current-change\""
)]
fn explore_modified_relative_and_absolute(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    let relative = "givn/changes/current-change/specs/current-change/current-change.feature";
    let first = common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--change",
            "current-change",
            "--file",
            relative,
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    );
    let absolute = env.path(relative).to_string_lossy().into_owned();
    let second = common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--change",
            "current-change",
            "--file",
            &absolute,
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    );
    world.previous_output = Some(first);
    world.last_output = Some(second);
}

#[then("both requests exit 0")]
fn both_requests_exit_zero(world: &mut GivnWorld) {
    assert!(
        world
            .previous_output
            .as_ref()
            .expect("relative request output is missing")
            .status
            .success()
    );
    assert!(output(world).status.success(), "absolute request failed");
}

#[then(regex = r#"^neither result is titled "([^"]+)"$"#)]
fn neither_result_is_titled(world: &mut GivnWorld, title: String) {
    let first = world
        .previous_output
        .as_ref()
        .expect("relative request output is missing");
    assert!(!ranked_output_contains(first, &title));
    assert!(!ranked_output_contains(output(world), &title));
}

#[then("both requests retain another eligible candidate")]
fn both_requests_retain_candidate(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("relative request output is missing");
    assert!(ranked_output_has_result(first));
    assert!(ranked_output_has_result(output(world)));
}

fn ranked_output_contains(output: &std::process::Output, title: &str) -> bool {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.trim_start().starts_with("rank "))
        .any(|line| line.contains(title))
}

fn ranked_output_has_result(output: &std::process::Output) -> bool {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .any(|line| line.trim_start().starts_with("rank "))
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
