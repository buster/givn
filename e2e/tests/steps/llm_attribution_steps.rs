//! Step definitions for the llm-attribution capability.

use super::GivnWorld;
use crate::harness as common;
use cucumber::{given, then};
use std::process::{Command, Output};

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
    let binary = crate::harness::binary::givn();
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

fn attribution_state(world: &mut GivnWorld) -> &mut GitAttributionState {
    world
        .git_attribution
        .get_or_insert_with(GitAttributionState::default)
}

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
            let revision = common::fossil_revision(env);
            let parent = common::fossil_parent_revision(env, &revision).expect("Fossil parent");
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
