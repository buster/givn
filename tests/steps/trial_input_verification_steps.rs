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
    self, FOSSIL_FEATURE_PATH, REFERENCE_PATH, REPLACED_SOURCE_PATH, VERIFY_E2E_STEP_PATH,
    VERIFY_KEPT_PATH,
};
use crate::steps::{GivnWorld, output, stderr};

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

#[when(regex = r#"^the verifier changes the e2e file "tests/e2e/steps/keys\.rs" in the trial$"#)]
fn verifier_changes_the_e2e_file_in_the_trial(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(&trial(world).join(VERIFY_E2E_STEP_PATH));
}

#[when(regex = r#"^the verifier changes the kept source file "src/keys/steps\.rs" in the trial$"#)]
fn verifier_changes_the_kept_source_file(world: &mut GivnWorld) {
    ise_clean_room_support::append_bytes(&trial(world).join(VERIFY_KEPT_PATH));
}

#[when(
    regex = r#"^the verifier changes the retained files "givn/specs/demo/spec\.feature" and "docs/reference\.md" and the e2e file "tests/e2e/steps/keys\.rs" in the trial$"#
)]
fn verifier_changes_three_frozen_files(world: &mut GivnWorld) {
    for path in [FOSSIL_FEATURE_PATH, REFERENCE_PATH, VERIFY_E2E_STEP_PATH] {
        ise_clean_room_support::append_bytes(&trial(world).join(path));
    }
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
