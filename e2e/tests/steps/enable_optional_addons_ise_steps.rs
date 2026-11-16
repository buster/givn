//! Step definitions for the opt-in `ise` addon under `enable-optional-addons`.
//!
//! The registry default, the prerequisite refusals, and the config-line
//! assertions live here; the shared CLI and manifest steps stay in
//! `default_all_addons_steps` and `steps`. Step bindings are added scenario by
//! scenario at RED.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then};

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

#[then(regex = r#"^the active addon settings keep "([^"]+)" (enabled|disabled)$"#)]
fn active_addon_settings_keep(world: &mut GivnWorld, addon: String, state: String) {
    let expected = state == "enabled";
    let addons = common::addons(&env(world).root());
    assert_eq!(
        addons.get(&addon),
        Some(&expected),
        "addon '{addon}' did not stay {state}"
    );
}

fn enable_ise(env: &common::TestEnv) {
    let output = common::run_givn(env, &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn ise_enabled_project() -> common::TestEnv {
    let env = common::make_init_root();
    enable_ise(&env);
    env
}

#[given("an ise-enabled project selecting opencode")]
fn ise_enabled_project_selecting_opencode(world: &mut GivnWorld) {
    world.env = Some(ise_enabled_project());
}
