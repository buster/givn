//! E2E step definitions for installation and upgrade reporting.

use cucumber::{given, then};

use super::GivnWorld;
use super::llm_attribution_installation_steps::git;
use crate::common;

#[then("stdout reports the shared Git commit attribution hook")]
fn then_stdout_reports_hook(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("last Givn output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("commit attribution: .githooks/commit-msg"));
}

#[then("stdout reports the OpenCode attribution integration")]
fn then_stdout_reports_opencode(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("last Givn output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(".opencode/plugins/givn-attribution.js"));
}

#[then("stdout reports the Claude Code attribution integration")]
fn then_stdout_reports_claude(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("last Givn output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(".claude/hooks/givn-attribution.sh"));
}

#[then("stdout reports the omp attribution integration")]
fn then_stdout_reports_omp(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("last Givn output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(".omp/hooks/pre/givn-attribution.js"));
}

#[given("an initialized Git project with attribution plumbing removed after its initial commit")]
fn given_upgrade_project(world: &mut GivnWorld) {
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
    let init = common::run_givn(&env, &["init", "--agents", "opencode,claude,omp"]);
    assert!(init.status.success());
    // The scenario exercises the versioned rules policy: the plumbing is
    // repository content that the managed upgrade refreshes.
    let config = std::fs::read_to_string(root.join("givn/config.yaml")).expect("read config");
    let config = config.replace("rules: local-only", "rules: versioned");
    std::fs::write(root.join("givn/config.yaml"), config).expect("write config");
    let sync = common::run_givn(&env, &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "fixture sync failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["commit", "-m", "initial Givn project"])
            .status
            .success()
    );

    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".claude/hooks/givn-attribution.sh",
        ".claude/settings.local.json",
        ".omp/hooks/pre/givn-attribution.js",
    ] {
        let _ = std::fs::remove_file(root.join(path));
    }
    let hooks_dir =
        String::from_utf8_lossy(&git(&root, &["rev-parse", "--git-path", "hooks"]).stdout)
            .trim()
            .to_string();
    let _ = std::fs::remove_file(root.join(&hooks_dir).join("commit-msg"));
    let _ = std::fs::remove_file(root.join(&hooks_dir).join("commit-msg.givn-original"));
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["commit", "-m", "remove attribution plumbing"])
            .status
            .success()
    );
    world.env = Some(env);
}

#[then("stdout reports that commit attribution plumbing was refreshed")]
fn then_stdout_reports_retrofit(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("last Givn output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("commit attribution"));
    assert!(stdout.contains("Refreshed") || stdout.contains("refreshed"));
}

#[then("the managed upgrade commit contains the checked-in hook and attribution integrations")]
fn then_managed_commit_contains_attribution(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(output.status.success());
    let changed = String::from_utf8_lossy(&output.stdout);
    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".claude/hooks/givn-attribution.sh",
        ".claude/settings.local.json",
        ".omp/hooks/pre/givn-attribution.js",
    ] {
        assert!(changed.contains(path), "managed upgrade omitted {path}");
    }
}
