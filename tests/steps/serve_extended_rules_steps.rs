//! Step definitions for the `serve-extended-rules` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). The
//! shared served-instruction content step and the `served_instruction` helper
//! live here.

use cucumber::then;

use crate::common;
use crate::steps::GivnWorld;

/// Instruction ids that are manifest artifacts and therefore need a change
/// context when served. Policy and step ids resolve without one.
const MANIFEST_ARTIFACT_IDS: [&str; 6] = [
    "proposal",
    "specs",
    "design",
    "design-review",
    "tasks",
    "review",
];

/// The fixture's single active change id.
fn active_change_id(env: &common::TestEnv) -> String {
    let dir = env.path("givn/changes");
    let mut ids: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("read changes dir {}: {error}", dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    ids.sort();
    assert_eq!(
        ids.len(),
        1,
        "expected exactly one active change in the fixture, got {ids:?}"
    );
    ids.remove(0)
}

/// Run `givn instructions <id>` in the fixture and return its stdout. Manifest
/// artifact ids get `--change <active>`; policy and step ids do not.
pub(crate) fn served_instruction(env: &common::TestEnv, id: &str) -> String {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = active_change_id(env);
        args.push("--change");
        args.push(change.as_str());
    }
    let output = common::run_givn(env, &args);
    assert!(
        output.status.success(),
        "givn instructions {id} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[then(regex = r#"^the served ([a-z-]+) instruction contains "([^"]+)"$"#)]
fn served_instruction_contains(world: &mut GivnWorld, id: String, text: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = served_instruction(env, &id);
    assert!(
        content.contains(&text),
        "the served {id} instruction is missing {text:?}: {content}"
    );
}

#[then(regex = r#"^the served ([a-z-]+) instruction does not contain "([^"]+)"$"#)]
fn served_instruction_does_not_contain(world: &mut GivnWorld, id: String, text: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = served_instruction(env, &id);
    assert!(
        !content.contains(&text),
        "the served {id} instruction must not contain {text:?}: {content}"
    );
}
