//! Regular black-box steps for the semantic remediation log.

use std::fs;

use cucumber::{given, then, when};

use super::{GivnWorld, stderr, stdout};
use crate::common;

pub(crate) fn remediation_log_path(world: &GivnWorld) -> std::path::PathBuf {
    let env = world
        .env
        .as_ref()
        .expect("remediation log project is required");
    env.path("givn/changes/running-change/semantic-remediation.md")
}

pub(crate) fn read_remediation_log(world: &GivnWorld) -> String {
    fs::read_to_string(remediation_log_path(world)).expect("read semantic remediation log")
}

/// One parsed `- field: value` line or block scalar from a log section.
#[derive(Debug, Clone)]
pub(crate) struct LogField {
    pub(crate) name: String,
    pub(crate) value: String,
}

/// One parsed `## Run <n>` or `## Candidate <id>` section.
#[derive(Debug, Clone)]
pub(crate) struct LogSection {
    pub(crate) header: String,
    pub(crate) fields: Vec<LogField>,
}

impl LogSection {
    pub(crate) fn field(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|field| field.name == name)
            .map(|field| field.value.as_str())
    }
}

/// Split the log into its `## ` sections, resolving `|` block scalars.
pub(crate) fn parse_log_sections(text: &str) -> Vec<LogSection> {
    let mut sections: Vec<LogSection> = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if let Some(header) = line.strip_prefix("## ") {
            sections.push(LogSection {
                header: header.trim().to_string(),
                fields: Vec::new(),
            });
            continue;
        }
        let Some(rest) = line.strip_prefix("- ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(": ") else {
            continue;
        };
        let Some(section) = sections.last_mut() else {
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
            section.fields.push(LogField {
                name: name.to_string(),
                value: block.join("\n"),
            });
        } else {
            section.fields.push(LogField {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
    }
    sections
}

pub(crate) fn candidate_sections(world: &GivnWorld) -> Vec<LogSection> {
    parse_log_sections(&read_remediation_log(world))
        .into_iter()
        .filter(|section| section.header.starts_with("Candidate "))
        .collect()
}

pub(crate) fn run_sections(world: &GivnWorld) -> Vec<LogSection> {
    parse_log_sections(&read_remediation_log(world))
        .into_iter()
        .filter(|section| section.header.starts_with("Run "))
        .collect()
}

fn first_candidate_section(world: &GivnWorld) -> LogSection {
    let sections = candidate_sections(world);
    assert_eq!(sections.len(), 1, "exactly one candidate entry expected");
    sections.into_iter().next().unwrap()
}

/// Fields of the report's first `## Candidate` block.
fn report_candidate_fields(world: &GivnWorld) -> Vec<(String, String)> {
    let env = world.env.as_ref().expect("remediation project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    let mut fields = Vec::new();
    let mut in_candidate = false;
    for line in report.lines() {
        if line.starts_with("## Candidate ") {
            in_candidate = true;
            continue;
        }
        if in_candidate && line.starts_with("## ") {
            break;
        }
        if in_candidate {
            if let Some(rest) = line.strip_prefix("- ") {
                if let Some((name, value)) = rest.split_once(": ") {
                    fields.push((name.to_string(), value.to_string()));
                }
            }
        }
    }
    fields
}

fn report_field(fields: &[(String, String)], name: &str) -> String {
    fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| value.clone())
        .unwrap_or_else(|| panic!("report field {name} is missing"))
}

/// Independent reimplementation of the verbatim extraction used for
/// comparison against the log's block scalar.
fn expected_verbatim(root: &std::path::Path, file: &str, title: &str) -> String {
    let content = fs::read_to_string(root.join(file)).expect("read endpoint feature");
    let lines: Vec<&str> = content.lines().collect();
    let start = lines
        .iter()
        .position(|line| {
            let trimmed = line.trim();
            trimmed == format!("Scenario: {title}")
                || trimmed == format!("Scenario Outline: {title}")
        })
        .unwrap_or_else(|| panic!("scenario {title} not found in {file}"));
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
    lines[first..=last]
        .iter()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_entry_field(world: &GivnWorld, name: &str, value: &str) {
    let path = remediation_log_path(world);
    let log = fs::read_to_string(&path).expect("read remediation log for edit");
    let mut edited = String::new();
    let mut in_candidate = false;
    let mut replaced = false;
    for line in log.lines() {
        if line.starts_with("## Candidate ") {
            in_candidate = true;
            replaced = false;
        } else if line.starts_with("## ") {
            in_candidate = false;
        }
        if in_candidate && !replaced && line.starts_with(&format!("- {name}: ")) {
            edited.push_str(&format!("- {name}: {value}\n"));
            replaced = true;
            continue;
        }
        edited.push_str(line);
        edited.push('\n');
    }
    fs::write(&path, edited).expect("write remediation log edit");
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"(DUPLICATE|VARIANT|VALID-BOUNDARY)\" candidate and BGE and NLI evidence$"#
)]
fn remediation_project_with_second_stage(world: &mut GivnWorld, classification: String) {
    super::semantic_remediation_steps::remediation_project(world, classification);
    world.retrieval_bge = Some("stub".to_string());
}

#[given(
    "a project prepared for semantic remediation whose delta scenario contains blank lines and an Examples table"
)]
fn remediation_project_with_blank_lines(world: &mut GivnWorld) {
    unimplemented!("pending: blank-lines fixture")
}

