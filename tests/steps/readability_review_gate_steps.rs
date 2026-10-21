//! Step definitions for the `readability-review-gate` capability.
//!
//! Every Given builds a real temporary Git or Fossil repository through
//! `readability_review_gate_support`; the When runs the public gate API in
//! process; every Then asserts on the rendered report or the repository
//! state. The single `@e2e` scenario drives the real CLI from
//! `readability_review_gate_e2e_steps.rs`.

use crate::steps::GivnWorld;
use crate::steps::readability_review_gate_support as support;
use cucumber::{given, then, when};
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::TestEnv;

/// Per-scenario gate state: the last report, the expected rationale metadata
/// for justified units, and fixture hashes used by the history scenarios.
#[derive(Debug)]
pub(crate) struct GateState {
    pub(crate) change_id: String,
    pub(crate) passed: Option<bool>,
    pub(crate) report: String,
    pub(crate) reports: Vec<String>,
    pub(crate) changed_files: Vec<String>,
    pub(crate) rationales: support::RationaleMap,
    pub(crate) required_hashes: Vec<String>,
    pub(crate) forbidden_hashes: Vec<String>,
    #[allow(dead_code)] // keeps the shallow-clone directory alive
    pub(crate) clone: Option<tempfile::TempDir>,
    pub(crate) root_override: Option<PathBuf>,
    pub(crate) expect_pass_line: bool,
    pub(crate) main_branch: String,
    pub(crate) feature_branch: String,
    pub(crate) ancestor: Option<String>,
    pub(crate) first_touch: Option<String>,
    pub(crate) task_commit: String,
}

impl Default for GateState {
    fn default() -> Self {
        Self {
            change_id: support::CHANGE_ID.to_string(),
            passed: None,
            report: String::new(),
            reports: Vec::new(),
            changed_files: Vec::new(),
            rationales: support::RationaleMap::new(),
            required_hashes: Vec::new(),
            forbidden_hashes: Vec::new(),
            clone: None,
            root_override: None,
            expect_pass_line: false,
            main_branch: "main".to_string(),
            feature_branch: "feature".to_string(),
            ancestor: None,
            first_touch: None,
            task_commit: String::new(),
        }
    }
}

// --- helpers ---------------------------------------------------------------

fn env(world: &GivnWorld) -> &TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn gate<'a>(env: &'a TestEnv) -> support::GateFixture<'a> {
    support::GateFixture::new(env, support::CHANGE_ID)
}

fn remember(world: &mut GivnWorld, unit: &str, classification: &str, finding: &str) {
    world.readability_gate.rationales.insert(
        unit.to_string(),
        (classification.to_string(), finding.to_string()),
    );
}

fn spec_for(state: &GateState, unit: &str) -> support::RationaleSpec {
    let (classification, finding) = state
        .rationales
        .get(unit)
        .cloned()
        .unwrap_or_else(|| ("god-type".to_string(), "new-type".to_string()));
    support::RationaleSpec::complete(unit, &classification, &finding)
}

fn gate_report(world: &GivnWorld) -> &str {
    let state = &world.readability_gate;
    assert!(
        state.passed.is_some(),
        "the readability review has not assessed the change yet"
    );
    &state.report
}

fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn report_root(world: &GivnWorld) -> PathBuf {
    let state = &world.readability_gate;
    state
        .root_override
        .clone()
        .unwrap_or_else(|| env(world).root())
}

fn diff_paths(before: &[(String, Vec<u8>)], after: &[(String, Vec<u8>)]) -> Vec<String> {
    let before: std::collections::BTreeMap<&str, &[u8]> = before
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect();
    let after: std::collections::BTreeMap<&str, &[u8]> = after
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect();
    let mut changed = Vec::new();
    for (path, bytes) in &after {
        if before.get(path) != Some(bytes) {
            changed.push((*path).to_string());
        }
    }
    for path in before.keys() {
        if !after.contains_key(path) {
            changed.push((*path).to_string());
        }
    }
    changed.sort();
    changed
}

fn run_gate(world: &mut GivnWorld) {
    let root = report_root(world);
    let before = support::snapshot(&root);
    let change_id = world.readability_gate.change_id.clone();
    let (passed, text) = givn::readability::gate::evaluate(&root, &change_id)
        .unwrap_or_else(|error| panic!("gate evaluation failed: {error:#}"));
    let after = support::snapshot(&root);
    let changed = diff_paths(&before, &after);
    let state = &mut world.readability_gate;
    state.passed = Some(passed);
    state.report = text.clone();
    state.reports.push(text);
    state.changed_files = changed;
}

// --- Given: project and history fixtures ----------------------------------

#[given("a Fossil project with the readability experiment enabled")]
fn fossil_project(world: &mut GivnWorld) {
    let env = crate::common::make_fossil_root();
    world.env = Some(env);
    world.readability_gate.change_id = support::CHANGE_ID.to_string();
}

#[given(
    regex = r#"^a committed base with a god type of (\d+) code lines split across files in "([^"]+)"$"#
)]
fn committed_base_god_type(world: &mut GivnWorld, total: u32, declaration: String) {
    {
        let fixture = gate(env(world));
        fixture.base_god_type_split(total, &declaration);
    }
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-grown",
    );
    world.readability_gate.expect_pass_line = true;
}

