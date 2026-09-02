//! Black-box steps for the strict semantic scenario uniqueness gate.

use std::fs;

use cucumber::{given, then};

use super::GivnWorld;
use crate::common;

const PROVIDER_STEPS: &str = "    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";

#[given("a givn project with permanent near-duplicate scenarios in different feature files")]
pub(crate) fn permanent_cross_feature_pair(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_permanent_specs(&env);
    write_feature(
        &env,
        "givn/specs/provider/provider.feature",
        &format!(
            "Feature: provider lookup\n\n  Scenario: Provider name is unknown\n{PROVIDER_STEPS}"
        ),
    );
    write_feature(
        &env,
        "givn/specs/auth/auth.feature",
        &format!(
            "Feature: provider authentication\n\n  Scenario: Unknown provider is rejected\n{PROVIDER_STEPS}"
        ),
    );
    prepare_retrieval_commands(&env);
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("a givn project with permanent near-duplicate scenarios in one feature file")]
fn permanent_same_feature_pair(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_permanent_specs(&env);
    write_feature(
        &env,
        "givn/specs/provider/provider.feature",
        &format!(
            "Feature: provider lookup\n\n  Scenario: Provider name is unknown\n{PROVIDER_STEPS}\n  Scenario: Unknown provider selection is rejected\n{PROVIDER_STEPS}"
        ),
    );
    prepare_retrieval_commands(&env);
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[given("a givn project with one permanent scenario for similarity review")]
fn permanent_single_similarity_scenario(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_permanent_specs(&env);
    write_feature(
        &env,
        "givn/specs/provider/provider.feature",
        &format!(
            "Feature: provider lookup\n\n  Scenario: Provider name is unknown\n{PROVIDER_STEPS}"
        ),
    );
    prepare_retrieval_commands(&env);
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[given("a givn project with a near-duplicate pair above the fixed similarity threshold")]
fn permanent_pair_above_fixed_threshold(world: &mut GivnWorld) {
    permanent_cross_feature_pair(world);
}

#[given("a givn project with six permanent near-duplicate scenarios")]
fn permanent_six_near_duplicates(world: &mut GivnWorld) {
    let env = common::make_init_root();
    clear_permanent_specs(&env);
    let mut content = String::from("Feature: provider lookup\n\n");
    for suffix in ["one", "two", "three", "four", "five", "six"] {
        content.push_str(&format!(
            "  Scenario: Candidate {suffix}: provider name is unknown and command rejects the request\n{PROVIDER_STEPS}\n"
        ));
    }
    write_feature(&env, "givn/specs/provider/provider.feature", &content);
    prepare_retrieval_commands(&env);
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[given("an active change delta contains a distinct scenario")]
pub(crate) fn distinct_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    write_delta(
        env,
        "running-change",
        "running-change",
        "Distinct running behavior",
        "    Given a distinct lifecycle state\n    When the user completes the lifecycle\n    Then the archive records the completion\n",
    );
}

#[given("the active change contains a scenario similar to the permanent scenario")]
fn similar_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    write_delta(
        env,
        "running-change",
        "running-change",
        "Unknown provider is rejected",
        PROVIDER_STEPS,
    );
}

#[given(
    "the active change contains a boundary-specific scenario similar to the permanent scenario"
)]
fn boundary_similar_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    write_delta(
        env,
        "running-change",
        "running-change",
        "Unknown provider selection is rejected",
        PROVIDER_STEPS,
    );
}

#[given(
    "the active change contains a rewritten boundary-specific scenario below the similarity threshold"
)]
fn boundary_rewritten_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    write_delta(
        env,
        "running-change",
        "running-change",
        "Catalog boundary rejects unknown provider names before command dispatch",
        "    Given the provider catalog boundary is isolated from command dispatch\n    When an unknown provider name reaches catalog lookup\n    Then the catalog returns a boundary-specific rejection before authentication\n",
    );
}

#[given("the active change modifies that permanent scenario without changing its title")]
fn modify_own_scenario(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    let path = env.path("givn/changes/running-change/specs/provider/provider.feature");
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta parent");
    fs::write(
        path,
        "@givn.delta @provider\nFeature: provider lookup\n\n  @givn.modified\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request with the provider boundary named\n",
    )
    .expect("write modified similarity delta");
}

#[given("the active change removes the duplicate scenario named \"Unknown provider is rejected\"")]
pub(crate) fn remove_duplicate_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    let path = env.path("givn/changes/running-change/specs/auth/auth.feature");
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta directory");
    fs::write(
        path,
        "@givn.delta @auth\nFeature: provider authentication\n\n  @givn.removed\n  Scenario: Unknown provider is rejected\n    Given a removed duplicate\n",
    )
    .expect("write duplicate removal delta");
}

#[given("the project requests a similarity threshold of 1.00")]
fn requested_similarity_override(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("similarity project is required");
    let config = env.path("givn/config.yaml");
    let mut content = fs::read_to_string(&config).expect("read project config");
    content.push_str("\nretrieval:\n  similarity_threshold: 1.00\n");
    fs::write(config, content).expect("write project config override");
}

#[given(
    regex = r#"^a review disposition classifies "([^"]+)" and "([^"]+)" as "([^"]+)" with a boundary rationale$"#
)]
fn review_disposition_classification(
    world: &mut GivnWorld,
    first_title: String,
    second_title: String,
    decision: String,
) {
    let env = world.env.as_ref().expect("similarity project is required");
    let path = env.path("givn/changes/running-change/review.md");
    fs::create_dir_all(path.parent().expect("review parent")).expect("create review parent");
    fs::write(
        path,
        format!(
            "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition | Rationale |\n|---|---|---|---|\n| `{first_title}` | `{second_title}` | {decision} | Different observable boundary |\n"
        ),
    )
    .expect("write similarity disposition");
}

fn clear_permanent_specs(env: &common::TestEnv) {
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
}

fn prepare_retrieval_commands(env: &common::TestEnv) {
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\n",
    );
}

fn write_feature(env: &common::TestEnv, path: &str, content: &str) {
    let file = env.path(path);
    fs::create_dir_all(file.parent().expect("feature parent")).expect("create feature parent");
    fs::write(file, content).expect("write permanent feature");
}

fn write_delta(env: &common::TestEnv, change_id: &str, capability: &str, title: &str, steps: &str) {
    let path = env.path(&format!(
        "givn/changes/{change_id}/specs/{capability}/{capability}.feature"
    ));
    fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta parent");
    fs::write(
        path,
        format!(
            "@givn.delta @{capability}\nFeature: {capability}\n\n  @givn.added\n  Scenario: {title}\n{steps}"
        ),
    )
    .expect("write similarity delta");
}

#[then(regex = r#"^the local spec index contains scenario title "([^"]+)"$"#)]
fn index_contains_title(world: &mut GivnWorld, title: String) {
    let env = world.env.as_ref().expect("similarity project is required");
    let content = fs::read_to_string(env.path("givn/.spec-index.json"))
        .expect("read generated similarity index");
    assert!(
        content.contains(&format!("\"title\": \"{title}\"")),
        "index does not contain scenario title {title:?}"
    );
}
