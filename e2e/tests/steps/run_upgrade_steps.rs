//! Step definitions for the run-scaffold backfill.
//!
//! The unit scenarios exercise `backfill_commands` directly against a
//! fixture commands file; the @e2e scenarios (in `run_execution_e2e_steps`)
//! drive the real `givn upgrade` subprocess.

use super::GivnWorld;
use crate::harness as common;
use cucumber::{given, then};

fn git_ok(env: &common::TestEnv, args: &[&str]) -> String {
    let output = common::git(env, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[then("the upgrade revision carries no extension content")]
fn upgrade_revision_carries_no_extension_content(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = crate::steps::serve_extended_rules_steps::expected_rule(env, "proposal");
    // The no-context diff carries exactly the added and removed lines; the
    // user-owned extension block is neither.
    let diff = git_ok(env, &["show", "-U0", "--format=", "HEAD"]);
    assert!(
        !diff.contains(&rule),
        "the upgrade revision carries the extension content {rule:?}:\n{diff}"
    );
}

const LAST_0X_VERSION: &str = "0.9.9";

const SYNTHETIC_SOURCE: &str = "1.0.0";

const SYNTHETIC_TARGET: &str = "1.1.0";

const SYNTHETIC_ROUTE: &str = "synthetic-1-0-to-1-1";

const SYNTHETIC_CHANGE: &str = "migrate-1-0-0-to-1-1-0";

const USER_FILE_PATHS: [&str; 3] = [
    "givn/specs/example/example.feature",
    "givn/artifacts/proposal/proposal.md",
    "AGENTS.md",
];

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct UpgradeState {
    target: Option<String>,
    catalog: Option<String>,
    config_before: Option<String>,
    user_files: Vec<(String, String)>,
    head_before: Option<String>,
    commits_before: Option<usize>,
    plan_files: Vec<(String, String)>,
    opened_change: Vec<(String, String)>,
    migration_id: Option<String>,
    failure: Option<String>,
}

fn load_upgrade_state(world: &GivnWorld) -> UpgradeState {
    world
        .changed_boundary
        .as_deref()
        .and_then(|encoded| serde_json::from_str(encoded).ok())
        .unwrap_or_default()
}

fn store_upgrade_state(world: &mut GivnWorld, state: &UpgradeState) {
    world.changed_boundary = Some(serde_json::to_string(state).expect("encode upgrade state"));
}

fn update_upgrade_state(world: &mut GivnWorld, update: impl FnOnce(&mut UpgradeState)) {
    let mut state = load_upgrade_state(world);
    update(&mut state);
    store_upgrade_state(world, &state);
}

fn effective_target(state: &UpgradeState) -> String {
    state
        .target
        .clone()
        .unwrap_or_else(|| crate::harness::binary::product_version().to_string())
}

fn run_upgrade(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    world.last_output = Some(common::run_givn(env, &["upgrade"]));
}

fn mark_config_version(env: &common::TestEnv, version: &str) {
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read config");
    let rewritten: Vec<String> = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("givn_config_version:") {
                format!("givn_config_version: \"{version}\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    std::fs::write(path, format!("{}\n", rewritten.join("\n"))).expect("write config");
}

fn append_config(env: &common::TestEnv, extra: &str) {
    let path = env.path("givn/config.yaml");
    let mut content = std::fs::read_to_string(&path).expect("read config");
    content.push_str(extra);
    std::fs::write(path, content).expect("write config");
}

fn commit_fixture(env: &common::TestEnv, message: &str) {
    match env.backend() {
        common::BackendKind::Git => {
            let add = common::git(env, &["add", "--all"]);
            assert!(
                add.status.success(),
                "git add failed: {}",
                String::from_utf8_lossy(&add.stderr)
            );
            let commit = common::git(env, &["commit", "--allow-empty", "-q", "-m", message]);
            assert!(
                commit.status.success(),
                "git commit failed: {}",
                String::from_utf8_lossy(&commit.stderr)
            );
        }
        common::BackendKind::Fossil => common::fossil_commit_all(env, message),
    }
}

fn head_revision(env: &common::TestEnv) -> String {
    match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        common::BackendKind::Fossil => common::fossil_revision(env),
    }
}

fn revision_count(env: &common::TestEnv) -> usize {
    match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-list", "--count", "HEAD"]).stdout)
                .trim()
                .parse()
                .expect("Git revision count")
        }
        common::BackendKind::Fossil => String::from_utf8_lossy(
            &common::fossil(env, &["sql", "select count(*) from event where type='ci'"]).stdout,
        )
        .trim()
        .trim_matches('\'')
        .parse()
        .expect("Fossil check-in count"),
    }
}

fn latest_revision_changes(env: &common::TestEnv) -> Vec<(String, String)> {
    match env.backend() {
        common::BackendKind::Git => String::from_utf8_lossy(
            &common::git(
                env,
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
            (
                fields.next().unwrap_or_default().to_string(),
                fields.next().unwrap_or_default().to_string(),
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
                Some((status.to_string(), path.trim().to_string()))
            })
            .collect()
        }
    }
}

fn is_managed_allowlist_path(path: &str) -> bool {
    if matches!(path, |"givn/config.yaml"| "givn/commands.yaml"
        | "givn/specification.yaml"
        | ".gitignore"
        | ".fossil-settings/ignore-glob"
        | ".githooks/commit-msg"
        | ".claude/settings.local.json"
        | ".maki/lua/givn-timeline.lua")
    {
        return true;
    }
    [
        ".opencode/skills/givn-",
        ".agents/skills/givn-",
        ".claude/skills/givn-",
        ".omp/skills/givn-",
        ".maki/skills/givn-",
        ".opencode/plugins/",
        ".claude/hooks/",
        ".omp/hooks/pre/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
}

fn migration_changes(env: &common::TestEnv) -> Vec<String> {
    let mut found = Vec::new();
    for root in ["givn/changes", "givn/archive"] {
        let Ok(entries) = std::fs::read_dir(env.path(root)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("migrate-") {
                found.push(name);
            }
        }
    }
    found.sort();
    found
}

fn snapshot_user_files(env: &common::TestEnv) -> Vec<(String, String)> {
    USER_FILE_PATHS
        .iter()
        .map(|path| {
            let content =
                std::fs::read_to_string(env.path(path)).unwrap_or_else(|_| "<missing>".to_string());
            (path.to_string(), content)
        })
        .collect()
}

fn seed_user_files(env: &common::TestEnv) {
    std::fs::create_dir_all(env.path("givn/specs/example")).expect("create fixture specs");
    std::fs::write(
        env.path("givn/specs/example/example.feature"),
        "Feature: fixture\n\n  Scenario: fixture stays\n    Given a fixture\n",
    )
    .expect("write fixture feature");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", "example")
        .replace("<capability-id>", "example")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "fixture stays")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| example | exercise the capability | fixture stays |\n",
            "- none\n",
        );
    std::fs::write(env.path("givn/specs/example/usecase.md"), document)
        .expect("write fixture usecase");
    common::write_override(env, "proposal", "proposal.md", "# Custom proposal\n");
}

