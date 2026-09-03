//! CLI E2E steps for the semantic remediation log.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

fn git(world: &GivnWorld, change: &str, args: &[&str]) {
    let env = world.env.as_ref().expect("e2e project is required");
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(env.root())
        .args(args)
        .output()
        .expect("git is available");
    assert!(
        output.status.success(),
        "git {args:?} failed for {change}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_head(env: &common::TestEnv) -> String {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(env.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git is available");
    assert!(output.status.success(), "git rev-parse HEAD failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// The stamped-log E2E project: a delta-to-permanent duplicate under a custom
/// change id, inside a real Git repository with a baseline commit.
fn staged_remediation_log_project(world: &mut GivnWorld, change: &str) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    let env = world.env.as_ref().expect("e2e project is required");
    fs::rename(
        env.path("givn/changes/running-change"),
        env.path(&format!("givn/changes/{change}")),
    )
    .expect("rename change directory");
    git(world, change, &["init", "-q"]);
    git(
        world,
        change,
        &["config", "user.email", "fixture@givn.test"],
    );
    git(world, change, &["config", "user.name", "fixture"]);
    git(world, change, &["config", "commit.gpgsign", "false"]);
    git(world, change, &["add", "-A"]);
    git(
        world,
        change,
        &["commit", "-q", "-m", "chore: fixture baseline"],
    );
}

fn run_change_review(world: &mut GivnWorld, change: &str) {
    let env = world.env.as_ref().expect("e2e project is required");
    let scores = world
        .retrieval_stub_scores
        .clone()
        .expect("stub score fixture");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", change],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    ));
}

#[given(
    regex = r#"^an end-to-end project staged for a stamped remediation log on change "([^"]+)"$"#
)]
fn e2e_stamped_remediation_log_project(world: &mut GivnWorld, change: String) {
    staged_remediation_log_project(world, &change);
    run_change_review(world, &change);
    assert!(
        !world
            .last_output
            .as_ref()
            .expect("review output")
            .status
            .success(),
        "initial E2E review must be blocked"
    );
}

#[when(
    regex = r#"^the remediation E2E user runs change review for "([^"]+)" after classifying, removing, and stamp-committing the duplicate$"#
)]
fn e2e_review_with_stamped_remediation(world: &mut GivnWorld, change: String) {
    let change_dir = {
        let env = world.env.as_ref().expect("e2e project is required");
        env.path(&format!("givn/changes/{change}"))
    };
    let report_path = change_dir.join("semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("read E2E semantic report");
    let candidate_id = report
        .lines()
        .find_map(|line| line.strip_prefix("- candidate_id: "))
        .expect("candidate id in E2E report")
        .trim()
        .to_string();
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLICATE")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(&report_path, report).expect("classify E2E report");
    let log_path = change_dir.join("semantic-remediation.md");
    let log = fs::read_to_string(&log_path).expect("read E2E remediation log");
    let log = log.replace(
        "- remediation: null",
        "- remediation: \"removed the redundant scenario\"",
    );
    fs::write(&log_path, log).expect("stage E2E log edit");
    fs::remove_file(change_dir.join("specs/running.feature"))
        .expect("remove duplicate delta for E2E");
    git(
        world,
        &change,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            &format!(
                "fix({change}): remove the duplicate scenario\n\nsemantic-remediation: {change}/{candidate_id}\n"
            ),
        ],
    );
    let hash = git_head(world.env.as_ref().expect("e2e project is required"));
    let log = fs::read_to_string(&log_path).expect("read E2E remediation log");
    let log = log.replace("- commits: []", &format!("- commits: [\"{hash}\"]"));
    fs::write(&log_path, log).expect("record E2E commit in log");
    run_change_review(world, &change);
}

