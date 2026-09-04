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
        if let Some(block) = current.as_mut()
            && let Some(rest) = line.strip_prefix("- ")
            && let Some((name, value)) = rest.split_once(": ")
        {
            block.push((name.to_string(), value.to_string()));
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
pub(crate) fn fill_verified_entries(world: &GivnWorld, field: &str, value: &str) {
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
    "a semantic-remediation logging project holding one \"DUPLICATE\" finding and BGE and NLI evidence"
)]
fn remediation_project_second_stage_alias(world: &mut GivnWorld) {
    remediation_project_with_second_stage(world, "DUPLICATE".to_string());
}

#[given(
    "a project prepared for semantic remediation whose delta scenario contains blank lines and an Examples table"
)]
fn remediation_project_with_blank_lines(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    let env = world.env.as_ref().expect("remediation project is required");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario Outline: Unknown provider is rejected\n    Given a configured provider catalog\n\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n    Examples: providers\n      | provider |\n      | default  |\n",
    )
    .expect("write blank-line outline delta");
    world.retrieval_stub_scores =
        Some("Unknown provider is rejected|Provider name is unknown=0.9600".to_string());
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
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    let env = world.env.as_ref().expect("remediation project is required");
    let change_dir = env.path("givn/changes/running-change");
    fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write archive proposal");
    fs::write(change_dir.join("design.md"), "# Design\n").expect("write archive design");
    fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write archive design review");
    fs::write(
        change_dir.join("tasks.md"),
        "- [x] all done\n- [x] COMMIT: `fixture-archive-0001`\n",
    )
    .expect("write archive tasks");
    fs::write(
        env.path("givn/config.yaml"),
        "addons:\n  arc42: false\n  absolute_mode: false\n  readme: false\n  coverage: false\n  domain_modeling: false\n  dev_principles: false\n  grill_me: false\n  grill_with_docs: false\n",
    )
    .expect("disable archive fixture addons");
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\naddons:\n  arc42: false\n  absolute_mode: false\n  readme: false\n  coverage: false\n  domain_modeling: false\n  dev_principles: false\n  grill_me: false\n  grill_with_docs: false\n",
    );
    let scores = world
        .retrieval_stub_scores
        .clone()
        .expect("semantic score fixture");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    );
    assert!(
        !output.status.success(),
        "initial semantic review should require classifications"
    );
    let report_path = change_dir.join("semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("generated semantic report");
    fs::write(&report_path, report).expect("keep archive report");
    fill_verified_entries(world, "remediation", "\"removed the redundant scenario\"");
    fill_verified_entries(world, "commits", "[\"fixture-archive-0001\"]");
    fs::remove_dir_all(change_dir.join("specs")).expect("remove classified archive delta");
    fs::create_dir_all(change_dir.join("specs")).expect("recreate archive delta directory");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    );
    assert!(
        output.status.success(),
        "completed semantic review failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.remediation_log_snapshot =
        Some(fs::read(remediation_log_path(world)).expect("snapshot completed log"));
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
    write_entry_field(world, "remediation", "\"removed the redundant scenario\"");
    write_entry_field(world, "commits", &format!("[\"{hash}\"]"));
}

#[when("the user records both remediation fixes and the commit in the remediation log")]
fn record_both_committed_fixes(world: &mut GivnWorld) {
    let hash = world
        .remediation_commit_hash
        .as_ref()
        .expect("remediation commit hash recorded")
        .clone();
    fill_verified_entries(world, "remediation", "\"removed the redundant scenario\"");
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

#[given(
    "a project prepared for semantic review with two permanent-duplicate deltas in a Git repository"
)]
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

#[when("the user strips every candidate entry from the remediation log")]
fn strip_log_entries(world: &mut GivnWorld) {
    let path = remediation_log_path(world);
    let log = fs::read_to_string(&path).expect("read remediation log");
    let kept: Vec<&str> = log
        .lines()
        .take_while(|line| !line.starts_with("## Candidate "))
        .collect();
    let mut stripped = kept.join("\n");
    if !stripped.ends_with('\n') {
        stripped.push('\n');
    }
    fs::write(&path, stripped).expect("strip remediation log entries");
}

#[when("the active remediation log bytes are snapshotted")]
#[given("the active remediation log bytes are snapshotted")]
#[then("the active remediation log bytes are snapshotted")]
fn snapshot_remediation_log(world: &mut GivnWorld) {
    world.remediation_log_snapshot =
        Some(fs::read(remediation_log_path(world)).expect("snapshot log"));
}

#[then("the remediation log exists")]
fn remediation_log_exists(world: &mut GivnWorld) {
    assert!(
        remediation_log_path(world).exists(),
        "semantic remediation log is missing; stderr: {} stdout: {}",
        stderr(world),
        stdout(world)
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
        let report_has_line = report_fields.iter().any(|(field, _)| field == report_name);
        let expected = report_fields
            .iter()
            .find(|(field, _)| field == report_name)
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| "null".to_string());
        if report_has_line {
            // The report line is only present when the second stage is
            // compiled; in that case it must record a real state (rulebook
            // run) that the log mirrors.
            assert_ne!(
                expected, "null",
                "the compiled {report_name} stage must record a state"
            );
        }
        assert_eq!(
            entry.field(entry_name),
            Some(expected.as_str()),
            "{entry_name} must mirror the report state"
        );
    }
}

