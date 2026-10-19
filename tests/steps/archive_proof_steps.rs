//! Step definitions for the archive-proof capability (regular scenarios).

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use cucumber::{given, then, when};

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
    world.snapshot_on_run = true;
    world.skip_auto_commit = true;
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
    world.snapshot_on_run = true;
    world.skip_auto_commit = true;
}

#[given("a givn project nested inside a larger Git repository")]
fn nested_project(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    common::git_init(&env);
    let nested = env.path("nested");
    fs::create_dir_all(&nested).expect("create nested project dir");
    let out = common::run_givn_with_envs_in(&env, &nested, &["init", "--agents", "opencode"], &[]);
    assert!(
        out.status.success(),
        "givn init in nested project failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.cwd_override = Some(nested);
    world.env = Some(env);
    world.snapshot_on_run = true;
    world.skip_auto_commit = true;
}

#[given(regex = r#"^the project has an EXTRA file "([^"]+)"$"#)]
fn extra_file(world: &mut GivnWorld, path: String) {
    let target = support::project_base(world).join(&path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&target, "extra\n").expect("write EXTRA file");
    world.skip_auto_commit = true;
}

#[given(regex = r#"^the project has an edited file "([^"]+)"$"#)]
fn edited_file(world: &mut GivnWorld, path: String) {
    let target = support::project_base(world).join(&path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&target, "// original\n").expect("write tracked file");
    common::commit_fixture(support::env(world));
    fs::write(&target, "// edited\n").expect("edit tracked file");
    world.skip_auto_commit = true;
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

#[when(
    regex = r"^the user runs `givn archive --change ([^`]+)` without the `fossil` executable on PATH$"
)]
fn archive_without_fossil(world: &mut GivnWorld, id: String) {
    let env = world.env.as_ref().expect("project root");
    if world.snapshot_on_run {
        world.scm_fixture.snapshot = Some(
            crate::steps::scm_backend_selection_steps::snapshot_tree(&env.root()),
        );
        world.snapshot_on_run = false;
    }
    let empty_bin = env.path("no-fossil-bin");
    fs::create_dir_all(&empty_bin).expect("create empty PATH directory");
    let path = empty_bin.to_string_lossy().into_owned();
    let out = common::run_givn_with_envs(env, &["archive", "--change", &id], &[("PATH", &path)]);
    world.last_change_id = Some(id);
    world.last_output = Some(out);
}

#[then("stderr names the missing `fossil` executable")]
fn stderr_names_missing_fossil(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("`fossil`") && stderr.contains("not found"),
        "stderr does not name the missing `fossil` executable: {stderr}"
    );
}

#[when("the archive location is unblocked")]
fn archive_location_unblocked(world: &mut GivnWorld) {
    let archive = support::project_base(world).join("givn/archive");
    if archive.is_file() {
        fs::remove_file(&archive).expect("remove blocking file");
    }
    fs::create_dir_all(&archive).expect("create archive dir");
    let env = support::env(world);
    if matches!(env.backend(), common::BackendKind::Git) {
        let add = common::git(env, &["add", "-A", "--", "givn/archive"]);
        assert!(
            add.status.success(),
            "git add unblocked path failed: {}",
            String::from_utf8_lossy(&add.stderr)
        );
        let commit = common::git(env, &["commit", "-q", "-m", "fixture: unblock archive"]);
        assert!(
            commit.status.success(),
            "git commit unblock failed: {}",
            String::from_utf8_lossy(&commit.stderr)
        );
    }
}

#[when("the commit hook accepts the finalization revision")]
fn accepting_finalization_hook(world: &mut GivnWorld) {
    let hook = support::root(world).join(".git/hooks/commit-msg");
    fs::write(&hook, "#!/bin/sh\nexit 0\n").expect("write accepting hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("make hook executable");
}

#[then("the pending receipt is unchanged")]
fn pending_receipt_unchanged(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let expected = world
        .archive_receipt_snapshot
        .clone()
        .expect("the fixture captured a pending receipt");
    let actual = fs::read_to_string(support::archive_dir(world, &id).join("verification.json"))
        .expect("read pending receipt");
    assert_eq!(actual, expected, "the pending receipt was rewritten");
}

#[then("stderr says the project state is corrupt and both directories exist")]
fn stderr_says_both_directories(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("project state is corrupt") && stderr.contains("both exist"),
        "stderr missing the both-directory corruption message: {stderr}"
    );
}

#[then("stderr says the pending archive candidate conflicts with the active change")]
fn stderr_says_pending_conflict(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("pending archive candidate")
            && stderr.contains("conflicts with the active change"),
        "stderr missing the pending-candidate conflict message: {stderr}"
    );
}

#[then("the change directory no longer exists under givn/changes/")]
fn change_dir_no_longer_exists(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let dir = support::project_base(world).join(format!("givn/changes/{id}"));
    assert!(
        !dir.exists(),
        "active change directory still exists: {}",
        dir.display()
    );
}

