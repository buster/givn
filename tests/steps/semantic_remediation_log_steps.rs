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

/// Fields of every `## Candidate` block in the report.
fn report_candidate_blocks(world: &GivnWorld) -> Vec<Vec<(String, String)>> {
    let env = world.env.as_ref().expect("remediation project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    let mut blocks = Vec::new();
    let mut current: Option<Vec<(String, String)>> = None;
    for line in report.lines() {
        if line.starts_with("## Candidate ") {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            current = Some(Vec::new());
            continue;
        }
        if let Some(block) = current.as_mut() {
            if let Some(rest) = line.strip_prefix("- ") {
                if let Some((name, value)) = rest.split_once(": ") {
                    block.push((name.to_string(), value.to_string()));
                }
            }
        }
    }
    if let Some(block) = current.take() {
        blocks.push(block);
    }
    blocks
}

fn report_candidate_fields(world: &GivnWorld) -> Vec<(String, String)> {
    report_candidate_blocks(world)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Fill `field` with `value` in every log entry whose endpoint evidence hashes
/// match a candidate in the current report (the entries the gate will verify).
fn fill_verified_entries(world: &GivnWorld, field: &str, value: &str) {
    let wanted: Vec<(String, String)> = report_candidate_blocks(world)
        .iter()
        .filter_map(|block| {
            let left = block
                .iter()
                .find(|(name, _)| name == "left_evidence_hash")
                .map(|(_, value)| value.clone())?;
            let right = block
                .iter()
                .find(|(name, _)| name == "right_evidence_hash")
                .map(|(_, value)| value.clone())?;
            Some((left, right))
        })
        .collect();
    let path = remediation_log_path(world);
    let log = fs::read_to_string(&path).expect("read remediation log for edit");
    let mut edited = String::new();
    let mut in_candidate = false;
    let mut matches = false;
    let mut left_hash = String::new();
    let mut right_hash = String::new();
    for line in log.lines() {
        if line.starts_with("## Candidate ") {
            in_candidate = true;
            matches = false;
            left_hash.clear();
            right_hash.clear();
        } else if line.starts_with("## ") {
            in_candidate = false;
        }
        if in_candidate {
            if let Some(hash) = line.strip_prefix("- before_left_evidence_hash: ") {
                left_hash = hash.trim().to_string();
            }
            if let Some(hash) = line.strip_prefix("- before_right_evidence_hash: ") {
                right_hash = hash.trim().to_string();
            }
            if !left_hash.is_empty() && !right_hash.is_empty() && !matches {
                matches = wanted
                    .iter()
                    .any(|(left, right)| left == &left_hash && right == &right_hash);
            }
        }
        if in_candidate
            && matches
            && line.starts_with(&format!("- {field}: "))
            && (line.ends_with(": null") || line.ends_with(": []"))
        {
            edited.push_str(&format!("- {field}: {value}\n"));
            continue;
        }
        edited.push_str(line);
        edited.push('\n');
    }
    fs::write(&path, edited).expect("write remediation log edit");
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
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    init_git_repo(world);
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
    fill_verified_entries(
        world,
        "remediation",
        &format!("\"{}\"", fix.replace('"', "\\\"")),
    );
    fill_verified_entries(world, "commits", &format!("[{commits}]"));
}

#[when(
    regex = r#"^the user records the remediation fix \"([^\"]+)\" with no commits in the remediation log$"#
)]
fn record_fix_without_commits(world: &mut GivnWorld, fix: String) {
    fill_verified_entries(
        world,
        "remediation",
        &format!("\"{}\"", fix.replace('"', "\\\"")),
    );
    fill_verified_entries(world, "commits", "[]");
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
    let hash = world
        .remediation_commit_hash
        .as_ref()
        .expect("remediation commit hash recorded")
        .clone();
    fill_verified_entries(
        world,
        "remediation",
        "\"removed the redundant scenario\"",
    );
    fill_verified_entries(world, "commits", &format!("[\"{hash}\"]"));
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
    let ids = report_candidate_ids_from_report(world);
    assert_eq!(ids.len(), 2, "two stamped candidates expected");
    git(world, &["add", "-A"]);
    let mut message = String::from("fix(running-change): remove both duplicate scenarios\n");
    for id in &ids {
        message.push_str(&format!("\nsemantic-remediation: running-change/{id}\n"));
    }
    git(world, &["commit", "-q", "-m", &message]);
    world.remediation_commit_hash = Some(git_head(world));
}

