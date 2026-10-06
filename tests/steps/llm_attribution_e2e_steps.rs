//! E2E step definitions for the llm-attribution capability.

use std::process::{Command, Output};

use cucumber::{given, then, when};

use super::GivnWorld;
use super::llm_attribution_steps::{
    AttributionContext, GitAttributionState, git, git_with_givn_path,
};
use crate::common;

/// The five values the attribution plugin must put into the shell the real
/// OpenCode loader creates: all four attribution variables plus the committer.
pub(crate) fn assert_attribution_environment(
    result: &serde_json::Value,
    model: &str,
    reasoning: &str,
) {
    let expected = [
        ("GIVN_ATTRIBUTION_ACTIVE", "1".to_owned()),
        ("GIVN_ATTRIBUTION_LLM", model.to_owned()),
        ("GIVN_ATTRIBUTION_REASONING", reasoning.to_owned()),
        ("GIVN_ATTRIBUTION_HARNESS", "opencode".to_owned()),
        ("GIT_COMMITTER_NAME", format!("LLM: {model} via opencode")),
    ];
    for (name, value) in expected {
        assert_eq!(
            result[name].as_str(),
            Some(value.as_str()),
            "completed shell result {name}: {result}"
        );
    }
}

/// A generation failure warning of `givn init`/sync is a failure in the happy
/// path: an exit code alone does not prove the integration was written.
pub(crate) fn assert_no_generation_warning(world: &GivnWorld) {
    if let Some(output) = world.last_output.as_ref() {
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !text.contains("generation failed"),
            "the Givn command reported an integration generation warning: {text}"
        );
    }
}

/// The generated plugin of the prepared project is the embedded asset and the
/// only plugin file OpenCode can discover.
pub(crate) fn assert_canonical_plugin(root: &std::path::Path) {
    let plugins = root.join(".opencode/plugins");
    let names: Vec<_> = std::fs::read_dir(&plugins)
        .expect("plugin directory")
        .map(|entry| entry.expect("plugin entry").file_name())
        .collect();
    assert_eq!(names, ["givn-attribution.js"], "one canonical plugin only");
    assert_eq!(
        std::fs::read(plugins.join("givn-attribution.js")).expect("generated plugin"),
        include_bytes!("../../assets/agent-integrations/opencode/givn-attribution.js"),
        "the generated plugin must be the embedded asset"
    );
}

/// Run the real OpenCode `release` in the scenario's prepared project and
/// file the labeled transcript shard. Returns the runtime evidence.
pub(crate) fn run_release_and_record(
    world: &GivnWorld,
    release: &str,
    scenario: &str,
    extra_env: &[(&str, &str)],
) -> serde_json::Value {
    let root = world.env.as_ref().expect("project root").root();
    let mut extra_env = extra_env.to_vec();
    // Negative control for the RED evidence: another module (for example the
    // legacy V1-only plugin) stands in for the generated plugin, to show that
    // the real-loader assertions reject it. Never set by the regular suites.
    if let Some(control) = std::env::var_os("GIVN_OPENCODE_CONTROL_PLUGIN") {
        std::fs::copy(control, root.join(".opencode/plugins/givn-attribution.js"))
            .expect("install the control plugin");
        extra_env.push(("GIVN_FIXTURE_CONTROL_PLUGIN", "1"));
    } else {
        assert_canonical_plugin(&root);
    }
    assert_no_generation_warning(world);
    let evidence =
        common::opencode_runtime::run_opencode_shell(&root, release, scenario, &extra_env)
            .unwrap_or_else(|error| panic!("{error:#}"));
    let title = world.scenario_title.clone().unwrap_or_default();
    let key = common::opencode_runtime::release_key(release).expect("release key");
    let shard = match world.transcript_variant.as_deref() {
        Some(variant) => format!("{variant}--{key}"),
        None => key.to_owned(),
    };
    common::opencode_runtime::write_transcript_shard(
        &title,
        &shard,
        &transcript(world, release, &evidence),
    )
    .expect("write transcript shard");
    evidence
}

