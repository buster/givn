//! Gherkin step definitions for givn's executable specs.
//!
//! These bindings are executed by the `features_runner` test binary against
//! the `.feature` files under `givn/specs/`. The `.feature` file is the sole
//! source of scenario identity — there is no parallel `#[test]` file.

use std::process::Output;

use cucumber::{World, gherkin::Step, given, then, when};

use crate::common;

pub mod adr_qualification_gate_e2e_steps;
pub mod adr_qualification_gate_steps;
pub mod arc42_review_alignment_steps;
pub mod archive_proof_e2e_steps;
pub mod archive_proof_steps;
pub mod archive_proof_support;
/// Per-scenario state for the cucumber runner. A fresh `Default` instance is
/// constructed for each scenario.
pub mod choose_project_license_steps;
pub mod coverage_readme_example_steps;
pub mod coverage_script_workflow_e2e_steps;
pub mod coverage_script_workflow_steps;
pub mod deduplicate_change_dir_detection_steps;
pub mod enforce_post_merge_e2e_gate_steps;
pub mod extract_coverage_gate_steps;
pub mod fix_domain_modeling_skill_trigger_e2e_steps;
pub mod fix_domain_modeling_skill_trigger_steps;
pub mod fix_uppercase_checkbox_parsing_steps;
pub mod grouped_merge_steps;
pub mod init_agent_target_selection_steps;
pub mod instruction_integrity_e2e_steps;
pub mod instruction_integrity_steps;
pub mod llm_attribution_e2e_steps;
pub mod llm_attribution_installation_e2e_steps;
pub mod llm_attribution_installation_steps;
pub mod llm_attribution_steps;
pub mod local_only_rules_steps;
pub mod preserve_docstrings_on_archive_steps;
pub mod prose_consolidation_steps;
pub mod readme_coverage_badge_steps;
pub mod readme_enforcement_e2e_steps;
pub mod readme_enforcement_steps;
pub mod release_0_3_0_e2e_steps;
pub mod release_0_3_0_steps;
pub mod remove_dead_skip_list_code_steps;
pub mod repository_ownership_fixtures_steps;
pub mod repository_ownership_fixtures_support;
pub mod review_disposition_gates_steps;
pub mod run_command_e2e_steps;
pub mod run_command_steps;
pub mod run_declaration_fixtures_support;
pub mod run_execution_e2e_steps;
pub mod run_execution_steps;
pub mod run_execution_support;
pub mod run_readme_steps;
pub mod run_upgrade_steps;
pub mod scm_backend_selection_e2e_steps;
pub mod scm_backend_selection_steps;
pub mod skill_rewrites_steps;
pub mod spec_discovery_deterministic_steps;
pub mod spec_overlap_gates_steps;
pub mod template_conditional_rendering_e2e_steps;
pub mod template_conditional_rendering_steps;
pub mod template_conditional_rendering_support;
pub mod validate_artifact_id_input_steps;
pub mod validate_change_id_input_steps;
pub mod visual_design_contract_steps;
pub mod visual_review_gate_steps;
pub mod workflow_enforcement_e2e_steps;
pub mod workflow_enforcement_steps;

