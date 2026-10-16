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
