//! Regular black-box steps for semantic remediation verification.

use std::fs;

use cucumber::{given, when};

use super::GivnWorld;
use crate::common;

#[given(regex = r#"^the remediation case carries the distinguishing marker "([^"]+)"$"#)]
fn remediation_case_marker(_world: &mut GivnWorld, marker: String) {
    assert!(!marker.trim().is_empty());
}

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"(DUPLIKAT|VARIANTE|VALID-BOUNDARY)\" candidate$"#
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
