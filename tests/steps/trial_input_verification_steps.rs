//! Step definitions for the `trial-input-verification` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. The clean report assertions are
//! bound in the capability's E2E file: the `@e2e` scenario and the regular
//! no-drift variant assert the same two lines, and one phrase has exactly one
//! binding.

use std::path::{Path, PathBuf};

use cucumber::{given, then, when};

use crate::common;
use crate::steps::clean_room_preparation_steps::{prepare_trial, project_env};
use crate::steps::ise_clean_room_support::{
    self, ADDED_E2E_FEATURE_CONTENT, ADDED_E2E_FEATURE_PATH, ADDED_E2E_ROOT_MANIFEST_PATH,
    ADDED_E2E_STEP_PATH, FOSSIL_FEATURE_PATH, REFERENCE_PATH, REPLACED_SOURCE_PATH,
    VERIFY_E2E_STEP_PATH, VERIFY_KEPT_PATH,
};
use crate::steps::{GivnWorld, output, stderr, stdout};

fn trial(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.trial_path().to_path_buf()
}

fn record(world: &GivnWorld) -> PathBuf {
    world.ise_clean_room.receipt_path().to_path_buf()
}

/// Prepare the scenario's trial: one committed fixture revision, then a real
/// `givn ise replace` that replaces the parser and writes the record.
fn prepare(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::verify_trial_project());
    ise_clean_room_support::commit_verify_trial_revision(project_env(world));
    prepare_prepared(world);
}

/// Run the real `givn ise replace` over the project's committed revision and
/// require success.
fn prepare_prepared(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", REPLACED_SOURCE_PATH]);
    let out = output(world);
    assert!(
        out.status.success(),
        "preparing the fixture trial failed: {}",
        stderr(world)
    );
}

/// One `givn ise verify` run; `record_path` overrides the prepared record.
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

fn assert_failed_naming(world: &GivnWorld, names: &[&str]) {
    let out = output(world);
    let text = stderr(world);
    assert_eq!(out.status.code(), Some(1), "stderr={text}");
    for name in names {
        assert!(
            text.contains(name),
            "stderr {text:?} does not name {name:?}"
        );
    }
}

fn rewrite_record(world: &GivnWorld, mutate: impl FnOnce(&mut serde_yaml_ng::Mapping)) {
    let path = record(world);
    let mut mapping = ise_clean_room_support::record_mapping(&path);
    mutate(&mut mapping);
    ise_clean_room_support::write_record_mapping(&path, mapping);
}

// --- Givens ---------------------------------------------------------------

#[given(regex = r#"^a prepared Git trial with a receipt$"#)]
fn prepared_git_trial(world: &mut GivnWorld) {
    prepare(world);
}

#[given(
    regex = r#"^a prepared Git trial with a receipt whose frozen package holds twenty-five files$"#
)]
fn prepared_git_trial_with_a_broad_package(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::broad_package_project());
    prepare_prepared(world);
}

#[given(
    regex = r#"^a prepared Git trial whose record and trial were written before the frozen package was excluded$"#
)]
fn prepared_git_trial_written_before_the_exclusion(world: &mut GivnWorld) {
    prepare(world);
    let trial = trial(world);
    for path in [
        ise_clean_room_support::VERIFY_E2E_MANIFEST_PATH,
        VERIFY_E2E_STEP_PATH,
    ] {
        ise_clean_room_support::copy_into_trial(project_env(world), &trial, path);
    }
    ise_clean_room_support::rewrite_record_without_excluded_roles(&record(world));
}

#[given(regex = r#"^a prepared Git trial whose frozen role covers the declaration$"#)]
fn prepared_git_trial_whose_frozen_role_covers_the_declaration(world: &mut GivnWorld) {
    let env = ise_clean_room_support::e2e_wildcard_project();
    ise_clean_room_support::commit_files(
        &env,
        &[
            (
                ise_clean_room_support::DECLARATION_PATH,
                &ise_clean_room_support::e2e_declaration("\"givn/**\"", "\"src/**\""),
            ),
            ("givn/specs/a.md", "spec\n"),
            (
                ise_clean_room_support::KEPT_PATH,
                ise_clean_room_support::KEPT_CONTENT,
            ),
        ],
        "fixture: frozen role over the declaration",
    );
    world.env = Some(env);
    prepare_prepared(world);
}