#[given(regex = r#"^a committed change directory for "([^"]+)"$"#)]
fn committed_change_directory(world: &mut GivnWorld, change_id: String) {
    gate(env(world)).change_dir_commit();
    world.readability_gate.change_id = change_id;
}

#[given(
    regex = r#"^a committed change whose base has a god type of (\d+) code lines split across files in "([^"]+)"$"#
)]
fn change_base_god_type(world: &mut GivnWorld, total: u32, declaration: String) {
    {
        let fixture = gate(env(world));
        fixture.base_god_type_split(total, &declaration);
        fixture.change_dir_commit();
    }
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-grown",
    );
}

#[given("a committed change whose base has no type over the readable size")]
fn change_base_without_large_type(world: &mut GivnWorld) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
}

#[given("a committed change whose base has no module over the readable size")]
fn change_base_without_large_module(world: &mut GivnWorld) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
}

#[given(
    regex = r#"^a committed change whose base has a data-only enum of (\d+) code lines in one file "([^"]+)"$"#
)]
fn change_base_data_only_enum(world: &mut GivnWorld, total: u32, path: String) {
    let fixture = gate(env(world));
    fixture.base_data_only_enum(total, &path);
    fixture.change_dir_commit();
    remember(
        world,
        &format!("module:{path}"),
        "module",
        "existing-not-grown",
    );
}

#[given(regex = r#"^a committed change whose base has a type in "([^"]+)"$"#)]
fn change_base_small_type(world: &mut GivnWorld, path: String) {
    let fixture = gate(env(world));
    fixture.base_small_type_at(&path);
    fixture.change_dir_commit();
}

#[given("a committed change that adds a type over the readable size")]
fn disabled_ready_change(world: &mut GivnWorld) {
    let env = env(world);
    support::write_file(
        env,
        "src/engine.rs",
        &support::type_one_file("Engine", 620, &["a", "b", "c"]),
    );
    support::commit_with_body(
        env,
        "feat(test-change): add the type over the readable size",
        support::READY_TASK_TITLE,
    );
    let task_commit = support::revision(env);
    support::write_review_preconditions(env, support::CHANGE_ID, &task_commit);
    support::commit(env, "docs(test-change): complete the change preconditions");
    support::write_file(
        env,
        &format!("givn/changes/{}/review.md", support::CHANGE_ID),
        "# Review\n\nREADME-IMPACT: none\n",
    );
    support::commit_review_record(env);
    support::run_check_tasks(env, support::CHANGE_ID);
    world.readability_gate.change_id = support::CHANGE_ID.to_string();
}

#[given(
    regex = r#"^a committed change that adds a data-only enum of (\d+) code lines in one file "([^"]+)"$"#
)]
fn change_adds_data_only_enum(world: &mut GivnWorld, total: u32, path: String) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
    fixture.write(&path, &support::data_only_enum("Status", total));
    fixture.commit("feat(test-change): add the status vocabulary");
}

#[given(
    regex = r#"^a committed change that adds a module of exactly (\d+) code lines in "([^"]+)"$"#
)]
fn change_adds_module(world: &mut GivnWorld, total: u32, path: String) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
    fixture.add_module(total, &path);
    remember(world, &format!("module:{path}"), "module", "new-module");
}

#[given(regex = r#"^a committed change that adds a type of (\d+) code lines in "([^"]+)"$"#)]
fn change_adds_type(world: &mut GivnWorld, total: u32, path: String) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
    fixture.add_one_file_type("Engine", total, &path, &["a", "b", "c"]);
    remember(
        world,
        &format!("type:{path}::Engine"),
        "method-length",
        "method-length",
    );
}

#[given(
    regex = r#"^a committed change that adds a type of (\d+) code lines with (\d+) public methods in "([^"]+)"$"#
)]
fn change_adds_interface(world: &mut GivnWorld, total: u32, methods: u32, path: String) {
    let fixture = gate(env(world));
    fixture.base_without_large_units();
    fixture.change_dir_commit();
    fixture.add_interface("Api", total, methods, &path);
    remember(world, &format!("type:{path}::Api"), "interface", "new-type");
}

#[given("a committed change directory with no production sources at its parent")]
fn change_directory_without_production_base(world: &mut GivnWorld) {
    gate(env(world)).change_dir_commit();
}

#[given("a committed change directory with no production sources at base or head")]
fn change_directory_without_production_sources(world: &mut GivnWorld) {
    gate(env(world)).change_dir_commit();
}

#[given(
    regex = r#"^a commit that grows a type to (\d+) code lines before the change directory exists$"#
)]
fn production_commit_before_change_directory(world: &mut GivnWorld, total: u32) {
    gate(env(world)).base_god_type_split(total, "src/engine.rs");
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "crossed-threshold",
    );
}

#[given(
    regex = r#"^the first change commit adds the change directory and grows the type to (\d+) code lines$"#
)]
fn first_change_commit_grows_type(world: &mut GivnWorld, total: u32) {
    let fixture = gate(env(world));
    let change = format!("givn/changes/{}", support::CHANGE_ID);
    fixture.write(
        &format!("{change}/proposal.md"),
        "# Proposal: test change\n",
    );
    fixture.write(&format!("{change}/design.md"), "# Design: test change\n");
    fixture.write(&format!("{change}/tasks.md"), "# Tasks: test change\n");
    let (declaration, parts) = support::type_split("Engine", total, &["a", "b", "c"]);
    fixture.write("src/engine.rs", &declaration);
    fixture.write("src/engine/parts.rs", &parts);
    fixture.commit("feat(test-change): add the change directory and grow the type");
}

