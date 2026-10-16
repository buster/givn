//! `givn history`: the read view, the one intake (`givn history ingest`), and
//! the process-per-hook harness mapper (`givn history harness-hook`).
//!
//! Reading never records: `givn history` and `givn timeline` run without a
//! recorder, so the store keeps exactly the lines it had (design-review Q8).
//! The intake records normally, like every other command: it is the one place
//! a harness's reported activity reaches the record, and it validates every
//! event it is handed before anything is written.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::history::{
    HistoryEntry, HistoryStore, OUTCOME_UNKNOWN, Scope, USAGE_UNAVAILABLE, parse_rfc3339_ms,
    reported_label, store_path,
};

/// The intake's payload-level failure exit code: stdin was not readable JSON
/// at all. Every payload that was read exits `0`, however many events it
/// rejected — the two codes are the intake's whole machine contract, so a
/// stale reporter can tell "Givn refused my payload" from "Givn read it".
pub const INTAKE_UNREADABLE_PAYLOAD: i32 = 2;

/// The intake's outcome as its machine contract sees it: one enumeration for
/// both codes, never two unrelated returns. Only an unreadable payload is a
/// failure — rejected events and a store that cannot be written are warnings —
/// so a reporter's caller can branch on exactly one thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntakeOutcome {
    /// The payload was read; every rejected event and every unwritable store
    /// was a warning.
    Read,
    /// Stdin was not readable JSON at all; nothing was written.
    UnreadablePayload,
}

impl IntakeOutcome {
    /// The process exit code this outcome reports.
    pub fn exit_code(self) -> i32 {
        match self {
            IntakeOutcome::Read => 0,
            IntakeOutcome::UnreadablePayload => INTAKE_UNREADABLE_PAYLOAD,
        }
    }
}

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
    /// The harness's own status word (`kind: "subagent"`): a terminal word
    /// closes the span a start opened, a running word opens one.
    #[serde(default)]
    pub status: Option<String>,
    /// The agent definition that ran the subagent (`kind: "subagent"`).
    #[serde(default)]
    pub agent: Option<String>,
    /// The subagent's own identity, as the harness reports it
    /// (`kind: "subagent"`, e.g. `Architect.Scout`).
    #[serde(default)]
    pub subagent: Option<String>,
    /// The harness's own turn index (`kind: "turn"`).
    #[serde(default)]
    pub turn: Option<u64>,
    /// The model that produced the turn (`kind: "turn"`).
    #[serde(default)]
    pub model: Option<String>,
    /// The token usage the harness reported for the turn, inline
    /// (`kind: "turn"`): its absence is a signal the record keeps as absent.
    #[serde(default)]
    pub usage: Option<Usage>,
}

/// The token usage one harness report carries: the counts, the model that
/// produced them (`message.model` on a turn), and the subagent session they
/// belong to where the harness reports one.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Usage {
    /// Prompt tokens the model processed.
    #[serde(default)]
    pub input: Option<u64>,
    /// Tokens the model produced.
    #[serde(default)]
    pub output: Option<u64>,
    /// Prompt tokens served from the prompt cache.
    #[serde(default)]
    pub cache_read: Option<u64>,
    /// Prompt tokens written into the prompt cache.
    #[serde(default)]
    pub cache_write: Option<u64>,
    /// The harness's own total, where it reports one.
    #[serde(default)]
    pub total_tokens: Option<u64>,
    /// The model that produced the counts, where the harness reports it here.
    #[serde(default)]
    pub model: Option<String>,
    /// The subagent session the counts belong to, where the harness reports one.
    #[serde(default)]
    pub subagent: Option<String>,
}

