//! Compiled-CLI E2E steps for the 0.2.0-to-0.4.0 migration boundary.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a clean Git project created with Givn 0.2.0")]
fn clean_legacy_project(world: &mut GivnWorld) {
    world.env = Some(legacy_git_project());
}

pub(crate) fn legacy_git_project() -> crate::common::TestEnv {
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
            &format!("givn_config_version: \"{}\"", env!("CARGO_PKG_VERSION")),
            "givn_config_version: \"0.2.0\"",
        ),
    )
    .expect("write legacy fixture config");

    let generated_skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut skill = std::fs::read_to_string(&generated_skill).expect("read generated skill");
    skill.push_str("\nlegacy generated marker\n");
    std::fs::write(generated_skill, skill).expect("mark generated skill as legacy");

    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"./emit-result.sh regular\"\n  e2e_command: \"\"\n",
    );
    std::fs::write(env.path(".gitignore"), ".opencode/\nAGENTS.md\n")
        .expect("ignore generated agent files");

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

fn current_git_project_with_stale_integration() -> crate::common::TestEnv {
    let env = crate::common::make_bare_root();
    let init = crate::common::run_givn_with_stdin(&env, &["init", "--no-addons"], "");
    assert!(
        init.status.success(),
        "fixture init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let generated_skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut skill = std::fs::read_to_string(&generated_skill).expect("read generated skill");
    skill.push_str("\nstale generated marker\n");
    std::fs::write(generated_skill, skill).expect("mark generated skill as stale");
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"./emit-result.sh regular\"\n  e2e_command: \"\"\n",
    );

    git(&env, &["init"]);
    git(&env, &["config", "user.name", "Givn Migration Test"]);
    git(
        &env,
        &["config", "user.email", "givn-migration@example.test"],
    );
    git(&env, &["add", "--all"]);
    let commit = git(&env, &["commit", "-m", "fixture: Givn current project"]);
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

#[when("the consumer invokes `givn upgrade` for migration-free refresh")]
fn consumer_runs_migration_free_upgrade(world: &mut GivnWorld) {
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

#[then("the project config identifies the current Givn version")]
fn config_identifies_current_version(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains(&format!(
            "givn_config_version: \"{}\"",
            env!("CARGO_PKG_VERSION")
        )),
        "{config}"
    );
}

#[given("a clean Git project with current Givn files and a stale generated integration")]
fn current_project_with_stale_integration(world: &mut GivnWorld) {
    world.env = Some(current_git_project_with_stale_integration());
}

#[then("one managed upgrade commit exists")]
fn one_managed_upgrade_commit_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("current project fixture");
    let count = git_stdout(env, &["rev-list", "--count", "HEAD"]);
    assert_eq!(
        count.trim(),
        "2",
        "expected fixture commit plus managed commit"
    );
    let subjects = git_stdout(env, &["log", "-1", "--format=%s"]);
    assert!(subjects.contains("update managed files"), "{subjects}");
}

/// Resolve the migration change id a fixture upgrade actually created by
/// scanning `givn/changes/` for the `migrate-` prefix — version-agnostic
/// by construction, so the suite survives future catalog additions.
pub(crate) fn find_migration_change_id(env: &crate::common::TestEnv) -> String {
    let mut entries: Vec<String> = std::fs::read_dir(env.path("givn/changes"))
        .expect("read active changes")
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("migrate-"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "expected exactly one migration change, got {entries:?}"
    );
    entries.pop().expect("exactly one entry")
}

#[then("no migration change exists")]
fn no_migration_change_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("current project fixture");
    let changes = env.path("givn/changes");
    let archives = env.path("givn/archive");
    let no_migrate = |dir: &std::path::Path| {
        !dir.exists()
            || std::fs::read_dir(dir)
                .expect("read dir")
                .flatten()
                .all(|entry| !entry.file_name().to_string_lossy().starts_with("migrate-"))
    };
    assert!(no_migrate(&changes));
    assert!(no_migrate(&archives));
    assert!(
        std::fs::read_dir(&changes)
            .expect("read active changes")
            .flatten()
            .all(|entry| !entry
                .file_name()
                .to_string_lossy()
                .starts_with("migrate-0-2-0-to-"))
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
                || path.starts_with(".opencode/commands/")
                || path.starts_with(".opencode/plugins/")
                || path == ".githooks/commit-msg",
            "Commit A staged an unmanaged path: {path}"
        );
    }
}

#[then("a migration change for the current version exists")]
fn current_version_migration_change_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let id = find_migration_change_id(env);
    assert!(env.path(&format!("givn/changes/{id}")).is_dir());
    assert!(id.starts_with("migrate-0-2-0-to-"), "{id}");
}

#[then("the migration change records the managed upgrade commit")]
fn migration_records_upgrade_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let commits = git_stdout(env, &["rev-list", "--reverse", "HEAD"]);
    let hashes: Vec<&str> = commits.lines().collect();
    let commit_a = hashes.get(1).expect("Commit A hash");
    let design = {
        let id = find_migration_change_id(env);
        std::fs::read_to_string(env.path(&format!("givn/changes/{id}")).join("design.md"))
    }
    .expect("read migration design");
    assert!(
        design.contains(&format!("upgrade_commit: `{commit_a}`")),
        "{design}"
    );
}