fn git(world: &GivnWorld, args: &[&str]) {
    let env = world.env.as_ref().expect("project is required");
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(env.root())
        .args(args)
        .output()
        .expect("git is available");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_head(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("project is required");
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(env.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git is available");
    assert!(output.status.success(), "git rev-parse HEAD failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn init_git_repo(world: &GivnWorld) {
    git(world, &["init", "-q"]);
    git(world, &["config", "user.email", "fixture@givn.test"]);
    git(world, &["config", "user.name", "fixture"]);
    git(world, &["config", "commit.gpgsign", "false"]);
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"DUPLICATE\" candidate in a Git repository$"#
)]
fn remediation_project_in_git(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    init_git_repo(world);
    git(world, &["add", "-A"]);
    git(world, &["commit", "-q", "-m", "chore: fixture baseline"]);
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"DUPLICATE\" candidate in a Git repository without commits$"#
)]
fn remediation_project_in_git_without_commits(world: &mut GivnWorld) {
    unimplemented!("pending: unborn branch fixture")
}

#[given("a completed semantic remediation change with a generated log and archive prerequisites")]
fn completed_remediation_log_change(world: &mut GivnWorld) {
    unimplemented!("pending: completed log fixture")
}

#[when(
    regex = r#"^the user records the remediation fix \"([^\"]+)\" and commits \[([^\]]*)\] in the remediation log$"#
)]
fn record_fix_with_commits(world: &mut GivnWorld, fix: String, commits: String) {
    let commits = commits
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
    write_entry_field(
        world,
        "remediation",
        &format!("\"{}\"", fix.replace('"', "\\\"")),
    );
    write_entry_field(world, "commits", &format!("[{commits}]"));
}

#[when(
    regex = r#"^the user records the remediation fix \"([^\"]+)\" with no commits in the remediation log$"#
)]
fn record_fix_without_commits(world: &mut GivnWorld, fix: String) {
    write_entry_field(
        world,
        "remediation",
        &format!("\"{}\"", fix.replace('"', "\\\"")),
    );
    write_entry_field(world, "commits", "[]");
}

#[when("the user records the committed remediation fix and commit in the remediation log")]
fn record_committed_fix(world: &mut GivnWorld) {
    let hash = world
        .remediation_commit_hash
        .as_ref()
        .expect("remediation commit hash recorded")
        .clone();
    write_entry_field(
        world,
        "remediation",
        "\"removed the redundant scenario\"",
    );
    write_entry_field(world, "commits", &format!("[\"{hash}\"]"));
}

#[when("the user records both remediation fixes and the commit in the remediation log")]
fn record_both_committed_fixes(world: &mut GivnWorld) {
    unimplemented!("pending: record both fixes")
}

#[when("the user commits the remediation without the semantic-remediation stamp")]
fn commit_without_stamp(world: &mut GivnWorld) {
    git(world, &["add", "-A"]);
    git(
        world,
        &[
            "commit",
            "-q",
            "-m",
            "fix(running-change): remove the duplicate scenario",
        ],
    );
    world.remediation_commit_hash = Some(git_head(world));
}

#[when(
    regex = r#"^the user commits the remediation with the stamp naming candidate \"([^\"]+)\"$"#
)]
fn commit_with_wrong_stamp(world: &mut GivnWorld, candidate: String) {
    git(world, &["add", "-A"]);
    let message = format!(
        "fix(running-change): remove the duplicate scenario\n\nsemantic-remediation: running-change/{candidate}\n"
    );
    git(world, &["commit", "-q", "-m", &message]);
    world.remediation_commit_hash = Some(git_head(world));
}

