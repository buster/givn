//! Step definitions for the opt-in `ise` addon under `enable-optional-addons`.
//!
//! The registry default, the prerequisite refusals, and the config-line
//! assertions live here; the shared CLI and manifest steps stay in
//! `default_all_addons_steps` and `steps`. Step bindings are added scenario by
//! scenario at RED.

use cucumber::{given, then};

use crate::common;
use crate::steps::{GivnWorld, stdout};

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// Initialize a project selecting opencode, then disable the prerequisite the
/// scenario names so only the enable's prerequisite check gates `ise`.
#[given("an initialized project selecting opencode with architecture_principles disabled")]
fn initialized_project_with_architecture_principles_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let output = common::run_givn(
        env(world),
        &["addons", "disable", "architecture_principles"],
    );
    assert!(
        output.status.success(),
        "givn addons disable architecture_principles failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r#"^givn/config\.yaml contains the line "(.*)"$"#)]
fn config_contains_line(world: &mut GivnWorld, line: String) {
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    assert!(
        content.lines().any(|candidate| candidate == line),
        "givn/config.yaml has no line {:?}:\n{content}",
        line
    );
}

#[then(regex = r#"^givn/config\.yaml does not contain the line "(.*)"$"#)]
fn config_does_not_contain_line(world: &mut GivnWorld, line: String) {
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    assert!(
        !content.lines().any(|candidate| candidate == line),
        "givn/config.yaml has the line {:?}:\n{content}",
        line
    );
}

#[then("the listing reports ise as disabled")]
fn listing_reports_ise_disabled(world: &mut GivnWorld) {
    let listing = stdout(world);
    assert!(
        listing.contains("[disabled]  ise"),
        "the addon listing does not report ise disabled:\n{listing}"
    );
}

#[then(
    "omitted default-on addons including challenge and communication_policy use the same enabled defaults everywhere"
)]
fn omitted_default_on_addons_use_declared_defaults(world: &mut GivnWorld) {
    let manifest = common::effective_manifest(env(world));
    let listing = stdout(world);
    for addon in ["challenge", "communication_policy"] {
        assert_eq!(
            manifest.addons.get(addon),
            Some(true),
            "omitted addon '{addon}' did not resolve to its enabled declared default"
        );
        assert!(
            listing.contains(&format!("[enabled ]  {addon}")),
            "the listing does not report '{addon}' enabled:\n{listing}"
        );
    }
    assert!(
        env(world)
            .path(".opencode/skills/givn-challenge/SKILL.md")
            .exists(),
        "the synchronized guidance omits the enabled challenge skill"
    );
}

#[then(regex = r#"^the active addon settings keep "([^"]+)" (enabled|disabled)$"#)]
fn active_addon_settings_keep(world: &mut GivnWorld, addon: String, state: String) {
    let expected = state == "enabled";
    let manifest = common::effective_manifest(env(world));
    assert_eq!(
        manifest.addons.get(&addon),
        Some(expected),
        "addon '{addon}' did not stay {state}"
    );
}
