//! Shared fixtures for the `inspect-development-history` capabilities.
//!
//! Owns the record fixtures, the seeded-history builder, and the three
//! harness drivers, so the per-capability step files bind steps while this
//! module owns how a scenario reaches the record:
//!
//! - the record fixtures read `givn/history/events.jsonl` the way a consumer
//!   does (parse the JSONL, never the recorder's in-memory state) through the
//!   existing black-box seam `common::run_givn` (the compiled binary in a
//!   `tempfile` project root);
//! - the seeded-history builder writes a fixture record with known entries;
//! - the harness drivers are a fake `pi` extension context for omp, a fake
//!   plugin context capturing `session.hook` / `tool.hook` / `shell.hook` plus
//!   the event subscription for opencode, and a payload-on-stdin invocation of
//!   the generated Claude shim with a fixture transcript.

use std::path::{Path, PathBuf};

use givn::history::{HistoryEntry, HistoryStore, Scope};
use serde_json::Value;

use crate::common;

/// The verification command the Task 2 fixture declares: a real subprocess that
/// sleeps, so the recorded duration is a real measurement of a real run —
/// never a mocked clock.
pub const SLOW_VERIFY_COMMAND: &str = "sleep 2";

/// Declare `command` as the fixture project's `verify.command`, which is what
/// `givn check tasks --change <id>` runs through the shell choke point.
pub fn declare_verify_command(env: &common::TestEnv, command: &str) {
    let yaml = format!("verify:\n  command: \"{command}\"\n");
    common::write_commands_yaml(&env.path("givn/commands.yaml"), &yaml);
}

/// The project's own name as the project's own state derives it: the name of
/// its root directory. A scenario reads it from its fixture root, never from a
/// constant.
pub fn project_name(env: &common::TestEnv) -> String {
    env.root()
        .file_name()
        .and_then(|name| name.to_str())
        .expect("the fixture project root has a name")
        .to_string()
}

/// Create one change through its own recorded command: `givn new <id>` is the
/// command during which the change directory and its `proposal.md` appear, so
/// the dispatcher's post-command artifact observation is exercised through the
/// real binary instead of a handwritten record.
pub fn create_change_during_a_recorded_command(
    env: &common::TestEnv,
    id: &str,
) -> std::process::Output {
    common::run_givn(env, &["new", id])
}

/// The record's artifact transitions for one change: the `phase` entries whose
/// label is `label`, scoped to that change. The gate phase keeps the artifact
/// id label, so it never matches a transition filename here.
pub fn transitions_for<'a>(entries: &'a [Value], change: &str, label: &str) -> Vec<&'a Value> {
    entries
        .iter()
        .filter(|entry| entry["kind"].as_str() == Some("phase"))
        .filter(|entry| entry["scope"]["kind"].as_str() == Some("change"))
        .filter(|entry| entry["scope"]["id"].as_str() == Some(change))
        .filter(|entry| entry["label"].as_str() == Some(label))
        .collect()
}

/// Assert the fixture project has no change a recorder could bind to: no
/// directory under `givn/changes/` (every directory there is a change for
/// `list_changes`).
pub fn assert_no_active_change(env: &common::TestEnv) {
    let changes = read_dir_names(&env.path("givn/changes"));
    assert!(
        changes.is_empty(),
        "the fixture project has changes: {changes:?}"
    );
}

/// Assert the fixture project is bare in the recorder's sense: no change and
/// no ideation session, so the project fallback is exercised rather than
/// assumed.
pub fn assert_bare_project(env: &common::TestEnv) {
    assert_no_active_change(env);
    for topic in read_dir_names(&env.path("givn/ideation")) {
        let session = env.path(&format!("givn/ideation/{topic}/session.md"));
        assert!(
            !session.exists(),
            "the fixture project has an ideation session: {}",
            session.display()
        );
    }
}

/// Build `givn/ideation/<topic>/session.md` as a written-but-unfinished
/// session: the phase and progress lines the frame step writes, and no
/// `## Completion` section (the section the handoff step appends). The builder
/// is the fixture's, so Task 28 can report the same topic before any change
/// exists.
pub fn declare_unfinished_ideation_topic(env: &common::TestEnv, topic: &str) {
    let session = env.path(&format!("givn/ideation/{topic}/session.md"));
    let dir = session
        .parent()
        .expect("the session path has a directory")
        .to_path_buf();
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|error| panic!("create the topic {}: {error}", dir.display()));
    let body = "\
# Ideation session: timeline-ideas

## Phase

frame

## Progress

- frame written
- questions answered

## Next steps

- elaborate the use cases
";
    std::fs::write(&session, body)
        .unwrap_or_else(|error| panic!("write the session {}: {error}", session.display()));
}

/// The steps an ideation topic's record holds: the work the topic did while it
/// was elaborated — before any change directory exists — as the record's own
/// `step` entries.
const IDEATION_STEPS: [(&str, &str, u64); 3] = [
    ("ideation", "givn instructions ideation", 20_000),
    ("eventstorming", "givn instructions eventstorming", 600_000),
    ("elaboration", "givn instructions usecase", 120_000),
];

/// The recorded end each of [`IDEATION_STEPS`] carries, in the same order.
const IDEATION_STEP_AT: [&str; 3] = [
    "2026-10-16T08:00:00.000Z",
    "2026-10-16T08:30:00.000Z",
    "2026-10-16T09:00:00.000Z",
];

/// Seed the fixture project's record with `topic`'s own steps, under the topic
/// scope Task 7 records: the session the frame step wrote (no `## Completion`
/// section), and one measured step entry per action the topic's own work ran.
/// The fixture is the support file's rather than a scenario's, so the browser
/// tier's topic row reports the same topic through the same action. The
/// project must hold no change — a topic's work exists before any change
/// directory does.
pub fn seed_ideation_topic(env: &common::TestEnv, topic: &str) -> Vec<HistoryEntry> {
    declare_unfinished_ideation_topic(env, topic);
    let scope = Scope::Ideation {
        id: topic.to_string(),
    };
    let mut history = SeededHistory::new();
    for (index, (name, label, duration_ms)) in IDEATION_STEPS.into_iter().enumerate() {
        let entry = history.span(&scope, "step", name, IDEATION_STEP_AT[index], duration_ms);
        history.label(entry, label);
    }
    history.seed(env)
}

/// The steps the out-of-order fixture's record holds, in the order the store
/// holds them: the second entry's recorded end is earlier than the first's —
/// exactly what a late intake flush produces — so the store's line order and the
/// record's own time order are different things.
const OUT_OF_ORDER_STEPS: [(&str, &str, &str, u64); 3] = [
    (
        "verification",
        "cargo test --locked --test features_runner",
        "2026-10-16T10:00:00.000Z",
        60_000,
    ),
    (
        "lint",
        "cargo clippy --locked --all-targets -- -D warnings",
        "2026-10-16T09:00:00.000Z",
        120_000,
    ),
    (
        "coverage",
        "cargo llvm-cov nextest --locked --test features_runner",
        "2026-10-16T11:00:00.000Z",
        300_000,
    ),
];

/// Seed the fixture project's record with steps whose file order contradicts
/// their recorded time order, under `scope`. A report that reused the store's
/// line order (or the entry ids, which are minted at append time and therefore
/// file-ordered too) cannot order these entries correctly. Returns the written
/// entries.
pub fn seed_out_of_order_steps(env: &common::TestEnv, scope: &Scope) -> Vec<HistoryEntry> {
    let mut history = SeededHistory::new();
    for (name, label, at, duration_ms) in OUT_OF_ORDER_STEPS {
        let entry = history.span(scope, "step", name, at, duration_ms);
        history.label(entry, label);
    }
    history.seed(env)
}

/// The change workflow's artifact transitions, in the order Givn records them:
/// one `phase` entry per artifact file whose appearance marks a stage's work as
/// done — and therefore opens the stage that follows it. The report scenarios
/// seed the record's transitions from this list, up to the artifact the
/// scenario names.
pub const WORKFLOW_ARTIFACTS: [&str; 7] = [
    "proposal.md",
    "specs",
    "design.md",
    "arc42.md",
    "design-review.md",
    "tasks.md",
    "review.md",
];

/// The recorded end of each transition in [`WORKFLOW_ARTIFACTS`], one hour
/// apart: a report's phase names stay readable at the initial zoom, and a
/// scenario's own steps fall between two transitions.
const TRANSITION_AT: [&str; 7] = [
    "2026-10-16T08:00:00.000Z",
    "2026-10-16T09:00:00.000Z",
    "2026-10-16T10:00:00.000Z",
    "2026-10-16T11:00:00.000Z",
    "2026-10-16T12:00:00.000Z",
    "2026-10-16T13:00:00.000Z",
    "2026-10-16T14:00:00.000Z",
];

