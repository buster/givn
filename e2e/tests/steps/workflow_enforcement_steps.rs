use super::GivnWorld;
use crate::harness as common;
use cucumber::{given, then, when};
use std::process::Output;

fn output(world: &GivnWorld) -> &Output {
    world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet")
}

#[allow(dead_code)]
fn stdout(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&output(world).stdout).into_owned()
}

fn stderr(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&output(world).stderr).into_owned()
}

fn current_change_id(world: &GivnWorld) -> String {
    if let Some(id) = world.last_change_id.clone() {
        return id;
    }
    let root = project_root(world);
    let mut ids: Vec<String> = std::fs::read_dir(root.join("givn/changes"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .collect();
    ids.sort();
    assert_eq!(
        ids.len(),
        1,
        "exactly one active change is required: {ids:?}"
    );
    ids.remove(0)
}

fn project_root(world: &GivnWorld) -> std::path::PathBuf {
    world
        .env
        .as_ref()
        .map(|e| e.path(""))
        .expect("a project root is required first")
}

fn set_verify_command_in_config(world: &GivnWorld, cmd: &str) {
    let commands_path = project_root(world).join("givn/commands.yaml");
    if commands_path.exists() {
        let config = std::fs::read_to_string(&commands_path).unwrap();
        let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
        for line in &mut lines {
            let trimmed = line.trim_start();
            if trimmed.starts_with("command:") {
                let indent = line.len() - trimmed.len();
                *line = format!("{:indent$}command: \"{cmd}\"", "");
                break;
            }
        }
        std::fs::write(&commands_path, lines.join("\n")).unwrap();
        return;
    }
    let config_path = project_root(world).join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
    for line in &mut lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with("command:") {
            let indent = line.len() - trimmed.len();
            *line = format!("{:indent$}command: \"{cmd}\"", "");
            break;
        }
    }
    std::fs::write(&config_path, lines.join("\n")).unwrap();
}

fn set_verify_true(world: &GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::harness::remove_addon_locks(env);
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    set_verify_command_in_config(world, "./emit-result.sh regular");
    let commands_path = project_root(world).join("givn/commands.yaml");
    if commands_path.exists() {
        let content = std::fs::read_to_string(&commands_path).unwrap();
        let updated = content
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("e2e_command:") {
                    "  e2e_command: \"./emit-result.sh e2e\"".to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(commands_path, updated + "\n").unwrap();
    }
}

fn set_coverage_true(world: &GivnWorld) {
    crate::harness::remove_addon_locks(
        world
            .env
            .as_ref()
            .expect("a project root is required first"),
    );
    let cmds_path = project_root(world).join("givn/commands.yaml");
    let content = std::fs::read_to_string(&cmds_path).unwrap_or_default();
    let sentinel = "givn missing-coverage";
    let updated = content
        .replace(
            &format!("measure_command: \"{sentinel}\""),
            "measure_command: \"true\"",
        )
        .replace(
            &format!("merge_command: \"{sentinel}\""),
            "merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"",
        );
    if updated != content {
        std::fs::write(&cmds_path, updated).unwrap();
    }
}

fn disable_coverage_addon(world: &GivnWorld) {
    crate::harness::remove_addon_locks(
        world
            .env
            .as_ref()
            .expect("a project root is required first"),
    );
    let config_path = project_root(world).join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let updated = config.replace("coverage: true", "coverage: false");
    std::fs::write(&config_path, updated).unwrap();
}

fn disable_outline_addon(world: &GivnWorld) {
    crate::harness::remove_addon_locks(
        world
            .env
            .as_ref()
            .expect("a project root is required first"),
    );
    let config_path = project_root(world).join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let updated = config.replace("outline: true", "outline: false");
    std::fs::write(&config_path, updated).unwrap();
}

fn scaffold_init_root_change(world: &mut GivnWorld, id: &str) {
    world.env = Some(common::make_init_root());
    let env_ref = world.env.as_ref().unwrap();
    let out = common::run_givn(env_ref, &["new", id]);
    assert!(
        out.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some(id.to_string());
}

fn change_dir_for(world: &GivnWorld, id: &str) -> std::path::PathBuf {
    project_root(world).join(format!("givn/changes/{id}"))
}

fn behavior_snapshot(e2e: bool, interaction: Option<&str>) -> String {
    let e2e = if e2e { "true" } else { "false" };
    let interaction = interaction
        .map(|value| format!("\"{value}\""))
        .unwrap_or_else(|| "null".to_string());
    format!(
        r#"{{
  "schema": 1,
  "entries": [{{
    "feature_path": "old/capability.feature",
    "usecase": "old-usecase",
    "capability": "capability",
    "scenario": "Preserved behavior",
    "behavior_hash": "behavior-hash",
    "e2e": {e2e},
    "interaction": {interaction},
    "personas": []
  }}]
}}"#
    )
}

fn setup_migration(
    world: &mut GivnWorld,
    after: String,
    source_before: Option<&str>,
    source_after: Option<&str>,
) {
    world.env = Some(common::make_init_root());
    let migration_dir = project_root(world).join("givn/changes/migration-test");
    std::fs::create_dir_all(&migration_dir).unwrap();
    std::fs::write(
        migration_dir.join("coverage-before.json"),
        behavior_snapshot(true, Some("preserve behavior")),
    )
    .unwrap();
    std::fs::write(migration_dir.join("coverage-after.json"), after).unwrap();
    if let (Some(before), Some(after)) = (source_before, source_after) {
        std::fs::write(migration_dir.join("source-coverage-before.json"), before).unwrap();
        std::fs::write(migration_dir.join("source-coverage-after.json"), after).unwrap();
    }
}

#[given("a migration evidence change with matching behavior snapshots")]
fn matching_migration_snapshots(world: &mut GivnWorld) {
    setup_migration(
        world,
        behavior_snapshot(true, Some("preserve behavior")),
        None,
        None,
    );
}

/// Write the change's verification receipt by running the product's check.
///
/// The fixture declares a passing verification command; `givn check tasks`
/// executes it and writes the receipt, so the package never mirrors the
/// receipt's digests or counts.
fn write_fixture_change_receipt(world: &GivnWorld, id: &str) {
    let change = change_dir_for(world, id);
    let env = world.env.as_ref().expect("a fixture project");
    let head = match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        common::BackendKind::Fossil => common::fossil_revision(env),
    };
    common::proof_fixture::write_change_receipt(&change, id, &head, "./emit-result.sh regular");
}

