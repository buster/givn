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

/// The question wait the fixture's agent reports: the complete question text
/// the harness handed over (two lines, so the first-line rule discriminates)
/// and the wait duration the payload provides.
const REPORTED_QUESTION: &str = "Which store should the record use?\nDuckDB, SQLite, or the append-only JSONL — the design's storage rationale decides.";
const REPORTED_QUESTION_WAIT_MS: u64 = 480_000;
const REPORTED_QUESTION_AT: &str = "2026-10-16T09:13:45.501Z";

/// The completed turn the fixture's agent reports, with its token usage: the
/// index, the harness's own duration, the model, and the counts are the values
/// the record must carry.
const REPORTED_TURN: u64 = 12;
const REPORTED_TURN_MS: u64 = 41_230;
const REPORTED_TURN_AT: &str = "2026-10-16T09:14:46.221Z";
const REPORTED_MODEL: &str = "deepseek/deepseek-flash";

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

#[when("the agent reports a completed turn without token usage")]
fn reports_a_turn_without_token_usage(world: &mut GivnWorld) {
    let change = reported_change(world);
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![json!({
            "kind": "turn",
            "at": REPORTED_TURN_AT,
            "turn": REPORTED_TURN,
            "duration_ms": REPORTED_TURN_MS,
            "model": REPORTED_MODEL,
            "status": "completed",
        })],
    );
    report(world, payload);
}

#[when("the agent reports a completed turn with token usage")]
fn reports_a_turn_with_token_usage(world: &mut GivnWorld) {
    let change = reported_change(world);
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![json!({
            "kind": "turn",
            "at": REPORTED_TURN_AT,
            "turn": REPORTED_TURN,
            "duration_ms": REPORTED_TURN_MS,
            "model": REPORTED_MODEL,
            "status": "completed",
            "usage": {
                "input": 12_043,
                "output": 812,
                "cache_read": 9_000,
                "cache_write": 0,
                "total_tokens": 21_855,
            },
        })],
    );
    report(world, payload);
}

#[when("the agent reports that it asked a question and resumed")]
fn reports_a_question_wait(world: &mut GivnWorld) {
    let change = reported_change(world);
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![json!({
            "kind": "question",
            "at": REPORTED_QUESTION_AT,
            "label": REPORTED_QUESTION,
            "duration_ms": REPORTED_QUESTION_WAIT_MS,
            "status": "completed",
        })],
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

#[then("the record marks that turn's token usage as unavailable")]
fn record_marks_the_turns_usage_unavailable(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let turns: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("turn"))
        .collect();
    assert_eq!(
        turns.len(),
        1,
        "expected exactly one recorded turn; the record holds: {entries:#?}"
    );
    let entry = turns[0].clone();
    assert_eq!(
        entry["data"]["usage"],
        json!("unavailable"),
        "the turn's absent usage is not marked unavailable: {entry}"
    );
    // No zero-valued counts anywhere for it: no usage entry was fabricated, no
    // recorded `data` carries a zero count, and no model was invented.
    const COUNT_KEYS: [&str; 5] = [
        "input",
        "output",
        "cache_read",
        "cache_write",
        "total_tokens",
    ];
    let usages: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("usage"))
        .collect();
    assert!(
        usages.is_empty(),
        "a usage entry was fabricated for an unmeasured turn: {usages:#?}"
    );
    for recorded in &entries {
        for key in COUNT_KEYS {
            assert_ne!(
                recorded["data"][key],
                json!(0),
                "a zero-valued `{key}` count was recorded: {recorded}"
            );
        }
        assert!(
            recorded["data"].get("model").is_none(),
            "a model was recorded for an unmeasured turn: {recorded}"
        );
    }
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_entry = Some(entry);
}

