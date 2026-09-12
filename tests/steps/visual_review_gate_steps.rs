use cucumber::given;
use std::path::PathBuf;

use crate::steps::GivnWorld;

fn project_root(world: &GivnWorld) -> PathBuf {
    world
        .env
        .as_ref()
        .map(|env| env.path(""))
        .expect("a project root is required first")
}

fn change_dir(world: &GivnWorld) -> PathBuf {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    project_root(world).join(format!("givn/changes/{id}"))
}

fn write_visual_contract(world: &GivnWorld, interface: &str, viewports: Option<&str>) {
    let mut contract = format!("# Design: fixture\n\n## Visual Design Contract\n\n- Interface: {interface}\n");
    if let Some(viewports) = viewports {
        contract.push_str(&format!("- Viewports: {viewports}\n"));
    }
    contract.push_str("- Design system: docs/design/design-system.md\n\nFixture contract.\n");
    std::fs::write(change_dir(world).join("design.md"), contract).expect("write design.md");
}

#[given(
    regex = r##"^the change's design\.md declares a visual design contract naming viewports "([^"]+)" and "([^"]+)"$"##
)]
fn design_declares_contract(world: &mut GivnWorld, first: String, second: String) {
    write_visual_contract(
        world,
        "web",
        Some(&format!("{first} (360x780), {second} (1920x1080)")),
    );
}

#[given("the change's design.md declares a visual design contract for the terminal interface")]
fn design_declares_terminal_contract(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given("the change's design.md declares no visual design contract")]
fn design_declares_no_contract(world: &mut GivnWorld) {
    let design = "# Design: fixture\n\nNo visual contract in this design.\n";
    std::fs::write(change_dir(world).join("design.md"), design).expect("write design.md");
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
fn review_md_no_verdict(world: &mut GivnWorld) {
    let review = "# Review: fixture\n\nREADME-IMPACT: none\n\nREVIEW: PASS\n";
    std::fs::write(change_dir(world).join("review.md"), review).expect("write review.md");
}
