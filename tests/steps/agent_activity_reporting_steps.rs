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
    /// The record bytes as they stood before the last payload was handed to
    /// the real intake, so a following step can assert the record's
    /// append-only property at the content level.
    pub(crate) store_before: Option<Vec<u8>>,
    /// The contract-violating event the last payload reported, with its 1-based
    /// position in the payload: the event the intake's rejection must name.
    pub(crate) violating_event: Option<(Value, usize)>,
    /// The last run of the generated Claude shim, as the driver observed it.
    pub(crate) harness_run: Option<HarnessRun>,
}

/// One run of the generated Claude shim: the wall time the driver's two hook
/// invocations spanned, so a duration assertion compares the record against the
/// run itself rather than a constant.
#[derive(Debug)]
pub(crate) struct HarnessRun {
    /// The driver's own measured span of the two invocations.
    gap_ms: u64,
}

/// The wait the driver injects between the permission request and the
/// completion: long enough that a reporter-measured duration is unmistakable
/// beside the harness's own 40 ms `duration_ms`.
const HARNESS_WAIT_GAP: std::time::Duration = std::time::Duration::from_millis(600);

/// The run the driver observed.
fn harness_run(world: &GivnWorld) -> &HarnessRun {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.harness_run.as_ref())
        .expect("the harness has not reported a permission request yet")
}

/// The capability's state, created on first use: every fixture and assertion
/// of this capability reaches the payload and the entry it last read through
/// it.
fn state(world: &mut GivnWorld) -> &mut AgentActivityReportingState {
    world
        .agent_activity_reporting
        .get_or_insert_with(AgentActivityReportingState::default)
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
    state(world).last_payload = Some(payload);
}

/// The change the fixture's recorded history built: the change the shared
/// `a change "…" with a recorded history` Given created and this capability's
/// payloads report into.
fn recorded_change(world: &GivnWorld) -> String {
    world
        .last_change_id
        .clone()
        .expect("a change with a recorded history is required first")
}

/// Snapshot the record as it stands before the next payload is handed over.
/// The append-only property is asserted on these bytes, so the "before" state
/// is read from the store itself, never re-derived from the payload.
fn snapshot_record(world: &mut GivnWorld) -> Vec<u8> {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let bytes = support::store_bytes(&env.root());
    state(world).store_before = Some(bytes.clone());
    bytes
}

/// The record bytes the last payload was handed against.
fn recorded_before(world: &GivnWorld) -> Vec<u8> {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("no record was read before the last payload")
}

/// The event the fixture reports that the intake contract does not know: a
/// `kind` no entry maps. The intake must reject exactly this event — named on
/// stderr — and leave the record it inherited untouched.
fn contract_violating_event() -> Value {
    json!({
        "kind": "teleport",
        "at": REPORTED_TOOL_AT,
        "label": "an event the intake contract does not know",
    })
}

/// The intake's rejection lines on stderr: what the reporter's author reads to
/// fix the payload.
fn rejection_lines(world: &GivnWorld) -> Vec<String> {
    String::from_utf8_lossy(&crate::steps::output(world).stderr)
        .lines()
        .filter(|line| line.contains("rejected"))
        .map(str::to_owned)
        .collect()
}

/// The fixture's violating event, with its 1-based position in the payload —
/// the two things the intake's rejection must name.
fn violating_event(world: &GivnWorld) -> (Value, usize) {
    world
        .agent_activity_reporting
        .as_ref()
        .and_then(|state| state.violating_event.clone())
        .expect("the fixture has not reported a contract-violating event yet")
}

/// The agent targets that receive a development reporter, with the project
/// path each target's reporter is installed at — the design's asset table.
const REPORTER_ASSETS: &[(&str, &str)] = &[
    ("omp", ".omp/hooks/pre/givn-timeline.js"),
    ("opencode", ".opencode/plugins/givn-timeline.js"),
    ("claude", ".claude/hooks/givn-timeline.sh"),
];

