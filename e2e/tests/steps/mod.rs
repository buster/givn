//! The package's Gherkin step definitions and world.
//!
//! One module per capability area. Every definition here is bound to a
//! scenario the `@e2e` scope selects; component-only definitions stay in the
//! product's `tests/steps/`. The `.feature` files stay shared, so this tree
//! binds the same phrases against the built binary instead of the library.

use std::process::Output;

use cucumber::{World, gherkin::Step, given, then, when};

use crate::harness;

pub mod adr_qualification_gate_e2e_steps;
pub mod agent_activity_reporting_e2e_steps;
pub mod arc42_review_alignment_steps;
pub mod archive_proof_e2e_steps;
pub mod archive_proof_steps;
pub mod archive_proof_support;
pub mod change_strategy_e2e_steps;
pub mod clean_room_preparation_e2e_steps;
pub mod clean_room_preparation_steps;
pub mod coverage_readme_example_steps;
pub mod coverage_script_workflow_e2e_steps;
pub mod coverage_script_workflow_steps;
pub mod deduplicate_change_dir_detection_steps;
pub mod default_all_addons_steps;
pub mod development_record_e2e_steps;
pub mod development_record_steps;
pub mod development_report_e2e_steps;
pub mod development_report_steps;
pub mod development_timeline_support;
pub mod enable_optional_addons_ise_e2e_steps;
pub mod enable_optional_addons_ise_steps;
pub mod enforce_post_merge_e2e_gate_steps;
pub mod extract_coverage_gate_steps;
pub mod fix_domain_modeling_skill_trigger_e2e_steps;
pub mod fix_domain_modeling_skill_trigger_steps;
pub mod fix_uppercase_checkbox_parsing_steps;
pub mod grouped_merge_steps;
pub mod init_agent_target_selection_steps;
pub mod initialize_a_project_e2e_steps;
pub mod instruction_extension_e2e_steps;
pub mod instruction_extension_steps;
pub mod instruction_integrity_e2e_steps;
pub mod instruction_integrity_steps;
pub mod ise_clean_room_support;
pub mod llm_attribution_e2e_steps;
pub mod llm_attribution_installation_e2e_steps;
pub mod llm_attribution_installation_steps;
pub mod llm_attribution_steps;
pub mod outline_report_e2e_steps;
pub mod outline_report_steps;
pub mod outline_report_support;
pub mod preserve_docstrings_on_archive_steps;
pub mod prose_consolidation_e2e_steps;
pub mod prose_consolidation_steps;
pub mod readme_coverage_badge_steps;
pub mod readme_enforcement_e2e_steps;
pub mod readme_enforcement_steps;
pub mod release_0_3_0_e2e_steps;
pub mod release_0_3_0_steps;
pub mod release_e2e_steps;
pub mod release_steps;
pub mod remove_dead_skip_list_code_steps;
pub mod repository_ownership_fixtures_steps;
pub mod repository_ownership_fixtures_support;
pub mod require_durable_retained_paths_e2e_steps;
pub mod require_durable_retained_paths_steps;
pub mod require_merged_step_definitions_e2e_steps;
pub mod require_merged_step_definitions_steps;
pub mod review_disposition_gates_steps;
pub mod run_command_steps;
pub mod run_declaration_fixtures_support;
pub mod run_execution_e2e_steps;
pub mod run_execution_steps;
pub mod run_execution_support;
pub mod run_upgrade_steps;
pub mod scm_backend_selection_e2e_steps;
pub mod scm_backend_selection_steps;
pub mod serve_extended_rules_steps;
pub mod skill_rewrites_steps;
pub mod spec_discovery_deterministic_steps;
pub mod spec_gap_e2e_steps;
pub mod spec_gap_steps;
pub mod spec_overlap_gates_steps;
pub mod specification_declaration_e2e_steps;
pub mod template_conditional_rendering_e2e_steps;
pub mod template_conditional_rendering_steps;
pub mod template_conditional_rendering_support;
pub mod trial_input_verification_e2e_steps;
pub mod trial_input_verification_steps;
pub mod validate_artifact_id_input_steps;
pub mod validate_change_id_input_steps;
pub mod visual_review_gate_steps;
pub mod workflow_enforcement_e2e_steps;
pub mod workflow_enforcement_steps;