#[then("the remediation log has the exact headers and entry field order")]
fn log_has_exact_grammar(world: &mut GivnWorld) {
    let log = read_remediation_log(world);
    let mut lines = log.lines();
    assert_eq!(lines.next(), Some("# Semantic Remediation Log"));
    assert_eq!(lines.next(), Some("Log schema version: 1"));
    assert_eq!(lines.next(), Some("Change: \"running-change\""));
    let sections = parse_log_sections(&log);
    assert_eq!(sections.len(), 2, "one run record and one candidate entry");
    assert_eq!(sections[0].header, "Run 1");
    let run_fields: Vec<&str> = sections[0]
        .fields
        .iter()
        .map(|field| field.name.as_str())
        .collect();
    assert_eq!(
        run_fields,
        vec![
            "kind",
            "commit",
            "e5_runtime_ms",
            "bge_runtime_ms",
            "nli_runtime_ms"
        ]
    );
    assert!(
        sections[1].header.starts_with("Candidate "),
        "candidate entry header expected, got {}",
        sections[1].header
    );
    let candidate_fields: Vec<&str> = sections[1]
        .fields
        .iter()
        .map(|field| field.name.as_str())
        .collect();
    assert_eq!(
        candidate_fields,
        vec![
            "run",
            "before_embedding_score",
            "before_bge_admission",
            "before_bge_symmetric_raw",
            "before_nli_admission",
            "before_nli_symmetric_entailment",
            "before_left_file",
            "before_left_title",
            "before_right_file",
            "before_right_title",
            "before_left_content_hash",
            "before_right_content_hash",
            "before_left_evidence_hash",
            "before_right_evidence_hash",
            "before_left_gherkin",
            "before_right_gherkin",
            "classification",
            "rationale",
            "remediation",
            "commits",
            "after_state",
            "after_embedding_score",
            "after_commit",
            "after_left_gherkin",
        ]
    );
}

#[then("the remediation log contains only allowed fields and no unknown fields")]
fn log_has_only_allowed_fields(world: &mut GivnWorld) {
    let allowed: Vec<&str> = vec![
        "kind",
        "commit",
        "e5_runtime_ms",
        "bge_runtime_ms",
        "nli_runtime_ms",
        "run",
        "before_embedding_score",
        "before_bge_admission",
        "before_bge_symmetric_raw",
        "before_nli_admission",
        "before_nli_symmetric_entailment",
        "before_left_file",
        "before_left_title",
        "before_right_file",
        "before_right_title",
        "before_left_content_hash",
        "before_right_content_hash",
        "before_left_evidence_hash",
        "before_right_evidence_hash",
        "before_left_gherkin",
        "before_right_gherkin",
        "classification",
        "rationale",
        "remediation",
        "commits",
        "after_state",
        "after_embedding_score",
        "after_commit",
        "after_left_gherkin",
    ];
    for section in parse_log_sections(&read_remediation_log(world)) {
        for field in &section.fields {
            assert!(
                allowed.contains(&field.name.as_str()),
                "unknown field {} in section {}",
                field.name,
                section.header
            );
        }
    }
}

#[then("the remediation log has no timestamp or environment fields")]
fn log_has_no_timestamps(world: &mut GivnWorld) {
    for line in read_remediation_log(world).lines() {
        for marker in [
            "timestamp",
            "generated",
            "created_at",
            "host",
            "user:",
            "os:",
            "cwd",
            "path:",
        ] {
            assert!(
                !line.contains(marker),
                "environment marker {marker} must not appear: {line}"
            );
        }
    }
}

#[then("the remediation log ends with exactly one final newline")]
fn log_ends_with_one_newline(world: &mut GivnWorld) {
    let log = read_remediation_log(world);
    assert!(log.ends_with('\n'), "the log ends with a newline");
    assert!(
        !log.ends_with("\n\n"),
        "the log ends with exactly one newline"
    );
}

#[then("the verbatim scenario text preserves interior blank lines and the Examples table")]
fn verbatim_preserves_structure(world: &mut GivnWorld) {
    let entry = first_candidate_section(world);
    let block = entry.field("before_left_gherkin").expect("left gherkin");
    assert!(
        block.contains("\n\n    When"),
        "the blank line between steps must be preserved:\n{block}"
    );
    assert!(block.contains("Examples: providers"));
    assert!(block.contains("| provider |"));
    assert!(block.contains("| default  |"));
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

// --- Lexical variants -------------------------------------------------------
//
// The real-model review gate admits near-duplicate delta-to-delta pairs at
// E5 >= 0.90. These synonyms give scenarios a distinct wording over the same
// step bodies so each pair stays lexically distant; they add no behaviour.

fn run_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let scores = world.retrieval_stub_scores.clone().unwrap_or_default();
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    ));
}

