use std::io::Write;
use std::path::{Path, PathBuf};

use crate::common;
use crate::steps::GivnWorld;
use cucumber::gherkin::Step;
use cucumber::{given, then, when};

/// Write the fixture `givn/commands.yaml` (verify `true`, coverage commands
/// `true`, dummy Cobertura XMLs) so every archive gate passes in the temp
/// project, which has no Rust runner.
fn write_command_overrides(env: &common::TestEnv) {
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    crate::steps::archive_proof_support::configure_commands(env);
    crate::steps::archive_proof_support::disable_coverage(env);
}

/// Create a change whose every archive-gate artifact is complete and whose
/// delta spec is written by `delta_content`.
fn complete_change_with_delta(
    env: &common::TestEnv,
    id: &str,
    delta_content: &str,
) -> std::path::PathBuf {
    let change_dir = env.path(&format!("givn/changes/{id}"));
    if !change_dir.exists() {
        let out = common::run_givn(env, &["new", id]);
        assert!(
            out.status.success(),
            "givn new {id} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
    for file in &[
        "proposal.md",
        "design.md",
        "design-review.md",
        "tasks.md",
        "review.md",
    ] {
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        if *file == "design-review.md" {
            writeln!(f, "DESIGN-REVIEW: PASS").ok();
        } else if *file == "review.md" {
            writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] COMMIT: a1b2c3d4 -- all done").ok();
        }
    }
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    write_command_overrides(env);
    let specs_dir = change_dir.join("specs");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    let capability = delta_content
        .split_whitespace()
        .find_map(|token| {
            token
                .strip_prefix('@')
                .filter(|tag| !tag.starts_with("givn."))
        })
        .unwrap_or("delta")
        .to_string();
    let owner = givn::spec::owner::resolve_capability_owner(&env.path("givn/specs"), &capability)
        .expect("resolve fixture capability owner");
    let delta_dir = match owner {
        Some(owner) => {
            let dir = specs_dir.join(owner.file_name().expect("owner directory name"));
            std::fs::create_dir_all(&dir).expect("create owned delta dir");
            dir
        }
        None => specs_dir.clone(),
    };
    std::fs::write(delta_dir.join(format!("delta-{id}.feature")), delta_content)
        .expect("write delta feature");
    common::declare_fixture_routing(env, id);
    change_dir
}

/// Wrap a re-indented docstring content (from a cucumber Step) into a valid
/// Gherkin docstring block at 6-space indentation.
fn docstring_block(content: &str) -> String {
    let mut out = String::from("      \"\"\"\n");
    for line in content.lines() {
        if line.is_empty() {
            out.push('\n');
        } else {
            out.push_str(&format!("      {line}\n"));
        }
    }
    out.push_str("      \"\"\"\n");
    out
}

// --- Given ---

#[given(
    regex = r#"^a permanent spec for capability "([^"]+)" contains a scenario named "([^"]+)"$"#
)]
fn permanent_spec_with_scenario(world: &mut GivnWorld, cap: String, title: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let dir = env.path(&format!("givn/specs/{cap}"));
    std::fs::create_dir_all(&dir).expect("create specs dir");
    std::fs::write(
        dir.join(format!("{cap}.feature")),
        format!("Feature: {cap}\n\n  Scenario: {title}\n    Given  a baseline step exists\n"),
    )
    .expect("write permanent spec");
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose delta modifies scenario "([^"]+)" within capability "([^"]+)" and gives a step the docstring:$"#
)]
fn completed_change_modifies_with_docstring(
    world: &mut GivnWorld,
    id: String,
    title: String,
    cap: String,
    step: &Step,
) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let doc = step.docstring.as_ref().expect("docstring required");
    let delta = format!(
        "@givn.delta @{cap}\nFeature: Delta\n\n  @givn.modified\n  Scenario: {title}\n    Given  a baseline step exists\n    And  a step uses the payload:\n{docstring}",
        docstring = docstring_block(doc),
    );
    complete_change_with_delta(env, &id, &delta);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose delta adds scenario "([^"]+)" within capability "([^"]+)" and gives a step the docstring:$"#
)]
fn completed_change_adds_with_docstring(
    world: &mut GivnWorld,
    id: String,
    title: String,
    cap: String,
    step: &Step,
) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let doc = step.docstring.as_ref().expect("docstring required");
    let delta = format!(
        "@givn.delta @{cap}\nFeature: Delta\n\n  @givn.added\n  Scenario: {title}\n    Given  a baseline step exists\n    And  a step uses the payload:\n{docstring}",
        docstring = docstring_block(doc),
    );
    complete_change_with_delta(env, &id, &delta);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose delta modifies scenario "([^"]+)" within capability "([^"]+)" and gives a step a data table containing the text "([^"]+)"$"#
)]
fn completed_change_modifies_with_table(
    world: &mut GivnWorld,
    id: String,
    title: String,
    cap: String,
    cell: String,
) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let delta = format!(
        "@givn.delta @{cap}\nFeature: Delta\n\n  @givn.modified\n  Scenario: {title}\n    Given  a baseline step exists\n    And  a step uses the payload:\n      | {cell} |\n",
    );
    complete_change_with_delta(env, &id, &delta);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a givn project with a completed change "([^"]+)" whose delta modifies scenario "([^"]+)" within capability "([^"]+)" and gives a step the docstring:$"#
)]
fn e2e_project_with_completed_change(
    world: &mut GivnWorld,
    id: String,
    title: String,
    cap: String,
    step: &Step,
) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().expect("env set above");
    let dir = env.path(&format!("givn/specs/{cap}"));
    std::fs::create_dir_all(&dir).expect("create specs dir");
    std::fs::write(
        dir.join(format!("{cap}.feature")),
        format!("Feature: {cap}\n\n  Scenario: {title}\n    Given  a baseline step exists\n"),
    )
    .expect("write permanent spec");
    let doc = step.docstring.as_ref().expect("docstring required");
    let delta = format!(
        "@givn.delta @{cap}\nFeature: Delta\n\n  @givn.modified\n  Scenario: {title}\n    Given  a baseline step exists\n    And  a step uses the payload:\n{docstring}",
        docstring = docstring_block(doc),
    );
    complete_change_with_delta(env, &id, &delta);
    world.last_change_id = Some(id);
}