/// The Claude Code events the development reporter registers for: the events
/// its binary-side mapper reads, one settings entry each.
const CLAUDE_REPORTER_EVENTS: [&str; 9] = [
    "PreToolUse",
    "PostToolUse",
    "PostToolUseFailure",
    "PermissionRequest",
    "Notification",
    "SubagentStart",
    "SubagentStop",
    "Stop",
    "SessionEnd",
];

/// The command a Claude Code event registration runs.
const CLAUDE_REPORTER_COMMAND: &str = "${CLAUDE_PROJECT_DIR}/.claude/hooks/givn-timeline.sh";

/// Read one generated project file, asserting it exists and is not empty.
fn generated_file(root: &std::path::Path, relative: &str, subject: &str) -> String {
    let path = root.join(relative);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the generated {subject} {}: {error}", path.display()));
    assert!(
        !content.trim().is_empty(),
        "the generated {subject} {} is empty",
        path.display()
    );
    content
}

/// Assert the generated Claude settings register the shim for every event the
/// reporter's mapper reads, and for no other event: the registration set is
/// the reporter's own contract.
fn assert_claude_reporter_registration(root: &std::path::Path) {
    let settings = generated_file(root, ".claude/settings.local.json", "Claude settings");
    let settings: Value = serde_json::from_str(&settings)
        .unwrap_or_else(|error| panic!("the generated Claude settings are not JSON: {error}"));
    let hooks = settings["hooks"].as_object().unwrap_or_else(|| {
        panic!("the generated Claude settings carry no hooks object: {settings}")
    });
    let mut registered = Vec::new();
    for (event, entries) in hooks {
        let entries = entries
            .as_array()
            .unwrap_or_else(|| panic!("the Claude `{event}` hooks are not an array: {entries}"));
        for entry in entries {
            let commands = entry["hooks"]
                .as_array()
                .unwrap_or_else(|| panic!("a Claude `{event}` entry carries no handlers: {entry}"));
            for handler in commands {
                if handler["command"].as_str() == Some(CLAUDE_REPORTER_COMMAND) {
                    assert_eq!(
                        handler["type"], "command",
                        "the reporter's `{event}` registration is not a command hook: {entry}"
                    );
                    registered.push(event.clone());
                }
            }
        }
    }
    registered.sort();
    registered.dedup();
    let mut expected: Vec<String> = CLAUDE_REPORTER_EVENTS
        .iter()
        .map(|event| (*event).to_string())
        .collect();
    expected.sort();
    assert_eq!(
        registered, expected,
        "the generated Claude settings do not register the reporter for exactly its events"
    );
}

#[then(
    regex = r#"^the project's generated guidance for "([^"]+)" includes the development reporter$"#
)]
fn guidance_includes_the_development_reporter(world: &mut GivnWorld, target: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let root = env.root();
    // The generated project file is read from the project, and it is the
    // shipped reporter: the install copies the asset verbatim.
    let (source, install) = REPORTER_ASSETS
        .iter()
        .find(|(name, _)| *name == target)
        .unwrap_or_else(|| panic!("`{target}` is not a supported agent target"));
    let installed = generated_file(&root, install, "development reporter");
    let asset = match target.as_str() {
        "omp" => include_str!("../../assets/agent-integrations/omp/givn-timeline.js"),
        "opencode" => include_str!("../../assets/agent-integrations/opencode/givn-timeline.js"),
        "claude" => include_str!("../../assets/agent-integrations/claude/givn-timeline.sh"),
        other => unreachable!("unhandled reporter target `{other}`"),
    };
    assert_eq!(
        installed, asset,
        "the project's {source} reporter is not the shipped asset: {install}"
    );
    // The reporter delivers through the intake: the two supported harness
    // entry points are the payload path and the raw-payload shim.
    let delivery = match target.as_str() {
        "claude" => "givn history harness-hook --harness claude",
        _ => "givn history ingest",
    };
    assert!(
        installed.contains(delivery),
        "the project's {target} reporter does not deliver through `{delivery}`: {install}"
    );
    if target == "claude" {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(root.join(install))
                .expect("the generated shim")
                .permissions()
                .mode();
            assert!(
                mode & 0o111 != 0,
                "the generated Claude shim is not executable: {install}"
            );
        }
        assert_claude_reporter_registration(&root);
    }
}