#[then("the migration change contains a proposal and detailed design")]
fn migration_contains_plan(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let proposal = env
        .path(&format!("givn/changes/{}", find_migration_change_id(env)))
        .join("proposal.md");
    let design = env
        .path(&format!("givn/changes/{}", find_migration_change_id(env)))
        .join("design.md");
    let metadata = env
        .path(&format!("givn/changes/{}", find_migration_change_id(env)))
        .join("migration.yaml");
    assert!(proposal.exists());
    assert!(design.exists());
    assert!(metadata.exists());
    let metadata = std::fs::read_to_string(metadata).expect("read migration metadata");
    assert!(metadata.contains("sequence: 1"), "{metadata}");
    assert!(
        metadata.contains("id: migrate-0-2-0-to-0-3-0"),
        "{metadata}"
    );
    assert!(
        std::fs::metadata(design).expect("design metadata").len() > 1_000,
        "migration design is not detailed"
    );
}

#[then("the migration change intentionally has no product specification")]
fn migration_has_no_product_spec(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let migration = env.path(&format!("givn/changes/{}", find_migration_change_id(env)));
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

#[given("a project that already has the current-version migration change")]
fn prepared_project(world: &mut GivnWorld) {
    let env = legacy_git_project();
    let first = crate::common::run_givn(&env, &["upgrade"]);
    assert!(
        first.status.success(),
        "initial upgrade failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let design = {
        let id = find_migration_change_id(&env);
        std::fs::read_to_string(env.path(&format!("givn/changes/{id}")).join("design.md"))
    }
    .expect("read prepared migration design");
    let count = git_stdout(&env, &["rev-list", "--count", "HEAD"]);
    world.additional_output = Some(format!("{count}\n---DESIGN---\n{design}"));
    world.env = Some(env);
}

#[then("the command does not overwrite the migration change")]
fn migration_not_overwritten(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("prepared project fixture");
    let actual = {
        let id = find_migration_change_id(env);
        std::fs::read_to_string(env.path(&format!("givn/changes/{id}")).join("design.md"))
    }
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
        &format!("givn_config_version: \"{}\"", env!("CARGO_PKG_VERSION")),
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
    // The pre-created migrate-* change is respected whatever its version suffix.
    assert!(id.starts_with("migrate-"), "{id}");
    let env = legacy_git_project();
    let migration = env.path(&format!("givn/changes/{id}"));
    std::fs::create_dir_all(&migration).expect("create existing migration plan");
    // The upgrade dedup detection keys on migration.yaml — the fixture's
    // pre-existing plan carries one so it is recognised as pending.
    std::fs::write(
        migration.join("migration.yaml"),
        "schema_version: 1\nsource_version: \"0.2.0\"\ntarget_version: \"0.4.0\"\n",
    )
    .expect("write existing migration metadata");
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
    let migration = env.path(&format!("givn/changes/{}", find_migration_change_id(env)));
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

#[given("a completed automatic current-version migration change")]
fn completed_automatic_migration(world: &mut GivnWorld) {
    let env = legacy_git_project();
    let upgrade = crate::common::run_givn(&env, &["upgrade"]);
    assert!(
        upgrade.status.success(),
        "upgrade failed: {}",
        String::from_utf8_lossy(&upgrade.stderr)
    );
    // A real 0.2.0 project fills in the backfilled run scaffold before its
    // next archive; the fixture mimics that with a complete cli declaration.
    let migration = env.path(&format!("givn/changes/{}", find_migration_change_id(&env)));
    std::fs::write(
        migration.join("design-review.md"),
        "# Design Review\n\nDESIGN-REVIEW: PASS\n",
    )
    .expect("write completed design review");
    std::fs::write(
        migration.join("tasks.md"),
        "# Tasks\n\n- [x] Complete the migration\n",
    )
    .expect("write completed tasks");
    std::fs::write(
        migration.join("review.md"),
        "# Review\n\nREVIEW: PASS\nREADME-IMPACT: none\n",
    )
    .expect("write completed review");
    world.env = Some(env);
    crate::steps::run_declaration_fixtures_support::declare_run(world, "cli");
}

#[when("the consumer archives the maintenance migration")]
fn archive_maintenance_migration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("completed migration fixture");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    common::commit_fixture(env);
    let id = find_migration_change_id(env);
    world.last_output = Some(crate::common::run_givn(env, &["archive", "--change", &id]));
}

#[then("the maintenance migration is archived successfully")]
fn maintenance_migration_archived(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "archive failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let env = world.env.as_ref().expect("completed migration fixture");
    let archived: Vec<String> = std::fs::read_dir(env.path("givn/archive"))
        .expect("read archive")
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("migrate-"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        archived.len(),
        1,
        "the migration change moved to the archive"
    );
    let remaining: Vec<String> = std::fs::read_dir(env.path("givn/changes"))
        .expect("read active changes")
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("migrate-"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        remaining.is_empty(),
        "migration change still active: {remaining:?}"
    );
}

#[then("the permanent specification tree contains no migration feature")]
fn permanent_tree_has_no_migration_feature(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("completed migration fixture");
    let specs = env.path("givn/specs");
    let features = walk_feature_files(&specs);
    assert!(
        features
            .iter()
            .all(|path| !path.to_string_lossy().contains("migrate-0-2-0-to-0-4-0")),
        "migration feature appeared in permanent specs: {features:?}"
    );
}

fn walk_feature_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk_feature_files(&path));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("feature") {
                files.push(path);
            }
        }
    }
    files
}
