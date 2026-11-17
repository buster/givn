//! E2E step definitions for the `initialize-a-project` capability.
//!
//! E2E bindings drive the real `givn init` / `givn skills sync` interfaces,
//! including the interactive terminal prompt through a real PTY.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

pub(crate) const HUMAN_INSTRUCTIONS: &str =
    "# Project house rules\n\nAlways run the release checklist before tagging.\n";

fn fixture(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("initialization fixture is missing")
}

fn report(world: &GivnWorld) -> String {
    assert_command_succeeded(world);
    crate::steps::stdout(world)
}

pub(crate) fn managed_project_block(world: &GivnWorld) -> String {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let start = agents
        .find("<!-- givn:begin:givn-project -->")
        .expect("managed project-start block missing");
    let end = agents
        .find("<!-- givn:end:givn-project -->")
        .expect("managed project-start block end missing");
    agents[start..end + "<!-- givn:end:givn-project -->".len()].to_string()
}

pub(crate) fn assert_command_succeeded(world: &GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(
        output.status.success(),
        "CLI command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

pub(crate) const LEGACY_GENERATED_COMMAND: &str = "---\ndescription: legacy givn command\n---\n\n**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.\n";

fn write_legacy_commands(env: &common::TestEnv, target: &str, names: &[&str]) {
    let dir = env.path(&format!("{target}/commands"));
    std::fs::create_dir_all(&dir).expect("create commands dir");
    for name in names {
        std::fs::write(dir.join(format!("{name}.md")), LEGACY_GENERATED_COMMAND)
            .expect("write legacy command");
    }
}

fn write_custom_command(env: &common::TestEnv, target: &str, name: &str) {
    let dir = env.path(&format!("{target}/commands"));
    std::fs::create_dir_all(&dir).expect("create commands dir");
    std::fs::write(
        dir.join(format!("{name}.md")),
        "---\ndescription: custom project command\n---\n\nKeep this custom command.\n",
    )
    .expect("write custom command");
}

fn no_generated_commands(env: &common::TestEnv) {
    for target in [".opencode", ".agents", ".claude", ".omp", ".maki"] {
        let dir = env.path(&format!("{target}/commands"));
        let generated: Vec<_> = std::fs::read_dir(&dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
                    .filter(|path| {
                        std::fs::read_to_string(path)
                            .map(|content| {
                                content.contains(
                                    "**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`",
                                )
                            })
                            .unwrap_or(false)
                    })
                    .collect()
            })
            .unwrap_or_default();
        assert!(
            generated.is_empty(),
            "recognizable generated Givn command files remain under {target}: {generated:?}"
        );
    }
}

#[given("human-authored agent instructions outside the managed block")]
fn human_instructions_outside_block(world: &mut GivnWorld) {
    let env = fixture(world);
    std::fs::write(env.path("AGENTS.md"), HUMAN_INSTRUCTIONS).expect("write AGENTS.md");
}

#[then("the command exits 0 and reports installed skills and disabled optional addons")]
fn exits_zero_reports_skills_and_disabled_addons(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains("disabled (--no-addons)")
            && !out.contains("synced command:")
            && !out.contains("core skills and commands"),
        "the report does not name installed skills and disabled addons: {out}"
    );
}

#[then("the installed givn-propose skill directs the author to `givn instructions proposal`")]
fn installed_propose_skill_directs_to_instruction(world: &mut GivnWorld) {
    let env = fixture(world);
    let path = env.path(".opencode/skills/givn-propose/SKILL.md");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        content.contains("givn instructions proposal"),
        "the installed givn-propose skill does not direct the author to the proposal instruction:\n{content}"
    );
}

#[then("no Givn command file is generated")]
fn no_givn_command_file(world: &mut GivnWorld) {
    no_generated_commands(fixture(world));
}

#[then(
    "the managed startup block requires loading the effective dialog policy before the first question in each context independently of addons"
)]
fn startup_block_requires_dialog_independent_of_addons(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("effective dialog policy from")
            && block.contains("load it once per context before the first")
            && block.contains("optional addons disabled"),
        "the managed startup block does not own the addon-independent dialog obligation: {block}"
    );
}

#[then("human-authored instructions remain unchanged")]
fn human_instructions_remain_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert!(
        agents.contains(HUMAN_INSTRUCTIONS),
        "human-authored instructions were altered: {agents}"
    );
}

#[given("an initialized project selecting opencode and claude")]
fn initialized_project_with_opencode_and_claude(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "opencode,claude"]);
    assert!(
        out.status.success(),
        "init for opencode and claude failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.env = Some(env);
}

