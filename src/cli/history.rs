//! `givn history`: the read view, the one intake (`givn history ingest`), and
//! the process-per-hook harness mapper (`givn history harness-hook`).
//!
//! Reading never records: `givn history` and `givn timeline` run without a
//! recorder, so the store keeps exactly the lines it had (design-review Q8).
//! The intake records normally, like every other command: it is the one place
//! a harness's reported activity reaches the record, and it validates every
//! event it is handed before anything is written.

use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::history::{
    HistoryEntry, HistoryStore, Scope, bound_label, parse_rfc3339_ms, store_path,
};

/// The intake payload: one reporter flush — the optional scope overrides and
/// the events the reporter buffered.
///
/// The payload's other fields (`schema`, `reporter`, `session`) belong to the
/// reporter; every decision the intake makes is derived from the scope
/// overrides and the events themselves.
#[derive(Debug, Default, serde::Deserialize)]
pub struct IntakePayload {
    /// The change this flush's events belong to; wins over the topic and over
    /// the binary's own resolution.
    #[serde(default)]
    pub change: Option<String>,
    /// The ideation topic this flush's events belong to.
    #[serde(default)]
    pub topic: Option<String>,
    /// The events the reporter buffered.
    #[serde(default)]
    pub events: Vec<Value>,
}

/// One reported event from the intake payload, with its per-kind validation.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct IngestedEvent {
    /// The event kind (`turn`, `tool`, `subagent`, `question`, `usage`, …).
    pub kind: String,
    /// RFC 3339 UTC end timestamp of the event.
    pub at: String,
    /// The harness tool name (`kind: "tool"`).
    #[serde(default)]
    pub tool: Option<String>,
    /// The bounded subject (`kind: "tool"`, `kind: "question"`).
    #[serde(default)]
    pub label: Option<String>,
    /// The measured duration, where the harness or the reporter measured one.
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// The harness's own outcome word (`kind: "tool"`).
    #[serde(default)]
    pub outcome: Option<String>,
}

impl IngestedEvent {
    /// The intake's one validation table: the kind must be one the intake maps,
    /// the `at` must be an RFC 3339 UTC timestamp, and the event must carry its
    /// kind's own identity. A violation returns the reason the event is
    /// rejected with — the event alone, never the payload.
    fn validate(&self) -> std::result::Result<(), String> {
        if parse_rfc3339_ms(&self.at).is_none() {
            return Err(format!(
                "`at` is not an RFC 3339 UTC timestamp: {}",
                self.at
            ));
        }
        match self.kind.as_str() {
            "tool" if self.tool.is_some() => Ok(()),
            "tool" => Err("missing required field `tool`".to_string()),
            kind => Err(format!("unknown kind `{kind}`")),
        }
    }

    /// The record entries this event contributes, in payload order — empty when
    /// the event is only the start of a span a later event closes. Only an
    /// event [`IngestedEvent::validate`] accepted reaches this point, so a kind
    /// the table rejects can never disappear silently.
    fn entries(&self, scope: &Scope) -> Vec<HistoryEntry> {
        match self.kind.as_str() {
            "tool" => self.tool_entry(scope).into_iter().collect(),
            _ => Vec::new(),
        }
    }

    /// The one `tool` entry a reported tool call produces: the harness tool
    /// name as the entry's name, the reported subject through the record's one
    /// bounded-label path, and the harness's own measured duration.
    fn tool_entry(&self, scope: &Scope) -> Option<HistoryEntry> {
        let tool = self.tool.as_deref()?;
        let mut entry = HistoryEntry::reported(scope.clone(), "tool", tool, &self.at);
        entry.label = self.label.as_deref().map(bound_label);
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.reported_outcome();
        Some(entry)
    }

    /// The record outcome this event reports: its `outcome` word, and `unknown`
    /// when it reports none — a reported word outside the record's vocabulary
    /// is never promoted to a passing state.
    fn reported_outcome(&self) -> String {
        self.outcome
            .as_deref()
            .and_then(outcome_of)
            .unwrap_or("unknown")
            .to_string()
    }
}

/// The record outcome a reported outcome or status word maps to; `None` for a
/// word that reports no outcome.
fn outcome_of(word: &str) -> Option<&'static str> {
    match word {
        "ok" | "completed" | "complete" | "success" | "succeeded" => Some("ok"),
        "failed" | "failure" | "error" | "errored" => Some("failed"),
        "skipped" | "cancelled" | "canceled" | "interrupted" | "aborted" => Some("skipped"),
        "unknown" => Some("unknown"),
        _ => None,
    }
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

