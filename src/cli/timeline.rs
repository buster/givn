//! `givn timeline`: the development report — the model the report screen
//! renders and the self-contained HTML document it is rendered into.
//!
//! Like `givn history`, this command runs without a recorder: rendering a
//! report must never extend the record it renders.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::history::{HistoryEntry, Scope, USAGE_UNAVAILABLE};

/// The asset's view marker: the model rendered as the page's own text — the
/// scope line and the summary. The generator fills it, so the written file
/// carries the report's rendered text and needs no script to be read.
const VIEW_SLOT: &str = "<!--__GIVN_REPORT_VIEW__-->";

/// The asset's single data slot: the generator replaces it with the JSON
/// model, so the document stays one self-contained file and the inline script
/// works from the model the text was rendered from.
const DATA_SLOT: &str = "/*__GIVN_REPORT_DATA__*/";

/// The report asset, embedded so the binary needs no data directory.
const REPORT_ASSET: &str = include_str!("../../assets/timeline/report.html");

/// The recorded `phase` kind: one entry per artifact file whose appearance
/// marks a workflow stage's work as done, and per gate phase the archive and
/// check runs record.
const PHASE: &str = "phase";

/// The recorded `question` kind: a wait for an answer, its own recorded span.
const QUESTION: &str = "question";

/// The recorded `turn` kind: one agent turn.
const TURN: &str = "turn";

/// The recorded `subagent` kind: one subagent span.
const SUBAGENT: &str = "subagent";

/// The recorded `usage` kind: the token usage one harness reported.
const USAGE: &str = "usage";

/// Whether a kind records a *measured span*, so an absent duration is a
/// duration the record does not have — never a point entry, which records an
/// instant (a minted revision, a reported usage, a session) and never had a
/// duration to measure.
fn records_span(kind: &str) -> bool {
    matches!(
        kind,
        "command" | "phase" | "hook" | "step" | "turn" | "tool" | "subagent" | "question"
    )
}

/// The documented total budget for full content embedded in one report
/// document, in bytes: the report embeds every entry's `data.content` in record
/// order until this cap, and the remaining content is not embedded (the store
/// keeps every byte). Applied in record order, so the same record always
/// produces the same report.
pub const CONTENT_BUDGET_BYTES: usize = 8 * 1024 * 1024;

/// The exact note a report states for an entry whose content the document
/// budget left out: the store keeps every byte, and the report says where to
/// find it.
pub const CONTENT_OMITTED_NOTE: &str = "content omitted; see the record";

/// Everything the report screen renders: scope, bands, entries, summary.
#[derive(Debug, Default, Serialize)]
pub struct ReportModel {
    /// The named scope this report covers; `None` renders the project report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The report's rows: one band per scope the window covers.
    pub bands: Vec<Band>,
    /// The record lines the report renders, in file order.
    pub entries: Vec<HistoryEntry>,
    /// Whether the requested scope holds no recorded activity at all. The
    /// report root renders its explicit empty state then — a heading, one
    /// sentence naming the state, and a way back to the report root — instead
    /// of a blank page, an error, or an axis drawn over nothing.
    pub empty: bool,
    /// The initial zoom's aggregate markers: the entries too short to draw one
    /// shape per entry, grouped one marker per band and time bucket. The
    /// summary and the longest-step ranking are computed over the whole record
    /// and never over these.
    pub aggregates: Vec<Aggregate>,
    /// The stage each entry worked in: the report's one phase derivation, read
    /// by the band rows' phase names, by every span's `data-stage`, and by the
    /// browser tier's phase placement assertion. A scope whose record carries
    /// no artifact transition has no name here — never an invented phase name.
    pub stages: Vec<StageAssignment>,
    /// The workflow stack: the change workflow's nine phases in order, each
    /// carrying its status and its recorded facts, for the change or ideation
    /// scope the report was asked for. Empty for the project report, which
    /// keeps its band rows.
    pub workflow: Vec<WorkflowPhase>,
    /// The change state embedded at generation, where the change has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<CurrentState>,
    /// The "no recorded stage transition" group: the same phase scaffold for
    /// the work the record assigns to no stage, absent when every entry has a
    /// stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstaged: Option<WorkflowPhase>,
    /// The linked ideation group: `Some` only for a change report whose
    /// proposal names an existing, recorded topic. Its entries are also in
    /// `entries`, once, and form their own ideation band.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideation: Option<IdeationGroup>,
    /// The summary: the numbers the report states about the whole window.
    pub summary: Summary,
    /// The documented total content budget, in bytes.
    pub content_budget_bytes: usize,
    /// The content bytes the document embeds.
    pub content_bytes: usize,
    /// The ids whose full content was left out of the document because the
    /// budget was exhausted; the panel states the omission instead of rendering
    /// a partial bundle. The store keeps every byte.
    pub omitted_content: Vec<u64>,
    /// The exact note the text view and the model state for an omitted entry.
    pub content_omitted_note: String,
}

/// One aggregate marker: the entries of one band and one time bucket that the
/// timeline collapses at the initial zoom, because drawing one shape per entry
/// would be unreadable. It names how many entries it covers and their combined
/// measured duration, and it carries the bucket's own bounds, so a click on it
/// can zoom to that bucket instead of opening a details panel for one entry
/// that stands for many.
#[derive(Debug, Serialize)]
pub struct Aggregate {
    /// The scope of the band the marker belongs to.
    pub scope: Scope,
    /// The bucket's first instant (inclusive), milliseconds since the Unix
    /// epoch — the same clock the records' `at` carries.
    pub bucket_start_ms: i64,
    /// The bucket's end (exclusive), milliseconds.
    pub bucket_end_ms: i64,
    /// The entries the marker stands for, in the model's own order — the ids
    /// of spans that are *not* drawn individually at the initial zoom.
    pub entries: Vec<u64>,
    /// The covered entries' combined measured duration; `None` when the record
    /// measured none of them, never zero.
    pub duration_ms: Option<u64>,
}

/// One band: a row of the report covering one scope — one archived change, one
/// change or ideation topic in flight, or the project's "outside a scope" band.
#[derive(Debug, Serialize)]
pub struct Band {
    /// The scope the band covers.
    pub scope: Scope,
    /// The band's scope class (`change`, `ideation`, `project`). The row's
    /// visible label is [`Band::label`], never this internal name.
    pub kind: String,
    /// The row's visible label, naming the scope's own identifier.
    pub label: String,
    /// The band's spans: the ids of its entries, in the model's own order.
    pub entries: Vec<u64>,
    /// The band's phases: the stages its entries worked in, in the workflow's
    /// own order, drawn as the row's visible phase names.
    pub stages: Vec<String>,
}

impl Band {
    /// The band of one scope over the given entry ids.
    fn of(scope: &Scope, entries: Vec<u64>) -> Self {
        let row = scope_row(scope);
        Band {
            scope: scope.clone(),
            kind: row.kind.to_string(),
            label: row.label,
            entries,
            stages: Vec::new(),
        }
    }
}

/// The ideation workflow's stages, in the order a topic works through them:
/// `frame` is the opening stage, and each later stage is started by its own
/// marker. A topic report's stack and an ideation band's phase names read this
/// order, never the change workflow's.
pub(crate) const IDEATION_STAGES: [&str; 6] = [
    "frame",
    "personas",
    "event-storming",
    "use-case-definition",
    "use-case-elaboration",
    "handoff",
];

/// The resolved link snapshot a change report carries: the ideation topic its
/// proposal names, and that topic's recorded entries. `run()` resolves it once
/// from the proposal (and validates the topic directory's existence); the model
/// embeds the topic id, so the written document is deterministic over the
/// record, the change state, and this snapshot.
#[derive(Debug, Clone)]
pub struct LinkedIdeation {
    /// The normalized topic id the change's proposal names.
    pub topic: String,
    /// The topic's own recorded entries.
    pub entries: Vec<HistoryEntry>,
}

/// The linked ideation group a change report renders above the change workflow:
/// the topic's six ideation phases with the topic report's marker statuses, the
/// group's own entry ids, and its own totals. It is historical — it never
/// becomes the change's current phase — and it joins the model's `entries` once.
#[derive(Debug, Serialize)]
pub struct IdeationGroup {
    /// The linked topic's id, as the link snapshot carries it.
    pub topic: String,
    /// Whether the topic's record holds any entry at all. A topic that
    /// predates the record states that instead of inventing phases.
    pub activity: bool,
    /// The six ideation phases with the topic report's marker statuses; empty
    /// when the record has no activity.
    pub phases: Vec<WorkflowPhase>,
    /// The group's entry ids, in the model's own order.
    pub entries: Vec<u64>,
    /// The group's own recorded duration: its first-to-last entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// The recorded `tool` entries the group holds.
    pub tools: usize,
    /// The recorded subagent spans the group holds.
    pub subagents: usize,
    /// The recorded question waits the group holds.
    pub waits: usize,
    /// The waits the group's record measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_ms: Option<u64>,
}

/// The summary the report states about the whole record, never about the
/// visible window alone: the entry count, the elapsed time, the durations the
/// record does not have, and the ranked longest steps.
#[derive(Debug, Default, Serialize)]
pub struct Summary {
    /// The number of entries the window holds.
    pub entries: usize,
    /// The window's elapsed time — its last recorded end minus its first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<u64>,
    /// The spanning entries whose duration the record does not carry. Missing
    /// data is stated, never summed as zero.
    pub unmeasured_durations: usize,
    /// Every entry the record measured, longest first — of any kind, so an
    /// agent tool call and a shell step compete on the time they took. The JSON
    /// field name stays `longest_steps` for the readers that already use it.
    pub longest_steps: Vec<RankedStep>,
    /// The recorded question waits the window holds. A wait the record does not
    /// time is still a wait.
    pub waits: usize,
    /// The waits the record does not time; the total never adds them as zero.
    pub unmeasured_waits: usize,
    /// The total of the waits the record measured, or `None` when it measured
    /// none of them.
    pub wait_ms: Option<u64>,
    /// The recorded subagent spans the window holds — the agent's fan-out. The
    /// count comes from the record's own `subagent` entries, never from a spawn
    /// relationship inferred out of label text.
    pub subagents: usize,
    /// The combined duration of the subagents the record measured, or `None`
    /// when it measured none of them. A subagent the record does not time is
    /// excluded from the total while still counting as a subagent.
    pub subagent_ms: Option<u64>,
    /// The subagent spans the record does not time; the combined duration never
    /// adds them as zero.
    pub unmeasured_subagents: usize,
    /// The tokens every reported usage adds up to, or `None` when the record
    /// reports no usage at all.
    pub tokens_total: Option<u64>,
    /// The tokens each model that produced usage accounts for.
    pub tokens_by_model: Vec<TokenTotal>,
    /// The tokens each recorded subagent accounts for; a subagent the harness
    /// reports no usage for reads "unavailable", never a zero.
    pub tokens_by_subagent: Vec<TokenTotal>,
    /// The turns whose token usage the harness never reported.
    pub unmeasured_turns: usize,
    /// Elapsed time minus waiting time when both are measured; `None` ("not
    /// measured") otherwise. The record's own two measured ends, never an
    /// invented active reading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_ms: Option<u64>,
    /// The number of recorded `tool` entries — the agent's tool calls.
    pub tool_calls: usize,
    /// The sum of every recorded real cost (`data.cost.total` for the cost
    /// object, `data.cost` for a number), or `None` ("unavailable") when the
    /// record carries none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_total: Option<f64>,
}

impl Summary {
    /// The summary over the report scope's own entries: one pass over those
    /// entries feeds every number it states, so no summary figure is ever a
    /// second, differently-shaped scan of the record. A linked topic's entries
    /// are not in that slice: its totals are the group's own.
    fn of(entries: &[&HistoryEntry]) -> Self {
        let mut summary = Summary {
            entries: entries.len(),
            ..Summary::default()
        };
        for entry in entries {
            if records_span(&entry.kind) && entry.duration_ms.is_none() {
                summary.unmeasured_durations += 1;
            }
            if entry.kind == "tool" {
                summary.tool_calls += 1;
            }
            if let Some(cost) = recorded_cost(entry) {
                *summary.cost_total.get_or_insert(0.0) += cost;
            }
            if entry.kind == QUESTION {
                summary.waits += 1;
                if !add_measured(&mut summary.wait_ms, entry.duration_ms) {
                    summary.unmeasured_waits += 1;
                }
            }
            if entry.kind == SUBAGENT {
                summary.subagents += 1;
                if !add_measured(&mut summary.subagent_ms, entry.duration_ms) {
                    summary.unmeasured_subagents += 1;
                }
            }
            if entry.kind == USAGE {
                add_measured(&mut summary.tokens_total, usage_total(entry));
            }
            if usage_unavailable(entry) {
                summary.unmeasured_turns += 1;
            }
        }
        summary.elapsed_ms = recorded_span(entries);
        summary.active_ms = summary
            .elapsed_ms
            .zip(summary.wait_ms)
            .map(|(elapsed, waiting)| elapsed.saturating_sub(waiting));
        summary.longest_steps = ranked_steps(entries);
        let tokens = TokenRollUp::of(entries);
        summary.tokens_by_model = tokens.model_rows();
        summary.tokens_by_subagent = tokens.subagent_rows();
        summary
    }
}

/// The window's elapsed time: its last recorded end minus its first. The window
/// is ordered by `at` before the summary reads it, so its ends are the window's
/// extremes and the elapsed time is never a second scan of the record; an end
/// the record cannot read leaves the elapsed time unmeasured rather than
/// guessed.
fn recorded_span(entries: &[&HistoryEntry]) -> Option<u64> {
    let first = crate::history::parse_rfc3339_ms(&entries.first()?.at)?;
    let last = crate::history::parse_rfc3339_ms(&entries.last()?.at)?;
    u64::try_from(last - first).ok()
}

/// One token total: the model or subagent it belongs to and the tokens the
/// record reports for it. `None` is an account the record reports no usage for:
/// the row renders "unavailable" rather than a zero.
#[derive(Debug, Serialize)]
pub struct TokenTotal {
    /// The model's or subagent's own name, as the record reports it.
    pub name: String,
    /// The tokens the record reports for this account.
    pub tokens: Option<u64>,
}

/// The window's usage roll-up: one total per model that produced usage, and one
/// row per recorded subagent. Both views are the same aggregation over the same
/// `usage` entries — grouped by the model an entry names, and by the subagent
/// span an entry belongs to — so the two views can never disagree about a
/// count, and a session-level usage is never split across subagents by
/// guesswork.
#[derive(Debug, Default)]
struct TokenRollUp {
    by_model: BTreeMap<String, u64>,
    by_subagent: BTreeMap<String, Option<u64>>,
}

impl TokenRollUp {
    /// The window's entries → both views.
    fn of(entries: &[&HistoryEntry]) -> Self {
        let usage: Vec<&HistoryEntry> = entries
            .iter()
            .copied()
            .filter(|entry| entry.kind == USAGE)
            .collect();
        let mut rollup = TokenRollUp::default();
        for entry in &usage {
            let Some(tokens) = usage_total(entry) else {
                continue;
            };
            if let Some(model) = usage_model(entry) {
                *rollup.by_model.entry(model.to_string()).or_default() += tokens;
            }
        }
        for span in entries.iter().filter(|entry| entry.kind == SUBAGENT) {
            let owned = usage
                .iter()
                .filter(|entry| usage_belongs_to(entry, span))
                .filter_map(|entry| usage_total(entry))
                .reduce(|total, tokens| total + tokens);
            match owned {
                Some(tokens) => {
                    let row = rollup
                        .by_subagent
                        .entry(span.name.clone())
                        .or_insert(Some(0));
                    *row = row.map(|total| total + tokens);
                }
                None => {
                    rollup.by_subagent.entry(span.name.clone()).or_insert(None);
                }
            }
        }
        rollup
    }

    /// The per-model view the summary carries, one row per model that produced
    /// usage.
    fn model_rows(&self) -> Vec<TokenTotal> {
        self.by_model
            .iter()
            .map(|(name, tokens)| TokenTotal {
                name: name.clone(),
                tokens: Some(*tokens),
            })
            .collect()
    }

    /// The per-subagent view the summary carries, one row per recorded
    /// subagent; a subagent the record reports no usage for keeps `None`.
    fn subagent_rows(&self) -> Vec<TokenTotal> {
        self.by_subagent
            .iter()
            .map(|(name, tokens)| TokenTotal {
                name: name.clone(),
                tokens: *tokens,
            })
            .collect()
    }
}