// --- Then ---

#[then(regex = r#"^the permanent spec file for "([^"]+)" contains the text "([^"]+)"$"#)]
fn permanent_spec_contains(world: &mut GivnWorld, cap: String, needle: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path(&format!("givn/specs/{cap}/{cap}.feature"));
    let content = std::fs::read_to_string(&path).expect("read permanent spec file");
    assert!(
        content.contains(&needle),
        "permanent spec {:?} missing {:?}:\n{}",
        path,
        needle,
        content,
    );
}

fn routed_proposal(cap: &str, decision: &str) -> String {
    format!(
        "# Proposal: fixture\n\n## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | {cap} | (fixture) | {decision} | fixture |\n"
    )
}

fn permanent_path(env: &common::TestEnv, usecase: &str, cap: &str) -> PathBuf {
    env.path(&format!("givn/specs/{usecase}/{cap}.feature"))
}

fn write_archive_artifacts(
    env: &common::TestEnv,
    id: &str,
    proposal: &str,
    usecase: &str,
    cap: &str,
    permanent: Option<&str>,
    delta: &str,
) {
    let out = common::run_givn(env, &["new", id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let change_dir = env.path(&format!("givn/changes/{id}"));
    std::fs::write(change_dir.join("proposal.md"), proposal).expect("write proposal");
    for file in ["design.md", "design-review.md", "tasks.md", "review.md"] {
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        match file {
            "design-review.md" => writeln!(f, "DESIGN-REVIEW: PASS").ok(),
            "review.md" => writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok(),
            "tasks.md" => writeln!(f, "- [x] COMMIT: a1b2c3d4 -- fixture").ok(),
            _ => writeln!(f, "# Design").ok(),
        };
    }
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").expect("arc42");
    if let Some(content) = permanent {
        let path = permanent_path(env, usecase, cap);
        std::fs::create_dir_all(path.parent().unwrap()).expect("create use-case dir");
        std::fs::write(&path, content).expect("write permanent feature");
    }
    let delta_dir = change_dir.join("specs").join(usecase);
    std::fs::create_dir_all(&delta_dir).expect("create delta specs dir");
    std::fs::write(delta_dir.join(format!("{cap}.feature")), delta).expect("write delta");
    if permanent.is_none() {
        let document = givn::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", usecase)
            .replace("<capability-id>", cap)
            .replace("<actor role>", "Fixture operator")
            .replace("<the domain goal>", "Exercise the fixture behavior.")
            .replace("<consumer action>", "exercise the capability")
            .replace("<scenario title>", "fixture scenario");
        std::fs::write(delta_dir.join("usecase.md"), document).expect("write change usecase");
    }
    write_command_overrides(env);
}

fn finish_archive_fixture(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, id);
    world.last_change_id = Some(id.to_string());
}

fn parsed_permanent(world: &GivnWorld, usecase: &str, cap: &str) -> gherkin::Feature {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = permanent_path(env, usecase, cap);
    let text = std::fs::read_to_string(&path).expect("read permanent feature");
    givn::gherkin::parse::parse_supported_source(&text, &path).expect("parse permanent feature")
}

fn parse_reference(source: &str) -> gherkin::Feature {
    gherkin::Feature::parse(source, gherkin::GherkinEnv::default())
        .expect("parse reference feature")
}

fn snapshot_tree(path: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    fn walk(root: &Path, path: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let Ok(entries) = std::fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let relative = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();
                out.push((relative, std::fs::read(&path).unwrap_or_default()));
            }
        }
    }
    if path.exists() {
        walk(path, path, &mut out);
    }
    out.sort();
    out
}