#[given("a givn project with a change that has passed design-review")]
fn change_passed_design_review(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "test-change");
    set_verify_true(world);
    set_coverage_true(world);
    // These fixtures exercise the review gate, not the outline gate. The
    // outline addon is enabled by default, so opt this fixture out explicitly.
    disable_outline_addon(world);
    // Write design-review.md with PASS marker.
    let dr_path = change_dir_for(world, "test-change").join("design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
    // The visual-review fixtures name `fixture-persona`; the review-time
    // persona resolution needs the file to exist.
    let personas = project_root(world).join("givn/personas");
    std::fs::create_dir_all(&personas).unwrap();
    std::fs::write(personas.join("fixture-persona.md"), "# Persona: fixture\n").unwrap();
    declare_fixture_review_baseline(world, "test-change");
    write_fixture_change_receipt(world, "test-change");
}

fn declare_fixture_review_baseline(world: &GivnWorld, id: &str) {
    let change = change_dir_for(world, id);
    let proposal = change.join("proposal.md");
    let mut content = std::fs::read_to_string(&proposal).unwrap_or_default();
    if !content.ends_with('\n') && !content.is_empty() {
        content.push('\n');
    }
    content.push_str("\nBehavior change: none\n");
    std::fs::write(&proposal, content).unwrap();
    std::fs::write(change.join(".givn-skip"), "specs\n").unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
}

