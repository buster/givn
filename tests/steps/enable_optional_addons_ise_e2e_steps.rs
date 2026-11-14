//! E2E step definitions for the opt-in `ise` addon under
//! `enable-optional-addons`.
//!
//! The `@e2e` addon-posture scenarios are the ones that read the written
//! `givn/config.yaml` and the effective addon set after an interactive or
//! non-interactive `givn init`. The regular scenarios' posture steps stay in
//! `enable_optional_addons_ise_steps.rs`; a phrase is bound once across all
//! files.

use cucumber::then;

use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// Assert a fresh initialization wrote an explicit `true` for exactly the
/// addons the scenario names, and that the effective manifest resolves each of
/// them to enabled. The written settings are the only expected set, so an
/// addon the scenario does not name cannot be switched on without a failure.
#[then(regex = r#"^the active addon settings enable exactly "([^"]+)"$"#)]
fn active_addon_settings_enable_exactly(world: &mut GivnWorld, expected: String) {
    let names: Vec<&str> = expected.split(", ").collect();
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    for name in &names {
        assert!(
            content
                .lines()
                .any(|line| line == format!("  {name}: true")),
            "givn/config.yaml has no explicit line \"  {name}: true\":\n{content}"
        );
    }

    let mut enabled: Vec<&str> = Vec::new();
    let mut in_addons = false;
    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(' ') {
            in_addons = line == "addons:";
            continue;
        }
        if !in_addons {
            continue;
        }
        if let Some(name) = line
            .strip_prefix("  ")
            .and_then(|entry| entry.strip_suffix(": true"))
        {
            enabled.push(name);
        }
    }
    let mut expected = names.clone();
    expected.sort_unstable();
    enabled.sort_unstable();
    assert_eq!(
        enabled, expected,
        "givn/config.yaml enables a different addon set than the scenario names"
    );

    let manifest = common::effective_manifest(env(world));
    for name in &names {
        assert_eq!(
            manifest.addons.get(name),
            Some(true),
            "addon '{name}' does not resolve to enabled=true"
        );
    }
}

/// Assert `givn/config.yaml` carries no setting for the named addon, so its
/// state comes from the declared default alone.
#[then(regex = r#"^the active addon settings do not name "([^"]+)"$"#)]
fn active_addon_settings_do_not_name(world: &mut GivnWorld, addon: String) {
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    let prefix = format!("{addon}:");
    assert!(
        !content
            .lines()
            .any(|line| line.trim_start().starts_with(&prefix)),
        "givn/config.yaml names '{addon}' instead of omitting it:\n{content}"
    );
}
