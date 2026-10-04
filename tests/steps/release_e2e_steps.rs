//! `@e2e` step bindings for the `describe-the-release` capability: the
//! changelog dedupe smoke test (change `release-truth`).
//!
//! The module reuses the fixture, generator, and assertion helpers from
//! `release_steps.rs`; it does not duplicate bindings.

use cucumber::{given, then};

use super::GivnWorld;

#[given(
    expr = "a release range with three feature revisions of one change that share the release note {string}"
)]
fn given_three_revisions(_world: &mut GivnWorld, _note: String) {
    unimplemented!("implemented by the dedupe scenario task");
}

#[given("a documentation revision in the same range")]
fn given_documentation_revision(_world: &mut GivnWorld) {
    unimplemented!("implemented by the dedupe scenario task");
}

#[then(expr = "the changelog lists {string} exactly once")]
fn then_note_once(_world: &mut GivnWorld, _note: String) {
    unimplemented!("implemented by the dedupe scenario task");
}

#[then("the changelog lists no documentation entry")]
fn then_no_documentation_entry(_world: &mut GivnWorld) {
    unimplemented!("implemented by the dedupe scenario task");
}
