//! Compiled-CLI E2E steps for the 0.2.0-to-0.3.0 migration boundary.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("a clean Git project created with Givn 0.2.0")]
fn clean_legacy_project(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[when("the consumer runs `givn upgrade`")]
fn consumer_runs_upgrade(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the command exits successfully")]
fn command_exits_successfully(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the project config identifies Givn 0.3.0")]
fn config_identifies_current_version(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("one commit contains only the managed scaffold and generated agent integration changes")]
fn first_commit_has_managed_paths(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then(regex = r#"^a migration change named "([^"]+)" exists$"#)]
fn migration_change_exists(_world: &mut GivnWorld, _id: String) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the migration change records the managed upgrade commit")]
fn migration_records_upgrade_commit(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the migration change contains a proposal and detailed design")]
fn migration_contains_plan(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the migration change intentionally has no product specification")]
fn migration_has_no_product_spec(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[given("a Givn 0.2.0 project with uncommitted user changes")]
fn dirty_legacy_project(_world: &mut GivnWorld) {
    unimplemented!("implemented in the dirty-worktree scenario");
}

#[then("the command fails before changing project files")]
fn command_fails_before_file_changes(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("no commit is created")]
fn no_commit_is_created(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[then("the user changes remain byte-for-byte unchanged")]
fn user_changes_remain_unchanged(_world: &mut GivnWorld) {
    unimplemented!("implemented in the migration scenario");
}

#[given("a project that already has the 0.3.0 migration change")]
fn prepared_project(_world: &mut GivnWorld) {
    unimplemented!("implemented in the idempotency scenario");
}

#[then("the command does not overwrite the migration change")]
fn migration_not_overwritten(_world: &mut GivnWorld) {
    unimplemented!("implemented in the idempotency scenario");
}

#[then("no second migration commit is created")]
fn no_second_migration_commit(_world: &mut GivnWorld) {
    unimplemented!("implemented in the idempotency scenario");
}

#[then("the command reports that the project is already prepared for migration")]
fn already_prepared_report(_world: &mut GivnWorld) {
    unimplemented!("implemented in the idempotency scenario");
}

#[given("a Givn 0.2.0 project that is not a Git repository")]
fn non_git_legacy_project(_world: &mut GivnWorld) {
    unimplemented!("implemented in the non-Git scenario");
}

#[then("the command explains that a Git repository is required for the automatic migration")]
fn git_required_report(_world: &mut GivnWorld) {
    unimplemented!("implemented in the non-Git scenario");
}

#[given(regex = r#"^a Givn 0.2.0 project with an existing "([^"]+)" change$"#)]
fn existing_migration_project(_world: &mut GivnWorld, _id: String) {
    unimplemented!("implemented in the existing-plan scenario");
}

#[then("the command does not overwrite the existing migration proposal or design")]
fn existing_plan_preserved(_world: &mut GivnWorld) {
    unimplemented!("implemented in the existing-plan scenario");
}

#[then("the command reports the existing migration change")]
fn existing_plan_reported(_world: &mut GivnWorld) {
    unimplemented!("implemented in the existing-plan scenario");
}
