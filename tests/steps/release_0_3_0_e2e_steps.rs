//! Compiled-CLI E2E steps for the 0.2.0-to-0.3.0 migration boundary.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a clean Git project created with Givn 0.2.0")]
fn clean_legacy_project(world: &mut GivnWorld) {
    world.env = Some(legacy_git_project());
}

fn legacy_git_project() -> crate::common::TestEnv {
    let env = crate::common::make_bare_root();
    let init = crate::common::run_givn_with_stdin(&env, &["init", "--no-addons"], "");
    assert!(
        init.status.success(),
        "fixture init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let config_path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).expect("read fixture config");
    std::fs::write(
        &config_path,
        config.replace(
            "givn_config_version: \"0.3.0\"",
            "givn_config_version: \"0.2.0\"",
        ),
    )
    .expect("write legacy fixture config");

    let generated_skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut skill = std::fs::read_to_string(&generated_skill).expect("read generated skill");
    skill.push_str("\nlegacy generated marker\n");
    std::fs::write(generated_skill, skill).expect("mark generated skill as legacy");

    git(&env, &["init"]);
    git(&env, &["config", "user.name", "Givn Migration Test"]);
    git(
        &env,
        &["config", "user.email", "givn-migration@example.test"],
    );
    git(&env, &["add", "--all"]);
    let commit = git(&env, &["commit", "-m", "fixture: Givn 0.2.0 project"]);
    assert!(
        commit.status.success(),
        "fixture commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );

    env
}

#[when("the consumer invokes `givn upgrade` for migration")]
fn consumer_runs_upgrade(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    world.last_output = Some(crate::common::run_givn(env, &["upgrade"]));
}

#[when("the consumer runs `givn upgrade` again")]
fn consumer_runs_upgrade_again(world: &mut GivnWorld) {
    consumer_runs_upgrade(world);
}

#[then("the command exits successfully")]
fn command_exits_successfully(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "upgrade failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then("the project config identifies Givn 0.3.0")]
fn config_identifies_current_version(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains("givn_config_version: \"0.3.0\""),
        "{config}"
    );
}

#[then("one commit contains only the managed scaffold and generated agent integration changes")]
fn first_commit_has_managed_paths(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let commits = git_stdout(env, &["rev-list", "--reverse", "HEAD"]);
    let hashes: Vec<&str> = commits.lines().collect();
    assert!(
        hashes.len() >= 3,
        "expected fixture, A, and B commits: {commits}"
    );
    let changed = git_stdout(
        env,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            hashes[1],
        ],
    );
    assert!(!changed.is_empty(), "Commit A has no changed paths");
    for path in changed.lines() {
        assert!(
            path == "givn/config.yaml"
                || path == "AGENTS.md"
                || path.starts_with(".agents/skills/")
                || path.starts_with(".claude/skills/")
                || path.starts_with(".opencode/skills/")
                || path.starts_with(".opencode/commands/"),
            "Commit A staged an unmanaged path: {path}"
        );
    }
}

#[then(regex = r#"^a migration change named "([^"]+)" exists$"#)]
fn migration_change_exists(world: &mut GivnWorld, id: String) {
    let env = world.env.as_ref().expect("legacy project fixture");
    assert!(env.path(&format!("givn/changes/{id}")).is_dir());
}

#[then("the migration change records the managed upgrade commit")]
fn migration_records_upgrade_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let commits = git_stdout(env, &["rev-list", "--reverse", "HEAD"]);
    let hashes: Vec<&str> = commits.lines().collect();
    let commit_a = hashes.get(1).expect("Commit A hash");
    let design = std::fs::read_to_string(env.path("givn/changes/migrate-0-2-0-to-0-3-0/design.md"))
        .expect("read migration design");
    assert!(
        design.contains(&format!("upgrade_commit: `{commit_a}`")),
        "{design}"
    );
}

#[then("the migration change contains a proposal and detailed design")]
fn migration_contains_plan(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let proposal = env.path("givn/changes/migrate-0-2-0-to-0-3-0/proposal.md");
    let design = env.path("givn/changes/migrate-0-2-0-to-0-3-0/design.md");
    assert!(proposal.exists());
    assert!(design.exists());
    assert!(
        std::fs::metadata(design).expect("design metadata").len() > 1_000,
        "migration design is not detailed"
    );
}

#[then("the migration change intentionally has no product specification")]
fn migration_has_no_product_spec(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let migration = env.path("givn/changes/migrate-0-2-0-to-0-3-0");
    assert!(!migration.join("specs").exists());
    assert_eq!(
        std::fs::read_to_string(migration.join(".givn-skip")).expect("read skip marker"),
        "specs\n"
    );
}

fn git(env: &crate::common::TestEnv, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("run git fixture command")
}

