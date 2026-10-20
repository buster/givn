//! Non-E2E step definitions for the readability report capability.
//!
//! CLI scenarios drive the real `givn` binary; pipeline scenarios call the
//! public `givn::readability` API in process.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a Givn project with the readability experiment disabled")]
fn readability_disabled_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[when(regex = r#"^the developer runs "([^"]+)"$"#)]
fn developer_runs(world: &mut GivnWorld, cmd: String) {
    crate::steps::user_runs_quoted(world, cmd);
}

#[then("the readability addon is listed as disabled")]
fn readability_addon_is_listed_as_disabled(world: &mut GivnWorld) {
    readability_addon_is_disabled(world);
}

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
