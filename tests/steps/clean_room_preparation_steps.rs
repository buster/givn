//! Step definitions for the `clean-room-preparation` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. Unimplemented phrases use the
//! not-implemented stub pattern from `design.md` "Strict Mode": RED is a
//! failed step that names the phrase, never an unbound or skipped step.

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use crate::steps::GivnWorld;

// --- Batch 1: admission-refusals -------------------------------------------

use crate::common;
use crate::steps::ise_clean_room_support;
use crate::steps::{output, stderr, stdout};

pub(crate) fn project_env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn assert_exit_one(world: &GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(world));
}

fn assert_exit_two(world: &GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(2), "stderr={}", stderr(world));
}

fn assert_failure_names_path(world: &GivnWorld, path: &std::path::Path) {
    assert_exit_one(world);
    let name = path.to_string_lossy();
    let text = stderr(world);
    assert!(
        text.contains(name.as_ref()),
        "stderr {text:?} does not name {name:?}"
    );
}

/// The named absence refusal, not a write failure that merely mentions the
/// path: create-new semantics are what the scenario protects.
fn assert_failure_refuses_existing(world: &GivnWorld, path: &std::path::Path) {
    assert_failure_names_path(world, path);
    let text = stderr(world);
    assert!(
        text.contains("already exists"),
        "stderr {text:?} does not report the existing path"
    );
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

/// The written record parsed as YAML; the receipt path must exist.
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

#[cfg(unix)]
fn assert_trial_executable(world: &GivnWorld, path: &str) {
    use std::os::unix::fs::PermissionsExt;

    let target = trial_path(world).join(path);
    let metadata =
        std::fs::symlink_metadata(&target).unwrap_or_else(|error| panic!("stat {path}: {error}"));
    assert!(
        metadata.is_file(),
        "the trial's {path} is not a regular file"
    );
    assert!(
        metadata.permissions().mode() & 0o111 != 0,
        "the trial's {path} is not executable (mode {:o})",
        metadata.permissions().mode() & 0o777
    );
}

#[cfg(not(unix))]
fn assert_trial_executable(world: &GivnWorld, path: &str) {
    let target = trial_path(world).join(path);
    assert!(
        target.is_file(),
        "the trial at {} does not hold {path}",
        target.display()
    );
}

fn assert_trial_symlink(world: &GivnWorld, path: &str, expected_target: &str) {
    let target = trial_path(world).join(path);
    let metadata =
        std::fs::symlink_metadata(&target).unwrap_or_else(|error| panic!("stat {path}: {error}"));
    assert!(
        metadata.file_type().is_symlink(),
        "the trial's {path} is not a symlink"
    );
    let link = std::fs::read_link(&target).expect("read the trial symlink");
    assert_eq!(
        link,
        std::path::Path::new(expected_target),
        "the trial's {path} points at {}",
        link.display()
    );
}

/// Walk the trial without following links and refuse any version-control
/// metadata entry, at any depth.
fn assert_trial_has_no_vcs_entries(world: &GivnWorld) {
    let trial = trial_path(world);
    let mut directories = vec![trial.clone()];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(&directory).expect("read the trial directory") {
            let path = entry.expect("read the trial entry").path();
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            assert!(
                name != ".git" && name != ".fslckout",
                "the trial at {} holds the version-control entry {}",
                trial.display(),
                path.display()
            );
            if path.is_dir() {
                directories.push(path);
            }
        }
    }
}

/// Run one replacement form against the standard trial and receipt targets.
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

/// Run a full replacement against the state's trial target, with the state's
/// receipt target only when the scenario supplies one.
fn prepare_with_target(world: &mut GivnWorld, with_receipt: bool) {
    let trial = trial_path(world).to_string_lossy().into_owned();
    let mut args: Vec<String> = [
        "ise",
        "replace",
        "--at",
        "HEAD",
        "--full",
        "--into",
        trial.as_str(),
    ]
    .iter()
    .map(|argument| (*argument).to_string())
    .collect();
    if with_receipt {
        args.push("--receipt".to_string());
        args.push(receipt_path(world).to_string_lossy().into_owned());
    }
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let env = project_env(world);
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, &argv, &[]),
        None => common::run_givn_with_envs(env, &argv, &[]),
    };
    world.last_output = Some(out);
}

#[given(regex = r#"^a Git project with ise enabled and no declaration$"#)]
fn git_project_without_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_without_declaration());
}

#[given(regex = r#"^a Git project with ise enabled and a malformed declaration$"#)]
fn git_project_with_malformed_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::malformed_declaration_project());
}

#[given(regex = r#"^a Git project with a declaration and ise off$"#)]
fn git_project_with_ise_off(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_off_project());
}

#[given(
    regex = r#"^a Git project with ise on, architecture principles off, and a malformed declaration$"#
)]
fn git_project_with_architecture_principles_off(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::architecture_principles_off_project());
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration, and one modified tracked file and one untracked file$"#
)]
fn git_project_with_a_dirty_checkout(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::dirty_project());
}

#[given(regex = r#"^a directory with ise enabled, a declaration, and no version-control backend$"#)]
fn directory_without_a_backend(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::no_backend_project());
}

#[given(regex = r#"^a Git worktree whose project root is nested inside the worktree top level$"#)]
fn git_worktree_with_a_nested_project_root(world: &mut GivnWorld) {
    let (env, project) = ise_clean_room_support::nested_project();
    world.ise_clean_room.nested_project_root = Some(project.clone());
    world.cwd_override = Some(project);
    world.env = Some(env);
}

#[given(regex = r#"^a project root with both a Git worktree marker and a Fossil checkout marker$"#)]
fn project_root_with_both_markers(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::mixed_markers_project());
}

#[given(regex = r#"^a Git project with ise enabled and a declaration$"#)]
fn git_project_with_a_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project());
}

#[when(regex = r#"^the developer prepares a trial from "([^"]+)" with a full replacement$"#)]
fn developer_prepares_a_full_trial(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
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
    let env = project_env(world);
    let args = [
        "ise",
        "replace",
        "--at",
        revision.as_str(),
        "--full",
        "--into",
        trial.as_str(),
        "--receipt",
        receipt.as_str(),
    ];
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, &args, &[]),
        None => common::run_givn_with_envs(env, &args, &[]),
    };
    world.last_output = Some(out);
}

#[then(regex = r#"^preparation fails and names "([^"]+)"$"#)]
fn preparation_fails_and_names(world: &mut GivnWorld, needle: String) {
    let out = output(world);
    let text = stderr(world);
    assert!(
        !out.status.success(),
        "the command succeeded; stderr={text:?}"
    );
    // A clap usage error exits 2; every refusal exits 1. The message class and
    // the exit code come from the same branch, so neither failure may borrow
    // the other's code.
    let expected = if text.starts_with("error:") { 2 } else { 1 };
    assert_eq!(out.status.code(), Some(expected), "stderr={text:?}");
    assert!(
        text.contains(&needle),
        "stderr {text:?} does not name {needle:?}"
    );
}

#[then(
    regex = r#"^preparation fails and names "givn/specification\.yaml" and the loader's reason$"#
)]
fn preparation_fails_and_names_the_loader_reason(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    assert!(
        text.contains("givn/specification.yaml"),
        "stderr {text:?} does not name the declaration"
    );
    assert!(
        text.contains("unknown key 'component'"),
        "stderr {text:?} does not carry the loader's reason"
    );
}

#[then(regex = r#"^preparation fails and names "givn/specification\.yaml" and the UTF-8 error$"#)]
fn preparation_fails_and_names_the_declaration_and_the_utf8_error(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    assert!(
        text.contains(ise_clean_room_support::DECLARATION_PATH),
        "stderr {text:?} does not name the declaration"
    );
    assert!(
        text.contains("invalid utf-8"),
        "stderr {text:?} does not carry the UTF-8 error"
    );
}

#[then(regex = r#"^the declaration is not read$"#)]
fn the_declaration_is_not_read(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        !text.contains("givn/specification.yaml"),
        "stderr {text:?} mentions the declaration, so it was read"
    );
}

#[then(regex = r#"^preparation fails and names the modified tracked file and the untracked file$"#)]
fn preparation_fails_and_names_every_dirty_path(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    for path in [
        ise_clean_room_support::MODIFIED_PATH,
        ise_clean_room_support::UNTRACKED_PATH,
    ] {
        assert!(
            text.contains(path),
            "stderr {text:?} does not name {path:?}"
        );
    }
}

#[then(regex = r#"^preparation fails and names the missing backend$"#)]
fn preparation_fails_and_names_the_missing_backend(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    assert!(
        text.contains("no Git worktree or Fossil checkout"),
        "stderr {text:?} does not name the missing backend"
    );
}

#[then(regex = r#"^preparation fails and names the nested project root$"#)]
fn preparation_fails_and_names_the_nested_project_root(world: &mut GivnWorld) {
    assert_exit_one(world);
    let root = world
        .ise_clean_room
        .nested_project_root
        .as_ref()
        .expect("the Given recorded the nested project root")
        .display()
        .to_string();
    let text = stderr(world);
    assert!(
        text.contains(&root),
        "stderr {text:?} does not name the nested project root {root:?}"
    );
}

#[then(regex = r#"^preparation fails and names the ambiguous version control$"#)]
fn preparation_fails_and_names_the_ambiguous_version_control(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    assert!(
        text.contains("ambiguous version control"),
        "stderr {text:?} does not name the ambiguous version control"
    );
}

#[then(regex = r#"^preparation fails and names the revision$"#)]
fn preparation_fails_and_names_the_revision(world: &mut GivnWorld) {
    assert_exit_one(world);
    let revision = world
        .ise_clean_room
        .requested_revision
        .as_ref()
        .expect("the When recorded the revision");
    let text = stderr(world);
    assert!(
        text.contains(revision),
        "stderr {text:?} does not name the revision {revision:?}"
    );
}

#[then(regex = r#"^the trial path and the receipt path remain absent$"#)]
fn the_trial_and_the_receipt_remain_absent(world: &mut GivnWorld) {
    let trial = world.ise_clean_room.trial_path();
    let receipt = world.ise_clean_room.receipt_path();
    assert!(
        !trial.exists(),
        "the trial path exists: {}",
        trial.display()
    );
    assert!(
        !receipt.exists(),
        "the receipt path exists: {}",
        receipt.display()
    );
}

#[given(
    regex = r#"^a Fossil checkout with ise enabled, allow-symlinks on, and a declaration with one source root "src/\*\*"$"#
)]
fn fossil_checkout_with_allow_symlinks(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::fossil_kind_project());
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "givn/specs/\*\*"$"#
)]
fn git_project_retaining_the_specs(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::retained_behavior_project());
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "nested/_FOSSIL_"$"#
)]
fn git_project_retaining_the_nested_marker(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::nested_marker_project());
}

