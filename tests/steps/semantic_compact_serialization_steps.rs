//! Regular acceptance steps for compact semantic retrieval serialization.

use std::fs;

use cucumber::{given, then, when};
use serde_json::Value;

use super::GivnWorld;
use crate::common;

const MINIMAL_TITLE: &str = "Minimal scenario";
const RICH_TITLE: &str = "Rich scenario";
const MINIMAL_HASH: &str = "95c980800a322345";
const RICH_HASH: &str = "c630c767c566666c";

const MINIMAL_FEATURE: &str = "Feature: compact retrieval\n\n  Scenario: Minimal scenario\n    Given minimal context\n    When minimal action runs\n    Then minimal result appears\n";

const RICH_FEATURE: &str = "Feature: rich retrieval\n  Feature description for semantic retrieval.\n\n  Background: feature setup\n    Given feature background is active\n\n  Rule: provider boundaries\n    Rule description for semantic retrieval.\n\n    Background: rule setup\n      Given rule background is active\n\n    Scenario Outline: Rich scenario\n      Given a scenario context\n        | header | value |\n        | input | first |\n      And the scenario includes documentation\n        \"\"\"\n        scenario docstring\n        \"\"\"\n      When the rich action runs\n      Then the rich result appears\n\n      Examples: rich example\n        | variant |\n        | first |\n";

pub(crate) fn prepare_rich_project(world: &mut GivnWorld) {
    prepare_project(world, RICH_FEATURE);
}

pub(crate) fn prepare_compact_fixture_project(world: &mut GivnWorld) {
    prepare_projects(
        world,
        &[
            ("minimal.feature", MINIMAL_FEATURE),
            ("rich.feature", RICH_FEATURE),
        ],
    );
}

fn prepare_project(world: &mut GivnWorld, feature: &str) {
    prepare_projects(world, &[("compact.feature", feature)]);
}

fn prepare_projects(world: &mut GivnWorld, fixtures: &[(&str, &str)]) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    let fixture_dir = specs.join("compact");
    fs::create_dir_all(&fixture_dir).expect("create compact fixture");
    for (file, feature) in fixtures {
        fs::write(fixture_dir.join(file), feature).expect("write compact fixture");
    }
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("disabled".to_string());
}

#[given("a permanent corpus with minimal and rich retrieval fixtures")]
fn compact_fixtures(world: &mut GivnWorld) {
    prepare_compact_fixture_project(world);
}

#[when("the user rebuilds the permanent scenario retrieval index")]
fn rebuild_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("compact serialization project");
    let output = common::run_givn_with_env(
        env,
        &["spec", "index"],
        Some(("GIVN_SPEC_EMBEDDER", "stub")),
    );
    assert!(
        output.status.success(),
        "index rebuild failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.last_output = Some(output);
}

#[then("the index contains both fixtures with their expected compact content hashes")]
fn compact_fixture_hashes(world: &mut GivnWorld) {
    let document = index(world);
    let minimal = entry(&document, MINIMAL_TITLE);
    let rich = entry(&document, RICH_TITLE);
    assert_eq!(minimal["content_hash"], MINIMAL_HASH);
    assert_eq!(rich["content_hash"], RICH_HASH);
    assert!(
        minimal["embedding"]
            .as_array()
            .is_some_and(|values| !values.is_empty())
    );
    assert!(
        rich["embedding"]
            .as_array()
            .is_some_and(|values| !values.is_empty())
    );
}

#[then("the index contains only the two fixture vectors")]
fn only_compact_fixtures(world: &mut GivnWorld) {
    let document = index(world);
    assert_eq!(
        document["entries"].as_array().expect("index entries").len(),
        2
    );
}

pub(crate) fn index(world: &GivnWorld) -> Value {
    let env = world.env.as_ref().expect("compact serialization project");
    serde_json::from_slice(
        &fs::read(env.path("givn/.spec-index.json")).expect("read compact serialization index"),
    )
    .expect("parse compact serialization index")
}

pub(crate) fn entry<'a>(document: &'a Value, title: &str) -> &'a Value {
    document["entries"]
        .as_array()
        .expect("index entries")
        .iter()
        .find(|entry| entry["title"] == title)
        .unwrap_or_else(|| panic!("missing index entry {title:?}: {document}"))
}
