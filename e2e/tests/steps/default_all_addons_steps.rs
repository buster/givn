//! Steps for the all-addons-enabled default and effective addon policy.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then};

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn effective_addons(world: &GivnWorld) -> std::collections::BTreeMap<String, bool> {
    common::addons(&env(world).root())
}

fn replace_addons(world: &GivnWorld, block: &str) {
    common::remove_addon_locks(env(world));
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let mut kept = Vec::new();
    let mut in_addons = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "addons:" {
            in_addons = true;
            continue;
        }
        if in_addons {
            if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                in_addons = false;
                kept.push(line.to_string());
            }
            continue;
        }
        kept.push(line.to_string());
    }
    while kept.last().is_some_and(|line| line.trim().is_empty()) {
        kept.pop();
    }
    kept.push(block.trim_end().to_string());
    std::fs::write(path, kept.join("\n") + "\n").expect("write givn/config.yaml");
}

fn addon_free(world: &mut GivnWorld, agents: &str) {
    world.env = Some(common::make_bare_root());
    let environment = env(world);
    let output = common::run_givn(environment, &["init", "--no-addons", "--agents", agents]);
    assert!(
        output.status.success(),
        "givn init --no-addons failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[given("an initialized addon-free project with outline disabled")]
fn initialized_addon_free_project_outline_disabled(world: &mut GivnWorld) {
    addon_free(world, "opencode");
    replace_addons(world, "addons:\n  outline: false\n  challenge: false");
}

#[then("unrelated addon opt-outs remain unchanged")]
fn unrelated_addon_opt_outs_remain(world: &mut GivnWorld) {
    let addons = effective_addons(world);
    assert_eq!(addons.get("challenge"), Some(&false));
    assert_eq!(addons.get("outline"), Some(&true));
}

#[then("the command exits 0 and reports that outline is enabled")]
fn command_reports_outline_enabled(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    assert!(output.status.success(), "the addon command failed");
    let out = String::from_utf8_lossy(&output.stdout);
    assert!(out.contains("enabled addon 'outline'"), "{out}");
}

#[then("`givn addons list` reports the effective outline state as enabled")]
fn addons_list_effective_outline(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[enabled ]  outline"), "{stdout}");
}
