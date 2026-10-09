//! E2E step definitions for the llm-attribution capability.

use std::process::{Command, Output};

use cucumber::{given, then, when};

use super::GivnWorld;
use super::llm_attribution_steps::{
    AttributionContext, GitAttributionState, git, git_with_givn_path,
};
use crate::common;

#[given("a project with the generated OpenCode attribution integration")]
fn given_generated_opencode_integration(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let output = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(
        output.status.success(),
        "givn init failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        include_str!("../../assets/agent-integrations/opencode/givn-attribution.js"),
        include_str!("../../.opencode/plugins/givn-attribution.js"),
        "checked-in OpenCode integration must match the embedded asset"
    );
    let generated = std::fs::read_to_string(env.root().join(".opencode/plugins/givn-attribution.js"))
        .expect("read generated OpenCode integration");
    assert_eq!(
        include_str!("../../assets/agent-integrations/opencode/givn-attribution.js"),
        generated,
        "the generated OpenCode integration must match the embedded asset"
    );
    std::fs::write(env.path("package.json"), r#"{"type":"module"}"#)
        .expect("write JavaScript module configuration");
    world.env = Some(env);
}

#[when(
    regex = r##"^OpenCode supplies the selected provider "([^"]+)" and model "([^"]+)" for session "([^"]+)"$"##
)]
fn when_opencode_supplies_selected_model(
    world: &mut GivnWorld,
    provider: String,
    model: String,
    session: String,
) {
    let state = world
        .opencode_plugin
        .get_or_insert_with(OpencodePluginState::default);
    state.supplies.push((provider, model, session));
}