#[given(regex = r"^the file ([^ ]+) does not exist$")]
fn file_does_not_exist(_world: &mut GivnWorld, path: String) {
    let p = project_root(_world).join(&path);
    assert!(!p.exists(), "precondition failed: file {path} exists");
}

#[given("a givn project")]
fn givn_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("a clean committed givn project with a change whose delta specs pass")]
fn clean_project_with_passing_change(world: &mut GivnWorld) {
    crate::steps::archive_proof_support::ready_change(world, "verify-demo", false, "pass");
    // The review routing check resolves the declared NEW-in use case. Give the
    // fixture a permanent use case that owns the capability.
    let root = project_root(world);
    let usecase = root.join("givn/specs/demo-cap");
    std::fs::create_dir_all(&usecase).expect("create fixture usecase");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", "demo-cap")
        .replace("<capability-id>", "demo-cap")
        .replace("<scenario title>", "Existing scenario")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| demo-cap | <consumer action> | Existing scenario |",
            "- none",
        );
    std::fs::write(usecase.join("usecase.md"), document).expect("write fixture usecase");
    std::fs::write(
        usecase.join("demo-cap.feature"),
        "Feature: Demo Capability\n\n  Scenario: Existing scenario\n    Given an existing thing\n    Then it already works\n",
    )
    .expect("write fixture permanent feature");
    let proposal = root.join("givn/changes/verify-demo/proposal.md");
    std::fs::write(
        &proposal,
        "# Proposal: fixture\n\n## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | demo-cap | EXTEND demo-cap | `EXTEND demo-cap` | fixture |\n",
    )
    .expect("write fixture proposal");
    common::commit_fixture(world.env.as_ref().expect("a fixture project"));
    crate::steps::archive_proof_support::refresh_change_receipt(world, "verify-demo");
}

#[given("the change has no verification receipt")]
fn change_has_no_receipt(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let path = change_dir_for(world, &id).join("verification/tasks-verification.json");
    if path.exists() {
        std::fs::remove_file(&path).expect("remove verification receipt");
    }
    world.skip_receipt_autowrite = true;
}

#[given("a givn project with a result file path")]
fn project_with_result_path(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    world.skip_auto_commit = true;
}

#[when("the user runs the result helper for 3 total and 3 passed")]
fn run_result_helper(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root");
    let result = env.path("result.json");
    let path = result.to_string_lossy().into_owned();
    let out = common::run_givn_with_envs(
        env,
        &[
            "test-result",
            "--total",
            "3",
            "--passed",
            "3",
            "--failed",
            "0",
            "--skipped",
            "0",
        ],
        &[("GIVN_RESULT_FILE", &path)],
    );
    world.last_output = Some(out);
}

#[then("the result file contains total 3, passed 3, failed 0 and skipped 0 without a scope")]
fn result_file_has_counts(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("a project root");
    let value: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(env.path("result.json")).expect("result file"),
    )
    .expect("valid result JSON");
    assert_eq!(value["total"], 3);
    assert_eq!(value["passed"], 3);
    assert_eq!(value["failed"], 0);
    assert_eq!(value["skipped"], 0);
    assert!(value.get("scope").is_none());
}

fn write_review_fixture(
    world: &mut GivnWorld,
    id: &str,
    usecase_dir: &str,
    contract: &str,
    capabilities: &[&str],
    routing: &[(&str, &str)],
) {
    write_review_fixture_with_options(
        world,
        id,
        usecase_dir,
        contract,
        capabilities,
        routing,
        false,
    );
}