impl Usage {
    /// The `data` a usage entry carries, wherever the usage came from: every
    /// count the harness reported, the model that produced it, and the subagent
    /// session it belongs to where the harness reports one. A count the harness
    /// did not report is absent, never zero.
    fn data(&self, model: Option<&str>, subagent: Option<&str>) -> Value {
        let mut data = serde_json::Map::new();
        for (key, count) in [
            ("input", self.input),
            ("output", self.output),
            ("cache_read", self.cache_read),
            ("cache_write", self.cache_write),
            ("total_tokens", self.total_tokens),
        ] {
            if let Some(count) = count {
                data.insert(key.to_string(), Value::from(count));
            }
        }
        if let Some(model) = model {
            data.insert("model".to_string(), Value::String(model.to_string()));
        }
        if let Some(subagent) = subagent {
            data.insert("subagent".to_string(), Value::String(subagent.to_string()));
        }
        Value::Object(data)
    }
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
            "subagent" if self.subagent.is_some() => Ok(()),
            "subagent" => Err("missing required field `subagent`".to_string()),
            "question" if self.label.is_some() => Ok(()),
            "question" => Err("missing required field `label`".to_string()),
            // A turn's usage is optional: a turn the harness did not measure is
            // still a turn (its usage is recorded as unavailable, never zero).
            "turn" => Ok(()),
            kind => Err(format!("unknown kind `{kind}`")),
        }
    }

    /// Append this event's record plan to `batch`: one entry per reported span,
    /// with each entry's parent expressed as the batch index of the entry it is
    /// nested in — the store resolves the index to the id minted in the same
    /// append, so a parent link can never dangle. Only an event
    /// [`IngestedEvent::validate`] accepted reaches this point, so a kind the
    /// table rejects can never disappear silently.
    fn map(&self, scope: &Scope, batch: &mut IntakeBatch) {
        match self.kind.as_str() {
            // A reported span is nested in the turn enclosing the flush, when
            // the payload reported one.
            "tool" => {
                if let Some(tool) = self.tool.as_deref() {
                    batch.push(self.tool_entry(scope, tool), batch.turn);
                }
            }
            "subagent" => {
                if let Some(identity) = self.subagent.as_deref() {
                    self.map_subagent(scope, identity, batch);
                }
            }
            "question" => {
                if let Some(question) = self.label.as_deref() {
                    batch.push(self.question_entry(scope, question), batch.turn);
                }
            }
            "turn" => self.map_turn(scope, batch),
            _ => {}
        }
    }

    /// The turn one reporter flush is built around: the turn entry, and the
    /// usage entry the harness reported inline, parented to it. The turn is the
    /// reporter's flush boundary, so every later span of the same payload nests
    /// in it.
    fn map_turn(&self, scope: &Scope, batch: &mut IntakeBatch) {
        let name = match self.turn {
            Some(index) => format!("turn {index}"),
            None => "turn".to_string(),
        };
        let mut entry = HistoryEntry::reported(scope.clone(), "turn", &name, &self.at);
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        if self.usage.is_none() {
            // Gap honesty: a turn the harness did not measure is still a turn,
            // and its usage is marked unavailable — never a zero-filled usage
            // entry and never a fabricated model.
            entry.data = Some(serde_json::json!({ "usage": USAGE_UNAVAILABLE }));
        }
        let turn = batch.push(entry, batch.turn);
        batch.turn = Some(turn);
        if let Some(usage) = &self.usage {
            batch.push(self.usage_entry(scope, usage), Some(turn));
        }
    }

    /// The one `usage` entry a reported usage produces: every count the harness
    /// reported, the model that produced them (never normalised or truncated),
    /// and the subagent session they belong to where the harness reports one.
    fn usage_entry(&self, scope: &Scope, usage: &Usage) -> HistoryEntry {
        let model = usage.model.as_deref().or(self.model.as_deref());
        let subagent = self.subagent.as_deref().or(usage.subagent.as_deref());
        let mut entry =
            HistoryEntry::reported(scope.clone(), "usage", model.unwrap_or("usage"), &self.at);
        entry.data = Some(usage.data(model, subagent));
        entry.outcome = self.outcome_word();
        entry
    }

    /// The one `question` entry a reported wait produces: the wait is its own
    /// span, never merged into the turn around it, and its label is the
    /// question's first line only — the complete question text is never stored.
    fn question_entry(&self, scope: &Scope, question: &str) -> HistoryEntry {
        let mut entry = HistoryEntry::reported(scope.clone(), "question", "question", &self.at);
        entry.label = Some(reported_label("question", question));
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        entry
    }

    /// The one `tool` entry a reported tool call produces: the harness tool
    /// name as the entry's name, the reported subject through the record's one
    /// bounded-label path, and the harness's own measured duration.
    fn tool_entry(&self, scope: &Scope, tool: &str) -> HistoryEntry {
        let mut entry = HistoryEntry::reported(scope.clone(), "tool", tool, &self.at);
        entry.label = self
            .label
            .as_deref()
            .map(|label| reported_label("tool", label));
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        entry
    }

    /// One reported subagent span: a completion closes the start this payload
    /// reported (or stands alone when the harness measured the span itself),
    /// and an unclosed start still becomes one unmeasured entry — losing a
    /// duration never loses an entry.
    fn map_subagent(&self, scope: &Scope, identity: &str, batch: &mut IntakeBatch) {
        let Some(outcome) = self.reported_outcome() else {
            // A start is remembered until its completion: one reported span
            // yields one entry, never a pair.
            batch.spans.open(
                identity,
                SpanStart {
                    at: self.at.clone(),
                    agent: self.agent.clone(),
                    parent: batch.turn,
                },
            );
            return;
        };
        let start = batch.spans.close(identity);
        let agent = self
            .agent
            .clone()
            .or_else(|| start.as_ref().and_then(|start| start.agent.clone()));
        let mut entry = HistoryEntry::reported(scope.clone(), "subagent", identity, &self.at);
        // The harness's own measured duration where it reports one, otherwise
        // the span between the two reported events.
        entry.duration_ms = self.duration_ms.or_else(|| {
            start
                .as_ref()
                .and_then(|start| crate::history::reported_duration_ms(&start.at, &self.at))
        });
        entry.outcome = outcome.to_string();
        entry.data = Some(subagent_data(agent.as_deref(), identity));
        // The span is nested in the turn that reported it, when the payload
        // reported one, else in the turn its start was reported inside.
        let parent = start.and_then(|start| start.parent).or(batch.turn);
        batch.push(entry, parent);
    }

    /// The record outcome this event reports, or `None` when it reports only a
    /// state the record has no outcome for (a harness's own `started`/`running`
    /// word).
    fn reported_outcome(&self) -> Option<&'static str> {
        self.outcome
            .as_deref()
            .and_then(outcome_of)
            .or_else(|| self.status.as_deref().and_then(outcome_of))
    }

    /// The record outcome word the entry carries: the reported outcome, and
    /// [`OUTCOME_UNKNOWN`] when the event reports none — a reported word
    /// outside the record's vocabulary is never promoted to a passing state.
    fn outcome_word(&self) -> String {
        self.reported_outcome()
            .unwrap_or(OUTCOME_UNKNOWN)
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
        OUTCOME_UNKNOWN => Some(OUTCOME_UNKNOWN),
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
/// the intake never changes the outcome of the agent action it reports. Only
/// the payload itself is a failure: stdin that is not readable JSON at all
/// reports [`IntakeOutcome::UnreadablePayload`], so the reporter's caller gets
/// a code it can act on, and nothing is written.
pub fn ingest() -> Result<IntakeOutcome> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let payload = match read_payload() {
        Ok(payload) => payload,
        Err(error) => {
            // The payload-level failure: distinguishable from every accepted
            // payload (any number of rejected events), and the store is
            // untouched because nothing is written before the payload parses.
            eprintln!("error: {error:#}");
            return Ok(IntakeOutcome::UnreadablePayload);
        }
    };
    // The scope every accepted event is recorded under: the payload's named
    // override (a scope this project does not have is a per-event rejection,
    // never a silent fallback) or the project's own resolution.
    let scope = payload_scope(&root, &payload);
    let IntakeBatch {
        mut entries,
        parents,
        ..
    } = accepted_entries(&scope, payload.events);
    if entries.is_empty() {
        return Ok(IntakeOutcome::Read);
    }
    let store = HistoryStore::at(store_path(&root));
    // The record's ignore entries are re-asserted on every record write, so a
    // store that predates the repository still becomes ignored.
    crate::history::ensure_ignored(&root);
    if let Err(error) = store.append_linked(&mut entries, &parents) {
        crate::history::warn_unwritten(&store, &error);
    }
    Ok(IntakeOutcome::Read)
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

/// The record a payload contributes: every event is read in payload order, each
/// accepted event contributes its entries to the batch plan, and each rejected
/// event is named on stderr with the reason — the payload's remaining events
/// are still processed, because a reporter must never be able to block or
/// corrupt the record.
fn accepted_entries(scope: &std::result::Result<Scope, String>, events: Vec<Value>) -> IntakeBatch {
    let mut batch = IntakeBatch::default();
    for (index, raw) in events.into_iter().enumerate() {
        let kind = raw.get("kind").and_then(Value::as_str).map(str::to_owned);
        if let Err(reason) = accept_event(scope, raw, &mut batch) {
            warn_rejected(index, kind.as_deref(), &reason);
        }
    }
    // A reported start the payload never completed still becomes one unmeasured
    // entry.
    if let Ok(scope) = scope {
        batch.finish(scope);
    }
    batch
}

/// Read one raw event into the batch plan, or return the reason it is rejected.
/// Both gates an event passes live here — the payload's scope (a named scope
/// this project does not have rejects every event) and the event's own
/// validation ([`IngestedEvent::validate`]) — so no kind can reach the append
/// path without them.
fn accept_event(
    scope: &std::result::Result<Scope, String>,
    raw: Value,
    batch: &mut IntakeBatch,
) -> std::result::Result<(), String> {
    let scope = scope.as_ref().map_err(Clone::clone)?;
    let event: IngestedEvent = serde_json::from_value(raw).map_err(|error| error.to_string())?;
    event.validate()?;
    event.map(scope, batch);
    Ok(())
}

/// The record one payload contributes, in payload order: the entries the store
/// will append, each with the batch index of the entry it is nested in, plus
/// the subagent starts still waiting for their completion and the turn
/// enclosing this flush's later spans.
#[derive(Debug, Default)]
struct IntakeBatch {
    entries: Vec<HistoryEntry>,
    parents: Vec<Option<usize>>,
    /// The batch index of the turn entry the payload reported, when it reported
    /// one: a turn is the reporter's flush boundary, so every later span of the
    /// same flush is nested in it.
    turn: Option<usize>,
    /// The open spans the payload reported, keyed by the harness's own call
    /// id: its start and its completion become exactly one entry, never a pair.
    spans: OpenSpans,
}

impl IntakeBatch {
    /// Plan one entry, nested in the batch entry at `parent` when the payload
    /// reported an enclosing entry. Returns the entry's index in the plan, so a
    /// span reported after it can nest in it.
    fn push(&mut self, entry: HistoryEntry, parent: Option<usize>) -> usize {
        self.parents.push(parent);
        self.entries.push(entry);
        self.entries.len() - 1
    }

    /// Close the batch: a start the payload never closed still becomes one
    /// unmeasured entry, with the outcome the record keeps for a duration it
    /// does not have — losing a duration never loses an entry.
    fn finish(&mut self, scope: &Scope) {
        for (identity, start) in self.spans.take_unclosed() {
            let mut entry = HistoryEntry::reported(scope.clone(), "subagent", &identity, &start.at);
            entry.data = Some(subagent_data(start.agent.as_deref(), &identity));
            self.push(entry, start.parent);
        }
    }
}

/// The open spans of one payload: a span whose start and completion are two
/// reported events — the shape a subagent's lifecycle and a question wait
/// share — is paired here once, keyed by the harness's own call id, so both
/// kinds express the pairing through the same mechanism.
#[derive(Debug, Default)]
struct OpenSpans {
    starts: BTreeMap<String, SpanStart>,
}

impl OpenSpans {
    /// Open a span at the harness's own call id.
    fn open(&mut self, id: &str, start: SpanStart) {
        self.starts.insert(id.to_string(), start);
    }

    /// Close the span `id` names, returning the start that opened it.
    fn close(&mut self, id: &str) -> Option<SpanStart> {
        self.starts.remove(id)
    }

    /// Take the starts no completion closed.
    fn take_unclosed(&mut self) -> BTreeMap<String, SpanStart> {
        std::mem::take(&mut self.starts)
    }
}

/// One reported span start: where the span opened, the agent definition that
/// ran it (`kind: "subagent"`), and the turn it was reported inside (when the
/// payload reported one).
#[derive(Debug)]
struct SpanStart {
    at: String,
    agent: Option<String>,
    parent: Option<usize>,
}

/// The `data` a subagent entry carries: the subagent's own identity (the
/// report's fan-out view groups by it) and the agent definition that ran it,
/// where the harness reported one.
fn subagent_data(agent: Option<&str>, identity: &str) -> Value {
    let mut data = serde_json::Map::new();
    if let Some(agent) = agent {
        data.insert("agent".to_string(), Value::String(agent.to_string()));
    }
    data.insert("subagent".to_string(), Value::String(identity.to_string()));
    Value::Object(data)
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
