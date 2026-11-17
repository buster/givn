//! The `@e2e` package's fixture harness.
//!
//! Black-box seam: every scenario drives the built `givn` binary (resolved
//! from `GIVN_BIN`) as a subprocess with a per-scenario `tempfile::TempDir` as
//! the current directory. The package links no product crate, so nothing here
//! may import `givn`.
//!
//! The `TempDir` is held by the returned `TestEnv` so it lives as long as the
//! scenario needs it; it is removed automatically on drop, including on panic.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

pub mod binary;
pub mod browser;
pub mod history_record;
pub mod proof_fixture;
pub mod terminal_evidence;
pub mod timing;

/// The version-control fixture the project root emulates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Git,
    Fossil,
}

/// Handle carrying the per-test temp directory. Drop removes the directory.
#[derive(Debug)]
pub struct TestEnv {
    dir: TempDir,
    /// Repository file for a Fossil fixture; kept outside the checkout so it
    /// is never an `EXTRA` entry.
    fossil_repo: Option<TempDir>,
    backend: BackendKind,
}

impl TestEnv {
    /// Absolute path to the temp directory (the project root for `givn`).
    pub fn root(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    /// Join a relative path under the temp root.
    pub fn path(&self, rel: &str) -> PathBuf {
        self.root().join(rel)
    }

    /// The version-control fixture this environment emulates.
    pub fn backend(&self) -> BackendKind {
        self.backend
    }

    /// A private Fossil config home. Parallel scenarios otherwise share
    /// `~/.fossil`, and its SQLite database locks under concurrent
    /// `fossil init` calls.
    pub fn fossil_home(&self) -> PathBuf {
        let home = match &self.fossil_repo {
            Some(repo) => repo.path().join("home"),
            None => std::env::temp_dir().join(format!("givn-fossil-home-{}", std::process::id())),
        };
        std::fs::create_dir_all(&home).expect("create the Fossil config home");
        home
    }
}

/// A bare temp directory with no `givn/` subtree — for the uninitialised
/// project scenario. Do NOT run `givn init` here.
pub fn make_bare_root() -> TestEnv {
    let dir = TempDir::new().expect("create bare temp dir");
    TestEnv {
        dir,
        fossil_repo: None,
        backend: BackendKind::Git,
    }
}

/// The default fixture: a temp directory with `givn init` already run, so
/// `givn/specs`, `givn/changes`, `givn/archive`, config, and skills are
/// scaffolded exactly as a real initialised project (no duplicated logic).
/// The README is written in its complete, givn-managed form — a real project
/// that archives has filled its scaffold sections, and the README
/// completeness gate would otherwise block every archive-flow fixture. The
/// commands file likewise carries a complete run declaration — a real
/// project that archives has filled in the scaffolded `run:` block, and the
/// run-declaration gate would otherwise block every archive-flow fixture.
/// Scenario-specific states override via the shared declaration Givens
/// (`declare_run`/`strip_run` in `run_declaration_fixtures_support`).
pub fn make_init_root() -> TestEnv {
    let env = make_init_root_without_git();
    git_init(&env);
    env
}

/// An initialized fixture whose project root is a real Fossil checkout.
pub fn make_fossil_root() -> TestEnv {
    let mut env = make_init_root_without_git();
    env.backend = BackendKind::Fossil;
    env.fossil_repo = Some(TempDir::new().expect("create fossil repo temp dir"));
    fossil_init(&env);
    env
}

/// The default fixture without the final Git initialization — for scenarios
/// that need a non-Git project or a repository one level above the project.
pub fn make_init_root_without_git() -> TestEnv {
    let env = make_bare_root();
    let out = run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(
        out.status.success(),
        "givn init failed in temp dir: stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    std::fs::write(env.path("README.md"), complete_readme_fixture())
        .expect("write complete README fixture");
    let commands = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&commands).expect("read commands.yaml");
    let updated = replace_top_level_key(&content, "run", &complete_run_declaration("cli"));
    std::fs::write(&commands, updated).expect("write commands.yaml");
    env
}

/// Write a fixture `givn/commands.yaml`, appending a complete run
/// declaration unless the content already carries a top-level `run:` key.
/// A real project at any workflow stage that reaches the review or archive
/// gates has filled in its run scaffold; fixtures mimic that state.
pub(crate) fn write_commands_yaml(path: &std::path::Path, yaml: &str) {
    let has_run = yaml.lines().any(is_top_level_key_named_run);
    let content = if has_run {
        yaml.to_string()
    } else {
        format!("{}\n{}", yaml, complete_run_declaration("cli"))
    };
    fs::write(path, content).expect("write commands.yaml");
}

fn is_top_level_key_named_run(line: &str) -> bool {
    line.starts_with("run:")
        && !line.starts_with(' ')
        && !line.starts_with('\t')
        && !line.starts_with('#')
}

/// The run declaration block for a product type, as YAML text. The commands
/// are never executed by the gates — validation only reads them.
pub(crate) fn complete_run_declaration(product_type: &str) -> String {
    match product_type {
        "server" => "\
run:
  command: \"./serve-fixture.sh\"
  product_type: server
  port_env: \"PORT\"
  ready_path: \"/healthz\"
  ready_timeout_s: 60
"
        .to_string(),
        "cli" => "\
run:
  command: \"./fixture-start.sh\"
  product_type: cli
"
        .to_string(),
        "library" => "\
run:
  command: \"./build-lib.sh\"
  product_type: library
"
        .to_string(),
        other => panic!("unknown fixture product type: {other}"),
    }
}

/// Replace the block of lines belonging to a top-level key with new text.
/// A top-level key is a line that starts in column 0 and begins the key
/// name. Comment lines and blanks inside the block belong to it.
pub(crate) fn replace_top_level_key(content: &str, key: &str, replacement: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut out: Vec<&str> = Vec::new();
    let mut replaced = false;
    let mut in_block = false;
    for line in &lines {
        if is_top_level_key(line) {
            let name = line.split(':').next().unwrap_or(line).trim();
            if name == key {
                if !replaced {
                    out.extend(replacement.lines());
                    replaced = true;
                }
                in_block = true;
                continue;
            }
            in_block = false;
        }
        if !in_block {
            out.push(line);
        }
    }
    if !replaced {
        out.extend(replacement.lines());
    }
    let mut result = out.join("\n");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

/// Remove the block of lines belonging to a top-level key, including any
/// blank line that directly precedes it.
pub(crate) fn remove_top_level_key(content: &str, key: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut out: Vec<&str> = Vec::new();
    let mut in_block = false;
    for line in &lines {
        if is_top_level_key(line) {
            let name = line.split(':').next().unwrap_or(line).trim();
            in_block = name == key;
            if in_block {
                // Drop a blank line directly before the removed block.
                if out.last().is_some_and(|l| l.trim().is_empty()) {
                    out.pop();
                }
                continue;
            }
        }
        if !in_block {
            out.push(line);
        }
    }
    let mut result = out.join("\n");
    if !result.ends_with('\n') && !result.is_empty() {
        result.push('\n');
    }
    result
}

/// True when the line starts a top-level YAML mapping entry (column 0,
/// not a comment, non-empty).
fn is_top_level_key(line: &str) -> bool {
    !line.is_empty()
        && !line.starts_with(' ')
        && !line.starts_with('\t')
        && !line.starts_with('#')
        && line.contains(':')
}

/// A complete, givn-managed README: filled sections, badge fences, and an
/// intact development managed block.
fn complete_readme_fixture() -> String {
    "\
# test-project

<!-- givn:begin:coverage-badge -->
[![Line Coverage: n/a](https://img.shields.io/badge/line%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

A small command-line tool used as a givn test fixture.

## Quick start

```sh
cargo install test-project
test-project --help
```

## Prerequisites

Rust (latest stable).

## Usage

`test-project <args>` prints the requested output to stdout.

## Contributing

Conventional Commits; PRs require a green test suite.

## License

MIT

<!-- givn:begin:development -->
## Development

Run the project-owned scripts used by `givn/commands.yaml`:

```sh
./run-tests.sh
./run-tests.sh --e2e
```
<!-- givn:end:development -->
"
    .to_string()
}

/// Invoke the `givn` binary with `env`'s temp dir as current directory and
/// the given args, returning captured stdout/stderr/exit code.
pub fn run_givn(env: &TestEnv, args: &[&str]) -> Output {
    run_givn_with_env(env, args, None)
}

/// Environment variables a fixture child needs, and nothing else.
///
/// `env_clear()` keeps the suite's runner-scoped variables (`GIVN_FEATURES`,
/// `GIVN_RESULT_FILE`, `GIVN_RESULT_SCOPE`, `GIVN_CUCUMBER_TAGS`) out of every
/// fixture project. The four forwarded variables are the ones a child cannot
/// work without: `PATH` for the tools a scenario runs, `HOME` for the tools
/// that read a per-user config, `LLVM_PROFILE_FILE` so the coverage scope still
/// records the instrumented binary, and `FOSSIL_USER` because Fossil refuses a
/// commit without an author while Git fixtures ignore it.
fn fixture_env(command: &mut Command) {
    command.env_clear();
    for key in ["PATH", "HOME", "LLVM_PROFILE_FILE"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command.env("FOSSIL_USER", "fixture");
}

/// The compiled binary in a project root, before any fixture-specific
/// environment is added.
fn givn_command(project_root: &std::path::Path) -> Command {
    let mut command = Command::new(binary::givn());
    command.current_dir(project_root);
    fixture_env(&mut command);
    command
}

/// Resolve a repository-relative path independent of the process cwd.
///
/// The in-process `givn run` scenario temporarily changes the process-wide
/// current directory, and the single-process runner (`cargo test`) shares one
/// process across scenarios; anchoring repo-relative reads at the manifest
/// directory removes that race.
pub fn repo_path(relative: &str) -> std::path::PathBuf {
    binary::product_root().join(relative)
}

/// Invoke the `givn` binary with one optional environment override.
pub fn run_givn_with_env(
    env: &TestEnv,
    args: &[&str],
    environment: Option<(&str, &str)>,
) -> Output {
    let environments = environment.into_iter().collect::<Vec<_>>();
    run_givn_with_envs(env, args, &environments)
}

/// Invoke the `givn` binary with multiple optional environment overrides.
pub fn run_givn_with_envs(env: &TestEnv, args: &[&str], environments: &[(&str, &str)]) -> Output {
    run_givn_with_envs_in(env, &env.root(), args, environments)
}

/// Invoke the `givn` binary from a specific working directory inside the
/// fixture (nested-project scenarios).
pub fn run_givn_with_envs_in(
    env: &TestEnv,
    cwd: &std::path::Path,
    args: &[&str],
    environments: &[(&str, &str)],
) -> Output {
    let mut command = givn_command(cwd);
    command.args(args);
    if env.backend == BackendKind::Fossil {
        command.env("FOSSIL_HOME", env.fossil_home());
    }
    for (key, value) in environments {
        command.env(key, value);
    }
    command.output().expect("spawn givn subprocess")
}

/// Invoke the `givn` binary with `env`'s temp dir as current directory,
/// one stdin line written (`answer`), then stdin closed. Simulates an
/// interactive answer to a prompt without a terminal.
pub fn run_givn_with_stdin(env: &TestEnv, args: &[&str], answer: &str) -> Output {
    run_givn_with_stdin_and_envs(env, args, answer, &[])
}

/// Invoke the `givn` binary with piped stdin and environment overrides.
pub fn run_givn_with_stdin_and_envs(
    env: &TestEnv,
    args: &[&str],
    input: &str,
    environments: &[(&str, &str)],
) -> Output {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new(binary::givn());
    child
        .args(args)
        .current_dir(env.root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    fixture_env(&mut child);
    child.envs(environments.iter().copied());
    if env.backend == BackendKind::Fossil {
        child.env("FOSSIL_HOME", env.fossil_home());
    }
    let mut child = child
        .spawn()
        .expect("spawn givn subprocess with piped stdin");

    {
        let mut stdin = child.stdin.take().expect("piped stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("write answer to givn stdin");
    }

    child.wait_with_output().expect("wait for givn subprocess")
}

/// Drive `givn init`'s interactive prompt through a real PTY.
///
/// `keys` is the raw byte sequence sent to the terminal. util-linux `script`
/// gives the child a terminal on stdin, so the dialoguer checkbox list runs;
/// the transcript becomes the command output for exit-code and stderr
/// assertions. `AgentTarget::ALL[0]` is `.opencode` and the only pre-marked
/// entry, so one space (`\x20`) toggles exactly that default off.
///
/// Raw-key semantics, the 60-second deadline and the propagated child exit
/// status are preserved for every caller.
pub fn run_init_in_terminal(env: &TestEnv, keys: &[u8]) -> Output {
    use std::io::Write;
    use std::process::{Command, Output, Stdio};
    use std::time::{Duration, Instant};

    let transcript = env.path(".terminal-transcript.txt");
    let bin = binary::givn();
    let mut child = Command::new("script")
        .args([
            "-qec",
            &format!("{} init", bin.display()),
            &transcript.to_string_lossy(),
        ])
        .current_dir(env.root())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn `script` (util-linux) to drive the terminal prompt");
    child
        .stdin
        .take()
        .expect("piped stdin for the PTY session")
        .write_all(keys)
        .expect("write the terminal keys");
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
    // The terminal transcript is the command's observable output: prompt,
    // banner, and messages all write to the same PTY.
    Output {
        status,
        stdout: bytes.clone(),
        stderr: bytes,
    }
}

/// Remove change-local add-on locks for fixtures that intentionally change
/// add-on configuration after creating a change. The lock scenarios use
/// `givn new` directly and do not call this helper.
pub fn remove_addon_locks(env: &TestEnv) {
    let changes = env.path("givn/changes");
    let Ok(entries) = fs::read_dir(&changes) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let _ = fs::remove_file(entry.path().join("config-lock.yaml"));
        }
    }
}

/// Replace the project's addon policy in `givn/config.yaml`.
///
/// Addon flags are policy: `givn/commands.yaml` rejects them. Fixtures that
/// need a specific addon set call this instead of writing an `addons:` block
/// beside the command settings.
pub fn replace_config_addons(env: &TestEnv, addons: &str) {
    remove_addon_locks(env);
    let path = env.path("givn/config.yaml");
    let content = fs::read_to_string(&path).expect("read givn/config.yaml");
    let updated = replace_top_level_key(&content, "addons", addons);
    fs::write(path, updated).expect("write givn/config.yaml");
}

/// Drop a fixture's explicit no-behavior-change declaration once the scenario
/// writes executable delta features.
pub fn clear_no_behavior_fixture(env: &TestEnv, change: &str) {
    let change_dir = env.path(&format!("givn/changes/{change}"));
    let proposal = change_dir.join("proposal.md");
    if let Ok(content) = fs::read_to_string(&proposal) {
        let filtered: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim_start().starts_with("Behavior change:"))
            .collect();
        fs::write(&proposal, format!("{}\n", filtered.join("\n").trim_end()))
            .expect("rewrite fixture proposal");
    }
    let _ = fs::remove_file(change_dir.join(".givn-skip"));
}

/// The literal the product writes into `.fossil-settings/ignore-glob` for the
/// record store. Copied from the product's own constant: the fixture must
/// produce the same checkout shape, and the value is a file-format literal,
/// not behaviour.
const FOSSIL_IGNORE_ENTRY: &str = "givn/history/*";

/// The permanent use-case that owns a capability: the directory under
/// `givn/specs` holding `<capability>.feature`. Several owners are the
/// product's ambiguity error; a fixture writes one, so the first sorted
/// directory wins and the ambiguity check stays the product's.
pub fn resolve_capability_owner(
    specs_root: &std::path::Path,
    capability: &str,
) -> Option<std::path::PathBuf> {
    let mut owners: Vec<_> = fs::read_dir(specs_root)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.join(format!("{capability}.feature")).is_file())
        .collect();
    owners.sort();
    owners.into_iter().next()
}

/// The use-case narrative a fixture writes when its delta introduces a new
/// capability. It is the product template's shape as a literal: the product
/// parses it, so the package must not import the product's renderer.
pub fn usecase_md_template() -> String {
    r#"# Use case: <usecase-id>

## Level

!

## Actors

- <actor role>

## Goal

<the domain goal>

## Trigger

<what starts this use case>

## Preconditions

- <precondition that holds before the trigger>

## Main flow

1. <actor intention>; System responsibility: <what the system does>

## Extensions

- 1: <condition> → <handling>; protects the minimal guarantee

## Rules

- <domain rule>

## Examples

- <concrete example>

## Minimal guarantee

<what remains true when the goal cannot be completed>

## Success guarantee

<what is true when the goal succeeds>

## Personas

- none

## Capabilities

- <capability-id>

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| <capability-id> | <consumer action> | <scenario title> |

## Includes

- none

## Extends

- none

## Out of scope

<what this use case deliberately does not cover>

## Diagram

```mermaid
flowchart LR
  Actor((Actor)) --> UC1[[Use case]]
```
"#
    .to_string()
}

/// The fragment narrative a fixture writes for a fragment-owned capability.
pub fn fragment_md_template() -> String {
    r#"# Fragment: <fragment-id>

## Capabilities

- <capability-id>

## Interactions

- none

## Diagram

```mermaid
flowchart LR
  Consumer((Consumer)) --> Fragment[[Fragment]]
```
"#
    .to_string()
}

/// The agent targets the product supports, in its canonical order.
///
/// A file-format literal: a scenario that asks for "all targets" must name the
/// same set the product's own `AgentTarget::ALL` names, and a new target must
/// be added in both places.
pub const AGENT_TARGETS: [&str; 5] = ["opencode", "agents", "claude", "omp", "maki"];

/// The skills targets the fixture's `givn/config.yaml` selects.
///
/// Reads the written configuration with the package's YAML parser; the default
/// when the file names none is `opencode`, as the product's own reader
/// defaults.
pub fn skills_targets(project_root: &std::path::Path) -> Result<Vec<String>, String> {
    #[derive(serde::Deserialize)]
    struct Doc {
        #[serde(default)]
        skills: Option<Skills>,
    }
    #[derive(serde::Deserialize)]
    struct Skills {
        #[serde(default)]
        targets: Vec<String>,
    }
    let path = project_root.join("givn/config.yaml");
    let mut targets: Vec<String> = Vec::new();
    if path.exists() {
        let yaml = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let doc: Doc = serde_yaml_ng::from_str(&yaml)
            .map_err(|error| format!("failed to parse givn/config.yaml: {error}"))?;
        if let Some(skills) = doc.skills {
            for raw in skills.targets {
                let name = raw.trim().trim_start_matches('.').to_ascii_lowercase();
                if AGENT_TARGETS.contains(&name.as_str()) && !targets.contains(&name) {
                    targets.push(name);
                }
            }
        }
    }
    if targets.is_empty() {
        targets.push("opencode".to_string());
    }
    Ok(targets)
}

/// The addon flags the fixture's `givn/config.yaml` declares.
///
/// Reads the written configuration: a scenario that asserts an effective addon
/// state asserts the file the product rewrote, not an in-process overlay.
pub fn addons(project_root: &std::path::Path) -> std::collections::BTreeMap<String, bool> {
    #[derive(serde::Deserialize)]
    struct Doc {
        #[serde(default)]
        addons: Option<std::collections::BTreeMap<String, bool>>,
    }
    let path = project_root.join("givn/config.yaml");
    let text = fs::read_to_string(&path).unwrap_or_default();
    let doc: Doc = serde_yaml_ng::from_str(&text).unwrap_or_else(|error| {
        panic!("parse {}: {error}", path.display());
    });
    doc.addons.unwrap_or_default()
}

/// The current Fossil checkout's 40-hex revision.
pub fn fossil_revision(env: &TestEnv) -> String {
    let out = fossil(env, &["info"]);
    assert!(
        out.status.success(),
        "fossil info failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("checkout:"))
        .and_then(|rest| rest.split_whitespace().next())
        .map(str::to_string)
        .expect("the Fossil checkout has a current revision")
}

/// The parent revision of a Fossil check-in, when it has one.
pub fn fossil_parent_revision(env: &TestEnv, revision: &str) -> Option<String> {
    let out = fossil(env, &["info", revision]);
    if !out.status.success() {
        return None;
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|line| line.trim().strip_prefix("parent:"))
        .and_then(|rest| rest.split_whitespace().next())
        .map(str::to_string)
}

/// Declare a routing decision for every capability the change's delta
/// actually declares.
///
/// The archive/review routing gate is fail-closed: a delta capability without
/// a filled `## Capability Routing` row blocks the command. Fixtures model an
/// authored proposal by recording `EXTEND <capability>` when the permanent
/// corpus already owns the capability and `NEW in <usecase>` otherwise. A new
/// capability also needs its change-local use-case narrative, so one is
/// scaffolded from the shipped template when the fixture did not write it.
pub fn declare_fixture_routing(env: &TestEnv, change: &str) {
    let change_dir = env.path(&format!("givn/changes/{change}"));
    let specs = change_dir.join("specs");
    let mut capabilities = std::collections::BTreeMap::new();
    collect_capability_names(&specs, &specs, &mut capabilities);
    if capabilities.is_empty() {
        return;
    }
    let permanent = env.path("givn/specs");
    let mut rows = Vec::new();
    for (usecase, capability) in capabilities {
        let owned = resolve_capability_owner(&permanent, &capability);
        let decision = match (&usecase, owned) {
            (_, Some(_)) => format!("`EXTEND {capability}`"),
            (Some(usecase), None) => {
                write_fixture_usecase(&specs, usecase, &capability);
                format!("`NEW in {usecase}`")
            }
            (None, None) => format!("`NEW in {capability}`"),
        };
        rows.push(format!(
            "| {capability} | (fixture) | {decision} | fixture |"
        ));
    }
    let table = format!(
        "## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n{}\n",
        rows.join("\n")
    );
    let proposal_path = change_dir.join("proposal.md");
    let proposal = fs::read_to_string(&proposal_path).unwrap_or_default();
    let updated = replace_section(&proposal, "## Capability Routing", &table);
    fs::write(proposal_path, updated).expect("write fixture proposal routing");
}

fn write_fixture_usecase(specs: &std::path::Path, usecase: &str, capability: &str) {
    let is_fragment = usecase == "fragments";
    let path = specs.join(usecase).join(if is_fragment {
        "fragment.md"
    } else {
        "usecase.md"
    });
    if path.exists() {
        return;
    }
    let feature_path = specs.join(usecase).join(format!("{capability}.feature"));
    let e2e_titles: Vec<String> =
        gherkin::Feature::parse_path(&feature_path, gherkin::GherkinEnv::default())
            .ok()
            .map(|feature| {
                let inherited = feature.tags.iter().any(|tag| tag == "e2e");
                feature
                    .scenarios
                    .iter()
                    .filter(|scenario| inherited || scenario.tags.iter().any(|tag| tag == "e2e"))
                    .map(|scenario| scenario.name.clone())
                    .collect()
            })
            .unwrap_or_default();
    let first = e2e_titles
        .first()
        .map(String::as_str)
        .unwrap_or("No E2E interaction");
    let table = "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n";
    let rows = e2e_titles
        .iter()
        .map(|title| format!("| {capability} | {capability} | {title} |\n"))
        .collect::<String>();
    let mut document = if is_fragment {
        fragment_md_template()
            .replace("<fragment-id>", usecase)
            .replace("<capability-id>", capability)
    } else {
        usecase_md_template()
            .replace("<usecase-id>", usecase)
            .replace("<capability-id>", capability)
            .replace("<actor role>", "Fixture operator")
            .replace("<the domain goal>", "Exercise the fixture behavior.")
            .replace("<consumer action>", capability)
            .replace("<scenario title>", first)
    };
    if is_fragment {
        if !e2e_titles.is_empty() {
            document = document.replace("- none\n", &format!("{table}{rows}"));
        }
    } else {
        let original_row = format!("| {capability} | {capability} | {first} |\n");
        if e2e_titles.is_empty() {
            document = document.replace(&format!("{table}{original_row}"), "- none\n");
        } else {
            document =
                document.replace(&format!("{table}{original_row}"), &format!("{table}{rows}"));
        }
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create fixture usecase dir");
    }
    fs::write(path, document).expect("write fixture usecase");
}

fn collect_capability_names(
    dir: &std::path::Path,
    specs_root: &std::path::Path,
    capabilities: &mut std::collections::BTreeMap<Option<String>, String>,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_capability_names(&path, specs_root, capabilities);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "feature")
        {
            let declared = gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default())
                .ok()
                .and_then(|feature| {
                    feature
                        .tags
                        .iter()
                        .find(|tag| !tag.starts_with("givn."))
                        .cloned()
                });
            let capability = declared.or_else(|| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_string)
            });
            let Some(capability) = capability else {
                continue;
            };
            let usecase = path
                .parent()
                .filter(|parent| *parent != specs_root)
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .map(str::to_string);
            capabilities.insert(usecase, capability);
        }
    }
}