fn write_review_fixture_with_options(
    world: &mut GivnWorld,
    id: &str,
    usecase_dir: &str,
    contract: &str,
    capabilities: &[&str],
    routing: &[(&str, &str)],
    archive_ready: bool,
) {
    scaffold_init_root_change(world, id);
    let change = change_dir_for(world, id);
    let specs = change.join("specs").join(usecase_dir);
    std::fs::create_dir_all(&specs).unwrap();

    let mut proposal = format!(
        "# Proposal: {id}\n\n## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n"
    );
    for (capability, decision) in routing {
        proposal.push_str(&format!(
            "| {capability} | (fixture) | {decision} | fixture |\n"
        ));
    }
    std::fs::write(change.join("proposal.md"), proposal).unwrap();
    std::fs::write(specs.join("usecase.md"), contract).unwrap();
    for capability in capabilities {
        let feature = format!(
            "@givn.delta @{capability}\nFeature: {capability}\n\n  @givn.added @e2e\n  Scenario: Consumer runs {capability}\n    Given x\n    Then y\n"
        );
        std::fs::write(specs.join(format!("{capability}.feature")), feature).unwrap();
    }
    if archive_ready {
        disable_coverage_addon(world);
        disable_outline_addon(world);
        std::fs::write(
            change.join("tasks.md"),
            "- [x] COMMIT: a1b2c3d4 -- Fixture scenario\n",
        )
        .unwrap();
        std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
        std::fs::create_dir_all(project_root(world).join("src")).unwrap();
        std::fs::write(
            project_root(world).join("src/lib.rs"),
            "pub fn fixture() -> u32 { 1 }\n",
        )
        .unwrap();
    }
    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    let mut review = String::from(
        "README-IMPACT: none\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n",
    );
    for (index, left) in capabilities.iter().enumerate() {
        for right in capabilities.iter().skip(index + 1) {
            review.push_str(&format!(
                "| Fixture {left} | Fixture {right} | boundary |\n"
            ));
        }
    }
    review.push_str("\nREVIEW: PASS\n");
    std::fs::write(change.join("review.md"), review).unwrap();

    set_verify_true(world);
    common::commit_fixture(world.env.as_ref().unwrap());
    write_fixture_change_receipt(world, id);
}

fn review_contract(
    id: &str,
    title: Option<&str>,
    capabilities: &str,
    interactions: &str,
    examples: &str,
    extra: &str,
) -> String {
    review_contract_with_level(id, title, "!", capabilities, interactions, examples, extra)
}

fn review_contract_with_level(
    id: &str,
    title: Option<&str>,
    level: &str,
    capabilities: &str,
    interactions: &str,
    examples: &str,
    extra: &str,
) -> String {
    let default_title = format!("# Use case: {id}");
    let title = title.unwrap_or(&default_title);
    format!(
        "{title}\n\n\
         ## Level\n\n{level}\n\n\
         ## Actors\n\n- Developer\n\n\
         ## Goal\n\nCheck a change.\n\n\
         ## Trigger\n\nThe developer requests verification.\n\n\
         ## Preconditions\n\n- The change exists.\n\n\
         ## Main flow\n\n1. The developer requests verification.\n\n\
         ## Extensions\n\n- none\n\n\
         ## Rules\n\n- Structural failures outrank other findings.\n\n\
         ## Examples\n\n{examples}\n\n\
         {extra}\
         ## Minimal guarantee\n\nAn invalid change cannot pass.\n\n\
         ## Success guarantee\n\nThe developer receives a complete result.\n\n\
         ## Personas\n\n- developer--primary-user\n\n\
         ## Capabilities\n\n{capabilities}\n\n\
         ## Interactions\n\n{interactions}\n\n\
         ## Includes\n\n- fragment: support\n\n\
         ## Extends\n\n- none\n\n\
         ## Out of scope\n\nRunning the product suite.\n\n\
         ## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
    )
}

#[when(regex = r"^the developer runs `([^`]+)`$")]
fn developer_runs(world: &mut GivnWorld, cmd: String) {
    let args = crate::steps::parse_command(&cmd);
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &argv);
    if let Some(id) = args
        .iter()
        .position(|arg| arg == "--change")
        .and_then(|index| args.get(index + 1))
    {
        world.last_change_id = Some(id.clone());
    }
    world.last_output = Some(out);
}