fn clean_git_project_with_stale_integration() -> common::TestEnv {
    let env = common::make_init_root_without_git();
    let skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut content = std::fs::read_to_string(&skill).expect("read generated skill");
    content.push_str("\nstale generated marker\n");
    std::fs::write(&skill, content).expect("mark skill stale");
    seed_user_files(&env);
    append_config(
        &env,
        "\nextensions:\n  specs:\n    - Keep fixture specs stable.\n",
    );
    common::git_init(&env);
    env
}

#[given("a clean Git project with stale generated integration and a committed checkpoint")]
fn given_clean_git_project(world: &mut GivnWorld) {
    let env = clean_git_project_with_stale_integration();
    let mut state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        user_files: snapshot_user_files(&env),
        ..UpgradeState::default()
    };
    state.config_before =
        Some(std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config"));
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given("its current-format configuration was prepared manually with the last 0.x version marker")]
fn given_manual_last_0x_marker(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    mark_config_version(env, LAST_0X_VERSION);
    let config_path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).expect("read config");
    std::fs::write(
        &config_path,
        config.replace(
            &format!("givn_config_version: \"{LAST_0X_VERSION}\""),
            &format!("givn_config_version: {LAST_0X_VERSION}"),
        ),
    )
    .expect("write manually prepared config");
    commit_fixture(
        env,
        "fixture: manually prepared current-format configuration",
    );
    let head = head_revision(env);
    let commits = revision_count(env);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    update_upgrade_state(world, |state| {
        state.head_before = Some(head);
        state.commits_before = Some(commits);
        state.config_before = Some(config);
    });
}

#[given("the installed Givn version is 0.9.10")]
fn given_installed_version(world: &mut GivnWorld) {
    // The preparation release is 0.9.10 without a version bump: the fixture
    // exercises the installed binary's own version, which is the current
    // release the upgrade writes into the project configuration.
    let _ = world;
}

#[then("the command exits successfully and reports the managed upgrade revision")]
fn then_upgrade_reports_revision(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "upgrade failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let target = effective_target(&load_upgrade_state(world));
    assert!(stdout.contains("Refreshed"), "{stdout}");
    assert!(stdout.contains(&target), "{stdout}");
    assert!(stdout.contains("commit"), "{stdout}");
}

