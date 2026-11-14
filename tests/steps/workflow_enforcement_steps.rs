use cucumber::{given, then, when};
use std::process::Output;

use super::GivnWorld;
use crate::common;
use crate::steps::e2e_boundary_support;

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

fn change_id(world: &GivnWorld) -> String {
    world
        .last_change_id
        .clone()
        .expect("a change id is required")
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

fn change_dir(world: &GivnWorld) -> std::path::PathBuf {
    project_root(world).join(format!("givn/changes/{}", change_id(world)))
}

// ---- Given helpers ----

fn default_valid_tasks_md() -> String {
    "- [x] COMMIT: a1b2c3d4 -- Implement scenario one\n".to_string()
}

fn default_valid_design_md() -> String {
    "## Step Definitions\n\n| Capability | Step definition file |\n|---|---|\n| test-cap | tests/steps/test_steps.rs |\n".to_string()
}

fn create_valid_step_file(world: &GivnWorld) {
    let path = project_root(world).join("tests/steps/test_steps.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "// test step definitions\n").unwrap();
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
    crate::common::remove_addon_locks(env);
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

/// Create a verify script that exits 0 during gate hooks (GIVN_FEATURES points to
/// a change specs dir containing "changes") and exits 1 during post-merge verify
/// (GIVN_FEATURES points to "givn/specs").
fn set_coverage_true(world: &GivnWorld) {
    crate::common::remove_addon_locks(
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
    crate::common::remove_addon_locks(
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
    crate::common::remove_addon_locks(
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

fn set_verify_fails_post_merge(world: &GivnWorld) {
    let script_path = project_root(world).join(".verify_gate.sh");
    std::fs::write(&script_path, "#!/bin/sh\nexit 1\n").unwrap();
    std::fs::set_permissions(
        &script_path,
        std::os::unix::fs::PermissionsExt::from_mode(0o755),
    )
    .unwrap();
    set_verify_command_in_config(world, &script_path.to_string_lossy());
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

#[given("a migration evidence change whose after snapshot is missing one behavior")]
fn missing_behavior_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(
        world,
        r#"{"schema":1,"entries":[]}"#.to_string(),
        None,
        None,
    );
}

#[given("a migration evidence change that loses E2E evidence")]
fn lost_e2e_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(
        world,
        behavior_snapshot(false, Some("preserve behavior")),
        None,
        None,
    );
}

#[given("a migration evidence change with lower interaction coverage")]
fn lower_interaction_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(world, behavior_snapshot(true, None), None, None);
}

#[given("a migration evidence change with lower line coverage")]
fn lower_line_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(
        world,
        behavior_snapshot(true, Some("preserve behavior")),
        Some(r#"{"lines_covered":90,"lines_valid":100,"branches_covered":9,"branches_valid":10}"#),
        Some(r#"{"lines_covered":89,"lines_valid":100,"branches_covered":9,"branches_valid":10}"#),
    );
}

#[given("a migration evidence change with lower branch coverage")]
fn lower_branch_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(
        world,
        behavior_snapshot(true, Some("preserve behavior")),
        Some(r#"{"lines_covered":90,"lines_valid":100,"branches_covered":9,"branches_valid":10}"#),
        Some(r#"{"lines_covered":90,"lines_valid":100,"branches_covered":8,"branches_valid":10}"#),
    );
}

#[given("a migration evidence change with an explicitly retired behavior")]
fn explicitly_retired_migration_snapshot(world: &mut GivnWorld) {
    setup_migration(
        world,
        r#"{"schema":1,"entries":[]}"#.to_string(),
        None,
        None,
    );
    std::fs::write(
        project_root(world).join("givn/changes/migration-test/usecase-migration.md"),
        "| Old entry | Old owner | Operation | New owner |\n|---|---|---|---|\n| old/capability.feature — Preserved behavior | old | RETIRE | none |\n",
    )
    .unwrap();
}

// ---- Given steps ----

fn write_fixture_change_receipt(world: &GivnWorld, id: &str) {
    let root = project_root(world);
    let change = change_dir_for(world, id);
    let env = world.env.as_ref().expect("a fixture project");
    let head = match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        common::BackendKind::Fossil => {
            String::from_utf8_lossy(&common::fossil(env, &["info"]).stdout)
                .lines()
                .find_map(|line| line.strip_prefix("checkout:"))
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or_default()
                .to_string()
        }
    };
    let backend = givn::scm::select(&root).expect("select backend for receipt");
    let manifest = givn::config::manifest::Manifest::load_embedded().expect("embedded manifest");
    let tested_content_digest = givn::proof::digest_tested_content(&root, &backend, id, &manifest)
        .expect("digest tested content");
    givn::proof::write_change_receipt(
        &root,
        &change,
        id,
        &head,
        "./emit-result.sh regular",
        &tested_content_digest,
        givn::proof::RunnerResult {
            scope: None,
            total: 2,
            passed: 2,
            failed: 0,
            skipped: 0,
        },
        1,
    )
    .expect("write fixture change receipt");
}

fn ensure_demo_cap_baseline(env: &common::TestEnv) {
    let dir = env.path("givn/specs/demo-cap");
    std::fs::create_dir_all(&dir).expect("create permanent demo-cap dir");
    let usecase = dir.join("usecase.md");
    if !usecase.exists() {
        let document = givn::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", "demo-cap")
            .replace("<capability-id>", "demo-cap")
            .replace("<actor role>", "Fixture operator")
            .replace("<the domain goal>", "Exercise the fixture behavior.")
            .replace("<consumer action>", "submit an order")
            .replace("<scenario title>", "Consumer submits an order");
        std::fs::write(&usecase, document).expect("write permanent usecase");
    }
    let feature = dir.join("demo-cap.feature");
    if !feature.exists() {
        std::fs::write(
            &feature,
            "Feature: Demo Capability\n\n  @e2e\n  Scenario: Consumer submits an order\n    Given a thing\n    Then it works\n",
        )
        .expect("write permanent feature");
    }
}

fn e2e_titles(path: &std::path::Path) -> Vec<String> {
    gherkin::Feature::parse_path(path, gherkin::GherkinEnv::default())
        .map(|feature| {
            feature
                .scenarios
                .iter()
                .filter(|scenario| scenario.tags.iter().any(|tag| tag == "e2e"))
                .map(|scenario| scenario.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// Add an E2E scenario to the active change's `demo-cap` delta. The fixture
/// starts from the review or archive baseline and leaves a committed, fresh
/// change receipt.
fn add_delta_e2e(world: &GivnWorld, title: &str, with_row: bool) {
    let env = world.env.as_ref().expect("a fixture project");
    let id = change_id(world);
    let change = env.path(&format!("givn/changes/{id}"));
    let specs = change.join("specs/demo-cap");
    std::fs::create_dir_all(&specs).expect("create delta specs dir");
    ensure_demo_cap_baseline(env);

    let scenario =
        format!("  @givn.added @e2e\n  Scenario: {title}\n    Given a thing\n    Then it works\n");
    let feature_path = specs.join("demo-cap.feature");
    if feature_path.exists() {
        let mut content = std::fs::read_to_string(&feature_path).expect("read delta feature");
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&scenario);
        std::fs::write(&feature_path, content).expect("append delta scenario");
    } else {
        std::fs::write(
            &feature_path,
            format!("@givn.delta @demo-cap\nFeature: Demo Capability\n\n{scenario}"),
        )
        .expect("write delta feature");
    }

    if with_row {
        let mut titles = e2e_titles(&env.path("givn/specs/demo-cap/demo-cap.feature"));
        titles.extend(e2e_titles(&feature_path));
        titles.sort();
        titles.dedup();
        let rows = titles
            .iter()
            .map(|title| format!("| demo-cap | exercise the capability | {title} |"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut document = givn::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", "demo-cap")
            .replace("<capability-id>", "demo-cap")
            .replace("<actor role>", "Fixture operator")
            .replace("<the domain goal>", "Exercise the fixture behavior.")
            .replace("<consumer action>", "exercise the capability");
        document = document.replace(
            "| demo-cap | exercise the capability | <scenario title> |",
            &rows,
        );
        if let Ok(permanent) = std::fs::read_to_string(env.path("givn/specs/demo-cap/usecase.md")) {
            let hash: String = {
                use sha2::Digest;
                sha2::Sha256::digest(permanent.as_bytes())
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect()
            };
            document.push_str(&format!("\n<!-- givn:base-sha256: {hash} -->\n"));
        }
        std::fs::write(specs.join("usecase.md"), document).expect("write delta usecase");
    }

    common::clear_no_behavior_fixture(env, &id);
    common::declare_fixture_routing(env, &id);
    common::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
}

#[given(
    regex = r#"^the change's delta adds an E2E scenario "([^"]+)" without an interaction row$"#
)]
fn delta_adds_e2e_without_row(world: &mut GivnWorld, title: String) {
    add_delta_e2e(world, &title, false);
}

#[given(regex = r#"^the change's delta adds an E2E scenario "([^"]+)" with its interaction row$"#)]
fn delta_adds_e2e_with_row(world: &mut GivnWorld, title: String) {
    add_delta_e2e(world, &title, true);
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

/// Model a change that reached review without executable behavior changes.
///
/// The review gate demands either delta feature files or an explicit
/// no-behavior-change declaration with a skipped specs artifact. A scenario
/// that later writes a delta removes the declaration through its own fixture
/// step.
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

#[given(regex = r#"^the change's proposal records review kind "([^"]+)"$"#)]
fn proposal_records_review_kind(world: &mut GivnWorld, kind: String) {
    let path = change_dir(world).join("proposal.md");
    let mut content = std::fs::read_to_string(&path).unwrap_or_default();
    if !content.ends_with('\n') && !content.is_empty() {
        content.push('\n');
    }
    content.push_str(&format!("\nReview kind: {kind}\n"));
    std::fs::write(path, content).unwrap();
}

fn write_review_record(path: &std::path::Path, kind: &str, author: &str, reviewer: &str) {
    std::fs::write(
        path,
        format!(
            "Review kind: {kind}\nAuthor: {author}\nReviewer context: {reviewer}\n\nREADME-IMPACT: none\n\nREVIEW: PASS\n"
        ),
    )
    .unwrap();
}

#[given(
    regex = r#"^the change's design-review records review kind "([^"]+)" by author "([^"]+)" in reviewer context "([^"]+)"$"#
)]
fn design_review_records_review_context(
    world: &mut GivnWorld,
    kind: String,
    author: String,
    reviewer: String,
) {
    write_review_record(
        &change_dir(world).join("design-review.md"),
        &kind,
        &author,
        &reviewer,
    );
    let mut content = std::fs::read_to_string(change_dir(world).join("design-review.md")).unwrap();
    content.push_str("\nDESIGN-REVIEW: PASS\n");
    std::fs::write(change_dir(world).join("design-review.md"), content).unwrap();
}

#[given(
    regex = r#"^the change's review records review kind "([^"]+)" by author "([^"]+)" in reviewer context "([^"]+)"$"#
)]
fn review_records_review_context(
    world: &mut GivnWorld,
    kind: String,
    author: String,
    reviewer: String,
) {
    write_review_record(
        &change_dir(world).join("review.md"),
        &kind,
        &author,
        &reviewer,
    );
}

#[when("the user runs `givn check review --change test-change` twice")]
fn run_check_review_twice(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let first = common::run_givn(env, &["check", "review", "--change", "test-change"]);
    let second = common::run_givn(env, &["check", "review", "--change", "test-change"]);
    world.previous_output = Some(first);
    world.last_output = Some(second);
}

#[then("both runs exit non-zero")]
fn both_runs_exit_non_zero(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("the first run must be recorded");
    let second = output(world);
    assert!(!first.status.success(), "first run unexpectedly succeeded");
    assert!(
        !second.status.success(),
        "second run unexpectedly succeeded"
    );
}

#[when("the developer reads the review instruction")]
fn developer_reads_review_instruction(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let changes = env.path("givn/changes");
    let has_change = std::fs::read_dir(&changes)
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false);
    if !has_change {
        let out = common::run_givn(env, &["new", "instruction-demo"]);
        assert!(
            out.status.success(),
            "givn new failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    world.last_output = Some(common::run_givn(env, &["instructions", "review"]));
}

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn review_instruction_stdout(world: &GivnWorld) -> String {
    let out = output(world);
    assert!(
        out.status.success(),
        "givn instructions review failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[then("the review instruction says the default is `Review kind: self-review`")]
fn review_instruction_names_default_kind(world: &mut GivnWorld) {
    let text = review_instruction_stdout(world);
    assert!(
        normalized(&text).contains("Review kind: self-review"),
        "review instruction does not name the self-review default: {text}"
    );
}

#[then("the review instruction permits exactly one fresh context for a major change")]
fn review_instruction_permits_one_context(world: &mut GivnWorld) {
    let text = review_instruction_stdout(world);
    assert!(
        normalized(&text).contains("exactly one fresh context"),
        "review instruction does not permit exactly one fresh context: {text}"
    );
}

#[then("the review instruction says one context covers design review and final review")]
fn review_instruction_shares_one_context(world: &mut GivnWorld) {
    let text = review_instruction_stdout(world);
    assert!(
        normalized(&text)
            .contains("One context covers both the design-review and final-review obligations"),
        "review instruction does not state the shared-context rule: {text}"
    );
}

#[then("the review instruction forbids more than one review context")]
fn review_instruction_forbids_second_context(world: &mut GivnWorld) {
    let text = review_instruction_stdout(world);
    assert!(
        normalized(&text).contains("Never start more than one review context"),
        "review instruction does not forbid a second review context: {text}"
    );
}

#[then("the review instruction says the reviewer must not be the author")]
fn review_instruction_requires_different_author(world: &mut GivnWorld) {
    let text = review_instruction_stdout(world);
    assert!(
        normalized(&text).contains("must not be the author"),
        "review instruction does not separate author and reviewer: {text}"
    );
}

const GUIDANCE_ROOTS: &[&str] = &[
    "assets/instructions",
    "assets/commands",
    "assets/skills",
    "assets/templates",
];

fn collect_guidance_files(root: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_guidance_files(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

fn guidance_asset_paths() -> Vec<std::path::PathBuf> {
    let repo = crate::common::repo_path("");
    let mut paths = Vec::new();
    for root in GUIDANCE_ROOTS {
        collect_guidance_files(&repo.join(root), &mut paths);
    }
    let hint = repo.join("assets/agents-project-hint.md");
    if hint.is_file() {
        paths.push(hint);
    }
    paths.sort();
    paths
}

#[when("the guidance assets are scanned for subagent vocabulary")]
fn scan_guidance_assets(world: &mut GivnWorld) {
    let mut subagent_matches = Vec::new();
    let mut prescriptions = Vec::new();
    let forbidden = [
        "fresh-context agent",
        "delegate to a fresh context",
        "fresh delegated context",
        "architect subagent",
        "grilling subagent",
        "hardening subagent",
        "coding subagent",
        "review subagent",
        "spawn both subagents",
        "start one fresh review subagent",
    ];
    for path in guidance_asset_paths() {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let lower = text.to_lowercase();
        let relative = path
            .strip_prefix(crate::common::repo_path(""))
            .unwrap_or(&path)
            .display()
            .to_string();
        if lower.contains("subagent") || lower.contains("sub-agent") {
            subagent_matches.push(relative.clone());
        }
        if relative != "assets/instructions/review.md" {
            for phrase in forbidden {
                if lower.contains(phrase) {
                    prescriptions.push(format!("{relative}: {phrase}"));
                }
            }
        }
    }
    world.additional_output = Some(format!(
        "subagent matches:\n{}\nprescriptions:\n{}",
        subagent_matches.join("\n"),
        prescriptions.join("\n")
    ));
}

#[then("only assets/instructions/review.md names a review subagent")]
fn only_review_instruction_names_subagent(world: &mut GivnWorld) {
    let text = world
        .additional_output
        .as_deref()
        .expect("the guidance assets were not scanned");
    let matches = text
        .split("prescriptions:")
        .next()
        .unwrap_or("")
        .trim_start_matches("subagent matches:")
        .trim();
    let paths: Vec<&str> = matches.lines().filter(|line| !line.is_empty()).collect();
    assert_eq!(
        paths,
        vec!["assets/instructions/review.md"],
        "guidance assets other than the review instruction name a subagent: {matches}"
    );
}

#[then(
    "no other guidance asset prescribes a separate agent, session, architect, or reviewer to execute a step"
)]
fn no_other_guidance_prescribes_delegation(world: &mut GivnWorld) {
    let text = world
        .additional_output
        .as_deref()
        .expect("the guidance assets were not scanned");
    let prescriptions = text.split("prescriptions:").nth(1).unwrap_or("").trim();
    assert!(
        prescriptions.is_empty(),
        "guidance assets still prescribe execution topology:\n{prescriptions}"
    );
}

#[given(regex = r"^the change's tasks\.md contains `(.+)`$")]
fn tasks_md_contains(world: &mut GivnWorld, content: String) {
    std::fs::write(change_dir(world).join("tasks.md"), &content).unwrap();
}

#[given("the change's tasks.md contains valid commit hashes")]
fn tasks_md_valid_hashes(world: &mut GivnWorld) {
    std::fs::write(change_dir(world).join("tasks.md"), default_valid_tasks_md()).unwrap();
}

#[given("the change's design.md contains a Step Definitions table with valid step file paths")]
fn design_md_valid_table(world: &mut GivnWorld) {
    std::fs::write(
        change_dir(world).join("design.md"),
        default_valid_design_md(),
    )
    .unwrap();
    create_valid_step_file(world);
}

#[given("the change's design.md contains a valid Step Definitions table with existing files")]
fn design_md_valid_table_existing(world: &mut GivnWorld) {
    std::fs::write(
        change_dir(world).join("design.md"),
        default_valid_design_md(),
    )
    .unwrap();
    create_valid_step_file(world);
}

#[given(regex = r"^the change's design\.md Step Definitions table lists `([^`]+)`$")]
fn design_md_lists_file(world: &mut GivnWorld, file_path: String) {
    let content = format!(
        "## Step Definitions\n\n| Capability | Step definition file |\n|---|---|\n| test-cap | {file_path} |\n"
    );
    std::fs::write(change_dir(world).join("design.md"), content).unwrap();
}

#[given(regex = r"^the file ([^ ]+) does not exist$")]
fn file_does_not_exist(_world: &mut GivnWorld, path: String) {
    let p = project_root(_world).join(&path);
    assert!(!p.exists(), "precondition failed: file {path} exists");
}

#[given(regex = r"^the file ([^ ]+) exists and is zero bytes$")]
fn file_exists_zero_bytes(world: &mut GivnWorld, path: String) {
    let p = project_root(world).join(&path);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(&p, "").unwrap();
}

#[given(
    "the change's design.md Step Definitions section contains prose with backticks instead of a markdown table"
)]
fn design_md_prose(world: &mut GivnWorld) {
    let content = "## Step Definitions\n\nThe step definitions are in `tests/steps/my_steps.rs`.\n"
        .to_string();
    std::fs::write(change_dir(world).join("design.md"), content).unwrap();
}

#[given("a givn project with arc42 addon disabled")]
fn project_arc42_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let root = project_root(world);
    let config_path = root.join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let new_config = config.replace("arc42: true", "arc42: false");
    std::fs::write(&config_path, new_config).unwrap();
    crate::common::remove_addon_locks(
        world
            .env
            .as_ref()
            .expect("a project root is required first"),
    );
}

#[given("a givn project with arc42 addon enabled")]
fn project_arc42_enabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    set_verify_true(world);
    set_coverage_true(world);
    // These fixtures exercise the Arc42 integrity hook, not the outline gate.
    disable_outline_addon(world);
    // Create the test change so subsequent steps can write to it.
    let out = common::run_givn(world.env.as_ref().unwrap(), &["new", "test-change"]);
    assert!(
        out.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some("test-change".to_string());
    declare_fixture_review_baseline(world, "test-change");
    write_fixture_change_receipt(world, "test-change");
}

#[given("a change that has passed design-review")]
fn standalone_change_design_review(world: &mut GivnWorld) {
    let out = common::run_givn(world.env.as_ref().unwrap(), &["new", "test-change"]);
    assert!(
        out.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some("test-change".to_string());
    set_verify_true(world);
    set_coverage_true(world);
    // These fixtures exercise the integrity hook, not the outline gate.
    disable_outline_addon(world);
    let dr_path = project_root(world).join("givn/changes/test-change/design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
    declare_fixture_review_baseline(world, "test-change");
    write_fixture_change_receipt(world, "test-change");
}

#[given(expr = "the change's arc42.md marks chapter {string} as Affected: Yes")]
fn arc42_marks_chapter_yes(world: &mut GivnWorld, chapter_slug: String) {
    let content = format!(
        "# arc42 Documentation Update: test-change\n\
         \n\
         ## Impact assessment\n\
         \n\
         | # | Chapter | Affected? | Reason | Summary of change (if Yes) |\n\
         |---|---|---|---|---|\n\
         | 1 | {chapter_slug} | Yes | Needs update | |\n\
         | 2 | 02-architecture-constraints | No | | |\n\
         \n\
         ## Notes\n\
         \n\
         ---\n\
         \n\
         ## Status\n\
         \n\
         STATUS: DONE\n"
    );
    std::fs::write(change_dir(world).join("arc42.md"), content).unwrap();
}

#[given("docs/arc42/01-introduction-and-goals.md is byte-identical to the givn template")]
fn arc42_chapter_byte_identical(world: &mut GivnWorld) {
    let content = givn::config::embedded::Assets::get_str(
        "addons/arc42-docs/chapters/01-introduction-and-goals.md",
    )
    .expect("embedded chapter template");
    let path = project_root(world).join("docs/arc42/01-introduction-and-goals.md");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, content.as_bytes()).unwrap();
}

#[given("docs/arc42/01-introduction-and-goals.md differs from the givn template")]
fn arc42_chapter_edited(world: &mut GivnWorld) {
    let path = project_root(world).join("docs/arc42/01-introduction-and-goals.md");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "# Custom content\n").unwrap();
}

#[given("the change's arc42.md marks an unresolvable chapter name as Affected: Yes")]
fn arc42_chapter_unresolvable(world: &mut GivnWorld) {
    let content = "# arc42 Documentation Update: test-change\n\
         \n\
         ## Impact assessment\n\
         \n\
         | # | Chapter | Affected? | Reason | Summary of change (if Yes) |\n\
         |---|---|---|---|---|\n\
         | 99 | 99-bogus-chapter | Yes | Needs update | |\n\
         \n\
         ## Notes\n\
         \n\
         ---\n\
         \n\
         ## Status\n\
         \n\
         STATUS: DONE\n"
        .to_string();
    std::fs::write(change_dir(world).join("arc42.md"), content).unwrap();
}

#[given("no corresponding docs/arc42/ file exists")]
fn no_corresponding_arc42_file(_world: &mut GivnWorld) {
    // This is declarative — the file shouldn't exist.
    // Verified by the scenario setup (we never create 99-bogus-chapter.md).
}

fn write_permanent_test_cap_narrative(root: &std::path::Path) {
    let dir = root.join("givn/specs/test-cap");
    std::fs::create_dir_all(&dir).expect("create permanent usecase dir");
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", "test-cap")
        .replace("<capability-id>", "test-cap")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "No E2E interaction")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| test-cap | exercise the capability | No E2E interaction |\n",
            "- none\n",
        );
    std::fs::write(dir.join("usecase.md"), document).expect("write permanent usecase");
}

#[given("a givn project with a completed change configured for archive")]
fn change_complete_for_archive(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "integrity-hook-and-archive-gate");
    disable_coverage_addon(world);
    let root = project_root(world);

    let change = root.join("givn/changes/integrity-hook-and-archive-gate");

    // Write all required artifacts for the archive gate.
    std::fs::write(change.join("proposal.md"), "# Proposal\n").unwrap();

    // Specs — create a delta feature file.
    let specs_dir = change.join("specs").join("test-cap");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(
        specs_dir.join("test-cap.feature"),
        "@givn.delta @test-cap\nFeature: Test Capability\n\n  @givn.added\n  Scenario: Test scenario\n    Given a thing\n    Then it works\n",
    ).unwrap();

    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- Implement all tasks\n",
    )
    .unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();

    // Create a corresponding permanent spec so merging works.
    let perm_dir = root.join("givn/specs/test-cap");
    std::fs::create_dir_all(&perm_dir).unwrap();
    std::fs::write(
        perm_dir.join("test-cap.feature"),
        "Feature: Test Capability\n",
    )
    .unwrap();
    write_permanent_test_cap_narrative(&root);
    let env = world.env.as_ref().expect("a fixture project is required");
    common::declare_fixture_routing(env, "integrity-hook-and-archive-gate");
}

#[given("the change's review.md has no README impact decision")]
fn review_md_no_readme_impact(world: &mut GivnWorld) {
    std::fs::write(
        change_dir(world).join("review.md"),
        "# Review\n\nREVIEW: PASS\n",
    )
    .expect("write review.md without README impact");
}

#[given("the change's design-review.md is removed")]
fn design_review_removed(world: &mut GivnWorld) {
    std::fs::remove_file(change_dir(world).join("design-review.md"))
        .expect("remove design-review.md");
}

#[given("the project's verify.command exits 0 on the current permanent specs")]
fn verify_passes(world: &mut GivnWorld) {
    set_verify_true(world);
}

#[given("the delta spec introduces a scenario that conflicts with an existing permanent scenario")]
fn delta_conflicts(world: &mut GivnWorld) {
    // verify must pass gate hooks but fail post-merge
    set_verify_fails_post_merge(world);
}

#[given("verify.command exits non-zero on the merged permanent spec")]
fn verify_fails_after_merge(world: &mut GivnWorld) {
    // verify must pass gate hooks but fail post-merge
    set_verify_fails_post_merge(world);
}

#[then("the change's design.md template contains a section named \"Internal Primitives\"")]
fn design_has_primitives_section(_world: &mut GivnWorld) {
    let content = givn::config::embedded::Assets::get_str("templates/design.md")
        .expect("embedded design.md template");
    assert!(
        content.contains("Internal Primitives"),
        "design.md template missing Internal Primitives section"
    );
}

#[then("the section includes a table with columns \"Primitive\", \"Type\", and \"Purpose\"")]
fn design_has_primitives_table(_world: &mut GivnWorld) {
    let content = givn::config::embedded::Assets::get_str("templates/design.md")
        .expect("embedded design.md template");
    assert!(
        content.contains("| Primitive | Type | Purpose |"),
        "design.md template missing primitives table columns"
    );
}

#[given("a givn project")]
fn givn_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given("a givn project with an archive-ready change whose delta spec contains @wip")]
fn archive_ready_change_with_wip(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "wip-change");
    let root = project_root(world);
    let change = root.join("givn/changes/wip-change");

    // Write all required artifacts for the archive gate.
    std::fs::write(change.join("proposal.md"), "# Proposal\n").unwrap();

    // Delta spec with a @wip scenario.
    let specs_dir = change.join("specs").join("test-cap");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(
        specs_dir.join("test-cap.feature"),
        "@givn.delta @test-cap\nFeature: Test Capability\n\n  @givn.added @wip\n  Scenario: Not done yet\n    Given x\n    Then y\n",
    ).unwrap();

    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- Implement all tasks\n",
    )
    .unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    set_verify_true(world);

    // Create a corresponding permanent spec so merging works.
    let perm_dir = root.join("givn/specs/test-cap");
    std::fs::create_dir_all(&perm_dir).unwrap();
    std::fs::write(
        perm_dir.join("test-cap.feature"),
        "Feature: Test Capability\n",
    )
    .unwrap();
    write_permanent_test_cap_narrative(&root);
    let env = world.env.as_ref().expect("a fixture project is required");
    common::declare_fixture_routing(env, "wip-change");
}

