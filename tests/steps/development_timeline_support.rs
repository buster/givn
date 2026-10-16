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

use std::path::Path;

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

/// The development record of the project rooted at `root`, read the way a
/// consumer reads it: `givn/history/events.jsonl`, one JSON object per line.
/// Every line must parse as a JSON object — that is the store's own contract,
/// so a malformed line fails the scenario instead of being skipped, and a
/// missing store fails it too (the scenario ran a command that records).
pub fn record_entries(root: &Path) -> Vec<Value> {
    let path = root.join("givn/history/events.jsonl");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the development record {}: {error}", path.display()));
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .unwrap_or_else(|error| panic!("store line is not a JSON object: {error}\n{line}"))
        })
        .collect()
}
