//! Gherkin step definitions for the `init-agent-target-selection` capability.
//!
//! Covers: the `givn init` agent-target prompt (interactive answer via
//! piped stdin), persisting the selection in `givn/config.yaml`, and the
//! `givn skills sync` target selection.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn expected_targets(list: &str) -> Vec<String> {
    list.split(',')
        .map(|s| s.trim().trim_start_matches('.').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn write_skills_targets(world: &mut GivnWorld, targets: &[String]) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let list = targets
        .iter()
        .map(|t| format!("'{}'", t))
        .collect::<Vec<_>>()
        .join(", ");
    let block = format!("skills:\n  targets: [{list}]\n");

    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    // Remove any existing skills: block (top-level key).
    let mut out: Vec<String> = Vec::new();
    let mut skipping = false;
    for line in lines.drain(..) {
        if skipping {
            if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                skipping = false;
            } else {
                continue;
            }
        }
        if line.trim() == "skills:" {
            skipping = true;
            continue;
        }
        out.push(line);
    }
    if !out.last().map(|l| l.is_empty()).unwrap_or(true) {
        out.push(String::new());
    }
    for line in block.lines() {
        out.push(line.to_string());
    }
    std::fs::write(&path, out.join("\n") + "\n").expect("write givn/config.yaml");
}

fn read_skills_targets(world: &GivnWorld) -> Vec<String> {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content =
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    let mut in_skills = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "skills:" {
            in_skills = true;
            continue;
        }
        if in_skills {
            if !line.is_empty() && !line.starts_with(' ') {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("targets:") {
                return rest
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    vec![]
}

#[given(regex = r#"^givn skills targets in givn/config.yaml equal "([^"]+)"$"#)]
fn set_skills_targets(world: &mut GivnWorld, list: String) {
    write_skills_targets(world, &expected_targets(&list));
}

#[then(regex = r#"^givn skills targets in givn/config.yaml equal "([^"]+)"$"#)]
fn assert_skills_targets(world: &mut GivnWorld, list: String) {
    let actual = read_skills_targets(world);
    let expected = expected_targets(&list);
    assert_eq!(actual, expected, "givn/config.yaml skills targets mismatch");
}

#[when(regex = r#"^the consumer runs `([^`]+)`$"#)]
fn consumer_runs(world: &mut GivnWorld, cmd: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let argv = crate::steps::parse_command(&cmd);
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let output = if argv.first().is_some_and(|arg| arg == "commit") {
        consumer_commit(env, world, &args)
    } else {
        crate::harness::run_givn(env, &args)
    };
    world.last_output = Some(output);
}

fn consumer_commit(
    env: &crate::harness::TestEnv,
    world: &GivnWorld,
    args: &[&str],
) -> std::process::Output {
    let mut command = std::process::Command::new(crate::harness::binary::givn());
    command
        .args(args)
        .current_dir(env.root())
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS")
        .env_remove("GIT_COMMITTER_NAME")
        .env("FOSSIL_USER", "fixture");
    if let Some(context) = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.context.as_ref())
    {
        command.env("GIVN_ATTRIBUTION_ACTIVE", "1");
        if let Some(value) = &context.llm {
            command.env("GIVN_ATTRIBUTION_LLM", value);
        }
        if let Some(value) = &context.reasoning {
            command.env("GIVN_ATTRIBUTION_REASONING", value);
        }
        if let Some(value) = &context.harness {
            command.env("GIVN_ATTRIBUTION_HARNESS", value);
        }
        if let (Some(llm), Some(harness)) = (&context.llm, &context.harness) {
            command.env("GIT_COMMITTER_NAME", format!("LLM: {llm} via {harness}"));
        }
    }
    command.output().expect("spawn givn subprocess")
}

#[given("a Fossil checkout without a givn/ directory")]
fn given_fossil_checkout_without_givn(world: &mut GivnWorld) {
    world.env = Some(common::make_fossil_root_bare());
}

#[then("no .githooks directory exists")]
fn then_no_githooks_directory(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(
        !env.root().join(".githooks").exists(),
        "the Fossil project received Git attribution plumbing"
    );
}