#[then("no development reporter is installed for that project")]
fn no_development_reporter_is_installed(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let root = env.root();
    // The design's install paths, and anywhere else in the project: a reporter
    // installed under another path would still claim activity that can never
    // arrive.
    for (target, install) in REPORTER_ASSETS {
        assert!(
            !root.join(install).exists(),
            "the neutral `agents` target installed the {target} reporter at {install}"
        );
    }
    let mut reporters = Vec::new();
    collect_reporter_files(&root, &root, &mut reporters);
    assert!(
        reporters.is_empty(),
        "a development reporter is installed somewhere in the project: {reporters:#?}"
    );
    // No registration either: the settings file a Claude reporter would be
    // registered in must not name the shim.
    let settings = root.join(".claude/settings.local.json");
    if settings.exists() {
        let content = std::fs::read_to_string(&settings)
            .unwrap_or_else(|error| panic!("read {}: {error}", settings.display()));
        assert!(
            !content.contains("givn-timeline"),
            "the neutral `agents` target registered a development reporter: {}",
            settings.display()
        );
    }
}

/// Collect the development reporter files installed anywhere under `dir`, so
/// "no reporter is installed for that project" is asserted on the project tree
/// rather than on the four paths the design names.
fn collect_reporter_files(root: &std::path::Path, dir: &std::path::Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == ".git" {
            continue;
        }
        if path.is_dir() {
            collect_reporter_files(root, &path, found);
            continue;
        }
        if name.ends_with("givn-timeline.js") || name.ends_with("givn-timeline.sh") {
            found.push(
                path.strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
}

#[when("the agent reports an event that does not match the intake contract")]
fn reports_an_event_that_does_not_match_the_intake_contract(world: &mut GivnWorld) {
    let change = recorded_change(world);
    snapshot_record(world);
    let violating = contract_violating_event();
    // The fixture knows the event it reports as violating and where in the
    // payload it sits, so the assertion reads what was really sent rather than
    // re-deriving it from the intake's own behaviour.
    let position = 1;
    state(world).violating_event = Some((violating.clone(), position));
    let payload = support::intake_payload(Some(&change), None, vec![violating]);
    report(world, payload);
}

#[when("the agent reports a completed tool call")]
fn reports_a_completed_tool_call_for_the_recorded_change(world: &mut GivnWorld) {
    // The scenario's recorded history names the change; the reported call is the
    // fixture's one completed tool call, handed to the real intake.
    let change = recorded_change(world);
    let payload = support::intake_payload(Some(&change), None, vec![tool_event()]);
    report(world, payload);
}

#[then("the intake reports no failure to the agent")]
fn intake_reports_no_failure_to_the_agent(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = crate::steps::output(world);
    let stderr = crate::steps::stderr(world);
    // The reporter's caller must not fail because the store could not be
    // written: a non-zero exit is a hook error for Claude Code and exit 2
    // blocks the tool, so the intake always exits 0.
    assert_eq!(
        out.status.code(),
        Some(0),
        "the intake failed the agent action it reported (exit {:?}): {stderr}",
        out.status.code()
    );
    // The failure stays visible — as a warning, never as a failed command.
    let store = support::store_file(&env.root());
    let failures: Vec<&str> = stderr
        .lines()
        .filter(|line| line.contains("could not be written"))
        .collect();
    assert!(
        !failures.is_empty(),
        "the write failure was not reported at all: {stderr:?}"
    );
    assert!(
        failures.iter().all(|line| line.starts_with("warning:")),
        "the write failure is not reported as a warning: {failures:#?}"
    );
    assert!(
        failures
            .iter()
            .any(|line| line.contains(&store.display().to_string())),
        "the warning does not name the store it could not write ({failures:#?}): {store:?}"
    );
    assert!(
        !stderr.lines().any(|line| line.starts_with("error:")),
        "the intake reported the write failure as an error: {stderr:?}"
    );
    // Do not turn this into a retry or queue feature: there is no second write
    // location, so the blocked store path is still the blocking directory.
    assert!(
        store.is_dir(),
        "the record was written somewhere other than the blocked store path {}",
        store.display()
    );
}

/// The text the fixture pipes as a payload that is not readable JSON at all: a
/// reporter's flush cut off mid-line, so the intake must refuse the payload
/// itself rather than read events out of it.
const UNREADABLE_PAYLOAD: &str =
    "{\"schema\": 1, \"reporter\": \"fixture\", \"events\": [{\"kind\": \"tool\"";

/// Hand one raw stdin payload to the real intake and keep its output.
fn report_raw(world: &mut GivnWorld, text: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = support::report_intake_raw(env, text);
    world.last_output = Some(out);
    world.last_command = Some("history".to_string());
}

#[when("the agent pipes a payload that is not JSON into `givn history ingest`")]
fn pipes_a_payload_that_is_not_json(world: &mut GivnWorld) {
    // The record is read before the payload is handed over, so the Then steps
    // can assert what the failed read left behind.
    snapshot_record(world);
    report_raw(world, UNREADABLE_PAYLOAD);
}

#[then("the intake exits 2")]
fn intake_exits_2(world: &mut GivnWorld) {
    let out = crate::steps::output(world);
    assert_eq!(
        out.status.code(),
        Some(2),
        "the intake did not report the payload-level failure with its own exit code (exit {:?}): {}",
        out.status.code(),
        crate::steps::stderr(world)
    );
}

#[then("the development record gains no entry")]
fn record_gains_no_entry(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = recorded_before(world);
    let after = support::store_bytes(&env.root());
    // The failed read wrote nothing: no partial line and no new entry, so the
    // bytes are still exactly the snapshot's — which also means the intake's
    // own command entry was not recorded for a payload it refused.
    assert_eq!(
        after,
        before,
        "the failed payload read changed the record:\nbefore: {}\nafter: {}",
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&after)
    );
    // No lock left behind either: the history directory holds nothing the
    // failed read created.
    let leftovers: Vec<String> = std::fs::read_dir(env.path("givn/history"))
        .expect("the fixture's history directory")
        .map(|entry| {
            entry
                .expect("read a history entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".lock") || name.ends_with(".tmp"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "the failed read left a lock or a partial file behind: {leftovers:?}"
    );
}

/// The acceptable event of a mixed payload: a completed tool call the intake's
/// table maps — the event a rejected sibling must not take down with it. Its
/// tool name and label are distinct from the seed record's entries, so the
/// appended line is identified by the payload's own values.
fn acceptable_tool_event() -> Value {
    json!({
        "kind": "tool",
        "at": REPORTED_TOOL_AT,
        "tool": "grep",
        "label": "grep -rn development src",
        "duration_ms": 37,
        "outcome": "ok",
    })
}

#[when(
    "the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`"
)]
fn pipes_a_payload_with_one_rejected_and_one_acceptable_event(world: &mut GivnWorld) {
    snapshot_record(world);
    // The rejected event comes first, so the assertion proves a rejection early
    // in the batch does not take the rest of the payload down with it.
    let events = vec![contract_violating_event(), acceptable_tool_event()];
    let (violating, position) = (events[0].clone(), 1);
    state(world).violating_event = Some((violating, position));
    let payload = support::intake_payload(None, None, events);
    report(world, payload);
}

/// Assert the intake names the fixture's contract-violating event: exactly one
/// `warning:` line on stderr carrying the event's 1-based payload position and
/// its kind — legible enough for the reporter's author to fix the payload — and
/// no `error:` line, because a rejected event is never a failed payload.
fn assert_rejection_names_the_event(world: &GivnWorld) {
    let (rejected, position) = violating_event(world);
    let kind = rejected["kind"]
        .as_str()
        .expect("the fixture's violating event names its kind");
    let rejections = rejection_lines(world);
    assert_eq!(
        rejections.len(),
        1,
        "expected the payload's one contract-violating event to be named on stderr: {rejections:#?}"
    );
    let line = &rejections[0];
    assert!(
        line.starts_with("warning:"),
        "the rejection is not reported as a warning: {line}"
    );
    assert!(
        line.contains(&format!("event {position}")),
        "the rejection does not name the rejected event's position {position}: {line}"
    );
    assert!(
        line.contains(kind),
        "the rejection does not name the rejected event's kind `{kind}`: {line}"
    );
    let stderr = crate::steps::stderr(world);
    assert!(
        !stderr.lines().any(|line| line.starts_with("error:")),
        "the rejected event was reported as a failure of the payload: {stderr:?}"
    );
}

/// The record bytes appended since `before`, asserting the append-only property
/// at the content level: the bytes that were recorded before the payload are
/// still the store's first bytes, byte for byte.
fn appended_since<'a>(before: &[u8], after: &'a [u8], subject: &str) -> &'a [u8] {
    assert!(
        after.starts_with(before),
        "{subject} was rewritten: the store's earlier bytes are not a prefix of the new file:\nbefore: {}\nafter: {}",
        String::from_utf8_lossy(before),
        String::from_utf8_lossy(after)
    );
    &after[before.len()..]
}