#[given(regex = r#"^a Git project with ise enabled and a declaration that retains "vendor/lib"$"#)]
fn git_project_retaining_the_vendor_link(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::repository_link_project());
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "contracts" role names "docs/contract\.md"$"#
)]
fn git_project_with_a_contracts_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::contracts_project());
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration with one source root "src/\*\*"$"#
)]
fn git_project_with_source_only_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::source_root_project());
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration that retains "givn/specs/\*\*", and one source root "givn/\*\*"$"#
)]
fn git_project_retaining_the_specs_under_givn(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::behavior_declaration("\"givn/specs/**\"", "\"givn/**\""),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration that retains "givn/specs/\*\*", and one source root "src/\*\*"$"#
)]
fn git_project_with_retained_behavior(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::behavior_declaration("\"givn/specs/**\"", "\"src/**\""),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration that retains "src/keep\.rs", and one source root "src/\*\*"$"#
)]
fn git_project_retaining_a_source_file(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::structure_declaration("\"src/keep.rs\"", "\"src/**\""),
    ));
}

#[given(regex = r#"^a committed revision with "src/keep\.rs" and "src/other\.rs"$"#)]
fn commit_retained_and_replaced_source(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            (
                ise_clean_room_support::RETAINED_SOURCE_PATH,
                "pub fn keep() {}\n",
            ),
            (
                ise_clean_room_support::KEPT_SOURCE_PATH,
                "pub fn other() {}\n",
            ),
        ],
        "fixture: retained and replaced source files",
    );
}

#[given(
    regex = r#"^a committed revision where "src/keys/parser\.rs" sits beside "src/keys/mod\.rs" and "src/keys/tests\.rs"$"#
)]
fn commit_revision_with_three_keys_files(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("src/keys/parser.rs", "pub fn parse() {}\n"),
            ("src/keys/mod.rs", "pub mod parser;\n"),
            ("src/keys/tests.rs", "fn parses() {}\n"),
        ],
        "fixture: three keys files",
    );
}

#[given(regex = r#"^a committed revision whose tracked blob is absent from the object store$"#)]
fn commit_revision_without_its_blob(world: &mut GivnWorld) {
    let revision = ise_clean_room_support::remove_kept_blob(project_env(world));
    world.ise_clean_room.unreadable_revision = Some(revision);
}

#[given(regex = r#"^the committed declaration's blob is absent from the object store$"#)]
fn remove_the_committed_declaration_blob(world: &mut GivnWorld) {
    let revision = ise_clean_room_support::remove_declaration_blob(project_env(world));
    world.ise_clean_room.unreadable_revision = Some(revision);
}

#[given(regex = r#"^a committed revision with "givn/specs/demo\.feature"$"#)]
fn commit_revision_with_demo_feature(world: &mut GivnWorld) {
    ise_clean_room_support::commit_demo_feature(project_env(world));
}

#[given(regex = r#"^a committed revision with "nested/_FOSSIL_"$"#)]
fn commit_revision_with_the_nested_marker(world: &mut GivnWorld) {
    ise_clean_room_support::commit_nested_marker(project_env(world));
}

#[given(regex = r#"^a committed revision with "src/kept\.rs"$"#)]
fn commit_revision_with_kept_file(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/kept.rs\"", "\"src/**\""),
    );
    ise_clean_room_support::commit_files(
        env,
        &[(
            ise_clean_room_support::KEPT_PATH,
            ise_clean_room_support::KEPT_CONTENT,
        )],
        "fixture: kept file",
    );
}

#[given(
    regex = r#"^a committed revision with "src/kept\.rs" and a later revision adding "src/later\.rs"$"#
)]
fn commit_two_revisions(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/kept.rs\"", "\"src/**\""),
    );
    let first = ise_clean_room_support::commit_files(
        env,
        &[(
            ise_clean_room_support::KEPT_PATH,
            ise_clean_room_support::KEPT_CONTENT,
        )],
        "fixture: first revision",
    );
    ise_clean_room_support::commit_files(
        env,
        &[("src/later.rs", "pub fn later() {}\n")],
        "fixture: later revision",
    );
    world.ise_clean_room.first_revision = Some(first);
}

#[given(regex = r#"^a committed revision with "src/keys/parser\.rs"$"#)]
fn commit_revision_with_parser(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[("src/keys/parser.rs", "pub fn parse() {}\n")],
        "fixture: parser file",
    );
}

#[given(
    regex = r#"^a committed revision with "src/keys/parser\.rs", "src/keys/mod\.rs", and "src/other\.rs"$"#
)]
fn commit_revision_with_keys_and_other(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("src/keys/parser.rs", "pub fn parse() {}\n"),
            ("src/keys/mod.rs", "pub mod parser;\n"),
            ("src/other.rs", "pub fn other() {}\n"),
        ],
        "fixture: keys and other files",
    );
}

#[given(regex = r#"^a committed revision with "src/one\.rs" and "src/nested/two\.rs"$"#)]
fn commit_revision_with_nested_source(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("src/one.rs", "pub fn one() {}\n"),
            ("src/nested/two.rs", "pub fn two() {}\n"),
        ],
        "fixture: nested source files",
    );
}

#[given(regex = r#"^a committed revision with "src/one\.rs" and "src/two\.rs"$"#)]
fn commit_revision_with_two_source_files(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("src/one.rs", "pub fn one() {}\n"),
            ("src/two.rs", "pub fn two() {}\n"),
        ],
        "fixture: two source files",
    );
}

#[given(
    regex = r#"^a committed revision with "src/one\.rs", "src/nested/two\.rs", and "givn/specs/demo\.feature"$"#
)]
fn commit_revision_with_source_and_retained(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("src/one.rs", "pub fn one() {}\n"),
            ("src/nested/two.rs", "pub fn two() {}\n"),
            (
                "givn/specs/demo.feature",
                "Feature: Demo\n\n  Scenario: A demo step\n    Given a step\n",
            ),
        ],
        "fixture: source and retained files",
    );
}

#[given(regex = r#"^a committed revision with a repository link at "vendor/lib"$"#)]
fn commit_revision_with_the_repository_link(world: &mut GivnWorld) {
    ise_clean_room_support::commit_repository_link(project_env(world));
}

#[given(
    regex = r#"^a committed revision with a retained file, a kept source file, a replaced source file, an executable script, and a symlink to it$"#
)]
fn commit_fossil_kinds(world: &mut GivnWorld) {
    ise_clean_room_support::commit_fossil_kinds(project_env(world));
}

#[given(
    regex = r#"^a committed revision with an executable script "src/run\.sh" and a symlink "src/link" to "run\.sh"$"#
)]
fn commit_executable_and_symlink(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration(
            "\"src/run.sh\", \"src/link\"",
            "\"src/**\"",
        ),
    );
    ise_clean_room_support::commit_executable_and_symlink(env);
}

#[given(regex = r#"^a committed revision without "docs/contract\.md"$"#)]
fn commit_revision_without_the_contract(world: &mut GivnWorld) {
    ise_clean_room_support::commit_revision_without_contract(project_env(world));
}

#[given(regex = r#"^a receipt file that already exists$"#)]
fn a_receipt_file_that_already_exists(world: &mut GivnWorld) {
    ise_clean_room_support::existing_receipt(&mut world.ise_clean_room);
}

#[given(regex = r#"^a receipt path whose parent directory is a regular file$"#)]
fn a_receipt_path_whose_parent_is_a_file(world: &mut GivnWorld) {
    ise_clean_room_support::blocked_receipt(&mut world.ise_clean_room);
}

#[given(regex = r#"^a symlink outside the checkout that points into the checkout$"#)]
fn a_symlink_pointing_into_the_checkout(world: &mut GivnWorld) {
    let checkout = project_env(world).root();
    ise_clean_room_support::symlinked_target(&mut world.ise_clean_room, &checkout);
}

#[given(regex = r#"^a target directory that already exists$"#)]
fn a_target_directory_that_already_exists(world: &mut GivnWorld) {
    ise_clean_room_support::existing_target_directory(&mut world.ise_clean_room);
}

#[given(regex = r#"^a target path that exists as a regular file$"#)]
fn a_target_path_that_exists_as_a_regular_file(world: &mut GivnWorld) {
    ise_clean_room_support::existing_target_file(&mut world.ise_clean_room);
}

#[given(regex = r#"^an ignored file in the working tree$"#)]
fn an_ignored_file_in_the_working_tree(world: &mut GivnWorld) {
    ise_clean_room_support::ignore_note(project_env(world));
}

#[then(regex = r#"^no record file is written$"#)]
fn no_record_file_is_written(world: &mut GivnWorld) {
    let receipt = receipt_path(world);
    assert!(
        !receipt.exists(),
        "a record file exists at {}",
        receipt.display()
    );
}

#[then(regex = r#"^preparation fails and names "--full" and "--path"$"#)]
fn preparation_fails_and_names_the_two_forms(world: &mut GivnWorld) {
    assert_exit_two(world);
    let text = stderr(world);
    for flag in ["--full", "--path"] {
        assert!(
            text.contains(flag),
            "stderr {text:?} does not name {flag:?}"
        );
    }
}

#[then(regex = r#"^preparation fails and names "--full" and "--path" as mutually exclusive$"#)]
fn preparation_fails_and_names_the_conflicting_forms(world: &mut GivnWorld) {
    assert_exit_two(world);
    let text = stderr(world);
    for needle in ["--full", "--path", "cannot be used with"] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
}

#[then(regex = r#"^preparation fails and names "docs/contract\.md" and the revision$"#)]
fn preparation_fails_and_names_the_contract_and_revision(world: &mut GivnWorld) {
    assert_exit_one(world);
    let revision = ise_clean_room_support::head_revision(project_env(world));
    let text = stderr(world);
    for needle in [ise_clean_room_support::CONTRACT_PATH, revision.as_str()] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not name {needle:?}"
        );
    }
}

#[then(
    regex = r#"^preparation fails and names "givn/specs/demo\.feature" and the "behavior" role$"#
)]
fn preparation_fails_and_names_the_retained_feature(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    for needle in [ise_clean_room_support::DEMO_FEATURE_PATH, "behavior"] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not name {needle:?}"
        );
    }
}

#[then(regex = r#"^preparation fails and names the existing receipt$"#)]
fn preparation_fails_and_names_the_existing_receipt(world: &mut GivnWorld) {
    assert_failure_refuses_existing(world, &receipt_path(world));
}

#[then(regex = r#"^preparation fails and names the existing target$"#)]
fn preparation_fails_and_names_the_existing_target(world: &mut GivnWorld) {
    assert_failure_refuses_existing(world, &trial_path(world));
}

#[then(regex = r#"^preparation fails and names the receipt$"#)]
fn preparation_fails_and_names_the_receipt(world: &mut GivnWorld) {
    assert_failure_names_path(world, &receipt_path(world));
}

#[then(regex = r#"^preparation fails and names the receipt path$"#)]
fn preparation_fails_and_names_the_record_path(world: &mut GivnWorld) {
    assert_failure_names_path(world, &receipt_path(world));
}

#[then(regex = r#"^preparation fails and names the target$"#)]
fn preparation_fails_and_names_the_target(world: &mut GivnWorld) {
    assert_failure_names_path(world, &trial_path(world));
}

#[then(regex = r#"^preparation fails and names the unreadable path and the revision$"#)]
fn preparation_fails_and_names_the_unreadable_blob(world: &mut GivnWorld) {
    assert_exit_one(world);
    let revision = world
        .ise_clean_room
        .unreadable_revision
        .clone()
        .expect("the Given recorded the unreadable revision");
    let text = stderr(world);
    assert!(
        text.contains(ise_clean_room_support::KEPT_PATH),
        "stderr {text:?} does not name the unreadable path"
    );
    assert!(
        text.contains(&revision),
        "stderr {text:?} does not name the revision {revision:?}"
    );
}

