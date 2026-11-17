//! E2E step definitions for the `prose-consolidation` capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess and inspect
//! the delivered consumer documents.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn fixture(world: &GivnWorld) -> &crate::harness::TestEnv {
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
    let env = crate::harness::make_bare_root();
    let selection = crate::harness::AGENT_TARGETS.join(",");
    let output = crate::harness::run_givn(&env, &["init", "--agents", selection.as_str()]);
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
    world.last_output = Some(crate::harness::run_givn(env, &["skills", "sync"]));
}

#[then("the generated givn-spec skill directs the author to the canonical specs instruction")]
fn generated_spec_skill_points_to_canonical_instruction(world: &mut GivnWorld) {
    let targets = crate::harness::skills_targets(&fixture(world).root())
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
    let targets = crate::harness::skills_targets(&root).expect("load configured skills targets");
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
    let policy =
        std::fs::read_to_string(crate::harness::repo_path("assets/instructions/dialog.md"))
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
    for target in crate::harness::skills_targets(&root).unwrap_or_default() {
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
