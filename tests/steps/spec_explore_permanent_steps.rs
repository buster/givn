//! Regular black-box steps for permanent scenario exploration.

use std::fs;
use std::process::{Command, Stdio};

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

const PERMANENT_CORPUS: &str = "Feature: provider behavior

  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Unknown provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Authentication token is missing
    Given a configured provider catalog
    When the user authenticates without a token
    Then authentication is rejected
";

pub(crate) fn prepare_exploration_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_feature(&env, "givn/specs/provider.feature", PERMANENT_CORPUS);
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

fn prepare_identity_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_specs(&env);
    write_feature(
        &env,
        "givn/specs/provider.feature",
        "Feature: provider behavior\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    );
    write_feature(
        &env,
        "givn/specs/auth.feature",
        "Feature: provider authentication\n\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    );
    write_feature(
        &env,
        "givn/specs/token.feature",
        "Feature: authentication\n\n  Scenario: Authentication token is missing\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("a project prepared for permanent scenario exploration")]
fn exploration_project(world: &mut GivnWorld) {
    prepare_exploration_project(world);
}

#[given("a project prepared for exact scenario identity exploration")]
fn identity_project(world: &mut GivnWorld) {
    prepare_identity_project(world);
}

#[given(regex = r#"^a file at "([^"]+)" containing invalid Gherkin$"#)]
fn invalid_gherkin_file(world: &mut GivnWorld, path: String) {
    write_project_file(
        world,
        &path,
        "this is not a Gherkin feature document\nwithout a structural keyword\n",
    );
}

#[given(regex = r#"^a file at "([^"]+)" containing two complete scenarios$"#)]
fn multiple_scenario_file(world: &mut GivnWorld, path: String) {
    write_project_file(
        world,
        &path,
        "Feature: multiple\n\n  Scenario: First\n    Given a first scenario\n\n  Scenario: Second\n    Given a second scenario\n",
    );
}

#[given(regex = r#"^a file at "([^"]+)" containing no scenario$"#)]
fn empty_scenario_file(world: &mut GivnWorld, path: String) {
    write_project_file(world, &path, "Feature: no scenarios\n");
}

#[when(regex = r#"^the user explores "([^"]+)" with the file input$"#)]
fn explore_file(world: &mut GivnWorld, path: String) {
    run_file_explore(world, &path, false);
}

#[when(regex = r#"^the user explores "([^"]+)" with both file and standard input$"#)]
fn explore_both_inputs(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("exploration project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", &path, "--stdin"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

pub(crate) fn run_file_explore(world: &mut GivnWorld, path: &str, json: bool) {
    let env = world.env.as_ref().expect("exploration project is required");
    let mut args = vec!["spec", "explore", "--file", path];
    if json {
        args.extend(["--format", "json"]);
    }
    let environments = [("GIVN_SPEC_EMBEDDER", "stub")];
    world.last_output = Some(common::run_givn_with_envs(env, &args, &environments));
}

#[when(regex = r#"^the permanent scenario "([^"]+)" is changed$"#)]
fn change_permanent_scenario(world: &mut GivnWorld, title: String) {
    let env = world.env.as_ref().expect("exploration project is required");
    let path = env.path("givn/specs/provider.feature");
    let content = fs::read_to_string(&path).expect("read permanent feature to change");
    let changed = match title.as_str() {
        "Authentication token is missing" => content.replace(
            "When the user authenticates without a token",
            "When the user authenticates without a configured token",
        ),
        other => panic!("unsupported permanent scenario fixture: {other}"),
    };
    fs::write(path, changed).expect("write changed permanent feature");
}

#[when("the user runs explore through a build without spec retrieval")]
fn explore_without_retrieval_feature(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("exploration project is required");
    let manifest = env!("CARGO_MANIFEST_DIR");
    let manifest_path = format!("{manifest}/Cargo.toml");
    let target_dir = env.path(".cargo-target-no-spec-retrieval");
    let output = Command::new("cargo")
        .current_dir(env.root())
        .args([
            "run",
            "--quiet",
            "--manifest-path",
            manifest_path.as_str(),
            "--no-default-features",
            "--bin",
            "givn",
            "--",
            "spec",
            "explore",
            "--stdin",
        ])
        .env("CARGO_TARGET_DIR", target_dir)
        .stdin(Stdio::null())
        .output()
        .expect("run no-feature givn process");
    world.last_output = Some(output);
}

#[given("the permanent specifications are snapshotted")]
fn snapshot_permanent_specs(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("exploration project is required");
    let specs = env.path("givn/specs");
    let mut snapshot = Vec::new();
    for path in givn::gherkin::parse::collect_feature_files(&specs) {
        let relative = path
            .strip_prefix(env.root())
            .expect("permanent feature under project root")
            .to_string_lossy()
            .replace('\\', "/");
        snapshot.push((relative, fs::read(path).expect("read permanent feature")));
    }
    world.permanent_spec_snapshot = Some(snapshot);
}

#[then(regex = r#"^no human-readable result is titled "([^"]+)"$"#)]
fn no_human_result_title(world: &mut GivnWorld, title: String) {
    assert!(
        !stdout(world)
            .lines()
            .filter(|line| line.trim_start().starts_with("rank "))
            .any(|line| line.contains(&title)),
        "human result unexpectedly contains {title:?}: {}",
        stdout(world)
    );
}

#[then("the permanent specifications are unchanged")]
fn permanent_specs_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("exploration project is required");
    let before = world
        .permanent_spec_snapshot
        .as_ref()
        .expect("permanent specifications were not snapshotted");
    for (relative, expected) in before {
        let actual = fs::read(env.path(relative)).expect("read permanent feature after explore");
        assert_eq!(
            &actual, expected,
            "permanent feature changed after exploration: {relative}"
        );
    }
}

fn clear_specs(env: &common::TestEnv) {
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
}

fn write_feature(env: &common::TestEnv, path: &str, content: &str) {
    write_project_path(env, path, content);
}

fn write_project_file(world: &mut GivnWorld, path: &str, content: &str) {
    let env = world.env.as_ref().expect("exploration project is required");
    write_project_path(env, path, content);
}

fn write_project_path(env: &common::TestEnv, path: &str, content: &str) {
    let file = env.path(path);
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).expect("create project file parent");
    }
    fs::write(file, content).expect("write project file");
}