fn archive_mutation_snapshot(world: &GivnWorld, id: &str) -> Vec<(String, Vec<u8>)> {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let mut out = snapshot_tree(&env.path("givn/specs"));
    out.extend(snapshot_tree(&env.path(&format!("givn/changes/{id}"))));
    out.extend(snapshot_tree(&env.path(&format!("givn/archive/{id}"))));
    out.sort();
    out
}

fn assert_no_mutation(world: &GivnWorld, id: &str) {
    let before: Vec<(String, Vec<u8>)> = serde_json::from_str(
        world
            .verification_log_before
            .as_deref()
            .expect("the fixture must snapshot the tree before archive"),
    )
    .expect("parse snapshot");
    assert_eq!(
        before,
        archive_mutation_snapshot(world, id),
        "a refused archive mutated permanent state"
    );
}

const PAYLOAD_PERMANENT: &str = "Feature: Payload capability\n\n  Scenario: Existing behaviour\n    Given a baseline step\n\n  Scenario: Unaffected behaviour\n    Then unaffected\n";

#[given("a completed change modifies an existing scenario in a supported Gherkin feature")]
fn modifies_existing_with_typed_docstring(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @payload-cap\nFeature: Payload capability\n\n  @givn.modified\n  Scenario: Existing behaviour\n    Given a baseline step\n    And a typed payload:\n      \"\"\"json\n      {\"purpose\": \"docstring-survives-modified-merge\"}\n      Scenario: payload heading\n      Background: payload heading\n      \"\"\"\n";
    write_archive_artifacts(
        env,
        "docstring-modified",
        &routed_proposal("payload-cap", "EXTEND payload-cap"),
        "payload",
        "payload-cap",
        Some(PAYLOAD_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "docstring-modified");
}

#[given(
    "the modified step has a typed multiline docstring containing literal lines \"Scenario: payload heading\" and \"Background: payload heading\""
)]
fn typed_docstring_has_headings(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/docstring-modified/specs/payload/payload-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    assert!(content.contains("Scenario: payload heading"));
    assert!(content.contains("Background: payload heading"));
}