#[given(regex = r#"^a prepared Git trial with a corrupted record$"#)]
fn prepared_git_trial_with_a_corrupted_record(world: &mut GivnWorld) {
    prepare(world);
    std::fs::write(record(world), "not: a-trial-record\n").expect("corrupt the record");
}

#[given(regex = r#"^a prepared Git trial with a receipt carrying an unknown field$"#)]
fn prepared_git_trial_with_an_unknown_field(world: &mut GivnWorld) {
    prepare(world);
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("extra_field".to_string()),
            serde_yaml_ng::Value::Number(1.into()),
        );
    });
}

#[given(regex = r#"^a prepared Git trial with a receipt missing the counts field$"#)]
fn prepared_git_trial_missing_the_counts_field(world: &mut GivnWorld) {
    prepare(world);
    rewrite_record(world, |mapping| {
        mapping.remove(serde_yaml_ng::Value::String("counts".to_string()));
    });
}

#[given(regex = r#"^a prepared Git trial with a record that is not a trial record$"#)]
fn prepared_git_trial_with_a_foreign_record(world: &mut GivnWorld) {
    prepare(world);
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("content_scope".to_string()),
            serde_yaml_ng::Value::String("other/v1".to_string()),
        );
    });
}

#[given(regex = r#"^a prepared Git trial with a receipt carrying the schema 2$"#)]
fn prepared_git_trial_with_another_schema(world: &mut GivnWorld) {
    prepare(world);
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("schema".to_string()),
            serde_yaml_ng::Value::Number(2.into()),
        );
    });
}

#[given(
    regex = r#"^a prepared Git trial with a receipt whose replaced digest is not a SHA-256 value$"#
)]
fn prepared_git_trial_with_a_short_replaced_digest(world: &mut GivnWorld) {
    prepare(world);
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("replaced_digest".to_string()),
            serde_yaml_ng::Value::String("abc123".to_string()),
        );
    });
}

#[given(regex = r#"^the project disables ise after preparation$"#)]
fn project_disables_ise(world: &mut GivnWorld) {
    ise_clean_room_support::set_addon(project_env(world), "ise", false);
}

#[given(regex = r#"^the project disables architecture principles after preparation$"#)]
fn project_disables_architecture_principles(world: &mut GivnWorld) {
    ise_clean_room_support::set_addon(project_env(world), "architecture_principles", false);
}

// --- Whens ----------------------------------------------------------------

#[when(
    regex = r#"^the verifier changes the retained file "givn/specs/demo/spec\.feature" in the trial$"#
)]
fn verifier_changes_the_retained_file(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(&trial(world).join(FOSSIL_FEATURE_PATH));
}

#[when(regex = r#"^the verifier copies the e2e file "([^"]+)" into the trial$"#)]
fn verifier_copies_the_e2e_file_into_the_trial(world: &mut GivnWorld, path: String) {
    ise_clean_room_support::copy_into_trial(project_env(world), &trial(world), &path);
}

#[when(regex = r#"^the verifier changes the kept source file "src/keys/steps\.rs" in the trial$"#)]
fn verifier_changes_the_kept_source_file(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(&trial(world).join(VERIFY_KEPT_PATH));
}

#[when(regex = r#"^the verifier changes the retained files "([^"]+)" and "([^"]+)" in the trial$"#)]
fn verifier_changes_two_retained_files(world: &mut GivnWorld, first: String, second: String) {
    for path in [first, second] {
        ise_clean_room_support::append_bytes(&trial(world).join(path));
    }
}

#[when(regex = r#"^the verifier copies the whole e2e package "([^"]+)" into the trial$"#)]
fn verifier_copies_the_whole_e2e_package(world: &mut GivnWorld, directory: String) {
    ise_clean_room_support::copy_tree_into_trial(project_env(world), &trial(world), &directory);
}

#[when(
    regex = r#"^the verifier changes the retained file "givn/specification\.yaml" in the trial$"#
)]
fn verifier_changes_the_declaration_in_the_trial(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(
        &trial(world).join(ise_clean_room_support::DECLARATION_PATH),
    );
}

#[when(regex = r#"^the trial loses the retained file "givn/specification\.yaml"$"#)]
fn trial_loses_the_declaration(world: &mut GivnWorld) {
    std::fs::remove_file(trial(world).join(ise_clean_room_support::DECLARATION_PATH))
        .expect("remove the declaration from the trial");
}

#[when(
    regex = r#"^the developer rewrites the checkout declaration's "structure" role to "docs/glossary\.md"$"#
)]
fn developer_rewrites_the_structure_role(world: &mut GivnWorld) {
    ise_clean_room_support::rewrite_checkout_role(
        project_env(world),
        "structure",
        "[\"docs/glossary.md\"]",
    );
}