#[given(
    "a givn project with an archive-ready change whose @wip is only on a @givn.removed scenario"
)]
fn archive_ready_change_wip_only_on_removed(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "wip-removed-change");
    disable_coverage_addon(world);
    let root = project_root(world);
    let change = root.join("givn/changes/wip-removed-change");

    // Write all required artifacts for the archive gate.
    std::fs::write(change.join("proposal.md"), "# Proposal\n").unwrap();

    // Delta spec with @wip on a @givn.removed placeholder only.
    let specs_dir = change.join("specs").join("test-cap");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(
        specs_dir.join("test-cap.feature"),
        "@givn.delta @test-cap\nFeature: Test Capability\n\n  @givn.removed @wip\n  Scenario: Old\n    Given x\n",
    ).unwrap();

    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- Implement all tasks\n",
    )
    .unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    set_verify_true(world);

    // Permanent spec with an "Old" scenario that the delta will remove.
    let perm_dir = root.join("givn/specs/test-cap");
    std::fs::create_dir_all(&perm_dir).unwrap();
    std::fs::write(
        perm_dir.join("test-cap.feature"),
        "Feature: Test Capability\n\n  Scenario: Old\n    Given x\n",
    )
    .unwrap();
    write_permanent_test_cap_narrative(&root);
    let env = world.env.as_ref().expect("a fixture project is required");
    common::declare_fixture_routing(env, "wip-removed-change");
}

