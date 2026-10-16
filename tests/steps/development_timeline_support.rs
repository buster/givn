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
    let path = match std::env::var_os("PATH") {
        Some(existing) => format!("{}:{}", bin.display(), existing.to_string_lossy()),
        None => bin.display().to_string(),
    };
    let mut child = std::process::Command::new(&shim)
        .current_dir(env.root())
        .env("PATH", path)
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
    common::run_givn_with_stdin_and_envs(env, &["history", "ingest"], &payload.to_string(), &[])
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