#[then("the turn is still recorded")]
fn the_turn_is_still_recorded(world: &mut GivnWorld) {
    let reported = reported_event(world, "turn");
    let entry = identified_entry(world);
    assert_eq!(
        entry["kind"],
        json!("turn"),
        "the reported turn was not recorded: {entry}"
    );
    assert_eq!(
        entry["source"],
        json!("agent"),
        "the recorded turn is not an agent-reported entry: {entry}"
    );
    assert_eq!(
        entry["scope"],
        json!({"kind": "change", "id": reported_change(world)}),
        "the turn is not recorded for the change the payload named: {entry}"
    );
    assert_eq!(
        entry["at"], reported["at"],
        "the recorded turn is not the reported one: {entry}"
    );
    assert_eq!(
        entry["duration_ms"], reported["duration_ms"],
        "the turn's own measured duration was not kept: {entry}"
    );
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "a completed turn is not recorded as ok: {entry}"
    );
}

#[then("the record holds that turn's token usage")]
fn record_holds_the_turns_token_usage(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let reported = reported_event(world, "turn");
    let reported_usage = reported["usage"]
        .as_object()
        .expect("the fixture reports the turn's usage");
    let entries = support::record_entries(&env.root());
    let turns: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("turn"))
        .collect();
    assert_eq!(
        turns.len(),
        1,
        "expected exactly one recorded turn; the record holds: {entries:#?}"
    );
    let usages: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent") && entry["kind"] == json!("usage"))
        .collect();
    assert_eq!(
        usages.len(),
        1,
        "expected exactly one usage entry for the reported turn; the record holds: {entries:#?}"
    );
    let entry = usages[0].clone();
    assert_eq!(
        entry["scope"],
        json!({"kind": "change", "id": reported_change(world)}),
        "the reported usage is not recorded for the change the payload named: {entry}"
    );
    assert_eq!(
        entry["parent"], turns[0]["id"],
        "the usage entry is not parented to the turn that produced it: {entry}"
    );
    for (key, count) in reported_usage {
        assert_eq!(
            entry["data"][key], *count,
            "the usage entry does not carry the reported `{key}` count: {entry}"
        );
    }
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_entry = Some(entry);
}

#[then("the usage names the model that produced it")]
fn usage_names_the_model_that_produced_it(world: &mut GivnWorld) {
    let reported = reported_event(world, "turn");
    let entry = identified_entry(world);
    let model = entry["data"]["model"]
        .as_str()
        .unwrap_or_else(|| panic!("the usage entry names no model: {entry}"));
    assert_eq!(
        model,
        reported["model"]
            .as_str()
            .expect("the fixture reports the turn's model"),
        "the usage does not name the reported model: {entry}"
    );
    assert_eq!(
        model, REPORTED_MODEL,
        "the model is not kept verbatim, never normalised or truncated: {entry}"
    );
}

#[then("the record holds one entry for the question wait")]
fn record_holds_one_entry_for_the_question_wait(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let recorded: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent") && entry["kind"] == json!("question"))
        .collect();
    assert_eq!(
        recorded.len(),
        1,
        "expected exactly one entry for the reported question wait; the record holds: {entries:#?}"
    );
    let entry = recorded[0].clone();
    assert_eq!(
        entry["scope"],
        json!({"kind": "change", "id": reported_change(world)}),
        "the question wait is not recorded for the change the payload named: {entry}"
    );
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
        .last_entry = Some(entry);
}

#[then("that entry carries the wait duration")]
fn entry_carries_the_wait_duration(world: &mut GivnWorld) {
    let reported = reported_event(world, "question");
    let entry = identified_entry(world);
    assert_eq!(
        entry["duration_ms"], reported["duration_ms"],
        "the wait is not recorded with the reported duration: {entry}"
    );
    // The label is the question's first line only: the complete question text
    // is never stored (D2).
    let complete = reported["label"]
        .as_str()
        .expect("the fixture reports the complete question text");
    let first_line = complete
        .lines()
        .next()
        .expect("the fixture's question has a first line");
    let label = entry["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the question entry carries no label: {entry}"));
    assert_eq!(
        label, first_line,
        "the entry's label is not the question's first line: {entry}"
    );
    assert!(
        label.chars().count() <= 120,
        "the entry's label is not bounded to 120 characters: {entry}"
    );
    for line in complete.lines().skip(1) {
        assert!(
            !label.contains(line),
            "the record kept more than the question's first line: {entry}"
        );
    }
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "a resumed wait is not recorded as ok: {entry}"
    );
}
