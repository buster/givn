//! Step definitions for the run-scaffold backfill.
//!
//! The unit scenarios exercise `backfill_commands` directly against a
//! fixture commands file; the @e2e scenarios (in `run_execution_e2e_steps`)
//! drive the real `givn upgrade` subprocess.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

/// The full-copy givn-propose skill an earlier version installed: rule text,
/// no direction to the served instruction.
const EARLIER_VERSION_FULL_COPY: &str = "---\nname: givn-propose\ndescription: Write the proposal for a givn change — earlier-version full copy.\n---\n\n# givn-propose\n\n## Rules (earlier version)\n\n- Write from the user's perspective, not the implementer's.\n- Every revision the change records carries the release note as its message subject.\n";

/// The direction a rewritten generated file carries.
const DIRECTION_TO_PROPOSAL: &str = "givn instructions proposal";

fn git_ok(env: &common::TestEnv, args: &[&str]) -> String {
    let output = common::git(env, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Set the project's generated-rules policy to `versioned`: a project that
/// tracks generated rules. Under the default local-only policy the upgrade
/// removes a tracked rule from the revision instead of rewriting it.
fn set_rules_versioned(env: &common::TestEnv) {
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let mut replaced = false;
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            if !replaced
                && !line.trim_start().starts_with('#')
                && line.trim_start().starts_with("rules:")
            {
                replaced = true;
                "rules: versioned".to_string()
            } else {
                line.to_string()
            }
        })
        .collect();
    assert!(
        replaced,
        "the fixture config must declare the generated-rules policy"
    );
    std::fs::write(&path, format!("{}\n", lines.join("\n"))).expect("write givn/config.yaml");
}

#[given(regex = r#"^a tracked full-copy generated rule at "([^"]+)"$"#)]
fn tracked_full_copy_generated_rule(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a committed project root is required first");
    let full = env.path(&path);
    std::fs::create_dir_all(full.parent().expect("rule parent directory"))
        .expect("create the rule directory");
    std::fs::write(&full, EARLIER_VERSION_FULL_COPY).expect("write the earlier-version full copy");
    let content = std::fs::read_to_string(&full).expect("read the earlier-version full copy");
    assert!(
        content.contains("earlier version") && !content.contains(DIRECTION_TO_PROPOSAL),
        "the fixture must be a full copy with rule text and no direction: {content}"
    );
    set_rules_versioned(env);
    git_ok(env, &["add", "-f", "--", &path, "givn/config.yaml"]);
    git_ok(
        env,
        &[
            "commit",
            "-q",
            "-m",
            "fixture: track the generated full copy",
            "--allow-empty",
        ],
    );
}

#[then(regex = r#"^the revision shows "([^"]+)" rewritten to the direction$"#)]
fn revision_shows_rewritten_to_the_direction(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let changed = git_ok(env, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        changed.lines().any(|line| line.trim() == path),
        "the upgrade revision does not touch {path}:\n{changed}"
    );
    let head = common::git(env, &["show", &format!("HEAD:{path}")]);
    assert!(
        head.status.success(),
        "the revision does not carry {path}: {}",
        String::from_utf8_lossy(&head.stderr)
    );
    let content = String::from_utf8_lossy(&head.stdout);
    assert!(
        content.contains(DIRECTION_TO_PROPOSAL),
        "{path} is not rewritten to the direction: {content}"
    );
    assert!(
        !content.contains("earlier-version"),
        "{path} still carries the earlier-version full copy: {content}"
    );
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

#[then("the recorded extension is unchanged")]
fn recorded_extension_is_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let recorded = crate::steps::serve_extended_rules_steps::recorded_extension_block(env);
    let current = crate::steps::serve_extended_rules_steps::config_extension_block(env);
    assert_eq!(
        recorded, current,
        "the recorded extension block changed across the upgrade"
    );
}

#[then("the project-owned replacement is byte-for-byte unchanged")]
fn project_owned_replacement_is_byte_for_byte_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let recorded = crate::steps::serve_extended_rules_steps::recorded_replacement(env, "proposal");
    let path = env.path("givn/artifacts/proposal/instructions.md");
    let current = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert_eq!(
        recorded, current,
        "the project-owned replacement is not byte-for-byte unchanged"
    );
}

#[then("the upgrade revision carries no injected text for the replacement")]
fn upgrade_revision_carries_no_injected_text_for_the_replacement(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let changed = git_ok(env, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !changed
            .lines()
            .any(|line| line.trim() == "givn/artifacts/proposal/instructions.md"),
        "the upgrade revision touches the project-owned replacement:\n{changed}"
    );
    let diff = git_ok(env, &["show", "-U0", "--format=", "HEAD"]);
    assert!(
        !diff.contains(crate::steps::serve_extended_rules_steps::REPLACEMENT_MARKER),
        "the upgrade revision injected text for the replacement:\n{diff}"
    );
}

#[given(regex = r"^a commands file without a run declaration:$")]
fn commands_file_without_run_declaration(world: &mut GivnWorld, step: &cucumber::gherkin::Step) {
    let doc = step
        .docstring
        .as_deref()
        .expect("a commands file docstring is required");
    world.env.get_or_insert_with(common::make_init_root);
    let env = world.env.as_ref().expect("a project root is required");
    std::fs::write(env.path("givn/commands.yaml"), doc).expect("write commands.yaml");
}