use crate::harness as common;

#[derive(Debug, Default, World)]
pub struct GivnWorld {
    /// The scenario currently executing, set by the runner's before hook.
    pub(crate) scenario_title: Option<String>,
    /// The temp project root (init'd or bare) for this scenario.
    pub(crate) env: Option<harness::TestEnv>,
    /// Captured stdout/stderr/exit of the last `givn` invocation.
    pub(crate) last_output: Option<Output>,
    /// The change id parsed from the last `givn new <id>` command.
    pub(crate) last_change_id: Option<String>,
    /// The command name of the last `givn` invocation.
    pub(crate) last_command: Option<String>,
    /// Combined output captured by a multi-command guidance scenario.
    pub(crate) additional_output: Option<String>,
    /// Working directory override for the next `givn` invocation.
    pub(crate) cwd_override: Option<std::path::PathBuf>,
    /// Skip the generic archive auto-commit for intentionally dirty fixtures.
    pub(crate) skip_auto_commit: bool,
    /// Take the tree snapshot right before the next invocation.
    pub(crate) snapshot_on_run: bool,
    /// Keep a deliberately missing or stale change-scoped review receipt.
    pub(crate) skip_receipt_autowrite: bool,
    /// Number of change-suite invocations recorded before a review/archive run.
    pub(crate) verification_invocations_before: Option<usize>,
    /// The verification invocation log before an archive run.
    pub(crate) verification_log_before: Option<String>,
    /// The tested-content boundary a stale-receipt outline example changed.
    pub(crate) changed_boundary: Option<String>,
    /// Reserved fixed port written into the fixture's run declaration.
    pub(crate) run_fixed_port: Option<u16>,
    /// Caller-provided port handed to `givn run --port`.
    pub(crate) run_caller_port: Option<u16>,
    /// The start command declared by the fixture's run declaration.
    pub(crate) run_declared_command: Option<String>,
    /// Marker file the fixture writes when it receives a signal.
    pub(crate) run_trap_marker: Option<std::path::PathBuf>,
    /// State for the commit-attribution scenarios.
    pub(crate) git_attribution: Option<llm_attribution_steps::GitAttributionState>,
    /// State for the generated agent-integration scenarios.
    pub(crate) opencode_plugin: Option<llm_attribution_e2e_steps::OpencodePluginState>,
    pub(crate) claude_attribution: Option<llm_attribution_e2e_steps::ClaudeAttributionState>,
    pub(crate) omp_attribution: Option<llm_attribution_e2e_steps::OmpAttributionState>,
    pub(crate) maki_attribution: Option<llm_attribution_e2e_steps::MakiAttributionState>,
    /// State for the SCM backend selection scenarios.
    pub(crate) scm_fixture: scm_backend_selection_steps::ScmFixture,
    /// State for the release-truth changelog and guard scenarios.
    pub(crate) release: release_steps::ReleaseState,
    /// State for the `development-record` scenarios.
    pub(crate) development_record: Option<development_record_steps::DevelopmentRecordState>,
    /// State for the `development-report` scenarios.
    pub(crate) development_report: Option<development_report_steps::DevelopmentReportState>,
    /// State for the `@e2e` `agent-activity-reporting` scenarios.
    pub(crate) agent_activity_reporting_e2e:
        Option<agent_activity_reporting_e2e_steps::AgentActivityReportingE2eState>,
    /// State for the `@e2e` `development-record` scenarios.
    pub(crate) development_record_e2e:
        Option<development_record_e2e_steps::DevelopmentRecordE2eState>,
    /// State for the `@e2e` `development-report` scenarios.
    pub(crate) development_report_e2e:
        Option<development_report_e2e_steps::DevelopmentReportE2eState>,
    /// State for the clean-room trial scenarios.
    pub(crate) ise_clean_room: ise_clean_room_support::IseCleanRoomState,
}

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

