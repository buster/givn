//! E2E-only step definitions for the archive step-definition gate.
//!
//! Shared Givens and Thens live in `require_merged_step_definitions_steps.rs`.

use cucumber::then;

use crate::steps::GivnWorld;

#[then(regex = r#"^the archived change still contains the step-definition file "([^"]+)"$"#)]
fn archived_change_contains_step_file(_world: &mut GivnWorld, _relative: String) {
    todo!("TODO: assert the delta step file survived the move");
}