#[when("the user commits the remediation with the matching semantic-remediation stamp")]
fn commit_with_matching_stamp(world: &mut GivnWorld) {
    let ids = report_candidate_ids_from_report(world);
    assert_eq!(ids.len(), 1, "one stamped candidate expected");
    git(world, &["add", "-A"]);
    let message = format!(
        "fix(running-change): remove the duplicate scenario\n\nsemantic-remediation: running-change/{}\n",
        ids[0]
    );
    git(world, &["commit", "-q", "-m", &message]);
    world.remediation_commit_hash = Some(git_head(world));
}

fn report_candidate_ids_from_report(world: &GivnWorld) -> Vec<String> {
    let env = world.env.as_ref().expect("remediation project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    report
        .lines()
        .filter_map(|line| line.strip_prefix("- candidate_id: "))
        .map(|value| value.trim().to_string())
        .collect()
}

#[when("the user commits the remediation with one matching stamp per candidate")]
fn commit_with_stamp_per_candidate(world: &mut GivnWorld) {
    unimplemented!("pending: per-candidate stamps")
}

#[when("the user removes the remediation commit from Git history")]
fn remove_commit_from_history(world: &mut GivnWorld) {
    git(world, &["reset", "-q", "--soft", "HEAD~1"]);
}

#[when("the classified duplicate candidates are removed from the active deltas")]
fn remove_both_duplicates(world: &mut GivnWorld) {
    unimplemented!("pending: remove both deltas")
}

#[when("the user deletes the remediation log")]
fn delete_remediation_log(world: &mut GivnWorld) {
    fs::remove_file(remediation_log_path(world)).expect("delete remediation log");
}

#[when("the active remediation log bytes are snapshotted")]
fn snapshot_remediation_log(world: &mut GivnWorld) {
    world.remediation_log_snapshot =
        Some(fs::read(remediation_log_path(world)).expect("snapshot log"));
}

#[then("the remediation log exists")]
fn remediation_log_exists(world: &mut GivnWorld) {
    assert!(
        remediation_log_path(world).exists(),
        "semantic remediation log is missing"
    );
}

#[then("the remediation log does not exist")]
fn remediation_log_does_not_exist(world: &mut GivnWorld) {
    assert!(
        !remediation_log_path(world).exists(),
        "semantic remediation log must not exist"
    );
}

#[then("the remediation log declares log schema version 1 and the change id")]
fn log_declares_schema_and_change(world: &mut GivnWorld) {
    let log = read_remediation_log(world);
    let mut lines = log.lines();
    assert_eq!(lines.next(), Some("# Semantic Remediation Log"));
    assert_eq!(lines.next(), Some("Log schema version: 1"));
    assert_eq!(lines.next(), Some("Change: \"running-change\""));
}

#[then(
    "the remediation log contains a before run record with embedding, BGE, and NLI stage durations"
)]
fn log_has_before_run_record(world: &mut GivnWorld) {
    let runs = run_sections(world);
    assert_eq!(runs.len(), 1, "exactly one run record expected");
    assert_eq!(runs[0].header, "Run 1");
    assert_eq!(runs[0].field("kind"), Some("before"));
    assert_eq!(runs[0].field("commit"), Some("null"), "no Git history");
    for name in ["e5_runtime_ms", "bge_runtime_ms", "nli_runtime_ms"] {
        let value = runs[0]
            .field(name)
            .unwrap_or_else(|| panic!("run record field {name} is missing"));
        assert!(
            value == "null" || value.parse::<u128>().is_ok(),
            "{name} must be an integer or null, got {value}"
        );
    }
}

#[then(
    "the remediation log contains a candidate entry with the report's candidate id and embedding score"
)]
fn log_has_candidate_entry(world: &mut GivnWorld) {
    let report_fields = report_candidate_fields(world);
    let entry = first_candidate_section(world);
    assert_eq!(
        entry.header,
        format!("Candidate {}", report_field(&report_fields, "candidate_id"))
    );
    assert_eq!(
        entry.field("before_embedding_score"),
        Some(report_field(&report_fields, "embedding_score").as_str())
    );
}

