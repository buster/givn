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

/// A Git project with OpenCode and Claude selected whose rules are versioned
/// and whose generated files are synchronized but not yet committed.
fn versioned_project_with_both_targets() -> common::TestEnv {
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
    env
}

/// Attribution plumbing that a missing-variant fixture removes after the
/// initial commit.
const ATTRIBUTION_PLUMBING: [&str; 5] = [
    ".githooks/commit-msg",
    ".opencode/plugins/givn-attribution.js",
    ".claude/hooks/givn-attribution.sh",
    ".claude/settings.local.json",
    ".omp/hooks/pre/givn-attribution.js",
];

/// Remove the plumbing, including the launcher Git installed for the hook.
fn remove_attribution_plumbing(root: &std::path::Path) {
    for path in ATTRIBUTION_PLUMBING {
        let _ = std::fs::remove_file(root.join(path));
    }
    let hooks_dir =
        String::from_utf8_lossy(&git(root, &["rev-parse", "--git-path", "hooks"]).stdout)
            .trim()
            .to_string();
    let _ = std::fs::remove_file(root.join(&hooks_dir).join("commit-msg"));
    let _ = std::fs::remove_file(root.join(&hooks_dir).join("commit-msg.givn-original"));
}

#[given("an initialized Git project with attribution plumbing removed after its initial commit")]
fn given_upgrade_project(world: &mut GivnWorld) {
    let env = versioned_project_with_both_targets();
    let root = env.root();
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["commit", "-m", "initial Givn project"])
            .status
            .success()
    );
    remove_attribution_plumbing(&root);
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["commit", "-m", "remove attribution plumbing"])
            .status
            .success()
    );
    world.env = Some(env);
}

/// Subject of the revision that records each prior integration state.
fn prior_state_subject(variant: &str) -> &'static str {
    match variant {
        "missing" => "remove attribution plumbing",
        "v1-only" => "seed the V1-only OpenCode integration",
        other => panic!("unknown prior integration variant: {other}"),
    }
}

#[given(
    regex = r##"^a clean initialized Git project with "([^"]+)" attribution plumbing recorded in its history$"##
)]
fn given_clean_project_with_prior_integration(world: &mut GivnWorld, prior: String) {
    let variant = match prior.as_str() {
        "missing" => "missing",
        "V1 only" => "v1-only",
        other => panic!("unsupported prior integration: {other}"),
    };
    let env = versioned_project_with_both_targets();
    let root = env.root();
    // Every integration is tracked in the initial revision, including the
    // normally ignored local Claude settings, so only the tested difference
    // can appear in the upgrade revision.
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["add", "-f", ".claude/settings.local.json"])
            .status
            .success()
    );
    assert!(
        git(&root, &["commit", "-m", "initial Givn project"])
            .status
            .success()
    );
    if variant == "missing" {
        remove_attribution_plumbing(&root);
    } else {
        // The legacy file is the supplied V1-only integration (no default export).
        std::fs::copy(
            common::repo_path("tests/fixtures/opencode/prototype/givn-attribution_v1.js"),
            root.join(".opencode/plugins/givn-attribution.js"),
        )
        .expect("seed the V1-only integration at the canonical path");
    }
    assert!(git(&root, &["add", "."]).status.success());
    assert!(
        git(&root, &["commit", "-m", prior_state_subject(variant)])
            .status
            .success()
    );
    let status = git(&root, &["status", "--porcelain"]);
    assert!(
        status.stdout.is_empty(),
        "the upgrade fixture must be clean: {}",
        String::from_utf8_lossy(&status.stdout)
    );
    world.transcript_variant = Some(variant.to_owned());
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

/// The managed upgrade revision: one new revision on top of the recorded
/// history, only managed paths, and each refreshed file equal to its asset.
#[then("the managed upgrade commit contains the refreshed attribution plumbing")]
fn then_managed_commit_contains_refreshed_plumbing(world: &mut GivnWorld) {
    let variant = world
        .transcript_variant
        .clone()
        .expect("the prior integration variant");
    let root = world.env.as_ref().expect("project root").root();
    let git_text = |args: &[&str]| {
        let output = git(&root, args);
        assert!(output.status.success(), "git {args:?} failed");
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    // History: initial revision, recorded prior state, then the single upgrade revision.
    assert_eq!(git_text(&["rev-list", "--count", "HEAD"]).trim(), "3");
    let subjects = git_text(&["log", "--format=%s"]);
    let subjects: Vec<&str> = subjects.lines().collect();
    assert_eq!(subjects[1], prior_state_subject(&variant));
    assert_eq!(subjects[2], "initial Givn project");

    let changed = git_text(&["show", "--name-only", "--format=", "HEAD"]);
    let changed: Vec<&str> = changed.lines().collect();
    for path in &changed {
        assert!(
            ["givn/config.yaml", "givn/commands.yaml", ".gitignore"].contains(path)
                || [".opencode/", ".claude/", ".githooks/"]
                    .iter()
                    .any(|prefix| path.starts_with(prefix)),
            "the managed upgrade revision touched an unmanaged path: {path}"
        );
    }
    let refreshed: &[&str] = if variant == "missing" {
        &ATTRIBUTION_PLUMBING
    } else {
        &[".opencode/plugins/givn-attribution.js"]
    };
    for path in refreshed {
        assert!(changed.contains(path), "managed upgrade omitted {path}");
    }
    if variant == "v1-only" {
        for path in [
            ".githooks/commit-msg",
            ".claude/hooks/givn-attribution.sh",
            ".claude/settings.local.json",
        ] {
            assert!(
                !changed.contains(&path),
                "the unchanged integration {path} entered the upgrade revision"
            );
        }
    }
    // Refreshed bytes equal the embedded assets.
    let at_head = |path: &str| {
        let output = git(&root, &["show", &format!("HEAD:{path}")]);
        assert!(
            output.status.success(),
            "{path} is not in the upgrade revision"
        );
        output.stdout
    };
    assert_eq!(
        at_head(".opencode/plugins/givn-attribution.js"),
        include_bytes!("../../assets/agent-integrations/opencode/givn-attribution.js")
    );
    if variant == "missing" {
        assert_eq!(
            at_head(".claude/hooks/givn-attribution.sh"),
            include_bytes!("../../assets/agent-integrations/claude/givn-attribution.sh")
        );
        assert_eq!(
            at_head(".githooks/commit-msg"),
            include_bytes!("../../assets/hooks/commit-msg")
        );
    }
}
