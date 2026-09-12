use cucumber::given;

use crate::steps::GivnWorld;

#[given(
    regex = r##"^the change's design\.md declares a visual design contract naming viewports "([^"]+)" and "([^"]+)"$"##
)]
fn design_declares_contract(_world: &mut GivnWorld, _first: String, _second: String) {
    unimplemented!()
}

#[given("the change's design.md declares a visual design contract for the terminal interface")]
fn design_declares_terminal_contract(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("the change's design.md declares no visual design contract")]
fn design_declares_no_contract(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given(regex = r##"^the change's design\.md declares the visual interface "([^"]+)"$"##)]
fn design_declares_interface(_world: &mut GivnWorld, _kind: String) {
    unimplemented!()
}

#[given("the change's design.md declares a web visual contract without viewports")]
fn design_declares_web_without_viewports(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given(regex = r##"^the change's delta spec contains an @e2e scenario "([^"]+)"$"##)]
fn delta_spec_contains_e2e(_world: &mut GivnWorld, _title: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has a final-state screenshot for "([^"]+)" at viewport "([^"]+)"$"##)]
fn has_final_state_screenshot(_world: &mut GivnWorld, _title: String, _viewport: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has an empty screenshot file for "([^"]+)" at viewport "([^"]+)"$"##)]
fn has_empty_screenshot(_world: &mut GivnWorld, _title: String, _viewport: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has a captured transcript for "([^"]+)"$"##)]
fn has_captured_transcript(_world: &mut GivnWorld, _title: String) {
    unimplemented!()
}

#[given(regex = r##"^the change's review\.md contains "([^"]+)"$"##)]
fn review_md_contains(_world: &mut GivnWorld, _text: String) {
    unimplemented!()
}

#[given("the change's review.md contains no visual verdict")]
fn review_md_no_verdict(_world: &mut GivnWorld) {
    unimplemented!()
}
