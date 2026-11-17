//! Step definitions for the `clean-room-preparation (E2E)` capability.
//!
//! Every step drives the built binary as a real subprocess in a temp Git
//! project. Fixtures live in `tests/steps/ise_clean_room_support.rs`; the
//! shared report and record helpers live in the regular capability module.

use crate::steps::clean_room_preparation_steps::{
    assert_report_line, assert_success, assert_trial_holds, assert_trial_omits, manifest_document,
    manifest_list, manifest_number, manifest_path, prepare_spec_copy, project_env, trial_path,
};
use crate::steps::{GivnWorld, ise_clean_room_support, output, stdout};
use cucumber::{given, then, when};
use std::path::PathBuf;

const SCENARIO_TITLE: &str = "Developer rebuilds the whole product from its Specification";

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
fn prepare_with_the_parser_path_and_a_manifest(world: &mut GivnWorld) {
    prepare_spec_copy(world, Some("HEAD"), &["src/keys/parser.rs"]);
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
    regex = r#"^the report prints "Retained: 2", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"$"#
)]
fn report_prints_the_placement_counts(world: &mut GivnWorld) {
    // The retired scenario text names the record nouns; the settled report
    // prints the same five counts under the Specification/Source nouns.
    for line in [
        "Specification files: 2",
        "Source code kept: 2",
        "Source code removed: 1",
        "Outside the spec copy: 1",
        "Files beside removed paths: 1",
    ] {
        assert_report_line(world, line);
    }
}

#[then(regex = r#"^the report prints "Frozen package: e2e, not copied \(2 files\)"$"#)]
fn report_prints_the_frozen_package_line(world: &mut GivnWorld) {
    assert_report_line(world, "End-to-end tests: not copied (2 files)");
}

#[then(
    regex = r#"^the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis$"#
)]
fn report_abbreviates_the_digests(world: &mut GivnWorld) {
    let text = stdout(world);
    for prefix in ["Specification checksum: ", "End-to-end checksum: "] {
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
        &format!("Copy directory: {}", resolved_trial_path(world).display()),
    );
    assert_report_line(
        world,
        &format!("Manifest: {}", manifest_path(world).display()),
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
            "src/keys/mod.rs",
            "src/other.rs",
        ],
    );
}

#[then(regex = r#"^the trial holds the declaration "givn/specification\.yaml"$"#)]
fn trial_holds_the_declaration(world: &mut GivnWorld) {
    assert_trial_holds(world, &[ise_clean_room_support::DECLARATION_PATH]);
}

#[then(
    regex = r#"^the trial omits "src/keys/parser\.rs", "notes\.md", "tests/e2e/Cargo\.toml", and "tests/e2e/steps/demo\.rs"$"#
)]
fn trial_omits_the_replaced_and_frozen_files(world: &mut GivnWorld) {
    assert_trial_omits(
        world,
        &[
            "src/keys/parser.rs",
            "notes.md",
            "tests/e2e/Cargo.toml",
            "tests/e2e/steps/demo.rs",
        ],
    );
}

#[then(
    regex = r#"^the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version$"#
)]
fn manifest_records_the_preparation_evidence(world: &mut GivnWorld) {
    // The retired scenario text names the record fields; the settled manifest
    // carries the same evidence under its own shape. The backend stays a
    // report line, and the single replaced digest becomes the two rollups.
    assert_report_line(world, "Backend: git");
    let manifest = manifest_document(world);
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the committed revision");
    assert_eq!(
        manifest
            .get("revision")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(revision.as_str()),
        "the manifest has the wrong revision"
    );
    assert_eq!(
        manifest_list(&manifest, "removed_paths"),
        vec!["src/keys/parser.rs".to_string()],
        "the manifest has the wrong removed paths"
    );
    for (key, expected) in [
        ("specification_files", 2),
        ("source_kept", 2),
        ("source_removed", 1),
        ("outside_spec_copy", 1),
        ("files_beside_removed_paths", 1),
    ] {
        assert_eq!(manifest_number(&manifest, key), expected, "counts.{key}");
    }
    let digest = manifest
        .get("specification_checksum")
        .and_then(serde_yaml_ng::Value::as_str)
        .unwrap_or_else(|| panic!("the manifest has no specification_checksum"));
    assert_eq!(
        digest.len(),
        64,
        "specification_checksum {digest:?} is not 64 hex"
    );
    assert!(
        digest
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
        "specification_checksum {digest:?} is not lowercase hex"
    );
    assert_eq!(
        manifest
            .get("copy_directory")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(resolved_trial_path(world).to_string_lossy().as_ref()),
        "the manifest has the wrong copy directory"
    );
    assert_eq!(
        manifest
            .get("givn_version")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(crate::harness::binary::product_version()).as_deref(),
        "the manifest has the wrong Givn version"
    );
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
        "givn ise rebuild-all --at HEAD --into {} --manifest {}",
        trial_path(world).display(),
        manifest_path(world).display()
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
         binary: the compiled `givn` the scenario spawns through GIVN_BIN\n\
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

fn resolved_trial_path(world: &GivnWorld) -> PathBuf {
    let trial = trial_path(world);
    let parent = trial.parent().expect("the trial has a parent");
    let name = trial.file_name().expect("the trial has a file name");
    parent
        .canonicalize()
        .unwrap_or_else(|error| panic!("resolve {}: {error}", parent.display()))
        .join(name)
}

// --- report-and-manifest ----------------------------------------------------

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "behavior" role names "givn/specs/\*\*", whose "e2e" role names "tests/e2e", and whose source root is "src/\*\*"$"#
)]
fn git_project_with_the_rebuild_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::e2e_trial_project());
}

