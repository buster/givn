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

/// Assert the project's explicit addon lines and its effective resolution
/// both disable exactly the named registry set, `ise` included.
#[then(regex = r#"^the active addon settings disable exactly "([^"]+)"$"#)]
fn active_addon_settings_disable_exactly(world: &mut GivnWorld, expected: String) {
    let names: Vec<&str> = expected.split(", ").collect();
    assert_eq!(
        names,
        givn::config::manifest::Addons::NAMES,
        "the scenario names a different addon set than the registry"
    );
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    for name in &names {
        assert!(
            content
                .lines()
                .any(|line| line == format!("  {name}: false")),
            "givn/config.yaml has no explicit line \"  {name}: false\":\n{content}"
        );
    }
    let manifest = common::effective_manifest(env(world));
    for name in &names {
        assert_eq!(
            manifest.addons.get(name),
            Some(false),
            "addon '{name}' does not resolve to disabled"
        );
    }
}

const DECLARATION_PATH: &str = "givn/specification.yaml";

/// Project bytes the preservation scenarios record: valid six-key content that
/// differs from the scaffold, so a rewrite would be visible.
const PROJECT_DECLARATION: &str = "# Project-owned declaration fixture.\n\
behavior: [\"givn/specs/**\"]\n\
structure: []\n\
contracts: [\"docs/reference.md\"]\n\
limits: []\n\
e2e: []\n\
source: [\"src/**\"]\n";

fn enable_ise(env: &common::TestEnv) {
    let output = common::run_givn(env, &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A Git-backed initialized fixture whose `ise` addon is enabled.
fn ise_enabled_project() -> common::TestEnv {
    let env = common::make_init_root();
    enable_ise(&env);
    env
}

#[given("an initialized project selecting opencode")]
fn initialized_project_selecting_opencode(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("an ise-enabled project selecting opencode")]
fn ise_enabled_project_selecting_opencode(world: &mut GivnWorld) {
    world.env = Some(ise_enabled_project());
}

const LOCK_FIXTURE_CHANGE: &str = "add-declaration-check";

/// The lock a change created before `ise` existed recorded: the nine
/// default-on addons, no `ise` key.
const PRE_ISE_LOCK: &str = r"version: 1
addons:
  arc42: true
  communication_policy: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  architecture_principles: true
  challenge: true
  outline: true
overrides: []
";

#[given("an active change whose addon lock lists the nine default-on addons and omits \"ise\"")]
fn active_change_with_pre_ise_lock(world: &mut GivnWorld) {
    let env = env(world);
    common::write_fixture_delta(
        env,
        LOCK_FIXTURE_CHANGE,
        "addon-lock",
        "Lock acceptance fixture behavior",
    );
    std::fs::write(
        env.path(&format!(
            "givn/changes/{LOCK_FIXTURE_CHANGE}/config-lock.yaml"
        )),
        PRE_ISE_LOCK,
    )
    .expect("write the pre-ise add-on lock");
}

#[given(regex = r#"^a Specification Manifest at "([^"]+)" whose bytes are recorded$"#)]
fn declaration_at_path_with_recorded_bytes(world: &mut GivnWorld, path: String) {
    std::fs::write(env(world).path(&path), PROJECT_DECLARATION)
        .expect("write the project-owned declaration");
    world.specification_declaration.recorded = Some(PROJECT_DECLARATION.to_string());
}

#[given(
    "an ise-enabled project selecting opencode whose Specification Manifest bytes are recorded"
)]
fn ise_enabled_project_with_recorded_declaration(world: &mut GivnWorld) {
    let env = ise_enabled_project();
    std::fs::write(env.path(DECLARATION_PATH), PROJECT_DECLARATION)
        .expect("write the project-owned declaration");
    world.specification_declaration.recorded = Some(PROJECT_DECLARATION.to_string());
    world.env = Some(env);
}

#[given(regex = r#"^the path "([^"]+)" is a directory$"#)]
fn path_is_a_directory(world: &mut GivnWorld, path: String) {
    std::fs::create_dir(env(world).path(&path)).expect("create the directory occupying the path");
}

#[then(regex = r#"^stdout names "([^"]+)"$"#)]
fn stdout_names(world: &mut GivnWorld, needle: String) {
    let observed = stdout(world);
    assert!(
        observed.contains(&needle),
        "stdout does not name {needle:?}:\n{observed}"
    );
}

#[then("the Specification Manifest names exactly the five categories and one source list")]
fn declaration_names_five_roles_and_source(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(env(world).path(DECLARATION_PATH))
        .expect("read the scaffolded declaration");
    let mut keys = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("the declaration line carries no key: {line}"));
        let value = value.trim();
        assert!(
            value.starts_with('[') && value.ends_with(']'),
            "the declaration value for '{key}' is not a list: {value}"
        );
        keys.push(key.to_string());
    }
    assert_eq!(
        keys,
        [
            "behavior",
            "structure",
            "contracts",
            "limits",
            "e2e",
            "source"
        ],
        "the declaration does not name the five roles plus one source list"
    );
}