/// The work the fixture's record holds: one measured step per workflow stage,
/// recorded after the transition that opened the stage and before the next one.
/// The first row is the opening stage's own work (recorded before the record's
/// first transition); the last row is the step the CLI report scenario's
/// assertion names — recorded after the record's last transition — and it is
/// also the record's longest measured step, so the browser scenarios select it
/// as the longest recorded step.
const STAGE_WORK: [(&str, &str, u64); 7] = [
    (
        "givn instructions propose",
        "2026-10-16T07:50:00.000Z",
        300_000,
    ),
    (
        "givn instructions specs",
        "2026-10-16T08:30:00.000Z",
        600_000,
    ),
    (
        "givn instructions design",
        "2026-10-16T09:30:00.000Z",
        900_000,
    ),
    (
        "givn instructions arc42",
        "2026-10-16T10:30:00.000Z",
        120_000,
    ),
    (
        "givn skills design-review",
        "2026-10-16T11:30:00.000Z",
        600_000,
    ),
    (
        "givn instructions tasks",
        "2026-10-16T12:30:00.000Z",
        300_000,
    ),
    (
        "cargo test --locked --test features_runner",
        "2026-10-16T13:30:00.000Z",
        1_800_000,
    ),
];

/// The record one report scenario renders: the written steps and the artifact
/// transitions the scenario's own assertions name.
#[derive(Debug, Clone)]
pub struct SeededReportHistory {
    /// The step recorded after the record's last transition: the one the
    /// `implement` phase's assertion names.
    pub post_transition_step: HistoryEntry,
    /// The artifact transitions the record carries, in the order they appeared.
    pub transitions: Vec<String>,
}

/// Seed the fixture project's record with a change's recorded history: one
/// artifact transition per workflow artifact up to `last_artifact`, and one
/// measured step per stage those transitions opened. The change itself is
/// created with the real `givn new`, the transitions are written as the
/// record's own `phase` entries — the shape the report's phase derivation reads
/// — and every entry goes into the store through the store's own append path.
pub fn seed_report_history(
    env: &common::TestEnv,
    change: &str,
    last_artifact: &str,
) -> SeededReportHistory {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let last = WORKFLOW_ARTIFACTS
        .iter()
        .position(|artifact| *artifact == last_artifact)
        .unwrap_or_else(|| {
            panic!("the workflow records no artifact `{last_artifact}`: {WORKFLOW_ARTIFACTS:?}")
        });
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    let mut transitions = Vec::new();
    for (index, artifact) in WORKFLOW_ARTIFACTS[..=last].iter().enumerate() {
        let transition = history.point(&scope, "phase", artifact, TRANSITION_AT[index]);
        history.label(transition, artifact);
        transitions.push(artifact.to_string());
    }
    let work = last + 1;
    assert!(
        work < STAGE_WORK.len(),
        "the fixture records no step for the stage `{last_artifact}` opens"
    );
    let mut steps = Vec::new();
    for (label, at, duration_ms) in STAGE_WORK[..=work].iter() {
        let step = history.span(&scope, "step", label, at, *duration_ms);
        steps.push(history.label(step, label));
    }
    let entries = history.seed(env);
    SeededReportHistory {
        post_transition_step: entries[steps[work]].clone(),
        transitions,
    }
}

/// The Task 25b fixture: the record a captured-metadata, full-content report
/// renders — the metadata tool entry, the ids of the content entries that fill
/// the budget, and the store's bytes as seeded.
pub struct SeededDetailsHistory {
    /// The tool entry carrying every captured metadata fact the panel names,
    /// plus a small content bundle.
    pub metadata_entry: HistoryEntry,
    /// The entries whose content bundles fill the 8 MiB report budget.
    pub content_entry_ids: Vec<u64>,
    /// The store's bytes as seeded, so the report run is shown not to change a
    /// single content byte.
    pub store_bytes: Vec<u8>,
}

/// The metadata the fixture's tool entry carries: every fact the report model
/// and the details panel must name.
fn captured_metadata() -> Value {
    serde_json::json!({
        "title": "Development report details",
        "status": "needs_input",
        "stop_reason": "end_turn",
        "model": "deepseek/deepseek-flash",
        "cost": 0.42,
        "list_cost": 0.55,
        "context_size": 48_000,
        "context_window": 128_000,
        "tool_call_id": "call-25b",
        "output_bytes": 4_096,
        "error": "the command failed: exit status 101",
        "agent": "omp",
        "attempt": 2,
        "max_attempts": 3,
        "delay_ms": 1_500,
        "fallback_from": "model-a",
        "fallback_to": "model-b",
        "permission_mode": "acceptEdits",
        "content": {
            "tool_input": {
                "text": "cargo test --locked",
                "truncated": false,
                "bytes": 19
            },
            "tool_output": {
                "text": "test result: ok.",
                "truncated": false,
                "bytes": 16
            }
        }
    })
}

/// One entry's content bundle at the documented per-entry cap: four fields,
/// each at the per-field cap, all filled with `marker`.
fn full_content_bundle(marker: char) -> Value {
    let text: String = std::iter::repeat_n(marker, givn::history::CONTENT_FIELD_LIMIT).collect();
    let field = serde_json::json!({
        "text": text,
        "truncated": false,
        "bytes": text.len()
    });
    serde_json::json!({
        "tool_input": field.clone(),
        "tool_output": field.clone(),
        "assistant_text": field.clone(),
        "thinking_text": field
    })
}

/// Seed the fixture's record: a metadata-rich tool entry (nested in a turn, so
/// its parent fact is real) and 34 content entries whose bundles together
/// exceed the report document's 8 MiB budget.
pub fn seed_captured_metadata_and_content(
    env: &common::TestEnv,
    change: &str,
) -> SeededDetailsHistory {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    let turn = history.span(
        &scope,
        "turn",
        "turn 3",
        "2026-10-16T09:00:00.000Z",
        600_000,
    );
    let metadata = history.span(
        &scope,
        "tool",
        "Bash",
        "2026-10-16T09:10:00.000Z",
        2_400_000,
    );
    history.label(metadata, "cargo test --locked");
    history.session(metadata, "session-25b");
    history.data(metadata, captured_metadata());
    history.nested_in(metadata, turn);
    let mut content_indices = Vec::new();
    for index in 0..34usize {
        let at = format!("2026-10-16T10:{index:02}:00.000Z");
        let entry = history.span(&scope, "tool", "Bash", &at, 5_000);
        history.label(entry, &format!("content call {index}"));
        let marker = (b'a' + (index % 26) as u8) as char;
        history.data(
            entry,
            serde_json::json!({ "content": full_content_bundle(marker) }),
        );
        content_indices.push(entry);
    }
    let entries = history.seed(env);
    let content_entry_ids = content_indices
        .iter()
        .map(|index| entries[*index].id)
        .collect();
    SeededDetailsHistory {
        metadata_entry: entries[metadata].clone(),
        content_entry_ids,
        store_bytes: std::fs::read(store_file(&env.root()))
            .unwrap_or_else(|error| panic!("read the seeded store: {error}")),
    }
}

/// Append one tool entry to the fixture's record: the longest recorded step,
/// carrying a tool call id and a small bounded content field. The content text
/// carries markup, so the panel's escaping is observable.
pub fn seed_report_detail_tool(env: &common::TestEnv, change: &str) -> HistoryEntry {
    let mut entry = HistoryEntry::reported(
        Scope::Change {
            id: change.to_string(),
        },
        "tool",
        "Bash",
        "2026-10-16T14:30:00.000Z",
    );
    entry.label = Some("cargo test --locked".to_string());
    entry.duration_ms = Some(2_400_000);
    entry.outcome = "ok".to_string();
    entry.session = Some("e7-session".to_string());
    entry.data = Some(serde_json::json!({
        "tool_call_id": "call-e7",
        "output_bytes": 16,
        "content": {
            "tool_input": {
                "text": "<b>cargo test --locked</b>",
                "truncated": false,
                "bytes": 25
            }
        }
    }));
    let store = HistoryStore::at(store_file(&env.root()));
    store
        .append(std::slice::from_mut(&mut entry))
        .unwrap_or_else(|error| panic!("append the E7 detail tool entry: {error}"));
    entry
}