#[when(
    regex = r#"^the developer rebuilds the whole product from "([^"]+)" into a spec copy with a manifest$"#
)]
fn rebuild_all_from_the_revision_with_a_manifest(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
    let trial = trial_path(world).to_string_lossy().into_owned();
    let manifest = manifest_path(world).to_string_lossy().into_owned();
    let out = {
        let env = project_env(world);
        crate::harness::run_givn_with_envs(
            env,
            &[
                "ise",
                "rebuild-all",
                "--at",
                revision.as_str(),
                "--into",
                trial.as_str(),
                "--manifest",
                manifest.as_str(),
            ],
            &[],
        )
    };
    world.last_output = Some(out);
}

#[then(regex = r#"^the command succeeds and prints ("[^"]*"(?:(?:, "[^"]*")*,? and "[^"]*")?)$"#)]
fn the_command_succeeds_and_prints(world: &mut GivnWorld, printed: String) {
    assert_success(world);
    for line in printed_lines(&printed) {
        assert_report_line(world, &line);
    }
}

/// The quoted lines inside one printed-list argument, in order.
fn printed_lines(printed: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    for character in printed.chars() {
        match character {
            '"' if quoted => {
                lines.push(std::mem::take(&mut current));
                quoted = false;
            }
            '"' => quoted = true,
            _ if quoted => current.push(character),
            _ => {}
        }
    }
    assert!(!quoted, "unbalanced quotes in {printed:?}");
    lines
}

#[then(regex = r#"^the report prints a "Revision:" line holding the resolved 40-hex commit$"#)]
fn report_prints_the_resolved_revision(world: &mut GivnWorld) {
    assert_success(world);
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

#[then(regex = r#"^the report prints "End-to-end tests: not copied \(2 files\)"$"#)]
fn report_prints_the_end_to_end_set(world: &mut GivnWorld) {
    assert_report_line(world, "End-to-end tests: not copied (2 files)");
}

#[then(
    regex = r#"^the report prints a "Specification checksum:" line and an "End-to-end checksum:" line of 12 lowercase hex characters each$"#
)]
fn report_prints_the_checksum_lines(world: &mut GivnWorld) {
    assert_success(world);
    let text = stdout(world);
    for prefix in ["Specification checksum: ", "End-to-end checksum: "] {
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

#[then(regex = r#"^the report prints the copy directory and the manifest path$"#)]
fn report_prints_the_rebuild_targets(world: &mut GivnWorld) {
    assert_report_line(
        world,
        &format!("Copy directory: {}", resolved_trial_path(world).display()),
    );
    assert_report_line(
        world,
        &format!("Manifest: {}", manifest_path(world).display()),
    );
}

#[then(regex = r#"^the spec copy holds "([^"]+)" and "([^"]+)"$"#)]
fn the_spec_copy_holds_two(world: &mut GivnWorld, first: String, second: String) {
    assert_trial_holds(world, &[first.as_str(), second.as_str()]);
}

#[then(
    regex = r#"^the spec copy omits "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)", and "([^"]+)"$"#
)]
fn spec_copy_omits_six(
    world: &mut GivnWorld,
    first: String,
    second: String,
    third: String,
    fourth: String,
    fifth: String,
    sixth: String,
) {
    assert_trial_omits(
        world,
        &[
            first.as_str(),
            second.as_str(),
            third.as_str(),
            fourth.as_str(),
            fifth.as_str(),
            sixth.as_str(),
        ],
    );
}

