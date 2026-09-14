//! Step definitions for the archive-proof capability (regular scenarios).

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use cucumber::{given, then};

use super::archive_proof_support as support;
use crate::common;
use crate::steps::GivnWorld;

fn project_base(world: &GivnWorld) -> PathBuf {
    support::project_base(world)
}

// ─── Given: fixtures ───────────────────────────────────────────────────────

#[given(regex = r#"^a givn project with a completed change "([^"]+)" ready to archive$"#)]
fn ready_change(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
}

#[given(regex = r#"^a (Git|Fossil) project with a completed change "([^"]+)" ready to archive$"#)]
fn backend_ready_change(world: &mut GivnWorld, backend: String, id: String) {
    support::ready_change_on(world, support::backend_kind(&backend), &id, false, "pass");
}

#[then("the candidate revision is published")]
fn candidate_revision_published(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert!(
        support::revision_contains(world, &format!("givn/archive/{id}/verification.json")),
        "the candidate revision does not contain the pending receipt; stderr: {}",
        crate::steps::stderr(world)
    );
}

#[then("the candidate revision contains exactly the merged corpus paths")]
fn candidate_revision_exact_paths(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let prefixes = [
        "givn/specs/".to_string(),
        format!("givn/archive/{id}/"),
        format!("givn/changes/{id}/"),
    ];
    let paths = support::revision_paths(world);
    assert!(!paths.is_empty(), "the candidate revision changed no paths");
    for path in &paths {
        assert!(
            prefixes.iter().any(|prefix| path.starts_with(prefix)),
            "the candidate revision contains a path outside the merged corpus: {path}"
        );
    }
    assert!(
        paths.iter().any(|path| path.starts_with("givn/specs/")),
        "the candidate revision does not contain the merged permanent specs"
    );
    assert!(
        paths
            .iter()
            .any(|path| path == &format!("givn/archive/{id}/verification.json")),
        "the candidate revision does not contain the pending receipt"
    );
}

#[given(
    regex = r#"^a givn project with a completed change "([^"]+)" whose delta adds an @e2e scenario$"#
)]
fn ready_change_with_e2e(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, true, "record");
}

#[given("a givn project without Git version control")]
fn project_without_git(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root_without_git());
}