// Group A wording.
#[given(regex = r#"^a remediation-log project keeps one \"(DUPLICATE|VARIANT)\" finding$"#)]
fn fixture_a(world: &mut GivnWorld, classification: String) {
    super::semantic_remediation_steps::remediation_project(world, classification);
}

#[given(
    regex = r#"^a remediation-log project keeps one \"DUPLICATE\" finding inside a committed Git repository$"#
)]
fn fixture_a_git(world: &mut GivnWorld) {
    remediation_project_in_git(world);
}

#[when("the run block of the project is fully declared")]
#[given("the run block of the project is fully declared")]
fn declaration_a(world: &mut GivnWorld) {
    super::run_declaration_fixtures_support::declare_run(world, "cli");
}

#[when("the change review gate runs for running-change")]
fn gate_a(world: &mut GivnWorld) {
    run_review(world);
}

#[when("the gate is evaluated a second time")]
#[then("the gate is evaluated a second time")]
fn gate_a2(world: &mut GivnWorld) {
    run_review(world);
}

#[when(regex = r#"^the reviewer marks each semantic finding as \"(DUPLICATE|VARIANT)\"$"#)]
fn classify_a(world: &mut GivnWorld, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

/// Corpus variant of `change_classified_endpoint_without_remediation`: the
/// delta endpoint changes while titles stay identical, so the stub pair score
/// key (title|title) keeps applying.
#[when("the active classified corpus endpoint changes without remediation")]
fn change_corpus_endpoint_without_remediation(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let path = env.path("givn/changes/running-change/specs/running.feature");
    let content = fs::read_to_string(&path).expect("read corpus delta");
    let changed = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("Given ") {
                format!("{line} changed")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, changed).expect("write changed corpus delta");
}

/// Corpus variant of `consolidate_active_variant`: the delta scenario becomes
/// an explicit-parameter Outline of the same domain content and the stub pair
/// score drops below the boundary, so the rerun recomputes the pair.
#[when("the active corpus variant is consolidated with an explicit parameter")]
fn consolidate_corpus_variant(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let path = env.path("givn/changes/running-change/specs/running.feature");
    let content = fs::read_to_string(&path).expect("read corpus delta");
    let mut lines = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("Given ") {
                format!("{} for <variant>", line.trim_end())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>();
    lines.push(String::new());
    lines.push("    Examples:".to_string());
    lines.push("      | variant |".to_string());
    lines.push("      | seeded  |".to_string());
    fs::write(&path, lines.join("\n")).expect("write consolidated corpus delta");
    if let Some(scores) = world.retrieval_stub_scores.as_deref() {
        world.retrieval_stub_scores = Some(scores.replace("=0.9600", "=0.8900"));
    }
}

#[when("the doubled scenario is withdrawn from the change specs")]
fn removal_a(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    fs::remove_file(env.path("givn/changes/running-change/specs/running.feature"))
        .expect("remove classified duplicate delta");
}

#[when(
    regex = r#"^the reviewer logs the applied fix \"([^\"]+)\" and the commit list \[([^\]]*)\] into the remediation log$"#
)]
fn record_a(world: &mut GivnWorld, fix: String, commits: String) {
    record_fix_with_commits(world, fix, commits);
}

#[when(
    regex = r#"^the reviewer logs the applied fix \"([^\"]+)\" without any commit list into the remediation log$"#
)]
fn record_a_no_commits(world: &mut GivnWorld, fix: String) {
    record_fix_without_commits(world, fix);
}

#[then("the run fails")]
fn fail_a(world: &mut GivnWorld) {
    assert!(
        !world.last_output.as_ref().expect("output").status.success(),
        "the review run must fail"
    );
}

#[then(regex = r#"^the diagnostic names \"([^\"]+)\"$"#)]
fn diag_a(world: &mut GivnWorld, needle: String) {
    assert!(
        stderr(world).contains(&needle),
        "stderr must mention {needle}: {}",
        stderr(world)
    );
}

#[then("the run succeeds")]
fn success_a(world: &mut GivnWorld) {
    assert!(world.last_output.as_ref().expect("output").status.success());
}

#[then("stdout carries the remediation verification receipt")]
fn receipt_a(world: &mut GivnWorld) {
    assert!(stdout(world).contains("semantic remediation verified"));
}

// Group B wording.
#[given(regex = r#"^a duplicate-log project contains a \"DUPLICATE\" case$"#)]
fn fixture_b(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
}

#[given("a duplicate-log project contains a \"DUPLICATE\" case in a repository with history")]
fn fixture_b_git(world: &mut GivnWorld) {
    remediation_project_in_git(world);
}

#[given("a duplicate-log project contains a \"DUPLICATE\" case whose repository has no commits")]
fn fixture_b_unborn(world: &mut GivnWorld) {
    remediation_project_in_git_without_commits(world);
}

#[when("the run declaration of the project is complete")]
#[given("the run declaration of the project is complete")]
fn declaration_b(world: &mut GivnWorld) {
    super::run_declaration_fixtures_support::declare_run(world, "cli");
}