/// Replace a level-two Markdown section, or append it when absent.
fn replace_section(document: &str, heading: &str, replacement: &str) -> String {
    if let Some(start) = document.find(heading) {
        let rest = &document[start + heading.len()..];
        let end = rest
            .find("\n## ")
            .map(|offset| start + heading.len() + offset + 1)
            .unwrap_or(document.len());
        let mut updated = String::new();
        updated.push_str(&document[..start]);
        updated.push_str(replacement);
        updated.push_str(&document[end..]);
        return updated;
    }
    let mut updated = document.to_string();
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    if !updated.is_empty() {
        updated.push('\n');
    }
    updated.push_str(replacement);
    updated
}

/// Add a minimal passing review record to a change when the fixture does not
/// write one itself.
///
/// The review artifact requires an actual verdict (`REVIEW: PASS`) and, when
/// the readme addon is enabled, a README impact decision. Fixtures that
/// exercise a later gate still model a project that completed the review.
pub fn ensure_review_evidence(env: &TestEnv, change: &str, readme_addon: bool) {
    let path = env.path(&format!("givn/changes/{change}/review.md"));
    let mut content = fs::read_to_string(&path).unwrap_or_default();
    if !content.contains("REVIEW:") && !content.contains("VERDICT:") {
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str("\nREVIEW: PASS\n");
    }
    if readme_addon && !content.contains("README-IMPACT:") {
        content = format!("README-IMPACT: none\n\n{content}");
    }
    fs::write(path, content).expect("write review.md evidence");
}