#[when(
    regex = r#"^the developer adds the e2e root "tests/extra-e2e" to the checkout declaration$"#
)]
fn developer_adds_the_extra_e2e_root(world: &mut GivnWorld) {
    ise_clean_room_support::rewrite_checkout_role(
        project_env(world),
        "e2e",
        "[\"tests/e2e\", \"tests/extra-e2e\"]",
    );
}

#[when(
    regex = r#"^the developer adds the e2e file "tests/extra-e2e/Cargo\.toml" to the checkout$"#
)]
fn developer_adds_the_extra_e2e_manifest(world: &mut GivnWorld) {
    ise_clean_room_support::add_checkout_file(
        project_env(world),
        ADDED_E2E_ROOT_MANIFEST_PATH,
        "[package]\nname = \"extra-e2e\"\n",
    );
}

#[when(regex = r#"^the trial loses the retained file "givn/specs/demo/spec\.feature"$"#)]
fn trial_loses_the_retained_file(world: &mut GivnWorld) {
    std::fs::remove_file(trial(world).join(FOSSIL_FEATURE_PATH))
        .expect("remove the retained file from the trial");
}

#[when(
    regex = r#"^the developer changes the e2e file "tests/e2e/steps/keys\.rs" in the checkout$"#
)]
fn developer_changes_the_e2e_file(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(&project_env(world).path(VERIFY_E2E_STEP_PATH));
}

#[when(
    regex = r#"^the developer changes a scenario tagged "e2e" in "givn/specs/demo/spec\.feature"$"#
)]
fn developer_changes_the_e2e_scenario(world: &mut GivnWorld) {
    let path = project_env(world).path(FOSSIL_FEATURE_PATH);
    let text = std::fs::read_to_string(&path).expect("read the checkout feature");
    let changed = text.replace("Given nothing", "Given something else");
    assert_ne!(changed, text, "the fixture feature has no step to change");
    std::fs::write(&path, changed).expect("rewrite the checkout feature");
}

#[when(regex = r#"^the developer adds the e2e file "tests/e2e/steps/extra\.rs" to the checkout$"#)]
fn developer_adds_the_e2e_file(world: &mut GivnWorld) {
    ise_clean_room_support::add_checkout_file(
        project_env(world),
        ADDED_E2E_STEP_PATH,
        "// extra e2e steps\n",
    );
}

#[when(
    regex = r#"^the developer adds the e2e scenario file "givn/specs/demo/extra\.feature" to the checkout$"#
)]
fn developer_adds_the_e2e_feature(world: &mut GivnWorld) {
    ise_clean_room_support::add_checkout_file(
        project_env(world),
        ADDED_E2E_FEATURE_PATH,
        ADDED_E2E_FEATURE_CONTENT,
    );
}

#[when(regex = r#"^the verifier re-checks the trial against the record$"#)]
fn verifier_re_checks(world: &mut GivnWorld) {
    re_check(world, None);
}

#[when(regex = r#"^the verifier copies the record inside the trial and re-checks it there$"#)]
fn verifier_copies_the_record_inside_the_trial(world: &mut GivnWorld) {
    let inside = trial(world).join("record.yaml");
    std::fs::copy(record(world), &inside).expect("copy the record inside the trial");
    re_check(world, Some(&inside));
}

#[when(
    regex = r#"^the verifier rewrites the record's written path and re-checks it against the trial$"#
)]
fn verifier_rewrites_the_written_path(world: &mut GivnWorld) {
    let record_path = record(world);
    let elsewhere = record_path
        .parent()
        .expect("the record has a parent")
        .join("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("create the other trial directory");
    let elsewhere = elsewhere.to_string_lossy().into_owned();
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("written_path".to_string()),
            serde_yaml_ng::Value::String(elsewhere),
        );
    });
    re_check(world, None);
}

#[when(regex = r#"^the verifier rewrites the record's backend to "fossil" and re-checks it$"#)]
fn verifier_rewrites_the_backend(world: &mut GivnWorld) {
    rewrite_record(world, |mapping| {
        mapping.insert(
            serde_yaml_ng::Value::String("backend".to_string()),
            serde_yaml_ng::Value::String("fossil".to_string()),
        );
    });
    re_check(world, None);
}

// --- Thens ----------------------------------------------------------------