#[derive(Debug, Default, World)]
pub struct GivnWorld {
    /// The temp project root (init'd or bare) for this scenario.
    env: Option<common::TestEnv>,
    /// State for the commit-attribution scenarios.
    pub(crate) git_attribution: Option<llm_attribution_steps::GitAttributionState>,
    /// State for the generated OpenCode attribution integration scenario.
    pub(crate) opencode_plugin: Option<llm_attribution_e2e_steps::OpencodePluginState>,
    /// State for the generated Claude attribution integration scenario.
    pub(crate) claude_attribution: Option<llm_attribution_e2e_steps::ClaudeAttributionState>,
    /// Captured stdout/stderr/exit of the last `givn` invocation.
    last_output: Option<Output>,
    /// Output from the previous command in a parity scenario.
    pub(crate) previous_output: Option<Output>,
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
    /// Combined output captured by a multi-command guidance scenario.
    pub(crate) additional_output: Option<String>,
    /// Findings from the last run-declaration validation.
    pub(crate) run_findings: Vec<givn::cli::run_block::RunFinding>,
    /// Reserved fixed port written into the fixture's run declaration.
    pub(crate) run_fixed_port: Option<u16>,
    /// Caller-provided port handed to `givn run --port`.
    pub(crate) run_caller_port: Option<u16>,
    /// The start command declared by the fixture's run declaration.
    pub(crate) run_declared_command: Option<String>,
    /// In-flight readiness probe for the in-process probe scenarios.
    pub(crate) run_probe: Option<ProbeJob>,
    /// Canned HTTP responder for the probe scenarios.
    pub(crate) run_responder: Option<run_execution_support::Responder>,
    /// In-process `givn run` execution for the SIGTERM scenario.
    pub(crate) run_inproc: Option<run_execution_support::InProcStart>,
    /// Marker file the fixture writes when it receives a signal.
    pub(crate) run_trap_marker: Option<std::path::PathBuf>,
    /// The run managed block text produced by the render step.
    pub(crate) run_rendered_block: Option<String>,
    /// Commands-file content captured before the backfill ran.
    pub(crate) run_commands_before: Option<String>,
    /// Whether the backfill applied a change to the commands file.
    pub(crate) run_backfill_applied: Option<bool>,
    /// Git commit hash captured before a second upgrade.
    pub(crate) run_prev_commit: Option<String>,
    /// Working directory override for the next `givn` invocation (nested
    /// project scenario); `None` means the fixture root.
    pub(crate) cwd_override: Option<std::path::PathBuf>,
    /// Skip the generic archive auto-commit for fixtures that intentionally
    /// present a dirty or hook-rejecting worktree.
    pub(crate) skip_auto_commit: bool,
    /// Take the tree snapshot right before the next invocation, after every
    /// Given has finished writing, for the no-mutation assertion.
    pub(crate) snapshot_on_run: bool,
    /// State for the SCM backend selection scenarios.
    pub(crate) scm_fixture: scm_backend_selection_steps::ScmFixture,
}

/// A readiness probe running on a background thread.
#[derive(Debug)]
pub(crate) struct ProbeJob {
    pub(crate) port: u16,
    pub(crate) handle: std::thread::JoinHandle<givn::cli::run::ProbeOutcome>,
}

// --- helpers ---------------------------------------------------------------

/// A completed-change fixture proposal that records a `NEW in <cap>` routing
/// decision for the given capability. Since the fail-closed archive rule
/// blocks new capabilities without a routing decision, every fixture that
/// archives a brand-new capability must declare one. Declaring the group as
/// the capability name reproduces the delta's own specs layout.
pub(crate) fn fixture_routing_proposal(cap: &str) -> String {
    format!(
        "# Proposal: fixture\n\n\
         ## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | {cap} | (fixture) | `NEW in {cap}` | fixture |\n"
    )
}

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
    let out = common::run_givn(&env, &["init", "--no-addons", "--agents", "opencode"]);
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

#[given("placeholder")]
fn removed_scenario_placeholder(_world: &mut GivnWorld) {
    // No-op: a @givn.removed scenario carries exactly one placeholder step so
    // the parser accepts it; the scenario is deleted at archive.
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
    ensure_fixture_usecase(env, &path, content);
}

fn ensure_fixture_usecase(env: &common::TestEnv, feature_path: &str, content: &str) {
    let Some(relative) = feature_path.strip_prefix("givn/specs/") else {
        return;
    };
    let Some(parent) = std::path::Path::new(relative).parent() else {
        return;
    };
    let Some(usecase) = parent.file_name().and_then(|name| name.to_str()) else {
        return;
    };
    let Some(capability) = std::path::Path::new(relative)
        .file_stem()
        .and_then(|name| name.to_str())
    else {
        return;
    };
    let path = env.path(&format!("givn/specs/{usecase}/usecase.md"));
    if path.exists() {
        return;
    }
    let mut e2e_titles = Vec::new();
    let mut e2e = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('@') {
            e2e = trimmed.split_whitespace().any(|tag| tag == "@e2e");
        } else if e2e && trimmed.starts_with("Scenario:") {
            e2e_titles.push(trimmed.trim_start_matches("Scenario:").trim().to_string());
            e2e = false;
        } else if !trimmed.is_empty() && !trimmed.starts_with("Scenario:") {
            e2e = false;
        }
    }
    let mut document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", capability)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace(
            "<scenario title>",
            e2e_titles
                .first()
                .map(String::as_str)
                .unwrap_or("No E2E interaction"),
        );
    let table = "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n";
    let rows = e2e_titles
        .iter()
        .map(|title| format!("| {capability} | exercise the capability | {title} |\n"))
        .collect::<String>();
    let original_row = format!(
        "| {capability} | exercise the capability | {} |\n",
        e2e_titles
            .first()
            .map(String::as_str)
            .unwrap_or("No E2E interaction")
    );
    if e2e_titles.is_empty() {
        document = document.replace(&format!("{table}{original_row}"), "- none\n");
    } else {
        document = document.replace(&format!("{table}{original_row}"), &format!("{table}{rows}"));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture use-case directory");
    }
    std::fs::write(path, document).expect("write fixture use-case document");
}

