//! Gherkin step definitions for givn's executable specs.
//!
//! These bindings are executed by the `features_runner` test binary against
//! the `.feature` files under `givn/specs/`. The `.feature` file is the sole
//! source of scenario identity — there is no parallel `#[test]` file.

use std::process::Output;

use cucumber::{World, gherkin::Step, given, then, when};

use crate::common;

/// Per-scenario state for the cucumber runner. A fresh `Default` instance is
/// constructed for each scenario.
pub mod choose_project_license_steps;
pub mod coverage_readme_example_steps;
pub mod deduplicate_change_dir_detection_steps;
pub mod enforce_post_merge_e2e_gate_steps;
pub mod extract_coverage_gate_steps;
pub mod fix_uppercase_checkbox_parsing_steps;
pub mod remove_dead_skip_list_code_steps;
pub mod validate_artifact_id_input_steps;
pub mod validate_change_id_input_steps;
pub mod workflow_enforcement_steps;

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
    /// Successful overlay result (for config-overlay characterization).
    overlay_manifest: Option<givn::config::manifest::Manifest>,
    /// Overlay error message (for config-overlay characterization).
    overlay_error: Option<String>,
    /// Real project root for license check scenarios.
    project_root: Option<std::path::PathBuf>,
    /// License check state.
    license_state: choose_project_license_steps::LicenseState,
}

// --- helpers ---------------------------------------------------------------

pub(crate) fn output(world: &GivnWorld) -> &Output {
    world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet")
}

pub(crate) fn stdout(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&output(world).stdout).into_owned()
}

pub(crate) fn stderr(world: &GivnWorld) -> String {
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
    args.iter().skip(1).rfind(|a| a.as_str() != "--").cloned()
}

// --- Given -----------------------------------------------------------------

#[given("a project root with givn/ initialised")]
fn init_root(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("a project root with givn/ initialised without addons")]
fn init_root_no_addons(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--no-addons"]);
    assert!(
        out.status.success(),
        "givn init --no-addons failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    world.env = Some(env);
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    std::fs::create_dir_all(env.path(&format!("givn/changes/{id}")))
        .expect("pre-create change dir");
}

#[given("a project override file at givn/artifacts/proposal/proposal.md with custom content")]
fn override_proposal(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = "# Custom Proposal\nThis is a test override.\n";
    common::write_override(env, "proposal", "proposal.md", content);
    world.override_content = Some(content.to_string());
}

#[given(regex = r#"^a \.feature file at "([^"]+)" with:$"#)]
fn feature_file_at(world: &mut GivnWorld, path: String, step: &Step) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content =
        "@givn.delta @foo\nFeature: Foo\n\n  @givn.removed @wip\n  Scenario: Old\n    Given x\n";
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(&file_path, content).expect("write .feature file");
}

#[given(
    regex = r#"^a feature file with a removed scenario and a separate wip scenario at "([^"]+)"$"#
)]
fn feature_file_removed_and_wip(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let argv: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let out = common::run_givn(env, &argv);
    if let Some(id) = change_id_from_args(&args) {
        world.last_change_id = Some(id);
    }
    world.last_output = Some(out);
}