#[given("a change whose delta spec contains @wip")]
fn change_with_wip_spec(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "wip-check-change");
    let root = project_root(world);
    let change = root.join("givn/changes/wip-check-change");

    // Create a delta spec with @wip.
    let specs_dir = change.join("specs").join("test-cap");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(
        specs_dir.join("test-cap.feature"),
        "@givn.delta @test-cap\nFeature: Test Capability\n\n  @givn.added @wip\n  Scenario: Not done\n    Given x\n    Then y\n",
    ).unwrap();

    // Write other minimal artifacts.
    std::fs::write(change.join("proposal.md"), "# Proposal\n").unwrap();
    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(change.join("tasks.md"), "- [x] COMMIT: a1b2c3d4 -- Task\n").unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    set_verify_true(world);
}

// ─── forward-only archive and review bindings ─────────────────────────────

#[then("the merged permanent spec preserves the merged delta")]
fn merged_spec_preserves_delta(world: &mut GivnWorld) {
    let specs = project_root(world).join("givn/specs");
    let mut stack = vec![specs];
    let mut found = false;
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "feature")
                && let Ok(content) = std::fs::read_to_string(&path)
                && content.contains("Test scenario")
            {
                found = true;
            }
        }
    }
    assert!(found, "merged delta scenario missing from permanent specs");
}

#[then("stderr says the candidate is unproven and can be repaired in place")]
fn stderr_says_candidate_unproven(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("unproven") && (s.contains("repair") || s.contains("repairable")),
        "stderr missing forward-repair message: {s}"
    );
}

#[given("the project records whether its verify commands run")]
fn project_records_verify(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let script = env.path("record-verify.sh");
    std::fs::write(&script, "#!/bin/sh\ntouch verify-ran.marker\nexit 0\n").unwrap();
    std::fs::set_permissions(&script, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    set_verify_command_in_config(world, "./record-verify.sh");
    if let Some(id) = world.last_change_id.clone() {
        write_fixture_change_receipt(world, &id);
    }
}

#[then("the verify command did not run")]
fn verify_command_did_not_run(world: &mut GivnWorld) {
    let marker = project_root(world).join("verify-ran.marker");
    assert!(
        !marker.exists(),
        "verify command ran during review: {}",
        marker.display()
    );
}

#[given("a givn project with a completed change that introduces a brand-new capability")]
fn change_new_capability(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "integrity-hook-and-archive-gate");
    disable_coverage_addon(world);
    let root = project_root(world);

    // Remove any auto-created specs.
    let specs_root = root.join("givn/specs");
    if specs_root.exists() {
        std::fs::remove_dir_all(&specs_root).unwrap();
    }
    std::fs::create_dir_all(&specs_root).unwrap();
    let permanent = specs_root.join("new-cap");
    std::fs::create_dir_all(&permanent).unwrap();
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", "new-cap")
        .replace("<capability-id>", "new-cap")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "New scenario");
    std::fs::write(permanent.join("usecase.md"), document).unwrap();

    let change = root.join("givn/changes/integrity-hook-and-archive-gate");
    std::fs::write(
        change.join("proposal.md"),
        crate::steps::fixture_routing_proposal("new-cap"),
    )
    .unwrap();

    // Delta for a new capability.
    let specs_dir = change.join("specs").join("new-cap");
    std::fs::create_dir_all(&specs_dir).unwrap();
    std::fs::write(
        specs_dir.join("new-cap.feature"),
        "@givn.delta @new-cap\nFeature: New Capability\n\n  @givn.added @e2e\n  Scenario: New scenario\n    Given x\n    Then y\n",
    ).unwrap();

    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- Implement new capability\n",
    )
    .unwrap();
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();

    // Set verify to fail after merge (pass during gate hooks, fail post-merge).
    set_verify_true(world);
    set_verify_fails_post_merge(world);
}

