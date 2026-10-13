//! Step definitions for the `serve-extended-rules` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). The
//! shared served-instruction content step and the `served_instruction` helper
//! live here.

use std::fs;
use std::process::Output;

use cucumber::{then, when};

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

// --- instruction-extension serving fixtures ---------------------------------

/// Fixture directory the instruction-extension steps use to record the
/// expected rules and the raw guidance outputs.
pub(crate) const EXTENSION_FIXTURE_DIR: &str = ".givn-extension-fixture";

/// The marked section the served extension appends.
pub(crate) const EXTENSION_SECTION_MARKER: &str = "## Project extension";

pub(crate) fn extension_fixture_dir(env: &common::TestEnv) -> std::path::PathBuf {
    env.path(EXTENSION_FIXTURE_DIR)
}

/// Record the rule the extension scenarios expect the served content of `id`
/// to carry in the marked extension section.
pub(crate) fn record_expected_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("rules");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the expected rule for {id}: {error}"));
}

/// Record the rule the extension scenarios expect the served content of `id`
/// NOT to carry (the earlier of two entries for one instruction key).
pub(crate) fn record_excluded_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("excluded");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the excluded rule for {id}: {error}"));
}

/// The rule recorded for `id` by a fixture step.
pub(crate) fn expected_rule(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("rules").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no expected rule recorded for {id}: {error}"))
        .trim()
        .to_string()
}

/// The rule a fixture step recorded as excluded for `id`.
pub(crate) fn excluded_rule(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("excluded").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no excluded rule recorded for {id}: {error}"))
        .trim()
        .to_string()
}

/// Run `givn instructions <id>` — with `--change <active>` for manifest
/// artifact ids — and record stdout, stderr, and the exit code for the
/// instruction-extension assertions.
pub(crate) fn request_instruction(env: &common::TestEnv, id: &str) -> Output {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = active_change_id(env);
        args.push("--change");
        args.push(change.as_str());
    }
    let output = common::run_givn(env, &args);
    let dir = extension_fixture_dir(env).join("served");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(format!("{id}.stdout")), &output.stdout).expect("record served stdout");
    fs::write(dir.join(format!("{id}.stderr")), &output.stderr).expect("record served stderr");
    fs::write(
        dir.join(format!("{id}.status")),
        output.status.code().unwrap_or(-1).to_string(),
    )
    .expect("record served status");
    output
}

/// The recorded served stdout of `id`.
pub(crate) fn recorded_stdout(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stdout"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served output recorded for {id}: {error}"))
}

/// The instruction ids with a recorded served output, in id order.
pub(crate) fn recorded_served_ids(env: &common::TestEnv) -> Vec<String> {
    let dir = extension_fixture_dir(env).join("served");
    let mut ids: Vec<String> = fs::read_dir(&dir)
        .unwrap_or_else(|error| {
            panic!(
                "no guidance request was recorded ({}): {error}",
                dir.display()
            )
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".stdout").map(str::to_string)
        })
        .collect();
    ids.sort();
    assert!(!ids.is_empty(), "no guidance request was recorded");
    ids
}

/// The recorded served stderr of `id`.
pub(crate) fn recorded_stderr(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stderr"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served stderr recorded for {id}: {error}"))
}

/// The marked extension section of a served content, panicking when the
/// section is absent.
pub(crate) fn extension_section<'a>(id: &str, content: &'a str) -> &'a str {
    content
        .split_once(EXTENSION_SECTION_MARKER)
        .unwrap_or_else(|| {
            panic!("the served {id} content carries no marked extension section: {content}")
        })
        .1
}

#[when(regex = r"^the agent requests the ([a-z-]+) (?:instructions|policy) through the Givn CLI$")]
fn agent_requests_instruction(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let output = request_instruction(env, &id);
    world.last_output = Some(output);
}

#[then("the served content carries the changed rule")]
fn served_content_carries_the_changed_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let section = extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing {rule:?}: {content}"
    );
}

#[then("the served content is the built-in rules alone")]
fn served_content_is_the_builtin_rules_alone(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = recorded_stdout(env, "proposal");
    assert!(
        !content.contains(EXTENSION_SECTION_MARKER),
        "the served proposal content carries an extension section: {content}"
    );
    assert!(
        content.contains("proposed by"),
        "the served proposal content is missing the built-in proposal rules: {content}"
    );
}

#[then("the served content carries the built-in proposal rules")]
fn served_content_carries_the_builtin_proposal_rules(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = recorded_stdout(env, "proposal");
    assert!(
        content.contains("proposed by"),
        "the served proposal content is missing the built-in proposal rules: {content}"
    );
}

#[then("the served content carries no extension section")]
fn served_content_carries_no_extension_section(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = recorded_stdout(env, "proposal");
    assert!(
        !content.contains(EXTENSION_SECTION_MARKER),
        "the served proposal content carries an extension section: {content}"
    );
}

#[then("the served content carries the rule from the last entry")]
fn served_content_carries_the_last_entry_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let section = extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing the last entry's rule {rule:?}: {content}"
    );
}

#[then("the served content does not carry the rule from the earlier entry")]
fn served_content_does_not_carry_the_earlier_entry_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = excluded_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    assert!(
        !content.contains(&rule),
        "the served proposal content carries the earlier entry's rule {rule:?}: {content}"
    );
}

#[then("each served content carries its project's rule in the marked extension section")]
fn each_served_content_carries_its_project_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let ids = recorded_served_ids(env);
    assert!(
        ids.len() > 1,
        "expected more than one served content, got {ids:?}"
    );
    for id in ids {
        let rule = expected_rule(env, &id);
        let content = recorded_stdout(env, &id);
        let section = extension_section(&id, &content);
        assert!(
            section.contains(&rule),
            "the served {id} extension section is missing {rule:?}: {content}"
        );
    }
}
