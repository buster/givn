//! Gherkin step definitions for givn's executable specs.
//!
//! These bindings are executed by the `features_runner` test binary against
//! the `.feature` files under `givn/specs/`. The `.feature` file is the sole
//! source of scenario identity — there is no parallel `#[test]` file.

use std::process::Output;

use cucumber::{given, gherkin::Step, then, when, World};

use crate::common;

/// Per-scenario state for the cucumber runner. A fresh `Default` instance is
/// constructed for each scenario.
#[derive(Debug, Default, World)]
pub struct GivnWorld {
    /// The temp project root (init'd or bare) for this scenario.
    env: Option<common::TestEnv>,
    /// Captured stdout/stderr/exit of the last `givn` invocation.
    last_output: Option<Output>,
    /// The change id parsed from the last `givn new <id>` command, for steps
    /// that refer to "the proposal.md" without naming the path.
    last_change_id: Option<String>,
    /// Override content written by the override Given step, for the
    /// "equals the project override content byte for byte" step.
    override_content: Option<String>,
}

// --- helpers ---------------------------------------------------------------

fn output(world: &GivnWorld) -> &Output {
    world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet")
}

fn stdout(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&output(world).stdout).into_owned()
}

fn stderr(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&output(world).stderr).into_owned()
}

/// Parse a command string captured from `the user runs `...`` into argv,
/// dropping the leading program name (`givn`). Double quotes delimit a single
/// argument (which may be empty or contain spaces); unquoted whitespace
/// separates arguments.
fn parse_command(raw: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut has_token = false;
    for ch in raw.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                has_token = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if has_token {
                    args.push(std::mem::take(&mut current));
                    has_token = false;
                }
            }
            c => {
                current.push(c);
                has_token = true;
            }
        }
    }
    if has_token {
        args.push(current);
    }
    if args.first().map(|s| s == "givn").unwrap_or(false) {
        args.remove(0);
    }
    args
}

/// The change id from a `new` argv: the last positional argument, skipping a
/// lone `--`.
fn change_id_from_args(args: &[String]) -> Option<String> {
    if args.first()?.as_str() != "new" {
        return None;
    }
    args.iter()
        .skip(1)
        .filter(|a| a.as_str() != "--")
        .last()
        .cloned()
}

// --- Given -----------------------------------------------------------------

#[given("a project root with givn/ initialised")]
fn init_root(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("a placeholder step")]
fn placeholder_step(_world: &mut GivnWorld) {
    // No-op: used by @givn.removed scenarios that need a single step to parse.
}

#[given("a project root without a givn/ directory")]
fn bare_root(world: &mut GivnWorld) {
    world.env = Some(common::make_bare_root());
}

#[given(regex = r#"^no existing change named "([^"]+)"$"#)]
fn no_existing_change(_world: &mut GivnWorld, _id: String) {
    // A fresh init root has no changes; nothing to pre-create.
}

#[given(regex = r#"^an existing change named "([^"]+)"$"#)]
fn existing_change(world: &mut GivnWorld, id: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    std::fs::create_dir_all(env.path(&format!("givn/changes/{id}")))
        .expect("pre-create change dir");
}

#[given("a project override file at givn/artifacts/proposal/proposal.md with custom content")]
fn override_proposal(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required first");
    let content = "# Custom Proposal\nThis is a test override.\n";
    common::write_override(env, "proposal", "proposal.md", content);
    world.override_content = Some(content.to_string());
}

#[given(regex = r#"^a \.feature file at "([^"]+)" with:$"#)]
fn feature_file_at(world: &mut GivnWorld, path: String, step: &Step) {
    let env = world.env.as_ref().expect("a project root is required first");
    let content = step
        .docstring
        .as_ref()
        .expect("docstring content for .feature file step");
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs for .feature file");
    }
    std::fs::write(&file_path, content).expect("write .feature file");
}

#[given(regex = r#"^a feature file with a removed-wip placeholder at "([^"]+)"$"#)]
fn feature_file_removed_wip_placeholder(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let content = "@givn.delta @foo\nFeature: Foo\n\n  @givn.removed @wip\n  Scenario: Old\n    Given x\n";
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(&file_path, content).expect("write .feature file");
}