#[then("the delta scenarios have been merged into givn/specs/")]
fn delta_merged(world: &mut GivnWorld) {
    let p = project_root(world).join("givn/specs");
    assert!(p.is_dir(), "givn/specs does not exist");
    // Check at least one .feature file exists under givn/specs/
    let has_feature = std::fs::read_dir(&p)
        .expect("read givn/specs")
        .filter_map(|e| e.ok())
        .any(|e| {
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                std::fs::read_dir(e.path())
                    .ok()
                    .map(|rd| {
                        rd.filter_map(|f| f.ok()).any(|f| {
                            f.path()
                                .extension()
                                .map(|ext| ext == "feature")
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            } else {
                false
            }
        });
    assert!(has_feature, "no .feature files found under givn/specs/");
}

#[then("the migration comparison is marked failed")]
fn migration_comparison_failed(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(
        project_root(world).join("givn/changes/migration-test/coverage-comparison.json"),
    )
    .unwrap();
    assert!(
        content.contains("\"passed\": false"),
        "comparison was not failed: {content}"
    );
}

#[then("the migration comparison is marked passed")]
fn migration_comparison_passed(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(
        project_root(world).join("givn/changes/migration-test/coverage-comparison.json"),
    )
    .unwrap();
    assert!(
        content.contains("\"passed\": true"),
        "comparison was not passed: {content}"
    );
}

// --- change verification receipt scenarios --------------------------------

#[given("a clean committed givn project with a change whose delta specs pass")]
fn clean_project_with_passing_change(world: &mut GivnWorld) {
    crate::steps::archive_proof_support::ready_change(world, "verify-demo", false, "pass");
    // The review routing check resolves the declared NEW-in use case. Give the
    // fixture a permanent use case that owns the capability.
    let root = project_root(world);
    let usecase = root.join("givn/specs/demo-cap");
    std::fs::create_dir_all(&usecase).expect("create fixture usecase");
    let document = givn::gherkin::usecase_md::usecase_md_template()
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

#[given("the change has a current verification receipt")]
fn change_has_current_receipt(world: &mut GivnWorld) {
    if let Some(id) = world.last_change_id.clone() {
        crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    }
    world.skip_receipt_autowrite = true;
}

#[given("the delta specs changed after the receipt")]
fn delta_specs_changed_after_receipt(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let specs = change_dir_for(world, &id).join("specs");
    let file = givn::gherkin::parse::collect_feature_files(&specs)
        .into_iter()
        .next()
        .expect("a delta feature file");
    let mut content = std::fs::read_to_string(&file).expect("read delta feature");
    content.push_str("\n  @givn.added\n  Scenario: Late scenario\n    Given x\n    Then y\n");
    std::fs::write(&file, content).expect("write changed delta feature");
    if let Some(env) = world.env.as_ref() {
        common::commit_fixture(env);
    }
    world.skip_receipt_autowrite = true;
}

#[then("the change directory contains a passed verification receipt naming the tested revision")]
fn change_directory_has_passed_receipt(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let change = change_dir_for(world, &id);
    let receipt = givn::proof::ChangeReceipt::read(&change).expect("read change receipt");
    assert_eq!(receipt.status, "passed");
    let env = world.env.as_ref().expect("a fixture project");
    let head = match env.backend() {
        common::BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        common::BackendKind::Fossil => {
            String::from_utf8_lossy(&common::fossil(env, &["info"]).stdout)
                .lines()
                .find_map(|line| line.strip_prefix("checkout:"))
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or_default()
                .to_string()
        }
    };
    assert_eq!(receipt.head, head);
}

#[then(regex = r"^stderr routes to `givn check tasks --change ([^`]+)`$")]
fn stderr_routes_to_check_tasks(world: &mut GivnWorld, id: String) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains(&format!("givn check tasks --change {id}")),
        "stderr does not route to check tasks: {stderr}"
    );
}

#[then("stderr says the verification receipt is stale")]
fn stderr_says_receipt_stale(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("verification receipt is stale"),
        "stderr missing stale receipt: {stderr}"
    );
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

// --- strict scenario commit checks ----------------------------------------

fn empty_commit_with_message(world: &GivnWorld, message: &str) -> String {
    let env = world.env.as_ref().expect("a fixture project");
    let out = common::git(env, &["commit", "--allow-empty", "-m", message]);
    assert!(
        out.status.success(),
        "empty commit failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_string()
}

fn write_tasks_and_commit(world: &GivnWorld, id: &str, content: &str) {
    let change = change_dir_for(world, id);
    std::fs::write(change.join("tasks.md"), content).expect("write tasks");
    common::commit_fixture(world.env.as_ref().expect("a fixture project"));
}

#[given("the change's tasks.md checks a scenario without a commit")]
fn tasks_checked_scenario_without_commit(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    write_tasks_and_commit(
        world,
        &id,
        "- [x] Scenario: A checked scenario\n  - [x] done\n",
    );
}

#[given("the change's tasks.md checks two scenarios with the same batch revision")]
fn tasks_duplicate_scenario_revision(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let hash = empty_commit_with_message(world, "feat: Batch first-and-second");
    write_tasks_and_commit(
        world,
        &id,
        &format!(
            "- [x] Scenario: First scenario\n  - [x] COMMIT: {hash} -- Batch: first-and-second\n\
             - [x] Scenario: Second scenario\n  - [x] COMMIT: {hash} -- Batch: first-and-second\n"
        ),
    );
}

#[given("the change's tasks.md checks two scenarios with the same revision and no batch label")]
fn tasks_duplicate_scenario_revision_without_batch(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let hash = empty_commit_with_message(world, "feat: shared revision");
    write_tasks_and_commit(
        world,
        &id,
        &format!(
            "- [x] Scenario: First scenario\n  - [x] COMMIT: {hash} -- first\n\
             - [x] Scenario: Second scenario\n  - [x] COMMIT: {hash} -- second\n"
        ),
    );
}

#[given("the change's tasks.md checks a scenario whose commit message omits the title")]
fn tasks_message_without_title(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let hash = empty_commit_with_message(world, "unrelated commit");
    write_tasks_and_commit(
        world,
        &id,
        &format!("- [x] Scenario: Scenario with a title\n  - [x] COMMIT: {hash} -- unrelated\n"),
    );
}

#[given("the change's tasks.md checks one scenario with its own commit")]
fn tasks_one_scenario_with_commit(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let hash = empty_commit_with_message(world, "feat: Scenario with a title");
    write_tasks_and_commit(
        world,
        &id,
        &format!(
            "- [x] Scenario: Scenario with a title\n  - [x] COMMIT: {hash} -- Scenario with a title\n"
        ),
    );
}

#[then("stderr names the scenario and the missing commit")]
fn stderr_names_missing_scenario_commit(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("A checked scenario") && stderr.contains("has no `COMMIT:` line"),
        "stderr missing the scenario commit finding: {stderr}"
    );
}

#[then("stderr says a shared revision needs a batch label")]
fn stderr_says_shared_revision_needs_batch(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("shares revision") && stderr.contains("no `Batch:` label"),
        "stderr missing the shared revision finding: {stderr}"
    );
}

#[then("stderr says the commit does not name the scenario")]
fn stderr_says_commit_missing_title(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("does not name the scenario in its message"),
        "stderr missing the commit message finding: {stderr}"
    );
}

#[then("stdout says scenario commits passed")]
fn stdout_says_scenario_commits_passed(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("scenario commits passed"),
        "stdout missing the scenario commit pass line: {stdout}"
    );
}

#[given("the change's review.md records `VISUAL-REVIEW: NOT APPLICABLE`")]
fn review_records_not_applicable(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    std::fs::write(
        change_dir_for(world, &id).join("review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\nVISUAL-REVIEW: NOT APPLICABLE\n",
    )
    .expect("write review marker");
}

// ─── review diagnostics and proof reuse fixtures ─────────────────────────

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

#[given(
    "a clean committed change whose use-case contract starts with \"# Use Case: review-diagnostics-demo\""
)]
fn review_fixture_case_mismatch_title(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: review-diagnostics-demo"),
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
        "",
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
    "a clean committed change whose use-case contract has an unknown \"## Guarantees\" heading"
)]
fn review_fixture_unknown_heading(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        None,
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
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
    "a clean committed change whose use-case contract omits \"## Minimal guarantee\" and \"## Success guarantee\""
)]
fn review_fixture_missing_sections(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        None,
        "- alpha-capability",
        "| Capability | Consumer action | E2E scenario |\n| --- |\n| alpha-capability | do it | Consumer does it |",
        "- A valid example.",
        "",
    )
    .replace("## Minimal guarantee\n\nAn invalid change cannot pass.\n\n", "")
    .replace(
        "## Success guarantee\n\nThe developer receives a complete result.\n\n",
        "",
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

#[given("the interaction table separator row has the wrong column count")]
fn interaction_separator_wrong_columns(_world: &mut GivnWorld) {
    // The contract written by the preceding Given already carries the malformed
    // separator row.
}

#[given(
    "a clean committed change with three delta capabilities routed \"NEW in review-diagnostics-demo\""
)]
fn review_fixture_three_capabilities(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: review-diagnostics-demo"),
        "- alpha-capability\n- beta-capability\n- gamma-capability",
        &valid_interactions(&["alpha-capability", "beta-capability", "gamma-capability"]),
        "- A valid example.",
        "## Guarantees\n\nThis heading is not part of the schema.\n\n",
    );
    write_review_fixture(
        world,
        "review-diagnostics-demo",
        "review-diagnostics-demo",
        &contract,
        &["alpha-capability", "beta-capability", "gamma-capability"],
        &[
            ("alpha-capability", "NEW in review-diagnostics-demo"),
            ("beta-capability", "NEW in review-diagnostics-demo"),
            ("gamma-capability", "NEW in review-diagnostics-demo"),
        ],
    );
}

#[given(
    "the use-case contract has a case-mismatched title and an unknown \"## Guarantees\" heading"
)]
fn review_fixture_title_and_heading(_world: &mut GivnWorld) {
    // The contract written by the preceding Given already carries both.
}

#[given(
    "a clean committed change that replaces an existing use-case narrative and whose routing decision extends that use case's capability"
)]
fn review_fixture_extend_narrative(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let id = "review-diagnostics-demo";
    let out = common::run_givn(env, &["new", id]);
    assert!(out.status.success(), "givn new failed");
    world.last_change_id = Some(id.to_string());

    let change = env.path(&format!("givn/changes/{id}"));
    let specs = change.join("specs/existing-use");
    std::fs::create_dir_all(&specs).unwrap();
    std::fs::write(
        change.join("proposal.md"),
        "# Proposal: fixture\n\n## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| existing-cap | (fixture) | EXTEND existing-cap | fixture |\n",
    )
    .unwrap();
    let permanent = env.path("givn/specs/existing-use");
    std::fs::create_dir_all(&permanent).unwrap();
    std::fs::write(
        permanent.join("existing-cap.feature"),
        "@existing-cap\nFeature: Existing\n\n  Scenario: Existing\n    Given x\n    Then y\n",
    )
    .unwrap();
    std::fs::write(
        permanent.join("usecase.md"),
        review_contract(
            "existing-use",
            None,
            "- existing-cap",
            &valid_interactions(&["existing-cap"]),
            "- A valid example.",
            "",
        ),
    )
    .unwrap();
    let replacement = review_contract(
        "existing-use",
        Some("# Use Case: existing-use"),
        "- existing-cap",
        &valid_interactions(&["existing-cap"]),
        "- A valid example.",
        "## Guarantees\n\nThis heading is not part of the schema.\n\n",
    );
    std::fs::write(specs.join("usecase.md"), replacement).unwrap();
    std::fs::write(
        specs.join("existing-cap.feature"),
        "@givn.delta @existing-cap\nFeature: Existing\n\n  @givn.added\n  Scenario: Fixture\n    Given x\n    Then y\n",
    )
    .unwrap();
    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
    std::fs::write(
        change.join("review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\n\n## Overlap dispositions\n\n         | Scenario A | Scenario B | Disposition |\n|---|---|---|\n         | `Existing` | `Fixture` | boundary |\n",
    )
    .unwrap();

    set_verify_true(world);
    common::commit_fixture(env);
    write_fixture_change_receipt(world, id);
}

#[given(
    "the replacement narrative has a case-mismatched title and an unknown \"## Guarantees\" heading"
)]
fn replacement_narrative_has_problems(_world: &mut GivnWorld) {
    // The replacement written by the preceding Given already carries both.
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

#[then("stderr says headings are case-sensitive")]
fn stderr_says_headings_case_sensitive(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("headings are case-sensitive"),
        "stderr missing case-sensitivity note: {text}"
    );
}

#[then(regex = r#"^stderr suggests "([^"]+)"$"#)]
fn stderr_suggests(world: &mut GivnWorld, suggestion: String) {
    let text = stderr(world);
    assert!(
        text.contains("did you mean") && text.contains(&suggestion),
        "stderr missing suggestion {suggestion:?}: {text}"
    );
}

#[then("stderr names both missing sections and the malformed interaction table together")]
fn stderr_names_missing_sections_and_table(world: &mut GivnWorld) {
    let text = stderr(world);
    for needle in [
        "document schema does not recognise '## Minimal guarantee'",
        "document schema does not recognise '## Success guarantee'",
        "malformed separator row",
    ] {
        assert!(text.contains(needle), "stderr missing {needle:?}: {text}");
    }
}

#[then("stderr names the case-mismatched title once")]
fn stderr_names_title_once(world: &mut GivnWorld) {
    let text = stderr(world);
    assert_eq!(
        text.matches("got '# Use Case: review-diagnostics-demo'")
            .count(),
        1,
        "stderr repeated the title diagnostic: {text}"
    );
}

#[then("stderr names the unknown heading once")]
fn stderr_names_heading_once(world: &mut GivnWorld) {
    let text = stderr(world);
    assert_eq!(
        text.matches("unknown heading '## Guarantees'").count(),
        1,
        "stderr repeated the heading diagnostic: {text}"
    );
}