#[then("stderr names the archive location")]
fn stderr_names_archive_location(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("givn/archive") || stderr.contains(&format!("givn/archive/{id}")),
        "stderr does not name the archive location: {stderr}"
    );
}

#[then(regex = r#"^the archived change contains the evidence file "([^"]+)"$"#)]
fn archived_change_contains_evidence(world: &mut GivnWorld, evidence: String) {
    let id = world.last_change_id.clone().expect("a change id");
    let path = support::archive_dir(world, &id).join(&evidence);
    assert!(
        path.exists(),
        "archived evidence missing: {}",
        path.display()
    );
}

#[then("the tested history records the archive as unproven")]
fn tested_history_unproven(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(
        support::committed_receipt_status(world, &id),
        "pending",
        "the tested revision does not carry a pending receipt"
    );
}

#[then("stderr names the finalization rejection")]
fn stderr_names_finalization_rejection(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("rejected finalization") && stderr.contains("prove archive"),
        "stderr missing the finalization rejection: {stderr}"
    );
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

#[given("a commit hook rejects the archive candidate revision")]
fn rejecting_commit_hook_backend(world: &mut GivnWorld) {
    match support::env(world).backend() {
        common::BackendKind::Git => {
            let hooks = support::root(world).join(".git/hooks");
            fs::create_dir_all(&hooks).expect("create hooks dir");
            let hook = hooks.join("commit-msg");
            fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("write rejecting hook");
            let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&hook, permissions).expect("make hook executable");
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(
                support::env(world),
                &[
                    "hook",
                    "add",
                    "--command",
                    "exit 1",
                    "--type",
                    "before-commit",
                ],
            );
            assert!(
                out.status.success(),
                "fossil hook add failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
    world.skip_auto_commit = true;
}

#[when("the commit hook accepts the archive candidate revision")]
fn accepting_commit_hook(world: &mut GivnWorld) {
    match support::env(world).backend() {
        common::BackendKind::Git => {
            let hook = support::root(world).join(".git/hooks/commit-msg");
            fs::remove_file(&hook).expect("remove rejecting hook");
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(support::env(world), &["hook", "delete", "all"]);
            assert!(
                out.status.success(),
                "fossil hook delete failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
    world.skip_auto_commit = false;
}

#[then("stderr names the candidate revision rejection")]
fn stderr_names_candidate_revision_rejection(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("candidate revision") && stderr.contains("rejected"),
        "stderr missing candidate rejection: {stderr}"
    );
}

#[then("the tested revision contains the merged permanent specs")]
fn tested_revision_contains_merged_specs(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    let tested = receipt["tested_revision"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        !tested.is_empty(),
        "the receipt does not name the tested revision: {receipt}"
    );
    let merged = "givn/specs/demo-cap/demo-cap.feature";
    assert!(
        support::revision_contains_at(world, &tested, merged),
        "the tested revision {tested} does not contain {merged}"
    );
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

#[then("the verification receipt names the tested revision")]
fn receipt_names_tested_revision(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    let tested = receipt["tested_revision"].as_str().unwrap_or_default();
    assert!(
        !tested.is_empty(),
        "the receipt does not name the tested revision: {receipt}"
    );
    assert_eq!(tested, support::parent_revision(world));
}

#[then("the history has a finalization revision after the tested revision")]
fn history_has_finalization_revision(world: &mut GivnWorld) {
    let current = support::current_revision(world);
    let parent = support::parent_revision(world);
    assert_ne!(
        current, parent,
        "the current revision is not a finalization revision after the tested revision"
    );
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let receipt = support::receipt(world, &id);
    assert_eq!(
        receipt["tested_revision"].as_str().unwrap_or_default(),
        parent
    );
}

#[then("the finalization revision contains only the resolved evidence paths")]
fn finalization_revision_only_evidence(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let archive_prefix = format!("givn/archive/{id}/");
    let paths = support::revision_paths(world);
    assert!(
        !paths.is_empty(),
        "the finalization revision changed no paths"
    );
    for path in &paths {
        assert!(
            path.starts_with(&archive_prefix) || path == "README.md",
            "the finalization revision contains a path outside the resolved evidence: {path}"
        );
    }
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
    let receipt = support::write_pending_receipt(&dir, &id, true, &[], None, &[], &[]);
    common::commit_fixture(env);
    world.archive_receipt_snapshot = Some(receipt);
    world.last_change_id = Some(id);
}

#[given("the pending archive candidate conflicts with the active change")]
fn pending_candidate_conflicts(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let active = support::project_base(world).join(format!("givn/changes/{id}/proposal.md"));
    let archived = support::archive_dir(world, &id).join("proposal.md");
    let active = fs::read_to_string(&active).expect("read active proposal");
    let archived = fs::read_to_string(&archived).expect("read archived proposal");
    assert_ne!(active, archived, "fixture candidates must differ");
}

#[given("the archive candidate has no verification receipt")]
fn archive_candidate_without_receipt(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let path = support::archive_dir(world, &id).join("verification.json");
    if path.exists() {
        fs::remove_file(&path).expect("remove pending receipt");
    }
}

#[given(
    regex = r#"^an interrupted archive move for change "([^"]+)" with an identical pending candidate$"#
)]
fn interrupted_move_identical(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let archive = support::archive_dir(world, &id);
    let active = support::project_base(world).join(format!("givn/changes/{id}"));
    support::copy_candidate_content(&archive, &active);
    let env = support::env(world);
    if matches!(env.backend(), common::BackendKind::Git) {
        let add = common::git(env, &["add", "-A", "--", &format!("givn/changes/{id}")]);
        assert!(
            add.status.success(),
            "git add duplicate failed: {}",
            String::from_utf8_lossy(&add.stderr)
        );
    }
    // The follow-up scenario step rewrites the passing runner; commit only
    // that path so the duplicate stays staged.
    support::write_result_runner(env, "pass");
    support::commit_path(env, "emit-result.sh", "fixture: passing runner");
    world.skip_auto_commit = true;
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a pending archive candidate "([^"]+)" whose delta merge did not complete$"#)]
fn pending_candidate_merge_incomplete(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
    let env = support::env(world);
    let active = support::project_base(world).join(format!("givn/changes/{id}"));
    let archive = support::archive_dir(world, &id);
    support::copy_dir_all(&active, &archive);
    support::write_pending_receipt(
        &archive,
        &id,
        false,
        &[],
        None,
        &["demo-cap/demo-cap.feature"],
        &[(
            "demo-cap/demo-cap.feature",
            "givn/specs/demo-cap/demo-cap.feature",
        )],
    );
    fs::remove_dir_all(&active).expect("remove active change");
    common::commit_fixture(env);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a pending archive candidate "([^"]+)" whose delta file was applied but not recorded$"#
)]
fn pending_candidate_in_flight(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let env = support::env(world);
    let receipt_path = support::archive_dir(world, &id).join("verification.json");
    let mut receipt: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&receipt_path).expect("read receipt"))
            .expect("parse receipt");
    receipt["merge_completed"] = serde_json::json!(false);
    receipt["merged_deltas"] = serde_json::json!([]);
    receipt["merge_in_progress"] = serde_json::json!("demo-cap/demo-cap.feature");
    fs::write(
        &receipt_path,
        format!("{}\n", serde_json::to_string_pretty(&receipt).unwrap()),
    )
    .expect("write modified receipt");
    support::write_result_runner(env, "pass");
    support::commit_path(env, "emit-result.sh", "fixture: passing runner");
    world.skip_auto_commit = true;
    world.last_change_id = Some(id);
}