#[then(regex = r#"^preparation succeeds$"#)]
fn preparation_succeeds(world: &mut GivnWorld) {
    assert_success(world);
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 1" and "Not replaced: 1"$"#)]
fn preparation_succeeds_with_a_retained_sibling(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Replaced: 1");
    assert_report_line(world, "Not replaced: 1");
}

#[then(regex = r#"^preparation succeeds and prints "Backend: fossil"$"#)]
fn preparation_succeeds_with_the_fossil_backend(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Backend: fossil");
}

#[then(regex = r#"^preparation succeeds and prints "Not replaced: 2"$"#)]
fn preparation_succeeds_with_two_siblings_left(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Not replaced: 2");
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 0"$"#)]
fn preparation_succeeds_with_nothing_replaced(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Replaced: 0");
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 1"$"#)]
fn preparation_succeeds_with_one_replacement(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Replaced: 1");
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 2" and "Not replaced: 0"$"#)]
fn preparation_succeeds_with_two_replacements(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Replaced: 2");
    assert_report_line(world, "Not replaced: 0");
}

#[then(
    regex = r#"^preparation succeeds and prints "Revision:" with the first revision identifier$"#
)]
fn preparation_succeeds_and_reports_the_first_revision(world: &mut GivnWorld) {
    assert_success(world);
    let first = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    assert_report_line(world, &format!("Revision: {first}"));
}

#[then(regex = r#"^preparation succeeds and prints the trial record to stdout$"#)]
fn preparation_prints_the_record_to_stdout(world: &mut GivnWorld) {
    assert_success(world);
    let text = stdout(world);
    for line in ["schema: 1", "content_scope: ise-trial/v1"] {
        assert!(
            text.lines().any(|printed| printed == line),
            "stdout {text:?} does not carry the record line {line:?}"
        );
    }
}

/// The `Would write:` entries the default fixture's `--full` trial holds: its
/// four retained files and the declaration every trial retains, with no
/// tracked source file kept and the frozen e2e package excluded.
const DEFAULT_TRIAL_PATHS: [&str; 5] = [
    "docs/arc42/02-architecture-constraints.md",
    "docs/glossary.md",
    "docs/reference.md",
    "givn/specification.yaml",
    "givn/specs/demo.feature",
];

#[then(regex = r#"^the "Would write:" block lists the sorted paths the trial would hold$"#)]
fn would_write_block_lists_the_sorted_paths(world: &mut GivnWorld) {
    let entries = would_write_entries(world);
    let expected: Vec<String> = DEFAULT_TRIAL_PATHS
        .iter()
        .map(|path| path.to_string())
        .collect();
    assert_eq!(
        entries, expected,
        "the Would write block does not list the sorted trial paths"
    );
}

/// The paths in one run's `Would write:` block, in printed order.
fn would_write_entries(world: &GivnWorld) -> Vec<String> {
    let text = stdout(world);
    let mut entries: Vec<String> = Vec::new();
    let mut in_block = false;
    for line in text.lines() {
        if line == "Would write:" {
            in_block = true;
            continue;
        }
        if !in_block {
            continue;
        }
        let Some(path) = line.strip_prefix("  ") else {
            break;
        };
        entries.push(path.to_string());
    }
    assert!(in_block, "stdout {text:?} has no 'Would write:' block");
    entries
}

#[then(regex = r#"^the checkout holds no receipt$"#)]
fn the_checkout_holds_no_receipt(world: &mut GivnWorld) {
    let receipt = project_env(world).path("record.yaml");
    assert!(
        !receipt.exists(),
        "the checkout holds a receipt at {}",
        receipt.display()
    );
}

#[then(regex = r#"^the checkout holds no trial directory$"#)]
fn the_checkout_holds_no_trial_directory(world: &mut GivnWorld) {
    let trial = project_env(world).path("trial");
    assert!(
        !trial.exists(),
        "the checkout holds a trial directory at {}",
        trial.display()
    );
}

#[then(regex = r#"^the checkout holds no trial directory and no receipt$"#)]
fn the_checkout_holds_no_trial_directory_and_no_receipt(world: &mut GivnWorld) {
    let trial = project_env(world).path("trial");
    let receipt = project_env(world).path("record.yaml");
    assert!(
        !trial.exists(),
        "the checkout holds a trial directory at {}",
        trial.display()
    );
    assert!(
        !receipt.exists(),
        "the checkout holds a receipt at {}",
        receipt.display()
    );
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and no receipt$"#
)]
fn prepare_without_a_receipt(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    prepare_with_target(world, false);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and no target$"#
)]
fn prepare_without_a_target(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some("HEAD".to_string());
    let args = ["ise", "replace", "--at", "HEAD", "--full"];
    let env = project_env(world);
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, &args, &[]),
        None => common::run_givn_with_envs(env, &args, &[]),
    };
    world.last_output = Some(out);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and that receipt$"#
)]
fn prepare_with_the_named_receipt(world: &mut GivnWorld) {
    prepare_with_target(world, true);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with both a full replacement and a path$"#
)]
fn prepare_with_both_replacement_forms(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--full", "--path", "src/one.rs"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with neither a full replacement nor a path$"#
)]
fn prepare_without_a_replacement_form(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &[]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the directory path "givn/specs"$"#
)]
fn prepare_with_the_specs_directory(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "givn/specs"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the directory path "src/keys"$"#
)]
fn prepare_with_the_keys_directory(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "src/keys"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "docs/reference\.md"$"#
)]
fn prepare_with_the_reference_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "docs/reference.md"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "givn/specs/demo\.feature"$"#
)]
fn prepare_with_the_demo_feature_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "givn/specs/demo.feature"]);
}

#[when(regex = r#"^the developer prepares a trial from "HEAD" with the path "src/\*\*"$"#)]
fn prepare_with_a_glob_shaped_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "src/**"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parser\.rs"$"#
)]
fn prepare_with_the_parser_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "src/keys/parser.rs"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parsr\.rs"$"#
)]
fn prepare_with_an_absent_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "src/keys/parsr.rs"]);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the paths "src/one\.rs" and "src/one\.rs"$"#
)]
fn prepare_with_a_repeated_path(world: &mut GivnWorld) {
    prepare_trial(
        world,
        Some("HEAD"),
        &["--path", "src/one.rs", "--path", "src/one.rs"],
    );
}

#[when(
    regex = r#"^the developer prepares a trial from the first revision with a full replacement$"#
)]
fn prepare_from_the_first_revision(world: &mut GivnWorld) {
    let first = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    prepare_trial(world, Some(&first), &["--full"]);
}

#[when(regex = r#"^the developer prepares a trial into a directory inside the checkout$"#)]
fn prepare_into_the_checkout(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let checkout = project_env(world).root().to_path_buf();
    world.ise_clean_room.trial_path = Some(checkout.join("trial"));
    world.ise_clean_room.receipt_path = Some(checkout.join("record.yaml"));
    prepare_with_target(world, false);
}

#[when(regex = r#"^the developer prepares a trial into that target$"#)]
fn prepare_into_the_recorded_target(world: &mut GivnWorld) {
    prepare_with_target(world, false);
}

#[when(regex = r#"^the developer prepares a trial through the symlinked target$"#)]
fn prepare_through_the_symlinked_target(world: &mut GivnWorld) {
    prepare_with_target(world, false);
}

#[when(
    regex = r#"^the developer prepares a trial whose receipt path is an ancestor of the trial directory$"#
)]
fn prepare_with_an_ancestor_receipt(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let container = world
        .ise_clean_room
        .targets
        .as_ref()
        .expect("the targets exist")
        .path()
        .to_path_buf();
    let parent = container.join("parent");
    world.ise_clean_room.trial_path = Some(parent.join("trial"));
    world.ise_clean_room.receipt_path = Some(parent);
    prepare_with_target(world, true);
}

#[when(regex = r#"^the developer prepares a trial with a receipt inside the checkout$"#)]
fn prepare_with_a_receipt_inside_the_checkout(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let checkout = project_env(world).root().to_path_buf();
    world.ise_clean_room.receipt_path = Some(checkout.join("record.yaml"));
    prepare_with_target(world, true);
}

#[when(regex = r#"^the developer prepares a trial with a receipt inside the trial directory$"#)]
fn prepare_with_a_receipt_inside_the_trial(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world);
    world.ise_clean_room.receipt_path = Some(trial.join("record.yaml"));
    prepare_with_target(world, true);
}

#[when(regex = r#"^the developer prepares a trial with that receipt$"#)]
fn prepare_with_that_receipt(world: &mut GivnWorld) {
    prepare_with_target(world, true);
}

#[when(
    regex = r#"^the developer runs a dry run from "HEAD" with a full replacement and a receipt$"#
)]
fn run_a_dry_run(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    let receipt = receipt_path(world).to_string_lossy().into_owned();
    let args = [
        "ise",
        "replace",
        "--at",
        "HEAD",
        "--full",
        "--into",
        trial.as_str(),
        "--receipt",
        receipt.as_str(),
        "--dry-run",
    ];
    let env = project_env(world);
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, &args, &[]),
        None => common::run_givn_with_envs(env, &args, &[]),
    };
    world.last_output = Some(out);
}

/// The ten fixed report lines, in order, plus the named receipt line.
const FIXED_REPORT_LINES: [&str; 10] = [
    "Backend:",
    "Revision:",
    "Retained:",
    "Kept source:",
    "Replaced:",
    "Outside the declaration:",
    "Not replaced:",
    "Retained digest:",
    "E2E digest:",
    "Written path:",
];

#[then(
    regex = r#"^the dry run prints the fixed report lines, the receipt path line, the frozen line, and a "Would write:" block$"#
)]
fn dry_run_prints_the_fixed_lines(world: &mut GivnWorld) {
    assert_success(world);
    let text = stdout(world);
    for prefix in FIXED_REPORT_LINES {
        assert!(
            text.lines().any(|line| line.starts_with(prefix)),
            "stdout {text:?} does not print the {prefix:?} line"
        );
    }
    let receipt = receipt_path(world);
    assert_report_line(world, &format!("Receipt path: {}", receipt.display()));
    assert_report_line(world, "Frozen package: e2e, not copied (1 file)");
    assert_report_line(world, "Would write:");
}

#[then(regex = r#"^the existing directory is left unchanged$"#)]
fn the_existing_directory_is_left_unchanged(world: &mut GivnWorld) {
    let trial = trial_path(world);
    assert!(
        trial.is_dir(),
        "the existing target directory {} is gone",
        trial.display()
    );
    let mut entries = std::fs::read_dir(&trial).expect("read the existing target directory");
    assert!(
        entries.next().is_none(),
        "the existing target directory {} gained an entry",
        trial.display()
    );
}

#[then(regex = r#"^the existing file is left unchanged$"#)]
fn the_existing_file_is_left_unchanged(world: &mut GivnWorld) {
    let target = trial_path(world);
    let bytes = std::fs::read(&target).expect("read the existing target file");
    assert_eq!(
        bytes,
        ise_clean_room_support::TARGET_FILE_CONTENT.as_bytes(),
        "the existing target file {} changed",
        target.display()
    );
}

