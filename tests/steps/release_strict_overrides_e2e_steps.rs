//! Step definitions for the installed-build strictness scenarios: an
//! installed (release) givn build ignores fixture requests, announces them
//! once, and always requires the real models. These run against a release
//! binary resolved by `tests/common` (GIVN_RELEASE_BIN or a lazily built
//! `cargo build --release --bin givn`), with a clean subprocess environment
//! and an isolated XDG cache so no ambient request leaks an announcement.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

const ANNOUNCEMENT_SUFFIX: &str =
    "is ignored in installed builds; the real model output is always used";

/// The announcement wording for a named request channel, e.g. the stderr
/// line `GIVN_SPEC_BGE is ignored in installed builds; …`.
fn announcement_for(name: &str) -> String {
    format!("{name} {ANNOUNCEMENT_SUFFIX}")
}

#[given("a request to disable the reranker evidence stage")]
fn request_disable_reranker(world: &mut GivnWorld) {
    world.retrieval_bge = Some("disabled".to_string());
}

#[given("a request for simulated entailment evidence")]
fn request_simulated_entailment(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("a request for the real models from a chosen cache location")]
fn request_real_models_from_cache(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project prepared for semantic indexing is required first");
    let temp_cache = env.path(".release-model-cache");
    world.retrieval_bge = Some("auto".to_string());
    world.release_model_cache = Some(temp_cache);
}

#[when("a consumer runs `givn spec index` with an installed givn build")]
fn run_index_installed_build(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project prepared for semantic indexing is required first");
    let xdg = env.path(".release-xdg-cache");
    let mut environments: Vec<(&str, &str)> = Vec::new();
    if let Some(value) = world.retrieval_embedder.as_deref() {
        environments.push(("GIVN_SPEC_EMBEDDER", value));
    }
    if let Some(value) = world.retrieval_bge.as_deref() {
        environments.push(("GIVN_SPEC_BGE", value));
    }
    if let Some(value) = world.retrieval_reranker.as_deref() {
        environments.push(("GIVN_SPEC_RERANKER", value));
    }
    if let Some(path) = world.release_model_cache.as_deref() {
        environments.push((
            "GIVN_SPEC_MODEL_CACHE",
            path.to_str().expect("cache path is UTF-8"),
        ));
    }
    environments.push(("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")));
    environments.push(("HF_HOME", xdg.to_str().expect("XDG path is UTF-8")));
    if world.artifact_server.is_none() {
        world.artifact_server = Some(common::ArtifactFixtureServer::start());
    }
    if let Some(server) = world.artifact_server.as_ref() {
        environments.push(("HF_ENDPOINT", server.endpoint()));
    }
    world.last_output = Some(common::run_release_givn(
        env,
        &["spec", "index"],
        &environments,
    ));
}

#[then("the installed givn announces that the simulated-embedder request is ignored")]
fn announces_simulated_embedder_ignored(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains(&announcement_for("GIVN_SPEC_EMBEDDER")),
        "stderr missing the simulated-embedder announcement: {}",
        stderr,
    );
}

#[then("the installed givn refuses to proceed without the real embedder model")]
fn refuses_without_real_embedder(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("retrieval model is unavailable"),
        "stderr missing the real-embedder refusal: {}",
        stderr,
    );
    assert!(
        !super::output(world).status.success(),
        "installed givn succeeded without the real embedder model"
    );
}

#[then("the installed givn announces that the disable-reranker request is ignored")]
fn announces_disable_reranker_ignored(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains(&announcement_for("GIVN_SPEC_BGE")),
        "stderr missing the disable-reranker announcement: {}",
        stderr,
    );
}

#[then("the installed givn announces that the simulated-entailment request is ignored")]
fn announces_simulated_entailment_ignored(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains(&announcement_for("GIVN_SPEC_RERANKER")),
        "stderr missing the simulated-entailment announcement: {}",
        stderr,
    );
}

#[then("the installed givn refuses to proceed without the real model evidence")]
fn refuses_without_real_evidence(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("retrieval model is unavailable"),
        "stderr missing the real-model refusal: {}",
        stderr,
    );
    assert!(
        !super::output(world).status.success(),
        "installed givn succeeded without the real model evidence"
    );
}

#[then("the installed givn does not announce that any model request is ignored")]
fn does_not_announce_any_ignored(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        !stderr.contains(ANNOUNCEMENT_SUFFIX),
        "stderr unexpectedly announced an ignored model request: {}",
        stderr,
    );
}
