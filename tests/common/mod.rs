//! Shared test fixtures for givn characterization tests.
//!
//! Black-box seam: invoke the compiled `givn` binary as a subprocess with a
//! per-test `tempfile::TempDir` as the current directory. The `TempDir` is
//! held by the returned `TestEnv` so it lives as long as the test needs it;
//! it is removed automatically on drop, including on panic.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

pub mod reference_check;

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

/// Resolve a repository-relative path independent of the process cwd.
///
/// The in-process `givn run` scenario temporarily changes the process-wide
/// current directory, and the single-process runner (`cargo test`) shares one
/// process across scenarios; anchoring repo-relative reads at the manifest
/// directory removes that race.
pub fn repo_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
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
    _env: &TestEnv,
    cwd: &std::path::Path,
    args: &[&str],
    environments: &[(&str, &str)],
) -> Output {
    let bin = env!("CARGO_BIN_EXE_givn");
    let mut command = Command::new(bin);
    command
        .args(args)
        .current_dir(cwd)
        // Runner-scoped variables describe the outer suite invocation and
        // must never leak into fixture projects.
        .env_remove("GIVN_FEATURES")
        // Fossil refuses to commit without a user; Git fixtures ignore it.
        .env("FOSSIL_USER", "fixture");
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

    let bin = env!("CARGO_BIN_EXE_givn");
    let mut child = Command::new(bin)
        .args(args)
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .envs(environments.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
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
    let bin = env!("CARGO_BIN_EXE_givn");
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
    Command::new("fossil")
        .args(args)
        .current_dir(env.root())
        .env("FOSSIL_USER", "fixture")
        .env("USER", "fixture")
        .output()
        .expect("spawn fossil subprocess")
}

/// Turn the fixture into a real Fossil checkout and commit its initial state.
pub fn fossil_init(env: &TestEnv) {
    fossil_checkout(env);
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

/// An initialized fixture with a real Fossil checkout whose scaffold is not
/// recorded yet; the caller stages and commits (optionally after setting an
/// ignore-glob so generated agent files stay unmanaged).
pub fn make_fossil_root_uncommitted() -> TestEnv {
    let mut env = make_init_root_without_git();
    env.backend = BackendKind::Fossil;
    env.fossil_repo = Some(TempDir::new().expect("create fossil repo temp dir"));
    fossil_checkout(&env);
    env
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