#[given(
    regex = r#"^the change directory and a god type of (\d+) code lines are added in the root commit$"#
)]
fn root_commit_with_change_directory(world: &mut GivnWorld, total: u32) {
    let env = env(world);
    let git_dir = env.path(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir).expect("remove the scaffold repository");
    }
    support::git(env, &["init", "-q"]);
    for args in [
        ["config", "user.email", "fixture@example.com"],
        ["config", "user.name", "Givn Fixture"],
        ["config", "commit.gpgsign", "false"],
    ] {
        support::git(env, &args);
    }
    let fixture = gate(env);
    let change = format!("givn/changes/{}", support::CHANGE_ID);
    fixture.write(
        &format!("{change}/proposal.md"),
        "# Proposal: test change\n",
    );
    fixture.write(&format!("{change}/design.md"), "# Design: test change\n");
    fixture.write(&format!("{change}/tasks.md"), "# Tasks: test change\n");
    let (declaration, parts) = support::type_split("Engine", total, &["a", "b", "c"]);
    fixture.write("src/engine.rs", &declaration);
    fixture.write("src/engine/parts.rs", &parts);
    fixture.commit("feat(test-change): root change commit");
    remember(world, "type:src/engine.rs::Engine", "god-type", "new-type");
}

#[given(regex = r#"^a change directory commit grows a type from (\d+) to (\d+) code lines$"#)]
fn change_directory_commit_grows_type(world: &mut GivnWorld, base: u32, head: u32) {
    let env = env(world);
    let fixture = gate(env);
    fixture.base_god_type_split(base, "src/engine.rs");
    let main = support::git_stdout(env, &["branch", "--show-current"]);
    support::git(env, &["checkout", "-q", "-b", "feature"]);
    let change = format!("givn/changes/{}", support::CHANGE_ID);
    fixture.write(
        &format!("{change}/proposal.md"),
        "# Proposal: test change\n",
    );
    fixture.write(&format!("{change}/design.md"), "# Design: test change\n");
    fixture.write(&format!("{change}/tasks.md"), "# Tasks: test change\n");
    let (declaration, parts) = support::type_split("Engine", head, &["a", "b", "c"]);
    fixture.write("src/engine.rs", &declaration);
    fixture.write("src/engine/parts.rs", &parts);
    fixture.commit("feat(test-change): add the change directory and grow the type");
    let ancestor = support::git_stdout(env, &["rev-parse", "HEAD~1"]);
    let state = &mut world.readability_gate;
    state.main_branch = main;
    state.feature_branch = "feature".to_string();
    state.ancestor = Some(ancestor);
}

#[given(
    regex = r#"^a change directory commit on the branch grows a type from (\d+) to (\d+) code lines$"#
)]
fn branch_change_commit_grows_type(world: &mut GivnWorld, base: u32, head: u32) {
    change_directory_commit_grows_type(world, base, head);
}

#[given("a change directory \"givn/changes/test-change/\" that no commit touched")]
fn uncommitted_change_directory(world: &mut GivnWorld) {
    let env = env(world);
    support::write_file(
        env,
        "givn/changes/test-change/proposal.md",
        "# Proposal: test change\n",
    );
    support::write_file(
        env,
        "givn/changes/test-change/design.md",
        "# Design: test change\n",
    );
    support::write_file(
        env,
        "givn/changes/test-change/tasks.md",
        "# Tasks: test change\n",
    );
}

#[given("a shallow clone whose first change commit is beyond the fetched history")]
fn shallow_clone_fixture(world: &mut GivnWorld) {
    let env = env(world);
    {
        let fixture = gate(env);
        fixture.base_god_type_split(620, "src/engine.rs");
        fixture.change_dir_commit();
        fixture.resize_type(480, "src/engine.rs");
    }
    let clone = support::shallow_clone(env);
    let state = &mut world.readability_gate;
    state.root_override = Some(clone.path().to_path_buf());
    state.clone = Some(clone);
}

#[given("a sibling timeline that touches the change directory")]
fn fossil_sibling_timeline(world: &mut GivnWorld) {
    let env = env(world);
    let ancestor = support::fossil_current_hash(env);
    support::fossil(env, &["branch", "new", "sibling", &ancestor]);
    support::fossil(env, &["update", "sibling"]);
    support::write_file(
        env,
        &format!("givn/changes/{}/proposal.md", support::CHANGE_ID),
        "# Proposal: sibling\n",
    );
    support::commit(env, "docs(test-change): sibling timeline change directory");
    let sibling = support::fossil_current_hash(env);
    support::fossil(env, &["update", "trunk"]);
    let state = &mut world.readability_gate;
    state.ancestor = Some(ancestor);
    state.forbidden_hashes.push(sibling[..12].to_string());
}

