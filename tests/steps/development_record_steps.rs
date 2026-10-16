//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use cucumber::{given, then};
use serde_json::{Value, json};

use crate::common;
use crate::steps::development_timeline_support as support;
use crate::steps::{GivnWorld, stderr};

/// State for the development-record scenarios: the entry the last reading
/// step identified, so a following step can assert on "that entry", plus the
/// store snapshot the append-only scenario takes before its second command.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordState {
    pub(crate) last_entry: Option<Value>,
    /// The store's bytes as they stood before this scenario's next command.
    pub(crate) store_before: Option<Vec<u8>>,
    /// The earlier entry's own field values, read the way a consumer reads
    /// them.
    pub(crate) earlier_entry: Option<Value>,
}

/// The byte-level form of "nothing earlier was rewritten": the store still
/// begins with the bytes it held before the scenario's next command. Both
/// append-only steps state the property through this one assertion.
fn assert_store_prefix_unchanged(before: &[u8], after: &[u8]) {
    assert!(
        after.starts_with(before),
        "the store's earlier bytes were rewritten:\nbefore: {}\nafter:  {}",
        String::from_utf8_lossy(before),
        String::from_utf8_lossy(after)
    );
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

#[given("the development record cannot be written")]
fn record_cannot_be_written(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // The store path itself is blocked (a directory where the file belongs), so
    // the append fails the way a full or read-only disk fails it — a real
    // failure of the real write path, never a simulated one.
    support::make_record_unwritable(env);
}

#[then("stderr warns that the development record could not be written")]
fn stderr_warns_that_the_record_could_not_be_written(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let store = support::store_file(&env.root());
    let lines = stderr(world);
    let warnings: Vec<&str> = lines
        .lines()
        .filter(|line| line.contains("development record"))
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected exactly one warning naming the development record (no retry loop): {lines:?}"
    );
    let warning = warnings[0];
    assert!(
        warning.starts_with("warning:") && warning.contains("could not be written"),
        "the line is not a warning that the record could not be written: {warning}"
    );
    assert!(
        warning.contains(&store.display().to_string()),
        "the warning does not name the store path {}: {warning}",
        store.display()
    );
    // The warning is the only observable effect: no silent fallback wrote the
    // entry elsewhere, so the store path is still the blocking directory and
    // the record holds no entry.
    assert!(
        store.is_dir(),
        "the append wrote to the blocked store path {}",
        store.display()
    );
}

#[given("the development record holds an entry from an earlier command")]
fn record_holds_an_entry_from_an_earlier_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["status"]);
    assert!(
        out.status.success(),
        "the earlier command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let entries = support::record_entries(&env.root());
    let earlier = entries
        .last()
        .cloned()
        .expect("the earlier command wrote no entry");
    assert_eq!(
        (earlier["kind"].clone(), earlier["name"].clone()),
        (json!("command"), json!("status")),
        "the last entry is not the earlier command's own entry: {earlier}"
    );
    let state = world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default);
    state.store_before = Some(support::store_bytes(&env.root()));
    state.earlier_entry = Some(earlier);
}

#[then("the earlier entry is still present unchanged")]
fn earlier_entry_is_still_present_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let state = world
        .development_record
        .as_ref()
        .expect("the earlier-command fixture ran first");
    let before = state
        .store_before
        .clone()
        .expect("the fixture took no store snapshot");
    let earlier = state
        .earlier_entry
        .clone()
        .expect("the fixture identified no earlier entry");
    let after = support::store_bytes(&env.root());
    assert_store_prefix_unchanged(&before, &after);
    let entries = support::record_entries(&env.root());
    let read_back = entries
        .iter()
        .find(|entry| entry["id"] == earlier["id"])
        .unwrap_or_else(|| panic!("the earlier entry is gone: {entries:#?}"));
    assert_eq!(
        read_back, &earlier,
        "the earlier entry's field values changed"
    );
}

#[then("the new entry is appended after it without rewriting the store")]
fn new_entry_is_appended_after_it_without_rewriting_the_store(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("the earlier-command fixture ran first");
    let after = support::store_bytes(&env.root());
    assert_store_prefix_unchanged(&before, &after);
    let appended = &after[before.len()..];
    let appended = std::str::from_utf8(appended)
        .unwrap_or_else(|error| panic!("the appended bytes are not UTF-8: {error}"));
    let appended_lines: Vec<&str> = appended.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        appended_lines.len(),
        1,
        "the command appended {} lines, not one: {appended:?}",
        appended_lines.len()
    );
    let entry: Value = serde_json::from_str(appended_lines[0])
        .unwrap_or_else(|error| panic!("the appended line is not a JSON object: {error}"));
    assert_eq!(
        (entry["kind"].clone(), entry["name"].clone()),
        (json!("command"), json!("status")),
        "the appended entry is not this command's own entry: {entry}"
    );
}

#[given("no change and no ideation is active for this project")]
fn no_change_and_no_ideation_is_active(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    support::assert_bare_project(env);
}

#[then("the recorded entry names the project as its scope")]
fn recorded_entry_names_the_project_as_its_scope(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let command = world
        .last_command
        .clone()
        .expect("no `givn` command has been run yet");
    let entries = support::record_entries(&env.root());
    let recorded: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("command") && entry["name"] == json!(command))
        .collect();
    assert_eq!(
        recorded.len(),
        1,
        "expected exactly one `command` entry for `{command}`; the record holds: {entries:#?}"
    );
    let scope = &recorded[0]["scope"];
    assert_eq!(
        scope["kind"],
        json!("project"),
        "the entry is not scoped to the project: {scope}"
    );
    assert_eq!(
        scope["id"],
        json!(support::project_name(env)),
        "the entry's scope does not name the fixture's own project: {scope}"
    );
}