/// The names of a directory's entries, empty when the directory does not
/// exist.
fn read_dir_names(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// ─── The Claude Code development-reporter driver ───────────────────────────

/// The `PATH` entry the driver adds so the generated shim's `givn` is this
/// build: a one-line script executing the compiled binary.
const SHIM_BIN_DIR: &str = ".givn-test-bin";

/// The session the Claude driver's payloads report under: one session id, so
/// the pending state the first payload opens is the state the second consumes.
pub const HARNESS_SESSION: &str = "claude-timeline-fixture";

/// The tool call the wait fixture reports: one Bash call whose permission
/// prompt the harness reports and whose completion closes the wait.
pub const WAIT_TOOL: &str = "Bash";

/// The harness's own `duration_ms` for that call. It excludes time spent in
/// the permission prompt, so the recorded wait must not reuse it.
pub const HARNESS_TOOL_MS: u64 = 40;

/// The tool input the fixture reports: a real command subject, so the tool
/// entry's label is the bounding path's own business.
const WAIT_COMMAND: &str = "cargo test --locked --test features_runner";

/// Bring up a project whose omp development reporter was generated by the real
/// managed sync (`givn skills targets add omp`) — never a hand-written file.
pub fn initialize_with_omp_reporter(env: &common::TestEnv) {
    let out = common::run_givn(env, &["skills", "targets", "add", "omp"]);
    assert!(
        out.status.success(),
        "givn skills targets add omp failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        env.path(".omp/hooks/pre/givn-timeline.js").is_file(),
        "the managed sync did not install the omp development reporter"
    );
}

/// Bring up a project whose Claude Code development reporter was generated by
/// the real `givn init --agents claude` — never a hand-written file.
pub fn initialize_with_claude_reporter(env: &common::TestEnv) {
    let out = common::run_givn(env, &["init", "--agents", "claude"]);
    assert!(
        out.status.success(),
        "givn init --agents claude failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        env.path(".claude/hooks/givn-timeline.sh").is_file(),
        "givn init did not install the Claude Code development reporter"
    );
}

/// The permission request the harness reports: no timestamp and no duration,
/// only the tool and its input — the payload the reporter must stamp itself.
pub fn permission_request_payload() -> Value {
    serde_json::json!({
        "hook_event_name": "PermissionRequest",
        "session_id": HARNESS_SESSION,
        "tool_name": WAIT_TOOL,
        "tool_input": {"command": WAIT_COMMAND},
    })
}

/// The completion of that same tool call: the harness's own `duration_ms`,
/// which excludes the permission prompt's time.
pub fn tool_completion_payload() -> Value {
    serde_json::json!({
        "hook_event_name": "PostToolUse",
        "session_id": HARNESS_SESSION,
        "tool_name": WAIT_TOOL,
        "tool_use_id": "toolu_givn_fixture",
        "tool_input": {"command": WAIT_COMMAND},
        "duration_ms": HARNESS_TOOL_MS,
        "tool_response": {},
    })
}

/// The turn's end: the harness names the session record it wrote
/// asynchronously beside the session id.
pub fn stop_payload(transcript: &Path) -> Value {
    serde_json::json!({
        "hook_event_name": "Stop",
        "session_id": HARNESS_SESSION,
        "transcript_path": transcript.display().to_string(),
    })
}

/// The `PATH` entry that makes a generated reporter's `givn` this build: a
/// one-line script executing the compiled binary, in the fixture's own
/// `.givn-test-bin` directory. Every harness driver prepends it to `PATH`, so
/// the reporter's real intake invocation (`givn history ingest`, or the shim's
/// `givn history harness-hook`) reaches the binary under test.
pub fn shim_givn_on_path(env: &common::TestEnv) -> PathBuf {
    let bin = env.path(SHIM_BIN_DIR);
    std::fs::create_dir_all(&bin).unwrap_or_else(|error| {
        panic!("create the PATH shim directory {}: {error}", bin.display())
    });
    let givn = bin.join("givn");
    std::fs::write(
        &givn,
        format!("#!/bin/sh\nexec '{}' \"$@\"\n", env!("CARGO_BIN_EXE_givn")),
    )
    .unwrap_or_else(|error| panic!("write the PATH shim {}: {error}", givn.display()));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&givn)
            .expect("the PATH shim's metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&givn, permissions).expect("make the PATH shim executable");
    }
    bin
}

/// `PATH` with the given directories in front of the process's own, as every
/// reporter process is started: the `givn` shim first (so a reporter's own
/// `givn` is this build) and the harness's own bin directory where the harness
/// is not resolved from `PATH` (the pinned Maki binary).
fn driver_path(dirs: &[&Path]) -> String {
    let mut parts: Vec<String> = dirs.iter().map(|dir| dir.display().to_string()).collect();
    if let Some(existing) = std::env::var_os("PATH") {
        parts.push(existing.to_string_lossy().into_owned());
    }
    parts.join(":")
}

/// Run the generated Claude shim with one real hook payload on stdin, through
/// its real entry point: the shim is executed directly and its `givn` is
/// `PATH`-shimmed to the compiled binary, so the payload reaches the
/// binary-side mapper exactly as it would in a live session.
pub fn report_harness_hook(env: &common::TestEnv, payload: &Value) -> std::process::Output {
    let shim = env.path(".claude/hooks/givn-timeline.sh");
    assert!(
        shim.is_file(),
        "the generated Claude shim is missing: {}",
        shim.display()
    );
    let bin = shim_givn_on_path(env);
    let mut child = std::process::Command::new(&shim)
        .current_dir(env.root())
        .env("PATH", driver_path(&[&bin]))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("run the generated shim {}: {error}", shim.display()));
    use std::io::Write as _;
    child
        .stdin
        .as_mut()
        .expect("the generated shim's stdin")
        .write_all(payload.to_string().as_bytes())
        .expect("write the hook payload");
    child
        .wait_with_output()
        .unwrap_or_else(|error| panic!("wait for the generated shim: {error}"))
}

/// The fixture session record one Claude Code scenario reads its token usage
/// from, with the values the record must then carry.
#[derive(Debug)]
pub struct TranscriptFixture {
    /// The transcript file the `Stop` payload names.
    pub path: PathBuf,
    /// The usage the record must carry, in the record's own `data` keys.
    pub counts: Value,
    /// The model that produced the usage.
    pub model: String,
}

/// Write a fixture transcript holding one turn's assistant usage in the
/// harness's own shape: a user prompt entry, then the assistant entry whose
/// `message.usage` and `message.model` the reader must find.
pub fn write_transcript_with_usage(env: &common::TestEnv, model: &str) -> TranscriptFixture {
    let path = env.path("transcript.jsonl");
    let input = 12_043u64;
    let output = 812u64;
    let cache_read = 9_000u64;
    let cache_write = 240u64;
    let text = format!(
        "{}\n{}\n",
        serde_json::json!({
            "type": "user",
            "sessionId": HARNESS_SESSION,
            "message": {
                "role": "user",
                "content": [{"type": "text", "text": "Implement the change"}],
            },
        }),
        serde_json::json!({
            "type": "assistant",
            "sessionId": HARNESS_SESSION,
            "message": {
                "role": "assistant",
                "model": model,
                "usage": {
                    "input_tokens": input,
                    "output_tokens": output,
                    "cache_read_input_tokens": cache_read,
                    "cache_creation_input_tokens": cache_write,
                    "output_tokens_details": {"thinking_tokens": 160},
                },
            },
        }),
    );
    std::fs::write(&path, text)
        .unwrap_or_else(|error| panic!("write the fixture transcript {}: {error}", path.display()));
    TranscriptFixture {
        path,
        counts: serde_json::json!({
            "input": input,
            "output": output,
            "cache_read": cache_read,
            "cache_write": cache_write,
            "total_tokens": input + output + cache_read + cache_write,
        }),
        model: model.to_string(),
    }
}

/// The number of concurrent rounds the two-writer fixture races: on every
/// round two `givn` invocations record into the same store, and all rounds are
/// released at the same moment, so the overlap is real rather than lucky.
pub const RACING_ROUNDS: usize = 6;

/// The verification command the two-writer fixture declares: it holds every
/// racing process inside its `givn check tasks` run until the fixture releases
/// them together, so their appends really overlap. A busy loop, so the release
/// is noticed within microseconds by every process — the failure the fixture
/// creates is a real `EISDIR`-free, real-process interleaving, never a mocked
/// clock and never a simulated failure.
pub const RACE_BARRIER_COMMAND: &str = "until [ -e .givn-race-go ]; do :; done";

/// The marker that releases the barrier.
const RACE_MARKER: &str = ".givn-race-go";

/// How long the racing processes get to reach the barrier before the fixture
/// releases it. A process that arrives late appends later instead of racing —
/// the assertions still hold, the race is only weaker.
const RACE_ARMED: std::time::Duration = std::time::Duration::from_millis(300);