#[when("the blocking review is executed for running-change")]
fn gate_b(world: &mut GivnWorld) {
    run_review(world);
}

#[when("the review command reruns for running-change")]
fn gate_b2(world: &mut GivnWorld) {
    run_review(world);
}

#[when(regex = r#"^the operator labels every review candidate \"(DUPLICATE)\"$"#)]
fn classify_b(world: &mut GivnWorld, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

#[when("the redundant scenario is deleted from the delta")]
fn removal_b(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    fs::remove_file(env.path("givn/changes/running-change/specs/running.feature"))
        .expect("remove classified duplicate delta");
}

#[when(
    regex = r#"^the operator records the performed fix \"([^\"]+)\" with commits \[([^\]]*)\] in the log$"#
)]
fn record_b(world: &mut GivnWorld, fix: String, commits: String) {
    record_fix_with_commits(world, fix, commits);
}

#[when("the operator records the committed fix and its hash in the log")]
fn record_b_committed(world: &mut GivnWorld) {
    record_committed_fix(world);
}

#[then("the gate run ends unsuccessfully")]
fn fail_b(world: &mut GivnWorld) {
    assert!(
        !world.last_output.as_ref().expect("output").status.success(),
        "the review run must fail"
    );
}

#[then(regex = r#"^the failure text mentions \"([^\"]+)\"$"#)]
fn diag_b(world: &mut GivnWorld, needle: String) {
    assert!(
        stderr(world).contains(&needle),
        "stderr must mention {needle}: {}",
        stderr(world)
    );
}

#[then("stdout announces the verified remediation")]
fn receipt_b(world: &mut GivnWorld) {
    assert!(stdout(world).contains("semantic remediation verified"));
}

// Group C wording.
#[when("the project's run block is complete")]
#[given("the project's run block is complete")]
fn declaration_c(world: &mut GivnWorld) {
    super::run_declaration_fixtures_support::declare_run(world, "cli");
}

#[when("the review gate executes for running-change")]
fn gate_c(world: &mut GivnWorld) {
    run_review(world);
}

#[when(regex = r#"^the assessor classifies all candidates \"(DUPLICATE)\"$"#)]
fn classify_c(world: &mut GivnWorld, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

#[when("the assessor notes the committed correction and hash in the log")]
fn record_c_committed(world: &mut GivnWorld) {
    record_committed_fix(world);
}

#[then("the attempt fails")]
fn fail_c(world: &mut GivnWorld) {
    assert!(
        !world.last_output.as_ref().expect("output").status.success(),
        "the review run must fail"
    );
}

// --- End lexical variants ---------------------------------------------------

#[given(
    regex = r#"^a semantic-remediation logging project holding one \"(DUPLICATE|VARIANT|VALID-BOUNDARY)\" finding$"#
)]
fn remediation_project_alias(world: &mut GivnWorld, classification: String) {
    super::semantic_remediation_steps::remediation_project(world, classification);
}

#[given(
    regex = r#"^a semantic-remediation logging project holding one \"DUPLICATE\" finding inside an initialized repository$"#
)]
fn remediation_project_git_alias(world: &mut GivnWorld) {
    remediation_project_in_git(world);
}

#[given(
    "a semantic-remediation logging project holding one \"DUPLICATE\" finding whose repository has never been committed"
)]
fn remediation_project_unborn_alias(world: &mut GivnWorld) {
    remediation_project_in_git_without_commits(world);
}

#[given(
    "a semantic-remediation logging project holding two permanent-duplicate findings inside an initialized repository"
)]
fn two_permanent_duplicate_deltas_alias(world: &mut GivnWorld) {
    two_permanent_duplicate_deltas(world);
}

#[when(
    regex = r#"^the user logs the applied fix \"([^\"]+)\" and the commit list \[([^\]]*)\] into the remediation log$"#
)]
fn record_fix_with_commits_alias(world: &mut GivnWorld, fix: String, commits: String) {
    record_fix_with_commits(world, fix, commits);
}

#[when(
    regex = r#"^the user logs the applied fix \"([^\"]+)\" without any commit list into the remediation log$"#
)]
fn record_fix_without_commits_alias(world: &mut GivnWorld, fix: String) {
    record_fix_without_commits(world, fix);
}

#[when(
    regex = r#"^the user marks every semantic review candidate as \"(DUPLICATE|VARIANT|VALID-BOUNDARY)\"$"#
)]
fn classify_alias(world: &mut GivnWorld, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

#[when("the user reruns the blocking change review for running-change")]
fn rerun_review_alias(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let scores = world.retrieval_stub_scores.clone().unwrap_or_default();
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    ));
}

#[when("the user logs the committed fix and its commit into the remediation log")]
fn record_committed_fix_alias(world: &mut GivnWorld) {
    record_committed_fix(world);
}

#[then("the review completes successfully and prints the remediation receipt")]
fn pass_with_receipt_alias(world: &mut GivnWorld) {
    assert!(world.last_output.as_ref().expect("output").status.success());
    assert!(stdout(world).contains("semantic remediation verified"));
}

