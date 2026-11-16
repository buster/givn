//! Step definitions for the `clean-room-preparation` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. Unimplemented phrases use the
//! not-implemented stub pattern from `design.md` "Strict Mode": RED is a
//! failed step that names the phrase, never an unbound or skipped step.

use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::{output, stderr, stdout};
use cucumber::then;

pub(crate) fn project_env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

pub(crate) fn assert_success(world: &GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(0), "stderr={}", stderr(world));
}

pub(crate) fn assert_report_line(world: &GivnWorld, line: &str) {
    let text = stdout(world);
    assert!(
        text.lines().any(|printed| printed == line),
        "stdout {text:?} does not print the line {line:?}"
    );
}

pub(crate) fn record_document(world: &GivnWorld) -> serde_yaml_ng::Value {
    let path = receipt_path(world);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the record {}: {error}", path.display()));
    serde_yaml_ng::from_str(&text)
        .unwrap_or_else(|error| panic!("parse the record {}: {error}", path.display()))
}

pub(crate) fn record_list(record: &serde_yaml_ng::Value, key: &str) -> Vec<String> {
    record
        .get(key)
        .and_then(|value| value.as_sequence())
        .unwrap_or_else(|| panic!("the record has no '{key}' list"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("'{key}' holds a non-string entry"))
                .to_string()
        })
        .collect()
}

pub(crate) fn record_number(record: &serde_yaml_ng::Value, key: &str) -> u64 {
    record
        .get("counts")
        .and_then(|counts| counts.get(key))
        .and_then(serde_yaml_ng::Value::as_u64)
        .unwrap_or_else(|| panic!("the record has no counts.{key}"))
}

pub(crate) fn trial_path(world: &GivnWorld) -> std::path::PathBuf {
    world.ise_clean_room.trial_path().to_path_buf()
}

pub(crate) fn assert_trial_holds(world: &GivnWorld, paths: &[&str]) {
    let trial = trial_path(world);
    for path in paths {
        let target = trial.join(path);
        assert!(
            target.is_file(),
            "the trial at {} does not hold {path}",
            trial.display()
        );
    }
}

pub(crate) fn assert_trial_omits(world: &GivnWorld, paths: &[&str]) {
    let trial = trial_path(world);
    for path in paths {
        let target = trial.join(path);
        assert!(
            !target.exists(),
            "the trial at {} holds {path}",
            trial.display()
        );
    }
}

pub(crate) fn prepare_trial(world: &mut GivnWorld, revision: Option<&str>, form: &[&str]) {
    world.ise_clean_room.ensure_targets();
    let trial = world
        .ise_clean_room
        .trial_path()
        .to_string_lossy()
        .into_owned();
    let receipt = world
        .ise_clean_room
        .receipt_path()
        .to_string_lossy()
        .into_owned();
    if let Some(revision) = revision {
        world.ise_clean_room.requested_revision = Some(revision.to_string());
    }
    let args = {
        let mut args = vec!["ise", "replace"];
        if let Some(revision) = revision {
            args.push("--at");
            args.push(revision);
        }
        args.extend(form.iter().copied());
        args.push("--into");
        args.push(&trial);
        args.push("--receipt");
        args.push(&receipt);
        args
    };
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn_with_envs(env, &args, &[]);
    world.last_output = Some(out);
}

pub(crate) fn receipt_path(world: &GivnWorld) -> std::path::PathBuf {
    world.ise_clean_room.receipt_path().to_path_buf()
}

#[then(regex = r#"^preparation succeeds$"#)]
fn preparation_succeeds(world: &mut GivnWorld) {
    assert_success(world);
}