#[then(regex = r#"^verification fails and names "([^"]+)"$"#)]
fn verification_fails_and_names(world: &mut GivnWorld, name: String) {
    assert_failed_naming(world, &[name.as_str()]);
}

#[then(regex = r#"^verification fails and names all three paths$"#)]
fn verification_fails_and_names_all_three_paths(world: &mut GivnWorld) {
    assert_failed_naming(
        world,
        &[FOSSIL_FEATURE_PATH, REFERENCE_PATH, VERIFY_E2E_STEP_PATH],
    );
    let text = stderr(world);
    assert!(
        text.contains("3 drifting path(s)"),
        "stderr {text:?} does not report three drifting paths"
    );
}

#[then(regex = r#"^verification reports the trial holds a path the frozen package covers$"#)]
fn verification_reports_the_frozen_path(world: &mut GivnWorld) {
    let text = stderr(world);
    for needle in [
        "holds",
        "the frozen e2e package covers",
        "the builder must not receive the acceptance suite",
    ] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
}

#[then(regex = r#"^verification fails and names the first twenty frozen paths in sorted order$"#)]
fn verification_names_the_first_twenty_frozen_paths(world: &mut GivnWorld) {
    assert_failed_naming(world, &["25 path(s)"]);
    let mut expected: Vec<String> =
        vec![ise_clean_room_support::VERIFY_E2E_MANIFEST_PATH.to_string()];
    for index in 0..19 {
        expected.push(format!("tests/e2e/steps/f{index:02}.rs"));
    }
    let text = stderr(world);
    let mut cursor = 0;
    for path in &expected {
        let rest = &text[cursor..];
        let position = rest
            .find(path)
            .unwrap_or_else(|| panic!("stderr {text:?} does not name {path:?}"));
        cursor += position + path.len();
    }
    assert!(
        !text.contains("tests/e2e/steps/f19.rs"),
        "stderr {text:?} names a path beyond the twenty-path bound"
    );
}

#[then(regex = r#"^verification reports the count of the remaining frozen paths$"#)]
fn verification_reports_the_remaining_count(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("(and 5 more)"),
        "stderr {text:?} does not report the five remaining frozen paths"
    );
}

#[then(regex = r#"^the record names no excluded role$"#)]
fn record_names_no_excluded_role(world: &mut GivnWorld) {
    let path = record(world);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the record {}: {error}", path.display()));
    let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
        .unwrap_or_else(|error| panic!("parse the record {}: {error}", path.display()));
    assert!(
        document.get("excluded_roles").is_none(),
        "the record still carries an excluded role"
    );
}

#[then(regex = r#"^verification fails and names the record$"#)]
fn verification_fails_and_names_the_record(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str()]);
}

#[then(regex = r#"^verification fails and names the record and the backend$"#)]
fn verification_fails_and_names_the_record_and_the_backend(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), "fossil"]);
}

#[then(regex = r#"^verification fails and names the record and the missing field$"#)]
fn verification_fails_and_names_the_record_and_the_missing_field(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), "counts"]);
}

#[then(regex = r#"^verification fails and names the record and the trial path$"#)]
fn verification_fails_and_names_the_record_and_the_trial(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    let trial = trial(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), trial.as_str()]);
}

#[then(regex = r#"^verification fails and names the record and the unknown field$"#)]
fn verification_fails_and_names_the_record_and_the_unknown_field(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), "extra_field"]);
}

#[then(regex = r#"^verification fails and names the record path$"#)]
fn verification_fails_and_names_the_record_path(world: &mut GivnWorld) {
    let copied = trial(world).join("record.yaml");
    assert_failed_naming(world, &[copied.to_string_lossy().as_ref()]);
}

#[then(regex = r#"^verification names the expected record scope$"#)]
fn verification_names_the_expected_record_scope(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("ise-trial/v1"),
        "stderr {text:?} does not name the expected record scope"
    );
}

#[then(regex = r#"^verification reports no record error$"#)]
fn verification_reports_no_record_error(world: &mut GivnWorld) {
    let text = stderr(world);
    let record = record(world).to_string_lossy().into_owned();
    assert!(
        !text.contains(&record) && !text.contains("trial record"),
        "stderr {text:?} reports a record error; an ise-off project reads no record"
    );
}

#[then(regex = r#"^verification reports the backend mismatch$"#)]
fn verification_reports_the_backend_mismatch(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("backend"),
        "stderr {text:?} does not report the backend mismatch"
    );
}

