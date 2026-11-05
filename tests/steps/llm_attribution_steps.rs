//! Step definitions for the llm-attribution capability.

use std::path::PathBuf;
use std::process::{Command, Output};

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[derive(Debug, Default)]
pub(crate) struct GitAttributionState {
    pub(crate) last_output: Option<Output>,
    pub(crate) context: Option<AttributionContext>,
    pub(crate) commits: Vec<String>,
    pub(crate) requested_path: Option<String>,
    pub(crate) snapshot: Option<String>,
    pub(crate) revision_before: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AttributionContext {
    pub(crate) llm: Option<String>,
    pub(crate) reasoning: Option<String>,
    pub(crate) harness: Option<String>,
}

#[given("a Givn project with the shared commit-attribution hook active")]
fn given_givn_project_with_hook(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let root = env.root();
    assert!(git(&root, &["init"]).status.success());
    assert!(
        git(&root, &["config", "user.name", "Human Developer"])
            .status
            .success()
    );
    assert!(
        git(&root, &["config", "user.email", "human@example.test"])
            .status
            .success()
    );
    let output = common::run_givn(&env, &["init", "--agents", "opencode,claude"]);
    assert!(
        output.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState::default());
}

#[given(
    regex = r##"^the current attribution context has LLM "([^"]+)", reasoning mode "([^"]+)", and harness "([^"]+)"$"##
)]
fn given_complete_context(world: &mut GivnWorld, llm: String, reasoning: String, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        llm: Some(llm),
        reasoning: Some(reasoning),
        harness: Some(harness),
    });
}

#[given("no current attribution context")]
fn given_no_context(_world: &mut GivnWorld) {
    _world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = None;
}

#[given(
    regex = r##"^the current attribution context has harness "([^"]+)" but no LLM or reasoning mode$"##
)]
fn given_partial_context(world: &mut GivnWorld, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        harness: Some(harness),
        ..AttributionContext::default()
    });
}

#[given("a Givn project with an existing commit hook that writes a marker")]
fn given_existing_hook(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let root = env.root();
    assert!(git(&root, &["init"]).status.success());
    assert!(
        git(&root, &["config", "user.name", "Human Developer"])
            .status
            .success()
    );
    assert!(
        git(&root, &["config", "user.email", "human@example.test"])
            .status
            .success()
    );
    let hooks = String::from_utf8_lossy(&git(&root, &["rev-parse", "--git-path", "hooks"]).stdout)
        .trim()
        .to_string();
    let hooks = root.join(hooks);
    std::fs::create_dir_all(&hooks).expect("create hooks directory");
    let hook = hooks.join("commit-msg");
    std::fs::write(
        &hook,
        "#!/bin/sh\nprintf marker > \"$(git rev-parse --show-toplevel)/existing-hook.marker\"\n",
    )
    .expect("write existing hook");
    make_executable(&hook);
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState::default());
}

#[when(regex = r##"^the consumer creates a commit with message "([^"]+)"$"##)]
fn when_consumer_creates_commit(world: &mut GivnWorld, message: String) {
    let root = world.env.as_ref().expect("project root").root();
    std::fs::write(root.join("fixture.txt"), &message).expect("write fixture");
    assert!(
        git_with_givn_path(&root, &["add", "fixture.txt"], None)
            .status
            .success()
    );
    let context = world
        .git_attribution
        .as_ref()
        .expect("attribution state")
        .context
        .clone();
    let output = git_with_givn_path(&root, &["commit", "-m", &message], context.as_ref());
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .last_output = Some(output);
    if world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .is_some_and(|output| output.status.success())
    {
        let hash = git(&root, &["rev-parse", "HEAD"]);
        assert!(hash.status.success());
        world
            .git_attribution
            .as_mut()
            .expect("attribution state")
            .commits
            .push(String::from_utf8_lossy(&hash.stdout).trim().to_string());
    }
}

#[when(
    regex = r##"^the current attribution context changes to LLM "([^"]+)", reasoning mode "([^"]+)", and harness "([^"]+)"$"##
)]
fn when_context_changes(world: &mut GivnWorld, llm: String, reasoning: String, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        llm: Some(llm),
        reasoning: Some(reasoning),
        harness: Some(harness),
    });
}

