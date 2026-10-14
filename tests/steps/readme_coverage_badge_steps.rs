use crate::common;
use crate::steps::GivnWorld;
use cucumber::then;

/// The served coverage guidance for the fixture project.
fn served_coverage_guidance(world: &mut GivnWorld) -> String {
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
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[then("the served coverage guidance places the coverage badge directly after the README title")]
fn badge_after_title(world: &mut GivnWorld) {
    let guidance = served_coverage_guidance(world);
    assert!(
        guidance.contains("Place the two")
            && guidance.contains("generated badges directly after the README"),
        "coverage guidance missing the badge-placement rule"
    );
    assert!(
        guidance.contains("# <project name>")
            && guidance.contains("img.shields.io")
            && guidance.contains("coverage/cobertura-coverage.xml")
            && guidance.contains("Line Coverage")
            && guidance.contains("Branch Coverage"),
        "coverage guidance example must show the badge directly under the title"
    );
}

#[then("the served coverage guidance keeps the totals and reproduction commands")]
fn totals_and_commands(world: &mut GivnWorld) {
    let guidance = served_coverage_guidance(world);
    assert!(
        guidance.contains("## Coverage")
            && guidance.contains("non-E2E")
            && guidance.contains("E2E runs"),
        "coverage guidance example must keep the Coverage section with non-E2E and E2E totals"
    );
    assert!(
        guidance.contains("measure-coverage.sh") && guidance.contains("merge-coverages.sh"),
        "coverage guidance example must keep the reproduction commands"
    );
}