#[when("the user removes the remediation commit from Git history")]
fn remove_commit_from_history(world: &mut GivnWorld) {
    git(world, &["reset", "-q", "--soft", "HEAD~1"]);
}

#[given("a project prepared for semantic review with two permanent-duplicate deltas in a Git repository")]
fn two_permanent_duplicate_deltas(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("retrieval.feature"),
        "Feature: retrieval fixtures\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Authentication token is missing\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n\n  Scenario: Provider configuration is valid\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write permanent corpus");
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    );
    let first = env.path("givn/changes/running-change/specs/first.feature");
    let second = env.path("givn/changes/running-change/specs/second.feature");
    fs::create_dir_all(first.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        &first,
        "@givn.delta @running-change\nFeature: first\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write first delta");
    fs::write(
        &second,
        "@givn.delta @running-change\nFeature: second\n\n  @givn.added\n  Scenario: Missing authentication token is rejected\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n",
    )
    .expect("write second delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Provider name is unknown` | related |\n| `Missing authentication token is rejected` | `Authentication token is missing` | related |\n| `Unknown provider is rejected` | `Missing authentication token is rejected` | related |\n| `Unknown provider is rejected` | `Authentication token is missing` | related |\n| `Missing authentication token is rejected` | `Provider name is unknown` | related |\n",
    )
    .expect("write overlap dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores = Some(
        "Unknown provider is rejected|Provider name is unknown=0.9600;Missing authentication token is rejected|Authentication token is missing=0.9600".to_string(),
    );
    init_git_repo(world);
    git(world, &["add", "-A"]);
    git(world, &["commit", "-q", "-m", "chore: fixture baseline"]);
}

#[when("the classified duplicate candidates are removed from the active deltas")]
fn remove_both_duplicates(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    fs::remove_file(env.path("givn/changes/running-change/specs/first.feature"))
        .expect("remove first duplicate delta");
    fs::remove_file(env.path("givn/changes/running-change/specs/second.feature"))
        .expect("remove second duplicate delta");
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
    let sections = candidate_sections(world);
    let first = sections
        .first()
        .expect("the current log contains the prior entry");
    assert_eq!(first.field("run"), Some("1"), "prior entry links to run 1");
    assert_eq!(first.field("remediation"), Some("null"));
    assert_eq!(first.field("commits"), Some("[]"));
    assert_eq!(first.field("after_state"), Some("null"));
    assert_eq!(first.field("after_commit"), Some("null"));
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
    let sections = candidate_sections(world);
    assert_eq!(sections.len(), 2, "two candidate entries expected");
    assert_eq!(sections[0].field("after_state"), Some("null"));
    assert_eq!(sections[0].field("after_commit"), Some("null"));
    assert_eq!(
        sections[1].field("after_state"),
        Some("endpoint-absent"),
        "the verified (second) entry records the after state"
    );
}

#[then(
    "the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report"
)]
fn entry_records_non_null_evidence(world: &mut GivnWorld) {
    let report_fields = report_candidate_fields(world);
    let entry = first_candidate_section(world);
    for (report_name, entry_name) in [
        ("bge_admission", "before_bge_admission"),
        ("nli_admission", "before_nli_admission"),
    ] {
        let expected = report_field(&report_fields, report_name);
        assert_ne!(expected, "null", "the report must record {report_name}");
        assert_eq!(
            entry.field(entry_name),
            Some(expected.as_str()),
            "{entry_name} must mirror the non-null report state"
        );
    }
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
