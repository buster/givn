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
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;

use crate::history::{
    BoundedText, HistoryEntry, HistoryStore, OUTCOME_UNKNOWN, PendingStart, PendingState,
    RecordMode, Scope, USAGE_UNAVAILABLE, bound_content, bound_error, bound_question,
    bound_result_meta, bound_text_metadata, parse_rfc3339_ms, reported_label, store_path,
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
    /// The harness session every event of this flush belongs to: the entry
    /// envelope carries it once (`session`), so a report can group a session.
    #[serde(default)]
    pub session: Option<String>,
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
    /// The harness's own stop reason for the turn (`kind: "turn"`): a raw
    /// word, kept verbatim and bounded as short descriptive text.
    #[serde(default)]
    pub stop_reason: Option<String>,
    /// The turn's cost as the harness reports it (real and list price where
    /// present) (`kind: "turn"`).
    #[serde(default)]
    pub cost: Option<Value>,
    /// The list-price cost, where the harness reports it beside the real one
    /// (`kind: "turn"`).
    #[serde(default)]
    pub list_cost: Option<Value>,
    /// The turn's context size, where the harness measures one
    /// (`kind: "turn"`).
    #[serde(default)]
    pub context_size: Option<u64>,
    /// The model's context window, where the harness reports one
    /// (`kind: "turn"`).
    #[serde(default)]
    pub context_window: Option<u64>,
    /// The harness's own tool call id (`kind: "tool"`).
    #[serde(default)]
    pub tool_call_id: Option<String>,
    /// The failed call's error text (`kind: "tool"`): the intake keeps its
    /// first line within the error bound.
    #[serde(default)]
    pub error: Option<String>,
    /// The tool output's size in bytes (`kind: "tool"`).
    #[serde(default)]
    pub output_bytes: Option<u64>,
    /// The host's own result details (`kind: "tool"`): exit code, wall time,
    /// truncation counts — bounded short values only.
    #[serde(default)]
    pub result_meta: Option<Value>,
    /// Whether the measured duration came from the host or from the reporter
    /// (`host` or `reporter`, `kind: "tool"`).
    #[serde(default)]
    pub duration_source: Option<String>,
    /// A short descriptive value the record keeps bounded: a subagent's title,
    /// a notification text, or another metadata label.
    #[serde(default)]
    pub title: Option<String>,
    /// The complete question text the reporter captured (`kind: "question"`):
    /// the one content value stored in every mode, under the question bound.
    #[serde(default)]
    pub question: Option<BoundedText>,
    /// The selected option when the question was answered (`kind: "question"`).
    #[serde(default)]
    pub selected: Option<String>,
    /// Who or what answered the question (`kind: "question"`).
    #[serde(default, alias = "answer_source")]
    pub answered_by: Option<String>,
    /// RFC 3339 UTC answer timestamp (`kind: "question"`).
    #[serde(default)]
    pub answered_at: Option<String>,
    /// Free-text answer when the reporter captured one (`kind: "question"`).
    #[serde(default)]
    pub answer: Option<BoundedText>,
    /// The provider that produced the turn or usage (`kind: "turn" | "usage"`).
    #[serde(default)]
    pub provider: Option<String>,
    /// The model variant the provider reported (`kind: "turn" | "usage"`).
    #[serde(default)]
    pub variant: Option<String>,
    /// The reporter's LLM-facing content fields: stored only when the effective
    /// mode is `full`, and always re-capped by the intake.
    #[serde(default)]
    pub content: Option<BTreeMap<String, BoundedText>>,
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
    /// Tokens the model produced as reasoning, where the harness reports them.
    #[serde(default)]
    pub reasoning: Option<u64>,
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
            ("reasoning", self.reasoning),
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
            "tool" if self.tool.is_some() => match self.duration_source.as_deref() {
                None | Some("host") | Some("reporter") => Ok(()),
                Some(other) => Err(format!("unknown duration_source `{other}`")),
            },
            "tool" => Err("missing required field `tool`".to_string()),
            "subagent" if self.subagent.is_some() => Ok(()),
            "subagent" => Err("missing required field `subagent`".to_string()),
            "question" if self.label.is_some() => Ok(()),
            "question" => Err("missing required field `label`".to_string()),
            // A reported phase (a harness's own long step, e.g. Maki's
            // compaction) is named by its label and carries its measured
            // duration.
            "phase" if self.label.is_some() => Ok(()),
            "phase" => Err("missing required field `label`".to_string()),
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
    /// table rejects can never disappear silently. The effective `mode` gates
    /// the content set; every bound is applied here regardless of the mode.
    fn map(&self, scope: &Scope, batch: &mut IntakeBatch, mode: RecordMode) {
        match self.kind.as_str() {
            // A reported span is nested in the turn enclosing the flush, when
            // the payload reported one.
            "tool" => {
                if let Some(tool) = self.tool.as_deref() {
                    batch.push(self.tool_entry(scope, tool, mode), batch.turn);
                }
            }
            "subagent" => {
                if let Some(identity) = self.subagent.as_deref() {
                    self.map_subagent(scope, identity, batch, mode);
                }
            }
            "question" => {
                if let Some(question) = self.label.as_deref() {
                    batch.push(self.question_entry(scope, question, mode), batch.turn);
                }
            }
            "phase" => {
                if let Some(phase) = self.label.as_deref() {
                    batch.push(self.phase_entry(scope, phase, mode), batch.turn);
                }
            }
            "turn" => self.map_turn(scope, batch, mode),
            _ => {}
        }
    }

    /// The turn one reporter flush is built around: the turn entry, and the
    /// usage entry the harness reported inline, parented to it. The turn is the
    /// reporter's flush boundary, so every later span of the same payload nests
    /// in it.
    fn map_turn(&self, scope: &Scope, batch: &mut IntakeBatch, mode: RecordMode) {
        let name = match self.turn {
            Some(index) => format!("turn {index}"),
            None => "turn".to_string(),
        };
        let mut entry = HistoryEntry::reported(scope.clone(), "turn", &name, &self.at);
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        let mut data = serde_json::Map::new();
        if let Some(index) = self.turn {
            data.insert("index".to_string(), Value::from(index));
        }
        // Gap honesty: a turn the harness did not measure records no model —
        // the usage and its model arrive together, and an unmeasured turn is
        // marked unavailable rather than filled with a bare model name.
        if let Some(model) = &self.model
            && self.usage.is_some()
        {
            data.insert("model".to_string(), Value::String(model.clone()));
        }
        if let Some(provider) = &self.provider {
            data.insert(
                "provider".to_string(),
                Value::String(bound_text_metadata(provider)),
            );
        }
        if let Some(variant) = &self.variant {
            data.insert(
                "variant".to_string(),
                Value::String(bound_text_metadata(variant)),
            );
        }
        if let Some(reason) = &self.stop_reason {
            data.insert(
                "stop_reason".to_string(),
                Value::String(bound_text_metadata(reason)),
            );
        }
        match &self.usage {
            Some(usage) => {
                data.insert(
                    "usage".to_string(),
                    usage.data(self.model.as_deref(), self.subagent.as_deref()),
                );
            }
            None => {
                // Gap honesty: a turn the harness did not measure is still a
                // turn, and its usage is marked unavailable — never a
                // zero-filled usage and never a fabricated model.
                data.insert(
                    "usage".to_string(),
                    Value::String(USAGE_UNAVAILABLE.to_string()),
                );
            }
        }
        if let Some(cost) = &self.cost {
            data.insert("cost".to_string(), cost.clone());
        }
        if let Some(cost) = &self.list_cost {
            data.insert("list_cost".to_string(), cost.clone());
        }
        if let Some(error) = &self.error {
            data.insert("error".to_string(), Value::String(bound_error(error)));
        }
        if let Some(size) = self.context_size {
            data.insert("context_size".to_string(), Value::from(size));
        }
        if let Some(window) = self.context_window {
            data.insert("context_window".to_string(), Value::from(window));
        }
        if let Some(title) = &self.title {
            data.insert(
                "title".to_string(),
                Value::String(bound_text_metadata(title)),
            );
        }
        entry.data = Some(Value::Object(data));
        self.attach_content(&mut entry, mode);
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
        let mut data = usage.data(model, subagent);
        if let Some(object) = data.as_object_mut() {
            object.insert("derived".to_string(), Value::Bool(true));
            if let Some(provider) = &self.provider {
                object.insert(
                    "provider".to_string(),
                    Value::String(bound_text_metadata(provider)),
                );
            }
            if let Some(variant) = &self.variant {
                object.insert(
                    "variant".to_string(),
                    Value::String(bound_text_metadata(variant)),
                );
            }
        }
        entry.data = Some(data);
        entry.outcome = self.outcome_word();
        entry
    }

    /// The one `question` entry a reported wait produces: the wait is its own
    /// span, never merged into the turn around it, and its label is the
    /// question's first line only. The complete question text is captured in
    /// `data.question` in every mode — the one content value always kept — and
    /// the intake re-applies its character bound.
    fn question_entry(&self, scope: &Scope, question: &str, mode: RecordMode) -> HistoryEntry {
        let mut entry = HistoryEntry::reported(scope.clone(), "question", "question", &self.at);
        entry.label = Some(reported_label("question", question));
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        let mut data = serde_json::Map::new();
        if let Some(question) = &self.question {
            data.insert(
                "question".to_string(),
                serde_json::json!(bound_question(question)),
            );
        }
        if let Some(title) = &self.title {
            data.insert(
                "title".to_string(),
                Value::String(bound_text_metadata(title)),
            );
        }
        if let Some(selected) = &self.selected {
            data.insert(
                "selected".to_string(),
                Value::String(bound_text_metadata(selected)),
            );
        }
        if let Some(source) = &self.answered_by {
            data.insert(
                "source".to_string(),
                Value::String(bound_text_metadata(source)),
            );
        }
        if let Some(at) = &self.answered_at {
            data.insert("answered_at".to_string(), Value::String(at.clone()));
        }
        if let Some(answer) = &self.answer {
            data.insert("answer".to_string(), serde_json::json!(answer));
        }
        if !data.is_empty() {
            entry.data = Some(Value::Object(data));
        }
        self.attach_content(&mut entry, mode);
        entry
    }

    /// The one `phase` entry a reported phase produces: a harness's own long
    /// step (compaction), named by its label with its measured duration.
    fn phase_entry(&self, scope: &Scope, phase: &str, mode: RecordMode) -> HistoryEntry {
        let mut entry = HistoryEntry::reported(scope.clone(), "phase", "phase", &self.at);
        entry.label = Some(reported_label("phase", phase));
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        self.attach_content(&mut entry, mode);
        entry
    }

    /// The one `tool` entry a reported tool call produces: the harness tool
    /// name as the entry's name, the reported subject through the record's one
    /// bounded-label path, and the harness's own measured duration.
    fn tool_entry(&self, scope: &Scope, tool: &str, mode: RecordMode) -> HistoryEntry {
        let mut entry = HistoryEntry::reported(scope.clone(), "tool", tool, &self.at);
        entry.label = self
            .label
            .as_deref()
            .map(|label| reported_label("tool", label));
        entry.duration_ms = self.duration_ms;
        entry.outcome = self.outcome_word();
        let mut data = serde_json::Map::new();
        if let Some(id) = &self.tool_call_id {
            data.insert("tool_call_id".to_string(), Value::String(id.clone()));
        }
        if let Some(error) = &self.error {
            data.insert("error".to_string(), Value::String(bound_error(error)));
        }
        if let Some(bytes) = self.output_bytes {
            data.insert("output_bytes".to_string(), Value::from(bytes));
        }
        if let Some(meta) = &self.result_meta {
            data.insert("result_meta".to_string(), bound_result_meta(meta));
        }
        if let Some(source) = &self.duration_source {
            data.insert("duration_source".to_string(), Value::String(source.clone()));
        }
        if let Some(title) = &self.title {
            data.insert(
                "title".to_string(),
                Value::String(bound_text_metadata(title)),
            );
        }
        if !data.is_empty() {
            entry.data = Some(Value::Object(data));
        }
        self.attach_content(&mut entry, mode);
        entry
    }

    /// Attach the reporter's complete content set to its entry when the
    /// effective mode is `full`. The intake is the single enforcement site for
    /// the content caps; under every other mode the content is dropped while
    /// the event is kept, so a stale reporter cannot bloat or leak the store.
    fn attach_content(&self, entry: &mut HistoryEntry, mode: RecordMode) {
        if mode != RecordMode::Full {
            return;
        }
        let Some(content) = self.content.as_ref().and_then(bound_content) else {
            return;
        };
        match entry.data.as_mut().and_then(Value::as_object_mut) {
            Some(data) => {
                data.insert("content".to_string(), content);
            }
            None => entry.data = Some(serde_json::json!({ "content": content })),
        }
    }

    /// One reported subagent span: a completion closes the start this payload
    /// reported (or stands alone when the harness measured the span itself),
    /// and an unclosed start still becomes one unmeasured entry — losing a
    /// duration never loses an entry.
    fn map_subagent(
        &self,
        scope: &Scope,
        identity: &str,
        batch: &mut IntakeBatch,
        mode: RecordMode,
    ) {
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
        if let Some(title) = &self.title
            && let Some(data) = entry.data.as_mut().and_then(Value::as_object_mut)
        {
            data.insert(
                "title".to_string(),
                Value::String(bound_text_metadata(title)),
            );
        }
        if self.usage.is_none() {
            // Gap honesty: a harness that never reports a subagent's tokens
            // marks the span's usage unavailable, never zero.
            if let Some(data) = entry.data.as_mut().and_then(Value::as_object_mut) {
                data.insert(
                    "usage".to_string(),
                    Value::String(USAGE_UNAVAILABLE.to_string()),
                );
            }
        }
        self.attach_content(&mut entry, mode);
        // The span is nested in the turn that reported it, when the payload
        // reported one, else in the turn its start was reported inside.
        let parent = start.and_then(|start| start.parent).or(batch.turn);
        let subagent = batch.push(entry, parent);
        // A subagent whose own transcript reported tokens contributes one
        // usage entry beside its span, parented to it: the report's per
        // subagent totals read this parent link, never a session-wide guess.
        if let Some(usage) = &self.usage {
            batch.push(self.usage_entry(scope, usage), Some(subagent));
        }
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
/// mapper reads (the Claude Code payload's own field names).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct HarnessPayload {
    /// The harness event this payload reports (`PreToolUse`, `PostToolUse`,
    /// `Stop`, `SessionEnd`, …).
    #[serde(default)]
    pub hook_event_name: Option<String>,
    /// The harness's own session id (the pending-state file key).
    #[serde(default)]
    pub session_id: Option<String>,
    /// The harness tool name, on tool events.
    #[serde(default)]
    pub tool_name: Option<String>,
    /// The tool's own input, on tool events.
    #[serde(default)]
    pub tool_input: Option<Value>,
    /// The harness's own call id, on tool events.
    #[serde(default)]
    pub tool_use_id: Option<String>,
    /// The failed call's error text, on `PostToolUseFailure`.
    #[serde(default)]
    pub error: Option<String>,
    /// The tool's own response, on `PostToolUse`: its size is the recorded
    /// output size.
    #[serde(default)]
    pub tool_response: Option<Value>,
    /// The harness's own measured duration, on completion events.
    #[serde(default)]
    pub duration_ms: Option<u64>,
    /// The notification type, on `Notification`.
    #[serde(default)]
    pub notification_type: Option<String>,
    /// The session record the harness writes asynchronously, on `Stop`.
    #[serde(default)]
    pub transcript_path: Option<PathBuf>,
    /// The subagent's own run id, on subagent events.
    #[serde(default)]
    pub agent_id: Option<String>,
    /// The agent definition that ran the subagent.
    #[serde(default)]
    pub agent_type: Option<String>,
    /// The subagent's own session record, on `SubagentStop`.
    #[serde(default)]
    pub agent_transcript_path: Option<PathBuf>,
    /// The turn's final assistant message, on `Stop`: content the reporter
    /// stores only when the project captures content.
    #[serde(default)]
    pub last_assistant_message: Option<String>,
}

impl HarnessPayload {
    /// The event this payload reports, empty when it names none.
    fn event(&self) -> &str {
        self.hook_event_name.as_deref().unwrap_or_default()
    }
}

/// The Claude Code question tool: its own PreToolUse/PostToolUse pair is the
/// question wait, and its payload carries the question text.
const QUESTION_TOOL: &str = "AskUserQuestion";

/// The pending key a permission wait opens when the harness names no tool (a
/// `Notification` permission prompt).
const PERMISSION_WAIT_KEY: &str = "permission";

/// The pending key an idle prompt opens.
const IDLE_WAIT_KEY: &str = "idle";

/// The pending-state kind a tool start produces.
const PENDING_KIND_TOOL: &str = "tool";

/// The pending-state kind a wait start produces.
const PENDING_KIND_QUESTION: &str = "question";

/// The pending key a subagent start opens.
fn subagent_key(identity: &str) -> String {
    format!("subagent:{identity}")
}

/// The pending key a permission request opens: the harness names the tool, not
/// the call, so the tool name is the harness's own identity for that wait.
fn permission_key(tool: &str) -> String {
    format!("permission:{tool}")
}

/// The measured span between a pending start and `now`, or `None` when the
/// start's stamp is unreadable — an unmeasured wait is never invented.
fn measured_from(start: Option<&PendingStart>, now_ms: i64) -> Option<u64> {
    start
        .and_then(|start| parse_rfc3339_ms(&start.started_at))
        .and_then(|at| u64::try_from(now_ms.saturating_sub(at)).ok())
}

/// The bounded subject label of a tool call from the harness's own input: the
/// command for a shell call, the path for a file call, the tool name
/// otherwise. The complete input is never stored (D2).
fn tool_subject(tool: &str, input: Option<&Value>) -> String {
    let input = input.and_then(Value::as_object);
    let subject = input.and_then(|input| {
        ["command", "file_path", "path", "pattern", "query"]
            .into_iter()
            .find_map(|key| input.get(key).and_then(Value::as_str))
    });
    crate::history::bound_label(subject.unwrap_or(tool))
}

/// The question text of an `AskUserQuestion` call, as the record's bounded
/// label path keeps it (the first line only).
fn question_subject(input: Option<&Value>) -> Option<String> {
    let question = input
        .and_then(Value::as_object)
        .and_then(|input| input.get("questions"))
        .and_then(Value::as_array)
        .and_then(|questions| questions.first())
        .and_then(|first| first.get("question"))
        .and_then(Value::as_str)?;
    Some(reported_label("question", question))
}

/// Every question and option label of a question tool's input, joined: the
/// always-captured text the record keeps in `data.question`. `None` when the
/// payload carries no question text at all.
fn question_text(input: Option<&Value>) -> Option<String> {
    let questions = input
        .and_then(Value::as_object)
        .and_then(|input| input.get("questions"))
        .and_then(Value::as_array)?;
    let mut parts: Vec<String> = Vec::new();
    for question in questions {
        if let Some(text) = question.get("question").and_then(Value::as_str) {
            parts.push(text.to_string());
        }
        if let Some(options) = question.get("options").and_then(Value::as_array) {
            for option in options {
                let label = option
                    .get("label")
                    .and_then(Value::as_str)
                    .or_else(|| option.as_str());
                if let Some(label) = label {
                    parts.push(label.to_string());
                }
            }
        }
    }
    (!parts.is_empty()).then(|| parts.join("\n"))
}

/// The question text as the wire shape the intake accepts: `{text, truncated,
/// bytes}` with the reporter's own measurement (the intake re-caps it).
fn reported_question(input: Option<&Value>) -> Option<BoundedText> {
    question_text(input).map(|text| BoundedText {
        bytes: text.len(),
        truncated: false,
        text,
    })
}

/// The byte size of a tool response: a string's own bytes, otherwise the
/// response's serialized size — what the harness reported as output.
fn tool_response_bytes(response: &Value) -> u64 {
    match response {
        Value::String(text) => text.len() as u64,
        other => serde_json::to_string(other)
            .map(|text| text.len() as u64)
            .unwrap_or(0),
    }
}

/// A harness value as one content field: a string as-is, anything else
/// serialized. The intake re-applies the content caps.
fn reported_content(value: &Value) -> BoundedText {
    let text = match value {
        Value::String(text) => text.clone(),
        other => serde_json::to_string(other).unwrap_or_default(),
    };
    BoundedText {
        bytes: text.len(),
        truncated: false,
        text,
    }
}

/// One reported event from the mapper's own reading of the harness's payload.
fn reported_event(kind: &str, at: &str) -> IngestedEvent {
    IngestedEvent {
        kind: kind.to_string(),
        at: at.to_string(),
        tool: None,
        label: None,
        duration_ms: None,
        outcome: None,
        status: None,
        agent: None,
        subagent: None,
        turn: None,
        model: None,
        usage: None,
        selected: None,
        answered_by: None,
        answered_at: None,
        answer: None,
        provider: None,
        variant: None,
        stop_reason: None,
        cost: None,
        list_cost: None,
        context_size: None,
        context_window: None,
        tool_call_id: None,
        error: None,
        output_bytes: None,
        result_meta: None,
        duration_source: None,
        title: None,
        question: None,
        content: None,
    }
}

/// Raw harness payload → record events plus pending-state updates; the Claude
/// mapper's only entry point, also usable by a future shell-based harness.
///
/// Every mapping step is best-effort: a payload shape the mapper cannot read
/// contributes no event, and the caller still exits `0`, because a non-zero
/// hook exit is a hook error for the harness.
pub fn map_harness_event(
    payload: &HarnessPayload,
    pending: &mut PendingState,
    mode: RecordMode,
) -> Result<Vec<IngestedEvent>> {
    Ok(match payload.event() {
        "PreToolUse" => {
            map_pre_tool_use(payload, pending);
            Vec::new()
        }
        "PostToolUse" | "PostToolUseFailure" => map_tool_end(payload, pending, mode),
        "PermissionRequest" => {
            map_permission_request(payload, pending);
            Vec::new()
        }
        "Notification" => {
            map_notification(payload, pending);
            Vec::new()
        }
        "SubagentStart" => {
            map_subagent_start(payload, pending);
            Vec::new()
        }
        "SubagentStop" => map_subagent_stop(payload, pending),
        "Stop" => map_turn_end(payload, mode),
        "SessionEnd" => {
            // Session end prunes the whole session's pending state: a start
            // whose end never arrived costs a duration, never an entry.
            pending.clear();
            Vec::new()
        }
        _ => Vec::new(),
    })
}

/// The tool's own start: `PreToolUse` carries no timestamp and no duration, so
/// the reporter's own stamp is the only start the tool span can have. An
/// `AskUserQuestion` start is a question wait instead of a tool span.
fn map_pre_tool_use(payload: &HarnessPayload, pending: &mut PendingState) {
    let (Some(tool), Some(call)) = (payload.tool_name.as_deref(), payload.tool_use_id.as_deref())
    else {
        return;
    };
    let (kind, label) = if tool == QUESTION_TOOL {
        (
            PENDING_KIND_QUESTION,
            question_subject(payload.tool_input.as_ref()),
        )
    } else {
        (
            PENDING_KIND_TOOL,
            Some(tool_subject(tool, payload.tool_input.as_ref())),
        )
    };
    pending.open(
        call,
        PendingStart {
            started_at: crate::history::now_rfc3339(),
            kind: kind.to_string(),
            label,
        },
    );
}

/// The tool's own end: the harness's measured `duration_ms` where it reports
/// one, the reporter's stamped start otherwise — and the permission or idle
/// wait the same completion closes, whose duration the harness's own
/// `duration_ms` excludes.
fn map_tool_end(
    payload: &HarnessPayload,
    pending: &mut PendingState,
    mode: RecordMode,
) -> Vec<IngestedEvent> {
    let at = crate::history::now_rfc3339();
    let now = crate::history::now_ms();
    let tool = payload
        .tool_name
        .clone()
        .unwrap_or_else(|| "tool".to_string());
    let failed = payload.event() == "PostToolUseFailure";
    let start = payload
        .tool_use_id
        .as_deref()
        .and_then(|call| pending.take(call));
    let wait = pending.take_first(&[
        permission_key(&tool),
        PERMISSION_WAIT_KEY.to_string(),
        IDLE_WAIT_KEY.to_string(),
    ]);
    let mut events = Vec::new();
    if start
        .as_ref()
        .is_some_and(|start| start.kind == PENDING_KIND_QUESTION)
    {
        let mut question = reported_event("question", &at);
        question.label = Some(
            start
                .as_ref()
                .and_then(|start| start.label.clone())
                .unwrap_or_else(|| QUESTION_TOOL.to_string()),
        );
        question.duration_ms = payload
            .duration_ms
            .or_else(|| measured_from(start.as_ref(), now));
        question.question = reported_question(payload.tool_input.as_ref());
        question.outcome = Some(if failed { "failed" } else { "ok" }.to_string());
        events.push(question);
    } else {
        let mut tool_event = reported_event("tool", &at);
        tool_event.tool = Some(tool.clone());
        tool_event.tool_call_id = payload.tool_use_id.clone();
        tool_event.label = Some(
            start
                .as_ref()
                .and_then(|start| start.label.clone())
                .unwrap_or_else(|| tool_subject(&tool, payload.tool_input.as_ref())),
        );
        let host_duration = payload.duration_ms;
        tool_event.duration_ms = host_duration.or_else(|| measured_from(start.as_ref(), now));
        tool_event.duration_source = tool_event.duration_ms.map(|_| {
            if host_duration.is_some() {
                "host"
            } else {
                "reporter"
            }
            .to_string()
        });
        tool_event.error = if failed { payload.error.clone() } else { None };
        tool_event.output_bytes = payload.tool_response.as_ref().map(tool_response_bytes);
        tool_event.outcome = Some(if failed { "failed" } else { "ok" }.to_string());
        if mode == RecordMode::Full {
            let mut content = BTreeMap::new();
            if let Some(input) = &payload.tool_input {
                content.insert("tool_input".to_string(), reported_content(input));
            }
            if let Some(response) = &payload.tool_response {
                content.insert("tool_output".to_string(), reported_content(response));
            }
            if !content.is_empty() {
                tool_event.content = Some(content);
            }
        }
        events.push(tool_event);
    }
    if let Some(wait) = wait {
        // The wait the harness does not time: `duration_ms` excludes time spent
        // in permission prompts, so the reporter's own stamps are the
        // measurement.
        let mut question = reported_event("question", &at);
        question.label = wait.label.clone().or_else(|| Some(tool.clone()));
        question.duration_ms = measured_from(Some(&wait), now);
        question.outcome = Some("ok".to_string());
        events.push(question);
    }
    events
}

/// A permission request: the harness reports the tool (and its input), never a
/// call id or a timestamp, so the wait opens under the tool's own name.
fn map_permission_request(payload: &HarnessPayload, pending: &mut PendingState) {
    let key = payload
        .tool_name
        .as_deref()
        .map(permission_key)
        .unwrap_or_else(|| PERMISSION_WAIT_KEY.to_string());
    let label = match payload.tool_name.as_deref() {
        Some(tool) => format!("permission request: {tool}"),
        None => "permission request".to_string(),
    };
    pending.open(
        &key,
        PendingStart {
            started_at: crate::history::now_rfc3339(),
            kind: PENDING_KIND_QUESTION.to_string(),
            label: Some(label),
        },
    );
}

/// A notification: a permission prompt or an idle prompt opens the wait the
/// next completion closes.
fn map_notification(payload: &HarnessPayload, pending: &mut PendingState) {
    let (key, label) = match payload.notification_type.as_deref() {
        Some("permission_prompt") => (PERMISSION_WAIT_KEY, "permission request"),
        Some("idle_prompt") => (IDLE_WAIT_KEY, "idle wait"),
        _ => return,
    };
    pending.open(
        key,
        PendingStart {
            started_at: crate::history::now_rfc3339(),
            kind: PENDING_KIND_QUESTION.to_string(),
            label: Some(label.to_string()),
        },
    );
}

/// A subagent's start: the harness's own run id opens the span, and the
/// agent definition that ran it is kept for the completion.
fn map_subagent_start(payload: &HarnessPayload, pending: &mut PendingState) {
    let Some(identity) = payload.agent_id.as_deref() else {
        return;
    };
    pending.open(
        &subagent_key(identity),
        PendingStart {
            started_at: crate::history::now_rfc3339(),
            kind: "subagent".to_string(),
            label: payload.agent_type.clone(),
        },
    );
}

/// A subagent's completion: one span entry, measured from the reporter's own
/// start stamp, with the subagent's tokens read from its own session record.
fn map_subagent_stop(payload: &HarnessPayload, pending: &mut PendingState) -> Vec<IngestedEvent> {
    let identity = match payload.agent_id.clone() {
        Some(identity) => identity,
        None => match payload.agent_type.clone() {
            Some(agent) => agent,
            None => return Vec::new(),
        },
    };
    let start = pending.take(&subagent_key(&identity));
    let usage = session_record_usage(payload.agent_transcript_path.as_deref());
    let mut event = reported_event("subagent", &crate::history::now_rfc3339());
    event.subagent = Some(identity);
    event.agent = payload.agent_type.clone();
    event.status = Some("completed".to_string());
    event.duration_ms = measured_from(start.as_ref(), crate::history::now_ms());
    event.usage = usage;
    vec![event]
}

/// The turn boundary: the harness's session record is written asynchronously
/// and may lag the live turn, so its usage is read here, at the turn's end —
/// and a turn whose usage the record does not report is still recorded, with
/// its usage marked unavailable.
fn map_turn_end(payload: &HarnessPayload, mode: RecordMode) -> Vec<IngestedEvent> {
    let usage = session_record_usage(payload.transcript_path.as_deref());
    let cost = session_record_cost(payload.transcript_path.as_deref());
    let mut event = reported_event("turn", &crate::history::now_rfc3339());
    event.status = Some("completed".to_string());
    event.model = usage.as_ref().and_then(|usage| usage.model.clone());
    event.usage = usage;
    event.cost = cost;
    if mode == RecordMode::Full
        && let Some(message) = &payload.last_assistant_message
    {
        event.content = Some(BTreeMap::from([(
            "assistant_text".to_string(),
            BoundedText {
                text: message.clone(),
                truncated: false,
                bytes: message.len(),
            },
        )]));
    }
    vec![event]
}

/// The session cost rollup one harness session record reports: the last
/// `cost-state` entry's `totalCostUSD`, kept as the number the harness wrote.
/// `None` when the record carries no rollup.
fn session_record_cost(path: Option<&Path>) -> Option<Value> {
    let text = std::fs::read_to_string(path?).ok()?;
    let mut cost = None;
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let Ok(record) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if record.get("type").and_then(Value::as_str) != Some("cost-state") {
            continue;
        }
        if let Some(total) = record.get("totalCostUSD") {
            cost = Some(total.clone());
        }
    }
    cost
}