#[then("stderr names the replacement narrative's title and unknown heading")]
fn stderr_names_replacement_narrative(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("got '# Use Case: existing-use'"),
        "stderr missing replacement title: {text}"
    );
    assert!(
        text.contains("unknown heading '## Guarantees'"),
        "stderr missing replacement heading: {text}"
    );
}

#[given(
    "a clean committed change with a green change verification receipt recorded at one revision"
)]
fn review_proof_fixture(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-proof-demo",
        None,
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
        "",
    );
    write_review_fixture_with_options(
        world,
        "review-proof-demo",
        "review-proof-demo",
        &contract,
        &["alpha-capability"],
        &[("alpha-capability", "NEW in review-proof-demo")],
        false,
    );
    world.verification_invocations_before = Some(invocation_count(world));
    world.skip_receipt_autowrite = true;
}

fn invocation_count(world: &GivnWorld) -> usize {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    let path = project_root(world).join(format!("givn/changes/{id}/verification/invocations.log"));
    std::fs::read_to_string(path)
        .map(|text| text.lines().count())
        .unwrap_or(0)
}

#[given(
    "the developer commits only proposal, design, tasks, review, arc42, and use-case narrative edits after that revision"
)]
fn commit_documentation_only_review_edits(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let change = change_dir_for(world, &id);
    for name in ["proposal.md", "design.md", "review.md"] {
        let path = change.join(name);
        let mut content = std::fs::read_to_string(&path).unwrap_or_default();
        content.push_str("\nDocumentation-only review edit.\n");
        std::fs::write(path, content).unwrap();
    }
    let head = crate::steps::archive_proof_support::current_revision(world);
    std::fs::write(
        change.join("tasks.md"),
        format!("- [x] COMMIT: {head} -- Documentation-only review edit\n"),
    )
    .unwrap();
    std::fs::write(
        change.join("arc42.md"),
        "## Status\n\nSTATUS: DONE\n\nDocumentation-only review edit.\n",
    )
    .unwrap();
    let usecase = change.join("specs").join(&id).join("usecase.md");
    let content = std::fs::read_to_string(&usecase).unwrap();
    std::fs::write(
        usecase,
        content.replace(
            "- Structural failures outrank other findings.",
            "- Structural failures outrank other findings.\n- Documentation-only review edit.",
        ),
    )
    .unwrap();
    common::commit_fixture(world.env.as_ref().unwrap());
}

#[then("the change suite did not run again")]
fn change_suite_did_not_run_again(world: &mut GivnWorld) {
    let before = world
        .verification_invocations_before
        .expect("the fixture must record the invocation count");
    let after = invocation_count(world);
    assert_eq!(
        after, before,
        "the change suite ran again: {before} -> {after} invocations"
    );
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
    let path = project_root(world).join(format!("givn/changes/{id}/verification/invocations.log"));
    std::fs::read_to_string(path)
        .map(|text| {
            text.lines()
                .filter(|line| line.contains("givn/changes/"))
                .count()
        })
        .unwrap_or(0)
}

#[given(
    "a clean committed change whose use-case contract starts with \"# Use Case: Lending Register\" and has an unknown \"## Guarantees\" heading"
)]
fn review_fixture_canonical_correction(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: Lending Register"),
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
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

#[then(regex = r#"^stderr suggests exactly "([^"]+)"$"#)]
fn stderr_suggests_exactly(world: &mut GivnWorld, suggestion: String) {
    let text = stderr(world);
    let needle = format!("did you mean '{suggestion}'?");
    assert!(text.contains(&needle), "stderr missing {needle:?}: {text}");
}

#[given(
    "a clean committed change whose use-case contract starts with \"# Use Case: Lending Register\" and whose capability list is a table"
)]
fn review_fixture_non_kebab_title_table_capabilities(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: Lending Register"),
        "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| alpha-capability | do it | Consumer does it |",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
        "",
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
    "a clean committed change whose use-case contract starts with \"# Use case: Lending Register\""
)]
fn review_fixture_exact_prefix_non_kebab_title(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use case: Lending Register"),
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "- A valid example.",
        "",
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
    "a clean committed change whose use-case contract starts with \"# Use Case: Lending Register\", omits \"## Rules\", and leaves \"## Examples\" empty"
)]
fn review_fixture_malformed_title_missing_and_empty_sections(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: Lending Register"),
        "- alpha-capability",
        &valid_interactions(&["alpha-capability"]),
        "",
        "",
    )
    .replace(
        "## Rules\n\n- Structural failures outrank other findings.\n\n",
        "",
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
    "a clean committed change whose use-case contract has a malformed Level and omits \"## Examples\""
)]
fn review_fixture_malformed_level(world: &mut GivnWorld) {
    let contract = review_contract_with_level(
        "review-diagnostics-demo",
        None,
        "`+` — summary use case. Its five capabilities decompose the lending-desk activity into the goals below.",
        "- none",
        "- none",
        "- A valid example.",
        "",
    )
    .replace("## Examples\n\n- A valid example.\n\n", "");
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
    "a clean committed change whose use-case contract starts with \"# Use Case: Lending Register\", has a rule line that is not a bullet, and has an interaction that names an undeclared capability"
)]
fn review_fixture_independent_local_findings(world: &mut GivnWorld) {
    let contract = review_contract(
        "review-diagnostics-demo",
        Some("# Use Case: Lending Register"),
        "- alpha-capability",
        "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| register-readiness | do it | Consumer does it |",
        "- A valid example.",
        "",
    )
    .replace(
        "- Structural failures outrank other findings.",
        "This rule is prose, not a bullet.",
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

// --- 0.9.10 fresh-verification and gate fixtures ---------------------------

fn workflow_env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("a fixture project is required")
}

fn set_ready_proposal(world: &GivnWorld, id: &str) {
    std::fs::write(
        change_dir_for(world, id).join("proposal.md"),
        "# Proposal: fixture\n\n## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | demo-cap | EXTEND demo-cap | `EXTEND demo-cap` | fixture |\n",
    )
    .expect("write ready proposal");
}

fn review_ready_change(world: &mut GivnWorld, id: &str) {
    crate::steps::archive_proof_support::ready_change(world, id, false, "pass");
    std::fs::write(
        project_root(world).join("givn/specs/demo-cap/demo-cap.feature"),
        "Feature: Demo Capability\n\n  Scenario: Existing permanent scenario\n    Given an existing thing\n    Then it works\n",
    )
    .expect("write permanent fixture feature");
    set_ready_proposal(world, id);
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, id);
}

fn verification_ready_change(world: &mut GivnWorld, id: &str) {
    crate::steps::archive_proof_support::ready_change(world, id, false, "pass");
    set_ready_proposal(world, id);
    let change = change_dir_for(world, id);
    let _ = std::fs::remove_file(change.join("verification/tasks-verification.json"));
    let _ = std::fs::remove_file(change.join("verification/tasks-result.json"));
    common::commit_fixture(workflow_env(world));
}

fn write_workflow_runner(world: &GivnWorld, body: &str) {
    let path = workflow_env(world).path("emit-result.sh");
    std::fs::write(&path, body).expect("write workflow runner");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("chmod workflow runner");
}

fn commit_workflow_runner(world: &GivnWorld) {
    common::commit_fixture(workflow_env(world));
}

fn valid_runner(mutation: &str) -> String {
    format!(
        r#"#!/bin/sh
{mutation}
printf '{{"scope":"change","total":2,"passed":2,"failed":0,"skipped":0}}\n' > "$GIVN_RESULT_FILE"
"#
    )
}

#[given("a change ready for changed-scenario verification")]
fn change_ready_for_verification(world: &mut GivnWorld) {
    verification_ready_change(world, "verify-demo");
}

#[given("a change ready for changed-scenario verification with a stale passing runner result")]
fn change_with_stale_result(world: &mut GivnWorld) {
    verification_ready_change(world, "verify-demo");
    let result = change_dir_for(world, "verify-demo").join("verification/tasks-result.json");
    std::fs::create_dir_all(result.parent().expect("result parent")).expect("create result dir");
    std::fs::write(
        &result,
        "{\"scope\":\"change\",\"total\":2,\"passed\":2,\"failed\":0,\"skipped\":0}\n",
    )
    .expect("write stale result");
    write_workflow_runner(
        world,
        r#"#!/bin/sh
if [ -e "$GIVN_RESULT_FILE" ]; then
  printf 'saw-stale' > runner-saw-stale-result
fi
exit 0
"#,
    );
    commit_workflow_runner(world);
}

#[given("the runner exits successfully without writing a result")]
fn runner_exits_without_result(_world: &mut GivnWorld) {
    // The stale-result fixture's runner already exits 0 without writing.
}

#[given(regex = r#"^the runner exits successfully and writes fresh output that is (.+)$"#)]
fn runner_writes_invalid_output(world: &mut GivnWorld, kind: String) {
    let kind = match kind.as_str() {
        "malformed" => "malformed",
        "reporting failures" => "failed",
        "reporting skipped scenarios" => "skipped",
        "reporting zero scenarios" => "zero",
        "reporting counts that do not reconcile" => "unreconciled",
        "reporting a different scope" => "wrong-scope",
        other => panic!("unknown invalid output {other}"),
    };
    crate::steps::archive_proof_support::write_result_runner(workflow_env(world), kind);
    let runner = r#"#!/bin/sh
kind=__KIND__
scope=$1
if [ -z "${GIVN_RESULT_FILE:-}" ]; then exit 0; fi
case "$kind" in
    malformed) printf '{"scope":' > "$GIVN_RESULT_FILE"; exit 0 ;;
    failed) total=2; passed=1; failed=1; skipped=0 ;;
    skipped) total=2; passed=1; failed=0; skipped=1 ;;
    zero) total=0; passed=0; failed=0; skipped=0 ;;
    unreconciled) total=3; passed=2; failed=0; skipped=0 ;;
    wrong-scope) scope=regular; total=2; passed=2; failed=0; skipped=0 ;;
    *) total=2; passed=2; failed=0; skipped=0 ;;
esac
printf '{"scope":"%s","total":%s,"passed":%s,"failed":%s,"skipped":%s}\n' "$scope" "$total" "$passed" "$failed" "$skipped" > "$GIVN_RESULT_FILE"
"#;
    write_workflow_runner(world, &runner.replace("__KIND__", kind));
    commit_workflow_runner(world);
}

#[given("the runner produces fresh valid passing output")]
fn runner_produces_valid_output(_world: &mut GivnWorld) {
    // The mutation fixtures rewrite the runner in the next Given.
}

#[given(regex = r#"^the (.+) changes during execution and remains changed afterward$"#)]
fn runner_changes_input(world: &mut GivnWorld, input: String) {
    let id = world
        .last_change_id
        .clone()
        .expect("a verification change id");
    let mutation = match input.as_str() {
        "tested revision" => "git update-ref HEAD $(git commit-tree $(git write-tree) -p HEAD -m 'fixture: runner revision')".to_string(),
        "selected delta feature" => format!(
            "printf '\\n  @givn.added\\n  Scenario: Runner changed delta\\n    Given x\\n    Then y\\n' >> givn/changes/{id}/specs/demo-cap/demo-cap.feature"
        ),
        "runner command" => "sed -i 's|command: \"./emit-result.sh regular\"|command: \"./emit-result.sh regular \"|' givn/commands.yaml".to_string(),
        "relevant source file" => "printf '\\n// runner mutation\\n' >> tests/steps/demo_steps.rs".to_string(),
        other => panic!("unknown mutated input {other}"),
    };
    write_workflow_runner(world, &valid_runner(&mutation));
    commit_workflow_runner(world);
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
    let receipt = givn::proof::ChangeReceipt::read(&change).expect("read change receipt");
    assert_eq!(receipt.status, "passed");
    assert!(!receipt.head.is_empty());
    assert!(!receipt.features_digest.is_empty());
    assert!(!receipt.command_digest.is_empty());
    assert!(receipt.tested_content_digest.is_some());
    assert_eq!(
        receipt.content_scope.as_deref(),
        Some(givn::proof::CONTENT_SCOPE)
    );
    assert_eq!(receipt.result.total, receipt.result.passed);
}

