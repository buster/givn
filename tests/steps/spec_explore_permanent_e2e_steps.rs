//! E2E black-box steps for the real permanent exploration CLI.

use std::fs;

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

#[given("an end-to-end project prepared for permanent scenario exploration")]
fn e2e_exploration_project(world: &mut GivnWorld) {
    super::spec_explore_permanent_steps::prepare_exploration_project(world);
}

#[when(regex = r#"^the end-to-end user explores "([^"]+)" with the file input$"#)]
fn e2e_explore_file(world: &mut GivnWorld, path: String) {
    super::spec_explore_permanent_steps::run_file_explore(world, &path, false);
}

#[when("the end-to-end user explores the same draft through standard input")]
fn e2e_explore_stdin(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("e2e exploration project is required");
    let draft = fs::read_to_string(env.path("draft.feature")).expect("read draft feature");
    if let Some(previous) = world.last_output.take() {
        world.previous_output = Some(previous);
    }
    world.last_output = Some(common::run_givn_with_stdin_and_envs(
        env,
        &["spec", "explore", "--stdin"],
        &draft,
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

#[when(regex = r#"^the end-to-end user explores "([^"]+)" with JSON output$"#)]
fn e2e_explore_json(world: &mut GivnWorld, path: String) {
    super::spec_explore_permanent_steps::run_file_explore(world, &path, true);
}

#[then("the standard-input results equal the file results")]
fn stdin_results_equal_file_results(world: &mut GivnWorld) {
    let file_output = world
        .previous_output
        .as_ref()
        .expect("file exploration output was not saved");
    let file_stdout = String::from_utf8_lossy(&file_output.stdout);
    let stdin_stdout = stdout(world);
    let file_results: Vec<&str> = file_stdout
        .lines()
        .filter(|line| line.trim_start().starts_with("rank "))
        .collect();
    let stdin_results: Vec<&str> = stdin_stdout
        .lines()
        .filter(|line| line.trim_start().starts_with("rank "))
        .collect();
    assert_eq!(stdin_results, file_results);
}

#[then("stdout is a stable exploration JSON document")]
fn stdout_is_exploration_json(world: &mut GivnWorld) {
    let value: serde_json::Value =
        serde_json::from_str(&stdout(world)).expect("explore stdout must be one JSON document");
    assert!(value.is_object(), "explore JSON must be an object");
}

#[then(regex = r#"^the exploration JSON query title is "([^"]+)"$"#)]
fn json_query_title(world: &mut GivnWorld, expected: String) {
    let value: serde_json::Value =
        serde_json::from_str(&stdout(world)).expect("parse explore JSON");
    assert_eq!(value["query"]["title"], expected);
}

#[then("the exploration JSON includes the query content hash")]
fn json_query_hash(world: &mut GivnWorld) {
    let value: serde_json::Value =
        serde_json::from_str(&stdout(world)).expect("parse explore JSON");
    assert!(
        value["query"]["content_hash"]
            .as_str()
            .is_some_and(|hash| !hash.is_empty())
    );
}

#[then("the exploration JSON includes the embedding model and index metadata")]
fn json_index_metadata(world: &mut GivnWorld) {
    let value: serde_json::Value =
        serde_json::from_str(&stdout(world)).expect("parse explore JSON");
    assert!(
        value["index"]["model"]
            .as_str()
            .is_some_and(|model| !model.is_empty())
    );
    assert!(value["index"]["version"].as_u64().is_some());
    assert!(value["index"]["entries"].as_u64().is_some());
}

#[then("every exploration JSON result has rank title file and score")]
fn json_result_fields(world: &mut GivnWorld) {
    let value: serde_json::Value =
        serde_json::from_str(&stdout(world)).expect("parse explore JSON");
    let results = value["results"].as_array().expect("results array");
    assert!(
        !results.is_empty(),
        "JSON should contain at least one result"
    );
    for result in results {
        assert!(result["rank"].as_u64().is_some());
        assert!(result["title"].as_str().is_some());
        assert!(result["file"].as_str().is_some());
        assert!(result["score"].as_f64().is_some());
    }
}
