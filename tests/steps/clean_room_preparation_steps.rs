//! Step definitions for the `clean-room-preparation` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. Unimplemented phrases use the
//! not-implemented stub pattern from `design.md` "Strict Mode": RED is a
//! failed step that names the phrase, never an unbound or skipped step.

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

// --- Batch 1: admission-refusals -------------------------------------------

use crate::common;
use crate::steps::ise_clean_room_support;
use crate::steps::{output, stderr};

fn project_env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn assert_exit_one(world: &GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(world));
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

#[given(regex = r#"^a Git project with ise on, architecture principles off, and a declaration$"#)]
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
    assert_exit_one(world);
    let text = stderr(world);
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
fn not_implemented_0001(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Fossil checkout with ise enabled, allow-symlinks on, and a declaration with one source root "src/**""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "givn/specs/\*\*"$"#
)]
fn not_implemented_0002(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration that retains "givn/specs/**""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "nested/_FOSSIL_"$"#
)]
fn not_implemented_0003(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration that retains "nested/_FOSSIL_""#
    ))
}

#[given(regex = r#"^a Git project with ise enabled and a declaration that retains "vendor/lib"$"#)]
fn not_implemented_0004(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration that retains "vendor/lib""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration whose "contracts" role names "docs/contract\.md"$"#
)]
fn not_implemented_0005(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration whose "contracts" role names "docs/contract.md""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled and a declaration with one source root "src/\*\*"$"#
)]
fn not_implemented_0006(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration with one source root "src/**""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration that retains "givn/specs/\*\*", and one source root "givn/\*\*"$"#
)]
fn not_implemented_0007(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled, a declaration that retains "givn/specs/**", and one source root "givn/**""#
    ))
}

#[given(
    regex = r#"^a Git project with ise enabled, a declaration that retains "givn/specs/\*\*", and one source root "src/\*\*"$"#
)]
fn not_implemented_0008(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled, a declaration that retains "givn/specs/**", and one source root "src/**""#
    ))
}

#[given(
    regex = r#"^a committed revision where "src/keys/parser\.rs" sits beside "src/keys/mod\.rs" and "src/keys/tests\.rs"$"#
)]
fn not_implemented_0009(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision where "src/keys/parser.rs" sits beside "src/keys/mod.rs" and "src/keys/tests.rs""#
    ))
}

#[given(regex = r#"^a committed revision whose tracked blob is absent from the object store$"#)]
fn not_implemented_0010(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision whose tracked blob is absent from the object store"#
    ))
}

#[given(regex = r#"^a committed revision with "givn/specs/demo\.feature"$"#)]
fn not_implemented_0011(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "givn/specs/demo.feature""#
    ))
}

#[given(regex = r#"^a committed revision with "nested/_FOSSIL_"$"#)]
fn not_implemented_0012(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "nested/_FOSSIL_""#
    ))
}

#[given(regex = r#"^a committed revision with "src/kept\.rs"$"#)]
fn not_implemented_0013(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/kept.rs""#
    ))
}

#[given(
    regex = r#"^a committed revision with "src/kept\.rs" and a later revision adding "src/later\.rs"$"#
)]
fn not_implemented_0014(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/kept.rs" and a later revision adding "src/later.rs""#
    ))
}

#[given(regex = r#"^a committed revision with "src/keys/parser\.rs"$"#)]
fn not_implemented_0015(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/keys/parser.rs""#
    ))
}

#[given(
    regex = r#"^a committed revision with "src/keys/parser\.rs", "src/keys/mod\.rs", and "src/other\.rs"$"#
)]
fn not_implemented_0016(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/keys/parser.rs", "src/keys/mod.rs", and "src/other.rs""#
    ))
}

#[given(regex = r#"^a committed revision with "src/one\.rs" and "src/nested/two\.rs"$"#)]
fn not_implemented_0017(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/one.rs" and "src/nested/two.rs""#
    ))
}

#[given(regex = r#"^a committed revision with "src/one\.rs" and "src/two\.rs"$"#)]
fn not_implemented_0018(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/one.rs" and "src/two.rs""#
    ))
}