#[then("the intake exits 0")]
fn intake_exits_0(world: &mut GivnWorld) {
    let out = crate::steps::output(world);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the intake did not read the payload (exit {:?}): {}",
        out.status.code(),
        crate::steps::stderr(world)
    );
}

#[then("stderr names the rejected event")]
fn stderr_names_the_rejected_event(world: &mut GivnWorld) {
    assert_rejection_names_the_event(world);
}

#[then("the development record holds the acceptable event")]
fn record_holds_the_acceptable_event(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = recorded_before(world);
    let after = support::store_bytes(&env.root());
    // The earlier bytes are still the store's first bytes: the append is an
    // append, never a rewrite.
    let appended = support::entries_of(appended_since(&before, &after, "the store"));
    let reported: Vec<&Value> = appended
        .iter()
        .filter(|entry| entry["source"] == json!("agent"))
        .collect();
    assert_eq!(
        reported.len(),
        1,
        "expected exactly the acceptable event in the appended region: {appended:#?}"
    );
    let entry = reported[0];
    // The appended entry is the acceptable event the payload reported, by the
    // payload's own values.
    let acceptable = reported_event(world, "tool");
    assert_eq!(
        entry["kind"], acceptable["kind"],
        "the appended entry is not the acceptable event's kind: {entry}"
    );
    assert_eq!(
        entry["name"], acceptable["tool"],
        "the appended entry does not name the acceptable event's tool: {entry}"
    );
    assert_eq!(
        entry["label"], acceptable["label"],
        "the appended entry does not keep the acceptable event's label: {entry}"
    );
    // The rejected sibling was never recorded, in the appended region or
    // anywhere else: the record holds exactly one agent-sourced entry, and it
    // is the acceptable event.
    let entries = support::record_entries(&env.root());
    let agent_entries: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent"))
        .collect();
    assert_eq!(
        agent_entries.len(),
        1,
        "the rejected event was recorded beside the acceptable one: {agent_entries:#?}"
    );
    assert_eq!(
        agent_entries[0]["kind"], acceptable["kind"],
        "the recorded agent entry is not the acceptable event: {agent_entries:#?}"
    );
}

