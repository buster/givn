//! E2E acceptance steps for compact semantic retrieval serialization.

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

#[given("a shell project with one rich fixture")]
fn rich_corpus(world: &mut GivnWorld) {
    super::semantic_compact_serialization_steps::prepare_rich_project(world);
}

#[when("the maintainer runs the catalogue command")]
fn rebuild_compact_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("compact serialization project");
    world.last_output = Some(common::run_givn_with_env(
        env,
        &["spec", "index"],
        Some(("GIVN_SPEC_EMBEDDER", "stub")),
    ));
}

#[then("stdout reports one indexed permanent scenario")]
fn indexed_stdout(world: &mut GivnWorld) {
    assert!(stdout(world).contains("Indexed 1 permanent scenarios"));
}

#[then("the catalogue contains the rich fixture")]
fn indexed_scenarios(world: &mut GivnWorld) {
    let document = super::semantic_compact_serialization_steps::index(world);
    let entry = super::semantic_compact_serialization_steps::entry(&document, "Rich scenario");
    assert!(
        entry["embedding"]
            .as_array()
            .is_some_and(|values| !values.is_empty())
    );
}
