//! Regular black-box steps for bounded NLI evidence.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("the NLI strict-mode setup step")]
fn nli_strict_mode_setup(_world: &mut GivnWorld) {
    unimplemented!("NLI step skeleton is implemented scenario by scenario");
}

#[given("a project with one semantic candidate and directional NLI evidence scores")]
fn one_nli_candidate(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove generated NLI specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create NLI permanent specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write NLI permanent candidate");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("NLI delta parent"))
        .expect("create NLI delta directory");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write NLI active candidate");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[when("the user runs advisory NLI review")]
fn run_advisory_nli_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI candidate project");
    let nli_mode = world.retrieval_reranker.as_deref().unwrap_or("nli-stub");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", nli_mode),
        ],
    ));
}

#[then(
    "the output reports entailment, neutral, and contradiction probabilities in both directions"
)]
fn reports_nli_directional_probabilities(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("forward [entailment"), "stdout: {output}");
    assert!(output.contains("reverse [entailment"), "stdout: {output}");
    assert!(output.contains("neutral"), "stdout: {output}");
    assert!(output.contains("contradiction"), "stdout: {output}");
}

#[then("the output reports the symmetric NLI probability summary")]
fn reports_nli_symmetric_probabilities(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("symmetric [entailment"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the output reports an NLI admission status")]
fn reports_nli_admission_status(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("NLI_RECOMMENDED")
            || super::stdout(world).contains("NLI_FILTERED"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the symmetric summary is independent of endpoint order")]
fn nli_symmetric_summary_is_order_independent(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let symmetric = output
        .lines()
        .find(|line| line.contains("symmetric [entailment"))
        .expect("symmetric NLI output");
    let probabilities = symmetric
        .split("symmetric [")
        .nth(1)
        .and_then(|value| value.split(']').next())
        .expect("symmetric probability list")
        .split(',')
        .map(|value| {
            value
                .split_whitespace()
                .last()
                .expect("probability value")
                .parse::<f32>()
                .expect("numeric probability")
        })
        .collect::<Vec<_>>();
    assert_eq!(probabilities.len(), 3);
    assert!((probabilities.iter().sum::<f32>() - 1.0).abs() < 1e-3);
}

#[given("a project with semantic candidates and independent BGE and NLI evidence")]
fn combined_evidence_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove combined specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create combined specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write combined permanent candidates");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("combined delta parent"))
        .expect("create combined delta");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write combined active candidate");
    world.env = Some(env);
}

#[when("the user runs combined second-stage advisory review")]
fn run_combined_advisory_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("combined candidate project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/specs/provider.feature|Provider request is accepted->givn/specs/running-change/running-change.feature|Active provider request=4.0;givn/specs/running-change/running-change.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0",
            ),
        ],
    ));
}

#[then("the E5 raw candidate identities remain unchanged")]
fn e5_raw_identities_remain_unchanged(world: &mut GivnWorld) {
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

#[then("a candidate is recommended only when both BGE and NLI recommend it")]
fn combined_admission_requires_both(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("combined recommended 1"),
        "stdout: {output}"
    );
    assert!(output.contains("BGE_FILTERED"), "stdout: {output}");
}

#[then("each filtered candidate remains evidence without a Givn classification")]
fn filtered_combined_candidates_are_non_classifying(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("evidence is advisory"), "stdout: {output}");
    assert!(!output.contains("DUPLIKAT"), "stdout: {output}");
    assert!(!output.contains("VARIANTE"), "stdout: {output}");
}

#[given("a project with a semantic candidate that BGE recommends and NLI cannot score")]
fn combined_unavailable_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove unavailable specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create unavailable specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write unavailable permanent candidate");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("unavailable delta parent"))
        .expect("create unavailable delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write unavailable active candidate");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write unavailable commands");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Active provider request` | `Provider request is accepted` | related |\n",
    )
    .expect("write unavailable review disposition");
    world.env = Some(env);
}

#[when("the user runs combined second-stage blocking review")]
fn run_combined_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "stub"),
            ("GIVN_SPEC_RERANKER", "nli-missing"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                "Active provider request|Provider request is accepted=0.9900",
            ),
            (
                "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
                "givn/changes/running-change/specs/running.feature|Active provider request->givn/specs/provider.feature|Provider request is accepted=4.0;givn/specs/provider.feature|Provider request is accepted->givn/changes/running-change/specs/running.feature|Active provider request=4.0",
            ),
        ],
    ));
}

