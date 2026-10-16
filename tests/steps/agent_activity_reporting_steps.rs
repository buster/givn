//! Step definitions for the `agent-activity-reporting` capability.
//!
//! The scenarios hand a reporter payload to the intake (`givn history
//! ingest`) and observe the record it produced; the harness drivers live in
//! [`crate::steps::development_timeline_support`].

use cucumber::{then, when};
use serde_json::{Value, json};

use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;

/// State for the agent-activity-reporting scenarios: the payload the last
/// intake invocation was fed, so a following step can assert on its events,
/// and the record entry the last reading step identified, so a following step
/// can assert on "that entry".
#[derive(Debug, Default)]
pub(crate) struct AgentActivityReportingState {
    pub(crate) last_payload: Option<Value>,
    pub(crate) last_entry: Option<Value>,
}

/// The tool call the fixture's agent reports: the harness tool name, the
/// bounded subject label, the harness's own measured duration, and the tool
/// event's own timestamp — a fixed RFC 3339 UTC instant, so the assertions
/// read the payload's values, never the clock.
const REPORTED_TOOL: &str = "bash";
const REPORTED_TOOL_LABEL: &str = "cargo test --locked --test features_runner";
const REPORTED_TOOL_MS: u64 = 1_240;
const REPORTED_TOOL_AT: &str = "2026-10-16T09:12:44.321Z";

/// The payload the last intake invocation was fed, as the fixture built it.
fn reported_payload(world: &GivnWorld) -> Value {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.last_payload.clone())
        .expect("no intake payload has been reported")
}

/// The fixture's one reported event of `kind`, read from the payload itself:
/// the assertions compare the record against the payload's own values, never
/// against a constant.
fn reported_event(world: &GivnWorld, kind: &str) -> Value {
    let payload = reported_payload(world);
    let events: Vec<&Value> = payload["events"]
        .as_array()
        .expect("the payload carries an events array")
        .iter()
        .filter(|event| event["kind"] == json!(kind))
        .collect();
    assert_eq!(
        events.len(),
        1,
        "the fixture reports exactly one `{kind}` event: {payload}"
    );
    events[0].clone()
}

/// The record entry the previous reading step identified.
fn identified_entry(world: &GivnWorld) -> Value {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no record entry was identified by the previous step")
}

/// Hand one payload to the real intake and keep it: the following steps assert
/// the record against the payload's own values, never against a constant.
fn report(world: &mut GivnWorld, payload: Value) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = support::report_intake(env, &payload);
    world.last_output = Some(out);
    world.last_command = Some("history".to_string());
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_payload = Some(payload);
}

#[when(regex = r#"^the agent reports a completed tool call for "([^"]+)"$"#)]
fn reports_a_completed_tool_call(world: &mut GivnWorld, id: String) {
    let payload = support::intake_payload(
        Some(&id),
        None,
        vec![json!({
            "kind": "tool",
            "at": REPORTED_TOOL_AT,
            "tool": REPORTED_TOOL,
            "label": REPORTED_TOOL_LABEL,
            "duration_ms": REPORTED_TOOL_MS,
            "outcome": "ok",
        })],
    );
    report(world, payload);
}

#[then("the development record holds one agent entry for that tool call")]
fn record_holds_one_agent_entry_for_tool_call(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let recorded: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent") && entry["kind"] == json!("tool"))
        .collect();
    assert_eq!(
        recorded.len(),
        1,
        "expected exactly one agent-sourced `tool` entry; the record holds: {entries:#?}"
    );
    let entry = recorded[0].clone();
    assert_eq!(
        entry["scope"],
        json!({"kind": "change", "id": reported_payload(world)["change"]}),
        "the reported tool call is not recorded for the change the payload named: {entry}"
    );
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_entry = Some(entry);
}

#[then("that entry names the tool and its measured duration")]
fn entry_names_the_tool_and_its_measured_duration(world: &mut GivnWorld) {
    let reported = reported_event(world, "tool");
    let entry = identified_entry(world);
    assert_eq!(
        entry["name"], reported["tool"],
        "the entry does not name the reported tool: {entry}"
    );
    assert_eq!(
        entry["duration_ms"], reported["duration_ms"],
        "the entry does not carry the reported measured duration: {entry}"
    );
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "a completed tool call is not recorded as ok: {entry}"
    );
    assert_eq!(
        entry["label"], reported["label"],
        "the entry does not keep the reported label: {entry}"
    );
}
