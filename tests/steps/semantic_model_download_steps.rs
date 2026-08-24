//! Regular Gherkin bindings for semantic model acquisition and cache policy.

use std::fs;
use std::process::{Command, Stdio};

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

#[given(
    "an initialized project with a completed E5 model cache in the XDG Givn cache and permanent semantic scenarios"
)]
fn completed_e5_cache(_world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("cache.feature"),
        "Feature: cache\n\n  Scenario: Cached semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the cached behavior is returned\n",
    )
    .expect("write permanent cache fixture");

    let xdg = env.path(".xdg-cache");
    let cache = xdg.join("givn/e5");
    fs::create_dir_all(&cache).expect("create XDG E5 cache");
    fs::write(
        cache.join("givn-artifacts.json"),
        "{\"model\":\"intfloat/multilingual-e5-large\",\"complete\":true}\n",
    )
    .expect("write E5 cache manifest");

    _world.env = Some(env);
    _world.xdg_cache_home = Some(xdg);
    _world.retrieval_embedder = Some("stub".to_string());
}

#[then("stdout reports that existing semantic model artifacts were reused")]
fn reports_reused_model_artifacts(world: &mut GivnWorld) {
    let cache = world
        .xdg_cache_home
        .as_ref()
        .expect("XDG cache root")
        .join("givn/e5");
    assert!(
        stdout(world).contains(&format!(
            "Reused E5 model artifacts from {}",
            cache.display()
        )),
        "stdout did not report E5 cache reuse: {}",
        stdout(world)
    );
}

#[given("an initialized project with unavailable semantic model sources")]
fn unavailable_semantic_model_sources(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/unavailable.feature"),
        "Feature: unavailable\n\n  Scenario: Unavailable semantic model\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write unavailable model fixture");
    let xdg = env.path(".xdg-cache");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("missing".to_string());
}

#[then("stderr identifies the E5 model cache and failed artifact")]
fn reports_e5_cache_failure(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    let cache = world
        .xdg_cache_home
        .as_ref()
        .expect("XDG cache root")
        .join("givn/e5");
    assert!(
        stderr.contains(&format!("E5 cache={}", cache.display())),
        "stderr did not identify E5 cache: {stderr}"
    );
    assert!(
        stderr.contains("model.onnx") || stderr.contains("tokenizer.json"),
        "stderr did not identify failed E5 artifact: {stderr}"
    );
}

#[then("stderr names a local artifact or environment-variable alternative")]
fn reports_local_artifact_alternative(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("GIVN_SPEC_MODEL_CACHE") && stderr.contains("GIVN_SPEC_E5_TOKENIZER"),
        "stderr did not name local E5 alternatives: {stderr}"
    );
}

#[then("no permanent semantic index file is written")]
fn no_permanent_index_written(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    assert!(!env.path("givn/.spec-index.json").exists());
}

#[given(
    "an initialized project with an existing permanent semantic index and unavailable semantic model sources"
)]
fn existing_index_and_unavailable_sources(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/existing.feature"),
        "Feature: existing\n\n  Scenario: Existing semantic model\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write existing index fixture");
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
        "build existing semantic index failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.permanent_index_snapshot =
        Some(fs::read(env.path("givn/.spec-index.json")).expect("read existing index snapshot"));
    fs::write(
        env.path("givn/specs/existing.feature"),
        "Feature: existing\n\n  Scenario: Existing semantic model\n    Given a configured provider catalog\n    When the provider query changes\n    Then the provider result is returned\n",
    )
    .expect("change existing semantic input");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("missing".to_string());
}

#[then("the existing permanent semantic index remains unchanged byte-for-byte")]
fn existing_index_is_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let before = world
        .permanent_index_snapshot
        .as_ref()
        .expect("existing index snapshot");
    let after = fs::read(env.path("givn/.spec-index.json")).expect("read existing index");
    assert_eq!(after, *before, "E5 failure changed the existing index");
}

#[given("an initialized project with a malformed or incompatible permanent semantic index")]
fn malformed_or_incompatible_index(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/invalid-index.feature"),
        "Feature: invalid index\n\n  Scenario: Indexed semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write invalid-index fixture");
    fs::write(env.path("givn/.spec-index.json"), "not-json")
        .expect("write malformed index fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[when("the user searches for a permanent semantic scenario")]
fn search_for_permanent_scenario(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "search", "permanent semantic scenario"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

#[then("stderr reports the index validation problem")]
fn reports_index_validation_problem(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("failed to parse index"),
        "stderr did not report index parsing failure: {}",
        super::stderr(world)
    );
}

#[then("stderr does not report `spec index is missing at <path>; run `givn spec index` first`")]
fn no_missing_index_diagnostic(world: &mut GivnWorld) {
    assert!(
        !super::stderr(world).contains("spec index is missing at"),
        "malformed index was mislabeled as absent: {}",
        super::stderr(world)
    );
}

#[given(
    "an initialized project with permanent semantic scenarios and no persistent semantic index"
)]
fn project_without_persistent_index(_world: &mut GivnWorld) {
    let world = _world;
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/absent-index.feature"),
        "Feature: absent index\n\n  Scenario: Permanent semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write absent-index fixture");
    let xdg = env.path(".xdg-cache");
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
    world.retrieval_embedder = Some("stub".to_string());
}

