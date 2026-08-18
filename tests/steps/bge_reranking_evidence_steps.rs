//! Regular black-box steps for bounded BGE evidence.

use std::fs;
use std::process::Command;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

const APPROVAL_REPORT: &str = "bge-approval-report.json";
const APPROVAL_LABELS: &str = "bge-approval-labels.json";
const APPROVAL_PARITY: &str = "bge-rust-parity.json";
const APPROVAL_OUTPUT: &str = "givn/.spec-bge-admission.json";
const APPROVAL_MODEL: &str = "bge-approval-model";

fn approval_artifacts(env: &common::TestEnv) -> serde_json::Value {
    let cache = env.path(APPROVAL_MODEL);
    fs::create_dir_all(&cache).expect("create approval BGE model directory");
    fs::write(cache.join("model.onnx"), b"local model").expect("write approval model");
    for name in [
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        fs::write(cache.join(name), b"{}").expect("write approval tokenizer artifact");
    }
    serde_json::json!({
        "config.json": {
            "bytes": 2,
            "sha256": "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        },
        "model.onnx": {
            "bytes": 11,
            "sha256": "ffa838f6509a5a99bd5d39d5b9bd4243a9c586f359a2830c93603c1908d13a6e"
        },
        "special_tokens_map.json": {
            "bytes": 2,
            "sha256": "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        },
        "tokenizer.json": {
            "bytes": 2,
            "sha256": "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        },
        "tokenizer_config.json": {
            "bytes": 2,
            "sha256": "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a"
        }
    })
}

fn write_approval_benchmark(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let artifacts = approval_artifacts(&env);
    let report = serde_json::json!({
        "schema_version": 1,
        "candidate_scope": "bounded",
        "candidate_pipeline": "python-reference-v1",
        "top_k": 5,
        "bge_model": "rozgo/bge-reranker-v2-m3",
        "bge_threshold": 3.0,
        "bge_runtime_config": {
            "max_length_tokens": 512,
            "intra_op_threads": 4,
            "inter_op_threads": 1,
            "batch_size": 32,
            "max_unique_pairs": 512
        },
        "bge_artifacts": artifacts,
        "scenarios": [
            {"source": "givn", "file": "left.feature", "title": "Positive pair", "character_count": 10, "step_count": 3, "assertion_count": 1},
            {"source": "givn", "file": "left.feature", "title": "Positive twin", "character_count": 10, "step_count": 3, "assertion_count": 1},
            {"source": "givn", "file": "right.feature", "title": "Boundary pair", "character_count": 10, "step_count": 3, "assertion_count": 1},
            {"source": "givn", "file": "right.feature", "title": "Boundary twin", "character_count": 10, "step_count": 3, "assertion_count": 1}
        ],
        "stats": {
            "strict_pair_ids": [[0, 1], [2, 3]],
            "top_k_pairs": [[0, 1], [2, 3]],
            "bounded_pair_ids": [[0, 1], [2, 3]],
            "deterministic_pair_ids": []
        },
        "pairs": [
            {"left": 0, "right": 1, "embedding": 0.99, "bge_evidence": {"forward_raw": 4.0, "reverse_raw": 4.0, "symmetric_raw": 4.0}},
            {"left": 2, "right": 3, "embedding": 0.99, "bge_evidence": {"forward_raw": 1.0, "reverse_raw": 1.0, "symmetric_raw": 1.0}}
        ],
        "timing_seconds": {"bge": 0.001}
    });
    let labels = serde_json::json!({
        "schema_version": 1,
        "pairs": [
            {
                "left_source": "givn", "left_file": "left.feature", "left_title": "Positive pair",
                "right_source": "givn", "right_file": "left.feature", "right_title": "Positive twin",
                "label": "DUPLIKAT", "split": "holdout", "deterministic": false
            },
            {
                "left_source": "givn", "left_file": "right.feature", "left_title": "Boundary pair",
                "right_source": "givn", "right_file": "right.feature", "right_title": "Boundary twin",
                "label": "UNRELATED", "split": "holdout", "deterministic": false
            }
        ]
    });
    fs::write(
        env.path(APPROVAL_REPORT),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .expect("write approval benchmark report");
    fs::write(
        env.path(APPROVAL_LABELS),
        serde_json::to_vec_pretty(&labels).unwrap(),
    )
    .expect("write approval benchmark labels");
    world.env = Some(env);
}

fn approval_env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("approval fixture")
}

fn write_rust_parity(world: &mut GivnWorld) {
    let env = approval_env(world);
    let output = common::run_givn(env, &["spec", "bge-parity"]);
    assert!(
        output.status.success(),
        "bge parity failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::write(env.path(APPROVAL_PARITY), output.stdout).expect("write Rust parity attestation");
}

fn run_approval_writer(world: &mut GivnWorld, enforce: bool) {
    let env = approval_env(world);
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tools/approach-comparison/evaluate_bge_admission.py");
    let mut command = Command::new("python3");
    command
        .arg(script)
        .args(["--report", APPROVAL_REPORT, "--labels", APPROVAL_LABELS]);
    if enforce {
        command.args(["--enforce"]);
        if env.path(APPROVAL_PARITY).is_file() {
            command.args(["--rust-parity", APPROVAL_PARITY]);
        }
        command.args([
            "--givn-binary",
            env!("CARGO_BIN_EXE_givn"),
            "--bge-model",
            APPROVAL_MODEL,
            "--approval-file",
            APPROVAL_OUTPUT,
        ]);
    } else {
        command.args(["--approval-file", APPROVAL_OUTPUT]);
    }
    world.last_output = Some(
        command
            .current_dir(env.root())
            .output()
            .expect("run BGE approval writer"),
    );
}

#[when("the user runs exploration through a binary without the retrieval capability")]
fn run_no_retrieval_exploration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("retrieval compatibility fixture");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft request\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write no-retrieval draft");

    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = repository.join("target/bge-no-retrieval");
    let build = Command::new("cargo")
        .args([
            "build",
            "--no-default-features",
            "--bin",
            "givn",
            "--target-dir",
        ])
        .arg(&target_dir)
        .current_dir(repository)
        .output()
        .expect("build no-retrieval binary");
    assert!(
        build.status.success(),
        "no-retrieval build failed: {}",
        String::from_utf8_lossy(&build.stderr)
    );

    world.last_output = Some(
        Command::new(target_dir.join("debug/givn"))
            .args(["spec", "explore", "--file", "draft.feature"])
            .current_dir(env.root())
            .output()
            .expect("run no-retrieval exploration"),
    );
}

#[when("the user explores the same draft twice through the givn CLI")]
fn explore_same_draft_twice(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("repeated BGE exploration fixture");
    let environments = [("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    world.previous_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &environments,
    ));
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &environments,
    ));
}

#[given("a project with a permanent semantic candidate for second-stage evidence")]
fn bge_project(world: &mut GivnWorld) {
    super::spec_explore_permanent_steps::prepare_exploration_project(world);
    world.retrieval_reranker = Some("disabled".to_string());
}

#[given("a benchmark report with the Python reference candidate pipeline")]
fn python_reference_benchmark_report(world: &mut GivnWorld) {
    write_approval_benchmark(world);
}

#[given("a passing benchmark report without a Rust production parity attestation")]
fn passing_benchmark_without_parity(world: &mut GivnWorld) {
    write_approval_benchmark(world);
}

#[given("a passing benchmark report with a Rust production parity attestation")]
fn passing_benchmark_with_parity(world: &mut GivnWorld) {
    write_approval_benchmark(world);
    write_rust_parity(world);
}

#[given("a Rust production parity attestation emitted by `givn spec bge-parity`")]
fn rust_production_parity_attestation(world: &mut GivnWorld) {
    write_rust_parity(world);
}

#[given("a forged Rust production parity attestation")]
fn forged_rust_production_parity_attestation(world: &mut GivnWorld) {
    write_rust_parity(world);
    let env = approval_env(world);
    let mut parity: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_PARITY)).unwrap()).unwrap();
    parity["pipeline_fingerprint"] = serde_json::json!("0000000000000000");
    fs::write(
        env.path(APPROVAL_PARITY),
        serde_json::to_vec_pretty(&parity).unwrap(),
    )
    .expect("write forged Rust parity attestation");
}

