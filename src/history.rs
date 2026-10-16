//! The development record (ADR-073): the append-only per-project store
//! `givn/history/events.jsonl`, scope resolution, bounded labels, pending
//! state for process-per-hook harnesses, and the per-process recorder.
//!
//! Best-effort by construction: a write that cannot happen warns and
//! continues, and never changes the outcome of the work it records.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// The record's schema version. A reader skips a line whose schema it does not
/// know.
pub const SCHEMA: u32 = 1;

/// The record's one vocabulary for a signal it does not have — missing data is
/// recorded as missing, never as a zero:
///
/// - an unmeasured duration is *absent* (no `duration_ms`),
/// - an outcome the harness did not report is [`OUTCOME_UNKNOWN`],
/// - a turn whose token usage the harness did not report carries
///   `data.usage: `[`USAGE_UNAVAILABLE`].
pub const OUTCOME_UNKNOWN: &str = "unknown";

/// The turn `data.usage` marker for a token usage the harness never reported.
pub const USAGE_UNAVAILABLE: &str = "unavailable";

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Measured duration; absent for a point entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// `ok`, `failed`, `skipped`, or `unknown`.
    pub outcome: String,
    /// The `id` of the enclosing entry, when the same process minted it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    /// Only where the report needs a non-generic value.
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl HistoryEntry {
    /// One entry the agent intake reports: `agent` as its source, the reported
    /// end timestamp, the intake's resolved scope, `unknown` until the mapping
    /// sets the reported outcome, and no `id` — the store mints it at append
    /// time. A caller sets `label` through [`bound_label`] (or
    /// [`bound_question_label`]), the record's one label path, so no raw
    /// subject can reach the store.
    pub fn reported(scope: Scope, kind: &str, name: &str, at: &str) -> Self {
        HistoryEntry {
            schema: SCHEMA,
            id: 0,
            at: at.to_string(),
            scope,
            source: "agent".to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
            label: None,
            duration_ms: None,
            outcome: OUTCOME_UNKNOWN.to_string(),
            parent: None,
            data: None,
        }
    }
}

/// Append and read the project's `givn/history/events.jsonl` (one JSON object
/// per line, opened append-only, never rewritten, never reordered); owns the
/// schema and mints each `id` at append time under the append lock.
#[derive(Debug, Clone)]
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    /// The store whose record file is `path`.
    pub fn at(path: PathBuf) -> Self {
        HistoryStore { path }
    }

    /// The record file this store owns.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The store's bytes, or `None` when the store does not exist yet.
    fn text(&self) -> std::io::Result<Option<String>> {
        match std::fs::read_to_string(&self.path) {
            Ok(text) => Ok(Some(text)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    /// Read every entry this reader knows, in file order. A line whose schema
    /// this reader does not know is skipped rather than guessed at, and a
    /// missing store reads as empty.
    pub fn read(&self) -> std::io::Result<Vec<HistoryEntry>> {
        Ok(self
            .text()?
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str::<HistoryEntry>(line).ok())
            .filter(|entry| entry.schema <= SCHEMA)
            .collect())
    }

    /// Append the entries in order as `append` does, resolving each entry's
    /// `parent` from the batch index of the entry it is nested in. Both the
    /// parent and the child are minted by this same append and written under
    /// the store's one append lock, so a recorded parent link always names an
    /// entry this process wrote and can never dangle.
    pub fn append_linked(
        &self,
        entries: &mut [HistoryEntry],
        parents: &[Option<usize>],
    ) -> std::io::Result<u64> {
        use std::io::{Read as _, Write as _};

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.path)?;
        file.lock()?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let first_id = last_id(&text) + 1;
        for (offset, entry) in entries.iter_mut().enumerate() {
            entry.id = first_id + offset as u64;
            if let Some(parent) = parents.get(offset).copied().flatten() {
                entry.parent = Some(first_id + parent as u64);
            }
            let line = serde_json::to_string(entry).map_err(std::io::Error::other)?;
            writeln!(file, "{line}")?;
        }
        Ok(first_id)
    }

    /// Append the entries in order (creating the store's directory on first
    /// write), minting each `id` at append time under the store's exclusive
    /// append lock, from the store's current last line. Returns the first id
    /// minted, so a caller can parent its children to the entry it just wrote.
    ///
    /// One handle carries both halves: the lock is taken before the store's
    /// last line is read and released only after the lines are written, so two
    /// processes recording at the same time mint from different last lines and
    /// neither can interleave its lines with the other's.
    pub fn append(&self, entries: &mut [HistoryEntry]) -> std::io::Result<u64> {
        let parents = vec![None; entries.len()];
        self.append_linked(entries, &parents)
    }
}

