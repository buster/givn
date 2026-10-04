//! `@e2e` step bindings for the `describe-the-release` capability: the
//! changelog dedupe smoke test (change `release-truth`).
//!
//! The module reuses the fixture, generator, and assertion helpers from
//! `release_steps.rs`; it does not duplicate bindings.

use cucumber::{given, then};

use super::GivnWorld;
use super::release_steps::{changelog_stdout, fixture_commit, install_fixture, make_fixture};

/// The capability the fixture changes are attributed to.
const CAPABILITY: &str = "describe-the-release";

#[given(
    expr = "a release range with three feature revisions of one change that share the release note {string}"
)]
fn given_three_revisions(world: &mut GivnWorld, note: String) {
    let fixture = make_fixture();
    for _ in 0..3 {
        fixture.commit(&format!("feat({CAPABILITY}): {note}"));
    }
    install_fixture(world, fixture);
}

#[given("a documentation revision in the same range")]
fn given_documentation_revision(world: &mut GivnWorld) {
    fixture_commit(
        world,
        "docs(describe-the-release): A documentation revision",
    );
}

#[then(expr = "the changelog lists {string} exactly once")]
fn then_note_once(world: &mut GivnWorld, note: String) {
    let changelog = changelog_stdout(world);
    let occurrences = changelog.matches(&note).count();
    assert_eq!(
        occurrences, 1,
        "the changelog lists {note:?} {occurrences} times:\n{changelog}"
    );
}

#[then("the changelog lists no documentation entry")]
fn then_no_documentation_entry(world: &mut GivnWorld) {
    let changelog = changelog_stdout(world);
    assert!(
        !changelog.contains("A documentation revision"),
        "the changelog lists the documentation revision:\n{changelog}"
    );
}