#[when(regex = r##"^OpenCode prepares the environment for a shell in session "([^"]+)"$"##)]
fn when_opencode_prepares_shell(world: &mut GivnWorld, session: String) {
    let root = world.env.as_ref().expect("project root").root();
    let state = world
        .opencode_plugin
        .get_or_insert_with(OpencodePluginState::default);
    state.session = session;

    let mut script = format!(
        "import plugin from {};
const handlers = new Map();
const ctx = {{
  session: {{ hook: (kind, handler) => handlers.set(`session:${{kind}}`, handler) }},
  tool: {{ hook: (kind, handler) => handlers.set(`tool:${{kind}}`, handler) }},
  shell: {{ hook: (kind, handler) => handlers.set(`shell:${{kind}}`, handler) }},
  model: {{ default: async () => ({{ data: undefined }}) }},
}};
await plugin.setup(ctx);
const fire = (key, event) => handlers.get(key)(event);
",
        javascript_string(&root.join(".opencode/plugins/givn-attribution.js")),
    );
    for (provider, model, supplied_session) in &state.supplies {
        script.push_str(&format!(
            "await fire('session:generate', {{ sessionID: {}, model: {{ providerID: {}, id: {}, variant: 'high' }} }});\n",
            javascript_string_value(supplied_session),
            javascript_string_value(provider),
            javascript_string_value(model),
        ));
    }
    script.push_str(&format!(
        "const command = 'git commit -m test';
await fire('tool:execute.before', {{ tool: 'shell', sessionID: {}, messageID: 'message-1', id: 'call-1', input: {{ command }} }});
const shellEvent = {{ command, env: {{}} }};
await fire('shell:create.before', shellEvent);
console.log(JSON.stringify(shellEvent.env));
",
        javascript_string_value(&state.session),
    ));

    let script_path = root.join("invoke-opencode-shell.mjs");
    std::fs::write(&script_path, script).expect("write OpenCode shell script");
    let output = Command::new("node")
        .arg(&script_path)
        .current_dir(&root)
        .output()
        .expect("run node");
    assert!(
        output.status.success(),
        "OpenCode shell hook failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    state.output = Some(output);
}

#[then(regex = r##"^the shell environment identifies LLM "([^"]+)"$"##)]
fn then_opencode_shell_identifies_llm(world: &mut GivnWorld, expected: String) {
    assert_opencode_env(world, "GIVN_ATTRIBUTION_LLM", &expected);
}

#[then(regex = r##"^the shell environment identifies harness "([^"]+)"$"##)]
fn then_opencode_shell_identifies_harness(world: &mut GivnWorld, expected: String) {
    assert_opencode_env(world, "GIVN_ATTRIBUTION_HARNESS", &expected);
}

#[then(regex = r##"^the shell environment identifies reasoning mode "([^"]+)"$"##)]
fn then_opencode_shell_identifies_reasoning(world: &mut GivnWorld, expected: String) {
    assert_opencode_env(world, "GIVN_ATTRIBUTION_REASONING", &expected);
}

#[then(regex = r##"^the shell environment identifies committer "([^"]+)"$"##)]
fn then_opencode_shell_identifies_committer(world: &mut GivnWorld, expected: String) {
    assert_opencode_env(world, "GIT_COMMITTER_NAME", &expected);
}

#[then(regex = r##"^the shell environment carries no committer identity$"##)]
fn then_opencode_shell_carries_no_committer_identity(world: &mut GivnWorld) {
    let state = world
        .opencode_plugin
        .as_ref()
        .expect("OpenCode plugin state");
    let output = state.output.as_ref().expect("OpenCode shell output");
    let env: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("parse OpenCode shell environment");
    assert!(
        env.get("GIT_COMMITTER_NAME").is_none(),
        "shell environment must not carry a committer identity: {:?}",
        env.get("GIT_COMMITTER_NAME")
    );
}

#[derive(Debug, Default)]
pub(crate) struct OpencodePluginState {
    supplies: Vec<(String, String, String)>,
    session: String,
    output: Option<Output>,
}

fn assert_opencode_env(world: &GivnWorld, name: &str, expected: &str) {
    let state = world
        .opencode_plugin
        .as_ref()
        .expect("OpenCode plugin state");
    let output = state.output.as_ref().expect("OpenCode shell output");
    let env: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("parse OpenCode shell environment");
    assert_eq!(
        env.get(name).and_then(serde_json::Value::as_str),
        Some(expected)
    );
}

fn javascript_string(path: &std::path::Path) -> String {
    serde_json::to_string(&path.to_string_lossy()).expect("encode JavaScript string")
}

fn javascript_string_value(value: &str) -> String {
    serde_json::to_string(value).expect("encode JavaScript string")
}

#[given("a project with the generated Claude attribution integration")]
fn given_generated_claude_integration(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let root = env.root();
    assert!(git(&root, &["init"]).status.success());
    assert!(
        git(&root, &["config", "user.name", "Human Developer"])
            .status
            .success()
    );
    assert!(
        git(&root, &["config", "user.email", "human@example.test"])
            .status
            .success()
    );
    let output = common::run_givn(&env, &["init", "--agents", "claude"]);
    assert!(
        output.status.success(),
        "givn init failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let rtk = env.root().join("rtk");
    std::fs::write(&rtk, "#!/bin/sh\n[ \"$1\" = run ] && shift\nexec \"$@\"\n")
        .expect("write RTK fixture");
    make_executable(&rtk);
    world.env = Some(env);
}

#[when(regex = r##"^Claude provides current model "([^"]+)" in its session transcript$"##)]
fn when_claude_provides_model(world: &mut GivnWorld, model: String) {
    let root = world.env.as_ref().expect("project root").root();
    let transcript = root.join("transcript.jsonl");
    std::fs::write(
        &transcript,
        format!(
            "{{\"type\":\"assistant\",\"message\":{{\"role\":\"assistant\",\"model\":{}}}}}\n",
            javascript_string_value(&model)
        ),
    )
    .expect("write Claude transcript");
    world.claude_attribution = Some(ClaudeAttributionState {
        transcript,
        output: None,
    });
}

#[when(regex = r##"^Claude prepares an RTK-wrapped Bash commit command with message "([^"]+)"$"##)]
fn when_claude_prepares_commit(world: &mut GivnWorld, message: String) {
    let state = world
        .claude_attribution
        .as_ref()
        .expect("Claude attribution state");
    let root = world.env.as_ref().expect("project root").root();
    std::fs::write(root.join("fixture.txt"), &message).expect("write fixture");
    assert!(git(&root, &["add", "fixture.txt"]).status.success());
    let payload = serde_json::json!({
        "transcript_path": state.transcript,
        "tool_name": "Bash",
        "tool_input": {"command": format!("rtk run git commit -m '{message}'")},
        "effort": {"level": "high"}
    });
    let hook = root.join(".claude/hooks/givn-attribution.sh");
    let mut command = Command::new(&hook);
    command
        .current_dir(&root)
        .env("PATH", path_with_rtk(&root))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    let mut child = command.spawn().expect("run Claude hook");
    use std::io::Write;
    child
        .stdin
        .take()
        .expect("Claude hook stdin")
        .write_all(
            serde_json::to_string(&payload)
                .expect("serialize payload")
                .as_bytes(),
        )
        .expect("write Claude hook payload");
    let hook_output = child.wait_with_output().expect("wait for Claude hook");
    assert!(
        hook_output.status.success(),
        "Claude hook failed: {}",
        String::from_utf8_lossy(&hook_output.stderr)
    );
    let response: serde_json::Value =
        serde_json::from_slice(&hook_output.stdout).expect("parse Claude hook response");
    let updated = response["hookSpecificOutput"]["updatedInput"]["command"]
        .as_str()
        .expect("updated Claude command");
    let output = Command::new("sh")
        .arg("-c")
        .arg(updated)
        .current_dir(&root)
        .env("PATH", path_with_rtk(&root))
        .output()
        .expect("run attributed Claude command");
    world
        .claude_attribution
        .as_mut()
        .expect("Claude attribution state")
        .output = Some(output);
}

#[then("the attributed Claude commit succeeds")]
fn then_claude_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .claude_attribution
        .as_ref()
        .and_then(|state| state.output.as_ref())
        .expect("Claude commit output");
    assert!(
        output.status.success(),
        "Claude commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r##"^its Claude committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_claude_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("LLM: {llm} via {harness}")));
}

