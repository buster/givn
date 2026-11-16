//! Step definitions for the durable retained-path archive guard (`@e2e`).
//!
//! The capability is a CLI: the scenario spawns the compiled `givn` binary
//! through `tests/common` and asserts on its exit status and the archived
//! tree. The declaration Givens are shared from
//! `require_durable_retained_paths_steps`.

use super::archive_proof_support as support;
use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::then;
use std::fs;

const SCENARIO: &str = "Consumer archives a change whose declared retained paths survive the move";

#[then(regex = r#"^the archived change contains "([^"]+)"$"#)]
fn archived_change_contains(world: &mut GivnWorld, name: String) {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let archived = support::project_base(world).join(format!("givn/archive/{id}/{name}"));
    assert!(
        archived.is_file(),
        "the archived change does not contain {name:?}: {}",
        archived.display()
    );

    let output = world
        .last_output
        .as_ref()
        .expect("the scenario must run the real CLI first");
    let dir = common::terminal_evidence::transcript_dir(SCENARIO);
    fs::create_dir_all(&dir).expect("create the transcript directory");
    let text = format!(
        "Scenario: {SCENARIO}\nargv: givn archive --change {id}\ncwd (fixture project root): {}\nexit: {}\n\n--- stdout (raw) ---\n{}\n--- stderr (raw) ---\n{}\n--- archived {} ---\n{}\n",
        support::root(world).display(),
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        archived.display(),
        fs::metadata(&archived)
            .map(|meta| format!("{} bytes", meta.len()))
            .unwrap_or_else(|_| "missing".to_string()),
    );
    fs::write(dir.join("transcript.txt"), text).expect("write the transcript");
}