#[then("stdout is valid duplicate-audit JSON")]
fn valid_duplicate_audit_json(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("duplicate output JSON");
    assert!(
        document.is_object(),
        "duplicate output is not a JSON object"
    );
}

#[then("stderr reports `spec index is missing at <path>; run `givn spec index` first`")]
fn reports_missing_index_diagnostic(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let expected = format!(
        "spec index is missing at {}; run `givn spec index` first",
        env.path("givn/.spec-index.json").display()
    );
    assert!(
        super::stderr(world).contains(&expected),
        "stderr did not report missing index: {}",
        super::stderr(world)
    );
}

#[then("the persistent semantic index remains absent")]
fn persistent_index_remains_absent(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    assert!(!env.path("givn/.spec-index.json").exists());
}

#[given("an initialized project with unavailable BGE and NLI model sources")]
fn unavailable_second_stage_sources(world: &mut GivnWorld) {
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
    world.retrieval_bge = Some("missing".to_string());
    world.retrieval_reranker = Some("nli-missing".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/second-stage.feature|Primary semantic behavior::givn/specs/second-stage.feature|Secondary semantic behavior=0.9600".to_string(),
    );
}

#[given("the project contains permanent semantic candidates for both evidence layers")]
fn permanent_candidates_for_both_layers(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("second-stage project");
    assert!(env.path("givn/specs/second-stage.feature").is_file());
}

#[given("BGE and NLI evidence are explicitly enabled for automatic acquisition")]
fn enable_automatic_second_stage_acquisition(world: &mut GivnWorld) {
    assert!(matches!(
        world.retrieval_bge.as_deref(),
        Some("missing" | "auto")
    ));
    assert!(matches!(
        world.retrieval_reranker.as_deref(),
        Some("nli-missing" | "nli-auto")
    ));
}

#[then("stdout is valid duplicate-audit JSON with unavailable BGE and NLI evidence")]
fn duplicate_json_reports_unavailable_layers(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("duplicate output JSON");
    assert_eq!(document["bge_run"]["status"], "BGE_UNAVAILABLE");
    assert_eq!(document["nli_run"]["status"], "NLI_UNAVAILABLE");
}

#[then("stderr names the failed model artifacts and their local override variables")]
fn reports_second_stage_artifact_failure(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("BGE model is unavailable"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("NLI model is unavailable"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("model.onnx"), "stderr: {stderr}");
    assert!(stderr.contains("onnx/model.onnx"), "stderr: {stderr}");
    assert!(
        stderr.contains("GIVN_SPEC_BGE_MODEL_DIR"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("GIVN_SPEC_NLI_MODEL_DIR"),
        "stderr: {stderr}"
    );
}

#[given(
    "an initialized project with corrupt BGE and NLI cache snapshots and reachable model sources"
)]
fn corrupt_second_stage_caches(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/second-stage.feature"),
        "Feature: second stage\n\n  Scenario: Primary semantic behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n\n  Scenario: Secondary semantic behavior\n    Given a configured service catalog\n    When the service is queried\n    Then the service result is returned\n",
    )
    .expect("write second-stage fixture");
    let xdg = env.path(".xdg-cache");
    let bge_cache = xdg.join("givn/bge");
    let nli_cache = xdg.join("givn/nli");
    seed_corrupt_snapshot(
        &bge_cache,
        "rozgo/bge-reranker-v2-m3",
        "fbd57b17b4db111a9d16813bb08b4c804fac18e9",
        &[
            "model.onnx",
            "model.onnx.data",
            "tokenizer.json",
            "config.json",
            "special_tokens_map.json",
            "tokenizer_config.json",
        ],
    );
    seed_corrupt_snapshot(
        &nli_cache,
        "MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7",
        "b5113eb38ab63efdd7f280f8c144ea8b13f978ce",
        &[
            "onnx/model.onnx",
            "config.json",
            "tokenizer.json",
            "tokenizer_config.json",
        ],
    );
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

