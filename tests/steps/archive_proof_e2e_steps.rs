//! E2E step definitions for the archive-proof capability.

use cucumber::{given, then};

use super::archive_proof_support as support;
use crate::common;
use crate::steps::GivnWorld;

#[given(regex = r#"^a committed givn project with a completed change "([^"]+)" ready to archive$"#)]
fn committed_ready_change(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
}

#[given("the project reports scenario counts for every verification scope")]
fn project_reports_counts(world: &mut GivnWorld) {
    support::ensure_env(world);
    let env = support::env(world);
    support::write_result_runner(&env, "pass");
    common::git_commit_all(&env, "fixture: report scenario counts");
}

#[given(regex = r#"^a committed givn project with an unproven archive candidate "([^"]+)"$"#)]
fn committed_unproven_candidate(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_change_id = Some(id);
}

#[then(regex = r"^the archive at givn/archive/([^/]+)/ carries a proven verification receipt$")]
fn carries_proven_receipt(world: &mut GivnWorld, id: String) {
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert!(
        receipt["tested_commit"].as_str().is_some_and(|s| !s.is_empty()),
        "proven receipt does not name the tested commit: {receipt}"
    );
}