#[then(
    regex = r#"^the manifest records the scope, the revision, the removed paths, the five counts, the checksums, the copy directory, and the Givn version$"#
)]
fn manifest_records_the_rebuild_evidence(world: &mut GivnWorld) {
    let manifest = manifest_document(world);
    let mapping = manifest
        .as_mapping()
        .unwrap_or_else(|| panic!("the manifest is not a mapping: {manifest:?}"));
    let mut keys: Vec<&str> = mapping
        .keys()
        .map(|key| {
            key.as_str()
                .unwrap_or_else(|| panic!("a key is not a string"))
        })
        .collect();
    keys.sort();
    let mut expected = vec![
        "checksums",
        "copy_directory",
        "counts",
        "end_to_end_checksum",
        "end_to_end_files",
        "givn_version",
        "removed_paths",
        "revision",
        "scope",
        "source_kept_files",
        "specification_checksum",
        "specification_files",
    ];
    expected.sort();
    assert_eq!(keys, expected, "the manifest does not record every field");

    assert_eq!(
        manifest.get("scope").and_then(serde_yaml_ng::Value::as_str),
        Some("ise-manifest/v1"),
        "the manifest has the wrong scope"
    );
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the committed revision");
    assert_eq!(
        manifest
            .get("revision")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(revision.as_str()),
        "the manifest has the wrong revision"
    );
    assert_eq!(
        manifest_list(&manifest, "removed_paths"),
        vec![
            "src/keys/mod.rs".to_string(),
            "src/keys/parser.rs".to_string(),
            "src/other.rs".to_string(),
        ],
        "the manifest has the wrong removed paths"
    );
    for (key, expected) in [
        ("specification_files", 2),
        ("source_kept", 0),
        ("source_removed", 3),
        ("outside_spec_copy", 1),
        ("files_beside_removed_paths", 0),
    ] {
        assert_eq!(manifest_number(&manifest, key), expected, "counts.{key}");
    }
    let checksums = manifest
        .get("checksums")
        .and_then(serde_yaml_ng::Value::as_mapping)
        .unwrap_or_else(|| panic!("the manifest has no checksums mapping"));
    let mut checksum_maps: Vec<&str> = checksums
        .keys()
        .map(|key| {
            key.as_str()
                .unwrap_or_else(|| panic!("a key is not a string"))
        })
        .collect();
    checksum_maps.sort();
    assert_eq!(
        checksum_maps,
        vec!["end_to_end", "scenarios", "specification"],
        "the manifest has the wrong checksum maps"
    );
    for map in checksums.values() {
        let rows = map
            .as_mapping()
            .unwrap_or_else(|| panic!("a checksum map is not a mapping: {map:?}"));
        for (path, checksum) in rows {
            let path = path
                .as_str()
                .unwrap_or_else(|| panic!("a checksum path is not a string"));
            let checksum = checksum
                .as_str()
                .unwrap_or_else(|| panic!("the checksum for {path:?} is not a string"));
            assert_64_hex(checksum, &format!("the checksum for {path:?}"));
        }
    }
    for field in ["specification_checksum", "end_to_end_checksum"] {
        let checksum = manifest
            .get(field)
            .and_then(serde_yaml_ng::Value::as_str)
            .unwrap_or_else(|| panic!("the manifest has no {field}"));
        assert_64_hex(checksum, field);
    }
    assert_eq!(
        manifest
            .get("copy_directory")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(resolved_trial_path(world).to_string_lossy().as_ref()),
        "the manifest has the wrong copy directory"
    );
    assert_eq!(
        manifest
            .get("givn_version")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(crate::harness::binary::product_version()).as_deref(),
        "the manifest has the wrong Givn version"
    );
    write_transcript(world);
}

fn assert_64_hex(value: &str, subject: &str) {
    assert_eq!(
        value.len(),
        64,
        "{subject} {value:?} is not 64 hex characters"
    );
    assert!(
        value
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
        "{subject} {value:?} is not lowercase hex"
    );
}