#[given("a commands file that already has a run declaration")]
fn commands_file_with_run_declaration(world: &mut GivnWorld) {
    world.env.get_or_insert_with(common::make_init_root);
    let env = world.env.as_ref().expect("a project root is required");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert!(
        content.lines().any(|line| line.starts_with("run:")),
        "fixture commands file must already carry a run declaration:\n{content}"
    );
}

#[when("the backfill runs")]
fn the_backfill_runs(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required");
    let path = env.path("givn/commands.yaml");
    let existing = std::fs::read_to_string(&path).expect("read commands.yaml");
    world.run_commands_before = Some(existing.clone());
    match givn::cli::upgrade::backfill_commands(&existing) {
        Some(backfilled) => {
            std::fs::write(&path, backfilled).expect("write commands.yaml");
            world.run_backfill_applied = Some(true);
        }
        None => world.run_backfill_applied = Some(false),
    }
}

#[then(regex = r#"^the backfilled commands file contains "(.*)"$"#)]
fn backfilled_contains(world: &mut GivnWorld, needle: String) {
    let env = world.env.as_ref().expect("a project root is required");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert!(
        content.contains(&needle),
        "backfilled commands file lacks {needle:?}:\n{content}"
    );
}

#[then("the backfilled commands file keeps the original content")]
fn backfilled_keeps_original(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    let original = world
        .run_commands_before
        .as_deref()
        .expect("the pre-backfill content is required");
    let original_trimmed = original.trim_end();
    assert!(
        content.starts_with(original_trimmed),
        "backfill dropped or reordered the original content.\noriginal:\n{original}\nbackfilled:\n{content}"
    );
}

#[then("the commands file is unchanged")]
fn commands_file_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert_eq!(
        world.run_backfill_applied,
        Some(false),
        "the backfill must not rewrite a file that already declares run:"
    );
    assert_eq!(
        Some(&content),
        world.run_commands_before.as_ref(),
        "commands file changed although the backfill reported no diff"
    );
}

// --- Managed upgrade: supported format and migration baseline -------------

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
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string())
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

