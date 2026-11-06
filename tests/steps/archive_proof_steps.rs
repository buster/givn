//! Step definitions for the archive-proof capability (regular scenarios).

use std::fs;
use std::io::Write;
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
    if let Some(id) = world.last_change_id.clone() {
        support::refresh_change_receipt(world, &id);
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
    if let Some(id) = world.last_change_id.clone() {
        support::refresh_change_receipt(world, &id);
    }
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
    if let Some(id) = world.last_change_id.clone() {
        support::refresh_change_receipt(world, &id);
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

#[when(regex = r"^the user runs `givn archive --change ([^`]+)` without committing the fixture$")]
fn archive_without_commit(world: &mut GivnWorld, id: String) {
    let out = support::run_archive(world, &id);
    world.last_change_id = Some(id);
    world.last_output = Some(out);
}

#[then("the runner was never invoked with a change features path")]
fn runner_never_used_change_features(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let log = support::archive_dir(world, &id).join("results/invocations.log");
    let content = fs::read_to_string(&log)
        .unwrap_or_else(|e| panic!("read invocation log {}: {e}", log.display()));
    assert!(
        !content.contains("givn/changes/"),
        "the change suite was invoked during archive: {content}"
    );
}

// ─── proof reuse across documentation-only revisions ──────────────────────

#[given(
    "a clean committed change with a green change verification receipt recorded at an earlier revision"
)]
fn proof_reuse_fixture(world: &mut GivnWorld) {
    super::workflow_enforcement_steps::write_archive_ready_review_fixture(
        world,
        "review-proof-demo",
    );
}

#[given("the developer commits only the verification receipt after that revision")]
fn commit_only_receipt(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    support::commit_path(
        support::env(world),
        &format!("givn/changes/{id}/verification/tasks-verification.json"),
        "fixture: commit the change verification receipt",
    );
}

#[given("the verify command records every invocation with its scope and features path")]
fn verify_command_records_invocations(world: &mut GivnWorld) {
    world.verification_invocations_before =
        Some(super::workflow_enforcement_steps::change_suite_invocation_count(world));
}

#[then("the archive is proven")]
fn archive_is_proven(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(
        support::receipt_status(world, &id),
        "proven",
        "the archive receipt is not proven"
    );
}

#[then("the change suite was not invoked again")]
fn change_suite_was_not_invoked(world: &mut GivnWorld) {
    let before = world
        .verification_invocations_before
        .expect("the fixture must record the invocation count");
    let after = super::workflow_enforcement_steps::change_suite_invocation_count(world);
    assert_eq!(
        after, before,
        "the change suite ran again: {before} -> {after} invocations"
    );
}

#[given(regex = r"^the (.+) is changed and committed after that revision$")]
fn change_tested_content(world: &mut GivnWorld, boundary: String) {
    let id = world.last_change_id.clone().expect("a change id");
    let root = support::root(world);
    match boundary.as_str() {
        "delta feature" => {
            let path = root.join(format!(
                "givn/changes/{id}/specs/{id}/alpha-capability.feature"
            ));
            let mut content = std::fs::read_to_string(&path).unwrap();
            content.push_str("\n  @givn.added\n  Scenario: Fixture changed alpha-capability\n    Given a changed thing\n    Then it still works\n");
            std::fs::write(path, content).unwrap();
        }
        "verify command" => {
            let path = root.join("givn/commands.yaml");
            let content = std::fs::read_to_string(&path).unwrap();
            std::fs::write(
                path,
                content.replace(
                    "command: \"./emit-result.sh regular\"",
                    "command: \"./emit-result.sh regular-changed\"",
                ),
            )
            .unwrap();
        }
        "source code" => {
            std::fs::write(root.join("src/lib.rs"), "pub fn fixture() -> u32 { 2 }\n").unwrap();
        }
        other => panic!("unknown changed boundary '{other}'"),
    }
    common::commit_fixture(support::env(world));
    world.changed_boundary = Some(boundary);
}

#[then("stderr names the changed boundary")]
fn stderr_names_changed_boundary(world: &mut GivnWorld) {
    let boundary = world
        .changed_boundary
        .clone()
        .expect("the fixture must record the changed boundary");
    let needle = match boundary.as_str() {
        "delta feature" => "delta features changed",
        "verify command" => "verify command changed",
        "source code" => "tested content changed",
        other => panic!("unknown changed boundary '{other}'"),
    };
    let text = crate::steps::stderr(world);
    assert!(text.contains(needle), "stderr missing {needle:?}: {text}");
}