#[given(regex = r#"^a feature file with a removed scenario and a separate wip scenario at "([^"]+)"$"#)]
fn feature_file_removed_and_wip(world: &mut GivnWorld, path: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let content = "@givn.delta @foo\nFeature: Foo\n\n  @givn.removed\n  Scenario: Old\n    Given x\n\n  @wip\n  Scenario: NewNotDone\n    Given y\n";
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(&file_path, content).expect("write .feature file");
}

// --- When ------------------------------------------------------------------

#[when(regex = r"^the user runs `([^`]+)`$")]
fn user_runs(world: &mut GivnWorld, cmd: String) {
    let args = parse_command(&cmd);
    let env = world.env.as_ref().expect("a project root is required first");
    let argv: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let out = common::run_givn(env, &argv);
    if let Some(id) = change_id_from_args(&args) {
        world.last_change_id = Some(id);
    }
    world.last_output = Some(out);
}

// --- Then: exit codes ------------------------------------------------------

#[then("the command exits 0")]
fn exits_zero(world: &mut GivnWorld) {
    let out = output(world);
    assert!(
        out.status.success(),
        "expected exit 0, got {:?}: stderr={}",
        out.status.code(),
        stderr(world),
    );
}

#[then("the command exits 1")]
fn exits_one(world: &mut GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(world));
}

#[then("the command exits 2")]
fn exits_two_generic(world: &mut GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(2), "stderr={}", stderr(world));
}

#[then("the command exits non-zero")]
fn exits_nonzero(world: &mut GivnWorld) {
    let out = output(world);
    assert!(
        !out.status.success(),
        "expected non-zero exit, got success: stdout={}",
        stdout(world),
    );
}

#[then("the command exits with the argument-parsing exit code 2")]
fn exits_two(world: &mut GivnWorld) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(2), "stderr={}", stderr(world));
}

// --- Then: filesystem existence -------------------------------------------

#[then(regex = r"^the directory (givn/changes/[^/]+/) exists$")]
fn directory_exists(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let p = env.path(&rel);
    assert!(p.is_dir(), "directory not created: {:?}", p);
}

#[then(regex = r"^the file (givn/changes/[^/]+/proposal\.md) exists$")]
fn file_exists(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let p = env.path(&rel);
    assert!(p.is_file(), "file not created: {:?}", p);
}

#[then("no change directory has been created")]
fn no_change_created(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required first");
    let changes = env.path("givn/changes");
    if changes.is_dir() {
        let count = std::fs::read_dir(&changes)
            .expect("read givn/changes")
            .filter_map(|e| e.ok())
            .count();
        assert_eq!(count, 0, "givn/changes is not empty: {:?}", changes);
    }
    // If givn/changes does not exist (bare root), nothing was created.
}

#[then(regex = r"^no directory (givn/changes/[^/]+/) has been created$")]
fn no_directory_created(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let p = env.path(&rel);
    assert!(!p.exists(), "directory unexpectedly created: {:?}", p);
}

// --- Then: stdout assertions ----------------------------------------------

#[then(regex = r#"^stdout contains a line "(.*)"$"#)]
fn stdout_contains_line(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    assert!(
        s.lines().any(|l| l == needle),
        "stdout missing line {:?}: {}",
        needle,
        s,
    );
}

#[then(regex = r"^stdout contains the absolute path to (.+)$")]
fn stdout_contains_abs_path(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let abs = env.path(rel.trim()).to_string_lossy().into_owned();
    let s = stdout(world);
    assert!(s.contains(&abs), "stdout missing absolute path {}: {}", abs, s);
}

#[then(regex = r#"^stdout contains a line beginning with "(.*)"$"#)]
fn stdout_line_beginning_with(world: &mut GivnWorld, prefix: String) {
    let s = stdout(world);
    assert!(
        s.lines().any(|l| l.starts_with(&prefix)),
        "stdout missing a line beginning with {:?}: {}",
        prefix,
        s,
    );
}

#[then(regex = r#"^stdout mentions "(.*)"$"#)]
fn stdout_mentions(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    assert!(s.contains(&needle), "stdout missing {:?}: {}", needle, s);
}

// --- Then: stderr assertions ----------------------------------------------

#[then(regex = r#"^stderr contains "(.*)"$"#)]
fn stderr_contains(world: &mut GivnWorld, needle: String) {
    let s = stderr(world);
    assert!(s.contains(&needle), "stderr missing {:?}: {}", needle, s);
}

