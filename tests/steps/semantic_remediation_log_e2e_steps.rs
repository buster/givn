//! CLI E2E steps for the semantic remediation log.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given(
    regex = r#"^an end-to-end project staged for a stamped remediation log on change "([^"]+)"$"#
)]
fn e2e_stamped_remediation_log_project(world: &mut GivnWorld, change_id: String) {
    unimplemented!("pending: e2e stamped log fixture {change_id}")
}

#[when(
    regex = r#"^the remediation E2E user runs change review for "([^"]+)" after classifying, removing, and stamp-committing the duplicate$"#
)]
fn e2e_review_with_stamped_remediation(world: &mut GivnWorld, change: String) {
    unimplemented!("pending: e2e stamped review {change}")
}

#[when(
    regex = r#"^the remediation E2E user archives change "([^"]+)" after a verified stamped remediation$"#
)]
fn e2e_archive_after_verified_remediation(world: &mut GivnWorld, change: String) {
    unimplemented!("pending: e2e archive {change}")
}

#[then("the archived remediation log exists next to the archived semantic report")]
fn archived_log_next_to_report(world: &mut GivnWorld) {
    unimplemented!("pending: archived log next to report")
}

#[then(
    "the remediation log records the before and after run records with stage durations and the stamped commit"
)]
fn e2e_log_records_runs_and_stamp(world: &mut GivnWorld) {
    unimplemented!("pending: e2e log content")
}