#[when("the consumer initializes Givn in the project")]
fn when_consumer_initializes_givn(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let output = common::run_givn(env, &["init", "--agents", "opencode,claude"]);
    assert!(
        output.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.last_output = Some(output);
}

#[then("the commit succeeds")]
fn then_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .expect("commit output");
    assert!(
        output.status.success(),
        "commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r##"^the commit committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    let committer = String::from_utf8_lossy(&output.stdout);
    assert!(committer.contains(&format!("LLM: {llm} via {harness}")));
}

#[then("the commit message ends with the current Givn version, LLM, reasoning mode, and harness")]
fn then_message_has_complete_attribution(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.trim_end().ends_with(&format!(
        "Co-Authored-By: openai/gpt-5.6-luna/high@opencode with Givn {}",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

#[then("the commit message contains only the managed co-author attribution")]
fn then_message_has_only_coauthor_attribution(world: &mut GivnWorld) {
    then_message_has_complete_attribution(world);
}

#[then(regex = r#"^the revision message remains "([^"]+)" without managed attribution trailers$"#)]
fn then_revision_message_remains(world: &mut GivnWorld, expected: String) {
    let message = latest_revision_message(world);
    assert_eq!(message.trim(), expected);
    assert!(!message.contains(" with Givn "), "{message}");
}

#[then(regex = r##"^the commit message remains "([^"]+)"$"##)]
fn then_message_remains(world: &mut GivnWorld, expected: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
}

#[then(regex = r##"^the commit message identifies the harness as "([^"]+)"$"##)]
fn then_message_identifies_harness(world: &mut GivnWorld, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!("@{harness} with Givn ")));
}

#[then("the commit message marks the LLM and reasoning mode as unavailable")]
fn then_message_marks_missing(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains("Co-Authored-By: unavailable/unavailable@"));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

#[then("the existing hook marker is present")]
fn then_existing_marker_present(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    assert_eq!(
        std::fs::read_to_string(root.join("existing-hook.marker")).expect("existing marker"),
        "marker"
    );
}

#[then("the commit message contains the Givn attribution trailers")]
fn then_message_contains_trailers(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains("Co-Authored-By:"));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM-Harness:"));
}

#[then(regex = r##"^the first commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_first_commit_identifies(
    world: &mut GivnWorld,
    llm: String,
    reasoning: String,
    harness: String,
) {
    assert_commit_attribution(world, 0, &llm, &reasoning, &harness);
}

#[then(regex = r##"^the second commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_second_commit_identifies(
    world: &mut GivnWorld,
    llm: String,
    reasoning: String,
    harness: String,
) {
    assert_commit_attribution(world, 1, &llm, &reasoning, &harness);
}

