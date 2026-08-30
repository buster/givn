//! Regular black-box steps for semantic remediation verification.

use std::fs;

use cucumber::{given, when};

use super::{GivnWorld, remediation_change_dir};
use crate::common;

#[given(regex = r#"^the remediation case carries the distinguishing marker "([^"]+)"$"#)]
fn remediation_case_marker(_world: &mut GivnWorld, marker: String) {
    assert!(!marker.trim().is_empty());
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"(DUPLICATE|VARIANT|VALID-BOUNDARY)\" candidate$"#
)]
pub(crate) fn remediation_project(world: &mut GivnWorld, classification: String) {
    let env = common::make_init_root();
    let permanent = env.path("givn/specs");
    if permanent.exists() {
        fs::remove_dir_all(&permanent).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&permanent).expect("create permanent specs");
    fs::write(
        permanent.join("provider.feature"),
        "Feature: Provider catalog\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write remediation permanent spec");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write remediation commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create remediation delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write remediation delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Provider name is unknown` | boundary |\n",
    )
    .expect("write remediation dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Unknown provider is rejected|Provider name is unknown=0.9600".to_string());
    world.semantic_remediation_mode = Some(classification);
}

#[when("the classified duplicate candidate is removed from the active delta")]
fn remove_classified_duplicate(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("remediation project is required");
    fs::remove_file(env.path("givn/changes/running-change/specs/running.feature"))
        .expect("remove classified duplicate delta");
}

#[when("the active variant is consolidated with an explicit parameter")]
fn consolidate_active_variant(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario Outline: Unknown provider is rejected\n    Given a configured provider catalog for <provider>\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n    Examples: providers\n      | provider |\n      | default  |\n",
        "0.9400",
    );
}

#[when("the active variant is rewritten below the boundary without consolidation")]
fn rewrite_variant_without_consolidation(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a differently worded provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9400",
    );
}

#[when(
    "the valid boundary is rewritten with an explicit distinction below the similarity boundary"
)]
fn rewrite_valid_boundary_below_threshold(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command reports a distinct boundary result\n",
        "0.9400",
    );
}

#[when("the valid boundary is rewritten below the boundary without an explicit distinction")]
fn rewrite_valid_boundary_without_distinction(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a differently worded provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9400",
    );
}

#[when("the valid boundary is rewritten with an explicit distinction at the similarity boundary")]
fn rewrite_valid_boundary_at_threshold(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command reports a distinct boundary result\n",
        "0.9500",
    );
}

#[when("the active classified endpoint changes without remediation")]
fn change_classified_endpoint_without_remediation(world: &mut GivnWorld) {
    write_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a changed provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9600",
    );
}

pub(crate) fn write_delta(world: &mut GivnWorld, content: &str, score: &str) {
    let env = world.env.as_ref().expect("remediation project is required");
    fs::write(
        env.path("givn/changes/running-change/specs/running.feature"),
        content,
    )
    .expect("write remediation delta");
    world.retrieval_stub_scores = Some(format!(
        "Unknown provider is rejected|Provider name is unknown={score}"
    ));
}

const DELTA_TO_DELTA_PAIR_SCORE_KEY: &str =
    "Unknown provider is rejected|Unknown provider selection is rejected";

/// A fresh repository with no committed specification history: the permanent
/// spec directory is empty, so the only admitted pair is delta-to-delta.
/// A scenario may stage the pair under a distinct change id so the review
/// surface stays semantically separable.
#[given(
    regex = r#"^a project prepared for semantic remediation with a delta-to-delta "(DUPLICATE|VARIANT|VALID-BOUNDARY)" candidate$"#
)]
pub(crate) fn remediation_delta_to_delta_project(world: &mut GivnWorld, classification: String) {
    remediation_delta_to_delta_world(world, classification, None);
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a delta-to-delta "(DUPLICATE|VARIANT|VALID-BOUNDARY)" candidate on change "([^"]+)"$"#
)]
fn remediation_delta_to_delta_project_named(
    world: &mut GivnWorld,
    classification: String,
    change_id: String,
) {
    remediation_delta_to_delta_world(world, classification, Some(change_id));
}

pub(crate) fn remediation_delta_to_delta_world(
    world: &mut GivnWorld,
    classification: String,
    change_id: Option<String>,
) {
    let change_id = change_id.unwrap_or_else(|| "running-change".to_string());
    world.last_change_id = Some(change_id.clone());
    let env = common::make_init_root();
    let permanent = env.path("givn/specs");
    if permanent.exists() {
        fs::remove_dir_all(&permanent).expect("remove generated specs");
    }
    fs::create_dir_all(&permanent).expect("create empty permanent specs");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write delta-to-delta remediation commands");
    let delta = env.path(&format!("givn/changes/{change_id}/specs/running.feature"));
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta-to-delta delta");
    let initial = "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n"
        .replace("@running-change", &format!("@{change_id}"));
    fs::write(&delta, initial).expect("write delta-to-delta remediation delta");
    fs::write(
        env.path(&format!("givn/changes/{change_id}/review.md")),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Unknown provider selection is rejected` | related |\n",
    )
    .expect("write delta-to-delta remediation dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores = Some(format!("{DELTA_TO_DELTA_PAIR_SCORE_KEY}=0.9600"));
    world.semantic_remediation_mode = Some(classification);
}

#[when("the delta-to-delta duplicate scenario is removed from the active change")]
fn remove_delta_to_delta_duplicate(world: &mut GivnWorld) {
    write_delta_to_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9600",
    );
}

#[when("the delta-to-delta duplicate pair is reworded below the boundary without removal")]
fn reword_delta_to_delta_duplicate(world: &mut GivnWorld) {
    write_delta_to_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a differently worded provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9400",
    );
}

#[when("the delta-to-delta variant is consolidated into a parameterized Scenario Outline")]
fn consolidate_delta_to_delta_variant(world: &mut GivnWorld) {
    write_delta_to_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario Outline: Unknown provider is rejected\n    Given a configured provider catalog for <provider>\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n    Examples: providers\n      | provider |\n      | default  |\n\n  @givn.removed\n  Scenario: Unknown provider selection is rejected\n    Given a removed scenario\n",
        "0.9600",
    );
}

#[when(
    "the delta-to-delta valid boundary is sharpened with an explicit distinction below the similarity boundary"
)]
fn sharpen_delta_to_delta_boundary(world: &mut GivnWorld) {
    write_delta_to_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command reports a distinct boundary result\n",
        "0.9400",
    );
}

#[when(
    "the delta-to-delta valid boundary is reworded below the boundary without an explicit distinction"
)]
fn reword_delta_to_delta_boundary_without_distinction(world: &mut GivnWorld) {
    write_delta_to_delta(
        world,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a differently worded provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
        "0.9400",
    );
}

pub(crate) fn write_delta_to_delta(world: &mut GivnWorld, content: &str, score: &str) {
    let change_id = world.last_change_id.as_deref().unwrap_or("running-change");
    let content = content.replace("@running-change", &format!("@{change_id}"));
    let dir = remediation_change_dir(world);
    fs::write(dir.join("specs/running.feature"), content)
        .expect("write delta-to-delta remediation delta");
    world.retrieval_stub_scores = Some(format!("{DELTA_TO_DELTA_PAIR_SCORE_KEY}={score}"));
}