#[given("a local BGE artifact set missing `special_tokens_map.json`")]
fn incomplete_approval_artifact_set(world: &mut GivnWorld) {
    let env = approval_env(world);
    fs::remove_file(env.path(&format!("{APPROVAL_MODEL}/special_tokens_map.json")))
        .expect("remove incomplete approval artifact");
}

#[given("a complete local BGE artifact set with matching hashes")]
fn complete_approval_artifact_set(world: &mut GivnWorld) {
    let env = approval_env(world);
    let model = env.path(APPROVAL_MODEL);
    for name in [
        "model.onnx",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        assert!(
            model.join(name).is_file(),
            "missing approval artifact {name}"
        );
    }
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_REPORT)).unwrap()).unwrap();
    assert_eq!(report["bge_artifacts"].as_object().unwrap().len(), 5);
}

#[when("the maintainer writes the BGE approval artifact with enforcement enabled")]
fn maintainer_writes_enforced_approval(world: &mut GivnWorld) {
    run_approval_writer(world, true);
}

#[when("the maintainer writes the BGE approval artifact without enforcement")]
fn maintainer_writes_unenforced_approval(world: &mut GivnWorld) {
    run_approval_writer(world, false);
}

#[then("the approval command exits non-zero")]
fn approval_command_exits_non_zero(world: &mut GivnWorld) {
    assert!(!super::output(world).status.success());
}

#[then("stderr says that Rust parity is required")]
fn approval_requires_rust_parity(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("Rust parity is required"),
        "stderr: {}",
        super::stderr(world)
    );
}

#[then("stderr says that `--approval-file` requires `--enforce`")]
fn approval_requires_enforcement(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("--approval-file requires --enforce"),
        "stderr: {}",
        super::stderr(world)
    );
}

#[then("stderr identifies the missing `special_tokens_map.json` artifact")]
fn approval_identifies_missing_artifact(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("special_tokens_map.json"),
        "stderr: {}",
        super::stderr(world)
    );
}

#[then("stderr says that the parity file does not match the compiled binary")]
fn approval_identifies_forged_parity(world: &mut GivnWorld) {
    assert!(
        super::stderr(world).contains("parity file does not match the compiled binary"),
        "stderr: {}",
        super::stderr(world)
    );
}

#[then("no BGE approval artifact is written")]
fn no_approval_artifact_written(world: &mut GivnWorld) {
    assert!(!approval_env(world).path(APPROVAL_OUTPUT).exists());
}

#[then("the approval artifact has exactly the documented top-level fields")]
fn approval_has_documented_fields(world: &mut GivnWorld) {
    let env = approval_env(world);
    let document: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_OUTPUT)).unwrap()).unwrap();
    let mut fields = document
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    fields.sort_unstable();
    assert_eq!(
        fields,
        vec![
            "acceptance",
            "artifact_hashes",
            "candidate_pipeline",
            "max_unique_pairs",
            "model",
            "pipeline_fingerprint",
            "rust_attestation",
            "schema_version",
            "threshold",
            "top_k",
        ]
    );
}

#[then("the approval artifact records the Rust pipeline fingerprint and all artifact hashes")]
fn approval_records_parity_and_hashes(world: &mut GivnWorld) {
    let env = approval_env(world);
    let approval: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_OUTPUT)).unwrap()).unwrap();
    let parity: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_PARITY)).unwrap()).unwrap();
    assert_eq!(
        approval["pipeline_fingerprint"],
        parity["pipeline_fingerprint"]
    );
    let hashes = approval["artifact_hashes"].as_object().unwrap();
    assert_eq!(hashes.len(), 5);
    for metadata in hashes.values() {
        assert!(metadata["bytes"].as_u64().unwrap() > 0);
        assert_eq!(metadata["sha256"].as_str().unwrap().len(), 64);
    }
}

#[then("the approval artifact records every benchmark acceptance boolean")]
fn approval_records_acceptance_booleans(world: &mut GivnWorld) {
    let env = approval_env(world);
    let approval: serde_json::Value =
        serde_json::from_slice(&fs::read(env.path(APPROVAL_OUTPUT)).unwrap()).unwrap();
    let acceptance = &approval["acceptance"];
    for name in [
        "holdout_recall_pass",
        "false_positive_reduction_pass",
        "precision_pass",
        "false_negative_delta_pass",
        "unscored_positive_pass",
        "candidate_pipeline_pass",
        "pass",
    ] {
        assert!(
            acceptance[name].is_boolean(),
            "missing acceptance boolean {name}"
        );
    }
}

#[given("a project with one E5 candidate and a complete local BGE artifact set")]
fn complete_local_bge_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write local BGE permanent fixture");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("local BGE delta parent"))
        .expect("create local BGE delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write local BGE delta fixture");

    let cache = env.path("givn/.spec-reranker-cache");
    fs::create_dir_all(&cache).expect("create complete local BGE cache");
    fs::write(cache.join("model.onnx"), b"local test model").expect("write local BGE model");
    fs::write(cache.join("tokenizer.json"), b"{}").expect("write local BGE tokenizer");
    for name in [
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        fs::write(cache.join(name), b"{}").expect("write local BGE metadata");
    }

    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("local".to_string());
    world.retrieval_bge_local_stub = true;
}

#[given("the local second-stage evidence is disabled")]
fn bge_disabled(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("disabled".to_string());
}

#[given("second-stage evidence is disabled")]
fn bge_disabled_alias(world: &mut GivnWorld) {
    bge_disabled(world);
}

#[given("the local second-stage evidence is enabled")]
fn bge_enabled(world: &mut GivnWorld) {
    world.retrieval_reranker = None;
}

#[given("the local second-stage evidence is enabled without NLI evidence")]
fn bge_enabled_without_nli(world: &mut GivnWorld) {
    world.retrieval_reranker = None;
}

#[given("the independent NLI evidence capability is enabled")]
fn nli_capability_enabled(world: &mut GivnWorld) {
    advisory_bge_project(world);
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("the legacy NLI selector is explicitly selected")]
fn legacy_nli_selector_selected(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("the local second-stage model artifacts are unavailable")]
fn bge_artifacts_missing(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("missing".to_string());
}

#[given("a project with a corrupt local second-stage model cache")]
fn corrupt_bge_cache_project(world: &mut GivnWorld) {
    bge_project(world);
    let env = world.env.as_ref().expect("BGE project");
    let cache = env.path("givn/.spec-reranker-cache");
    fs::create_dir_all(&cache).expect("create corrupt BGE cache");
    fs::write(cache.join("model.onnx"), b"not-an-onnx-model").expect("write corrupt model");
    world.retrieval_reranker = Some("corrupt".to_string());
}