#[then("the candidate entry records the BGE and NLI evidence states from the report")]
fn entry_mirrors_evidence_states(world: &mut GivnWorld) {
    let report_fields = report_candidate_fields(world);
    let entry = first_candidate_section(world);
    for (report_name, entry_name) in [
        ("bge_admission", "before_bge_admission"),
        ("nli_admission", "before_nli_admission"),
    ] {
        let expected = report_fields
            .iter()
            .find(|(field, _)| field == report_name)
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| "null".to_string());
        assert_eq!(
            entry.field(entry_name),
            Some(expected.as_str()),
            "{entry_name} must mirror the report"
        );
    }
}

#[then("the candidate entry records the endpoint scenario text verbatim from the active delta")]
fn entry_records_verbatim_text(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let entry = first_candidate_section(world);
    for (file_field, title_field, block_field) in [
        (
            "before_left_file",
            "before_left_title",
            "before_left_gherkin",
        ),
        (
            "before_right_file",
            "before_right_title",
            "before_right_gherkin",
        ),
    ] {
        let file = entry.field(file_field).expect(file_field).trim_matches('"');
        let title = entry
            .field(title_field)
            .expect(title_field)
            .trim_matches('"');
        let expected = expected_verbatim(&env.root(), file, title);
        assert_eq!(
            entry.field(block_field),
            Some(expected.as_str()),
            "{block_field} must record the scenario text verbatim"
        );
    }
}

#[then("the candidate entry records the endpoint content and evidence hashes from the report")]
fn entry_records_report_hashes(world: &mut GivnWorld) {
    let report_fields = report_candidate_fields(world);
    let entry = first_candidate_section(world);
    for name in [
        "left_content_hash",
        "right_content_hash",
        "left_evidence_hash",
        "right_evidence_hash",
    ] {
        assert_eq!(
            entry.field(&format!("before_{name}")),
            Some(report_field(&report_fields, name).as_str()),
            "before_{name} must mirror the report"
        );
    }
}

#[then("the remediation log still contains exactly one candidate entry and one run record")]
fn log_is_not_grown(world: &mut GivnWorld) {
    assert_eq!(
        candidate_sections(world).len(),
        1,
        "unchanged rerun must not append a candidate entry"
    );
    assert_eq!(
        run_sections(world).len(),
        1,
        "unchanged rerun must not append a run record"
    );
}

#[then("the remediation log contains a second candidate entry with different endpoint hashes")]
fn log_has_second_entry(world: &mut GivnWorld) {
    let sections = candidate_sections(world);
    assert_eq!(sections.len(), 2, "exactly two candidate entries expected");
    let first = &sections[0];
    let second = &sections[1];
    assert_eq!(first.header, second.header, "same candidate id");
    let differing = ["before_left_content_hash", "before_left_evidence_hash"]
        .iter()
        .filter(|name| first.field(name) != second.field(name))
        .count();
    assert!(
        differing > 0,
        "the new entry must record changed endpoint hashes"
    );
    assert_eq!(
        second.field("run"),
        Some("2"),
        "the new entry links to run 2"
    );
    assert_eq!(
        first.field("run"),
        Some("1"),
        "the prior entry links to run 1"
    );
}

#[then("the prior candidate entry is retained unchanged")]
fn prior_entry_retained(world: &mut GivnWorld) {
    let snapshot = world
        .remediation_log_snapshot
        .as_ref()
        .expect("log snapshot is required");
    let snapshot_text = String::from_utf8_lossy(snapshot);
    let prior = parse_log_sections(&snapshot_text)
        .into_iter()
        .find(|section| section.header.starts_with("Candidate "))
        .expect("snapshot contains a candidate entry");
    let current = candidate_sections(world)
        .into_iter()
        .next()
        .expect("current log contains the prior entry");
    let render = |section: &LogSection| {
        section
            .fields
            .iter()
            .map(|field| format!("{}: {}", field.name, field.value))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        render(&prior),
        render(&current),
        "prior entry must be unchanged"
    );
}

#[then("the remediation log contains an after run record")]
fn log_has_after_run_record(world: &mut GivnWorld) {
    let runs = run_sections(world);
    assert!(
        runs.iter().any(|run| run.field("kind") == Some("after")),
        "an after run record is expected"
    );
}

#[then(
    regex = r#"^the removed candidate entry records after state \"endpoint-absent\" with a null after score$"#
)]
fn removed_entry_after_state(world: &mut GivnWorld) {
    let entry = candidate_sections(world)
        .into_iter()
        .next()
        .expect("candidate entry expected");
    assert_eq!(entry.field("after_state"), Some("endpoint-absent"));
    assert_eq!(entry.field("after_embedding_score"), Some("null"));
}

