//! Step definitions for the generated-rules policy capability set:
//! `rules-policy`, `local-only-transition`, `publication-cleanliness`, and
//! the local-only stage set on `attribute-commits`.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};
use std::path::PathBuf;
use std::process::Command;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn git(env: &common::TestEnv, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("spawn git subprocess")
}

fn git_ok(env: &common::TestEnv, args: &[&str]) -> String {
    let output = git(env, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

// --- Given -----------------------------------------------------------------

#[given(regex = r#"^a givn/config.yaml with rules "([^"]+)"$"#)]
fn config_with_rules(world: &mut GivnWorld, value: String) {
    let env = env(world);
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim_start().starts_with("rules:"))
        .collect();
    let joined = format!("{}\nrules: {value}\n", lines.join("\n"));
    std::fs::write(path, joined).expect("write givn/config.yaml");
}

#[given("a committed project root with givn/ initialised")]
fn committed_project_root(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(
        out.status.success(),
        "fixture givn init failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    project_is_git_repository_in(&env);
    let out = common::run_givn(&env, &["skills", "sync"]);
    assert!(
        out.status.success(),
        "fixture skills sync failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    common::commit_fixture(&env);
    world.env = Some(env);
}

fn project_is_git_repository_in(env: &common::TestEnv) {
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "fixture@example.com"],
        vec!["config", "user.name", "Givn Fixture"],
        vec!["config", "commit.gpgsign", "false"],
    ] {
        git_ok(env, &args);
    }
}

#[given(regex = r#"^a tracked generated rule at "([^"]+)"$"#)]
fn tracked_generated_rule(world: &mut GivnWorld, path: String) {
    track_rule(world, &path);
}

#[given(regex = r#"^a tracked orphan rule at "([^"]+)"$"#)]
fn tracked_orphan_rule(world: &mut GivnWorld, path: String) {
    track_rule(world, &path);
}

fn track_rule(world: &mut GivnWorld, path: &str) {
    if world.env.is_none() {
        committed_project_root(world);
    }
    let env = env(world);
    let full = env.path(path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).expect("create rule parent");
    }
    std::fs::write(&full, "generated rule\n").expect("write rule file");
    git_ok(env, &["add", "-f", "--", path]);
    git_ok(env, &["commit", "-m", "fixture: track generated rule"]);
}

#[given(regex = r#"^a generated rule committed in an earlier revision at "([^"]+)"$"#)]
fn generated_rule_in_history(world: &mut GivnWorld, path: String) {
    tracked_generated_rule(world, path.clone());
    let env = env(world);
    git_ok(env, &["rm", "--cached", "--ignore-unmatch", "--", &path]);
    git_ok(env, &["commit", "-m", "fixture: untrack generated rule"]);
}

#[given(regex = r#"^an untracked project file at "([^"]+)"$"#)]
fn untracked_project_file(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let full = env.path(&path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).expect("create file parent");
    }
    std::fs::write(&full, "project content\n").expect("write project file");
}

#[given("the AGENTS.md file has a user change")]
fn agents_user_change(world: &mut GivnWorld) {
    let env = env(world);
    let path = env.path("AGENTS.md");
    let mut content = std::fs::read_to_string(&path).expect("read AGENTS.md");
    content.push_str("\nuser-owned note\n");
    std::fs::write(path, content).expect("write AGENTS.md");
    git_ok(env, &["add", "--", "AGENTS.md"]);
    git_ok(env, &["commit", "-m", "fixture: user note in AGENTS.md"]);
}

#[given("a pending AGENTS.md user change")]
fn pending_agents_user_change(world: &mut GivnWorld) {
    let env = env(world);
    let path = env.path("AGENTS.md");
    let mut content = std::fs::read_to_string(&path).expect("read AGENTS.md");
    content.push_str("\npending user-owned note\n");
    std::fs::write(path, content).expect("write AGENTS.md");
}

#[when("the staged removals are committed")]
fn staged_removals_committed(world: &mut GivnWorld) {
    let env = env(world);
    git_ok(env, &["add", "--all"]);
    git_ok(
        env,
        &["commit", "-m", "fixture: commit the local-only removals"],
    );
}

#[given("the project is a Git repository")]
fn project_is_git_repository(world: &mut GivnWorld) {
    let env = env(world);
    project_is_git_repository_in(env);
}

// --- Then ------------------------------------------------------------------