/// Write a change delta with one added scenario and its routing decision.
///
/// A fixture delta must leave the merged corpus consistent. When the
/// capability has no permanent owner, the fixture first creates one so the
/// change is an `EXTEND` (a `NEW in` capability without an `@e2e` scenario
/// has no consistent interaction row). The permanent narrative always
/// declares the permanent feature's actual `@e2e` titles, or `- none`.
pub fn write_fixture_delta(env: &TestEnv, change: &str, capability: &str, scenario: &str) {
    let permanent = env.path("givn/specs");
    let owner = resolve_capability_owner(&permanent, capability);
    let usecase = match owner {
        Some(dir) => dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(capability)
            .to_string(),
        None => {
            let dir = permanent.join(capability);
            fs::create_dir_all(&dir).expect("create fixture permanent dir");
            fs::write(
                dir.join(format!("{capability}.feature")),
                format!(
                    "Feature: {capability}\n\n  Scenario: Existing {capability} behavior\n    Given a fixture precondition\n"
                ),
            )
            .expect("write fixture permanent feature");
            capability.to_string()
        }
    };
    write_fixture_usecase(&permanent, &usecase, capability);
    let dir = env.path(&format!("givn/changes/{change}/specs/{usecase}"));
    fs::create_dir_all(&dir).expect("create fixture delta dir");
    let feature = format!(
        "@givn.delta @{capability}\nFeature: {capability}\n\n  @givn.added\n  Scenario: {scenario}\n    Given a fixture precondition\n    When the fixture action runs\n    Then the fixture result is observed\n"
    );
    fs::write(dir.join(format!("{capability}.feature")), feature).expect("write fixture delta");
    clear_no_behavior_fixture(env, change);
    declare_fixture_routing(env, change);
}

