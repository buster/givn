//! Regular black-box steps for retrieval progress output.

use cucumber::{given, then};

use super::GivnWorld;

#[given("an initialized project with unreachable semantic model sources")]
fn unreachable_semantic_model_sources(_world: &mut GivnWorld) {
    unimplemented!()
}

#[given(
    "an initialized project with a completed E5 model cache in the XDG Givn cache and three permanent semantic scenarios"
)]
fn completed_e5_cache_with_three_scenarios(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr announces the start of the retrieval model download with the E5 cache location")]
fn announces_e5_download_start(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr reports per-file download progress for the E5 model artifacts")]
fn reports_e5_per_file_progress(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr announces the completion of the retrieval model download")]
fn announces_e5_download_completion(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr announces the preparation of the retrieval embedder")]
fn announces_embedder_preparation(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr announces the embedding of three pending scenarios")]
fn announces_three_pending_scenarios(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr reports the embedding progress for each pending scenario")]
fn reports_per_scenario_embedding_progress(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stderr announces the completion of the embedding phase")]
fn announces_embedding_completion(_world: &mut GivnWorld) {
    unimplemented!()
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
fn identifies_failed_e5_artifact(_world: &mut GivnWorld) {
    unimplemented!()
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
fn does_not_announce_second_stage_downloads(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stdout reports ranked search results")]
fn reports_ranked_search_results(_world: &mut GivnWorld) {
    unimplemented!()
}

#[then("stdout reports advisory retrieval evidence")]
fn reports_advisory_retrieval_evidence(_world: &mut GivnWorld) {
    unimplemented!()
}