fn assert_commit_attribution(
    world: &mut GivnWorld,
    index: usize,
    llm: &str,
    reasoning: &str,
    harness: &str,
) {
    let root = world.env.as_ref().expect("project root").root();
    let hash = world
        .git_attribution
        .as_ref()
        .expect("attribution state")
        .commits
        .get(index)
        .expect("commit hash");
    let committer_output = git(&root, &["show", "-s", "--format=%cn", hash]);
    assert!(committer_output.status.success());
    let committer = String::from_utf8_lossy(&committer_output.stdout);
    assert!(committer.contains(&format!("LLM: {llm} via {harness}")));

    let output = git(&root, &["show", "-s", "--format=%B", hash]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!(
        "Co-Authored-By: {llm}/{reasoning}@{harness} with Givn {}",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

pub(crate) fn git(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git")
}

pub(crate) fn make_executable(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .expect("hook metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).expect("make hook executable");
    }
}

pub(crate) fn git_with_givn_path(
    root: &std::path::Path,
    args: &[&str],
    context: Option<&AttributionContext>,
) -> Output {
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_givn"));
    let binary_dir = binary.parent().expect("givn binary directory");
    let mut paths = vec![binary_dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    let path = std::env::join_paths(paths).expect("join Git hook PATH");
    let mut command = Command::new("git");
    command
        .args(args)
        .current_dir(root)
        .env("PATH", path)
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS")
        .env_remove("GIT_COMMITTER_NAME")
        .env_remove("GIT_COMMITTER_EMAIL");
    if let Some(context) = context {
        command.env("GIVN_ATTRIBUTION_ACTIVE", "1");
        if let Some(value) = &context.llm {
            command.env("GIVN_ATTRIBUTION_LLM", value);
        }
        if let Some(value) = &context.reasoning {
            command.env("GIVN_ATTRIBUTION_REASONING", value);
        }
        if let Some(value) = &context.harness {
            command.env("GIVN_ATTRIBUTION_HARNESS", value);
        }
        if let (Some(llm), Some(harness)) = (&context.llm, &context.harness) {
            command.env("GIT_COMMITTER_NAME", format!("LLM: {llm} via {harness}"));
        }
    }
    command.output().expect("run git with givn")
}

#[given("an initialized Fossil project")]
fn given_initialized_fossil_project(world: &mut GivnWorld) {
    world.env = Some(common::make_fossil_root());
}

#[when(
    regex = r##"^the consumer commits with message "([^"]+)" attributing LLM "([^"]+)", reasoning "([^"]+)", and harness "([^"]+)"$"##
)]
fn when_givn_commit_with_context(
    world: &mut GivnWorld,
    message: String,
    llm: String,
    reasoning: String,
    harness: String,
) {
    let env = world.env.as_ref().expect("project root");
    std::fs::write(env.root().join("fixture.txt"), &message).expect("write fixture");
    let output = common::run_givn_with_envs(
        env,
        &["commit", "fixture.txt", "-m", &message],
        &[
            ("GIVN_ATTRIBUTION_ACTIVE", "1"),
            ("GIVN_ATTRIBUTION_LLM", &llm),
            ("GIVN_ATTRIBUTION_REASONING", &reasoning),
            ("GIVN_ATTRIBUTION_HARNESS", &harness),
        ],
    );
    world.last_output = Some(output);
}

#[when(regex = r##"^the consumer commits with message "([^"]+)" without attribution context$"##)]
fn when_givn_commit_without_context(world: &mut GivnWorld, message: String) {
    let env = world.env.as_ref().expect("project root");
    std::fs::write(env.root().join("fixture.txt"), &message).expect("write fixture");
    // Strip the harness attribution context explicitly: the outer suite may
    // run inside a harness that exports it.
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_givn"));
    command
        .args(["commit", "fixture.txt", "-m", &message])
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .env("FOSSIL_USER", "fixture")
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS");
    let output = command.output().expect("spawn givn");
    world.last_output = Some(output);
}

/// The message of the most recent revision, on either backend.
fn latest_revision_message(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("project root");
    match env.backend() {
        common::BackendKind::Git => {
            let out = git(&env.root(), &["log", "-1", "--format=%B"]);
            String::from_utf8_lossy(&out.stdout).into_owned()
        }
        common::BackendKind::Fossil => {
            crate::steps::archive_proof_support::fossil_latest_comment(env)
        }
    }
}

#[then("the revision message carries the managed attribution trailer")]
fn then_message_has_trailer(world: &mut GivnWorld) {
    let message = latest_revision_message(world);
    assert!(
        message.contains("Co-Authored-By:") && message.contains(" with Givn "),
        "the revision message carries no managed trailer: {message}"
    );
}

#[then("the revision message carries no managed attribution trailer")]
fn then_message_has_no_trailer(world: &mut GivnWorld) {
    let message = latest_revision_message(world);
    assert!(
        !message.contains(" with Givn "),
        "the revision message unexpectedly carries the managed trailer: {message}"
    );
}

#[when(regex = r#"^the consumer commits only "([^"]+)" with message "([^"]+)"$"#)]
fn when_givn_commit_named_path(world: &mut GivnWorld, path: String, message: String) {
    let env = world.env.as_ref().expect("project root");
    let output = common::run_givn(env, &["commit", "-m", &message, &path]);
    world.last_output = Some(output);
}

#[then(regex = r#"^the latest revision does not contain "([^"]+)"$"#)]
fn then_latest_revision_lacks(world: &mut GivnWorld, path: String) {
    assert!(
        !crate::steps::archive_proof_support::revision_contains(world, &path),
        "the latest revision unexpectedly contains {path}"
    );
}

#[given(regex = r#"^an initialized (Git|Fossil) project with a change to "([^"]+)"$"#)]
fn given_selected_project(world: &mut GivnWorld, backend: String, path: String) {
    world.env = Some(if backend == "Git" {
        common::make_init_root()
    } else {
        common::make_fossil_root()
    });
    let env = world.env.as_ref().unwrap();
    std::fs::write(env.path(&path), "selected working content").unwrap();
}

#[when(
    regex = r#"^the consumer commits only "([^"]+)" with message "([^"]+)" attributing LLM "([^"]+)", reasoning "([^"]+)", and harness "([^"]+)"$"#
)]
fn when_selected_with_context(
    world: &mut GivnWorld,
    path: String,
    message: String,
    llm: String,
    reasoning: String,
    harness: String,
) {
    let env = world.env.as_ref().unwrap();
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["commit", &path, "-m", &message],
        &[
            ("GIVN_ATTRIBUTION_ACTIVE", "1"),
            ("GIVN_ATTRIBUTION_LLM", &llm),
            ("GIVN_ATTRIBUTION_REASONING", &reasoning),
            ("GIVN_ATTRIBUTION_HARNESS", &harness),
        ],
    ));
}

#[given(
    regex = r#"^an initialized Fossil project with pending changes to "([^"]+)" and "([^"]+)"$"#
)]
fn given_fossil_selected_changes(world: &mut GivnWorld, first: String, second: String) {
    world.env = Some(common::make_fossil_root());
    let env = world.env.as_ref().unwrap();
    for path in [&first, &second] {
        std::fs::write(env.path(path), "initial").unwrap();
    }
    common::fossil_commit_all(env, "initial paths");
    for path in [&first, &second] {
        std::fs::write(env.path(path), "selected working content").unwrap();
    }
}

#[given(regex = r#"^an unrelated pending addition "([^"]+)"$"#)]
fn given_fossil_pending_addition(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().unwrap();
    std::fs::write(env.path(&path), "pending addition").unwrap();
    assert!(common::fossil(env, &["add", &path]).status.success());
}

#[then(regex = r#"^the revision contains only "([^"]+)"$"#)]
fn then_revision_only(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().unwrap();
    match env.backend() {
        common::BackendKind::Git => {
            let output = git(
                &env.root(),
                &[
                    "diff-tree",
                    "--root",
                    "--no-commit-id",
                    "--name-only",
                    "-r",
                    "HEAD",
                ],
            );
            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), path);
        }
        common::BackendKind::Fossil => {
            let checkout = givn::fossil::FossilCheckout::at(&env.root());
            let parent = checkout
                .parent_revision(&checkout.revision().unwrap())
                .unwrap()
                .unwrap();
            let output = common::fossil(
                env,
                &["diff", "--from", &parent, "--to", "current", "--brief"],
            );
            assert!(output.status.success());
            let changed: Vec<String> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| line.split_whitespace().last().unwrap().to_string())
                .collect();
            assert_eq!(changed, vec![path]);
        }
    }
}

