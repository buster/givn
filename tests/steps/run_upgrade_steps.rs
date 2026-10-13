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
