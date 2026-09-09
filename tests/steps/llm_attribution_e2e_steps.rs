//! E2E step definitions for the llm-attribution capability.

use std::process::Command;

use cucumber::{given, then, when};

use super::GivnWorld;
use super::llm_attribution_steps::{
    AttributionContext, GitAttributionState, git, git_with_givn_path,
};
use crate::common;

#[given("a project root without a givn/ directory and with a Git repository")]
fn given_bare_git_project(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let output = Command::new("git")
        .args(["init"])
        .current_dir(env.root())
        .output()
        .expect("run git init");
    assert!(output.status.success());
    world.env = Some(env);
}

#[then("the checked-in commit attribution hook exists")]
fn then_checked_in_hook_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".githooks/commit-msg").is_file());
}

#[then("the Git repository uses the checked-in commit attribution hook")]
fn then_git_uses_hook(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks"])
        .current_dir(env.root())
        .output()
        .expect("read Git hooks path");
    assert!(output.status.success());
    let hooks = env
        .root()
        .join(String::from_utf8_lossy(&output.stdout).trim());
    let launcher = hooks.join("commit-msg");
    assert!(launcher.is_file());
    assert!(
        std::fs::read_to_string(launcher)
            .expect("read launcher")
            .contains("givn commit-attribution launcher")
    );
}

#[then("the OpenCode attribution integration exists")]
fn then_opencode_integration_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".opencode/plugins/givn-attribution.js").is_file());
}

#[then("the Claude Code attribution integration exists")]
fn then_claude_integration_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".claude/hooks/givn-attribution.sh").is_file());
    assert!(env.path(".claude/settings.json").is_file());
}

#[given("a Git project with complete commit attribution context")]
fn given_complete_commit_context(world: &mut GivnWorld) {
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
    let output = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(output.status.success());
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState {
        context: Some(AttributionContext {
            llm: Some("openai/gpt-5.6-luna".to_string()),
            reasoning: Some("high".to_string()),
            harness: Some("opencode".to_string()),
        }),
        ..GitAttributionState::default()
    });
}

#[when(regex = r##"^the consumer creates an attributed Git commit with message "([^"]+)"$"##)]
fn when_consumer_creates_attributed_commit(world: &mut GivnWorld, message: String) {
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

#[then("the attributed commit succeeds")]
fn then_attributed_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .expect("commit output");
    assert!(output.status.success(), "attributed commit failed");
}

#[then(regex = r##"^its committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_attributed_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("LLM: {llm} via {harness}")));
}

#[then("its message ends with the current Givn version, LLM, reasoning mode, and harness")]
fn then_attributed_message_has_complete_context(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!("Givn-Version: {}", env!("CARGO_PKG_VERSION"))));
    assert!(message.contains("LLM: openai/gpt-5.6-luna"));
    assert!(message.contains("LLM-Reasoning: high"));
    assert!(message.contains("LLM-Harness: opencode"));
    assert!(
        message
            .trim_end()
            .ends_with("Givn-Attribution-Status: complete")
    );
}
