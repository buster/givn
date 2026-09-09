//! E2E step definitions for the llm-attribution capability.

use std::process::Command;

use cucumber::{given, then};

use super::GivnWorld;
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
