//! CLI E2E steps for semantic remediation verification.

use std::fs;

use cucumber::{given, when};

use super::GivnWorld;
use crate::common;

#[given("an end-to-end project prepared for a duplicate remediation")]
fn e2e_duplicate_remediation_project(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLICATE".to_string());
    let env = world.env.as_ref().expect("remediation project is required");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Unknown provider is rejected|Provider name is unknown=0.9600",
            ),
        ],
    );
    assert!(
        !output.status.success(),
        "initial remediation review passed"
    );
    let report_path = env.path("givn/changes/running-change/semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("read remediation report");
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLICATE")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(report_path, report).expect("classify remediation report");
    fs::remove_file(env.path("givn/changes/running-change/specs/running.feature"))
        .expect("remove duplicate delta for E2E");
}

#[given("an end-to-end project prepared for a delta-to-delta duplicate remediation")]
fn e2e_delta2delta_duplicate_remediation_project(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_delta_to_delta_world(
        world,
        "DUPLICATE".to_string(),
        None,
    );
    let env = world
        .env
        .as_ref()
        .expect("delta-to-delta project is required");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Unknown provider is rejected|Unknown provider selection is rejected=0.9600",
            ),
        ],
    );
    assert!(
        !output.status.success(),
        "initial delta-to-delta remediation review passed"
    );
    let report_path = env.path("givn/changes/running-change/semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("read delta-to-delta report");
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLICATE")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(report_path, report).expect("classify delta-to-delta report");
    // Remove the redundant delta scenario, retaining the other one.
    fs::write(
        env.path("givn/changes/running-change/specs/running.feature"),
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("remove delta-to-delta duplicate for E2E");
}

#[given(
    regex = r#"^an e2e project staged for a delta-to-delta duplicate remediation on change "([^"]+)"$"#
)]
fn e2e_delta2delta_duplicate_remediation_project_named(world: &mut GivnWorld, change_id: String) {
    super::semantic_remediation_steps::remediation_delta_to_delta_world(
        world,
        "DUPLICATE".to_string(),
        Some(change_id.clone()),
    );
    let env = world
        .env
        .as_ref()
        .expect("delta-to-delta project is required");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", &change_id],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Unknown provider is rejected|Unknown provider selection is rejected=0.9600",
            ),
        ],
    );
    assert!(
        !output.status.success(),
        "initial delta-to-delta remediation review passed"
    );
    let report_path = env.path(&format!("givn/changes/{change_id}/semantic-review.md"));
    let report = fs::read_to_string(&report_path).expect("read delta-to-delta report");
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLICATE")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(report_path, report).expect("classify delta-to-delta report");
    // Remove the redundant delta scenario, retaining the other one.
    fs::write(
        env.path(&format!("givn/changes/{change_id}/specs/running.feature")),
        format!(
            "@givn.delta @{change_id}\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n"
        ),
    )
    .expect("remove delta-to-delta duplicate for E2E");
}

#[when(regex = r#"^the remediation E2E user runs change review for "([^"]+)"$"#)]
fn e2e_run_change_review(world: &mut GivnWorld, change: String) {
    let env = world.env.as_ref().expect("remediation project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", &change],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}
