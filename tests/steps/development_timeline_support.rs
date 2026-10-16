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

/// Assert the fixture project is bare in the recorder's sense: no directory
/// under `givn/changes/` (every directory there is a change for
/// `list_changes`) and no `givn/ideation/<topic>/session.md` — so the project
/// fallback is exercised rather than assumed.
pub fn assert_bare_project(env: &common::TestEnv) {
    let changes = read_dir_names(&env.path("givn/changes"));
    assert!(
        changes.is_empty(),
        "the fixture project has changes: {changes:?}"
    );
    for topic in read_dir_names(&env.path("givn/ideation")) {
        let session = env.path(&format!("givn/ideation/{topic}/session.md"));
        assert!(
            !session.exists(),
            "the fixture project has an ideation session: {}",
            session.display()
        );
    }
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

/// The development record of the project rooted at `root`, read the way a
/// consumer reads it: `givn/history/events.jsonl`, one JSON object per line.
/// Every line must parse as a JSON object — that is the store's own contract,
/// so a malformed line fails the scenario instead of being skipped, and a
/// missing store fails it too (the scenario ran a command that records).
pub fn record_entries(root: &Path) -> Vec<Value> {
    parse_entries(&store_bytes(root))
}

/// The store's raw bytes. "Append-only" and "never rewritten" are claims about
/// these bytes, so the fixture reads the file itself instead of a
/// re-serialized view of its entries.
pub fn store_bytes(root: &Path) -> Vec<u8> {
    let path = store_file(root);
    std::fs::read(&path)
        .unwrap_or_else(|error| panic!("read the development record {}: {error}", path.display()))
}

/// The entries of a store's bytes: one JSON object per line, every line
/// required to parse.
fn parse_entries(bytes: &[u8]) -> Vec<Value> {
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
