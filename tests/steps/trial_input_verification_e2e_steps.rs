//! Step definitions for the `trial-input-verification (E2E)` capability.
//!
//! The clean report is the `@e2e` scenario's real-interface assertion: the
//! compiled binary's stdout and exit code, never an internal call. The
//! regular no-drift variant asserts the same two lines, so they are bound
//! here once rather than duplicated across the two step files.

use cucumber::then;

use crate::steps::{GivnWorld, output, stderr, stdout};

/// The `@e2e` scenario whose run this file captures.
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

/// Capture the run as the scenario's terminal evidence. Only the `@e2e`
/// scenario writes it; the regular no-drift variant shares both assertion
/// phrases and would otherwise add an unreviewed directory.
fn write_transcript(world: &GivnWorld) {
    if world.scenario_title.as_deref() != Some(SCENARIO_TITLE) {
        return;
    }
    let out = output(world);
    let dir = crate::common::terminal_evidence::transcript_dir(SCENARIO_TITLE);
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
