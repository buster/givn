//! Gherkin step definitions for the `init-agent-target-selection` capability.
//!
//! Covers: the `givn init` agent-target prompt (interactive answer via
//! piped stdin), persisting the selection in `givn/config.yaml`, and the
//! `givn skills sync` target selection.

use crate::common;
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

#[given(regex = r#"^a givn/config.yaml without a skills block$"#)]
fn scaffold_config_without_skills(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let dir = env.path("givn");
    std::fs::create_dir_all(&dir).expect("create givn dir");
    std::fs::write(
        env.path("givn/config.yaml"),
        "givn_config_version: \"0.2.0\"\n",
    )
    .expect("write givn/config.yaml");
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

#[when("the user runs `givn init` in a terminal and confirms without marking a target")]
fn init_in_terminal_empty_confirmation(world: &mut GivnWorld) {
    use std::io::Write;
    use std::process::{Command, Output, Stdio};
    use std::time::{Duration, Instant};

    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let transcript = env.path(".terminal-transcript.txt");
    let bin = env!("CARGO_BIN_EXE_givn");
    // A real PTY: `script` (util-linux) gives the child a terminal on stdin, so
    // the dialoguer checkbox prompt runs; the piped newline confirms the list
    // without marking any entry.
    let mut child = Command::new("script")
        .args([
            "-qec",
            &format!("{bin} init"),
            &transcript.to_string_lossy(),
        ])
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn `script` (util-linux) to drive the terminal prompt");
    child
        .stdin
        .take()
        .expect("piped stdin for the PTY session")
        .write_all(b"\n")
        .expect("write the confirmation key");
    let deadline = Instant::now() + Duration::from_secs(60);
    let status = loop {
        match child.try_wait().expect("poll the PTY session") {
            Some(status) => break status,
            None if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(50)),
            None => {
                let _ = child.kill();
                panic!("the terminal prompt did not finish within 60 seconds");
            }
        }
    };
    let bytes = std::fs::read(&transcript).expect("read the terminal transcript");
    // The terminal transcript is the command's observable output: the prompt,
    // the banner, and the refusal all write to the same PTY.
    world.last_output = Some(Output {
        status,
        stdout: bytes.clone(),
        stderr: bytes,
    });
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

#[given("a Fossil checkout without a givn/ directory")]
fn given_fossil_checkout_without_givn(world: &mut GivnWorld) {
    world.env = Some(common::make_fossil_root_bare());
}

#[then("stdout names the givn commit attribution path")]
fn then_stdout_names_givn_commit_path(world: &mut GivnWorld) {
    assert!(
        crate::steps::stdout(world).contains("givn commit path only"),
        "stdout does not name the givn commit attribution path: {}",
        crate::steps::stdout(world)
    );
}

#[then("no .githooks directory exists")]
fn then_no_githooks_directory(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(
        !env.root().join(".githooks").exists(),
        "the Fossil project received Git attribution plumbing"
    );
}

#[then("the generated guidance names the commit path of the active backend")]
fn then_guidance_names_commit_path(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let skill = env.root().join(".opencode/skills/givn-implement/SKILL.md");
    let text = std::fs::read_to_string(&skill)
        .unwrap_or_else(|e| panic!("read generated skill {}: {e}", skill.display()));
    assert!(
        text.contains("givn commit"),
        "the generated guidance does not name `givn commit`"
    );
}

#[then("the generated guidance states that a raw fossil commit carries no attribution")]
fn then_guidance_states_fossil_limitation(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let skill = env.root().join(".opencode/skills/givn-implement/SKILL.md");
    let text = std::fs::read_to_string(&skill)
        .unwrap_or_else(|e| panic!("read generated skill {}: {e}", skill.display()));
    assert!(
        text.contains("carries no attribution"),
        "the generated guidance does not state the Fossil limitation"
    );
}

#[then("stdout warns that archive and managed upgrade are unavailable")]
fn then_stdout_warns_no_scm(world: &mut GivnWorld) {
    assert!(
        crate::steps::stdout(world).contains("archive and managed upgrade are unavailable"),
        "stdout does not warn about the missing backend: {}",
        crate::steps::stdout(world)
    );
}

#[then("a later archive is refused by name")]
fn then_later_archive_refused(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let out = common::run_givn(env, &["archive", "--change", "demo"]);
    assert!(
        !out.status.success(),
        "archive unexpectedly succeeded without a backend"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("version-controlled project"),
        "the refusal does not name the missing version-controlled project: {stderr}"
    );
}

#[given(regex = r"^a (Git|Fossil) project$")]
fn given_backend_project(world: &mut GivnWorld, backend: String) {
    world.env = Some(match backend.as_str() {
        "Git" => common::make_init_root(),
        "Fossil" => common::make_fossil_root(),
        other => panic!("unknown backend '{other}'"),
    });
}

#[then(regex = r"^stdout reports the active backend as (\w+)$")]
fn then_stdout_reports_scm(world: &mut GivnWorld, scm: String) {
    assert!(
        crate::steps::stdout(world).contains(&format!("SCM: {scm}")),
        "stdout does not report SCM: {scm} — {}",
        crate::steps::stdout(world)
    );
}

#[then("stdout names archive and managed upgrade as unavailable")]
fn then_stdout_names_unavailable(world: &mut GivnWorld) {
    assert!(
        crate::steps::stdout(world).contains("archive and managed upgrade are unavailable"),
        "stdout does not name the unavailable commands: {}",
        crate::steps::stdout(world)
    );
}
