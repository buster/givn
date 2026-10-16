//! `@e2e` step definitions for the `development-record` capability (Task E2,
//! Task E3).
//!
//! The scenarios under this file drive the real CLI as a subprocess in a
//! temporary project root (`common::run_givn`) and assert on what it printed
//! and on the files it wrote — never on the store read behind the command's
//! back. The capability's unit/integration steps live in
//! [`crate::steps::development_record_steps`]; this file holds only the steps
//! that need the real consumer interface.
//!
//! Both scenarios are CLI interfaces, so the browser is used for one thing
//! only: the change-wide visual gate's evidence — the report the scenario's own
//! record produced, rendered in the real browser at both contract viewports,
//! captured after the final assertion.

use serde_json::{Value, json};

use cucumber::then;

use super::GivnWorld;
use super::development_timeline_support as support;
use crate::common;
use crate::common::TestEnv;
use crate::common::browser::{self, BrowserDriver};

/// The `@e2e` scenario whose record names the lifecycle command; spelled as the
/// delta feature spells it, because the visual gate slugifies exactly this text.
const LIFECYCLE_SCENARIO: &str = "Consumer runs a lifecycle step and reads its recorded actions";

/// The `@e2e` scenario that reads the whole record as machine-readable output.
const MACHINE_READABLE_SCENARIO: &str =
    "Consumer reads the development record as machine-readable output";

/// State for the `@e2e` development-record scenarios: the machine-readable
/// record the scenario's own read printed
/// (`givn history --change … --json`), parsed once, so every Then step asserts
/// against the same parse its When read.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordE2eState {
    /// The entries the last machine-readable read printed.
    pub(crate) entries: Vec<Value>,
}

/// The scenario's project root, as its Givens left it.
fn project_root(world: &GivnWorld) -> &TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// The entries the scenario's own machine-readable read printed, parsed once:
/// the first Then step parses the read view's own stdout into the scenario's
/// state and every later Then step asserts against that same parse, never
/// against a second invocation or a private store read.
fn printed_entries(world: &mut GivnWorld) -> Vec<Value> {
    if let Some(state) = world.development_record_e2e.as_ref() {
        return state.entries.clone();
    }
    let printed = super::stdout(world);
    let entries: Vec<Value> = serde_json::from_str(printed.trim()).unwrap_or_else(|error| {
        panic!("the machine-readable read printed no JSON entries ({error}): {printed:?}")
    });
    world
        .development_record_e2e
        .get_or_insert_with(DevelopmentRecordE2eState::default)
        .entries = entries.clone();
    entries
}

/// The one printed `command` entry for `name`, or a failure naming what the
/// read actually printed.
fn command_entry<'a>(entries: &'a [Value], name: &str) -> &'a Value {
    entries
        .iter()
        .find(|entry| entry["kind"] == json!("command") && entry["name"] == json!(name))
        .unwrap_or_else(|| {
            panic!("the machine-readable record names no `{name}` command entry: {entries:#?}")
        })
}

/// Every printed entry is complete on the read view's own terms — its kind, its
/// exactly-one scope, and a measured duration — and the read covers exactly one
/// scope throughout. Returns the scope the read named.
fn assert_entries_complete(entries: &[Value]) -> Value {
    assert!(
        !entries.is_empty(),
        "the machine-readable read printed no entry at all"
    );
    for entry in entries {
        let kind = entry["kind"].as_str().unwrap_or_default();
        assert!(!kind.is_empty(), "a printed entry names no kind: {entry:#}");
        assert!(
            entry["scope"]["kind"].is_string() && entry["scope"]["id"].is_string(),
            "the printed {kind} entry names no scope: {entry:#}"
        );
        assert!(
            entry["duration_ms"].is_u64(),
            "the printed {kind} entry names no duration: {entry:#}"
        );
    }
    let scope = entries[0]["scope"].clone();
    assert!(
        scope["kind"].is_string(),
        "the read printed a scope with no kind: {scope:#}"
    );
    for entry in entries {
        assert_eq!(
            entry["scope"], scope,
            "a printed entry left the read's scope: {entry:#}"
        );
    }
    scope
}

/// The read is complete for the scope it named: it printed one entry per
/// recorded line of that scope. The printed entries are the primary evidence;
/// the record's own line count is the secondary check that none of them was
/// dropped.
fn assert_read_is_complete(entries: &[Value], scope: &Value, recorded: &[Value]) {
    let lines = recorded
        .iter()
        .filter(|entry| &entry["scope"] == scope)
        .count();
    assert_eq!(
        entries.len(),
        lines,
        "the machine-readable read printed {} of the scope's {lines} recorded entries: {entries:#?}",
        entries.len()
    );
}

#[then(regex = r#"^the machine-readable record names the status command for "([^"]+)"$"#)]
fn record_names_the_status_command(world: &mut GivnWorld, change: String) {
    let entries = printed_entries(world);
    let command = command_entry(&entries, "status");
    assert_eq!(
        command["scope"],
        json!({ "kind": "change", "id": change }),
        "the recorded status command does not belong to the change {change}: {command:#}"
    );
    let label = command["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the recorded status command carries no label: {command:#}"));
    assert!(
        label.contains("status") && label.contains(&change),
        "the recorded status command's label does not name the invocation: {label:?}"
    );
}

#[then("the machine-readable record carries a duration for that command")]
fn record_carries_a_duration_for_that_command(world: &mut GivnWorld) {
    let entries = printed_entries(world);
    let command = command_entry(&entries, "status");
    let duration = command["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the recorded status command carries no duration: {command:#}"));
    // A measured span, not a placeholder: the entry's own wall time, in
    // milliseconds, is the only thing the field can hold.
    assert!(
        duration < 60_000,
        "the recorded status command's duration is not a measured span: {duration} ms"
    );
    commit_evidence(project_root(world), LIFECYCLE_SCENARIO);
}

#[then("the output names every recorded entry with its kind, its scope, and its duration")]
fn output_names_every_recorded_entry(world: &mut GivnWorld) {
    let entries = printed_entries(world);
    let scope = assert_entries_complete(&entries);
    assert_read_is_complete(
        &entries,
        &scope,
        &support::record_entries(&project_root(world).root()),
    );
    commit_evidence(project_root(world), MACHINE_READABLE_SCENARIO);
}

/// The change-wide visual gate's evidence for one `@e2e` CLI scenario: render
/// the report the scenario's own record produced with the real `givn timeline`,
/// then open that one file in the real browser and capture the contract's pair
/// (`mobile.png` 360x780, `desktop.png` 1920x1080) under
/// `evidence/visual/<scenario-slug>/`. Captured after the scenario's final
/// assertion, with animations disabled by the driver; the report is rendered
/// from the record's embedded timestamps, so the capture never depends on
/// timing.
fn commit_evidence(root: &TestEnv, scenario: &str) {
    let out = common::run_givn(
        root,
        &["timeline", "--change", "demo", "--out", "report.html"],
    );
    assert!(
        out.status.success(),
        "givn timeline failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = root.path("report.html");
    let mut browser = BrowserDriver::start();
    browser.goto(&browser::file_url(&report));
    browser.capture_evidence(scenario);
    browser.quit();
}
