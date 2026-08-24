//! Regular black-box steps for retrieval progress output.

use std::fs;

use cucumber::{given, then};

use super::GivnWorld;
use crate::common;

#[given("an initialized project with unreachable semantic model sources")]
fn unreachable_semantic_model_sources(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/unreachable.feature"),
        "Feature: unreachable\n\n  Scenario: Unreachable semantic model\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write unreachable model fixture");
    let xdg = env.path(".xdg-cache");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("download-stub".to_string());
    world.hf_endpoint_override = Some("http://127.0.0.1:1/".to_string());
}

#[given(
    "an initialized project with a completed E5 model cache in the XDG Givn cache and three permanent semantic scenarios"
)]
fn completed_e5_cache_with_three_scenarios(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("three.feature"),
        "Feature: three\n\n  Scenario: First cached semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the first behavior is returned\n\n  Scenario: Second cached semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the second behavior is returned\n\n  Scenario: Third cached semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the third behavior is returned\n",
    )
    .expect("write three-scenario cache fixture");
    let xdg = env.path(".xdg-cache");
    let cache = xdg.join("givn/e5");
    fs::create_dir_all(&cache).expect("create XDG E5 cache");
    fs::write(
        cache.join("givn-artifacts.json"),
        "{\"model\":\"intfloat/multilingual-e5-large\",\"complete\":true}\n",
    )
    .expect("write E5 cache manifest");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("stub".to_string());
}

#[then("stderr announces the start of the retrieval model download with the E5 cache location")]
fn announces_e5_download_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving retrieval model artifacts (E5) into "),
        "stderr did not announce the E5 download start: {stderr}"
    );
    let cache = world
        .xdg_cache_home
        .as_ref()
        .expect("XDG cache root")
        .join("givn/e5");
    assert!(
        stderr.contains(&cache.display().to_string()),
        "stderr did not name the E5 cache location: {stderr}"
    );
}

#[then("stderr reports per-file download progress for the E5 model artifacts")]
fn reports_e5_per_file_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for artifact in ["model.onnx", "tokenizer.json"] {
        assert!(
            stderr.contains(&format!("downloading {artifact} ")),
            "stderr missing per-file progress for {artifact}: {stderr}"
        );
    }
}

#[then("stderr announces the completion of the retrieval model download")]
fn announces_e5_download_completion(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("retrieval model artifacts (E5) resolved"),
        "stderr did not announce the E5 download completion: {stderr}"
    );
}

#[then("stderr announces the preparation of the retrieval embedder")]
fn announces_embedder_preparation(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("preparing retrieval embedder"),
        "stderr did not announce the embedder preparation: {stderr}"
    );
}

#[then("stderr announces the embedding of three pending scenarios")]
fn announces_three_pending_scenarios(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding 3 pending scenarios"),
        "stderr did not announce the three pending scenarios: {stderr}"
    );
}

#[then("stderr reports the embedding progress for each pending scenario")]
fn reports_per_scenario_embedding_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for line in [
        "embedded 1/3 scenarios",
        "embedded 2/3 scenarios",
        "embedded 3/3 scenarios",
    ] {
        assert!(
            stderr.contains(line),
            "stderr missing embedding progress {line}: {stderr}"
        );
    }
}

#[then("stderr announces the completion of the embedding phase")]
fn announces_embedding_completion(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding complete"),
        "stderr did not announce the embedding completion: {stderr}"
    );
}

#[then("stderr does not announce a model download")]
fn does_not_announce_model_download(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for needle in [
        "resolving retrieval model artifacts",
        "resolving reranker model artifacts",
        "resolving NLI model artifacts",
    ] {
        assert!(
            !stderr.contains(needle),
            "stderr announced a model download: {stderr}"
        );
    }
}

#[then("stderr does not report per-file download progress")]
fn does_not_report_per_file_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        !stderr.contains("downloading "),
        "stderr reported per-file download progress: {stderr}"
    );
}

#[then("stderr announces the embedding of the pending scenarios with their count")]
fn announces_embedding_count(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding 1 pending scenarios"),
        "stderr did not announce the pending scenario count: {stderr}"
    );
}

#[then("stderr identifies the failed E5 model artifact")]
fn identifies_failed_e5_artifact(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("model.onnx") || stderr.contains("tokenizer.json"),
        "stderr did not identify the failed E5 artifact: {stderr}"
    );
}

#[then("stderr announces the BGE and NLI model downloads")]
fn announces_second_stage_downloads(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr reports per-file download progress for the second-stage model artifacts")]
fn reports_second_stage_per_file_progress(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr does not announce a BGE or NLI model download")]
fn does_not_announce_second_stage_downloads(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for needle in [
        "resolving reranker model artifacts (BGE)",
        "resolving NLI model artifacts",
    ] {
        assert!(
            !stderr.contains(needle),
            "stderr announced a second-stage download: {stderr}"
        );
    }
}

#[then("stdout reports ranked search results")]
fn reports_ranked_search_results(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stdout reports advisory retrieval evidence")]
fn reports_advisory_retrieval_evidence(_world: &mut GivnWorld) {
    unimplemented!()
}