/// The id of a store's current last line (`0` when the store is empty).
fn last_id(text: &str) -> u64 {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| serde_json::from_str::<HistoryEntry>(line).ok())
        .map(|entry| entry.id)
        .unwrap_or(0)
}

/// The process's recorder for this invocation: the runtime command name and
/// label are only known here, so the cell is installed rather than derived from
/// a fixed initializer.
static RECORDER: OnceLock<HistoryRecorder> = OnceLock::new();

/// Install the process-wide recorder for one `givn` invocation and return it.
/// The first installation wins: there is one recorder per process.
pub fn install(name: &str, label: &str) -> &'static HistoryRecorder {
    RECORDER.get_or_init(|| HistoryRecorder::start(name, label))
}

/// The process's recorder, when this process records at all (the read-only
/// commands do not).
pub fn current() -> Option<&'static HistoryRecorder> {
    RECORDER.get()
}

/// Per-process recorder: the store handle, the scope resolved once, `record`,
/// and a `span(name)` guard that measures `Instant::now()..drop`. It never
/// returns an error to its caller.
#[derive(Debug)]
pub struct HistoryRecorder {
    root: PathBuf,
    store: HistoryStore,
    scope: Scope,
    command: String,
    label: String,
    started: Instant,
    pending: Mutex<Vec<HistoryEntry>>,
}

impl HistoryRecorder {
    /// Start the recorder for one command in the current project.
    pub fn start(name: &str, label: &str) -> Self {
        let root = std::env::current_dir().unwrap_or_default();
        HistoryRecorder {
            store: HistoryStore::at(store_path(&root)),
            scope: resolve_scope(&root),
            root,
            command: name.to_string(),
            label: label.to_string(),
            started: Instant::now(),
            pending: Mutex::new(Vec::new()),
        }
    }

    /// A fresh entry in this process's scope: the values every entry starts
    /// with — the schema version, its end timestamp, `givn` as the source, no
    /// parent, and no measured duration. The store mints the `id` at append
    /// time. Every label a caller hands in is bounded here.
    fn entry(&self, kind: &str, name: &str, label: Option<String>) -> HistoryEntry {
        HistoryEntry {
            schema: SCHEMA,
            id: 0,
            at: rfc3339_ms(SystemTime::now()),
            scope: self.scope.clone(),
            source: "givn".to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
            label: label.map(|label| bound_label(&label)),
            duration_ms: None,
            outcome: OUTCOME_UNKNOWN.to_string(),
            parent: None,
            data: None,
        }
    }

    /// Record one entry under this process's command entry: it is written by
    /// the flush, after the command entry, which is this process's parent. A
    /// caller-built entry's label is bounded here too, so no raw subject can
    /// reach the store.
    pub fn record(&self, mut entry: HistoryEntry) {
        entry.label = entry.label.map(|label| bound_label(&label));
        if let Ok(mut pending) = self.pending.lock() {
            pending.push(entry);
        }
    }