#[given("local second-stage artifact mode is selected")]
fn local_bge_artifact_mode(world: &mut GivnWorld) {
    if world.retrieval_reranker.is_none() {
        world.retrieval_reranker = Some("local".to_string());
    }
}

#[given("the legacy NLI selector is set without enabling the independent capability")]
fn legacy_nli_selector_without_capability(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("a complete draft scenario is available for permanent exploration")]
fn complete_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft provider request\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write complete draft scenario");
}

#[given("a complete draft scenario is available for change-aware exploration")]
fn complete_change_aware_draft(world: &mut GivnWorld) {
    complete_permanent_draft(world);
}

#[when("the user explores the draft through the givn CLI")]
fn explore_permanent_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
        ],
    ));
}

#[when("the user explores the draft with JSON output")]
fn explore_permanent_draft_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--file",
            "draft.feature",
            "--format",
            "json",
        ],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
        ],
    ));
}

#[when("the user explores the draft once with second-stage evidence disabled")]
fn explore_disabled_for_parity(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.previous_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
        ],
    ));
}

#[when("the user explores the same draft once with second-stage evidence enabled")]
fn explore_enabled_for_parity(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")],
    ));
}

#[when("the user explores the draft with second-stage evidence enabled")]
fn explore_enabled_with_legacy_nli_selector(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    let bge_mode = if world.retrieval_reranker.as_deref() == Some("missing") {
        "missing"
    } else if world.retrieval_reranker.as_deref() == Some("corrupt") {
        "local"
    } else {
        "stub"
    };
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)];
    if world.retrieval_reranker.as_deref() == Some("unapproved") {
        environments.push(("GIVN_SPEC_BGE_APPROVAL", "failed"));
    }
    if world.retrieval_reranker.as_deref() == Some("nli-stub") {
        environments.push(("GIVN_SPEC_RERANKER", "nli-stub"));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &environments,
    ));
}

#[when("the user runs advisory change review through the NLI-capable CLI")]
fn run_nli_advisory_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
        ],
    ));
}

#[when("the user explores the draft with JSON output and second-stage evidence enabled")]
fn explore_json_with_bge(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("BGE project is required");
    let bge_mode = if world.retrieval_reranker.as_deref() == Some("missing") {
        "missing"
    } else {
        "stub"
    };
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--file",
            "draft.feature",
            "--format",
            "json",
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)],
    ));
}

#[then("the result set contains the E5 candidates")]
fn result_set_contains_e5_candidates(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("rank 1:"),
        "expected an E5-ranked result: {}",
        super::stdout(world)
    );
}

#[then("stdout identifies the E5-only configuration")]
fn stdout_identifies_e5_only(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("model: intfloat/multilingual-e5-large")
            || super::stdout(world).contains("E5-only configuration"),
        "stdout did not identify E5: {}",
        super::stdout(world)
    );
}

#[then("stdout contains no participating second-stage model, score, or runtime")]
fn stdout_has_no_bge_participation(world: &mut GivnWorld) {
    let output = super::stdout(world).to_ascii_lowercase();
    assert!(
        !output.contains("bge"),
        "stdout unexpectedly contains BGE evidence: {output}"
    );
}

#[then("the JSON retains the existing E5 query, index, and result contract")]
fn json_retains_e5_contract(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid explore JSON");
    assert!(document.get("query").is_some());
    assert!(document.get("index").is_some());
    assert!(document.get("results").is_some());
}

#[then("the JSON contains no BGE evidence object or participating model")]
fn json_has_no_bge_evidence(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid explore JSON");
    assert!(document.get("evidence").is_none());
    assert!(!super::stdout(world).to_ascii_lowercase().contains("bge"));
}

#[given("a project with one permanent semantic pair and directional local evidence scores")]
fn directional_bge_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("pair.feature"),
        "Feature: pair\n\n  Scenario: Alpha request\n    Given an alpha context\n    When alpha runs\n    Then alpha succeeds\n\n  Scenario: Beta request\n    Given a beta context\n    When beta runs\n    Then beta succeeds\n",
    )
    .expect("write directional BGE fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/pair.feature|Alpha request::givn/specs/pair.feature|Beta request=0.9600"
            .to_string(),
    );
    world.retrieval_bge_scores = Some(
        "givn/specs/pair.feature|Alpha request->givn/specs/pair.feature|Beta request=2.0000;givn/specs/pair.feature|Beta request->givn/specs/pair.feature|Alpha request=4.0000".to_string(),
    );
}

#[when("the user runs the permanent duplicate audit with second-stage evidence enabled")]
fn run_directional_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("directional BGE project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_duplicate_scores.as_deref() {
        environments.push(("GIVN_SPEC_DUPLICATES_TEST_SCORES", scores));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    if let Some(pool) = world.retrieval_bge_test_pool.as_deref() {
        environments.push(("GIVN_SPEC_BGE_TEST_POOL", pool));
    }
    let json = world.retrieval_bge_test_pool.as_deref() != Some("overflow-513")
        && world.retrieval_bge_test_pool.as_deref() != Some("bounded-512");
    let args = if json {
        vec!["spec", "duplicates", "--format", "json"]
    } else {
        vec!["spec", "duplicates"]
    };
    world.last_output = Some(common::run_givn_with_envs(env, &args, &environments));
}

#[when(
    "the user runs the permanent duplicate audit with JSON output and second-stage evidence enabled"
)]
fn run_partial_duplicate_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial duplicate project");
    let bge_mode = if world.retrieval_reranker.as_deref() == Some("missing") {
        "missing"
    } else {
        "stub"
    };
    let approval_failed = world.retrieval_reranker.as_deref() == Some("unapproved");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)];
    if approval_failed {
        environments.push(("GIVN_SPEC_BGE_APPROVAL", "failed"));
    }
    environments.push((
        "GIVN_SPEC_DUPLICATES_TEST_SCORES",
        "givn/specs/pair.feature|Alpha request::givn/specs/pair.feature|Beta request=0.9600",
    ));
    environments.push(("GIVN_SPEC_BGE_TEST_POOL", "partial-raw-513"));
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &environments,
    ));
}

#[then("the audit reports both directional scores")]
fn audit_reports_directional_scores(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    let candidate = &document["candidates"][0];
    assert_eq!(
        candidate["evidence"]["bge"]["forward_raw"],
        serde_json::json!(2.0)
    );
    assert_eq!(
        candidate["evidence"]["bge"]["reverse_raw"],
        serde_json::json!(4.0)
    );
}

#[then("the reported raw score is their documented symmetric average")]
fn audit_reports_symmetric_score(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON");
    assert_eq!(
        document["candidates"][0]["evidence"]["bge"]["symmetric_raw"],
        serde_json::json!(3.0)
    );
}

#[then("reversing endpoint order does not change the reported raw score")]
fn reversed_direction_is_symmetric(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("directional BGE project");
    let output = common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/pair.feature|Alpha request::givn/specs/pair.feature|Beta request=0.9600",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/pair.feature|Beta request->givn/specs/pair.feature|Alpha request=4.0000;givn/specs/pair.feature|Alpha request->givn/specs/pair.feature|Beta request=2.0000",
            ),
        ],
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(
        document["candidates"][0]["evidence"]["bge"]["symmetric_raw"],
        serde_json::json!(3.0)
    );
}