/// Start the two-writer race: `RACING_ROUNDS` rounds of two concurrent
/// recording commands, all held at the barrier and then released together, so
/// every round's appends overlap. Returns the processes; the caller waits for
/// them (the scenario's `both commands finish` step).
pub fn start_racing_commands(env: &common::TestEnv) -> Vec<std::process::Child> {
    declare_verify_command(env, RACE_BARRIER_COMMAND);
    let marker = env.path(RACE_MARKER);
    let _ = std::fs::remove_file(&marker);
    let children: Vec<std::process::Child> = (0..RACING_ROUNDS * 2)
        .map(|_| spawn_recording_command(env))
        .collect();
    std::thread::sleep(RACE_ARMED);
    std::fs::write(&marker, b"").expect("release the racing commands");
    children
}

/// Wait for the racing commands, assert every one of them finished
/// successfully (a failed racer would record a different shape than this
/// scenario claims to read), and clear the barrier marker.
pub fn finish_racing_commands(env: &common::TestEnv, children: Vec<std::process::Child>) {
    for (index, child) in children.into_iter().enumerate() {
        let out = child
            .wait_with_output()
            .expect("wait for a racing givn process");
        assert!(
            out.status.success(),
            "racing command {index} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let _ = std::fs::remove_file(env.path(RACE_MARKER));
}

/// One recording `givn` invocation in the fixture project: `givn check tasks
/// --change demo` writes the command entry plus its two children (the
/// verification step and the mandatory-checks phase), so the store the racers
/// append to also carries parent links to reconcile.
fn spawn_recording_command(env: &common::TestEnv) -> std::process::Child {
    std::process::Command::new(env!("CARGO_BIN_EXE_givn"))
        .args(["check", "tasks", "--change", "demo"])
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn a racing givn process")
}

/// Remove the fixture's development record, so the project really holds no
/// recorded activity for any scope. The fixture's claim "no recorded activity"
/// is then the record's own state — the setup commands the fixture ran left
/// entries behind, and a report about a scope that recorded nothing must not
/// see them.
pub fn clear_record(env: &common::TestEnv) {
    let path = store_file(&env.root());
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("clear the store {}: {error}", path.display()),
    }
}

/// The development record file of the project rooted at `root`:
/// `givn/history/events.jsonl`, the store's own path.
pub fn store_file(root: &Path) -> PathBuf {
    root.join("givn/history/events.jsonl")
}

/// Make the record unwritable inside `env`: a directory where the store's file
/// belongs makes the append fail deterministically (`EISDIR`) without
/// depending on file permissions — a `chmod` fixture is unreliable when the
/// suite runs as root. The mechanism is the fixture's: the production code is
/// never asked to simulate a failure. The fixture project recorded its own
/// setup commands, so the existing store file is removed first.
pub fn make_record_unwritable(env: &common::TestEnv) {
    let path = store_file(&env.root());
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("clear the store {}: {error}", path.display()),
    }
    std::fs::create_dir_all(&path)
        .unwrap_or_else(|error| panic!("create the unwritable store {}: {error}", path.display()));
}

/// One intake payload as a reporter flushes it: the schema version, the
/// reporter's own name, its session id, the optional scope overrides, and the
/// events the reporter buffered for this flush.
pub fn intake_payload(change: Option<&str>, topic: Option<&str>, events: Vec<Value>) -> Value {
    serde_json::json!({
        "schema": 1,
        "reporter": "fixture",
        "session": "fixture-session",
        "change": change,
        "topic": topic,
        "events": events,
    })
}

/// Hand one payload to the real intake through its machine contract: the
/// compiled binary's `history ingest` reading the payload from stdin. The
/// output is returned, never asserted here — the scenario's own Then steps
/// read the record the intake wrote.
pub fn report_intake(env: &common::TestEnv, payload: &Value) -> std::process::Output {
    report_intake_with_args(env, payload, &[])
}

/// Hand one payload to the real intake with extra argv (the per-invocation
/// `--with-full-content` override). The output is returned, never asserted
/// here.
pub fn report_intake_with_args(
    env: &common::TestEnv,
    payload: &Value,
    args: &[&str],
) -> std::process::Output {
    let mut argv = vec!["history", "ingest"];
    argv.extend_from_slice(args);
    common::run_givn_with_stdin_and_envs(env, &argv, &payload.to_string(), &[])
}

/// Hand one raw stdin payload to the real intake: the payload-level case,
/// where the text is not one JSON object at all. The output is returned, never
/// asserted here — the scenario's own Then steps read the exit code and the
/// record the intake left behind.
pub fn report_intake_raw(env: &common::TestEnv, payload: &str) -> std::process::Output {
    common::run_givn_with_stdin_and_envs(env, &["history", "ingest"], payload, &[])
}

/// The development record of the project rooted at `root`, read the way a
/// consumer reads it: `givn/history/events.jsonl`, one JSON object per line.
/// Every line must parse as a JSON object — that is the store's own contract,
/// so a malformed line fails the scenario instead of being skipped, and a
/// missing store fails it too (the scenario ran a command that records).
pub fn record_entries(root: &Path) -> Vec<Value> {
    entries_of(&store_bytes(root))
}

/// The store's raw bytes. "Append-only" and "never rewritten" are claims about
/// these bytes, so the fixture reads the file itself instead of a
/// re-serialized view of its entries.
pub fn store_bytes(root: &Path) -> Vec<u8> {
    let path = store_file(root);
    std::fs::read(&path)
        .unwrap_or_else(|error| panic!("read the development record {}: {error}", path.display()))
}

/// The entries of a record region: one JSON object per line, every line
/// required to parse. Callers hand in a byte range of the store — the whole
/// store, or the bytes appended after a snapshot — so the append-only claims
/// are asserted on the bytes themselves.
pub fn entries_of(bytes: &[u8]) -> Vec<Value> {
    let text = std::str::from_utf8(bytes)
        .unwrap_or_else(|error| panic!("the development record is not UTF-8: {error}"));
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .unwrap_or_else(|error| panic!("store line is not a JSON object: {error}\n{line}"))
        })
        .collect()
}

// ─── The seeded-history builder ────────────────────────────────────────────

/// The seeded-history builder: one fixture record, entry by entry. A scenario
/// that reports a record it built itself seeds it here, so the report's
/// assertions derive from the fixture the scenario's Given claims rather than
/// from constants.
///
/// The entries go into the store through the store's own append path
/// ([`HistoryStore::append_linked`]), so every `id` is minted and every parent
/// link resolved exactly as production mints them — a seeded record is shaped
/// like a recorded one, never hand-written JSON. The builder clears whatever
/// the fixture's own setup commands left behind: the record a seeded scenario
/// reads is the fixture's own known record.
#[derive(Debug, Default)]
pub struct SeededHistory {
    entries: Vec<HistoryEntry>,
    parents: Vec<Option<usize>>,
}

impl SeededHistory {
    /// A builder for a record with no entries yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// One measured span: the entry kind, its name, its recorded end
    /// timestamp, and the duration a fixture measured. Returns the entry's
    /// index in the seeded batch, which the setters and [`Self::nested_in`]
    /// address.
    pub fn span(
        &mut self,
        scope: &Scope,
        kind: &str,
        name: &str,
        at: &str,
        duration_ms: u64,
    ) -> usize {
        let mut entry = seeded_entry(scope, kind, name, at);
        entry.duration_ms = Some(duration_ms);
        self.push(entry)
    }

    /// One point entry: the kind records an instant (a reported usage, a
    /// minted revision, an activity the record does not time), so it carries
    /// no duration.
    pub fn point(&mut self, scope: &Scope, kind: &str, name: &str, at: &str) -> usize {
        self.push(seeded_entry(scope, kind, name, at))
    }

    /// The bounded subject label of the entry at `index`.
    pub fn label(&mut self, index: usize, label: &str) -> usize {
        self.entry(index).label = Some(label.to_string());
        index
    }

    /// The entry's `data`, where the record carries a non-generic value.
    pub fn data(&mut self, index: usize, data: Value) -> usize {
        self.entry(index).data = Some(data);
        index
    }

    /// The harness session the entry belongs to (the envelope's `session`).
    pub fn session(&mut self, index: usize, session: &str) -> usize {
        self.entry(index).session = Some(session.to_string());
        index
    }

    /// The entry's outcome (`ok`, `failed`, `skipped`, or `unknown`).
    pub fn outcome(&mut self, index: usize, outcome: &str) -> usize {
        self.entry(index).outcome = outcome.to_string();
        index
    }