fn transcript(world: &GivnWorld, release: &str, evidence: &serde_json::Value) -> String {
    use std::fmt::Write as _;
    let mut text = String::new();
    let version = evidence["version"].as_str().unwrap_or("unknown");
    let _ = writeln!(text, "=== {release} (OpenCode {version}) ===");
    if let Some(variant) = world.transcript_variant.as_deref() {
        let _ = writeln!(text, "example: {variant}");
    }
    if let (Some(argv), Some(output)) = (world.last_argv.as_ref(), world.last_output.as_ref()) {
        let _ = writeln!(text, "\n$ givn {}", argv.join(" "));
        let _ = writeln!(text, "exit status: {}", output.status.code().unwrap_or(-1));
        let _ = writeln!(
            text,
            "--- givn stdout ---\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let _ = writeln!(
            text,
            "--- givn stderr ---\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let argv: Vec<&str> = evidence["argv"]
        .as_array()
        .map(|items| items.iter().filter_map(|item| item.as_str()).collect())
        .unwrap_or_default();
    let _ = writeln!(text, "$ {}", argv.join(" "));
    let _ = writeln!(text, "exit status: {}", evidence["cli"]["exitCode"]);
    let _ = writeln!(
        text,
        "--- opencode stdout (raw) ---\n{}",
        evidence["cli"]["stdout"].as_str().unwrap_or_default()
    );
    let _ = writeln!(
        text,
        "--- opencode stderr (raw) ---\n{}",
        evidence["cli"]["stderr"].as_str().unwrap_or_default()
    );
    let label = if evidence["child"].is_null() {
        "completed shell tool result".to_owned()
    } else {
        format!(
            "completed shell tool result of the subagent's child session {} (read from GET /api/session/<child>/message)",
            evidence["child"]["sessionID"].as_str().unwrap_or_default()
        )
    };
    let _ = writeln!(
        text,
        "--- {label} ---\n{}",
        evidence["toolOutput"].as_str().unwrap_or_default()
    );
    let _ = writeln!(
        text,
        "--- attribution environment ---\n{}",
        serde_json::to_string_pretty(&evidence["result"]).unwrap_or_default()
    );
    if !evidence["child"].is_null() {
        let _ = writeln!(
            text,
            "--- subagent session ---\n{}",
            serde_json::to_string_pretty(&evidence["child"]).unwrap_or_default()
        );
    }
    text
}

#[given("a project with the generated OpenCode attribution integration")]
fn given_generated_opencode_integration(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    assert!(
        git(&env.root(), &["init", "--quiet"]).status.success(),
        "git init failed"
    );
    let output = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(
        output.status.success(),
        "givn init failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    world.last_argv = Some(vec!["init".into(), "--agents".into(), "opencode".into()]);
    world.last_output = Some(output);
    assert_no_generation_warning(world);
    let installed = std::fs::read_to_string(env.path(".opencode/plugins/givn-attribution.js"))
        .expect("read generated OpenCode integration");
    assert_eq!(
        include_str!("../../assets/agent-integrations/opencode/givn-attribution.js"),
        installed,
        "generated OpenCode integration must match the embedded asset"
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
    world.opencode_plugin = Some(OpencodePluginState {
        provider,
        model,
        session,
        output: None,
        real_result: None,
    });
}

#[when(regex = r##"^OpenCode prepares the environment for a shell in session "([^"]+)"$"##)]
fn when_opencode_prepares_shell(world: &mut GivnWorld, session: String) {
    let state = world
        .opencode_plugin
        .as_ref()
        .expect("OpenCode plugin state");
    assert_eq!(state.session, session);
    let root = world.env.as_ref().expect("project root").root();
    let plugin_state = world
        .opencode_plugin
        .as_ref()
        .expect("OpenCode plugin state");
    let script = format!(
        "import {{ GivnAttributionPlugin }} from {};
 const hooks = await GivnAttributionPlugin();
 await hooks['chat.message']({{ sessionID: {}, model: {{ providerID: {}, modelID: {} }}, variant: 'high' }}, {{}});
await hooks['chat.params']({{ sessionID: {}, model: {{ providerID: {}, modelID: {} }}, agent: 'build', provider: {{}}, message: {{}} }}, {{}});
 const output = {{ env: {{}} }};
await hooks['shell.env']({{ sessionID: {} }}, output);
console.log(JSON.stringify(output.env));
",
        javascript_string(&root.join(".opencode/plugins/givn-attribution.js")),
        javascript_string_value(&session),
        javascript_string_value(&plugin_state.provider),
        javascript_string_value(&plugin_state.model),
        javascript_string_value(&session),
        javascript_string_value(&plugin_state.provider),
        javascript_string_value(&plugin_state.model),
        javascript_string_value(&session),
    );
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
    world
        .opencode_plugin
        .as_mut()
        .expect("OpenCode plugin state")
        .output = Some(output);
}

#[when(
    regex = r##"^the developer asks OpenCode "([^"]+)" with model "([^"]+)" and reasoning mode "([^"]+)" to execute the attribution fixture command$"##
)]
fn when_developer_runs_real_opencode(
    world: &mut GivnWorld,
    release: String,
    model: String,
    reasoning: String,
) {
    assert_eq!(model, "givn-fixture/model-a", "the fixture's primary model");
    assert_eq!(reasoning, "high", "the fixture's primary reasoning mode");
    let evidence = run_release_and_record(world, &release, "main", &[]);
    world.opencode_plugin = Some(OpencodePluginState {
        provider: "givn-fixture".to_owned(),
        model,
        session: "real-runtime".to_owned(),
        output: None,
        real_result: Some(evidence),
    });
}

#[given(
    regex = r##"^the project's subagent is configured with model "([^"]+)" and reasoning mode "([^"]+)"$"##
)]
fn given_subagent_configuration(world: &mut GivnWorld, model: String, reasoning: String) {
    assert_eq!(
        model, "givn-fixture/model-b",
        "the fixture provider twin serves the subagent model-b"
    );
    world.subagent_config = Some((model, reasoning));
}

