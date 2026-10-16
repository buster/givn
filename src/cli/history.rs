//! `givn history`: the read view, the one intake (`givn history ingest`), and
//! the process-per-hook harness mapper (`givn history harness-hook`).
//!
//! Reading never records: `givn history` and `givn timeline` run without a
//! recorder, so the store keeps exactly the lines it had (design-review Q8).

use std::path::Path;

use anyhow::{Context, Result};

use crate::history::{HistoryEntry, HistoryStore, Scope, store_path};

/// One reported event from the intake payload, with its per-kind validation.
// Skeleton (tasks.md S4): wired by the agent-activity-reporting scenarios.
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct IngestedEvent {
    /// The event kind (`turn`, `tool`, `subagent`, `question`, `usage`, …).
    pub kind: String,
    /// RFC 3339 UTC end timestamp of the event.
    pub at: String,
}

/// One raw hook payload from a process-per-hook harness, with the fields the
/// mapper reads.
// Skeleton (tasks.md S4): wired by the agent-activity-reporting scenarios.
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct HarnessPayload {
    /// The harness's own session id (the pending-state file key).
    pub session_id: String,
}

/// The scope a read covers: the change the caller named, the topic the caller
/// named, or the scope the project's own state resolves to — the exactly-one
/// rule `resolve_scope` applies.
pub fn scope_for(root: &Path, change: Option<&str>, topic: Option<&str>) -> Option<Scope> {
    match (change, topic) {
        (Some(id), _) => Some(Scope::Change { id: id.to_string() }),
        (None, Some(id)) => Some(Scope::Ideation { id: id.to_string() }),
        (None, None) => Some(crate::history::resolve_scope(root)),
    }
}

/// `root`'s record lines, in file order, limited to `scope` when the caller
/// named or resolved one.
pub fn entries_in_scope(root: &Path, scope: Option<&Scope>) -> Result<Vec<HistoryEntry>> {
    let entries = HistoryStore::at(store_path(root))
        .read()
        .with_context(|| format!("read the development record of {}", root.display()))?;
    Ok(match scope {
        Some(scope) => entries
            .into_iter()
            .filter(|entry| &entry.scope == scope)
            .collect(),
        None => entries,
    })
}

/// `givn history` — read one scope's record.
pub fn run(change: Option<&str>, topic: Option<&str>, json: bool) -> Result<()> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let scope = scope_for(&root, change, topic);
    let entries = entries_in_scope(&root, scope.as_ref())?;
    if json {
        println!(
            "{}",
            serde_json::to_string(&entries).context("serialize the development record")?
        );
        return Ok(());
    }
    for entry in &entries {
        println!(
            "{}  {:<9} {}{}",
            entry.at,
            entry.kind,
            entry.name,
            match &entry.label {
                Some(label) => format!("  {label}"),
                None => String::new(),
            }
        );
    }
    Ok(())
}

/// `givn history ingest` — read one JSON payload from stdin and store its
/// accepted events.
// Skeleton (tasks.md S4): wired into dispatch by the intake scenario.
#[allow(dead_code)]
pub fn ingest() -> Result<()> {
    todo!("TODO: development-timeline")
}

/// `givn history harness-hook --harness <name>` — map one raw harness payload
/// from stdin into the record (the process-per-hook entry point).
// Skeleton (tasks.md S4): wired into dispatch by the harness scenarios.
#[allow(dead_code)]
pub fn harness_hook(_harness: &str) -> Result<()> {
    todo!("TODO: development-timeline")
}

/// Raw harness payload → record events plus pending-state updates; the Claude
/// mapper's only entry point, also usable by a future shell-based harness.
// Skeleton (tasks.md S4): called by `harness_hook` once it is wired.
#[allow(dead_code)]
pub fn map_harness_event(_payload: &HarnessPayload) -> Result<Vec<IngestedEvent>> {
    todo!("TODO: development-timeline")
}