#[then(
    regex = r#"^the remediation log records after state \"outline-consolidated\" with the recomputed embedding score below the boundary$"#
)]
fn consolidated_entry_after_state(world: &mut GivnWorld) {
    let entry = candidate_sections(world)
        .into_iter()
        .next()
        .expect("candidate entry expected");
    assert_eq!(entry.field("after_state"), Some("outline-consolidated"));
    let score = entry
        .field("after_embedding_score")
        .expect("after score field")
        .parse::<f32>()
        .expect("after score is a four-decimal number");
    assert!(
        score < 0.9,
        "the recomputed score {score} must be below the 0.9000 boundary"
    );
}

#[then("the candidate entry records the consolidated outline text verbatim from the active delta")]
fn entry_records_consolidated_text(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let entry = candidate_sections(world)
        .into_iter()
        .next()
        .expect("candidate entry expected");
    let file = entry
        .field("before_left_file")
        .expect("left file")
        .trim_matches('"');
    let title = entry
        .field("before_left_title")
        .expect("left title")
        .trim_matches('"');
    let expected = expected_verbatim(&env.root(), file, title);
    assert_eq!(
        entry.field("after_left_gherkin"),
        Some(expected.as_str()),
        "after_left_gherkin must record the consolidated outline verbatim"
    );
}

#[then("the remediation log records the before run anchor as null")]
fn anchors_are_null(world: &mut GivnWorld) {
    let runs = run_sections(world);
    assert_eq!(runs.len(), 1, "one run record expected");
    assert_eq!(runs[0].field("kind"), Some("before"));
    assert_eq!(runs[0].field("commit"), Some("null"));
}

#[then("the remediation log records the after state with the stamped commit as the after anchor")]
fn after_anchor_is_stamped_commit(world: &mut GivnWorld) {
    let hash = world
        .remediation_commit_hash
        .as_ref()
        .expect("remediation commit hash recorded");
    let entry = candidate_sections(world)
        .into_iter()
        .next()
        .expect("candidate entry expected");
    assert_eq!(entry.field("after_commit"), Some(hash.as_str()));
    assert_eq!(
        entry.field("after_state"),
        Some("endpoint-absent"),
        "the duplicate entry records its verified after state"
    );
}

#[then("the remediation log records the after state on the second candidate entry")]
fn after_state_on_second_entry(world: &mut GivnWorld) {
    unimplemented!("pending: after on second entry")
}

#[then(
    "the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report"
)]
fn entry_records_non_null_evidence(world: &mut GivnWorld) {
    unimplemented!("pending: non-null evidence")
}

#[then("the remediation log has the exact headers and entry field order")]
fn log_has_exact_grammar(world: &mut GivnWorld) {
    unimplemented!("pending: exact grammar")
}

#[then("the remediation log contains only allowed fields and no unknown fields")]
fn log_has_only_allowed_fields(world: &mut GivnWorld) {
    unimplemented!("pending: allowed fields")
}

#[then("the remediation log has no timestamp or environment fields")]
fn log_has_no_timestamps(world: &mut GivnWorld) {
    unimplemented!("pending: no timestamps")
}

#[then("the remediation log ends with exactly one final newline")]
fn log_ends_with_one_newline(world: &mut GivnWorld) {
    unimplemented!("pending: final newline")
}

#[then("the verbatim scenario text preserves interior blank lines and the Examples table")]
fn verbatim_preserves_structure(world: &mut GivnWorld) {
    unimplemented!("pending: structure preserved")
}

#[then("the remediation log bytes equal the active log snapshot")]
fn log_bytes_equal_snapshot(world: &mut GivnWorld) {
    let snapshot = world
        .remediation_log_snapshot
        .as_ref()
        .expect("log snapshot is required");
    let current = fs::read(remediation_log_path(world)).expect("read log");
    assert_eq!(
        snapshot.as_slice(),
        current.as_slice(),
        "the log must be byte-identical after the rerun"
    );
}

#[then("`givn/archive/running-change/semantic-remediation.md` exists")]
fn archived_log_exists(world: &mut GivnWorld) {
    unimplemented!("pending: archived log exists")
}

#[then("the archived remediation log bytes equal the active log snapshot")]
fn archived_log_bytes_equal_snapshot(world: &mut GivnWorld) {
    unimplemented!("pending: archived bytes equal")
}

#[then("the active remediation log is unchanged by the archive run")]
fn active_log_unchanged_by_archive(world: &mut GivnWorld) {
    unimplemented!("pending: archive does not mutate log")
}