#[when(
    regex = r#"^the remediation E2E user archives change "([^"]+)" after a verified stamped remediation$"#
)]
fn e2e_archive_after_verified_remediation(world: &mut GivnWorld, change: String) {
    let change_dir = {
        let env = world.env.as_ref().expect("e2e project is required");
        env.path(&format!("givn/changes/{change}"))
    };
    let report_path = change_dir.join("semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("read E2E semantic report");
    let candidate_id = report
        .lines()
        .find_map(|line| line.strip_prefix("- candidate_id: "))
        .expect("candidate id in E2E report")
        .trim()
        .to_string();
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLICATE")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(&report_path, report).expect("classify E2E report");
    let log_path = change_dir.join("semantic-remediation.md");
    let log = fs::read_to_string(&log_path).expect("read E2E remediation log");
    let log = log.replace(
        "- remediation: null",
        "- remediation: \"removed the redundant scenario\"",
    );
    fs::write(&log_path, log).expect("stage E2E log edit");
    fs::remove_file(change_dir.join("specs/running.feature"))
        .expect("remove duplicate delta for E2E");
    git(
        world,
        &change,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            &format!(
                "fix({change}): remove the duplicate scenario\n\nsemantic-remediation: {change}/{candidate_id}\n"
            ),
        ],
    );
    let hash = git_head(world.env.as_ref().expect("e2e project is required"));
    let log = fs::read_to_string(&log_path).expect("read E2E remediation log");
    let log = log.replace("- commits: []", &format!("- commits: [\"{hash}\"]"));
    fs::write(&log_path, log).expect("record E2E commit in log");
    run_change_review(world, &change);
    assert!(
        world
            .last_output
            .as_ref()
            .expect("review output")
            .status
            .success(),
        "the stamped E2E remediation must verify: {}",
        String::from_utf8_lossy(&world.last_output.as_ref().expect("review output").stderr)
    );
    fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write archive proposal");
    fs::write(change_dir.join("design.md"), "# Design\n").expect("write archive design");
    fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write archive design review");
    fs::write(
        change_dir.join("tasks.md"),
        "- [x] all done\n- [x] COMMIT: `fixture-archive-0001`\n",
    )
    .expect("write archive tasks");
    {
        let env = world.env.as_ref().expect("e2e project is required");
        fs::write(
            env.path("givn/config.yaml"),
            "addons:\n  arc42: false\n  absolute_mode: false\n  readme: false\n  coverage: false\n  domain_modeling: false\n  dev_principles: false\n  grill_me: false\n  grill_with_docs: false\n",
        )
        .expect("disable E2E archive fixture addons");
    }
    let scores = world
        .retrieval_stub_scores
        .clone()
        .expect("stub score fixture");
    world.last_output = Some(common::run_givn_with_envs(
        world.env.as_ref().expect("e2e project is required"),
        &["archive", "--change", &change],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    ));
}

#[then("the archived remediation log exists next to the archived semantic report")]
fn archived_log_next_to_report(world: &mut GivnWorld) {
    let change = "rc-log-archive-e2e";
    let env = world.env.as_ref().expect("e2e project is required");
    assert!(
        env.path(&format!("givn/archive/{change}/semantic-remediation.md"))
            .is_file(),
        "the archived change must retain the remediation log"
    );
    assert!(
        env.path(&format!("givn/archive/{change}/semantic-review.md"))
            .is_file(),
        "the archived change must retain the semantic report"
    );
}

#[then(
    "the remediation log records the before and after run records with stage durations and the stamped commit"
)]
fn e2e_log_records_runs_and_stamp(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e project is required");
    let log = fs::read_to_string(env.path("givn/changes/rc-log-e2e/semantic-remediation.md"))
        .expect("read E2E remediation log");
    assert!(log.contains("- kind: before"), "before run record expected");
    assert!(log.contains("- kind: after"), "after run record expected");
    assert!(
        log.contains("- e5_runtime_ms: "),
        "E5 stage duration expected"
    );
    assert!(
        log.contains("- bge_runtime_ms: "),
        "BGE stage field expected"
    );
    assert!(
        log.contains("- nli_runtime_ms: "),
        "NLI stage field expected"
    );
    assert!(
        !log.contains("- commits: []"),
        "the verified entry records the stamped commit"
    );
}
