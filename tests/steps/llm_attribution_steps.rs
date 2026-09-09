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
fn given_complete_context(
    _world: &mut GivnWorld,
    _llm: String,
    _reasoning: String,
    _harness: String,
) {
    unimplemented!()
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
fn given_partial_context(_world: &mut GivnWorld, _harness: String) {
    unimplemented!()
}

#[given("a Givn project with an existing commit hook that writes a marker")]
fn given_existing_hook(_world: &mut GivnWorld) {
    unimplemented!()
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
}

#[when(
    regex = r##"^the current attribution context changes to LLM "([^"]+)", reasoning mode "([^"]+)", and harness "([^"]+)"$"##
)]
fn when_context_changes(
    _world: &mut GivnWorld,
    _llm: String,
    _reasoning: String,
    _harness: String,
) {
    unimplemented!()
}

#[when("the consumer initializes Givn in the project")]
fn when_consumer_initializes_givn(_world: &mut GivnWorld) {
    unimplemented!()
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
fn then_committer_identifies(_world: &mut GivnWorld, _llm: String, _harness: String) {
    unimplemented!()
}

#[then("the commit message ends with the current Givn version, LLM, reasoning mode, and harness")]
fn then_message_has_complete_attribution(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(regex = r##"^the commit message remains "([^"]+)"$"##)]
fn then_message_remains(world: &mut GivnWorld, expected: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
}

#[then(regex = r##"^the commit message identifies the harness as "([^"]+)"$"##)]
fn then_message_identifies_harness(_world: &mut GivnWorld, _harness: String) {
    unimplemented!()
}

#[then("the commit message marks the LLM and reasoning mode as unavailable")]
fn then_message_marks_missing(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the existing hook marker is present")]
fn then_existing_marker_present(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the commit message contains the Givn attribution trailers")]
fn then_message_contains_trailers(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then(regex = r##"^the first commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_first_commit_identifies(
    _world: &mut GivnWorld,
    _llm: String,
    _reasoning: String,
    _harness: String,
) {
    unimplemented!()
}

#[then(regex = r##"^the second commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_second_commit_identifies(
    _world: &mut GivnWorld,
    _llm: String,
    _reasoning: String,
    _harness: String,
) {
    unimplemented!()
}

fn git(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git")
}

fn git_with_givn_path(
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
    command.args(args).current_dir(root).env("PATH", path);
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
    }
    command.output().expect("run git with givn")
}
