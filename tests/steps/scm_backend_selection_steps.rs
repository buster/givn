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
    /// Revision before the invocation, for the "publishes a revision"
    /// assertion.
    pub(crate) head_before: Option<String>,
}

#[given("a Git project whose project root is the worktree top level")]
fn given_git_project_at_root(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let head = common::git(&env, &["rev-parse", "HEAD"]);
    assert!(head.status.success(), "read fixture HEAD");
    world.scm_fixture.head_before = Some(String::from_utf8_lossy(&head.stdout).trim().to_string());
    world.env = Some(env);
}

#[given("a managed file is missing from the project")]
fn given_managed_file_missing(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    std::fs::remove_file(env.path(".opencode/plugins/givn-attribution.js"))
        .expect("remove managed integration");
    common::git_commit_all(env, "fixture: drop managed integration");
}

#[given("a project root without a Git worktree or a Fossil checkout")]
fn given_project_without_backend(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    world.scm_fixture.snapshot = Some(snapshot_tree(&env.root()));
    world.env = Some(env);
}

#[given("a Fossil checkout whose project root is the checkout root")]
fn given_fossil_checkout(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    std::fs::write(env.path(".fslckout"), b"fossil checkout fixture\n")
        .expect("write Fossil checkout marker");
    world.scm_fixture.snapshot = Some(snapshot_tree(&env.root()));
    world.env = Some(env);
}

#[given("a project root that is both a Git worktree and a Fossil checkout")]
fn given_ambiguous_root(world: &mut GivnWorld) {
    let env = common::make_init_root();
    std::fs::write(env.path(".fslckout"), b"fossil checkout fixture\n")
        .expect("write Fossil checkout marker");
    world.scm_fixture.snapshot = Some(snapshot_tree(&env.root()));
    world.env = Some(env);
}

#[given("a Git project whose project root is nested inside the worktree top level")]
fn given_nested_project(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    common::git_init(&env);
    let nested = env.path("nested");
    std::fs::create_dir_all(&nested).expect("create nested project dir");
    let out = common::run_givn_with_envs_in(&env, &nested, &["init"], &[]);
    assert!(
        out.status.success(),
        "givn init in nested project failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.scm_fixture.snapshot = Some(snapshot_tree(&nested));
    world.cwd_override = Some(nested);
    world.env = Some(env);
}

#[given("a Git worktree whose `git` executable is unavailable")]
fn given_git_executable_unavailable(world: &mut GivnWorld) {
    let env = common::make_init_root();
    world.scm_fixture.snapshot = Some(snapshot_tree(&env.root()));
    world.env = Some(env);
}

#[when("the user runs `givn upgrade` without the `git` executable on PATH")]
fn when_upgrade_without_git(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let empty_bin = env.path("no-git-bin");
    std::fs::create_dir_all(&empty_bin).expect("create empty PATH directory");
    let path = empty_bin.to_string_lossy().into_owned();
    let out = common::run_givn_with_envs(env, &["upgrade"], &[("PATH", &path)]);
    world.last_output = Some(out);
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
fn then_fossil_refusal(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let stderr = stderr(world);
    assert!(
        stderr.contains("Fossil checkout") && stderr.contains(&root.display().to_string()),
        "stderr does not name the Fossil checkout at the project root: {stderr}"
    );
    assert!(
        stderr.contains("not available"),
        "stderr does not name the unavailable Fossil backend: {stderr}"
    );
}

#[then("stderr names the Git marker and the Fossil marker")]
fn then_ambiguity_refusal(world: &mut GivnWorld) {
    let stderr = stderr(world);
    assert!(
        stderr.contains("Git") && stderr.contains(".fslckout"),
        "stderr does not name both markers: {stderr}"
    );
    assert!(
        stderr.contains("ambiguous"),
        "stderr does not refuse the ambiguity: {stderr}"
    );
}

#[then("stderr names the project root and the worktree top level")]
fn then_nested_refusal(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let root = env.path("nested");
    let top = env.root();
    let stderr = stderr(world);
    assert!(
        stderr.contains(&root.display().to_string()),
        "stderr does not name the nested project root: {stderr}"
    );
    let marker = "nested inside '";
    let index = stderr
        .find(marker)
        .unwrap_or_else(|| panic!("stderr does not name the nesting: {stderr}"))
        + marker.len();
    assert!(
        stderr[index..].starts_with(&top.display().to_string()),
        "stderr does not name the worktree top level: {stderr}"
    );
}

#[then("stderr names the missing `git` executable")]
fn then_missing_executable_refusal(world: &mut GivnWorld) {
    let stderr = stderr(world);
    assert!(
        stderr.contains("`git`") && stderr.contains("not found"),
        "stderr does not name the missing `git` executable: {stderr}"
    );
}

#[then("no file in the project changed")]
fn then_no_file_changed(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let root = world.cwd_override.clone().unwrap_or_else(|| env.root());
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
