//! Step definitions for the `serve-extended-rules` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). The
//! shared served-instruction content step and the `served_instruction` helper
//! live here.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};
use std::fs;
use std::process::Output;

const MANIFEST_ARTIFACT_IDS: [&str; 6] = [
    "proposal",
    "specs",
    "design",
    "design-review",
    "tasks",
    "review",
];

fn single_active_change(env: &common::TestEnv) -> Option<String> {
    let dir = env.path("givn/changes");
    let mut ids: Vec<String> = std::fs::read_dir(&dir)
        .ok()?
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
    (ids.len() == 1).then(|| ids.remove(0))
}

fn active_change_id(env: &common::TestEnv) -> String {
    single_active_change(env).unwrap_or_else(|| {
        panic!(
            "expected exactly one active change in the fixture under {}",
            env.path("givn/changes").display()
        )
    })
}

pub(crate) fn served_instruction(env: &common::TestEnv, id: &str) -> String {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = single_active_change(env).unwrap_or_else(|| id.to_string());
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

pub(crate) fn request_change_guidance(env: &common::TestEnv, id: &str) -> Output {
    let change = match single_active_change(env) {
        Some(change) => change,
        None => {
            let output = common::run_givn(env, &["new", "guidance-amendments"]);
            assert!(
                output.status.success(),
                "create the guidance fixture change failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            "guidance-amendments".to_string()
        }
    };
    common::run_givn(env, &["instructions", id, "--change", &change])
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

pub(crate) const RECORDED_EXTENSION_RULE: &str = "Keep proposals under 500 words";

pub(crate) fn record_builtin_content(env: &common::TestEnv, id: &str, content: &[u8]) {
    let dir = extension_fixture_dir(env).join("builtin");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), content)
        .unwrap_or_else(|error| panic!("record the built-in content of {id}: {error}"));
}

pub(crate) fn recorded_builtin_content(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("builtin").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no built-in content recorded for {id}: {error}"))
}

pub(crate) fn record_extension_block(env: &common::TestEnv, block: &str) {
    let dir = extension_fixture_dir(env);
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join("extension-block"), block).expect("record the extension block");
}

pub(crate) fn recorded_extension_block(env: &common::TestEnv) -> String {
    let path = extension_fixture_dir(env).join("extension-block");
    fs::read_to_string(&path).expect("the fixture recorded the extension block")
}

pub(crate) fn config_extension_block(env: &common::TestEnv) -> String {
    let config = fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    let mut block: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in config.lines() {
        if !inside {
            if line == "extensions:" {
                inside = true;
                block.push(line);
            }
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            block.push(line);
        } else {
            break;
        }
    }
    assert!(
        !block.is_empty(),
        "the project config carries no active extensions block:\n{config}"
    );
    format!("{}\n", block.join("\n"))
}

#[given("the project records an extension for the proposal instruction")]
fn project_records_proposal_extension(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // Capture the built-in served content before the extension exists, so the
    // ordering assertions compare against the same request path. Only the
    // fixtures with a change context resolve an artifact instruction; the
    // upgrade fixtures carry no change and need no capture.
    let changes_dir = env.path("givn/changes");
    let single_change = fs::read_dir(&changes_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .count()
        })
        .unwrap_or(0)
        == 1;
    if single_change {
        let builtin = request_instruction(env, "proposal");
        assert!(
            builtin.status.success(),
            "givn instructions proposal failed: {}",
            String::from_utf8_lossy(&builtin.stderr)
        );
        record_builtin_content(env, "proposal", &builtin.stdout);
    }
    let block = format!("extensions:\n  proposal:\n    - {RECORDED_EXTENSION_RULE}\n");
    crate::steps::instruction_extension_steps::write_extensions(
        env,
        &format!("  proposal:\n    - {RECORDED_EXTENSION_RULE}\n"),
    );
    record_extension_block(env, &block);
    record_expected_rule(env, "proposal", RECORDED_EXTENSION_RULE);
    // The upgrade scenarios need a clean checkout; the fixture's own state
    // directory travels with the fixture commit.
    common::commit_fixture(env);
}

pub(crate) fn without_skill_route(content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    if lines
        .last()
        .is_some_and(|line| line.starts_with("Skill to load now:"))
    {
        lines.pop();
    }
    lines.join("\n")
}

pub(crate) const REPLACEMENT_MARKER: &str = "REPLACEMENT-MARKER";

pub(crate) fn recorded_replacement(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("replacement").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no replacement recorded for {id}: {error}"))
}

pub(crate) const EXTENSION_FIXTURE_DIR: &str = ".givn-extension-fixture";

pub(crate) fn extension_fixture_dir(env: &common::TestEnv) -> std::path::PathBuf {
    env.path(EXTENSION_FIXTURE_DIR)
}

pub(crate) fn record_expected_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("rules");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the expected rule for {id}: {error}"));
}

pub(crate) fn record_excluded_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("excluded");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the excluded rule for {id}: {error}"));
}

pub(crate) fn expected_rule(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("rules").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no expected rule recorded for {id}: {error}"))
        .trim()
        .to_string()
}

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

pub(crate) fn recorded_stdout(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stdout"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served output recorded for {id}: {error}"))
}

pub(crate) fn recorded_stderr(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stderr"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served stderr recorded for {id}: {error}"))
}

#[when(
    regex = r"^the agent requests the (proposal|dialog|ideation|explore) (?:instructions|policy) through the Givn CLI$"
)]
fn agent_requests_instruction(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let output = request_instruction(env, &id);
    world.last_output = Some(output);
}

#[then("the served content carries the project's rule in the marked extension section")]
fn served_content_carries_the_project_rule_in_the_extension_section(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let (section, _) = marked_extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing {rule:?}: {content}"
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

/// The served extension section of one instruction and its offset.
pub(crate) const EXTENSION_SECTION_MARKER: &str = "## Project extension";

pub(crate) fn marked_extension_section<'a>(id: &str, content: &'a str) -> (&'a str, usize) {
    let at = content.find(EXTENSION_SECTION_MARKER).unwrap_or_else(|| {
        panic!("the served {id} content carries no marked extension section: {content}")
    });
    (&content[at..], at)
}
