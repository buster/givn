//! Step definitions for the llm-attribution capability.

use std::path::PathBuf;
use std::process::{Command, Output};

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;
use crate::common::opencode_characterize::Characterization;

#[derive(Debug, Default)]
pub(crate) struct GitAttributionState {
    pub(crate) last_output: Option<Output>,
    pub(crate) context: Option<AttributionContext>,
    pub(crate) commits: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AttributionContext {
    pub(crate) llm: Option<String>,
    pub(crate) reasoning: Option<String>,
    pub(crate) harness: Option<String>,
}

#[given("a Givn project with the shared commit-attribution hook active")]
fn given_givn_project_with_hook(world: &mut GivnWorld) {
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
    let output = common::run_givn(&env, &["init", "--agents", "opencode,claude"]);
    assert!(
        output.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState::default());
}

#[given(
    regex = r##"^the current attribution context has LLM "([^"]+)", reasoning mode "([^"]+)", and harness "([^"]+)"$"##
)]
fn given_complete_context(world: &mut GivnWorld, llm: String, reasoning: String, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        llm: Some(llm),
        reasoning: Some(reasoning),
        harness: Some(harness),
    });
}

#[given("no current attribution context")]
fn given_no_context(_world: &mut GivnWorld) {
    _world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = None;
}

#[given(
    regex = r##"^the current attribution context has harness "([^"]+)" but no LLM or reasoning mode$"##
)]
fn given_partial_context(world: &mut GivnWorld, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        harness: Some(harness),
        ..AttributionContext::default()
    });
}

#[given("a Givn project with an existing commit hook that writes a marker")]
fn given_existing_hook(world: &mut GivnWorld) {
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
    let hooks = String::from_utf8_lossy(&git(&root, &["rev-parse", "--git-path", "hooks"]).stdout)
        .trim()
        .to_string();
    let hooks = root.join(hooks);
    std::fs::create_dir_all(&hooks).expect("create hooks directory");
    let hook = hooks.join("commit-msg");
    std::fs::write(
        &hook,
        "#!/bin/sh\nprintf marker > \"$(git rev-parse --show-toplevel)/existing-hook.marker\"\n",
    )
    .expect("write existing hook");
    make_executable(&hook);
    world.env = Some(env);
    world.git_attribution = Some(GitAttributionState::default());
}

#[when(regex = r##"^the consumer creates a commit with message "([^"]+)"$"##)]
fn when_consumer_creates_commit(world: &mut GivnWorld, message: String) {
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
    if world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .is_some_and(|output| output.status.success())
    {
        let hash = git(&root, &["rev-parse", "HEAD"]);
        assert!(hash.status.success());
        world
            .git_attribution
            .as_mut()
            .expect("attribution state")
            .commits
            .push(String::from_utf8_lossy(&hash.stdout).trim().to_string());
    }
}

#[when(
    regex = r##"^the current attribution context changes to LLM "([^"]+)", reasoning mode "([^"]+)", and harness "([^"]+)"$"##
)]
fn when_context_changes(world: &mut GivnWorld, llm: String, reasoning: String, harness: String) {
    world
        .git_attribution
        .as_mut()
        .expect("attribution state")
        .context = Some(AttributionContext {
        llm: Some(llm),
        reasoning: Some(reasoning),
        harness: Some(harness),
    });
}