#[then(regex = r#"^the revision tracks "([^"]+)"$"#)]
fn revision_tracks(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let output = git(env, &["ls-files", "--error-unmatch", "--", &path]);
    assert!(
        output.status.success(),
        "expected the revision to track {path}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r#"^the revision does not track "([^"]+)"$"#)]
fn revision_does_not_track(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let output = git(env, &["ls-files", "--error-unmatch", "--", &path]);
    assert!(
        !output.status.success(),
        "expected the revision not to track {path}"
    );
}

#[then(regex = r#"^"([^"]+)" is staged for removal$"#)]
fn staged_for_removal(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let status = git_ok(env, &["status", "--porcelain", "--", &path]);
    assert!(
        status.starts_with("D "),
        "expected {path} staged for removal, got {status:?}"
    );
}

#[then(regex = r#"^the tracked ignore block contains "([^"]+)"$"#)]
fn tracked_ignore_block_contains(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let content = std::fs::read_to_string(env.path(".gitignore")).expect("read .gitignore");
    assert!(
        content.lines().any(|line| line.trim() == path),
        ".gitignore block missing {path}:\n{content}"
    );
}

#[then("the tracked ignore block masks the rule names")]
fn tracked_ignore_block_masks_rule_names(world: &mut GivnWorld) {
    let env = env(world);
    let content = std::fs::read_to_string(env.path(".gitignore")).expect("read .gitignore");
    let mut in_block = false;
    let mut block: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.trim() == "# givn:begin:rules" {
            in_block = true;
            continue;
        }
        if line.trim() == "# givn:end:rules" {
            in_block = false;
            continue;
        }
        if in_block {
            block.push(line.trim());
        }
    }
    assert!(!block.is_empty(), "the rules block is missing:\n{content}");
    for name in ["givn-propose", "givn-spec", "givn-archive", "grill-me"] {
        assert!(
            !block.iter().any(|line| line.contains(name)),
            "the rules block leaks the rule name {name}: {block:?}"
        );
    }
}

#[then(regex = r#"^the tracked ignore block does not contain "([^"]+)"$"#)]
fn tracked_ignore_block_does_not_contain(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let content = std::fs::read_to_string(env.path(".gitignore")).unwrap_or_default();
    assert!(
        !content.lines().any(|line| line.contains(&path)),
        ".gitignore block unexpectedly contains {path}:\n{content}"
    );
}

#[then(regex = r#"^the local exclude contains "([^"]+)"$"#)]
fn local_exclude_contains(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let raw = git_ok(env, &["rev-parse", "--git-path", "info/exclude"]);
    let candidate = PathBuf::from(raw.trim());
    let full = if candidate.is_absolute() {
        candidate
    } else {
        env.root().join(candidate)
    };
    let content = std::fs::read_to_string(full).unwrap_or_default();
    assert!(
        content.lines().any(|line| line.trim() == path),
        "local exclude missing {path}:\n{content}"
    );
}

#[then(regex = r#"^the file "([^"]+)" exists on disk$"#)]
fn file_exists_on_disk(world: &mut GivnWorld, path: String) {
    let env = env(world);
    assert!(env.path(&path).exists(), "expected file {path} on disk");
}

#[then(regex = r#"^the latest revision touches "([^"]+)"$"#)]
fn latest_revision_touches(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let changed = git_ok(env, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        changed.lines().any(|line| line.trim() == path),
        "latest revision does not touch {path}:\n{changed}"
    );
}

#[then(regex = r#"^the reachable history contains "([^"]+)"$"#)]
fn reachable_history_contains(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let log = git_ok(env, &["log", "--all", "--format=%H", "--", &path]);
    assert!(
        !log.trim().is_empty(),
        "expected reachable history to contain {path}"
    );
}

#[then(regex = r#"^the reachable history does not contain "([^"]+)"$"#)]
fn reachable_history_does_not_contain(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let log = git_ok(env, &["log", "--all", "--format=%H", "--", &path]);
    assert!(
        log.trim().is_empty(),
        "expected reachable history not to contain {path}:\n{log}"
    );
}

#[then(regex = r#"^the latest revision does not touch "([^"]+)"$"#)]
fn latest_revision_does_not_touch(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let changed = git_ok(env, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !changed.lines().any(|line| line.trim() == path),
        "latest revision unexpectedly touches {path}:\n{changed}"
    );
}