#[given("a permanent corpus containing exact, structural, and semantic relationships")]
fn deterministic_exclusion_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("exact.feature"),
        "Feature: exact\n\n  Scenario: Exact left\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Exact right\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write exact fixture");
    fs::write(
        specs.join("structural.feature"),
        "Feature: structural\n\n  Scenario: Structural left\n    Given a configured provider catalog\n    When the provider command is run normally\n    Then the command exits 0 and output contains ready\n\n  @givn.done\n  Scenario: Structural right\n    Given a configured provider catalog\n    When the provider command is run in TTY mode\n    Then the command exits 2 and output does not contain ready\n",
    )
    .expect("write structural fixture");
    fs::write(
        specs.join("semantic.feature"),
        "Feature: semantic\n\n  Scenario: Token is required\n    Given a client request\n    When the client sends a request without credentials\n    Then the server responds with unauthorized status\n\n  Scenario: Authentication needs a token\n    Given an identity service\n    When the client authenticates without a token\n    Then access is denied with token required\n",
    )
    .expect("write semantic fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_duplicate_scores = Some(
        "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600".to_string(),
    );
    world.retrieval_bge_scores = Some(
        "givn/specs/semantic.feature|Token is required->givn/specs/semantic.feature|Authentication needs a token=4.0000;givn/specs/semantic.feature|Authentication needs a token->givn/specs/semantic.feature|Token is required=4.0000".to_string(),
    );
}

#[when("the user runs the permanent duplicate audit with JSON output")]
fn run_deterministic_exclusion_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("deterministic exclusion project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/semantic.feature|Token is required->givn/specs/semantic.feature|Authentication needs a token=4.0000;givn/specs/semantic.feature|Authentication needs a token->givn/specs/semantic.feature|Token is required=4.0000",
            ),
        ],
    ));
}

#[then("exact and structural relationships retain their deterministic evidence")]
fn deterministic_candidates_have_no_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    for candidate in document["candidates"].as_array().unwrap() {
        if candidate["kind"] == "exact_duplicate" || candidate["kind"] == "structural_overlap" {
            assert!(candidate["evidence"]["bge"].is_null());
        }
    }
}

#[then("only the eligible semantic neighbour union has second-stage evidence")]
fn only_semantic_candidates_have_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    for collection in ["candidates", "filtered_candidates", "unscored_candidates"] {
        if let Some(candidates) = document[collection].as_array() {
            for candidate in candidates {
                if !candidate["evidence"]["bge"].is_null() {
                    assert_eq!(candidate["kind"], "semantic_candidate");
                }
            }
        }
    }
}

#[then("no deterministic exclusion is counted as a second-stage pair")]
fn deterministic_pairs_excluded_from_bge(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(1));
}

#[given("a project with two E5 candidates and one labeled false positive")]
fn advisory_bge_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write advisory permanent fixture");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().unwrap()).expect("create advisory delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write advisory delta");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[given("the local second-stage evidence gives the false positive a low relevance score")]
fn advisory_bge_scores(world: &mut GivnWorld) {
    world.retrieval_bge_scores = Some(
        "givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0000;givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=4.0000;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider warning is reported=1.0000;givn/specs/provider.feature|Provider warning is reported->givn/specs/running-change/running-change.feature|Active provider request=1.0000".to_string(),
    );
}

#[given("a project with one E5 candidate whose symmetric second-stage score is exactly `3.0000`")]
fn exact_threshold_advisory_project(world: &mut GivnWorld) {
    advisory_bge_project(world);
    world.retrieval_bge_scores = Some(
        "givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=3.0000;givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=3.0000;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider warning is reported=1.0000;givn/specs/provider.feature|Provider warning is reported->givn/specs/running-change/running-change.feature|Active provider request=1.0000".to_string(),
    );
}

#[given("a project with one E5 candidate and malformed directional local evidence scores")]
fn malformed_directional_project(world: &mut GivnWorld) {
    advisory_bge_project(world);
    world.retrieval_bge_scores = Some("malformed-entry".to_string());
}

#[when("the user runs advisory change review with second-stage evidence enabled")]
fn run_advisory_bge_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("advisory BGE project");
    let bge_mode = match world.retrieval_reranker.as_deref() {
        Some("missing") => "missing",
        Some("local") | Some("corrupt") => "local",
        _ => "stub",
    };
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)];
    if world.retrieval_reranker.as_deref() == Some("unapproved") {
        environments.push(("GIVN_SPEC_BGE_APPROVAL", "failed"));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    if world.retrieval_bge_local_stub {
        environments.push(("GIVN_SPEC_BGE_LOCAL_STUB", "1"));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &environments,
    ));
}

#[given("a project with an active semantic review candidate")]
fn active_advisory_project(world: &mut GivnWorld) {
    advisory_bge_project(world);
}

#[given("the BGE admission benchmark has failed")]
fn failed_bge_approval(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("unapproved".to_string());
}

#[then("stdout remains advisory and does not assign a Givn classification")]
fn unavailable_advisory_is_non_classifying(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("advisory"));
    assert!(!output.contains("DUPLIKAT"));
    assert!(!output.contains("VARIANTE"));
    assert!(!output.contains("VALID-BOUNDARY"));
}

#[then("stdout contains no `BGE_FILTERED` evidence")]
fn no_filtered_when_unapproved(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("BGE_FILTERED"));
}

#[then("stderr identifies the malformed directional evidence entry")]
fn malformed_directional_error(world: &mut GivnWorld) {
    assert!(super::stderr(world).contains("malformed directional evidence entry"));
}

#[then("stderr reports zero BGE model load attempts")]
fn malformed_directional_no_load(world: &mut GivnWorld) {
    assert!(super::stderr(world).contains("BGE model load attempts: 0"));
}

#[then("the false positive is recorded as filtered from the recommended list")]
fn false_positive_is_filtered(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE_FILTERED"));
}

#[then(
    "the false positive is not classified as `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`"
)]
fn filtered_candidate_has_no_classification(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for classification in ["DUPLIKAT", "VARIANTE", "VALID-BOUNDARY", "UNCERTAIN"] {
        assert!(
            !output.contains(classification),
            "filtered output contains {classification}"
        );
    }
}

#[then("the remaining recommended candidate retains its E5 and second-stage evidence")]
fn recommended_candidate_retains_evidence(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE_RECOMMENDED"));
    assert!(output.contains("E5 "));
    assert!(output.contains("symmetric"));
}

#[given("a project with only E5 candidates that receive filtered second-stage evidence")]
fn filtered_blocking_project(world: &mut GivnWorld) {
    advisory_bge_project(world);
    let env = world.env.as_ref().expect("advisory fixture");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write blocking commands");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Active provider request` | `Provider request is accepted` | related |\n| `Active provider request` | `Provider warning is reported` | related |\n",
    )
    .expect("write blocking review disposition");
    world.retrieval_bge_scores = Some(
        "givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=1.0000;givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=1.0000;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider warning is reported=1.0000;givn/specs/provider.feature|Provider warning is reported->givn/specs/running-change/running-change.feature|Active provider request=1.0000".to_string(),
    );
}

#[given(
    "a project with exactly 513 E5 raw semantic pairs and a successful bounded top-K union of exactly 512 pairs"
)]
fn partial_unscored_blocking_project(world: &mut GivnWorld) {
    filtered_blocking_project(world);
    world.retrieval_bge_test_pool = Some("partial-raw-513".to_string());
}

