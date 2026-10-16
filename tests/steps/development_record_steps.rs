//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use cucumber::then;
use serde_json::{Value, json};

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