#[then(regex = r#"^the existing receipt is left unchanged$"#)]
fn the_existing_receipt_is_left_unchanged(world: &mut GivnWorld) {
    let receipt = receipt_path(world);
    let bytes = std::fs::read(&receipt).expect("read the existing receipt");
    assert_eq!(
        bytes,
        ise_clean_room_support::RECEIPT_CONTENT.as_bytes(),
        "the existing receipt {} changed",
        receipt.display()
    );
    let trial = trial_path(world);
    assert!(
        !trial.exists(),
        "the trial path exists: {}",
        trial.display()
    );
}

#[then(regex = r#"^the receipt records one named path and one replaced path$"#)]
fn receipt_records_one_named_and_one_replaced_path(world: &mut GivnWorld) {
    let record = record_document(world);
    let named = record_list(&record, "named_paths");
    assert_eq!(
        named,
        vec!["src/one.rs".to_string()],
        "named_paths={named:?}"
    );
    let replaced = record_list(&record, "replaced_paths");
    assert_eq!(
        replaced,
        vec!["src/one.rs".to_string()],
        "replaced_paths={replaced:?}"
    );
    assert_eq!(record_number(&record, "replaced"), 1);
}

#[then(regex = r#"^the report holds no "Receipt path" line$"#)]
fn report_holds_no_receipt_path_line(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        !text.lines().any(|line| line.starts_with("Receipt path:")),
        "stdout {text:?} holds a Receipt path line"
    );
}

#[then(regex = r#"^the trial directory it created is removed$"#)]
fn the_created_trial_is_removed(world: &mut GivnWorld) {
    let trial = trial_path(world);
    assert!(
        !trial.exists(),
        "the trial directory {} was left behind",
        trial.display()
    );
}

#[then(regex = r#"^the trial does not hold "src/later\.rs"$"#)]
fn the_trial_does_not_hold_the_later_file(world: &mut GivnWorld) {
    assert_trial_omits(world, &["src/later.rs"]);
}

#[then(regex = r#"^the trial does not hold the ignored file$"#)]
fn the_trial_does_not_hold_the_ignored_file(world: &mut GivnWorld) {
    assert_trial_omits(world, &[ise_clean_room_support::IGNORED_PATH]);
}

#[then(regex = r#"^the trial holds "([^"]+)"$"#)]
fn the_trial_holds(world: &mut GivnWorld, path: String) {
    assert_trial_holds(world, &[path.as_str()]);
}

#[then(regex = r#"^the trial holds "src/keys/mod\.rs" and "src/keys/tests\.rs"$"#)]
fn the_trial_holds_the_keys_siblings(world: &mut GivnWorld) {
    assert_trial_holds(world, &["src/keys/mod.rs", "src/keys/tests.rs"]);
}

#[then(regex = r#"^the trial holds "src/one\.rs" and "src/nested/two\.rs"$"#)]
fn the_trial_holds_the_nested_source(world: &mut GivnWorld) {
    assert_trial_holds(world, &["src/one.rs", "src/nested/two.rs"]);
}

#[then(regex = r#"^the trial holds no "\.git" entry and no "\.fslckout" entry$"#)]
fn trial_holds_no_version_control_metadata(world: &mut GivnWorld) {
    assert_trial_has_no_vcs_entries(world);
}

#[then(regex = r#"^the trial holds no file under "src"$"#)]
fn the_trial_holds_no_source_file(world: &mut GivnWorld) {
    let trial = trial_path(world);
    let mut directories = vec![trial.clone()];
    while let Some(directory) = directories.pop() {
        let entries = std::fs::read_dir(&directory).expect("read the trial directory");
        for entry in entries {
            let path = entry.expect("read the trial entry").path();
            if path.is_dir() {
                directories.push(path);
            } else {
                let relative = path
                    .strip_prefix(&trial)
                    .expect("the entry is under the trial")
                    .to_string_lossy()
                    .replace('\\', "/");
                assert!(!relative.starts_with("src/"), "the trial holds {relative}");
            }
        }
    }
}

#[then(regex = r#"^the trial holds the retained file and the kept source file$"#)]
fn trial_holds_the_fossil_retained_and_kept_files(world: &mut GivnWorld) {
    assert_trial_holds(
        world,
        &[
            ise_clean_room_support::FOSSIL_FEATURE_PATH,
            ise_clean_room_support::KEPT_SOURCE_PATH,
        ],
    );
}

#[then(regex = r#"^the trial omits "src/keys/parser\.rs" and "src/keys/mod\.rs"$"#)]
fn the_trial_omits_the_replaced_keys_files(world: &mut GivnWorld) {
    assert_trial_omits(world, &["src/keys/parser.rs", "src/keys/mod.rs"]);
}

#[then(regex = r#"^the trial omits the replaced source file$"#)]
fn trial_omits_the_fossil_replaced_source(world: &mut GivnWorld) {
    assert_trial_omits(world, &[ise_clean_room_support::REPLACED_SOURCE_PATH]);
}

#[then(regex = r#"^the trial's "src/link" is a symlink to "run\.sh"$"#)]
fn trial_symlink_points_at_the_script(world: &mut GivnWorld) {
    assert_trial_symlink(
        world,
        ise_clean_room_support::SYMLINK_PATH,
        ise_clean_room_support::SYMLINK_TARGET,
    );
}

#[then(regex = r#"^the trial's "src/run\.sh" is executable$"#)]
fn trial_script_is_executable(world: &mut GivnWorld) {
    assert_trial_executable(world, ise_clean_room_support::RUN_SCRIPT_PATH);
}

#[then(regex = r#"^the trial's executable script is executable$"#)]
fn trial_fossil_script_is_executable(world: &mut GivnWorld) {
    assert_trial_executable(world, ise_clean_room_support::RUN_SCRIPT_PATH);
}

#[then(regex = r#"^the trial's symlink is a symlink to its target$"#)]
fn trial_fossil_symlink_points_at_its_target(world: &mut GivnWorld) {
    assert_trial_symlink(
        world,
        ise_clean_room_support::SYMLINK_PATH,
        ise_clean_room_support::SYMLINK_TARGET,
    );
}

// --- retain-declaration-in-trial: declaration-retained-partition ----------

#[given(regex = r#"^a Git project with ise enabled and a declaration that sources "givn/\*\*"$"#)]
fn git_project_sourcing_givn(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::source_only_declaration("\"givn/**\""),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "contracts" role names "givn/\*\*"$"#
)]
fn git_project_with_a_wildcard_contract_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::contracts_wildcard_project());
}

#[given(regex = r#"^a committed revision with "givn/notes\.md"$"#)]
fn commit_revision_with_notes(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[("givn/notes.md", "notes\n")],
        "fixture: notes",
    );
}

#[given(regex = r#"^a committed revision with "givn/specs/a\.md"$"#)]
fn commit_revision_with_a_spec_document(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[("givn/specs/a.md", "spec\n")],
        "fixture: spec document",
    );
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and a receipt$"#
)]
fn developer_prepares_a_full_trial_with_a_receipt(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    prepare_with_target(world, true);
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "givn/specification\.yaml"$"#
)]
fn prepare_with_the_declaration_path(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "givn/specification.yaml"]);
}

#[when(regex = r#"^the developer prepares a trial from "HEAD" with the directory path "givn"$"#)]
fn prepare_with_the_givn_directory(world: &mut GivnWorld) {
    prepare_trial(world, Some("HEAD"), &["--path", "givn"]);
}

#[then(regex = r#"^preparation fails and names "givn/specification\.yaml" as retained$"#)]
fn preparation_fails_and_names_the_declaration_retained(world: &mut GivnWorld) {
    assert_exit_one(world);
    let text = stderr(world);
    for needle in [
        "'givn/specification.yaml' is retained by the Specification declaration",
        "a retained path is never replaced",
    ] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 3"$"#)]
fn preparation_succeeds_with_three_replacements(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Replaced: 3");
}

#[then(regex = r#"^preparation succeeds and prints "Retained: 2"$"#)]
fn preparation_succeeds_with_two_retained_entries(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Retained: 2");
}

#[then(regex = r#"^the receipt lists "givn/specification\.yaml" in its retained set$"#)]
fn receipt_lists_the_declaration_in_its_retained_set(world: &mut GivnWorld) {
    let record = record_document(world);
    let retained = record
        .get("retained_files")
        .and_then(serde_yaml_ng::Value::as_mapping)
        .unwrap_or_else(|| panic!("the record has no 'retained_files' mapping"));
    let digest = retained
        .get("givn/specification.yaml")
        .and_then(serde_yaml_ng::Value::as_str)
        .unwrap_or_else(|| {
            panic!("the record does not list givn/specification.yaml in retained_files")
        });
    assert_eq!(
        digest.len(),
        64,
        "the declaration's recorded identity is not a SHA-256 value"
    );
    assert!(
        digest
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase()),
        "the declaration's recorded identity is not lowercase hex"
    );
    assert_eq!(
        record_number(&record, "retained") as usize,
        retained.len(),
        "counts.retained does not equal the recorded retained entries"
    );
}

#[then(regex = r#"^the trial omits "([^"]+)"$"#)]
fn the_trial_omits(world: &mut GivnWorld, path: String) {
    assert_trial_omits(world, &[path.as_str()]);
}

// --- retain-declaration-in-trial: revision-declaration-authority ----------

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "structure" role names "docs/glossary\.md"$"#
)]
fn git_project_with_a_glossary_structure_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::structure_declaration("\"docs/glossary.md\"", "\"src/**\""),
    ));
}

#[given(regex = r#"^a committed revision with "docs/glossary\.md" and "src/one\.rs"$"#)]
fn commit_revision_with_a_glossary_and_a_source_file(world: &mut GivnWorld) {
    let revision = ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("docs/glossary.md", "glossary\n"),
            ("src/one.rs", "pub fn one() {}\n"),
        ],
        "fixture: glossary and source",
    );
    world.ise_clean_room.first_revision = Some(revision);
}

#[given(
    regex = r#"^a later revision that rewrites the declaration without the "structure" entry$"#
)]
fn commit_a_later_declaration_without_the_structure_entry(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::source_only_declaration("\"src/**\""),
    );
    common::git_commit_all(env, "fixture: declaration without the structure entry");
}

#[given(
    regex = r#"^a Git project with ise enabled whose first revision predates the declaration$"#
)]
fn git_project_whose_first_revision_predates_the_declaration(world: &mut GivnWorld) {
    let (env, first) = ise_clean_room_support::declaration_late_project();
    world.ise_clean_room.first_revision = Some(first);
    world.env = Some(env);
}

#[given(
    regex = r#"^a Git project with ise enabled, a malformed declaration in its first revision, and a valid declaration at "HEAD"$"#
)]
fn git_project_with_a_malformed_first_revision(world: &mut GivnWorld) {
    let (env, first) = ise_clean_room_support::malformed_first_revision_project();
    world.ise_clean_room.first_revision = Some(first);
    world.env = Some(env);
}

