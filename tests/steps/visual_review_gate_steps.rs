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
fn design_declares_terminal_contract(world: &mut GivnWorld) {
    write_visual_contract(world, "terminal", None);
}

#[given("the change's design.md declares no visual design contract")]
fn design_declares_no_contract(world: &mut GivnWorld) {
    let design = "# Design: fixture\n\nNo visual contract in this design.\n";
    std::fs::write(change_dir(world).join("design.md"), design).expect("write design.md");
}

#[given(regex = r##"^the change's design\.md declares the visual interface "([^"]+)"$"##)]
fn design_declares_interface(world: &mut GivnWorld, kind: String) {
    write_visual_contract(world, &kind, None);
}

#[given("the change's design.md declares a web visual contract without viewports")]
fn design_declares_web_without_viewports(world: &mut GivnWorld) {
    write_visual_contract(world, "web", None);
}

#[given(regex = r##"^the change's delta spec contains an @e2e scenario "([^"]+)"$"##)]
fn delta_spec_contains_e2e(world: &mut GivnWorld, title: String) {
    let dir = change_dir(world).join("specs/fixture");
    std::fs::create_dir_all(&dir).expect("create delta specs dir");
    let feature = format!(
        "@givn.delta @fixture-capability\n\nFeature: Fixture capability\n\n  @givn.added @e2e\n  Scenario: {title}\n    Given the fixture universe is ready\n    When the fixture observer looks\n    Then the fixture records a unique observation\n"
    );
    std::fs::write(dir.join("fixture-capability.feature"), feature)
        .expect("write delta feature");
}

fn evidence_path(world: &GivnWorld, title: &str, file: &str) -> PathBuf {
    change_dir(world)
        .join("evidence/visual")
        .join(givn::integrity_hook::scenario_slug(title))
        .join(file)
}

#[given(regex = r##"^the change has a final-state screenshot for "([^"]+)" at viewport "([^"]+)"$"##)]
fn has_final_state_screenshot(world: &mut GivnWorld, title: String, viewport: String) {
    let path = evidence_path(world, &title, &format!("{viewport}.png"));
    std::fs::create_dir_all(path.parent().unwrap()).expect("create evidence dir");
    std::fs::write(&path, b"final-state screenshot bytes").expect("write screenshot");
}

#[given(regex = r##"^the change has an empty screenshot file for "([^"]+)" at viewport "([^"]+)"$"##)]
fn has_empty_screenshot(world: &mut GivnWorld, title: String, viewport: String) {
    let path = evidence_path(world, &title, &format!("{viewport}.png"));
    std::fs::create_dir_all(path.parent().unwrap()).expect("create evidence dir");
    std::fs::write(&path, b"").expect("write empty screenshot");
}

#[given(regex = r##"^the change has a captured transcript for "([^"]+)"$"##)]
fn has_captured_transcript(world: &mut GivnWorld, title: String) {
    let path = evidence_path(world, &title, "transcript.txt");
    std::fs::create_dir_all(path.parent().unwrap()).expect("create evidence dir");
    std::fs::write(&path, b"$ givn export\nok\n").expect("write transcript");
}

#[given(regex = r##"^the change's review\.md contains "([^"]+)"$"##)]
fn review_md_contains(world: &mut GivnWorld, text: String) {
    let review = format!("# Review: fixture\n\nREADME-IMPACT: none\n\nREVIEW: PASS\n\n{text}\n");
    std::fs::write(change_dir(world).join("review.md"), review).expect("write review.md");
}

#[given("the change's review.md contains no visual verdict")]
fn review_md_no_verdict(world: &mut GivnWorld) {
    let review = "# Review: fixture\n\nREADME-IMPACT: none\n\nREVIEW: PASS\n";
    std::fs::write(change_dir(world).join("review.md"), review).expect("write review.md");
}

#[given(regex = r##"^the change's design\.md declares the UI states "([^"]+)"$"##)]
fn design_declares_ui_states(world: &mut GivnWorld, states: String) {
    let path = change_dir(world).join("design.md");
    let mut design = std::fs::read_to_string(&path).expect("read design.md");
    design.push_str(&format!("- States: {states}\n"));
    std::fs::write(&path, design).expect("write design.md");
}

#[given(regex = r##"^the change has final-state screenshots for "([^"]+)" at both viewports$"##)]
fn has_final_state_screenshots_both(_world: &mut GivnWorld, _title: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has state screenshots for "([^"]+)" at both viewports for states "([^"]+)"$"##)]
fn has_state_screenshots_both(_world: &mut GivnWorld, _title: String, _states: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has a state screenshot for "([^"]+)" at viewport "([^"]+)" in state "([^"]+)"$"##)]
fn has_state_screenshot(_world: &mut GivnWorld, _title: String, _viewport: String, _state: String) {
    unimplemented!()
}

#[given(regex = r##"^the change has an empty state screenshot for "([^"]+)" at viewport "([^"]+)" in state "([^"]+)"$"##)]
fn has_empty_state_screenshot(
    _world: &mut GivnWorld,
    _title: String,
    _viewport: String,
    _state: String,
) {
    unimplemented!()
}

#[given(regex = r##"^the change's review\.md declares the persona disposition "(.*)"$"##)]
fn review_md_persona_disposition(_world: &mut GivnWorld, _value: String) {
    unimplemented!()
}

#[given("the change's review.md declares no persona disposition")]
fn review_md_no_persona_disposition(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given(regex = r##"^the change's review\.md also contains "([^"]+)"$"##)]
fn review_md_also_contains(_world: &mut GivnWorld, _text: String) {
    unimplemented!()
}