#[then(regex = r##"^its Claude message identifies LLM "([^"]+)"$"##)]
fn then_claude_message_identifies(world: &mut GivnWorld, llm: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("Co-Authored-By: {llm}/")));
}

#[then("its Claude message identifies the co-author attribution")]
fn then_claude_message_identifies_coauthor(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!(
        "Co-Authored-By: claude-sonnet-5/high@claude-code with Givn {}",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

#[derive(Debug)]
pub(crate) struct ClaudeAttributionState {
    transcript: std::path::PathBuf,
    output: Option<Output>,
}

fn givn_path() -> std::ffi::OsString {
    let binary = std::path::PathBuf::from(env!("CARGO_BIN_EXE_givn"));
    let binary_dir = binary.parent().expect("givn binary directory");
    let mut paths = vec![binary_dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    std::env::join_paths(paths).expect("join hook PATH")
}

fn path_with_rtk(root: &std::path::Path) -> std::ffi::OsString {
    let mut paths = vec![root.to_path_buf()];
    paths.extend(std::env::split_paths(&givn_path()));
    std::env::join_paths(paths).expect("join RTK fixture PATH")
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = std::fs::metadata(path)
        .expect("RTK fixture metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).expect("make RTK fixture executable");
}

#[cfg(not(unix))]
fn make_executable(_path: &std::path::Path) {}

#[given("a project root without a givn/ directory and with a Git repository")]
fn given_bare_git_project(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let output = Command::new("git")
        .args(["init"])
        .current_dir(env.root())
        .output()
        .expect("run git init");
    assert!(output.status.success());
    world.env = Some(env);
}

#[then("the checked-in commit attribution hook exists")]
fn then_checked_in_hook_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".githooks/commit-msg").is_file());
}

#[then("the Git repository uses the checked-in commit attribution hook")]
fn then_git_uses_hook(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks"])
        .current_dir(env.root())
        .output()
        .expect("read Git hooks path");
    assert!(output.status.success());
    let hooks = env
        .root()
        .join(String::from_utf8_lossy(&output.stdout).trim());
    let launcher = hooks.join("commit-msg");
    assert!(launcher.is_file());
    assert!(
        std::fs::read_to_string(launcher)
            .expect("read launcher")
            .contains("givn commit-attribution launcher")
    );
}

#[then("the OpenCode attribution integration exists")]
fn then_opencode_integration_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".opencode/plugins/givn-attribution.js").is_file());
}

#[then("the Claude Code attribution integration exists")]
fn then_claude_integration_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".claude/hooks/givn-attribution.sh").is_file());
    assert!(env.path(".claude/settings.local.json").is_file());
}

#[given("a Git project with complete commit attribution context")]
fn given_complete_commit_context(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let root = env.root();
    assert!(git(&root, &["init"]).status.success());
    assert!(
        git(&root, &["config", "user.name", "Human Developer"])
            .status
            .success()
    );
    assert!(
        git(&root, &["config", "user.email", "human@example.test"])
            .status
            .success()
    );
    let output = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(output.status.success());
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState {
        context: Some(AttributionContext {
            llm: Some("openai/gpt-5.6-luna".to_string()),
            reasoning: Some("high".to_string()),
            harness: Some("opencode".to_string()),
        }),
        ..GitAttributionState::default()
    });
}

#[when(regex = r##"^the consumer creates an attributed Git commit with message "([^"]+)"$"##)]
fn when_consumer_creates_attributed_commit(world: &mut GivnWorld, message: String) {
    let root = world.env.as_ref().expect("project root").root();
    std::fs::write(root.join("fixture.txt"), &message).expect("write fixture");
    assert!(
        git_with_givn_path(&root, &["add", "fixture.txt"], None)
            .status
            .success()
    );
    let context = world
        .git_attribution
        .as_ref()
        .expect("attribution state")
        .context
        .clone();
    let output = git_with_givn_path(&root, &["commit", "-m", &message], context.as_ref());
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .last_output = Some(output);
}

