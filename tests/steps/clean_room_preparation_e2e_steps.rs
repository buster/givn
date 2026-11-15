//! Step definitions for the `clean-room-preparation (E2E)` capability.
//!
//! Every step drives the built binary as a real subprocess in a temp Git
//! project. Fixtures live in `tests/steps/ise_clean_room_support.rs`; the
//! shared report and record helpers live in the regular capability module.

use std::path::PathBuf;

use cucumber::{given, then, when};

use crate::steps::clean_room_preparation_steps::{
    assert_report_line, assert_success, assert_trial_holds, assert_trial_omits, prepare_trial,
    project_env, receipt_path, record_document, record_list, record_number, trial_path,
};
use crate::steps::{GivnWorld, ise_clean_room_support, output, stdout};

/// The `@e2e` scenario whose run this file captures as terminal evidence.
const SCENARIO_TITLE: &str = "Developer prepares a clean-room trial from a recorded revision";

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "givn/specs/\*\*", declares "tests/e2e", and replaces "src/\*\*"$"#
)]
fn git_project_with_the_e2e_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::e2e_trial_project());
}

#[given(
    regex = r#"^a committed revision with "givn/specs/demo\.feature", "tests/e2e/Cargo\.toml", "tests/e2e/steps/demo\.rs", "src/keys/parser\.rs", "src/keys/mod\.rs", "src/other\.rs", and "notes\.md"$"#
)]
fn commit_the_e2e_trial_revision(world: &mut GivnWorld) {
    let revision = ise_clean_room_support::commit_e2e_trial_revision(project_env(world));
    world.ise_clean_room.first_revision = Some(revision);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parser\.rs" and a receipt$"#
)]
fn prepare_with_the_parser_path_and_a_receipt(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "src/keys/parser.rs"]);
}

#[then(
    regex = r#"^the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit$"#
)]
fn report_prints_the_git_backend_and_revision(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Backend: git");
    let text = stdout(world);
    let revision = text
        .lines()
        .find_map(|line| line.strip_prefix("Revision: "))
        .unwrap_or_else(|| panic!("stdout {text:?} has no Revision line"));
    assert_eq!(
        revision.len(),
        40,
        "Revision {revision:?} is not 40 hex characters"
    );
    assert!(
        revision
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
        "Revision {revision:?} is not lowercase hex"
    );
    let head = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the committed revision");
    assert_eq!(revision, head, "Revision is not the resolved commit");
}

#[then(
    regex = r#"^the report prints "Retained: 4", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"$"#
)]
fn report_prints_the_placement_counts(world: &mut GivnWorld) {
    for line in [
        "Retained: 4",
        "Kept source: 2",
        "Replaced: 1",
        "Outside the declaration: 1",
        "Not replaced: 1",
    ] {
        assert_report_line(world, line);
    }
}

#[then(
    regex = r#"^the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis$"#
)]
fn report_abbreviates_the_digests(world: &mut GivnWorld) {
    let text = stdout(world);
    for prefix in ["Retained digest: ", "E2E digest: "] {
        let value = text
            .lines()
            .find_map(|line| line.strip_prefix(prefix))
            .unwrap_or_else(|| panic!("stdout {text:?} has no {prefix:?} line"));
        let hex = value
            .strip_suffix('…')
            .unwrap_or_else(|| panic!("{prefix}{value:?} has no ellipsis"));
        assert_eq!(hex.len(), 12, "{prefix}{value:?} is not 12 hex characters");
        assert!(
            hex.chars()
                .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
            "{prefix}{value:?} is not lowercase hex"
        );
    }
}

#[then(regex = r#"^the report prints the written trial path and the receipt path$"#)]
fn report_prints_the_write_targets(world: &mut GivnWorld) {
    assert_report_line(
        world,
        &format!("Written path: {}", resolved_trial_path(world).display()),
    );
    assert_report_line(
        world,
        &format!("Receipt path: {}", receipt_path(world).display()),
    );
}

#[then(
    regex = r#"^the trial holds the retained and kept source files, including "src/keys/mod\.rs"$"#
)]
fn trial_holds_the_retained_and_kept_files(world: &mut GivnWorld) {
    assert_trial_holds(
        world,
        &[
            ise_clean_room_support::E2E_FEATURE_PATH,
            "tests/e2e/Cargo.toml",
            "tests/e2e/steps/demo.rs",
            "src/keys/mod.rs",
            "src/other.rs",
        ],
    );
}

