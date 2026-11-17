//! Step definitions for the archive-proof capability (regular scenarios).

use super::archive_proof_support as support;
use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn project_base(world: &GivnWorld) -> PathBuf {
    support::project_base(world)
}

#[given(regex = r#"^a givn project with a completed change "([^"]+)" ready to archive$"#)]
fn ready_change(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
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

#[then(regex = r"^stdout routes the developer to resume it with `givn archive --change ([^`]+)`$")]
fn stdout_routes_resume_instructions(world: &mut GivnWorld, id: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains(&format!("givn archive --change {id}")),
        "stdout missing resume route for {id}: {stdout}"
    );
}

#[given(
    "a clean committed change with a green change verification receipt recorded at an earlier revision"
)]
fn proof_reuse_fixture(world: &mut GivnWorld) {
    super::workflow_enforcement_steps::write_archive_ready_review_fixture(
        world,
        "review-proof-demo",
    );
    // The documentation-only reuse rule needs the content-bound receipt the
    // product writes: the fixture is archive-ready, so its own check produces
    // one whose tested-content digest survives later documentation edits.
    super::workflow_enforcement_steps::write_product_receipt(world, "review-proof-demo");
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
    let slug = crate::harness::timing::scenario_slug(title);
    let dir = crate::harness::binary::product_root()
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