#[given("the archive location is blocked")]
fn archive_location_blocked(world: &mut GivnWorld) {
    let base = support::project_base(world);
    let archive = base.join("givn/archive");
    if archive.is_dir() {
        fs::remove_dir_all(&archive).expect("remove archive dir");
    }
    fs::write(&archive, "blocked\n").expect("write blocking file");
    common::commit_fixture(support::env(world));
    world.skip_auto_commit = true;
}

#[given(regex = r#"^the change has an uncommitted evidence file "([^"]+)"$"#)]
fn uncommitted_evidence(world: &mut GivnWorld, evidence: String) {
    let id = world.last_change_id.clone().expect("a change id");
    let target = support::project_base(world)
        .join(format!("givn/changes/{id}"))
        .join(&evidence);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create evidence dir");
    }
    fs::write(&target, "evidence\n").expect("write evidence file");
    world.skip_auto_commit = true;
}

#[given("a commit hook rejects the finalization revision")]
fn rejecting_finalization_hook(world: &mut GivnWorld) {
    let hooks = support::root(world).join(".git/hooks");
    fs::create_dir_all(&hooks).expect("create hooks dir");
    let hook = hooks.join("commit-msg");
    fs::write(
        &hook,
        "#!/bin/sh\nif grep -q 'prove archive' \"$1\"; then\n  echo 'rejected finalization: prove archive' >&2\n  exit 1\nfi\nexit 0\n",
    )
    .expect("write finalization hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("make hook executable");
    world.skip_auto_commit = true;
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
        stderr.contains("must be the worktree top level") && stderr.contains("nested inside"),
        "stderr missing the boundary message: {stderr}"
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
        stderr.contains("candidate revision") && stderr.contains("rejected"),
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

#[then("the merged state is present in the checkout but not committed")]
fn merged_state_in_checkout_uncommitted(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let dirty = support::dirty_entries(world);
    assert!(
        dirty.iter().any(|path| path.starts_with("givn/specs/")),
        "the merged state is not present in the checkout: {dirty:?}"
    );
    assert!(
        !support::revision_contains(world, &format!("givn/archive/{id}/verification.json")),
        "the merged state was committed despite the rejected hook"
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
    let tested = receipt["tested_revision"].as_str().unwrap_or_default();
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
        receipt["tested_revision"].as_str().unwrap_or_default(),
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
    if world.skip_auto_commit {
        support::commit_path(env, "emit-result.sh", "fixture: runner");
    } else {
        common::commit_fixture(env);
    }
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