#[given("an ancestor commit that does not touch the change directory")]
fn fossil_ancestor_commit(world: &mut GivnWorld) {
    let env = env(world);
    support::write_file(
        env,
        &format!("givn/changes/{}/proposal.md", support::CHANGE_ID),
        "# Proposal: trunk\n",
    );
    support::commit(env, "docs(test-change): trunk change directory");
    let output = support::fossil_stdout(
        env,
        &[
            "timeline",
            "-n",
            "0",
            "-t",
            "ci",
            "-F",
            "%H",
            "--reverse",
            "ancestors",
            "current",
            "-p",
            "givn/changes/test-change/",
        ],
    );
    let first_touch = output.lines().next().unwrap_or_default().trim().to_string();
    let state = &mut world.readability_gate;
    state.required_hashes.push(first_touch.clone());
    state.first_touch = Some(first_touch);
}

#[given(
    regex = r#"^a change directory check-in on the branch grows a type from (\d+) to (\d+) code lines$"#
)]
fn fossil_branch_check_in_grows_type(world: &mut GivnWorld, base: u32, head: u32) {
    let env = env(world);
    {
        let fixture = gate(env);
        fixture.base_god_type_split(base, "src/engine.rs");
    }
    let ancestor = support::fossil_current_hash(env);
    support::fossil(env, &["branch", "new", "feature", &ancestor]);
    support::fossil(env, &["update", "feature"]);
    {
        let fixture = gate(env);
        let change = format!("givn/changes/{}", support::CHANGE_ID);
        fixture.write(
            &format!("{change}/proposal.md"),
            "# Proposal: test change\n",
        );
        fixture.write(&format!("{change}/design.md"), "# Design: test change\n");
        fixture.write(&format!("{change}/tasks.md"), "# Tasks: test change\n");
        let (declaration, parts) = support::type_split("Engine", head, &["a", "b", "c"]);
        fixture.write("src/engine.rs", &declaration);
        fixture.write("src/engine/parts.rs", &parts);
        fixture.commit("change(feature): add the change directory and grow the type");
    }
    let state = &mut world.readability_gate;
    state.ancestor = Some(ancestor);
    state.feature_branch = "feature".to_string();
}

#[given(
    regex = r#"^a merged-in check-in grows the type to (\d+) code lines and is named by "merged-from:"$"#
)]
fn fossil_merged_in_check_in(world: &mut GivnWorld, total: u32) {
    let env = env(world);
    let ancestor = world
        .readability_gate
        .ancestor
        .clone()
        .expect("the branch fixture records its ancestor");
    let feature = world.readability_gate.feature_branch.clone();
    support::fossil(env, &["branch", "new", "merged", &ancestor]);
    support::fossil(env, &["update", "merged"]);
    {
        let fixture = gate(env);
        fixture.resize_type(total, "src/engine.rs");
    }
    let merged_in = support::fossil_current_hash(env);
    support::fossil(env, &["update", &feature]);
    let _ = support::fossil_merge(env, "merged");
    {
        let fixture = gate(env);
        let (declaration, parts) = support::type_split("Engine", 620, &["a", "b", "c"]);
        fixture.write("src/engine.rs", &declaration);
        fixture.write("src/engine/parts.rs", &parts);
    }
    support::write_file(env, "merge-note.txt", "resolved to the branch version\n");
    support::commit(env, "merge(feature): merge the sibling line");
    let info = support::fossil_stdout(env, &["info", "current"]);
    assert!(
        info.contains("merged-from:"),
        "the merge check-in must record merged-from: {info}"
    );
    assert!(
        info.contains(&merged_in[..12]),
        "the merged-from field must name the merged-in check-in: {info}"
    );
}

// --- Given: head changes ---------------------------------------------------

#[given(
    regex = r#"^the change adds a god type of (\d+) code lines split across files in "([^"]+)" at head$"#
)]
fn head_add_god_type(world: &mut GivnWorld, total: u32, declaration: String) {
    gate(env(world)).add_split_type("Engine", total, &declaration, &["a", "b", "c"]);
    remember(
        world,
        &format!("type:{declaration}::Engine"),
        "god-type",
        "new-type",
    );
}

#[given(
    regex = r#"^the change adds a god type of (\d+) code lines in one file "([^"]+)" at head$"#
)]
fn head_add_god_type_one_file(world: &mut GivnWorld, total: u32, path: String) {
    gate(env(world)).add_one_file_type("Engine", total, &path, &["a", "b", "c"]);
    remember(
        world,
        &format!("type:{path}::Engine"),
        "god-type",
        "new-type",
    );
}

#[given(regex = r#"^the change adds a module of (\d+) code lines in "([^"]+)" at head$"#)]
fn head_add_module(world: &mut GivnWorld, total: u32, path: String) {
    gate(env(world)).add_module(total, &path);
    remember(world, &format!("module:{path}"), "module", "new-module");
}

#[given(
    regex = r#"^the change adds a cohesive type of (\d+) code lines split across files in "([^"]+)" at head$"#
)]
fn head_add_cohesive_type(world: &mut GivnWorld, total: u32, declaration: String) {
    gate(env(world)).add_cohesive_type("Cohesive", total, &declaration);
    remember(
        world,
        &format!("type:{declaration}::Cohesive"),
        "cohesive-large-type",
        "new-type",
    );
}

