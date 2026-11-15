//! Step definitions for the durable retained-path archive guard
//! (`require-durable-retained-paths`, regular scenarios).
//!
//! The declarations are project-owned `givn/specification.yaml` files; the
//! fixtures enable `ise` after `ready_change`, which removed the change's
//! add-on lock, so no operator override is needed. The generic archive step in
//! `steps::user_runs` commits the fixture and refreshes the change receipt.

use std::fs;

use cucumber::{given, then};

use super::archive_proof_support as support;
use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    support::env(world)
}

fn change_id(world: &GivnWorld) -> String {
    world
        .last_change_id
        .clone()
        .expect("a change id is required")
}

/// Enable `ise` in the fixture (which also scaffolds the declaration).
fn enable_ise(world: &GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A complete six-key declaration, empty except for one populated role.
fn declaration_naming(role: &str, entries: &[String]) -> String {
    let mut text = String::from("# fixture declaration\n");
    for key in ["behavior", "structure", "contracts", "limits", "e2e"] {
        if key == role {
            text.push_str(&format!("{key}:\n"));
            for entry in entries {
                text.push_str(&format!("  - \"{entry}\"\n"));
            }
        } else {
            text.push_str(&format!("{key}: []\n"));
        }
    }
    text.push_str("source:\n  - \"src/**\"\n");
    text
}

fn declaration_role(role: &str) -> &str {
    match role {
        "behavior" | "structure" | "contracts" | "limits" | "e2e" => role,
        other => panic!("unknown declaration role '{other}'"),
    }
}

fn write_declaration(env: &common::TestEnv, content: &str) {
    fs::write(env.path("givn/specification.yaml"), content).expect("write the declaration");
}

// ─── Given: declarations ───────────────────────────────────────────────────

#[given(regex = r#"^an effective declaration whose (\w+) role names "([^"]+)"$"#)]
fn effective_declaration_naming_one_entry(world: &mut GivnWorld, role: String, entry: String) {
    let role = declaration_role(&role);
    enable_ise(world);
    write_declaration(env(world), &declaration_naming(role, &[entry]));
}

#[given(regex = r#"^an effective declaration whose limits role names 21 paths under "([^"]+)"$"#)]
fn effective_declaration_naming_twenty_one_paths(world: &mut GivnWorld, directory: String) {
    enable_ise(world);
    let mut entries = Vec::new();
    for index in 1..=21 {
        let path = format!("{directory}limit-{index:02}.md");
        let target = support::project_base(world).join(&path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).expect("create the limit directory");
        }
        fs::write(&target, format!("limit {index}\n")).expect("write the limit file");
        entries.push(path);
    }
    write_declaration(env(world), &declaration_naming("limits", &entries));
}

#[given("an effective but malformed declaration")]
fn effective_but_malformed_declaration(world: &mut GivnWorld) {
    enable_ise(world);
    let text = "behavior: []\nstructure: []\ncontracts: []\nlimits: []\ne2e: []\nsource: []\nunknown_key: []\n";
    write_declaration(env(world), text);
}

#[given(regex = r#"^a declaration whose (\w+) role names "([^"]+)" while ise is off$"#)]
fn declaration_while_ise_is_off(world: &mut GivnWorld, role: String, entry: String) {
    let role = declaration_role(&role);
    write_declaration(env(world), &declaration_naming(role, &[entry]));
}

// ─── Then: refusal shape ───────────────────────────────────────────────────

#[then("stderr says the archive moves a declared retained path")]
fn stderr_says_a_declared_retained_path_moves(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("a declared retained path is moved by this archive"),
        "stderr does not say a declared retained path moves: {stderr}"
    );
}

#[then("stderr lists 20 declared retained paths")]
fn stderr_lists_twenty_retained_paths(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    let listed = stderr
        .lines()
        .filter(|line| line.trim_start().starts_with("- limits '"))
        .count();
    assert_eq!(
        listed, 20,
        "stderr lists {listed} declared retained paths, not 20: {stderr}"
    );
}

#[then("the change directory has no pending archive receipt")]
fn change_directory_has_no_pending_receipt(world: &mut GivnWorld) {
    let id = change_id(world);
    let receipt = support::project_base(world)
        .join(format!("givn/changes/{id}"))
        .join(givn::proof::RECEIPT_FILE);
    assert!(
        !receipt.exists(),
        "the change directory carries a pending archive receipt: {}",
        receipt.display()
    );
}