fn install_archive_coverage(env: &common::TestEnv, id: &str) {
    let log = format!("givn/archive/{id}/results/coverage-invocations.log");
    let report = "<?xml version=\"1.0\"?>\n<coverage line-rate=\"0.5\" branch-rate=\"0\" lines-covered=\"2\" lines-valid=\"4\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\">\n  <packages />\n</coverage>\n";
    let measure = format!(
        "#!/bin/sh\nmkdir -p coverage\necho run >> \"{log}\"\ncat > \"$GIVN_COVERAGE_NON_E2E_PATH\" <<'XML'\n{report}XML\ncat > \"$GIVN_COVERAGE_E2E_PATH\" <<'XML'\n{report}XML\n"
    );
    let merge = "#!/bin/sh\nmkdir -p \"$(dirname \"$GIVN_COVERAGE_OUTPUT_PATH\")\"\ncp \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_OUTPUT_PATH\"\n";
    for (name, content) in [
        ("emit-coverage.sh", measure),
        ("merge-coverage.sh", merge.to_string()),
    ] {
        let path = env.path(name);
        fs::write(&path, content).expect("write coverage script");
        let mut permissions = fs::metadata(&path).expect("script metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("make script executable");
    }
}

fn rejecting_finalization(env: &common::TestEnv) {
    let hooks = env.path(".git/hooks");
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
}

fn accepting_finalization(env: &common::TestEnv) {
    let hook = env.path(".git/hooks/commit-msg");
    fs::write(&hook, "#!/bin/sh\nexit 0\n").expect("write accepting hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook, permissions).expect("make hook executable");
}

fn archive_invocations(world: &GivnWorld, id: &str) -> String {
    fs::read_to_string(
        support::root(world).join(format!("givn/archive/{id}/results/invocations.log")),
    )
    .unwrap_or_default()
}

fn coverage_invocations(world: &GivnWorld, id: &str) -> usize {
    fs::read_to_string(support::root(world).join(format!(
        "givn/archive/{id}/results/coverage-invocations.log"
    )))
    .map(|content| content.lines().count())
    .unwrap_or(0)
}

fn failed_finalization_candidate(
    world: &mut GivnWorld,
    id: &str,
    with_e2e: bool,
    with_coverage: bool,
) {
    support::ready_change(world, id, with_e2e, "record");
    let env = support::env(world);
    if with_coverage {
        support::enable_coverage(env);
        install_archive_coverage(env, id);
    }
    common::commit_fixture(env);
    support::refresh_change_receipt(world, id);
    rejecting_finalization(env);
    let out = support::run_archive(world, id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_output = Some(out);
    world.last_change_id = Some(id.to_string());
    assert_eq!(support::receipt_status(world, id), "pending");
}

#[given(
    "an archive candidate has passed every applicable verification scope and coverage measurement"
)]
fn candidate_passed_scopes_and_coverage(world: &mut GivnWorld) {
    failed_finalization_candidate(world, "proof-demo", false, true);
}

#[given("a commit hook rejects the evidence-only finalization publication")]
fn finalization_publication_rejected(world: &mut GivnWorld) {
    let env = support::env(world);
    rejecting_finalization(env);
    world.verification_log_before = Some(archive_invocations(world, "proof-demo"));
}

#[then("the command exits non-zero and names the finalization rejection")]
fn command_names_finalization_rejection(world: &mut GivnWorld) {
    stderr_names_finalization_rejection(world);
}

#[then("the archive becomes proven")]
fn archive_becomes_proven(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[given(
    regex = r#"^an unproven archive candidate "([^"]+)" with a merged corpus and no verification scopes$"#
)]
fn merged_unproven_candidate(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
    let env = support::env(world);
    let change = env.path(&format!("givn/changes/{id}"));
    let permanent = env.path("givn/specs/demo-cap/demo-cap.feature");
    let delta = change.join("specs/demo-cap/demo-cap.feature");
    let feature = gherkin::Feature::parse_path(&delta, gherkin::GherkinEnv::default())
        .expect("parse fixture delta feature");
    givn::spec::merge_plan::write_feature(&permanent, &feature).expect("merge fixture delta");
    let archive = env.path(&format!("givn/archive/{id}"));
    std::fs::create_dir_all(archive.parent().expect("archive parent")).expect("create archive dir");
    std::fs::rename(&change, &archive).expect("move fixture change to archive");
    support::write_pending_receipt(
        &archive,
        &id,
        true,
        &["demo-cap/demo-cap.feature"],
        None,
        &["demo-cap/demo-cap.feature"],
        &[(
            "demo-cap/demo-cap.feature",
            "givn/specs/demo-cap/demo-cap.feature",
        )],
    );
    common::commit_fixture(env);
    world.last_change_id = Some(id);
    world.snapshot_on_run = true;
    world.skip_auto_commit = true;
}

#[given(
    "the permanent interaction inventory does not declare an E2E scenario that the merged corpus contains"
)]
fn permanent_inventory_misses_merged_scenario(world: &mut GivnWorld) {
    let env = support::env(world);
    let permanent = env.path("givn/specs/demo-cap/demo-cap.feature");
    let mut content = fs::read_to_string(&permanent).expect("read merged permanent feature");
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(
        "\n  @e2e\n  Scenario: Consumer cancels an order\n    Given a thing\n    Then it works\n",
    );
    fs::write(&permanent, content).expect("append unlisted permanent scenario");
    common::commit_fixture(env);
}

