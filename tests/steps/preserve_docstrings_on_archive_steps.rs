use std::io::Write;

use crate::common;
use crate::steps::GivnWorld;
use cucumber::gherkin::Step;
use cucumber::{given, then};

/// Write the fixture `givn/commands.yaml` (verify `true`, coverage commands
/// `true`, dummy Cobertura XMLs) so every archive gate passes in the temp
/// project, which has no Rust runner.
fn write_command_overrides(env: &common::TestEnv) {
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
";
    std::fs::write(&commands_path, yaml).expect("write patched commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");
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
            writeln!(f, "REVIEW: PASS").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] all done").ok();
        }
    }
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    write_command_overrides(env);
    let specs_dir = change_dir.join("specs");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    let cap_file = specs_dir.join(format!("delta-{id}.feature"));
    std::fs::write(&cap_file, delta_content).expect("write delta feature");
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