#[given(
    regex = r#"^a committed revision with "src/one\.rs", "src/nested/two\.rs", and "givn/specs/demo\.feature"$"#
)]
fn not_implemented_0019(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "src/one.rs", "src/nested/two.rs", and "givn/specs/demo.feature""#
    ))
}

#[given(regex = r#"^a committed revision with a repository link at "vendor/lib"$"#)]
fn not_implemented_0020(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with a repository link at "vendor/lib""#
    ))
}

#[given(
    regex = r#"^a committed revision with a retained file, a kept source file, a replaced source file, an executable script, and a symlink to it$"#
)]
fn not_implemented_0021(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with a retained file, a kept source file, a replaced source file, an executable script, and a symlink to it"#
    ))
}

#[given(
    regex = r#"^a committed revision with an executable script "src/run\.sh" and a symlink "src/link" to "run\.sh"$"#
)]
fn not_implemented_0022(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with an executable script "src/run.sh" and a symlink "src/link" to "run.sh""#
    ))
}

#[given(regex = r#"^a committed revision without "docs/contract\.md"$"#)]
fn not_implemented_0023(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision without "docs/contract.md""#
    ))
}

#[given(regex = r#"^a receipt file that already exists$"#)]
fn not_implemented_0024(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a receipt file that already exists"#
    ))
}

#[given(regex = r#"^a receipt path whose parent directory is a regular file$"#)]
fn not_implemented_0025(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a receipt path whose parent directory is a regular file"#
    ))
}

#[given(regex = r#"^a symlink outside the checkout that points into the checkout$"#)]
fn not_implemented_0026(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a symlink outside the checkout that points into the checkout"#
    ))
}

#[given(regex = r#"^a target directory that already exists$"#)]
fn not_implemented_0027(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a target directory that already exists"#
    ))
}

#[given(regex = r#"^a target path that exists as a regular file$"#)]
fn not_implemented_0028(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a target path that exists as a regular file"#
    ))
}

#[given(regex = r#"^an ignored file in the working tree$"#)]
fn not_implemented_0029(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: an ignored file in the working tree"#
    ))
}

#[then(regex = r#"^no record file is written$"#)]
fn not_implemented_0030(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: no record file is written"#
    ))
}

#[then(regex = r#"^preparation fails and names "--full" and "--path"$"#)]
fn not_implemented_0031(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names "--full" and "--path""#
    ))
}

#[then(regex = r#"^preparation fails and names "--full" and "--path" as mutually exclusive$"#)]
fn not_implemented_0032(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names "--full" and "--path" as mutually exclusive"#
    ))
}

#[then(regex = r#"^preparation fails and names "docs/contract\.md" and the revision$"#)]
fn not_implemented_0033(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names "docs/contract.md" and the revision"#
    ))
}

#[then(
    regex = r#"^preparation fails and names "givn/specs/demo\.feature" and the "behavior" role$"#
)]
fn not_implemented_0034(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names "givn/specs/demo.feature" and the "behavior" role"#
    ))
}

#[then(regex = r#"^preparation fails and names the existing receipt$"#)]
fn not_implemented_0035(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the existing receipt"#
    ))
}

#[then(regex = r#"^preparation fails and names the existing target$"#)]
fn not_implemented_0036(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the existing target"#
    ))
}

#[then(regex = r#"^preparation fails and names the receipt$"#)]
fn not_implemented_0037(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the receipt"#
    ))
}

#[then(regex = r#"^preparation fails and names the receipt path$"#)]
fn not_implemented_0038(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the receipt path"#
    ))
}

#[then(regex = r#"^preparation fails and names the target$"#)]
fn not_implemented_0039(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the target"#
    ))
}

#[then(regex = r#"^preparation fails and names the unreadable path and the revision$"#)]
fn not_implemented_0040(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation fails and names the unreadable path and the revision"#
    ))
}

#[then(regex = r#"^preparation succeeds$"#)]
fn not_implemented_0041(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(r#"not implemented: preparation succeeds"#))
}

#[then(regex = r#"^preparation succeeds and prints "Backend: fossil"$"#)]
fn not_implemented_0042(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Backend: fossil""#
    ))
}

#[then(regex = r#"^preparation succeeds and prints "Not replaced: 2"$"#)]
fn not_implemented_0043(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Not replaced: 2""#
    ))
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 0"$"#)]
fn not_implemented_0044(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Replaced: 0""#
    ))
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 1"$"#)]
fn not_implemented_0045(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Replaced: 1""#
    ))
}