    /// Nest the entry at `index` inside the entry at `parent` (an earlier
    /// index of this same batch), the link the report nests spans by.
    pub fn nested_in(&mut self, index: usize, parent: usize) -> usize {
        assert!(
            parent < index,
            "a parent is only ever an entry an earlier append minted: {parent} >= {index}"
        );
        self.parents[index] = Some(parent);
        index
    }

    /// Write the record: clear the fixture's store and append these entries
    /// through the store's own append path. Returns the written entries — ids
    /// minted, parent links resolved — so the scenario asserts on the record it
    /// seeded.
    pub fn seed(mut self, env: &common::TestEnv) -> Vec<HistoryEntry> {
        let path = store_file(&env.root());
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("clear the record {}: {error}", path.display()),
        }
        let store = HistoryStore::at(path);
        store
            .append_linked(&mut self.entries, &self.parents)
            .unwrap_or_else(|error| panic!("seed the development record: {error}"));
        self.entries
    }

    /// The entry at `index`, for the setters.
    fn entry(&mut self, index: usize) -> &mut HistoryEntry {
        self.entries
            .get_mut(index)
            .unwrap_or_else(|| panic!("the seeded record has no entry {index}"))
    }

    /// Append one entry with no parent link yet.
    fn push(&mut self, entry: HistoryEntry) -> usize {
        self.entries.push(entry);
        self.parents.push(None);
        self.entries.len() - 1
    }
}

/// One fixture entry as the record's own shape: a recorded (`givn`) entry with
/// the fixture's timestamp, its kind and name, `ok` until a setter says
/// otherwise, and no id — the store mints it at append time.
fn seeded_entry(scope: &Scope, kind: &str, name: &str, at: &str) -> HistoryEntry {
    HistoryEntry {
        schema: givn::history::SCHEMA,
        id: 0,
        at: at.to_string(),
        scope: scope.clone(),
        source: "givn".to_string(),
        kind: kind.to_string(),
        name: name.to_string(),
        label: None,
        duration_ms: None,
        outcome: "ok".to_string(),
        parent: None,
        session: None,
        data: None,
    }
}

// ─── The omp and opencode development-reporter drivers ─────────────────────

/// One activity a driver fires at a *generated* reporter through that
/// harness's own events: the synthetic session the agent-activity-reporting
/// scenarios report, expressed once so the omp and the opencode driver consume
/// the same vocabulary instead of two hand-rolled event streams.
#[derive(Debug, Clone)]
pub enum HarnessActivity {
    /// One completed tool call.
    Tool {
        /// The harness's own call id (the key both reporters pair a call by).
        id: String,
        /// The tool the harness names.
        tool: String,
        /// The call's input, in the harness's own shape.
        args: Value,
    },
    /// One question wait: the question tool's call, from its start to its
    /// completion.
    Question {
        /// The harness's own call id.
        id: String,
        /// The question text the wait's label is bounded from.
        question: String,
    },
    /// One subagent, from its start to its completion.
    Subagent {
        /// The subagent's own identity.
        id: String,
        /// The agent definition that ran it.
        agent: String,
    },
    /// One completed turn, with the harness's own usage where it reports one.
    Turn {
        /// The turn's own index.
        index: u64,
        /// The model that produced the turn.
        model: String,
        /// The usage the harness reports for the turn, in its own shape.
        usage: Option<Value>,
    },
    /// One completed turn with the default-capture metadata a driven reporter
    /// reports: the harness's stop reason, its cost, and its context reading.
    TurnMetadata {
        /// The turn's own index.
        index: u64,
        /// The model that produced the turn.
        model: String,
        /// The harness's own stop reason (a raw word).
        stop_reason: String,
        /// The cost the harness reports (real and list price where present).
        cost: Value,
        /// The context size the harness measured.
        context_size: u64,
        /// The model's context window.
        context_window: u64,
        /// The usage the harness reports for the turn.
        usage: Option<Value>,
    },
    /// One failed tool call with the harness's own error text and output size.
    FailedTool {
        /// The harness's own call id.
        id: String,
        /// The tool the harness names.
        tool: String,
        /// The call's input, in the harness's own shape.
        args: Value,
        /// The harness's own error text (a stale reporter may send it unbounded).
        error: String,
        /// The output's size in bytes.
        output_bytes: u64,
    },
    /// The session's end: the reporter materializes whatever never closed and
    /// flushes.
    Shutdown,
}

impl HarnessActivity {
    /// The activity as the driver scripts consume it.
    fn as_json(&self) -> Value {
        match self {
            HarnessActivity::Tool { id, tool, args } => serde_json::json!({
                "kind": "tool",
                "id": id,
                "tool": tool,
                "args": args,
            }),
            HarnessActivity::Question { id, question } => serde_json::json!({
                "kind": "question",
                "id": id,
                "question": question,
            }),
            HarnessActivity::Subagent { id, agent } => serde_json::json!({
                "kind": "subagent",
                "id": id,
                "agent": agent,
            }),
            HarnessActivity::Turn {
                index,
                model,
                usage,
            } => serde_json::json!({
                "kind": "turn",
                "index": index,
                "model": model,
                "usage": usage,
            }),
            HarnessActivity::TurnMetadata {
                index,
                model,
                stop_reason,
                cost,
                context_size,
                context_window,
                usage,
            } => serde_json::json!({
                "kind": "turn_metadata",
                "index": index,
                "model": model,
                "stop_reason": stop_reason,
                "cost": cost,
                "context_size": context_size,
                "context_window": context_window,
                "usage": usage,
            }),
            HarnessActivity::FailedTool {
                id,
                tool,
                args,
                error,
                output_bytes,
            } => serde_json::json!({
                "kind": "failed_tool",
                "id": id,
                "tool": tool,
                "args": args,
                "error": error,
                "output_bytes": output_bytes,
            }),
            HarnessActivity::Shutdown => serde_json::json!({ "kind": "shutdown" }),
        }
    }
}

/// The session id every driver reports under: one session per driver run, so
/// the record's intake sees one reporter session.
pub const DRIVER_SESSION: &str = "givn-timeline-driver";

/// Drive the project's *generated* omp reporter through its real entry point:
/// `node` imports the generated `.omp/hooks/pre/givn-timeline.js`, calls its
/// factory with a fake `pi` extension context (`on`, `events.on`, `exec`,
/// `getThinkingLevel`), and fires `activities` at the reporter's own harness
/// events. The reporter's real `givn history ingest` reaches the compiled
/// binary through the `PATH` shim.
pub fn drive_omp_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let reporter = env.path(".omp/hooks/pre/givn-timeline.js");
    assert!(
        reporter.is_file(),
        "the generated omp reporter is missing: {}",
        reporter.display()
    );
    run_node_driver(
        env,
        "invoke-omp-reporter.mjs",
        OMP_DRIVER,
        &reporter,
        activities,
    )
}

/// Drive the project's *generated* opencode reporter through its real entry
/// point: `node` imports the generated `.opencode/plugins/givn-timeline.js`,
/// calls `setup(ctx)` with a fake `ctx` that captures
/// `session.hook`/`tool.hook`/`shell.hook` registrations plus an
/// `event.subscribe` async iterable, and emits the synthetic events the
/// activities name through that same stream. The reporter's real
/// `givn history ingest` reaches the compiled binary through the `PATH` shim.
pub fn drive_opencode_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let reporter = env.path(".opencode/plugins/givn-timeline.js");
    assert!(
        reporter.is_file(),
        "the generated opencode reporter is missing: {}",
        reporter.display()
    );
    run_node_driver(
        env,
        "invoke-opencode-reporter.mjs",
        OPENCODE_DRIVER,
        &reporter,
        activities,
    )
}

/// Drive the project's *generated* Claude Code development reporter through its
/// real entry point: the generated `.claude/hooks/givn-timeline.sh` shim, run
/// once per harness event with a real hook payload on stdin and the `givn`
/// `PATH` shim in front (`report_harness_hook`). The shim execs `givn history
/// harness-hook --harness claude`, so the payload reaches the binary's own
/// mapper exactly as it does in a live session.
///
/// Every invocation is asserted to have exited `0` here — a non-zero hook exit
/// is a hook error for the harness — and the last invocation's output is
/// returned, so the scenario's own assertion reads what the reporter process
/// produced. A `Turn` activity writes the harness's session record the shim's
/// mapper reads the turn's usage from (`write_transcript_with_usage`): Claude
/// Code reports a turn's usage through that record, never inline.
pub fn drive_claude_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let shim = env.path(".claude/hooks/givn-timeline.sh");
    assert!(
        shim.is_file(),
        "the generated Claude reporter is missing: {}",
        shim.display()
    );
    let mut last = None;
    for activity in activities {
        for payload in claude_payloads(env, activity) {
            let event = payload["hook_event_name"]
                .as_str()
                .unwrap_or("unknown event")
                .to_string();
            let out = report_harness_hook(env, &payload);
            assert!(
                out.status.success(),
                "the generated Claude reporter failed on `{event}` (exit {:?}): {}{}",
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
            last = Some(out);
        }
    }
    last.expect("the driver reported no harness event")
}