fn valid_interactions(capabilities: &[&str]) -> String {
    let mut table =
        String::from("| Capability | Consumer action | E2E scenario |\n|---|---|---|\n");
    for capability in capabilities {
        table.push_str(&format!(
            "| {capability} | run {capability} | Consumer runs {capability} |\n"
        ));
    }
    table
}

#[given(
    "the use-case contract has a case-mismatched title, an unknown \"## Guarantees\" heading, a capability line that is not a bullet, an interaction that names an undeclared capability, and an example line that is not a bullet"
)]
fn review_fixture_all_schema_problems(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: review-diagnostics-demo"),
        "- alpha-capability\nthis line is not a bullet",
        "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| register-readiness | do it | Consumer does it |",
        "This line is prose, not a bullet.",
        "## Guarantees\n\nThis heading is not part of the schema.\n\n",
    );
    write_review_fixture(
        world,
        "review-diagnostics-demo",
        "review-diagnostics-demo",
        &contract,
        &["alpha-capability"],
        &[("alpha-capability", "NEW in review-diagnostics-demo")],
    );
}

#[given(
    "a clean committed change whose delta capability is routed \"NEW in review-diagnostics-demo\""
)]
fn review_fixture_routed_new(world: &mut GivnWorld) {
    // The following Given writes the contract and the fixture.
    world.last_change_id = Some("review-diagnostics-demo".to_string());
}

#[then(
    "stderr reports the title, the unknown heading, the capability line, the undeclared interaction, and the example line together in one review run"
)]
fn stderr_reports_all_schema_problems(world: &mut GivnWorld) {
    let text = stderr(world);
    for needle in [
        "expected '# Use case: <kebab-case-id>', got '# Use Case: review-diagnostics-demo'",
        "unknown heading '## Guarantees'",
        "section '## Capabilities' must contain one capability per bullet",
        "interaction references undeclared capability 'register-readiness'",
        "section '## Examples' must contain one example per bullet",
    ] {
        assert!(text.contains(needle), "stderr missing {needle:?}: {text}");
    }
}

#[then("stderr reports each distinct document problem once")]
fn stderr_reports_each_problem_once(world: &mut GivnWorld) {
    let text = stderr(world);
    for needle in [
        "got '# Use Case: review-diagnostics-demo'",
        "unknown heading '## Guarantees'",
        "interaction references undeclared capability 'register-readiness'",
    ] {
        assert_eq!(
            text.matches(needle).count(),
            1,
            "stderr repeated {needle:?}: {text}"
        );
    }
}

#[then("stdout says the change verification receipt was reused")]
fn stdout_says_receipt_reused(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        text.contains("reused across documentation-only revision"),
        "stdout missing reuse message: {text}"
    );
}

pub(crate) fn write_archive_ready_review_fixture(world: &mut GivnWorld, id: &str) {
    let contract = review_contract(
        id,
        None,
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
        "",
    );
    write_review_fixture_with_options(
        world,
        id,
        id,
        &contract,
        &["alpha-capability"],
        &[("alpha-capability", &format!("NEW in {id}"))],
        true,
    );
    world.verification_invocations_before = Some(change_suite_invocation_count(world));
    world.skip_receipt_autowrite = true;
}

pub(crate) fn change_suite_invocation_count(world: &GivnWorld) -> usize {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let root = project_root(world);
    // The log follows the change into the archive, so a count taken after the
    // archive reads the archived copy.
    let active = root.join(format!("givn/changes/{id}/verification/invocations.log"));
    let archived = root.join(format!("givn/archive/{id}/verification/invocations.log"));
    let path = if active.is_file() { active } else { archived };
    std::fs::read_to_string(path)
        .map(|text| {
            text.lines()
                .filter(|line| line.contains("givn/changes/"))
                .count()
        })
        .unwrap_or(0)
}

