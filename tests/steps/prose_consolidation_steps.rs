use std::fs;

use cucumber::{given, then, when};
use givn::skills::targets::AgentTarget;

// --- decision-ready questions: delegated question preparation ---------------

pub(crate) const DELEGATING_COMMANDS: [&str; 6] = [
    "givn-propose",
    "givn-design-review",
    "givn-implement",
    "givn-review",
    "givn-characterize",
    "givn-reverse-engineer",
];

pub(crate) const DELEGATING_SKILLS: [&str; 6] = [
    "givn-implement",
    "givn-design-review",
    "givn-review",
    "givn-characterize",
    "givn-ideation",
    "givn-reverse-engineer",
];

const DELEGATION_INHERITANCE: &str = "does not inherit the parent's loaded policy";
const DELEGATION_FRESH_LOAD: &str = "load it in the fresh context before the first question";
const DELEGATION_RESPONSIBILITY: &str = "presenting agent remains responsible for the question";
const DELEGATION_TOOL_LIMIT: &str = "question tool's recommendation or option label does not replace context and answer consequences";
const DELEGATION_POLICY: &str = "effective policy from `givn instructions dialog`";

/// Slice every fenced prompt block that opens with a self-contained
/// instruction ("You are a ...") out of a delivered command document.
pub(crate) fn delegated_prompt_blocks(content: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<Vec<&str>> = None;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            match current.take() {
                Some(block) => {
                    if block
                        .iter()
                        .any(|l| l.trim_start().starts_with("You are a"))
                    {
                        blocks.push(block.join("\n"));
                    }
                }
                None => current = Some(Vec::new()),
            }
            continue;
        }
        if let Some(block) = current.as_mut() {
            block.push(line);
        }
    }
    if let Some(block) = current
        && block
            .iter()
            .any(|l| l.trim_start().starts_with("You are a"))
    {
        blocks.push(block.join("\n"));
    }
    blocks
}

fn assert_delegation_obligation(where_: &str, text: &str) {
    for marker in [
        DELEGATION_POLICY,
        DELEGATION_INHERITANCE,
        DELEGATION_FRESH_LOAD,
        DELEGATION_RESPONSIBILITY,
        DELEGATION_TOOL_LIMIT,
    ] {
        assert!(
            text.contains(marker),
            "{where_} is missing the delegated-dialog obligation marker {marker:?}: {text}"
        );
    }
}

// --- decision-ready questions: project-owned replacements -------------------

pub(crate) const REPLACEMENT_SKILL: &str = "---\nname: givn-propose\ndescription: Project-owned propose skill.\n---\n\n# givn-propose\n\nProject-owned propose guidance.\n";

pub(crate) const REPLACEMENT_COMMAND: &str =
    "---\ndescription: Project-owned propose command\n---\n\nProject-owned propose command body.\n";

const MANAGED_POLICY_REFERENCE: &str = "follow the effective dialog policy from";
const MANAGED_HANDOFF_REFERENCE: &str =
    "pass the obligation explicitly to fresh delegated contexts";

#[given("a Givn project with all agent skill targets selected")]
fn project_with_all_skill_targets(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let selection = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(",");
    let output = crate::common::run_givn(&env, &["init", "--agents", selection.as_str()]);
    assert!(
        output.status.success(),
        "init with all targets failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[given(
    "valid project-owned skill and command replacements intentionally omit the bundled dialog obligation"
)]
fn project_replacements_omit_obligation(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let skill_path = env.path("givn/skills/givn-propose/SKILL.md.tmpl");
    std::fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("create skills dir");
    std::fs::write(&skill_path, REPLACEMENT_SKILL).expect("write replacement skill");
    let command_path = env.path("givn/commands/givn-propose.md");
    std::fs::create_dir_all(command_path.parent().expect("command parent"))
        .expect("create commands dir");
    std::fs::write(&command_path, REPLACEMENT_COMMAND).expect("write replacement command");
    assert!(
        !REPLACEMENT_SKILL.contains("givn instructions dialog")
            && !REPLACEMENT_COMMAND.contains("givn instructions dialog"),
        "the replacement fixture must intentionally omit the bundled obligation"
    );
}

#[then("the project-owned skill and command source bytes remain unchanged")]
fn replacement_sources_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let skill = std::fs::read_to_string(env.path("givn/skills/givn-propose/SKILL.md.tmpl"))
        .expect("replacement skill missing");
    assert_eq!(
        skill, REPLACEMENT_SKILL,
        "the project-owned skill template bytes changed"
    );
    let command = std::fs::read_to_string(env.path("givn/commands/givn-propose.md"))
        .expect("replacement command missing");
    assert_eq!(
        command, REPLACEMENT_COMMAND,
        "the project-owned command template bytes changed"
    );
}