#[then("the attributed commit succeeds")]
fn then_attributed_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .expect("commit output");
    assert!(output.status.success(), "attributed commit failed");
}

#[then(regex = r##"^its committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_attributed_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("LLM: {llm} via {harness}")));
}

#[then("its message contains only the managed co-author attribution")]
fn then_attributed_message_has_complete_context(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.trim_end().ends_with(&format!(
        "Co-Authored-By: openai/gpt-5.6-luna/high@opencode with Givn {}",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

#[then("its message ends with the current Givn version, LLM, reasoning mode, and harness")]
fn then_legacy_attributed_message_wording_uses_current_contract(world: &mut GivnWorld) {
    then_attributed_message_has_complete_context(world);
}

/// State for the generated omp attribution integration scenarios.
#[derive(Debug)]
pub(crate) struct OmpAttributionState {
    model: Option<(String, String)>,
    reasoning: Option<String>,
    prepared: Option<String>,
    output: Option<Output>,
}

#[given("a project with the generated omp attribution integration")]
fn given_generated_omp_integration(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let root = env.root();
    assert!(git(&root, &["init"]).status.success());
    assert!(
        git(&root, &["config", "user.name", "Human Developer"])
            .status
            .success()
    );
    assert!(
        git(&root, &["config", "user.email", "human@example.test"])
            .status
            .success()
    );
    let output = common::run_givn(&env, &["init", "--agents", "omp"]);
    assert!(
        output.status.success(),
        "givn init failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        include_str!("../../assets/agent-integrations/omp/givn-attribution.js"),
        std::fs::read_to_string(root.join(".omp/hooks/pre/givn-attribution.js"))
            .expect("read generated omp integration"),
        "the generated omp integration must match the embedded asset"
    );
    // `node` resolves a `.js` file's module kind from the nearest package.json.
    std::fs::write(root.join("package.json"), r#"{"type":"module"}"#)
        .expect("write JavaScript module configuration");
    world.env = Some(env);
    world.omp_attribution = Some(OmpAttributionState {
        model: None,
        reasoning: None,
        prepared: None,
        output: None,
    });
}

#[given(regex = r##"^the omp attribution integration exists$"##)]
fn given_omp_integration_exists(world: &mut GivnWorld) {
    then_omp_integration_exists(world);
}

#[given(regex = r##"^omp's current model is "([^"]+)" at reasoning mode "([^"]+)"$"##)]
fn given_omp_current_model(world: &mut GivnWorld, model: String, reasoning: String) {
    let (provider, id) = model
        .split_once('/')
        .map(|(provider, id)| (provider.to_string(), id.to_string()))
        .unwrap_or_else(|| (String::new(), model.clone()));
    let state = world
        .omp_attribution
        .as_mut()
        .expect("omp attribution state");
    state.model = Some((provider, id));
    state.reasoning = Some(reasoning);
}

#[given("omp has no current model")]
fn given_omp_without_model(world: &mut GivnWorld) {
    let state = world
        .omp_attribution
        .as_mut()
        .expect("omp attribution state");
    state.model = None;
    state.reasoning = None;
}

#[when(regex = r##"^omp prepares the Bash command "([^"]+)"$"##)]
fn when_omp_prepares_command(world: &mut GivnWorld, command: String) {
    let prepared = omp_prepared_command(world, &command);
    world
        .omp_attribution
        .as_mut()
        .expect("omp attribution state")
        .prepared = Some(prepared);
}

#[then(regex = r##"^the prepared command is "([^"]+)"$"##)]
fn then_prepared_command_is(world: &mut GivnWorld, expected: String) {
    let prepared = world
        .omp_attribution
        .as_ref()
        .and_then(|state| state.prepared.as_ref())
        .expect("prepared omp command");
    assert_eq!(prepared, &expected);
}

#[then(regex = r##"^the prepared command identifies LLM "([^"]+)"$"##)]
fn then_prepared_command_identifies_llm(world: &mut GivnWorld, expected: String) {
    assert_prepared_env(world, "GIVN_ATTRIBUTION_LLM", &expected);
}

#[then(regex = r##"^the prepared command identifies harness "([^"]+)"$"##)]
fn then_prepared_command_identifies_harness(world: &mut GivnWorld, expected: String) {
    assert_prepared_env(world, "GIVN_ATTRIBUTION_HARNESS", &expected);
}

#[when(regex = r##"^omp prepares and runs a Bash revision command with message "([^"]+)"$"##)]
fn when_omp_prepares_and_runs_commit(world: &mut GivnWorld, message: String) {
    let root = world.env.as_ref().expect("project root").root();
    std::fs::write(root.join("fixture.txt"), &message).expect("write fixture");
    assert!(git(&root, &["add", "fixture.txt"]).status.success());

    let prepared = omp_prepared_command(world, &format!("git commit -m '{message}'"));
    {
        let state = world
            .omp_attribution
            .as_mut()
            .expect("omp attribution state");
        state.prepared = Some(prepared.clone());
    }
    // The checked-in commit-msg hook only stamps attribution when `givn` is
    // resolvable, so the shell that runs the prepared command needs the
    // compiled binary on PATH — CI has no installed `givn`.
    let output = Command::new("sh")
        .arg("-c")
        .arg(&prepared)
        .current_dir(&root)
        .env("PATH", givn_path())
        .output()
        .expect("run attributed omp command");
    world
        .omp_attribution
        .as_mut()
        .expect("omp attribution state")
        .output = Some(output);
}

#[then("the attributed omp commit succeeds")]
fn then_omp_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .omp_attribution
        .as_ref()
        .and_then(|state| state.output.as_ref())
        .expect("omp commit output");
    assert!(
        output.status.success(),
        "omp commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r##"^its omp committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_omp_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("LLM: {llm} via {harness}")));
}

#[then(regex = r##"^its omp message identifies LLM "([^"]+)"$"##)]
fn then_omp_message_identifies_llm(world: &mut GivnWorld, llm: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("Co-Authored-By: {llm}/")));
}

