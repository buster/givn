//! Non-E2E step definitions for the readability report capability.
//!
//! CLI scenarios drive the real `givn` binary; pipeline scenarios call the
//! public `givn::readability` API in process.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::then;

#[then("the readability addon is disabled")]
fn readability_addon_is_disabled(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["addons", "list"]);
    assert!(
        out.status.success(),
        "givn addons list failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[disabled]  readability"),
        "readability should be disabled, got: {stdout}"
    );
}