#[then(regex = r"^the (\w+) finding appears before the (\w+) finding on stderr$")]
fn stderr_finding_order(world: &mut GivnWorld, needle_a: String, needle_b: String) {
    let s = stderr(world);
    let idx_a = s.lines().position(|l| l.contains(&needle_a));
    let idx_b = s.lines().position(|l| l.contains(&needle_b));
    let (idx_a, idx_b) = match (idx_a, idx_b) {
        (Some(a), Some(b)) => (a, b),
        (a, b) => panic!(
            "stderr missing a finding line: {:?} at {:?}, {:?} at {:?}: {}",
            needle_a, a, needle_b, b, s,
        ),
    };
    assert!(
        idx_a < idx_b,
        "expected {:?} finding before {:?} finding, got indices {} and {}: {}",
        needle_a,
        needle_b,
        idx_a,
        idx_b,
        s,
    );
}

#[then("stderr mentions an unexpected argument")]
fn stderr_unexpected_argument(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.to_lowercase().contains("unexpected"),
        "stderr missing 'unexpected': {}",
        s,
    );
}

#[then("stderr mentions running init first")]
fn stderr_mentions_init(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(s.contains("init"), "stderr missing 'init': {}", s);
}

#[then(regex = r"^stderr contains the absolute path to (.+)$")]
fn stderr_contains_abs_path(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let abs = env.path(rel.trim()).to_string_lossy().into_owned();
    let s = stderr(world);
    assert!(s.contains(&abs), "stderr missing absolute path {}: {}", abs, s);
}

// --- Then: change directory contents --------------------------------------

#[then(regex = r"^the change directory (givn/changes/[^/]+/) contains exactly one file: proposal\.md$")]
fn change_dir_exactly_proposal(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let dir = env.path(&rel);
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("read change dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec!["proposal.md".to_string()],
        "change dir entries: {:?}",
        entries,
    );
}

#[then(regex = r"^no subdirectory named specs/ exists inside (givn/changes/[^/]+)/$")]
fn no_specs_subdir(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let p = env.path(&format!("{rel}/specs"));
    assert!(!p.exists(), "specs/ unexpectedly exists: {:?}", p);
}

// --- Then: proposal.md content --------------------------------------------

#[then(regex = r"^the file (givn/changes/[^/]+/proposal\.md) equals the built-in proposal template byte for byte$")]
fn equals_builtin_template(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let actual = std::fs::read(env.path(&rel)).expect("read proposal.md");
    let expected = include_bytes!("../../assets/templates/proposal.md");
    assert_eq!(
        actual.as_slice(),
        expected.as_slice(),
        "proposal.md does not equal the built-in template",
    );
}

#[then(regex = r#"^the proposal\.md heading contains the literal text "\{\{CHANGE_ID\}\}"$"#)]
fn proposal_heading_has_change_id(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("no change id was recorded from a `givn new` command");
    let env = world.env.as_ref().expect("a project root is required first");
    let content =
        std::fs::read_to_string(env.path(&format!("givn/changes/{id}/proposal.md")))
            .expect("read proposal.md");
    assert!(
        content.contains("{{CHANGE_ID}}"),
        "proposal.md missing {{{{CHANGE_ID}}}} placeholder: {}",
        content,
    );
}

#[then(regex = r"^the file (givn/changes/[^/]+/proposal\.md) equals the project override content byte for byte$")]
fn equals_override(world: &mut GivnWorld, rel: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let actual = std::fs::read(env.path(&rel)).expect("read proposal.md");
    let expected = world
        .override_content
        .clone()
        .expect("no project override content was recorded");
    assert_eq!(
        actual.as_slice(),
        expected.as_bytes(),
        "proposal.md does not equal the project override content",
    );
}

#[then(regex = r#"^the file (givn/changes/[^/]+/proposal\.md) does not contain "(.*)"$"#)]
fn file_does_not_contain(world: &mut GivnWorld, rel: String, needle: String) {
    let env = world.env.as_ref().expect("a project root is required first");
    let content = std::fs::read_to_string(env.path(&rel)).expect("read proposal.md");
    assert!(
        !content.contains(&needle),
        "proposal.md should not contain {:?}: {}",
        needle,
        content,
    );
}