#[given(
    "a permanent corpus with exactly 513 E5 raw semantic pairs and a successful bounded top-K union of exactly 512 pairs"
)]
fn partial_unscored_duplicate_project(world: &mut GivnWorld) {
    directional_bge_project(world);
    world.retrieval_bge_test_pool = Some("partial-raw-513".to_string());
}

#[given("a permanent corpus with semantic candidates and unavailable second-stage artifacts")]
fn unavailable_duplicate_project(world: &mut GivnWorld) {
    directional_bge_project(world);
    world.retrieval_reranker = Some("missing".to_string());
}

#[given("a permanent corpus with semantic candidates")]
fn semantic_duplicate_project_alias(world: &mut GivnWorld) {
    directional_bge_project(world);
}

#[given("an end-to-end project with an unresolved blocking semantic candidate")]
fn unavailable_blocking_project(world: &mut GivnWorld) {
    filtered_blocking_project(world);
    let env = world.env.as_ref().expect("blocking project");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request with a warning\n",
    )
    .expect("write close semantic blocking candidate");
    world.permanent_spec_snapshot = Some(vec![(
        "givn/specs/provider.feature".to_string(),
        fs::read(env.path("givn/specs/provider.feature")).expect("snapshot permanent specs"),
    )]);
    world.active_change_snapshot = Some(vec![(
        "givn/changes/running-change/specs/running.feature".to_string(),
        fs::read(env.path("givn/changes/running-change/specs/running.feature"))
            .expect("snapshot active specs"),
    )]);
    world.retrieval_bge_scores = Some(
        "givn/changes/running-change/specs/running.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0000;givn/specs/provider.feature|Provider request is accepted->givn/changes/running-change/specs/running.feature|Active provider request=4.0000;givn/changes/running-change/specs/running.feature|Active provider request->givn/specs/provider.feature|Provider warning is reported=1.0000;givn/specs/provider.feature|Provider warning is reported->givn/changes/running-change/specs/running.feature|Active provider request=1.0000".to_string(),
    );
    world.retrieval_stub_scores = Some(
        "Active provider request|Provider request is accepted=0.9900;Active provider request|Provider warning is reported=0.9900".to_string(),
    );
    world.retrieval_reranker = Some("missing".to_string());
}

#[given("a project with permanent and active-change semantic candidates")]
fn unavailable_change_explore_project(world: &mut GivnWorld) {
    advisory_bge_project(world);
    let env = world.env.as_ref().expect("change exploration project");
    let current = env.path("givn/changes/current-change/specs/current.feature");
    fs::create_dir_all(current.parent().unwrap()).expect("create current change");
    fs::write(
        &current,
        "@givn.delta @current-change\nFeature: current\n\n  @givn.added\n  Scenario: Current provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write current-change fixture");
    world.retrieval_reranker = Some("missing".to_string());
}

#[when(
    "the user explores the draft against change \"current-change\" with second-stage evidence enabled"
)]
fn run_unavailable_change_exploration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("change exploration project");
    let bge_mode = if world.retrieval_reranker.as_deref() == Some("unapproved") {
        "stub"
    } else {
        "missing"
    };
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)];
    if world.retrieval_reranker.as_deref() == Some("unapproved") {
        environments.push(("GIVN_SPEC_BGE_APPROVAL", "failed"));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--change",
            "current-change",
            "--file",
            "draft.feature",
        ],
        &environments,
    ));
}

#[when("the user explores the draft against change \"current-change\" through the givn CLI")]
fn run_unapproved_change_exploration(world: &mut GivnWorld) {
    run_unavailable_change_exploration(world);
}

#[then("stdout reports BGE status \"BGE_UNAVAILABLE\"")]
fn unavailable_change_reports_status(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE status: BGE_UNAVAILABLE"));
}

#[then("stdout retains the permanent and current-change E5 raw candidates")]
fn unavailable_change_retains_candidates(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("scope permanent"));
    assert!(output.contains("scope current-change"));
}

#[given(
    "an end-to-end project with an unresolved blocking semantic candidate and a 513-pair BGE pool"
)]
fn pair_cap_blocking_project(world: &mut GivnWorld) {
    filtered_blocking_project(world);
    world.retrieval_bge_test_pool = Some("overflow-513".to_string());
}

#[when(
    "the end-to-end user runs blocking change review for \"running-change\" through the givn CLI"
)]
fn run_unavailable_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable blocking project");
    let bge_mode = if world.retrieval_reranker.as_deref() == Some("missing") {
        "missing"
    } else {
        "stub"
    };
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", bge_mode)];
    if let Some(pool) = world.retrieval_bge_test_pool.as_deref() {
        environments.push(("GIVN_SPEC_BGE_TEST_POOL", pool));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    if let Some(scores) = world.retrieval_stub_scores.as_deref() {
        environments.push(("GIVN_SPEC_STUB_PAIR_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &environments,
    ));
}

fn semantic_report_text(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("semantic project");
    fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report")
}

#[then("the command exits non-zero because the E5 candidate is unresolved")]
fn unavailable_blocking_exits_nonzero(world: &mut GivnWorld) {
    assert!(!super::output(world).status.success());
}

#[then("the semantic review report has BGE status \"BGE_UNAVAILABLE\"")]
fn unavailable_report_status(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("BGE status: BGE_UNAVAILABLE"));
}

#[then("the semantic review report has BGE reason \"missing-artifact\"")]
fn unavailable_report_reason(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("BGE reason: missing-artifact"));
}

#[then("the semantic review report has BGE reason \"pair-cap\"")]
fn pair_cap_report_reason(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("BGE reason: pair-cap"));
}

#[then(
    "the semantic report has zero BGE unique pairs, directional calls, filtered pairs, and model load attempts"
)]
fn unavailable_report_zero_counts(world: &mut GivnWorld) {
    let report = semantic_report_text(world);
    assert!(report.contains("BGE unique pair count: 0"));
    assert!(report.contains("BGE directional call count: 0"));
    assert!(report.contains("BGE filtered count: 0"));
    assert!(report.contains("BGE model load attempts: 0"));
}

#[then("the semantic review report contains a normal E5 candidate block")]
fn unavailable_report_normal_candidate(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("## Candidate"));
}

#[then("the report contains no `BGE_FILTERED` candidate block")]
fn unavailable_report_no_filtered(world: &mut GivnWorld) {
    assert!(!semantic_report_text(world).contains("## BGE Filtered Candidate"));
}

#[then("the semantic report retains the E5 candidate block without `BGE_FILTERED`")]
fn pair_cap_retains_e5_block(world: &mut GivnWorld) {
    let report = semantic_report_text(world);
    assert!(report.contains("## Candidate"));
    assert!(!report.contains("## BGE Filtered Candidate"));
}

#[then("the semantic review report uses schema version 1")]
fn e5_only_report_schema(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("Report schema version: 1"));
}

#[then("the report contains no BGE status, model, score, or admission fields")]
fn e5_only_report_has_no_bge_fields(world: &mut GivnWorld) {
    let report = semantic_report_text(world);
    assert!(!report.contains("BGE status:"));
    assert!(!report.contains("bge_"));
}