/// Write a coverage measurement script that publishes two distinct reports.
///
/// The archive coverage gate removes stale reports before running the
/// measurement, so a fixture that archives with the coverage addon enabled
/// must produce the reports during that run. Returns the configured command.
pub fn write_fixture_coverage_measure(env: &TestEnv, non_e2e: &str, e2e: &str) -> String {
    let dir = env.path("coverage");
    fs::create_dir_all(&dir).expect("create coverage dir");
    fs::write(dir.join("fixture-non-e2e.xml"), non_e2e).expect("write fixture non-e2e report");
    fs::write(dir.join("fixture-e2e.xml"), e2e).expect("write fixture e2e report");
    let script = "#!/bin/sh\n\
                  set -eu\n\
                  mkdir -p \"$(dirname \"$GIVN_COVERAGE_NON_E2E_PATH\")\" \"$(dirname \"$GIVN_COVERAGE_E2E_PATH\")\"\n\
                  cp coverage/fixture-non-e2e.xml \"$GIVN_COVERAGE_NON_E2E_PATH\"\n\
                  cp coverage/fixture-e2e.xml \"$GIVN_COVERAGE_E2E_PATH\"\n";
    let path = env.path("measure-coverage.sh");
    fs::write(&path, script).expect("write measure script");
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
        .expect("make measure script executable");
    "./measure-coverage.sh".to_string()
}

