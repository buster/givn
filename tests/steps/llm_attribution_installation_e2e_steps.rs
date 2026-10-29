//! E2E step definitions for installation and upgrade reporting.

use cucumber::{given, then, when};

use super::GivnWorld;
use super::llm_attribution_installation_steps::git;
use super::llm_attribution_steps::make_executable;
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

#[given(
    "a clean initialized Git project using `.githooks` with attribution plumbing missing and a preserved user hook"
)]
fn given_upgrade_project_with_preserved_user_hook(world: &mut GivnWorld) {
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
    assert!(
        git(&root, &["config", "core.hooksPath", ".githooks"])
            .status
            .success()
    );
    std::fs::create_dir_all(root.join(".githooks")).expect("create shared hooks directory");
    let user_hook = root.join(".githooks/commit-msg");
    std::fs::write(&user_hook, "#!/bin/sh\nprintf 'user\\n' >> hook-marker\n")
        .expect("write user hook");
    make_executable(&user_hook);

    let init = common::run_givn(&env, &["init", "--agents", "opencode,claude,omp"]);
    assert!(
        init.status.success(),
        "fixture init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    // The attribution plumbing is repository content in this fixture, so the
    // managed upgrade revision can be checked for the refreshed files.
    let config_path = root.join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).expect("read config");
    std::fs::write(
        &config_path,
        config.replace("rules: local-only", "rules: versioned"),
    )
    .expect("write config");
    let sync = common::run_givn(&env, &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "fixture sync failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );

    assert!(git(&root, &["add", "--all"]).status.success());
    assert!(
        git(&root, &["commit", "-m", "initial Givn project"])
            .status
            .success()
    );

    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".opencode/plugins/givn-timeline.js",
        ".claude/hooks/givn-attribution.sh",
        ".claude/hooks/givn-timeline.sh",
        ".claude/settings.local.json",
        ".omp/hooks/pre/givn-attribution.js",
        ".omp/hooks/pre/givn-timeline.js",
    ] {
        let _ = std::fs::remove_file(root.join(path));
    }
    assert!(git(&root, &["add", "--all"]).status.success());
    assert!(
        git(&root, &["commit", "-m", "remove attribution plumbing"])
            .status
            .success()
    );
    world.env = Some(env);
}

#[then("the command exits 0 and reports refreshed attribution plumbing")]
fn then_upgrade_reports_refreshed_plumbing(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "upgrade failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Refreshed"), "{stdout}");
    assert!(stdout.contains("attribution"), "{stdout}");
}

#[then("a repeated installation preserves the user hook without duplicating dispatch")]
fn then_repeated_installation_preserves_user_hook(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let root = env.root();
    let sync = common::run_givn(env, &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "repeated installation failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let _ = std::fs::remove_file(root.join("hook-marker"));
    std::fs::write(root.join("human.txt"), "human content").expect("write human file");
    assert!(git(&root, &["add", "human.txt"]).status.success());
    let commit = std::process::Command::new("git")
        .args(["commit", "-m", "human message"])
        .current_dir(&root)
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS")
        .output()
        .expect("run human commit");
    assert!(
        commit.status.success(),
        "human commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(root.join("hook-marker")).expect("user hook marker"),
        "user\n"
    );
    assert!(
        std::fs::read_to_string(root.join(".githooks/commit-msg.givn-original"))
            .expect("preserved user hook")
            .contains("hook-marker")
    );
    let shared = std::fs::read_to_string(root.join(".githooks/commit-msg")).expect("shared hook");
    assert!(!shared.contains("exec \"$root/.githooks/commit-msg\""));
    let message = String::from_utf8_lossy(
        &std::process::Command::new("git")
            .args(["show", "-s", "--format=%B", "HEAD"])
            .current_dir(&root)
            .output()
            .expect("read commit message")
            .stdout,
    )
    .to_string();
    assert_eq!(message.trim(), "human message");
}

#[when("the consumer initializes Givn with an explicit agent target")]
fn when_fossil_init_explicit_target(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("Fossil checkout");
    world.last_output = Some(common::run_givn(env, &["init", "--agents", "opencode"]));
}

#[then("stdout names `givn commit` with explicit file paths as the attribution path")]
fn then_stdout_names_explicit_commit_path(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("givn commit path only"), "{stdout}");
    assert!(stdout.contains("explicit file paths"), "{stdout}");
}

#[then("generated guidance states that raw Fossil commits are not stamped")]
fn then_generated_guidance_names_fossil_limit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("Fossil checkout");
    let guidance = std::fs::read_to_string(env.path("AGENTS.md")).expect("read AGENTS.md");
    let normalized = guidance.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("raw Fossil check-in is not"),
        "generated guidance does not state the raw Fossil limitation: {guidance}"
    );
}

#[then("the managed upgrade revision contains the hook and selected attribution integrations")]
fn then_managed_upgrade_revision_contains_selected_attribution(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "--name-status", "--format=", "HEAD"]);
    assert!(output.status.success());
    let changed = String::from_utf8_lossy(&output.stdout);
    for path in [
        ".githooks/commit-msg",
        ".opencode/plugins/givn-attribution.js",
        ".claude/hooks/givn-attribution.sh",
        ".claude/settings.local.json",
        ".omp/hooks/pre/givn-attribution.js",
    ] {
        let line = changed
            .lines()
            .find(|line| line.ends_with(path))
            .unwrap_or_else(|| panic!("managed upgrade revision omitted {path}: {changed}"));
        assert!(
            line.starts_with('A') || line.starts_with('M'),
            "managed upgrade revision did not install {path}: {line}"
        );
    }
}