#[then(
    "the permanent scenario retains the complete docstring payload, media type and step attachment"
)]
fn typed_docstring_survives(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "payload", "payload-cap");
    let scenario = feature
        .scenarios
        .iter()
        .find(|scenario| scenario.name == "Existing behaviour")
        .expect("modified scenario");
    let expected = "json\n      {\"purpose\": \"docstring-survives-modified-merge\"}\n      Scenario: payload heading\n      Background: payload heading";
    let docstring = scenario.steps[1]
        .docstring()
        .expect("the payload step keeps its docstring");
    assert_eq!(docstring.trim_end(), expected);
}

#[then("the payload headings do not become scenarios or Backgrounds")]
fn payload_headings_are_not_structure(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "payload", "payload-cap");
    assert_eq!(feature.scenarios.len(), 2);
    assert_eq!(feature.background.as_ref().map(|b| b.steps.len()), None);
}

#[given("a completed change adds a scenario to an existing supported Gherkin feature")]
fn adds_scenario_with_literal_payload(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @payload-cap\nFeature: Payload capability\n\n  @givn.added\n  Scenario: New behaviour\n    Given a payload:\n      \"\"\"\n      @literal\n      ```\n      Scenario Outline: payload heading\n      \"\"\"\n";
    write_archive_artifacts(
        env,
        "docstring-added",
        &routed_proposal("payload-cap", "EXTEND payload-cap"),
        "payload",
        "payload-cap",
        Some(PAYLOAD_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "docstring-added");
}

#[given(
    "the added step has a multiline docstring containing literal tags, triple backticks and a \"Scenario Outline:\" line"
)]
fn added_docstring_has_literals(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/docstring-added/specs/payload/payload-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    for needle in ["@literal", "```", "Scenario Outline: payload heading"] {
        assert!(
            content.contains(needle),
            "delta lacks {needle:?}: {content}"
        );
    }
}

#[then("the added permanent scenario retains the complete docstring and its step attachment")]
fn added_docstring_survives(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "payload", "payload-cap");
    let scenario = feature
        .scenarios
        .iter()
        .find(|scenario| scenario.name == "New behaviour")
        .expect("added scenario");
    let docstring = scenario.steps[0]
        .docstring()
        .expect("the payload step keeps its docstring");
    for needle in ["@literal", "```", "Scenario Outline: payload heading"] {
        assert!(
            docstring.contains(needle),
            "docstring missing {needle:?}: {docstring}"
        );
    }
}

#[then("payload text is not interpreted as feature structure")]
fn payload_text_not_structure(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "payload", "payload-cap");
    assert_eq!(feature.scenarios.len(), 3);
    assert!(feature.background.is_none());
    assert!(feature.rules.is_empty());
}

#[given("a completed change modifies an existing supported scenario with a step data table")]
fn modifies_existing_with_table(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @payload-cap\nFeature: Payload capability\n\n  @givn.modified\n  Scenario: Existing behaviour\n    Given a baseline step\n    And a mapping:\n      | name | value |\n      | first | a\\|b |\n      | second |  |\n      | third | c\\\\d |\n";
    write_archive_artifacts(
        env,
        "table-modified",
        &routed_proposal("payload-cap", "EXTEND payload-cap"),
        "payload",
        "payload-cap",
        Some(PAYLOAD_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "table-modified");
}

#[given("the table contains ordered rows, empty cells and escaped pipe characters")]
fn table_has_ordered_cells(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/table-modified/specs/payload/payload-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    assert!(content.contains("| first | a\\|b |"));
    assert!(content.contains("| second |  |"));
    assert!(content.contains("| third | c\\\\d |"));
}

#[then(
    "the permanent scenario retains every table cell and row in order attached to the same step"
)]
fn table_survives(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "payload", "payload-cap");
    let scenario = feature
        .scenarios
        .iter()
        .find(|scenario| scenario.name == "Existing behaviour")
        .expect("modified scenario");
    let table = scenario.steps[1]
        .table()
        .expect("the payload step keeps its table");
    let reference = parse_reference(
        "Feature: reference\n  Scenario: reference\n    Given a mapping:\n      | name | value |\n      | first | a\\|b |\n      | second |  |\n      | third | c\\\\d |\n",
    );
    let expected = reference.scenarios[0].steps[0]
        .table()
        .expect("reference table");
    assert_eq!(table.rows, expected.rows);
}