#[then("the command exits non-zero because the E5 candidate remains unresolved")]
fn combined_unavailable_exits_nonzero(world: &mut GivnWorld) {
    assert!(
        !super::output(world).status.success(),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the report records NLI as unavailable without NLI filtered evidence")]
fn report_records_nli_unavailable(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read unavailable NLI report");
    assert!(
        report.contains("NLI status: NLI_UNAVAILABLE"),
        "report: {report}"
    );
    assert!(
        report.contains("NLI reason: missing-artifact"),
        "report: {report}"
    );
    assert!(!report.contains("NLI_FILTERED"), "report: {report}");
}

#[then("the candidate remains a normal blocking candidate with no NLI probabilities")]
fn unavailable_candidate_remains_blocking(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("unavailable combined project");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read unavailable NLI report");
    assert!(report.contains("## Candidate"), "report: {report}");
    assert!(
        report.contains("classification: UNCERTAIN"),
        "report: {report}"
    );
    assert!(
        report.contains("nli_admission: NLI_UNSCORED"),
        "report: {report}"
    );
    assert!(
        report.contains("nli_forward_entailment: null"),
        "report: {report}"
    );
}

#[given("a permanent corpus whose bounded NLI pool contains exactly 513 eligible pairs")]
fn nli_overflow_duplicate_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove NLI duplicate specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create NLI duplicate specs");
    fs::write(
        env.path("givn/specs/semantic.feature"),
        "Feature: semantic\n\n  Scenario: Token is required\n    Given a client request\n    When the client sends a request without credentials\n    Then the server responds with unauthorized status\n\n  Scenario: Authentication needs a token\n    Given an identity service\n    When the client authenticates without a token\n    Then access is denied with token required\n",
    )
    .expect("write NLI duplicate candidates");
    world.env = Some(env);
}

#[when("the user runs the permanent NLI duplicate audit")]
fn run_nli_duplicate_audit(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI duplicate project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            ("GIVN_SPEC_NLI_TEST_POOL", "overflow-513"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600",
            ),
        ],
    ));
}

#[then("the audit reports NLI as unavailable for reason \"pair-cap\"")]
fn nli_duplicate_reports_pair_cap(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("NLI status: NLI_UNAVAILABLE, reason pair-cap"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then(
    "the audit reports zero NLI pairs, directional calls, recommendations, filters, and model loads"
)]
fn nli_duplicate_reports_zero_pair_counts(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for line in [
        "NLI unique pair count: 0",
        "NLI directional call count: 0",
        "NLI recommended count: 0",
        "NLI filtered count: 0",
        "NLI model load attempts: 0",
    ] {
        assert!(output.contains(line), "missing {line}: {output}");
    }
}

#[then("all E5 semantic candidates are recorded as NLI unscored")]
fn nli_duplicate_reports_unscored(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("NLI_UNAVAILABLE: bounded candidate pool exceeds 512 pairs"));
    assert!(output.contains("NLI unscored:"), "stdout: {output}");
    assert!(
        output.contains("nli_status=NLI_UNSCORED"),
        "stdout: {output}"
    );
}

#[given(
    "a permanent corpus with 513 E5 semantic candidates and a successful bounded NLI pool of 512 pairs"
)]
fn nli_partial_duplicate_project(world: &mut GivnWorld) {
    // The deterministic pool seam supplies the 513 raw / 512 scored counts.
    // The permanent fixture itself only needs one eligible semantic pair.
    nli_overflow_duplicate_project(world);
}

#[when("the user runs the permanent NLI duplicate audit with JSON output")]
fn run_nli_duplicate_audit_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI partial duplicate project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "nli-stub"),
            ("GIVN_SPEC_NLI_TEST_POOL", "partial-raw-513"),
            (
                "GIVN_SPEC_DUPLICATES_TEST_SCORES",
                "givn/specs/semantic.feature|Token is required::givn/specs/semantic.feature|Authentication needs a token=0.9600",
            ),
        ],
    ));
}

#[then("the JSON reports NLI as enabled with 512 unique pairs and 1024 directional calls")]
fn nli_json_reports_partial_counts(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI JSON");
    assert_eq!(
        document["nli_run"]["status"],
        serde_json::json!("NLI_ENABLED")
    );
    assert_eq!(document["nli_run"]["unique_pairs"], serde_json::json!(512));
    assert_eq!(
        document["nli_run"]["directional_calls"],
        serde_json::json!(1024)
    );
}

#[then("the JSON contains NLI unscored candidates with null NLI probabilities")]
fn nli_json_reports_unscored_probabilities(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI JSON");
    let unscored = document["nli_unscored_candidates"]
        .as_array()
        .expect("NLI unscored collection");
    assert!(!unscored.is_empty());
    assert!(unscored[0]["evidence"]["nli"].is_null());
}