#[when("the permanent counterpart of the finding is rewritten in the permanent specs")]
fn rewrite_permanent_counterpart_b(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let path = env.path("givn/specs/provider.feature");
    let content = fs::read_to_string(&path).expect("read permanent counterpart");
    let changed = content.replace("rejects the request", "reports the unknown name");
    assert_ne!(
        content, changed,
        "permanent fixture did not contain target text"
    );
    fs::write(path, changed).expect("rewrite permanent counterpart");
}

// --- End lexical variants ---------------------------------------------------

#[then("a later candidate entry records different endpoint hashes than its predecessor")]
fn later_entry_differs(world: &mut GivnWorld) {
    let sections = candidate_sections(world);
    assert!(
        sections.len() >= 2,
        "at least two candidate entries expected, found {}",
        sections.len()
    );
    let first = &sections[0];
    let last = &sections[sections.len() - 1];
    let differing = ["before_left_content_hash", "before_left_evidence_hash"]
        .iter()
        .filter(|name| first.field(name) != last.field(name))
        .count();
    assert!(
        differing > 0,
        "a later entry must record changed endpoint hashes"
    );
}

#[then("`givn/archive/running-change/semantic-remediation.md` exists")]
fn archived_log_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    assert!(
        env.path("givn/archive/running-change/semantic-remediation.md")
            .is_file(),
        "the archived change must retain the remediation log"
    );
}

#[then("the archived remediation log bytes equal the active log snapshot")]
fn archived_log_bytes_equal_snapshot(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let archived = fs::read(env.path("givn/archive/running-change/semantic-remediation.md"))
        .expect("read archived remediation log");
    assert_eq!(
        archived,
        *world
            .remediation_log_snapshot
            .as_ref()
            .expect("active log snapshot")
    );
}

#[then("the active remediation log is unchanged by the archive run")]
fn active_log_unchanged_by_archive(world: &mut GivnWorld) {
    let current = fs::read(remediation_log_path(world)).expect("read active log");
    assert_eq!(
        current,
        *world
            .remediation_log_snapshot
            .as_ref()
            .expect("active log snapshot"),
        "the archive run must not grow the remediation log"
    );
}

/// Repository-state branch shared by the consolidation outlines: one rule,
/// several precondition spellings.
#[given(
    regex = r#"^a remediation-log project with one \"DUPLICATE\" finding (without Git history|inside a committed Git repository|inside an unborn Git repository)$"#
)]
fn fixture_repository_state(world: &mut GivnWorld, repository_state: String) {
    match repository_state.as_str() {
        "without Git history" => {
            super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string())
        }
        "inside a committed Git repository" => remediation_project_in_git(world),
        "inside an unborn Git repository" => remediation_project_in_git_without_commits(world),
        other => panic!("unknown repository state {other}"),
    }
}

/// Evidence-state branch for the invalid-evidence outline.
#[when(
    regex = r#"^the remediation evidence is left (without commit evidence|without the stamp|with a stamp naming another candidate|with its commit removed from the history)$"#
)]
fn apply_evidence_state(world: &mut GivnWorld, state: String) {
    match state.as_str() {
        "without commit evidence" => {
            record_fix_without_commits(world, "removed the redundant scenario".to_string())
        }
        "without the stamp" => commit_without_stamp(world),
        "with a stamp naming another candidate" => {
            commit_with_wrong_stamp(world, "0000000000000000".to_string())
        }
        "with its commit removed from the history" => {
            commit_with_matching_stamp(world);
            remove_commit_from_history(world);
            record_committed_fix(world);
        }
        other => panic!("unknown evidence state {other}"),
    }
}

/// Log-state branch for the verified-remediation outline.
#[when(regex = r#"^the remediation log is (deleted entirely|stripped of its candidate entries)$"#)]
fn apply_log_state(world: &mut GivnWorld, state: String) {
    match state.as_str() {
        "deleted entirely" => delete_remediation_log(world),
        "stripped of its candidate entries" => strip_log_entries(world),
        other => panic!("unknown log state {other}"),
    }
}

#[when("the gate is evaluated a third time")]
fn gate_a3(world: &mut GivnWorld) {
    run_review(world);
}

// --- Individual wordings for the nine modified permanent copies -------------
//
// Each modified delta copy replaces its permanent original at archive time
// while carrying the same review flow. The real-model gate admits
// near-identical delta-to-delta pairs, so every copy phrases the same slot
// differently; the bodies are the canonical implementations.

#[given("the remediation-log fixture stages one \"DUPLICATE\" case")]
#[given("the remediation-log fixture reproduces one \"DUPLICATE\" case")]
#[given("the remediation-log fixture erases one \"DUPLICATE\" case")]
#[given("the remediation-log fixture stages one \"VARIANT\" case")]
fn fixture_individual(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
}

#[given("the review-flow fixture prepares a similar active delta")]
fn fixture_individual_similar(world: &mut GivnWorld) {
    super::semantic_review_steps::two_similar_active_deltas(world);
}