#[then("exactly one allowlist-only managed revision exists and no migration change is created")]
fn then_one_allowlist_revision(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let state = load_upgrade_state(world);
    assert_eq!(
        revision_count(env),
        state.commits_before.expect("commit count before") + 1,
        "expected exactly one managed revision"
    );
    for (status, path) in latest_revision_changes(env) {
        assert!(
            is_managed_allowlist_path(&path),
            "managed revision staged an unmanaged path: {status} {path}"
        );
        assert_ne!(path, "AGENTS.md", "AGENTS.md must never be staged");
    }
    assert!(
        migration_changes(env).is_empty(),
        "a migration change was created"
    );
}

#[then("no automatic configuration migration or embedded 0.x migration route is applied")]
fn then_no_automatic_migration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    assert!(migration_changes(env).is_empty());
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    for retired in ["absolute_mode", "grill_me", "grill_with_docs"] {
        assert!(
            !config.contains(retired),
            "an automatic compatibility alias introduced {retired}"
        );
    }
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("no migration change was required"),
        "{stdout}"
    );
}

#[then("project specifications, extensions, and custom replacements are unchanged")]
fn then_user_files_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let state = load_upgrade_state(world);
    assert_eq!(snapshot_user_files(env), state.user_files);
}

fn supply_synthetic_route(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("synthetic fixture");
    let catalog_relative = "givn/test-migration-catalog.yaml";
    let catalog_path = env.path(catalog_relative);
    std::fs::write(
        &catalog_path,
        format!(
            "schema_version: 1\nmigrations:\n  - id: {SYNTHETIC_ROUTE}\n    source_version: \"{SYNTHETIC_SOURCE}\"\n    target_version: \"{SYNTHETIC_TARGET}\"\n    order: 10\n    proposal_asset: migrations/fixtures/synthetic/proposal.md\n    design_asset: migrations/fixtures/synthetic/design.md\n"
        ),
    )
    .expect("write synthetic catalog");
    let seam_path = env.path(".givn-test/upgrade.yaml");
    std::fs::create_dir_all(seam_path.parent().unwrap()).expect("create seam directory");
    std::fs::write(
        &seam_path,
        format!("target_version: \"{SYNTHETIC_TARGET}\"\ncatalog: \"{catalog_relative}\"\n"),
    )
    .expect("write upgrade seam");
    commit_fixture(env, "fixture: synthetic migration route seam");
    let catalog = catalog_path.to_string_lossy().into_owned();
    let head = head_revision(env);
    let commits = revision_count(env);
    update_upgrade_state(world, |state| {
        state.catalog = Some(catalog);
        state.target = Some(SYNTHETIC_TARGET.to_string());
        state.head_before = Some(head);
        state.commits_before = Some(commits);
    });
}

#[given(
    "a completed no-behavior-change maintenance migration change prepared manually in a disposable project"
)]
fn given_completed_maintenance_migration(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    mark_config_version(&env, SYNTHETIC_SOURCE);
    seed_user_files(&env);
    common::git_init(&env);
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        user_files: snapshot_user_files(&env),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    let dir = env.path(&format!("givn/changes/{SYNTHETIC_CHANGE}"));
    world.env = Some(env);
    supply_synthetic_route(world);
    run_upgrade(world);
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "fixture upgrade failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(
        dir.join("design-review.md"),
        "# Design Review\n\nDESIGN-REVIEW: PASS\n",
    )
    .expect("write design review");
    let revision = head_revision(world.env.as_ref().expect("migration fixture"));
    std::fs::write(
        dir.join("tasks.md"),
        format!("# Tasks\n\n- [x] COMMIT: {revision} -- complete the migration\n"),
    )
    .expect("write tasks");
    std::fs::write(
        dir.join("review.md"),
        "# Review\n\nREADME-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .expect("write review");
    std::fs::write(dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n")
        .expect("write arc42 impact assessment");
    crate::steps::run_declaration_fixtures_support::declare_run(world, "cli");
    update_upgrade_state(world, |state| {
        state.migration_id = Some(SYNTHETIC_CHANGE.to_string());
    });
}

#[given("its review and archive gates are satisfied without a product specification")]
fn given_gates_satisfied_without_spec(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration fixture");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    crate::steps::archive_proof_support::configure_commands(env);
    crate::steps::archive_proof_support::disable_coverage(env);
    common::commit_fixture(env);
    let id = load_upgrade_state(world)
        .migration_id
        .expect("migration change id");
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    assert!(env.path(&format!("givn/changes/{id}/.givn-skip")).is_file());
}

#[then("the command reports that the maintenance migration was archived successfully")]
fn then_maintenance_archived(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "archive failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let env = world.env.as_ref().expect("migration fixture");
    let archived = env.path(&format!("givn/archive/{SYNTHETIC_CHANGE}"));
    assert!(archived.is_dir(), "the migration change was not archived");
}
