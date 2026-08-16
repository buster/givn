//! E2E black-box steps for the semantic review report.

use cucumber::{given, when};

use super::GivnWorld;

#[given("an end-to-end project prepared for semantic review with a similar active delta")]
fn e2e_semantic_review_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when(regex = r#"^the end-to-end user runs change review for "([^"]+)"$"#)]
fn e2e_run_change_review(_world: &mut GivnWorld, _change: String) {
    unimplemented!()
}
