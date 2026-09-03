//! Step definitions for the run-scaffold backfill.
//!
//! The unit scenarios exercise `backfill_commands` directly against a
//! fixture commands file; the @e2e scenarios (in `run_execution_e2e_steps`)
//! drive the real `givn upgrade` subprocess.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

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