#[then("the pending changes and addition for the other paths remain unchanged")]
fn then_fossil_others_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    assert_eq!(
        std::fs::read_to_string(env.path("second.txt")).unwrap(),
        "selected working content"
    );
    assert_eq!(
        std::fs::read_to_string(env.path("third.txt")).unwrap(),
        "pending addition"
    );
    let output = common::fossil(env, &["changes", "--differ"]);
    assert!(output.status.success());
    let changes = String::from_utf8_lossy(&output.stdout);
    assert!(changes.contains("EDITED     second.txt"), "{changes}");
    assert!(changes.contains("ADDED      third.txt"), "{changes}");
    assert!(!changes.contains("first.txt"), "{changes}");
}

#[given("a Git project using `.githooks` with a user commit-msg hook")]
#[given("a new Git project with `core.hooksPath` set to `.githooks`")]
#[given("a new Git project with `core.hooksPath` set to `.githooks` and a user commit-msg hook")]
fn given_shared_user_hook(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    common::git_init(&env);
    assert!(
        git(&env.root(), &["config", "core.hooksPath", ".githooks"])
            .status
            .success()
    );
    std::fs::create_dir_all(env.path(".githooks")).unwrap();
    let hook = env.path(".githooks/commit-msg");
    std::fs::write(&hook, "#!/bin/sh\nprintf 'user\\n' >> hook-marker\n").unwrap();
    make_executable(&hook);
    world.env = Some(env);
}

#[given("an existing user commit-msg hook that writes a marker")]
fn given_user_hook_marker(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    assert!(
        std::fs::read_to_string(env.path(".githooks/commit-msg"))
            .unwrap()
            .contains("hook-marker")
    );
}

#[when(
    "the consumer initializes Givn twice and then creates a human commit without attribution context"
)]
fn when_init_twice_human_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    for _ in 0..2 {
        let output = common::run_givn(env, &["init", "--agents", "opencode,claude,omp"]);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    create_bounded_human_commit(world);
}

fn create_bounded_human_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    std::fs::write(env.path("human.txt"), "human content").unwrap();
    assert!(git(&env.root(), &["add", "human.txt"]).status.success());
    world.last_output = Some(
        Command::new("timeout")
            .args(["5", "git", "commit", "-m", "human message"])
            .current_dir(env.root())
            .env_remove("GIVN_ATTRIBUTION_ACTIVE")
            .env_remove("GIVN_ATTRIBUTION_LLM")
            .env_remove("GIVN_ATTRIBUTION_HARNESS")
            .output()
            .unwrap(),
    );
}

#[then("the commit succeeds within the hook completion deadline")]
#[then("the commit succeeds without waiting indefinitely")]
fn then_bounded_human_success(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then("the user hook runs exactly once and remains preserved")]
fn then_user_hook_once(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    assert_eq!(
        std::fs::read_to_string(env.path("hook-marker")).unwrap(),
        "user\n"
    );
    assert!(
        std::fs::read_to_string(env.path(".githooks/commit-msg.givn-original"))
            .unwrap()
            .contains("hook-marker")
    );
}

#[then("no launcher invokes itself directly or through a saved hook")]
#[then("the installed hook does not dispatch itself")]
fn then_no_launcher_recursion(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    let shared = std::fs::read_to_string(env.path(".githooks/commit-msg")).unwrap();
    assert!(!shared.contains("exec \"$root/.githooks/commit-msg\""));
    let backup = std::fs::read_to_string(env.path(".githooks/commit-msg.givn-original")).unwrap();
    assert!(!backup.contains("givn commit-attribution launcher"));
}

#[then("no managed attribution is invented")]
#[then("the human commit message is unchanged")]
fn then_human_message_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    let output = git(&env.root(), &["log", "-1", "--format=%B"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "human message\n\n");
}

#[then(
    "a subsequent human Git commit succeeds and runs the user hook exactly once without recursive dispatch"
)]
fn then_subsequent_human_commit(world: &mut GivnWorld) {
    create_bounded_human_commit(world);
    then_bounded_human_success(world);
    then_user_hook_once(world);
    then_no_launcher_recursion(world);
}

#[then("the command exits 0 and reports installed attribution plumbing")]
fn then_installed_success(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("commit attribution"));
}

#[then("the checked-in attribution hook and selected harness integrations exist")]
fn then_integrations_exist(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".claude/hooks/givn-attribution.sh",
        ".omp/hooks/pre/givn-attribution.js",
        ".maki/lua/givn-attribution.lua",
    ] {
        assert!(env.path(path).exists(), "missing {path}");
    }
}

#[then("stdout reports the shared Git hook and each selected harness attribution integration")]
fn then_installation_report(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".claude/hooks/givn-attribution.sh",
        ".omp/hooks/pre/givn-attribution.js",
        ".maki/lua/givn-attribution.lua",
    ] {
        assert!(stdout.contains(path), "missing {path}: {stdout}");
    }
}

