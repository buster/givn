//! Compiled-CLI E2E bindings for semantic model acquisition and diagnostics.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("an initialized project with a fresh XDG Givn cache and a reachable model source")]
fn e2e_fresh_e5_cache(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let xdg = env.path(".xdg-cache");
    assert!(!xdg.exists(), "E5 fixture cache must start cold");
    let event_path = env.path("download-events.log");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.download_event_path = Some(event_path);
    world.artifact_server = Some(common::ArtifactFixtureServer::start());
}

#[given("the project contains fitting permanent semantic scenarios")]
fn e2e_fitting_permanent_scenarios(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 project");
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/retrieval.feature"),
        "Feature: retrieval\n\n  Scenario: Fitting retrieval behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write E5 retrieval fixture");
}

#[when("the end-to-end user builds the permanent semantic index with `givn spec index`")]
fn e2e_runs_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let server = world
        .artifact_server
        .as_ref()
        .expect("artifact fixture server");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "index"],
        &[
            ("GIVN_SPEC_EMBEDDER", "download-stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
            ("HF_ENDPOINT", server.endpoint()),
            (
                "GIVN_SPEC_DOWNLOAD_EVENTS",
                world
                    .download_event_path
                    .as_ref()
                    .expect("download event path")
                    .to_str()
                    .expect("download event path is UTF-8"),
            ),
        ],
    ));
}

#[then("stdout reports a completed permanent semantic index")]
fn e2e_index_completed(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("Indexed 1 permanent scenarios"),
        "stdout did not report the completed index: {}",
        super::stdout(world)
    );
}

#[then("the permanent semantic index file exists")]
fn e2e_index_file_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    assert!(
        env.path("givn/.spec-index.json").is_file(),
        "permanent semantic index was not written"
    );
}

#[then("the E5 tokenizer and model artifacts exist in the XDG Givn cache")]
fn e2e_e5_artifacts_exist(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let snapshot = xdg.join("givn/e5/models--Qdrant--multilingual-e5-large-onnx/snapshots/main");
    for file in [
        "model.onnx",
        "model.onnx_data",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        assert!(snapshot.join(file).is_file(), "missing E5 artifact {file}");
    }
    assert!(xdg.join("givn/e5/givn-artifacts.json").is_file());
    let events = fs::read_to_string(
        world
            .download_event_path
            .as_ref()
            .expect("download event path"),
    )
    .expect("read download events");
    let events: Vec<&str> = events.lines().collect();
    assert_eq!(events, ["artifact-resolved", "token-budget-measured"]);
}

#[when(
    "the end-to-end user searches for a permanent semantic scenario without an index through the givn CLI"
)]
fn e2e_search_without_index(_world: &mut GivnWorld) {
    let world = _world;
    let env = world.env.as_ref().expect("semantic project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "search", "permanent semantic scenario"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
        ],
    ));
}

#[then("the search reports `spec index is missing at <path>; run `givn spec index` first`")]
fn e2e_search_reports_missing_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let expected = format!(
        "spec index is missing at {}; run `givn spec index` first",
        env.path("givn/.spec-index.json").display()
    );
    assert!(
        super::stderr(world).contains(&expected),
        "stderr did not report missing search index: {}",
        super::stderr(world)
    );
}

#[given("a complete draft scenario is available for exploration")]
fn e2e_draft_for_exploration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write draft feature");
}

#[when("the end-to-end user explores the draft without an index through the givn CLI")]
fn e2e_explore_without_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
        ],
    ));
}

#[then("the exploration reports `spec index is missing at <path>; run `givn spec index` first`")]
fn e2e_explore_reports_missing_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let expected = format!(
        "spec index is missing at {}; run `givn spec index` first",
        env.path("givn/.spec-index.json").display()
    );
    assert!(
        super::stderr(world).contains(&expected),
        "stderr did not report missing exploration index: {}",
        super::stderr(world)
    );
}

#[given("an active change delta is available for advisory review")]
fn e2e_advisory_change_available(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let path = env.path("givn/changes/running-change/specs/running-change.feature");
    fs::create_dir_all(path.parent().expect("advisory delta parent"))
        .expect("create advisory delta");
    fs::write(
        path,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Advisory change behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write advisory delta");
}

#[when("the end-to-end user runs advisory review without an index through the givn CLI")]
fn e2e_advisory_review_without_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
        ],
    ));
}

#[then(
    "the advisory review reports `spec index is missing at <path>; run `givn spec index` first`"
)]
fn e2e_advisory_reports_missing_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let expected = format!(
        "spec index is missing at {}; run `givn spec index` first",
        env.path("givn/.spec-index.json").display()
    );
    assert!(
        super::stderr(world).contains(&expected),
        "stderr did not report missing advisory-review index: {}",
        super::stderr(world)
    );
}