#[then("no verification scope ran for the candidate")]
fn no_verification_scope_ran(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let results = support::archive_dir(world, &id).join("results");
    assert!(
        !results.exists(),
        "a verification scope ran and wrote {}",
        results.display()
    );
}

#[then("the candidate remains unproven and repairable in place")]
fn candidate_unproven_repairable(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
    assert!(support::archive_dir(world, &id).is_dir());
    assert!(
        !support::root(world)
            .join(format!("givn/changes/{id}"))
            .exists()
    );
}

#[when("the hook accepts publication and the developer retries with unchanged relevant inputs")]
fn accept_hook_and_retry(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let env = support::env(world);
    accepting_finalization(env);
    common::commit_fixture(env);
    support::refresh_change_receipt(world, &id);
    let out = support::run_archive(world, &id);
    world.last_output = Some(out);
}

#[then("the archive becomes proven using the recorded scope results and coverage")]
fn proven_from_recorded_evidence(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert_eq!(receipt["scopes"]["regular"]["status"], "passed");
    assert_eq!(receipt["coverage"]["status"], "passed");
}

#[then("no verification suite or coverage measurement runs again")]
fn no_resuite_or_coverage(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    assert_eq!(before, archive_invocations(world, &id));
    assert_eq!(coverage_invocations(world, &id), 1);
}