fn seed_corrupt_snapshot(
    cache_root: &std::path::Path,
    repository: &str,
    revision: &str,
    files: &[&str],
) {
    let repository_dir = repository.replace('/', "--");
    let snapshot = cache_root.join(format!("models--{repository_dir}/snapshots/{revision}"));
    for file in files {
        let path = snapshot.join(file);
        fs::create_dir_all(path.parent().expect("artifact parent")).expect("create snapshot");
        fs::write(path, b"corrupt-artifact").expect("write corrupt artifact");
    }
    let refs = cache_root.join(format!("models--{repository_dir}/refs"));
    fs::create_dir_all(&refs).expect("create snapshot refs");
    fs::write(refs.join(revision), revision).expect("write snapshot ref");
    fs::create_dir_all(cache_root).expect("create artifact cache");
    fs::write(
        cache_root.join("givn-artifacts.json"),
        format!(
            "{{\"schema_version\":1,\"model\":\"{repository}\",\"repository\":\"{repository}\",\"revision\":\"{revision}\",\"files\":{{}}}}"
        ),
    )
    .expect("write corrupt artifact manifest");
}

#[then("stdout is valid duplicate-audit JSON with participating BGE and NLI evidence")]
fn duplicate_json_reports_participating_layers(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("duplicate output JSON");
    assert_eq!(document["bge_run"]["status"], "BGE_ENABLED");
    assert_eq!(document["nli_run"]["status"], "NLI_ENABLED");
}

#[then("the corrupt cache snapshots are replaced by complete validated snapshots")]
fn corrupt_caches_are_repaired(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    for role in ["bge", "nli"] {
        let manifest = fs::read_to_string(xdg.join(format!("givn/{role}/givn-artifacts.json")))
            .expect("read repaired artifact manifest");
        assert!(manifest.contains("\"files\""));
        assert!(!manifest.contains("\"files\":{}"));
    }
    let requests = world
        .artifact_server
        .as_ref()
        .expect("artifact fixture server")
        .events();
    assert!(
        requests
            .iter()
            .any(|path| path.contains("bge-reranker-v2-m3")),
        "BGE cache repair did not request the fixture source: {requests:?}"
    );
    assert!(
        requests
            .iter()
            .any(|path| path.contains("mDeBERTa-v3-base-xnli-multilingual-nli-2mil7")),
        "NLI cache repair did not request the fixture source: {requests:?}"
    );
}

#[given(
    "an initialized project with fresh BGE and NLI caches in the XDG Givn cache and reachable model sources"
)]
fn fresh_second_stage_caches(_world: &mut GivnWorld) {
    let world = _world;
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

#[given("BGE and NLI evidence are explicitly disabled")]
fn disable_second_stage_evidence(world: &mut GivnWorld) {
    assert_eq!(world.retrieval_bge.as_deref(), Some("disabled"));
    assert_eq!(world.retrieval_reranker.as_deref(), Some("disabled"));
}

#[then("the JSON contains the E5 candidates without BGE or NLI evidence")]
fn json_contains_e5_without_second_stage(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("duplicate output JSON");
    assert!(document["candidate_count"].as_u64().unwrap_or(0) > 0);
    assert!(document.get("bge_run").is_none());
    assert!(document.get("nli_run").is_none());
}

#[then("the BGE and NLI caches remain empty")]
fn second_stage_caches_remain_empty(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    assert!(!xdg.join("givn/bge").exists());
    assert!(!xdg.join("givn/nli").exists());
}

#[given("an initialized project with a completed permanent semantic index and a cold model cache")]
fn completed_index_and_cold_model_cache(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::create_dir_all(env.path("givn/specs/archive")).expect("create archive capability");
    fs::write(
        env.path("givn/specs/archive/archive.feature"),
        "Feature: archive\n\n  Scenario: Existing archived behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write archive permanent fixture");
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
        "build archive index failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let index = env.path("givn/.spec-index.json");
    world.permanent_index_snapshot = Some(fs::read(&index).expect("read archive index"));
    let model_cache = xdg.join("givn/e5");
    if model_cache.exists() {
        fs::remove_dir_all(model_cache).expect("clear cold E5 cache");
    }
    world.env = Some(env);
    world.xdg_cache_home = Some(xdg);
}

#[given("a completed change is ready to archive without semantic model artifacts")]
fn completed_change_without_model_artifacts(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("archive project");
    let change_dir = env.path("givn/changes/archive-download");
    let specs_dir = change_dir.join("specs/archive");
    fs::create_dir_all(&specs_dir).expect("create archive change specs");
    fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write archive proposal");
    fs::write(
        specs_dir.join("archive.feature"),
        "@givn.delta @archive\nFeature: archive\n\n  @givn.removed\n  Scenario: Existing archived behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write archive delta");
    fs::write(change_dir.join("design.md"), "# Design\n").expect("write archive design");
    fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write archive design review");
    fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n")
        .expect("write archive Arc42 marker");
    fs::write(
        change_dir.join("tasks.md"),
        "- [x] COMMIT: 0000000000000000000000000000000000000000 -- archive\n",
    )
    .expect("write archive tasks");
    fs::write(change_dir.join("review.md"), "REVIEW: PASS\n").expect("write archive review");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\naddons:\n  arc42: false\n  coverage: false\n",
    )
    .expect("write archive commands");
    world.last_change_id = Some("archive-download".to_string());
}

