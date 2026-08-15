//! Black-box steps for local semantic scenario retrieval.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("a givn project with permanent scenarios for retrieval")]
fn retrieval_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("retrieval.feature"),
        "Feature: retrieval fixtures\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Authentication token is missing\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n\n  Scenario: Provider configuration is valid\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write retrieval fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write retrieval test commands");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("the retrieval model is unavailable")]
fn unavailable_retrieval_model(world: &mut GivnWorld) {
    world.retrieval_embedder = Some("missing".to_string());
}

#[when("the user rebuilds the local spec index without changing specifications")]
fn rebuild_retrieval_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let output = common::run_givn_with_env(
        env,
        &["spec", "index"],
        Some((
            "GIVN_SPEC_EMBEDDER",
            world.retrieval_embedder.as_deref().unwrap_or("stub"),
        )),
    );
    world.last_output = Some(output);
}

#[given("the local spec index has been built")]
fn local_spec_index_built(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let output = common::run_givn_with_env(
        env,
        &["spec", "index"],
        Some((
            "GIVN_SPEC_EMBEDDER",
            world.retrieval_embedder.as_deref().unwrap_or("stub"),
        )),
    );
    assert!(
        output.status.success(),
        "retrieval index setup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[given(regex = r#"^an active change delta contains a scenario named "([^"]+)"$"#)]
fn active_change_delta(world: &mut GivnWorld, title: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let path = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        path,
        format!(
            "@givn.delta @running-change\nFeature: running change\n\n  Scenario: {title}\n    Given a running change\n"
        ),
    )
    .expect("write active delta");
}

#[given(regex = r#"^an active change delta contains a scenario similar to "([^"]+)"$"#)]
fn active_change_similar_to(world: &mut GivnWorld, _permanent_title: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let path = env.path("givn/changes/running-change/specs/running-change.feature");
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        path,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write similar active delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Provider name is unknown` | related |\n",
    )
    .expect("write review gate fixture");
}

#[given(regex = r#"^an active change delta contains a removed scenario named "([^"]+)"$"#)]
fn removed_change_delta(world: &mut GivnWorld, title: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let path = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        path,
        format!(
            "@givn.delta @running-change\nFeature: running change\n\n  @givn.removed\n  Scenario: {title}\n    Given a removed scenario\n"
        ),
    )
    .expect("write removed active delta");
}

#[given(regex = r#"^a review disposition exists for "([^"]+)" and "([^"]+)"$"#)]
fn review_disposition(world: &mut GivnWorld, delta_title: String, permanent_title: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    let path = env.path("givn/changes/running-change/review.md");
    fs::create_dir_all(path.parent().expect("review parent")).expect("create review parent");
    fs::write(
        path,
        format!(
            "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `{delta_title}` | `{permanent_title}` | boundary |\n"
        ),
    )
    .expect("write review disposition");
}

#[when(regex = r#"^the user searches for "([^"]+)"$"#)]
fn search_retrieval(world: &mut GivnWorld, query: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    world.last_output = Some(common::run_givn_with_env(
        env,
        &["spec", "search", &query],
        Some((
            "GIVN_SPEC_EMBEDDER",
            world.retrieval_embedder.as_deref().unwrap_or("stub"),
        )),
    ));
}

#[then("the local spec index exists")]
fn local_spec_index_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("retrieval project is required");
    assert!(env.path("givn/.spec-index.json").is_file());
}
