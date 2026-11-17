//! Step definitions for the durable retained-path archive guard
//! (`require-durable-retained-paths`, regular scenarios).
//!
//! The declarations are project-owned `givn/specification.yaml` files; the
//! fixtures enable `ise` after `ready_change`, which removed the change's
//! add-on lock, so no operator override is needed. The generic archive step in
//! `steps::user_runs` commits the fixture and refreshes the change receipt.

use super::archive_proof_support as support;
use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::given;
use std::fs;

fn env(world: &GivnWorld) -> &common::TestEnv {
    support::env(world)
}

fn enable_ise(world: &GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

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

#[given(regex = r#"^an effective declaration whose (\w+) role names "([^"]+)"$"#)]
fn effective_declaration_naming_one_entry(world: &mut GivnWorld, role: String, entry: String) {
    let role = declaration_role(&role);
    enable_ise(world);
    write_declaration(env(world), &declaration_naming(role, &[entry]));
}