#[given(
    regex = r#"^the change adds a type of exactly (\d+) code lines with cohesive methods split across "([^"]+)" and "([^"]+)" at head$"#
)]
fn head_add_boundary_type(world: &mut GivnWorld, total: u32, declaration: String, parts: String) {
    let fixture = gate(env(world));
    let (declaration_source, parts_source) = support::type_split("Engine", total, &["value"]);
    fixture.write(&declaration, &declaration_source);
    fixture.write(&parts, &parts_source);
    fixture.commit("feat(test-change): add the boundary type");
}

#[given(regex = r#"^the change adds two god types in one file "([^"]+)" at head$"#)]
fn head_add_two_types(world: &mut GivnWorld, path: String) {
    gate(env(world)).add_two_types_one_file(620, &path);
    remember(
        world,
        &format!("type:{path}::First"),
        "god-type",
        "new-type",
    );
    remember(
        world,
        &format!("type:{path}::Second"),
        "god-type",
        "new-type",
    );
}

#[given(
    regex = r#"^the change adds a god type "([^"]+)" of (\d+) code lines split across "([^"]+)" and "([^"]+)" at head$"#
)]
fn head_add_named_god_type(
    world: &mut GivnWorld,
    name: String,
    total: u32,
    declaration: String,
    parts: String,
) {
    gate(env(world)).add_split_type_at(&name, total, &declaration, &parts);
    remember(
        world,
        &format!("type:{declaration}::{name}"),
        "god-type",
        "new-type",
    );
}

#[given(regex = r#"^the change grows the type to (\d+) code lines at head$"#)]
fn head_grow_type(world: &mut GivnWorld, total: u32) {
    gate(env(world)).resize_type(total, "src/engine.rs");
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-grown",
    );
}

#[given(regex = r#"^the change shrinks the type to (\d+) code lines at head$"#)]
fn head_shrink_type(world: &mut GivnWorld, total: u32) {
    gate(env(world)).resize_type(total, "src/engine.rs");
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-grown",
    );
}

#[given(regex = r#"^the change keeps the type at (\d+) code lines at head$"#)]
fn head_keep_type(world: &mut GivnWorld, _total: u32) {
    gate(env(world)).touch_without_growing("src/engine.rs");
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-not-grown",
    );
}

#[given("the change touches the type without growing it")]
fn head_touch_type(world: &mut GivnWorld) {
    gate(env(world)).touch_without_growing("src/engine.rs");
    remember(
        world,
        "type:src/engine.rs::Engine",
        "god-type",
        "existing-not-grown",
    );
}

#[given("the change touches the enum without growing it")]
fn head_touch_enum(world: &mut GivnWorld) {
    gate(env(world)).touch_without_growing("src/status.rs");
}

#[given(
    regex = r#"^the change grows the type to (\d+) code lines and adds a god type of (\d+) code lines split across "([^"]+)" and "([^"]+)" at head$"#
)]
fn head_grow_and_add(
    world: &mut GivnWorld,
    total: u32,
    added_total: u32,
    declaration: String,
    parts: String,
) {
    let fixture = gate(env(world));
    fixture.resize_type(total, "src/engine.rs");
    fixture.add_split_type_at("Cohesive", added_total, &declaration, &parts);
    remember(
        world,
        "type:src/cohesive.rs::Cohesive",
        "god-type",
        "new-type",
    );
}

#[given(
    regex = r#"^the change shrinks the type to (\d+) code lines and adds a cohesive type of (\d+) code lines split across files in "([^"]+)"$"#
)]
fn head_shrink_and_add_cohesive(
    world: &mut GivnWorld,
    total: u32,
    added_total: u32,
    declaration: String,
) {
    let env = env(world);
    let fixture = gate(env);
    let (engine_declaration, engine_parts) = support::type_split("Engine", total, &["a", "b", "c"]);
    fixture.write("src/engine.rs", &engine_declaration);
    fixture.write("src/engine/parts.rs", &engine_parts);
    let (cohesive_declaration, cohesive_parts) =
        support::type_split("Cohesive", added_total, &["value"]);
    fixture.write(&declaration, &cohesive_declaration);
    fixture.write(&support::parts_path(&declaration), &cohesive_parts);
    support::commit_with_body(
        env,
        "feat(test-change): reshape the types",
        support::READY_TASK_TITLE,
    );
    world.readability_gate.task_commit = support::revision(env);
    remember(
        world,
        &format!("type:{declaration}::Cohesive"),
        "cohesive-large-type",
        "new-type",
    );
    world.readability_gate.expect_pass_line = true;
}

#[given(regex = r#"^the enum has a method of (\d+) code lines$"#)]
fn enum_long_method(world: &mut GivnWorld, code_lines: u32) {
    gate(env(world)).add_long_method("src/status.rs", "Status", code_lines);
}

#[given(regex = r#"^the type has a method of (\d+) code lines$"#)]
fn type_long_method(world: &mut GivnWorld, code_lines: u32) {
    gate(env(world)).add_long_method("src/engine.rs", "Engine", code_lines);
}

#[given(regex = r#"^the change writes malformed Rust at head in "([^"]+)"$"#)]
fn malformed_source(world: &mut GivnWorld, path: String) {
    gate(env(world)).replace_with_malformed(&path);
}

