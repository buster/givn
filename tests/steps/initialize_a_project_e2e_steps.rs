//! E2E step definitions for the `initialize-a-project` capability.
//!
//! E2E bindings drive the real `givn init` / `givn skills sync` interfaces,
//! including the interactive terminal prompt through a real PTY.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

/// Human-authored guidance that must survive initialization/synchronization.
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

/// Slice the managed `givn-project` block out of the fixture's AGENTS.md.
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

// --- non-interactive initialization -----------------------------------------

#[given("the root contains human-authored agent instructions to preserve")]
fn root_contains_human_instructions(world: &mut GivnWorld) {
    let env = fixture(world);
    std::fs::write(env.path("AGENTS.md"), HUMAN_INSTRUCTIONS).expect("write AGENTS.md");
}

#[then("the initialization report names opencode and states that optional addons are disabled")]
fn init_report_names_selection_and_addons(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("skills will be installed to: .opencode")
            && out.contains("disabled (--no-addons)"),
        "the initialization report does not name the selection or the disabled addons: {out}"
    );
}

#[then(
    "the generated project-start guidance requires the effective dialog policy before every context's first user-facing question"
)]
fn project_start_requires_policy(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("follow the effective dialog policy from")
            && block.contains("load it once per context before the first"),
        "managed project-start guidance does not require the policy load: {block}"
    );
}

#[then(
    "the obligation covers questions outside named skills and requires explicit handoff to delegated agents"
)]
fn obligation_covers_outside_skills_and_handoff(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("outside any named")
            && block.contains("pass the obligation explicitly to fresh delegated contexts"),
        "managed obligation does not cover named-skill boundaries or delegation: {block}"
    );
}

#[then(
    "the project-start guidance contains a policy reference rather than the worked dialog examples"
)]
fn project_start_has_reference_not_examples(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        !block.contains("### Example 1 decision record")
            && !block.contains("## Rule 1 — Ask only necessary"),
        "managed project-start guidance embeds the worked dialog policy: {block}"
    );
}

#[then("the human-authored agent instructions remain unchanged")]
fn human_instructions_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert!(
        agents.contains(HUMAN_INSTRUCTIONS),
        "human-authored agent instructions were altered: {agents}"
    );
}

#[then("no optional tone addon is required for the obligation")]
fn no_tone_addon_required(world: &mut GivnWorld) {
    let env = fixture(world);
    let block = managed_project_block(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("config missing");
    let active: String = config
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !block.contains("Absolute Mode") && !active.contains("absolute_mode: true"),
        "the obligation depends on the optional tone addon: {block}"
    );
}

// --- interactive terminal initialization ------------------------------------

#[when("the consumer selects opencode and claude in the real terminal init prompt")]
fn consumer_selects_targets_in_terminal(world: &mut GivnWorld) {
    let env = fixture(world);
    // Option order is .opencode, .agents, .claude with only .opencode
    // premarked: two down keys move to .claude, space marks it, enter confirms.
    world.last_output = Some(common::run_init_in_terminal(env, b"\x1b[B\x1b[B\x20\n"));
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

// --- resynchronization ------------------------------------------------------

#[given(
    "the project contains an older managed startup block beside human-authored agent instructions"
)]
fn project_contains_stale_managed_block(world: &mut GivnWorld) {
    let env = fixture(world);
    let stale = format!(
        "{HUMAN_INSTRUCTIONS}\n<!-- givn:begin:givn-project -->\n{stale_body}\n<!-- givn:end:givn-project -->\n",
        stale_body = "This project uses **givn**. Old block without the dialog obligation."
    );
    std::fs::write(env.path("AGENTS.md"), stale).expect("write stale AGENTS.md");
}

#[then("the synchronization report identifies the selected guidance destinations")]
fn sync_report_identifies_destinations(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains("synced command: /givn-propose")
            && out.contains("Done. Generated:"),
        "the synchronization report does not identify the selected guidance: {out}"
    );
}

#[then(
    "the managed project-start block now requires the effective dialog policy for every user-facing question"
)]
fn managed_block_now_requires_policy(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("every user-facing question")
            && block.contains("effective dialog policy from"),
        "the refreshed managed block does not require the policy: {block}"
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