#[then(regex = r#"^verification reports the record is inside the trial$"#)]
fn verification_reports_the_record_is_inside_the_trial(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("inside the trial"),
        "stderr {text:?} does not report the record's placement"
    );
}

#[then(regex = r#"^verification reports the written path mismatch$"#)]
fn verification_reports_the_written_path_mismatch(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("written path"),
        "stderr {text:?} does not report the written-path mismatch"
    );
}

#[then(regex = r#"^verification reports the e2e package gained the path$"#)]
fn verification_reports_the_gained_path(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("e2e package gained this path"),
        "stderr {text:?} does not report the gained e2e package path"
    );
}

#[then(regex = r#"^verification reports the selected e2e scenarios differ$"#)]
fn verification_reports_the_selected_scenarios_differ(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("selected @e2e scenarios differ"),
        "stderr {text:?} does not report the selected-scenario drift"
    );
}

#[then(regex = r#"^verification fails and names the record and the schema$"#)]
fn verification_fails_and_names_the_record_and_the_schema(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), "schema"]);
}

#[then(regex = r#"^verification reports the expected record schema$"#)]
fn verification_reports_the_expected_record_schema(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("a trial record is schema 1"),
        "stderr {text:?} does not name the expected record schema"
    );
}

#[then(regex = r#"^verification fails and names the record and the replaced digest$"#)]
fn verification_fails_and_names_the_record_and_the_digest(world: &mut GivnWorld) {
    let record = record(world).to_string_lossy().into_owned();
    assert_failed_naming(world, &[record.as_str(), "replaced_digest"]);
}

#[then(regex = r#"^verification reports the record is no trial evidence$"#)]
fn verification_reports_the_record_is_no_trial_evidence(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("no trial evidence"),
        "stderr {text:?} does not report that the record is no trial evidence"
    );
}

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

// --- Batch 9: compare-report ----------------------------------------------

/// The added file under the copy's declared `behavior` role.
const ADDED_SPECIFICATION_PATH: &str = "givn/specs/extra.feature";
/// The added file under the copy's declared `e2e` role.
const ADDED_E2E_COPY_PATH: &str = "tests/e2e/steps/extra.rs";

/// Build one spec copy from HEAD with a manifest, replacing the parser.
fn prepare_spec_copy(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::verify_trial_project());
    ise_clean_room_support::commit_verify_trial_revision(project_env(world));
    world.ise_clean_room.ensure_targets();
    let copy = trial(world).to_string_lossy().into_owned();
    let manifest = record(world).to_string_lossy().into_owned();
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

fn add_to_copy(world: &GivnWorld, path: &str, content: &str) {
    let target = trial(world).join(path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("create the added file's parent");
    }
    std::fs::write(&target, content).expect("write the added file");
}

fn assert_stdout_line(world: &GivnWorld, expected: &str) {
    let text = stdout(world);
    assert!(
        text.lines().any(|printed| printed == expected),
        "stdout {text:?} does not print the line {expected:?}"
    );
}

#[given(regex = r#"^a prepared spec copy with a manifest$"#)]
fn prepared_spec_copy(world: &mut GivnWorld) {
    prepare_spec_copy(world);
}

#[when(regex = r#"^the verifier compares the spec copy with the manifest$"#)]
fn verifier_compares(world: &mut GivnWorld) {
    let copy = trial(world).to_string_lossy().into_owned();
    let manifest = record(world).to_string_lossy().into_owned();
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

#[when(
    regex = r#"^the agent edits the Specification file "([^"]+)" the spec copy was written with$"#
)]
fn agent_edits_the_copy_specification_file(world: &mut GivnWorld, path: String) {
    ise_clean_room_support::append_bytes(&trial(world).join(path));
}

#[when(
    regex = r#"^the agent deletes the Specification file "([^"]+)" the spec copy was written with$"#
)]
fn agent_deletes_the_copy_specification_file(world: &mut GivnWorld, path: String) {
    std::fs::remove_file(trial(world).join(path)).expect("delete the Specification file");
}

#[when(regex = r#"^the agent adds a rebuild file at a removed path$"#)]
fn agent_adds_a_rebuild_file(world: &mut GivnWorld) {
    add_to_copy(world, REPLACED_SOURCE_PATH, "pub fn rebuilt() {}\n");
}

#[when(
    regex = r#"^the agent edits the kept Source code file "([^"]+)" the spec copy was written with$"#
)]
fn agent_edits_the_kept_source_file(world: &mut GivnWorld, path: String) {
    ise_clean_room_support::append_bytes(&trial(world).join(path));
}