#[then("the report states that the existing user hook was preserved")]
fn then_preservation_report(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("existing user hook preserved"));
}

// --- Explicit-path publication and safe hook scenarios ---------------------

fn attribution_state(world: &mut GivnWorld) -> &mut GitAttributionState {
    world
        .git_attribution
        .get_or_insert_with(GitAttributionState::default)
}

fn head_revision(env: &common::TestEnv) -> String {
    match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&git(&env.root(), &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        common::BackendKind::Fossil => givn::fossil::FossilCheckout::at(&env.root())
            .revision()
            .expect("read Fossil revision"),
    }
}

/// The paths the latest revision changed, as `<status>\t<path>` lines.
fn revision_changes(env: &common::TestEnv) -> Vec<String> {
    match env.backend() {
        common::BackendKind::Git => String::from_utf8_lossy(
            &git(
                &env.root(),
                &[
                    "diff-tree",
                    "--root",
                    "--no-commit-id",
                    "--name-status",
                    "-r",
                    "HEAD",
                ],
            )
            .stdout,
        )
        .lines()
        .map(|line| {
            let mut fields = line.splitn(2, '\t');
            format!(
                "{}\t{}",
                fields.next().unwrap_or_default(),
                fields.next().unwrap_or_default()
            )
        })
        .collect(),
        common::BackendKind::Fossil => {
            let checkout = givn::fossil::FossilCheckout::at(&env.root());
            let revision = checkout.revision().expect("read Fossil revision");
            let parent = checkout
                .parent_revision(&revision)
                .expect("read Fossil parent")
                .expect("Fossil parent");
            String::from_utf8_lossy(
                &common::fossil(
                    env,
                    &["diff", "--from", &parent, "--to", "current", "--brief"],
                )
                .stdout,
            )
            .lines()
            .filter_map(|line| {
                let (status, path) = line.split_once(char::is_whitespace)?;
                Some(format!("{status}\t{}", path.trim()))
            })
            .collect()
        }
    }
}

fn collect_fixture_files(root: &std::path::Path, relative: &str, out: &mut Vec<String>) {
    let directory = root.join(relative);
    let Ok(entries) = std::fs::read_dir(&directory) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if matches!(
            name.as_str(),
            ".git" | ".fslckout" | ".fossil" | ".fixture-bin"
        ) || (relative == "givn" && name == "history")
        {
            continue;
        }
        let child = if relative.is_empty() {
            name
        } else {
            format!("{relative}/{name}")
        };
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_files(root, &child, out);
        } else {
            out.push(child);
        }
    }
}

/// Index/worktree state for the byte-for-byte preservation assertions.
fn workspace_snapshot(env: &common::TestEnv) -> String {
    let mut files = Vec::new();
    collect_fixture_files(&env.root(), "", &mut files);
    files.sort();
    let mut snapshot = String::new();
    for relative in files {
        let content = std::fs::read(env.path(&relative)).unwrap_or_default();
        snapshot.push_str(&relative);
        snapshot.push('\u{1f}');
        snapshot.push_str(&String::from_utf8_lossy(&content));
        snapshot.push('\n');
    }
    match env.backend() {
        common::BackendKind::Git => {
            snapshot.push_str(&String::from_utf8_lossy(
                &git(&env.root(), &["status", "--porcelain=v1", "-uall"]).stdout,
            ));
            snapshot.push_str(&String::from_utf8_lossy(
                &git(&env.root(), &["diff", "--cached", "--name-status"]).stdout,
            ));
        }
        common::BackendKind::Fossil => {
            snapshot.push_str(&String::from_utf8_lossy(
                &common::fossil(env, &["changes", "--differ"]).stdout,
            ));
            snapshot.push_str(&String::from_utf8_lossy(
                &common::fossil(env, &["extras"]).stdout,
            ));
        }
    }
    snapshot
}

fn git_head(env: &common::TestEnv) -> String {
    String::from_utf8_lossy(&git(&env.root(), &["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_string()
}

#[given(regex = r#"^changed "([^"]+)" beside unrelated staged and unstaged work$"#)]
fn given_changed_beside_unrelated_work(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("project root");
    std::fs::write(env.path(&path), "selected working content").expect("write selected path");
    std::fs::write(env.path("staged-other.txt"), "staged unrelated content")
        .expect("write staged other");
    assert!(
        git(&env.root(), &["add", "staged-other.txt"])
            .status
            .success()
    );
    std::fs::write(env.path("unstaged-other.txt"), "unstaged unrelated content")
        .expect("write unstaged other");
    attribution_state(world).requested_path = Some(path);
}

#[then("the command succeeds and reports the created revision")]
fn then_reports_created_revision(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("committed revision"));
}

#[then("the revision contains only the supplied path")]
fn then_revision_contains_only_supplied_path(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let path = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.requested_path.clone())
        .expect("requested path");
    let changed: Vec<String> = revision_changes(env)
        .into_iter()
        .map(|line| {
            line.split_once('\t')
                .map(|(_, p)| p.to_string())
                .unwrap_or(line)
        })
        .collect();
    assert_eq!(changed, vec![path]);
}

#[then("unrelated index entries and working changes remain unchanged")]
fn then_unrelated_work_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let staged =
        String::from_utf8_lossy(&git(&env.root(), &["diff", "--cached", "--name-status"]).stdout)
            .to_string();
    assert_eq!(staged.trim(), "A\tstaged-other.txt");
    assert_eq!(
        std::fs::read_to_string(env.path("staged-other.txt")).unwrap(),
        "staged unrelated content"
    );
    assert_eq!(
        std::fs::read_to_string(env.path("unstaged-other.txt")).unwrap(),
        "unstaged unrelated content"
    );
    let status = String::from_utf8_lossy(&git(&env.root(), &["status", "--porcelain=v1"]).stdout)
        .to_string();
    assert!(status.contains("?? unstaged-other.txt"), "{status}");
}