#[given(
    regex = r#"^the change adds a Rust file in the analyzer-excluded test directory "([^"]+)" at head$"#
)]
fn analyzer_excluded_file(world: &mut GivnWorld, path: String) {
    gate(env(world)).add_excluded_test_file(&path);
}

#[given(
    regex = r#"^the change moves the type and its impl to "([^"]+)" and "([^"]+)" without changing its name$"#
)]
fn move_type(world: &mut GivnWorld, declaration: String, parts: String) {
    gate(env(world)).move_type("src/engine.rs", &declaration, &parts);
    remember(
        world,
        &format!("type:{declaration}::Engine"),
        "god-type",
        "new-type",
    );
}

#[given("the change deletes the type at head")]
fn delete_type(world: &mut GivnWorld) {
    gate(env(world)).delete_paths(&["src/engine.rs", "src/engine/parts.rs"]);
}

#[given(regex = r#"^the change deletes "([^"]+)" and "([^"]+)" at head$"#)]
fn delete_paths(world: &mut GivnWorld, first: String, second: String) {
    gate(env(world)).delete_paths(&[&first, &second]);
}

#[given(regex = r#"^the change replaces "([^"]+)" with a symlink at head$"#)]
fn replace_with_symlink(world: &mut GivnWorld, path: String) {
    #[cfg(unix)]
    gate(env(world)).replace_with_symlink(&path);
    #[cfg(not(unix))]
    {
        let _ = (world, path);
        panic!("the symlink scenario requires a Unix filesystem");
    }
}

#[given("the branch is merged from main with a merge commit")]
fn merge_branch(world: &mut GivnWorld) {
    let env = env(world);
    let main = world.readability_gate.main_branch.clone();
    let feature = world.readability_gate.feature_branch.clone();
    support::git(env, &["checkout", "-q", &main]);
    support::write_file(env, "src/unrelated.rs", "pub struct Unrelated;\n");
    support::commit(env, "chore: unrelated main change");
    support::git(env, &["checkout", "-q", &feature]);
    support::git(env, &["merge", "-q", "--no-edit", &main]);
}

#[given(regex = r#"^the branch is rebased onto a commit that grows the type to (\d+) code lines$"#)]
fn rebase_branch(world: &mut GivnWorld, total: u32) {
    let env = env(world);
    let main = world.readability_gate.main_branch.clone();
    let feature = world.readability_gate.feature_branch.clone();
    support::git(env, &["checkout", "-q", &main]);
    {
        let fixture = gate(env);
        fixture.resize_type(total, "src/engine.rs");
    }
    support::git(env, &["checkout", "-q", &feature]);
    let rebase = support::git_rebase(env, &main);
    assert!(
        rebase.status.success(),
        "git rebase failed: stdout={}\nstderr={}",
        String::from_utf8_lossy(&rebase.stdout),
        String::from_utf8_lossy(&rebase.stderr)
    );
}

#[given("the change has complete step files, design, tasks, README, and routing preconditions")]
fn complete_preconditions(world: &mut GivnWorld) {
    let env = env(world);
    let task_commit = if world.readability_gate.task_commit.is_empty() {
        support::revision(env)
    } else {
        world.readability_gate.task_commit.clone()
    };
    support::write_review_preconditions(env, support::CHANGE_ID, &task_commit);
    support::commit(env, "docs(test-change): complete the change preconditions");
}

#[given("the change's review record exists")]
fn review_record_exists(world: &mut GivnWorld) {
    gate(env(world)).record_review_line("");
}

#[given(regex = r#"^the change's review record records "([^"]+)"$"#)]
fn review_record_line(world: &mut GivnWorld, line: String) {
    gate(env(world)).record_review_line(&line);
}

