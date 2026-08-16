//! Regular black-box steps for the semantic review report.

use std::fs;

use cucumber::{given, then};

use super::{GivnWorld, stderr};
use crate::common;

const PERMANENT_CORPUS: &str = "Feature: retrieval fixtures

  Scenario: Provider name is unknown
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Authentication token is missing
    Given a configured provider catalog
    When the user authenticates without a token
    Then authentication is rejected

  Scenario: Provider configuration is valid
    Given a configured provider catalog
    When the user selects a known provider
    Then the command accepts the request
";

#[given("a project prepared for semantic review with only a removed delta")]
fn removed_semantic_delta_project(_world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(specs.join("retrieval.feature"), PERMANENT_CORPUS)
        .expect("write permanent semantic fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.removed\n  Scenario: Removed scenario\n    Given a removed scenario\n",
    )
    .expect("write removed delta");
    _world.env = Some(env);
    _world.retrieval_embedder = Some("stub".to_string());
    _world.retrieval_reranker = Some("stub".to_string());
}

#[given("a project prepared for semantic review with two similar active deltas")]
fn two_similar_active_deltas(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(specs.join("retrieval.feature"), PERMANENT_CORPUS)
        .expect("write permanent semantic fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let first = env.path("givn/changes/running-change/specs/first.feature");
    let second = env.path("givn/changes/running-change/specs/second.feature");
    fs::create_dir_all(first.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        &first,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write first active delta");
    fs::write(
        &second,
        "@givn.delta @running-change\nFeature: running change two\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write second active delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Provider name is unknown` | related |\n| `Unknown provider selection is rejected` | `Provider name is unknown` | related |\n| `Unknown provider is rejected` | `Unknown provider selection is rejected` | related |\n",
    )
    .expect("write overlap dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("a project prepared for semantic review with active deltas below the similarity boundary")]
fn below_boundary_semantic_project(_world: &mut GivnWorld) {
    let world = _world;
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(specs.join("retrieval.feature"), PERMANENT_CORPUS)
        .expect("write permanent semantic fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Quantum ledger opens a weather window\n    Given a quantum ledger\n    When the weather window opens\n    Then the ledger emits a blue result\n",
    )
    .expect("write below-boundary delta");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given(
    "a project prepared for semantic review with a candidate scored exactly at the similarity boundary"
)]
fn exact_boundary_semantic_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("boundary.feature"),
        "Feature: boundary fixtures\n\n  Scenario: Exact boundary permanent\n    Given a boundary catalog\n    When the boundary action runs\n    Then the boundary result appears\n",
    )
    .expect("write boundary permanent");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        delta,
        "@givn.delta @running-change\nFeature: boundary delta\n\n  @givn.added\n  Scenario: Exact boundary delta\n    Given a boundary catalog\n    When the boundary action runs\n    Then the boundary result appears\n",
    )
    .expect("write boundary delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Exact boundary delta` | `Exact boundary permanent` | related |\n",
    )
    .expect("write boundary dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Exact boundary delta|Exact boundary permanent=0.9500".to_string());
}

#[then(regex = r#"^the candidate embedding score is exactly "([^"]+)"$"#)]
fn candidate_score_is_exact(world: &mut GivnWorld, expected: String) {
    assert!(semantic_report(world).contains(&format!("- embedding_score: {expected}")));
}

#[then("the semantic review report contains a candidate")]
fn semantic_report_contains_candidate(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/semantic-review.md");
    assert!(
        path.is_file(),
        "semantic report missing: {}",
        stderr(_world)
    );
    assert!(semantic_report(_world).contains("Candidate count: 1"));
}

#[then("the candidate id is exactly 16 lowercase hexadecimal characters")]
fn candidate_id_has_fixed_format(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    let id = report
        .lines()
        .find_map(|line| line.strip_prefix("- candidate_id: "))
        .expect("candidate id");
    assert_eq!(id.len(), 16);
    assert!(
        id.chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    );
}

#[given("a project prepared for semantic review with a Rule-nested active delta")]
fn rule_nested_semantic_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(specs.join("retrieval.feature"), PERMANENT_CORPUS)
        .expect("write permanent semantic fixtures");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: running change\n\n  Rule: provider rules\n\n    @givn.added\n    Scenario: Rule nested rejects an unknown provider\n      Given a configured provider catalog\n      When the user selects an unknown provider\n      Then the command rejects the request\n",
    )
    .expect("write Rule-nested delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n",
    )
    .expect("write Rule overlap dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Rule nested rejects an unknown provider|Provider name is unknown=0.9600".to_string());
}

