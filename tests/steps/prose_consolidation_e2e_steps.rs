//! E2E step definitions for the `prose-consolidation` capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess and inspect
//! the delivered consumer documents.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

/// Every skill family the bundled defaults ship (13 core + 2 optional).
pub(crate) const ALL_SKILL_FAMILIES: [&str; 15] = [
    "givn-propose",
    "givn-spec",
    "givn-design",
    "givn-tasks",
    "givn-implement",
    "givn-review",
    "givn-archive",
    "givn-explore",
    "givn-design-review",
    "givn-steps",
    "givn-reverse-engineer",
    "givn-characterize",
    "givn-ideation",
    "givn-domain-modeling",
    "givn-dev-principles",
];

/// Every bundled command (13 core + the 2 optional grill commands).
pub(crate) const ALL_COMMANDS: [&str; 15] = [
    "givn-propose",
    "givn-spec",
    "givn-design",
    "givn-tasks",
    "givn-implement",
    "givn-review",
    "givn-archive",
    "givn-explore",
    "givn-design-review",
    "givn-steps",
    "givn-reverse-engineer",
    "givn-characterize",
    "givn-ideation",
    "grill-me",
    "grill-with-docs",
];

const OBLIGATION_SCOPE: &str = "for every user-facing question";
const OBLIGATION_POLICY: &str = "effective policy from `givn instructions dialog`";

fn fixture(world: &GivnWorld) -> &crate::common::TestEnv {
    world.env.as_ref().expect("e2e fixture is missing")
}

fn delivered(rel: &str, world: &GivnWorld) -> String {
    std::fs::read_to_string(fixture(world).path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn e2e_stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(
        output.status.success(),
        "CLI command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[given("a Givn project with all agent skill targets and optional guidance enabled")]
fn project_all_targets_with_optional_guidance(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let selection = givn::skills::targets::AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(",");
    let output = crate::common::run_givn(&env, &["init", "--agents", selection.as_str()]);
    assert!(
        output.status.success(),
        "init with all targets and addons failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[when("the consumer synchronizes the selected agent guidance through the Givn CLI")]
fn consumer_synchronizes_agent_guidance(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["skills", "sync"]));
}

#[then(
    "every emitted skill and command requires the effective dialog policy before its context asks a user-facing question"
)]
fn emitted_entries_require_policy(world: &mut GivnWorld) {
    let targets = givn::config::overlay::load_skills_targets(&fixture(world).root())
        .expect("load configured skills targets");
    for skill in ALL_SKILL_FAMILIES {
        for target in &targets {
            let rel = format!(".{target}/skills/{skill}/SKILL.md");
            let content = delivered(&rel, world);
            assert!(
                content.contains(OBLIGATION_SCOPE) && content.contains(OBLIGATION_POLICY),
                "{rel} does not require the effective dialog policy: {content}"
            );
        }
    }
    for command in ALL_COMMANDS {
        let rel = format!(".opencode/commands/{command}.md");
        let content = delivered(&rel, world);
        assert!(
            content.contains(OBLIGATION_SCOPE) && content.contains(OBLIGATION_POLICY),
            "{rel} does not require the effective dialog policy: {content}"
        );
    }
}

#[then("direct skill invocation has the same obligation as session-start invocation")]
fn direct_skill_invocation_shares_obligation(world: &mut GivnWorld) {
    let content = delivered(".opencode/skills/givn-review/SKILL.md", world);
    assert!(
        content.contains(OBLIGATION_POLICY)
            && content.contains("load it before this context's first question"),
        "a directly invoked skill does not carry the session-start obligation: {content}"
    );
}

#[then("reference skills do not narrow the obligation to one workflow phase")]
fn reference_skills_keep_obligation(world: &mut GivnWorld) {
    for skill in ["givn-steps", "givn-dev-principles", "givn-domain-modeling"] {
        let rel = format!(".opencode/skills/{skill}/SKILL.md");
        let content = delivered(&rel, world);
        assert!(
            content.contains(OBLIGATION_SCOPE) && content.contains(OBLIGATION_POLICY),
            "{rel} narrows the obligation to one workflow phase: {content}"
        );
    }
}

#[then(
    "the generated guidance preserves its references to the canonical interaction and E2E policy"
)]
fn guidance_preserves_canonical_references(world: &mut GivnWorld) {
    for rel in [
        ".opencode/skills/givn-spec/SKILL.md",
        ".opencode/skills/givn-design/SKILL.md",
        ".opencode/skills/givn-tasks/SKILL.md",
        ".opencode/commands/givn-spec.md",
    ] {
        let content = delivered(rel, world);
        assert!(
            content.contains("givn instructions specs"),
            "{rel} lost its canonical specs/E2E policy reference: {content}"
        );
    }
}

#[then(
    "detailed dialog rules and worked examples remain in the effective dialog instructions rather than competing generated copies"
)]
fn generated_copies_absent(world: &mut GivnWorld) {
    for skill in ALL_SKILL_FAMILIES {
        let rel = format!(".opencode/skills/{skill}/SKILL.md");
        let content = delivered(&rel, world);
        assert!(
            !content.contains("### Example 1 decision record")
                && !content.contains("## Rule 1 — Ask only necessary"),
            "{rel} embeds a competing dialog policy copy: {content}"
        );
    }
}

#[then("the synchronization result reports the selected guidance successfully")]
fn sync_reports_success(world: &mut GivnWorld) {
    let output = e2e_stdout(world);
    assert!(
        output.contains("synced skill: givn-propose")
            && output.contains("synced command: /givn-propose")
            && output.contains("Done. Generated:")
            && output.contains(".opencode/skills/")
            && output.contains(".claude/skills/")
            && output.contains(".opencode/commands/"),
        "the synchronization report does not identify the selected guidance: {output}"
    );
}
