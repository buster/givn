//! Compiled-CLI E2E bindings for retrieval progress output.

use std::fs;

use cucumber::{given, when};

use super::GivnWorld;
use crate::common;

#[given(
    "an initialized project with a completed permanent semantic index and a cold E5 model cache"
)]
fn completed_index_and_cold_e5_cache(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/search.feature"),
        "Feature: search\n\n  Scenario: Searchable semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write search fixture");
    let xdg = env.path(".xdg-cache");
    let xdg_value = xdg.to_str().expect("XDG path is UTF-8");
    let output = common::run_givn_with_envs(
        &env,
        &["spec", "index"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("XDG_CACHE_HOME", xdg_value),
        ],
    );
    assert!(
        output.status.success(),
        "build search index failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let model_cache = xdg.join("givn/e5");
    if model_cache.exists() {
        fs::remove_dir_all(model_cache).expect("clear cold E5 cache");
    }
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
}

#[given("a reachable model source")]
fn reachable_model_source(world: &mut GivnWorld) {
    world.artifact_server = Some(common::ArtifactFixtureServer::start());
}

#[when(
    "the end-to-end user searches for a permanent semantic scenario with model acquisition through the givn CLI"
)]
fn e2e_search_with_model_acquisition(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("search project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let server = world
        .artifact_server
        .as_ref()
        .expect("artifact fixture server");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "search", "permanent semantic scenario"],
        &[
            ("GIVN_SPEC_EMBEDDER", "download-stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
            ("HF_ENDPOINT", server.endpoint()),
        ],
    ));
}

#[given(
    "an initialized project with a completed permanent semantic index and a cold second-stage model cache"
)]
fn completed_index_and_cold_second_stage_cache(world: &mut GivnWorld) {
    unimplemented!()
}

#[when("the end-to-end user runs advisory review for the active change through the givn CLI")]
fn e2e_advisory_review_with_acquisition(world: &mut GivnWorld) {
    unimplemented!()
}
