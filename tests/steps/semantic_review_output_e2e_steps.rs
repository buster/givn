//! Compiled-CLI subprocess steps for decision-first semantic review output.

use cucumber::{given, when};

use super::GivnWorld;

#[given("an end-to-end project whose semantic review recommends one pair and filters many")]
fn e2e_review_output_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when(
    "the end-to-end user runs the advisory semantic review for \"running-change\" through the givn CLI"
)]
fn e2e_run_review_default(_world: &mut GivnWorld) {
    unimplemented!()
}