pub(crate) fn parse_command(raw: &str) -> Vec<String> {
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

fn change_id_from_args(args: &[String]) -> Option<String> {
    if args.first()?.as_str() != "new" {
        return None;
    }
    args.iter().skip(1).rfind(|a| a.as_str() != "--").cloned()
}

#[given("a project root with givn/ initialised")]
fn init_root(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("a project root without a givn/ directory")]
fn bare_root(world: &mut GivnWorld) {
    world.env = Some(common::make_bare_root());
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
    if let Some(change) = path
        .strip_prefix("givn/changes/")
        .and_then(|rest| rest.split_once("/specs/"))
        .map(|(change, _)| change)
    {
        common::clear_no_behavior_fixture(env, change);
        common::declare_fixture_routing(env, change);
    }
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
    let is_fragment = usecase == "fragments";
    let path = env.path(&format!(
        "givn/specs/{usecase}/{}",
        if is_fragment {
            "fragment.md"
        } else {
            "usecase.md"
        }
    ));
    if path.exists() {
        return;
    }
    let e2e_titles: Vec<String> =
        match gherkin::Feature::parse(content, gherkin::GherkinEnv::default()) {
            Ok(feature) => {
                let inherited = feature.tags.iter().any(|tag| tag == "e2e");
                feature
                    .scenarios
                    .iter()
                    .filter(|scenario| inherited || scenario.tags.iter().any(|tag| tag == "e2e"))
                    .map(|scenario| scenario.name.clone())
                    .collect()
            }
            Err(_) => Vec::new(),
        };
    let table = "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n";
    let rows = e2e_titles
        .iter()
        .map(|title| format!("| {capability} | exercise the capability | {title} |\n"))
        .collect::<String>();
    let mut document = if is_fragment {
        crate::harness::fragment_md_template()
            .replace("<fragment-id>", usecase)
            .replace("<capability-id>", capability)
    } else {
        crate::harness::usecase_md_template()
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
            )
    };
    if is_fragment {
        if !e2e_titles.is_empty() {
            document = document.replace("- none\n", &format!("{table}{rows}"));
        }
    } else {
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
            document =
                document.replace(&format!("{table}{original_row}"), &format!("{table}{rows}"));
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture use-case directory");
    }
    std::fs::write(path, document).expect("write fixture use-case document");
}

pub(crate) fn world_envs(_world: &GivnWorld) -> Vec<(&'static str, &'static str)> {
    Vec::new()
}

pub(crate) fn user_runs_quoted(world: &mut GivnWorld, cmd: String) {
    user_runs(world, cmd);
}

#[when(regex = r"^the user runs `([^`]+)`$")]
fn user_runs(world: &mut GivnWorld, cmd: String) {
    let args = parse_command(&cmd);
    world.last_command = args.first().cloned();
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
    if args.first().is_some_and(|arg| arg == "check")
        && args.get(1).is_some_and(|arg| arg == "tasks")
        && !world.skip_auto_commit
    {
        common::commit_fixture(env);
    }
    if args.first().is_some_and(|arg| arg == "archive") && !world.skip_auto_commit {
        common::commit_fixture(env);
        if let Some(id) = world.last_change_id.clone() {
            archive_proof_support::refresh_change_receipt(world, &id);
        }
    }
    if args.first().is_some_and(|arg| arg == "check")
        && args.get(1).is_some_and(|arg| arg == "review")
        && !world.skip_receipt_autowrite
        && let Some(id) = world.last_change_id.clone()
    {
        archive_proof_support::refresh_change_receipt(world, &id);
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

#[then(regex = r#"^stdout confirms integrity checks passed$"#)]
fn stdout_confirms_integrity_passed(world: &mut GivnWorld) {
    let s = stdout(world);
    assert!(
        s.contains("integrity") && (s.contains("passed") || s.contains("PASS")),
        "stdout missing integrity passed confirmation: {}",
        s,
    );
}

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

#[then(regex = r#"^stderr contains "(.*)"$"#)]
fn stderr_contains(world: &mut GivnWorld, needle: String) {
    let s = stderr(world);
    assert!(s.contains(&needle), "stderr missing {:?}: {}", needle, s);
}

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
    crate::harness::remove_addon_locks(env);
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

#[given(regex = r#"^a change "([^"]+)" exists with all required artifacts complete$"#)]
fn change_all_required_done(world: &mut GivnWorld, id: String) {
    let env = world.env.get_or_insert_with(common::make_init_root);
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
        "README-IMPACT: none\n\nREVIEW: PASS",
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