#[given("a givn project nested inside a larger Git repository")]
fn nested_project(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    common::git_init(&env);
    let nested = env.path("nested");
    fs::create_dir_all(&nested).expect("create nested project dir");
    let out = common::run_givn_with_envs_in(&env, &nested, &["init"], &[]);
    assert!(
        out.status.success(),
        "givn init in nested project failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.cwd_override = Some(nested);
    world.env = Some(env);
}

#[given(regex = r#"^the project has an uncommitted change to "([^"]+)"$"#)]
fn uncommitted_change(world: &mut GivnWorld, path: String) {
    let base = project_base(world);
    let target = base.join(&path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&target, "// uncommitted\n").expect("write uncommitted file");
    world.skip_auto_commit = true;
}

#[given(regex = r#"^the project has an untracked file "([^"]+)"$"#)]
fn untracked_file(world: &mut GivnWorld, path: String) {
    let target = project_base(world).join(&path);
    fs::write(&target, "untracked\n").expect("write untracked file");
    world.skip_auto_commit = true;
}

#[given(regex = r#"^a completed change "([^"]+)" ready to archive$"#)]
fn completed_change_artifacts_only(world: &mut GivnWorld, id: String) {
    let base = project_base(world);
    support::write_change_artifacts_at(&base, &id, false);
    let env = support::env(world);
    common::git_commit_all(env, "fixture: change artifacts");
    world.last_change_id = Some(id);
}

#[given("the full suite fails against the merged permanent specs")]
fn full_suite_fails(world: &mut GivnWorld) {
    set_runner_kind(world, "fail");
}

#[given("the full suite passes against the merged permanent specs")]
fn full_suite_passes(world: &mut GivnWorld) {
    set_runner_kind(world, "pass");
}

#[given(regex = r"^the full suite passes but its runner result is (.+)$")]
fn full_suite_result_kind(world: &mut GivnWorld, description: String) {
    let kind = match description.as_str() {
        "missing" => "missing",
        "malformed" => "malformed",
        "declaring failed scenarios" => "failed",
        "declaring skipped scenarios" => "skipped",
        "whose counts do not reconcile" => "unreconciled",
        "declaring zero scenarios" => "zero",
        "declaring the wrong scope" => "wrong-scope",
        other => panic!("unknown runner result description '{other}'"),
    };
    set_runner_kind(world, kind);
}

#[given("the runner records every invocation with its scope and features path")]
fn runner_records_invocations(world: &mut GivnWorld) {
    set_runner_kind(world, "record");
}

#[given("a commit hook rejects the archive candidate commit")]
fn rejecting_commit_hook(world: &mut GivnWorld) {
    let hooks = support::root(world).join(".git/hooks");
    fs::create_dir_all(&hooks).expect("create hooks dir");
    let hook = hooks.join("commit-msg");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("write rejecting hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("make hook executable");
    world.skip_auto_commit = true;
}

#[given(regex = r#"^an unproven archive candidate "([^"]+)" whose full suite failed$"#)]
fn unproven_candidate_failed_suite(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^an unproven archive candidate "([^"]+)" in a (Git|Fossil) project whose full suite failed$"#
)]
fn backend_unproven_candidate_failed_suite(world: &mut GivnWorld, id: String, backend: String) {
    support::ready_change_on(world, support::backend_kind(&backend), &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_change_id = Some(id);
}

#[then("the verification receipt names the repaired revision")]
fn receipt_names_repaired_revision(world: &mut GivnWorld) {
    receipt_names_tested_commit(world);
}

#[given(regex = r#"^an unproven archive candidate "([^"]+)" whose coverage gate failed$"#)]
fn unproven_candidate_failed_coverage(world: &mut GivnWorld, id: String) {
    support::ensure_env(world);
    let env = support::env(world);
    support::enable_coverage(env);
    support::write_coverage_scripts(env, true, false);
    support::write_result_runner(env, "pass");
    support::configure_commands(env);
    support::write_change_artifacts(env, &id, false);
    common::commit_fixture(env);
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_change_id = Some(id);
}

#[given("the developer commits a repair")]
fn developer_commits_repair(world: &mut GivnWorld) {
    let env = support::env(world);
    common::commit_fixture(env);
}

#[given("the full suite now passes against the merged permanent specs")]
fn full_suite_now_passes(world: &mut GivnWorld) {
    set_runner_kind(world, "pass");
}

#[given("the coverage commands now succeed")]
fn coverage_now_succeeds(world: &mut GivnWorld) {
    support::ensure_env(world);
    let env = support::env(world);
    support::enable_coverage(env);
    support::write_coverage_scripts(env, false, false);
    common::git_commit_all(env, "fix: coverage commands");
}

#[given(regex = r#"^a proven archive "([^"]+)"$"#)]
fn proven_archive(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
    let out = support::run_archive(world, &id);
    assert!(
        out.status.success(),
        "fixture archive failed: {}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a legacy archive "([^"]+)" without a verification receipt$"#)]
fn legacy_archive(world: &mut GivnWorld, id: String) {
    support::ensure_env(world);
    let env = support::env(world);
    let dir = env.path(&format!("givn/archive/{id}"));
    fs::create_dir_all(&dir).expect("create legacy archive dir");
    fs::write(dir.join("proposal.md"), "# legacy\n").expect("write legacy file");
    common::git_commit_all(env, "fixture: legacy archive");
}

#[given(regex = r#"^a change "([^"]+)" that exists both under givn/changes/ and givn/archive/$"#)]
fn both_active_and_archived(world: &mut GivnWorld, id: String) {
    support::ensure_env(world);
    let env = support::env(world);
    support::write_change_artifacts(env, &id, false);
    let dir = env.path(&format!("givn/archive/{id}"));
    fs::create_dir_all(&dir).expect("create archive dir");
    fs::write(dir.join("proposal.md"), "# archived\n").expect("write archived file");
    common::git_commit_all(env, "fixture: corrupt state");
}

#[given("the coverage command writes an undeclared source file")]
fn coverage_writes_undeclared(world: &mut GivnWorld) {
    support::ensure_env(world);
    let env = support::env(world);
    support::enable_coverage(env);
    support::write_coverage_scripts(env, false, true);
    common::git_commit_all(env, "fixture: coverage writes undeclared file");
}

// ─── Then: assertions ──────────────────────────────────────────────────────

#[then(regex = r#"^stderr names "([^"]+)"$"#)]
fn stderr_names(world: &mut GivnWorld, needle: String) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains(&needle),
        "stderr missing {needle:?}: {stderr}"
    );
}

#[then("stderr mentions Git")]
fn stderr_mentions_git(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.to_lowercase().contains("git"),
        "stderr missing Git: {stderr}"
    );
}

#[then("stderr says the project root must be the repository root")]
fn stderr_says_repo_root(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("must be the repository root"),
        "stderr missing repository-root message: {stderr}"
    );
}

#[then("stderr says the archive is already proven")]
fn stderr_says_proven(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("already proven"),
        "stderr missing already-proven message: {stderr}"
    );
}