#[then("the unresolved E5 candidate remains blocking")]
fn e5_only_report_remains_blocking(world: &mut GivnWorld) {
    assert!(semantic_report_text(world).contains("classification: UNCERTAIN"));
}

#[given("a project with an unresolved blocking semantic candidate")]
fn blocking_project_alias(world: &mut GivnWorld) {
    unavailable_blocking_project(world);
}

#[when("the user runs blocking change review with second-stage evidence enabled")]
fn run_filtered_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("filtered blocking project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if world.retrieval_reranker.as_deref() == Some("unapproved") {
        environments.push(("GIVN_SPEC_BGE_APPROVAL", "failed"));
    }
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    if let Some(pool) = world.retrieval_bge_test_pool.as_deref() {
        environments.push(("GIVN_SPEC_BGE_TEST_POOL", pool));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &environments,
    ));
}

#[when("the user runs blocking change review with second-stage evidence disabled")]
fn run_e5_only_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5-only blocking project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
        ],
    ));
}

#[then("the semantic review report records the E5 candidates as filtered")]
fn semantic_report_records_filtered(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("filtered blocking project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains("BGE_FILTERED"));
}

#[then("the filtered candidates have no Givn classification")]
fn filtered_report_has_no_classification(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("filtered blocking project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    for block in report.split("## BGE Filtered Candidate").skip(1) {
        let block = block.split("## ").next().unwrap_or(block);
        assert!(!block.contains("classification:"));
    }
}

#[then("the report status is \"PASS\"")]
fn semantic_report_status_pass(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("filtered blocking project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains("Status: PASS"));
}

#[then("the semantic review report records BGE status \"BGE_ENABLED\"")]
fn partial_report_enabled(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(report.contains("BGE status: BGE_ENABLED"));
}

#[then("the semantic review report records exactly 512 unique pairs and 1024 directional calls")]
fn partial_report_counts(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(report.contains("BGE unique pair count: 512"));
    assert!(report.contains("BGE directional call count: 1024"));
}

#[then("exactly one E5 candidate outside the bounded set is recorded as `BGE_UNSCORED`")]
fn partial_report_one_unscored(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(report.contains("BGE unscored count: 1"));
    assert!(report.contains("bge_admission: BGE_UNSCORED"));
}

#[then("the semantic review report records the outside candidates as `BGE_UNSCORED`")]
fn partial_report_records_outside(world: &mut GivnWorld) {
    partial_report_one_unscored(world);
}

#[then("those candidates remain blocking E5 candidates")]
fn partial_candidates_remain_blocking(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(report.contains("Candidate count: 1"));
}

#[then(
    "each unscored candidate block has null BGE directional scores and the E5 rank/endpoint order"
)]
fn partial_unscored_fields_are_null(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(report.contains("bge_forward_raw: null"));
    assert!(report.contains("bge_reverse_raw: null"));
    assert!(report.contains("bge_symmetric_raw: null"));
}

#[then("no unscored candidate is labeled `BGE_FILTERED`")]
fn partial_unscored_not_filtered(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(!report.contains("BGE_UNSCORED\n") || !report.contains("BGE_FILTERED"));
}

#[then("no unscored candidate is present in the filtered evidence collection")]
fn partial_no_filtered_collection(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("partial blocking project");
    let report =
        fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md")).unwrap();
    assert!(!report.contains("## BGE Filtered Candidate"));
}

#[then(
    "the JSON reports BGE status \"BGE_ENABLED\" with 512 unique pairs and 1024 directional calls"
)]
fn partial_json_counts(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert_eq!(
        document["bge_run"]["status"],
        serde_json::json!("BGE_ENABLED")
    );
    assert_eq!(document["bge_run"]["unique_pairs"], serde_json::json!(512));
    assert_eq!(
        document["bge_run"]["directional_calls"],
        serde_json::json!(1024)
    );
}

#[then("the JSON contains an `unscored_candidates` collection")]
fn partial_json_has_unscored_collection(world: &mut GivnWorld) {
    assert!(
        super::output(world)
            .stdout
            .windows(b"unscored_candidates".len())
            .any(|window| window == b"unscored_candidates")
    );
}

#[then("each unscored JSON candidate has an endpoint, E5 score, and status `BGE_UNSCORED`")]
fn partial_json_unscored_fields(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let candidate = &document["unscored_candidates"][0];
    assert!(!candidate["left"].is_null());
    assert!(!candidate["evidence"]["score"].is_null());
    assert_eq!(candidate["evidence"]["bge"], serde_json::Value::Null);
}

#[then("unscored JSON candidates use descending E5 score and endpoint tuple order")]
fn partial_json_unscored_order(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let candidates = document["unscored_candidates"].as_array().unwrap();
    let scores = candidates
        .iter()
        .map(|candidate| candidate["evidence"]["score"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(scores.windows(2).all(|window| window[0] >= window[1]));
}

#[then("each unscored candidate remains outside the filtered collection")]
fn partial_json_unscored_outside_filtered(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(
        document["filtered_candidates"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[then("the JSON contains no unscored candidate with BGE directional scores")]
fn partial_json_unscored_no_directionals(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    for candidate in document["unscored_candidates"].as_array().unwrap() {
        assert!(candidate["evidence"]["bge"].is_null());
    }
}

#[then("the candidate is recommended rather than filtered")]
fn exact_threshold_is_recommended(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE_RECOMMENDED"));
    assert!(output.contains("symmetric 3.0000"));
}

#[then("the candidate still requires an explicit Givn classification")]
fn exact_threshold_still_advisory(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("no Givn classification is assigned"));
}

#[given("an end-to-end project with an active semantic review candidate")]
fn gate_combination_project(world: &mut GivnWorld) {
    super::semantic_review_steps::two_similar_active_deltas(world);
    world.retrieval_reranker = None;
    world.retrieval_bge_scores = Some(
        "givn/changes/running-change/specs/first.feature|Unknown provider is rejected->givn/specs/retrieval.feature|Provider name is unknown=1.0000;givn/specs/retrieval.feature|Provider name is unknown->givn/changes/running-change/specs/first.feature|Unknown provider is rejected=1.0000;givn/changes/running-change/specs/first.feature|Unknown provider is rejected->givn/specs/retrieval.feature|Authentication token is missing=1.0000;givn/specs/retrieval.feature|Authentication token is missing->givn/changes/running-change/specs/first.feature|Unknown provider is rejected=1.0000;givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected->givn/specs/retrieval.feature|Provider name is unknown=1.0000;givn/specs/retrieval.feature|Provider name is unknown->givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected=1.0000;givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected->givn/specs/retrieval.feature|Authentication token is missing=1.0000;givn/specs/retrieval.feature|Authentication token is missing->givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected=1.0000;givn/changes/running-change/specs/first.feature|Unknown provider is rejected->givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected=1.0000;givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected->givn/changes/running-change/specs/first.feature|Unknown provider is rejected=1.0000".to_string(),
    );
}

#[when(
    "the user runs blocking change review with the E5 similarity gate and second-stage evidence enabled"
)]
fn run_gate_combination_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("gate combination project");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")];
    if let Some(scores) = world.retrieval_bge_scores.as_deref() {
        environments.push(("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &environments,
    ));
}

#[then("the command reports the E5 gate result separately from BGE admission")]
fn gate_combination_reports_both(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("similarity gate"),
        "stdout: {output}\nstderr: {}",
        super::stderr(world)
    );
    assert!(output.contains("BGE"), "stdout: {output}");
}

#[then("a BGE status does not replace the E5 gate result")]
fn gate_combination_keeps_e5_gate(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("similarity gate"));
    assert!(output.contains("BGE:"));
}

