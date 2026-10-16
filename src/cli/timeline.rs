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

/// The recorded `step` kind: the shell steps Givn ran, and the entries the
/// summary's longest-step ranking is about.
const STEP: &str = "step";

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
    /// The summary: the numbers the report states about the whole window.
    pub summary: Summary,
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
        }
    }
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
    /// The recorded steps, longest first.
    pub longest_steps: Vec<RankedStep>,
    /// The recorded question waits the window holds. A wait the record does not
    /// time is still a wait.
    pub waits: usize,
    /// The waits the record does not time; the total never adds them as zero.
    pub unmeasured_waits: usize,
    /// The total of the waits the record measured, or `None` when it measured
    /// none of them.
    pub wait_ms: Option<u64>,
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
}

impl Summary {
    /// The summary over the report's window: one pass over the window's entries
    /// feeds every number it states, so no summary figure is ever a second,
    /// differently-shaped scan of the record.
    fn of(entries: &[HistoryEntry]) -> Self {
        let mut summary = Summary {
            entries: entries.len(),
            ..Summary::default()
        };
        for entry in entries {
            if records_span(&entry.kind) && entry.duration_ms.is_none() {
                summary.unmeasured_durations += 1;
            }
            if entry.kind == QUESTION {
                summary.waits += 1;
                if !add_measured(&mut summary.wait_ms, entry.duration_ms) {
                    summary.unmeasured_waits += 1;
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
fn recorded_span(entries: &[HistoryEntry]) -> Option<u64> {
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
    fn of(entries: &[HistoryEntry]) -> Self {
        let usage: Vec<&HistoryEntry> =
            entries.iter().filter(|entry| entry.kind == USAGE).collect();
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

/// One ranked step: the recorded step and the duration it was measured with.
#[derive(Debug, Serialize)]
pub struct RankedStep {
    /// The recorded entry's id.
    pub id: u64,
    /// The entry's visible label (its bounded subject label, or its name).
    pub label: String,
    /// The measured duration the ranking orders by.
    pub duration_ms: u64,
}

/// Record lines → `ReportModel` (bands, ordering, ranking, totals).
pub fn build_report_model(scope: Option<Scope>, entries: Vec<HistoryEntry>) -> ReportModel {
    let entries = ordered_by_recorded_time(entries);
    let bands = bands_for(scope.as_ref(), &entries);
    let summary = Summary::of(&entries);
    ReportModel {
        scope,
        bands,
        entries,
        summary,
    }
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
    if let Some(scope) = scope {
        bands.push(Band::of(
            scope,
            entries.iter().map(|entry| entry.id).collect(),
        ));
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

/// The recorded steps, longest first — the report's one ranking: the summary
/// shows it, and every other view (the axis's longest-step mark) reads the
/// ranked list the model carries rather than sorting a second time. The order
/// is by the recorded duration, never by the store's line order; ties fall back
/// to the recorded time and then the entry id, so the ranking is deterministic
/// for every record.
fn ranked_steps(entries: &[HistoryEntry]) -> Vec<RankedStep> {
    let mut steps: Vec<&HistoryEntry> = entries
        .iter()
        .filter(|entry| entry.kind == STEP && entry.duration_ms.is_some())
        .collect();
    steps.sort_by(|a, b| {
        b.duration_ms
            .cmp(&a.duration_ms)
            .then_with(|| a.at.cmp(&b.at))
            .then_with(|| a.id.cmp(&b.id))
    });
    steps
        .into_iter()
        .map(|entry| RankedStep {
            id: entry.id,
            label: visible_label(entry),
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

/// The model as the page's own text: the scope line, the summary, and the
/// timeline's initial view — the bands, one visible row per scope the window
/// covers, with the band's spans drawn on the report's time axis.
fn render_view(model: &ReportModel) -> String {
    let mut view = String::new();
    view.push_str(&format!(
        "<p class=\"report-scope\" id=\"report-scope\">{}</p>\n",
        escape(&scope_line(model.scope.as_ref()))
    ));
    view.push_str(&render_summary(&model.summary));
    view.push('\n');
    view.push_str(&render_timeline(model));
    view
}

/// The timeline's drawing width and the left gutter the band rows' labels
/// occupy, in the SVG's own units.
const WIDTH: u64 = 1000;
const GUTTER: f64 = 12.0;
const LABEL_WIDTH: f64 = 168.0;
const MARGIN: f64 = 8.0;

/// One band row's height and the narrowest mark a span draws as.
const BAND_HEIGHT: u64 = 56;
const MIN_SPAN: f64 = 2.0;

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

/// The timeline section: one row per band, each row naming its scope and
/// drawing its spans across the window's time range, above an axis strip of
/// round tick labels. A window with no bands draws no timeline at all.
fn render_timeline(model: &ReportModel) -> String {
    if model.bands.is_empty() {
        return String::new();
    }
    let scale = TimeScale::of(&model.entries);
    let height = AXIS_HEIGHT + model.bands.len() as u64 * BAND_HEIGHT + MARGIN as u64;
    let mut svg = format!(
        "<section class=\"timeline-section\" aria-label=\"Development timeline\">\n<svg class=\"timeline\" role=\"img\" aria-labelledby=\"timeline-title\" viewBox=\"0 0 {WIDTH} {height}\">\n<title id=\"timeline-title\">Development timeline</title>\n"
    );
    svg.push_str(&render_axis(&scale));
    for (index, band) in model.bands.iter().enumerate() {
        let y = AXIS_HEIGHT + index as u64 * BAND_HEIGHT;
        svg.push_str(&render_band(y, band, &model.entries, &scale));
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
/// name) and one span per entry the band covers, in the model's own order.
fn render_band(y: u64, band: &Band, entries: &[HistoryEntry], scale: &TimeScale) -> String {
    let mut svg = format!(
        "<g class=\"band\" data-band-kind=\"{}\" data-band=\"{}\" aria-label=\"{}\">\n<rect class=\"band-row\" x=\"0\" y=\"{y}\" width=\"{WIDTH}\" height=\"{BAND_HEIGHT}\"></rect>\n<text class=\"band-label\" x=\"{GUTTER}\" y=\"{}\">{}</text>\n",
        escape(&band.kind),
        escape(&scope_row(&band.scope).id),
        escape(&band.label),
        y + BAND_HEIGHT / 4,
        escape(&band.label),
    );
    for id in &band.entries {
        let Some(entry) = entries.iter().find(|entry| entry.id == *id) else {
            continue;
        };
        svg.push_str(&render_span(entry, y, scale));
    }
    svg.push_str("</g>\n");
    svg
}

/// One span: the entry drawn on the row's axis, with its accessible name always
/// its label and its duration.
fn render_span(entry: &HistoryEntry, y: u64, scale: &TimeScale) -> String {
    let label = visible_label(entry);
    let duration = duration_text(entry);
    let (x, width) = scale.span(entry);
    let bar_y = y as f64 + BAND_HEIGHT as f64 / 3.0;
    format!(
        "<g class=\"span\" data-entry-id=\"{}\" data-kind=\"{}\" data-at=\"{}\" aria-label=\"{}, {}\">\n<rect class=\"span-bar\" x=\"{x:.1}\" y=\"{bar_y:.1}\" width=\"{width:.1}\" height=\"16\"></rect>\n<text class=\"span-label\" x=\"{:.1}\" y=\"{:.1}\">{}</text>\n</g>\n",
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
        let mut times = entries
            .iter()
            .filter_map(|entry| crate::history::parse_rfc3339_ms(&entry.at));
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
            "<p class=\"summary-note\" data-unmeasured-durations=\"{}\">{} not measured</p>\n",
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
        html.push_str(&format!(
            "<li data-entry-id=\"{}\" data-duration-ms=\"{}\"><span class=\"step-label\">{}</span><span class=\"step-duration\">{}</span></li>\n",
            step.id,
            step.duration_ms,
            escape(&step.label),
            format_duration(step.duration_ms),
        ));
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
    let mut note = wait_count(summary.waits);
    if summary.unmeasured_waits > 0 {
        note.push_str(&format!(" · {} not measured", summary.unmeasured_waits));
    }
    note
}

/// `n waits`, in the singular for one.
fn wait_count(waits: usize) -> String {
    if waits == 1 {
        "1 wait".to_string()
    } else {
        format!("{waits} waits")
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

/// `givn timeline` — write the report for one scope and print its path. With
/// `--out` omitted the report goes to the project's own default path
/// `givn/history/reports/<scope>.html`.
pub fn run(change: Option<&str>, topic: Option<&str>, out: Option<&Path>) -> Result<PathBuf> {
    let root = std::env::current_dir().context("resolve the project root")?;
    let scope = crate::cli::history::scope_for(&root, change, topic);
    let entries = crate::cli::history::entries_in_scope(&root, scope.as_ref())?;
    let model = build_report_model(scope.clone(), entries);
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
