use crate::steps::GivnWorld;
use cucumber::then;

#[then("the archive guidance places the coverage badge directly after the README title")]
fn badge_after_title(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let skill = std::fs::read_to_string(env.path(".opencode/skills/givn-archive/SKILL.md"))
        .expect("read generated archive skill");
    assert!(
        skill.contains("Place the coverage badge directly after the README"),
        "archive skill missing the badge-placement rule"
    );
    assert!(
        skill.contains("# <project name>")
            && skill.contains("img.shields.io")
            && skill.contains("coverage/non-e2e-cobertura.xml"),
        "archive skill example must show the badge directly under the title"
    );
}

#[then("the archive guidance keeps the totals and reproduction commands")]
fn totals_and_commands(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let skill = std::fs::read_to_string(env.path(".opencode/skills/givn-archive/SKILL.md"))
        .expect("read generated archive skill");
    assert!(
        skill.contains("## Coverage") && skill.contains("non-E2E") && skill.contains("E2E subset"),
        "archive skill example must keep the Coverage section with non-E2E and E2E totals"
    );
    assert!(
        skill.contains("cargo llvm-cov"),
        "archive skill example must keep the reproduction commands"
    );
}
