//! CLI E2E steps for semantic remediation verification.

use std::fs;

use cucumber::{given, when};

use super::GivnWorld;
use crate::common;

#[given("an end-to-end project prepared for a duplicate remediation")]
fn e2e_duplicate_remediation_project(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLIKAT".to_string());
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
    assert!(!output.status.success(), "initial remediation review passed");
    let report_path = env.path("givn/changes/running-change/semantic-review.md");
    let report = fs::read_to_string(&report_path).expect("read remediation report");
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLIKAT")
        .replace(
            "- rationale: null",
            "- rationale: \"The duplicate is removed from the projected change\"",
        );
    fs::write(report_path, report).expect("classify remediation report");
    fs::remove_file(env.path("givn/changes/running-change/specs/running.feature"))
        .expect("remove duplicate delta for E2E");
}

#[when(regex = r#"^the end-to-end user runs change review for "([^"]+)"$"#)]
fn e2e_run_change_review(world: &mut GivnWorld, change: String) {
    let env = world.env.as_ref().expect("remediation project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", &change],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}