fn git_stdout(env: &crate::common::TestEnv, args: &[&str]) -> String {
    let output = git(env, args);
    assert!(
        output.status.success(),
        "git command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[given("a Givn 0.2.0 project with uncommitted user changes")]
fn dirty_legacy_project(world: &mut GivnWorld) {
    let env = legacy_git_project();
    let config = std::fs::read(env.path("givn/config.yaml")).expect("read legacy config");
    std::fs::write(env.path("user-work.txt"), "uncommitted user change\n")
        .expect("write uncommitted user change");
    world.additional_output = Some(String::from_utf8(config).expect("legacy config is UTF-8"));
    world.env = Some(env);
}

#[then("the command fails before changing project files")]
fn command_fails_before_file_changes(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "upgrade unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("clean Git worktree") || stderr.contains("requires a Git repository"),
        "{stderr}"
    );
    let env = world.env.as_ref().expect("dirty project fixture");
    let expected = world.additional_output.as_deref().expect("config snapshot");
    let actual = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert_eq!(
        actual, expected,
        "config changed before dirty-worktree rejection"
    );
}

#[then("no commit is created")]
fn no_commit_is_created(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dirty project fixture");
    assert_eq!(
        git_stdout(env, &["rev-list", "--count", "HEAD"]).trim(),
        "1"
    );
}

#[then("the user changes remain byte-for-byte unchanged")]
fn user_changes_remain_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dirty project fixture");
    assert_eq!(
        std::fs::read(env.path("user-work.txt")).expect("read user change"),
        b"uncommitted user change\n"
    );
    assert!(git_stdout(env, &["status", "--porcelain"]).contains("?? user-work.txt"));
}

#[given("a project that already has the 0.3.0 migration change")]
fn prepared_project(world: &mut GivnWorld) {
    let env = legacy_git_project();
    let first = crate::common::run_givn(&env, &["upgrade"]);
    assert!(
        first.status.success(),
        "initial upgrade failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let design = std::fs::read_to_string(env.path("givn/changes/migrate-0-2-0-to-0-3-0/design.md"))
        .expect("read prepared migration design");
    let count = git_stdout(&env, &["rev-list", "--count", "HEAD"]);
    world.additional_output = Some(format!("{count}\n---DESIGN---\n{design}"));
    world.env = Some(env);
}

#[then("the command does not overwrite the migration change")]
fn migration_not_overwritten(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("prepared project fixture");
    let actual = std::fs::read_to_string(env.path("givn/changes/migrate-0-2-0-to-0-3-0/design.md"))
        .expect("read migration design");
    let expected = world
        .additional_output
        .as_deref()
        .expect("prepared migration snapshot")
        .split_once("\n---DESIGN---\n")
        .expect("design snapshot separator")
        .1;
    assert_eq!(actual, expected);
}

#[then("no second migration commit is created")]
fn no_second_migration_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("prepared project fixture");
    let before = world
        .additional_output
        .as_deref()
        .expect("prepared migration snapshot")
        .split_once('\n')
        .expect("commit count snapshot")
        .0;
    let after = git_stdout(env, &["rev-list", "--count", "HEAD"]);
    assert_eq!(after.trim(), before.trim());
}

#[then("the command reports that the project is already prepared for migration")]
fn already_prepared_report(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("already prepared for migration"));
}

#[given("a Givn 0.2.0 project that is not a Git repository")]
fn non_git_legacy_project(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let init = crate::common::run_givn_with_stdin(&env, &["init", "--no-addons"], "");
    assert!(init.status.success(), "fixture init failed");
    let config_path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).expect("read fixture config");
    let legacy = config.replace(
        "givn_config_version: \"0.3.0\"",
        "givn_config_version: \"0.2.0\"",
    );
    std::fs::write(&config_path, &legacy).expect("write legacy config");
    world.additional_output = Some(legacy);
    world.env = Some(env);
}

#[then("the command explains that a Git repository is required for the automatic migration")]
fn git_required_report(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires a Git repository"), "{stderr}");
}

#[given(regex = r#"^a Givn 0.2.0 project with an existing "([^"]+)" change$"#)]
fn existing_migration_project(world: &mut GivnWorld, id: String) {
    assert_eq!(id, "migrate-0-2-0-to-0-3-0");
    let env = legacy_git_project();
    let migration = env.path("givn/changes/migrate-0-2-0-to-0-3-0");
    std::fs::create_dir_all(&migration).expect("create existing migration plan");
    let proposal = "# Existing migration proposal\nkeep this proposal\n";
    let design = "# Existing migration design\nkeep this design\n";
    std::fs::write(migration.join("proposal.md"), proposal).expect("write existing proposal");
    std::fs::write(migration.join("design.md"), design).expect("write existing design");
    world.additional_output = Some(format!("{proposal}---\n{design}"));
    world.env = Some(env);
}

#[then("the command does not overwrite the existing migration proposal or design")]
fn existing_plan_preserved(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("existing migration fixture");
    let migration = env.path("givn/changes/migrate-0-2-0-to-0-3-0");
    let actual = format!(
        "{}---\n{}",
        std::fs::read_to_string(migration.join("proposal.md")).expect("read proposal"),
        std::fs::read_to_string(migration.join("design.md")).expect("read design")
    );
    assert_eq!(
        actual,
        world
            .additional_output
            .as_deref()
            .expect("existing migration snapshot")
    );
}

#[then("the command reports the existing migration change")]
fn existing_plan_reported(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("already exists"));
}