/// `givn history ingest` — read one JSON payload from stdin and store the
/// events it accepts.
///
/// A rejected event is named on stderr and the payload's remaining events are
/// still appended, and a store that cannot be written warns instead of failing:
/// the intake never changes the outcome of the agent action it reports.
pub fn ingest() -> Result<()> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let payload = read_payload()?;
    // The scope every accepted event is recorded under: the payload's named
    // override (a scope this project does not have is a per-event rejection,
    // never a silent fallback) or the project's own resolution.
    let scope = payload_scope(&root, &payload);
    let mut entries = accepted_entries(&scope, payload.events);
    if entries.is_empty() {
        return Ok(());
    }
    let store = HistoryStore::at(store_path(&root));
    // The record's ignore entries are re-asserted on every record write, so a
    // store that predates the repository still becomes ignored.
    crate::history::ensure_ignored(&root);
    if let Err(error) = store.append(&mut entries) {
        eprintln!(
            "warning: the development record could not be written to {}: {error}",
            store.path().display()
        );
    }
    Ok(())
}

/// The intake's one stdin read: exactly one JSON object. Nothing is written
/// before the payload parses, so an unreadable payload leaves the store exactly
/// as it was.
fn read_payload() -> Result<IntakePayload> {
    let mut text = String::new();
    std::io::stdin()
        .read_to_string(&mut text)
        .context("read the intake payload")?;
    serde_json::from_str(&text).context("parse the intake payload as one JSON object")
}

/// The scope one payload's events are recorded under: the payload's named
/// override — a change first, then a topic — or the project's own exactly-one
/// resolution. A named scope this project does not have is returned as the
/// reason every event is rejected: a reporter's stale name is never silently
/// replaced by the resolution.
fn payload_scope(root: &Path, payload: &IntakePayload) -> std::result::Result<Scope, String> {
    if let Some(id) = payload.change.as_deref() {
        return if root.join("givn").join("changes").join(id).is_dir() {
            Ok(Scope::Change { id: id.to_string() })
        } else {
            Err(format!("no change `{id}` exists in this project"))
        };
    }
    if let Some(id) = payload.topic.as_deref() {
        return if root.join("givn").join("ideation").join(id).is_dir() {
            Ok(Scope::Ideation { id: id.to_string() })
        } else {
            Err(format!("no ideation topic `{id}` exists in this project"))
        };
    }
    Ok(crate::history::resolve_scope(root))
}

/// The entries a payload contributes: every event is read in payload order,
/// each accepted event contributes its entries, and each rejected event is
/// named on stderr with the reason — the payload's remaining events are still
/// processed, because a reporter must never be able to block or corrupt the
/// record.
fn accepted_entries(
    scope: &std::result::Result<Scope, String>,
    events: Vec<Value>,
) -> Vec<HistoryEntry> {
    let mut entries: Vec<HistoryEntry> = Vec::new();
    for (index, raw) in events.into_iter().enumerate() {
        let kind = raw.get("kind").and_then(Value::as_str).map(str::to_owned);
        match accepted_event(scope, raw) {
            Ok(contributed) => entries.extend(contributed),
            Err(reason) => warn_rejected(index, kind.as_deref(), &reason),
        }
    }
    entries
}

/// The entries one raw event contributes, or the reason it is rejected. Both
/// gates an event passes live here — the payload's scope (a named scope this
/// project does not have rejects every event) and the event's own validation
/// ([`IngestedEvent::validate`]) — so no kind can reach the append path
/// without them.
fn accepted_event(
    scope: &std::result::Result<Scope, String>,
    raw: Value,
) -> std::result::Result<Vec<HistoryEntry>, String> {
    let scope = scope.as_ref().map_err(Clone::clone)?;
    let event: IngestedEvent = serde_json::from_value(raw).map_err(|error| error.to_string())?;
    event.validate()?;
    Ok(event.entries(scope))
}

/// The intake's one rejection path: the rejected event is named on stderr by
/// its 1-based position in the payload, its kind when it carries a legible one,
/// and the reason — legible enough for the reporter's author to fix the
/// payload.
fn warn_rejected(index: usize, kind: Option<&str>, reason: &str) {
    eprintln!(
        "warning: the intake rejected event {} (kind `{}`): {reason}",
        index + 1,
        kind.unwrap_or("<none>"),
    );
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