#[then("the stale result was absent when execution began")]
fn stale_result_absent_at_start(world: &mut GivnWorld) {
    assert!(
        !workflow_env(world).path("runner-saw-stale-result").exists(),
        "runner observed the stale result file"
    );
}

#[then("the command exits non-zero because fresh runner output is missing")]
fn exits_because_output_missing(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("runner result"), "{stderr}");
}

#[then("no passed verification receipt is issued")]
fn no_passed_receipt(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let path = change_dir_for(world, &id).join("verification/tasks-verification.json");
    assert!(!path.exists(), "a verification receipt was issued");
}

#[then("the command exits non-zero and names the invalid runner result")]
fn names_invalid_runner_result(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("runner result"), "{stderr}");
}

#[then("the command exits non-zero because the tested inputs changed")]
fn exits_because_inputs_changed(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("tested inputs changed"), "{stderr}");
}

#[then("no passed verification receipt is issued for the changed inputs")]
fn no_receipt_for_changed_inputs(world: &mut GivnWorld) {
    no_passed_receipt(world);
}

// --- No-behavior-change route ---------------------------------------------

fn write_no_behavior_change(world: &mut GivnWorld, id: &str, proposal_identity: &str, tasks: &str) {
    world.env = Some(common::make_init_root());
    let out = common::run_givn(workflow_env(world), &["new", id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let change = change_dir_for(world, id);
    std::fs::write(
        change.join("proposal.md"),
        format!("# Proposal: {proposal_identity}\n\nBehavior change: none\n"),
    )
    .expect("write no-behavior proposal");
    std::fs::write(change.join(".givn-skip"), "specs\n").expect("write skip");
    std::fs::write(change.join("design.md"), "# Design\n").expect("write design");
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design-review");
    std::fs::write(change.join("tasks.md"), tasks).expect("write tasks");
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .expect("write review");
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").expect("write arc42");
    write_recording_runner(world);
    common::commit_fixture(workflow_env(world));
    world.last_change_id = Some(id.to_string());
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

#[given(
    "a committed change explicitly declares no behavior change and contains no delta feature files"
)]
fn committed_no_behavior_change(world: &mut GivnWorld) {
    write_no_behavior_change(
        world,
        "maintenance-demo",
        "maintenance-demo",
        "- [x] COMMIT: a1b2c3d4 -- maintenance\n",
    );
}

#[given("a committed change explicitly declares no behavior change and has no delta feature files")]
fn committed_no_behavior_change_short(world: &mut GivnWorld) {
    committed_no_behavior_change(world);
}

#[given("all applicable identity and integrity checks pass")]
fn identity_and_integrity_pass(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let proposal = std::fs::read_to_string(change_dir_for(world, &id).join("proposal.md"))
        .expect("read proposal");
    assert!(proposal.contains(&format!("# Proposal: {id}")));
}

#[given("the project records every verification command invocation")]
fn project_records_verification(world: &mut GivnWorld) {
    if !workflow_env(world).path("record-runner.sh").exists() {
        write_recording_runner(world);
        common::commit_fixture(workflow_env(world));
        if let Some(id) = world.last_change_id.clone() {
            let receipt = change_dir_for(world, &id).join("verification/tasks-verification.json");
            if receipt.exists() {
                crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
            }
        }
    }
}

#[then("changed-scenario verification is reported as not applicable")]
fn verification_not_applicable(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        text.contains("changed-scenario verification: not applicable"),
        "{text}"
    );
}

#[then("no replacement scenario is required")]
fn no_replacement_scenario(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(!text.contains("missing delta specs"), "{text}");
}

#[then("neither a changed-scenario runner nor a permanent full-suite runner executes")]
fn no_runner_executes(world: &mut GivnWorld) {
    assert!(
        !workflow_env(world).path("verification-invoked").exists(),
        "a verification command executed"
    );
}

#[given("a change has no delta feature files and no explicit no-behavior-change declaration")]
fn no_delta_and_no_declaration(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let out = common::run_givn(workflow_env(world), &["new", "maintenance-demo"]);
    assert!(out.status.success());
    let change = change_dir_for(world, "maintenance-demo");
    std::fs::write(change.join("proposal.md"), "# Proposal: maintenance-demo\n")
        .expect("write proposal");
    std::fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- maintenance\n",
    )
    .expect("write tasks");
    write_recording_runner(world);
    common::commit_fixture(workflow_env(world));
    world.last_change_id = Some("maintenance-demo".to_string());
}

#[then("the command exits non-zero and explains the missing declaration or delta specs")]
fn explains_missing_declaration(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("missing delta specs"), "{stderr}");
    assert!(stderr.contains("Behavior change: none"), "{stderr}");
}

#[then("no permanent full-suite runner executes as a substitute")]
fn no_full_suite_substitute(world: &mut GivnWorld) {
    assert!(!workflow_env(world).path("verification-invoked").exists());
}

#[given(
    regex = r#"^the change has (a mismatched change identity|a checked task with a placeholder revision)$"#
)]
fn no_behavior_integrity_failure(world: &mut GivnWorld, failure: String) {
    let id = current_change_id(world);
    let change = change_dir_for(world, &id);
    match failure.as_str() {
        "a mismatched change identity" => {
            std::fs::write(
                change.join("proposal.md"),
                "# Proposal: another-change\n\nBehavior change: none\n",
            )
            .expect("write mismatched identity");
        }
        "a checked task with a placeholder revision" => {
            std::fs::write(
                change.join("tasks.md"),
                "- [x] Scenario: Maintenance task\n  - [x] COMMIT: ___ -- placeholder\n",
            )
            .expect("write placeholder tasks");
        }
        other => panic!("unknown identity failure {other}"),
    }
    common::commit_fixture(workflow_env(world));
}

#[then(regex = r#"^the command exits non-zero and names the applicable integrity failure$"#)]
fn names_integrity_failure(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("change identity mismatch") || stderr.contains("placeholder revision"),
        "{stderr}"
    );
}

// --- Review routes --------------------------------------------------------

#[given("a change explicitly declares no behavior change and has no delta feature files")]
fn review_no_behavior_change(world: &mut GivnWorld) {
    write_no_behavior_change(
        world,
        "maintenance-demo",
        "maintenance-demo",
        "- [x] COMMIT: a1b2c3d4 -- maintenance\n",
    );
}

#[given("changed-scenario verification is recorded as not applicable")]
fn recorded_not_applicable(world: &mut GivnWorld) {
    let id = current_change_id(world);
    assert!(
        givn::proof::change_verification_not_applicable(&change_dir_for(world, &id))
            .expect("not-applicable inspection")
    );
}

#[given("the completed review and all applicable integrity checks pass")]
fn review_and_integrity_pass(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let review =
        std::fs::read_to_string(change_dir_for(world, &id).join("review.md")).expect("read review");
    assert!(review.contains("REVIEW: PASS"));
}

#[then("the command succeeds without requiring a changed-scenario receipt")]
fn succeeds_without_receipt(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    assert!(
        !change_dir_for(world, &current_change_id(world))
            .join("verification/tasks-verification.json")
            .exists()
    );
}

#[then("no verification suite executes")]
fn no_suite_executes(world: &mut GivnWorld) {
    assert!(!workflow_env(world).path("verification-invoked").exists());
}

#[given("a change has current applicable verification evidence")]
fn current_verification_evidence(world: &mut GivnWorld) {
    review_ready_change(world, "test-change");
}

#[given("the completed review and applicable integrity checks pass")]
fn review_and_integrity_pass_short(world: &mut GivnWorld) {
    review_and_integrity_pass(world);
}

#[then("neither change-scoped nor full-suite verification executes")]
fn no_verification_executes(world: &mut GivnWorld) {
    let change = change_dir_for(world, &current_change_id(world));
    assert!(!change.join("verification/tasks-result.json").exists());
    assert!(!change.join("verification/invocations.log").exists());
}

#[then("no future archive receipt is required")]
fn no_archive_receipt_required(world: &mut GivnWorld) {
    let root = project_root(world);
    assert!(!root.join("givn/archive/test-change").exists());
}

#[given("a change is ready for changed-scenario verification and its review has not been written")]
fn verification_without_review(world: &mut GivnWorld) {
    verification_ready_change(world, "verify-demo");
    let change = change_dir_for(world, "verify-demo");
    let _ = std::fs::remove_file(change.join("review.md"));
    common::commit_fixture(workflow_env(world));
}

#[given("no archive candidate or archive receipt exists")]
fn no_archive_candidate(world: &mut GivnWorld) {
    assert!(
        !project_root(world)
            .join("givn/archive/verify-demo")
            .exists()
    );
    assert!(
        !change_dir_for(world, "verify-demo")
            .join("givn/archive/verify-demo")
            .exists()
    );
}

#[given("the runner writes fresh valid passing results for stable inputs")]
fn runner_writes_valid_stable(world: &mut GivnWorld) {
    write_workflow_runner(
        world,
        "#!/bin/sh\nprintf '{\"scope\":\"change\",\"total\":2,\"passed\":2,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
    );
    commit_workflow_runner(world);
}

#[then("the command succeeds and records passed verification evidence")]
fn succeeds_with_evidence(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    let receipt = givn::proof::ChangeReceipt::read(&change_dir_for(world, "verify-demo"))
        .expect("read receipt");
    assert_eq!(receipt.status, "passed");
}

#[then("it does not require a completed review or future archive proof")]
fn no_review_or_archive_proof(world: &mut GivnWorld) {
    assert!(
        !change_dir_for(world, "verify-demo")
            .join("review.md")
            .exists()
    );
}

// --- Batch traceability ---------------------------------------------------

fn batch_change(world: &mut GivnWorld) {
    verification_ready_change(world, "verify-demo");
}

#[given("a change checks two scenarios in one named behavior batch")]
fn two_scenarios_one_batch(world: &mut GivnWorld) {
    batch_change(world);
    let id = current_change_id(world);
    common::git_commit_all(workflow_env(world), "feat: Batch first-and-second");
    let hash = common::git(workflow_env(world), &["rev-parse", "HEAD"]).stdout;
    let hash = String::from_utf8_lossy(&hash).trim().to_string();
    std::fs::write(
        change_dir_for(world, &id).join("tasks.md"),
        format!(
            "- [x] Scenario: First scenario\n  - [x] COMMIT: {hash} -- Batch: first-and-second\n\
             - [x] Scenario: Second scenario\n  - [x] COMMIT: {hash} -- Batch: first-and-second\n"
        ),
    )
    .expect("write batch tasks");
    common::commit_fixture(workflow_env(world));
    world.last_change_id = Some(id);
}

#[given("both checked scenarios record the same reachable revision")]
fn same_reachable_revision(world: &mut GivnWorld) {
    let tasks =
        std::fs::read_to_string(change_dir_for(world, &current_change_id(world)).join("tasks.md"))
            .expect("read tasks");
    assert_eq!(tasks.matches("COMMIT:").count(), 2);
}

#[given("human review confirms that revision contains their implementation or regression tests")]
fn human_review_confirms(_world: &mut GivnWorld) {}

#[given("the revision message names the behavior batch")]
fn revision_names_batch(world: &mut GivnWorld) {
    let tasks =
        std::fs::read_to_string(change_dir_for(world, &current_change_id(world)).join("tasks.md"))
            .expect("read tasks");
    assert!(tasks.contains("first-and-second"));
}

#[given("applicable change verification passes")]
fn applicable_verification_passes(world: &mut GivnWorld) {
    // The fixture runner emits a passing change-scope result.
    assert!(workflow_env(world).path("emit-result.sh").exists());
}

#[then("the command succeeds and reports that scenario revisions passed")]
fn reports_scenario_revisions(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    assert!(stdout(world).contains("scenario commits passed"));
}