#[given(regex = r#"^a Git project without attribution context and with a changed "([^"]+)"$"#)]
fn given_git_without_context(world: &mut GivnWorld, path: String) {
    world.env = Some(common::make_init_root());
    attribution_state(world).requested_path = Some(path.clone());
    std::fs::write(
        world.env.as_ref().unwrap().path(&path),
        "selected working content",
    )
    .expect("write selected path");
}

#[given(
    regex = r#"^an initialized "?(Git|Fossil)"? project with modified tracked files and an untracked file$"#
)]
fn given_initialized_with_modified_and_untracked(world: &mut GivnWorld, backend: String) {
    let env = if backend == "Git" {
        common::make_init_root()
    } else {
        common::make_fossil_root()
    };
    std::fs::write(env.path("tracked.txt"), "tracked base").expect("write tracked");
    common::commit_fixture(&env);
    std::fs::write(env.path("tracked.txt"), "tracked modified").expect("modify tracked");
    std::fs::write(env.path("untracked.txt"), "untracked content").expect("write untracked");
    attribution_state(world).revision_before = Some(head_revision(&env));
    attribution_state(world).snapshot = Some(workspace_snapshot(&env));
    world.env = Some(env);
}

#[given("unrelated pending selections have been recorded")]
fn given_pending_selections(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    match env.backend() {
        common::BackendKind::Git => {
            assert!(git(&env.root(), &["add", "untracked.txt"]).status.success());
        }
        common::BackendKind::Fossil => {
            assert!(
                common::fossil(env, &["add", "untracked.txt"])
                    .status
                    .success()
            );
        }
    }
    attribution_state(world).snapshot = Some(workspace_snapshot(env));
}

#[then("the command exits nonzero and explains that explicit paths are required")]
fn then_explicit_paths_required(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "commit unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("explicit paths are required"), "{stderr}");
}

#[then("no revision is created and no file is staged or added")]
fn then_no_revision_or_staging(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let before = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.revision_before.clone())
        .expect("revision before");
    assert_eq!(head_revision(env), before);
}

#[then("existing pending selections and all working changes remain unchanged")]
fn then_pending_selections_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let before = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.snapshot.clone())
        .expect("workspace snapshot");
    assert_eq!(workspace_snapshot(env), before);
}

#[given(
    regex = r#"^a Git project where "([^"]+)" has staged content and a different working version$"#
)]
fn given_staged_and_working_versions(world: &mut GivnWorld, path: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    std::fs::write(env.path(&path), "staged version").expect("write staged version");
    assert!(git(&env.root(), &["add", &path]).status.success());
    std::fs::write(env.path(&path), "working version").expect("write working version");
    attribution_state(world).requested_path = Some(path);
}

#[then(regex = r#"^the revision contains the selected working version of "([^"]+)" only$"#)]
fn then_revision_has_working_version(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("project root");
    let shown = git(&env.root(), &["show", &format!("HEAD:{path}")]);
    assert!(shown.status.success());
    assert_eq!(String::from_utf8_lossy(&shown.stdout), "working version");
    let changed: Vec<String> = revision_changes(env)
        .into_iter()
        .map(|line| {
            line.split_once('\t')
                .map(|(_, p)| p.to_string())
                .unwrap_or(line)
        })
        .collect();
    assert_eq!(changed, vec![path]);
}

#[then(regex = r#"^the unrelated staged content of "([^"]+)" is unchanged$"#)]
fn then_unrelated_staged_content_unchanged(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("project root");
    let staged = String::from_utf8_lossy(
        &git(
            &env.root(),
            &["diff", "--cached", "--name-status", "--", &path],
        )
        .stdout,
    )
    .to_string();
    assert!(staged.contains('A') && staged.contains(&path), "{staged}");
    let shown = git(&env.root(), &["show", &format!(":{path}")]);
    assert!(shown.status.success());
    assert_eq!(
        String::from_utf8_lossy(&shown.stdout),
        "unrelated staged content"
    );
}

#[then("no staged content of another path is included in the revision")]
fn then_no_other_staged_content(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let path = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.requested_path.clone())
        .expect("requested path");
    let changed: Vec<String> = revision_changes(env)
        .into_iter()
        .map(|line| {
            line.split_once('\t')
                .map(|(_, p)| p.to_string())
                .unwrap_or(line)
        })
        .collect();
    assert_eq!(changed, vec![path]);
}