const BACKGROUND_PERMANENT: &str = "Feature: Background capability\n\n  Background: Shared setup\n    Given a session\n      \"\"\"json\n      {\"ok\": true}\n      \"\"\"\n    And mappings\n      | key | value |\n      | a | 1 |\n\n  Scenario: Existing behaviour\n    Then existing\n\n  Scenario: Unaffected behaviour\n    Then unaffected\n";

#[given("a permanent supported feature has Background steps with a docstring and data table")]
fn permanent_background_with_arguments(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @background-cap\nFeature: Background capability\n\n  @givn.modified\n  Scenario: Existing behaviour\n    Then updated\n";
    write_archive_artifacts(
        env,
        "background-modified",
        &routed_proposal("background-cap", "EXTEND background-cap"),
        "background",
        "background-cap",
        Some(BACKGROUND_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "background-modified");
}

#[given("a completed change modifies one scenario without changing that Background")]
fn change_leaves_background_alone(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/background-modified/specs/background/background-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    assert!(!content.contains("Background:"));
}

#[then("the permanent feature retains every Background step, argument and order")]
fn background_arguments_survive(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "background", "background-cap");
    let background = feature.background.as_ref().expect("background kept");
    let reference = parse_reference(BACKGROUND_PERMANENT);
    let expected = reference.background.as_ref().expect("reference background");
    assert_eq!(background.steps.len(), expected.steps.len());
    for (actual, expected) in background.steps.iter().zip(&expected.steps) {
        assert_eq!(actual.keyword, expected.keyword);
        assert_eq!(actual.value, expected.value);
        assert_eq!(actual.docstring, expected.docstring);
        assert_eq!(
            actual.table.as_ref().map(|t| &t.rows),
            expected.table.as_ref().map(|t| &t.rows)
        );
    }
}

#[then("the modified and unaffected scenarios retain the same Background setup")]
fn both_scenarios_share_background(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "background", "background-cap");
    assert_eq!(feature.scenarios.len(), 2);
    assert!(
        feature
            .scenarios
            .iter()
            .any(|scenario| scenario.name == "Existing behaviour")
    );
    assert!(
        feature
            .scenarios
            .iter()
            .any(|scenario| scenario.name == "Unaffected behaviour")
    );
}

const TAGS_PERMANENT: &str = "@e2e @regression\nFeature: Tag capability\n\n  @smoke\n  Scenario: Existing behaviour\n    Then existing\n";

#[given(
    "a completed change modifies a scenario in a feature with supported inherited execution tags"
)]
fn modifies_tagged_scenario(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @tag-cap @e2e @regression\nFeature: Tag capability\n\n  @givn.modified @fast\n  Scenario: Existing behaviour\n    Then updated\n";
    write_archive_artifacts(
        env,
        "tags-modified",
        &routed_proposal("tag-cap", "EXTEND tag-cap"),
        "tags",
        "tag-cap",
        Some(TAGS_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "tags-modified");
}

#[given("the scenario also has its own execution tags")]
fn scenario_has_own_tags(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/tags-modified/specs/tags/tag-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    assert!(content.contains("@givn.modified @fast"));
}

#[then("the permanent scenario retains the inherited and scenario-specific execution tags")]
fn inherited_and_specific_tags_survive(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "tags", "tag-cap");
    let scenario = feature
        .scenarios
        .iter()
        .find(|scenario| scenario.name == "Existing behaviour")
        .expect("modified scenario");
    for tag in ["e2e", "regression", "fast"] {
        assert!(
            scenario.tags.iter().any(|value| value == tag),
            "scenario lacks @{tag}: {:?}",
            scenario.tags
        );
    }
    assert!(feature.tags.iter().any(|value| value == "e2e"));
    assert!(feature.tags.iter().any(|value| value == "regression"));
}