#[given(
    regex = r#"^a Git project with ise enabled, a non-UTF-8 declaration in its first revision, and a valid declaration at "HEAD"$"#
)]
fn git_project_with_a_non_utf8_first_revision(world: &mut GivnWorld) {
    let (env, first) = ise_clean_room_support::non_utf8_first_revision_project();
    world.ise_clean_room.first_revision = Some(first);
    world.env = Some(env);
}

#[then(regex = r#"^preparation fails and names "givn/specification\.yaml" and the revision$"#)]
fn preparation_fails_and_names_the_declaration_and_the_revision(world: &mut GivnWorld) {
    assert_exit_one(world);
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    let text = stderr(world);
    for needle in [
        ise_clean_room_support::DECLARATION_PATH,
        revision.as_str(),
        "does not record",
    ] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
}

#[then(
    regex = r#"^the trial's "givn/specification\.yaml" is the declaration committed at the selected revision$"#
)]
fn trial_declaration_is_the_selected_revisions_copy(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .requested_revision
        .clone()
        .expect("the When recorded the selected revision");
    let expected = ise_clean_room_support::revision_content(
        project_env(world),
        &revision,
        ise_clean_room_support::DECLARATION_PATH,
    );
    let actual = std::fs::read(trial_path(world).join(ise_clean_room_support::DECLARATION_PATH))
        .unwrap_or_else(|error| panic!("read the trial declaration: {error}"));
    assert_eq!(
        actual, expected,
        "the trial's declaration is not the copy recorded at revision {revision}"
    );
}

// --- keep-frozen-verifier-out-of-the-trial: frozen-package-exclusion -------

#[given(
    regex = r#"^a committed revision with "givn/specs/demo\.feature", "tests/e2e/Cargo\.toml", "tests/e2e/steps/demo\.rs", and "src/kept\.rs"$"#
)]
fn commit_revision_with_the_e2e_package(world: &mut GivnWorld) {
    common::git_commit_all(project_env(world), "fixture: declaration");
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            (
                ise_clean_room_support::E2E_FEATURE_PATH,
                ise_clean_room_support::E2E_FEATURE_CONTENT,
            ),
            ("tests/e2e/Cargo.toml", "[package]\nname = \"e2e\"\n"),
            ("tests/e2e/steps/demo.rs", "// e2e steps\n"),
            (
                ise_clean_room_support::KEPT_PATH,
                ise_clean_room_support::KEPT_CONTENT,
            ),
        ],
        "fixture: e2e package and kept source",
    );
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "e2e" role names "givn/\*\*"$"#
)]
fn git_project_whose_e2e_role_names_givn(world: &mut GivnWorld) {
    let env = ise_clean_room_support::e2e_wildcard_project();
    common::git_commit_all(&env, "fixture: e2e wildcard declaration");
    world.env = Some(env);
}

#[given(regex = r#"^a committed revision with "givn/specs/a\.md" and "src/kept\.rs"$"#)]
fn commit_revision_with_a_spec_document_and_kept_source(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("givn/specs/a.md", "spec\n"),
            (
                ise_clean_room_support::KEPT_PATH,
                ise_clean_room_support::KEPT_CONTENT,
            ),
        ],
        "fixture: spec document and kept source",
    );
}

#[then(regex = r#"^the receipt's excluded roles hold "([^"]+)"$"#)]
fn receipt_excluded_roles_hold(world: &mut GivnWorld, role: String) {
    let record = record_document(world);
    assert_eq!(
        record_list(&record, "excluded_roles"),
        vec![role],
        "the receipt names another excluded role set"
    );
}

#[then(regex = r#"^the receipt's excluded roles are empty$"#)]
fn receipt_excluded_roles_are_empty(world: &mut GivnWorld) {
    let record = record_document(world);
    assert!(
        record_list(&record, "excluded_roles").is_empty(),
        "the receipt names an excluded role while nothing is frozen"
    );
}

#[then(regex = r#"^the receipt's retained set holds no path under "([^"]+)"$"#)]
fn receipt_retained_set_holds_no_path_under(world: &mut GivnWorld, directory: String) {
    let record = record_document(world);
    let retained = record
        .get("retained_files")
        .and_then(serde_yaml_ng::Value::as_mapping)
        .unwrap_or_else(|| panic!("the record has no 'retained_files' mapping"));
    let prefix = format!("{directory}/");
    for key in retained.keys() {
        let path = key
            .as_str()
            .unwrap_or_else(|| panic!("a retained_files key is not a string"));
        assert!(
            path != directory && !path.starts_with(&prefix),
            "the receipt's retained set holds {path}"
        );
    }
}

#[then(regex = r#"^the receipt holds one e2e identity per resolved package path$"#)]
fn receipt_holds_one_e2e_identity_per_package_path(world: &mut GivnWorld) {
    let record = record_document(world);
    let identities = record
        .get("e2e_files")
        .and_then(serde_yaml_ng::Value::as_mapping)
        .unwrap_or_else(|| panic!("the record has no 'e2e_files' mapping"));
    let mut paths: Vec<&str> = identities.keys().filter_map(|key| key.as_str()).collect();
    paths.sort();
    assert_eq!(
        paths,
        vec!["tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs"],
        "the record does not hold one identity per resolved package path"
    );
    for (path, digest) in identities {
        let digest = digest
            .as_str()
            .unwrap_or_else(|| panic!("the e2e identity for {path:?} is not a string"));
        assert_eq!(
            digest.len(),
            64,
            "the e2e identity for {path:?} is not a SHA-256 value"
        );
    }
}

#[then(regex = r#"^the report holds no "Frozen package" line$"#)]
fn report_holds_no_frozen_package_line(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        !text.lines().any(|line| line.starts_with("Frozen package:")),
        "stdout {text:?} prints a Frozen package line while nothing is frozen"
    );
}

#[then(
    regex = r#"^preparation succeeds and prints "Retained: 1" and "Frozen package: e2e, not copied \(1 file\)"$"#
)]
fn preparation_succeeds_with_one_retained_and_one_frozen(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Retained: 1");
    assert_report_line(world, "Frozen package: e2e, not copied (1 file)");
}

#[then(regex = r#"^the "Would write:" block lists no path under "([^"]+)"$"#)]
fn would_write_block_lists_no_path_under(world: &mut GivnWorld, directory: String) {
    let entries = would_write_entries(world);
    let prefix = format!("{directory}/");
    for path in &entries {
        assert!(
            path != &directory && !path.starts_with(&prefix),
            "the Would write block lists {path}"
        );
    }
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "givn/specs/\*\*", declares "tests/e2e", and replaces "src/\*\*"$"#
)]
fn git_project_with_the_e2e_declaration(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::e2e_trial_project());
}

// --- command-surface: rebuild-all and rebuild-component --------------------

/// The manifest target the fixture plans; the writer lands with the manifest
/// slice.
fn manifest_path(world: &GivnWorld) -> std::path::PathBuf {
    world.ise_clean_room.receipt_path().to_path_buf()
}

/// The refusal prefix of the command that last ran.
fn assert_refusal_prefix(world: &GivnWorld) {
    let verb = world
        .ise_clean_room
        .last_verb
        .expect("the When recorded the command verb");
    let text = stderr(world);
    let prefix = format!("{verb}: ");
    assert!(
        text.starts_with(&prefix),
        "stderr {text:?} does not start with the {prefix:?} prefix"
    );
}

/// Assert exit 1, the verb's refusal prefix, and every named token in stderr.
fn assert_refusal_names(world: &GivnWorld, needles: &[&str]) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    for needle in needles {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
}

/// Run one rebuild command against the planned targets and record its verb.
fn run_rebuild(world: &mut GivnWorld, args: &[&str], verb: &'static str) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.last_verb = Some(verb);
    let env = project_env(world);
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, args, &[]),
        None => common::run_givn_with_envs(env, args, &[]),
    };
    world.last_output = Some(out);
}

#[when(regex = r#"^the developer rebuilds the whole product from "([^"]+)" into a spec copy$"#)]
fn rebuild_whole_product_into_a_spec_copy(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
    let trial = trial_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &[
            "ise",
            "rebuild-all",
            "--at",
            revision.as_str(),
            "--into",
            trial.as_str(),
        ],
        "ise rebuild-all",
    );
}

#[when(
    regex = r#"^the developer rebuilds the whole product from the first revision into a spec copy$"#
)]
fn rebuild_whole_product_from_the_first_revision(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    rebuild_whole_product_into_a_spec_copy(world, revision);
}

#[when(regex = r#"^the developer runs rebuild-all from "([^"]+)" with no target$"#)]
fn run_rebuild_all_without_a_target(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
    run_rebuild(
        world,
        &["ise", "rebuild-all", "--at", revision.as_str()],
        "ise rebuild-all",
    );
}

#[when(regex = r#"^the developer runs "givn ise rebuild-component" with no path and a target$"#)]
fn run_rebuild_component_without_a_path(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &["ise", "rebuild-component", "--into", trial.as_str()],
        "ise rebuild-component",
    );
}

/// One retired command line plus its target flag, as `givn` arguments.
fn retired_args<'a>(command: &'a str, flag: &'a str, target: &'a str) -> Vec<&'a str> {
    let mut args: Vec<&str> = command.split_whitespace().skip(1).collect();
    args.push(flag);
    args.push(target);
    args
}

#[when(regex = r#"^the developer runs the retired command "([^"]+)" with a target$"#)]
fn run_retired_command_with_a_target(world: &mut GivnWorld, command: String) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    let args = retired_args(&command, "--into", trial.as_str());
    world.ise_clean_room.retired_command = args.get(1).map(|name| (*name).to_string());
    run_rebuild(world, &args, "ise rebuild-all");
}

#[when(regex = r#"^the developer runs the retired command "([^"]+)" with a manifest$"#)]
fn run_retired_command_with_a_manifest(world: &mut GivnWorld, command: String) {
    world.ise_clean_room.ensure_targets();
    let manifest = manifest_path(world).to_string_lossy().into_owned();
    let args = retired_args(&command, "--manifest", manifest.as_str());
    world.ise_clean_room.retired_command = args.get(1).map(|name| (*name).to_string());
    run_rebuild(world, &args, "ise rebuild-all");
}

#[when(regex = r#"^the developer runs "givn ise rebuild-all --full" with a target$"#)]
fn run_rebuild_all_with_the_full_flag(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &["ise", "rebuild-all", "--full", "--into", trial.as_str()],
        "ise rebuild-all",
    );
}

#[then(regex = r#"^the command fails and names "([^-"][^"]*)"$"#)]
fn the_command_fails_and_names(world: &mut GivnWorld, needle: String) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    assert!(
        text.contains(&needle),
        "stderr {text:?} does not name {needle:?}"
    );
}

#[then(
    regex = r#"^the command fails and names "givn/specification\.yaml" and the loader's reason$"#
)]
fn the_command_fails_and_names_the_loader_reason(world: &mut GivnWorld) {
    assert_refusal_names(
        world,
        &[
            ise_clean_room_support::DECLARATION_PATH,
            &format!("unknown key '{}'", ise_clean_room_support::MALFORMED_KEY),
        ],
    );
}

#[then(regex = r#"^the command fails and names the revision$"#)]
fn the_command_fails_and_names_the_revision(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .requested_revision
        .clone()
        .expect("the When recorded the revision");
    assert_refusal_names(world, &[&revision]);
}