#[when(regex = r"^the user invokes `([^`]+)`$")]
fn user_invokes(world: &mut GivnWorld, cmd: String) {
    user_runs(world, cmd);
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

#[then("the command fails")]
fn command_fails(world: &mut GivnWorld) {
    let out = output(world);
    assert!(
        !out.status.success(),
        "expected failure, got exit {:?}: stdout={}",
        out.status.code(),
        stdout(world),
    );
}

#[then("the command succeeds")]
fn command_succeeds(world: &mut GivnWorld) {
    exits_zero(world);
}

#[then("the command exits non-zero")]
fn command_exits_nonzero(world: &mut GivnWorld) {
    command_fails(world);
}

// --- Then: e2e output assertions ------------------------------------------

#[then(regex = r#"^stdout confirms integrity checks passed$"#)]
fn stdout_confirms_integrity_passed(world: &mut GivnWorld) {
    let s = stdout(world);
    assert!(
        s.contains("integrity") && (s.contains("passed") || s.contains("PASS")),
        "stdout missing integrity passed confirmation: {}",
        s,
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let p = env.path(&rel);
    assert!(p.is_dir(), "directory not created: {:?}", p);
}

#[then(regex = r"^the file (givn/changes/[^/]+/proposal\.md) exists$")]
fn file_exists(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let p = env.path(&rel);
    assert!(p.is_file(), "file not created: {:?}", p);
}

#[then("no change directory has been created")]
fn no_change_created(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let p = env.path(&rel);
    assert!(!p.exists(), "directory unexpectedly created: {:?}", p);
}

// --- Then: stdout assertions ----------------------------------------------

#[then(regex = r#"^stdout contains a line "(.*)"$"#)]
fn stdout_contains_line(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    assert!(
        s.lines().any(|l| l.trim() == needle),
        "stdout missing line {:?}: {}",
        needle,
        s,
    );
}

#[then(regex = r"^stdout contains the absolute path to (.+)$")]
fn stdout_contains_abs_path(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let abs = env.path(rel.trim()).to_string_lossy().into_owned();
    let s = stdout(world);
    assert!(
        s.contains(&abs),
        "stdout missing absolute path {}: {}",
        abs,
        s
    );
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

#[then(regex = r#"^stdout contains "(.*)"$"#)]
fn stdout_contains_quoted_then(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    assert!(s.contains(&needle), "stdout missing {:?}: {}", needle, s);
}

#[then(regex = r#"^stdout mentions "(.*)"$"#)]
fn stdout_mentions(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    // Gherkin step text escapes " as \", so unescape for comparison.
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stdout missing {:?}: {}", needle, s);
}
#[allow(dead_code)]
fn stdout_contains_quoted(world: &mut GivnWorld, needle: String) {
    let s = stdout(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stdout missing {:?}: {}", needle, s);
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

#[then("stderr mentions coverage gate failure")]
fn stderr_mentions_coverage_failure(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("coverage") && (s.contains("fail") || s.contains("blocked")),
        "stderr missing coverage failure: {}",
        s,
    );
}

#[then(regex = r"^stderr contains the absolute path to (.+)$")]
fn stderr_contains_abs_path(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let abs = env.path(rel.trim()).to_string_lossy().into_owned();
    let s = stderr(world);
    assert!(
        s.contains(&abs),
        "stderr missing absolute path {}: {}",
        abs,
        s
    );
}

// --- e2e Then steps -------------------------------------------------------

#[then(regex = r#"^stdout shows (.+) as skipped$"#)]
fn stdout_shows_as_skipped(world: &mut GivnWorld, artifact: String) {
    let s = stdout(world);
    // givn status renders skipped artifacts as "⊘ <name>" (U+2298)
    assert!(
        s.contains(&format!("\u{2298} {}", artifact)),
        "stdout missing '{} as skipped': {}",
        artifact,
        s,
    );
}

#[then(regex = r#"^stderr reports the placeholder hash as invalid$"#)]
fn stderr_reports_invalid_hash(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("hash") && (s.contains("invalid") || s.contains("placeholder")),
        "stderr missing hash validity report: {}",
        s,
    );
}

#[then("the README.md contains a coverage summary fence")]
fn readme_has_coverage_fence(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains("<!-- givn:begin:coverage -->"),
        "README.md missing coverage fence: {}",
        readme,
    );
}

// --- Then: change directory contents --------------------------------------

#[then(
    regex = r"^the change directory (givn/changes/[^/]+/) contains exactly one file: proposal\.md$"
)]
fn change_dir_exactly_proposal(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let p = env.path(&format!("{rel}/specs"));
    assert!(!p.exists(), "specs/ unexpectedly exists: {:?}", p);
}

// --- Then: proposal.md content --------------------------------------------

#[then(
    regex = r"^the file (givn/changes/[^/]+/proposal\.md) equals the built-in proposal template byte for byte$"
)]
fn equals_builtin_template(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = std::fs::read_to_string(env.path(&format!("givn/changes/{id}/proposal.md")))
        .expect("read proposal.md");
    assert!(
        content.contains("{{CHANGE_ID}}"),
        "proposal.md missing {{{{CHANGE_ID}}}} placeholder: {}",
        content,
    );
}