#[then("delta-operation tags are stripped without dropping execution tags")]
fn delta_tags_stripped(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "tags", "tag-cap");
    for tag in &feature.tags {
        assert!(!tag.starts_with("givn."), "feature kept @{tag}");
    }
    for scenario in &feature.scenarios {
        for tag in &scenario.tags {
            assert!(!tag.starts_with("givn."), "scenario kept @{tag}");
            assert_ne!(tag, "wip");
        }
    }
}

const OUTLINE_PERMANENT: &str = "Feature: Outline capability\n\n  Scenario Outline: Existing outline\n    Given value <name>\n    Then seen\n\n    Examples:\n      | name |\n      | old |\n";

#[given(
    "a completed change modifies a supported Scenario Outline with placeholder steps and step arguments"
)]
fn modifies_outline(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @outline-cap\nFeature: Outline capability\n\n  @givn.modified\n  Scenario Outline: Existing outline\n    Given payload <name>\n      \"\"\"json\n      {\"name\": \"<name>\"}\n      \"\"\"\n    And mapping\n      | key | value |\n      | name | <name> |\n    Then seen\n\n    @api\n    Examples: First\n      | name |\n      | alpha |\n      | beta |\n\n    @slow\n    Examples: Second\n      | name |\n      | gamma |\n";
    write_archive_artifacts(
        env,
        "outline-modified",
        &routed_proposal("outline-cap", "EXTEND outline-cap"),
        "outline",
        "outline-cap",
        Some(OUTLINE_PERMANENT),
        delta,
    );
    finish_archive_fixture(world, "outline-modified");
}

#[given("the outline has multiple tagged Examples blocks with ordered headers and rows")]
fn outline_has_tagged_examples(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/changes/outline-modified/specs/outline/outline-cap.feature");
    let content = std::fs::read_to_string(&path).expect("read delta");
    assert!(content.contains("@api"));
    assert!(content.contains("@slow"));
    assert!(content.matches("Examples:").count() == 2);
}

#[then("the permanent outline retains every placeholder step, docstring and table")]
fn outline_steps_survive(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "outline", "outline-cap");
    let scenario = &feature.scenarios[0];
    assert_eq!(scenario.keyword, "Scenario Outline");
    assert_eq!(scenario.steps.len(), 3);
    assert!(scenario.steps[0].value.contains("<name>"));
    assert!(scenario.steps[0].docstring().is_some());
    assert!(scenario.steps[1].table().is_some());
}

#[then("every Examples block retains its execution tags, headers, values and row order")]
fn outline_examples_survive(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "outline", "outline-cap");
    let examples = &feature.scenarios[0].examples;
    assert_eq!(examples.len(), 2);
    assert_eq!(examples[0].tags, vec!["api".to_string()]);
    assert_eq!(examples[1].tags, vec!["slow".to_string()]);
    assert_eq!(
        examples[0].table.as_ref().expect("first table").rows,
        vec![
            vec!["name".to_string()],
            vec!["alpha".to_string()],
            vec!["beta".to_string()]
        ]
    );
    assert_eq!(
        examples[1].table.as_ref().expect("second table").rows,
        vec![vec!["name".to_string()], vec!["gamma".to_string()]]
    );
}

#[given("the supported Gherkin dialect does not support Rule")]
fn dialect_has_no_rule(_world: &mut GivnWorld) {}

#[given("a completed change contains a Rule with nested Background and scenarios")]
fn change_contains_rule(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "@givn.delta @rule-cap\nFeature: Rule capability\n\n  Rule: Restricted\n    Background:\n      Given nested setup\n\n    Scenario: Nested\n      Then visible\n";
    write_archive_artifacts(
        env,
        "rule-change",
        &routed_proposal("rule-cap", "NEW in rule-group"),
        "rule-group",
        "rule-cap",
        None,
        delta,
    );
    finish_archive_fixture(world, "rule-change");
}