/// The tokens one `usage` entry reports (`data.total_tokens`), or `None` where
/// the record reports none — never read as a zero.
fn usage_total(entry: &HistoryEntry) -> Option<u64> {
    entry.data.as_ref()?.get("total_tokens")?.as_u64()
}

/// The model one `usage` entry names as the producer of its counts.
fn usage_model(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("model")?.as_str()
}

/// The subagent one `usage` entry names as the owner of its counts.
fn usage_named_subagent(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("subagent")?.as_str()
}

/// Whether one usage entry belongs to one subagent span: the entry is nested in
/// the span (the record's own parent link), or it names the span's identity.
fn usage_belongs_to(usage: &HistoryEntry, span: &HistoryEntry) -> bool {
    usage.parent == Some(span.id) || usage_named_subagent(usage) == Some(span.name.as_str())
}

/// Whether one entry is a turn whose token usage the harness never reported:
/// the record marks the turn's `data.usage` unavailable, so the report states
/// the gap instead of folding the turn in as zero.
fn usage_unavailable(entry: &HistoryEntry) -> bool {
    entry.kind == TURN
        && entry
            .data
            .as_ref()
            .and_then(|data| data.get("usage"))
            .and_then(|value| value.as_str())
            == Some(crate::history::USAGE_UNAVAILABLE)
}

/// The real cost one entry records: `data.cost.total` when the fact is the cost
/// object, `data.cost` when it is a number, and `None` for an entry the record
/// carries no cost for — never a zero.
fn recorded_cost(entry: &HistoryEntry) -> Option<f64> {
    match entry.data.as_ref()?.get("cost")? {
        serde_json::Value::Number(number) => number.as_f64(),
        serde_json::Value::Object(object) => object.get("total")?.as_f64(),
        _ => None,
    }
}

/// One ranked entry: the recorded entry, the kind that recorded it, and the
/// duration it was measured with. Every entry carrying a measured `duration_ms`
/// is ranked, whichever layer recorded it — a Givn command, a gate phase, a
/// hook run, a shell step, an agent turn, a tool call, a subagent span, or a
/// question wait — because the operator's question is what took the time, not
/// which subprocess took it.
#[derive(Debug, Serialize)]
pub struct RankedStep {
    /// The recorded entry's id.
    pub id: u64,
    /// The kind that recorded the entry (`command`, `phase`, `hook`, `step`,
    /// `revision`, `session`, `turn`, `tool`, `subagent`, `question`), so the
    /// row says which layer spent the time.
    pub kind: String,
    /// The entry's visible label (its bounded subject label, or its name).
    pub label: String,
    /// The measured duration the ranking orders by.
    pub duration_ms: u64,
}

/// One workflow phase of the report's stack, or the "no recorded stage
/// transition" group: the phase's recorded facts, read from the entries the one
/// derivation places in it. The scaffold holds ids, names, and totals only —
/// the entries themselves appear once in [`ReportModel::entries`].
#[derive(Debug, Default, Serialize)]
pub struct WorkflowPhase {
    /// The workflow stage's name; absent for the "no recorded stage transition"
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// `done`, `active`, `queued`, or `skipped`; absent where no live state
    /// derives one. The group carries no status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The phase's first recorded instant, milliseconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_ms: Option<i64>,
    /// The phase's last recorded instant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ms: Option<i64>,
    /// Wall time: the first to the last recorded entry assigned to the phase; a
    /// phase with no recorded entries records zero.
    pub wall_ms: Option<u64>,
    /// Wall time minus the question waits inside the phase, when both are
    /// measured.
    pub active_ms: Option<u64>,
    /// The question waits inside the phase; zero when the phase holds no wait,
    /// absent when it holds only waits the record does not time.
    pub waiting_ms: Option<u64>,
    /// The recorded `tool` entries in the phase.
    pub tools: usize,
    /// The recorded `subagent` spans in the phase.
    pub subagents: usize,
    /// One lane per distinct recorded session.
    pub lanes: Vec<Lane>,
    /// The phase's repeated-step groups.
    pub repeated: Vec<RepeatedGroup>,
    /// The phase's Fit-zoom aggregate buckets.
    pub aggregates: Vec<PhaseAggregate>,
}

/// One lane of a phase: a distinct recorded session and the entries it holds.
#[derive(Debug, Default, Serialize)]
pub struct Lane {
    /// The recorded session id; absent for the unattributed lane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// The lane's visible name.
    pub name: String,
    /// The lane's member entry ids, in the model's own order.
    pub entries: Vec<u64>,
    /// The lane's first recorded instant.
    pub first_ms: Option<i64>,
    /// The lane's last recorded instant.
    pub last_ms: Option<i64>,
    /// The lane's measured duration: its last instant minus its first.
    pub duration_ms: Option<u64>,
}

/// One repeated-step group: entries of one kind and one normalized label,
/// grouped when the group holds at least five entries.
#[derive(Debug, Default, Serialize)]
pub struct RepeatedGroup {
    /// The recorded kind the group's members share.
    pub kind: String,
    /// The normalized label (trimmed, whitespace collapsed).
    pub label: String,
    /// How many entries the group holds.
    pub count: usize,
    /// The members' combined measured duration; `None` when none was measured.
    pub duration_ms: Option<u64>,
    /// The member entry ids.
    pub entries: Vec<u64>,
}

/// One Fit-zoom aggregate of a phase: the phase's entries too short to draw
/// individually, bucketed by recorded time.
#[derive(Debug, Default, Serialize)]
pub struct PhaseAggregate {
    /// The bucket's first instant (inclusive), milliseconds since the epoch.
    pub bucket_start_ms: i64,
    /// The bucket's end (exclusive).
    pub bucket_end_ms: i64,
    /// The entries the bucket stands for.
    pub entries: Vec<u64>,
    /// How many entries the bucket holds.
    pub count: usize,
    /// The covered entries' combined measured duration.
    pub duration_ms: Option<u64>,
}

/// The change state embedded in the report at generation: the stage label, the
/// current workflow phase, the next required artifact, and the task progress
/// the change state exposes. Absent for a topic, the project report, and an
/// archived change.
#[derive(Debug, Default, Serialize)]
pub struct CurrentState {
    /// The change state's own stage label.
    pub stage_label: String,
    /// The workflow phase the state maps to; absent when the state's next
    /// required artifact is outside the mapping table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The next required artifact's id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_artifact: Option<String>,
    /// The task counts, where the change state has them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<TasksProgress>,
}

/// The task counts one change state exposes.
#[derive(Debug, Default, Serialize)]
pub struct TasksProgress {
    /// The tasks.md checkbox total.
    pub total: usize,
    /// The checked boxes.
    pub done: usize,
}

/// The report's workflow scaffold: the scope's workflow phases, the current
/// state, the no-stage group, and the linked ideation group.
#[derive(Debug, Default)]
struct PhaseScaffold {
    workflow: Vec<WorkflowPhase>,
    current_state: Option<CurrentState>,
    unstaged: Option<WorkflowPhase>,
    ideation: Option<IdeationGroup>,
}

/// The change's state as the report can resolve it.
enum ChangeResolution {
    /// A live change directory: its derived state.
    Live(Box<crate::state::ChangeState>),
    /// The change's directory is under `givn/archive/<id>`.
    Archived,
    /// Neither directory exists: the record names a change the project does not
    /// hold.
    Absent,
}

/// Resolve one change scope against the project root: `givn/changes/<id>`
/// existing is the live state, else `givn/archive/<id>` existing is an archived
/// change, else neither — never assume an archive for a missing directory.
fn change_resolution(scope: &Scope) -> ChangeResolution {
    let Scope::Change { id } = scope else {
        return ChangeResolution::Absent;
    };
    let Ok(root) = std::env::current_dir() else {
        return ChangeResolution::Absent;
    };
    let change_dir = root.join("givn").join("changes").join(id);
    if change_dir.is_dir() {
        let Ok(manifest) = crate::config::overlay::load_effective() else {
            return ChangeResolution::Absent;
        };
        return ChangeResolution::Live(Box::new(crate::state::ChangeState::from_manifest(
            &change_dir,
            &manifest,
        )));
    }
    if root.join("givn").join("archive").join(id).is_dir() {
        return ChangeResolution::Archived;
    }
    ChangeResolution::Absent
}

/// The workflow phase one change state is in: the state's `next_required()`
/// artifact mapped through the artifact-id table, `implement` while the tasks
/// artifact is `partial` (the stage `tasks.md`'s appearance opens), and
/// `archive` with every required artifact done. An artifact outside the table
/// leaves the phase underived instead of guessing one.
fn current_phase_index(state: &crate::state::ChangeState) -> Option<usize> {
    if state.all_required_done() {
        return WORKFLOW_STAGES.iter().position(|stage| *stage == "archive");
    }
    let next = state.next_required()?;
    if next.id == "tasks" && next.status == crate::state::ArtifactStatus::Partial {
        return WORKFLOW_STAGES
            .iter()
            .position(|stage| *stage == "implement");
    }
    let stage = ARTIFACT_STAGES.iter().find(|(id, _)| *id == next.id)?.1;
    WORKFLOW_STAGES.iter().position(|known| *known == stage)
}

/// The state artifact id that maps onto one workflow phase.
fn state_artifact_of(stage: &str) -> Option<&'static str> {
    ARTIFACT_STAGES
        .iter()
        .find(|(_, phase)| *phase == stage)
        .map(|(id, _)| *id)
}

/// One status per workflow phase, in [`WORKFLOW_STAGES`] order. Phases before
/// the current phase are `done`, the current phase is `active`, phases after it
/// are `queued`; a phase whose mapped artifact is skipped is `skipped` (which
/// wins over the positional statuses); an archived change renders every phase
/// `done`; a change the project does not hold derives no status and marks no
/// phase done.
fn phase_statuses(resolution: &ChangeResolution) -> Vec<Option<&'static str>> {
    match resolution {
        ChangeResolution::Live(state) => {
            let change_dir = std::env::current_dir()
                .unwrap_or_default()
                .join("givn")
                .join("changes")
                .join(&state.id);
            let current = current_phase_index(state);
            WORKFLOW_STAGES
                .iter()
                .enumerate()
                .map(|(index, stage)| {
                    if state_artifact_of(stage)
                        .is_some_and(|artifact| crate::state::is_skipped(artifact, &change_dir))
                    {
                        return Some("skipped");
                    }
                    match current {
                        Some(current) if index < current => Some("done"),
                        Some(current) if index == current => Some("active"),
                        Some(_) => Some("queued"),
                        None => None,
                    }
                })
                .collect()
        }
        ChangeResolution::Archived => vec![Some("done"); WORKFLOW_STAGES.len()],
        ChangeResolution::Absent => vec![None; WORKFLOW_STAGES.len()],
    }
}

/// One status per ideation phase, in [`IDEATION_STAGES`] order, from the
/// topic's recorded markers: the furthest marker in ideation order is `active`,
/// every earlier ideation phase `done`, every later one `queued`; before any
/// marker `frame` is `active` and the rest `queued`. A marker starts its phase,
/// so the marker itself is the current phase, not the one before it.
fn ideation_statuses(entries: &[&HistoryEntry]) -> Vec<Option<&'static str>> {
    let current = entries
        .iter()
        .copied()
        .filter_map(ideation_marker)
        .filter_map(|(_, phase)| IDEATION_STAGES.iter().position(|stage| *stage == phase))
        .max()
        .unwrap_or(0);
    (0..IDEATION_STAGES.len())
        .map(|index| {
            Some(if index < current {
                "done"
            } else if index == current {
                "active"
            } else {
                "queued"
            })
        })
        .collect()
}

/// The current state the report embeds for a live change: exactly what the
/// change state exposes — stage label, mapped phase, next required artifact,
/// and task counts — and nothing it does not.
fn current_state_of(state: &crate::state::ChangeState) -> CurrentState {
    CurrentState {
        stage_label: state.stage_label(),
        phase: current_phase_index(state).map(|index| WORKFLOW_STAGES[index].to_string()),
        next_artifact: state.next_required().map(|artifact| artifact.id.clone()),
        tasks: state
            .artifacts
            .iter()
            .find(|artifact| artifact.id == "tasks")
            .and_then(|artifact| artifact.tasks_progress.clone())
            .map(|progress| TasksProgress {
                total: progress.total,
                done: progress.done,
            }),
    }
}

/// The entries the one derivation assigns to one stage (`None` for the no-stage
/// group), in the model's own order.
fn phase_members<'a>(
    stage: Option<&str>,
    stages: &[StageAssignment],
    entries: &'a [HistoryEntry],
) -> Vec<&'a HistoryEntry> {
    stages
        .iter()
        .filter(|assignment| assignment.stage.as_deref() == stage)
        .filter_map(|assignment| entries.iter().find(|entry| entry.id == assignment.entry))
        .collect()
}

/// One phase's scaffold from the entries the derivation places in it: the
/// first/last instants, wall/active/waiting time, tool and subagent counts, its
/// lanes, its repeated groups, and its Fit-zoom aggregates. Nothing is inferred:
/// a duration the record does not carry stays unmeasured, and a phase with no
/// recorded entries records zero minutes.
fn build_phase(
    stage: Option<&str>,
    status: Option<&str>,
    members: &[&HistoryEntry],
    scale: &TimeScale,
) -> WorkflowPhase {
    let mut first_ms: Option<i64> = None;
    let mut last_ms: Option<i64> = None;
    let mut waiting_ms: Option<u64> = None;
    let mut waits = 0usize;
    let mut tools = 0usize;
    let mut subagents = 0usize;
    for entry in members {
        if let Some(at) = crate::history::parse_rfc3339_ms(&entry.at) {
            first_ms = Some(first_ms.map_or(at, |first| first.min(at)));
            last_ms = Some(last_ms.map_or(at, |last| last.max(at)));
        }
        match entry.kind.as_str() {
            "tool" => tools += 1,
            SUBAGENT => subagents += 1,
            QUESTION => {
                waits += 1;
                add_measured(&mut waiting_ms, entry.duration_ms);
            }
            _ => {}
        }
    }
    let wall_ms = match (members.is_empty(), first_ms.zip(last_ms)) {
        (true, _) => Some(0),
        (false, Some((first, last))) => u64::try_from((last - first).max(0)).ok(),
        (false, None) => None,
    };
    let waiting_ms = if waits == 0 { Some(0) } else { waiting_ms };
    let active_ms = wall_ms
        .zip(waiting_ms)
        .map(|(wall, waiting)| wall.saturating_sub(waiting));
    WorkflowPhase {
        stage: stage.map(str::to_string),
        status: status.map(str::to_string),
        first_ms,
        last_ms,
        wall_ms,
        active_ms,
        waiting_ms,
        tools,
        subagents,
        lanes: lanes_of(members),
        repeated: repeated_groups(members),
        aggregates: phase_aggregates(members, scale),
    }
}

/// The phase's lanes: one per distinct recorded session, named by the recorded
/// `session` entry's title when one exists, else by the session's short id;
/// entries the record names no session for go to the unattributed lane.
fn lanes_of(members: &[&HistoryEntry]) -> Vec<Lane> {
    let mut lanes: Vec<Lane> = Vec::new();
    for entry in members {
        let index = match lanes.iter().position(|lane| lane.session == entry.session) {
            Some(index) => index,
            None => {
                let name = lane_name(entry.session.as_deref(), members);
                lanes.push(Lane {
                    session: entry.session.clone(),
                    name,
                    ..Lane::default()
                });
                lanes.len() - 1
            }
        };
        let lane = &mut lanes[index];
        lane.entries.push(entry.id);
        if let Some(at) = crate::history::parse_rfc3339_ms(&entry.at) {
            lane.first_ms = Some(lane.first_ms.map_or(at, |first| first.min(at)));
            lane.last_ms = Some(lane.last_ms.map_or(at, |last| last.max(at)));
        }
    }
    for lane in &mut lanes {
        lane.duration_ms = lane
            .first_ms
            .zip(lane.last_ms)
            .and_then(|(first, last)| u64::try_from((last - first).max(0)).ok());
    }
    lanes
}

/// The child session one subagent span names (`data.subagent`): the session id
/// a reporter delivered for that subagent's own entries. A span whose
/// `data.subagent` equals a lane's session id is that session's own recorded
/// span (opencode's child-session flush); a span recorded in any other session
/// sits in that spawning session's lane and never renames it.
fn subagent_session(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("subagent")?.as_str()
}

