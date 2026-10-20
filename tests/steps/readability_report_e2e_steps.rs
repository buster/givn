//! E2E step definitions for the readability report capability.
//!
//! Every step in this file drives the real `givn` binary through
//! `common::run_givn`; no in-process pipeline call belongs here.

use crate::steps::{GivnWorld, stderr, stdout};
use cucumber::then;
use std::path::Path;

#[then("the report recommends a plan")]
fn report_recommends_plan(world: &mut GivnWorld) {
    let stdout = stdout(world);
    assert!(
        stdout.contains("PLAN") || stdout.contains("decomposition plan"),
        "expected a plan recommendation, got: {stdout}"
    );
}

#[then("the report writes no baseline file")]
fn report_writes_no_baseline(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        !env.path("givn/readability.json").exists(),
        "unexpected readability state file; stderr: {}",
        stderr(world)
    );
    assert!(
        !contains_baseline_name(&env.root()),
        "unexpected baseline file under {}",
        env.root().display()
    );
}

fn contains_baseline_name(path: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(path) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry
            .file_name()
            .to_string_lossy()
            .to_lowercase()
            .contains("baseline")
        {
            return true;
        }
        if path.is_dir() && contains_baseline_name(&path) {
            return true;
        }
    }
    false
}