fn e2e_toggle_candidate(world: &mut GivnWorld, backend: common::BackendKind, id: &str) {
    support::ready_change_on(world, backend, id, true, "pass");
    let env = support::env(world);
    let toggle = format!(
        "#!/bin/sh\nif [ -f \"givn/archive/{id}/evidence/e2e-fixed\" ]; then\n  exec ./emit-result.sh e2e\nfi\nexit 1\n"
    );
    let path = env.path("emit-e2e.sh");
    fs::write(&path, toggle).expect("write e2e toggle");
    let mut permissions = fs::metadata(&path).expect("toggle metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("make toggle executable");
    let commands = env.path("givn/commands.yaml");
    let content = fs::read_to_string(&commands).expect("read commands");
    fs::write(
        &commands,
        content.replace(
            "e2e_command: \"./emit-result.sh e2e\"",
            "e2e_command: \"./emit-e2e.sh\"",
        ),
    )
    .expect("point e2e at the toggle");
    common::commit_fixture(env);
    support::refresh_change_receipt(world, id);
    let out = support::run_archive(world, id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_output = Some(out);
    world.last_change_id = Some(id.to_string());
}

#[given(regex = r#"^an unproven archive candidate "([^"]+)" in a (Git|Fossil) project$"#)]
fn unproven_candidate_split_scopes(world: &mut GivnWorld, id: String, backend: String) {
    e2e_toggle_candidate(world, support::backend_kind(&backend), &id);
}

#[given("its regular scope passed but its end-to-end scope failed")]
fn regular_passed_e2e_failed(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["scopes"]["regular"]["status"], "passed");
    assert!(receipt["scopes"].get("e2e").is_none());
    assert!(receipt["scope_evidence"].get("e2e").is_none());
}

#[given("the developer commits a repair changing only inputs relevant to the end-to-end scope")]
fn commits_e2e_repair(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let env = support::env(world);
    let marker = support::root(world).join(format!("givn/archive/{id}/evidence/e2e-fixed"));
    fs::create_dir_all(marker.parent().unwrap()).expect("create evidence dir");
    fs::write(&marker, "fixed\n").expect("write repair marker");
    common::commit_fixture(env);
    support::refresh_change_receipt(world, &id);
}

#[then("the archive becomes proven after fresh passing end-to-end verification")]
fn proven_after_e2e(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert_eq!(receipt["scopes"]["e2e"]["status"], "passed");
}

#[then("the unchanged regular scope result is reused without another execution")]
fn regular_scope_reused(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    let evidence = &receipt["scope_evidence"]["regular"]["inputs"]["revision"];
    assert_ne!(
        evidence.as_str().unwrap_or_default(),
        receipt["tested_revision"].as_str().unwrap_or_default()
    );
    let log = archive_invocations(world, &id);
    assert_eq!(
        log.lines()
            .filter(|line| line.starts_with("regular "))
            .count(),
        1,
        "the regular scope ran again: {log}"
    );
}

#[then("the receipt names the repaired revision and the reused scope evidence")]
fn receipt_names_repair_and_reuse(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(
        receipt["tested_revision"].as_str().unwrap_or_default(),
        support::parent_revision(world)
    );
    assert!(receipt["scope_evidence"]["regular"].is_object());
    assert!(receipt["scope_evidence"]["e2e"].is_object());
}

#[given("an unproven candidate has a passed regular scope and a failed end-to-end scope")]
fn candidate_split_scopes(world: &mut GivnWorld) {
    e2e_toggle_candidate(world, common::BackendKind::Git, "proof-demo");
}

#[given("only archive evidence has changed since those executions")]
fn only_archive_evidence_changed(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    world.verification_log_before = Some(archive_invocations(world, &id));
    let evidence =
        support::root(world).join(format!("givn/archive/{id}/evidence/recovery-note.txt"));
    fs::create_dir_all(evidence.parent().unwrap()).expect("create evidence dir");
    fs::write(evidence, "evidence\n").expect("write evidence");
}

#[given("the end-to-end runner now produces a fresh valid passing result")]
fn e2e_runner_now_passes(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let marker = support::root(world).join(format!("givn/archive/{id}/evidence/e2e-fixed"));
    fs::create_dir_all(marker.parent().unwrap()).expect("create evidence dir");
    fs::write(marker, "fixed\n").expect("write marker");
}

#[when(regex = r"^the developer retries `givn archive --change ([^`]+)`$")]
fn developer_retries_archive(world: &mut GivnWorld, id: String) {
    let env = support::env(world);
    common::commit_fixture(env);
    support::refresh_change_receipt(world, &id);
    let out = support::run_archive(world, &id);
    world.last_output = Some(out);
    world.last_change_id = Some(id);
}

#[then("the regular scope is not executed again")]
fn regular_not_executed(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    let after = archive_invocations(world, &id);
    assert_eq!(
        before
            .lines()
            .filter(|line| line.starts_with("regular "))
            .count(),
        after
            .lines()
            .filter(|line| line.starts_with("regular "))
            .count(),
        "the regular scope ran again: {before} -> {after}"
    );
}

#[then("only the failed end-to-end scope executes")]
fn only_e2e_executes(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    let after = archive_invocations(world, &id);
    assert_eq!(
        before
            .lines()
            .filter(|line| line.starts_with("e2e "))
            .count()
            + 1,
        after
            .lines()
            .filter(|line| line.starts_with("e2e "))
            .count(),
        "the e2e scope did not run exactly once more: {before} -> {after}"
    );
}

fn failed_scope_candidate(world: &mut GivnWorld, id: &str, with_e2e: bool, kind: &str) {
    support::ready_change(world, id, with_e2e, kind);
    let out = support::run_archive(world, id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_output = Some(out);
    world.last_change_id = Some(id.to_string());
}

#[given("an unproven candidate has passed regular and end-to-end scope results")]
fn candidate_passed_both_scopes(world: &mut GivnWorld) {
    failed_finalization_candidate(world, "proof-demo", true, false);
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["scopes"]["regular"]["status"], "passed");
    assert_eq!(receipt["scopes"]["e2e"]["status"], "passed");
    accepting_finalization(support::env(world));
    world.verification_log_before = Some(archive_invocations(world, &id));
}

#[given("a committed source repair changes relevant inputs shared by both scopes")]
fn committed_shared_source_repair(world: &mut GivnWorld) {
    let env = support::env(world);
    fs::create_dir_all(env.path("src")).expect("create src dir");
    fs::write(env.path("src/lib.rs"), "pub fn repaired() -> u32 { 2 }\n").expect("write source");
    common::commit_fixture(env);
}

#[then("neither stale scope result is reused as proof")]
fn no_stale_scope_reuse(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    let after = archive_invocations(world, &id);
    for scope in ["regular", "e2e"] {
        assert_eq!(
            before
                .lines()
                .filter(|line| line.starts_with(&format!("{scope} ")))
                .count()
                + 1,
            after
                .lines()
                .filter(|line| line.starts_with(&format!("{scope} ")))
                .count(),
            "the {scope} scope did not run again: {before} -> {after}"
        );
    }
}

#[then("each affected scope executes once against the repaired candidate")]
fn each_scope_once(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    let tested = receipt["tested_revision"].as_str().unwrap_or_default();
    for scope in ["regular", "e2e"] {
        assert_eq!(
            receipt["scope_evidence"][scope]["inputs"]["revision"], tested,
            "{scope} evidence is not bound to the repaired revision"
        );
    }
}

#[then("proof requires fresh valid passing results for the repaired inputs")]
fn proof_fresh_repaired(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    for scope in ["regular", "e2e"] {
        assert_eq!(receipt["scopes"][scope]["status"], "passed");
    }
}

#[given("an unproven candidate has completed merging and passed verification")]
fn candidate_merged_and_verified(world: &mut GivnWorld) {
    failed_finalization_candidate(world, "proof-demo", false, false);
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["merge_completed"], true);
    assert_eq!(receipt["scopes"]["regular"]["status"], "passed");
    accepting_finalization(support::env(world));
    world.verification_log_before = Some(archive_invocations(world, &id));
}

