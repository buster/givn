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
/// registry's default-on addons and that every registered addon resolves to
/// the state the scenario names.
#[then(regex = r#"^the active addon settings enable exactly "([^"]+)"$"#)]
fn active_addon_settings_enable_exactly(world: &mut GivnWorld, expected: String) {
    let names: Vec<&str> = expected.split(", ").collect();
    let declared_on: Vec<&str> = givn::config::addons::KNOWN_ADDONS
        .iter()
        .filter(|addon| addon.default_enabled)
        .map(|addon| addon.id)
        .collect();
    assert_eq!(
        names, declared_on,
        "the scenario names a different default-on addon set than the registry"
    );
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
    let manifest = common::effective_manifest(env(world));
    for name in givn::config::manifest::Addons::NAMES {
        let enabled = names.contains(&name);
        assert_eq!(
            manifest.addons.get(name),
            Some(enabled),
            "addon '{name}' does not resolve to enabled={enabled}"
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