#[then(regex = r#"^the command fails and names the unreadable path and the revision$"#)]
fn the_command_fails_and_names_the_unreadable_blob(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .unreadable_revision
        .clone()
        .expect("the Given recorded the unreadable revision");
    assert_refusal_names(world, &[ise_clean_room_support::KEPT_PATH, &revision]);
}

#[then(
    regex = r#"^the command fails and names "givn/specification\.yaml", the revision, and "does not record"$"#
)]
fn the_command_fails_and_names_the_not_recorded_refusal(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    assert_refusal_names(
        world,
        &[
            ise_clean_room_support::DECLARATION_PATH,
            &revision,
            "does not record",
        ],
    );
    let text = stderr(world);
    assert!(
        !text.contains("returned no blob"),
        "stderr {text:?} names the blob-absent phrase instead of the tree check"
    );
}

#[then(regex = r#"^the command fails and names "givn/specification\.yaml" and the UTF-8 error$"#)]
fn the_command_fails_and_names_the_utf8_error(world: &mut GivnWorld) {
    assert_refusal_names(
        world,
        &[ise_clean_room_support::DECLARATION_PATH, "invalid utf-8"],
    );
}

#[then(
    regex = r#"^the command fails and names "givn/specification\.yaml", the revision, and "returned no blob"$"#
)]
fn the_command_fails_and_names_the_no_blob_refusal(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .unreadable_revision
        .clone()
        .expect("the Given recorded the unreadable revision");
    assert_refusal_names(
        world,
        &[
            ise_clean_room_support::DECLARATION_PATH,
            &revision,
            "returned no blob",
        ],
    );
    let text = stderr(world);
    assert!(
        !text.contains("does not record"),
        "stderr {text:?} names the tree-check phrase instead of the blob read"
    );
}

#[then(regex = r#"^the command fails and names "docs/contract\.md" and the revision$"#)]
fn the_command_fails_and_names_the_missing_specification_entry(world: &mut GivnWorld) {
    let revision = ise_clean_room_support::head_revision(project_env(world));
    assert_refusal_names(
        world,
        &[
            ise_clean_room_support::CONTRACT_PATH,
            revision.as_str(),
            "declared Specification entry",
        ],
    );
}

#[then(regex = r#"^the command fails and names "--into"$"#)]
fn the_command_fails_and_names_the_target_flag(world: &mut GivnWorld) {
    let out = output(world);
    assert!(
        !out.status.success(),
        "the command succeeded; stderr={}",
        stderr(world)
    );
    let text = stderr(world);
    assert!(
        text.contains("--into"),
        "stderr {text:?} does not name --into"
    );
}

#[then(regex = r#"^the command fails and names the modified tracked file and the untracked file$"#)]
fn the_command_fails_and_names_every_dirty_path(world: &mut GivnWorld) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    for path in [
        ise_clean_room_support::MODIFIED_PATH,
        ise_clean_room_support::UNTRACKED_PATH,
    ] {
        assert!(
            text.contains(path),
            "stderr {text:?} does not name {path:?}"
        );
    }
}

#[then(regex = r#"^the command fails and names the missing backend$"#)]
fn the_command_fails_and_names_the_missing_backend(world: &mut GivnWorld) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    assert!(
        text.contains("no Git worktree or Fossil checkout"),
        "stderr {text:?} does not name the missing backend"
    );
}

#[then(regex = r#"^the command fails and names the nested project root$"#)]
fn the_command_fails_and_names_the_nested_project_root(world: &mut GivnWorld) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let root = world
        .ise_clean_room
        .nested_project_root
        .as_ref()
        .expect("the Given recorded the nested project root")
        .display()
        .to_string();
    let text = stderr(world);
    assert!(
        text.contains(&root),
        "stderr {text:?} does not name the nested project root {root:?}"
    );
}

#[then(regex = r#"^the command fails and names the ambiguous version control$"#)]
fn the_command_fails_and_names_the_ambiguous_version_control(world: &mut GivnWorld) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    assert!(
        text.contains("ambiguous version control"),
        "stderr {text:?} does not name the ambiguous version control"
    );
}

#[then(regex = r#"^the spec copy path and the manifest path remain absent$"#)]
fn the_spec_copy_and_manifest_remain_absent(world: &mut GivnWorld) {
    let trial = trial_path(world);
    let manifest = manifest_path(world);
    assert!(
        !trial.exists(),
        "the spec copy path exists: {}",
        trial.display()
    );
    assert!(
        !manifest.exists(),
        "the manifest path exists: {}",
        manifest.display()
    );
}

#[then(regex = r#"^the command exits 2 and names the missing path list$"#)]
fn the_command_exits_two_and_names_the_missing_path_list(world: &mut GivnWorld) {
    assert_exit_two(world);
    let text = stderr(world);
    assert!(
        text.contains("<PATHS>"),
        "stderr {text:?} does not name the missing path list"
    );
}

#[then(regex = r#"^the command exits 2 and names the unknown command$"#)]
fn the_command_exits_two_and_names_the_unknown_command(world: &mut GivnWorld) {
    assert_exit_two(world);
    let command = world
        .ise_clean_room
        .retired_command
        .clone()
        .expect("the When recorded the retired command");
    let text = stderr(world);
    assert!(
        text.contains("unrecognized subcommand"),
        "stderr {text:?} does not name an unrecognized subcommand"
    );
    assert!(
        text.contains(&command),
        "stderr {text:?} does not name the retired command {command:?}"
    );
}

#[then(regex = r#"^the command exits 2 and names the unknown flag$"#)]
fn the_command_exits_two_and_names_the_unknown_flag(world: &mut GivnWorld) {
    assert_exit_two(world);
    let text = stderr(world);
    assert!(
        text.contains("unexpected argument") && text.contains("--full"),
        "stderr {text:?} does not name the unexpected --full flag"
    );
}

#[then(regex = r#"^the spec copy path remains absent$"#)]
fn the_spec_copy_path_remains_absent(world: &mut GivnWorld) {
    let trial = trial_path(world);
    assert!(
        !trial.exists(),
        "the spec copy path exists: {}",
        trial.display()
    );
}

// --- component-paths: Specification and source-root refusals ---------------

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "behavior" role names "givn/specs/\*\*"$"#
)]
fn git_project_whose_behavior_role_names_the_specs(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::retained_behavior_project());
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration whose "behavior" role names "givn/specs/\*\*" and whose source root is "givn/\*\*"$"#
)]
fn git_project_whose_behavior_role_names_the_specs_under_givn(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::behavior_declaration("\"givn/specs/**\"", "\"givn/**\""),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose source root is "givn/\*\*"$"#
)]
fn git_project_whose_source_root_is_givn(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::givn_source_root_project());
}

#[when(regex = r#"^the developer rebuilds the component path "([^"]+)" into a spec copy$"#)]
fn rebuild_the_component_path_into_a_spec_copy(world: &mut GivnWorld, path: String) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &[
            "ise",
            "rebuild-component",
            path.as_str(),
            "--into",
            trial.as_str(),
        ],
        "ise rebuild-component",
    );
}

#[then(regex = r#"^the command fails and exits 1, naming "([^"]+)" and "([^"]+)"$"#)]
fn the_command_fails_and_exits_one_naming(world: &mut GivnWorld, path: String, word: String) {
    assert_refusal_names(world, &[&path, &word]);
}

#[then(regex = r#"^the command fails and names "([^"]+)" and the "([^"]+)" role$"#)]
fn the_command_fails_and_names_the_role(world: &mut GivnWorld, path: String, role: String) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    for needle in [path.as_str(), format!("{role} role").as_str()] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
    assert!(
        !text.contains("retained"),
        "stderr {text:?} keeps the retired retained vocabulary"
    );
}

#[then(regex = r#"^the command fails and names "([^"]+)" as a Specification file$"#)]
fn the_command_fails_and_names_the_specification_file(world: &mut GivnWorld, path: String) {
    assert_exit_one(world);
    assert_refusal_prefix(world);
    let text = stderr(world);
    for needle in [path.as_str(), "Specification file"] {
        assert!(
            text.contains(needle),
            "stderr {text:?} does not carry {needle:?}"
        );
    }
    assert!(
        !text.contains("retained"),
        "stderr {text:?} keeps the retired retained vocabulary"
    );
}

// --- copy-set: repository links, nested markers, and symlink targets -------

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "structure" role names "vendor/lib"$"#
)]
fn git_project_with_a_vendor_link_structure_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::repository_link_project());
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "structure" role names "nested/_FOSSIL_"$"#
)]
fn git_project_with_a_nested_marker_structure_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::nested_marker_project());
}

#[given(regex = r#"^a committed revision with a symlink "src/out" to "\.\./\.\./outside"$"#)]
fn commit_revision_with_a_symlink_outside_the_copy(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/out\"", "\"src/**\""),
    );
    ise_clean_room_support::commit_symlink(env, &[], "src/out", "../../outside");
}

#[given(
    regex = r#"^a committed revision with "src/run\.sh" and a symlink "src/abs" to an absolute target inside the spec copy$"#
)]
fn commit_revision_with_an_absolute_symlink(world: &mut GivnWorld) {
    world.ise_clean_room.ensure_targets();
    let target = trial_path(world).join("src/run.sh");
    let target = target.to_string_lossy().into_owned();
    world.ise_clean_room.absolute_symlink_target = Some(target.clone());
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/run.sh\", \"src/abs\"", "\"src/**\""),
    );
    ise_clean_room_support::commit_symlink(
        env,
        &[("src/run.sh", "#!/bin/sh\necho fixture\n")],
        "src/abs",
        &target,
    );
}

#[then(regex = r#"^the command fails and exits 1, naming "src/abs" and its absolute target$"#)]
fn the_command_fails_and_names_the_absolute_symlink_target(world: &mut GivnWorld) {
    let target = world
        .ise_clean_room
        .absolute_symlink_target
        .clone()
        .expect("the Given recorded the absolute target");
    assert_refusal_names(world, &["src/abs", &target]);
}

// --- partition-and-copy: count labels, declaration, and copy tree -----------

/// The quoted strings a `prints ...` step lists, in order.
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

fn assert_success_and_prints(world: &GivnWorld, printed: &str) {
    assert_success(world);
    for line in printed_lines(printed) {
        assert_report_line(world, &line);
    }
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "structure" role names "src/kept\.rs" and "src/nested/notes\.md" and whose source root is "src/\*\*"$"#
)]
fn git_project_with_two_structure_files_under_src(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::ise_project_with_declaration(
        &ise_clean_room_support::structure_declaration(
            "\"src/kept.rs\", \"src/nested/notes.md\"",
            "\"src/**\"",
        ),
    ));
}

#[given(
    regex = r#"^a committed revision with "src/kept\.rs", "src/nested/notes\.md", and "src/other\.rs"$"#
)]
fn commit_two_specification_files_and_one_source(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            (
                ise_clean_room_support::KEPT_PATH,
                ise_clean_room_support::KEPT_CONTENT,
            ),
            ("src/nested/notes.md", "notes\n"),
            ("src/other.rs", "pub fn other() {}\n"),
        ],
        "fixture: Specification files under a source root",
    );
}

