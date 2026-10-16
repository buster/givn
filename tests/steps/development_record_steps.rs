//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use cucumber::{given, then};
use serde_json::{Value, json};

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;

/// State for the development-record scenarios: the entry the last reading
/// step identified, so a following step can assert on "that entry".
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordState {
    pub(crate) last_entry: Option<Value>,
}

#[then("the development record holds one entry for that command")]
fn record_holds_one_entry_for_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let command = world
        .last_command
        .clone()
        .expect("no `givn` command has been run yet");
    let entries = support::record_entries(&env.root());
    let matching: Vec<Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("command") && entry["name"] == json!(command))
        .cloned()
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "expected exactly one `command` entry for `{command}`; the record holds: {entries:#?}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(matching[0].clone());
}

#[given(regex = r#"^a change "([^"]+)" with a verification command that takes measurable time$"#)]
fn change_with_a_slow_verification_command(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr),
    );
    support::declare_verify_command(env, support::SLOW_VERIFY_COMMAND);
    world.last_change_id = Some(id);
}

#[then("the development record holds one entry for the verification run")]
fn record_holds_one_entry_for_the_verification_run(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let command = entries
        .iter()
        .find(|entry| entry["kind"] == json!("command") && entry["name"] == json!("check"))
        .unwrap_or_else(|| panic!("no `command` entry for the check invocation: {entries:#?}"));
    let command_id = command["id"].clone();
    let steps: Vec<Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("step") && entry["parent"] == command_id)
        .cloned()
        .collect();
    assert_eq!(
        steps.len(),
        1,
        "expected exactly one `step` entry under the check command; the record holds: {entries:#?}"
    );
    let step = &steps[0];
    let label = step["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the verification step carries no label: {step}"));
    assert!(
        label.contains(support::SLOW_VERIFY_COMMAND),
        "the label does not name the verification run: {step}"
    );
    assert!(
        label.chars().count() <= 120,
        "the label is not bounded to 120 characters: {step}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(step.clone());
}

#[then("that entry carries a duration of at least one second")]
fn entry_carries_at_least_one_second(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no record entry was identified by the previous step");
    let duration_ms = entry["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the entry carries no measured duration: {entry}"));
    assert!(
        duration_ms >= 1_000,
        "the verification step is not measured as at least one second: {entry}"
    );
    let outcome = entry["outcome"]
        .as_str()
        .unwrap_or_else(|| panic!("the entry carries no outcome: {entry}"));
    assert!(
        ["ok", "failed", "skipped", "unknown"].contains(&outcome),
        "unexpected outcome `{outcome}`: {entry}"
    );
}

#[then("that entry carries a measured duration and its outcome")]
fn entry_carries_a_measured_duration_and_its_outcome(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no record entry was identified by the previous step");
    let duration_ms = entry["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the entry carries no measured duration: {entry}"));
    assert!(
        duration_ms < 600_000,
        "implausible measured duration {duration_ms} ms: {entry}"
    );
    assert_eq!(entry["outcome"], json!("ok"), "unexpected outcome: {entry}");
}