#[then(regex = r##"^its omp message identifies reasoning mode "([^"]+)"$"##)]
fn then_omp_message_identifies_reasoning(world: &mut GivnWorld, reasoning: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!("/{reasoning}@omp with Givn ")));
}

#[then("the omp attribution integration exists")]
fn then_omp_integration_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    assert!(env.path(".omp/hooks/pre/givn-attribution.js").is_file());
}

/// Drive the generated omp hook through `node` for one Bash command and return
/// the command it prepared (unchanged when the hook returned no revision).
fn omp_prepared_command(world: &GivnWorld, command: &str) -> String {
    let state = world
        .omp_attribution
        .as_ref()
        .expect("omp attribution state");
    let root = world.env.as_ref().expect("project root").root();
    let model = match &state.model {
        Some((provider, id)) if provider.is_empty() => {
            format!("{{ id: {} }}", javascript_string_value(id))
        }
        Some((provider, id)) => format!(
            "{{ provider: {}, id: {} }}",
            javascript_string_value(provider),
            javascript_string_value(id)
        ),
        None => "null".to_string(),
    };
    let reasoning = match &state.reasoning {
        Some(level) => javascript_string_value(level),
        None => "null".to_string(),
    };
    let script = format!(
        "import givn from {};
const handlers = [];
const model = {};
const reasoning = {};
const pi = {{
  on: (event, handler) => handlers.push({{ event, handler }}),
  getThinkingLevel: () => reasoning ?? undefined,
}};
givn(pi);
const entry = handlers.find((candidate) => candidate.event === 'tool_call');
if (!entry) throw new Error('no tool_call handler registered');
const ctx = {{ models: {{ current: () => model ?? undefined }}, model: model ?? undefined }};
const result = await entry.handler(
  {{ type: 'tool_call', toolCallId: 'omp-1', toolName: 'bash', input: {{ command: {} }} }},
  ctx,
);
console.log(JSON.stringify({{ command: result?.input?.command ?? null }}));
",
        javascript_string(&root.join(".omp/hooks/pre/givn-attribution.js")),
        model,
        reasoning,
        javascript_string_value(command),
    );
    let script_path = root.join("invoke-omp-hook.mjs");
    std::fs::write(&script_path, script).expect("write omp hook script");
    let output = Command::new("node")
        .arg(&script_path)
        .current_dir(&root)
        .output()
        .expect("run node");
    assert!(
        output.status.success(),
        "omp hook invocation failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let parsed: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("parse omp hook output");
    parsed
        .get("command")
        .and_then(serde_json::Value::as_str)
        .unwrap_or(command)
        .to_string()
}

fn assert_prepared_env(world: &GivnWorld, name: &str, expected: &str) {
    let prepared = world
        .omp_attribution
        .as_ref()
        .and_then(|state| state.prepared.as_ref())
        .expect("prepared omp command");
    assert!(
        prepared.contains(&format!("{name}='{expected}'")),
        "prepared command does not identify {name} {expected}: {prepared}"
    );
}