fn amend_fixture_commit(env: &common::TestEnv, message: &str) {
    match env.backend() {
        common::BackendKind::Git => {
            let add = common::git(env, &["add", "--all"]);
            assert!(add.status.success());
            let commit = common::git(
                env,
                &["commit", "--amend", "--allow-empty", "-q", "-m", message],
            );
            assert!(
                commit.status.success(),
                "git amend failed: {}",
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
        common::BackendKind::Fossil => givn::fossil::FossilCheckout::at(&env.root())
            .revision()
            .expect("read Fossil revision"),
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
            let checkout = givn::fossil::FossilCheckout::at(&env.root());
            let revision = checkout.revision().expect("read Fossil revision");
            let parent = checkout
                .parent_revision(&revision)
                .expect("read Fossil parent")
                .expect("Fossil parent");
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
    let document = givn::gherkin::usecase_md::usecase_md_template()
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

fn current_fossil_project_with_stale_integration() -> common::TestEnv {
    let env = common::make_fossil_root();
    let skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut content = std::fs::read_to_string(&skill).expect("read generated skill");
    content.push_str("\nstale generated marker\n");
    std::fs::write(&skill, content).expect("mark skill stale");
    seed_user_files(&env);
    append_config(
        &env,
        "\nextensions:\n  specs:\n    - Keep fixture specs stable.\n",
    );
    common::fossil_commit_all(&env, "fixture: stale generated integration");
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

#[given("a clean Fossil project with stale generated integration and a committed checkpoint")]
fn given_clean_fossil_project(world: &mut GivnWorld) {
    let env = current_fossil_project_with_stale_integration();
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

#[then("the command exits successfully and reports one allowlist-only managed check-in")]
fn then_fossil_managed_checkin(world: &mut GivnWorld) {
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
    let env = world.env.as_ref().expect("upgrade fixture");
    let state = load_upgrade_state(world);
    assert_eq!(
        revision_count(env),
        state.commits_before.expect("check-in count before") + 1
    );
    for (status, path) in latest_revision_changes(env) {
        assert!(
            is_managed_allowlist_path(&path),
            "managed check-in carried an unmanaged path: {status} {path}"
        );
        assert_ne!(path, "AGENTS.md");
    }
}

#[then("no migration change is created")]
fn then_no_migration_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    assert!(migration_changes(env).is_empty());
}

#[given("a Git project with uncommitted user changes")]
fn given_git_dirty_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    std::fs::write(env.path("user-work.txt"), "uncommitted user change\n")
        .expect("write user change");
    append_config(&env, "\n# uncommitted user edit\n");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given("a Fossil project with uncommitted user changes")]
fn given_fossil_dirty_project(world: &mut GivnWorld) {
    let env = common::make_fossil_root();
    std::fs::write(env.path("user-work.txt"), "uncommitted user change\n")
        .expect("write user change");
    append_config(&env, "\n# uncommitted user edit\n");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[then("the command fails before changing project files and names the dirty paths")]
fn then_fails_naming_dirty_paths(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "upgrade unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("clean checkout"), "{stderr}");
    assert!(stderr.contains("user-work.txt"), "{stderr}");
    assert!(stderr.contains("givn/config.yaml"), "{stderr}");
    let env = world.env.as_ref().expect("upgrade fixture");
    let before = load_upgrade_state(world)
        .config_before
        .expect("config snapshot");
    assert_eq!(
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config"),
        before
    );
}

#[then("no revision is created and user changes remain byte-for-byte unchanged")]
fn then_no_revision_and_user_changes_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let state = load_upgrade_state(world);
    assert_eq!(head_revision(env), state.head_before.expect("head before"));
    assert_eq!(
        revision_count(env),
        state.commits_before.expect("commit count before")
    );
    assert_eq!(
        std::fs::read_to_string(env.path("user-work.txt")).expect("read user change"),
        "uncommitted user change\n"
    );
}

#[given("a project without either a Git worktree or a Fossil checkout")]
fn given_project_without_scm(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        config_before: Some(config.clone()),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.additional_output = Some(config);
    world.env = Some(env);
}

#[then("the diagnostic explains that a version-controlled checkpoint is required")]
fn then_checkpoint_required(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("version-controlled checkpoint"), "{stderr}");
}

#[given(
    "a clean checkpointed project using local-only rules and manually prepared current-format configuration"
)]
fn given_local_only_checkpointed_project(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    mark_config_version(&env, LAST_0X_VERSION);
    seed_user_files(&env);
    common::git_init(&env);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        user_files: snapshot_user_files(&env),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given("a tracked generated skill and a committed human-authored AGENTS.md")]
fn given_tracked_rule_and_agents(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut content = std::fs::read_to_string(&skill).expect("read generated skill");
    content.push_str("\nstale generated marker\n");
    std::fs::write(&skill, content).expect("mark tracked skill");
    assert!(
        common::git(
            env,
            &["add", "-f", "--", ".opencode/skills/givn-spec/SKILL.md"]
        )
        .status
        .success(),
        "track the generated skill"
    );
    let agents = env.path("AGENTS.md");
    let mut agents_content = std::fs::read_to_string(&agents).expect("read AGENTS.md");
    agents_content.push_str("\nHuman-authored project instructions.\n");
    std::fs::write(&agents, agents_content).expect("write AGENTS.md");
    amend_fixture_commit(env, "fixture: tracked rule and human AGENTS.md");
    let head = head_revision(env);
    let commits = revision_count(env);
    let user_files = snapshot_user_files(env);
    update_upgrade_state(world, |state| {
        state.head_before = Some(head);
        state.commits_before = Some(commits);
        state.user_files = user_files;
    });
}

#[then(
    "the managed revision removes the generated skill from tracking while leaving its refreshed skill on disk"
)]
fn then_rule_untracked_but_present(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let skill = ".opencode/skills/givn-spec/SKILL.md";
    let tracked = common::git(env, &["ls-files", "--error-unmatch", "--", skill]);
    assert!(
        !tracked.status.success(),
        "the generated skill is still tracked after the managed revision"
    );
    let content = std::fs::read_to_string(env.path(skill)).expect("refreshed skill on disk");
    assert!(
        !content.contains("stale generated marker"),
        "the on-disk skill was not refreshed"
    );
    let mut saw_removal = false;
    for (status, path) in latest_revision_changes(env) {
        if path == skill {
            assert!(
                status.starts_with('D'),
                "expected a tracked removal: {status}"
            );
            saw_removal = true;
        }
    }
    assert!(saw_removal, "the managed revision did not remove {skill}");
}

#[then("the revision does not modify AGENTS.md or human-authored instructions")]
fn then_agents_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    for (status, path) in latest_revision_changes(env) {
        assert_ne!(
            path, "AGENTS.md",
            "managed revision modified AGENTS.md: {status}"
        );
    }
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("read AGENTS.md");
    assert!(agents.contains("Human-authored project instructions."));
    let expected = load_upgrade_state(world)
        .user_files
        .into_iter()
        .find(|(path, _)| path == "AGENTS.md")
        .expect("AGENTS.md snapshot")
        .1;
    assert_eq!(agents, expected);
}

#[given("a clean disposable project fixture with a synthetic 1.0 version marker")]
fn given_git_synthetic_project(world: &mut GivnWorld) {
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
    world.env = Some(env);
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
    "a synthetic migration route to the simulated later target 1.1 is supplied through the test-only seam"
)]
fn given_synthetic_route(world: &mut GivnWorld) {
    supply_synthetic_route(world);
}

#[given("an existing migration proposal and design for that synthetic route")]
fn given_existing_plan(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("synthetic fixture");
    let dir = env.path(&format!("givn/changes/{SYNTHETIC_CHANGE}"));
    std::fs::create_dir_all(&dir).expect("create existing migration change");
    std::fs::write(dir.join("proposal.md"), "# Proposal: existing route\n")
        .expect("write existing proposal");
    std::fs::write(dir.join("design.md"), "# Design: existing route\n")
        .expect("write existing design");
    commit_fixture(env, "fixture: existing migration plan");
    let plan_files = vec![
        (
            "proposal.md".to_string(),
            std::fs::read_to_string(dir.join("proposal.md")).expect("read proposal"),
        ),
        (
            "design.md".to_string(),
            std::fs::read_to_string(dir.join("design.md")).expect("read design"),
        ),
    ];
    let head = head_revision(env);
    let commits = revision_count(env);
    update_upgrade_state(world, |state| {
        state.plan_files = plan_files;
        state.migration_id = Some(SYNTHETIC_CHANGE.to_string());
        state.commits_before = Some(commits);
        state.head_before = Some(head);
    });
}