#[given(regex = r#"^a change seed at "([^"]+)" with:$"#)]
fn change_seed_at(world: &mut GivnWorld, path: String, step: &Step) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = step
        .docstring
        .as_ref()
        .expect("docstring content for change seed step");
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs for change seed");
    }
    std::fs::write(&file_path, content).expect("write change seed");
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
    ensure_fixture_usecase(env, &path, content);
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
    ensure_fixture_usecase(env, &path, content);
}

/// Extra environment overrides for the current subprocess. The
/// deterministic binary needs none; the helper exists so step definitions
/// share one subprocess-invocation path.
pub(crate) fn world_envs(_world: &GivnWorld) -> Vec<(&'static str, &'static str)> {
    Vec::new()
}

// --- When ------------------------------------------------------------------

#[when(regex = r"^the user runs `([^`]+)`$")]
fn user_runs(world: &mut GivnWorld, cmd: String) {
    let args = parse_command(&cmd);
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // The archive starts from a committed project state; generic fixtures
    // emulate the developer's commit. Dirty-tree scenarios use a dedicated
    // step that skips this.
    if world.snapshot_on_run {
        let root = world.cwd_override.clone().unwrap_or_else(|| env.root());
        world.scm_fixture.snapshot = Some(scm_backend_selection_steps::snapshot_tree(&root));
        world.snapshot_on_run = false;
    }
    if args.first().is_some_and(|arg| arg == "archive") && !world.skip_auto_commit {
        common::commit_fixture(env);
    }
    let argv: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let environments = world_envs(world);
    let out = match world.cwd_override.clone() {
        Some(cwd) => common::run_givn_with_envs_in(env, &cwd, &argv, &environments),
        None => common::run_givn_with_envs(env, &argv, &environments),
    };
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

#[then(regex = r#"^stderr does not contain "(.*)"$"#)]
fn stderr_does_not_contain(world: &mut GivnWorld, needle: String) {
    let s = stderr(world);
    assert!(
        !s.contains(&needle),
        "expected stderr to not contain {:?}\nactual stderr:\n{}",
        needle,
        s,
    );
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
        if *file == "proposal.md" {
            std::fs::write(change_dir.join(file), fixture_routing_proposal("example"))
                .expect("write proposal");
            continue;
        }
        std::fs::write(change_dir.join(file), "").expect("write artifact");
    }
    // Write DESIGN-REVIEW: PASS marker.
    std::fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    // Write all-checked tasks.md.
    std::fs::write(change_dir.join("tasks.md"), "- [x] done\n").unwrap();
    // Write REVIEW: PASS marker.
    std::fs::write(
        change_dir.join("review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none",
    )
    .unwrap();
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

// --- persona-identity-reuse: fixture Givens --------------------------------

#[given(regex = r#"^a change use-case document at "([^"]+)" records the persona "([^"]+)"$"#)]
fn change_usecase_document_records_persona(world: &mut GivnWorld, path: String, persona: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", "persona-change")
        .replace("<capability-id>", "persona-change")
        .replace("<scenario title>", "S")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace(
            "## Personas\n\n- none\n",
            &format!("## Personas\n\n- {persona}\n"),
        );
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs for use-case document");
    }
    std::fs::write(&file_path, document).expect("write use-case document");
}

#[given(regex = r#"^the change use-case document also records the persona "([^"]+)"$"#)]
fn change_usecase_document_also_records_persona(world: &mut GivnWorld, persona: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/persona-change/specs/persona-change/usecase.md");
    let content = std::fs::read_to_string(&path).expect("read use-case document");
    let updated = content.replace("## Personas\n\n", &format!("## Personas\n\n- {persona}\n"));
    std::fs::write(&path, updated).expect("append persona to use-case document");
}

#[given(regex = r#"^a project persona file "([^"]+)" exists$"#)]
fn project_persona_file_exists(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path(&format!("givn/personas/{id}.md"));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create personas dir");
    }
    std::fs::write(&path, format!("# Persona: {id}\n")).expect("write persona file");
}

#[given(regex = r#"^a topic persona file at "([^"]+)" exists$"#)]
fn topic_persona_file_exists(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create topic personas dir");
    }
    std::fs::write(&file_path, "# Persona: topic\n").expect("write topic persona file");
}