/// The visible name of one lane: the recorded subagent span's `data.agent` when
/// a span in the phase names this lane's session id as its own child session,
/// else the recorded `session` entry's title, else the session's short id (the
/// first eight characters, the whole id when shorter), and `unattributed` for
/// the entries the record names no session for.
fn lane_name(session: Option<&str>, members: &[&HistoryEntry]) -> String {
    let Some(session) = session else {
        return "unattributed".to_string();
    };
    let subagent_agent = members
        .iter()
        .filter(|entry| entry.kind == SUBAGENT)
        .filter(|entry| subagent_session(entry) == Some(session))
        .find_map(|entry| entry.data.as_ref()?.get("agent")?.as_str());
    if let Some(agent) = subagent_agent {
        return agent.to_string();
    }
    let title = members
        .iter()
        .filter(|entry| entry.kind == "session")
        .filter(|entry| entry.session.as_deref() == Some(session))
        .find_map(|entry| {
            entry
                .data
                .as_ref()?
                .get("title")?
                .as_str()
                .map(str::to_string)
        });
    title.unwrap_or_else(|| session.chars().take(8).collect())
}

/// The subagent spans a phase holds with no attributable tool calls: no `tool`
/// entry is nested in the span and no tool entry carries the span's own child
/// session. The report states this limit (R-93) instead of assigning a tool call
/// to a subagent the record does not name.
fn unattributed_spans<'a>(
    phase: &WorkflowPhase,
    entries: &'a [HistoryEntry],
) -> Vec<&'a HistoryEntry> {
    let members: Vec<&HistoryEntry> = phase
        .lanes
        .iter()
        .flat_map(|lane| lane.entries.iter())
        .filter_map(|id| entries.iter().find(|entry| entry.id == *id))
        .collect();
    members
        .iter()
        .filter(|entry| entry.kind == SUBAGENT)
        .filter(|span| {
            let child = subagent_session(span);
            !members.iter().any(|entry| {
                entry.kind == "tool"
                    && (entry.parent == Some(span.id)
                        || child.is_some_and(|child| entry.session.as_deref() == Some(child)))
            })
        })
        .copied()
        .collect()
}

/// The normalized label a repeated-step group groups by and the ranking row
/// states: the entry's visible label, trimmed with its whitespace collapsed, so
/// two recordings that differ only in spacing read as the same step.
fn normalized_label(entry: &HistoryEntry) -> String {
    visible_label(entry)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The phase's repeated-step groups: entries of the same kind and the same
/// normalized label, grouped when the group holds at least five entries.
fn repeated_groups(members: &[&HistoryEntry]) -> Vec<RepeatedGroup> {
    let mut groups: BTreeMap<(String, String), Vec<&HistoryEntry>> = BTreeMap::new();
    for entry in members {
        groups
            .entry((entry.kind.clone(), normalized_label(entry)))
            .or_default()
            .push(entry);
    }
    groups
        .into_iter()
        .filter(|(_, group)| group.len() >= 5)
        .map(|((kind, label), group)| {
            let mut duration_ms = None;
            for entry in &group {
                add_measured(&mut duration_ms, entry.duration_ms);
            }
            RepeatedGroup {
                kind,
                label,
                count: group.len(),
                duration_ms,
                entries: group.iter().map(|entry| entry.id).collect(),
            }
        })
        .collect()
}

/// The phase's Fit-zoom aggregates: its entries too short to draw one shape per
/// entry at the initial zoom, bucketed by recorded time with the report's one
/// bucketing rule.
fn phase_aggregates(members: &[&HistoryEntry], scale: &TimeScale) -> Vec<PhaseAggregate> {
    let time_per_pixel = scale.time_per_pixel();
    let bucket_ms = (time_per_pixel as f64 * AGGREGATE_BUCKET_PX).ceil() as i64;
    let mut aggregates: Vec<PhaseAggregate> = Vec::new();
    for entry in members {
        let Some(at) = crate::history::parse_rfc3339_ms(&entry.at) else {
            continue;
        };
        if !collapses(entry, time_per_pixel) {
            continue;
        }
        let bucket_start_ms =
            scale.start_ms + (at - scale.start_ms).div_euclid(bucket_ms) * bucket_ms;
        match aggregates
            .iter_mut()
            .find(|marker| marker.bucket_start_ms == bucket_start_ms)
        {
            Some(marker) => {
                marker.entries.push(entry.id);
                marker.count += 1;
                add_measured(&mut marker.duration_ms, entry.duration_ms);
            }
            None => aggregates.push(PhaseAggregate {
                bucket_start_ms,
                bucket_end_ms: bucket_start_ms + bucket_ms,
                entries: vec![entry.id],
                count: 1,
                duration_ms: entry.duration_ms,
            }),
        }
    }
    aggregates
}

/// The workflow scaffold of the scope the report was asked for: the scope's own
/// workflow phases, the live current state, the group of work the record
/// assigns to no change stage, and — for a change report whose proposal names an
/// existing, recorded topic — the linked ideation group. The project report
/// keeps its band rows and gets no stack.
fn phase_scaffold(
    scope: Option<&Scope>,
    stages: &[StageAssignment],
    entries: &[HistoryEntry],
    scale: &TimeScale,
    ideation_topic: Option<&str>,
) -> PhaseScaffold {
    let Some(scope) = scope else {
        return PhaseScaffold::default();
    };
    // The one workflow selector: the scope's own phase order, and the statuses
    // derived from that workflow's own markers. Every band row, the stack, and
    // the statuses read this order, never a second match on the scope.
    let workflow_order = workflow_stages(scope);
    let (statuses, current_state) = match scope {
        Scope::Change { .. } => {
            let resolution = change_resolution(scope);
            let current_state = match &resolution {
                ChangeResolution::Live(state) => Some(current_state_of(state)),
                ChangeResolution::Archived | ChangeResolution::Absent => None,
            };
            (phase_statuses(&resolution), current_state)
        }
        Scope::Ideation { .. } => {
            let own: Vec<&HistoryEntry> = entries
                .iter()
                .filter(|entry| &entry.scope == scope)
                .collect();
            (ideation_statuses(&own), None)
        }
        // The project report keeps its band rows and builds no phase stack: a
        // nine-phase scaffold of `None` statuses would claim a workflow the
        // project scope never records.
        Scope::Project { .. } => return PhaseScaffold::default(),
    };
    let workflow = workflow_order
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            let members = phase_members(Some(stage), stages, entries);
            build_phase(Some(stage), statuses[index], &members, scale)
        })
        .collect();
    // The ideation derivation always places an entry in an ideation phase, so a
    // topic report never has a no-stage group.
    let unstaged_members = match scope {
        Scope::Ideation { .. } => Vec::new(),
        Scope::Change { .. } | Scope::Project { .. } => phase_members(None, stages, entries),
    };
    let unstaged =
        (!unstaged_members.is_empty()).then(|| build_phase(None, None, &unstaged_members, scale));
    PhaseScaffold {
        workflow,
        current_state,
        unstaged,
        ideation: ideation_topic.map(|topic| ideation_group(topic, entries)),
    }
}

/// The linked ideation group of a change report: the topic's own entries, its
/// six ideation phases with the topic report's marker statuses, and the group's
/// own totals. A topic whose record holds no entries carries no phases — the
/// group states that the activity predates the record instead of inventing
/// them — and a group that holds entries derives its phases and totals from
/// those entries alone, never from the change's.
fn ideation_group(topic: &str, entries: &[HistoryEntry]) -> IdeationGroup {
    let topic_scope = Scope::Ideation {
        id: topic.to_string(),
    };
    let group_entries: Vec<&HistoryEntry> = entries
        .iter()
        .filter(|entry| entry.scope == topic_scope)
        .collect();
    let ids: Vec<u64> = group_entries.iter().map(|entry| entry.id).collect();
    let activity = !group_entries.is_empty();
    let assignments = ideation_stage_assignments(&ids, entries);
    // The group's own scale, never the change's: its Fit buckets cover the
    // topic's window.
    let group_scale = TimeScale::of_refs(&group_entries);
    let phases = if activity {
        let statuses = ideation_statuses(&group_entries);
        IDEATION_STAGES
            .iter()
            .enumerate()
            .map(|(index, stage)| {
                let members = phase_members(Some(stage), &assignments, entries);
                build_phase(Some(stage), statuses[index], &members, &group_scale)
            })
            .collect()
    } else {
        Vec::new()
    };
    let summary = Summary::of(&group_entries);
    IdeationGroup {
        topic: topic.to_string(),
        activity,
        phases,
        entries: ids,
        duration_ms: summary.elapsed_ms,
        tools: summary.tool_calls,
        subagents: summary.subagents,
        waits: summary.waits,
        wait_ms: summary.wait_ms,
    }
}

/// Whether one record entry is one of the recorder's own invocations rather
/// than recorded work: the intake (`givn history ingest`, one flush per turn)
/// and the process-per-hook mapper (`givn history harness-hook --harness
/// <name>`, one process per hook event) record themselves like any other
/// command, but they are recording infrastructure. The store keeps them and
/// the read view still prints them; only the report leaves them out.
///
/// The exclusion is exact: a `command` entry named `history` whose bounded
/// label is exactly `history ingest`, or begins with `history ingest `, or
/// begins with `history harness-hook ` — the subcommand is followed by a word
/// boundary, never matched as a substring. `history ingest-extra`, a bare
/// `history`, and `history settings --json` are other commands and stay, as
/// does any entry of another kind (a `tool` or a `commit` that merely quotes
/// the phrase); a `None` label never matches. [`crate::history::bound_label`]
/// keeps the prefix at 120 characters, so the prefix test survives truncation.
fn is_recorder_command(entry: &HistoryEntry) -> bool {
    if entry.kind != "command" || entry.name != "history" {
        return false;
    }
    let Some(label) = entry.label.as_deref() else {
        return false;
    };
    label == "history ingest"
        || label.starts_with("history ingest ")
        || label.starts_with("history harness-hook ")
}

/// The report scope's own entries, borrowed: all entries for the project
/// report, the matching scope's entries for a change or topic report. The
/// summary, ranking, waits, KPI facts, and `empty` read this slice, so a linked
/// topic's entries never leak into the change's totals.
fn report_scope_entries<'a>(
    scope: Option<&Scope>,
    entries: &'a [HistoryEntry],
) -> Vec<&'a HistoryEntry> {
    match scope {
        None | Some(Scope::Project { .. }) => entries.iter().collect(),
        Some(scope) => entries
            .iter()
            .filter(|entry| &entry.scope == scope)
            .collect(),
    }
}

/// Record lines → `ReportModel` (bands, ordering, ranking, totals, and the
/// initial zoom's aggregates).
///
/// `link` is the resolved link snapshot the caller read from the change's
/// proposal and the topic's record; the model stays pure and deterministic over
/// the record, the state snapshot, and that snapshot.
pub fn build_report_model(
    scope: Option<Scope>,
    mut entries: Vec<HistoryEntry>,
    link: Option<LinkedIdeation>,
) -> ReportModel {
    // The model's first step: the recorder's own command entries are dropped
    // before the content budget and before every derivation (ordering, bands,
    // stage assignment, the phase scaffold, lanes, group counts, aggregates,
    // the summary, elapsed, and entry totals). Every report surface is derived
    // from this one filtered vector, so no number and no view can disagree,
    // and neither the budget nor `omitted_content` ever names one. A scope
    // whose record holds only these entries therefore renders the empty
    // state.
    entries.retain(|entry| !is_recorder_command(entry));
    // The resolved link is a change report's alone: a topic or project report
    // parses no link. Its topic entries join the model's entries once and form
    // their own ideation band; they never join the change's band.
    let (ideation_topic, linked) = match link {
        Some(link) if matches!(scope, Some(Scope::Change { .. })) => {
            (Some(link.topic), link.entries)
        }
        _ => (None, Vec::new()),
    };
    entries.extend(
        linked
            .into_iter()
            .filter(|entry| !is_recorder_command(entry)),
    );
    // The content budget is applied in record order — the store's own line
    // order, before the report's time ordering — so a record over the budget
    // always produces the same document. An entry whose bundle would cross the
    // cap is not embedded; the panel states the omission and the store keeps
    // every byte.
    let (content_bytes, omitted_content) = apply_content_budget(&mut entries);
    let entries = ordered_by_recorded_time(entries);
    let mut bands = bands_for(scope.as_ref(), &entries);
    // The one phase derivation: every band's entries are placed in the stage
    // their own `at` reached, and the row's phase names are read back from the
    // same assignment — so a span's `data-stage` and its row's phase line can
    // never disagree.
    let stages = stage_assignments(&bands, &entries);
    let phases: Vec<Vec<String>> = bands
        .iter()
        .map(|band| band_phases(band, &stages))
        .collect();
    for (band, phases) in bands.iter_mut().zip(phases) {
        band.stages = phases;
    }
    // The summary, the ranking, the KPI facts, and `empty` are the report
    // scope's own: a linked topic's entries are in the model once but never in
    // these totals. `empty` is true only when the model holds no entries at
    // all.
    let own = report_scope_entries(scope.as_ref(), &entries);
    let summary = Summary::of(&own);
    let empty = entries.is_empty();
    let scale = TimeScale::of(&entries);
    // The workflow scaffold: the scope's own workflow phases, the live current
    // state, and the no-stage group, from the same one derivation the model's
    // `stages` carries, plus the linked ideation group for a change report. The
    // scaffold holds ids and totals only; the entries stay in `entries`,
    // embedded once.
    let scaffold = phase_scaffold(
        scope.as_ref(),
        &stages,
        &entries,
        &scale,
        ideation_topic.as_deref(),
    );
    // The one bucketing: the markers the initial view draws are computed here,
    // from the window the bands were built from, and carried in the model — so
    // the zoom path reads the aggregate that already stands for an entry
    // instead of bucketing a second time.
    let aggregates = aggregates_for(&bands, &entries, &scale);
    ReportModel {
        scope,
        bands,
        entries,
        empty,
        aggregates,
        stages,
        workflow: scaffold.workflow,
        current_state: scaffold.current_state,
        unstaged: scaffold.unstaged,
        ideation: scaffold.ideation,
        summary,
        content_budget_bytes: CONTENT_BUDGET_BYTES,
        content_bytes,
        omitted_content,
        content_omitted_note: CONTENT_OMITTED_NOTE.to_string(),
    }
}

/// The content bytes one report document embeds and the ids whose content was
/// left out: every entry's `data.content` in record order until
/// [`CONTENT_BUDGET_BYTES`], then omission. An entry that crosses the cap has
/// its content removed from the model (so the document never carries it) and is
/// listed by id.
fn apply_content_budget(entries: &mut [HistoryEntry]) -> (usize, Vec<u64>) {
    let mut used = 0usize;
    let mut omitted: Vec<u64> = Vec::new();
    for entry in entries {
        let Some(content) = entry
            .data
            .as_ref()
            .and_then(|data| data.get("content"))
            .cloned()
        else {
            continue;
        };
        let size = content_bytes(&content);
        if used.saturating_add(size) > CONTENT_BUDGET_BYTES {
            if let Some(data) = entry
                .data
                .as_mut()
                .and_then(serde_json::Value::as_object_mut)
            {
                data.remove("content");
                data.insert("content_omitted".to_string(), serde_json::Value::Bool(true));
            }
            omitted.push(entry.id);
            continue;
        }
        used = used.saturating_add(size);
    }
    (used, omitted)
}

/// The embedded byte size of one `data.content` object: the UTF-8 bytes of
/// every named field's kept text. The object's own `truncated` marker and each
/// field's original `bytes` are accounting, not embedded text.
fn content_bytes(content: &serde_json::Value) -> usize {
    content
        .as_object()
        .map(|object| {
            object
                .iter()
                .filter(|(name, _)| name.as_str() != "truncated")
                .filter_map(|(_, field)| field.get("text").and_then(serde_json::Value::as_str))
                .map(str::len)
                .sum()
        })
        .unwrap_or_default()
}