#[given(
    regex = r#"^a committed revision with "src/kept\.rs", a Specification file through the "structure" role$"#
)]
fn commit_kept_file_through_the_structure_role(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/kept.rs\"", "\"src/**\""),
    );
    ise_clean_room_support::commit_files(
        env,
        &[(
            ise_clean_room_support::KEPT_PATH,
            ise_clean_room_support::KEPT_CONTENT,
        )],
        "fixture: kept file through the structure role",
    );
}

#[given(
    regex = r#"^a committed revision with an executable script "src/run\.sh" and a symlink "src/link" to "run\.sh", a Specification file through the "structure" role$"#
)]
fn commit_executable_and_symlink_through_the_structure_role(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration(
            "\"src/run.sh\", \"src/link\"",
            "\"src/**\"",
        ),
    );
    ise_clean_room_support::commit_executable_and_symlink(env);
}

#[given(
    regex = r#"^a committed revision with "src/kept\.rs", a Specification file through the "structure" role, and a later revision adding "src/later\.rs"$"#
)]
fn commit_kept_file_then_a_later_revision(world: &mut GivnWorld) {
    let env = project_env(world);
    ise_clean_room_support::rewrite_declaration(
        env,
        &ise_clean_room_support::structure_declaration("\"src/kept.rs\"", "\"src/**\""),
    );
    let first = ise_clean_room_support::commit_files(
        env,
        &[(
            ise_clean_room_support::KEPT_PATH,
            ise_clean_room_support::KEPT_CONTENT,
        )],
        "fixture: first revision",
    );
    ise_clean_room_support::commit_files(
        env,
        &[("src/later.rs", "pub fn later() {}\n")],
        "fixture: later revision",
    );
    world.ise_clean_room.first_revision = Some(first);
}

#[given(
    regex = r#"^a committed revision with a Specification file, a Source code kept file, a removed source file, an executable script, and a symlink to it$"#
)]
fn commit_fossil_specification_kept_removed_script_and_symlink(world: &mut GivnWorld) {
    ise_clean_room_support::commit_fossil_kinds(project_env(world));
}

#[then(regex = r#"^the command succeeds and prints ("[^"]*"(?:(?:, "[^"]*")*,? and "[^"]*")?)$"#)]
fn the_command_succeeds_and_prints(world: &mut GivnWorld, printed: String) {
    assert_success_and_prints(world, &printed);
}

#[then(
    regex = r#"^the command succeeds and prints "Backend: git" and "Revision:" with the first revision identifier$"#
)]
fn the_command_succeeds_and_reports_the_first_revision(world: &mut GivnWorld) {
    assert_success(world);
    assert_report_line(world, "Backend: git");
    let first = world
        .ise_clean_room
        .first_revision
        .clone()
        .expect("the Given recorded the first revision");
    assert_report_line(world, &format!("Revision: {first}"));
}

#[then(regex = r#"^the spec copy holds "([^"]+)"$"#)]
fn the_spec_copy_holds_one(world: &mut GivnWorld, path: String) {
    assert_trial_holds(world, &[path.as_str()]);
}

#[then(regex = r#"^the spec copy holds "([^"]+)" and "([^"]+)"$"#)]
fn the_spec_copy_holds_two(world: &mut GivnWorld, first: String, second: String) {
    assert_trial_holds(world, &[first.as_str(), second.as_str()]);
}

#[then(regex = r#"^the spec copy omits "([^"]+)"$"#)]
fn the_spec_copy_omits_one(world: &mut GivnWorld, path: String) {
    assert_trial_omits(world, &[path.as_str()]);
}

#[then(regex = r#"^the spec copy omits "([^"]+)" and "([^"]+)"$"#)]
fn the_spec_copy_omits_two(world: &mut GivnWorld, first: String, second: String) {
    assert_trial_omits(world, &[first.as_str(), second.as_str()]);
}

#[then(regex = r#"^the spec copy does not hold "([^"]+)"$"#)]
fn the_spec_copy_does_not_hold(world: &mut GivnWorld, path: String) {
    assert_trial_omits(world, &[path.as_str()]);
}

#[then(regex = r#"^the spec copy does not hold the ignored file$"#)]
fn the_spec_copy_does_not_hold_the_ignored_file(world: &mut GivnWorld) {
    assert_trial_omits(world, &[ise_clean_room_support::IGNORED_PATH]);
}

#[then(regex = r#"^the spec copy holds no "\.git" entry and no "\.fslckout" entry$"#)]
fn the_spec_copy_holds_no_version_control_metadata(world: &mut GivnWorld) {
    assert_trial_has_no_vcs_entries(world);
}

#[then(
    regex = r#"^the spec copy's "givn/specification\.yaml" is the declaration committed at the selected revision$"#
)]
fn the_spec_copy_holds_the_selected_revision_declaration(world: &mut GivnWorld) {
    let revision = world
        .ise_clean_room
        .requested_revision
        .clone()
        .expect("the When recorded the selected revision");
    let expected = ise_clean_room_support::revision_content(
        project_env(world),
        &revision,
        ise_clean_room_support::DECLARATION_PATH,
    );
    let actual = std::fs::read(trial_path(world).join(ise_clean_room_support::DECLARATION_PATH))
        .unwrap_or_else(|error| panic!("read the spec copy declaration: {error}"));
    assert_eq!(
        actual, expected,
        "the spec copy's declaration is not the copy recorded at revision {revision}"
    );
}

#[then(regex = r#"^the spec copy's "src/run\.sh" is executable$"#)]
fn the_spec_copy_script_is_executable(world: &mut GivnWorld) {
    assert_trial_executable(world, ise_clean_room_support::RUN_SCRIPT_PATH);
}

#[then(regex = r#"^the spec copy's "src/link" is a symlink to "run\.sh"$"#)]
fn the_spec_copy_link_points_at_the_script(world: &mut GivnWorld) {
    assert_trial_symlink(
        world,
        ise_clean_room_support::SYMLINK_PATH,
        ise_clean_room_support::SYMLINK_TARGET,
    );
}

#[then(regex = r#"^the spec copy holds the Specification file and the Source code kept file$"#)]
fn the_spec_copy_holds_the_fossil_specification_and_kept_file(world: &mut GivnWorld) {
    assert_trial_holds(
        world,
        &[
            ise_clean_room_support::FOSSIL_FEATURE_PATH,
            ise_clean_room_support::KEPT_SOURCE_PATH,
        ],
    );
}

#[then(regex = r#"^the spec copy omits the removed source file$"#)]
fn the_spec_copy_omits_the_fossil_removed_source(world: &mut GivnWorld) {
    assert_trial_omits(world, &[ise_clean_room_support::REPLACED_SOURCE_PATH]);
}

#[then(regex = r#"^the spec copy's executable script is executable$"#)]
fn the_spec_copy_fossil_script_is_executable(world: &mut GivnWorld) {
    assert_trial_executable(world, ise_clean_room_support::RUN_SCRIPT_PATH);
}

#[then(regex = r#"^the spec copy's symlink is a symlink to its target$"#)]
fn the_spec_copy_fossil_symlink_points_at_its_target(world: &mut GivnWorld) {
    assert_trial_symlink(
        world,
        ise_clean_room_support::SYMLINK_PATH,
        ise_clean_room_support::SYMLINK_TARGET,
    );
}

// --- report-and-manifest ----------------------------------------------------

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "e2e" role names "tests/e2e"$"#
)]
fn git_project_with_the_e2e_role(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::bare_ise_project(
        &ise_clean_room_support::e2e_declaration("\"tests/e2e\"", "\"src/**\""),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "e2e" role names "([^"]+)" and whose source root is "([^"]+)"$"#
)]
fn git_project_with_the_e2e_role_and_source_root(
    world: &mut GivnWorld,
    entry: String,
    source: String,
) {
    world.env = Some(ise_clean_room_support::bare_ise_project(
        &ise_clean_room_support::e2e_declaration(&format!("\"{entry}\""), &format!("\"{source}\"")),
    ));
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "behavior" role names "([^"]+)", whose "e2e" role names "([^"]+)", and whose source root is "([^"]+)"$"#
)]
fn git_project_with_the_behavior_and_e2e_roles(
    world: &mut GivnWorld,
    behavior: String,
    e2e: String,
    source: String,
) {
    world.env = Some(ise_clean_room_support::bare_ise_project(
        &ise_clean_room_support::behavior_e2e_declaration(
            &format!("\"{behavior}\""),
            &format!("\"{e2e}\""),
            &format!("\"{source}\""),
        ),
    ));
}

#[given(
    regex = r#"^a committed revision with "tests/e2e/Cargo\.toml" and "tests/e2e/steps/demo\.rs"$"#
)]
fn commit_the_e2e_package(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("tests/e2e/Cargo.toml", "[package]\nname = \"e2e\"\n"),
            ("tests/e2e/steps/demo.rs", "// e2e steps\n"),
        ],
        "fixture: e2e package",
    );
}

#[given(regex = r#"^a committed revision with "src/kept\.rs" and "notes\.md"$"#)]
fn commit_the_kept_file_and_notes(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            (
                ise_clean_room_support::KEPT_PATH,
                ise_clean_room_support::KEPT_CONTENT,
            ),
            ("notes.md", "notes\n"),
        ],
        "fixture: kept file and notes",
    );
}

#[given(
    regex = r#"^a committed revision with "tests/e2e/runner\.rs" and "tests/unit/helper\.rs"$"#
)]
fn commit_the_overlapping_e2e_and_unit_files(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            ("tests/e2e/runner.rs", "// e2e runner\n"),
            ("tests/unit/helper.rs", "// unit helper\n"),
        ],
        "fixture: overlapping e2e and unit files",
    );
}

#[given(
    regex = r#"^a committed revision with "givn/specs/demo\.feature", "tests/e2e/Cargo\.toml", "src/keys/parser\.rs", "src/keys/mod\.rs", and "src/other\.rs"$"#
)]
fn commit_the_component_revision(world: &mut GivnWorld) {
    ise_clean_room_support::commit_files(
        project_env(world),
        &[
            (
                ise_clean_room_support::DEMO_FEATURE_PATH,
                ise_clean_room_support::DEMO_FEATURE_CONTENT,
            ),
            ("tests/e2e/Cargo.toml", "[package]\nname = \"e2e\"\n"),
            ("src/keys/parser.rs", "pub fn parse() {}\n"),
            ("src/keys/mod.rs", "pub mod parser;\n"),
            ("src/other.rs", "pub fn other() {}\n"),
        ],
        "fixture: component revision",
    );
}

#[when(
    regex = r#"^the developer rebuilds the whole product from "([^"]+)" into a spec copy without a manifest$"#
)]
fn rebuild_whole_product_without_a_manifest(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
    let trial = trial_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &[
            "ise",
            "rebuild-all",
            "--at",
            revision.as_str(),
            "--into",
            trial.as_str(),
        ],
        "ise rebuild-all",
    );
}

#[when(
    regex = r#"^the developer rebuilds the whole product from "([^"]+)" into a spec copy with a manifest$"#
)]
fn rebuild_whole_product_with_a_manifest(world: &mut GivnWorld, revision: String) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.clone());
    let trial = trial_path(world).to_string_lossy().into_owned();
    let manifest = manifest_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
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
        "ise rebuild-all",
    );
}

