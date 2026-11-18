//! Step definitions for the `check-specification-health` capability.
//!
//! Every step drives the built binary through `common::run_givn` and reads the
//! fixture tree; none imports `lint` or the declaration loader. Each scenario
//! starts from an `ise`-enabled project and rewrites the declaration so only
//! the list the scenario names is populated.

use cucumber::{given, then};
use serde_json::Value;

use crate::common;
use crate::steps::{GivnWorld, stderr, stdout};

const DECLARATION_PATH: &str = "givn/specification.yaml";

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// An initialized project selecting opencode with `ise` effective. Enabling
/// the addon scaffolds `givn/specification.yaml`, which each Given rewrites.
fn ise_enabled_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let output = common::run_givn(&env, &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

fn yaml_list(values: &[&str]) -> String {
    let entries = values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{entries}]")
}

#[allow(clippy::too_many_arguments)]
fn declaration(
    behavior: &[&str],
    structure: &[&str],
    contracts: &[&str],
    limits: &[&str],
    e2e: &[&str],
    source: &[&str],
) -> String {
    format!(
        "behavior: {}\nstructure: {}\ncontracts: {}\nlimits: {}\ne2e: {}\nsource: {}\n",
        yaml_list(behavior),
        yaml_list(structure),
        yaml_list(contracts),
        yaml_list(limits),
        yaml_list(e2e),
        yaml_list(source),
    )
}

fn write_declaration(world: &GivnWorld, content: &str) {
    std::fs::write(env(world).path(DECLARATION_PATH), content)
        .expect("write the declaration fixture");
}

fn write_manifest(world: &GivnWorld, relative: &str, content: &str) {
    let path = env(world).path(relative);
    std::fs::create_dir_all(path.parent().expect("a manifest has a parent"))
        .expect("create the package directory");
    std::fs::write(path, content).expect("write the package manifest");
}

// --- Given -----------------------------------------------------------------

#[given("an ise-enabled project selecting opencode with no Specification Manifest file")]
fn project_without_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    let path = env(world).path(DECLARATION_PATH);
    std::fs::remove_file(&path).expect("remove the scaffolded declaration");
    assert!(!path.exists(), "the declaration still exists after removal");
}

#[given(
    regex = r#"^an ise-enabled project selecting opencode whose limits category declares "([^"]+)"$"#
)]
fn project_with_limits_entry(world: &mut GivnWorld, path: String) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration(&[], &[], &[], &[path.as_str()], &[], &[]),
    );
}

#[given(
    regex = r#"^an ise-enabled project selecting opencode whose structure and behavior categories both declare "([^"]+)"$"#
)]
fn project_with_duplicate_role_entry(world: &mut GivnWorld, path: String) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration(&[path.as_str()], &[path.as_str()], &[], &[], &[], &[]),
    );
}

#[given(
    regex = r#"^an ise-enabled project selecting opencode whose behavior category declares only "([^"]+)"$"#
)]
fn project_with_behavior_entry(world: &mut GivnWorld, path: String) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration(&[path.as_str()], &[], &[], &[], &[], &[]),
    );
}

#[given(
    regex = r#"^an ise-enabled project selecting opencode whose e2e category declares "([^"]+)"$"#
)]
fn project_with_e2e_entry(world: &mut GivnWorld, path: String) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration(&[], &[], &[], &[], &[path.as_str()], &[]),
    );
}

#[given(
    "an ise-enabled project selecting opencode whose e2e package depends on the source package"
)]
fn project_with_direct_source_dependency(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_manifest(
        world,
        "tests/e2e/Cargo.toml",
        "[package]\nname = \"e2e\"\nversion = \"0.1.0\"\n\n\
         [dependencies]\napp = { path = \"../../crates/app\" }\n",
    );
    write_manifest(
        world,
        "crates/app/Cargo.toml",
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
    );
    write_declaration(
        world,
        &declaration(&[], &[], &[], &[], &["tests/e2e"], &["crates/app/**"]),
    );
}

