//! Step definitions for the enable-commit-attribution capability.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use cucumber::{given, then};
use serde_json::{Value, json};

use super::GivnWorld;
use crate::common;
use crate::common::opencode_characterize;

/// The saved project configuration files, captured byte for byte.
#[derive(Debug, Default)]
pub(crate) struct ConfigSnapshot {
    files: Vec<(&'static str, Vec<u8>)>,
}

const CONFIGURATION_FILES: [&str; 2] = ["givn/config.yaml", "givn/commands.yaml"];

pub(crate) fn git(root: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git")
}

#[given("an initialized project selecting OpenCode with a V1-only attribution integration")]
fn given_initialized_project_with_v1_only_integration(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    assert!(git(&env.root(), &["init", "--quiet"]).status.success());
    let init = common::run_givn(&env, &["init", "--agents", "opencode", "--no-addons"]);
    assert!(
        init.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    // The legacy file is the supplied V1-only integration: no default export.
    std::fs::copy(
        common::repo_path("tests/fixtures/opencode/prototype/givn-attribution_v1.js"),
        env.path(".opencode/plugins/givn-attribution.js"),
    )
    .expect("seed the V1-only integration at the canonical path");
    std::fs::write(env.path("package.json"), r#"{"type":"module"}"#)
        .expect("write JavaScript module configuration");
    world.env = Some(env);
}

#[given("the project configuration is snapshotted")]
fn given_project_configuration_snapshotted(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let files = CONFIGURATION_FILES
        .iter()
        .map(|name| {
            (
                *name,
                std::fs::read(env.path(name)).expect("read configuration"),
            )
        })
        .collect();
    world.config_snapshot = Some(ConfigSnapshot { files });
}

#[then("the project configuration is unchanged")]
fn then_project_configuration_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let snapshot = world
        .config_snapshot
        .as_ref()
        .expect("a configuration snapshot");
    assert!(!snapshot.files.is_empty(), "the snapshot is empty");
    for (name, before) in &snapshot.files {
        let after = std::fs::read(env.path(name)).expect("read configuration");
        assert_eq!(
            String::from_utf8_lossy(before),
            String::from_utf8_lossy(&after),
            "{name} changed"
        );
    }
}

/// The observations replayed through every generation: sessions A and B each
/// remember their own model, then shells are prepared for A, for an unknown
/// session and for no session. On V2 only the first shell is linked.
fn probe_actions(generation: &str) -> Vec<Value> {
    let remember = |session: &str, model: &str, variant: &str| {
        json!({"op": "remember", "session": session, "provider": "givn-fixture",
               "model": model, "variant": variant})
    };
    let unknown = if generation == "V1" {
        json!("unknown-session")
    } else {
        Value::Null
    };
    vec![
        remember("session-a", "model-a", "high"),
        remember("session-b", "model-b", "low"),
        json!({"op": "run", "session": "session-a", "command": "git commit"}),
        json!({"op": "run", "session": unknown, "command": "git commit"}),
        json!({"op": "run", "session": null, "command": "git commit"}),
    ]
}

fn attribution_env(model: &str, reasoning: &str) -> Value {
    json!({
        "GIT_COMMITTER_NAME": format!("LLM: givn-fixture/{model} via opencode"),
        "GIVN_ATTRIBUTION_ACTIVE": "1",
        "GIVN_ATTRIBUTION_HARNESS": "opencode",
        "GIVN_ATTRIBUTION_LLM": format!("givn-fixture/{model}"),
        "GIVN_ATTRIBUTION_REASONING": reasoning,
    })
}

#[then(
    "the generated OpenCode attribution integration matches the supplied attribution behavior across supported generations"
)]
fn then_generated_integration_matches_supplied_behavior(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let generated = env.path(".opencode/plugins/givn-attribution.js");
    // An alternative module may stand in for the generated plugin, to show this
    // assertion rejects it (for example the obsolete V1-only integration).
    let alternative = std::env::var_os("GIVN_OPENCODE_CHARACTERIZE_ASSET").map(PathBuf::from);
    if alternative.is_none() {
        assert_eq!(
            std::fs::read(&generated).expect("generated OpenCode plugin"),
            include_bytes!("../../assets/agent-integrations/opencode/givn-attribution.js"),
            "the generated plugin must be the embedded asset"
        );
    }
    let asset = alternative.unwrap_or(generated);
    let prototype = common::repo_path("tests/fixtures/opencode/prototype/givn-attribution.js");
    let initial = serde_json::Map::new();
    for generation in ["V1", "V2"] {
        let actions = probe_actions(generation);
        let result = opencode_characterize::run(&asset, generation, &initial, &actions)
            .unwrap_or_else(|error| panic!("{generation} plugin did not load: {error:#}"));
        let expected = vec![
            attribution_env("model-a", "high"),
            attribution_env("model-b", "low"),
            attribution_env("model-b", "low"),
        ];
        assert_eq!(result.shells, expected, "{generation} shell environments");
        // The prototype is the oracle for every shell its observable contract
        // covers: all of V1, and the unlinked V2 shells.
        let oracle = opencode_characterize::run(&prototype, generation, &initial, &actions)
            .expect("the immutable prototype replays the same actions");
        let from = usize::from(generation == "V2");
        assert_eq!(
            result.shells[from..],
            oracle.shells[from..],
            "{generation} differs from the supplied prototype"
        );
    }
}

#[given(regex = r##"^an initialized project selecting "([^"]+)" without addons$"##)]
fn given_initialized_project_selecting_without_addons(world: &mut GivnWorld, targets: String) {
    let env = common::make_bare_root();
    assert!(git(&env.root(), &["init", "--quiet"]).status.success());
    let init = common::run_givn(&env, &["init", "--agents", &targets, "--no-addons"]);
    assert!(
        init.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    world.env = Some(env);
}

#[then("no OpenCode attribution integration exists")]
fn then_no_opencode_integration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let plugin = env.path(".opencode/plugins/givn-attribution.js");
    assert!(
        !plugin.exists(),
        "unexpected OpenCode attribution integration at {}",
        plugin.display()
    );
}