#[then(
    "every selected skill consumer receives the authoritative rendered replacement without a bundled prefix"
)]
fn skill_consumers_receive_replacement(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let targets = givn::config::overlay::load_skills_targets(&env.root())
        .expect("load configured skills targets");
    for target in targets {
        let rel = format!(".{target}/skills/givn-propose/SKILL.md");
        let delivered =
            std::fs::read_to_string(env.path(&rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert_eq!(
            delivered, REPLACEMENT_SKILL,
            "{rel} is not the authoritative replacement (a bundled prefix or suffix was injected)"
        );
    }
}

#[then(
    "the opencode command consumer receives the authoritative rendered replacement without a bundled prefix"
)]
fn command_consumer_receives_replacement(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let delivered = std::fs::read_to_string(env.path(".opencode/commands/givn-propose.md"))
        .expect("delivered command missing");
    assert_eq!(
        delivered, REPLACEMENT_COMMAND,
        "the delivered command is not the authoritative replacement"
    );
}

#[then("the managed project-start guidance still references the effective dialog policy")]
fn managed_guidance_references_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let start = agents
        .find("<!-- givn:begin:givn-project -->")
        .expect("managed block start missing");
    let end = agents
        .find("<!-- givn:end:givn-project -->")
        .expect("managed block end missing");
    let block = &agents[start..end];
    assert!(
        block.contains(MANAGED_POLICY_REFERENCE) && block.contains(MANAGED_HANDOFF_REFERENCE),
        "managed project-start guidance lost the effective-policy reference: {block}"
    );
}

#[given("a Givn project with delegated authoring and review guidance enabled")]
fn project_with_delegated_guidance(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let output = crate::common::run_givn(&env, &["init", "--agents", "opencode,agents,claude"]);
    assert!(
        output.status.success(),
        "init for delegated guidance failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[when("the author reads the synchronized delegation instructions")]
fn author_reads_delegation_instructions(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["skills", "sync"]));
    assert!(
        world
            .last_output
            .as_ref()
            .expect("sync ran")
            .status
            .success(),
        "skills sync failed"
    );
}

#[then(
    "every question-preparing delegation explicitly requires the effective dialog policy in the fresh context"
)]
fn delegations_require_policy_in_fresh_context(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    for command in DELEGATING_COMMANDS {
        let rel = format!(".opencode/commands/{command}.md");
        let content = std::fs::read_to_string(env.path(&rel))
            .unwrap_or_else(|e| panic!("{rel} missing: {e}"));
        let blocks = delegated_prompt_blocks(&content);
        assert!(
            !blocks.is_empty(),
            "{rel} carries no self-contained delegated prompt"
        );
        for (index, block) in blocks.iter().enumerate() {
            assert_delegation_obligation(&format!("{rel} prompt {}", index + 1), block);
        }
    }
    for skill in DELEGATING_SKILLS {
        let rel = format!(".opencode/skills/{skill}/SKILL.md");
        let content = std::fs::read_to_string(env.path(&rel))
            .unwrap_or_else(|e| panic!("{rel} missing: {e}"));
        assert_delegation_obligation(&rel, &content);
    }
}

#[then("the delegation instructions do not assume the parent's loaded policy is inherited")]
fn delegations_do_not_assume_inheritance(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    for command in DELEGATING_COMMANDS {
        let content =
            std::fs::read_to_string(env.path(&format!(".opencode/commands/{command}.md")))
                .expect("command missing");
        for block in delegated_prompt_blocks(&content) {
            assert!(
                block.contains(DELEGATION_INHERITANCE),
                "{command} prompt assumes inherited policy: {block}"
            );
        }
    }
}

#[then("the presenting agent remains responsible for the question before it reaches the person")]
fn presenting_agent_remains_responsible(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    for skill in DELEGATING_SKILLS {
        let content =
            std::fs::read_to_string(env.path(&format!(".opencode/skills/{skill}/SKILL.md")))
                .expect("skill missing");
        assert!(
            content.contains(DELEGATION_RESPONSIBILITY),
            "{skill} does not keep the presenting agent responsible: {content}"
        );
    }
}

#[then(
    "a question-tool recommendation or option label does not replace context and answer consequences"
)]
fn question_tools_do_not_replace_context(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    let content = std::fs::read_to_string(env.path(".opencode/commands/givn-design-review.md"))
        .expect("design-review command missing");
    for block in delegated_prompt_blocks(&content) {
        assert!(
            block.contains(DELEGATION_TOOL_LIMIT),
            "design-review prompt lets a question tool replace context: {block}"
        );
    }
}

