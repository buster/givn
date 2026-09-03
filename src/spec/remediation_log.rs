//! The append-only semantic remediation log (ADR-068).
//!
//! Blocking `givn check review` maintains `givn/changes/<id>/semantic-remediation.md`
//! beside the regenerated semantic report. Run records and candidate entries are
//! appended once per changed candidate universe and never rewritten; the agent
//! completes `classification`, `rationale`, `remediation`, and `commits` in
//! place; the passing rerun completes the after-state fields. The review gate
//! validates entry completeness and cross-validates recorded commits against
//! Git history (reachability plus the `semantic-remediation:` stamp).

use std::path::Path;

use anyhow::{Context, Result, bail};

use super::retrieval::LogCandidateInput;

const LOG_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LogRunKind {
    Before,
    After,
}

impl LogRunKind {
    fn as_str(self) -> &'static str {
        match self {
            LogRunKind::Before => "before",
            LogRunKind::After => "after",
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct LogCompletion {
    pub candidate_id: String,
    pub left_evidence_hash: String,
    pub right_evidence_hash: String,
    pub state: &'static str,
    pub after_score: Option<f32>,
    pub after_gherkin: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct VerifiedLogEntry {
    pub candidate_id: String,
    pub left_evidence_hash: String,
    pub right_evidence_hash: String,
}

#[derive(Debug, Clone)]
struct RunRecord {
    kind: LogRunKind,
    commit: Option<String>,
    e5_runtime_ms: Option<u128>,
    bge_runtime_ms: Option<u128>,
    nli_runtime_ms: Option<u128>,
}

#[derive(Debug, Clone)]
struct EntryRecord {
    run: usize,
    candidate_id: String,
    embedding_score: f32,
    bge_admission: Option<String>,
    bge_symmetric_raw: Option<f32>,
    nli_admission: Option<String>,
    nli_symmetric_entailment: Option<f32>,
    left_file: String,
    left_title: String,
    right_file: String,
    right_title: String,
    left_content_hash: String,
    right_content_hash: String,
    left_evidence_hash: String,
    right_evidence_hash: String,
    left_gherkin: Option<String>,
    right_gherkin: Option<String>,
    classification: String,
    rationale: Option<String>,
    remediation: Option<String>,
    commits: Vec<String>,
    after_state: Option<String>,
    after_embedding_score: Option<f32>,
    after_commit: Option<String>,
    after_left_gherkin: Option<String>,
}

#[derive(Debug, Default)]
struct LogState {
    runs: Vec<RunRecord>,
    entries: Vec<EntryRecord>,
}

fn optional_text(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", value.replace('"', "\\\"")))
        .unwrap_or_else(|| "null".to_string())
}

fn score_text(value: Option<f32>) -> String {
    value
        .map(|value| format!("{value:.4}"))
        .unwrap_or_else(|| "null".to_string())
}

fn runtime_text(value: Option<u128>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn commits_text(commits: &[String]) -> String {
    if commits.is_empty() {
        return "[]".to_string();
    }
    let items: Vec<String> = commits
        .iter()
        .map(|commit| format!("\"{commit}\""))
        .collect();
    format!("[{}]", items.join(", "))
}

fn gherkin_field(name: &str, value: Option<&str>) -> String {
    let Some(value) = value else {
        return format!("- {name}: null\n");
    };
    let mut rendered = format!("- {name}: |\n");
    for line in value.lines() {
        let line = if line.is_empty() {
            String::new()
        } else {
            line.to_string()
        };
        rendered.push_str("    ");
        rendered.push_str(&line);
        rendered.push('\n');
    }
    rendered
}

fn render_log(change_id: &str, state: &LogState) -> String {
    let mut report = format!(
        "# Semantic Remediation Log\nLog schema version: {LOG_SCHEMA_VERSION}\nChange: \"{change_id}\"\n"
    );
    for (index, run) in state.runs.iter().enumerate() {
        report.push_str(&format!(
            "\n## Run {}\n- kind: {}\n- commit: {}\n- e5_runtime_ms: {}\n- bge_runtime_ms: {}\n- nli_runtime_ms: {}\n",
            index + 1,
            run.kind.as_str(),
            run.commit.as_deref().unwrap_or("null"),
            runtime_text(run.e5_runtime_ms),
            runtime_text(run.bge_runtime_ms),
            runtime_text(run.nli_runtime_ms),
        ));
    }
    for entry in &state.entries {
        report.push_str(&format!(
            "\n## Candidate {}\n- run: {}\n- before_embedding_score: {:.4}\n- before_bge_admission: {}\n- before_bge_symmetric_raw: {}\n- before_nli_admission: {}\n- before_nli_symmetric_entailment: {}\n- before_left_file: \"{}\"\n- before_left_title: \"{}\"\n- before_right_file: \"{}\"\n- before_right_title: \"{}\"\n- before_left_content_hash: {}\n- before_right_content_hash: {}\n- before_left_evidence_hash: {}\n- before_right_evidence_hash: {}\n",
            entry.candidate_id,
            entry.run,
            entry.embedding_score,
            optional_text(entry.bge_admission.as_deref()),
            score_text(entry.bge_symmetric_raw),
            optional_text(entry.nli_admission.as_deref()),
            score_text(entry.nli_symmetric_entailment),
            entry.left_file,
            entry.left_title,
            entry.right_file,
            entry.right_title,
            entry.left_content_hash,
            entry.right_content_hash,
            entry.left_evidence_hash,
            entry.right_evidence_hash,
        ));
        report.push_str(&gherkin_field(
            "before_left_gherkin",
            entry.left_gherkin.as_deref(),
        ));
        report.push_str(&gherkin_field(
            "before_right_gherkin",
            entry.right_gherkin.as_deref(),
        ));
        report.push_str(&format!(
            "- classification: {}\n- rationale: {}\n- remediation: {}\n- commits: {}\n- after_state: {}\n- after_embedding_score: {}\n- after_commit: {}\n",
            entry.classification,
            optional_text(entry.rationale.as_deref()),
            optional_text(entry.remediation.as_deref()),
            commits_text(&entry.commits),
            entry.after_state.as_deref().unwrap_or("null"),
            score_text(entry.after_embedding_score),
            entry.after_commit.as_deref().unwrap_or("null"),
        ));
        report.push_str(&gherkin_field(
            "after_left_gherkin",
            entry.after_left_gherkin.as_deref(),
        ));
    }
    report
}

/// Parse the `- name: value` / block-scalar fields until the next `## `
/// header, which is left in the iterator.
fn parse_fields(lines: &mut std::iter::Peekable<std::str::Lines<'_>>) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    while let Some(next) = lines.peek() {
        if next.starts_with("## ") {
            break;
        }
        let line = lines.next().unwrap_or_default();
        let Some(rest) = line.strip_prefix("- ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(": ") else {
            continue;
        };
        if value == "|" {
            let mut block: Vec<String> = Vec::new();
            while let Some(next) = lines.peek() {
                match next.strip_prefix("    ") {
                    Some(content) => {
                        block.push(content.trim_end().to_string());
                        lines.next();
                    }
                    None => break,
                }
            }
            fields.push((name.to_string(), block.join("\n")));
        } else {
            fields.push((name.to_string(), value.to_string()));
        }
    }
    fields
}

fn unquote(value: &str) -> Option<String> {
    let value = value.trim();
    if value == "null" {
        return None;
    }
    let unquoted = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value);
    Some(unquoted.replace("\\\"", "\""))
}

fn raw_optional(value: &str) -> Option<String> {
    if value == "null" || value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_optional_score(value: &str) -> Option<f32> {
    if value == "null" {
        return None;
    }
    value.parse::<f32>().ok()
}

fn parse_commits(value: &str) -> Vec<String> {
    let inner = value.trim().trim_start_matches('[').trim_end_matches(']');
    if inner.trim().is_empty() {
        return Vec::new();
    }
    inner
        .split(',')
        .filter_map(|item| unquote(item.trim()))
        .collect()
}

fn parse_log(content: &str) -> LogState {
    let mut state = LogState::default();
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("## Run ") {
            let fields = parse_fields(&mut lines);
            let lookup = |name: &str| {
                fields
                    .iter()
                    .find(|(field, _)| field == name)
                    .map(|(_, value)| value.clone())
                    .unwrap_or_else(|| "null".to_string())
            };
            state.runs.push(RunRecord {
                kind: match lookup("kind").as_str() {
                    "after" => LogRunKind::After,
                    _ => LogRunKind::Before,
                },
                commit: unquote(&lookup("commit")),
                e5_runtime_ms: lookup("e5_runtime_ms").parse::<u128>().ok(),
                bge_runtime_ms: lookup("bge_runtime_ms").parse::<u128>().ok(),
                nli_runtime_ms: lookup("nli_runtime_ms").parse::<u128>().ok(),
            });
            continue;
        }
        if line.starts_with("## Candidate ") {
            let candidate_id = line["## Candidate ".len()..].trim().to_string();
            let fields = parse_fields(&mut lines);
            let lookup = |name: &str| {
                fields
                    .iter()
                    .find(|(field, _)| field == name)
                    .map(|(_, value)| value.clone())
                    .unwrap_or_else(|| "null".to_string())
            };
            state.entries.push(EntryRecord {
                run: lookup("run").parse::<usize>().unwrap_or(0),
                candidate_id,
                embedding_score: parse_optional_score(&lookup("before_embedding_score"))
                    .unwrap_or(0.0),
                bge_admission: unquote(&lookup("before_bge_admission")),
                bge_symmetric_raw: parse_optional_score(&lookup("before_bge_symmetric_raw")),
                nli_admission: unquote(&lookup("before_nli_admission")),
                nli_symmetric_entailment: parse_optional_score(&lookup(
                    "before_nli_symmetric_entailment",
                )),
                left_file: unquote(&lookup("before_left_file")).unwrap_or_default(),
                left_title: unquote(&lookup("before_left_title")).unwrap_or_default(),
                right_file: unquote(&lookup("before_right_file")).unwrap_or_default(),
                right_title: unquote(&lookup("before_right_title")).unwrap_or_default(),
                left_content_hash: lookup("before_left_content_hash"),
                right_content_hash: lookup("before_right_content_hash"),
                left_evidence_hash: lookup("before_left_evidence_hash"),
                right_evidence_hash: lookup("before_right_evidence_hash"),
                left_gherkin: raw_optional(&lookup("before_left_gherkin")),
                right_gherkin: raw_optional(&lookup("before_right_gherkin")),
                classification: lookup("classification"),
                rationale: unquote(&lookup("rationale")),
                remediation: unquote(&lookup("remediation")),
                commits: parse_commits(&lookup("commits")),
                after_state: unquote(&lookup("after_state")),
                after_embedding_score: parse_optional_score(&lookup("after_embedding_score")),
                after_commit: unquote(&lookup("after_commit")),
                after_left_gherkin: raw_optional(&lookup("after_left_gherkin")),
            });
        }
    }
    state
}

fn entries_cover(state: &LogState, candidate: &LogCandidateInput) -> bool {
    state.entries.iter().any(|entry| {
        entry.candidate_id == candidate.candidate_id
            && entry.left_content_hash == candidate.left_content_hash
            && entry.right_content_hash == candidate.right_content_hash
            && entry.left_evidence_hash == candidate.left_evidence_hash
            && entry.right_evidence_hash == candidate.right_evidence_hash
    })
}

fn write_atomic(path: &Path, content: &str) -> Result<()> {
    let temp_path = path.with_extension("md.tmp");
    std::fs::write(&temp_path, content)
        .with_context(|| format!("write temporary remediation log {}", temp_path.display()))?;
    std::fs::rename(&temp_path, path)
        .with_context(|| format!("replace remediation log {}", path.display()))?;
    Ok(())
}

/// Append-only update of the remediation log for one review run.
///
/// Appends a run record when the current candidate universe is not already
/// covered by the log, appends entries for newly seen endpoint evidence, and
/// applies after-state completions for verified remediations. Returns whether
/// any byte changed; an unchanged log is not rewritten.
pub(super) fn update_log(
    path: &Path,
    change_id: &str,
    kind: LogRunKind,
    head_commit: Option<&str>,
    e5_runtime_ms: Option<u128>,
    bge_runtime_ms: Option<u128>,
    nli_runtime_ms: Option<u128>,
    candidates: &[LogCandidateInput],
    completions: &[LogCompletion],
) -> Result<bool> {
    let mut state = std::fs::read_to_string(path)
        .map(|content| parse_log(&content))
        .unwrap_or_default();
    let mut changed = false;

    let uncovered = candidates
        .iter()
        .any(|candidate| !entries_cover(&state, candidate));
    if state.runs.is_empty() || (kind == LogRunKind::Before && uncovered) {
        state.runs.push(RunRecord {
            kind,
            commit: head_commit.map(|commit| commit.to_string()),
            e5_runtime_ms,
            bge_runtime_ms,
            nli_runtime_ms,
        });
        changed = true;
    }
    let run_index = state.runs.len();

    if kind == LogRunKind::Before {
        for candidate in candidates {
            if entries_cover(&state, candidate) {
                continue;
            }
            state.entries.push(EntryRecord {
                run: run_index,
                candidate_id: candidate.candidate_id.clone(),
                embedding_score: candidate.embedding_score,
                bge_admission: candidate.bge_admission.clone(),
                bge_symmetric_raw: candidate.bge_symmetric_raw,
                nli_admission: candidate.nli_admission.clone(),
                nli_symmetric_entailment: candidate.nli_symmetric_entailment,
                left_file: candidate.left_file.clone(),
                left_title: candidate.left_title.clone(),
                right_file: candidate.right_file.clone(),
                right_title: candidate.right_title.clone(),
                left_content_hash: candidate.left_content_hash.clone(),
                right_content_hash: candidate.right_content_hash.clone(),
                left_evidence_hash: candidate.left_evidence_hash.clone(),
                right_evidence_hash: candidate.right_evidence_hash.clone(),
                left_gherkin: candidate.left_gherkin.clone(),
                right_gherkin: candidate.right_gherkin.clone(),
                classification: candidate.classification.clone(),
                rationale: candidate.rationale.clone(),
                remediation: None,
                commits: Vec::new(),
                after_state: None,
                after_embedding_score: None,
                after_commit: None,
                after_left_gherkin: None,
            });
            changed = true;
        }
    }

    for completion in completions {
        let Some(entry) = state.entries.iter_mut().find(|entry| {
            entry.candidate_id == completion.candidate_id
                && entry.left_evidence_hash == completion.left_evidence_hash
                && entry.right_evidence_hash == completion.right_evidence_hash
        }) else {
            continue;
        };
        if entry.after_state.is_some() {
            continue;
        }
        entry.after_state = Some(completion.state.to_string());
        entry.after_embedding_score = completion.after_score;
        entry.after_commit = head_commit.map(|commit| commit.to_string());
        entry.after_left_gherkin = completion.after_gherkin.clone();
        changed = true;
        if state.runs.last().map(|run| run.kind) != Some(LogRunKind::After) {
            state.runs.push(RunRecord {
                kind: LogRunKind::After,
                commit: head_commit.map(|commit| commit.to_string()),
                e5_runtime_ms,
                bge_runtime_ms,
                nli_runtime_ms,
            });
        }
    }

    if !changed {
        return Ok(false);
    }
    write_atomic(path, &render_log(change_id, &state)).map(|()| true)
}

fn is_sha1(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
}

pub(super) fn git_head_commit(root: &Path) -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    is_sha1(&hash).then_some(hash)
}

fn git_commit_message(root: &Path, commit: &str) -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["show", "-s", "--format=%B", commit])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).to_string())
}