/// The token usage one harness session record reports for its current turn:
/// the record's one reader, shared by the turn path (`transcript_path`) and the
/// subagent path (`agent_transcript_path`). `None` — never a zero — when the
/// path is absent, the record is missing or unreadable, or it reports no usage
/// at all.
fn session_record_usage(path: Option<&Path>) -> Option<Usage> {
    let text = std::fs::read_to_string(path?).ok()?;
    let mut usage = SessionUsage::default();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        // Defensive parsing: one malformed line never costs the record the rest
        // of the transcript.
        let Ok(record) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        match record.get("type").and_then(Value::as_str) {
            Some("user") if is_prompt(&record) => usage = SessionUsage::default(),
            Some("assistant") => usage.add(&record),
            _ => {}
        }
    }
    usage.into_usage()
}

/// Whether a transcript `user` entry is the prompt that opens a turn. A tool
/// result is a `user` entry too, and never opens one.
fn is_prompt(record: &Value) -> bool {
    record
        .get("message")
        .and_then(|message| message.get("content"))
        .and_then(Value::as_array)
        .is_some_and(|content| {
            content.iter().any(|part| {
                part.get("type").and_then(Value::as_str) == Some("text")
                    && part.get("text").and_then(Value::as_str).is_some()
            })
        })
}