use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn target_skill_paths(world: &GivnWorld, skill: &str) -> Vec<std::path::PathBuf> {
    let targets = givn::config::overlay::load_skills_targets(&env(world).root())
        .unwrap_or_else(|error| panic!("load skills targets from givn/config.yaml: {error}"));
    targets
        .into_iter()
        .map(|target| env(world).path(&format!(".{target}/skills/givn-{skill}/SKILL.md")))
        .collect()
}

fn generated_skill_content(world: &GivnWorld, skill: &str) -> String {
    target_skill_paths(world, skill)
        .into_iter()
        .map(|path| {
            fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read generated skill {}: {error}", path.display()))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[given("a givn project with all agent skill targets enabled")]
fn project_with_all_targets(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let selection = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(", ");
    let output = common::run_givn_with_stdin(&env, &["init"], &format!("{selection}\n"));
    assert!(output.status.success(), "givn init failed: {:?}", output);
    world.env = Some(env);
}

#[given(regex = r##"^a project override for specs instructions containing "([^"]+)"$"##)]
fn specs_instruction_override(world: &mut GivnWorld, text: String) {
    common::write_override(env(world), "specs", "instructions.md", &text);
}

#[when("the user reads the design and review instructions for prose-consolidation")]
fn read_design_and_review_instructions(world: &mut GivnWorld) {
    let design = common::run_givn(
        env(world),
        &["instructions", "design", "--change", "prose-consolidation"],
    );
    let review = common::run_givn(
        env(world),
        &["instructions", "review", "--change", "prose-consolidation"],
    );
    assert!(
        design.status.success(),
        "design instructions failed: {:?}",
        design
    );
    assert!(
        review.status.success(),
        "review instructions failed: {:?}",
        review
    );
    let mut combined = String::from_utf8_lossy(&design.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&review.stdout));
    world.additional_output = Some(combined);
}

#[then(regex = r##"^combined instruction output contains "([^"]+)"$"##)]
fn combined_output_contains(world: &mut GivnWorld, text: String) {
    let output = world
        .additional_output
        .as_ref()
        .expect("combined instruction output was not captured");
    assert!(
        output.contains(&text),
        "combined output missing {text:?}: {output}"
    );
}

#[then(regex = r##"^combined instruction output does not contain "([^"]+)"$"##)]
fn combined_output_does_not_contain(world: &mut GivnWorld, text: String) {
    let output = world
        .additional_output
        .as_ref()
        .expect("combined instruction output was not captured");
    assert!(
        !output.contains(&text),
        "combined output contains forbidden {text:?}: {output}"
    );
}

#[then(regex = r##"^the generated givn-([a-z-]+) guidance contains "([^"]+)"$"##)]
fn generated_guidance_contains(world: &mut GivnWorld, skill: String, text: String) {
    let content = generated_skill_content(world, &skill);
    assert!(
        content.contains(&text),
        "givn-{skill} guidance missing {text:?}: {content}"
    );
}

#[then(regex = r##"^the generated givn-([a-z-]+) command contains "([^"]+)"$"##)]
fn generated_command_contains(world: &mut GivnWorld, command: String, text: String) {
    let path = env(world).path(&format!(".opencode/commands/givn-{command}.md"));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read generated command {}: {error}", path.display()));
    assert!(
        content.contains(&text),
        "command missing {text:?}: {content}"
    );
}

#[then(regex = r##"^the generated grill-([a-z-]+) command contains "([^"]+)"$"##)]
fn generated_grill_command_contains(world: &mut GivnWorld, command: String, text: String) {
    let path = env(world).path(&format!(".opencode/commands/grill-{command}.md"));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read generated command {}: {error}", path.display()));
    assert!(
        content.contains(&text),
        "command missing {text:?}: {content}"
    );
}

#[then(regex = r##"^generated guidance does not contain "([^"]+)"$"##)]
fn generated_guidance_does_not_contain(world: &mut GivnWorld, text: String) {
    let mut content = String::new();
    for skill in [
        "spec",
        "design",
        "review",
        "steps",
        "tasks",
        "implement",
        "design-review",
    ] {
        content.push_str(&generated_skill_content(world, skill));
    }
    let command_dir = env(world).path(".opencode/commands");
    if let Ok(entries) = fs::read_dir(command_dir) {
        for entry in entries.flatten() {
            if let Ok(value) = fs::read_to_string(entry.path()) {
                content.push_str(&value);
            }
        }
    }
    assert!(
        !content.contains(&text),
        "generated guidance contains forbidden {text:?}"
    );
}
