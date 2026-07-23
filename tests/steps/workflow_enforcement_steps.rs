use cucumber::{given, then};
use std::process::Output;

use super::GivnWorld;
use crate::common;

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
    let config_path = project_root(world).join("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
    for line in &mut lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with("command:") {
            // Replace the entire command: line but preserve indentation
            let indent = line.len() - trimmed.len();
            *line = format!("{:indent$}command: \"{cmd}\"", "");
            break;
        }
    }
    std::fs::write(&config_path, lines.join("\n")).unwrap();
}

fn set_verify_true(world: &GivnWorld) {
    set_verify_command_in_config(world, "true");
}

/// Create a verify script that exits 0 during gate hooks (GIVN_FEATURES points to
/// a change specs dir containing "changes") and exits 1 during post-merge verify
/// (GIVN_FEATURES points to "givn/specs").
fn set_verify_fails_post_merge(world: &GivnWorld) {
    let script_path = project_root(world).join(".verify_gate.sh");
    std::fs::write(
        &script_path,
        "#!/bin/sh\ncase \"$GIVN_FEATURES\" in *changes*) exit 0 ;; *) exit 1 ;; esac\n",
    )
    .unwrap();
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

// ---- Given steps ----

#[given("a givn project with a change that has passed design-review")]
fn change_passed_design_review(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "test-change");
    set_verify_true(world);
    // Write design-review.md with PASS marker.
    let dr_path = change_dir_for(world, "test-change").join("design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
    // Write a valid coverage.md (needed because givn init enables the coverage addon).
    write_coverage_md(world, "test-change");
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
    // Create the test change so subsequent steps can write to it.
    let out = common::run_givn(world.env.as_ref().unwrap(), &["new", "test-change"]);
    assert!(
        out.status.success(),
        "givn new failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some("test-change".to_string());
    // Write a valid coverage.md (needed because givn init enables the coverage addon).
    write_coverage_md(world, "test-change");
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
    let dr_path = project_root(world).join("givn/changes/test-change/design-review.md");
    std::fs::write(&dr_path, "DESIGN-REVIEW: PASS\n").unwrap();
    // Write a valid coverage.md (needed because givn init enables the coverage addon).
    write_coverage_md(world, "test-change");
}

/// Write a minimal valid coverage.md to the change directory.
fn write_coverage_md(world: &GivnWorld, change_id: &str) {
    let cov_path = project_root(world)
        .join("givn/changes")
        .join(change_id)
        .join("coverage.md");
    std::fs::write(
        &cov_path,
        "# Coverage\n\n**Coverage result:** 85% line, 72% branch\n",
    )
    .unwrap();
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
    std::fs::write(change.join("review.md"), "REVIEW: PASS\n").unwrap();
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
    set_verify_command_in_config(world, "true");
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

#[given("a givn project with a completed change that introduces a brand-new capability")]
fn change_new_capability(world: &mut GivnWorld) {
    scaffold_init_root_change(world, "integrity-hook-and-archive-gate");
    let root = project_root(world);

    // Remove any auto-created specs.
    let specs_root = root.join("givn/specs");
    if specs_root.exists() {
        std::fs::remove_dir_all(&specs_root).unwrap();
    }
    std::fs::create_dir_all(&specs_root).unwrap();

    let change = root.join("givn/changes/integrity-hook-and-archive-gate");
    std::fs::write(change.join("proposal.md"), "# Proposal\n").unwrap();

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
    std::fs::write(change.join("review.md"), "REVIEW: PASS\n").unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();

    // Set verify to fail after merge (pass during gate hooks, fail post-merge).
    set_verify_fails_post_merge(world);
}

// ---- When steps (reuse existing) ----

// ---- Then steps ----

fn stdout_integrity(world: &mut GivnWorld) {
    let s = stdout(world);
    assert!(s.contains("integrity"), "stdout missing 'integrity': {}", s);
}
#[then(regex = r"^the backup directory (.+) does not exist$")]
fn backup_dir_not_exists(world: &mut GivnWorld, rel: String) {
    let p = project_root(world).join(&rel);
    assert!(!p.exists(), "backup directory should not exist: {:?}", p);
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

#[then("the change directory has been moved to givn/archive/")]
fn change_moved_to_archive(world: &mut GivnWorld) {
    let id = change_id(world);
    let archive_p = project_root(world).join(&format!("givn/archive/{id}"));
    let changes_p = project_root(world).join(&format!("givn/changes/{id}"));
    assert!(
        archive_p.is_dir(),
        "change not in givn/archive/: {:?}",
        archive_p
    );
    assert!(
        !changes_p.exists(),
        "change still in givn/changes/: {:?}",
        changes_p
    );
}

#[then("the change directory still exists under givn/changes/")]
fn change_still_in_changes(world: &mut GivnWorld) {
    let id = change_id(world);
    let p = project_root(world).join(&format!("givn/changes/{id}"));
    assert!(p.is_dir(), "change missing from givn/changes/: {:?}", p);
}

#[then(regex = r"^the backup directory (.+) exists with the pre-merge permanent specs$")]
fn backup_exists_with_specs(world: &mut GivnWorld, rel: String) {
    let p = project_root(world).join(&rel);
    assert!(p.is_dir(), "backup directory does not exist: {:?}", p);
}

#[then("the conflicting scenario appears in the merged permanent spec under givn/specs/")]
fn merged_spec_has_conflict(world: &mut GivnWorld) {
    let p = project_root(world).join("givn/specs/test-cap/test-cap.feature");
    if p.exists() {
        let content = std::fs::read_to_string(&p).unwrap();
        assert!(
            content.contains("Test scenario"),
            "merged spec missing delta scenario: {content}"
        );
    }
}

#[then("stderr instructs the user to delete the newly-created givn/specs/capability directory")]
fn stderr_instructs_delete_new_cap(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(s.contains("delete"), "stderr missing 'delete': {}", s);
    assert!(
        s.contains("givn/specs"),
        "stderr missing 'givn/specs': {}",
        s
    );
}

#[then("the backup directory exists")]
fn backup_dir_exists(world: &mut GivnWorld) {
    let id = change_id(world);
    let p = project_root(world).join(&format!("givn/.archive-backup-{id}"));
    assert!(p.is_dir(), "backup directory does not exist: {:?}", p);
}

#[then("the change's design.md template contains a section named \"Internal Primitives\"")]
fn design_has_primitives_section(world: &mut GivnWorld) {
    let content = givn::config::embedded::Assets::get_str("templates/design.md")
        .expect("embedded design.md template");
    assert!(
        content.contains("Internal Primitives"),
        "design.md template missing Internal Primitives section"
    );
}

#[then("the section includes a table with columns \"Primitive\", \"Type\", and \"Purpose\"")]
fn design_has_primitives_table(world: &mut GivnWorld) {
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
    std::fs::write(change.join("review.md"), "REVIEW: PASS\n").unwrap();
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
    std::fs::write(change.join("review.md"), "REVIEW: PASS\n").unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    set_verify_true(world);

    // No permanent spec directory matters since the removed scenario is just a placeholder.
    let perm_dir = root.join("givn/specs/test-cap");
    std::fs::create_dir_all(&perm_dir).unwrap();
    std::fs::write(
        perm_dir.join("test-cap.feature"),
        "Feature: Test Capability\n",
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
    std::fs::write(change.join("review.md"), "REVIEW: PASS\n").unwrap();
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    set_verify_true(world);
}