/// The Claude Code hook payloads one activity reaches the generated shim
/// through: the harness's own event names and field shapes, in the order a live
/// session sends them. `Turn` names the session record the harness writes
/// asynchronously (written here for the model the activity reports) and
/// `Shutdown` is the harness's session end; the activity's inline `usage` is
/// the omp/opencode shape and belongs to those drivers, so the Claude path
/// reports through its record instead.
fn claude_payloads(env: &common::TestEnv, activity: &HarnessActivity) -> Vec<Value> {
    let tool_call = |event: &str, tool: &str, id: &str, input: Value| {
        serde_json::json!({
            "hook_event_name": event,
            "session_id": DRIVER_SESSION,
            "tool_name": tool,
            "tool_use_id": id,
            "tool_input": input,
        })
    };
    match activity {
        HarnessActivity::Tool { id, tool, args } => {
            let mut completion = tool_call("PostToolUse", tool, id, args.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["tool_response"] = serde_json::json!({});
            vec![tool_call("PreToolUse", tool, id, args.clone()), completion]
        }
        HarnessActivity::Question { id, question } => {
            let input = serde_json::json!({ "questions": [{ "question": question }] });
            let mut completion = tool_call("PostToolUse", CLAUDE_QUESTION_TOOL, id, input.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["tool_response"] = serde_json::json!({});
            vec![
                tool_call("PreToolUse", CLAUDE_QUESTION_TOOL, id, input),
                completion,
            ]
        }
        HarnessActivity::Subagent { id, agent } => vec![
            serde_json::json!({
                "hook_event_name": "SubagentStart",
                "session_id": DRIVER_SESSION,
                "agent_id": id,
                "agent_type": agent,
            }),
            serde_json::json!({
                "hook_event_name": "SubagentStop",
                "session_id": DRIVER_SESSION,
                "agent_id": id,
                "agent_type": agent,
            }),
        ],
        HarnessActivity::Turn { model, .. } => {
            let transcript = write_transcript_with_usage(env, model);
            vec![serde_json::json!({
                "hook_event_name": "Stop",
                "session_id": DRIVER_SESSION,
                "transcript_path": transcript.path.display().to_string(),
            })]
        }
        HarnessActivity::TurnMetadata { model, cost, .. } => {
            let transcript = write_transcript_with_usage(env, model);
            if let Some(total) = cost.as_f64() {
                let mut text = std::fs::read_to_string(&transcript.path)
                    .unwrap_or_else(|error| panic!("read the transcript fixture: {error}"));
                text.push_str(
                    &serde_json::json!({
                        "type": "cost-state",
                        "totalCostUSD": total,
                    })
                    .to_string(),
                );
                text.push('\n');
                std::fs::write(&transcript.path, text)
                    .expect("append the cost-state rollup to the transcript fixture");
            }
            vec![serde_json::json!({
                "hook_event_name": "Stop",
                "session_id": DRIVER_SESSION,
                "transcript_path": transcript.path.display().to_string(),
            })]
        }
        HarnessActivity::FailedTool {
            id,
            tool,
            args,
            error,
            output_bytes,
        } => {
            let mut completion = tool_call("PostToolUseFailure", tool, id, args.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["error"] = Value::String(error.clone());
            completion["tool_response"] = Value::String("x".repeat(*output_bytes as usize));
            vec![tool_call("PreToolUse", tool, id, args.clone()), completion]
        }
        HarnessActivity::Shutdown => vec![serde_json::json!({
            "hook_event_name": "SessionEnd",
            "session_id": DRIVER_SESSION,
        })],
    }
}

/// The Claude Code question tool: its own `PreToolUse`/`PostToolUse` pair is
/// the question wait the binary's mapper records.
const CLAUDE_QUESTION_TOOL: &str = "AskUserQuestion";

/// The harness's own `duration_ms` for a completed tool call: Claude Code times
/// its tools, and its completion payload is where the number arrives.
const CLAUDE_TOOL_MS: u64 = 40;

/// The Maki binary the Maki scenarios run: the setup script's local bin
/// directory (`GIVN_E2E_BIN`, defaulting to `~/.cache/givn-e2e/bin`), then
/// `PATH`. A missing binary fails the fixture, never skips it.
fn maki_binary() -> PathBuf {
    if let Some(dir) = std::env::var_os("GIVN_E2E_BIN") {
        let candidate = PathBuf::from(dir).join("maki");
        if candidate.is_file() {
            return candidate;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let candidate = PathBuf::from(home).join(".cache/givn-e2e/bin/maki");
        if candidate.is_file() {
            return candidate;
        }
    }
    PathBuf::from("maki")
}

/// Bring up a project whose Maki development reporter was generated by the
/// real `givn init --agents maki` — never a hand-written file.
pub fn initialize_with_maki_reporter(env: &common::TestEnv) {
    let out = common::run_givn(env, &["init", "--agents", "maki"]);
    assert!(
        out.status.success(),
        "givn init --agents maki failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        env.path(".maki/lua/givn-timeline.lua").is_file(),
        "givn init did not install the Maki development reporter"
    );
}

/// One scripted Maki autocmd: the harness's own event name and the `data` the
/// driver passes it.
pub type MakiEvent = (&'static str, Value);

/// One JSON value as a Lua literal: the driver script is generated Lua, so the
/// scripted events travel inside the script (the reporter's grants deliberately
/// exclude `env`, and the binary is driven without any model call).
fn lua_literal(value: &Value) -> String {
    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => format!("{text:?}"),
        Value::Array(items) => format!(
            "{{ {} }}",
            items.iter().map(lua_literal).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(map) => format!(
            "{{ {} }}",
            map.iter()
                .map(|(key, value)| format!(
                    "[{}] = {}",
                    lua_literal(&Value::String(key.clone())),
                    lua_literal(value)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// The driver body appended to the generated `.maki/init.lua`: the shipped
/// fence lines stay the file's first statements (verbatim, so the fence itself
/// is what loads and `setup()`s the generated reporter), then the driver writes
/// the load marker, fires each scripted event with
/// `maki.api.exec_autocmds`, and flushes through the reporter's exported
/// `run()`. A small busy-wait after every event gives the paired stamps a real
/// gap, because Maki's events carry no timestamp of their own. Maki's Lua
/// sandbox has no `dofile`, so the fence is executed as part of the file rather
/// than loaded from a copy.
fn maki_driver_script(events: &[MakiEvent]) -> String {
    let mut scripts = String::new();
    for (event, data) in events {
        scripts.push_str(&format!("  {{ {event:?}, {} }},\n", lua_literal(data)));
    }
    format!(
        r#"-- Givn timeline test driver: the shipped fence above ran verbatim, then the
-- scripted events are fired through the harness's own dispatch.
local marker = maki.fs.write(".maki-driver-loaded", "loaded\n")
if not marker then
  error("the driver's load marker could not be written")
end
local events = {{
{scripts}}}
for _, event in ipairs(events) do
  local fired, fire_err = pcall(function()
    maki.api.exec_autocmds(event[1], {{ data = event[2] }})
  end)
  if not fired then
    maki.log.error("givn-timeline driver: " .. event[1] .. " failed: " .. tostring(fire_err))
  end
  -- No host event carries a timestamp: give the paired stamps a real gap.
  local until_t = os.clock() + 0.02
  while os.clock() < until_t do end
end
local reporter = require("givn-timeline")
local flushed, flush_err = pcall(reporter.run)
if not flushed then
  maki.log.error("givn-timeline driver: the flush failed: " .. tostring(flush_err))
end
"#
    )
}

/// Run the project's *generated* Maki reporter through its real entry point:
/// the real `maki` binary (`--trust prompt`, no model call, no network) loads a
/// test-only `.maki/init.lua` that executes the shipped fence, fires the
/// scripted autocmds, and flushes; the reporter's real `givn history ingest`
/// reaches the compiled binary through the `PATH` shim. Called once per
/// scenario action, so each action's events really cross the harness dispatch.
pub fn run_maki_driver(env: &common::TestEnv, events: &[MakiEvent]) -> std::process::Output {
    assert_maki_reporter(env);
    run_maki_init(env, &maki_driver_script(events))
}

/// The generated Maki reporter and fence must exist before any driver runs: a
/// driver against a hand-written file would not be the generated reporter.
fn assert_maki_reporter(env: &common::TestEnv) {
    let reporter = env.path(".maki/lua/givn-timeline.lua");
    assert!(
        reporter.is_file(),
        "the generated Maki reporter is missing: {}",
        reporter.display()
    );
    let fence = env.path(".maki/init.lua");
    assert!(
        fence.is_file(),
        "the generated Maki fence is missing: {}",
        fence.display()
    );
}

/// Run the project's generated `.maki/init.lua` with `driver` appended: the
/// real `maki` binary (`--trust prompt`, no model call, no network), the
/// project root as cwd, and the `givn` shim on PATH. The driver's load marker
/// proves the real harness loaded the project plugin.
fn run_maki_init(env: &common::TestEnv, driver: &str) -> std::process::Output {
    // The driver file starts with the shipped fence verbatim, so the fence text
    // itself is what loads the reporter in the real harness; the generated file
    // is restored by the next `givn init`/`givn skills sync`.
    let fence = env.path(".maki/init.lua");
    let generated = std::fs::read_to_string(&fence)
        .unwrap_or_else(|error| panic!("read the generated Maki fence: {error}"));
    std::fs::write(&fence, format!("{generated}\n{driver}"))
        .expect("write the Maki driver init.lua");
    let marker = env.path(".maki-driver-loaded");
    let _ = std::fs::remove_file(&marker);
    let maki = maki_binary();
    let shim = shim_givn_on_path(env);
    let mut path_parts: Vec<String> = Vec::new();
    if let Some(parent) = maki
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        path_parts.push(parent.display().to_string());
    }
    path_parts.push(shim.display().to_string());
    if let Some(existing) = std::env::var_os("PATH") {
        path_parts.push(existing.to_string_lossy().into_owned());
    }
    let output = std::process::Command::new(&maki)
        .args(["--trust", "prompt"])
        .current_dir(env.root())
        .env("PATH", path_parts.join(":"))
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "run the real maki binary {} (run ./setup-e2e-harness.sh): {error}",
                maki.display()
            )
        });
    assert!(
        output.status.success(),
        "maki --trust prompt failed (exit {:?}): {}{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        marker.is_file(),
        "the real maki binary did not load the project plugin (no load marker): {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

/// The session id the Maki question scenarios report under.
pub const MAKI_QUESTION_SESSION: &str = "maki-question-fixture";

/// The live-transcript messages one Maki question wait reads at its close: one
/// assistant message carrying the blocking `question` tool_use, with the
/// scenario's question and option labels. `None` builds a transcript with no
/// matching question, the residual a non-question wait leaves.
pub fn maki_question_transcript(question: Option<&str>, options: &[&str]) -> Value {
    match question {
        Some(question) => serde_json::json!([{
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "name": "question",
                "input": {
                    "questions": [{
                        "question": question,
                        "options": options
                            .iter()
                            .map(|label| serde_json::json!({ "label": label }))
                            .collect::<Vec<_>>(),
                    }],
                },
            }],
        }]),
        None => serde_json::json!([
            {
                "role": "assistant",
                "content": [{ "type": "text", "text": "Working on the change." }],
            },
        ]),
    }
}

/// Drive the generated Maki reporter through one question wait: the harness's
/// own `needs_input` -> `working` status pair, with `maki.session.messages`
/// overridden to the fixture transcript the reporter reads at the wait's
/// close. The reporter's real `givn history ingest` reaches the compiled
/// binary through the PATH shim.
pub fn drive_maki_question_wait(
    env: &common::TestEnv,
    title: &str,
    transcript: &Value,
) -> std::process::Output {
    assert_maki_reporter(env);
    let title = lua_literal(&Value::String(title.to_string()));
    let transcript = lua_literal(transcript);
    let driver = format!(
        r#"-- Givn question-wait driver: the shipped fence above ran verbatim; the
-- live transcript is the fixture, and the harness's own status pair opens and
-- closes the wait.
local marker = maki.fs.write(".maki-driver-loaded", "loaded\n")
if not marker then
  error("the driver's load marker could not be written")
end
maki.session.messages = function()
  return {transcript}
end
local status = function(value)
  maki.api.exec_autocmds("SessionStatusChanged", {{
    data = {{
      session_id = {MAKI_QUESTION_SESSION:?},
      status = value,
      title = {title},
      focused = true,
    }},
  }})
  local until_t = os.clock() + 0.02
  while os.clock() < until_t do end
end
status("needs_input")
status("working")
local reporter = require("givn-timeline")
local flushed, flush_err = pcall(reporter.run)
if not flushed then
  maki.log.error("givn-timeline driver: the flush failed: " .. tostring(flush_err))
end
"#
    );
    run_maki_init(env, &driver)
}

/// The Maki events one driver activity reaches the generated reporter through:
/// the harness's own autocmd names and payload fields, in the order a live
/// session fires them. Maki reports no question text on its status change, so
/// the wait's label is the session title the activity carries.
pub fn maki_events_for(activities: &[HarnessActivity]) -> Vec<MakiEvent> {
    let mut events: Vec<MakiEvent> = Vec::new();
    for activity in activities {
        match activity {
            HarnessActivity::Tool { id, tool, args } => {
                events.push((
                    "ToolStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                    }),
                ));
                events.push((
                    "ToolDone",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                        "is_error": false,
                        "bytes": 0,
                        "duration_ms": MAKI_TOOL_MS,
                        "input": args,
                    }),
                ));
            }
            HarnessActivity::Question { id, question } => {
                let _ = id;
                events.push((
                    "SessionStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "status": "needs_input",
                        "title": question,
                        "focused": true,
                    }),
                ));
                events.push((
                    "SessionStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "status": "working",
                        "title": question,
                        "focused": true,
                    }),
                ));
            }
            HarnessActivity::Subagent { id, agent } => {
                events.push((
                    "TaskStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "id": id,
                        "name": agent,
                        "status": "working",
                    }),
                ));
                events.push((
                    "TaskStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "id": id,
                        "name": agent,
                        "status": "done",
                    }),
                ));
            }
            HarnessActivity::Turn {
                index,
                model,
                usage,
            } => {
                events.push((
                    "ModelChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "model": { "id": model },
                    }),
                ));
                events.push((
                    "TurnStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "text": "Implement the change",
                    }),
                ));
                let usage = usage.as_ref().map(|usage| {
                    serde_json::json!({
                        "input_tokens": usage["input"],
                        "output_tokens": usage["output"],
                        "cache_read_tokens": usage["cache_read"],
                        "cache_write_tokens": usage["cache_write"],
                    })
                });
                events.push((
                    "TurnEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": "finished",
                        "num_turns": index,
                        "usage": usage,
                    }),
                ));
            }
            HarnessActivity::TurnMetadata {
                index,
                model,
                stop_reason,
                cost,
                context_size,
                context_window,
                usage,
            } => {
                events.push((
                    "ModelChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "model": { "id": model },
                    }),
                ));
                events.push((
                    "TurnStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "text": "Implement the change",
                    }),
                ));
                let usage = usage.as_ref().map(|usage| {
                    serde_json::json!({
                        "input_tokens": usage["input"],
                        "output_tokens": usage["output"],
                        "cache_read_tokens": usage["cache_read"],
                        "cache_write_tokens": usage["cache_write"],
                    })
                });
                events.push((
                    "TurnEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": stop_reason,
                        "num_turns": index,
                        "usage": usage,
                        "cost": cost,
                        "context_size": context_size,
                        "context_window": context_window,
                    }),
                ));
            }
            HarnessActivity::FailedTool {
                id,
                tool,
                args,
                output_bytes,
                ..
            } => {
                events.push((
                    "ToolStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                    }),
                ));
                events.push((
                    "ToolDone",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                        "is_error": true,
                        "bytes": output_bytes,
                        "duration_ms": MAKI_TOOL_MS,
                        "input": args,
                    }),
                ));
            }
            HarnessActivity::Shutdown => {
                events.push((
                    "SessionEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": "shutdown",
                    }),
                ));
            }
        }
    }
    events
}

