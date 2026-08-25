//! CLI acceptance steps for repository-owned review/archive fixtures.

use cucumber::{given, then, when};

use super::GivnWorld;
use super::repository_ownership_fixtures_support::{
    assert_tree_unchanged, duplicate_titles, feature_text, fixture_specs, invoke_archive,
    invoke_givn, setup_fixture,
};

#[given("an isolated ownership fixture with dispositions for every overlap finding")]
fn isolated_fixture(world: &mut GivnWorld) {
    setup_fixture(world, false);
}

#[given("an isolated ownership fixture with a failing pre-merge archive hook")]
fn failing_fixture(world: &mut GivnWorld) {
    setup_fixture(world, true);
}

#[when("the maintainer invokes the fixture review command")]
fn run_fixture_review(world: &mut GivnWorld) {
    invoke_givn(
        world,
        &["check", "review", "--change", "fixture-consolidation"],
    );
}

#[when("the maintainer invokes the fixture archive command")]
fn run_fixture_archive(world: &mut GivnWorld) {
    invoke_archive(world);
}

#[when("the maintainer runs `givn archive --change fixture-consolidation`")]
fn run_failing_fixture_archive(world: &mut GivnWorld) {
    invoke_archive(world);
}

#[then("the fixture command exits 0")]
fn command_exits_zero(world: &mut GivnWorld) {
    let output = super::output(world);
    assert!(output.status.success(), "stderr: {:?}", output.stderr);
}

#[then(regex = r##"^fixture stdout contains "([^"]+)"$"##)]
fn stdout_contains(world: &mut GivnWorld, text: String) {
    let output = super::stdout(world);
    assert!(
        output.contains(&text),
        "stdout missing {text:?}: {output:?}"
    );
}

#[then("the fixture permanent specification tree contains no duplicate scenario titles")]
fn no_duplicate_titles(world: &mut GivnWorld) {
    let duplicates = duplicate_titles(fixture_specs(world));
    assert!(duplicates.is_empty(), "duplicate titles: {duplicates:?}");
}

#[then(regex = r##"^the fixture permanent specification tree contains "([^"]+)"$"##)]
fn permanent_tree_contains(world: &mut GivnWorld, text: String) {
    let content = feature_text(fixture_specs(world));
    assert!(
        content.contains(&text),
        "tree missing {text:?}: {content:?}"
    );
}

#[then(regex = r##"^the fixture permanent specification tree does not contain "([^"]+)"$"##)]
fn permanent_tree_does_not_contain(world: &mut GivnWorld, text: String) {
    let content = feature_text(fixture_specs(world));
    assert!(
        !content.contains(&text),
        "tree contains {text:?}: {content:?}"
    );
}

#[then("the fixture command fails")]
fn command_fails(world: &mut GivnWorld) {
    let output = super::output(world);
    assert!(!output.status.success(), "archive unexpectedly succeeded");
}

#[then("the fixture permanent specification tree remains unchanged")]
fn permanent_tree_unchanged(world: &mut GivnWorld) {
    assert_tree_unchanged(world, fixture_specs(world));
}