#[given("an active change delta is available for blocking review")]
fn e2e_blocking_change_available(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let change_dir = env.path("givn/changes/running-change");
    let specs_dir = change_dir.join("specs/running-change");
    fs::create_dir_all(&specs_dir).expect("create blocking delta");
    fs::write(
        specs_dir.join("running-change.feature"),
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.removed\n  Scenario: Permanent semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write blocking delta");
    fs::write(
        env.path("givn/config.yaml"),
        "givn_config_version: \"0.3.0\"\naddons:\n  arc42: false\n  coverage: false\n",
    )
    .expect("disable optional archive addons");
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\n",
    );
}

#[when("the end-to-end user runs blocking review without an index through the givn CLI")]
fn e2e_blocking_review_without_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
        ],
    ));
}

#[then(
    "the blocking review reports `spec index is missing at <path>; run `givn spec index` first`"
)]
fn e2e_blocking_reports_missing_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let expected = format!(
        "spec index is missing at {}; run `givn spec index` first",
        env.path("givn/.spec-index.json").display()
    );
    assert!(
        super::stderr(world).contains(&expected),
        "stderr did not report missing blocking-review index: {}",
        super::stderr(world)
    );
}

#[given("fresh BGE and NLI caches in the XDG Givn cache and reachable model sources")]
fn e2e_fresh_second_stage_caches(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/second-stage.feature"),
        "Feature: second stage\n\n  Scenario: Primary semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n\n  Scenario: Secondary semantic behavior\n    Given a configured service catalog\n    When the service is queried\n    Then the service result is returned\n",
    )
    .expect("write second-stage fixture");
    let xdg = env.path(".xdg-cache");
    assert!(!xdg.exists(), "second-stage fixture cache must start cold");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_bge = Some("auto".to_string());
    world.retrieval_reranker = Some("nli-auto".to_string());
    world.retrieval_bge_local_stub = true;
    world.retrieval_nli_local_stub = true;
    world.retrieval_duplicate_scores = Some(
        "givn/specs/second-stage.feature|Primary semantic behavior::givn/specs/second-stage.feature|Secondary semantic behavior=0.9600".to_string(),
    );
    world.artifact_server = Some(common::ArtifactFixtureServer::start());
}

#[given("BGE and NLI evidence are explicitly enabled")]
fn e2e_enable_second_stage_evidence(world: &mut GivnWorld) {
    assert_eq!(world.retrieval_bge.as_deref(), Some("auto"));
    assert_eq!(world.retrieval_reranker.as_deref(), Some("nli-auto"));
}

#[when("the end-to-end user audits permanent scenarios with `givn spec duplicates --format json`")]
fn e2e_runs_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("second-stage project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let server = world
        .artifact_server
        .as_ref()
        .expect("artifact fixture server");
    let scores = world
        .retrieval_duplicate_scores
        .as_deref()
        .expect("second-stage score override");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "auto"),
            ("GIVN_SPEC_RERANKER", "nli-auto"),
            ("GIVN_SPEC_BGE_LOCAL_STUB", "1"),
            ("GIVN_SPEC_NLI_LOCAL_STUB", "1"),
            ("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores),
            ("XDG_CACHE_HOME", xdg.to_str().expect("XDG path is UTF-8")),
            ("HF_ENDPOINT", server.endpoint()),
        ],
    ));
}

#[then("the JSON reports participating BGE and NLI evidence")]
fn e2e_json_reports_bge_nli(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("duplicate output JSON");
    assert_eq!(document["bge_run"]["status"], "BGE_ENABLED");
    assert_eq!(document["nli_run"]["status"], "NLI_ENABLED");
}

#[then("the BGE model artifacts exist in the XDG Givn reranker cache")]
fn e2e_bge_artifacts_exist(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let snapshot = xdg.join(
        "givn/bge/models--rozgo--bge-reranker-v2-m3/snapshots/fbd57b17b4db111a9d16813bb08b4c804fac18e9",
    );
    for file in [
        "model.onnx",
        "model.onnx.data",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        assert!(snapshot.join(file).is_file(), "missing BGE artifact {file}");
    }
}

#[then("the NLI model artifacts exist in the XDG Givn NLI cache")]
fn e2e_nli_artifacts_exist(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let snapshot = xdg.join(
        "givn/nli/models--MoritzLaurer--mDeBERTa-v3-base-xnli-multilingual-nli-2mil7/snapshots/b5113eb38ab63efdd7f280f8c144ea8b13f978ce",
    );
    for file in [
        "onnx/model.onnx",
        "config.json",
        "tokenizer.json",
        "tokenizer_config.json",
    ] {
        assert!(snapshot.join(file).is_file(), "missing NLI artifact {file}");
    }
}
