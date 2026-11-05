//! Step definitions for the `initialize-a-project` capability.
//!
//! The scenarios drive the real `givn init` / `givn skills sync` interfaces and
//! read the generated `.maki/plugin.toml` as the filesystem contract.

use cucumber::{given, then};

use crate::common;
use crate::steps::GivnWorld;

/// The six permission keys an older Givn wrote: the fixture starts from this
/// manifest so synchronization must add the session read without flipping or
/// removing what the user already owns.
const LEGACY_MAKI_GRANTS: &str = "\
[permissions]
fs_read = true
fs_write = true
run = true
net = false
env = false
agent = false
";

#[given("a project with a Maki permission manifest that lacks the session read grant")]
fn project_with_legacy_maki_manifest(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "maki"]);
    assert!(
        out.status.success(),
        "givn init --agents maki failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::write(env.path(".maki/plugin.toml"), LEGACY_MAKI_GRANTS)
        .expect("write the legacy Maki manifest");
    world.env = Some(env);
}

#[given("a project with the previous generated Maki permission manifest")]
fn project_with_previous_generated_maki_manifest(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "maki"]);
    assert!(
        out.status.success(),
        "givn init --agents maki failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::write(
        env.path(".maki/plugin.toml"),
        include_str!("../../assets/agent-integrations/maki/plugin-before-attribution.toml"),
    )
    .expect("write the previous generated Maki manifest");
    std::fs::remove_file(env.path(".maki/lua/givn-attribution.lua"))
        .expect("remove the generated attribution integration");
    world.env = Some(env);
}

#[given("a project with an edited Maki permission manifest that revokes the attribution grant")]
fn project_with_edited_maki_manifest(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "maki"]);
    assert!(
        out.status.success(),
        "givn init --agents maki failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let edited = format!(
        "{}\n# user note\n",
        include_str!("../../assets/agent-integrations/maki/plugin-before-attribution.toml")
    );
    std::fs::write(env.path(".maki/plugin.toml"), edited).expect("write the edited Maki manifest");
    world.env = Some(env);
}

fn maki_manifest(world: &GivnWorld) -> String {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    std::fs::read_to_string(env.path(".maki/plugin.toml"))
        .expect("read the generated Maki manifest")
}

fn manifest_line(manifest: &str, key: &str) -> Option<String> {
    manifest
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(&format!("{key} =")))
        .map(str::to_string)
}

#[then(
    regex = r#"^the project's "([^"]+)" grants "([^"]+)" and revokes "([^"]+)", "([^"]+)", and "([^"]+)"$"#
)]
fn manifest_grants_and_revokes(
    world: &mut GivnWorld,
    path: String,
    grant: String,
    revoke_one: String,
    revoke_two: String,
    revoke_three: String,
) {
    assert_eq!(
        path, ".maki/plugin.toml",
        "the scenario named an unexpected manifest: {path}"
    );
    let manifest = maki_manifest(world);
    assert_eq!(
        manifest_line(&manifest, &grant).as_deref(),
        Some(format!("{grant} = true").as_str()),
        "the generated manifest does not grant `{grant}`: {manifest:?}"
    );
    for key in [revoke_one, revoke_two, revoke_three] {
        assert_eq!(
            manifest_line(&manifest, &key).as_deref(),
            Some(format!("{key} = false").as_str()),
            "the generated manifest does not revoke `{key}`: {manifest:?}"
        );
    }
}

#[then(regex = r#"^the project's "([^"]+)" grants "([^"]+)"$"#)]
fn manifest_grants(world: &mut GivnWorld, path: String, grant: String) {
    assert_eq!(
        path, ".maki/plugin.toml",
        "the scenario named an unexpected manifest: {path}"
    );
    let manifest = maki_manifest(world);
    assert_eq!(
        manifest_line(&manifest, &grant).as_deref(),
        Some(format!("{grant} = true").as_str()),
        "the synchronized manifest does not grant `{grant}`: {manifest:?}"
    );
}

#[then(regex = r#"^the project's "([^"]+)" revokes "([^"]+)"$"#)]
fn manifest_revokes(world: &mut GivnWorld, path: String, key: String) {
    assert_eq!(
        path, ".maki/plugin.toml",
        "the scenario named an unexpected manifest: {path}"
    );
    let manifest = maki_manifest(world);
    assert_eq!(
        manifest_line(&manifest, &key).as_deref(),
        Some(format!("{key} = false").as_str()),
        "the project's manifest does not revoke `{key}`: {manifest:?}"
    );
}

#[then("the file \".maki/init.lua\" contains the attribution fence")]
fn maki_init_contains_attribution_fence(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let init = std::fs::read_to_string(env.path(".maki/init.lua")).expect("read .maki/init.lua");
    for line in [
        "-- givn:begin:attribution",
        "local attribution_ok, attribution = pcall(require, \"givn-attribution\")",
        "if attribution_ok and type(attribution) == \"table\" and attribution.setup then",
        "  attribution.setup()",
        "end",
        "-- givn:end:attribution",
    ] {
        assert!(
            init.contains(line),
            "the generated .maki/init.lua lacks the attribution fence line {line:?}: {init:?}"
        );
    }
}

#[then("the project's \".maki/plugin.toml\" keeps the permission keys the user wrote")]
fn manifest_keeps_user_keys(world: &mut GivnWorld) {
    let manifest = maki_manifest(world);
    for line in LEGACY_MAKI_GRANTS
        .lines()
        .filter(|line| !line.starts_with('['))
    {
        let line = line.trim();
        assert!(
            manifest.lines().map(str::trim).any(|actual| actual == line),
            "the synchronized manifest dropped the user's `{line}`: {manifest:?}"
        );
    }
}

#[then("the project's \".maki/plugin.toml\" adds no revocation")]
fn manifest_adds_no_revocation(world: &mut GivnWorld) {
    let manifest = maki_manifest(world);
    let added_false: Vec<&str> = manifest
        .lines()
        .map(str::trim)
        .filter(|line| line.ends_with("= false"))
        .filter(|line| {
            !LEGACY_MAKI_GRANTS
                .lines()
                .map(str::trim)
                .any(|legacy| legacy == *line)
        })
        .collect();
    assert!(
        added_false.is_empty(),
        "synchronization added revocations the user did not write: {added_false:?}\n{manifest:?}"
    );
}