#[when(
    regex = r#"^the developer runs a dry run of the whole product from "([^"]+)" into a spec copy with a manifest$"#
)]
fn dry_run_whole_product_with_a_manifest(world: &mut GivnWorld, revision: String) {
    run_dry_run(world, &revision, true);
}

#[when(
    regex = r#"^the developer runs a dry run of the whole product from "([^"]+)" into a spec copy without a manifest$"#
)]
fn dry_run_whole_product_without_a_manifest(world: &mut GivnWorld, revision: String) {
    run_dry_run(world, &revision, false);
}

fn run_dry_run(world: &mut GivnWorld, revision: &str, manifest: bool) {
    world.ise_clean_room.ensure_targets();
    world.ise_clean_room.requested_revision = Some(revision.to_string());
    let trial = trial_path(world).to_string_lossy().into_owned();
    let manifest = manifest.then(|| manifest_path(world).to_string_lossy().into_owned());
    let mut args = vec![
        "ise",
        "rebuild-all",
        "--at",
        revision,
        "--into",
        trial.as_str(),
        "--dry-run",
    ];
    if let Some(manifest) = &manifest {
        args.push("--manifest");
        args.push(manifest.as_str());
    }
    run_rebuild(world, &args, "ise rebuild-all");
}

#[when(
    regex = r#"^the developer rebuilds the component paths "([^"]+)" and "([^"]+)" into a spec copy$"#
)]
fn rebuild_component_paths_with_a_manifest(world: &mut GivnWorld, first: String, second: String) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    let manifest = manifest_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &[
            "ise",
            "rebuild-component",
            first.as_str(),
            second.as_str(),
            "--into",
            trial.as_str(),
            "--manifest",
            manifest.as_str(),
        ],
        "ise rebuild-component",
    );
}

#[when(
    regex = r#"^the developer rebuilds the component path "([^"]+)" into a spec copy with a manifest$"#
)]
fn rebuild_component_path_with_a_manifest(world: &mut GivnWorld, path: String) {
    world.ise_clean_room.ensure_targets();
    let trial = trial_path(world).to_string_lossy().into_owned();
    let manifest = manifest_path(world).to_string_lossy().into_owned();
    run_rebuild(
        world,
        &[
            "ise",
            "rebuild-component",
            path.as_str(),
            "--into",
            trial.as_str(),
            "--manifest",
            manifest.as_str(),
        ],
        "ise rebuild-component",
    );
}

#[then(regex = r#"^the command succeeds and prints no "([^"]+)" line$"#)]
fn the_command_succeeds_and_prints_no_line(world: &mut GivnWorld, token: String) {
    assert_success(world);
    let text = stdout(world);
    assert!(
        !text.lines().any(|line| line.starts_with(&token)),
        "stdout {text:?} prints a {token:?} line"
    );
    // The report is the only output: an older build appended the trial record
    // document, whose first fields are `schema` and `content_scope`.
    assert!(
        !text
            .lines()
            .any(|line| line.starts_with("schema:") || line.starts_with("content_scope:")),
        "stdout {text:?} prints a trial record after the report"
    );
}

#[then(regex = r#"^no manifest file is written$"#)]
fn no_manifest_file_is_written(world: &mut GivnWorld) {
    let manifest = manifest_path(world);
    assert!(
        !manifest.exists(),
        "the manifest path exists: {}",
        manifest.display()
    );
}

#[then(regex = r#"^the spec copy holds the declaration "([^"]+)"$"#)]
fn the_spec_copy_holds_the_declaration(world: &mut GivnWorld, path: String) {
    assert_trial_holds(world, &[path.as_str()]);
}

#[then(regex = r#"^the dry run prints "([^"]+)" and "([^"]+)"$"#)]
fn the_dry_run_prints_two_labels(world: &mut GivnWorld, first: String, second: String) {
    assert_success(world);
    let text = stdout(world);
    for label in [first, second] {
        assert!(
            text.lines().any(|line| line.starts_with(&label)),
            "stdout {text:?} does not print a {label:?} line"
        );
    }
}

#[then(regex = r#"^the dry run prints "([^"]+)" and no "([^"]+)" line$"#)]
fn the_dry_run_prints_one_label_and_no_other(world: &mut GivnWorld, first: String, absent: String) {
    assert_success(world);
    let text = stdout(world);
    assert!(
        text.lines().any(|line| line.starts_with(&first)),
        "stdout {text:?} does not print a {first:?} line"
    );
    assert!(
        !text.lines().any(|line| line.starts_with(&absent)),
        "stdout {text:?} prints a {absent:?} line"
    );
}

#[then(regex = r#"^the manifest lists "([^"]+)" among the Specification files$"#)]
fn the_manifest_lists_among_the_specification_files(world: &mut GivnWorld, path: String) {
    let manifest = record_document(world);
    let files = record_list(&manifest, "specification_files");
    assert!(
        files.contains(&path),
        "specification_files {files:?} do not hold {path:?}"
    );
}

#[then(regex = r#"^the manifest's End-to-end files hold "([^"]+)" and "([^"]+)"$"#)]
fn the_manifest_end_to_end_files_hold(world: &mut GivnWorld, first: String, second: String) {
    let manifest = record_document(world);
    let mut expected = vec![first, second];
    expected.sort();
    assert_eq!(
        record_list(&manifest, "end_to_end_files"),
        expected,
        "the manifest's End-to-end files are wrong"
    );
}

#[then(regex = r#"^the manifest's End-to-end files are empty$"#)]
fn the_manifest_end_to_end_files_are_empty(world: &mut GivnWorld) {
    let manifest = record_document(world);
    assert_eq!(
        record_list(&manifest, "end_to_end_files"),
        Vec::<String>::new(),
        "the manifest's End-to-end files are not empty"
    );
}

#[then(regex = r#"^the manifest records one removed path$"#)]
fn the_manifest_records_one_removed_path(world: &mut GivnWorld) {
    let manifest = record_document(world);
    assert_eq!(
        record_list(&manifest, "removed_paths"),
        vec!["src/one.rs".to_string()],
        "the manifest does not record the collapsed removal"
    );
}

#[then(regex = r#"^the manifest's removed paths are:$"#)]
fn the_manifest_removed_paths_are(world: &mut GivnWorld, step: &Step) {
    let manifest = record_document(world);
    let expected: Vec<String> = step
        .table()
        .expect("the step carries a removed-paths table")
        .rows
        .iter()
        .flatten()
        .cloned()
        .collect();
    assert_eq!(
        record_list(&manifest, "removed_paths"),
        expected,
        "the manifest's removed paths are wrong"
    );
}

// --- checksums --------------------------------------------------------------

#[given(regex = r#"^the pinned canonical checksum fixture$"#)]
fn the_pinned_canonical_checksum_fixture(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::canonical_checksum_project());
}

#[given(regex = r#"^the pinned richer checksum fixture$"#)]
fn the_pinned_richer_checksum_fixture(world: &mut GivnWorld) {
    world.env = Some(ise_clean_room_support::rich_checksum_project());
}

#[given(
    regex = r#"^a committed revision with "tests/e2e/Cargo\.toml" and a repository link at "tests/e2e/vendor/dep"$"#
)]
fn commit_e2e_package_with_repository_link(world: &mut GivnWorld) {
    ise_clean_room_support::commit_e2e_package_with_repository_link(project_env(world));
}

fn manifest_checksums(world: &GivnWorld, map: &str) -> std::collections::BTreeMap<String, String> {
    let manifest = record_document(world);
    manifest
        .get("checksums")
        .and_then(|checksums| checksums.get(map))
        .and_then(|value| value.as_mapping())
        .unwrap_or_else(|| panic!("the manifest has no 'checksums.{map}' map"))
        .iter()
        .map(|(path, digest)| {
            (
                path.as_str()
                    .unwrap_or_else(|| panic!("checksums.{map} holds a non-string path"))
                    .to_string(),
                digest
                    .as_str()
                    .unwrap_or_else(|| panic!("checksums.{map} holds a non-string checksum"))
                    .to_string(),
            )
        })
        .collect()
}

fn manifest_string(world: &GivnWorld, key: &str) -> String {
    record_document(world)
        .get(key)
        .and_then(serde_yaml_ng::Value::as_str)
        .unwrap_or_else(|| panic!("the manifest has no '{key}' string"))
        .to_string()
}

fn assert_checksum_table(world: &GivnWorld, map: &str, step: &Step) {
    let expected: std::collections::BTreeMap<String, String> = step
        .table()
        .expect("the step carries a checksum table")
        .rows
        .iter()
        .skip(1)
        .map(|row| match row.as_slice() {
            [path, checksum] => (path.clone(), checksum.clone()),
            other => panic!("a checksum row needs two cells, got {other:?}"),
        })
        .collect();
    let actual = manifest_checksums(world, map);
    assert_eq!(actual, expected, "the manifest's {map} checksums are wrong");
}

#[then(regex = r#"^the manifest's scenario checksum for "([^"]+)" is "([^"]+)"$"#)]
fn the_manifest_scenario_checksum_for(world: &mut GivnWorld, path: String, checksum: String) {
    let map = manifest_checksums(world, "scenarios");
    assert_eq!(
        map.get(&path),
        Some(&checksum),
        "scenario checksums {map:?} do not hold {checksum:?} for {path:?}"
    );
}

#[then(regex = r#"^the manifest's specification checksum is "([^"]+)"$"#)]
fn the_manifest_specification_checksum_is(world: &mut GivnWorld, checksum: String) {
    assert_eq!(manifest_string(world, "specification_checksum"), checksum);
}

#[then(regex = r#"^the manifest's end-to-end checksum is "([^"]+)"$"#)]
fn the_manifest_end_to_end_checksum_is(world: &mut GivnWorld, checksum: String) {
    assert_eq!(manifest_string(world, "end_to_end_checksum"), checksum);
}

#[then(regex = r#"^the manifest's specification checksums are:$"#)]
fn the_manifest_specification_checksums_are(world: &mut GivnWorld, step: &Step) {
    assert_checksum_table(world, "specification", step);
}

#[then(regex = r#"^the manifest's end-to-end checksums are:$"#)]
fn the_manifest_end_to_end_checksums_are(world: &mut GivnWorld, step: &Step) {
    assert_checksum_table(world, "end_to_end", step);
}

#[then(regex = r#"^the manifest's scenario checksums are:$"#)]
fn the_manifest_scenario_checksums_are(world: &mut GivnWorld, step: &Step) {
    assert_checksum_table(world, "scenarios", step);
}

#[then(regex = r#"^the manifest's end-to-end checksums hold a 64-hex checksum for "([^"]+)"$"#)]
fn the_manifest_end_to_end_checksums_hold_one(world: &mut GivnWorld, path: String) {
    let map = manifest_checksums(world, "end_to_end");
    let checksum = map
        .get(&path)
        .unwrap_or_else(|| panic!("checksums.end_to_end has no row for {path:?}: {map:?}"));
    assert!(
        checksum.len() == 64
            && checksum
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')),
        "the checksum {checksum:?} for {path:?} is not 64 lowercase hex characters"
    );
}