#[when("the developer archives the change")]
fn developer_archives_the_change(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    world.verification_log_before =
        Some(serde_json::to_string(&archive_mutation_snapshot(world, &id)).expect("snapshot JSON"));
    let out = common::run_givn(env, &["archive", "--change", &id]);
    world.last_output = Some(out);
}

#[then("the command exits non-zero and names the unsupported Rule and feature path")]
fn names_rule_and_path(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    assert!(stderr.contains("unsupported Rule"), "stderr: {stderr}");
    assert!(
        stderr.contains(".feature") && stderr.contains("specs/"),
        "stderr lacks feature path: {stderr}"
    );
}

#[then("no active change, archive receipt or permanent specification is mutated")]
fn no_archive_mutation(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_no_mutation(world, &id);
}

const CONFLICT_PERMANENT: &str = "Feature: Background capability\n\n  Background: Shared setup\n    Given original session\n\n  Scenario: Existing behaviour\n    Then existing\n\n  Scenario: Unaffected behaviour\n    Then unaffected\n";

#[given(
    "a permanent English feature has Background setup shared by a modified and an unaffected scenario"
)]
fn permanent_conflict_background(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let path = permanent_path(env, "conflict", "conflict-cap");
    std::fs::create_dir_all(path.parent().unwrap()).expect("create use-case dir");
    std::fs::write(&path, CONFLICT_PERMANENT).expect("write permanent feature");
}

#[given(
    "a completed change modifies the first scenario while supplying different Background setup"
)]
fn delta_supplies_conflicting_background(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let delta = "@givn.delta @conflict-cap\nFeature: Background capability\n\n  Background: Shared setup\n    Given different session\n\n  @givn.modified\n  Scenario: Existing behaviour\n    Then updated\n";
    write_archive_artifacts(
        env,
        "conflict-change",
        &routed_proposal("conflict-cap", "EXTEND conflict-cap"),
        "conflict",
        "conflict-cap",
        None,
        delta,
    );
    finish_archive_fixture(world, "conflict-change");
}

#[then("the command exits non-zero and names the conflicting Background and feature path")]
fn names_background_conflict_and_path(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    assert!(
        stderr.contains("conflicting Background"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("conflict-change/specs") || stderr.contains("conflict-cap.feature"),
        "stderr lacks feature path: {stderr}"
    );
}

#[then("both permanent scenarios retain their original Background setup")]
fn conflict_background_unchanged(world: &mut GivnWorld) {
    let feature = parsed_permanent(world, "conflict", "conflict-cap");
    let reference = parse_reference(CONFLICT_PERMANENT);
    assert_eq!(
        givn::gherkin::content::background_content(feature.background.as_ref()),
        givn::gherkin::content::background_content(reference.background.as_ref())
    );
    assert_eq!(feature.scenarios.len(), 2);
}

#[given("a completed change contains a valid French feature declaring \"# language: fr\"")]
fn change_contains_french_feature(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let delta = "# language: fr\n@givn.delta @fr-cap\nFonctionnalité: Catalogue\n\n  @givn.added\n  Scénario: Lire\n    Alors visible\n";
    write_archive_artifacts(
        env,
        "french-change",
        &routed_proposal("fr-cap", "NEW in fr-group"),
        "fr-group",
        "fr-cap",
        None,
        delta,
    );
    finish_archive_fixture(world, "french-change");
}

#[given("0.9.10 supports English Gherkin only")]
fn english_only(_world: &mut GivnWorld) {}

#[then("the command exits non-zero and names the unsupported dialect \"fr\" and feature path")]
fn names_french_dialect_and_path(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    assert!(
        stderr.contains("unsupported dialect 'fr'"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("french-change/specs") || stderr.contains("fr-cap.feature"),
        "stderr lacks feature path: {stderr}"
    );
}
