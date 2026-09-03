//! Step definitions for the README run managed block.
//!
//! The render predicate is exercised through the real renderer over the
//! overlay manifest the shared declaration Given built.

use cucumber::{then, when};

use super::GivnWorld;

#[when("the run managed block is rendered")]
fn render_run_block(world: &mut GivnWorld) {
    let manifest = world
        .overlay_manifest
        .as_ref()
        .expect("a run declaration is required first");
    world.run_declared_command = Some(manifest.run.command.clone());
    world.run_rendered_block = Some(givn::cli::readme::render_run_block_for(manifest));
}

#[then("the rendered block has the run open placeholder")]
fn rendered_has_placeholder(world: &mut GivnWorld) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    assert!(
        block.contains(
            "<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->"
        ),
        "rendered block lacks the run open placeholder: {block}"
    );
    let todos = block.matches("<!-- givn:todo:").count();
    assert_eq!(todos, 1, "expected exactly one placeholder: {block}");
}

#[then("the rendered block has no open placeholder")]
fn rendered_has_no_placeholder(world: &mut GivnWorld) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    assert!(
        !block.contains("<!-- givn:todo:"),
        "rendered block contains a placeholder: {block}"
    );
}

#[then("the rendered block has no start command")]
fn rendered_has_no_start_command(world: &mut GivnWorld) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    let command = world
        .run_declared_command
        .as_deref()
        .expect("a declared start command is required");
    assert!(
        !block.contains(command),
        "rendered block names the start command {command:?}: {block}"
    );
}

#[then(regex = r#"^the rendered block mentions the start command "(.*)"$"#)]
fn rendered_mentions_start_command(world: &mut GivnWorld, command: String) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    assert!(
        block.contains(&command),
        "rendered block lacks the start command {command:?}: {block}"
    );
}

#[then(regex = r#"^the rendered block mentions the readiness path "(.*)"$"#)]
fn rendered_mentions_readiness_path(world: &mut GivnWorld, path: String) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    assert!(
        block.contains(&path),
        "rendered block lacks the readiness path {path:?}: {block}"
    );
}

#[then(regex = r#"^the rendered block mentions the port hand-off "(.*)"$"#)]
fn rendered_mentions_port_handoff(world: &mut GivnWorld, name: String) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    let needle = format!("${name}");
    assert!(
        block.contains(&needle),
        "rendered block lacks the port hand-off {needle:?}: {block}"
    );
}

#[then("the rendered block says there is nothing to start")]
fn rendered_says_nothing_to_start(world: &mut GivnWorld) {
    let block = world
        .run_rendered_block
        .as_deref()
        .expect("a rendered block is required");
    let library_line = givn::cli::run::LIBRARY_LINE;
    assert!(
        block.contains(library_line),
        "rendered block lacks the library line {library_line:?}: {block}"
    );
}
