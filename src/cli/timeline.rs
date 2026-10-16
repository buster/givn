//! `givn timeline`: the development report — the model the report screen
//! renders and the self-contained HTML document it is rendered into.
//!
//! Like `givn history`, this command runs without a recorder: rendering a
//! report must never extend the record it renders.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::history::{HistoryEntry, Scope};

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
        Band {
            scope: scope.clone(),
            kind: scope_kind(scope).to_string(),
            label: scope_label(scope),
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
        let (mut earliest, mut latest) = (None::<i64>, None::<i64>);
        for entry in entries {
            if let Some(time) = crate::history::parse_rfc3339_ms(&entry.at) {
                earliest = Some(earliest.map_or(time, |value| value.min(time)));
                latest = Some(latest.map_or(time, |value| value.max(time)));
            }
            if records_span(&entry.kind) && entry.duration_ms.is_none() {
                summary.unmeasured_durations += 1;
            }
        }
        summary.elapsed_ms = earliest
            .zip(latest)
            .and_then(|(earliest, latest)| u64::try_from(latest - earliest).ok());
        summary.longest_steps = ranked_steps(entries);
        summary
    }
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
    let bands = bands_for(scope.as_ref(), &entries);
    let summary = Summary::of(&entries);
    ReportModel {
        scope,
        bands,
        entries,
        summary,
    }
}

/// The report's window: the bands its rows cover. A named scope is one band —
/// the report of that change, topic, or project. The project report groups the
/// window's entries by the scope each one belongs to, in the order the window
/// first names them, so the project's own entries stay in the "outside a
/// scope" band instead of being folded into a change.
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

/// The scope's identifier: the report's default file name and the name every
/// rendered scope label shows.
fn scope_id(scope: &Scope) -> &str {
    match scope {
        Scope::Change { id } | Scope::Ideation { id } | Scope::Project { id } => id,
    }
}

/// The band's scope class, the CSS hook the screen styles a band row by.
fn scope_kind(scope: &Scope) -> &'static str {
    match scope {
        Scope::Change { .. } => "change",
        Scope::Ideation { .. } => "ideation",
        Scope::Project { .. } => "project",
    }
}

/// The visible label of one scope: visible text, never an internal kind name.
fn scope_label(scope: &Scope) -> String {
    match scope {
        Scope::Change { id } => format!("Change: {id}"),
        Scope::Ideation { id } => format!("Ideation topic: {id}"),
        Scope::Project { id } => format!("Outside a scope: {id}"),
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

/// The model as the page's own text: the scope line and the summary.
fn render_view(model: &ReportModel) -> String {
    let mut view = String::new();
    view.push_str(&format!(
        "<p class=\"report-scope\" id=\"report-scope\">{}</p>\n",
        escape(&scope_line(model.scope.as_ref()))
    ));
    view.push_str(&render_summary(&model.summary));
    view
}

/// The report's scope line: the scope the report covers, named by its own id.
fn scope_line(scope: Option<&Scope>) -> String {
    match scope {
        Some(scope) => scope_label(scope),
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
    if summary.unmeasured_durations > 0 {
        html.push_str(&format!(
            "<p class=\"summary-note\" data-unmeasured-durations=\"{}\">{} not measured</p>\n",
            summary.unmeasured_durations,
            unmeasured_note(summary.unmeasured_durations)
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
    html.push_str("</ol>\n</section>");
    html
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
        Some(scope) => scope_id(scope).to_string(),
        None => "project".to_string(),
    };
    root.join("givn")
        .join("history")
        .join("reports")
        .join(format!("{id}.html"))
}
