//! E2E step definitions for the `scm-backend-selection` fragment capability.
//!
//! The scenario drives the compiled `givn` binary through `givn upgrade` in a
//! real temporary Git project; the fixture Givens are shared with
//! `scm_backend_selection_steps.rs`.

use cucumber::then;

use super::GivnWorld;

#[then("the managed upgrade publishes a revision")]
fn then_upgrade_publishes_revision(_world: &mut GivnWorld) {
    unimplemented!()
}