#[given(regex = r#"^a rationale for "([^"]+)" has every required section$"#)]
fn rationale_complete(world: &mut GivnWorld, unit: String) {
    let spec = spec_for(&world.readability_gate, &unit);
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" has an independent reviewer$"#)]
fn rationale_independent(world: &mut GivnWorld, unit: String) {
    let spec = spec_for(&world.readability_gate, &unit);
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" omits "Reviewed-by:"$"#)]
fn rationale_omits_reviewer(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.reviewed_by = None;
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" records "Reviewed-by: agent"$"#)]
fn rationale_self_review(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.reviewed_by = Some("agent".to_string());
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" records "Review verdict: reject"$"#)]
fn rationale_reject(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.review_verdict = Some("reject".to_string());
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" carries "Follow-up:" and "Expiry:"$"#)]
fn rationale_temporary(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.follow_up = Some("Split the renderer and the state machine".to_string());
    spec.expiry = Some("2026-12-01".to_string());
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a temporary rationale for "([^"]+)" omits "Follow-up:"$"#)]
fn rationale_missing_follow_up(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.expiry = Some("2026-12-01".to_string());
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(
    regex = r#"^a rationale for "([^"]+)" names the rejected alternative "([^"]+)" and cites "([^"]+)" in its evidence$"#
)]
fn rationale_named_alternative(
    world: &mut GivnWorld,
    unit: String,
    alternative: String,
    evidence: String,
) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.alternatives = format!(
        "{alternative} into a separate module. Rejected: the extraction adds indirection without removing a responsibility."
    );
    spec.evidence =
        format!("{evidence}, 8 public methods; every call site is in the declaration file.");
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" names the unit "([^"]+)"$"#)]
fn rationale_wrong_unit(world: &mut GivnWorld, unit: String, named: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.unit = named;
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" omits the owner$"#)]
fn rationale_without_owner(world: &mut GivnWorld, unit: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.owner = None;
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" records "([^"]+)" as its alternative$"#)]
fn rationale_boilerplate_alternative(world: &mut GivnWorld, unit: String, alternative: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.alternatives = alternative;
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" records "([^"]+)" as its owner$"#)]
fn rationale_unconfirmed_owner(world: &mut GivnWorld, unit: String, owner: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.owner = Some(owner);
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" records the classification "([^"]+)"$"#)]
fn rationale_wrong_classification(world: &mut GivnWorld, unit: String, classification: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.classification = classification;
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given(regex = r#"^a rationale for "([^"]+)" says "([^"]+)" as its reason$"#)]
fn rationale_banned_reason(world: &mut GivnWorld, unit: String, reason: String) {
    let mut spec = spec_for(&world.readability_gate, &unit);
    spec.why =
        format!("This is {reason} for the Engine unit, and the responsibility sits elsewhere.");
    gate(env(world)).write_rationale(&unit, &spec);
}

#[given("the change's verification receipt is current")]
fn current_verification_receipt(world: &mut GivnWorld) {
    let env = env(world);
    support::commit_review_record(env);
    support::run_check_tasks(env, support::CHANGE_ID);
    world.readability_gate.change_id = support::CHANGE_ID.to_string();
}

#[given(regex = r#"^the working tree has an uncommitted change in "([^"]+)"$"#)]
fn dirty_working_tree(world: &mut GivnWorld, path: String) {
    let env = env(world);
    let absolute = env.path(&path);
    let mut content = fs::read_to_string(&absolute).expect("read production file");
    content.push_str("// uncommitted\n");
    fs::write(absolute, content).expect("write uncommitted change");
}

// --- When ------------------------------------------------------------------

#[when("the readability review assesses the change")]
fn assess_change(world: &mut GivnWorld) {
    run_gate(world);
}

#[when("the readability review assesses the change twice")]
fn assess_change_twice(world: &mut GivnWorld) {
    run_gate(world);
    run_gate(world);
}

#[when("the reviewer reads the review guidance")]
fn read_review_guidance(world: &mut GivnWorld) {
    let env = env(world);
    let change = env.path(&format!("givn/changes/{}", support::CHANGE_ID));
    if !change.exists() {
        fs::create_dir_all(&change).expect("create the change directory");
        fs::write(change.join("proposal.md"), "# Proposal: test change\n").expect("write proposal");
    }
    let out = crate::common::run_givn(
        env,
        &["instructions", "review", "--change", support::CHANGE_ID],
    );
    assert!(
        out.status.success(),
        "givn instructions review failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_output = Some(out);
}

// --- Then ------------------------------------------------------------------

#[then("the review fails")]
fn review_fails(world: &mut GivnWorld) {
    let state = &world.readability_gate;
    assert_eq!(
        state.passed,
        Some(false),
        "expected the review to fail, got:\n{}",
        state.report
    );
}

#[then("the review passes")]
fn review_passes(world: &mut GivnWorld) {
    let state = &world.readability_gate;
    assert_eq!(
        state.passed,
        Some(true),
        "expected the review to pass, got:\n{}",
        state.report
    );
}

#[then(regex = r#"^the review says "([^"]+)"$"#)]
fn review_says(world: &mut GivnWorld, expected: String) {
    let report = gate_report(world);
    let expected = expected.replace("\\\"", "\"");
    assert!(
        collapse(report).contains(&collapse(&expected)),
        "the review does not say {expected:?}:\n{report}"
    );
}

#[then(regex = r#"^the review reports "([^"]+)"$"#)]
fn review_reports(world: &mut GivnWorld, expected: String) {
    let report = gate_report(world);
    let expected = collapse(&expected.replace("\\\"", "\""));
    let found = report.lines().any(|line| {
        let trimmed = line.trim();
        (trimmed.starts_with("FINDING")
            || trimmed.starts_with("DECISION")
            || trimmed.starts_with("FAILURE"))
            && collapse(trimmed).contains(&expected)
    });
    assert!(
        found,
        "no FINDING, DECISION, or FAILURE line reports {expected:?}:\n{report}"
    );
}

#[then(regex = r#"^the review reports an outline for "([^"]+)"$"#)]
fn review_reports_outline(world: &mut GivnWorld, unit: String) {
    let report = gate_report(world);
    assert!(
        report.contains(&format!("OUTLINE   {unit}")),
        "the review does not report an outline for {unit}:\n{report}"
    );
    assert!(
        report.contains("outline:"),
        "the outline block is missing:\n{report}"
    );
    assert!(
        report
            .lines()
            .any(|line| line.trim_start().starts_with("method [")),
        "the outline has no method entries:\n{report}"
    );
}

#[then(regex = r#"^the review reports the outline check "([^"]+)" for "([^"]+)"$"#)]
fn review_reports_outline_check(world: &mut GivnWorld, check: String, unit: String) {
    let report = gate_report(world);
    let start = report
        .find(&format!("OUTLINE   {unit}"))
        .unwrap_or_else(|| panic!("no outline for {unit}:\n{report}"));
    let rest = &report[start + 1..];
    let end = rest
        .find("OUTLINE   ")
        .map(|index| start + 1 + index)
        .unwrap_or(report.len());
    let block = &report[start..end];
    assert!(
        block.contains(&format!("CHECK     {check}")),
        "outline check for {unit} is not {check}:\n{block}"
    );
}

#[then(regex = r#"^the review reports a supporting metric finding for "([^"]+)"$"#)]
fn review_reports_supporting_finding(world: &mut GivnWorld, unit: String) {
    let report = gate_report(world);
    let found = report
        .lines()
        .any(|line| line.trim_start().starts_with("SUPPORTING") && line.contains(&unit));
    assert!(found, "no SUPPORTING line reports {unit}:\n{report}");
}

#[then(regex = r#"^the review reports no finding for "([^"]+)"$"#)]
fn review_reports_no_finding(world: &mut GivnWorld, unit: String) {
    let report = gate_report(world);
    let found = report.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("FINDING") && trimmed.contains(&unit)
    });
    assert!(!found, "unexpected FINDING for {unit}:\n{report}");
}

#[then(regex = r#"^the review reports the base size (\d+)$"#)]
fn review_reports_base_size(world: &mut GivnWorld, size: u32) {
    let report = gate_report(world);
    let patterns = [format!(" {size} -> "), format!("base {size} code lines")];
    let found = report.lines().any(|line| {
        line.trim().starts_with("FINDING") && patterns.iter().any(|pattern| line.contains(pattern))
    });
    assert!(
        found,
        "no FINDING line reports the base size {size}:\n{report}"
    );
}

#[then(regex = r#"^the review reports the head size (\d+)$"#)]
fn review_reports_head_size(world: &mut GivnWorld, size: u32) {
    let report = gate_report(world);
    let patterns = [
        format!("-> {size} code lines"),
        format!("new type: {size} code lines"),
        format!("new module: {size} code lines"),
    ];
    let found = report.lines().any(|line| {
        line.trim().starts_with("FINDING") && patterns.iter().any(|pattern| line.contains(pattern))
    });
    assert!(
        found,
        "no FINDING line reports the head size {size}:\n{report}"
    );
}

#[then("the review uses the ancestor commit as the first touch")]
fn review_uses_ancestor(world: &mut GivnWorld) {
    let report = gate_report(world);
    let state = &world.readability_gate;
    let ancestor = state
        .ancestor
        .as_ref()
        .expect("the fixture records the ancestor commit");
    let first_touch = state
        .first_touch
        .as_ref()
        .expect("the fixture records the first touch");
    assert!(
        report.contains(&format!("base {ancestor}")),
        "the base is not the ancestor commit {ancestor}:\n{report}"
    );
    assert!(
        report.contains(&format!("parent of {first_touch}")),
        "the first touch is not the ancestor-line change commit {first_touch}:\n{report}"
    );
}

#[then("the review ignores the sibling timeline")]
fn review_ignores_sibling(world: &mut GivnWorld) {
    let report = gate_report(world);
    for forbidden in &world.readability_gate.forbidden_hashes {
        assert!(
            !report.contains(forbidden),
            "the review used the sibling check-in {forbidden}:\n{report}"
        );
    }
}

#[then("the review writes no readability baseline file")]
fn no_baseline(world: &mut GivnWorld) {
    let state = &world.readability_gate;
    assert!(
        state.changed_files.is_empty(),
        "the gate wrote project files: {:?}",
        state.changed_files
    );
    let root = report_root(world);
    let baselines: Vec<String> = support::snapshot(&root)
        .into_iter()
        .map(|(path, _)| path)
        .filter(|path| {
            let lower = path.to_lowercase();
            lower.contains("baseline")
                || lower.ends_with("readability.json")
                || lower.ends_with("readability-baseline")
        })
        .collect();
    assert!(
        baselines.is_empty(),
        "unexpected readability baseline files: {baselines:?}"
    );
    assert!(
        !Path::new(&root).join("givn/readability.json").exists(),
        "unexpected givn/readability.json"
    );
}

#[then("both results are identical")]
fn both_results_identical(world: &mut GivnWorld) {
    let reports = &world.readability_gate.reports;
    assert_eq!(reports.len(), 2, "expected two gate runs");
    assert_eq!(
        reports[0], reports[1],
        "the two gate runs produced different reports"
    );
}

#[then(regex = r#"^the guidance names "([^"]+)"$"#)]
fn guidance_names(world: &mut GivnWorld, expected: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        collapse(&stdout).contains(&collapse(&expected)),
        "the review guidance does not name {expected:?}:\n{stdout}"
    );
}

#[then("the guidance says a justification cannot use ownership or scope as a reason")]
fn guidance_banned_reason(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(
        collapse(&stdout).contains("A justification cannot use ownership or scope as a reason"),
        "the review guidance omits the ownership and scope boundary:\n{stdout}"
    );
}

#[then(regex = r#"^the guidance says to run "([^"]+)"$"#)]
fn guidance_run_command(world: &mut GivnWorld, expected: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        collapse(&stdout).contains(&collapse(&expected)),
        "the review guidance does not say to run {expected:?}:\n{stdout}"
    );
}