/// The harness's own `duration_ms` for a completed tool call: Maki times a call
/// that ran, and its completion payload is where the number arrives.
const MAKI_TOOL_MS: u64 = 40;

/// Drive the project's *generated* Maki reporter through its real entry point
/// with the scenario's activity vocabulary, so the `@e2e` outline's row uses
/// the same driver signature as every other harness.
pub fn drive_maki_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let events = maki_events_for(activities);
    run_maki_driver(env, &events)
}

/// Write one driver script into the fixture root and run it with `node`: the
/// driver's working directory is the project root (so a relative intake
/// invocation and the record both land where the scenario reads them) and its
/// `PATH` carries the `givn` shim. A driver that cannot run at all fails the
/// scenario's fixture, never its assertion.
fn run_node_driver(
    env: &common::TestEnv,
    script_name: &str,
    script: &str,
    reporter: &Path,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let root = env.root();
    let bin = shim_givn_on_path(env);
    let script_path = root.join(script_name);
    std::fs::write(&script_path, script)
        .unwrap_or_else(|error| panic!("write the driver {}: {error}", script_path.display()));
    let activities: Vec<Value> = activities.iter().map(HarnessActivity::as_json).collect();
    let output = std::process::Command::new("node")
        .arg(&script_path)
        .current_dir(&root)
        .env("PATH", driver_path(&[&bin]))
        .env("GIVN_DRIVER_REPORTER", reporter)
        .env("GIVN_DRIVER_SESSION", DRIVER_SESSION)
        .env(
            "GIVN_DRIVER_ACTIVITIES",
            serde_json::Value::Array(activities).to_string(),
        )
        .output()
        .unwrap_or_else(|error| panic!("run the driver {}: {error}", script_path.display()));
    assert!(
        output.status.success(),
        "the driver {} failed (exit {:?}): {}{}",
        script_path.display(),
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

/// The omp driver's script: the fake `pi` extension context the design names,
/// the reporter's own harness events, and nothing else.
const OMP_DRIVER: &str = r#"import { spawnSync } from "node:child_process"
import { pathToFileURL } from "node:url"

const activities = JSON.parse(process.env.GIVN_DRIVER_ACTIVITIES)
const handlers = new Map()
const eventHandlers = new Map()
const pi = {
  on: (event, handler) => handlers.set(event, handler),
  events: { on: (event, handler) => eventHandlers.set(event, handler) },
  getThinkingLevel: () => undefined,
  exec: async (command, args) => {
    const result = spawnSync(command, args, { env: process.env, stdio: ["ignore", "inherit", "inherit"] })
    if (result.status !== 0) throw new Error(`${command} exited ${result.status}`)
  },
}
const module = await import(pathToFileURL(process.env.GIVN_DRIVER_REPORTER).href)
module.default(pi)
const fire = async (event, payload, ctx = {}) => {
  const handler = handlers.get(event)
  if (!handler) throw new Error(`the reporter registered no ${event} handler`)
  await handler(payload, ctx)
}
await fire("session_start", {}, { sessionManager: { getSessionId: () => process.env.GIVN_DRIVER_SESSION } })
for (const activity of activities) {
  if (activity.kind === "tool") {
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: activity.tool, args: activity.args })
    await fire("tool_execution_end", { toolCallId: activity.id, toolName: activity.tool, args: activity.args, isError: false })
  } else if (activity.kind === "question") {
    const args = { questions: [{ question: activity.question }] }
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: "ask", args })
    await fire("tool_execution_end", { toolCallId: activity.id, toolName: "ask", args, isError: false })
  } else if (activity.kind === "subagent") {
    const handler = eventHandlers.get("task:subagent:lifecycle")
    if (!handler) throw new Error("the reporter subscribed to no subagent lifecycle")
    handler({ id: activity.id, agent: activity.agent, status: "started" })
    handler({ id: activity.id, agent: activity.agent, status: "completed" })
  } else if (activity.kind === "turn") {
    await fire("turn_start", { turnIndex: activity.index })
    await fire("turn_end", { turnIndex: activity.index, message: { model: activity.model, usage: activity.usage } })
  } else if (activity.kind === "turn_metadata") {
    await fire("turn_start", { turnIndex: activity.index })
    const usage = {
      ...(activity.usage ?? {}),
      cost: activity.cost,
      contextTokens: activity.context_size,
      contextWindow: activity.context_window,
    }
    await fire(
      "turn_end",
      { turnIndex: activity.index, message: { model: activity.model, stopReason: activity.stop_reason, usage } },
      { getContextUsage: () => ({ tokens: activity.context_size, contextWindow: activity.context_window }) },
    )
  } else if (activity.kind === "failed_tool") {
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: activity.tool, args: activity.args })
    await fire("tool_execution_end", {
      toolCallId: activity.id,
      toolName: activity.tool,
      args: activity.args,
      isError: true,
      result: { error: activity.error, bytes: activity.output_bytes, exit_code: 101 },
    })
  } else if (activity.kind === "shutdown") {
    await fire("session_shutdown", {})
  }
}
"#;