#[then("the existing proposal and design remain byte-for-byte unchanged")]
fn then_existing_plan_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("synthetic fixture");
    let state = load_upgrade_state(world);
    for (name, before) in &state.plan_files {
        let actual =
            std::fs::read_to_string(env.path(&format!("givn/changes/{SYNTHETIC_CHANGE}/{name}")))
                .expect("read plan file");
        assert_eq!(&actual, before);
    }
}

#[then("the report identifies the existing migration change")]
fn then_report_identifies_existing_change(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    let id = load_upgrade_state(world)
        .migration_id
        .unwrap_or_else(|| SYNTHETIC_CHANGE.to_string());
    assert!(stdout.contains(&id), "{stdout}");
    assert!(stdout.contains("already exists"), "{stdout}");
}

#[given(regex = r#"^a clean "?(Git|Fossil)"? project created with Givn "([^"]+)"$"#)]
fn given_clean_project_with_version(world: &mut GivnWorld, backend: String, version: String) {
    let env = if backend == "Git" {
        common::make_init_root()
    } else {
        common::make_fossil_root()
    };
    mark_config_version(&env, &version);
    seed_user_files(&env);
    commit_fixture(&env, "fixture: project at the declared version");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        user_files: snapshot_user_files(&env),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given("the project retains recognized retired configuration keys requiring manual transition")]
fn given_retired_keys(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy fixture");
    append_config(
        env,
        "\nabsolute_mode: true\ngrill_me: true\ngrill_with_docs: true\n",
    );
    commit_fixture(env, "fixture: retired configuration keys");
    let head = head_revision(env);
    let commits = revision_count(env);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    update_upgrade_state(world, |state| {
        state.head_before = Some(head);
        state.commits_before = Some(commits);
        state.config_before = Some(config);
    });
}

#[then("the report names 1.0 as the supported migration baseline and explains manual transition")]
fn then_report_names_baseline(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("supported migration baseline is 1.0"),
        "{stderr}"
    );
    assert!(stderr.contains("manual"), "{stderr}");
    assert!(stderr.contains("communication_policy"), "{stderr}");
    assert!(stderr.contains("challenge"), "{stderr}");
    assert!(stderr.contains("skills-only"), "{stderr}");
}

#[then(
    "the report does not treat the 0.x version marker alone as an unsupported configuration format"
)]
fn then_report_accepts_marker(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("unsupported configuration"), "{stderr}");
    assert!(!stderr.contains("unsupported version"), "{stderr}");
    assert!(
        stderr.contains("accepts current-format") || stderr.contains("not the version marker"),
        "{stderr}"
    );
}

#[then("no embedded pre-1.0 migration route or aggregate plan is applied")]
fn then_no_embedded_route(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy fixture");
    assert!(migration_changes(env).is_empty());
    let state = load_upgrade_state(world);
    assert_eq!(head_revision(env), state.head_before.expect("head before"));
    assert_eq!(
        revision_count(env),
        state.commits_before.expect("commit count before")
    );
}

#[then("no automatic compatibility aliases convert absolute_mode, grill_me, or grill_with_docs")]
fn then_no_aliases(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy fixture");
    let state = load_upgrade_state(world);
    let actual = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert_eq!(Some(actual.clone()), state.config_before);
    for retired in ["absolute_mode", "grill_me", "grill_with_docs"] {
        assert!(actual.contains(retired));
    }
}

#[then("project specifications and user-owned files are not destroyed")]
fn then_project_files_intact(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy fixture");
    assert_eq!(
        snapshot_user_files(env),
        load_upgrade_state(world).user_files
    );
}

#[given(regex = r#"^a clean checkpointed project with a 0.x version marker and "([^"]+)"$"#)]
fn given_checkpointed_config_defect(world: &mut GivnWorld, defect: String) {
    let env = common::make_init_root_without_git();
    mark_config_version(&env, LAST_0X_VERSION);
    match defect.as_str() {
        "malformed configuration" => append_config(&env, "\nbroken: [unclosed\n"),
        "the recognized retired absolute_mode key" => {
            append_config(&env, "\nabsolute_mode: true\n")
        }
        "the recognized retired grill_me key" => append_config(&env, "\ngrill_me: true\n"),
        "the recognized retired grill_with_docs key" => {
            append_config(&env, "\ngrill_with_docs: true\n")
        }
        other => panic!("unknown configuration defect '{other}'"),
    }
    common::git_init(&env);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[then(
    regex = r#"^the command refuses before changing project files because of "([^"]+)", not the version marker$"#
)]
fn then_refuses_for_defect(world: &mut GivnWorld, defect: String) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "upgrade unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not the version marker")
            || stderr.contains("not caused by the 0.x version marker"),
        "{stderr}"
    );
    let env = world.env.as_ref().expect("config fixture");
    let before = load_upgrade_state(world)
        .config_before
        .expect("config snapshot");
    assert_eq!(
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config"),
        before
    );
    assert!(
        !stderr.contains("unsupported configuration format"),
        "{stderr}"
    );
    if defect == "malformed configuration" {
        assert!(stderr.contains("malformed"), "{stderr}");
    } else {
        let key = defect
            .strip_prefix("the recognized retired ")
            .and_then(|value| value.strip_suffix(" key"))
            .expect("retired key name");
        assert!(stderr.contains(key), "{stderr}");
    }
}

