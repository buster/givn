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
    "an initialized project with fresh second-stage caches in the XDG Givn cache and reachable model sources"
)]
fn fresh_second_stage_caches_for_disabled(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/second-stage.feature"),
        "Feature: second stage\n\n  Scenario: Primary semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n\n  Scenario: Secondary semantic behavior\n    Given a configured service catalog\n    When the service is queried\n    Then the service result is returned\n",
    )
    .expect("write second-stage fixture");
    let xdg = env.path(".xdg-cache");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_bge = Some("disabled".to_string());
    world.retrieval_reranker = Some("disabled".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/second-stage.feature|Primary semantic behavior::givn/specs/second-stage.feature|Secondary semantic behavior=0.9600".to_string(),
    );
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

#[then("the index build logs the start of the E5 model download into the cache directory")]
fn index_build_logs_e5_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving retrieval model artifacts (E5) into "),
        "stderr did not log the E5 download start: {stderr}"
    );
    let cache = world
        .xdg_cache_home
        .as_ref()
        .expect("XDG cache root")
        .join("givn/e5");
    assert!(
        stderr.contains(&cache.display().to_string()),
        "stderr did not name the E5 cache directory: {stderr}"
    );
}

#[then("the index build logs byte progress for each E5 model file")]
fn index_build_logs_e5_byte_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for artifact in ["model.onnx", "tokenizer.json"] {
        assert!(
            stderr.contains(&format!("downloading {artifact} ")),
            "stderr missing per-file progress for {artifact}: {stderr}"
        );
    }
}

#[then("the index build logs the end of the E5 model download")]
fn index_build_logs_e5_end(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("retrieval model artifacts (E5) resolved"),
        "stderr did not log the E5 download end: {stderr}"
    );
}

#[then("the index build logs embedder preparation before scenario processing")]
fn index_build_logs_embedder_prep(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("preparing retrieval embedder"),
        "stderr did not log the embedder preparation: {stderr}"
    );
}

#[then("the index build logs the count of scenarios awaiting embedding")]
fn index_build_logs_embedding_count(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding 1 pending scenarios"),
        "stderr did not log the pending scenario count: {stderr}"
    );
}

#[then("the index build logs three scenarios awaiting embedding")]
fn index_build_logs_three_awaiting(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding 3 pending scenarios"),
        "stderr did not log the three pending scenarios: {stderr}"
    );
}

#[then("the index build logs one progress line per scenario slot")]
fn index_build_logs_one_per_slot(world: &mut GivnWorld) {
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

#[then("the index build logs the end of scenario processing")]
fn index_build_logs_end_of_processing(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("embedding complete"),
        "stderr did not log the end of scenario processing: {stderr}"
    );
}

#[then("the warm-cache run stays free of model-download wording")]
fn warm_cache_stays_free_of_download_wording(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for needle in [
        "resolving retrieval model artifacts",
        "resolving reranker model artifacts",
        "resolving NLI model artifacts",
    ] {
        assert!(
            !stderr.contains(needle),
            "stderr contains model-download wording: {stderr}"
        );
    }
}

#[then("the warm-cache run stays free of per-file byte progress")]
fn warm_cache_stays_free_of_byte_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        !stderr.contains("downloading "),
        "stderr contains per-file byte progress: {stderr}"
    );
}

#[then("the failed run still logs the start of the E5 model download into the cache directory")]
fn failed_run_still_logs_e5_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving retrieval model artifacts (E5) into "),
        "stderr did not log the E5 download start: {stderr}"
    );
    let cache = world
        .xdg_cache_home
        .as_ref()
        .expect("XDG cache root")
        .join("givn/e5");
    assert!(
        stderr.contains(&cache.display().to_string()),
        "stderr did not name the E5 cache directory: {stderr}"
    );
}

#[then("the failure names the E5 model file")]
fn failure_names_e5_model_file(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("model.onnx") || stderr.contains("tokenizer.json"),
        "stderr did not name the failed E5 model file: {stderr}"
    );
}

#[then("the audit emits the start of the reranker download")]
fn audit_emits_reranker_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving reranker model artifacts (BGE) into "),
        "stderr did not emit the reranker download start: {stderr}"
    );
}

#[then("the audit emits the start of the NLI download")]
fn audit_emits_nli_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving NLI model artifacts into "),
        "stderr did not emit the NLI download start: {stderr}"
    );
}

#[then("the audit emits byte progress for the reranker and NLI model files")]
fn audit_emits_second_stage_byte_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for artifact in ["model.onnx", "onnx/model.onnx"] {
        assert!(
            stderr.contains(&format!("downloading {artifact} ")),
            "stderr missing second-stage per-file progress for {artifact}: {stderr}"
        );
    }
}

#[then("the audit channel stays free of reranker and NLI download wording")]
fn audit_channel_stays_free_of_second_stage(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for needle in [
        "resolving reranker model artifacts (BGE)",
        "resolving NLI model artifacts",
    ] {
        assert!(
            !stderr.contains(needle),
            "stderr contains second-stage download wording: {stderr}"
        );
    }
}

#[then("the review run logs the start of the reranker download")]
fn review_run_logs_reranker_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving reranker model artifacts (BGE) into "),
        "stderr did not log the reranker download start: {stderr}"
    );
}

#[then("the review run logs the start of the NLI download")]
fn review_run_logs_nli_start(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("resolving NLI model artifacts into "),
        "stderr did not log the NLI download start: {stderr}"
    );
}

#[then("the review run logs byte progress for the reranker and NLI model files")]
fn review_run_logs_second_stage_byte_progress(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for artifact in ["model.onnx", "onnx/model.onnx"] {
        assert!(
            stderr.contains(&format!("downloading {artifact} ")),
            "stderr missing second-stage per-file progress for {artifact}: {stderr}"
        );
    }
}

#[then("the search prints ranked results on stdout")]
fn search_prints_ranked_results(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(
        stdout.contains("rank 1:"),
        "stdout did not print ranked search results: {stdout}"
    );
}

#[then("stdout reports BGE evidence status")]
fn stdout_reports_bge_evidence_status(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(
        stdout.contains("BGE status:"),
        "stdout did not report BGE evidence status: {stdout}"
    );
}