#[when(
    regex = r##"^the developer asks OpenCode "([^"]+)" with model "([^"]+)" and reasoning mode "([^"]+)" to have its subagent execute the attribution fixture command$"##
)]
fn when_developer_runs_real_opencode_subagent(
    world: &mut GivnWorld,
    release: String,
    model: String,
    reasoning: String,
) {
    assert_eq!(release, "V2 latest", "the subagent scenario is V2 only");
    assert_eq!(model, "givn-fixture/model-a", "the fixture's primary model");
    assert_eq!(reasoning, "high", "the fixture's primary reasoning mode");
    let (subagent_model, subagent_reasoning) = world
        .subagent_config
        .clone()
        .expect("the subagent configuration is given first");
    let configured = format!("{subagent_model}#{subagent_reasoning}");
    let evidence = run_release_and_record(
        world,
        &release,
        "subagent",
        &[("GIVN_FIXTURE_SUBAGENT_MODEL", configured.as_str())],
    );
    world.opencode_plugin = Some(OpencodePluginState {
        provider: "givn-fixture".to_owned(),
        model,
        session: "real-runtime".to_owned(),
        output: None,
        real_result: Some(evidence),
    });
}

#[then(
    regex = r##"^an attributed shell command through OpenCode "([^"]+)" reports model "([^"]+)" and reasoning mode "([^"]+)"$"##
)]
fn then_attributed_shell_command_reports(
    world: &mut GivnWorld,
    release: String,
    model: String,
    reasoning: String,
) {
    let evidence = run_release_and_record(world, &release, "main", &[]);
    assert_eq!(evidence["kind"], "attribution-result");
    assert_attribution_environment(&evidence["result"], &model, &reasoning);
}

#[then("the OpenCode command exits 0")]
fn then_real_opencode_exited_successfully(world: &mut GivnWorld) {
    let state = world
        .opencode_plugin
        .as_ref()
        .expect("real OpenCode result");
    assert!(
        state.real_result.is_some(),
        "real OpenCode run must finish successfully"
    );
}