/// The opencode driver's script: the fake plugin context the design names
/// (`session.hook`/`tool.hook`/`shell.hook` plus the `event.subscribe` async
/// iterable the driver itself feeds), and the reporter's own event shapes.
const OPENCODE_DRIVER: &str = r#"import { pathToFileURL } from "node:url"

const activities = JSON.parse(process.env.GIVN_DRIVER_ACTIVITIES)
const session = process.env.GIVN_DRIVER_SESSION
const hooks = new Map()
const queued = []
let wake = null
let closed = false
const stream = {
  async *[Symbol.asyncIterator]() {
    while (!closed) {
      if (queued.length > 0) {
        yield queued.shift()
        continue
      }
      await new Promise((resolve) => { wake = resolve })
      wake = null
    }
  },
}
const registration = (domain) => (kind, handler) => hooks.set(`${domain}:${kind}`, handler)
const ctx = {
  session: { hook: registration("session") },
  tool: { hook: registration("tool") },
  shell: { hook: registration("shell") },
  permission: { hook: registration("permission") },
  event: { subscribe: () => stream },
  signal: undefined,
}
const module = await import(pathToFileURL(process.env.GIVN_DRIVER_REPORTER).href)
await module.default.setup(ctx)
let clock = Date.now()
const stamp = () => (clock += 50)
const emit = async (event) => {
  queued.push(event)
  if (wake) {
    const resolve = wake
    wake = null
    resolve()
  }
  // The consumer handles one event at a time; give it the turn it needs before
  // the next event of the same session.
  await new Promise((resolve) => setTimeout(resolve, 25))
}
// The session's own boundary event: it is what makes the reporter materialize
// and flush, so the driver's activities and its session end share one shape.
const boundary = async () => {
  await emit({ type: "session.execution.succeeded", created: stamp(), data: { sessionID: session } })
  await new Promise((resolve) => setTimeout(resolve, 200))
}
for (const activity of activities) {
  if (activity.kind === "tool") {
    await emit({ type: "session.tool.called", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool, input: activity.args } })
    await emit({ type: "session.tool.success", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool } })
  } else if (activity.kind === "question") {
    await emit({ type: "permission.asked", created: stamp(), data: { sessionID: session, id: activity.id, tool: activity.question } })
    await emit({ type: "permission.replied", created: stamp(), data: { sessionID: session, id: activity.id } })
  } else if (activity.kind === "subagent") {
    await emit({ type: "session.created", created: stamp(), data: { sessionID: session, id: activity.id, parentID: session, agent: activity.agent } })
    await emit({ type: "session.execution.succeeded", created: stamp(), data: { sessionID: activity.id } })
  } else if (activity.kind === "turn") {
    await emit({ type: "session.step.started", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}` } })
    await emit({ type: "session.step.ended", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}`, model: activity.model, tokens: activity.usage } })
  } else if (activity.kind === "turn_metadata") {
    await emit({ type: "session.step.started", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}` } })
    await emit({ type: "session.step.ended", created: stamp(), data: {
      sessionID: session,
      assistantMessageID: `step-${activity.index}`,
      model: activity.model,
      finish: activity.stop_reason,
      rawFinish: activity.stop_reason,
      cost: activity.cost,
      tokens: activity.usage,
    } })
  } else if (activity.kind === "failed_tool") {
    await emit({ type: "session.tool.called", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool, input: activity.args } })
    await emit({ type: "session.tool.failed", created: stamp(), data: {
      sessionID: session,
      id: activity.id,
      name: activity.tool,
      error: activity.error,
      bytes: activity.output_bytes,
    } })
  } else if (activity.kind === "shutdown") {
    await boundary()
  }
}
// The session's own end: a driver whose activities did not report it still
// closes with one final boundary — the reporter materializes whatever never
// closed and flushes the last payload — so the record receives the buffer
// either way.
if (!activities.some((activity) => activity.kind === "shutdown")) await boundary()
closed = true
if (wake) {
  const resolve = wake
  wake = null
  resolve()
}
await new Promise((resolve) => setTimeout(resolve, 200))
"#;
