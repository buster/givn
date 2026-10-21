//! End-to-end step for the `@e2e` readability review gate scenario.
//!
//! The fixture is assembled by the `Given` steps in
//! `readability_review_gate_steps.rs`; this file owns the single user action
//! and drives the real `givn check review --change test-change` subprocess
//! through `common::run_givn`.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::when;

#[when("the developer runs \"givn check review --change test-change\"")]
fn the_developer_runs_the_review(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("the review fixture needs a project root");
    let output = common::run_givn(env, &["check", "review", "--change", "test-change"]);
    world.last_output = Some(output);
    let output = world
        .last_output
        .as_ref()
        .expect("the review subprocess ran");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if world.readability_gate.expect_pass_line {
        assert!(
            output.status.success(),
            "givn check review failed: exit={:?}\nstdout={stdout}\nstderr={}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr),
        );
        assert!(
            stdout.contains("touched 6 unit(s), 2 outline(s), 2 supporting metric finding(s)"),
            "the review did not report the expected touched units, outlines, and supporting findings:\n{stdout}"
        );
        assert_eq!(
            stdout.lines().last().unwrap_or_default(),
            "readability review: PASS",
            "the last stdout line is not the exact pass line:\n{stdout}"
        );
    }
}