#[then("stderr says the archive is already archived")]
fn stderr_says_archived(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("already archived"),
        "stderr missing already-archived message: {stderr}"
    );
}

#[then("stderr says the project state is corrupt")]
fn stderr_says_corrupt(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("corrupt"),
        "stderr missing corrupt-state message: {stderr}"
    );
}

#[then("stderr names the candidate commit rejection")]
fn stderr_names_candidate_rejection(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("candidate commit") && stderr.contains("rejected"),
        "stderr missing candidate rejection: {stderr}"
    );
}

#[then("stderr says the tested tree changed")]
fn stderr_says_tested_tree_changed(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("the tested tree changed"),
        "stderr missing tested-tree message: {stderr}"
    );
}

#[then("stderr mentions the runner result")]
fn stderr_mentions_runner_result(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("runner result"),
        "stderr missing runner-result message: {stderr}"
    );
}

#[then("stderr mentions the e2e scope")]
fn stderr_mentions_e2e_scope(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("e2e verification") || stderr.contains("e2e scope"),
        "stderr missing e2e scope message: {stderr}"
    );
}

#[then("the change directory still exists under givn/changes/")]
fn change_dir_still_exists(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let dir = project_base(world).join(format!("givn/changes/{id}"));
    assert!(dir.exists(), "change directory missing: {}", dir.display());
}

#[then(regex = r#"^no archive candidate exists for "([^"]+)"$"#)]
fn no_archive_candidate(world: &mut GivnWorld, id: String) {
    let dir = project_base(world).join(format!("givn/archive/{id}"));
    assert!(
        !dir.exists(),
        "archive candidate unexpectedly exists: {}",
        dir.display()
    );
}

#[then("the change directory has been moved to givn/archive/")]
fn change_moved_to_archive(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let base = project_base(world);
    assert!(
        !base.join(format!("givn/changes/{id}")).exists(),
        "active change directory still exists"
    );
    assert!(
        base.join(format!("givn/archive/{id}")).is_dir(),
        "archive directory missing"
    );
}

#[then("the permanent specs contain the merged delta scenarios")]
fn permanent_specs_contain_merged(world: &mut GivnWorld) {
    let text = read_permanent_specs(world);
    assert!(
        text.contains("Demo scenario"),
        "merged scenario missing from permanent specs"
    );
}

#[then("the merged permanent specs preserve the delta scenarios")]
fn permanent_specs_preserve_delta(world: &mut GivnWorld) {
    let text = read_permanent_specs(world);
    assert!(
        text.contains("Demo scenario") || text.contains("Scenario: e2e test"),
        "merged scenario missing from permanent specs"
    );
}

#[then("the archive candidate is recorded as unproven")]
fn candidate_unproven(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    assert_eq!(support::receipt_status(world, &id), "pending");
}

#[then("the archive candidate is proven")]
fn candidate_proven(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[then(regex = r"^the archive at givn/archive/([^/]+)/ is proven$")]
fn archive_at_proven(world: &mut GivnWorld, id: String) {
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[then(regex = r"^the archive at givn/archive/([^/]+)/ is unproven$")]
fn archive_at_unproven(world: &mut GivnWorld, id: String) {
    assert_eq!(support::receipt_status(world, &id), "pending");
}

#[then("the candidate state is committed")]
fn candidate_state_committed(world: &mut GivnWorld) {
    let paths = support::last_commit_paths(world);
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    assert!(
        paths
            .iter()
            .any(|p| p.starts_with(&format!("givn/archive/{id}/"))),
        "candidate commit does not contain the archive directory: {paths:?}"
    );
    assert!(
        paths.iter().any(|p| p.starts_with("givn/specs/")),
        "candidate commit does not contain the merged specs: {paths:?}"
    );
}

#[then("the merged state is staged but not committed")]
fn merged_state_staged(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let staged = support::staged_paths(world);
    assert!(
        staged
            .iter()
            .any(|p| p.starts_with(&format!("givn/archive/{id}/"))),
        "archive candidate not staged: {staged:?}"
    );
}

#[then(
    regex = r#"^the runner was invoked once for the regular scope with features path "([^"]+)"$"#
)]
fn invoked_once_regular(world: &mut GivnWorld, features: String) {
    assert_invocations(world, "regular", &features, 1);
}

#[then(regex = r#"^the runner was invoked once for the e2e scope with features path "([^"]+)"$"#)]
fn invoked_once_e2e(world: &mut GivnWorld, features: String) {
    assert_invocations(world, "e2e", &features, 1);
}

#[then("the verification receipt names the tested commit")]
fn receipt_names_tested_commit(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    let tested = receipt["tested_commit"].as_str().unwrap_or_default();
    let parent = support::parent_revision(world);
    assert_eq!(tested, parent);
}

