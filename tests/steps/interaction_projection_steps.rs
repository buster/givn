//! Fixtures for the projected interaction-inventory lint scenarios.

use cucumber::given;
use std::path::{Path, PathBuf};

use super::GivnWorld;
use crate::common;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn ensure_project(world: &mut GivnWorld) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
}

fn fragment_document(titles: &[String]) -> String {
    let rows = titles
        .iter()
        .map(|title| format!("| place-order | submit an order | {title} |"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Fragment: support\n\n## Capabilities\n\n- place-order\n\n## Interactions\n\n\
         | Capability | Consumer action | E2E scenario |\n|---|---|---|\n{rows}\n\n\
         ## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
    )
}

fn feature_text(titles: &[String]) -> String {
    let mut text = String::from("Feature: Place order\n");
    for title in titles {
        text.push_str(&format!(
            "\n  @e2e\n  Scenario: {title}\n    Given x\n    Then y\n"
        ));
    }
    text
}

fn write_permanent_fragment(world: &mut GivnWorld, title: &str) {
    ensure_project(world);
    let env = env(world);
    let dir = env.path("givn/specs/fragments");
    std::fs::create_dir_all(&dir).expect("create permanent fragment dir");
    std::fs::write(
        dir.join("fragment.md"),
        fragment_document(&[title.to_string()]),
    )
    .expect("write permanent fragment");
    std::fs::write(
        dir.join("place-order.feature"),
        feature_text(&[title.to_string()]),
    )
    .expect("write permanent feature");
}

fn change_specs(env: &common::TestEnv, id: &str) -> PathBuf {
    env.path(&format!("givn/changes/{id}/specs"))
}

fn write_delta(env: &common::TestEnv, id: &str, usecase: &str, capability: &str, body: &str) {
    let dir = change_specs(env, id).join(usecase);
    std::fs::create_dir_all(&dir).expect("create delta specs dir");
    std::fs::write(
        dir.join(format!("{capability}.feature")),
        format!("@givn.delta @{capability}\nFeature: {capability}\n\n{body}"),
    )
    .expect("write delta feature");
}

fn e2e_titles(path: &Path) -> Vec<String> {
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

fn write_added_e2e(world: &mut GivnWorld, id: &str, title: &str, with_row: bool) {
    let env = env(world);
    let body = format!("  @givn.added @e2e\n  Scenario: {title}\n    Given x\n    Then y\n");
    write_delta(env, id, "fragments", "place-order", &body);
    if with_row {
        let mut titles = e2e_titles(&env.path("givn/specs/fragments/place-order.feature"));
        titles.extend(e2e_titles(
            &change_specs(env, id)
                .join("fragments")
                .join("place-order.feature"),
        ));
        titles.sort();
        titles.dedup();
        let dir = change_specs(env, id).join("fragments");
        std::fs::write(dir.join("fragment.md"), fragment_document(&titles))
            .expect("write delta fragment");
    }
    world.last_change_id = Some(id.to_string());
}

#[given(
    regex = r#"^a project with a permanent fragment whose interaction inventory declares "([^"]+)"$"#
)]
fn permanent_fragment_with_interaction(world: &mut GivnWorld, title: String) {
    write_permanent_fragment(world, &title);
}

#[given(
    regex = r#"^a change "([^"]+)" whose delta adds an E2E scenario "([^"]+)" without an interaction row$"#
)]
fn change_adds_e2e_without_row(world: &mut GivnWorld, id: String, title: String) {
    ensure_project(world);
    write_added_e2e(world, &id, &title, false);
}

#[given(
    regex = r#"^a change "([^"]+)" whose delta adds an E2E scenario "([^"]+)" with its interaction row$"#
)]
fn change_adds_e2e_with_row(world: &mut GivnWorld, id: String, title: String) {
    ensure_project(world);
    write_added_e2e(world, &id, &title, true);
}

#[given(regex = r#"^a change "([^"]+)" whose delta removes the only E2E scenario for "([^"]+)"$"#)]
fn change_removes_only_e2e(world: &mut GivnWorld, id: String, title: String) {
    ensure_project(world);
    let env = env(world);
    let body = format!("  @givn.removed @e2e\n  Scenario: {title}\n    Given placeholder\n");
    write_delta(env, &id, "fragments", "place-order", &body);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" whose delta adds an E2E scenario "([^"]+)" under an undeclared capability "([^"]+)"$"#
)]
fn change_adds_undeclared_capability(
    world: &mut GivnWorld,
    id: String,
    title: String,
    capability: String,
) {
    ensure_project(world);
    let env = env(world);
    let body = format!("  @givn.added @e2e\n  Scenario: {title}\n    Given x\n    Then y\n");
    write_delta(env, &id, "fragments", &capability, &body);
    let change = env.path(&format!("givn/changes/{id}"));
    std::fs::write(
        change.join("proposal.md"),
        format!(
            "# Proposal: {id}\n\n## Capability Routing\n\n\
             | Proposed capability | Route's recommendation | Decision | Rationale |\n\
             |---|---|---|---|\n\
             | {capability} | NEW in fragments | `NEW in fragments` | fixture |\n"
        ),
    )
    .expect("write routing proposal");
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^the permanent corpus contains an E2E scenario "([^"]+)" in an undeclared capability "([^"]+)"$"#
)]
fn permanent_undeclared_capability(world: &mut GivnWorld, title: String, capability: String) {
    let env = env(world);
    let dir = env.path("givn/specs/fragments");
    std::fs::create_dir_all(&dir).expect("create permanent fragment dir");
    std::fs::write(
        dir.join(format!("{capability}.feature")),
        format!("Feature: {capability}\n\n  @e2e\n  Scenario: {title}\n    Given x\n    Then y\n"),
    )
    .expect("write undeclared permanent feature");
}

#[given(regex = r#"^the permanent corpus contains an unlisted E2E scenario "([^"]+)"$"#)]
fn permanent_unlisted_scenario(world: &mut GivnWorld, title: String) {
    let env = env(world);
    let path = env.path("givn/specs/fragments/place-order.feature");
    let mut content = std::fs::read_to_string(&path).expect("read permanent feature");
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(&format!(
        "\n  @e2e\n  Scenario: {title}\n    Given x\n    Then y\n"
    ));
    std::fs::write(&path, content).expect("append permanent scenario");
}

#[given(regex = r#"^a change "([^"]+)" with no delta features$"#)]
fn change_without_features(world: &mut GivnWorld, id: String) {
    ensure_project(world);
    let env = env(world);
    std::fs::create_dir_all(env.path(&format!("givn/changes/{id}")))
        .expect("create empty change dir");
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a raw delta feature at "([^"]+)" with:$"#)]
fn raw_delta_feature(world: &mut GivnWorld, path: String, step: &cucumber::gherkin::Step) {
    let content = step
        .docstring
        .as_ref()
        .expect("docstring content for a raw delta feature");
    let env = env(world);
    let file_path = env.path(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("create raw delta parent dirs");
    }
    std::fs::write(&file_path, content).expect("write raw delta feature");
}