#[given("finalization publication failed")]
fn finalization_failed(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("rejected finalization"),
        "stderr lacks finalization rejection: {stderr}"
    );
    accepting_finalization(support::env(world));
}

#[given("its relevant inputs and accepted review evidence are unchanged")]
fn inputs_and_review_unchanged(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
}

#[then("revision and relevant-content checks confirm reusable evidence")]
fn cheap_checks_confirm_reuse(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert_eq!(
        receipt["tested_revision"].as_str().unwrap_or_default(),
        support::parent_revision(world)
    );
}

#[then("the completed merge and review are not repeated")]
fn merge_and_review_not_repeated(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        !stdout.contains("merged →"),
        "the merge ran again: {stdout}"
    );
    assert!(
        !stdout.contains("Running static review gates"),
        "the review gates ran again: {stdout}"
    );
}

#[then("no full verification run is required solely because this is a retry")]
fn no_full_run_on_retry(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    assert_eq!(before, archive_invocations(world, &id));
}

#[given("an unproven candidate has valid passed verification results")]
fn candidate_valid_results(world: &mut GivnWorld) {
    failed_finalization_candidate(world, "proof-demo", false, false);
    let id = world.last_change_id.clone().expect("change id");
    accepting_finalization(support::env(world));
    world.verification_log_before = Some(archive_invocations(world, &id));
}

#[given("only files within the exempt archive directory have changed")]
fn only_archive_files_changed(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    for name in [
        "evidence/unlisted-output.bin",
        "results/extra.log",
        "custom.txt",
    ] {
        let path = support::archive_dir(world, &id).join(name);
        fs::create_dir_all(path.parent().unwrap()).expect("create archive evidence dir");
        fs::write(&path, "exempt\n").expect("write exempt file");
    }
}