#[given(
    regex = r#"^an initialized "?(Git|Fossil)"? project whose requested path cannot be isolated safely from unrelated pending work$"#
)]
fn given_unisolatable_project(world: &mut GivnWorld, backend: String) {
    let env = if backend == "Git" {
        let env = common::make_init_root();
        std::fs::write(env.path("batch.txt"), "base").expect("write base");
        common::git_commit_all(&env, "fixture: base");
        let branch = String::from_utf8_lossy(
            &git(&env.root(), &["rev-parse", "--abbrev-ref", "HEAD"]).stdout,
        )
        .trim()
        .to_string();
        assert!(
            git(&env.root(), &["checkout", "-q", "-b", "other"])
                .status
                .success()
        );
        std::fs::write(env.path("batch.txt"), "theirs").expect("write theirs");
        common::git_commit_all(&env, "fixture: theirs");
        assert!(
            git(&env.root(), &["checkout", "-q", &branch])
                .status
                .success()
        );
        std::fs::write(env.path("batch.txt"), "ours").expect("write ours");
        common::git_commit_all(&env, "fixture: ours");
        let merged = git(&env.root(), &["merge", "other"]);
        assert!(!merged.status.success(), "expected a merge conflict");
        env
    } else {
        let env = common::make_fossil_root();
        std::fs::write(env.path("batch.txt"), "base").expect("write base");
        assert!(common::fossil(&env, &["add", "batch.txt"]).status.success());
        common::fossil_commit_all(&env, "fixture: base");
        assert!(
            common::fossil(&env, &["branch", "new", "other", "current"])
                .status
                .success()
        );
        assert!(common::fossil(&env, &["update", "other"]).status.success());
        std::thread::sleep(std::time::Duration::from_millis(1100));
        std::fs::write(env.path("batch.txt"), "theirs").expect("write theirs");
        common::fossil_commit_all(&env, "fixture: theirs");
        assert!(common::fossil(&env, &["update", "trunk"]).status.success());
        std::thread::sleep(std::time::Duration::from_millis(1100));
        std::fs::write(env.path("batch.txt"), "ours").expect("write ours");
        common::fossil_commit_all(&env, "fixture: ours");
        let merged = common::fossil(&env, &["merge", "other"]);
        assert!(merged.status.success(), "fossil merge failed");
        env
    };
    attribution_state(world).requested_path = Some("batch.txt".to_string());
    attribution_state(world).revision_before = Some(head_revision(&env));
    attribution_state(world).snapshot = Some(workspace_snapshot(&env));
    world.env = Some(env);
}

#[when("the consumer commits only the requested path")]
fn when_commit_requested_path(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let path = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.requested_path.clone())
        .expect("requested path");
    world.last_output = Some(common::run_givn(
        env,
        &["commit", &path, "-m", "fix: selected path"],
    ));
}

#[then("the command refuses before creating a revision")]
fn then_refuses_before_revision(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "commit unexpectedly succeeded");
    let env = world.env.as_ref().expect("project root");
    let before = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.revision_before.clone())
        .expect("revision before");
    assert_eq!(head_revision(env), before);
}

#[then("pending selections and working changes remain unchanged")]
fn then_pending_and_working_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let before = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.snapshot.clone())
        .expect("workspace snapshot");
    assert_eq!(workspace_snapshot(env), before);
}

#[then("the diagnostic explains the isolation problem")]
fn then_isolation_diagnostic(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("unmerged index") || stderr.contains("pending merge"),
        "{stderr}"
    );
}

#[given(
    "an initialized Git project with no attribution context and an unavailable attribution helper"
)]
fn given_unavailable_helper(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    attribution_state(world);
}

#[when("the consumer creates a human Git commit")]
fn when_human_commit_without_helper(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let bin_dir = env.path(".fixture-bin");
    std::fs::create_dir_all(&bin_dir).expect("create fixture bin directory");
    let git_path = which_git();
    std::os::unix::fs::symlink(&git_path, bin_dir.join("git")).expect("link git into fixture bin");
    std::fs::write(env.path("human.txt"), "human content").expect("write human content");
    assert!(git(&env.root(), &["add", "human.txt"]).status.success());
    let start = std::time::Instant::now();
    let output = std::process::Command::new(&git_path)
        .args(["commit", "-m", "human message"])
        .current_dir(env.root())
        .env("PATH", &bin_dir)
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS")
        .output()
        .expect("spawn human commit");
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "the hook waited indefinitely for the unavailable helper"
    );
    world.last_output = Some(output);
}

fn which_git() -> PathBuf {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path)
        .map(|dir| dir.join("git"))
        .find(|candidate| candidate.is_file())
        .expect("git executable on PATH")
}

#[given(regex = r#"^an initialized "?(Git|Fossil)"? project with a tracked file "([^"]+)"$"#)]
fn given_tracked_file(world: &mut GivnWorld, backend: String, path: String) {
    let env = if backend == "Git" {
        common::make_init_root()
    } else {
        common::make_fossil_root()
    };
    std::fs::write(env.path(&path), "obsolete content").expect("write tracked file");
    common::commit_fixture(&env);
    world.env = Some(env);
}