#[then("the diagnostic identifies the malformed configuration or recognized retired keys")]
fn then_diagnostic_identifies_defect(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("malformed")
            || stderr.contains("absolute_mode")
            || stderr.contains("grill_me")
            || stderr.contains("grill_with_docs"),
        "{stderr}"
    );
}

#[then(
    "the diagnostic explains manual communication_policy, challenge, and skills-only transitions where applicable"
)]
fn then_diagnostic_manual_transitions(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    if stderr.contains("malformed") {
        assert!(stderr.contains("Repair"), "{stderr}");
    } else {
        assert!(stderr.contains("communication_policy"), "{stderr}");
        assert!(stderr.contains("challenge"), "{stderr}");
        assert!(stderr.contains("skills-only"), "{stderr}");
    }
}

#[then("no automatic configuration migration or compatibility alias is applied")]
fn then_no_config_migration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("config fixture");
    let state = load_upgrade_state(world);
    assert_eq!(
        Some(std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config")),
        state.config_before
    );
    assert!(migration_changes(env).is_empty());
}

#[given(regex = r#"^a clean checkpointed "?(Git|Fossil)"? project ready for a managed upgrade$"#)]
fn given_checkpointed_upgrade_project(world: &mut GivnWorld, backend: String) {
    let env = if backend == "Git" {
        common::make_init_root()
    } else {
        common::make_fossil_root()
    };
    mark_config_version(&env, LAST_0X_VERSION);
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
    commit_fixture(&env, "fixture: checkpointed project");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given(regex = r#"^the upgrade will fail "(.*)"$"#)]
fn given_upgrade_failure(world: &mut GivnWorld, failure: String) {
    let env = world.env.as_ref().expect("upgrade fixture");
    match failure.as_str() {
        "after some managed files are written" => {
            let skill = env.path(".opencode/skills/givn-spec/SKILL.md");
            let mut permissions = std::fs::metadata(&skill)
                .expect("skill metadata")
                .permissions();
            std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o444);
            std::fs::set_permissions(&skill, permissions).expect("make skill read-only");
        }
        "while publishing the managed revision" => match env.backend() {
            common::BackendKind::Git => {
                let hook = env.path(".git/hooks/pre-commit");
                std::fs::create_dir_all(hook.parent().unwrap()).expect("create hooks dir");
                std::fs::write(&hook, "#!/bin/sh\nexit 1\n").expect("write rejecting hook");
                use std::os::unix::fs::PermissionsExt;
                let mut permissions = std::fs::metadata(&hook)
                    .expect("hook metadata")
                    .permissions();
                permissions.set_mode(0o755);
                std::fs::set_permissions(&hook, permissions).expect("make hook executable");
            }
            common::BackendKind::Fossil => {
                let hook = common::fossil(
                    env,
                    &[
                        "hook",
                        "add",
                        "--type",
                        "before-commit",
                        "--command",
                        "exit 1",
                    ],
                );
                assert!(
                    hook.status.success(),
                    "fossil hook failed: {}",
                    String::from_utf8_lossy(&hook.stderr)
                );
            }
        },
        other => panic!("unknown upgrade failure '{other}'"),
    }
    update_upgrade_state(world, |state| state.failure = Some(failure));
}

#[then("the command exits nonzero and names the failed operation")]
fn then_exits_nonzero_names_operation(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "upgrade unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Managed upgrade failed:"), "{stderr}");
    assert!(
        stderr.contains("commit")
            || stderr.contains("revision")
            || stderr.contains("synchronize")
            || stderr.contains("write"),
        "{stderr}"
    );
}

#[then("the report identifies the checkpoint, completed writes, and revisions actually published")]
fn then_report_checkpoint_and_writes(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    let state = load_upgrade_state(world);
    assert!(stderr.contains("checkpoint:"), "{stderr}");
    assert!(
        stderr.contains(&state.head_before.expect("checkpoint")),
        "{stderr}"
    );
    assert!(stderr.contains("completed writes:"), "{stderr}");
    assert!(stderr.contains("revisions actually published:"), "{stderr}");
    if state.failure.as_deref() == Some("after some managed files are written") {
        assert!(
            stderr.contains("- givn/config.yaml"),
            "completed writes omit the refreshed config: {stderr}"
        );
    }
}

#[then(
    "the report accurately describes any best-effort restoration and the remaining partial state"
)]
fn then_report_restoration(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("best-effort restoration:"), "{stderr}");
    assert!(stderr.contains("remaining partial state"), "{stderr}");
}

#[then("the report distinguishes pending work from completed upgrade work")]
fn then_report_pending_vs_completed(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("pending work:"), "{stderr}");
}

