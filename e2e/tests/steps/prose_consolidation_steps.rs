use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then};
use std::fs;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn target_skill_paths(world: &GivnWorld, skill: &str) -> Vec<std::path::PathBuf> {
    let targets = crate::harness::skills_targets(&env(world).root())
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
    let selection = crate::harness::AGENT_TARGETS.join(", ");
    let output = common::run_givn_with_stdin(&env, &["init"], &format!("{selection}\n"));
    assert!(output.status.success(), "givn init failed: {:?}", output);
    world.env = Some(env);
}

#[given("a givn project with all agent skill targets enabled and an active change")]
fn project_with_all_targets_and_active_change(world: &mut GivnWorld) {
    project_with_all_targets(world);
    let env = world.env.as_ref().expect("init created the fixture");
    let output = common::run_givn(env, &["new", "add-user-auth"]);
    assert!(
        output.status.success(),
        "givn new add-user-auth failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(
    regex = r"^the generated givn-([a-z-]+) guidance directs the ([a-z]+(?: [a-z]+)?) to the ([a-z-]+) instruction$"
)]
fn generated_guidance_directs_to_instruction(
    world: &mut GivnWorld,
    skill: String,
    _actor: String,
    instruction: String,
) {
    let content = generated_skill_content(world, &skill);
    let needle = format!("givn instructions {instruction}");
    assert!(
        content.contains(&needle),
        "givn-{skill} guidance does not direct to the {instruction} instruction \
         (missing {needle:?}): {content}"
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
