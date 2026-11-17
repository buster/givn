//! E2E-only step definitions for the archive step-definition gate.
//!
//! Shared Givens and Thens live in `require_merged_step_definitions_steps.rs`.

use super::archive_proof_support as support;
use crate::steps::GivnWorld;
use cucumber::then;

#[then(regex = r#"^the archived change still contains the step-definition file "([^"]+)"$"#)]
fn archived_change_contains_step_file(world: &mut GivnWorld, relative: String) {
    let id = world.last_change_id.clone().expect("a change id");
    let path = support::archive_dir(world, &id).join(relative);
    assert!(
        path.exists(),
        "archived step-definition file is missing: {}",
        path.display()
    );
}
