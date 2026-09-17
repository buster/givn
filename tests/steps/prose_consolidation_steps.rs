use std::fs;

use cucumber::{given, then, when};

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
    let output = common::run_givn_with_stdin(&env, &["init"], "opencode, .agents, .claude\n");
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
fn generated_grill_command_contains(_world: &mut GivnWorld, _command: String, _text: String) {
    unimplemented!()
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
