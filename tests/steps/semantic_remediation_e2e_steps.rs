//! CLI E2E steps for semantic remediation verification.

use cucumber::{given, when};

use super::GivnWorld;

#[given("an end-to-end project prepared for a duplicate remediation")]
fn e2e_duplicate_remediation_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when(regex = r#"^the end-to-end user runs change review for "([^"]+)"$"#)]
fn e2e_run_change_review(_world: &mut GivnWorld, _change: String) {
    unimplemented!()
}
