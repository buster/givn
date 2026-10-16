//! Step definitions for the `agent-activity-reporting` capability.
//!
//! The scenarios hand a reporter payload to the intake (`givn history
//! ingest`) and observe the record it produced; the harness drivers live in
//! [`crate::steps::development_timeline_support`].

use cucumber::{given, then, when};
use serde_json::{Value, json};

use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;

/// State for the agent-activity-reporting scenarios: the payload the last
/// intake invocation was fed, so a following step can assert on its events,
/// and the record entry the last reading step identified, so a following step
/// can assert on "that entry".
#[derive(Debug, Default)]
pub(crate) struct AgentActivityReportingState {
    /// The change the fixture's payloads report into.
    pub(crate) change: Option<String>,
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

/// The subagent span the fixture's agent reports: the harness's own identity
/// and the agent definition that ran it. The two reported timestamps are
/// exactly `REPORTED_SUBAGENT_SPAN_MS` apart (09:11:43.021Z → 09:12:44.321Z),
/// and the completion carries no duration of its own, so the intake measures
/// the span from the pair.
const REPORTED_SUBAGENT: &str = "Architect.Scout";
const REPORTED_SUBAGENT_AGENT: &str = "scout";
const REPORTED_SUBAGENT_START_AT: &str = "2026-10-16T09:11:43.021Z";
const REPORTED_SUBAGENT_END_AT: &str = "2026-10-16T09:12:44.321Z";
const REPORTED_SUBAGENT_SPAN_MS: u64 = 61_300;

/// The one completed tool call the fixture seeds as already-reported agent
/// activity, and (for the tool scenario's own report) the call the assertions
/// read.
fn tool_event() -> Value {
    json!({
        "kind": "tool",
        "at": REPORTED_TOOL_AT,
        "tool": REPORTED_TOOL,
        "label": REPORTED_TOOL_LABEL,
        "duration_ms": REPORTED_TOOL_MS,
        "outcome": "ok",
    })
}

/// The payload the last intake invocation was fed, as the fixture built it.
fn reported_payload(world: &GivnWorld) -> Value {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.last_payload.clone())
        .expect("no intake payload has been reported")
}

/// The change the fixture's payload reports into — the change the scenario's
/// `Given` declared, never a constant.
fn reported_change(world: &GivnWorld) -> String {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.change.clone())
        .expect("the fixture has not declared its change yet")
}

/// The fixture's reported events of `kind`, read from the payload itself: the
/// assertions compare the record against the payload's own values, never
/// against a constant.
fn reported_events(world: &GivnWorld, kind: &str) -> Vec<Value> {
    reported_payload(world)["events"]
        .as_array()
        .expect("the payload carries an events array")
        .iter()
        .filter(|event| event["kind"] == json!(kind))
        .cloned()
        .collect()
}

/// The fixture's one reported event of `kind`.
fn reported_event(world: &GivnWorld, kind: &str) -> Value {
    let events = reported_events(world, kind);
    assert_eq!(
        events.len(),
        1,
        "the fixture reports exactly one `{kind}` event: {}",
        reported_payload(world)
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

#[given(regex = r#"^a change "([^"]+)" with reported agent activity$"#)]
fn change_with_reported_agent_activity(world: &mut GivnWorld, id: String) {
    // This scenario's first Given, so it brings up the project root itself
    // when the scenario did not name one; then the same change fixture the
    // artifact Given builds, plus one completed tool call reported through the
    // real intake — the activity the record already holds when the scenario
    // reports its own event.
    if world.env.is_none() {
        world.env = Some(crate::common::make_init_root());
    }
    crate::steps::change_exists_with_artifact(world, id.clone(), "proposal.md".to_string());
    let payload = support::intake_payload(Some(&id), None, vec![tool_event()]);
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = support::report_intake(env, &payload);
    assert!(
        out.status.success(),
        "the intake failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .change = Some(id);
}

#[when("the agent reports a subagent that started and completed")]
fn reports_a_subagent_that_started_and_completed(world: &mut GivnWorld) {
    let change = reported_change(world);
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![
            json!({
                "kind": "subagent",
                "at": REPORTED_SUBAGENT_START_AT,
                "agent": REPORTED_SUBAGENT_AGENT,
                "subagent": REPORTED_SUBAGENT,
                "status": "started",
            }),
            json!({
                "kind": "subagent",
                "at": REPORTED_SUBAGENT_END_AT,
                "agent": REPORTED_SUBAGENT_AGENT,
                "subagent": REPORTED_SUBAGENT,
                "status": "completed",
            }),
        ],
    );
    report(world, payload);
}

#[when(regex = r#"^the agent reports a completed tool call for "([^"]+)"$"#)]
fn reports_a_completed_tool_call(world: &mut GivnWorld, id: String) {
    let payload = support::intake_payload(Some(&id), None, vec![tool_event()]);
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

/// The reported subagent start and its completion, read from the payload: the
/// two events the one recorded span is measured from.
fn reported_subagent_span(world: &GivnWorld) -> (Value, Value) {
    let events = reported_events(world, "subagent");
    assert_eq!(
        events.len(),
        2,
        "the fixture reports the subagent's start and its completion: {}",
        reported_payload(world)
    );
    let start = events
        .iter()
        .find(|event| event["status"] == json!("started"))
        .expect("the fixture reports the subagent's start")
        .clone();
    let completion = events
        .iter()
        .find(|event| event["status"] == json!("completed"))
        .expect("the fixture reports the subagent's completion")
        .clone();
    (start, completion)
}

#[then("the record holds one entry for that subagent")]
fn record_holds_one_entry_for_that_subagent(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let identity = reported_subagent_span(world).1["subagent"].clone();
    let entries = support::record_entries(&env.root());
    let recorded: Vec<&Value> = entries
        .iter()
        .filter(|entry| {
            entry["source"] == json!("agent")
                && entry["kind"] == json!("subagent")
                && entry["data"]["subagent"] == identity
        })
        .collect();
    assert_eq!(
        recorded.len(),
        1,
        "expected exactly one recorded span for the reported subagent; the record holds: {entries:#?}"
    );
    let entry = recorded[0].clone();
    assert_eq!(
        entry["scope"],
        json!({"kind": "change", "id": reported_change(world)}),
        "the reported subagent is not recorded for the change the payload named: {entry}"
    );
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_entry = Some(entry);
}

#[then("that entry names the subagent and spans its start and its end")]
fn entry_names_the_subagent_and_spans_its_start_and_end(world: &mut GivnWorld) {
    let (_start, completion) = reported_subagent_span(world);
    let entry = identified_entry(world);
    assert_eq!(
        entry["data"]["subagent"],
        json!(REPORTED_SUBAGENT),
        "the entry does not carry the subagent's own identity: {entry}"
    );
    assert_eq!(
        entry["name"],
        json!(REPORTED_SUBAGENT),
        "the entry does not name the subagent: {entry}"
    );
    assert_eq!(
        entry["data"]["agent"],
        json!(REPORTED_SUBAGENT_AGENT),
        "the entry does not name the agent definition that ran the subagent: {entry}"
    );
    // The completion carried no duration of its own, so the recorded duration
    // must be the span between the two reported events (09:11:43.021Z →
    // 09:12:44.321Z).
    assert_eq!(
        entry["duration_ms"].as_u64(),
        Some(REPORTED_SUBAGENT_SPAN_MS),
        "the recorded duration does not cover the reported start and end: {entry}"
    );
    assert_eq!(
        entry["at"], completion["at"],
        "the entry does not end at the reported completion: {entry}"
    );
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "a completed subagent is not recorded as ok: {entry}"
    );
}