    /// A guard that measures one action's wall time (`Instant::now()..drop`)
    /// and records the entry it measured.
    pub fn span(&self, name: &str) -> Span<'_> {
        Span {
            recorder: self,
            kind: "step".to_string(),
            name: name.to_string(),
            label: None,
            started: Instant::now(),
            outcome: OUTCOME_UNKNOWN.to_string(),
        }
    }

    /// Write this process's `command` entry: the command name, its resolved
    /// scope, its measured duration, its outcome, and its exit code. Every
    /// entry recorded during the run follows it, parented to its id — the ids
    /// are all minted by this process, so every parent link names an entry of
    /// this process's run.
    ///
    /// Best-effort by construction: a store that cannot be written warns and
    /// continues, and the outcome of the command itself is untouched.
    pub fn finish(&self, outcome: &str, exit_code: i32) {
        // The engagement rule, checked before anything is created: a root with
        // no Givn scaffold receives no record, no ignore block, and no
        // directory.
        if !engages(&self.root) {
            return;
        }
        // The record's ignore entry is re-asserted on every write, before the
        // store is touched: a store that predates the repository still becomes
        // ignored, and no destination this writes can dirty the worktree.
        ensure_ignored(&self.root);
        let mut command = self.entry("command", &self.command, Some(self.label.clone()));
        command.duration_ms = Some(self.started.elapsed().as_millis() as u64);
        command.outcome = outcome.to_string();
        command.data = Some(serde_json::json!({ "exit_code": exit_code }));
        let command_id = match self.store.append(std::slice::from_mut(&mut command)) {
            Ok(first_id) => first_id,
            Err(error) => return self.warn(&error),
        };
        let mut pending = self.take_pending();
        for entry in &mut pending {
            if entry.parent.is_none() {
                entry.parent = Some(command_id);
            }
        }
        if let Err(error) = self.store.append(&mut pending) {
            self.warn(&error);
        }
    }

    /// The entries recorded during this run, in the order they were recorded.
    fn take_pending(&self) -> Vec<HistoryEntry> {
        self.pending
            .lock()
            .map(|mut pending| std::mem::take(&mut *pending))
            .unwrap_or_default()
    }

    /// The one place a store that cannot be written is reported: a warning,
    /// never an error to the caller.
    fn warn(&self, error: &std::io::Error) {
        eprintln!(
            "warning: the development record could not be written to {}: {error}",
            self.store.path().display()
        );
    }
}

/// A measured span: it measures `Instant::now()..drop` and records the entry it
/// measured when it drops, so an early return still records what ran.
#[derive(Debug)]
pub struct Span<'a> {
    recorder: &'a HistoryRecorder,
    kind: String,
    name: String,
    label: Option<String>,
    started: Instant,
    outcome: String,
}

impl Span<'_> {
    /// The entry kind this span records (`step`, `phase`, `hook`, `revision`).
    pub fn kind(mut self, kind: &str) -> Self {
        self.kind = kind.to_string();
        self
    }

    /// The bounded subject the entry keeps (a shell command's text, a
    /// question's first line).
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// The outcome the entry carries; an unmarked span records `unknown`.
    pub fn mark(mut self, outcome: &str) -> Self {
        self.outcome = outcome.to_string();
        self
    }
}

impl Drop for Span<'_> {
    fn drop(&mut self) {
        let mut entry = self
            .recorder
            .entry(&self.kind, &self.name, self.label.clone());
        entry.duration_ms = Some(self.started.elapsed().as_millis() as u64);
        entry.outcome = self.outcome.clone();
        self.recorder.record(entry);
    }
}

/// The scope every entry of this project resolves to: the first unambiguous
/// candidate in the recorder's order — the active change, then the unfinished
/// ideation topic — or the project itself. A candidate yields `None` when the
/// project's own state is ambiguous; a recorder never guesses between
/// candidates.
pub fn resolve_scope(root: &Path) -> Scope {
    let candidates: [fn(&Path) -> Option<Scope>; 2] = [change_scope, topic_scope];
    candidates
        .into_iter()
        .find_map(|candidate| candidate(root))
        .unwrap_or_else(|| Scope::Project {
            id: project_name(root),
        })
}

/// The change scope, when exactly one change is active. Several active changes
/// are ambiguous, and a recorder never guesses between them.
fn change_scope(root: &Path) -> Option<Scope> {
    let manifest = crate::config::overlay::load_effective().ok()?;
    let changes = crate::state::list_changes(root, &manifest);
    single(changes.into_iter().map(|change| change.id)).map(|id| Scope::Change { id })
}