/// The window's aggregates: the report's one bucketing, used by the initial
/// zoom's markup and by any later zoom that replaces an individual span with
/// the marker already standing for it.
///
/// At the initial zoom an entry is drawn on the axis [`TimeScale`] maps the
/// window onto; an entry whose drawn extent is shorter than the time one pixel
/// of that axis spends on — dense agent ticks and short steps in particular —
/// cannot be told apart from its neighbours, so it collapses into one marker
/// per band and time bucket. The bucket's bounds are the report's own buckets,
/// aligned to the window's start, so the markers of a band tile its axis and
/// every entry falls into exactly one of them.
fn aggregates_for(bands: &[Band], entries: &[HistoryEntry], scale: &TimeScale) -> Vec<Aggregate> {
    let time_per_pixel = scale.time_per_pixel();
    let bucket_ms = (time_per_pixel as f64 * AGGREGATE_BUCKET_PX).ceil() as i64;
    let mut aggregates: Vec<Aggregate> = Vec::new();
    for band in bands {
        for id in &band.entries {
            let Some(entry) = entries.iter().find(|entry| entry.id == *id) else {
                continue;
            };
            // An entry whose instant the record cannot read has no bucket to
            // fall into and stays an individual span.
            let Some(at) = crate::history::parse_rfc3339_ms(&entry.at) else {
                continue;
            };
            if !collapses(entry, time_per_pixel) {
                continue;
            }
            let bucket_start_ms =
                scale.start_ms + (at - scale.start_ms).div_euclid(bucket_ms) * bucket_ms;
            match aggregates.iter_mut().find(|marker| {
                marker.scope == band.scope && marker.bucket_start_ms == bucket_start_ms
            }) {
                Some(marker) => {
                    marker.entries.push(entry.id);
                    add_measured(&mut marker.duration_ms, entry.duration_ms);
                }
                None => aggregates.push(Aggregate {
                    scope: band.scope.clone(),
                    bucket_start_ms,
                    bucket_end_ms: bucket_start_ms + bucket_ms,
                    entries: vec![entry.id],
                    duration_ms: entry.duration_ms,
                }),
            }
        }
    }
    aggregates
}

/// Whether one entry collapses into an aggregate at the initial zoom: its drawn
/// extent — the duration it was measured with, or nothing at all for an entry
/// the record does not time — is shorter than the time one pixel of the window
/// spends on. An entry this short is a tick, not a span a reader can place.
fn collapses(entry: &HistoryEntry, time_per_pixel_ms: i64) -> bool {
    let extent = entry
        .duration_ms
        .and_then(|duration| i64::try_from(duration).ok())
        .unwrap_or_default();
    extent < time_per_pixel_ms
}

/// The window's entries in the record's own order: by `at`, the entry's
/// recorded end timestamp, and never by the entry's line position in the store.
/// The store is append-only and concurrent writers make file order and time
/// order different things, while `at` is the only ordering the record
/// guarantees. `id` is minted at append time under the append lock and is
/// therefore file-ordered too, so it is never the sort key — only the tie
/// breaker for two entries that record the same instant. An entry whose `at`
/// does not parse sorts after every timestamp the record can read.
fn ordered_by_recorded_time(mut entries: Vec<HistoryEntry>) -> Vec<HistoryEntry> {
    entries.sort_by(|a, b| {
        match (
            crate::history::parse_rfc3339_ms(&a.at),
            crate::history::parse_rfc3339_ms(&b.at),
        ) {
            (Some(left), Some(right)) => left.cmp(&right).then_with(|| a.id.cmp(&b.id)),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.id.cmp(&b.id),
        }
    });
    entries
}

/// The report's window: the bands its rows cover. A named scope is one band —
/// the report of that change, topic, or project. The project report groups the
/// window's entries by the scope each one belongs to, in the order the window
/// first names them, so the project's own entries stay in the "outside a
/// scope" band instead of being folded into a change. Every band takes its
/// spans in the window's own order, so the one `at` comparator
/// [`ordered_by_recorded_time`] applies to the window orders every band, the
/// drawn axis, and the summary alike.
fn bands_for(scope: Option<&Scope>, entries: &[HistoryEntry]) -> Vec<Band> {
    let mut bands: Vec<Band> = Vec::new();
    // A window with no entries has no rows: the report root renders its empty
    // state then, and an empty band row could only draw an axis over nothing.
    if entries.is_empty() {
        return bands;
    }
    if let Some(scope) = scope {
        // The named scope's own band first, then one band per other scope the
        // window carries — a change report's linked ideation topic entries form
        // their own ideation band and never join the change's.
        let own: Vec<u64> = entries
            .iter()
            .filter(|entry| &entry.scope == scope)
            .map(|entry| entry.id)
            .collect();
        if !own.is_empty() {
            bands.push(Band::of(scope, own));
        }
        for entry in entries {
            if &entry.scope == scope {
                continue;
            }
            match bands.iter_mut().find(|band| band.scope == entry.scope) {
                Some(band) => band.entries.push(entry.id),
                None => bands.push(Band::of(&entry.scope, vec![entry.id])),
            }
        }
        return bands;
    }
    for entry in entries {
        match bands.iter_mut().find(|band| band.scope == entry.scope) {
            Some(band) => band.entries.push(entry.id),
            None => bands.push(Band::of(&entry.scope, vec![entry.id])),
        }
    }
    bands
}

/// The change workflow's stages, in the order a change works through them: the
/// phase names a band's row carries.
pub(crate) const WORKFLOW_STAGES: [&str; 9] = [
    "propose",
    "specs",
    "design",
    "arc42",
    "design-review",
    "tasks",
    "implement",
    "review",
    "archive",
];

/// Each stage's own artifact, in [`WORKFLOW_STAGES`]' own order: the file whose
/// appearance is one recorded `phase` transition — and therefore opens the
/// stage that follows the stage it belongs to, so `tasks.md` appearing opens
/// `implement`. A stage that produces no artifact of its own (`implement`,
/// `archive`) is `None`, and the artifact of the stage before it opens the next
/// stage that has one.
pub(crate) const STAGE_ARTIFACTS: [Option<&str>; 9] = [
    Some("proposal.md"),
    Some("specs"),
    Some("design.md"),
    Some("arc42.md"),
    Some("design-review.md"),
    Some("tasks.md"),
    None,
    Some("review.md"),
    None,
];

/// The change state's artifact ids mapped onto the workflow phase that
/// produces each one: the design's current-phase table read in the direction
/// the transition writer needs. The id, never a list position, carries the
/// mapping; `tasks` is the phase that produces `tasks.md` (a partial tasks
/// artifact is not `Done`, so it emits no transition at all).
pub(crate) const ARTIFACT_STAGES: [(&str, &str); 7] = [
    ("proposal", "propose"),
    ("specs", "specs"),
    ("design", "design"),
    ("arc42-docs", "arc42"),
    ("design-review", "design-review"),
    ("tasks", "tasks"),
    ("review", "review"),
];

/// The [`STAGE_ARTIFACTS`] filename one change-state artifact's appearance is
/// recorded with, or `None` for an artifact the workflow does not map. The
/// filename is read at the mapped stage's own index, so a state artifact id and
/// the transition a change records can never drift apart.
pub(crate) fn artifact_transition_label(state_artifact: &str) -> Option<&'static str> {
    let stage = ARTIFACT_STAGES
        .iter()
        .find(|(id, _)| *id == state_artifact)?
        .1;
    let index = WORKFLOW_STAGES.iter().position(|known| *known == stage)?;
    STAGE_ARTIFACTS[index]
}

/// The workflow position of one transition label, so the writer emits the
/// artifacts one command first observes in workflow order.
pub(crate) fn artifact_workflow_rank(label: &str) -> Option<usize> {
    STAGE_ARTIFACTS
        .iter()
        .position(|known| *known == Some(label))
}

/// The stage one recorded artifact's appearance opens. An artifact the workflow
/// does not record opens no stage at all: a record's own labels never invent a
/// phase name.
fn stage_opened_by(artifact: &str) -> Option<&'static str> {
    let index = STAGE_ARTIFACTS
        .iter()
        .position(|known| *known == Some(artifact))?;
    WORKFLOW_STAGES.get(index + 1).copied()
}

/// One recorded artifact transition: the instant the artifact appeared, the
/// artifact label the record carries (the file whose appearance it records),
/// and the workflow stage that appearance opens. The label is kept because the
/// opening-stage rule must know whether the band's first transition is
/// `proposal.md` — the only one that makes the record start where the workflow
/// starts.
fn artifact_transition(entry: &HistoryEntry) -> Option<(i64, &str, &'static str)> {
    if entry.kind != PHASE {
        return None;
    }
    let artifact = entry.label.as_deref().unwrap_or(&entry.name);
    Some((
        crate::history::parse_rfc3339_ms(&entry.at)?,
        artifact,
        stage_opened_by(artifact)?,
    ))
}