#[then(regex = r#"^preparation succeeds and prints "Replaced: 2" and "Not replaced: 0"$"#)]
fn not_implemented_0046(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Replaced: 2" and "Not replaced: 0""#
    ))
}

#[then(
    regex = r#"^preparation succeeds and prints "Revision:" with the first revision identifier$"#
)]
fn not_implemented_0047(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints "Revision:" with the first revision identifier"#
    ))
}

#[then(regex = r#"^preparation succeeds and prints the trial record to stdout$"#)]
fn not_implemented_0048(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: preparation succeeds and prints the trial record to stdout"#
    ))
}

#[then(regex = r#"^the "Would write:" block lists the sorted paths the trial would hold$"#)]
fn not_implemented_0049(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the "Would write:" block lists the sorted paths the trial would hold"#
    ))
}

#[then(regex = r#"^the checkout holds no receipt$"#)]
fn not_implemented_0050(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the checkout holds no receipt"#
    ))
}

#[then(regex = r#"^the checkout holds no trial directory$"#)]
fn not_implemented_0051(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the checkout holds no trial directory"#
    ))
}

#[then(regex = r#"^the checkout holds no trial directory and no receipt$"#)]
fn not_implemented_0052(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the checkout holds no trial directory and no receipt"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and no receipt$"#
)]
fn not_implemented_0054(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with a full replacement and no receipt"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and no target$"#
)]
fn not_implemented_0055(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with a full replacement and no target"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with a full replacement and that receipt$"#
)]
fn not_implemented_0056(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with a full replacement and that receipt"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with both a full replacement and a path$"#
)]
fn not_implemented_0057(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with both a full replacement and a path"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with neither a full replacement nor a path$"#
)]
fn not_implemented_0058(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with neither a full replacement nor a path"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the directory path "givn/specs"$"#
)]
fn not_implemented_0059(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the directory path "givn/specs""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the directory path "src/keys"$"#
)]
fn not_implemented_0060(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the directory path "src/keys""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "docs/reference\.md"$"#
)]
fn not_implemented_0061(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "docs/reference.md""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "givn/specs/demo\.feature"$"#
)]
fn not_implemented_0062(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "givn/specs/demo.feature""#
    ))
}

#[when(regex = r#"^the developer prepares a trial from "HEAD" with the path "src/\*\*"$"#)]
fn not_implemented_0063(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "src/**""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parser\.rs"$"#
)]
fn not_implemented_0064(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parsr\.rs"$"#
)]
fn not_implemented_0065(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "src/keys/parsr.rs""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the paths "src/one\.rs" and "src/one\.rs"$"#
)]
fn not_implemented_0066(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the paths "src/one.rs" and "src/one.rs""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from the first revision with a full replacement$"#
)]
fn not_implemented_0067(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from the first revision with a full replacement"#
    ))
}

#[when(regex = r#"^the developer prepares a trial into a directory inside the checkout$"#)]
fn not_implemented_0068(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial into a directory inside the checkout"#
    ))
}

#[when(regex = r#"^the developer prepares a trial into that target$"#)]
fn not_implemented_0069(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial into that target"#
    ))
}

#[when(regex = r#"^the developer prepares a trial through the symlinked target$"#)]
fn not_implemented_0070(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial through the symlinked target"#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial whose receipt path is an ancestor of the trial directory$"#
)]
fn not_implemented_0071(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial whose receipt path is an ancestor of the trial directory"#
    ))
}

#[when(regex = r#"^the developer prepares a trial with a receipt inside the checkout$"#)]
fn not_implemented_0072(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial with a receipt inside the checkout"#
    ))
}

#[when(regex = r#"^the developer prepares a trial with a receipt inside the trial directory$"#)]
fn not_implemented_0073(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial with a receipt inside the trial directory"#
    ))
}

#[when(regex = r#"^the developer prepares a trial with that receipt$"#)]
fn not_implemented_0074(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial with that receipt"#
    ))
}

#[when(
    regex = r#"^the developer runs a dry run from "HEAD" with a full replacement and a receipt$"#
)]
fn not_implemented_0075(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer runs a dry run from "HEAD" with a full replacement and a receipt"#
    ))
}