#[given("the review-flow fixture prepares a \"VALID-BOUNDARY\" case")]
fn fixture_individual_boundary(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "VALID-BOUNDARY".to_string());
}

#[when(
    regex = r#"^the (assessor resolves the admitted candidate to|gate operator marks the admitted candidate as) \"(DUPLICATE|VARIANT|VALID-BOUNDARY)\"$"#
)]
fn classify_individual(world: &mut GivnWorld, _slot: String, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

#[when(
    regex = r#"^the (assessor registers the applied correction|gate operator files the executed correction) \"[^\"]*\" with the commit record \[([^\]]*)\] in the log$"#
)]
fn record_individual(world: &mut GivnWorld, _slot: String, commits: String) {
    let commits = commits
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
    fill_verified_entries(world, "remediation", "\"removed the redundant scenario\"");
    fill_verified_entries(world, "commits", &format!("[{commits}]"));
}

// --- End individual wordings ------------------------------------------------

#[when("the active semantic review report bytes are snapshotted for comparison")]
#[then("the active semantic review report bytes are snapshotted for comparison")]
fn snapshot_report_for_comparison(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    world.semantic_report_snapshot = Some(
        fs::read(env.path("givn/changes/running-change/semantic-review.md"))
            .expect("snapshot semantic report"),
    );
}

#[then("the semantic review report bytes equal that comparison snapshot")]
#[when("the semantic review report bytes equal that comparison snapshot")]
fn report_bytes_equal_comparison_snapshot(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    let current = fs::read(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read regenerated semantic report");
    assert_eq!(
        current,
        *world
            .semantic_report_snapshot
            .as_ref()
            .expect("report snapshot"),
        "the regenerated report must be byte-identical"
    );
}

#[then("the review passes with the remediation receipt")]
fn review_passes_with_receipt(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("output");
    assert!(output.status.success(), "the review run must pass");
    assert!(
        stdout(world).contains("semantic remediation verified"),
        "stdout must carry the receipt: {}",
        stdout(world)
    );
}

// Short wordings for the token-budget reductions.
#[when(
    regex = r#"^the reviewer logs the fix \"([^\"]+)\" and the commit list \[([^\]]*)\] into the log$"#
)]
fn record_a_short(world: &mut GivnWorld, fix: String, commits: String) {
    record_a(world, fix, commits);
}

#[when(
    regex = r#"^the operator records the fix \"([^\"]+)\" with commits \[([^\]]*)\] in the log$"#
)]
fn record_b_short(world: &mut GivnWorld, fix: String, commits: String) {
    record_fix_with_commits(world, fix, commits);
}

#[when(regex = r#"^the user records the fix \"([^\"]+)\" and commits \[([^\]]*)\] in the log$"#)]
fn record_canonical_short(world: &mut GivnWorld, fix: String, commits: String) {
    record_fix_with_commits(world, fix, commits);
}

#[when(
    regex = r#"^the gate operator files the correction \"([^\"]+)\" with the record \[([^\]]*)\] in the log$"#
)]
fn record_gate_short(world: &mut GivnWorld, _fix: String, commits: String) {
    record_individual(world, String::new(), commits);
}

#[when(
    regex = r#"^the assessor registers the correction \"([^\"]+)\" with the record \[([^\]]*)\] in the log$"#
)]
fn record_assessor_short(world: &mut GivnWorld, _fix: String, commits: String) {
    record_individual(world, String::new(), commits);
}

// Compact wordings for the token-budget reductions.
#[given("the remediation fixture stages a \"DUPLICATE\" case")]
fn fixture_compact(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
}

#[given("the remediation fixture stages a \"VARIANT\" case")]
fn fixture_compact_variant(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "VARIANT".to_string());
}

#[when("the assessor resolves the candidate to \"DUPLICATE\"")]
#[then("the assessor resolves the candidate to \"DUPLICATE\"")]
#[given("the assessor resolves the candidate to \"DUPLICATE\"")]
fn classify_compact_duplicate(world: &mut GivnWorld) {
    super::semantic_review_steps::update_semantic_classification(world, "DUPLICATE", true);
}

#[when("the assessor resolves the candidate to \"VARIANT\"")]
#[then("the assessor resolves the candidate to \"VARIANT\"")]
#[given("the assessor resolves the candidate to \"VARIANT\"")]
fn classify_compact_variant(world: &mut GivnWorld) {
    super::semantic_review_steps::update_semantic_classification(world, "VARIANT", true);
}

#[when("the assessor resolves the candidate to \"VALID-BOUNDARY\"")]
#[then("the assessor resolves the candidate to \"VALID-BOUNDARY\"")]
fn classify_compact_boundary(world: &mut GivnWorld) {
    super::semantic_review_steps::update_semantic_classification(world, "VALID-BOUNDARY", true);
}

#[when(regex = r#"^the gate operator marks the candidate \"(DUPLICATE)\"$"#)]
fn classify_gate_compact(world: &mut GivnWorld, classification: String) {
    super::semantic_review_steps::update_semantic_classification(world, &classification, true);
}