#[then("the verification receipt names the repaired commit")]
fn receipt_names_repaired_commit(world: &mut GivnWorld) {
    receipt_names_tested_commit(world);
}

#[then("the verification receipt records the regular scope scenario counts")]
fn receipt_records_regular_counts(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    let regular = &receipt["scopes"]["regular"];
    assert_eq!(regular["status"], "passed");
    assert!(regular["total"].as_u64().unwrap_or(0) > 0);
    assert_eq!(regular["failed"], 0);
}

#[then("the verification receipt records the end-to-end scope scenario counts")]
fn receipt_records_e2e_counts(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    let e2e = &receipt["scopes"]["e2e"];
    assert_eq!(e2e["status"], "passed");
    assert!(e2e["total"].as_u64().unwrap_or(0) > 0);
}

#[then("the verification receipt records the end-to-end scope as not applicable")]
fn receipt_records_e2e_not_applicable(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["scopes"]["e2e"]["status"], "not_applicable");
}

#[then("the verification receipt records the coverage result")]
fn receipt_records_coverage(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["coverage"]["status"], "passed");
}

#[then("the git history has a finalization commit after the tested commit")]
fn finalization_commit_after_tested(world: &mut GivnWorld) {
    let env = support::env(world);
    let head_subject = common::git(env, &["log", "-1", "--format=%s"]);
    let subject = String::from_utf8_lossy(&head_subject.stdout)
        .trim()
        .to_string();
    assert!(
        subject.contains("prove archive"),
        "HEAD is not the finalization commit: {subject}"
    );
    let parent = common::git(env, &["rev-parse", "HEAD~1"]);
    let parent = String::from_utf8_lossy(&parent.stdout).trim().to_string();
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    assert_eq!(
        receipt["tested_commit"].as_str().unwrap_or_default(),
        parent
    );
}

#[then("each merged scenario appears exactly once in the permanent specs")]
fn merged_scenario_once(world: &mut GivnWorld) {
    let text = read_permanent_specs(world);
    let count = text.matches("Scenario: Demo scenario").count();
    assert_eq!(count, 1, "merged scenario count is {count}");
}

#[then(regex = r#"^no archive backup exists for "([^"]+)"$"#)]
fn no_archive_backup(world: &mut GivnWorld, id: String) {
    let backup = support::root(world).join(format!("givn/.archive-backup-{id}"));
    assert!(
        !backup.exists(),
        "backup directory exists: {}",
        backup.display()
    );
}

#[then(regex = r#"^stdout lists "([^"]+)" as an unproven archive$"#)]
fn stdout_lists_unproven(world: &mut GivnWorld, id: String) {
    let stdout = crate::steps::stdout(world);
    let listed = stdout
        .lines()
        .any(|line| line.contains("Unproven archives") && line.contains(&id));
    assert!(
        listed,
        "stdout does not list unproven archive {id}: {stdout}"
    );
}

#[then(regex = r"^stdout routes the developer to `givn archive --change ([^`]+)`$")]
fn stdout_routes_resume(world: &mut GivnWorld, id: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains(&format!("givn archive --change {id}")),
        "stdout missing resume route for {id}: {stdout}"
    );
}

#[then(regex = r"^stdout routes the developer to resume it with `givn archive --change ([^`]+)`$")]
fn stdout_routes_resume_instructions(world: &mut GivnWorld, id: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains(&format!("givn archive --change {id}")),
        "stdout missing resume route for {id}: {stdout}"
    );
}

// ─── helpers ───────────────────────────────────────────────────────────────

fn set_runner_kind(world: &mut GivnWorld, kind: &str) {
    support::ensure_env(world);
    let env = support::env(world);
    support::write_result_runner(env, kind);
    common::commit_fixture(env);
}

fn read_permanent_specs(world: &GivnWorld) -> String {
    let specs = support::root(world).join("givn/specs");
    let mut text = String::new();
    let mut stack = vec![specs];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "feature")
                && let Ok(content) = fs::read_to_string(&path)
            {
                text.push_str(&content);
            }
        }
    }
    text
}

fn assert_invocations(world: &GivnWorld, scope: &str, features: &str, expected: usize) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let log = support::archive_dir(world, &id).join("results/invocations.log");
    let content = fs::read_to_string(&log)
        .unwrap_or_else(|e| panic!("read invocation log {}: {e}", log.display()));
    let count = content
        .lines()
        .filter(|line| *line == format!("{scope} {features}"))
        .count();
    assert_eq!(
        count, expected,
        "scope {scope} invocation count was {count}: {content}"
    );
}