#[then(
    regex = r#"^the dry run prints the ten fixed report lines, the receipt path line, and a "Would write:" block$"#
)]
fn not_implemented_0076(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the dry run prints the ten fixed report lines, the receipt path line, and a "Would write:" block"#
    ))
}

#[then(regex = r#"^the existing directory is left unchanged$"#)]
fn not_implemented_0077(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the existing directory is left unchanged"#
    ))
}

#[then(regex = r#"^the existing file is left unchanged$"#)]
fn not_implemented_0078(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the existing file is left unchanged"#
    ))
}

#[then(regex = r#"^the existing receipt is left unchanged$"#)]
fn not_implemented_0079(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the existing receipt is left unchanged"#
    ))
}

#[then(regex = r#"^the receipt records one named path and one replaced path$"#)]
fn not_implemented_0080(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the receipt records one named path and one replaced path"#
    ))
}

#[then(regex = r#"^the report holds no "Receipt path" line$"#)]
fn not_implemented_0081(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the report holds no "Receipt path" line"#
    ))
}

#[then(regex = r#"^the trial directory it created is removed$"#)]
fn not_implemented_0082(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial directory it created is removed"#
    ))
}

#[then(regex = r#"^the trial does not hold "src/later\.rs"$"#)]
fn not_implemented_0083(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial does not hold "src/later.rs""#
    ))
}

#[then(regex = r#"^the trial does not hold the ignored file$"#)]
fn not_implemented_0084(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial does not hold the ignored file"#
    ))
}

#[then(regex = r#"^the trial holds "givn/specs/demo\.feature"$"#)]
fn not_implemented_0085(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "givn/specs/demo.feature""#
    ))
}

#[then(regex = r#"^the trial holds "src/kept\.rs"$"#)]
fn not_implemented_0086(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "src/kept.rs""#
    ))
}

#[then(regex = r#"^the trial holds "src/keys/mod\.rs" and "src/keys/tests\.rs"$"#)]
fn not_implemented_0087(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "src/keys/mod.rs" and "src/keys/tests.rs""#
    ))
}

#[then(regex = r#"^the trial holds "src/keys/parser\.rs"$"#)]
fn not_implemented_0088(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "src/keys/parser.rs""#
    ))
}

#[then(regex = r#"^the trial holds "src/one\.rs" and "src/nested/two\.rs"$"#)]
fn not_implemented_0089(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "src/one.rs" and "src/nested/two.rs""#
    ))
}

#[then(regex = r#"^the trial holds "src/other\.rs"$"#)]
fn not_implemented_0090(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds "src/other.rs""#
    ))
}

#[then(regex = r#"^the trial holds no "\.git" entry and no "\.fslckout" entry$"#)]
fn not_implemented_0091(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds no ".git" entry and no ".fslckout" entry"#
    ))
}

#[then(regex = r#"^the trial holds no file under "src"$"#)]
fn not_implemented_0092(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds no file under "src""#
    ))
}

#[then(regex = r#"^the trial holds the retained file and the kept source file$"#)]
fn not_implemented_0093(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds the retained file and the kept source file"#
    ))
}

#[then(regex = r#"^the trial omits "src/keys/parser\.rs" and "src/keys/mod\.rs"$"#)]
fn not_implemented_0094(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial omits "src/keys/parser.rs" and "src/keys/mod.rs""#
    ))
}

#[then(regex = r#"^the trial omits the replaced source file$"#)]
fn not_implemented_0095(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial omits the replaced source file"#
    ))
}

#[then(regex = r#"^the trial's "src/link" is a symlink to "run\.sh"$"#)]
fn not_implemented_0096(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial's "src/link" is a symlink to "run.sh""#
    ))
}

#[then(regex = r#"^the trial's "src/run\.sh" is executable$"#)]
fn not_implemented_0097(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial's "src/run.sh" is executable"#
    ))
}

#[then(regex = r#"^the trial's executable script is executable$"#)]
fn not_implemented_0098(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial's executable script is executable"#
    ))
}

#[then(regex = r#"^the trial's symlink is a symlink to its target$"#)]
fn not_implemented_0099(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial's symlink is a symlink to its target"#
    ))
}