#[then("the NLI unscored candidates remain outside the NLI filtered collection")]
fn nli_json_unscored_not_filtered(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI JSON");
    assert!(
        document["nli_filtered_candidates"]
            .as_array()
            .expect("NLI filtered collection")
            .is_empty()
    );
}

#[given("a project with two E5 semantic candidates and NLI evidence")]
fn nli_explore_json_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    fs::remove_dir_all(env.path("givn/specs")).expect("remove NLI Explore specs");
    fs::create_dir_all(env.path("givn/specs")).expect("create NLI Explore specs");
    fs::write(
        env.path("givn/specs/provider.feature"),
        "Feature: provider\n\n  Scenario: Provider request is accepted\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n\n  Scenario: Provider warning is reported\n    Given a configured provider catalog\n    When the user selects a known provider with a warning\n    Then the command reports the warning\n",
    )
    .expect("write NLI Explore candidates");
    fs::write(
        env.path("draft.feature"),
        "Feature: draft\n\n  Scenario: Draft provider request\n    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n",
    )
    .expect("write NLI Explore draft");
    world.env = Some(env);
}

#[when("the user explores the draft with JSON output and NLI evidence enabled")]
fn run_nli_explore_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI Explore project");
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
            ("GIVN_SPEC_RERANKER", "nli-stub"),
        ],
    ));
}

#[then("the JSON uses schema version 3 for NLI evidence")]
fn nli_explore_json_schema(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI Explore JSON");
    assert_eq!(document["schema_version"], serde_json::json!(3));
}

#[then("the JSON preserves raw E5 result order and legacy result fields")]
fn nli_explore_json_raw_contract(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI Explore JSON");
    let results = document["results"].as_array().expect("results array");
    for (index, result) in results.iter().enumerate() {
        assert_eq!(result["rank"], serde_json::json!(index + 1));
        for field in ["title", "file", "score", "scope", "change"] {
            assert!(result.get(field).is_some(), "missing legacy field {field}");
        }
    }
}

#[then(
    "the JSON reports NLI model, probabilities, raw, recommended, filtered, unscored, pair, and runtime fields"
)]
fn nli_explore_json_fields(world: &mut GivnWorld) {
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid NLI Explore JSON");
    let evidence = &document["nli_evidence"];
    for field in [
        "status",
        "candidate_generator",
        "top_k",
        "e5_candidates",
        "recommended_candidates",
        "nli_filtered_candidates",
        "nli_unscored_candidates",
        "nli_pairs",
        "nli_directional_calls",
        "nli_model",
        "nli_model_load_attempts",
        "nli_runtime_ms",
    ] {
        assert!(evidence.get(field).is_some(), "missing NLI field {field}");
    }
    assert_eq!(
        evidence["nli_model"],
        serde_json::json!("MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7")
    );
    assert!(document["results"][0]["nli"].is_object());
}

#[given("a project with a permanent semantic candidate and the NLI capability compiled")]
fn nli_disabled_project(world: &mut GivnWorld) {
    nli_explore_json_project(world);
}

#[given("NLI evidence is disabled")]
fn disable_nli_evidence(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("disabled".to_string());
}

#[when("the user explores the draft through the NLI-capable CLI")]
fn explore_with_nli_disabled(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("NLI disabled project");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "explore", "--file", "draft.feature"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "disabled"),
        ],
    ));
}

#[then("the result contains the E5 candidates")]
fn disabled_result_contains_e5(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("rank 1:"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("stdout contains no NLI model, probability, runtime, or admission status")]
fn disabled_stdout_has_no_nli(world: &mut GivnWorld) {
    assert!(
        !super::stdout(world).contains("NLI"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the output uses the E5-only contract")]
fn disabled_output_is_e5_only(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(output.contains("query:"), "stdout: {output}");
    assert!(!output.contains("evidence"), "stdout: {output}");
}

#[given("a project with a semantic candidate and unavailable NLI model artifacts")]
fn nli_missing_artifacts_project(world: &mut GivnWorld) {
    one_nli_candidate(world);
    world.retrieval_reranker = Some("nli-missing".to_string());
}

#[then("the output reports NLI as unavailable")]
fn nli_missing_reports_unavailable(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("NLI status: NLI_UNAVAILABLE"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the output reports the unavailable reason as missing-artifact")]
fn nli_missing_reports_reason(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("reason missing-artifact"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("the E5 candidates remain visible and blocking when applicable")]
fn nli_missing_keeps_e5_candidates(world: &mut GivnWorld) {
    assert!(
        super::stdout(world).contains("E5 candidates"),
        "stdout: {}",
        super::stdout(world)
    );
}

#[then("no output claims that NLI participated")]
fn nli_missing_has_no_participation_claim(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(!output.contains("NLI model: Moritz"), "stdout: {output}");
}