#[then("those changes do not invalidate the recorded scope results")]
fn archive_changes_do_not_invalidate(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    assert_eq!(before, archive_invocations(world, &id));
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[then("proof reuse does not require an evidence-output allowlist")]
fn no_evidence_allowlist(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    assert!(support::archive_dir(world, &id).join("custom.txt").exists());
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[then(
    "the command reports that executable changes inside exempt paths may escape cheap input checks"
)]
fn reports_exempt_path_limit(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("executable changes inside exempt paths may escape cheap input checks"),
        "stdout lacks the exempt-path limit: {stdout}"
    );
}

fn merged_pending_candidate(world: &mut GivnWorld, id: &str) {
    failed_scope_candidate(world, id, false, "fail");
    let receipt_path = support::archive_dir(world, id).join("verification.json");
    let mut receipt: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&receipt_path).expect("read receipt"))
            .expect("parse receipt");
    receipt["merge_completed"] = serde_json::json!(true);
    receipt["scopes"] = serde_json::json!({});
    receipt["scope_evidence"] = serde_json::json!({});
    fs::write(
        &receipt_path,
        format!("{}\n", serde_json::to_string_pretty(&receipt).unwrap()),
    )
    .expect("write receipt");
    common::commit_fixture(support::env(world));
}

#[given("a merged archive candidate has an old passing regular result file")]
fn merged_candidate_with_old_result(world: &mut GivnWorld) {
    merged_pending_candidate(world, "proof-demo");
    let result = support::archive_dir(world, "proof-demo").join("results/regular.json");
    fs::create_dir_all(result.parent().unwrap()).expect("create results dir");
    fs::write(
        &result,
        "{\"scope\":\"regular\",\"total\":2,\"passed\":2,\"failed\":0,\"skipped\":0}\n",
    )
    .expect("write old result");
}

#[given(
    "regular verification is required and its runner exits successfully without writing output"
)]
fn runner_silent(world: &mut GivnWorld) {
    let env = support::env(world);
    let script = "#!/bin/sh\nif [ -e \"$GIVN_RESULT_FILE\" ]; then echo present > \"givn/archive/proof-demo/results/old-result-seen.log\"; fi\nexit 0\n";
    let path = env.path("silent-runner.sh");
    fs::write(&path, script).expect("write silent runner");
    let mut permissions = fs::metadata(&path).expect("runner metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("make runner executable");
    let commands = env.path("givn/commands.yaml");
    let content = fs::read_to_string(&commands).expect("read commands");
    let updated = common::replace_top_level_key(
        &content,
        "verify",
        "verify:\n  command: \"./silent-runner.sh\"\n  e2e_command: \"\"\n",
    );
    fs::write(commands, updated).expect("write commands");
    common::commit_fixture(env);
}

#[then("the old result was removed before execution")]
fn old_result_removed_before_run(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let seen = support::root(world).join(format!("givn/archive/{id}/results/old-result-seen.log"));
    assert!(!seen.exists(), "the runner saw the old result file");
    assert!(
        !support::archive_dir(world, &id)
            .join("results/regular.json")
            .exists(),
        "the old result file survived"
    );
}

#[then("the candidate remains unproven because fresh output is missing")]
fn unproven_missing_output(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("runner result"),
        "stderr lacks the missing runner result: {stderr}"
    );
}

#[given("a merged archive candidate is ready for regular verification")]
fn merged_candidate_ready(world: &mut GivnWorld) {
    merged_pending_candidate(world, "proof-demo");
    let env = support::env(world);
    fs::create_dir_all(env.path("src")).expect("create src dir");
    fs::write(env.path("src/lib.rs"), "pub fn initial() -> u32 { 1 }\n").expect("write source");
    common::commit_fixture(env);
    world.verification_log_before = Some(archive_invocations(world, "proof-demo"));
}

#[given(
    "the runner writes fresh valid passing output but changes a relevant source file during execution"
)]
fn runner_mutates_source(world: &mut GivnWorld) {
    let env = support::env(world);
    let script = "#!/bin/sh\nprintf '{\"scope\":\"regular\",\"total\":2,\"passed\":2,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\nprintf 'pub fn changed() -> u32 { 2 }\\n' > src/lib.rs\nexit 0\n";
    let path = env.path("mutating-runner.sh");
    fs::write(&path, script).expect("write mutating runner");
    let mut permissions = fs::metadata(&path).expect("runner metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("chmod");
    let commands = env.path("givn/commands.yaml");
    let content = fs::read_to_string(&commands).expect("read commands");
    let updated = common::replace_top_level_key(
        &content,
        "verify",
        "verify:\n  command: \"./mutating-runner.sh\"\n  e2e_command: \"\"\n",
    );
    fs::write(commands, updated).expect("write commands");
    common::commit_fixture(env);
}

#[then("the candidate remains unproven because relevant inputs changed during execution")]
fn unproven_inputs_changed(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("tested inputs changed during execution"),
        "stderr lacks the input-change refusal: {stderr}"
    );
}