#[then(
    regex = r"^the file (givn/changes/[^/]+/proposal\.md) equals the project override content byte for byte$"
)]
fn equals_override(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = std::fs::read_to_string(env.path(&rel)).expect("read proposal.md");
    assert!(
        !content.contains(&needle),
        "proposal.md should not contain {:?}: {}",
        needle,
        content,
    );
}

// --- Steps for install-extras -----------------------------------------------

#[then(regex = r#"^the file "([^"]+)" exists$"#)]
fn file_quoted_exists(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(env.path(&rel).exists(), "expected file {:?} to exist", rel,);
}

#[then(regex = r#"^the file "([^"]+)" does not exist$"#)]
fn file_quoted_does_not_exist(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        !env.path(&rel).exists(),
        "expected file {:?} to not exist",
        rel,
    );
}

#[then(regex = r#"^the directory "([^"]+)" does not exist$"#)]
fn dir_quoted_does_not_exist(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        !env.path(&rel).exists(),
        "expected directory {:?} to not exist",
        rel,
    );
}

#[given(regex = r#"^givn addon "([^"]+)" is "([^"]+)"$"#)]
fn givn_addon_key_is_set(world: &mut GivnWorld, name: String, value: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let target_line = format!("  {name}: {value}");
    let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
    let mut found = false;
    for line in &mut lines {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{name}:")) && !line.trim_start().starts_with('#') {
            *line = target_line.clone();
            found = true;
            break;
        }
    }
    if !found {
        // Insert inside the addons block or add it.
        let mut inserted = false;
        for i in 0..lines.len() {
            if lines[i].trim() == "addons:" {
                lines.insert(i + 1, target_line.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            lines.push("addons:".to_string());
            lines.push(target_line);
        }
    }
    std::fs::write(&path, lines.join("\n") + "\n").expect("write givn/config.yaml");
}

#[then(regex = r#"^givn addon "([^"]+)" is "([^"]+)"$"#)]
fn givn_addon_key_is(world: &mut GivnWorld, name: String, value: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let config =
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    let found = config.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == format!("{name}: {value}")
    });
    assert!(
        found,
        "expected givn/config.yaml to contain '  {name}: {value}'",
    );
}

#[then(regex = r#"^stdout does not contain "([^"]+)"$"#)]
fn stdout_does_not_contain(world: &mut GivnWorld, needle: String) {
    let out = stdout(world);
    assert!(
        !out.contains(&needle),
        "expected stdout to not contain {:?}\nactual stdout:\n{}",
        needle,
        out,
    );
}

#[given(regex = r#"^the skill "([^"]+)" exists in all targets$"#)]
fn skill_exists_in_all_targets(world: &mut GivnWorld, skill: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    for target in &[".agents", ".claude", ".opencode"] {
        let dir = env.path(&format!("{target}/skills/{skill}"));
        std::fs::create_dir_all(&dir).expect("create skill dir");
        std::fs::write(dir.join("SKILL.md"), "# stub\n").expect("write stub");
    }
}

#[given(regex = r#"^the command "([^"]+)" exists in \.opencode/commands$"#)]
fn command_exists_in_opencode(world: &mut GivnWorld, cmd: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let p = env.path(&format!(".opencode/commands/{cmd}.md"));
    std::fs::create_dir_all(p.parent().unwrap()).expect("create commands dir");
    std::fs::write(&p, "# stub\n").expect("write stub");
}

#[given(regex = r#"^a change "([^"]+)" exists with artifact "([^"]+)"$"#)]
fn change_exists_with_artifact(world: &mut GivnWorld, id: String, file: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // Create the change via givn new if it doesn't exist yet.
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = common::run_givn(env, &["new", &id]);
        assert!(
            out.status.success(),
            "givn new {} failed: {}",
            id,
            String::from_utf8_lossy(&out.stderr),
        );
    }
    // Create the specified artifact file under the change dir.
    let artifact_path = env.path(&format!("givn/changes/{}/{}", id, file));
    if let Some(parent) = artifact_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dir for artifact");
    }
    std::fs::write(&artifact_path, "").expect("write artifact file");
}

