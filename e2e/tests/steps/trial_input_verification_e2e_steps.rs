//! Step definitions for the `trial-input-verification (E2E)` capability.
//!
//! The clean report is the `@e2e` scenario's real-interface assertion: the
//! compiled binary's stdout and exit code, never an internal call. The
//! regular no-drift variant asserts the same two lines, so they are bound
//! here once rather than duplicated across the two step files.

use std::path::PathBuf;

use crate::harness as common;
use crate::steps::clean_room_preparation_steps::project_env;
use crate::steps::ise_clean_room_support::{self, REPLACED_SOURCE_PATH};
use crate::steps::{GivnWorld, output, stderr, stdout};
use cucumber::{given, then, when};

const SCENARIO_TITLE: &str =
    "Verifier re-checks a prepared trial and receives the frozen-input report";

#[then(regex = r#"^verification reports the retained and e2e identities clean$"#)]
fn verification_reports_the_identities_clean(world: &mut GivnWorld) {
    let text = stdout(world);
    for line in ["Retained: clean", "E2E: clean"] {
        assert!(
            text.lines().any(|printed| printed == line),
            "stdout {text:?} does not print the line {line:?}"
        );
    }
    let trial = world
        .ise_clean_room
        .trial_path()
        .canonicalize()
        .expect("resolve the prepared trial path");
    let expected = format!("Trial: {}", trial.display());
    assert!(
        text.lines().any(|printed| printed == expected),
        "stdout {text:?} does not print the line {expected:?}"
    );
}

#[then(regex = r#"^verification exits zero$"#)]
fn verification_exits_zero(world: &mut GivnWorld) {
    let code = output(world).status.code();
    assert_eq!(code, Some(0), "stderr={}", stderr(world));
    write_transcript(world);
}

fn write_transcript(world: &GivnWorld) {
    if world.scenario_title.as_deref() != Some(SCENARIO_TITLE) {
        return;
    }
    let out = output(world);
    let dir = crate::harness::terminal_evidence::transcript_dir(SCENARIO_TITLE);
    std::fs::create_dir_all(&dir).expect("create the transcript directory");
    let argv = format!(
        "givn ise verify --trial {} --at {}",
        world.ise_clean_room.receipt_path().display(),
        world.ise_clean_room.trial_path().display()
    );
    let text = format!(
        "Scenario: {SCENARIO_TITLE}\n\
         argv: {argv}\n\
         cwd (fixture project root): {}\n\
         binary: the compiled `givn` the scenario spawns (CARGO_BIN_EXE_givn)\n\
         exit code: {}\n\
         \n--- stdout (raw) ---\n{}\n--- stderr (raw) ---\n{}\n",
        world
            .env
            .as_ref()
            .map(|env| env.root().display().to_string())
            .unwrap_or_else(|| "<unknown>".to_string()),
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    std::fs::write(dir.join("transcript.txt"), text).expect("write the transcript");
}

// --- Batch 9: compare-report ----------------------------------------------

const COMPARE_SCENARIO_TITLE: &str = "Verifier compares the spec copy with the manifest";

fn compare_copy(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.trial_path().to_path_buf()
}

fn compare_manifest(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.receipt_path().to_path_buf()
}

#[given(regex = r#"^a prepared spec copy with a manifest$"#)]
fn prepared_spec_copy(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::verify_trial_project());
    ise_clean_room_support::commit_verify_trial_revision(project_env(world));
    world.ise_clean_room.ensure_targets();
    let copy = compare_copy(world).to_string_lossy().into_owned();
    let manifest = compare_manifest(world).to_string_lossy().into_owned();
    let args = [
        "ise",
        "rebuild-component",
        REPLACED_SOURCE_PATH,
        "--at",
        "HEAD",
        "--into",
        copy.as_str(),
        "--manifest",
        manifest.as_str(),
    ];
    let out = common::run_givn(project_env(world), &args);
    assert!(
        out.status.success(),
        "preparing the spec copy failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_output = Some(out);
}

#[when(regex = r#"^the verifier compares the spec copy with the manifest$"#)]
fn verifier_compares(world: &mut GivnWorld) {
    let copy = compare_copy(world).to_string_lossy().into_owned();
    let manifest = compare_manifest(world).to_string_lossy().into_owned();
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

#[then(regex = r#"^comparison prints "([^"]+)" and "([^"]+)"$"#)]
fn comparison_prints_two_lines(world: &mut GivnWorld, first: String, second: String) {
    let text = stdout(world);
    for expected in [first, second] {
        assert!(
            text.lines().any(|printed| printed == expected),
            "stdout {text:?} does not print the line {expected:?}"
        );
    }
}

#[then(regex = r#"^comparison exits zero$"#)]
fn comparison_exits_zero(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(0),
        "stderr={}",
        stderr(world)
    );
    write_compare_transcript(world);
}

#[then(regex = r#"^comparison prints no line beginning "- "$"#)]
fn comparison_prints_no_removed_line(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        !text.lines().any(|printed| printed.starts_with("- ")),
        "stdout {text:?} prints a difference line"
    );
}

fn write_compare_transcript(world: &GivnWorld) {
    if world.scenario_title.as_deref() != Some(COMPARE_SCENARIO_TITLE) {
        return;
    }
    let out = output(world);
    let dir = crate::harness::terminal_evidence::transcript_dir(COMPARE_SCENARIO_TITLE);
    std::fs::create_dir_all(&dir).expect("create the transcript directory");
    let argv = format!(
        "givn ise compare --manifest {} --copy {}",
        compare_manifest(world).display(),
        compare_copy(world).display()
    );
    let text = format!(
        "Scenario: {COMPARE_SCENARIO_TITLE}\n\
         argv: {argv}\n\
         cwd (fixture project root): {}\n\
         binary: the compiled `givn` the scenario spawns (GIVN_BIN)\n\
         exit code: {}\n\
         \n--- stdout (raw) ---\n{}\n--- stderr (raw) ---\n{}\n",
        world
            .env
            .as_ref()
            .map(|env| env.root().display().to_string())
            .unwrap_or_else(|| "<unknown>".to_string()),
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    std::fs::write(dir.join("transcript.txt"), text).expect("write the transcript");
}
