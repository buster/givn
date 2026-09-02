//! Shared temporary-project and subprocess helpers for ownership fixtures.

use std::fs;
use std::path::{Path, PathBuf};

use crate::common;
use crate::steps::GivnWorld;

pub(crate) fn setup_fixture(world: &mut GivnWorld, failing: bool) {
    let env = common::make_bare_root();
    let init = common::run_givn(&env, &["init", "--no-addons"]);
    assert!(
        init.status.success(),
        "givn init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let root = env.root();
    write_feature(
        &root.join("givn/specs/fixture-existing/fixture-existing.feature"),
        "Feature: Existing fixture\n\n  Scenario: Obsolete behavior\n    Given a fixture behavior\n    When the old behavior runs\n    Then the old result appears\n",
    );
    write_feature(
        &root.join("givn/specs/fixture-retained/fixture-retained.feature"),
        "Feature: Retained fixture\n\n  Scenario: Existing unrelated behavior\n    Given a different fixture behavior\n    When unrelated behavior runs\n    Then unrelated result appears\n",
    );

    let change = root.join("givn/changes/fixture-consolidation");
    write_feature(
        &change.join("specs/fixture-existing/fixture-existing.feature"),
        "@givn.delta @fixture-existing\nFeature: Existing fixture\n\n  @givn.removed\n  Scenario: Obsolete behavior\n    Given this obsolete scenario is removed\n",
    );
    write_feature(
        &change.join("specs/fixture-retained/fixture-retained.feature"),
        "@givn.delta @fixture-retained\nFeature: Retained fixture\n\n  @givn.added\n  Scenario: Canonical retained behavior\n    Given a consolidated canonical behavior\n    When the consolidated canonical flow runs\n    Then the consolidated canonical result appears\n",
    );

    fs::write(change.join("proposal.md"), "# fixture\n").expect("write proposal");
    fs::write(change.join("design.md"), "# fixture\n").expect("write design");
    fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design review");
    fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: 1111111111111111111111111111111111111111 -- fixture\n",
    )
    .expect("write tasks");
    fs::write(
        change.join("review.md"),
        "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Obsolete behavior` | `Canonical retained behavior` | merge |\n\nREVIEW: PASS\n",
    )
    .expect("write review");
    let verify = if failing { "false" } else { "true" };
    common::write_commands_yaml(
        &root.join("givn/commands.yaml"),
        &format!("verify:\n  command: \"{verify}\"\n  e2e_command: \"true\"\n"),
    );

    world.env = Some(env);
    world.last_change_id = Some("fixture-consolidation".to_string());
    world.ownership_before_tree = Some(snapshot_tree(&fixture_specs(world)));
}

fn write_feature(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().expect("feature parent")).expect("create feature directory");
    fs::write(path, content).expect("write feature");
}

pub(crate) fn invoke_givn(world: &mut GivnWorld, args: &[&str]) {
    let env = world.env.as_ref().expect("fixture project");
    world.last_output = Some(common::run_givn(env, args));
}

pub(crate) fn invoke_archive(world: &mut GivnWorld) {
    invoke_givn(world, &["archive", "--change", "fixture-consolidation"]);
}

pub(crate) fn assert_tree_unchanged(world: &GivnWorld, root: PathBuf) {
    let before = world
        .ownership_before_tree
        .as_ref()
        .expect("fixture snapshot");
    let after = snapshot_tree(&root);
    assert_eq!(after, *before, "archive changed the fixture after failure");
}

fn fixture_root(world: &GivnWorld) -> PathBuf {
    world.env.as_ref().expect("fixture project").root()
}

pub(crate) fn fixture_specs(world: &GivnWorld) -> PathBuf {
    fixture_root(world).join("givn/specs")
}

pub(crate) fn snapshot_tree(root: &Path) -> Vec<(String, Vec<u8>)> {
    let mut entries = Vec::new();
    collect_files(root, &mut entries);
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

fn collect_files(root: &Path, output: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, output);
        } else if let Ok(bytes) = fs::read(&path) {
            output.push((
                path.strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
                bytes,
            ));
        }
    }
}

pub(crate) fn feature_text(root: PathBuf) -> String {
    snapshot_tree(&root)
        .into_iter()
        .filter(|(path, _)| path.ends_with(".feature"))
        .filter_map(|(_, bytes)| String::from_utf8(bytes).ok())
        .collect::<Vec<_>>()
        .join("\n")
}

fn scenario_titles(root: PathBuf) -> Vec<String> {
    feature_text(root)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("Scenario: ").map(str::to_owned))
        .collect()
}

pub(crate) fn duplicate_titles(root: PathBuf) -> Vec<String> {
    let mut counts = std::collections::HashMap::new();
    for title in scenario_titles(root) {
        *counts.entry(title).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(title, count)| (count > 1).then_some(title))
        .collect()
}
