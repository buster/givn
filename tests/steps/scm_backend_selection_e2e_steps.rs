//! E2E step definitions for the `scm-backend-selection` fragment capability.
//!
//! The scenario drives the compiled `givn` binary through `givn upgrade` in a
//! real temporary Git project; the fixture Givens are shared with
//! `scm_backend_selection_steps.rs`.

use cucumber::then;

use super::GivnWorld;
use crate::common;

#[then("the managed upgrade publishes a revision")]
fn then_upgrade_publishes_revision(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let before = world
        .scm_fixture
        .head_before
        .as_ref()
        .expect("a fixture HEAD is required first");
    let head = common::git(env, &["rev-parse", "HEAD"]);
    assert!(head.status.success(), "read upgraded HEAD");
    let after = String::from_utf8_lossy(&head.stdout).trim().to_string();
    assert_ne!(
        &after, before,
        "the managed upgrade must publish a new revision"
    );
    let stdout = super::stdout(world);
    assert!(
        stdout.contains("Refreshed"),
        "stdout does not report the refresh: {stdout}"
    );
    assert!(
        stdout.contains(&after),
        "stdout does not name the published revision: {stdout}"
    );
    assert!(
        env.path(".opencode/plugins/givn-attribution.js").is_file(),
        "the managed upgrade must restore the missing managed file"
    );
}