#[then("no separate revision or production edit per scenario is required")]
fn no_separate_revision(_world: &mut GivnWorld) {}

#[then("no automated semantic diff classifier is required")]
fn no_semantic_classifier(_world: &mut GivnWorld) {}

#[given("a checked scenario records a revision that is not reachable from the current revision")]
fn unreachable_revision(world: &mut GivnWorld) {
    verification_ready_change(world, "verify-demo");
    let env = workflow_env(world);
    let branch =
        String::from_utf8_lossy(&common::git(env, &["rev-parse", "--abbrev-ref", "HEAD"]).stdout)
            .trim()
            .to_string();
    assert!(
        common::git(env, &["checkout", "-q", "-b", "side"])
            .status
            .success()
    );
    common::git_commit_all(env, "feat: Side work");
    let hash = String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_string();
    assert!(
        common::git(env, &["checkout", "-q", &branch])
            .status
            .success()
    );
    std::fs::write(
        change_dir_for(world, "verify-demo").join("tasks.md"),
        format!(
            "- [x] Scenario: Unreachable scenario\n  - [x] COMMIT: {hash} -- Unreachable scenario\n"
        ),
    )
    .expect("write unreachable tasks");
    common::commit_fixture(workflow_env(world));
}

#[given("all other applicable task checks pass")]
fn other_task_checks_pass(world: &mut GivnWorld) {
    assert!(
        change_dir_for(world, &current_change_id(world))
            .join("tasks.md")
            .exists()
    );
}

#[then("the command exits non-zero and names the scenario and its unreachable revision")]
fn names_unreachable_revision(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unreachable scenario"), "{stderr}");
    assert!(stderr.contains("not reachable from HEAD"), "{stderr}");
}

#[given(
    "a completed batch records its actual execution evidence and reachable implementation or regression-test revision"
)]
fn completed_batch_evidence(world: &mut GivnWorld) {
    review_ready_change(world, "verify-demo");
}

#[given("all applicable verification and review checks pass")]
fn all_verification_review_pass(world: &mut GivnWorld) {
    assert!(
        change_dir_for(world, &current_change_id(world))
            .join("verification/tasks-verification.json")
            .exists()
    );
}

#[given("the batch has no separate RED, GREEN or REFACTOR revisions")]
fn no_tdd_revisions(world: &mut GivnWorld) {
    let log = common::git(workflow_env(world), &["log", "--format=%s"]);
    let text = String::from_utf8_lossy(&log.stdout).to_lowercase();
    assert!(!text.contains("red:") && !text.contains("green:") && !text.contains("refactor:"));
}

#[then("the command succeeds without a retrospective commit-shape or TDD-order gate")]
fn no_tdd_gate(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    let text = format!("{}{}", stdout(world), stderr(world));
    assert!(!text.contains("TDD"), "{text}");
}

// --- Archive static gates -------------------------------------------------

#[then("the command exits non-zero and names the missing README impact decision")]
fn names_missing_readme_impact(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("README impact decision"), "{stderr}");
}

#[then("the active change and permanent specs are unchanged")]
fn active_change_and_specs_unchanged(world: &mut GivnWorld) {
    let root = project_root(world);
    assert!(
        root.join("givn/changes/integrity-hook-and-archive-gate")
            .is_dir()
    );
    let permanent = root.join("givn/specs/test-cap/test-cap.feature");
    assert_eq!(
        std::fs::read_to_string(permanent).expect("read permanent spec"),
        "Feature: Test Capability\n"
    );
}

// --- Preservation comparison ----------------------------------------------

fn preservation_usecase(id: &str, capability: &str) -> String {
    format!(
        "# Use case: {id}\n\n\
         ## Level\n\n!\n\n\
         ## Actors\n\n- Operator\n\n\
         ## Goal\n\nPreserve behavior.\n\n\
         ## Trigger\n\nThe operator asks.\n\n\
         ## Preconditions\n\n- A project exists.\n\n\
         ## Main flow\n\n1. Act.\n\n\
         ## Extensions\n\n- none\n\n\
         ## Rules\n\n- none\n\n\
         ## Examples\n\n- none\n\n\
         ## Minimal guarantee\n\nInspectable.\n\n\
         ## Success guarantee\n\nPreserved.\n\n\
         ## Personas\n\n- none\n\n\
         ## Capabilities\n\n- {capability}\n\n\
         ## Interactions\n\n- none\n\n\
         ## Includes\n\n- none\n\n\
         ## Extends\n\n- none\n\n\
         ## Out of scope\n\nNothing.\n\n\
         ## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
    )
}

fn preservation_snapshot(
    usecase: &str,
    feature_name: &str,
    feature_body: &str,
) -> givn::spec::coverage::CoverageSnapshot {
    let root = tempfile::TempDir::new().expect("snapshot tempdir");
    let dir = root.path().join(format!("givn/specs/{usecase}"));
    std::fs::create_dir_all(&dir).expect("create snapshot specs");
    std::fs::write(
        dir.join("usecase.md"),
        preservation_usecase(usecase, "capability"),
    )
    .expect("write snapshot usecase");
    std::fs::write(dir.join(format!("{feature_name}.feature")), feature_body)
        .expect("write snapshot feature");
    givn::spec::coverage::snapshot(root.path()).expect("snapshot behavior")
}

fn preservation_feature(tag: &str, examples_tag: &str) -> String {
    format!(
        r#"{tag}
Feature: Preserve
  Background: Setup
    Given a prepared workspace
  @tagged
  Scenario Outline: Preserved behavior
    Given input "<value>":
      """text
      body
      """
      | key | value |
      | a | b |
    When the action runs
    Then the result is <value>
    {examples_tag}
    Examples:
      | value |
      | one |
"#
    )
}

fn base_preservation() -> String {
    preservation_feature("@fast", "@examples")
}

fn setup_preservation(world: &mut GivnWorld, before: &str, after: &str) {
    world.env = Some(common::make_init_root());
    let change = project_root(world).join("givn/changes/migration-test");
    std::fs::create_dir_all(&change).expect("create migration change");
    let before = preservation_snapshot("old", "capability", before);
    let after = preservation_snapshot("old", "capability", after);
    std::fs::write(
        change.join("coverage-before.json"),
        serde_json::to_string(&before).unwrap(),
    )
    .expect("write before snapshot");
    std::fs::write(
        change.join("coverage-after.json"),
        serde_json::to_string(&after).unwrap(),
    )
    .expect("write after snapshot");
}

#[given("matching before and after behavior snapshots for a supported Gherkin feature")]
fn matching_preservation_snapshots(world: &mut GivnWorld) {
    let base = base_preservation();
    setup_preservation(world, &base, &base);
}

#[given(
    "matching before and after behavior snapshots including supported Backgrounds, tags, docstrings, tables and Examples"
)]
fn matching_full_preservation_snapshots(world: &mut GivnWorld) {
    matching_preservation_snapshots(world);
}

#[given(regex = r#"^only the (.+) differs without an explicit retirement$"#)]
fn preservation_content_differs(world: &mut GivnWorld, content: String) {
    let before = base_preservation();
    let after = match content.as_str() {
        "Background setup" => before.replace("a prepared workspace", "another prepared workspace"),
        "inherited execution tags" => before.replace("@fast", "@slow"),
        "step docstring payload" => before.replace("body", "changed body"),
        "step data table cell" => before.replace("| a | b |", "| a | c |"),
        "Scenario Outline Examples value" => before.replace("| one |", "| two |"),
        "Examples execution tags" => before.replace("@examples", "@other"),
        other => panic!("unknown executable content {other}"),
    };
    setup_preservation(world, &before, &after);
}

#[given("only feature paths differ between the snapshots")]
fn preservation_paths_differ(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let change = project_root(world).join("givn/changes/migration-test");
    std::fs::create_dir_all(&change).expect("create migration change");
    let base = base_preservation();
    let before = preservation_snapshot("old", "capability", &base);
    let after = preservation_snapshot("new", "capability", &base);
    std::fs::write(
        change.join("coverage-before.json"),
        serde_json::to_string(&before).unwrap(),
    )
    .expect("write before snapshot");
    std::fs::write(
        change.join("coverage-after.json"),
        serde_json::to_string(&after).unwrap(),
    )
    .expect("write after snapshot");
}

#[then("preservation comparison reports missing or changed behavior")]
fn preservation_reports_missing(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("migration coverage regression"), "{stderr}");
    let comparison =
        project_root(world).join("givn/changes/migration-test/coverage-comparison.json");
    let text = std::fs::read_to_string(comparison).expect("read comparison");
    let value: serde_json::Value = serde_json::from_str(&text).expect("comparison json");
    assert!(
        value["missing_behavior"]
            .as_array()
            .is_some_and(|entries| !entries.is_empty()),
        "{text}"
    );
}

#[then("preservation comparison passes without an extra verification suite")]
fn preservation_passes(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    assert!(stdout(world).contains("migration coverage passed"));
    assert!(!project_root(world).join("verification-invoked").exists());
}

#[then(
    "the result claims supported executable-content preservation rather than semantic equivalence"
)]
fn preservation_claim(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(text.contains("executable-content preservation"), "{text}");
    assert!(text.contains("not semantic equivalence"), "{text}");
}

// --- Actual verdict recognition -------------------------------------------

fn verdict_ready_review(world: &mut GivnWorld) {
    review_ready_change(world, "verify-demo");
    std::fs::write(
        change_dir_for(world, "verify-demo").join("review.md"),
        "README-IMPACT: none\n",
    )
    .expect("write review without verdict");
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, "verify-demo");
}

#[given(
    "a change has current applicable verification evidence and otherwise complete review evidence"
)]
fn verification_and_complete_review(world: &mut GivnWorld) {
    verdict_ready_review(world);
}

#[given("a change has otherwise valid review evidence")]
fn otherwise_valid_review(world: &mut GivnWorld) {
    verdict_ready_review(world);
}

fn append_review(world: &GivnWorld, text: &str) {
    let path = change_dir_for(world, &current_change_id(world)).join("review.md");
    let mut review = std::fs::read_to_string(&path).unwrap_or_default();
    review.push_str(text);
    std::fs::write(path, review).expect("append review");
}

#[given(regex = r#"^its review contains a real verdict "([^"]+)"$"#)]
fn review_real_verdict(world: &mut GivnWorld, verdict: String) {
    append_review(world, &format!("\n{verdict}\n"));
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &current_change_id(world));
}

#[given(
    regex = r#"^its review contains "([^"]+)" only inside (an HTML comment|a fenced code example)$"#
)]
fn review_verdict_example(world: &mut GivnWorld, verdict: String, context: String) {
    let block = if context == "an HTML comment" {
        format!("\n<!-- {verdict} -->\n")
    } else {
        format!("\n```\n{verdict}\n```\n")
    };
    append_review(world, &block);
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &current_change_id(world));
}

#[given(
    "its review contains actual verdict declarations \"VERDICT: PASS\" and \"VERDICT: FAIL\" outside comments and fences"
)]
fn review_conflicting_verdicts(world: &mut GivnWorld) {
    append_review(world, "\nVERDICT: PASS\nVERDICT: FAIL\n");
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &current_change_id(world));
}

#[given("its review contains only \"This review does not PASS\" instead of a verdict declaration")]
fn review_negated_verdict(world: &mut GivnWorld) {
    let path = change_dir_for(world, &current_change_id(world)).join("review.md");
    std::fs::write(path, "README-IMPACT: none\n\nThis review does not PASS\n")
        .expect("write negated review");
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &current_change_id(world));
}

#[then("the command exits non-zero because the actual verdict is FAIL")]
fn actual_verdict_fail(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("actual verdict is FAIL"), "{stderr}");
}

#[then("the command succeeds without reporting conflicting verdicts")]
fn succeeds_without_conflict(world: &mut GivnWorld) {
    assert_eq!(output(world).status.code(), Some(0), "{}", stderr(world));
    assert!(!stderr(world).contains("conflicting"));
}

