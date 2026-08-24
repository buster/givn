//! E2E black-box steps for the real evidence-search CLI.

use cucumber::{given, when};

use super::{GivnWorld, search_evidence_steps};
use crate::common;

#[given("an end-to-end project with a permanent spec index for evidence search")]
fn e2e_evidence_project(world: &mut GivnWorld) {
    super::spec_retrieval_steps::retrieval_project(world);
    super::spec_retrieval_steps::local_spec_index_built(world);
}

#[given("BGE and NLI evidence are enabled for the end-to-end search")]
fn e2e_evidence_enabled(world: &mut GivnWorld) {
    world.retrieval_bge = Some("stub".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("BGE and NLI evidence recommend the listed candidates")]
fn e2e_evidence_recommends(world: &mut GivnWorld) {
    world.retrieval_bge_scores = Some(search_evidence_steps::bge_recommend_fixture());
    world.retrieval_nli_scores = Some(search_evidence_steps::nli_recommend_fixture());
}

#[when(regex = r#"^the end-to-end user searches "([^"]+)" with evidence$"#)]
fn e2e_search_with_evidence(world: &mut GivnWorld, query: String) {
    let env = world.env.as_ref().expect("evidence search project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "search", "--evidence", &query],
        &super::world_envs(world),
    ));
}