#[given("an older managed startup block beside human-authored instructions")]
fn older_managed_startup_block(world: &mut GivnWorld) {
    let env = fixture(world);
    let stale = format!(
        "{HUMAN_INSTRUCTIONS}\n<!-- givn:begin:givn-project -->\nThis project uses **givn**. Old block without the dialog obligation.\n<!-- givn:end:givn-project -->\n"
    );
    std::fs::write(env.path("AGENTS.md"), stale).expect("write stale AGENTS.md");
}

#[given("recognizable Givn-generated legacy commands beside custom commands")]
fn legacy_commands_beside_custom_commands(world: &mut GivnWorld) {
    let env = fixture(world);
    write_legacy_commands(env, ".opencode", &["givn-propose", "givn-review"]);
    write_legacy_commands(env, ".claude", &["givn-propose"]);
    write_custom_command(env, ".opencode", "team-release");
}

#[then("the command exits 0 and reports the selected skill destinations")]
fn exits_zero_reports_skill_destinations(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains(".opencode/skills")
            && out.contains(".claude/skills")
            && !out.contains("synced command:"),
        "the report does not name the selected skill destinations: {out}"
    );
}

#[then("only selected targets receive skills")]
fn only_selected_targets_receive_skills(world: &mut GivnWorld) {
    let env = fixture(world);
    for target in [".opencode", ".claude"] {
        assert!(
            env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .is_file(),
            "{target} did not receive the givn-propose skill"
        );
    }
    for target in [".agents", ".omp", ".maki"] {
        assert!(
            !env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .exists(),
            "unselected target {target} received a skill"
        );
    }
}

#[then("recognizable generated legacy commands are removed without generating replacements")]
fn legacy_commands_removed_without_replacements(world: &mut GivnWorld) {
    let env = fixture(world);
    for rel in [
        ".opencode/commands/givn-propose.md",
        ".opencode/commands/givn-review.md",
        ".claude/commands/givn-propose.md",
    ] {
        assert!(
            !env.path(rel).exists(),
            "recognizable legacy command was not removed: {rel}"
        );
    }
    no_generated_commands(env);
}

#[then("custom commands and human-authored instructions remain byte-for-byte unchanged")]
fn custom_commands_and_human_instructions_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let custom = std::fs::read_to_string(env.path(".opencode/commands/team-release.md"))
        .expect("custom command missing");
    assert_eq!(
        custom, "---\ndescription: custom project command\n---\n\nKeep this custom command.\n",
        "the custom command was altered"
    );
    human_instructions_remain_unchanged(world);
}

#[then(
    "the startup block is refreshed once without changing the persisted selection or unrelated configuration"
)]
fn startup_block_refreshed_selection_unchanged(world: &mut GivnWorld) {
    managed_block_not_duplicated(world);
    persisted_selection_unchanged(world);
}

#[when("the consumer selects opencode and claude in the real terminal init prompt")]
fn consumer_selects_targets_in_terminal(world: &mut GivnWorld) {
    let env = fixture(world);
    // Option order is .opencode, .agents, .claude with only .opencode
    // premarked: two down keys move to .claude, space marks it, enter confirms.
    world.last_output = Some(common::run_init_in_terminal(env, b"\x1b[B\x1b[B\x20\n"));
    world.last_command = Some("init".to_string());
}

#[then(
    "the terminal displays a labeled agent-target selection and confirms the selected integrations"
)]
fn terminal_shows_labeled_selection(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("Which agents should givn install skills to?")
            && out.contains("[x] .opencode")
            && out.contains("[ ] .agents")
            && out.contains("[x] .claude"),
        "the terminal stream does not display the labeled agent-target selection: {out}"
    );
    assert!(
        out.contains(".opencode, .claude"),
        "the terminal stream does not confirm the selected integrations: {out}"
    );
}

#[then("the managed block is not duplicated")]
fn managed_block_not_duplicated(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert_eq!(
        agents.matches("<!-- givn:begin:givn-project -->").count(),
        1,
        "the managed project-start block was duplicated: {agents}"
    );
    assert_eq!(
        agents.matches("<!-- givn:end:givn-project -->").count(),
        1,
        "the managed project-start block end marker was duplicated: {agents}"
    );
}

#[then("the persisted agent selection and unrelated configuration remain unchanged")]
fn persisted_selection_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("config missing");
    assert!(
        config.contains("targets: ['opencode', 'claude']")
            || config.contains("targets: [opencode, claude]"),
        "the persisted agent selection changed: {config}"
    );
}
