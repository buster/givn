//! The development record (ADR-073): the append-only per-project store
//! `givn/history/events.jsonl`, scope resolution, bounded labels, pending
//! state for process-per-hook harnesses, and the per-process recorder.
//!
//! Best-effort by construction: a write that cannot happen warns and
//! continues, and never changes the outcome of the work it records.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// One record line: `id`, `at`, `scope`, `source`, `kind`, `name`, `label`,
/// `duration_ms`, `outcome`, `parent`, `data`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Record schema version; a reader skips a line whose schema it does not
    /// know.
    pub schema: u32,
    /// Unique per project, minted at append time under the store's append
    /// lock.
    pub id: u64,
    /// RFC 3339 UTC end timestamp of the entry, millisecond precision.
    pub at: String,
    /// The exactly-one scope the entry belongs to.
    pub scope: Scope,
    /// `givn` (Givn recorded it) or `agent` (reported through the intake).
    pub source: String,
    /// `command`, `phase`, `hook`, `step`, `revision`, `session`, `turn`,
    /// `tool`, `subagent`, `question`, or `usage`.
    pub kind: String,
    /// Domain name of the entry.
    pub name: String,
    /// The bounded subject label (≤ 120 characters, truncation marked `…`).
    pub label: Option<String>,
    /// Measured duration; absent for a point entry.
    pub duration_ms: Option<u64>,
    /// `ok`, `failed`, `skipped`, or `unknown`.
    pub outcome: String,
    /// The `id` of the enclosing entry, when the same process minted it.
    pub parent: Option<u64>,
    /// Only where the report needs a non-generic value.
    pub data: Option<serde_json::Value>,
}

/// The one scope an entry belongs to — `Change(id)`, `Ideation(id)`,
/// `Project(name)` — the exactly-one-scope rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Scope {
    /// The active change.
    Change { id: String },
    /// The unfinished ideation topic.
    Ideation { id: String },
    /// The project itself (the fallback).
    Project { id: String },
}

/// Append and read the project's `givn/history/events.jsonl` (one JSON object
/// per line, opened append-only, never rewritten, never reordered); owns the
/// schema and mints each `id` at append time under the append lock.
// Skeleton (tasks.md S4): the scenario that reads this state drops the
// allowance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    /// Read every entry the store holds; a line whose schema this reader does
    /// not know is skipped.
    pub fn read(&self) -> std::io::Result<Vec<HistoryEntry>> {
        todo!("TODO: development-timeline")
    }

    /// Append the entries in order, minting each `id` at append time under the
    /// store's exclusive append lock and from the store's current last line.
    pub fn append(&self, _entries: &mut [HistoryEntry]) -> std::io::Result<()> {
        todo!("TODO: development-timeline")
    }
}

/// Per-process recorder: the store handle, the scope resolved once, `record`,
/// and a `span(name)` guard that measures `Instant::now()..drop`. It never
/// returns an error to its caller.
// Skeleton (tasks.md S4): the scenario that reads this state drops the
// allowance.
#[allow(dead_code)]
#[derive(Debug)]
pub struct HistoryRecorder {
    store: HistoryStore,
    scope: Scope,
}

impl HistoryRecorder {
    /// Record one entry under this process's command entry.
    pub fn record(&self, _entry: HistoryEntry) {
        todo!("TODO: development-timeline")
    }

    /// A guard that measures one action's wall time and writes its entry when
    /// it is finished or dropped.
    pub fn span(&self, _name: &str) -> Span {
        todo!("TODO: development-timeline")
    }
}

/// A measured span: it measures `Instant::now()..drop` and writes the entry it
/// measured.
// Skeleton (tasks.md S4): the scenario that reads this state drops the
// allowance.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Span {
    started: std::time::Instant,
}

/// The scope every entry of this project resolves to: exactly one active
/// change, else exactly one unfinished ideation topic, else the project.
pub fn resolve_scope(_root: &Path) -> Scope {
    todo!("TODO: development-timeline")
}

/// The bounded subject label (≤ 120 characters, truncation marked with `…`) —
/// the only place raw subjects are touched.
pub fn bound_label(_subject: &str) -> String {
    todo!("TODO: development-timeline")
}

/// The project's store path: `givn/history/events.jsonl`.
pub fn store_path(_root: &Path) -> PathBuf {
    todo!("TODO: development-timeline")
}

/// Keep the store out of repository history through the managed
/// `# givn:begin:history` ignore block.
pub fn ensure_ignored(_root: &Path) -> std::io::Result<()> {
    todo!("TODO: development-timeline")
}

/// Start timestamps for process-per-hook harnesses, keyed by the harness's own
/// call id: written atomically, pruned by age and at session end, no payload
/// content.
// Skeleton (tasks.md S4): the scenario that reads this state drops the
// allowance.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PendingState {
    starts: BTreeMap<String, PendingStart>,
}

/// One pending start: when it began, the entry kind it will produce, and the
/// bounded label it keeps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingStart {
    /// RFC 3339 UTC start timestamp of the tool call or question.
    pub started_at: String,
    /// The entry kind the matching end event produces.
    pub kind: String,
    /// The bounded subject label.
    pub label: Option<String>,
}
