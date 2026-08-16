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
