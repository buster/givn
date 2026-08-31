//! E2E step definitions for the README enforcement capability.
//!
//! The real interface is the compiled `givn` binary invoked as a subprocess
//! against a temp project root. Shared fixtures live in the unit-level
//! `readme_enforcement_steps` module; this file holds the e2e-exclusive
//! fixture and the scaffold/archive assertions.

use cucumber::{given, then};

use super::GivnWorld;

#[given("a givn project providing its own workflow scripts")]
fn workflow_scripts_project(world: &mut GivnWorld) {
    unimplemented!("write run-tests.sh, measure-coverage.sh, merge-coverages.sh fixtures")
}

#[then("the README contains an open placeholder for the description")]
fn readme_has_description_placeholder(world: &mut GivnWorld) {
    unimplemented!("assert the README contains the description open placeholder")
}

#[then("the README contains an open placeholder for the license")]
fn readme_has_license_placeholder(world: &mut GivnWorld) {
    unimplemented!("assert the README contains the license open placeholder")
}

#[then("the README contains the development managed block naming the configured workflow commands")]
fn readme_has_dev_block(world: &mut GivnWorld) {
    unimplemented!("assert the scaffolded README contains the development managed block")
}

#[then("the development managed block lists the current configured workflow commands")]
fn dev_block_lists_commands(world: &mut GivnWorld) {
    unimplemented!("assert the refreshed block lists the configured commands")
}

#[then(regex = r#"^the instructions mention the unfinished README and \`givn readme check\`$"#)]
fn instructions_mention_readme(world: &mut GivnWorld) {
    unimplemented!("assert the instructions output surfaces the README need")
}
