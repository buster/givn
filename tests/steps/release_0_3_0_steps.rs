//! Regular step definitions for the 0.3.0 release and migration contract.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("the Givn release metadata")]
fn release_metadata(_world: &mut GivnWorld) {
    assert!(!env!("CARGO_PKG_VERSION").is_empty());
}

#[when("a consumer asks for the Givn version")]
fn asks_for_version(world: &mut GivnWorld) {
    let output = crate::common::run_givn(&crate::common::make_bare_root(), &["--version"]);
    world.last_output = Some(output);
}

#[then("the reported version matches the package version")]
fn reported_version(world: &mut GivnWorld) {
    let output = crate::steps::output(world);
    assert!(output.status.success(), "version command failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "expected package version {:?}, got {stdout:?}",
        env!("CARGO_PKG_VERSION")
    );
}

#[given("the Cargo feature metadata for the Givn release")]
fn cargo_feature_metadata(_world: &mut GivnWorld) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    assert!(path.exists(), "Cargo.toml is missing");
}

#[when("a maintainer inspects the default Cargo installation features")]
fn inspect_default_cargo_features(world: &mut GivnWorld) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(root)
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.additional_output = Some(String::from_utf8(output.stdout).expect("metadata is UTF-8"));
}

#[then("the default feature set is empty")]
fn default_feature_set_is_empty(world: &mut GivnWorld) {
    let metadata = world.additional_output.as_deref().expect("Cargo metadata");
    let document: serde_json::Value = serde_json::from_str(metadata).expect("parse Cargo metadata");
    let package = document["packages"]
        .as_array()
        .and_then(|packages| packages.iter().find(|package| package["name"] == "givn"))
        .expect("givn package metadata");
    let default_features = package["features"]["default"]
        .as_array()
        .expect("default feature list");
    assert!(
        default_features.is_empty(),
        "default features must be empty, got {default_features:?}"
    );
}

#[then("no optional model features are declared")]
fn no_optional_model_features_declared(world: &mut GivnWorld) {
    let metadata = world.additional_output.as_deref().expect("Cargo metadata");
    let document: serde_json::Value = serde_json::from_str(metadata).expect("parse Cargo metadata");
    let package = document["packages"]
        .as_array()
        .and_then(|packages| packages.iter().find(|package| package["name"] == "givn"))
        .expect("givn package metadata");
    let features = package["features"]
        .as_object()
        .expect("feature declarations must exist");
    let optional: Vec<&String> = features
        .keys()
        .filter(|feature| feature.as_str() != "default")
        .collect();
    assert!(
        optional.is_empty(),
        "no optional features may be declared, got {optional:?}"
    );
}

#[given("the Givn 0.3.0 release files")]
fn release_files(_world: &mut GivnWorld) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(root.join("Cargo.toml").exists());
    assert!(root.join("README.md").exists());
}

#[when("a maintainer reads the changelog")]
fn reads_changelog(world: &mut GivnWorld) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("CHANGELOG.md");
    world.additional_output = Some(std::fs::read_to_string(path).expect("read CHANGELOG.md"));
}

#[then("it contains the user-visible changes from the stacked release")]
fn changelog_contains_stack_scope(world: &mut GivnWorld) {
    let changelog = world
        .additional_output
        .as_deref()
        .expect("changelog content");
    for marker in ["0.3.0", "semantic", "Arc42", "overlap"] {
        assert!(changelog.contains(marker), "changelog lacks {marker:?}");
    }
}

#[then("it contains the 0.2.0 to 0.3.0 migration entry point")]
fn changelog_contains_migration_entry(world: &mut GivnWorld) {
    let changelog = world
        .additional_output
        .as_deref()
        .expect("changelog content");
    assert!(changelog.contains("0.2.0-to-0.3.0"), "{changelog}");
}

#[given("the 0.2.0 to 0.3.0 migration guide")]
fn migration_guide(_world: &mut GivnWorld) {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/migrations/0.2.0-to-0.3.0.md");
    assert!(path.exists(), "migration guide is missing");
}

#[when("an LLM reads the guide from a project using Givn")]
fn llm_reads_guide(world: &mut GivnWorld) {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/migrations/0.2.0-to-0.3.0.md");
    world.additional_output = Some(std::fs::read_to_string(path).expect("read migration guide"));
}

#[then(
    "it finds ordered prompts for inventory, upgrade, synchronization, review, and verification"
)]
fn guide_has_ordered_prompts(world: &mut GivnWorld) {
    let guide = world.additional_output.as_deref().expect("guide content");
    let headings = [
        "## Prompt 1: Inventory",
        "## Prompt 2: Machine-safe upgrade",
        "## Prompt 3: Synchronize generated guidance",
        "## Prompt 4: Review project changes",
        "## Prompt 5: Verify and archive",
    ];
    let mut previous = 0;
    for heading in headings {
        let position = guide
            .find(heading)
            .unwrap_or_else(|| panic!("missing {heading}"));
        assert!(position >= previous, "prompt order changed at {heading}");
        previous = position;
    }
}