#[then("the intake names the rejected event")]
fn intake_names_the_rejected_event(world: &mut GivnWorld) {
    assert_rejection_names_the_event(world);
}

#[then("the previously recorded history is unchanged")]
fn previously_recorded_history_is_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = recorded_before(world);
    let after = support::store_bytes(&env.root());
    // The append-only property at the content level: the bytes that were
    // recorded before the rejected payload are still the store's first bytes,
    // byte for byte.
    let appended = support::entries_of(appended_since(&before, &after, "the recorded history"));
    // What the payload appended is the intake's own command entry, never the
    // rejected event: nothing agent-reported entered the record.
    for entry in &appended {
        assert_eq!(
            entry["source"],
            json!("givn"),
            "the rejected event was recorded: {entry}"
        );
    }
    let entries = support::record_entries(&env.root());
    let reported: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent"))
        .collect();
    assert!(
        reported.is_empty(),
        "the rejected payload contributed an agent entry: {reported:#?}"
    );
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
    state(world).change = Some(id);
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
    state(world).last_entry = Some(entry);
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
    state(world).last_entry = Some(entry);
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
    state(world).last_entry = Some(entry);
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
    state(world).last_entry = Some(entry);
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
    state(world).last_entry = Some(entry);
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

#[given("a project with the generated Claude Code development reporter")]
fn project_with_generated_claude_reporter(world: &mut GivnWorld) {
    // The reporter is installed by the real `givn init --agents claude`, never
    // a hand-written file: the scenario drives what the generator produced.
    let env = crate::common::make_bare_root();
    support::initialize_with_claude_reporter(&env);
    world.env = Some(env);
}