#[then("the command exits non-zero and reports conflicting verdicts")]
fn conflicting_verdicts(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("conflicting actual verdicts"), "{stderr}");
}

#[then("the command exits non-zero because a valid actual verdict is missing")]
fn missing_actual_verdict(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("missing a valid actual verdict"),
        "{stderr}"
    );
}

// --- Independent cheap failures -------------------------------------------

#[given(
    "a change has a malformed use-case title, a checked task with a placeholder revision and a missing README impact decision"
)]
fn independent_cheap_failures(world: &mut GivnWorld) {
    review_ready_change(world, "verify-demo");
    let change = change_dir_for(world, "verify-demo");
    std::fs::write(change.join("review.md"), "# Review\n\nREVIEW: PASS\n")
        .expect("write review without README impact");
    std::fs::write(
        change.join("tasks.md"),
        "- [x] Scenario: Broken task\n  - [x] COMMIT: ___ -- placeholder\n",
    )
    .expect("write placeholder tasks");
    let usecase = change.join("specs/demo-cap/usecase.md");
    std::fs::create_dir_all(usecase.parent().expect("usecase parent")).expect("create usecase dir");
    std::fs::write(&usecase, "# Use Case: Demo Cap\n\n## Level\n\n!\n")
        .expect("write malformed usecase");
    let result = change.join("verification/tasks-result.json");
    let _ = std::fs::remove_file(result);
    common::commit_fixture(workflow_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, "verify-demo");
}

#[then("the command exits non-zero and reports all three independent failures together")]
fn reports_three_failures(world: &mut GivnWorld) {
    let out = output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("use-case") || stderr.contains("Use Case"),
        "{stderr}"
    );
    assert!(stderr.contains("placeholder revision"), "{stderr}");
    assert!(stderr.contains("README impact decision"), "{stderr}");
}

#[then("capability checks requiring the malformed use-case identity are reported as not performed")]
fn capability_checks_deferred(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        text.contains("capability checks requiring valid specification identity: not performed"),
        "{text}"
    );
}

#[then("no dependent success is fabricated and no expensive verification executes")]
fn no_dependent_success(world: &mut GivnWorld) {
    let text = format!("{}{}", stdout(world), stderr(world));
    assert!(!text.contains("overlap dispositions passed"), "{text}");
    assert!(
        !text.contains("capability routing decisions do not match"),
        "{text}"
    );
    assert!(
        !change_dir_for(world, &current_change_id(world))
            .join("verification/tasks-result.json")
            .exists()
    );
}

// --- Bare check guidance --------------------------------------------------

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

#[when("the developer runs the migration coverage check")]
fn developer_runs_migration_coverage(world: &mut GivnWorld) {
    let out = common::run_givn(
        workflow_env(world),
        &["check", "migration", "--change", "migration-test"],
    );
    world.last_output = Some(out);
}

// --- declared e2e boundary fixtures ---------------------------------------

#[given(
    regex = r#"^an ise-effective change ready for change-scoped verification whose e2e role "([^"]+)"$"#
)]
fn ise_change_with_e2e_role(world: &mut GivnWorld, state: String) {
    e2e_boundary_support::ise_change(world);
    match state.as_str() {
        "matches no project path" => {
            e2e_boundary_support::declare(world, &["tests/e2e"], &["src/**"]);
        }
        "resolves a package with a malformed manifest" => {
            e2e_boundary_support::declare(world, &["tests/e2e"], &["src/**"]);
            e2e_boundary_support::write_manifest(
                world,
                "tests/e2e/Cargo.toml",
                "[package\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n",
            );
        }
        other => panic!("unknown e2e role state '{other}'"),
    }
    e2e_boundary_support::commit(world);
}

#[given(
    regex = r#"^an ise-effective change ready for change-scoped verification whose e2e package reaches a source package through a "([^"]+)" dependency$"#
)]
fn ise_change_with_source_edge(world: &mut GivnWorld, edge: String) {
    e2e_boundary_support::ise_change(world);
    e2e_boundary_support::declare(world, &["tests/e2e"], &["src/**"]);
    match edge.as_str() {
        "direct" => {
            e2e_boundary_support::write_manifest(
                world,
                "tests/e2e/Cargo.toml",
                "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
                 [dependencies]\ngivn_product = { path = \"../../src\" }\n",
            );
            e2e_boundary_support::write_manifest(
                world,
                "src/Cargo.toml",
                "[package]\nname = \"givn_product\"\nversion = \"0.1.0\"\n",
            );
        }
        "transitive" => {
            e2e_boundary_support::write_manifest(
                world,
                "tests/e2e/Cargo.toml",
                "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
                 [dependencies]\nhelper = { path = \"../fixtures/helper\" }\n",
            );
            e2e_boundary_support::write_manifest(
                world,
                "tests/fixtures/helper/Cargo.toml",
                "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n\n\
                 [dependencies]\ngivn_product = { path = \"../../../src\" }\n",
            );
            e2e_boundary_support::write_manifest(
                world,
                "src/Cargo.toml",
                "[package]\nname = \"givn_product\"\nversion = \"0.1.0\"\n",
            );
        }
        "workspace-inherited" => {
            e2e_boundary_support::write_manifest(
                world,
                "Cargo.toml",
                "[workspace]\nmembers = [\"tests/e2e\"]\n\n\
                 [workspace.dependencies]\ngivn_product = { path = \"src\" }\n",
            );
            e2e_boundary_support::write_manifest(
                world,
                "tests/e2e/Cargo.toml",
                "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
                 [dependencies]\ngivn_product = { workspace = true }\n",
            );
            e2e_boundary_support::write_manifest(
                world,
                "src/Cargo.toml",
                "[package]\nname = \"givn_product\"\nversion = \"0.1.0\"\n",
            );
        }
        other => panic!("unknown dependency edge '{other}'"),
    }
    e2e_boundary_support::commit(world);
}

#[given(
    "an ise-effective change ready for change-scoped verification whose e2e path sits under a source root"
)]
fn ise_change_with_path_under_source(world: &mut GivnWorld) {
    e2e_boundary_support::ise_change(world);
    e2e_boundary_support::declare(world, &["src/e2e-tests"], &["src/**"]);
    e2e_boundary_support::write_file(world, "src/e2e-tests/harness.txt", "fixture\n");
    e2e_boundary_support::commit(world);
}

#[given(
    "an ise-effective change ready for change-scoped verification whose e2e package contains a source root"
)]
fn ise_change_with_source_inside_package(world: &mut GivnWorld) {
    e2e_boundary_support::ise_change(world);
    e2e_boundary_support::declare(world, &["."], &["src/**"]);
    e2e_boundary_support::write_manifest(
        world,
        "Cargo.toml",
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
    );
    e2e_boundary_support::write_file(world, "src/lib.rs", "pub fn helper() {}\n");
    e2e_boundary_support::commit(world);
}

#[given(
    "an ise-effective change ready for change-scoped verification whose e2e path resolves no package and imports a package under a source root"
)]
fn ise_change_with_import_edge(world: &mut GivnWorld) {
    e2e_boundary_support::ise_change(world);
    e2e_boundary_support::declare(world, &["tests/e2e"], &["src/**"]);
    e2e_boundary_support::write_file(
        world,
        "tests/e2e/steps.rs",
        "use givn::proof::ChangeReceipt;\n\nmod ../../src/helpers;\n",
    );
    e2e_boundary_support::write_file(world, "src/helpers.rs", "pub fn helper() {}\n");
    e2e_boundary_support::set_import_target(world, "src/helpers");
    e2e_boundary_support::commit(world);
}

#[given(
    regex = r#"^an ise-effective change ready for change-scoped verification whose E2E runner command is "([^"]+)"$"#
)]
fn ise_change_with_runner_command(world: &mut GivnWorld, command: String) {
    e2e_boundary_support::ise_change(world);
    e2e_boundary_support::declare(world, &["tests/e2e"], &["src/**"]);
    e2e_boundary_support::write_manifest(
        world,
        "tests/e2e/Cargo.toml",
        "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n",
    );
    e2e_boundary_support::set_e2e_command(world, &command);
    e2e_boundary_support::commit(world);
}

#[then(regex = r#"^stderr names the declared e2e package and "([^"]+)"$"#)]
fn stderr_names_declared_e2e_package(world: &mut GivnWorld, condition: String) {
    let observed = stderr(world);
    let entry = e2e_boundary_support::state(world)
        .declared_entry
        .clone()
        .expect("the fixture declares an e2e entry");
    assert!(
        observed.contains(&entry),
        "stderr does not name the declared e2e entry {entry:?}:\n{observed}"
    );
    assert!(
        observed.contains(&condition),
        "stderr does not state the {condition:?} condition:\n{observed}"
    );
}

#[then("stderr names the e2e path and the source root")]
fn stderr_names_e2e_path_and_source_root(world: &mut GivnWorld) {
    let observed = stderr(world);
    let state = e2e_boundary_support::state(world);
    let path = state
        .declared_entry
        .clone()
        .expect("the fixture declares an e2e path");
    assert!(
        observed.contains(&path),
        "stderr does not name the e2e path {path:?}:\n{observed}"
    );
    assert!(
        observed.contains("source root"),
        "stderr does not name the source root condition:\n{observed}"
    );
    assert!(
        state
            .source_roots
            .iter()
            .any(|root| observed.contains(root)),
        "stderr names no declared source root:\n{observed}"
    );
}

#[then("stderr names the e2e package and the source root")]
fn stderr_names_e2e_package_and_source_root(world: &mut GivnWorld) {
    let observed = stderr(world);
    let state = e2e_boundary_support::state(world);
    assert!(
        observed.contains("e2e package"),
        "stderr does not name the e2e package:\n{observed}"
    );
    assert!(
        state
            .source_roots
            .iter()
            .any(|root| observed.contains(root)),
        "stderr names no declared source root:\n{observed}"
    );
}

#[then("stderr names the e2e path and the imported source package")]
fn stderr_names_e2e_path_and_imported_package(world: &mut GivnWorld) {
    let observed = stderr(world);
    let state = e2e_boundary_support::state(world);
    let path = state
        .declared_entry
        .clone()
        .expect("the fixture declares an e2e path");
    let target = state
        .import_target
        .clone()
        .expect("the fixture imports a source path");
    assert!(
        observed
            .lines()
            .any(|line| line.contains("imports the source path")
                && line.contains(&path)
                && line.contains(&target)),
        "stderr does not name the import edge from {path:?} to {target:?}:\n{observed}"
    );
}

#[then("stderr names the declared e2e root and the E2E runner command")]
fn stderr_names_e2e_root_and_runner_command(world: &mut GivnWorld) {
    let observed = stderr(world);
    let state = e2e_boundary_support::state(world);
    let root = state
        .declared_entry
        .clone()
        .expect("the fixture declares an e2e root");
    let command = state
        .runner_command
        .clone()
        .expect("the fixture declares an E2E runner command");
    assert!(
        observed.contains(&root),
        "stderr does not name the declared e2e root {root:?}:\n{observed}"
    );
    assert!(
        observed.contains("E2E runner command"),
        "stderr does not name the E2E runner command:\n{observed}"
    );
    if command.is_empty() {
        assert!(
            observed.contains("command ''"),
            "an empty E2E runner command is not rendered explicitly:\n{observed}"
        );
    } else {
        assert!(
            observed.contains(&command),
            "stderr does not show the E2E runner command {command:?}:\n{observed}"
        );
    }
}

#[then("no change verification receipt is written")]
fn no_change_verification_receipt(world: &mut GivnWorld) {
    let id = current_change_id(world);
    let receipt = change_dir_for(world, &id).join("verification/tasks-verification.json");
    assert!(
        !receipt.exists(),
        "a change verification receipt was written at {}",
        receipt.display()
    );
}