#[given(
    "an ise-enabled project selecting opencode whose e2e package depends transitively on the source package"
)]
fn project_with_transitive_source_dependency(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_manifest(
        world,
        "tests/e2e/Cargo.toml",
        "[package]\nname = \"e2e\"\nversion = \"0.1.0\"\n\n\
         [dependencies]\nmid = { path = \"../../crates/mid\" }\n",
    );
    write_manifest(
        world,
        "crates/mid/Cargo.toml",
        "[package]\nname = \"mid\"\nversion = \"0.1.0\"\n\n\
         [dependencies]\napp = { path = \"../app\" }\n",
    );
    write_manifest(
        world,
        "crates/app/Cargo.toml",
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
    );
    write_declaration(
        world,
        &declaration(&[], &[], &[], &[], &["tests/e2e"], &["crates/app/**"]),
    );
}

#[given(
    "an ise-enabled project selecting opencode whose e2e category declares a package with no Cargo manifest"
)]
fn project_with_package_without_manifest(world: &mut GivnWorld) {
    ise_enabled_project(world);
    std::fs::create_dir_all(env(world).path("tests/e2e"))
        .expect("create the e2e directory without a manifest");
    write_declaration(world, &declaration(&[], &[], &[], &[], &["tests/e2e"], &[]));
}

#[given("an ise-enabled project selecting opencode whose e2e package manifest is malformed")]
fn project_with_malformed_manifest(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_manifest(
        world,
        "tests/e2e/Cargo.toml",
        "[package\nname = \"e2e\"\nversion = \"0.1.0\"\n",
    );
    write_declaration(world, &declaration(&[], &[], &[], &[], &["tests/e2e"], &[]));
}

#[given("an ise-enabled project selecting opencode with a malformed Specification Manifest")]
fn project_with_malformed_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_declaration(world, "behavior: [\"givn/specs/**\"\n");
}

#[given("an active change with one clean delta feature")]
fn active_change_with_clean_delta_feature(world: &mut GivnWorld) {
    common::write_fixture_delta(
        env(world),
        "add-declaration-check",
        "declaration-check",
        "A clean delta feature is checked",
    );
}

#[given("an ise-enabled project selecting opencode with the scaffolded Specification Manifest")]
fn project_with_scaffolded_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    assert!(
        env(world).path(DECLARATION_PATH).is_file(),
        "enabling ise scaffolded no declaration"
    );
}

#[given(regex = r#"^the path "([^"]+)" does not exist$"#)]
fn path_does_not_exist(world: &mut GivnWorld, path: String) {
    assert!(
        !env(world).path(&path).exists(),
        "expected {path:?} to be absent"
    );
}

#[given(regex = r#"^a \.feature file at "([^"]+)"$"#)]
fn feature_file_at(world: &mut GivnWorld, path: String) {
    let full = env(world).path(&path);
    let directory = full.parent().expect("a feature file has a parent");
    std::fs::create_dir_all(directory).expect("create the feature directory");
    std::fs::write(
        &full,
        "Feature: Billing\n\n  Scenario: An invoice is issued\n    Given a customer exists\n    Then an invoice is issued\n",
    )
    .expect("write the feature file");
    let usecase = directory
        .file_name()
        .and_then(|name| name.to_str())
        .expect("the feature directory names its use case");
    let capability = full
        .file_stem()
        .and_then(|name| name.to_str())
        .expect("the feature file names its capability");
    std::fs::write(
        directory.join("usecase.md"),
        format!(
            "# Use case: {usecase}\n\n## Level\n\n!\n\n## Actors\n\n- Operator\n\n## Goal\n\nInspect behavior.\n\n## Trigger\n\nThe operator asks.\n\n## Preconditions\n\n- The project exists.\n\n## Main flow\n\n1. Inspect.\n\n## Extensions\n\n- none\n\n## Rules\n\n- The project remains valid.\n\n## Examples\n\n- A project.\n\n## Minimal guarantee\n\nThe project remains inspectable.\n\n## Success guarantee\n\nThe result is visible.\n\n## Personas\n\n- none\n\n## Capabilities\n\n- {capability}\n\n## Interactions\n\n- none\n\n## Includes\n\n- none\n\n## Extends\n\n- none\n\n## Out of scope\n\nNothing.\n\n## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
        ),
    )
    .expect("write the use-case document");
}

