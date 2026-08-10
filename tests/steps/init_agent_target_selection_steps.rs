//! Gherkin step definitions for the `init-agent-target-selection` capability.
//!
//! Covers: the `givn init` agent-target prompt (interactive answer via
//! piped stdin), persisting the selection in `givn/config.yaml`, and the
//! `givn skills sync` target selection.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

/// Split an expected `"a, b, c"` target list into canonical names.
fn expected_targets(list: &str) -> Vec<String> {
    list.split(',')
        .map(|s| s.trim().trim_start_matches('.').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Write or replace the `skills:` block in `givn/config.yaml` with the
/// given canonical target names (used by the Given-form of the target step).
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

/// Parse the `targets:` list from the `skills:` block of givn/config.yaml.
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

#[given(regex = r#"^a givn/config.yaml with skills targets "([^"]+)"$"#)]
fn scaffold_config_with_skills_targets(world: &mut GivnWorld, list: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let dir = env.path("givn");
    std::fs::create_dir_all(&dir).expect("create givn dir");
    let list = expected_targets(&list)
        .iter()
        .map(|t| format!("'{}'", t))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("givn_config_version: \"0.2.0\"\nskills:\n  targets: [{list}]\n");
    std::fs::write(env.path("givn/config.yaml"), content).expect("write givn/config.yaml");
}

#[then(regex = r#"^givn skills targets in givn/config.yaml equal "([^"]+)"$"#)]
fn assert_skills_targets(world: &mut GivnWorld, list: String) {
    let actual = read_skills_targets(world);
    let expected = expected_targets(&list);
    assert_eq!(actual, expected, "givn/config.yaml skills targets mismatch");
}

#[when(regex = r#"^the user runs `givn init` and answers the agent prompt with "([^"]*)"$"#)]
fn init_with_answer(world: &mut GivnWorld, answer: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = crate::common::run_givn_with_stdin(env, &["init"], &answer);
    world.last_output = Some(out);
}

#[when(regex = r#"^the consumer runs `givn init` and answers the agent prompt with "([^"]*)"$"#)]
fn consumer_init_with_answer(world: &mut GivnWorld, answer: String) {
    init_with_answer(world, answer);
}

#[when("the consumer invokes `givn init` without answering a prompt")]
fn consumer_init_silent(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = crate::common::run_givn(env, &["init"]);
    world.last_output = Some(out);
}

#[when(regex = r#"^the consumer runs `([^`]+)`$"#)]
fn consumer_runs(world: &mut GivnWorld, cmd: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let mut argv: Vec<&str> = cmd.split_whitespace().collect();
    if argv.first() == Some(&"givn") {
        argv.remove(0);
    }
    let out = crate::common::run_givn(env, &argv);
    world.last_output = Some(out);
}
