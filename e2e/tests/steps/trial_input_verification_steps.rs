//! Step definitions for the retired `trial-input-verification` fixture.
//!
//! The delta replaces the scenario that uses these phrases with
//! `Verifier compares the spec copy with the manifest`; until the merge the
//! permanent scenario runs here. Every step drives the built binary, and the
//! fixture prepares the spec copy with `rebuild-component --manifest`.

use crate::harness as common;
use crate::steps::clean_room_preparation_steps::{prepare_spec_copy, project_env};
use crate::steps::ise_clean_room_support::{self, REPLACED_SOURCE_PATH};
use crate::steps::{GivnWorld, output, stderr};
use cucumber::{given, when};
use std::path::PathBuf;

fn spec_copy(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.trial_path().to_path_buf()
}

fn manifest(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.manifest_path().to_path_buf()
}

fn prepare(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::verify_trial_project());
    ise_clean_room_support::commit_verify_trial_revision(project_env(world));
    prepare_prepared(world);
}

fn prepare_prepared(world: &mut GivnWorld) {
    prepare_spec_copy(world, Some("HEAD"), &[REPLACED_SOURCE_PATH]);
    let out = output(world);
    assert!(
        out.status.success(),
        "preparing the fixture spec copy failed: {}",
        stderr(world)
    );
}

fn compare(world: &mut GivnWorld) {
    let copy = spec_copy(world).to_string_lossy().into_owned();
    let manifest = manifest(world).to_string_lossy().into_owned();
    let out = common::run_givn(
        project_env(world),
        &[
            "ise",
            "compare",
            "--manifest",
            manifest.as_str(),
            "--copy",
            copy.as_str(),
        ],
    );
    world.last_output = Some(out);
}

#[given(regex = r#"^a prepared Git trial with a receipt$"#)]
fn prepared_git_trial(world: &mut GivnWorld) {
    prepare(world);
}

#[when(regex = r#"^the verifier re-checks the trial against the record$"#)]
fn verifier_re_checks(world: &mut GivnWorld) {
    compare(world);
}