#[when("the harness reports a permission request and then completion of that same tool call")]
fn reports_a_permission_request_and_completion(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let started = std::time::Instant::now();
    let request = support::report_harness_hook(env, &support::permission_request_payload());
    assert!(
        request.status.success(),
        "the permission request hook failed (exit {:?}): {}",
        request.status.code(),
        String::from_utf8_lossy(&request.stderr)
    );
    // The wait the harness cannot report: the reporter's own stamps are the
    // only measurement, so the driver really waits between the two hooks.
    std::thread::sleep(HARNESS_WAIT_GAP);
    let completion = support::report_harness_hook(env, &support::tool_completion_payload());
    let gap_ms = started.elapsed().as_millis() as u64;
    assert!(
        completion.status.success(),
        "the completion hook failed (exit {:?}): {}",
        completion.status.code(),
        String::from_utf8_lossy(&completion.stderr)
    );
    world.last_output = Some(completion);
    state(world).harness_run = Some(HarnessRun { gap_ms });
}

#[then("the record holds one wait entry for the permission request")]
fn record_holds_one_wait_entry_for_the_permission_request(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let waits: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent") && entry["kind"] == json!("question"))
        .collect();
    assert_eq!(
        waits.len(),
        1,
        "expected exactly the permission wait in the record; the record holds: {entries:#?}"
    );
    let entry = waits[0].clone();
    let label = entry["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the wait entry carries no label: {entry}"));
    assert!(
        label.contains("permission request"),
        "the wait entry does not name the permission request: {entry}"
    );
    assert!(
        label.contains(support::WAIT_TOOL),
        "the wait entry does not name the tool it waited for: {entry}"
    );
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "a wait whose tool call completed is not recorded as ok: {entry}"
    );
    state(world).last_entry = Some(entry);
}

#[then("that entry carries a duration measured by the reporter itself")]
fn entry_carries_a_reporter_measured_duration(world: &mut GivnWorld) {
    let entry = identified_entry(world);
    let run_gap_ms = harness_run(world).gap_ms;
    let duration = entry["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the wait entry carries no measured duration: {entry}"));
    assert!(
        duration > 0,
        "the measured wait is not a positive duration: {entry}"
    );
    // The harness's own duration for that call excludes time spent in the
    // permission prompt, so the recorded wait is never that value.
    assert_ne!(
        duration,
        support::HARNESS_TOOL_MS,
        "the wait reuses the harness's excluding `duration_ms`: {entry}"
    );
    // Consistent with the driver's own run: the two shim invocations spanned
    // `run_gap_ms`, the reporter stamped the first and measured at the second.
    assert!(
        duration * 2 >= run_gap_ms,
        "the recorded wait ({duration} ms) does not cover the gap the driver ran ({run_gap_ms} ms): {entry}"
    );
    assert!(
        duration <= run_gap_ms + 1_000,
        "the recorded wait ({duration} ms) overshoots the gap the driver ran ({run_gap_ms} ms): {entry}"
    );
    // The tool entry the same completion reported keeps the harness's own
    // duration: the wait's measurement is the wait's alone.
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let tools: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["source"] == json!("agent") && entry["kind"] == json!("tool"))
        .collect();
    assert_eq!(
        tools.len(),
        1,
        "expected exactly the completed tool call in the record; the record holds: {entries:#?}"
    );
    assert_eq!(
        tools[0]["duration_ms"],
        json!(support::HARNESS_TOOL_MS),
        "the tool entry did not keep the harness's own duration: {}",
        tools[0]
    );
}