#[then(
    "recovery guidance offers operator-controlled restoration or fix-forward from that checkpoint"
)]
fn then_report_recovery(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("recovery:"), "{stderr}");
    assert!(
        stderr.contains("fix forward") || stderr.contains("restore from checkpoint"),
        "{stderr}"
    );
}

#[then("no automatic version-control reset, clean, or history rewrite is performed")]
fn then_no_history_rewrite(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("upgrade fixture");
    let state = load_upgrade_state(world);
    assert_eq!(head_revision(env), state.head_before.expect("head before"));
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("no automatic"), "{stderr}");
    assert!(stderr.contains("reset"), "{stderr}");
}

#[then("no complete rollback is promised")]
fn then_no_complete_rollback(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(stderr.contains("no complete rollback"), "{stderr}");
}

#[given(
    regex = r#"^a clean checkpointed disposable "?(Git|Fossil)"? project fixture with a synthetic 1.0 version marker$"#
)]
fn given_synthetic_future_project(world: &mut GivnWorld, backend: String) {
    let env = if backend == "Git" {
        common::make_init_root_without_git()
    } else {
        common::make_fossil_root_uncommitted()
    };
    mark_config_version(&env, SYNTHETIC_SOURCE);
    seed_user_files(&env);
    match env.backend() {
        common::BackendKind::Git => common::git_init(&env),
        common::BackendKind::Fossil => common::fossil_commit_all(&env, "fixture: synthetic 1.0"),
    }
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        user_files: snapshot_user_files(&env),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[when("the consumer requests the managed upgrade to the simulated target")]
fn when_upgrade_to_simulated_target(world: &mut GivnWorld) {
    run_upgrade(world);
}

#[then("version detection selects the synthetic route from 1.0 to 1.1")]
fn then_synthetic_route_selected(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "upgrade failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&format!(
            "Prepared Givn {SYNTHETIC_TARGET} migration from {SYNTHETIC_SOURCE}"
        )),
        "{stdout}"
    );
    assert!(stdout.contains(SYNTHETIC_ROUTE), "{stdout}");
}

#[then("the migration plan records the actual managed upgrade revision")]
fn then_plan_records_revision(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("synthetic fixture");
    let metadata = std::fs::read_to_string(
        env.path(&format!("givn/changes/{SYNTHETIC_CHANGE}/migration.yaml")),
    )
    .expect("read migration metadata");
    let parsed: serde_json::Value = serde_yaml_ng::from_str(&metadata).expect("parse metadata");
    let commit = parsed
        .get("upgrade_commit")
        .and_then(|value| value.as_str())
        .expect("upgrade_commit");
    let log = match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["log", "--format=%H"]).stdout).to_string()
        }
        common::BackendKind::Fossil => String::from_utf8_lossy(
            &common::fossil(
                env,
                &["timeline", "-n", "0", "-F", "%H", "ancestors", "current"],
            )
            .stdout,
        )
        .to_string(),
    };
    assert!(
        log.contains(commit),
        "managed revision {commit} is not in history"
    );
}

#[then("no removed pre-1.0 route is needed to produce the plan")]
fn then_no_pre_1_0_route(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("synthetic fixture");
    let ids = migration_changes(env);
    assert!(ids.iter().any(|id| id == SYNTHETIC_CHANGE), "{ids:?}");
    assert!(
        ids.iter().all(|id| !id.starts_with("migrate-0-")),
        "{ids:?}"
    );
}

#[given(
    "a disposable project fixture already prepared by the synthetic migration route from 1.0 to the simulated target 1.1"
)]
fn given_prepared_synthetic_project(world: &mut GivnWorld) {
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
    let plan_files = ["proposal.md", "design.md"]
        .iter()
        .map(|name| {
            (
                name.to_string(),
                std::fs::read_to_string(dir.join(name)).expect("read plan file"),
            )
        })
        .collect();
    let env = world.env.as_ref().expect("synthetic fixture");
    let commits = revision_count(env);
    let head = head_revision(env);
    let design = std::fs::read_to_string(dir.join("design.md")).expect("read design");
    update_upgrade_state(world, |state| {
        state.plan_files = plan_files;
        state.commits_before = Some(commits);
        state.head_before = Some(head);
        state.migration_id = Some(SYNTHETIC_CHANGE.to_string());
    });
    world.additional_output = Some(format!("{commits}\n---DESIGN---\n{design}"));
}

#[given("the same synthetic route and target remain supplied through the test-only seam")]
fn given_same_synthetic_seam(world: &mut GivnWorld) {
    let state = load_upgrade_state(world);
    assert_eq!(state.target.as_deref(), Some(SYNTHETIC_TARGET));
    assert!(state.catalog.is_some());
}

#[given(
    "a disposable project fixture with an automatically created migration change for the synthetic route from 1.0 to the simulated target 1.1"
)]
fn given_honest_migration_change(world: &mut GivnWorld) {
    given_prepared_synthetic_project(world);
}