#[when("the consumer initializes Givn in the project")]
fn when_consumer_initializes_givn(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let output = common::run_givn(env, &["init", "--agents", "opencode,claude"]);
    assert!(
        output.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.last_output = Some(output);
}

#[then("the commit succeeds")]
fn then_commit_succeeds(world: &mut GivnWorld) {
    let output = world
        .git_attribution
        .as_ref()
        .and_then(|state| state.last_output.as_ref())
        .expect("commit output");
    assert!(
        output.status.success(),
        "commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(regex = r##"^the commit committer identifies "([^"]+)" and "([^"]+)"$"##)]
fn then_committer_identifies(world: &mut GivnWorld, llm: String, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%cn", "HEAD"]);
    assert!(output.status.success());
    let committer = String::from_utf8_lossy(&output.stdout);
    assert!(committer.contains(&format!("LLM: {llm} via {harness}")));
}

#[then("the commit message ends with the current Givn version, LLM, reasoning mode, and harness")]
fn then_message_has_complete_attribution(world: &mut GivnWorld) {
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

#[then("the commit message contains only the managed co-author attribution")]
fn then_message_has_only_coauthor_attribution(world: &mut GivnWorld) {
    then_message_has_complete_attribution(world);
}

#[then(regex = r##"^the commit message remains "([^"]+)"$"##)]
fn then_message_remains(world: &mut GivnWorld, expected: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
}

#[then(regex = r##"^the commit message identifies the harness as "([^"]+)"$"##)]
fn then_message_identifies_harness(world: &mut GivnWorld, harness: String) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!("@{harness} with Givn ")));
}

#[then("the commit message marks the LLM and reasoning mode as unavailable")]
fn then_message_marks_missing(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains("Co-Authored-By: unavailable/unavailable@"));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

#[then("the existing hook marker is present")]
fn then_existing_marker_present(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    assert_eq!(
        std::fs::read_to_string(root.join("existing-hook.marker")).expect("existing marker"),
        "marker"
    );
}

#[then("the commit message contains the Givn attribution trailers")]
fn then_message_contains_trailers(world: &mut GivnWorld) {
    let root = world.env.as_ref().expect("project root").root();
    let output = git(&root, &["show", "-s", "--format=%B", "HEAD"]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains("Co-Authored-By:"));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM-Harness:"));
}

#[then(regex = r##"^the first commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_first_commit_identifies(
    world: &mut GivnWorld,
    llm: String,
    reasoning: String,
    harness: String,
) {
    assert_commit_attribution(world, 0, &llm, &reasoning, &harness);
}

#[then(regex = r##"^the second commit identifies "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn then_second_commit_identifies(
    world: &mut GivnWorld,
    llm: String,
    reasoning: String,
    harness: String,
) {
    assert_commit_attribution(world, 1, &llm, &reasoning, &harness);
}

