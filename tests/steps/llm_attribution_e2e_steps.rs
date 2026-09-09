//! E2E step definitions for the llm-attribution capability.

use cucumber::{given, then, when};

use super::GivnWorld;

#[given("a project root without a givn/ directory and with a Git repository")]
fn given_bare_git_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when("the consumer runs `givn init --agents opencode,claude`")]
fn when_consumer_runs_init(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the checked-in commit attribution hook exists")]
fn then_checked_in_hook_exists(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the Git repository uses the checked-in commit attribution hook")]
fn then_git_uses_hook(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the OpenCode attribution integration exists")]
fn then_opencode_integration_exists(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("the Claude Code attribution integration exists")]
fn then_claude_integration_exists(_world: &mut GivnWorld) {
    unimplemented!()
}