// --- Then ------------------------------------------------------------------

#[then("stderr reports the duplicate manifest entry once")]
fn duplicate_declaration_reported_once(world: &mut GivnWorld) {
    let observed = stderr(world);
    let count = observed
        .lines()
        .filter(|line| line.contains("duplicate declaration"))
        .count();
    assert_eq!(
        count, 1,
        "expected exactly one duplicate declaration finding:\n{observed}"
    );
}

#[then("stderr reports the missing e2e package once")]
fn missing_e2e_package_reported_once(world: &mut GivnWorld) {
    let observed = stderr(world);
    let count = observed
        .lines()
        .filter(|line| line.contains("role entry"))
        .count();
    assert_eq!(
        count, 1,
        "expected exactly one e2e boundary finding for the missing package:\n{observed}"
    );
}

#[then(regex = r#"^stderr names the e2e package and the dependency edge into "([^"]+)"$"#)]
fn e2e_package_and_edge_named(world: &mut GivnWorld, layer: String) {
    let observed = stderr(world);
    assert!(
        observed.contains("e2e package"),
        "stderr does not name the e2e package:\n{observed}"
    );
    let edge = format!("dependency edge into {layer}");
    assert!(
        observed.contains(&edge),
        "stderr does not name the {edge:?}:\n{observed}"
    );
}

#[then(regex = r#"^stderr names "([^"]+)" and the reason "([^"]+)"$"#)]
fn stderr_names_subject_and_reason(world: &mut GivnWorld, subject: String, reason: String) {
    let observed = stderr(world);
    assert!(
        observed.contains(&subject),
        "stderr does not name {subject:?}:\n{observed}"
    );
    assert!(
        observed.contains(&reason),
        "stderr does not state the reason {reason:?}:\n{observed}"
    );
}

#[then("stderr contains no dependency-edge finding")]
fn stderr_contains_no_dependency_edge(world: &mut GivnWorld) {
    let observed = stderr(world);
    assert!(
        !observed.contains("dependency edge"),
        "stderr carries a dependency-edge finding:\n{observed}"
    );
    assert!(
        !observed.contains("no package manifest"),
        "a malformed manifest was reported as a missing manifest:\n{observed}"
    );
}

#[then(regex = r#"^stdout reports a finding with severity "([^"]+)" naming "([^"]+)"$"#)]
fn stdout_reports_finding(world: &mut GivnWorld, severity: String, needle: String) {
    let observed = stdout(world);
    let findings: Vec<Value> =
        serde_json::from_str(&observed).expect("stdout is a JSON finding array");
    let matching = findings.iter().find(|finding| {
        finding["severity"].as_str() == Some(severity.as_str())
            && (finding["files"].as_array().is_some_and(|files| {
                files
                    .iter()
                    .any(|file| file.as_str() == Some(needle.as_str()))
            }) || finding["titles"].as_array().is_some_and(|titles| {
                titles.iter().any(|title| {
                    title
                        .as_str()
                        .is_some_and(|title| title.contains(needle.as_str()))
                })
            }))
    });
    let finding =
        matching.unwrap_or_else(|| panic!("no {severity} finding names {needle:?}:\n{observed}"));
    assert!(
        finding["kind"]
            .as_str()
            .is_some_and(|kind| !kind.is_empty()),
        "the finding carries no kind: {finding}"
    );
}