#[given(regex = r#"^the developer has deleted "([^"]+)" with no unrelated pending work$"#)]
fn given_deleted_tracked_file(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("project root");
    std::fs::remove_file(env.path(&path)).expect("delete tracked file");
}

#[then(regex = r#"^the command exits 0 and the revision records the deletion of "([^"]+)" only$"#)]
fn then_revision_records_deletion(world: &mut GivnWorld, path: String) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let env = world.env.as_ref().expect("project root");
    let changes = revision_changes(env);
    assert_eq!(changes.len(), 1, "{changes:?}");
    let (status, changed) = changes[0].split_once('\t').expect("status and path");
    assert!(status.starts_with('D'), "{changes:?}");
    assert_eq!(changed, path);
}

#[then(regex = r#"^"([^"]+)" remains absent from the working tree$"#)]
fn then_absent_from_working_tree(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("project root");
    assert!(!env.path(&path).exists());
}

#[given(regex = r#"^an initialized Git project with a working change to "([^"]+)"$"#)]
fn given_git_working_change(world: &mut GivnWorld, path: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    std::fs::write(env.path(&path), "selected working content").expect("write working change");
    attribution_state(world).requested_path = Some(path);
}

#[given(regex = r#"^"([^"]+)" has an unrelated staged (addition|deletion|change)$"#)]
fn given_unrelated_staged_operation(world: &mut GivnWorld, path: String, operation: String) {
    let env = world.env.as_ref().expect("project root");
    if operation != "deletion" {
        std::fs::write(env.path(&path), "unrelated staged content").expect("write other");
        assert!(git(&env.root(), &["add", &path]).status.success());
    } else {
        std::fs::write(env.path(&path), "unrelated content").expect("write other");
        assert!(git(&env.root(), &["add", &path]).status.success());
        assert!(
            git(&env.root(), &["commit", "-m", "fixture: other"])
                .status
                .success()
        );
        std::fs::remove_file(env.path(&path)).expect("delete other");
        assert!(
            git(&env.root(), &["add", "-A", "--", &path])
                .status
                .success()
        );
    }
    let snapshot = workspace_snapshot(env);
    let revision = git_head(env);
    attribution_state(world).snapshot = Some(snapshot);
    attribution_state(world).revision_before = Some(revision);
}

#[then(
    "the command either commits only the selected working content or refuses an isolation conflict before mutation"
)]
fn then_commit_or_isolation_refusal(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    let env = world.env.as_ref().expect("project root");
    if output.status.success() {
        let path = world
            .git_attribution
            .as_ref()
            .and_then(|state| state.requested_path.clone())
            .expect("requested path");
        let shown = git(&env.root(), &["show", &format!("HEAD:{path}")]);
        assert_eq!(
            String::from_utf8_lossy(&shown.stdout),
            "selected working content"
        );
    } else {
        let before = world
            .git_attribution
            .as_ref()
            .and_then(|state| state.revision_before.clone())
            .expect("revision before");
        assert_eq!(git_head(env), before);
    }
}

#[then(
    regex = r#"^the staged (addition|deletion) of "([^"]+)" and its working state remain unchanged$"#
)]
fn then_staged_operation_unchanged(world: &mut GivnWorld, operation: String, path: String) {
    let env = world.env.as_ref().expect("project root");
    let staged = String::from_utf8_lossy(
        &git(
            &env.root(),
            &["diff", "--cached", "--name-status", "--", &path],
        )
        .stdout,
    )
    .to_string();
    if operation != "deletion" {
        assert_eq!(staged.trim(), format!("A\t{path}"));
        assert_eq!(
            std::fs::read_to_string(env.path(&path)).unwrap(),
            "unrelated staged content"
        );
        let shown = git(&env.root(), &["show", &format!(":{path}")]);
        assert_eq!(
            String::from_utf8_lossy(&shown.stdout),
            "unrelated staged content"
        );
    } else {
        assert_eq!(staged.trim(), format!("D\t{path}"));
        assert!(!env.path(&path).exists());
    }
}

#[then(regex = r#"^a successful revision contains no addition or deletion of "([^"]+)"$"#)]
fn then_successful_revision_excludes(world: &mut GivnWorld, path: String) {
    if !crate::steps::output(world).status.success() {
        return;
    }
    let env = world.env.as_ref().expect("project root");
    for change in revision_changes(env) {
        assert!(
            !change.ends_with(&format!("\t{path}")),
            "the revision touched {path}: {change}"
        );
    }
}

#[then(
    "an isolation refusal creates no revision and leaves all index entries and working changes unchanged"
)]
fn then_isolation_refusal_unchanged(world: &mut GivnWorld) {
    if crate::steps::output(world).status.success() {
        return;
    }
    let env = world.env.as_ref().expect("project root");
    let state = world.git_attribution.as_ref().expect("attribution state");
    assert_eq!(
        git_head(env),
        state.revision_before.clone().expect("revision before")
    );
    assert_eq!(
        workspace_snapshot(env),
        state.snapshot.clone().expect("snapshot")
    );
}