#[then(regex = r#"^the trial holds the declaration "givn/specification\.yaml"$"#)]
fn trial_holds_the_declaration(world: &mut GivnWorld) {
    assert_trial_holds(world, &[ise_clean_room_support::DECLARATION_PATH]);
}

#[then(regex = r#"^the trial omits "src/keys/parser\.rs" and "notes\.md"$"#)]
fn trial_omits_the_replaced_and_outside_files(world: &mut GivnWorld) {
    assert_trial_omits(world, &["src/keys/parser.rs", "notes.md"]);
}

#[then(
    regex = r#"^the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version$"#
)]
fn receipt_records_the_preparation_evidence(world: &mut GivnWorld) {
    let record = record_document(world);
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the committed revision");
    assert_eq!(
        record
            .get("revision")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(revision.as_str()),
        "the record has the wrong revision"
    );
    assert_eq!(
        record.get("backend").and_then(serde_yaml_ng::Value::as_str),
        Some("git"),
        "the record has the wrong backend"
    );
    assert_eq!(
        record_list(&record, "replaced_paths"),
        vec!["src/keys/parser.rs".to_string()],
        "the record has the wrong replaced paths"
    );
    for (key, expected) in [
        ("retained", 4),
        ("kept_source", 2),
        ("replaced", 1),
        ("outside_declaration", 1),
        ("not_replaced", 1),
    ] {
        assert_eq!(record_number(&record, key), expected, "counts.{key}");
    }
    let digest = record
        .get("replaced_digest")
        .and_then(serde_yaml_ng::Value::as_str)
        .unwrap_or_else(|| panic!("the record has no replaced_digest"));
    assert_eq!(digest.len(), 64, "replaced_digest {digest:?} is not 64 hex");
    assert!(
        digest
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
        "replaced_digest {digest:?} is not lowercase hex"
    );
    assert_eq!(
        record
            .get("written_path")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(resolved_trial_path(world).to_string_lossy().as_ref()),
        "the record has the wrong written path"
    );
    assert_eq!(
        record
            .get("givn_version")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(env!("CARGO_PKG_VERSION")),
        "the record has the wrong Givn version"
    );
    write_transcript(world);
}

/// Capture the run as the scenario's terminal evidence. A delta run writes
/// into the change's `evidence/visual/`; a permanent-corpus run writes into
/// the gitignored `target/terminal-evidence/`, so an archived transcript is
/// never recreated as untracked work.
fn write_transcript(world: &GivnWorld) {
    if world.scenario_title.as_deref() != Some(SCENARIO_TITLE) {
        return;
    }
    let out = output(world);
    let dir = crate::common::terminal_evidence::transcript_dir(SCENARIO_TITLE);
    std::fs::create_dir_all(&dir).expect("create the transcript directory");
    let argv = format!(
        "givn ise replace --at HEAD --path src/keys/parser.rs --into {} --receipt {}",
        trial_path(world).display(),
        receipt_path(world).display()
    );
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .unwrap_or_else(|| "<unrecorded>".to_string());
    let text = format!(
        "Scenario: {SCENARIO_TITLE}\n\
         argv: {argv}\n\
         cwd (fixture project root): {}\n\
         binary: the compiled `givn` the scenario spawns (CARGO_BIN_EXE_givn)\n\
         revision: {revision}\n\
         exit code: {}\n\
         \n--- stdout (raw) ---\n{}\n--- stderr (raw) ---\n{}\n",
        project_env(world).root().display(),
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    std::fs::write(dir.join("transcript.txt"), text).expect("write the transcript");
}

/// The absolute trial path the writer creates: the existing container is
/// canonicalized and the missing `trial` component appended, matching
/// `ise`'s own resolution.
fn resolved_trial_path(world: &GivnWorld) -> PathBuf {
    let trial = trial_path(world);
    let parent = trial.parent().expect("the trial has a parent");
    let name = trial.file_name().expect("the trial has a file name");
    parent
        .canonicalize()
        .unwrap_or_else(|error| panic!("resolve {}: {error}", parent.display()))
        .join(name)
}
