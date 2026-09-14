//! Step definitions for the `scm-backend-selection` fragment capability.
//!
//! These bindings drive the compiled `givn` binary in temporary projects and
//! assert the selection and refusal behavior. The `@e2e` companion lives in
//! `scm_backend_selection_e2e_steps.rs`.

use std::path::PathBuf;

use cucumber::{given, then, when};

use super::{GivnWorld, stderr};
use crate::common;

/// Fixture state for the SCM backend selection scenarios.
#[derive(Debug, Default)]
pub(crate) struct ScmFixture {
    /// Project tree snapshot taken before the invocation, for the
    /// "no file in the project changed" assertion.
    pub(crate) snapshot: Option<Vec<(PathBuf, Vec<u8>)>>,
}

#[given("a Git project whose project root is the worktree top level")]
fn given_git_project_at_root(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("a managed file is missing from the project")]
fn given_managed_file_missing(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("a project root without a Git worktree or a Fossil checkout")]
fn given_project_without_backend(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    world.scm_fixture.snapshot = Some(snapshot_tree(&env.root()));
    world.env = Some(env);
}

#[given("a Fossil checkout whose project root is the checkout root")]
fn given_fossil_checkout(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("a project root that is both a Git worktree and a Fossil checkout")]
fn given_ambiguous_root(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("a Git project whose project root is nested inside the worktree top level")]
fn given_nested_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("a Git worktree whose `git` executable is unavailable")]
fn given_git_executable_unavailable(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when("the user runs `givn upgrade` without the `git` executable on PATH")]
fn when_upgrade_without_git(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr names the project root and the missing version-controlled project")]
fn then_no_backend_refusal(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let stderr = stderr(world);
    assert!(
        stderr.contains(&root.display().to_string()),
        "stderr does not name the project root: {stderr}"
    );
    assert!(
        stderr.contains("version-controlled project"),
        "stderr does not name the missing version-controlled project: {stderr}"
    );
}

#[then("stderr names the Fossil checkout and the unavailable Fossil backend")]
fn then_fossil_refusal(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr names the Git marker and the Fossil marker")]
fn then_ambiguity_refusal(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr names the project root and the worktree top level")]
fn then_nested_refusal(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr names the missing `git` executable")]
fn then_missing_executable_refusal(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("no file in the project changed")]
fn then_no_file_changed(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let before = world
        .scm_fixture
        .snapshot
        .as_ref()
        .expect("a project snapshot is required first");
    let after = snapshot_tree(&root);
    assert_eq!(before, &after, "the refusal must not change any file");
}

fn snapshot_tree(root: &std::path::Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut entries = Vec::new();
    collect_tree(root, root, &mut entries);
    entries.sort();
    entries
}

fn collect_tree(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
    let Ok(read) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_tree(root, &path, out);
        } else if let Ok(content) = std::fs::read(&path) {
            let relative = path
                .strip_prefix(root)
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|_| path.clone());
            out.push((relative, content));
        }
    }
}