#[given(regex = r#"^a change "([^"]+)" exists in archive$"#)]
fn change_exists_in_archive(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let dir = env.path(&format!("givn/archive/{}", id));
    std::fs::create_dir_all(&dir).expect("create archive dir");
    // list_archived only checks directory existence, not contents —
    // but having a file ensures it's not mistaken for an empty dir edge case.
    std::fs::write(dir.join(".gitkeep"), "").expect("write archive marker");
}

#[then(regex = r#"^stdout line (\d+) matches "([^"]+)"$"#)]
fn stdout_line_matches(world: &mut GivnWorld, n: usize, needle: String) {
    let s = stdout(world);
    let line = s.lines().nth(n.saturating_sub(1)).unwrap_or("");
    assert!(
        line.contains(&needle),
        "stdout line {} expected to contain {:?}, got: {:?}",
        n,
        needle,
        line,
    );
}

#[given(regex = r#"^a change "([^"]+)" exists with all required artifacts complete$"#)]
fn change_all_required_done(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = common::run_givn(env, &["new", &id]);
        assert!(out.status.success(), "givn new failed");
    }
    let change_dir = env.path(&format!("givn/changes/{}", id));
    // Write all manifest artifacts. Uses the default manifest artifact list.
    for file in &[
        "proposal.md",
        "design.md",
        "design-review.md",
        "tasks.md",
        "review.md",
    ] {
        std::fs::write(change_dir.join(file), "").expect("write artifact");
    }
    // Write DESIGN-REVIEW: PASS marker.
    std::fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    // Write all-checked tasks.md.
    std::fs::write(change_dir.join("tasks.md"), "- [x] done\n").unwrap();
    // Write REVIEW: PASS marker.
    std::fs::write(change_dir.join("review.md"), "REVIEW: PASS\n").unwrap();
    // Create specs dir + feature for specs artifact Done status.
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(specs_dir.join("example.feature"), "Feature: example\n").unwrap();
    // Write STATUS: DONE marker for arc42-docs — `givn init` enables the
    // arc42 addon by default, which gates archive, so a genuinely
    // "all required done" fixture must include it.
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
}

#[given(regex = r#"^the change "([^"]+)" has "\.givn-skip" listing "([^"]+)"$"#)]
fn change_has_skip_file(world: &mut GivnWorld, id: String, ids: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let skip_path = env.path(&format!("givn/changes/{}/.givn-skip", id));
    let content = ids.replace(',', "\n");
    std::fs::write(&skip_path, content).expect("write .givn-skip");
}

#[given(regex = r#"^the effective manifest has an optional artifact "([^"]+)"$"#)]
fn manifest_has_optional_artifact(world: &mut GivnWorld, _id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["addons", "enable", "arc42"]);
    assert!(
        out.status.success(),
        "givn addons enable arc42 failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// --- Config overlay steps (module-boundary characterization) ----------------

use givn::config::manifest::Manifest;
use givn::config::overlay;
#[when(regex = r#"^the overlay "(.*)" is applied to the default manifest$"#)]
fn apply_overlay(world: &mut GivnWorld, yaml: String) {
    let base = Manifest::load_embedded().expect("load default manifest");
    let yaml = yaml.replace("\\n", "\n");
    match overlay::apply(base, &yaml) {
        Ok(m) => {
            world.overlay_manifest = Some(m);
        }
        Err(e) => {
            world.overlay_error = Some(e.to_string());
        }
    }
}

#[when(regex = r#"^the overlay "(.*)" is applied twice to the default manifest$"#)]
fn apply_overlay_twice(world: &mut GivnWorld, yaml: String) {
    let base = Manifest::load_embedded().expect("load default manifest");
    let yaml = yaml.replace("\\n", "\n");
    let after = overlay::apply(base, &yaml).expect("first apply");
    match overlay::apply(after, &yaml) {
        Ok(m) => {
            world.overlay_manifest = Some(m);
        }
        Err(e) => {
            world.overlay_error = Some(e.to_string());
        }
    }
}

#[then("the effective manifest equals the default manifest")]
fn manifest_equals_default(world: &mut GivnWorld) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    let default = Manifest::load_embedded().expect("load default manifest");
    assert_eq!(
        m.artifacts.len(),
        default.artifacts.len(),
        "artifact count differs"
    );
    assert_eq!(
        m.verify.command, default.verify.command,
        "verify.command differs"
    );
    assert_eq!(
        m.apply.requires, default.apply.requires,
        "apply.requires differs"
    );
    assert_eq!(
        m.addons.domain_modeling, default.addons.domain_modeling,
        "addons.domain_modeling differs"
    );
    assert_eq!(
        m.addons.dev_principles, default.addons.dev_principles,
        "addons.dev_principles differs"
    );
    assert_eq!(
        m.addons.grill_me, default.addons.grill_me,
        "addons.grill_me differs"
    );
    assert_eq!(
        m.addons.grill_with_docs, default.addons.grill_with_docs,
        "addons.grill_with_docs differs"
    );
}