#[then("no BGE status is rendered as a Givn classification")]
fn gate_combination_no_bge_classification(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("classification: BGE_"));
}

#[when("the user runs blocking change review with the E5 similarity gate and BGE disabled")]
fn run_gate_only_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("gate-only project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Unknown provider is rejected|Provider name is unknown=0.0000;Unknown provider selection is rejected|Provider name is unknown=0.0000;Unknown provider is rejected|Unknown provider selection is rejected=0.0000",
            ),
        ],
    ));
}

#[then("the command reports the E5 gate result")]
fn gate_only_reports_e5(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("similarity gate"));
}

#[then("the output contains no participating BGE model or score")]
fn gate_only_has_no_bge(world: &mut GivnWorld) {
    let output = super::stdout(world).to_ascii_lowercase();
    assert!(!output.contains("bge"));
}

#[then("stdout reports BGE unavailable reason \"missing-artifact\"")]
fn missing_bge_reports_reason(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("reason missing-artifact"));
}

#[then(
    "stdout reports zero unique pairs, directional calls, recommended pairs, filtered pairs, and model load attempts"
)]
fn missing_bge_reports_zero_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("unique pairs 0"));
    assert!(output.contains("directional calls 0"));
    assert!(output.contains("recommended 0"));
    assert!(output.contains("filtered 0"));
    assert!(output.contains("model load attempts 0"));
}

#[then("stdout does not claim that the second-stage model participated")]
fn missing_bge_has_no_model_claim(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("BGE model: rozgo/bge-reranker-v2-m3"));
}

#[then("stdout reports reason \"missing-artifact\"")]
fn corrupt_bge_reports_reason(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("reason missing-artifact"));
}

#[then("stdout reports zero model load attempts")]
fn corrupt_bge_reports_no_load(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("model load attempts 0"));
}

#[given(
    "a modified active scenario shares its projected target file with another permanent scenario"
)]
fn same_file_identity_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs/shared");
    fs::create_dir_all(&specs).expect("create shared specs");
    fs::write(
        specs.join("shared.feature"),
        "Feature: shared\n\n  Scenario: Modified scenario\n    Given a shared context\n    When the shared action runs\n    Then the shared result is ready\n\n  Scenario: Same-file neighbor\n    Given a shared context\n    When the neighbor action runs\n    Then the shared result is ready\n",
    )
    .expect("write same-file permanent fixture");
    let delta = env.path("givn/changes/running-change/specs/change.feature");
    fs::create_dir_all(delta.parent().unwrap()).expect("create same-file delta");
    fs::write(
        &delta,
        "@givn.delta @shared @running-change\nFeature: shared change\n\n  @givn.modified\n  Scenario: Modified scenario\n    Given a shared context\n    When the shared action runs differently\n    Then the shared result is ready\n",
    )
    .expect("write same-file delta");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Modified scenario\n    Given a shared context\n    When the shared action runs differently\n    Then the shared result is ready\n",
    )
    .expect("write same-file draft");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[when("the user runs change-aware exploration against that change")]
fn run_same_file_change_exploration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("same-file project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--change",
            "running-change",
            "--file",
            "givn/changes/running-change/specs/change.feature",
        ],
        &[("GIVN_SPEC_EMBEDDER", "stub"), ("GIVN_SPEC_BGE", "stub")],
    ));
}

#[then("the other same-file scenario remains in the E5 raw candidates")]
fn same_file_neighbor_remains(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("Same-file neighbor"));
}

#[then("only the exact source and projected-target identities are excluded")]
fn same_file_exact_only(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        !output
            .lines()
            .filter(|line| line.starts_with("rank "))
            .any(|line| line.contains("Modified scenario"))
    );
    assert!(output.contains("Same-file neighbor"));
}

#[given("a project whose deterministic E5 neighbour union contains exactly 513 eligible pairs")]
fn overflow_pool_project(world: &mut GivnWorld) {
    directional_bge_project(world);
    world.retrieval_bge_test_pool = Some("overflow-513".to_string());
}

#[given("a project whose deterministic E5 neighbour union contains exactly 512 eligible pairs")]
fn bounded_pool_project(world: &mut GivnWorld) {
    directional_bge_project(world);
    world.retrieval_bge_test_pool = Some("bounded-512".to_string());
}

#[then("the command reports the bounded-pool overflow and the unavailable BGE status")]
fn overflow_reports_unavailable(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE_UNAVAILABLE: bounded candidate pool exceeds 512 pairs"));
    assert!(output.contains("BGE status: BGE_UNAVAILABLE"));
}

#[then("the output reports BGE unavailable reason \"pair-cap\"")]
fn overflow_reports_pair_cap(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("reason pair-cap"));
}

#[then(
    "the output reports zero unique pairs, directional calls, recommended pairs, filtered pairs, and model load attempts"
)]
fn overflow_reports_zero_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE unique pair count: 0"));
    assert!(output.contains("BGE directional call count: 0"));
    assert!(output.contains("BGE recommended count: 0"));
    assert!(output.contains("BGE filtered count: 0"));
    assert!(output.contains("BGE model load attempts: 0"));
}

#[then("the output reports all E5 raw pairs as unscored")]
fn overflow_reports_all_unscored(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE unscored count: 513"));
}

#[then("the output identifies the E5-only unavailable fallback")]
fn overflow_reports_e5_fallback(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE fallback: E5-only unavailable fallback"));
}

#[then("the output reports zero BGE model load attempts")]
fn overflow_reports_no_model_load(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE model load attempts: 0"));
}

#[then("no output claims that BGE participated")]
fn overflow_has_no_participation_claim(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("BGE model: rozgo/bge-reranker-v2-m3"));
}

#[then("the audit reports 512 unique pairs and 1024 directional calls")]
fn bounded_reports_exact_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("BGE unique pair count: 512"));
    assert!(output.contains("BGE directional call count: 1024"));
}

#[then("the audit reports no bounded-pool overflow")]
fn bounded_reports_no_overflow(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("BGE_UNAVAILABLE"));
}

#[then("both runs report the same E5 raw candidate identities in the same order")]
fn parity_raw_identities(world: &mut GivnWorld) {
    let before = world
        .previous_output
        .as_ref()
        .expect("disabled exploration output");
    let before = String::from_utf8_lossy(&before.stdout);
    let after = super::stdout(world);
    let ranks = |value: &str| {
        value
            .lines()
            .filter(|line| line.starts_with("rank "))
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    assert_eq!(ranks(&before), ranks(&after), "E5 raw identities changed");
}

#[then(
    "the enabled run reports a reduced recommended candidate list when the evidence rejects a candidate"
)]
fn enabled_run_reduces_recommendations(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("BGE filtered"),
        "missing BGE filtered accounting: {output}"
    );
    assert!(
        output.contains("recommended 0"),
        "expected a reduced recommendation list: {output}"
    );
}

#[then("only the enabled run reports participating second-stage evidence")]
fn only_enabled_reports_bge(world: &mut GivnWorld) {
    let before = world
        .previous_output
        .as_ref()
        .expect("disabled exploration output");
    assert!(!String::from_utf8_lossy(&before.stdout).contains("BGE"));
    assert!(super::stdout(world).contains("BGE"));
}

