use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;
use cucumber::then;

#[given("a givn project with the coverage addon enabled")]
fn coverage_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[then(
    "the served coverage guidance shows a README coverage example with a badge, report link, non-E2E and E2E totals, and reproduction commands"
)]
fn served_guidance_has_example(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let output = common::run_givn(env, &["instructions", "coverage"]);
    assert!(
        output.status.success(),
        "givn instructions coverage failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let guidance = String::from_utf8_lossy(&output.stdout);
    assert!(
        guidance.contains("## Coverage"),
        "coverage guidance missing '## Coverage' section"
    );
    assert!(
        guidance.contains("img.shields.io"),
        "coverage guidance missing coverage badge"
    );
    assert!(
        guidance.contains("coverage/cobertura-coverage.xml"),
        "coverage guidance missing report link"
    );
    assert!(
        guidance.contains("non-E2E") && guidance.contains("E2E runs"),
        "coverage guidance missing non-E2E and E2E totals"
    );
    assert!(
        guidance.contains("measure-coverage.sh") && guidance.contains("merge-coverages.sh"),
        "coverage guidance missing reproduction commands"
    );
}
