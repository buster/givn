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

#[then("the served specs instruction carries the canonical interaction and E2E policy")]
fn served_specs_carries_canonical_policy(world: &mut GivnWorld) {
    let content =
        crate::steps::serve_extended_rules_steps::served_instruction(fixture(world), "specs");
    for marker in [
        "## User Interaction Inventory",
        "the named real interface",
        "captured transcript",
    ] {
        assert!(
            content.contains(marker),
            "the served specs instruction is missing the canonical interaction/E2E \
             policy marker {marker:?}: {content}"
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

// --- prepare-0-9-10: canonical E2E and thin generated guidance -------------

#[then("the generated givn-spec skill directs the author to the canonical specs instruction")]
fn generated_spec_skill_points_to_canonical_instruction(world: &mut GivnWorld) {
    let targets = givn::config::overlay::load_skills_targets(&fixture(world).root())
        .expect("load configured skills targets");
    for target in targets {
        let rel = format!(".{target}/skills/givn-spec/SKILL.md");
        let content = delivered(&rel, world);
        assert!(
            content.contains("givn instructions specs"),
            "{rel} does not direct the author to the canonical specs instruction: {content}"
        );
    }
}

#[then(
    "managed bootstrap guidance owns loading the effective dialog policy before the first question once per context"
)]
fn managed_bootstrap_owns_dialog_obligation(world: &mut GivnWorld) {
    let agents = delivered("AGENTS.md", world);
    let normalized = agents.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("load it once per context before the first question"),
        "managed bootstrap does not own the once-per-context dialog load: {agents}"
    );
    assert!(
        normalized.contains("givn instructions dialog"),
        "managed bootstrap does not name the effective dialog policy: {agents}"
    );
}

#[then("thin skills contain no repeated dialog-loading or delegation boilerplate")]
fn thin_skills_do_not_repeat_policy(world: &mut GivnWorld) {
    let root = fixture(world).root();
    let targets =
        givn::config::overlay::load_skills_targets(&root).expect("load configured skills targets");
    for target in targets {
        let dir = root.join(format!(".{target}/skills"));
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path().join("SKILL.md");
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            let normalized = content.split_whitespace().collect::<Vec<_>>().join(" ");
            for marker in [
                "givn instructions dialog",
                "load it once per context",
                "presenting agent remains responsible",
                "delegated question preparation",
            ] {
                assert!(
                    !normalized.contains(marker),
                    "{} repeats thin-skill policy {marker:?}",
                    path.display()
                );
            }
        }
    }
}

#[then("detailed dialog examples remain available through the policy's companion index")]
fn dialog_examples_remain_indexed(world: &mut GivnWorld) {
    let agents = delivered("AGENTS.md", world);
    assert!(
        agents.contains("givn instructions dialog-examples"),
        "managed bootstrap does not point to the dialog-examples companion: {agents}"
    );
    let policy = std::fs::read_to_string(crate::common::repo_path("assets/instructions/dialog.md"))
        .expect("read canonical dialog policy");
    assert!(
        policy.contains("Companion examples")
            && policy.contains("givn instructions dialog-examples"),
        "the dialog policy does not index its companion examples"
    );
}

#[then("synchronization reports skills without separately generated commands")]
fn sync_reports_skills_without_commands(world: &mut GivnWorld) {
    let stdout = e2e_stdout(world);
    assert!(
        stdout.contains("synced skill:"),
        "sync output does not report generated skills: {stdout}"
    );
    assert!(
        !stdout.to_lowercase().contains("command"),
        "sync output still reports generated commands: {stdout}"
    );
    let root = fixture(world).root();
    for target in givn::config::overlay::load_skills_targets(&root).unwrap_or_default() {
        let commands = root.join(format!(".{target}/commands"));
        if commands.is_dir() {
            let count = std::fs::read_dir(&commands)
                .map(|entries| entries.flatten().count())
                .unwrap_or(0);
            assert_eq!(
                count,
                0,
                "{} still contains generated command files",
                commands.display()
            );
        }
    }
}

#[then("the served proposal instruction preserves the proposed-by seed stamp")]
fn proposal_preserves_proposed_by_stamp(_world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/proposal.md"))
            .expect("read proposal instruction");
    assert!(
        content.contains("Status: proposed by <change-id>"),
        "proposal instruction lost the proposed-by seed stamp"
    );
    assert!(
        content.contains("Seed stamping"),
        "proposal instruction lost the seed-stamping section"
    );
}

#[then("the served archive instruction preserves the implemented-in seed stamp")]
fn archive_preserves_implemented_in_stamp(_world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/archive.md"))
            .expect("read archive instruction");
    assert!(
        content.contains("Status: implemented in <change-id>"),
        "archive instruction lost the implemented-in seed stamp"
    );
}

#[then("generated propose and archive skills direct authors to those instructions")]
fn propose_and_archive_skills_direct_to_instructions(world: &mut GivnWorld) {
    let root = fixture(world).root();
    for (skill, instruction) in [("propose", "proposal"), ("archive", "archive")] {
        for target in givn::config::overlay::load_skills_targets(&root).unwrap_or_default() {
            let rel = format!(".{target}/skills/givn-{skill}/SKILL.md");
            let content = delivered(&rel, world);
            let needle = format!("givn instructions {instruction}");
            assert!(
                content.contains(&needle),
                "{rel} does not point to {needle}: {content}"
            );
        }
    }
}

#[then("no separately generated propose or archive command is installed")]
fn no_propose_or_archive_command(world: &mut GivnWorld) {
    let root = fixture(world).root();
    for target in givn::config::overlay::load_skills_targets(&root).unwrap_or_default() {
        for command in ["givn-propose", "givn-archive"] {
            let path = root.join(format!(".{target}/commands/{command}.md"));
            assert!(
                !path.exists(),
                "a generated command was installed: {}",
                path.display()
            );
        }
    }
}