#[then("no passing scope evidence is rebound to the modified inputs")]
fn no_scope_rebound(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert!(receipt["scope_evidence"].get("regular").is_none());
    assert!(receipt["scopes"].get("regular").is_none());
}

fn no_behavior_change_fixture(world: &mut GivnWorld, id: &str, delta: bool, skip_specs: bool) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    support::write_result_runner(env, "pass");
    support::configure_commands(env);
    support::disable_coverage(env);
    let out = common::run_givn(env, &["new", id]);
    assert!(out.status.success());
    let change_dir = env.path(&format!("givn/changes/{id}"));
    fs::write(
        change_dir.join("proposal.md"),
        "# Proposal: maintenance\n\nBehavior change: none\n\nNo capability changes.\n",
    )
    .expect("write proposal");
    for file in ["design.md", "design-review.md", "tasks.md", "review.md"] {
        let mut f = fs::File::create(change_dir.join(file)).expect("create artifact");
        match file {
            "design-review.md" => writeln!(f, "DESIGN-REVIEW: PASS").ok(),
            "review.md" => writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok(),
            "tasks.md" => writeln!(f, "- [x] COMMIT: a1b2c3d4 -- maintenance").ok(),
            _ => writeln!(f, "# Design").ok(),
        };
    }
    fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").expect("arc42");
    if skip_specs {
        fs::write(change_dir.join(".givn-skip"), "specs\n").expect("write skip");
    }
    if delta {
        let specs = change_dir.join("specs/maintenance");
        fs::create_dir_all(&specs).expect("create specs");
        fs::write(
            specs.join("maintenance.feature"),
            "@givn.delta @maintenance\nFeature: Maintenance\n\n  @givn.added\n  Scenario: Maintenance runs\n    Then visible\n",
        )
        .expect("write delta");
    }
    common::commit_fixture(env);
    world.last_change_id = None;
    world.skip_auto_commit = true;
}

#[given(
    "a completed change declares \"Behavior change: none\", explicitly skips specs and has no delta feature files"
)]
fn no_behavior_change_valid(world: &mut GivnWorld) {
    no_behavior_change_fixture(world, "maintenance-demo", false, true);
}

#[given("applicable identity, integrity and review checks pass")]
fn applicable_checks_pass(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .unwrap_or_else(|| "maintenance-demo".to_string());
    let change = support::root(world).join(format!("givn/changes/{id}"));
    for file in ["design-review.md", "review.md", "arc42.md"] {
        assert!(change.join(file).exists(), "missing {file}");
    }
}

fn archive_invocations_for(world: &GivnWorld, id: &str) -> String {
    archive_invocations(world, id)
}

#[then("archive does not require invented delta scenarios or a changed-scenario receipt")]
fn no_invented_scenarios(world: &mut GivnWorld) {
    let id = "maintenance-demo";
    let receipt = support::receipt(world, id);
    assert_eq!(receipt["status"], "proven");
    let archived = support::archive_dir(world, id);
    assert!(
        !archived
            .join("verification/tasks-verification.json")
            .exists()
    );
}

#[then("applicable merged-candidate verification still runs before the archive is proven")]
fn merged_verification_runs(world: &mut GivnWorld) {
    let log = archive_invocations_for(world, "maintenance-demo");
    assert!(
        log.lines().any(|line| line.starts_with("regular ")),
        "the regular merged-candidate scope did not run: {log}"
    );
    assert_eq!(
        support::receipt(world, "maintenance-demo")["scopes"]["regular"]["status"],
        "passed"
    );
}

#[given("a completed change declares \"Behavior change: none\"")]
fn declared_no_behavior_change(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().expect("fixture env");
    let change = env.path("givn/changes/maintenance-demo");
    fs::create_dir_all(&change).expect("create change");
    fs::write(
        change.join("proposal.md"),
        "# Proposal: maintenance\n\nBehavior change: none\n",
    )
    .expect("write proposal");
    world.last_change_id = None;
}