/// One entry's derived stage: the workflow stage the record places it in, so a
/// span can be read under its phase. `stage` is `None` for the unnamed stage a
/// scope whose record carries no artifact transition gets.
#[derive(Debug, Serialize)]
pub struct StageAssignment {
    /// The entry's own id.
    pub entry: u64,
    /// The stage's name, absent for the unnamed stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

impl ReportModel {
    /// The stage one entry worked in, as the model's one derivation placed it:
    /// the single accessor the CLI's rendering and the browser tier's phase
    /// placement both read, so a second derivation can never disagree with the
    /// assignment the model carries.
    fn stage_of(&self, entry: u64) -> Option<&str> {
        self.stages
            .iter()
            .find(|assignment| assignment.entry == entry)
            .and_then(|assignment| assignment.stage.as_deref())
    }
}

/// Every band's entries with the stage they worked in, in the band scope's own
/// workflow: a change band reads the change workflow's artifact transitions,
/// an ideation band reads its start markers directly. The one `stages` vector
/// stays the single assignment source for the whole model.
fn stage_assignments(bands: &[Band], entries: &[HistoryEntry]) -> Vec<StageAssignment> {
    let mut assignments = Vec::new();
    for band in bands {
        match &band.scope {
            Scope::Ideation { .. } => {
                assignments.extend(ideation_stage_assignments(&band.entries, entries));
            }
            Scope::Change { .. } | Scope::Project { .. } => {
                assignments.extend(change_stage_assignments(&band.entries, entries));
            }
        }
    }
    assignments
}

/// A change band's entries with the stage they worked in: an entry belongs to
/// the stage the latest artifact transition at or before its own `at` opened.
/// Entries recorded before the record's first transition stay in the workflow's
/// opening stage only when that first transition is `proposal.md` — the record
/// starts where the workflow starts; when the first transition is later, those
/// entries belong to no stage, and a scope whose record carries no transition
/// gets no stage either. A scope with no stage is never given an invented
/// phase name. This is the report's one derivation of a phase: the model
/// carries its result, so neither the renderer nor the browser script derives
/// a stage a second time.
fn change_stage_assignments(ids: &[u64], entries: &[HistoryEntry]) -> Vec<StageAssignment> {
    let mut assignments = Vec::new();
    let mut transitions: Vec<(i64, &str, &'static str)> = ids
        .iter()
        .filter_map(|id| entries.iter().find(|entry| entry.id == *id))
        .filter_map(artifact_transition)
        .collect();
    transitions.sort_by_key(|(at, _, _)| *at);
    let starts_at_workflow_start = transitions
        .first()
        .is_some_and(|(_, artifact, _)| *artifact == "proposal.md");
    for id in ids {
        let Some(entry) = entries.iter().find(|entry| entry.id == *id) else {
            continue;
        };
        let opened = crate::history::parse_rfc3339_ms(&entry.at).and_then(|at| {
            transitions
                .iter()
                .rev()
                .find(|(transition_at, _, _)| *transition_at <= at)
                .map(|(_, _, stage)| (*stage).to_string())
        });
        let stage = match opened {
            Some(stage) => Some(stage),
            None if starts_at_workflow_start => Some(WORKFLOW_STAGES[0].to_string()),
            None => None,
        };
        assignments.push(StageAssignment { entry: *id, stage });
    }
    assignments
}

/// An ideation band's entries with the ideation phase they worked in: a marker
/// label maps **directly** to the phase it starts (never through
/// [`STAGE_ARTIFACTS`], whose labels open the following change stage), an
/// entry belongs to the latest marker at or before its `at`, and entries
/// before the first marker are `frame`. The derivation never yields an absent
/// stage, so a topic report has no "no recorded stage transition" group.
fn ideation_stage_assignments(ids: &[u64], entries: &[HistoryEntry]) -> Vec<StageAssignment> {
    let mut markers: Vec<(i64, &'static str)> = ids
        .iter()
        .filter_map(|id| entries.iter().find(|entry| entry.id == *id))
        .filter_map(ideation_marker)
        .collect();
    markers.sort_by_key(|(at, _)| *at);
    ids.iter()
        .filter_map(|id| {
            let entry = entries.iter().find(|entry| entry.id == *id)?;
            let stage = crate::history::parse_rfc3339_ms(&entry.at)
                .and_then(|at| {
                    markers
                        .iter()
                        .rev()
                        .find(|(marker_at, _)| *marker_at <= at)
                        .map(|(_, phase)| (*phase).to_string())
                })
                .unwrap_or_else(|| IDEATION_STAGES[0].to_string());
            Some(StageAssignment {
                entry: *id,
                stage: Some(stage),
            })
        })
        .collect()
}

/// One recorded ideation start marker: the instant it was observed and the
/// phase its label starts. `frame.md` is not a marker, and a label outside the
/// marker table starts no ideation phase — never an inferred one.
fn ideation_marker(entry: &HistoryEntry) -> Option<(i64, &'static str)> {
    if entry.kind != PHASE {
        return None;
    }
    let label = entry.label.as_deref().unwrap_or(&entry.name);
    let phase = IDEATION_STAGES
        .iter()
        .copied()
        .skip(1)
        .find(|stage| *stage == label)?;
    Some((crate::history::parse_rfc3339_ms(&entry.at)?, phase))
}

/// The workflow stages one band's phases are named in: the band scope's own
/// workflow order, so an ideation band names ideation phases and a change band
/// names change phases.
fn workflow_stages(scope: &Scope) -> &'static [&'static str] {
    match scope {
        Scope::Ideation { .. } => &IDEATION_STAGES,
        Scope::Change { .. } | Scope::Project { .. } => &WORKFLOW_STAGES,
    }
}

/// One band's phases: the stages its entries worked in, in the band scope's own
/// workflow order and without repetition — the names the row's phase line shows
/// and the browser script draws from the same list.
fn band_phases(band: &Band, assignments: &[StageAssignment]) -> Vec<String> {
    workflow_stages(&band.scope)
        .iter()
        .filter(|stage| {
            assignments.iter().any(|assignment| {
                assignment.stage.as_deref() == Some(**stage)
                    && band.entries.contains(&assignment.entry)
            })
        })
        .map(|stage| (*stage).to_string())
        .collect()
}

/// One scope as the report shows it: the class the screen styles a band row by,
/// the row's visible label, and the identifier the default file name uses. This
/// is the report's one derivation of a scope — the window selection, the band
/// rows, and the default path all read it — so a change, a topic, and the
/// project can never be named or labelled in two different ways.
struct ScopeRow {
    /// The scope class (`change`, `ideation`, `project`); never visible text.
    kind: &'static str,
    /// The row's visible label.
    label: String,
    /// The scope's own identifier.
    id: String,
}

/// The report's own view of one scope.
fn scope_row(scope: &Scope) -> ScopeRow {
    match scope {
        Scope::Change { id } => ScopeRow {
            kind: "change",
            label: format!("Change: {id}"),
            id: id.clone(),
        },
        Scope::Ideation { id } => ScopeRow {
            kind: "ideation",
            label: format!("Ideation topic: {id}"),
            id: id.clone(),
        },
        Scope::Project { id } => ScopeRow {
            kind: "project",
            label: format!("Outside a scope: {id}"),
            id: id.clone(),
        },
    }
}

/// Add one recorded duration to a running total. A duration the record does not
/// carry is never added as zero: the total stays `None` while nothing was
/// measured, so every total this summary states (waits now, subagent spans and
/// token usage next) renders "not measured" instead of a false zero. Returns
/// whether the duration was measured.
fn add_measured(total: &mut Option<u64>, duration: Option<u64>) -> bool {
    match duration {
        Some(duration) => {
            *total = Some(total.unwrap_or_default() + duration);
            true
        }
        None => false,
    }
}

/// Every entry the record measured, longest first — the report's one ranking:
/// the summary shows it, and every other view (the axis's longest-step mark,
/// the details panel) reads the ranked list the model carries rather than
/// sorting a second time. Any kind is ranked that carries a measured
/// `duration_ms` — a command, a gate phase, a hook run, a shell step, an agent
/// turn, a tool call, a subagent span, a question wait — because the ranking
/// answers "what took the time", whatever layer recorded it. An entry without a
/// measured duration is absent, never ranked as a zero. The order is by the
/// recorded duration, never by the store's line order; ties fall back to the
/// recorded time and then the entry id, so the ranking is deterministic for
/// every record. The list is deliberately uncapped: the design bounds the
/// *drawn shapes* (the aggregation rule), never the summary rows.
fn ranked_steps(entries: &[&HistoryEntry]) -> Vec<RankedStep> {
    let mut measured: Vec<&HistoryEntry> = entries
        .iter()
        .copied()
        .filter(|entry| entry.duration_ms.is_some())
        .collect();
    measured.sort_by(|a, b| {
        b.duration_ms
            .cmp(&a.duration_ms)
            .then_with(|| a.at.cmp(&b.at))
            .then_with(|| a.id.cmp(&b.id))
    });
    measured
        .into_iter()
        .map(|entry| RankedStep {
            id: entry.id,
            kind: entry.kind.clone(),
            label: normalized_label(entry),
            duration_ms: entry.duration_ms.unwrap_or_default(),
        })
        .collect()
}

/// The visible label of one entry: its bounded subject label, or its name when
/// the record carries no label.
fn visible_label(entry: &HistoryEntry) -> String {
    entry.label.clone().unwrap_or_else(|| entry.name.clone())
}

/// A recorded duration as the report renders it: milliseconds under a second,
/// whole seconds under a minute, whole minutes under an hour, and hours with
/// minutes above that.
pub fn format_duration(ms: u64) -> String {
    match ms {
        0..=999 => format!("{ms} ms"),
        1_000..=59_999 => format!("{} s", ms / 1_000),
        60_000..=3_599_999 => format!("{} m", ms / 60_000),
        _ => format!("{} h {:02} m", ms / 3_600_000, ms % 3_600_000 / 60_000),
    }
}

/// `ReportModel` → the HTML document: the view rendered as the page's own
/// text, and the JSON model in the asset's single data slot.
pub fn render_report(model: &ReportModel) -> String {
    let data = serde_json::to_string(model).unwrap_or_else(|_| "null".to_string());
    REPORT_ASSET
        .replacen(VIEW_SLOT, &render_view(model), 1)
        .replacen(DATA_SLOT, &json_for_slot(&data), 1)
}

/// The model as the page's own text: the scope line, and then either the
/// report's empty state — the scope holds no recorded activity — or the summary
/// and the scope's own view: the workflow stack for a change or ideation
/// report, the band rows on the time axis for the project report. The gate is
/// the scope, never the entries.
fn render_view(model: &ReportModel) -> String {
    let scope = scope_line(model.scope.as_ref());
    let mut view = format!(
        "<p class=\"report-scope\" id=\"report-scope\">{}</p>\n",
        escape(&scope)
    );
    if model.empty {
        view.push_str(&render_empty_state(&scope));
        return view;
    }
    view.push_str(&render_current_state(model.current_state.as_ref()));
    view.push_str(&render_summary(&model.summary));
    view.push('\n');
    if is_project_report(model.scope.as_ref()) {
        view.push_str(&render_timeline(model));
    } else {
        // The linked ideation group prints above the change workflow: it is
        // pre-change history, a separate labelled section, never the change's
        // current phase.
        view.push_str(&render_ideation_group(
            model.ideation.as_ref(),
            &model.entries,
        ));
        view.push_str(&render_workflow(model));
    }
    view.push('\n');
    view.push_str(&render_content_omissions(&model.omitted_content));
    view
}

/// Whether the report covers the project itself rather than a live change or
/// ideation scope: the one gate between the report's two views. A project
/// report keeps its band rows and draws no workflow stack; a change or ideation
/// report draws the workflow stack and no band timeline.
fn is_project_report(scope: Option<&Scope>) -> bool {
    matches!(scope, None | Some(Scope::Project { .. }))
}

/// The header's current-state line: exactly what the change state exposes —
/// stage label, current phase, next required artifact, and task counts — or
/// the report's own words when no live state exists. A change the project does
/// not hold and an archived change both have no live state.
fn render_current_state(state: Option<&CurrentState>) -> String {
    let Some(state) = state else {
        return "<p class=\"report-current-state\" id=\"report-current-state\">no live change state</p>\n"
            .to_string();
    };
    let phase = state.phase.as_deref().unwrap_or("underived");
    let next = state.next_artifact.as_deref().unwrap_or("none");
    let tasks = state.tasks.as_ref().map_or_else(
        || "no task progress".to_string(),
        |tasks| format!("{} of {} tasks done", tasks.done, tasks.total),
    );
    format!(
        "<p class=\"report-current-state\" id=\"report-current-state\">Current state: stage <span class=\"state-stage\" data-stage-label=\"{}\">{}</span>, phase <span class=\"state-phase\">{}</span>, next artifact <span class=\"state-next\">{}</span>, {}</p>\n",
        escape(&state.stage_label),
        escape(&state.stage_label),
        escape(phase),
        escape(next),
        escape(&tasks),
    )
}

/// The linked ideation group's text view: its heading, its six ideation phases
/// with the topic report's marker statuses, and its own totals, printed above
/// the change stack. A group whose record holds no entries states
/// [`NO_IDEATION_ACTIVITY`] instead of inventing phases or durations.
fn render_ideation_group(group: Option<&IdeationGroup>, entries: &[HistoryEntry]) -> String {
    let Some(group) = group else {
        return String::new();
    };
    let mut html = format!(
        "<section class=\"ideation-group\" id=\"ideation-group\" data-topic=\"{}\">\n<h2>Ideation: {}</h2>\n",
        escape(&group.topic),
        escape(&group.topic),
    );
    if group.activity {
        for phase in &group.phases {
            html.push_str(&render_workflow_phase(phase, entries));
        }
        html.push_str(&format!(
            "<p class=\"ideation-totals\" id=\"ideation-totals\" data-entries=\"{}\">ideation recorded {}, {} tools, {} subagents, {} waits</p>\n",
            group.entries.len(),
            escape(&duration_or_not_measured(group.duration_ms)),
            group.tools,
            group.subagents,
            group.waits,
        ));
    } else {
        html.push_str(&format!(
            "<p class=\"ideation-no-activity\" id=\"ideation-no-activity\">{NO_IDEATION_ACTIVITY}</p>\n"
        ));
    }
    html.push_str("</section>\n");
    html
}

/// The exact words a linked group states when its topic's record holds no
/// entry at all: the work predates the record, and the group invents no phase
/// or duration.
pub const NO_IDEATION_ACTIVITY: &str = "no recorded activity (pre-dates the record)";

/// The workflow stack: one line per workflow phase, and the "no recorded stage
/// transition" group beneath it when the record assigns work to no stage. The
/// project report carries no stack — its model builds none — so it keeps its
/// band rows.
fn render_workflow(model: &ReportModel) -> String {
    if is_project_report(model.scope.as_ref()) {
        return String::new();
    }
    let mut html = String::from(
        "<section class=\"workflow-stack\" id=\"workflow-stack\" aria-label=\"Workflow phases\">\n",
    );
    for phase in &model.workflow {
        html.push_str(&render_workflow_phase(phase, &model.entries));
    }
    if let Some(unstaged) = &model.unstaged {
        html.push_str(&render_unstaged_group(unstaged));
    }
    html.push_str("</section>\n");
    html
}

/// One workflow phase line: the phase's status, its wall/active/waiting time,
/// its tool and subagent counts, the names of its lanes, and its repeated-group
/// and Fit-aggregate counts, followed by the R-93 statement for every subagent
/// span whose tool calls the record never attributes to it. A phase with no
/// recorded entries states zero recorded minutes, never an invented duration.
fn render_workflow_phase(phase: &WorkflowPhase, entries: &[HistoryEntry]) -> String {
    let stage = phase
        .stage
        .as_deref()
        .unwrap_or("no recorded stage transition");
    let status = phase.status.as_deref().unwrap_or("underived");
    let wall = duration_or_not_measured(phase.wall_ms);
    let active = duration_or_not_measured(phase.active_ms);
    let waiting = duration_or_not_measured(phase.waiting_ms);
    let lanes = if phase.lanes.is_empty() {
        "no lanes".to_string()
    } else {
        phase
            .lanes
            .iter()
            .map(|lane| lane.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut html = format!(
        "<p class=\"workflow-phase\" data-phase=\"{}\" data-status=\"{}\">phase {}: {}, wall {}, active {}, waiting {}, {} tools, {} subagents, lanes: {}, {} repeated groups, {} fit aggregates</p>\n",
        escape(stage),
        escape(status),
        escape(stage),
        escape(status),
        escape(&wall),
        escape(&active),
        escape(&waiting),
        phase.tools,
        phase.subagents,
        escape(&lanes),
        phase.repeated.len(),
        phase.aggregates.len(),
    );
    for span in unattributed_spans(phase, entries) {
        html.push_str(&format!(
            "<p class=\"attribution-limit\" data-subagent=\"{}\">no tool call was attributable to subagent span \"{}\"; the record names no per-subagent tool identity for it.</p>\n",
            escape(&span.name),
            escape(&span.name),
        ));
    }
    for group in &phase.repeated {
        html.push_str(&format!(
            "<p class=\"repeated-group\" data-kind=\"{}\" data-count=\"{}\" data-duration-ms=\"{}\">repeated {} \"{}\": {} entries, combined {}</p>\n",
            escape(&group.kind),
            group.count,
            group.duration_ms.map_or_else(String::new, |duration| duration.to_string()),
            escape(&group.kind),
            escape(&group.label),
            group.count,
            duration_or_not_measured(group.duration_ms),
        ));
    }
    for tick in &phase.aggregates {
        let duration_ms = tick
            .duration_ms
            .map_or_else(String::new, |duration| duration.to_string());
        html.push_str(&format!(
            "<p class=\"phase-aggregate\" data-bucket-start-ms=\"{}\" data-bucket-end-ms=\"{}\" data-count=\"{}\" data-duration-ms=\"{}\">{} entries · {}</p>\n",
            tick.bucket_start_ms,
            tick.bucket_end_ms,
            tick.count,
            duration_ms,
            tick.count,
            duration_or_not_measured(tick.duration_ms),
        ));
    }
    html
}

/// The "no recorded stage transition" group: the work the record assigns to no
/// workflow phase, stated as the gap it is — never placed under a guessed
/// phase.
fn render_unstaged_group(phase: &WorkflowPhase) -> String {
    let count = phase
        .lanes
        .iter()
        .map(|lane| lane.entries.len())
        .sum::<usize>();
    let lanes = if phase.lanes.is_empty() {
        "no lanes".to_string()
    } else {
        phase
            .lanes
            .iter()
            .map(|lane| lane.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "<section class=\"unstaged-group\" id=\"unstaged-group\" data-entries=\"{count}\">\n<h2>No recorded stage transition</h2>\n<p class=\"unstaged-line\">{} entries carry no recorded stage transition; the record assigns them to no workflow phase.</p>\n<p class=\"unstaged-lanes\">lanes: {}</p>\n</section>\n",
        count,
        escape(&lanes),
    )
}

/// A duration as the text view states it, or the record's own words for a
/// duration it does not have.
fn duration_or_not_measured(duration: Option<u64>) -> String {
    duration.map_or_else(|| "not measured".to_string(), format_duration)
}

/// The text view's omission notes: one line per entry whose content the
/// document budget left out. The store keeps every byte, so the exact note
/// tells the reader where to find it.
fn render_content_omissions(ids: &[u64]) -> String {
    if ids.is_empty() {
        return String::new();
    }
    let mut notes =
        String::from("<section class=\"content-omitted-notes\" id=\"content-omitted-notes\">\n");
    for id in ids {
        notes.push_str(&format!(
            "<p class=\"content-omitted\" data-entry-id=\"{id}\">{}</p>\n",
            escape(CONTENT_OMITTED_NOTE)
        ));
    }
    notes.push_str("</section>");
    notes
}

/// The empty state: a scope with no recorded activity has no timeline to draw,
/// so the report root renders this instead of a blank page, an error, or an
/// axis over nothing. It is one branch of the report root — the same markup for
/// every scope that records nothing — and it carries the three things the
/// design system's empty state requires: a heading, one sentence explaining the
/// state, and a way back to the report root. No chart axis is drawn here.
fn render_empty_state(scope: &str) -> String {
    format!(
        "<section class=\"empty-state\" data-state=\"empty\" id=\"report-empty-state\" aria-labelledby=\"report-empty-title\">\n<h2 id=\"report-empty-title\">No recorded activity</h2>\n<p class=\"empty-state-detail\">No activity is recorded for {} yet, so there is no timeline to draw.</p>\n<p class=\"empty-state-back\"><a href=\"#report-title\">Back to the report root</a></p>\n</section>",
        escape(scope)
    )
}

/// The timeline's drawing width and the left gutter the band rows' labels
/// occupy, in the SVG's own units.
const WIDTH: u64 = 1000;
const GUTTER: f64 = 12.0;
const LABEL_WIDTH: f64 = 168.0;
const MARGIN: f64 = 8.0;

/// The axis's drawing width: the band rows' spans, the ticks, and the aggregate
/// markers all map the window onto this many units.
const DRAWING_WIDTH: f64 = WIDTH as f64 - LABEL_WIDTH - MARGIN;

/// One band row's height: the row's visible phase names on top, the band's
/// spans beneath them.
const BAND_HEIGHT: u64 = 68;
const MIN_SPAN: f64 = 2.0;

/// The baseline the band row's phase names sit on, measured from the row's own
/// top: above the row's spans and below the row's top edge.
const PHASE_BASELINE: u64 = 13;

/// The width one aggregate marker covers on the axis, in the SVG's own units.
/// The initial zoom draws at most one marker per bucket and band, so a record
/// of tens of thousands of entries renders a bounded number of shapes — the
/// drawing width divided by this — instead of one per entry.
const AGGREGATE_BUCKET_PX: f64 = 16.0;

/// The axis strip above the band rows, and the most ticks the initial view
/// draws, so a report of any length carries a readable axis.
const AXIS_HEIGHT: u64 = 28;
const MAX_TICKS: usize = 6;

/// The steps the axis may mark between two ticks: round times, so a tick label
/// is a time a reader recognises rather than an arbitrary instant.
const TICK_STEPS_MS: [i64; 12] = [
    1_000,
    5_000,
    15_000,
    30_000,
    60_000,
    300_000,
    900_000,
    1_800_000,
    3_600_000,
    3 * 3_600_000,
    6 * 3_600_000,
    12 * 3_600_000,
];

/// The project report's timeline section: one row per band, each row naming its
/// scope and drawing its spans across the window's time range, above an axis
/// strip of round tick labels. A change or ideation report draws the workflow
/// stack instead, and a window with no bands draws no timeline at all.
fn render_timeline(model: &ReportModel) -> String {
    if !is_project_report(model.scope.as_ref()) || model.bands.is_empty() {
        return String::new();
    }
    let scale = TimeScale::of(&model.entries);
    let height = AXIS_HEIGHT + model.bands.len() as u64 * BAND_HEIGHT + MARGIN as u64;
    let mut svg = format!(
        "<section class=\"timeline-section\" aria-label=\"Development timeline\">\n<svg class=\"timeline\" role=\"img\" aria-label=\"Development timeline\" viewBox=\"0 0 {WIDTH} {height}\">\n<title id=\"timeline-title\">Development timeline</title>\n"
    );
    svg.push_str(&render_axis(&scale));
    for (index, band) in model.bands.iter().enumerate() {
        let y = AXIS_HEIGHT + index as u64 * BAND_HEIGHT;
        svg.push_str(&render_band(y, band, model, &scale));
    }
    svg.push_str("</svg>\n</section>");
    svg
}

/// The axis strip: the window's range as round tick labels in time order, the
/// same order the band rows draw their spans in. The axis is derived from the
/// embedded timestamps alone, never from the wall clock, so the same record
/// always renders identically.
fn render_axis(scale: &TimeScale) -> String {
    let mut svg = String::from("<g class=\"axis\" aria-hidden=\"true\">\n");
    for tick in scale.ticks() {
        svg.push_str(&format!(
            "<text class=\"tick\" data-at-ms=\"{tick}\" x=\"{:.1}\" y=\"16\">{}</text>\n",
            scale.x(tick),
            scale.label(tick),
        ));
    }
    svg.push_str("</g>\n");
    svg
}

/// One band row: the row's visible label (visible text, never an internal kind
/// name) and, per entry the band covers, in the model's own order, either the
/// entry's own span or — where the entry collapsed at the initial zoom — the
/// aggregate marker standing for it, drawn once per bucket and never alongside
/// an individual span for an entry it covers.
fn render_band(y: u64, band: &Band, model: &ReportModel, scale: &TimeScale) -> String {
    let mut svg = format!(
        "<g class=\"band\" data-band-kind=\"{}\" data-band=\"{}\" aria-label=\"{}\">\n<rect class=\"band-row\" x=\"0\" y=\"{y}\" width=\"{WIDTH}\" height=\"{BAND_HEIGHT}\"></rect>\n<text class=\"band-label\" x=\"{GUTTER}\" y=\"{}\">{}</text>\n",
        escape(&band.kind),
        escape(&scope_row(&band.scope).id),
        escape(&band.label),
        y + BAND_HEIGHT / 4,
        escape(&band.label),
    );
    // The band's phases: the stages its entries worked in, in the workflow's
    // own order — the row's visible phase names, above the row's spans.
    if !band.stages.is_empty() {
        svg.push_str(&format!(
            "<text class=\"band-phases\" x=\"{LABEL_WIDTH:.1}\" y=\"{}\">{}</text>\n",
            y + PHASE_BASELINE,
            escape(&band.stages.join(" · ")),
        ));
    }
    // The band's markers, indexed by the entries they stand for: the row's one
    // drawing pass reads the index instead of searching the window's markers
    // for every entry, and every covered entry reaches exactly the marker the
    // model carries.
    let mut covered: BTreeMap<u64, &Aggregate> = BTreeMap::new();
    for marker in model
        .aggregates
        .iter()
        .filter(|marker| marker.scope == band.scope)
    {
        for id in &marker.entries {
            covered.insert(*id, marker);
        }
    }
    let mut drawn_buckets: Vec<i64> = Vec::new();
    for id in &band.entries {
        let Some(entry) = model.entries.iter().find(|entry| entry.id == *id) else {
            continue;
        };
        match covered.get(id) {
            Some(marker) => {
                if drawn_buckets.contains(&marker.bucket_start_ms) {
                    continue;
                }
                drawn_buckets.push(marker.bucket_start_ms);
                svg.push_str(&render_aggregate(marker, y, scale));
            }
            None => {
                let stage = model.stage_of(entry.id);
                svg.push_str(&render_span(entry, y, scale, stage));
            }
        }
    }
    svg.push_str("</g>\n");
    svg
}

/// One aggregate marker: its bucket's own width on the axis, naming how many
/// entries it covers and their combined duration — in the machine-readable
/// attributes the zoom path reads and in its accessible name. A click on a
/// marker zooms to its bucket instead of opening a details panel for one entry
/// that stands for many, which is why the marker never pretends to be a span:
/// it carries no `data-entry-id`.
fn render_aggregate(marker: &Aggregate, y: u64, scale: &TimeScale) -> String {
    let count = marker.entries.len();
    let duration = marker
        .duration_ms
        .map_or_else(|| "not measured".to_string(), format_duration);
    let covered = if count == 1 {
        "1 entry".to_string()
    } else {
        format!("{count} entries")
    };
    let duration_attribute = marker
        .duration_ms
        .map_or_else(String::new, |ms| format!(" data-duration-ms=\"{ms}\""));
    let (x, width) = scale.bucket(marker);
    let bar_y = y as f64 + BAND_HEIGHT as f64 / 3.0;
    format!(
        "<g class=\"aggregate\" data-count=\"{count}\"{duration_attribute} data-bucket-start-ms=\"{}\" data-bucket-end-ms=\"{}\" aria-label=\"{covered}, {duration}\">\n<rect class=\"aggregate-bar\" x=\"{x:.1}\" y=\"{bar_y:.1}\" width=\"{width:.1}\" height=\"16\"></rect>\n<text class=\"aggregate-label\" x=\"{:.1}\" y=\"{:.1}\">{count} · {duration}</text>\n</g>\n",
        marker.bucket_start_ms,
        marker.bucket_end_ms,
        x + 4.0,
        bar_y + 12.0,
    )
}

/// One span: the entry drawn on the row's axis, with its accessible name always
/// its label and its duration, and its own stage as the machine-readable
/// placement the phase assertion reads.
fn render_span(entry: &HistoryEntry, y: u64, scale: &TimeScale, stage: Option<&str>) -> String {
    let label = visible_label(entry);
    let duration = duration_text(entry);
    let (x, width) = scale.span(entry);
    let bar_y = y as f64 + BAND_HEIGHT as f64 / 3.0;
    let stage_attribute = stage.map_or_else(String::new, |stage| {
        format!(" data-stage=\"{}\"", escape(stage))
    });
    format!(
        "<g class=\"span\" data-entry-id=\"{}\" data-kind=\"{}\" data-at=\"{}\"{stage_attribute} aria-label=\"{}, {}\">\n<rect class=\"span-bar\" x=\"{x:.1}\" y=\"{bar_y:.1}\" width=\"{width:.1}\" height=\"16\"></rect>\n<text class=\"span-label\" x=\"{:.1}\" y=\"{:.1}\">{}</text>\n</g>\n",
        entry.id,
        escape(&entry.kind),
        escape(&entry.at),
        escape(&label),
        escape(&duration),
        x + 4.0,
        bar_y + 12.0,
        escape(&format!("{label} · {duration}")),
    )
}

/// The duration one entry renders: its measured duration, or the record's own
/// word for a duration it does not have.
fn duration_text(entry: &HistoryEntry) -> String {
    entry
        .duration_ms
        .map_or_else(|| "not measured".to_string(), format_duration)
}

/// The report's time axis: the window's recorded range, mapped onto the
/// timeline's drawing width. The axis is derived from the embedded timestamps
/// alone — never from the wall clock — so the same record always renders
/// identically.
#[derive(Debug, Clone, Copy)]
struct TimeScale {
    /// The window's earliest recorded end.
    start_ms: i64,
    /// The window's span, never zero (a window that spans no time draws every
    /// span at its start).
    span_ms: i64,
}

impl TimeScale {
    /// The scale over the window's entries.
    fn of(entries: &[HistoryEntry]) -> Self {
        Self::over(
            entries
                .iter()
                .filter_map(|entry| crate::history::parse_rfc3339_ms(&entry.at)),
        )
    }

    /// The scale over the given entries, borrowed: the linked ideation group's
    /// own window, never the change's.
    fn of_refs(entries: &[&HistoryEntry]) -> Self {
        Self::over(
            entries
                .iter()
                .filter_map(|entry| crate::history::parse_rfc3339_ms(&entry.at)),
        )
    }

    /// The scale over one window's recorded instants.
    fn over(mut times: impl Iterator<Item = i64>) -> Self {
        let first = times.next().unwrap_or_default();
        let (mut start, mut end) = (first, first);
        for time in times {
            start = start.min(time);
            end = end.max(time);
        }
        TimeScale {
            start_ms: start,
            span_ms: (end - start).max(1),
        }
    }

    /// The x coordinate of one recorded instant.
    fn x(&self, at_ms: i64) -> f64 {
        let fraction = (at_ms - self.start_ms) as f64 / self.span_ms as f64;
        let x = LABEL_WIDTH + fraction * (WIDTH as f64 - LABEL_WIDTH - MARGIN);
        x.clamp(LABEL_WIDTH, WIDTH as f64 - MARGIN)
    }

    /// The x and width one entry's span draws at: a measured span starts at its
    /// recorded end minus its duration, and an entry the record does not time
    /// draws as the narrowest visible mark.
    fn span(&self, entry: &HistoryEntry) -> (f64, f64) {
        let Some(end) = crate::history::parse_rfc3339_ms(&entry.at) else {
            return (LABEL_WIDTH, MIN_SPAN);
        };
        let start = match entry.duration_ms {
            Some(duration_ms) => end - i64::try_from(duration_ms).unwrap_or_default(),
            None => end,
        };
        let x = self.x(start);
        let width = (self.x(end) - x).max(MIN_SPAN);
        (x, width)
    }

    /// The x and width one aggregate's bucket draws at: the bucket's own
    /// instants mapped onto the axis, so the markers of a band tile its axis
    /// instead of overlapping, and a bucket never draws narrower than the
    /// narrowest mark.
    fn bucket(&self, marker: &Aggregate) -> (f64, f64) {
        let x = self.x(marker.bucket_start_ms);
        let width = (self.x(marker.bucket_end_ms) - x).max(MIN_SPAN);
        (x, width)
    }

    /// The time one pixel of the drawing width spends on at the initial zoom.
    /// An entry drawn shorter than this cannot be told apart from its
    /// neighbours, which is the aggregation rule's threshold — and the report's
    /// one place that reads it.
    fn time_per_pixel(&self) -> i64 {
        (self.span_ms as f64 / DRAWING_WIDTH).ceil() as i64
    }

    /// The axis's ticks: round instants across the window's range, in time
    /// order, none of them outside the range. The step is the smallest one that
    /// keeps the count within [`MAX_TICKS`], so a short window marks seconds and
    /// a long one marks hours; a window shorter than the smallest step gets no
    /// tick at all rather than a tick outside it.
    fn ticks(&self) -> Vec<i64> {
        let end = self.start_ms + self.span_ms;
        let Some(step) = TICK_STEPS_MS
            .iter()
            .copied()
            .find(|step| self.span_ms / step <= MAX_TICKS as i64)
        else {
            return Vec::new();
        };
        let mut ticks = Vec::new();
        let mut tick = self.start_ms.div_euclid(step) * step;
        while tick <= end && ticks.len() < MAX_TICKS {
            if tick >= self.start_ms {
                ticks.push(tick);
            }
            tick += step;
        }
        ticks
    }

    /// A tick's label: the time of day the tick marks.
    fn label(&self, at_ms: i64) -> String {
        let minute_of_day = at_ms.div_euclid(60_000).rem_euclid(24 * 60);
        format!("{:02}:{:02}", minute_of_day / 60, minute_of_day % 60)
    }
}

/// The report's scope line: the scope the report covers, named by its own id.
fn scope_line(scope: Option<&Scope>) -> String {
    match scope {
        Some(scope) => scope_row(scope).label,
        None => "Project report".to_string(),
    }
}

/// The summary section: one tile per number, the ranked longest steps, and a
/// note per duration the record does not have.
fn render_summary(summary: &Summary) -> String {
    let mut html = String::from("<section class=\"summary\" aria-label=\"Summary\">\n");
    html.push_str(&tile(
        "elapsed",
        "Elapsed",
        &summary
            .elapsed_ms
            .map_or_else(|| "not measured".to_string(), format_duration),
        None,
    ));
    html.push('\n');
    html.push_str(&tile(
        "entries",
        "Entries",
        &summary.entries.to_string(),
        None,
    ));
    html.push('\n');
    html.push_str(&tile(
        "waits",
        "Waiting",
        &summary
            .wait_ms
            .map_or_else(|| "not measured".to_string(), format_duration),
        Some(&wait_note(summary)),
    ));
    html.push('\n');
    html.push_str(&tile(
        "active",
        "Active",
        &summary
            .active_ms
            .map_or_else(|| "not measured".to_string(), format_duration),
        None,
    ));
    html.push('\n');
    html.push_str(&tile(
        "tool-calls",
        "Tool calls",
        &summary.tool_calls.to_string(),
        None,
    ));
    html.push('\n');
    html.push_str(&tile(
        "cost",
        "Cost",
        &summary
            .cost_total
            .map_or_else(|| "unavailable".to_string(), |cost| format!("{cost:.2}")),
        None,
    ));
    html.push('\n');
    html.push_str(&tile(
        "subagents",
        "Subagents",
        &summary
            .subagent_ms
            .map_or_else(|| "not measured".to_string(), format_duration),
        Some(&subagent_note(summary)),
    ));
    html.push('\n');
    html.push_str(&tile(
        "tokens",
        "Tokens",
        &summary
            .tokens_total
            .map_or_else(|| USAGE_UNAVAILABLE.to_string(), format_count),
        None,
    ));
    html.push('\n');
    if summary.unmeasured_durations > 0 {
        html.push_str(&format!(
            "<p class=\"summary-note\" data-unmeasured-durations=\"{}\">{}</p>\n",
            summary.unmeasured_durations,
            unmeasured_note(summary.unmeasured_durations)
        ));
    }
    if summary.unmeasured_turns > 0 {
        html.push_str(&format!(
            "<p class=\"summary-note\" data-unmeasured-turns=\"{}\">{}</p>\n",
            summary.unmeasured_turns,
            unmeasured_turns_note(summary.unmeasured_turns)
        ));
    }
    html.push_str("<h2>Longest steps</h2>\n");
    html.push_str("<ol class=\"longest-steps\" id=\"longest-steps\">\n");
    for step in &summary.longest_steps {
        html.push_str(&ranked_row(step));
    }
    html.push_str("</ol>\n");
    html.push_str("<h2>Tokens per model</h2>\n");
    html.push_str(&token_list(
        "tokens-by-model",
        "data-model",
        &summary.tokens_by_model,
    ));
    html.push('\n');
    html.push_str("<h2>Tokens per subagent</h2>\n");
    html.push_str(&token_list(
        "tokens-by-subagent",
        "data-subagent",
        &summary.tokens_by_subagent,
    ));
    html.push_str("\n</section>");
    html
}

/// One row of the ranking list: the entry's visible label, the kind that
/// recorded it, and its rendered duration. The report renders every ranked
/// entry through this one function, so the two markers the row carries
/// (`data-entry-id`, `data-kind`) and the three elements stay in one place.
fn ranked_row(step: &RankedStep) -> String {
    format!(
        "<li data-entry-id=\"{}\" data-kind=\"{}\" data-duration-ms=\"{}\"><span class=\"step-label\">{}</span><span class=\"step-kind\">{}</span><span class=\"step-duration\">{}</span></li>\n",
        step.id,
        escape(&step.kind),
        step.duration_ms,
        escape(&step.label),
        escape(&step.kind),
        format_duration(step.duration_ms),
    )
}

/// One token list: one row per model or subagent, naming the account and the
/// tokens the record reports for it. A row the record reports no usage for
/// renders "unavailable", never a zero.
fn token_list(id: &str, attribute: &str, totals: &[TokenTotal]) -> String {
    let mut html = format!("<ul class=\"token-totals\" id=\"{id}\">\n");
    for total in totals {
        html.push_str(&format!(
            "<li {attribute}=\"{}\"><span class=\"token-name\">{}</span><span class=\"token-value\">{}</span></li>\n",
            escape(&total.name),
            escape(&total.name),
            total
                .tokens
                .map_or_else(|| USAGE_UNAVAILABLE.to_string(), format_count),
        ));
    }
    html.push_str("</ul>");
    html
}

/// An exact count as the report renders it (thousands separated). A summary
/// figure that hid its magnitude behind a rounded unit would be worse than a
/// long one.
pub fn format_count(count: u64) -> String {
    let digits = count.to_string();
    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.char_indices() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            formatted.push(',');
        }
        formatted.push(digit);
    }
    formatted
}

/// The note the turns without reported usage render as: the count and the
/// missing data's own word, never a silent zero.
fn unmeasured_turns_note(turns: usize) -> String {
    if turns == 1 {
        "1 turn without token usage: unavailable".to_string()
    } else {
        format!("{turns} turns without token usage: unavailable")
    }
}

/// The waiting tile's note: how many waits the record holds, and — where the
/// record does not time some of them — that they are not measured, so a missing
/// duration is never read as a zero wait.
fn wait_note(summary: &Summary) -> String {
    let mut note = counted(summary.waits, "wait");
    if summary.unmeasured_waits > 0 {
        note.push_str(&format!(" · {} not measured", summary.unmeasured_waits));
    }
    note
}

/// The subagents tile's note: how many subagent spans the record holds — the
/// agent's fan-out, so "a dozen tiny subagents whose combined overhead exceeds
/// the work" is visible — and, where the record does not time some of them,
/// that they are not measured, so a missing duration is never read as a
/// zero-length subagent.
fn subagent_note(summary: &Summary) -> String {
    let mut note = counted(summary.subagents, "subagent");
    if summary.unmeasured_subagents > 0 {
        note.push_str(&format!(" · {} not measured", summary.unmeasured_subagents));
    }
    note
}

/// A counted noun as the report renders it: the count and the noun, in the
/// singular for one. Every duration the summary totals renders its count this
/// way, so a waiter, a subagent, and any later span kind can never disagree
/// about how a count reads.
fn counted(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}

/// The note a duration the record does not carry renders as: missing data is
/// stated in words, never summed as zero.
fn unmeasured_note(count: usize) -> String {
    format!("{count} recorded duration not measured")
}

/// One summary tile: a written label, its rendered value, and an optional
/// note. A zero value renders `0`, never a blank.
fn tile(name: &str, label: &str, value: &str, note: Option<&str>) -> String {
    let note = match note {
        Some(note) => format!("<span class=\"tile-note\">{}</span>", escape(note)),
        None => String::new(),
    };
    format!(
        "<div class=\"tile\" data-tile=\"{}\"><span class=\"tile-label\">{}</span><span class=\"tile-value\">{}</span>{}</div>",
        escape(name),
        escape(label),
        escape(value),
        note,
    )
}

/// The JSON the data slot holds: valid JSON that can never close the element it
/// is embedded in, whatever a recorded label carries.
fn json_for_slot(json: &str) -> String {
    json.replace('<', "\\u003c")
}

/// The report's text escaping: every recorded string reaches the document
/// escaped, so a label can never become markup.
fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

/// The link snapshot a change report carries, resolved once from the change's
/// proposal: the topic the proposal names, when that topic's directory exists.
/// `None` means no link — a `none` line, no line, an absent topic directory, or
/// no readable proposal. No other file, project entry, time proximity, or
/// mutable topic state is consulted.
fn resolve_link(root: &Path, change: &str) -> Option<LinkedIdeation> {
    let text = std::fs::read_to_string(proposal_path(root, change)?).ok()?;
    let topic = link_value_from_proposal(&text)?;
    if !root.join("givn").join("ideation").join(&topic).is_dir() {
        return None;
    }
    let entries = crate::history::HistoryStore::at(crate::history::store_path(root))
        .read()
        .ok()?
        .into_iter()
        .filter(|entry| entry.scope == Scope::Ideation { id: topic.clone() })
        .collect();
    Some(LinkedIdeation { topic, entries })
}

/// The proposal a change report reads its link from: the live change's
/// `proposal.md`, or — when the live change directory does not exist — the
/// archived change's. A live directory always wins, even when the archive
/// holds a second copy.
fn proposal_path(root: &Path, change: &str) -> Option<PathBuf> {
    let live = root.join("givn").join("changes").join(change);
    if live.is_dir() {
        return Some(live.join("proposal.md"));
    }
    let archived = root
        .join("givn")
        .join("archive")
        .join(change)
        .join("proposal.md");
    archived.is_file().then_some(archived)
}

/// The topic a proposal's own lines name: `Ideation topic:` first, `Seed path:`
/// only when the topic line is absent. A present topic line decides even when it
/// says `none`, so a stale seed never overrides an explicit no-link.
fn link_value_from_proposal(text: &str) -> Option<String> {
    match proposal_field(text, "Ideation topic:") {
        Some(value) => normalize_link_value(&value),
        None => proposal_field(text, "Seed path:").and_then(|value| normalize_link_value(&value)),
    }
}

/// One proposal field's value: the line whose trimmed text (minus a list
/// marker) starts with `field`. The first such line wins.
fn proposal_field(text: &str, field: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let line = line.trim();
        let line = line
            .strip_prefix("- ")
            .or_else(|| line.strip_prefix("* "))
            .or_else(|| line.strip_prefix("+ "))
            .unwrap_or(line);
        line.strip_prefix(field)
            .map(|value| value.trim().to_string())
    })
}

/// One proposal link value, normalized before any existence check: surrounding
/// whitespace and backticks/quotes are stripped; a value whose first token is
/// `none` (including `none — <reason>`) means no link; a bare `<id>` or a
/// `givn/ideation/<id>` path (with or without a trailing slash) names the topic.
/// A seed path names the topic by its `givn/ideation/<topic>/` segment, so the
/// seed's own filename is never validated (both `<uc-id>` and `<change-id>` seed
/// names occur).
fn normalize_link_value(value: &str) -> Option<String> {
    let value = value
        .trim()
        .trim_matches(|character| matches!(character, '`' | '"' | '\''))
        .trim();
    if value.is_empty() {
        return None;
    }
    if value.split_whitespace().next() == Some("none") {
        return None;
    }
    let segments: Vec<&str> = value
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() >= 3 && segments[0] == "givn" && segments[1] == "ideation" {
        return Some(segments[2].to_string());
    }
    segments.last().map(|segment| (*segment).to_string())
}

/// `givn timeline` — write the report for one scope and print its path. With
/// `--out` omitted the report goes to the project's own default path
/// `givn/history/reports/<scope>.html`.
pub fn run(change: Option<&str>, topic: Option<&str>, out: Option<&Path>) -> Result<PathBuf> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let scope = crate::cli::history::scope_for(&root, change, topic);
    let entries = crate::cli::history::entries_in_scope(&root, scope.as_ref())?;
    // The link is resolved once, from the change's proposal, and embedded in
    // the model: the written document is deterministic over the record, the
    // change state, and that snapshot. A topic or project report parses none.
    let link = match &scope {
        Some(Scope::Change { id }) => resolve_link(&root, id),
        _ => None,
    };
    let model = build_report_model(scope.clone(), entries, link);
    let path = match out {
        Some(path) => path.to_path_buf(),
        None => default_report_path(&root, scope.as_ref()),
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create the report directory {}", parent.display()))?;
    }
    std::fs::write(&path, render_report(&model))
        .with_context(|| format!("write the report to {}", path.display()))?;
    println!("{}", path.display());
    Ok(path)
}