/// Write a project override at `givn/artifacts/<artifact>/<file>` inside the
/// temp root. `givn init` does not scaffold `givn/artifacts/` (SMELL-3), so
/// this helper `create_dir_all`s the subtree itself.
pub fn write_override(env: &TestEnv, artifact: &str, file: &str, content: &str) {
    let dir = env.path(&format!("givn/artifacts/{artifact}"));
    fs::create_dir_all(&dir).expect("create artifacts override dir");
    fs::write(dir.join(file), content).expect("write override file");
}

// ─── Git fixtures ──────────────────────────────────────────────────────────

/// Run a Git command in the fixture root.
pub fn git(env: &TestEnv, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("spawn git subprocess")
}

/// Whether the fixture has a Git worktree.
pub fn is_git_repo(env: &TestEnv) -> bool {
    env.path(".git").exists()
}

/// Initialize the fixture as a Git repository with a deterministic identity
/// and one commit of the current tree.
pub fn git_init(env: &TestEnv) {
    let init = git(env, &["init", "-q"]);
    assert!(
        init.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    for args in [
        ["config", "user.email", "fixture@example.com"],
        ["config", "user.name", "Givn Fixture"],
        ["config", "commit.gpgsign", "false"],
    ] {
        let out = git(env, &args);
        assert!(
            out.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    git_commit_all(env, "fixture: initial state");
}

/// Stage everything and commit. Used to model a developer who committed the
/// fixture setup before running a gate.
pub fn git_commit_all(env: &TestEnv, message: &str) {
    let add = git(env, &["add", "-A"]);
    assert!(
        add.status.success(),
        "git add failed: {}",
        String::from_utf8_lossy(&add.stderr)
    );
    let commit = git(env, &["commit", "-q", "-m", message, "--allow-empty"]);
    assert!(
        commit.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
}

/// Commit any pending fixture writes when the project is a Git worktree.
/// The archive starts from a committed state, so generic fixtures emulate
/// the developer's commit; dirty-tree scenarios invoke `givn` directly.
pub fn commit_fixture(env: &TestEnv) {
    match env.backend {
        BackendKind::Git => {
            if is_git_repo(env) {
                git_commit_all(env, "fixture: archive input");
            }
        }
        BackendKind::Fossil => fossil_commit_all(env, "fixture: archive input"),
    }
}

// ─── Fossil fixtures ───────────────────────────────────────────────────────

/// Run a Fossil command in the fixture root with a deterministic user.
pub fn fossil(env: &TestEnv, args: &[&str]) -> Output {
    let mut command = Command::new("fossil");
    command
        .args(args)
        .current_dir(env.root())
        .env("FOSSIL_USER", "fixture")
        .env("USER", "fixture")
        .env("FOSSIL_HOME", env.fossil_home());
    command.output().expect("spawn fossil subprocess")
}

/// Turn the fixture into a real Fossil checkout and commit its initial state.
pub fn fossil_init(env: &TestEnv) {
    fossil_checkout(env);
    ignore_history_store(env);
    fossil_commit_all(env, "fixture: initial state");
}

/// Create the Fossil repository and open it in the fixture root without
/// recording anything; the caller stages and commits.
pub fn fossil_checkout(env: &TestEnv) {
    let repo = env
        .fossil_repo
        .as_ref()
        .expect("a fossil repo temp dir")
        .path()
        .join("repo.fossil");
    let repo = repo.to_string_lossy().into_owned();
    let init = fossil(env, &["init", &repo]);
    assert!(
        init.status.success(),
        "fossil init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    let open = fossil(env, &["open", "-f", &repo]);
    assert!(
        open.status.success(),
        "fossil open failed: {}",
        String::from_utf8_lossy(&open.stderr)
    );
    // Tests rewrite files within the same second; disable the mtime shortcut
    // so Fossil always hashes and never reports a stale check-out.
    let settings = fossil(env, &["settings", "mtime-changes", "off"]);
    assert!(
        settings.status.success(),
        "fossil settings failed: {}",
        String::from_utf8_lossy(&settings.stderr)
    );
}

/// A bare Fossil checkout (no `givn/` scaffold yet) for scenarios that run
/// `givn init` inside an existing checkout.
pub fn make_fossil_root_bare() -> TestEnv {
    let mut env = make_bare_root();
    env.backend = BackendKind::Fossil;
    env.fossil_repo = Some(TempDir::new().expect("create fossil repo temp dir"));
    fossil_checkout(&env);
    env
}

/// A fixture that ran `givn init` before the Fossil checkout existed already
/// has the record store on disk. Ignore it before the first `addremove`, as a
/// real project's `givn init` would have done in the checkout.
fn ignore_history_store(env: &TestEnv) {
    let ignore = fossil(env, &["settings", "ignore-glob", FOSSIL_IGNORE_ENTRY]);
    assert!(
        ignore.status.success(),
        "fossil settings ignore-glob failed: {}",
        String::from_utf8_lossy(&ignore.stderr)
    );
}

/// Record all pending fixture writes and commit them.
pub fn fossil_commit_all(env: &TestEnv, message: &str) {
    let add = fossil(env, &["addremove"]);
    assert!(
        add.status.success(),
        "fossil addremove failed: {}",
        String::from_utf8_lossy(&add.stderr)
    );
    let commit = fossil(env, &["commit", "--allow-empty", "-m", message]);
    assert!(
        commit.status.success(),
        "fossil commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
}

/// The report's duration rendering for one measured duration.
///
/// The report under test prints durations in exactly four branches; a
/// scenario that asserts a printed duration compares against this rendering
/// so the assertion never re-derives the text from the page it is checking.
/// The durations are measured, so a fixture cannot pin the text as a literal.
pub fn duration_text(ms: u64) -> String {
    match ms {
        0..=999 => format!("{ms} ms"),
        1_000..=59_999 => format!("{} s", ms / 1_000),
        60_000..=3_599_999 => format!("{} m", ms / 60_000),
        _ => format!("{} h {:02} m", ms / 3_600_000, ms % 3_600_000 / 60_000),
    }
}