#[given(regex = r"^the change has (executable delta features|no delta feature files) and (.+)$")]
fn change_delta_state_and_specs(
    world: &mut GivnWorld,
    delta_state: String,
    specs_disposition: String,
) {
    let env = world.env.as_ref().expect("fixture env");
    let change = env.path("givn/changes/maintenance-demo");
    if delta_state == "executable delta features" {
        let specs = change.join("specs/maintenance");
        fs::create_dir_all(&specs).expect("create specs");
        fs::write(
            specs.join("maintenance.feature"),
            "@givn.delta @maintenance\nFeature: Maintenance\n\n  @givn.added\n  Scenario: Maintenance runs\n    Then visible\n",
        )
        .expect("write delta");
    }
    if specs_disposition == "an explicitly skipped specs artifact" {
        fs::write(change.join(".givn-skip"), "specs\n").expect("write skip");
    }
    world.last_change_id = Some("maintenance-demo".to_string());
    world.skip_auto_commit = true;
    world.verification_log_before = Some(
        serde_json::to_string(&mutation_snapshot(world, "maintenance-demo"))
            .expect("snapshot JSON"),
    );
    world.last_change_id = None;
}

fn mutation_snapshot(world: &GivnWorld, id: &str) -> Vec<(String, Vec<u8>)> {
    let root = support::root(world);
    let mut out = Vec::new();
    for path in [
        root.join("givn/specs"),
        root.join(format!("givn/changes/{id}")),
        root.join(format!("givn/archive/{id}")),
    ] {
        collect_tree(&path, &path, &mut out);
    }
    out.sort();
    out
}

fn collect_tree(root: &std::path::Path, path: &std::path::Path, out: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_tree(root, &path, out);
        } else {
            let relative = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            out.push((relative, fs::read(&path).unwrap_or_default()));
        }
    }
}

#[then(
    regex = r#"^the command exits non-zero and names (executable deltas contradicting no behavior change|the missing explicit specs skip)$"#
)]
fn exits_nonzero_names(world: &mut GivnWorld, reason: String) {
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    let stderr = crate::steps::stderr(world);
    let needle = match reason.as_str() {
        "executable deltas contradicting no behavior change" => {
            "executable deltas contradict no behavior change"
        }
        "the missing explicit specs skip" => "explicit specs skip",
        other => other,
    };
    assert!(stderr.contains(needle), "stderr lacks {needle:?}: {stderr}");
}

#[then("no merged-candidate verification runner executes")]
fn no_merged_runner(world: &mut GivnWorld) {
    let root = support::root(world);
    assert!(
        !root
            .join("givn/archive/maintenance-demo/results/invocations.log")
            .exists()
    );
}

#[given(
    "a committed givn project with an unproven candidate whose scopes passed but finalization publication failed"
)]
fn committed_unproven_finalization_failure(world: &mut GivnWorld) {
    failed_finalization_candidate(world, "proof-repair", false, false);
    let id = world.last_change_id.clone().expect("change id");
    world.verification_log_before = Some(archive_invocations(world, &id));
}

#[given("the publication blocker is repaired without changing relevant verification inputs")]
fn publication_blocker_repaired(world: &mut GivnWorld) {
    let env = support::env(world);
    accepting_finalization(env);
}

#[then("the archive carries a proven receipt reusing its passed scope evidence")]
fn proven_receipt_reuses_evidence(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert!(receipt["scope_evidence"]["regular"].is_object());
    let evidence_revision = receipt["scope_evidence"]["regular"]["inputs"]["revision"]
        .as_str()
        .unwrap_or_default();
    assert!(!evidence_revision.is_empty());
    capture_proof_transcript(world);
}

#[then("no verification suite runs again")]
fn no_suite_runs_again(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let before = world
        .verification_log_before
        .as_deref()
        .expect("log snapshot");
    assert_eq!(before, archive_invocations(world, &id));
}

fn capture_proof_transcript(world: &GivnWorld) {
    let output = world.last_output.as_ref().expect("archive ran");
    let title = "Consumer repairs a blocked archive and the rerun proves it";
    let slug = givn::integrity_hook::scenario_slug(title);
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/prepare-0-9-10-evidence")
        .join(&slug);
    fs::create_dir_all(&dir).expect("create transcript dir");
    fs::write(
        dir.join("transcript.txt"),
        format!(
            "Scenario: {title}\nargv: givn archive --change proof-repair\nexit code: {}\n--- stdout ---\n{}\n--- stderr ---\n{}\n",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
    )
    .expect("write transcript");
}