/// The report's default path: `givn/history/reports/<scope>.html`.
fn default_report_path(root: &Path, scope: Option<&Scope>) -> PathBuf {
    let id = match scope {
        Some(scope) => scope_row(scope).id,
        None => "project".to_string(),
    };
    root.join("givn")
        .join("history")
        .join("reports")
        .join(format!("{id}.html"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One record line of the unit fixture, scoped to the `demo` change.
    #[derive(Default)]
    struct FixtureEntry<'a> {
        id: u64,
        at: &'a str,
        kind: &'a str,
        name: &'a str,
        label: Option<&'a str>,
        duration_ms: Option<u64>,
        session: Option<&'a str>,
        data: Option<serde_json::Value>,
    }

    impl FixtureEntry<'_> {
        fn record(self) -> HistoryEntry {
            HistoryEntry {
                schema: crate::history::SCHEMA,
                id: self.id,
                at: self.at.to_string(),
                scope: Scope::Change {
                    id: "demo".to_string(),
                },
                source: "givn".to_string(),
                kind: self.kind.to_string(),
                name: self.name.to_string(),
                label: self.label.map(str::to_string),
                duration_ms: self.duration_ms,
                outcome: "ok".to_string(),
                parent: None,
                session: self.session.map(str::to_string),
                data: self.data,
            }
        }

        /// The same record line scoped to the given scope: the project report's
        /// own fixture needs a project-scoped entry.
        fn record_in(self, scope: Scope) -> HistoryEntry {
            HistoryEntry {
                scope,
                ..self.record()
            }
        }
    }

    /// The shared scaffold's proof: the model carries the nine-phase workflow
    /// stack, its lanes, repeated groups, and Fit aggregates as ids and totals
    /// only — the record stays embedded once — and the three KPI totals the KPI
    /// strip states (active time, tool calls, cost) are the record's own.
    #[test]
    fn hybrid_model_carries_the_phase_scaffold_and_kpi_totals() {
        let entries = vec![
            FixtureEntry {
                id: 1,
                at: "2026-10-17T10:00:00.000Z",
                kind: "question",
                name: "question",
                label: Some("Which scope?"),
                duration_ms: Some(30_000),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 2,
                at: "2026-10-17T10:00:30.000Z",
                kind: "phase",
                name: "proposal.md",
                label: Some("proposal.md"),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 3,
                at: "2026-10-17T10:00:32.000Z",
                kind: "tool",
                name: "task",
                label: Some("task"),
                duration_ms: Some(500),
                session: Some("s1"),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 4,
                at: "2026-10-17T10:00:40.000Z",
                kind: "turn",
                name: "turn",
                label: Some("turn"),
                duration_ms: Some(1_000),
                session: Some("s1"),
                data: Some(serde_json::json!({ "cost": 0.25 })),
            }
            .record(),
            FixtureEntry {
                id: 5,
                at: "2026-10-17T10:00:41.000Z",
                kind: "turn",
                name: "turn",
                label: Some("turn"),
                duration_ms: Some(500),
                session: Some("s1"),
                data: Some(serde_json::json!({ "cost": { "total": 0.5, "list": 0.75 } })),
            }
            .record(),
        ];
        let model = build_report_model(
            Some(Scope::Change {
                id: "demo".to_string(),
            }),
            entries.clone(),
            None,
        );

        assert_eq!(
            model.entries.len(),
            entries.len(),
            "the scaffold re-embedded the record: it must hold ids and totals only"
        );
        assert!(
            model.current_state.is_none(),
            "a change the project does not hold has no live state: {:?}",
            model.current_state
        );

        let stages: Vec<&str> = model
            .workflow
            .iter()
            .map(|phase| phase.stage.as_deref().unwrap_or_default())
            .collect();
        assert_eq!(
            stages,
            WORKFLOW_STAGES.to_vec(),
            "the stack is not the nine workflow phases in workflow order"
        );

        let ids: Vec<u64> = model.entries.iter().map(|entry| entry.id).collect();
        for phase in &model.workflow {
            for lane in &phase.lanes {
                assert!(
                    !lane.entries.is_empty(),
                    "the `{:?}` phase carries an empty lane",
                    phase.stage
                );
                for id in &lane.entries {
                    assert!(
                        ids.contains(id),
                        "lane member {id} is not in the model's entries"
                    );
                }
            }
            for group in &phase.repeated {
                for id in &group.entries {
                    assert!(
                        ids.contains(id),
                        "group member {id} is not in the model's entries"
                    );
                }
            }
            for aggregate in &phase.aggregates {
                assert_eq!(
                    aggregate.count,
                    aggregate.entries.len(),
                    "an aggregate's count disagrees with its member ids"
                );
                for id in &aggregate.entries {
                    assert!(
                        ids.contains(id),
                        "aggregate member {id} is not in the model's entries"
                    );
                }
            }
        }

        let specs = model
            .workflow
            .iter()
            .find(|phase| phase.stage.as_deref() == Some("specs"))
            .expect("the proposal.md transition opens the specs phase");
        assert_eq!(specs.tools, 1, "the phase's tool count");
        assert_eq!(specs.subagents, 0, "the phase's subagent count");
        assert_eq!(specs.wall_ms, Some(11_000), "the phase's wall time");
        assert_eq!(specs.waiting_ms, Some(0), "the phase holds no wait");
        assert_eq!(specs.active_ms, Some(11_000), "the phase's active time");
        let session_lane = specs
            .lanes
            .iter()
            .find(|lane| lane.session.as_deref() == Some("s1"))
            .expect("the tool entry's session has no lane");
        assert_eq!(session_lane.name, "s1", "the lane's short-id name");
        assert!(
            session_lane.entries.contains(&3),
            "the tool entry is not in its session's lane"
        );
        let unattributed = specs
            .lanes
            .iter()
            .find(|lane| lane.session.is_none())
            .expect("the transition entry has no unattributed lane");
        assert_eq!(unattributed.name, "unattributed");
        assert!(
            unattributed.entries.contains(&2),
            "the transition entry is not in the unattributed lane"
        );

        assert_eq!(model.summary.elapsed_ms, Some(41_000));
        assert_eq!(model.summary.wait_ms, Some(30_000));
        assert_eq!(
            model.summary.active_ms,
            Some(11_000),
            "active is elapsed minus waiting when both are measured"
        );
        assert_eq!(model.summary.tool_calls, 1, "the number of tool entries");
        assert_eq!(model.summary.subagents, 0);
        assert_eq!(
            model.summary.cost_total,
            Some(0.75),
            "the numeric cost and the cost object's total are summed"
        );
        assert!(
            model.unstaged.is_none(),
            "the no-stage group is absent when every entry has a stage"
        );

        let view = render_view(&model);
        assert!(
            view.contains("data-phase=\"specs\""),
            "the text view prints no specs phase line: {view}"
        );
        assert!(
            view.contains("no live change state"),
            "the text view does not state the absent live state: {view}"
        );

        let payload = serde_json::to_value(&model).expect("the model serializes");
        assert_eq!(
            payload["workflow"].as_array().map(Vec::len),
            Some(WORKFLOW_STAGES.len()),
            "the model payload carries the nine phases"
        );
        assert_eq!(payload["summary"]["tool_calls"], serde_json::json!(1));
        assert_eq!(payload["summary"]["cost_total"], serde_json::json!(0.75));
    }

    /// The scope gate's production proof: the same record renders the workflow
    /// stack and no band timeline when the report is scoped to a change, and
    /// the band timeline and no workflow stack when it is scoped to the
    /// project. The entries are identical, so only the scope decides.
    #[test]
    fn text_view_is_gated_on_the_scope() {
        let change_entry = FixtureEntry {
            id: 1,
            at: "2026-10-17T10:00:00.000Z",
            kind: "step",
            name: "cargo test",
            label: Some("cargo test"),
            duration_ms: Some(1_000),
            ..FixtureEntry::default()
        }
        .record();
        let change = build_report_model(
            Some(Scope::Change {
                id: "demo".to_string(),
            }),
            vec![change_entry],
            None,
        );
        let change_view = render_view(&change);
        for marker in [
            "class=\"workflow-stack\"",
            "data-phase=\"propose\"",
            "data-phase=\"archive\"",
        ] {
            assert!(
                change_view.contains(marker),
                "the change report's text view does not draw {marker}: {change_view}"
            );
        }
        for marker in [
            "<svg",
            "class=\"timeline-section\"",
            "class=\"band\"",
            "class=\"span\"",
            "class=\"axis\"",
        ] {
            assert!(
                !change_view.contains(marker),
                "the change report's text view still draws {marker}: {change_view}"
            );
        }

        let project_entry = FixtureEntry {
            id: 2,
            at: "2026-10-17T10:00:00.000Z",
            kind: "step",
            name: "cargo build",
            label: Some("cargo build"),
            duration_ms: Some(2_000),
            ..FixtureEntry::default()
        }
        .record_in(Scope::Project {
            id: "demo".to_string(),
        });
        let project = build_report_model(
            Some(Scope::Project {
                id: "demo".to_string(),
            }),
            vec![project_entry],
            None,
        );
        let project_view = render_view(&project);
        for marker in [
            "<svg",
            "class=\"timeline-section\"",
            "class=\"band\"",
            "class=\"span\"",
            "class=\"axis\"",
        ] {
            assert!(
                project_view.contains(marker),
                "the project report's text view does not draw {marker}: {project_view}"
            );
        }
        for marker in ["class=\"workflow-stack\"", "data-phase=", "data-status="] {
            assert!(
                !project_view.contains(marker),
                "the project report's text view still draws {marker}: {project_view}"
            );
        }
        assert!(
            project.workflow.is_empty() && project.unstaged.is_none(),
            "the project report builds a workflow scaffold: {:?}",
            project.workflow
        );
    }

    /// The opening-stage rule's production proof: an entry recorded before a
    /// band's first transition stays in the opening stage only when that
    /// transition is `proposal.md`. When the first transition is later, the
    /// early work belongs to the one no-stage group — never under a guessed
    /// `propose` phase.
    #[test]
    fn entries_before_a_later_first_transition_belong_to_no_stage() {
        let entries = vec![
            FixtureEntry {
                id: 1,
                at: "2026-10-17T09:00:00.000Z",
                kind: "step",
                name: "early",
                label: Some("early work"),
                duration_ms: Some(1_000),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 2,
                at: "2026-10-17T10:00:00.000Z",
                kind: "phase",
                name: "design.md",
                label: Some("design.md"),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 3,
                at: "2026-10-17T10:01:00.000Z",
                kind: "step",
                name: "later",
                label: Some("later work"),
                duration_ms: Some(2_000),
                ..FixtureEntry::default()
            }
            .record(),
        ];
        let model = build_report_model(
            Some(Scope::Change {
                id: "demo".to_string(),
            }),
            entries,
            None,
        );
        let early = model
            .stages
            .iter()
            .find(|assignment| assignment.entry == 1)
            .expect("the early entry has a stage assignment");
        assert!(
            early.stage.is_none(),
            "the early work is placed in an invented opening phase: {early:?}"
        );
        let later = model
            .stages
            .iter()
            .find(|assignment| assignment.entry == 3)
            .expect("the later entry has a stage assignment");
        assert_eq!(
            later.stage.as_deref(),
            Some("arc42"),
            "design.md opens the arc42 stage"
        );

        let unstaged = model
            .unstaged
            .as_ref()
            .expect("the early work has no no-stage group");
        assert!(
            unstaged.stage.is_none(),
            "the no-stage group carries a name"
        );
        assert!(
            unstaged.status.is_none(),
            "the no-stage group carries a status"
        );
        let members: Vec<u64> = unstaged
            .lanes
            .iter()
            .flat_map(|lane| lane.entries.iter().copied())
            .collect();
        assert_eq!(
            members,
            vec![1],
            "the no-stage group does not hold exactly the early work"
        );

        let propose = model
            .workflow
            .iter()
            .find(|phase| phase.stage.as_deref() == Some("propose"))
            .expect("the stack carries the propose phase");
        assert!(
            propose.lanes.iter().all(|lane| lane.entries.is_empty()),
            "the report places the early work under a guessed opening phase"
        );
        let view = render_view(&model);
        assert!(
            view.contains("no recorded stage transition"),
            "the text view does not state the gap: {view}"
        );
    }

    /// The recorder's own invocations are recording infrastructure: the model
    /// drops exactly them as its first step — before the content budget and
    /// every derivation — so they appear in no entry list, in no total, in no
    /// ranking row, and in no omission note, while every boundary case that
    /// merely spells the phrase survives.
    #[test]
    fn report_model_omits_only_the_recorder_command_entries() {
        // The flagged intake carries content larger than the whole document
        // budget: dropped before the budget is applied, it is never omitted
        // from a document it is not part of.
        let oversized = "x".repeat(CONTENT_BUDGET_BYTES + 1);
        let entries = vec![
            FixtureEntry {
                id: 1,
                at: "2026-10-16T09:00:00.000Z",
                kind: "tool",
                name: "Bash",
                label: Some("cargo test --locked"),
                duration_ms: Some(2_400_000),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 2,
                at: "2026-10-16T09:10:00.000Z",
                kind: "command",
                name: "status",
                label: Some("status --change demo"),
                duration_ms: Some(90_000),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 3,
                at: "2026-10-16T09:20:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history ingest"),
                duration_ms: Some(40),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 4,
                at: "2026-10-16T09:21:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history ingest --with-full-content"),
                duration_ms: Some(40),
                data: Some(serde_json::json!({
                    "content": { "tool_output": { "text": oversized } }
                })),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 5,
                at: "2026-10-16T09:22:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history harness-hook --harness claude"),
                duration_ms: Some(60),
                ..FixtureEntry::default()
            }
            .record(),
            // The boundary survivors: another command, a substring, and an
            // entry of another kind or name that quotes the phrase.
            FixtureEntry {
                id: 6,
                at: "2026-10-16T09:23:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history ingest-extra"),
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 7,
                at: "2026-10-16T09:24:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history"),
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 8,
                at: "2026-10-16T09:25:00.000Z",
                kind: "command",
                name: "history",
                label: Some("history settings --json"),
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 9,
                at: "2026-10-16T09:26:00.000Z",
                kind: "tool",
                name: "Bash",
                label: Some("history ingest"),
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 10,
                at: "2026-10-16T09:27:00.000Z",
                kind: "command",
                name: "history",
                label: None,
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
            FixtureEntry {
                id: 11,
                at: "2026-10-16T09:28:00.000Z",
                kind: "command",
                name: "commit",
                label: Some("commit -m \"history ingest\""),
                duration_ms: Some(10),
                ..FixtureEntry::default()
            }
            .record(),
        ];
        let recorder = [3u64, 4, 5];
        let survivors = [1u64, 2, 6, 7, 8, 9, 10, 11];
        let model = build_report_model(
            Some(Scope::Change {
                id: "demo".to_string(),
            }),
            entries,
            None,
        );

        let ids: Vec<u64> = model.entries.iter().map(|entry| entry.id).collect();
        for id in recorder {
            assert!(
                !ids.contains(&id),
                "the recorder's own entry {id} is in the model's entries: {ids:?}"
            );
        }
        for id in survivors {
            assert!(
                ids.contains(&id),
                "the entry {id} that merely spells the phrase was dropped: {ids:?}"
            );
        }
        let labels: Vec<Option<&str>> = model
            .entries
            .iter()
            .filter(|entry| entry.kind == "command")
            .map(|entry| entry.label.as_deref())
            .collect();
        for label in [
            "history ingest",
            "history ingest --with-full-content",
            "history harness-hook --harness claude",
        ] {
            assert!(
                !labels.contains(&Some(label)),
                "the model still names the recorder's `{label}` command entry: {labels:?}"
            );
        }
        // The `tool` that merely carries the intake's label is work, not the
        // intake: it survives with its label intact.
        assert!(
            model
                .entries
                .iter()
                .any(|entry| entry.id == 9 && entry.label.as_deref() == Some("history ingest")),
            "the tool labelled like the intake was dropped: {ids:?}"
        );

        assert_eq!(
            model.summary.entries,
            survivors.len(),
            "the summary counts the recorder's own entries"
        );
        assert!(!model.empty, "the filtered record still holds work");
        let ranked: Vec<u64> = model
            .summary
            .longest_steps
            .iter()
            .map(|step| step.id)
            .collect();
        for id in recorder {
            assert!(
                !ranked.contains(&id),
                "the ranking lists the recorder's own entry {id}: {ranked:?}"
            );
        }
        for id in survivors {
            assert!(
                ranked.contains(&id),
                "the ranked list lost the surviving entry {id}: {ranked:?}"
            );
        }
        let ranks: Vec<&str> = model
            .summary
            .longest_steps
            .iter()
            .filter(|step| step.kind == "command")
            .map(|step| step.label.as_str())
            .collect();
        for label in [
            "history ingest",
            "history ingest --with-full-content",
            "history harness-hook --harness claude",
        ] {
            assert!(
                !ranks.contains(&label),
                "the ranking names the recorder's own `{label}` invocation: {ranks:?}"
            );
        }

        // The budget ran after the omission: the oversized recorder content is
        // gone with its entry, so nothing is named as omitted and no content
        // byte is accounted for.
        assert!(
            model.omitted_content.is_empty(),
            "the recorder's own content was budgeted: {:?}",
            model.omitted_content
        );
        assert_eq!(
            model.content_bytes, 0,
            "the recorder's own content was counted against the budget"
        );
    }

    /// The ideation model's proof: the six ideation phases in the topic's own
    /// order, the direct marker-to-phase mapping (a `personas` marker places its
    /// own work in `personas`, never in the change stage the marker would open
    /// there), pre-marker entries in `frame` with no no-stage group, and the
    /// report scope's totals kept separate from a linked topic's.
    #[test]
    fn ideation_model_maps_markers_to_phases_and_keeps_scope_totals() {
        let topic = Scope::Ideation {
            id: "ideas".to_string(),
        };
        let topic_entries = vec![
            FixtureEntry {
                id: 1,
                at: "2026-10-16T08:00:00.000Z",
                kind: "step",
                name: "ideation",
                duration_ms: Some(10_000),
                ..FixtureEntry::default()
            }
            .record_in(topic.clone()),
            FixtureEntry {
                id: 2,
                at: "2026-10-16T09:00:00.000Z",
                kind: "phase",
                name: "personas",
                label: Some("personas"),
                ..FixtureEntry::default()
            }
            .record_in(topic.clone()),
            FixtureEntry {
                id: 3,
                at: "2026-10-16T10:00:00.000Z",
                kind: "step",
                name: "personas-work",
                duration_ms: Some(5_000),
                ..FixtureEntry::default()
            }
            .record_in(topic.clone()),
            FixtureEntry {
                id: 4,
                at: "2026-10-16T11:00:00.000Z",
                kind: "phase",
                name: "event-storming",
                label: Some("event-storming"),
                ..FixtureEntry::default()
            }
            .record_in(topic.clone()),
            FixtureEntry {
                id: 5,
                at: "2026-10-16T12:00:00.000Z",
                kind: "tool",
                name: "tool",
                duration_ms: Some(7_000),
                ..FixtureEntry::default()
            }
            .record_in(topic.clone()),
        ];

        // The topic report: the six ideation phases in order, with the furthest
        // marker active and every earlier phase done.
        let topic_model = build_report_model(Some(topic.clone()), topic_entries.clone(), None);
        let stages: Vec<Option<String>> = topic_model
            .workflow
            .iter()
            .map(|phase| phase.stage.clone())
            .collect();
        assert_eq!(
            stages,
            IDEATION_STAGES
                .iter()
                .map(|stage| Some((*stage).to_string()))
                .collect::<Vec<_>>(),
            "the topic report's workflow is not the six ideation phases in order"
        );
        let statuses: Vec<Option<String>> = topic_model
            .workflow
            .iter()
            .map(|phase| phase.status.clone())
            .collect();
        assert_eq!(
            statuses,
            vec![
                Some("done".to_string()),
                Some("done".to_string()),
                Some("active".to_string()),
                Some("queued".to_string()),
                Some("queued".to_string()),
                Some("queued".to_string()),
            ],
            "the topic report's marker statuses are not the furthest marker active"
        );
        assert_eq!(
            topic_model.stage_of(1).map(str::to_string),
            Some("frame".to_string()),
            "a pre-marker entry is not in the opening frame phase"
        );
        assert_eq!(
            topic_model.stage_of(2).map(str::to_string),
            Some("personas".to_string()),
            "a `personas` marker did not assign its own work to `personas`"
        );
        assert_eq!(
            topic_model.stage_of(3).map(str::to_string),
            Some("personas".to_string()),
            "work after the `personas` marker is not in `personas`"
        );
        assert_eq!(
            topic_model.stage_of(4).map(str::to_string),
            Some("event-storming".to_string()),
            "the `event-storming` marker did not start `event-storming`"
        );
        assert!(
            topic_model
                .stages
                .iter()
                .all(|assignment| assignment.stage.is_some()),
            "the ideation derivation left an entry without a phase: {:?}",
            topic_model.stages
        );
        assert!(
            topic_model.unstaged.is_none(),
            "the topic report carries a no-stage group"
        );

        // A change report linked to the same topic: its own nine-phase stack,
        // the group's six ideation phases, and two bands in which the change's
        // band excludes the topic's entries.
        let change = Scope::Change {
            id: "demo".to_string(),
        };
        let linked = build_report_model(
            Some(change.clone()),
            vec![
                FixtureEntry {
                    id: 10,
                    at: "2026-10-17T08:00:00.000Z",
                    kind: "step",
                    name: "specs",
                    duration_ms: Some(20_000),
                    ..FixtureEntry::default()
                }
                .record(),
            ],
            Some(LinkedIdeation {
                topic: "ideas".to_string(),
                entries: topic_entries.clone(),
            }),
        );
        assert_eq!(
            linked.workflow.len(),
            WORKFLOW_STAGES.len(),
            "the linked change report did not keep its own workflow"
        );
        let group = linked
            .ideation
            .as_ref()
            .expect("the linked change report carries no ideation group");
        assert_eq!(group.topic, "ideas");
        assert!(group.activity, "the active topic reads as no activity");
        assert_eq!(
            group.phases.len(),
            IDEATION_STAGES.len(),
            "the group does not carry the six ideation phases"
        );
        assert_eq!(
            group
                .phases
                .iter()
                .map(|phase| phase.stage.clone())
                .collect::<Vec<_>>(),
            IDEATION_STAGES
                .iter()
                .map(|stage| Some((*stage).to_string()))
                .collect::<Vec<_>>(),
            "the group's phases are not the ideation workflow"
        );
        assert_eq!(group.entries, vec![1, 2, 3, 4, 5]);
        assert_eq!(
            group.duration_ms,
            Some(14_400_000),
            "the group's duration is not its own first-to-last entry"
        );
        assert_eq!(group.tools, 1, "the group's tool count is not its own");
        assert_eq!(
            linked.bands.len(),
            2,
            "the linked change report does not carry the topic's own band"
        );
        let change_band = linked
            .bands
            .iter()
            .find(|band| band.scope == change)
            .expect("the change report has no change band");
        let ideation_band = linked
            .bands
            .iter()
            .find(|band| band.scope == topic)
            .expect("the change report has no ideation band");
        assert_eq!(
            change_band.entries,
            vec![10],
            "the change's band took the linked topic's entries: {change_band:?}"
        );
        assert_eq!(ideation_band.entries, vec![1, 2, 3, 4, 5]);
        assert!(
            change_band
                .entries
                .iter()
                .all(|id| !ideation_band.entries.contains(id)),
            "a topic id joined the change's band"
        );
        assert_eq!(
            linked.summary.entries, 1,
            "the change's summary counted the linked topic's entries"
        );
        assert_eq!(
            linked.summary.longest_steps.len(),
            1,
            "the change's ranking took the linked topic's measured entries"
        );
        assert_eq!(linked.summary.longest_steps[0].id, 10);
        assert!(
            !linked.empty,
            "the change report with its own entry renders the empty state"
        );

        // A change whose own record is empty but whose linked topic has
        // activity is not empty; a report with no entries anywhere is.
        let activity_only = build_report_model(
            Some(change.clone()),
            Vec::new(),
            Some(LinkedIdeation {
                topic: "ideas".to_string(),
                entries: topic_entries,
            }),
        );
        assert!(
            !activity_only.empty,
            "a linked topic's activity was reported as an empty report"
        );
        assert_eq!(
            activity_only.summary.entries, 0,
            "the change's summary counted entries it does not own"
        );
        assert!(
            activity_only
                .ideation
                .as_ref()
                .is_some_and(|group| group.activity),
            "the group does not carry the topic's own activity"
        );
        let bare = build_report_model(Some(change), Vec::new(), None);
        assert!(
            bare.empty,
            "a report with no entries anywhere did not render the empty state"
        );
    }

    /// The link resolver's proof over the real proposal forms: the primary
    /// topic line, the seed fallback, normalization (whitespace, backticks, a
    /// bare id, a topic path with or without a trailing slash), `none`
    /// including `none — <reason>`, the topic directory's existence, the
    /// archive proposal, and the live proposal winning over the archive.
    #[test]
    fn link_resolution_normalizes_real_proposal_forms() {
        let fixture = tempfile::TempDir::new().expect("create the fixture root");
        let root = fixture.path();
        for topic in ["multi-scm-support", "foo"] {
            std::fs::create_dir_all(root.join("givn/ideation").join(topic))
                .expect("create the topic directory");
        }
        std::fs::create_dir_all(root.join("givn/changes")).expect("create the change directory");
        std::fs::create_dir_all(root.join("givn/archive")).expect("create the archive directory");

        // One topic's recorded work: the link snapshot carries it.
        let mut stored = vec![HistoryEntry::reported(
            Scope::Ideation {
                id: "foo".to_string(),
            },
            "step",
            "ideation",
            "2026-10-16T08:00:00.000Z",
        )];
        crate::history::HistoryStore::at(crate::history::store_path(root))
            .append(&mut stored)
            .expect("seed the record");

        let write_proposal = |path: &str, body: &str| {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().expect("the proposal has a directory"))
                .expect("create the change directory");
            std::fs::write(path, body).expect("write the proposal");
        };

        for (proposal, expected) in [
            (
                "- Ideation topic: `multi-scm-support`\n- Seed path: none\n",
                Some("multi-scm-support"),
            ),
            ("- Ideation topic: givn/ideation/foo\n", Some("foo")),
            ("- Ideation topic: givn/ideation/foo/\n", Some("foo")),
            ("- Ideation topic: none\n", None),
            (
                "- Ideation topic: none — the change originates from a rules refinement\n",
                None,
            ),
            ("- Ideation topic: \n- Seed path: none\n", None),
            ("- Ideation topic: givn/ideation/missing/\n", None),
            (
                "- Ideation topic: none\n- Seed path: givn/ideation/foo/changes/<uc-id>.md\n",
                None,
            ),
            (
                "- Seed path: givn/ideation/foo/changes/<uc-id>.md\n",
                Some("foo"),
            ),
            ("- Seed path: none\n", None),
        ] {
            write_proposal("givn/changes/demo/proposal.md", proposal);
            let link = resolve_link(root, "demo");
            assert_eq!(
                link.as_ref().map(|link| link.topic.as_str()),
                expected,
                "the proposal form did not normalize as expected: {proposal:?}"
            );
            if expected == Some("foo") {
                assert_eq!(
                    link.as_ref().map(|link| link.entries.len()),
                    Some(1),
                    "the link snapshot does not carry the topic's record: {proposal:?}"
                );
            }
        }

        // The archive proposal is read when the live directory does not exist.
        write_proposal("givn/archive/old/proposal.md", "- Ideation topic: foo\n");
        assert_eq!(
            resolve_link(root, "old").map(|link| link.topic),
            Some("foo".to_string()),
            "the archive proposal did not resolve its link"
        );

        // The live proposal wins when both directories exist.
        write_proposal("givn/changes/both/proposal.md", "- Ideation topic: foo\n");
        write_proposal(
            "givn/archive/both/proposal.md",
            "- Ideation topic: multi-scm-support\n",
        );
        assert_eq!(
            resolve_link(root, "both").map(|link| link.topic),
            Some("foo".to_string()),
            "the archive proposal won over the live one"
        );
    }
}