#[given("that route and target were supplied through the test-only seam")]
fn given_route_was_supplied(world: &mut GivnWorld) {
    given_same_synthetic_seam(world);
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

#[given(
    "a clean checkpointed project with recognizable generated legacy commands beside customized command replacements"
)]
fn given_legacy_commands(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    mark_config_version(&env, LAST_0X_VERSION);
    let commands = env.path(".opencode/commands");
    std::fs::create_dir_all(&commands).expect("create commands dir");
    std::fs::write(
        commands.join("givn-implement.md"),
        "---\ndescription: generated legacy command\n---\n\n<!-- givn:generated -->\n\nRun the implementation step.\n",
    )
    .expect("write generated legacy command");
    let custom = "---\ndescription: project-owned replacement\n---\n\nCustom project workflow.\n";
    std::fs::write(commands.join("givn-review.md"), custom)
        .expect("write custom command replacement");
    common::git_init(&env);
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        user_files: vec![(
            ".opencode/commands/givn-review.md".to_string(),
            custom.to_string(),
        )],
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[then("recognizable generated legacy commands are removed without generating new command files")]
fn then_generated_commands_removed(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy commands fixture");
    assert!(
        !env.path(".opencode/commands/givn-implement.md").exists(),
        "the recognizable generated command was not removed"
    );
    assert!(
        !env.path(".opencode/commands/givn-spec.md").exists(),
        "upgrade generated a new command file"
    );
}

#[then("selected targets receive thin skills directing authors to served instructions")]
fn then_thin_skills(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy commands fixture");
    let skill = env.path(".opencode/skills/givn-implement/SKILL.md");
    let content = std::fs::read_to_string(skill).expect("read generated skill");
    assert!(content.contains("givn instructions"), "{content}");
}

#[then("customized commands remain byte-for-byte unchanged and are reported for manual transition")]
fn then_custom_commands_preserved(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy commands fixture");
    let expected = load_upgrade_state(world)
        .user_files
        .into_iter()
        .find(|(path, _)| path == ".opencode/commands/givn-review.md")
        .expect("custom command snapshot")
        .1;
    let actual = std::fs::read_to_string(env.path(".opencode/commands/givn-review.md"))
        .expect("read custom command");
    assert_eq!(actual, expected);
    let stdout = crate::steps::stdout(world);
    assert!(stdout.contains("manual transition"), "{stdout}");
    assert!(stdout.contains("givn-review.md"), "{stdout}");
}

// --- Declaration scaffold in the managed upgrade --------------------------

const DECLARATION_PATH: &str = "givn/specification.yaml";

/// Project-owned declaration bytes valid under the strict schema; a rewrite
/// would be visible because they differ from the scaffold.
const EDITED_DECLARATION: &str = "# Project-owned declaration fixture.\n\
behavior: [\"givn/specs/**\"]\n\
structure: []\n\
contracts: [\"docs/reference.md\"]\n\
limits: []\n\
e2e: []\n\
source: [\"src/**\"]\n";

fn declaration_env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("upgrade fixture")
}

/// Write an explicit addon setting without running the addon command, so a
/// fixture can present `ise` effective while the declaration is absent.
fn set_addon_flag(env: &common::TestEnv, addon: &str, enabled: bool) {
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read config");
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let start = lines
        .iter()
        .position(|line| line == "addons:")
        .expect("the fixture config declares the addons block");
    lines.insert(start + 1, format!("  {addon}: {enabled}"));
    std::fs::write(&path, format!("{}\n", lines.join("\n"))).expect("write config");
}

fn write_declaration(env: &common::TestEnv, content: &str) {
    std::fs::write(env.path(DECLARATION_PATH), content).expect("write the declaration fixture");
}

fn record_checkpoint(world: &mut GivnWorld, env: common::TestEnv) {
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        config_before: Some(config),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

fn commit_checkpoint(env: &common::TestEnv) {
    commit_fixture(env, "fixture: declaration checkpoint");
}

#[given("a clean Git project with a committed checkpoint and ise effective and no declaration")]
fn given_ise_effective_without_declaration(world: &mut GivnWorld) {
    let env = common::make_init_root();
    set_addon_flag(&env, "ise", true);
    assert!(
        !env.path(DECLARATION_PATH).exists(),
        "the fixture must not carry a declaration"
    );
    commit_checkpoint(&env);
    record_checkpoint(world, env);
}

#[given(
    "a clean Git project with a committed checkpoint and ise effective and a project-edited declaration whose bytes are recorded"
)]
fn given_ise_effective_with_edited_declaration(world: &mut GivnWorld) {
    let env = common::make_init_root();
    set_addon_flag(&env, "ise", true);
    write_declaration(&env, EDITED_DECLARATION);
    commit_checkpoint(&env);
    world.specification_declaration.recorded = Some(EDITED_DECLARATION.to_string());
    record_checkpoint(world, env);
}

#[given(
    "a clean Git project with a committed checkpoint and ise disabled and a leftover declaration whose bytes are recorded"
)]
fn given_ise_disabled_with_leftover_declaration(world: &mut GivnWorld) {
    let env = common::make_init_root();
    write_declaration(&env, EDITED_DECLARATION);
    commit_checkpoint(&env);
    world.specification_declaration.recorded = Some(EDITED_DECLARATION.to_string());
    record_checkpoint(world, env);
}