/// The ideation scope, when exactly one topic's session is unfinished — the
/// topic a project that has no active change is still working on. Several
/// unfinished topics are ambiguous, and a recorder never guesses between them.
fn topic_scope(root: &Path) -> Option<Scope> {
    let unfinished = std::fs::read_dir(root.join("givn").join("ideation"))
        .ok()?
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter(|entry| topic_is_unfinished(&entry.path()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned());
    single(unfinished).map(|id| Scope::Ideation { id })
}

/// Whether a topic is still in flight: `givn/ideation/<topic>/session.md`
/// exists and carries no `## Completion` section — the section the handoff
/// step appends beside its own phase line.
fn topic_is_unfinished(topic_dir: &Path) -> bool {
    let Ok(session) = std::fs::read_to_string(topic_dir.join("session.md")) else {
        return false;
    };
    !session
        .lines()
        .any(|line| line.trim_start().starts_with("## Completion"))
}

/// The exactly-one rule every scope case applies: exactly one candidate
/// resolves the scope, zero or several candidates fall through to the next
/// case.
fn single<T>(mut candidates: impl Iterator<Item = T>) -> Option<T> {
    let only = candidates.next()?;
    candidates.next().is_none().then_some(only)
}

/// The project's own name: the name of its root directory.
fn project_name(root: &Path) -> String {
    root.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project")
        .to_string()
}

/// The longest label the record keeps, in characters (never bytes).
pub const LABEL_LIMIT: usize = 120;

/// The bounded subject label (≤ 120 characters, truncation marked with `…`) —
/// the only place raw subjects are touched. A subject at or under the limit is
/// kept verbatim; a longer one keeps the first `LABEL_LIMIT - 1` characters and
/// marks the truncation, so the marked label is itself within the limit.
pub fn bound_label(subject: &str) -> String {
    let mut chars = subject.chars();
    let mut bounded: String = chars.by_ref().take(LABEL_LIMIT).collect();
    if chars.next().is_some() {
        bounded.pop();
        bounded.push('…');
    }
    bounded
}

/// The bounded subject label of a reported event: the record's one label path,
/// so every reported subject is bounded the same way and each kind's own rule
/// lives here — only a question's first line is kept, because the complete
/// question text is never stored (D2).
pub fn reported_label(kind: &str, subject: &str) -> String {
    let subject = match kind {
        "question" => subject.lines().next().unwrap_or_default(),
        _ => subject,
    };
    bound_label(subject)
}

/// The project's store path: `givn/history/events.jsonl`.
pub fn store_path(root: &Path) -> PathBuf {
    root.join("givn").join("history").join("events.jsonl")
}

/// The fence of the record's own managed block. The record is development
/// data, not a generated rule, so it never shares the `# givn:begin:rules`
/// block.
pub const HISTORY_BLOCK_BEGIN: &str = "# givn:begin:history";
/// The line that closes the record's managed block.
pub const HISTORY_BLOCK_END: &str = "# givn:end:history";

/// The record's entry in a Git destination: the store's directory.
pub const GIT_IGNORE_ENTRY: &str = "givn/history/";

/// The record's entry in a Fossil ignore list. A Fossil glob must match the
/// whole path and `*` crosses `/`, so a bare directory path matches nothing.
pub const FOSSIL_IGNORE_ENTRY: &str = "givn/history/*";

/// Whether the recorder engages in this project: only a root that carries the
/// Givn scaffold (`givn/`) records.
///
/// Before the scaffold exists the recorder writes nothing and creates no
/// directory, so a refused command in a bare root — `givn init` with no agent
/// target, or a root a backend preflight refuses — leaves that root exactly as
/// it was. The rule keys on the scaffold, never on the VCS markers: once
/// `givn/` exists, every command records, refusals and failures included.
fn engages(root: &Path) -> bool {
    root.join("givn").is_dir()
}

/// Keep the store out of repository history, re-asserted on every record
/// write.
///
/// The recorder never writes a versioned file: with a Git worktree it
/// re-asserts the record's block in the worktree-local exclude file, with a
/// Fossil checkout it writes the repository-level `ignore-glob` setting only
/// while no managed file overrides it, and it uses `.gitignore` only before a
/// repository exists (the scaffold-time safety net). The destinations and the
/// backend decision come from the ignore writer's one table. Best effort by
/// construction: a destination that cannot be written leaves the store exactly
/// as ignored as the project's own configuration made it.
pub fn ensure_ignored(root: &Path) {
    for write in crate::skills::rules::record_writes(root) {
        let _ = crate::skills::rules::apply_write(root, &write);
    }
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

/// `time` as an RFC 3339 UTC timestamp with millisecond precision.
fn rfc3339_ms(time: SystemTime) -> String {
    let elapsed = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = elapsed.as_secs() as i64;
    let millis = elapsed.subsec_millis();
    let (year, month, day) = civil_from_days(seconds.div_euclid(86_400));
    let second_of_day = seconds.rem_euclid(86_400);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        second_of_day / 3_600,
        second_of_day % 3_600 / 60,
        second_of_day % 60,
    )
}

/// Days since 1970-01-01 → `(year, month, day)`, after Howard Hinnant's
/// `civil_from_days` (the inverse of `days_from_civil`), valid for every date
/// a record can carry.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097) as u64;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era as i64 + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_position + 2) / 5 + 1) as u32;
    let month = if month_position < 10 {
        month_position + 3
    } else {
        month_position - 9
    } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// A reported `at` as milliseconds since the Unix epoch, or `None` when the
/// text is not an RFC 3339 UTC timestamp (`YYYY-MM-DDThh:mm:ss[.fff]Z`, the
/// form every reporter produces). This is the record's one timestamp parser:
/// the intake validates each reported `at` through it and measures a reported
/// span by subtracting two of them.
pub fn parse_rfc3339_ms(at: &str) -> Option<i64> {
    let (date, time) = at.split_once('T')?;
    let mut date_parts = date.split('-');
    let year: i64 = date_parts.next()?.parse().ok()?;
    let month: u32 = date_parts.next()?.parse().ok()?;
    let day: u32 = date_parts.next()?.parse().ok()?;
    if date_parts.next().is_some() || month == 0 || month > 12 || day == 0 {
        return None;
    }
    if day > days_in_month(year, month) {
        return None;
    }
    let time = time.strip_suffix('Z')?;
    let (clock, fraction) = match time.split_once('.') {
        Some((clock, fraction)) => (clock, Some(fraction)),
        None => (time, None),
    };
    let mut clock_parts = clock.split(':');
    let hour: u32 = clock_parts.next()?.parse().ok()?;
    let minute: u32 = clock_parts.next()?.parse().ok()?;
    let second: u32 = clock_parts.next()?.parse().ok()?;
    if clock_parts.next().is_some() || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let millis = match fraction {
        None => 0,
        Some(fraction) => {
            if fraction.is_empty()
                || fraction.len() > 9
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            {
                return None;
            }
            let mut millis = 0u32;
            for byte in fraction.bytes().chain(std::iter::repeat(b'0')).take(3) {
                millis = millis * 10 + u32::from(byte - b'0');
            }
            millis
        }
    };
    let days = days_from_civil(year, month, day);
    Some(
        ((days * 24 + i64::from(hour)) * 60 + i64::from(minute)) * 60 * 1_000
            + i64::from(second) * 1_000
            + i64::from(millis),
    )
}

/// The measured duration between two reported timestamps, or `None` when
/// either is unparsable or the span runs backwards — a negative span is never
/// recorded as a duration.
pub fn reported_duration_ms(start_at: &str, end_at: &str) -> Option<u64> {
    let start = parse_rfc3339_ms(start_at)?;
    let end = parse_rfc3339_ms(end_at)?;
    u64::try_from(end - start).ok()
}

/// Days since 1970-01-01 for a civil date, after Howard Hinnant's
/// `days_from_civil` — the exact inverse of [`civil_from_days`].
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month_position = i64::from(month) + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_position + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The number of days in `month` of `year`, so a reported date is validated
/// against the calendar rather than a flat `1..=31` range.
fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    }
}