fn assert_commit_attribution(
    world: &mut GivnWorld,
    index: usize,
    llm: &str,
    reasoning: &str,
    harness: &str,
) {
    let root = world.env.as_ref().expect("project root").root();
    let hash = world
        .git_attribution
        .as_ref()
        .expect("attribution state")
        .commits
        .get(index)
        .expect("commit hash");
    let committer_output = git(&root, &["show", "-s", "--format=%cn", hash]);
    assert!(committer_output.status.success());
    let committer = String::from_utf8_lossy(&committer_output.stdout);
    assert!(committer.contains(&format!("LLM: {llm} via {harness}")));

    let output = git(&root, &["show", "-s", "--format=%B", hash]);
    assert!(output.status.success());
    let message = String::from_utf8_lossy(&output.stdout);
    assert!(message.contains(&format!(
        "Co-Authored-By: {llm}/{reasoning}@{harness} with Givn {}",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!message.contains("Givn-Version:"));
    assert!(!message.contains("LLM:"));
    assert!(!message.contains("LLM-Reasoning:"));
    assert!(!message.contains("LLM-Harness:"));
    assert!(!message.contains("Givn-Attribution-Status:"));
}

pub(crate) fn git(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git")
}

fn make_executable(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .expect("hook metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).expect("make hook executable");
    }
}

pub(crate) fn git_with_givn_path(
    root: &std::path::Path,
    args: &[&str],
    context: Option<&AttributionContext>,
) -> Output {
    let binary = PathBuf::from(env!("CARGO_BIN_EXE_givn"));
    let binary_dir = binary.parent().expect("givn binary directory");
    let mut paths = vec![binary_dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    let path = std::env::join_paths(paths).expect("join Git hook PATH");
    let mut command = Command::new("git");
    command
        .args(args)
        .current_dir(root)
        .env("PATH", path)
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS")
        .env_remove("GIT_COMMITTER_NAME")
        .env_remove("GIT_COMMITTER_EMAIL");
    if let Some(context) = context {
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
    command.output().expect("run git with givn")
}

#[given("an initialized Fossil project")]
fn given_initialized_fossil_project(world: &mut GivnWorld) {
    world.env = Some(common::make_fossil_root());
}

#[when(
    regex = r##"^the consumer commits with message "([^"]+)" attributing LLM "([^"]+)", reasoning "([^"]+)", and harness "([^"]+)"$"##
)]
fn when_givn_commit_with_context(
    world: &mut GivnWorld,
    message: String,
    llm: String,
    reasoning: String,
    harness: String,
) {
    let env = world.env.as_ref().expect("project root");
    std::fs::write(env.root().join("fixture.txt"), &message).expect("write fixture");
    let output = common::run_givn_with_envs(
        env,
        &["commit", "-m", &message],
        &[
            ("GIVN_ATTRIBUTION_ACTIVE", "1"),
            ("GIVN_ATTRIBUTION_LLM", &llm),
            ("GIVN_ATTRIBUTION_REASONING", &reasoning),
            ("GIVN_ATTRIBUTION_HARNESS", &harness),
        ],
    );
    world.last_output = Some(output);
}

#[when(regex = r##"^the consumer commits with message "([^"]+)" without attribution context$"##)]
fn when_givn_commit_without_context(world: &mut GivnWorld, message: String) {
    let env = world.env.as_ref().expect("project root");
    std::fs::write(env.root().join("fixture.txt"), &message).expect("write fixture");
    // Strip the harness attribution context explicitly: the outer suite may
    // run inside a harness that exports it.
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_givn"));
    command
        .args(["commit", "-m", &message])
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .env("FOSSIL_USER", "fixture")
        .env_remove("GIVN_ATTRIBUTION_ACTIVE")
        .env_remove("GIVN_ATTRIBUTION_LLM")
        .env_remove("GIVN_ATTRIBUTION_REASONING")
        .env_remove("GIVN_ATTRIBUTION_HARNESS");
    let output = command.output().expect("spawn givn");
    world.last_output = Some(output);
}

/// The message of the most recent revision, on either backend.
fn latest_revision_message(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("project root");
    match env.backend() {
        common::BackendKind::Git => {
            let out = git(&env.root(), &["log", "-1", "--format=%B"]);
            String::from_utf8_lossy(&out.stdout).into_owned()
        }
        common::BackendKind::Fossil => {
            crate::steps::archive_proof_support::fossil_latest_comment(env)
        }
    }
}

#[then("the revision message carries the managed attribution trailer")]
fn then_message_has_trailer(world: &mut GivnWorld) {
    let message = latest_revision_message(world);
    assert!(
        message.contains("Co-Authored-By:") && message.contains(" with Givn "),
        "the revision message carries no managed trailer: {message}"
    );
}

#[then("the revision message carries no managed attribution trailer")]
fn then_message_has_no_trailer(world: &mut GivnWorld) {
    let message = latest_revision_message(world);
    assert!(
        !message.contains(" with Givn "),
        "the revision message unexpectedly carries the managed trailer: {message}"
    );
}

#[when(regex = r#"^the consumer commits only "([^"]+)" with message "([^"]+)"$"#)]
fn when_givn_commit_named_path(world: &mut GivnWorld, path: String, message: String) {
    let env = world.env.as_ref().expect("project root");
    let output = common::run_givn(env, &["commit", "-m", &message, &path]);
    world.last_output = Some(output);
}

#[then(regex = r#"^the latest revision does not contain "([^"]+)"$"#)]
fn then_latest_revision_lacks(world: &mut GivnWorld, path: String) {
    assert!(
        !crate::steps::archive_proof_support::revision_contains(world, &path),
        "the latest revision unexpectedly contains {path}"
    );
}

/// One public-hook characterization case: a generated OpenCode plugin of one
/// generation plus the ordered actions replayed through its hooks.
#[derive(Debug)]
pub(crate) struct CharacterizationState {
    generation: String,
    /// Keeps the project that holds the generated plugin alive.
    _project: common::TestEnv,
    asset: PathBuf,
    initial_env: serde_json::Map<String, serde_json::Value>,
    actions: Vec<serde_json::Value>,
    outcome: Option<Result<Characterization, String>>,
}

/// The environment of the most recently created characterized shell.
pub(crate) fn characterized_shell_env(world: &GivnWorld) -> Option<&serde_json::Value> {
    let state = world.opencode_characterization.as_ref()?;
    match state.outcome.as_ref().expect("a shell was prepared") {
        Ok(result) => Some(result.shells.last().expect("a shell was created")),
        Err(error) => panic!("OpenCode shell preparation failed: {error}"),
    }
}

fn characterization(world: &mut GivnWorld) -> &mut CharacterizationState {
    world
        .opencode_characterization
        .as_mut()
        .expect("an isolated OpenCode attribution integration")
}

fn prototype_path() -> PathBuf {
    common::repo_path("tests/fixtures/opencode/prototype/givn-attribution.js")
}

/// The immutable prototype is an oracle only where it has the same observable
/// contract: the V1 hooks, and V2 commands that are not linked to a session
/// (the prototype has no session link and a different default-lookup shape).
fn prototype_is_oracle(state: &CharacterizationState) -> bool {
    state.generation == "V1"
        || !state.actions.iter().any(|action| {
            matches!(action["op"].as_str(), Some("request"))
                || (action["op"] == "run" && !action["session"].is_null())
                || (action["op"] == "default" && action["mode"] == "model")
        })
}

/// Replay the recorded actions through the generated plugin, and compare the
/// result with the immutable prototype wherever it is an oracle.
fn replay(world: &mut GivnWorld) {
    let state = characterization(world);
    let outcome = common::opencode_characterize::run(
        &state.asset,
        &state.generation,
        &state.initial_env,
        &state.actions,
    );
    if let Ok(result) = &outcome {
        if state.asset != prototype_path() && prototype_is_oracle(state) {
            let oracle = common::opencode_characterize::run(
                &prototype_path(),
                &state.generation,
                &state.initial_env,
                &state.actions,
            )
            .expect("the immutable prototype replays the same actions");
            assert_eq!(
                result.shells, oracle.shells,
                "generated plugin differs from the supplied prototype's observable behavior"
            );
        }
    }
    state.outcome = Some(outcome.map_err(|error| format!("{error:#}")));
}

#[given(regex = r##"^an isolated OpenCode "([^"]+)" attribution integration$"##)]
fn given_isolated_opencode_integration(world: &mut GivnWorld, generation: String) {
    assert!(
        matches!(generation.as_str(), "V1" | "V2"),
        "unsupported OpenCode generation: {generation}"
    );
    let env = common::make_bare_root();
    let init = common::run_givn(&env, &["init", "--agents", "opencode", "--no-addons"]);
    assert!(
        init.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    std::fs::write(env.path("package.json"), r#"{"type":"module"}"#)
        .expect("write JavaScript module configuration");
    let generated = env.path(".opencode/plugins/givn-attribution.js");
    // An alternative module may stand in for the generated plugin, to show a
    // scenario fails against it (for example the immutable prototype).
    let alternative = std::env::var_os("GIVN_OPENCODE_CHARACTERIZE_ASSET").map(PathBuf::from);
    if alternative.is_none() {
        assert_eq!(
            std::fs::read(&generated).expect("generated OpenCode plugin"),
            include_bytes!("../../assets/agent-integrations/opencode/givn-attribution.js"),
            "the generated plugin must be the embedded asset"
        );
    }
    let state = CharacterizationState {
        generation,
        asset: alternative.unwrap_or(generated),
        _project: env,
        initial_env: serde_json::Map::new(),
        actions: Vec::new(),
        outcome: None,
    };
    let shape = common::opencode_characterize::run(
        &state.asset,
        &state.generation,
        &state.initial_env,
        &[],
    )
    .expect("the generated plugin loads through its public entrypoint");
    if state.asset != prototype_path()
        && std::env::var_os("GIVN_OPENCODE_CHARACTERIZE_ASSET").is_none()
    {
        assert_eq!(shape.module["id"], "givn-attribution");
        assert_eq!(shape.module["server"], "function");
        assert_eq!(shape.module["setup"], "function");
        let expected_hooks = if state.generation == "V1" {
            serde_json::json!({"v1": ["chat.message", "chat.params", "shell.env"]})
        } else {
            serde_json::json!({"v2": {
                "session": ["compaction", "context", "generate"],
                "tool": ["execute.after", "execute.before"],
                "shell": ["create.before"],
            }})
        };
        assert_eq!(shape.hooks, expected_hooks, "registered hooks");
    }
    world.opencode_characterization = Some(state);
}

#[when(
    regex = r##"^OpenCode reports provider "([^"]+)" with model "([^"]+)" using identifier "([^"]+)" for session "([^"]+)"$"##
)]
fn when_opencode_reports_identifier(
    world: &mut GivnWorld,
    provider: String,
    model: String,
    identifier: String,
    session: String,
) {
    assert!(
        matches!(identifier.as_str(), "id" | "modelID"),
        "unsupported identifier spelling: {identifier}"
    );
    characterization(world).actions.push(serde_json::json!({
        "op": "remember",
        "session": session,
        "provider": provider,
        "model": model,
        "identifier": identifier,
    }));
}

#[when(regex = r##"^OpenCode prepares an attributed shell for session "([^"]+)"$"##)]
fn when_opencode_prepares_shell_for_session(world: &mut GivnWorld, session: String) {
    characterization(world).actions.push(serde_json::json!({
        "op": "run",
        "session": session,
        "command": "git commit",
    }));
    replay(world);
}

/// Split `provider/model` at the first slash.
fn split_model(identity: &str) -> (&str, &str) {
    identity
        .split_once('/')
        .unwrap_or_else(|| panic!("model identity lacks a provider: {identity}"))
}

#[given(
    regex = r##"^OpenCode has (?:later )?remembered model "([^"]+)" with reasoning mode "([^"]+)" for session "([^"]+)"$"##
)]
fn given_opencode_remembered_model(
    world: &mut GivnWorld,
    model: String,
    reasoning: String,
    session: String,
) {
    let (provider, model) = split_model(&model);
    characterization(world).actions.push(serde_json::json!({
        "op": "remember",
        "session": session,
        "provider": provider,
        "model": model,
        "variant": reasoning,
    }));
}

#[when(
    regex = r##"^OpenCode reports model "([^"]+)" without reasoning metadata for session "([^"]+)"$"##
)]
fn when_opencode_reports_model_without_reasoning(
    world: &mut GivnWorld,
    model: String,
    session: String,
) {
    let (provider, model) = split_model(&model);
    characterization(world).actions.push(serde_json::json!({
        "op": "remember",
        "session": session,
        "provider": provider,
        "model": model,
    }));
}

/// A shell the plugin cannot link to a remembered session. V1 hands its shell
/// hook a session identifier, which may be unknown; V2 hands none, because
/// the tool hooks never announced a matching request.
#[when(regex = r##"^OpenCode prepares an attributed shell with "([^"]+)"$"##)]
fn when_opencode_prepares_shell_with_context(world: &mut GivnWorld, context: String) {
    let state = characterization(world);
    let session = match (context.as_str(), state.generation.as_str()) {
        ("unknown session", "V1") => serde_json::json!("unknown-session"),
        ("unknown session" | "no session", _) => serde_json::Value::Null,
        _ => panic!("unsupported session context: {context}"),
    };
    state.actions.push(serde_json::json!({
        "op": "run",
        "session": session,
        "command": "git commit",
    }));
    replay(world);
}
