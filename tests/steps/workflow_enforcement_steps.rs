use cucumber::{given, then, when};
use std::process::Output;

use super::GivnWorld;
use crate::common;

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
    let config_path = project_root(world).join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let updated = config.replace("coverage: true", "coverage: false");
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
        common::BackendKind::Git => String::from_utf8_lossy(
            &common::git(env, &["rev-parse", "HEAD"]).stdout,
        )
        .trim()
        .to_string(),
        common::BackendKind::Fossil => String::from_utf8_lossy(
            &common::fossil(env, &["info"]).stdout,
        )
        .lines()
        .find_map(|line| line.strip_prefix("checkout:"))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_default()
        .to_string(),
    };
    givn::proof::write_change_receipt(
        &root,
        &change,
        id,
        &head,
        "./emit-result.sh regular",
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

#[given("a givn project with a change that has passed design-review")]
fn change_passed_design_review(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "test-change");
    set_verify_true(world);
    set_coverage_true(world);
    // Write design-review.md with PASS marker.
    let dr_path = change_dir_for(world, "test-change").join("design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
    // The visual-review fixtures name `fixture-persona`; the review-time
    // persona resolution needs the file to exist.
    let personas = project_root(world).join("givn/personas");
    std::fs::create_dir_all(&personas).unwrap();
    std::fs::write(personas.join("fixture-persona.md"), "# Persona: fixture\n").unwrap();
    write_fixture_change_receipt(world, "test-change");
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
}

#[given("a givn project with arc42 addon enabled")]
fn project_arc42_enabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    set_verify_true(world);
    set_coverage_true(world);
    // Create the test change so subsequent steps can write to it.
    let out = common::run_givn(world.env.as_ref().unwrap(), &["new", "test-change"]);
    assert!(
        out.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some("test-change".to_string());
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
    let dr_path = project_root(world).join("givn/changes/test-change/design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
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
        "REVIEW: PASS\nREADME-IMPACT: noneREADME-IMPACT: none",
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
        "REVIEW: PASS\nREADME-IMPACT: noneREADME-IMPACT: none",
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
        "REVIEW: PASS\nREADME-IMPACT: noneREADME-IMPACT: none",
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
        "REVIEW: PASS\nREADME-IMPACT: noneREADME-IMPACT: none",
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
        "@givn.delta @new-cap\nFeature: New Capability\n\n  @givn.added\n  Scenario: New scenario\n    Given x\n    Then y\n",
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
        "REVIEW: PASS\nREADME-IMPACT: noneREADME-IMPACT: none",
    )
    .unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();

    // Set verify to fail after merge (pass during gate hooks, fail post-merge).
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
        .replace("<scenario title>", "Existing scenario");
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
    let id = world.last_change_id.clone().expect("a change id");
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
        common::BackendKind::Git => String::from_utf8_lossy(
            &common::git(env, &["rev-parse", "HEAD"]).stdout,
        )
        .trim()
        .to_string(),
        common::BackendKind::Fossil => String::from_utf8_lossy(
            &common::fossil(env, &["info"]).stdout,
        )
        .lines()
        .find_map(|line| line.strip_prefix("checkout:"))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_default()
        .to_string(),
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
    let value: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(env.path("result.json")).expect("result file"))
            .expect("valid result JSON");
    assert_eq!(value["total"], 3);
    assert_eq!(value["passed"], 3);
    assert_eq!(value["failed"], 0);
    assert_eq!(value["skipped"], 0);
    assert!(value.get("scope").is_none());
}