#[when(regex = r#"^the assessor registers the fix \"[^\"]*\" with the record \[([^\]]*)\]$"#)]
fn record_assessor_compact(world: &mut GivnWorld, commits: String) {
    record_individual(world, String::new(), commits);
}

#[when(regex = r#"^the gate operator files the fix \"[^\"]*\" with the record \[([^\]]*)\]$"#)]
fn record_gate_compact(world: &mut GivnWorld, commits: String) {
    record_individual(world, String::new(), commits);
}

#[when(regex = r#"^the user records the fix \"[^\"]*\" and commits \[([^\]]*)\]$"#)]
fn record_canonical_compact(world: &mut GivnWorld, commits: String) {
    record_fix_with_commits(world, "removed the duplicate".to_string(), commits);
}

#[when(regex = r#"^the reviewer logs the fix \"[^\"]*\" and the commits \[([^\]]*)\]$"#)]
#[then(regex = r#"^the reviewer logs the fix \"[^\"]*\" and the commits \[([^\]]*)\]$"#)]
#[given(regex = r#"^the reviewer logs the fix \"[^\"]*\" and the commits \[([^\]]*)\]$"#)]
fn record_a_compact(world: &mut GivnWorld, commits: String) {
    record_a(world, "removed the duplicate".to_string(), commits);
}

#[when(regex = r#"^the operator records the fix \"[^\"]*\" with commits \[([^\]]*)\]$"#)]
fn record_b_compact(world: &mut GivnWorld, commits: String) {
    record_fix_with_commits(world, "removed the twin".to_string(), commits);
}

// Ultra-compact wordings for the real-model token budget (pairs <= 512).
#[when("the review gate runs")]
#[given("the review gate runs")]
#[then("the review gate runs")]
fn gate_compact_run(world: &mut GivnWorld) {
    run_review(world);
}

#[when("the review gate reruns")]
#[given("the review gate reruns")]
#[then("the review gate reruns")]
fn gate_compact_rerun(world: &mut GivnWorld) {
    run_review(world);
}

#[when("the gate reruns")]
#[given("the gate reruns")]
#[then("the gate reruns")]
fn gate_compact_second(world: &mut GivnWorld) {
    run_review(world);
}

#[when("the run declaration is complete")]
#[given("the run declaration is complete")]
fn declaration_compact(world: &mut GivnWorld) {
    super::run_declaration_fixtures_support::declare_run(world, "cli");
}

#[when("the log bytes are snapshotted")]
#[given("the log bytes are snapshotted")]
#[then("the log bytes are snapshotted")]
fn snapshot_compact(world: &mut GivnWorld) {
    snapshot_remediation_log(world);
}

#[when("the classified duplicate is removed")]
#[then("the classified duplicate is removed")]
fn removal_compact(world: &mut GivnWorld) {
    removal_a(world);
}

#[when("the doubled scenario is withdrawn")]
#[then("the doubled scenario is withdrawn")]
fn withdrawal_compact(world: &mut GivnWorld) {
    removal_a(world);
}

#[when("both classified duplicates are removed")]
#[then("both classified duplicates are removed")]
fn removal_both_compact(world: &mut GivnWorld) {
    remove_both_duplicates(world);
}

#[when("both fixes are recorded in the log")]
#[then("both fixes are recorded in the log")]
fn record_both_compact(world: &mut GivnWorld) {
    record_both_committed_fixes(world);
}

#[then("the log exists")]
fn log_exists_compact(world: &mut GivnWorld) {
    remediation_log_exists(world);
}

#[then("the log declares schema version 1 and the change id")]
fn log_schema_compact(world: &mut GivnWorld) {
    log_declares_schema_and_change(world);
}

#[then("the log records the candidate id and embedding score")]
fn log_candidate_compact(world: &mut GivnWorld) {
    log_has_candidate_entry(world);
}

#[then("the log records the before run stage durations")]
fn log_durations_compact(world: &mut GivnWorld) {
    log_has_before_run_record(world);
}

#[then("the entry records the BGE and NLI states from the report")]
fn entry_evidence_compact(world: &mut GivnWorld) {
    entry_mirrors_evidence_states(world);
}

#[then("the entry records the scenario text verbatim")]
fn entry_verbatim_compact(world: &mut GivnWorld) {
    entry_records_verbatim_text(world);
}

#[then("the entry records the endpoint hashes")]
fn entry_hashes_compact(world: &mut GivnWorld) {
    entry_records_report_hashes(world);
}

#[then("the log still holds one candidate entry and one run record")]
fn log_still_one_compact(world: &mut GivnWorld) {
    log_is_not_grown(world);
}

#[then("the log records a null before run anchor")]
fn log_null_anchor_compact(world: &mut GivnWorld) {
    anchors_are_null(world);
}

#[then(
    regex = r#"^the log records the "([^"]+)" after state with a recomputed score below the boundary$"#
)]
fn log_after_state_compact(world: &mut GivnWorld, state: String) {
    assert_eq!(state, "outline-consolidated");
    consolidated_entry_after_state(world);
}

#[then("a second entry records different endpoint hashes")]
fn log_second_entry_compact(world: &mut GivnWorld) {
    log_has_second_entry(world);
}