#[then("the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters")]
fn candidate_hashes_have_fixed_format(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    let hashes: Vec<&str> = report
        .lines()
        .filter_map(|line| {
            line.strip_prefix("- left_content_hash: ")
                .or_else(|| line.strip_prefix("- left_evidence_hash: "))
                .or_else(|| line.strip_prefix("- right_content_hash: "))
                .or_else(|| line.strip_prefix("- right_evidence_hash: "))
        })
        .collect();
    assert_eq!(hashes.len(), 4);
    for hash in hashes {
        assert_eq!(hash.len(), 16);
        assert!(
            hash.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
    }
}

#[then(
    regex = r#"^the semantic review report contains the exact Rule-nested endpoint title "([^"]+)"$"#
)]
fn report_contains_rule_nested_title(world: &mut GivnWorld, title: String) {
    assert!(semantic_report(world).contains(&title));
}

#[then("the semantic review report contains non-empty content and evidence hashes")]
fn report_contains_non_empty_hashes(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(
        report
            .lines()
            .any(|line| { line.starts_with("- left_content_hash: ") && !line.ends_with(' ') })
    );
    assert!(
        report
            .lines()
            .any(|line| { line.starts_with("- left_evidence_hash: ") && !line.ends_with(' ') })
    );
}

#[then("the report contains no permanent-permanent candidate")]
fn report_contains_no_permanent_pair(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    assert!(!report.contains("permanent-permanent"));
    assert!(!report.contains("delta-to-permanent"));
}

#[then("the semantic review report exists")]
fn semantic_report_exists(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    assert!(
        env.path("givn/changes/running-change/semantic-review.md")
            .is_file()
    );
}

#[then(regex = r#"^the semantic review report status is "([^"]+)"$"#)]
fn semantic_report_status(_world: &mut GivnWorld, _status: String) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains(&format!("Status: {}", _status)));
}

#[then("the semantic review report candidate count is 0")]
fn semantic_report_has_no_candidates(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(report.contains("Candidate count: 0"));
}

#[then(regex = r#"^the semantic review report does not mention "([^"]+)"$"#)]
fn semantic_report_does_not_mention(_world: &mut GivnWorld, text: String) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let report = fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    assert!(!report.contains(&text), "semantic report contains {text:?}");
}

#[then("the permanent index remains structurally permanent-only")]
fn permanent_index_is_structurally_permanent(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    let index =
        fs::read_to_string(env.path("givn/.spec-index.json")).expect("read permanent index");
    assert!(!index.contains("givn/changes/"));
    assert!(!index.contains("Delta"));
    let value: serde_json::Value = serde_json::from_str(&index).expect("parse permanent index");
    for entry in value["entries"].as_array().expect("index entries") {
        assert!(
            entry["file"]
                .as_str()
                .is_some_and(|file| file.starts_with("givn/specs/"))
        );
    }
}

#[then(regex = r#"^the semantic review report contains scope "([^"]+)"$"#)]
fn report_contains_scope(world: &mut GivnWorld, scope: String) {
    assert!(semantic_report(world).contains(&format!("- scope: {scope}")));
}

#[then(regex = r#"^the report names the exact left and right endpoint titles for both scopes$"#)]
fn report_names_exact_endpoint_titles(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("Unknown provider is rejected"));
    assert!(report.contains("Unknown provider selection is rejected"));
    assert!(report.contains("Provider name is unknown"));
}

#[then(regex = r#"^all candidate scores have exactly four decimals and are at least "([^"]+)"$"#)]
fn report_scores_have_boundary(world: &mut GivnWorld, _boundary: String) {
    for line in semantic_report(world)
        .lines()
        .filter(|line| line.starts_with("- embedding_score:"))
    {
        let score = line.split_once(':').expect("score field").1.trim();
        assert_eq!(score.len(), 6);
        assert!(score.parse::<f32>().expect("score number") >= 0.95);
    }
}

#[then("all endpoint hashes are exactly 16 lowercase hexadecimal characters")]
fn report_hashes_are_fixed_format(world: &mut GivnWorld) {
    for line in semantic_report(world)
        .lines()
        .filter(|line| line.contains("hash:"))
    {
        let hash = line.split_once(':').expect("hash field").1.trim();
        assert_eq!(hash.len(), 16);
        assert!(
            hash.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
    }
}

#[then("the semantic review report contains deterministic facts")]
fn report_contains_deterministic_facts(world: &mut GivnWorld) {
    assert!(semantic_report(world).contains("fact_contradiction:"));
}

fn semantic_report(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("semantic project is required");
    fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report")
}