fn git_commit_reachable(root: &Path, commit: &str) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["merge-base", "--is-ancestor", commit, "HEAD"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn stamp_line(change_id: &str, candidate_id: &str) -> String {
    format!("semantic-remediation: {change_id}/{candidate_id}")
}

/// Gate: every verified remediation whose log entry exists needs a non-empty
/// `remediation`, at least one commit, and — with Git history — commits that
/// are reachable and carry the exact stamp line. Entries absent from the log
/// belong to remediations that predate the log and pass without one.
pub(super) fn validate_log(
    path: &Path,
    root: &Path,
    change_id: &str,
    verified: &[VerifiedLogEntry],
    head_commit: Option<&str>,
) -> Result<()> {
    let state = std::fs::read_to_string(path)
        .map(|content| parse_log(&content))
        .unwrap_or_default();
    for entry in verified {
        let Some(record) = state.entries.iter().find(|record| {
            record.candidate_id == entry.candidate_id
                && record.left_evidence_hash == entry.left_evidence_hash
                && record.right_evidence_hash == entry.right_evidence_hash
        }) else {
            continue;
        };
        let candidate_id = &record.candidate_id;
        let remediation = record.remediation.as_deref().unwrap_or_default();
        if remediation.trim().is_empty() {
            bail!(
                "remediation log incomplete (candidate {candidate_id}): the entry has no remediation text; record the applied fix in semantic-remediation.md"
            );
        }
        if record.commits.is_empty() {
            bail!(
                "remediation log incomplete (candidate {candidate_id}): the entry records no commits; record the remediation commit hashes in semantic-remediation.md"
            );
        }
        if head_commit.is_none() {
            continue;
        }
        for commit in &record.commits {
            if !is_sha1(commit) || !git_commit_reachable(root, commit) {
                bail!(
                    "remediation log: commit {commit} for candidate {candidate_id} is not reachable from HEAD; record only commits contained in the repository history"
                );
            }
            let stamped = git_commit_message(root, commit)
                .map(|message| {
                    message
                        .lines()
                        .any(|line| line.trim() == stamp_line(change_id, candidate_id))
                })
                .unwrap_or(false);
            if !stamped {
                bail!(
                    "remediation log: commit {commit} for candidate {candidate_id} is missing the semantic-remediation stamp; the commit message must contain \"{}\"",
                    stamp_line(change_id, candidate_id)
                );
            }
        }
    }
    Ok(())
}

/// Extract the exact source lines of a scenario (tags included) from its
/// feature file. Interior blank lines are preserved; trailing blank lines are
/// trimmed. The block ends before the next tag or section directive.
pub(super) fn verbatim_scenario_text(root: &Path, file: &str, title: &str) -> Option<String> {
    let content = std::fs::read_to_string(root.join(file)).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    let start = lines.iter().position(|line| {
        let trimmed = line.trim();
        trimmed == format!("Scenario: {title}") || trimmed == format!("Scenario Outline: {title}")
    })?;
    let mut first = start;
    while first > 0 && lines[first - 1].trim_start().starts_with('@') {
        first -= 1;
    }
    let mut last = start;
    for (index, line) in lines.iter().enumerate().skip(start + 1) {
        let trimmed = line.trim_start();
        if trimmed.starts_with('@')
            || trimmed.starts_with("Scenario:")
            || trimmed.starts_with("Scenario Outline:")
            || trimmed.starts_with("Rule:")
            || trimmed.starts_with("Feature:")
        {
            break;
        }
        last = index;
    }
    while last > first && lines[last].trim().is_empty() {
        last -= 1;
    }
    let block: Vec<String> = lines[first..=last]
        .iter()
        .map(|line| line.trim_end().to_string())
        .collect();
    Some(block.join("\n"))
}