fn stable_bge_lines(output: &std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.starts_with("rank ") || line.starts_with("BGE symmetric raw score"))
        .map(str::to_string)
        .collect()
}

fn reported_bge_runtime(output: &std::process::Output) -> u128 {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            line.split_once("elapsed runtime ms ")
                .and_then(|(_, value)| value.split_whitespace().next())
                .and_then(|value| value.parse::<u128>().ok())
        })
        .expect("BGE runtime report")
}

#[then("the E5 candidate identities and second-stage scores are byte-for-byte stable")]
fn repeated_bge_scores_are_stable(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("first repeated exploration output");
    let second = super::output(world);
    assert_eq!(stable_bge_lines(first), stable_bge_lines(second));
}

#[then("each run reports a non-negative second-stage elapsed runtime")]
fn repeated_bge_runtime_is_non_negative(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("first repeated exploration output");
    let _ = reported_bge_runtime(first);
    let _ = reported_bge_runtime(super::output(world));
}

#[then("the runtime does not affect candidate ordering")]
fn repeated_bge_runtime_does_not_order_candidates(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("first repeated exploration output");
    let first_ranks = stable_bge_lines(first)
        .into_iter()
        .filter(|line| line.starts_with("rank "))
        .collect::<Vec<_>>();
    let second_ranks = stable_bge_lines(super::output(world))
        .into_iter()
        .filter(|line| line.starts_with("rank "))
        .collect::<Vec<_>>();
    assert_eq!(first_ranks, second_ranks);
}

#[then("stdout contains no NLI evidence")]
fn stdout_has_no_nli_evidence(world: &mut GivnWorld) {
    assert!(!super::stdout(world).to_ascii_lowercase().contains("nli"));
}

#[then("stdout identifies BGE as the participating second-stage model")]
fn stdout_identifies_bge_model(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE model: rozgo/bge-reranker-v2-m3"));
}

#[then("stdout identifies the local BGE model as participating")]
fn stdout_identifies_local_bge_model(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("BGE: model rozgo/bge-reranker-v2-m3"));
}

#[then("stdout reports two directional calls for one unique pair")]
fn local_bge_reports_pair_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("unique pairs 1"), "stdout: {output}");
    assert!(output.contains("directional calls 2"), "stdout: {output}");
}

#[then("stdout reports no model download")]
fn local_bge_reports_no_download(world: &mut GivnWorld) {
    assert!(
        !super::stdout(world)
            .to_ascii_lowercase()
            .contains("download")
    );
}

#[then("stdout contains diagnostic NLI evidence")]
fn stdout_contains_nli_evidence(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("nli: model"), "stdout: {output}");
    assert!(output.contains("nli diagnostic:"), "stdout: {output}");
}

#[then("stdout contains no BGE admission status")]
fn stdout_contains_no_bge_admission(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(!output.contains("BGE_RECOMMENDED"), "stdout: {output}");
    assert!(!output.contains("BGE_FILTERED"), "stdout: {output}");
    assert!(!output.contains("BGE status"), "stdout: {output}");
}

#[then("the E5 candidate identities remain unchanged")]
fn nli_preserves_e5_candidate_identities(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("Provider request is accepted"),
        "stdout: {output}"
    );
    assert!(
        output.contains("Provider warning is reported"),
        "stdout: {output}"
    );
}

#[then("the JSON retains the E5 raw result order and legacy result fields")]
fn bge_json_retains_raw_order(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let results = document["results"].as_array().expect("results array");
    for (index, result) in results.iter().enumerate() {
        assert_eq!(result["rank"], serde_json::json!(index + 1));
        for field in ["title", "file", "score", "scope", "change"] {
            assert!(
                result.get(field).is_some(),
                "missing legacy result field {field}"
            );
        }
        assert!(!result["bge"].is_null());
    }
}

#[then(
    "the JSON evidence reports raw, recommended, filtered, unscored, unique-pair, and directional-call counts"
)]
fn bge_json_reports_counts(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let evidence = &document["evidence"];
    for field in [
        "e5_candidates",
        "recommended_candidates",
        "bge_filtered_candidates",
        "bge_unscored_candidates",
        "bge_pairs",
        "bge_directional_calls",
    ] {
        assert!(
            evidence[field].is_number(),
            "missing evidence count {field}"
        );
    }
}

#[then("the JSON evidence reports the BGE model and elapsed runtime")]
fn bge_json_reports_model_runtime(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert_eq!(
        document["evidence"]["bge_model"],
        serde_json::json!("rozgo/bge-reranker-v2-m3")
    );
    assert!(document["evidence"]["bge_runtime_ms"].is_number());
}

#[then("the JSON reports BGE status \"BGE_UNAVAILABLE\"")]
fn unavailable_json_status(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let status = document["evidence"]["status"]
        .as_str()
        .or_else(|| document["bge_run"]["status"].as_str());
    assert_eq!(status, Some("BGE_UNAVAILABLE"));
}

#[then("the JSON reports reason \"missing-artifact\"")]
fn unavailable_json_reason(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    let reason = document["evidence"]["reason"]
        .as_str()
        .or_else(|| document["bge_run"]["reason"].as_str());
    assert_eq!(reason, Some("missing-artifact"));
}

#[then("the JSON reports zero scored, recommended, and filtered pairs")]
fn unavailable_json_zero_counts(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    if document["evidence"].is_object() {
        let evidence = &document["evidence"];
        assert_eq!(evidence["bge_pairs"], serde_json::json!(0));
        assert_eq!(evidence["recommended_candidates"], serde_json::json!(0));
        assert_eq!(evidence["bge_filtered_candidates"], serde_json::json!(0));
    } else {
        let run = &document["bge_run"];
        assert_eq!(run["unique_pairs"], serde_json::json!(0));
        assert_eq!(run["recommended_count"], serde_json::json!(0));
        assert_eq!(run["filtered_count"], serde_json::json!(0));
    }
}

#[then("the JSON retains the E5 raw result identities")]
fn unavailable_json_retains_results(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(!document["results"].as_array().unwrap().is_empty());
}

#[then("the JSON counts the raw candidates as unscored rather than filtered")]
fn unavailable_json_counts_unscored(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(
        document["evidence"]["bge_unscored_candidates"]
            .as_u64()
            .unwrap()
            > 0
    );
    assert_eq!(
        document["evidence"]["bge_filtered_candidates"],
        serde_json::json!(0)
    );
}

#[then("the JSON counts all E5 semantic candidates as unscored")]
fn unavailable_duplicate_counts_unscored(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(document["bge_run"]["unscored_count"].as_u64().unwrap() > 0);
}

#[then("the JSON retains the E5 fallback candidates")]
fn unavailable_duplicate_retains_candidates(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(!document["candidates"].as_array().unwrap().is_empty());
}

#[then("the JSON schema version is exactly 1")]
fn e5_only_duplicate_schema(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert_eq!(document["schema_version"], serde_json::json!(1));
}

#[then("the JSON contains no BGE run, filtered, or unscored collections")]
fn e5_only_duplicate_no_bge_fields(world: &mut GivnWorld) {
    let document: serde_json::Value = serde_json::from_slice(&super::output(world).stdout).unwrap();
    assert!(document.get("bge_run").is_none());
    assert!(document.get("filtered_candidates").is_none());
    assert!(document.get("unscored_candidates").is_none());
}
