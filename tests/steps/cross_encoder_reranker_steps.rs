//! Black-box steps for the local cross-encoder/reranker experiment.

use std::fs;

use cucumber::given;

use super::GivnWorld;
use crate::common;

#[given("a givn project with a true duplicate pair for reranking")]
fn true_duplicate_pair(world: &mut GivnWorld) {
    super::enforce_spec_similarity_gate_steps::permanent_cross_feature_pair(world);
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("a givn project with a valid boundary pair that has high embedding similarity")]
fn valid_boundary_pair(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    write_feature(
        &env,
        "givn/specs/plain/plain.feature",
        "Feature: plain lint output\n\n  Scenario: A parse error is reported on stderr and exits 1\n    Given a malformed feature file\n    When the user runs `givn lint`\n    Then stderr reports the parse error\n",
    );
    write_feature(
        &env,
        "givn/specs/json/json.feature",
        "Feature: machine-readable lint output\n\n  Scenario: A parse error is reported as a machine-readable finding\n    Given a malformed feature file\n    When the user runs `givn lint --json`\n    Then stdout reports a JSON parse finding\n",
    );
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\n",
    );
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("the local pair reranker is unavailable")]
fn unavailable_pair_reranker(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("missing".to_string());
}

fn write_feature(env: &common::TestEnv, path: &str, content: &str) {
    let file = env.path(path);
    fs::create_dir_all(file.parent().expect("feature parent")).expect("create feature parent");
    fs::write(file, content).expect("write feature");
}