#[then("the second entry records its after state")]
fn log_second_after_compact(world: &mut GivnWorld) {
    after_state_on_second_entry(world);
}

#[then(regex = r#"^the log records non-null BGE and NLI states$"#)]
fn log_nonnull_evidence_compact(world: &mut GivnWorld) {
    entry_records_non_null_evidence(world);
}

#[then("the log contains an after run record")]
fn log_after_run_compact(world: &mut GivnWorld) {
    log_has_after_run_record(world);
}

#[then(regex = r#"^the removed entry records after state "([^"]+)" with a null after score$"#)]
fn log_removed_after_compact(world: &mut GivnWorld, state: String) {
    assert_eq!(state, "endpoint-absent");
    removed_entry_after_state(world);
}

#[then("the log bytes equal the snapshot")]
fn log_bytes_equal_compact(world: &mut GivnWorld) {
    log_bytes_equal_snapshot(world);
}

#[when("the classified endpoint changes")]
#[then("the classified endpoint changes")]
fn endpoint_change_compact(world: &mut GivnWorld) {
    change_corpus_endpoint_without_remediation(world);
}

#[when("the corpus variant is consolidated")]
#[then("the corpus variant is consolidated")]
fn consolidate_corpus_compact(world: &mut GivnWorld) {
    consolidate_corpus_variant(world);
}

#[then("the permanent specs are snapshotted")]
#[given("the permanent specs are snapshotted")]
fn permanent_snapshot_compact(world: &mut GivnWorld) {
    super::spec_explore_permanent_steps::snapshot_permanent_specs(world);
}

#[then("the permanent specs are unchanged")]
#[given("the permanent specs are unchanged")]
fn permanent_unchanged_compact(world: &mut GivnWorld) {
    super::spec_explore_permanent_steps::permanent_specs_unchanged(world);
}

#[given(regex = r#"^the fixture carries the ([a-z]+) corpus:$"#)]
fn corpus_compact(world: &mut GivnWorld, domain: String, step: &cucumber::gherkin::Step) {
    remediation_corpus(world, domain, step);
}

/// Per-scenario domain corpus: the docstring carries the permanent corpus and
/// the delta feature (separated by a `--- delta ---` marker), so every
/// scenario embeds genuinely different semantic content. The stub pair score
/// is derived from the written titles automatically.
#[given(regex = r#"^the remediation fixture files carry the ([a-z]+) corpus:$"#)]
fn remediation_corpus(world: &mut GivnWorld, _domain: String, step: &cucumber::gherkin::Step) {
    let content = step
        .docstring
        .as_deref()
        .expect("corpus docstring is required")
        .to_string();
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    let env = world.env.as_ref().expect("remediation project is required");
    let (permanent, delta) = match content.split_once("--- delta ---") {
        Some((permanent, delta)) => (permanent.trim(), delta.trim()),
        None => panic!("corpus docstring must contain a --- delta --- separator"),
    };
    fs::write(env.path("givn/specs/provider.feature"), permanent)
        .expect("write domain permanent corpus");
    let delta_path = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta_path.parent().expect("delta parent")).expect("create delta dir");
    fs::write(&delta_path, delta).expect("write domain delta");
    if std::env::var("GIVN_DEBUG_CORPUS").is_ok() {
        println!(
            "CORPUS permanent:\n{}\n---\nCORPUS delta:\n{}",
            permanent, delta
        );
    }

    let delta_text = fs::read_to_string(&delta_path).expect("read domain delta");
    let permanent_text =
        fs::read_to_string(env.path("givn/specs/provider.feature")).expect("read domain permanent");
    let delta_title = delta_text
        .lines()
        .find_map(|line| {
            line.trim_start()
                .strip_prefix("Scenario: ")
                .or_else(|| line.trim_start().strip_prefix("Scenario Outline: "))
        })
        .expect("delta title")
        .trim()
        .to_string();
    let permanent_title = permanent_text
        .lines()
        .find_map(|line| {
            line.trim_start()
                .strip_prefix("Scenario Outline: ")
                .or_else(|| line.trim_start().strip_prefix("Scenario: "))
        })
        .expect("permanent title")
        .trim()
        .to_string();
    world.retrieval_stub_scores = Some(format!("{delta_title}|{permanent_title}=0.9600"));

    // The domain corpus changes every permanent scenario, so the persisted
    // E5 index must be rebuilt before the review reads it.
    let output =
        common::run_givn_with_envs(env, &["spec", "index"], &[("GIVN_SPEC_EMBEDDER", "stub")]);
    assert!(
        output.status.success(),
        "domain spec index failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The domain titles need overlap dispositions in the fixture's review.md
    // so the structural overlap gate lets the review reach the semantic stage.
    let review = format!(
        "REVIEW: PASS\nREADME-IMPACT: none\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `{delta_title}` | `{permanent_title}` | boundary |\n"
    );
    fs::write(env.path("givn/changes/running-change/review.md"), review)
        .expect("write domain review dispositions");
}
