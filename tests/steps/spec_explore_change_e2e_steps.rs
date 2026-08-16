//! E2E black-box steps for current-change scenario exploration.

use cucumber::{given, when};

use super::GivnWorld;

#[given(
    "an end-to-end project prepared for current-change exploration with an active added scenario"
)]
fn e2e_added_scenario_project(_world: &mut GivnWorld) {
    unimplemented!()
}

#[when(regex = r#"^the end-to-end user explores "([^"]+)" against change "([^"]+)"$"#)]
fn e2e_explore_against_change(_world: &mut GivnWorld, _path: String, _change: String) {
    unimplemented!()
}