#[then(regex = r##"^the OpenCode command output identifies LLM "([^"]+)"$"##)]
fn then_real_opencode_identifies_model(world: &mut GivnWorld, expected: String) {
    assert_real_opencode_result(world, "GIVN_ATTRIBUTION_LLM", &expected);
}

#[then(regex = r##"^the OpenCode command output identifies reasoning mode "([^"]+)"$"##)]
fn then_real_opencode_identifies_reasoning(world: &mut GivnWorld, expected: String) {
    assert_real_opencode_result(world, "GIVN_ATTRIBUTION_REASONING", &expected);
}

#[then(regex = r##"^the OpenCode command output identifies harness "([^"]+)"$"##)]
fn then_real_opencode_identifies_harness(world: &mut GivnWorld, expected: String) {
    assert_real_opencode_result(world, "GIVN_ATTRIBUTION_HARNESS", &expected);
}

#[then(regex = r##"^the OpenCode command output identifies committer "([^"]+)"$"##)]
fn then_real_opencode_identifies_committer(world: &mut GivnWorld, expected: String) {
    assert_real_opencode_result(world, "GIT_COMMITTER_NAME", &expected);
}

#[then(regex = r##"^the subagent's command output identifies LLM "([^"]+)"$"##)]
fn then_subagent_identifies_model(world: &mut GivnWorld, expected: String) {
    assert_subagent_result(world, "GIVN_ATTRIBUTION_LLM", &expected);
}

#[then(regex = r##"^the subagent's command output identifies reasoning mode "([^"]+)"$"##)]
fn then_subagent_identifies_reasoning(world: &mut GivnWorld, expected: String) {
    assert_subagent_result(world, "GIVN_ATTRIBUTION_REASONING", &expected);
}

#[then(regex = r##"^the subagent's command output identifies committer "([^"]+)"$"##)]
fn then_subagent_identifies_committer(world: &mut GivnWorld, expected: String) {
    assert_subagent_result(world, "GIT_COMMITTER_NAME", &expected);
}

fn assert_subagent_result(world: &GivnWorld, name: &str, expected: &str) {
    let evidence = world
        .opencode_plugin
        .as_ref()
        .and_then(|state| state.real_result.as_ref())
        .expect("successful real OpenCode result");
    assert_eq!(evidence["kind"], "subagent-attribution-result");
    let child = evidence["child"]["sessionID"]
        .as_str()
        .expect("the subagent's child session");
    assert_eq!(
        evidence["sessionID"].as_str(),
        Some(child),
        "the asserted record is the subagent's own shell result"
    );
    assert_ne!(
        evidence["child"]["parentSessionID"].as_str(),
        Some(child),
        "the subagent runs in its own session"
    );
    assert_eq!(evidence["result"][name].as_str(), Some(expected));
}

#[then("the OpenCode command output reports active commit attribution")]
fn then_real_opencode_reports_active_attribution(world: &mut GivnWorld) {
    assert_real_opencode_result(world, "GIVN_ATTRIBUTION_ACTIVE", "1");
}

fn assert_real_opencode_result(world: &GivnWorld, name: &str, expected: &str) {
    let state = world
        .opencode_plugin
        .as_ref()
        .expect("real OpenCode result");
    let evidence = state
        .real_result
        .as_ref()
        .expect("successful real OpenCode result");
    assert_eq!(evidence["kind"], "attribution-result");
    assert_eq!(evidence["result"][name].as_str(), Some(expected));
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

#[derive(Debug)]
pub(crate) struct OpencodePluginState {
    provider: String,
    model: String,
    session: String,
    output: Option<Output>,
    real_result: Option<serde_json::Value>,
}

fn assert_opencode_env(world: &GivnWorld, name: &str, expected: &str) {
    if let Some(env) = super::llm_attribution_steps::characterized_shell_env(world) {
        assert_eq!(
            env.get(name).and_then(serde_json::Value::as_str),
            Some(expected),
            "characterized shell environment: {env}"
        );
        return;
    }
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