#[when(regex = r#"^the agent adds a file under a declared Specification path$"#)]
fn agent_adds_under_a_specification_path(world: &mut GivnWorld) {
    add_to_copy(world, ADDED_SPECIFICATION_PATH, "Feature: extra\n");
    world.ise_clean_room.added_copy_path = Some(ADDED_SPECIFICATION_PATH.to_string());
}

#[when(regex = r#"^the agent adds a file under the declared End-to-end role$"#)]
fn agent_adds_under_the_e2e_role(world: &mut GivnWorld) {
    add_to_copy(world, ADDED_E2E_COPY_PATH, "// extra e2e steps\n");
    world.ise_clean_room.added_copy_path = Some(ADDED_E2E_COPY_PATH.to_string());
}

#[when(
    regex = r#"^the agent deletes the kept Source code file "([^"]+)" the spec copy was written with$"#
)]
fn agent_deletes_the_kept_source_file(world: &mut GivnWorld, path: String) {
    std::fs::remove_file(trial(world).join(path)).expect("delete the kept Source code file");
}

#[when(regex = r#"^the developer edits the Specification file "([^"]+)" in the checkout$"#)]
fn developer_edits_the_checkout_specification_file(world: &mut GivnWorld, path: String) {
    ise_clean_room_support::append_bytes(&project_env(world).path(&path));
}

#[then(regex = r#"^comparison prints "([^"]+)" and "([^"]+)"$"#)]
fn comparison_prints_two_lines(world: &mut GivnWorld, first: String, second: String) {
    assert_stdout_line(world, &first);
    assert_stdout_line(world, &second);
}

#[then(regex = r#"^comparison prints "([^"]+)"$"#)]
fn comparison_prints_one_line(world: &mut GivnWorld, expected: String) {
    assert_stdout_line(world, &expected);
}

#[then(regex = r#"^comparison exits zero$"#)]
fn comparison_exits_zero(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(0),
        "stderr={}",
        stderr(world)
    );
}

#[then(regex = r#"^comparison exits 1$"#)]
fn comparison_exits_one(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(1),
        "stderr={}",
        stderr(world)
    );
}

#[then(
    regex = r#"^comparison fails and exits 1, naming the added path as a Specification change$"#
)]
fn comparison_refuses_the_added_path(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(1),
        "stderr={}",
        stderr(world)
    );
    let path = world
        .ise_clean_room
        .added_copy_path
        .clone()
        .expect("the When added a file to the copy");
    let text = stderr(world);
    assert!(
        text.contains(&path),
        "stderr {text:?} does not name the added path {path:?}"
    );
    assert!(
        text.contains("Specification"),
        "stderr {text:?} does not name the Specification change"
    );
}

#[then(regex = r#"^comparison prints no "([^"]+)" line$"#)]
fn comparison_prints_no_such_line(world: &mut GivnWorld, needle: String) {
    let text = stdout(world);
    assert!(
        !text.lines().any(|printed| printed == needle),
        "stdout {text:?} prints the line {needle:?}"
    );
}

#[then(regex = r#"^comparison fails and exits 1, naming "([^"]+)"$"#)]
fn comparison_fails_and_names(world: &mut GivnWorld, name: String) {
    assert_eq!(
        output(world).status.code(),
        Some(1),
        "stderr={}",
        stderr(world)
    );
    let text = stderr(world);
    assert!(
        text.contains(&name),
        "stderr {text:?} does not name {name:?}"
    );
}

#[then(regex = r#"^comparison prints exactly one "([^"]+)" line under the "([^"]+)" subject$"#)]
fn comparison_prints_one_line_under_the_subject(
    world: &mut GivnWorld,
    expected: String,
    subject: String,
) {
    let text = stdout(world);
    let lines: Vec<&str> = text.lines().collect();
    let header = format!("{subject}:");
    let start = lines
        .iter()
        .position(|line| line.starts_with(&header))
        .unwrap_or_else(|| panic!("stdout {text:?} prints no {header:?} line"));
    let end = lines
        .iter()
        .skip(start + 1)
        .position(|line| line.starts_with("End-to-end tests:"))
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    let count = lines[start + 1..end]
        .iter()
        .filter(|line| **line == expected.as_str())
        .count();
    assert_eq!(
        count, 1,
        "stdout {text:?} prints {count} {expected:?} lines under the {subject:?} subject"
    );
}