#[then("each prompt tells the LLM which project evidence to inspect before editing")]
fn guide_requires_evidence(world: &mut GivnWorld) {
    let guide = world.additional_output.as_deref().expect("guide content");
    let normalized = guide.to_ascii_lowercase();
    assert!(normalized.matches("inspect").count() >= 5, "{guide}");
    assert!(guide.contains("before editing"), "{guide}");
}

#[then("the guide forbids automatic changes to domain specifications and architecture decisions")]
fn guide_forbids_automatic_semantic_edits(world: &mut GivnWorld) {
    let guide = world.additional_output.as_deref().expect("guide content");
    let normalized = guide.to_ascii_lowercase();
    assert!(
        normalized.contains("must not automatically edit"),
        "{guide}"
    );
    assert!(guide.contains("ADR"), "{guide}");
    assert!(guide.contains("givn/specs"), "{guide}");
}

#[given("the embedded Givn migration catalog")]
fn embedded_migration_catalog(_world: &mut GivnWorld) {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/migrations/catalog.yaml");
    assert!(path.exists(), "migration catalog is missing");
}

#[when("a maintainer inspects the migration catalog")]
fn inspect_migration_catalog(world: &mut GivnWorld) {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/migrations/catalog.yaml");
    world.additional_output = Some(std::fs::read_to_string(path).expect("read migration catalog"));
}

#[then("it declares a catalog schema and migration bundle entries")]
fn catalog_declares_schema_and_bundles(world: &mut GivnWorld) {
    let catalog = world.additional_output.as_deref().expect("catalog content");
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(catalog).expect("valid catalog");
    assert!(parsed.get("schema_version").is_some(), "{catalog}");
    assert!(parsed["migrations"].as_sequence().is_some(), "{catalog}");
    assert!(
        !parsed["migrations"].as_sequence().unwrap().is_empty(),
        "{catalog}"
    );
}

#[then("every migration bundle has a source, target, order, proposal asset, and design asset")]
fn catalog_bundles_are_complete(world: &mut GivnWorld) {
    let catalog = world.additional_output.as_deref().expect("catalog content");
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(catalog).expect("valid catalog");
    for bundle in parsed["migrations"]
        .as_sequence()
        .expect("migration bundles")
    {
        for field in [
            "source_version",
            "target_version",
            "order",
            "proposal_asset",
            "design_asset",
        ] {
            assert!(
                bundle.get(field).is_some(),
                "bundle lacks {field}: {bundle:?}"
            );
        }
    }
}

#[given("an automatically created current-version migration change")]
fn automatic_migration_change(world: &mut GivnWorld) {
    let env = crate::steps::release_0_3_0_e2e_steps::legacy_git_project();
    let output = crate::common::run_givn(&env, &["upgrade"]);
    assert!(
        output.status.success(),
        "upgrade failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[when("an LLM opens the migration change")]
fn llm_opens_migration_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration project fixture");
    let id = crate::steps::release_0_3_0_e2e_steps::find_migration_change_id(env);
    let design = std::fs::read_to_string(env.path(&format!("givn/changes/{id}/design.md")))
        .expect("read migration design");
    world.additional_output = Some(design);
}

#[then("the change asks for project-specific task derivation")]
fn change_asks_for_tasks(world: &mut GivnWorld) {
    let design = world
        .additional_output
        .as_deref()
        .expect("migration design");
    assert!(design.contains("project-specific"), "{design}");
    assert!(design.contains("derive tasks"), "{design}");
}

#[then("it does not contain a fabricated design-review pass")]
fn no_fabricated_design_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration project fixture");
    let design = world
        .additional_output
        .as_deref()
        .expect("migration design");
    assert!(
        !design
            .lines()
            .any(|line| line.trim() == "DESIGN-REVIEW: PASS")
    );
    assert!(
        !env.path(&format!(
            "givn/changes/{}/design-review.md",
            crate::steps::release_0_3_0_e2e_steps::find_migration_change_id(env)
        ))
        .exists()
    );
}

#[then("it does not contain a fabricated implementation commit")]
fn no_fabricated_implementation_commit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration project fixture");
    let design = world
        .additional_output
        .as_deref()
        .expect("migration design");
    assert!(!design.contains("IMPLEMENTATION COMMIT:"));
    assert!(
        !env.path(&format!(
            "givn/changes/{}/tasks.md",
            crate::steps::release_0_3_0_e2e_steps::find_migration_change_id(env)
        ))
        .exists()
    );
}

#[then("the normal Givn review and archive gates remain required")]
fn normal_gates_remain_required(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration project fixture");
    let output = crate::common::run_givn(
        env,
        &[
            "graph",
            "--change",
            &crate::steps::release_0_3_0_e2e_steps::find_migration_change_id(env),
        ],
    );
    assert!(output.status.success());
    let graph = String::from_utf8_lossy(&output.stdout);
    assert!(graph.contains("design-review"), "{graph}");
    assert!(graph.contains("tasks"), "{graph}");
    assert!(graph.contains("review"), "{graph}");
}
