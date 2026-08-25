//! E2E black-box steps for current-change scenario exploration.

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

#[given(
    "an end-to-end project prepared for current-change exploration with an active added scenario"
)]
fn e2e_added_scenario_project(_world: &mut GivnWorld) {
    super::spec_explore_change_steps::active_added_scenario_project(_world);
}

#[when(regex = r#"^the end-to-end user explores "([^"]+)" against change "([^"]+)"$"#)]
fn e2e_explore_against_change(world: &mut GivnWorld, path: String, change: String) {
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

#[when(
    regex = r#"^the end-to-end user explores "([^"]+)" against change "([^"]+)" with JSON output$"#
)]
fn e2e_explore_against_change_json(world: &mut GivnWorld, path: String, change: String) {
    let env = world
        .env
        .as_ref()
        .expect("current-change project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec", "explore", "--change", &change, "--file", &path, "--format", "json",
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

#[then("stdout is a change exploration JSON document with permanent and current-change results")]
fn e2e_stdout_is_change_exploration_json(world: &mut GivnWorld) {
    let value = e2e_exploration_json(world);
    let results = value["results"].as_array().expect("results array");
    assert!(
        !results.is_empty(),
        "change exploration should return results"
    );
    assert_eq!(value["index"]["entries"], 2);
    assert!(results.iter().any(|result| result["scope"] == "permanent"));
    assert!(
        results
            .iter()
            .any(|result| result["scope"] == "current-change")
    );
    for result in results {
        assert!(result["rank"].as_u64().is_some());
        assert!(result["title"].as_str().is_some());
        assert!(result["file"].as_str().is_some());
        assert!(result["score"].as_f64().is_some());
    }
}

#[then("every JSON result carries its scope and owning change metadata")]
fn e2e_json_results_have_source_metadata(world: &mut GivnWorld) {
    let value = e2e_exploration_json(world);
    for result in value["results"].as_array().expect("results array") {
        match result["scope"].as_str() {
            Some("permanent") => assert!(result["change"].is_null()),
            Some("current-change") => {
                assert_eq!(result["change"], "current-change");
                assert_eq!(
                    result["file"],
                    "givn/changes/current-change/specs/current-change/current-change.feature"
                );
            }
            other => panic!("unexpected exploration scope: {other:?}"),
        }
    }
}

fn e2e_exploration_json(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_str(&stdout(world)).expect("change exploration stdout must be JSON")
}

#[then(regex = r#"^stdout contains a current-change candidate titled "([^"]+)"$"#)]
fn e2e_stdout_contains_current_change_candidate(world: &mut GivnWorld, title: String) {
    assert!(ranked_line_contains(world, &title));
}

#[then(
    regex = r#"^stdout identifies the current-change candidate with scope "([^"]+)" and change "([^"]+)"$"#
)]
fn e2e_stdout_identifies_current_change_scope(
    world: &mut GivnWorld,
    scope: String,
    change: String,
) {
    assert!(ranked_line_contains(
        world,
        &format!("scope {scope}, change {change}")
    ));
}

#[then(regex = r#"^stdout identifies the current-change candidate source as "([^"]+)"$"#)]
fn e2e_stdout_identifies_current_change_source(world: &mut GivnWorld, source: String) {
    assert!(ranked_line_contains(world, &format!("source {source}")));
}

fn ranked_line_contains(world: &GivnWorld, needle: &str) -> bool {
    stdout(world)
        .lines()
        .filter(|line| line.trim_start().starts_with("rank "))
        .any(|line| line.contains(needle))
}