fn workflow_env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("a fixture project is required")
}

#[then("stdout reports passed change-scoped verification")]
fn stdout_reports_passed_change_verification(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(text.contains("change verification receipt:"), "{text}");
    assert!(text.contains("0 failed"), "{text}");
}

#[then(
    "the change contains a passed receipt bound to the revision, selected features, runner command and relevant inputs captured before execution"
)]
fn receipt_bound_to_inputs(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let change = change_dir_for(world, &id);
    let path = change.join("verification/tasks-verification.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let receipt: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
    assert_eq!(receipt["status"], serde_json::json!("passed"));
    assert!(!receipt["head"].as_str().unwrap_or_default().is_empty());
    assert!(
        !receipt["features_digest"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
    assert!(
        !receipt["command_digest"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
    assert!(receipt["tested_content_digest"].is_string());
    assert_eq!(
        receipt["content_scope"],
        serde_json::json!("change-proof/v1")
    );
    assert_eq!(receipt["result"]["total"], receipt["result"]["passed"]);
}

fn write_recording_runner(world: &GivnWorld) {
    let path = workflow_env(world).path("record-runner.sh");
    std::fs::write(
        &path,
        "#!/bin/sh\nprintf 'invoked' > verification-invoked\nexit 9\n",
    )
    .expect("write recording runner");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("chmod recording runner");
    let commands = workflow_env(world).path("givn/commands.yaml");
    let content = std::fs::read_to_string(&commands).expect("read commands.yaml");
    let updated = common::replace_top_level_key(
        &content,
        "verify",
        "verify:\n  command: \"./record-runner.sh\"\n  e2e_command: \"./record-runner.sh\"\n",
    );
    std::fs::write(commands, updated).expect("write commands.yaml");
}

#[given("a givn project whose configured hooks and verification commands record execution")]
fn project_records_hooks_and_verification(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = workflow_env(world);
    let hook = env.path("record-hook.sh");
    std::fs::write(&hook, "#!/bin/sh\nprintf 'hook' > hook-invoked\n").expect("write hook");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod hook");
    write_recording_runner(world);
    let config = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&config).expect("read config");
    std::fs::write(
        config,
        format!(
            "{content}\nartifacts:\n  - id: proposal\n    post:\n      - \"./record-hook.sh\"\n"
        ),
    )
    .expect("write config hook");
    common::commit_fixture(env);
}

#[when("the consumer invokes `givn check`")]
fn consumer_invokes_bare_check(world: &mut GivnWorld) {
    let out = common::run_givn(workflow_env(world), &["check"]);
    world.last_output = Some(out);
}

#[then("terminal output explains that no checks were executed")]
fn terminal_explains_no_checks(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(text.contains("No checks were executed"), "{text}");
}

#[then("terminal output lists explicit check commands and labels which execute runners")]
fn terminal_lists_checks(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(text.contains("givn check tasks"), "{text}");
    assert!(
        text.contains("executes changed-scenario verification"),
        "{text}"
    );
    assert!(
        text.contains("without running verification suites") || text.contains("checks evidence"),
        "{text}"
    );
}

#[then("no configured hook or runner executes")]
fn no_hook_or_runner(world: &mut GivnWorld) {
    let env = workflow_env(world);
    assert!(!env.path("hook-invoked").exists());
    assert!(!env.path("verification-invoked").exists());
}

/// Write a change receipt by running the product's own change-scoped check.
///
/// Used where the receipt must carry the tested-content digest (the
/// documentation-only reuse rule), which a fixture mirror cannot compute
/// without the product's own manifest resolution.
pub(crate) fn write_product_receipt(world: &GivnWorld, id: &str) {
    let env = world.env.as_ref().expect("a fixture project");
    let out = common::run_givn(env, &["check", "tasks", "--change", id]);
    assert!(
        out.status.success(),
        "givn check tasks --change {id} failed while writing the content-bound receipt: \
         stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