/// The usage one session record's assistant entries report, summed while the
/// record is read.
#[derive(Debug, Default)]
struct SessionUsage {
    input: u64,
    output: u64,
    reasoning: u64,
    cache_read: u64,
    cache_write: u64,
    model: Option<String>,
    counted: bool,
}

impl SessionUsage {
    /// Add one assistant entry's usage: every count the entry reports, and the
    /// model that produced it — kept verbatim, never normalised.
    fn add(&mut self, record: &Value) {
        let Some(message) = record.get("message") else {
            return;
        };
        if let Some(model) = message.get("model").and_then(Value::as_str) {
            self.model = Some(model.to_string());
        }
        let Some(usage) = message.get("usage") else {
            return;
        };
        let count = |key: &str| usage.get(key).and_then(Value::as_u64).unwrap_or_default();
        let input = count("input_tokens");
        let output = count("output_tokens");
        let reasoning = usage
            .get("output_tokens_details")
            .and_then(|details| details.get("thinking_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or_default();
        let cache_read = count("cache_read_input_tokens");
        let cache_write = count("cache_creation_input_tokens");
        if input + output + reasoning + cache_read + cache_write == 0 {
            return;
        }
        self.input += input;
        self.output += output;
        self.reasoning += reasoning;
        self.cache_read += cache_read;
        self.cache_write += cache_write;
        self.counted = true;
    }

    /// The intake's usage object, or `None` when no entry reported a count.
    fn into_usage(self) -> Option<Usage> {
        if !self.counted {
            return None;
        }
        Some(Usage {
            input: Some(self.input),
            output: Some(self.output),
            reasoning: (self.reasoning > 0).then_some(self.reasoning),
            cache_read: Some(self.cache_read),
            cache_write: Some(self.cache_write),
            total_tokens: Some(
                self.input
                    .saturating_add(self.output)
                    .saturating_add(self.cache_read)
                    .saturating_add(self.cache_write),
            ),
            model: self.model,
            subagent: None,
        })
    }
}

/// One process-per-hook session's pending state: the file the harness's own
/// session id names, read once per invocation, and the one write path every
/// hook shares — prune by age, then one atomic store, so no caller can forget
/// either step.
struct PendingSession {
    path: PathBuf,
    state: PendingState,
}

impl PendingSession {
    /// Read the session's state. A missing or unreadable file is an empty
    /// state: losing it costs a duration, never an entry.
    fn read(root: &Path, harness: &str, session_id: &str) -> Self {
        let path = crate::history::pending_path(root, harness, session_id);
        let state = PendingState::load(&path);
        PendingSession { path, state }
    }

    /// The starts this invocation maps against.
    fn state(&mut self) -> &mut PendingState {
        &mut self.state
    }

    /// Write the state back: the record's one pruning rule (by age) and one
    /// atomic store, which removes the file when nothing is open.
    fn write(mut self) {
        self.state.prune(crate::history::now_ms());
        if let Err(error) = self.state.store(&self.path) {
            eprintln!(
                "warning: the pending state could not be written to {}: {error}",
                self.path.display()
            );
        }
    }
}

/// `givn history harness-hook --harness <name>` — map one raw harness payload
/// from stdin into the record (the process-per-hook entry point).
///
/// Best-effort by construction: every mapping step is contained and the command
/// always exits `0`, because a non-zero hook exit is a hook error for the
/// harness and exit `2` blocks the tool.
pub fn harness_hook(harness: &str) -> Result<()> {
    let Ok(root) = std::env::current_dir() else {
        return Ok(());
    };
    let mut text = String::new();
    if std::io::stdin().read_to_string(&mut text).is_err() {
        return Ok(());
    }
    let payload: HarnessPayload = match serde_json::from_str(&text) {
        Ok(payload) => payload,
        Err(error) => {
            eprintln!("warning: the {harness} hook payload could not be read: {error}");
            return Ok(());
        }
    };
    // The mode is resolved in-process, before any pending state is read or
    // written: under `off` the hook exits `0` with one warning and never
    // creates or extends a pending file, and never appends a store line.
    let setting = crate::history::record_setting(&root);
    if setting.mode == RecordMode::Off {
        crate::history::warn_record_off();
        return Ok(());
    }
    let mut session = payload
        .session_id
        .as_deref()
        .map(|session| PendingSession::read(&root, harness, session));
    let mut sessionless = PendingState::default();
    let pending = match session.as_mut() {
        Some(session) => session.state(),
        None => &mut sessionless,
    };
    let events = match map_harness_event(&payload, pending, setting.mode) {
        Ok(events) => events,
        Err(error) => {
            eprintln!("warning: the {harness} hook payload could not be mapped: {error:#}");
            return Ok(());
        }
    };
    if let Some(session) = session {
        session.write();
    }
    if events.is_empty() {
        return Ok(());
    }
    let scope = crate::history::resolve_scope(&root);
    let session_id = payload.session_id.clone();
    let mut batch = IntakeBatch::default();
    for event in events {
        event.map(&scope, &mut batch, setting.mode);
    }
    batch.finish(&scope);
    for entry in &mut batch.entries {
        entry.session = session_id.clone();
    }
    crate::history::ensure_ignored(&root);
    let store = HistoryStore::at(store_path(&root));
    if let Err(error) = store.append_linked(&mut batch.entries, &batch.parents) {
        crate::history::warn_unwritten(&store, &error);
    }
    Ok(())
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

/// `givn history settings [--json]` — report the project's resolved record mode
/// and where it came from. A non-recorded read view (the recorder is not
/// installed for it), so a reporter's mode query never appends a `command`
/// entry.
///
/// Human output is `record mode: <mode> (source: <config|default>)`; `--json`
/// prints `{"mode": …, "source": …}`. Both exit `0`. Outside a Givn project the
/// command fails with `not a Givn project (givn/ not found)`.
pub fn settings(json: bool) -> Result<()> {
    let root = std::env::current_dir().context("resolve the project root")?;
    if !root.join("givn").is_dir() {
        anyhow::bail!("not a Givn project (givn/ not found)");
    }
    let setting = crate::history::record_setting(&root);
    if json {
        println!(
            "{}",
            serde_json::json!({
                "mode": setting.mode.label(),
                "source": setting.source.label(),
            })
        );
    } else {
        println!(
            "record mode: {} (source: {})",
            setting.mode.label(),
            setting.source.label()
        );
        println!(
            "record storage: givn/history/ is a local, gitignored recording rather than repository history by default"
        );
        println!(
            "record content: recordings can contain sensitive tool input, prompts, and output; Givn makes no redaction, privacy-boundary, or complete-capture guarantee"
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
pub fn ingest(with_full_content: bool) -> Result<IntakeOutcome> {
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
    // The project's durable mode is the control; `off` is absolute. The
    // intake appends nothing, writes no pending state, and touches no
    // record-side ignore entry — even for a payload carrying content and even
    // with the one-shot `--with-full-content` override, which is ignored with
    // this one warning.
    let setting = crate::history::record_setting(&root);
    if setting.mode == RecordMode::Off {
        crate::history::warn_record_off();
        return Ok(IntakeOutcome::Read);
    }
    // The per-invocation override (tests and manual capture) raises this one
    // payload to `full`; otherwise the project's resolved mode applies.
    let mode = if with_full_content {
        RecordMode::Full
    } else {
        setting.mode
    };
    // The scope every accepted event is recorded under: the payload's named
    // override (a scope this project does not have is a per-event rejection,
    // never a silent fallback) or the project's own resolution.
    let session = payload.session.clone();
    let scope = payload_scope(&root, &payload);
    // The intake's topic override: a payload that reports topic work names the
    // topic the marker observation covers, even when this process's
    // start-resolved scope is the project or a change. The command's own entry
    // keeps the start-resolved scope; only the marker half reads the override.
    if let (Some(topic), Ok(Scope::Ideation { .. })) = (payload.topic.as_deref(), &scope)
        && let Some(recorder) = crate::history::current()
    {
        recorder.observe_topic(topic);
    }
    let IntakeBatch {
        mut entries,
        parents,
        ..
    } = accepted_entries(&scope, payload.events, mode);
    // The payload header is the session every entry of this flush belongs to.
    for entry in &mut entries {
        entry.session = session.clone();
    }
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
fn accepted_entries(
    scope: &std::result::Result<Scope, String>,
    events: Vec<Value>,
    mode: RecordMode,
) -> IntakeBatch {
    let mut batch = IntakeBatch::default();
    for (index, raw) in events.into_iter().enumerate() {
        let kind = raw.get("kind").and_then(Value::as_str).map(str::to_owned);
        if let Err(reason) = accept_event(scope, raw, &mut batch, mode) {
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
/// path without them. The effective mode is applied to the event's content at
/// the same one site.
fn accept_event(
    scope: &std::result::Result<Scope, String>,
    raw: Value,
    batch: &mut IntakeBatch,
    mode: RecordMode,
) -> std::result::Result<(), String> {
    let scope = scope.as_ref().map_err(Clone::clone)?;
    let event: IngestedEvent = serde_json::from_value(raw).map_err(|error| error.to_string())?;
    event.validate()?;
    event.map(scope, batch, mode);
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