#[then(regex = r#"^the effective manifest has verify command "([^"]+)"$"#)]
fn manifest_verify_command(world: &mut GivnWorld, cmd: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    assert_eq!(m.verify.command, cmd);
}

#[then(regex = r#"^the effective manifest has apply requires exactly "([^"]+)"$"#)]
fn manifest_apply_requires(world: &mut GivnWorld, req: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    assert_eq!(m.apply.requires, vec![req]);
}

#[then(regex = r#"^the effective manifest has an artifact "([^"]+)" that requires "([^"]+)"$"#)]
fn manifest_artifact_requires(world: &mut GivnWorld, id: String, dep: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    let a = m
        .artifacts
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("artifact '{}' not found", id));
    assert!(
        a.requires.contains(&dep),
        "artifact '{}' requires {:?}, expected '{}'",
        id,
        a.requires,
        dep
    );
}

#[then(regex = r#"^the effective manifest does not have an artifact "([^"]+)"$"#)]
fn manifest_no_artifact(world: &mut GivnWorld, id: String) {
    if let Some(err) = &world.overlay_error {
        panic!("overlay application failed (expected success): {}", err);
    }
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    assert!(
        m.artifacts.iter().all(|a| a.id != id),
        "artifact '{}' should not exist",
        id
    );
}
#[then(regex = r#"^the effective manifest has an artifact "([^"]+)" that is optional$"#)]
fn manifest_artifact_optional(world: &mut GivnWorld, id: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    let a = m
        .artifacts
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("artifact '{}' not found", id));
    assert!(a.optional, "artifact '{}' should be optional", id);
}

#[then(regex = r#"^the effective manifest has an artifact "([^"]+)" that generates "([^"]+)"$"#)]
fn manifest_artifact_generates(world: &mut GivnWorld, id: String, generates: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    let a = m
        .artifacts
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("artifact '{}' not found", id));
    assert!(
        a.generates.contains(&generates),
        "artifact '{}' generates {:?}, expected '{}'",
        id,
        a.generates,
        generates
    );
}

#[then(regex = r#"^the effective manifest has exactly one artifact "([^"]+)"$"#)]
fn manifest_exactly_one(world: &mut GivnWorld, id: String) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    let count = m.artifacts.iter().filter(|a| a.id == id).count();
    assert_eq!(
        count, 1,
        "expected exactly one artifact '{}', found {}",
        id, count
    );
}

#[then("the effective manifest has extras enabled")]
fn manifest_extras_enabled(world: &mut GivnWorld) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    assert!(m.extras, "expected extras to be enabled");
}

#[then("the effective manifest has the skill addons enabled")]
fn manifest_skill_addons_enabled(world: &mut GivnWorld) {
    let m = world.overlay_manifest.as_ref().expect("no overlay result");
    assert!(
        m.addons.domain_modeling,
        "expected domain_modeling addon to be enabled"
    );
    assert!(
        m.addons.dev_principles,
        "expected dev_principles addon to be enabled"
    );
    assert!(m.addons.grill_me, "expected grill_me addon to be enabled");
    assert!(
        m.addons.grill_with_docs,
        "expected grill_with_docs addon to be enabled"
    );
}

#[then(regex = r#"^the overlay application fails with "([^"]+)"$"#)]
fn overlay_fails_with(world: &mut GivnWorld, needle: String) {
    let err = world
        .overlay_error
        .as_ref()
        .expect("expected overlay to fail");
    assert!(
        err.contains(&needle),
        "error '{}' does not contain '{}'",
        err,
        needle
    );
}