#[given(
    "a clean Git project with a committed checkpoint and ise effective and an upgrade-written declaration"
)]
fn given_ise_effective_with_upgrade_written_declaration(world: &mut GivnWorld) {
    let env = common::make_init_root();
    set_addon_flag(&env, "ise", true);
    commit_checkpoint(&env);
    let first = common::run_givn(&env, &["upgrade"]);
    assert!(
        first.status.success(),
        "the first upgrade failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let written = std::fs::read_to_string(env.path(DECLARATION_PATH))
        .expect("the first upgrade writes the declaration");
    world.specification_declaration.recorded = Some(written);
    let state = UpgradeState {
        head_before: Some(head_revision(&env)),
        commits_before: Some(revision_count(&env)),
        ..UpgradeState::default()
    };
    store_upgrade_state(world, &state);
    world.env = Some(env);
}

#[given(
    "a clean Git project with a committed checkpoint and ise effective and the path \"givn/specification.yaml\" is a directory"
)]
fn given_ise_effective_with_declaration_directory(world: &mut GivnWorld) {
    let env = common::make_init_root();
    set_addon_flag(&env, "ise", true);
    std::fs::create_dir(env.path(DECLARATION_PATH))
        .expect("create the directory occupying the declaration path");
    commit_checkpoint(&env);
    record_checkpoint(world, env);
}

#[then(regex = r#"^the revision contains the path "([^"]+)"$"#)]
fn then_revision_contains_path(world: &mut GivnWorld, path: String) {
    let env = declaration_env(world);
    let changes = latest_revision_changes(env);
    assert!(
        changes
            .iter()
            .any(|(status, changed)| changed == &path && status != "D"),
        "the managed revision does not contain {path}: {changes:?}"
    );
}

#[then(regex = r#"^the revision contains no path "([^"]+)"$"#)]
fn then_revision_contains_no_path(world: &mut GivnWorld, path: String) {
    let env = declaration_env(world);
    let changes = latest_revision_changes(env);
    assert!(
        !changes.iter().any(|(_, changed)| changed == &path),
        "the managed revision contains {path}: {changes:?}"
    );
}

#[then(regex = r#"^the revision carries no diff for the path "([^"]+)"$"#)]
fn then_revision_carries_no_diff(world: &mut GivnWorld, path: String) {
    let env = declaration_env(world);
    let changes = latest_revision_changes(env);
    assert!(
        !changes.iter().any(|(_, changed)| changed == &path),
        "the managed revision carries a diff for {path}: {changes:?}"
    );
}

#[then("the report names the declaration as written")]
fn then_report_names_declaration_written(world: &mut GivnWorld) {
    let observed = crate::steps::stdout(world);
    assert!(
        observed.contains(DECLARATION_PATH),
        "the report does not name {DECLARATION_PATH}:\n{observed}"
    );
    assert!(
        observed.contains("declared Specification"),
        "the report does not name the declaration as written:\n{observed}"
    );
}

#[then("the report names the declaration as project-owned")]
fn then_report_names_declaration_project_owned(world: &mut GivnWorld) {
    let observed = crate::steps::stdout(world);
    assert!(
        observed.contains(DECLARATION_PATH),
        "the report does not name {DECLARATION_PATH}:\n{observed}"
    );
    assert!(
        observed.contains("project-owned"),
        "the report does not name the declaration as project-owned:\n{observed}"
    );
}

#[then("the command reports the project is current")]
fn then_project_is_current(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(
        output.status.success(),
        "upgrade failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let observed = crate::steps::stdout(world);
    assert!(
        observed.contains("already up to date"),
        "the command does not report the project is current:\n{observed}"
    );
}

#[then("no second managed revision is created")]
fn then_no_second_managed_revision(world: &mut GivnWorld) {
    let env = declaration_env(world);
    let state = load_upgrade_state(world);
    assert_eq!(
        revision_count(env),
        state
            .commits_before
            .expect("revision count before the second upgrade"),
        "the second upgrade created a managed revision"
    );
}

#[then("the declaration bytes are unchanged")]
fn then_declaration_bytes_unchanged(world: &mut GivnWorld) {
    let recorded = world
        .specification_declaration
        .recorded
        .clone()
        .expect("the Given records the declaration bytes first");
    let current = std::fs::read_to_string(declaration_env(world).path(DECLARATION_PATH))
        .expect("read the declaration after the second upgrade");
    assert_eq!(current, recorded, "the declaration bytes changed");
}

#[then(regex = r#"^the command exits nonzero and names "([^"]+)"$"#)]
fn then_exits_nonzero_names_path(world: &mut GivnWorld, path: String) {
    let output = crate::steps::output(world);
    assert!(!output.status.success(), "upgrade unexpectedly succeeded");
    let observed = crate::steps::stderr(world);
    assert!(
        observed.contains(&path),
        "stderr does not name {path}:\n{observed}"
    );
}

#[then("the report identifies the completed writes and the checkpoint")]
fn then_report_completed_writes_and_checkpoint(world: &mut GivnWorld) {
    let observed = crate::steps::stderr(world);
    let state = load_upgrade_state(world);
    assert!(observed.contains("checkpoint:"), "{observed}");
    assert!(
        observed.contains(&state.head_before.expect("checkpoint revision")),
        "{observed}"
    );
    assert!(observed.contains("completed writes:"), "{observed}");
    assert!(
        observed.contains("- givn/config.yaml"),
        "the completed writes omit the refreshed config:\n{observed}"
    );
}
