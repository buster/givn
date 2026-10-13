//! Step definitions for the `instruction-extension` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). Only
//! mutation, validation, and rejection steps live here; the shared serving
//! assertions live in `serve_extended_rules_steps.rs`.

use std::fs;
use std::path::{Path, PathBuf};

use cucumber::{given, then, when};

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::serve_extended_rules_steps::{expected_rule, record_expected_rule};

/// The rule the fixture records for the proposal instruction.
const RECORDED_PROPOSAL_RULE: &str = "Keep proposals under 500 words";

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// A fresh initialized project with one active change, mirroring the shared
/// `a Givn project with an active change` fixture.
fn active_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let output = common::run_givn(&env, &["new", "instruction-extension"]);
    assert!(
        output.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

/// Append an `extensions:` block (the caller supplies the indented body) to
/// the project's config.
fn write_extensions(env: &common::TestEnv, body: &str) {
    let path = env.path("givn/config.yaml");
    let mut config = fs::read_to_string(&path).expect("read givn/config.yaml");
    if !config.ends_with('\n') {
        config.push('\n');
    }
    config.push_str("\nextensions:\n");
    config.push_str(body);
    fs::write(&path, config).expect("write givn/config.yaml");
}

/// Drop the appended `extensions:` block from the project's config.
fn remove_extensions(env: &common::TestEnv) {
    let path = env.path("givn/config.yaml");
    let config = fs::read_to_string(&path).expect("read givn/config.yaml");
    let lines: Vec<&str> = config.lines().collect();
    let start = lines
        .iter()
        .rposition(|line| *line == "extensions:")
        .expect("the fixture records an extensions block");
    let mut kept = lines[..start].join("\n");
    kept.push('\n');
    fs::write(&path, kept).expect("write givn/config.yaml");
}

/// Replace the appended `extensions:` block with a new one.
fn replace_extensions(env: &common::TestEnv, body: &str) {
    remove_extensions(env);
    write_extensions(env, body);
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_files(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Every installed guidance file: `AGENTS.md` and the files of every agent
/// target.
fn guidance_files(env: &common::TestEnv) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let agents = env.path("AGENTS.md");
    if agents.is_file() {
        files.push(agents);
    }
    for target in [".opencode", ".agents", ".claude", ".omp"] {
        let root = env.path(target);
        if root.is_dir() {
            collect_files(&root, &mut files);
        }
    }
    files.sort();
    files
}

#[given("a Givn project with an active change and an extension for the proposal instruction")]
fn project_with_recorded_proposal_extension(world: &mut GivnWorld) {
    active_project(world);
    let env = env(world);
    write_extensions(
        env,
        &format!("  proposal:\n    - {RECORDED_PROPOSAL_RULE}\n"),
    );
    record_expected_rule(env, "proposal", RECORDED_PROPOSAL_RULE);
}

#[when(
    regex = r#"^the developer changes the project's extension for the proposal instruction to the rule "([^"]+)"$"#
)]
fn developer_changes_proposal_extension(world: &mut GivnWorld, rule: String) {
    let env = env(world);
    replace_extensions(env, &format!("  proposal:\n    - {rule}\n"));
    record_expected_rule(env, "proposal", &rule);
}

#[then("no installed guidance file carries the changed rule")]
fn no_installed_guidance_file_carries_the_changed_rule(world: &mut GivnWorld) {
    let env = env(world);
    let rule = expected_rule(env, "proposal");
    for path in guidance_files(env) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !content.contains(&rule),
            "the installed guidance file {} carries the extension rule {rule:?}",
            path.display()
        );
    }
}

#[then("no revision carries the changed rule")]
fn no_revision_carries_the_changed_rule(world: &mut GivnWorld) {
    let env = env(world);
    let rule = expected_rule(env, "proposal");
    let output = common::git(env, &["log", "--all", "--oneline", &format!("-S{rule}")]);
    assert!(
        output.status.success(),
        "git log failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(
        found.is_empty(),
        "a revision carries the extension rule {rule:?}: {found}"
    );
}