#[when("the user archives the completed change without model artifacts")]
fn archive_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("archive project");
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    let xdg_value = xdg.to_str().expect("XDG path is UTF-8");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["archive", "--change", "archive-download"],
        &[("XDG_CACHE_HOME", xdg_value)],
    ));
}

#[then("the command does not download model artifacts")]
fn archive_does_not_download_models(world: &mut GivnWorld) {
    let xdg = world.xdg_cache_home.as_ref().expect("XDG cache root");
    assert!(
        !xdg.join("givn/e5").exists(),
        "archive created an E5 model cache"
    );
}

#[then("the archive completes without a model-setup failure")]
fn archive_reuses_or_warns(_world: &mut GivnWorld) {
    let world = _world;
    assert!(
        super::output(world).status.success(),
        "archive failed: {}",
        super::stderr(world)
    );
    assert!(super::stdout(world).contains("Change archived successfully."));
}

#[given("a permanent corpus with deterministic semantic candidates for an independent NLI audit")]
fn independent_nli_audit_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("nli-pool.feature"),
        "Feature: NLI pool\n\n  Scenario: Alpha semantic behavior\n    Given a configured alpha catalog\n    When alpha is queried\n    Then alpha is returned\n\n  Scenario: Beta semantic behavior\n    Given a configured beta catalog\n    When beta is queried\n    Then beta is returned\n\n  Scenario: Gamma semantic behavior\n    Given a configured gamma catalog\n    When gamma is queried\n    Then gamma is returned\n",
    )
    .expect("write NLI pool fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_bge = Some("disabled".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/nli-pool.feature|Alpha semantic behavior::givn/specs/nli-pool.feature|Beta semantic behavior=0.9600;givn/specs/nli-pool.feature|Alpha semantic behavior::givn/specs/nli-pool.feature|Gamma semantic behavior=0.9600;givn/specs/nli-pool.feature|Beta semantic behavior::givn/specs/nli-pool.feature|Gamma semantic behavior=0.9600".to_string(),
    );
}

#[when("the user runs the permanent NLI duplicate audit")]
fn run_permanent_nli_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI audit project");
    let scores = world
        .retrieval_duplicate_scores
        .as_deref()
        .expect("NLI audit score overrides");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            ("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores),
        ],
    ));
}

#[then("NLI receives the full E5 semantic candidate pool independently of BGE")]
fn nli_receives_full_e5_pool(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("NLI E5 raw count: 3"), "stdout: {output}");
    assert!(
        output.contains("NLI unique pair count: 3"),
        "stdout: {output}"
    );
    assert!(
        !output.contains("BGE status:"),
        "BGE unexpectedly participated"
    );
}

#[then("the NLI audit retains the E5 candidate identities and ordering")]
fn nli_retains_e5_ordering(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let alpha = output
        .find("Alpha semantic behavior")
        .expect("alpha candidate in output");
    let beta = output
        .find("Beta semantic behavior")
        .expect("beta candidate in output");
    let gamma = output
        .find("Gamma semantic behavior")
        .expect("gamma candidate in output");
    assert!(
        alpha < beta && beta < gamma,
        "NLI E5 ordering changed: {output}"
    );
}

#[given("an initialized project with permanent semantic scenarios")]
fn feature_free_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::create_dir_all(env.path("givn/specs")).expect("create permanent specs");
    fs::write(
        env.path("givn/specs/feature-free.feature"),
        "Feature: feature free\n\n  Scenario: Retrieval behavior\n    Given a configured provider catalog\n    When the provider is queried\n    Then the provider result is returned\n",
    )
    .expect("write feature-free fixture");
    world.env = Some(env);
}

#[when("the user runs semantic search through a build without the retrieval capability")]
fn run_search_without_retrieval(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project");
    let manifest = env!("CARGO_MANIFEST_DIR");
    let target_dir = env.path(".cargo-target-no-retrieval");
    world.last_output = Some(
        Command::new("cargo")
            .current_dir(env.root())
            .args([
                "run",
                "--quiet",
                "--manifest-path",
                &format!("{manifest}/Cargo.toml"),
                "--no-default-features",
                "--bin",
                "givn",
                "--",
                "spec",
                "search",
                "semantic behavior",
            ])
            .env("CARGO_TARGET_DIR", target_dir)
            .stdin(Stdio::null())
            .output()
            .expect("run feature-free givn process"),
    );
}
