use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;
use cucumber::then;

#[given("a givn project with the coverage addon enabled")]
fn coverage_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[then(
    "the archive guidance shows a README coverage example with a badge, report link, non-E2E and E2E totals, and reproduction commands"
)]
fn archive_guidance_has_example(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let archive_skill = std::fs::read_to_string(env.path(".opencode/skills/givn-archive/SKILL.md"))
        .expect("read generated archive skill");
    assert!(
        archive_skill.contains("## Coverage"),
        "archive skill missing '## Coverage' section"
    );
    assert!(
        archive_skill.contains("img.shields.io"),
        "archive skill missing coverage badge"
    );
    assert!(
        archive_skill.contains("coverage/cobertura-coverage.xml"),
        "archive skill missing report link"
    );
    assert!(
        archive_skill.contains("non-E2E") && archive_skill.contains("E2E runs"),
        "archive skill missing non-E2E and E2E totals"
    );
    assert!(
        archive_skill.contains("measure-coverage.sh")
            && archive_skill.contains("merge-coverages.sh"),
        "archive skill missing reproduction commands"
    );
}
