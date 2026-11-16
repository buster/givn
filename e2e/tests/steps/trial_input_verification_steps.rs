//! Step definitions for the `trial-input-verification` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. The clean report assertions are
//! bound in the capability's E2E file: the `@e2e` scenario and the regular
//! no-drift variant assert the same two lines, and one phrase has exactly one
//! binding.

use crate::harness as common;
use crate::steps::clean_room_preparation_steps::{prepare_trial, project_env};
use crate::steps::ise_clean_room_support::{self, REPLACED_SOURCE_PATH};
use crate::steps::{GivnWorld, output, stderr};
use cucumber::{given, when};
use std::path::{Path, PathBuf};

fn trial(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.trial_path().to_path_buf()
}

fn record(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.receipt_path().to_path_buf()
}

fn prepare(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::verify_trial_project());
    ise_clean_room_support::commit_verify_trial_revision(project_env(world));
    prepare_prepared(world);
}

fn prepare_prepared(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", REPLACED_SOURCE_PATH]);
    let out = output(world);
    assert!(
        out.status.success(),
        "preparing the fixture trial failed: {}",
        stderr(world)
    );
}

fn re_check(world: &mut GivnWorld, record_path: Option<&Path>) {
    let trial = trial(world).to_string_lossy().into_owned();
    let record_path = record_path
        .map(Path::to_path_buf)
        .unwrap_or_else(|| record(world))
        .to_string_lossy()
        .into_owned();
    let args = [
        "ise",
        "verify",
        "--trial",
        record_path.as_str(),
        "--at",
        trial.as_str(),
    ];
    let out = common::run_givn(project_env(world), &args);
    world.last_output = Some(out);
}

#[given(regex = r#"^a prepared Git trial with a receipt$"#)]
fn prepared_git_trial(world: &mut GivnWorld) {
    prepare(world);
}

#[when(regex = r#"^the verifier re-checks the trial against the record$"#)]
fn verifier_re_checks(world: &mut GivnWorld) {
    re_check(world, None);
}
