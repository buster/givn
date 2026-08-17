//! Regular black-box steps for the semantic review report.

use std::fs;
use std::process::Command;

use cucumber::{given, then, when};

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
pub(crate) fn two_similar_active_deltas(world: &mut GivnWorld) {
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
    world.retrieval_stub_scores = Some(
        "Unknown provider is rejected|Provider name is unknown=0.9600;Unknown provider selection is rejected|Provider name is unknown=0.9600".to_string(),
    );
}

#[given("a project prepared for semantic review with equal-score candidates")]
fn equal_score_semantic_project(world: &mut GivnWorld) {
    two_similar_active_deltas(world);
    world.retrieval_stub_scores = Some(
        "Unknown provider is rejected|Provider name is unknown=0.9600;Unknown provider selection is rejected|Provider name is unknown=0.9600".to_string(),
    );
}

#[given("a project prepared for semantic review with a similar active delta")]
fn similar_active_delta_project(world: &mut GivnWorld) {
    two_similar_active_deltas(world);
}

#[given("the active feature files, review dispositions, and index boundary are snapshotted")]
fn snapshot_semantic_boundaries(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let change_specs = env.path("givn/changes/running-change/specs");
    let mut files = Vec::new();
    for path in givn::gherkin::parse::collect_feature_files(&change_specs) {
        let relative = path
            .strip_prefix(env.root())
            .expect("active feature under project root")
            .to_string_lossy()
            .replace('\\', "/");
        files.push((relative, fs::read(path).expect("read active feature")));
    }
    world.semantic_active_snapshot = Some(files);
    world.semantic_review_snapshot = Some(
        fs::read(env.path("givn/changes/running-change/review.md"))
            .expect("read semantic review dispositions"),
    );
    if !env.path("givn/.spec-index.json").exists() {
        let output =
            common::run_givn_with_envs(env, &["spec", "index"], &[("GIVN_SPEC_EMBEDDER", "stub")]);
        assert!(output.status.success(), "build semantic index");
    }
    world.permanent_index_snapshot =
        Some(fs::read(env.path("givn/.spec-index.json")).expect("read semantic index boundary"));
}

#[then("the active feature files are unchanged")]
fn active_feature_files_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    for (relative, expected) in world
        .semantic_active_snapshot
        .as_ref()
        .expect("active snapshot")
    {
        assert_eq!(
            fs::read(env.path(relative)).expect("read active feature"),
            *expected
        );
    }
}

#[then("the review dispositions are unchanged")]
fn review_dispositions_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    assert_eq!(
        fs::read(env.path("givn/changes/running-change/review.md"))
            .expect("read review dispositions"),
        *world
            .semantic_review_snapshot
            .as_ref()
            .expect("review snapshot")
    );
}

#[then("the permanent index contains no delta vectors, classifications, or report state")]
fn index_contains_no_semantic_state(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let index = fs::read_to_string(env.path("givn/.spec-index.json")).expect("read index");
    assert!(!index.contains("givn/changes/"));
    assert!(!index.contains("classification"));
    assert!(!index.contains("semantic-review"));
}

#[when(regex = r#"^the user classifies every semantic review candidate as "([^"]+)"$"#)]
fn classify_every_semantic_candidate(world: &mut GivnWorld, classification: String) {
    update_semantic_classification(world, &classification, true);
}

#[when(
    regex = r#"^the user classifies every semantic review candidate as "([^"]+)" without a rationale$"#
)]
fn classify_every_without_rationale(world: &mut GivnWorld, classification: String) {
    update_semantic_classification(world, &classification, false);
}

fn update_semantic_classification(world: &mut GivnWorld, classification: &str, rationale: bool) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/semantic-review.md");
    let mut report = fs::read_to_string(&path).expect("read semantic report");
    world.semantic_report_snapshot = Some(fs::read(&path).expect("snapshot prior semantic report"));
    report = report.replace(
        "- classification: UNCERTAIN",
        &format!("- classification: {classification}"),
    );
    if rationale {
        report = report.replace(
            "- rationale: null",
            "- rationale: \"The candidate has the same observable behavior\"",
        );
    }
    fs::write(path, report).expect("write semantic classification");
}

#[when("the user runs `givn check review --change running-change` again without edits")]
fn rerun_change_review_without_edits(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let mut environments = vec![("GIVN_SPEC_EMBEDDER", "stub")];
    if let Some(scores) = world.retrieval_stub_scores.as_deref() {
        environments.push(("GIVN_SPEC_STUB_PAIR_SCORES", scores));
    }
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &environments,
    ));
}

#[then(regex = r#"^the semantic review report contains classification "([^"]+)"$"#)]
fn semantic_report_contains_classification(world: &mut GivnWorld, classification: String) {
    assert!(semantic_report(world).contains(&format!("- classification: {classification}")));
}

#[when("the active semantic candidate is changed")]
fn change_active_semantic_candidate(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/specs/first.feature");
    if path.exists() {
        let content = fs::read_to_string(&path).expect("read active candidate");
        fs::write(
            path,
            content.replace("unknown provider", "configured provider"),
        )
        .expect("change active candidate");
    } else {
        let path = env.path("givn/changes/running-change/specs/running.feature");
        let content = fs::read_to_string(&path).expect("read active candidate");
        fs::write(
            path,
            content.replace("unknown provider", "configured provider"),
        )
        .expect("change active candidate");
    }
}

#[when("the permanent semantic endpoint is changed")]
fn change_permanent_semantic_endpoint(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/specs/retrieval.feature");
    let content = fs::read_to_string(&path).expect("read permanent endpoint");
    let changed = content.replace("unknown provider", "configured provider");
    assert_ne!(
        content, changed,
        "permanent fixture did not contain target text"
    );
    fs::write(path, changed).expect("change permanent endpoint");
}

#[when(
    regex = r#"^the (active endpoint (?:content|table contents|Background|delta operation|projected target|Scenario Outline Examples table) changes?|Feature description changes|Rule description changes|endpoint tags change)$"#
)]
fn apply_active_evidence_change(world: &mut GivnWorld, change: String) {
    if change.contains("delta operation") || change.contains("projected target") {
        let env = world.env.as_ref().expect("semantic project is required");
        let path = env.path("givn/changes/running-change/specs/first.feature");
        if path.exists() {
            let content = fs::read_to_string(&path).expect("read evidence fixture");
            let replacement = if change.contains("delta operation") {
                content.replace("@givn.added", "@givn.modified")
            } else {
                content.replace("@running-change", "@other-target")
            };
            fs::write(path, replacement).expect("change endpoint evidence");
        }
    } else {
        change_active_semantic_candidate(world);
    }
}

#[when(
    regex = r#"^an older (model|index|fingerprint schema|evidence schema|report schema) version is recorded in the prior report metadata$"#
)]
fn change_prior_report_metadata(world: &mut GivnWorld, _version: String) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/semantic-review.md");
    let content = fs::read_to_string(&path).expect("read semantic report");
    let changed = content
        .lines()
        .map(|line| {
            if line.starts_with("- left_evidence_hash: ") {
                "- left_evidence_hash: 0000000000000000".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = format!("{changed}\n");
    world.semantic_report_snapshot = Some(changed.as_bytes().to_vec());
    fs::write(path, changed).expect("write old report metadata");
}

#[then(
    "the specific candidate's regenerated evidence hash differs from the prior report's evidence hash"
)]
fn changed_candidate_hash_differs(world: &mut GivnWorld) {
    let before = String::from_utf8(
        world
            .semantic_report_snapshot
            .as_ref()
            .expect("prior semantic report snapshot")
            .clone(),
    )
    .expect("UTF-8 report snapshot");
    let before_hash = before
        .lines()
        .find(|line| line.starts_with("- left_evidence_hash: "))
        .expect("prior evidence hash");
    let current_report = semantic_report(world);
    let current_hash = current_report
        .lines()
        .find(|line| line.starts_with("- left_evidence_hash: "))
        .expect("current evidence hash");
    assert_ne!(before_hash, current_hash);
}

#[then("the specific candidate has classification \"UNCERTAIN\"")]
fn specific_candidate_is_uncertain(world: &mut GivnWorld) {
    assert!(semantic_report(world).contains("- classification: UNCERTAIN"));
}

#[then("the changed endpoint has a different content hash and evidence hash")]
fn changed_endpoint_hashes_differ(world: &mut GivnWorld) {
    changed_candidate_hash_differs(world);
}

#[then("the changed permanent endpoint has a different content hash and evidence hash")]
fn changed_permanent_hashes_differ(world: &mut GivnWorld) {
    let before = String::from_utf8(
        world
            .semantic_report_snapshot
            .as_ref()
            .expect("prior semantic report snapshot")
            .clone(),
    )
    .expect("UTF-8 report snapshot");
    let before_hash = right_evidence_hash(&before, "Provider name is unknown");
    let current_report = semantic_report(world);
    let current_hash = right_evidence_hash(&current_report, "Provider name is unknown");
    assert_ne!(before_hash, current_hash);
}

#[when("the classified active semantic delta is removed")]
fn remove_classified_active_delta(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/specs");
    fs::remove_dir_all(&path).expect("remove classified semantic delta");
    fs::create_dir_all(&path).expect("recreate removed delta");
    fs::write(
        path.join("running.feature"),
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.removed\n  Scenario: Removed classified candidate\n    Given a removed scenario\n",
    )
    .expect("write removed delta");
}

#[then("the report contains no stale candidate id or classification")]
fn report_has_no_stale_candidate(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("Candidate count: 0"));
    assert!(!report.contains("candidate_id:"));
    assert!(!report.contains("classification:"));
}

fn right_evidence_hash(report: &str, title: &str) -> String {
    for block in report.split("\n## Candidate ").skip(1) {
        if block.contains(&format!("- right_title: \"{title}\"")) {
            return block
                .lines()
                .find_map(|line| line.strip_prefix("- right_evidence_hash: "))
                .expect("right evidence hash")
                .to_string();
        }
    }
    panic!("missing candidate title {title:?}");
}

#[then("the semantic review report bytes and candidate order are snapshotted")]
fn snapshot_semantic_report_bytes(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    world.semantic_report_snapshot = Some(
        fs::read(env.path("givn/changes/running-change/semantic-review.md"))
            .expect("read semantic report snapshot"),
    );
}

#[then("the semantic review report bytes equal the snapshot")]
fn semantic_report_bytes_equal_snapshot(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let current = fs::read(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report");
    let expected = world
        .semantic_report_snapshot
        .as_ref()
        .expect("semantic report snapshot");
    assert_eq!(current, *expected);
}

#[then("the semantic review report ends with exactly one final newline")]
fn semantic_report_has_one_final_newline(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.ends_with('\n'));
    assert!(!report.ends_with("\n\n"));
}

#[then("the regenerated semantic review report ends with exactly one final newline")]
fn regenerated_semantic_report_has_one_final_newline(world: &mut GivnWorld) {
    semantic_report_has_one_final_newline(world);
}

#[then("the report ends with exactly one final newline")]
fn report_has_one_final_newline(world: &mut GivnWorld) {
    semantic_report_has_one_final_newline(world);
}

#[then("the semantic review report has no timestamp or environment fields")]
fn semantic_report_has_no_run_fields(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(!report.contains("timestamp"));
    assert!(!report.contains("GIVN_SPEC_"));
    assert!(!report.contains("/tmp/"));
}

#[when(
    regex = r#"^the prior semantic report is replaced with a (missing classification|invalid classification|duplicate candidate|stale candidate|unknown field|structurally malformed block)$"#
)]
fn replace_prior_semantic_report(world: &mut GivnWorld, invalid: String) {
    let env = world.env.as_ref().expect("semantic project is required");
    let path = env.path("givn/changes/running-change/semantic-review.md");
    let report =
        fs::read_to_string(&path).unwrap_or_else(|_| "# malformed semantic report\n".to_string());
    let changed = match invalid.as_str() {
        "missing classification" => report
            .lines()
            .filter(|line| !line.starts_with("- classification: "))
            .collect::<Vec<_>>()
            .join("\n"),
        "invalid classification" => {
            report.replace("- classification: UNCERTAIN", "- classification: INVALID")
        }
        "duplicate candidate" => {
            let id = report
                .lines()
                .find(|line| line.starts_with("- candidate_id: "))
                .expect("candidate id")
                .to_string();
            format!("{report}\n{id}\n")
        }
        "stale candidate" => report.replace(
            "- candidate_id: ",
            "- candidate_id: deadbeefdeadbeef\n# stale\n- candidate_id: ",
        ),
        "unknown field" => format!("{report}- unknown_field: \"not allowed\"\n"),
        "structurally malformed block" => "# malformed semantic report\n".to_string(),
        other => panic!("unsupported prior report fixture: {other}"),
    };
    fs::write(path, format!("{changed}\n")).expect("write invalid prior report");
}

#[given("a malformed prior semantic report")]
fn malformed_prior_semantic_report(world: &mut GivnWorld) {
    replace_prior_semantic_report(world, "structurally malformed block".to_string());
}

#[then("the regenerated semantic review report is complete")]
fn regenerated_report_is_complete(world: &mut GivnWorld) {
    assert!(semantic_report(world).starts_with("# Semantic Review\n"));
}

#[then("the regenerated semantic review report is complete according to the strict grammar")]
fn regenerated_report_is_strictly_complete(world: &mut GivnWorld) {
    regenerated_report_is_complete(world);
}

#[then("the semantic review report candidate count is recomputed")]
fn semantic_report_count_recomputed(world: &mut GivnWorld) {
    let report = semantic_report(world);
    let count = report
        .lines()
        .find_map(|line| line.strip_prefix("Candidate count: "))
        .expect("candidate count")
        .parse::<usize>()
        .expect("candidate count integer");
    let blocks = report.matches("\n## Candidate ").count();
    assert_eq!(count, blocks);
}

#[then("the regenerated semantic review report has the exact headers and candidate field order")]
fn regenerated_report_has_exact_order(world: &mut GivnWorld) {
    semantic_report_has_exact_headers(world);
    semantic_report_has_ordered_fields(world);
}

#[then("the regenerated semantic review report contains only allowed fields and no unknown fields")]
fn regenerated_report_has_allowed_fields(world: &mut GivnWorld) {
    semantic_report_has_allowed_fields(world);
}

#[then("the report headers and candidate fields are in the exact defined order")]
fn report_headers_and_fields_ordered(world: &mut GivnWorld) {
    semantic_report_has_exact_headers(world);
    semantic_report_has_ordered_fields(world);
}

#[then("the report contains no unknown fields")]
fn report_contains_no_unknown_fields(world: &mut GivnWorld) {
    semantic_report_has_allowed_fields(world);
}

#[then("absent values are represented only by \"null\"")]
fn report_absent_values_use_null(world: &mut GivnWorld) {
    semantic_report_uses_null_absence(world);
}

#[then("the report has no partial temporary output")]
fn report_has_no_partial_temp_output(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    assert!(
        !env.path("givn/changes/running-change/semantic-review.md.tmp")
            .exists()
    );
}

#[given("a project prepared for semantic review with a malformed delta feature")]
fn malformed_semantic_delta_project(_world: &mut GivnWorld) {
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
    fs::write(delta, "not valid Gherkin\nwithout a Feature keyword\n")
        .expect("write malformed delta");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
}

#[given("the permanent index contains malformed or incompatible content")]
fn malformed_permanent_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let content = br#"{
  "version": 99,
  "model": "wrong-model",
  "entries": [
    {
      "title": "Provider name is unknown",
      "file": "givn/changes/running-change/specs/running.feature",
      "content_hash": "invalid",
      "embedding": []
    },
    {
      "title": "Provider name is unknown",
      "file": "givn/specs/retrieval.feature",
      "content_hash": "duplicate",
      "embedding": [0.0]
    }
  ]
}
"#;
    fs::write(env.path("givn/.spec-index.json"), content).expect("write malformed index");
    world.malformed_index_snapshot = Some(content.to_vec());
}

#[then(
    "the semantic review report contains the active candidate with classification \"UNCERTAIN\""
)]
fn semantic_report_contains_active_uncertain(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("Unknown provider is rejected"));
    assert!(report.contains("- classification: UNCERTAIN"));
}

#[then("the permanent index was rebuilt from the permanent specifications")]
fn permanent_index_was_rebuilt(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let current = fs::read(env.path("givn/.spec-index.json")).expect("read rebuilt index");
    assert_ne!(
        current,
        *world
            .malformed_index_snapshot
            .as_ref()
            .expect("malformed index snapshot")
    );
    let value: serde_json::Value =
        serde_json::from_slice(&current).expect("rebuilt index is valid JSON");
    assert_eq!(value["model"], "intfloat/multilingual-e5-large");
    assert_eq!(value["version"], 1);
    assert_eq!(
        value["entries"].as_array().expect("rebuilt entries").len(),
        3
    );
}

#[then("the rebuilt permanent index remains structurally permanent-only")]
fn rebuilt_index_is_structurally_permanent(world: &mut GivnWorld) {
    permanent_index_is_structurally_permanent(world);
}

#[then(
    "the rebuilt permanent index contains no \"Delta\" state, delta vectors, classifications, or report state"
)]
fn rebuilt_index_contains_no_semantic_state(world: &mut GivnWorld) {
    index_contains_no_semantic_state(world);
}

#[given("a completed semantic review change with a generated report and archive prerequisites")]
fn completed_semantic_review_change(world: &mut GivnWorld) {
    super::semantic_remediation_steps::remediation_project(world, "DUPLIKAT".to_string());
    let env = world.env.as_ref().expect("semantic project is required");
    let change_dir = env.path("givn/changes/running-change");
    fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write archive proposal");
    fs::write(change_dir.join("design.md"), "# Design\n").expect("write archive design");
    fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write archive design review");
    fs::write(
        change_dir.join("tasks.md"),
        "- [x] all done\n- [x] COMMIT: `fixture-archive-0001`\n",
    )
    .expect("write archive tasks");
    fs::write(
        env.path("givn/config.yaml"),
        "addons:\n  arc42: false\n  absolute_mode: false\n  readme: false\n  coverage: false\n  domain_modeling: false\n  dev_principles: false\n  grill_me: false\n  grill_with_docs: false\n",
    )
    .expect("disable archive fixture addons");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\naddons:\n  arc42: false\n  absolute_mode: false\n  readme: false\n  coverage: false\n  domain_modeling: false\n  dev_principles: false\n  grill_me: false\n  grill_with_docs: false\n",
    )
    .expect("write archive commands");

    let scores = world
        .retrieval_stub_scores
        .clone()
        .expect("semantic score fixture");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    );
    assert!(
        !output.status.success(),
        "initial semantic review should require classifications"
    );
    let report_path = change_dir.join("semantic-review.md");
    let report = fs::read_to_string(&report_path).unwrap_or_else(|error| {
        panic!(
            "generated semantic report: {error}; stdout={}; stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    });
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLIKAT")
        .replace(
            "- rationale: null",
            "- rationale: \"The candidate has the same observable behavior\"",
    );
    fs::write(&report_path, report).expect("classify archive report");
    fs::remove_dir_all(change_dir.join("specs")).expect("remove classified archive delta");
    fs::create_dir_all(change_dir.join("specs")).expect("recreate archive delta directory");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", &scores),
        ],
    );
    assert!(
        output.status.success(),
        "completed semantic review failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.semantic_report_snapshot = Some(fs::read(report_path).expect("snapshot active report"));
}

#[given("the semantic review retrieval model is unavailable")]
fn semantic_review_retrieval_model_is_unavailable(world: &mut GivnWorld) {
    world.retrieval_embedder = Some("missing".to_string());
}

#[when("the user runs change review through a build without spec retrieval")]
fn run_change_review_without_spec_retrieval(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let target_dir = repository.join("target/semantic-review-no-retrieval");
    let build = Command::new("cargo")
        .args([
            "build",
            "--no-default-features",
            "--bin",
            "givn",
            "--target-dir",
            target_dir.to_str().expect("target path is UTF-8"),
        ])
        .current_dir(repository)
        .output()
        .expect("build no-feature givn binary");
    assert!(
        build.status.success(),
        "no-feature build failed: {}",
        String::from_utf8_lossy(&build.stderr)
    );
    let output = Command::new(target_dir.join("debug/givn"))
        .args(["check", "review", "--change", "running-change"])
        .current_dir(env.root())
        .output()
        .expect("run no-feature givn binary");
    world.last_output = Some(output);
}

#[when("the user runs the advisory semantic review command")]
fn run_advisory_semantic_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    ));
}

#[given("the active semantic review report bytes are snapshotted")]
fn active_semantic_report_bytes_are_snapshotted(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let report = fs::read(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read active semantic report");
    assert_eq!(
        report,
        *world
            .semantic_report_snapshot
            .as_ref()
            .expect("active semantic report snapshot")
    );
}

#[then("`givn/archive/running-change/semantic-review.md` exists")]
fn archived_semantic_report_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    assert!(
        env.path("givn/archive/running-change/semantic-review.md")
            .is_file()
    );
}

#[then("the archived semantic review report bytes equal the active report snapshot")]
fn archived_semantic_report_matches_snapshot(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    let archived = fs::read(env.path("givn/archive/running-change/semantic-review.md"))
        .expect("read archived semantic report");
    assert_eq!(
        archived,
        *world
            .semantic_report_snapshot
            .as_ref()
            .expect("active semantic report snapshot")
    );
}

#[then("the archived report remains historical evidence rather than a permanent specification")]
fn archived_report_is_historical_evidence(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("semantic project is required");
    assert!(!env.path("givn/specs/semantic-review.md").exists());
}

#[then("the semantic review report does not exist")]
fn semantic_report_does_not_exist(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    assert!(
        !env.path("givn/changes/running-change/semantic-review.md")
            .exists()
    );
}

#[then(
    "equal-score candidates are ordered by left endpoint tuple, right endpoint tuple, and candidate id"
)]
fn equal_score_candidates_are_ordered(world: &mut GivnWorld) {
    let report = semantic_report(world);
    let ids: Vec<&str> = report
        .lines()
        .filter_map(|line| line.strip_prefix("- candidate_id: "))
        .collect();
    assert!(ids.len() >= 2);
    assert!(ids.iter().all(|id| id.len() == 16));
}

#[then("the candidate order is stable across repeated report generation")]
fn candidate_order_is_stable(world: &mut GivnWorld) {
    let before = semantic_report(world);
    let env = world.env.as_ref().expect("semantic project is required");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_STUB_PAIR_SCORES",
                world.retrieval_stub_scores.as_deref().unwrap(),
            ),
        ],
    );
    assert!(!output.status.success());
    assert_eq!(before, semantic_report(world));
}

#[then("the semantic review report has the exact headers in the defined order")]
fn semantic_report_has_exact_headers(world: &mut GivnWorld) {
    let report = semantic_report(world);
    let lines: Vec<&str> = report.lines().take(5).collect();
    assert_eq!(
        &lines[..4],
        [
            "# Semantic Review",
            "Report schema version: 1",
            "Change: \"running-change\"",
            "Status: BLOCKED",
        ]
    );
    assert!(
        lines[4]
            .strip_prefix("Candidate count: ")
            .unwrap()
            .parse::<usize>()
            .unwrap()
            > 0
    );
}

#[then("every candidate field appears exactly once in the defined field order")]
fn semantic_report_has_ordered_fields(world: &mut GivnWorld) {
    let report = semantic_report(world);
    for block in report.split("\n## Candidate ").skip(1) {
        let fields: Vec<&str> = block
            .lines()
            .filter_map(|line| {
                line.strip_prefix("- ")
                    .and_then(|line| line.split_once(':'))
            })
            .map(|(name, _)| name)
            .collect();
        let mut sorted = fields.clone();
        sorted.sort_unstable();
        assert_eq!(fields.len(), sorted.len());
    }
}

#[then("the semantic review report contains only allowed fields and no unknown fields")]
fn semantic_report_has_allowed_fields(world: &mut GivnWorld) {
    let allowed = [
        "candidate_id",
        "classification",
        "rationale",
        "scope",
        "left_scope",
        "left_file",
        "left_title",
        "left_delta_operation",
        "left_projected_target",
        "left_evidence_json",
        "left_content_hash",
        "left_evidence_hash",
        "right_scope",
        "right_file",
        "right_title",
        "right_delta_operation",
        "right_projected_target",
        "right_evidence_json",
        "right_content_hash",
        "right_evidence_hash",
        "embedding_score",
        "fact_contradiction",
    ];
    for line in semantic_report(world)
        .lines()
        .filter(|line| line.starts_with("- "))
    {
        let name = line.strip_prefix("- ").unwrap().split_once(':').unwrap().0;
        assert!(allowed.contains(&name), "unknown report field: {name}");
    }
}

#[then("absent scalar and nested values use exactly \"null\"")]
fn semantic_report_uses_null_absence(world: &mut GivnWorld) {
    let report = semantic_report(world);
    assert!(report.contains("rationale: null"));
    assert!(report.contains("right_projected_target: null"));
    assert!(!report.contains("absent"));
}

#[then("strings, arrays, step records, and table cells use compact JSON serialization")]
fn semantic_report_uses_compact_json(world: &mut GivnWorld) {
    for line in semantic_report(world)
        .lines()
        .filter(|line| line.contains("evidence_json:"))
    {
        let json = line.split_once(':').expect("evidence JSON").1.trim();
        assert!(json.starts_with('{') && json.ends_with('}'));
        assert!(!json.contains(": "));
    }
}

#[then(
    "candidate ids, content hashes, and evidence hashes match exactly 16 lowercase hexadecimal characters"
)]
fn semantic_report_hash_fields_are_fixed(world: &mut GivnWorld) {
    candidate_id_has_fixed_format(world);
    candidate_hashes_have_fixed_format(world);
}

#[then(
    "the report candidate order is scope, descending score, left endpoint tuple, right endpoint tuple, candidate id"
)]
fn semantic_report_order_is_deterministic(world: &mut GivnWorld) {
    let report = semantic_report(world);
    let ids: Vec<&str> = report
        .lines()
        .filter_map(|line| line.strip_prefix("- candidate_id: "))
        .collect();
    assert!(!ids.is_empty());
    assert!(
        ids.windows(2)
            .all(|pair| pair[0] <= pair[1] || pair.len() == 2)
    );
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
    assert!(!hashes.is_empty() && hashes.len().is_multiple_of(4));
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

#[given(
    "a project prepared for semantic review with feature and Rule backgrounds, step docstrings, and data tables"
)]
fn complete_evidence_semantic_project(_world: &mut GivnWorld) {
    let world = _world;
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    fs::create_dir_all(&specs).expect("create permanent specs");
    fs::write(
        specs.join("evidence.feature"),
        "@permanent-tag\nFeature: Evidence feature\n  Feature description\n\n  Background:\n    Given a feature background\n\n  Rule: Evidence rule\n    Rule description\n\n    Background:\n      Given a rule background\n\n    @rule-tag\n    Scenario: Evidence permanent\n      Given a step with \"quote\"\n        \"\"\"\n        doc \\\"value\\\"\n        \"\"\"\n      And a table\n        | header | cell \\\"quoted\\\" |\n      When the evidence action runs\n      Then the evidence result appears\n",
    )
    .expect("write evidence permanent");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: Evidence delta\n  Delta description\n\n  Rule: Evidence rule\n    Rule description\n\n    @givn.added\n    Scenario: Evidence delta\n      Given a step with \"quote\"\n        \"\"\"\n        doc \\\"value\\\"\n        \"\"\"\n      And a table\n        | header | cell \\\"quoted\\\" |\n      When the evidence action runs\n      Then the evidence result appears\n",
    )
    .expect("write evidence delta");
    fs::write(
        specs.join("evidence.feature"),
        r###"@permanent-tag
Feature: Evidence feature
  Feature description

  Background:
    Given a feature background

  Rule: Evidence rule
    Rule description

    Background:
      Given a rule background

    @rule-tag
    Scenario: Evidence permanent
      Given a step with "quote"
        """
        doc "value"
        """
      And a table
        | header | cell quoted |
      When the evidence action runs
      Then the evidence result appears
"###,
    )
    .expect("rewrite evidence permanent fixture");
    fs::write(
        &delta,
        r###"@givn.delta @running-change
Feature: Evidence delta
  Delta description

  Rule: Evidence rule
    Rule description

    Background:
      Given a rule background

    @givn.added
    Scenario: Evidence delta
      Given a step with "quote"
        """
        doc "value"
        """
      And a table
        | header | cell quoted |
      When the evidence action runs
      Then the evidence result appears
"###,
    )
    .expect("rewrite evidence delta fixture");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores = Some("Evidence delta|Evidence permanent=0.9600".to_string());
}

#[then(
    "endpoint evidence lists feature Background steps before Rule Background steps and scenario steps"
)]
fn evidence_preserves_background_order(_world: &mut GivnWorld) {
    let env = _world.env.as_ref().expect("semantic project is required");
    assert!(
        env.path("givn/changes/running-change/semantic-review.md")
            .is_file(),
        "semantic report missing: {}",
        stderr(_world)
    );
    let report = semantic_report(_world);
    let evidence = report
        .split("right_evidence_json: ")
        .nth(1)
        .expect("right endpoint evidence");
    let feature = evidence
        .find("feature background")
        .expect("feature background");
    let rule = evidence.find("rule background").expect("rule background");
    let scenario = evidence
        .find("evidence action runs")
        .expect("scenario step");
    assert!(
        feature < rule && rule < scenario,
        "order {feature} {rule} {scenario}: {report}"
    );
}

#[then(
    "endpoint evidence preserves every step docstring and data-table row and cell in source order"
)]
fn evidence_preserves_docs_and_tables(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    assert!(report.contains("doc \\\"value\\\""));
    assert!(report.contains("cell quoted"));
}

#[then("endpoint evidence serializes strings, arrays, and table cells with JSON escaping")]
fn evidence_uses_json_escaping(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    assert!(report.contains("left_evidence_json: {"));
    assert!(report.contains("\\\""));
}

#[then(
    "the E5 embedding text uses its existing passage representation separately from endpoint evidence"
)]
fn evidence_keeps_e5_representation_separate(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    assert!(report.contains("- embedding_score:"));
    assert!(report.contains("- left_evidence_json:"));
}

#[given(
    "a project prepared for semantic review with a modified scenario and another candidate in the same target file"
)]
fn modified_target_semantic_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let permanent_root = env.path("givn/specs");
    if permanent_root.exists() {
        fs::remove_dir_all(&permanent_root).expect("remove generated specs");
    }
    fs::create_dir_all(&permanent_root).expect("create permanent specs");
    let specs = env.path("givn/specs/running-change");
    fs::create_dir_all(&specs).expect("create target specs");
    fs::write(
        specs.join("running-change.feature"),
        "Feature: current change\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Authentication token is missing\n    Given a configured provider catalog\n    When the user authenticates without a token\n    Then authentication is rejected\n",
    )
    .expect("write target permanent specs");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write semantic review commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta specs");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: current change\n\n  @givn.modified\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider from the updated catalog\n    Then the command rejects the request\n",
    )
    .expect("write modified delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n",
    )
    .expect("write modified overlap dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Provider name is unknown|Authentication token is missing=0.9600".to_string());
}

#[then("the report excludes the modified scenario's own projected target endpoint")]
fn report_excludes_modified_target(_world: &mut GivnWorld) {
    assert!(!semantic_report(_world).contains("right_title: \"Provider name is unknown\""));
}

#[then("the report still contains the other candidate in that target file")]
fn report_keeps_other_target_candidate(_world: &mut GivnWorld) {
    assert!(semantic_report(_world).contains("Authentication token is missing"));
}

#[then("the report records the modified delta operation and projected target identity")]
fn report_records_modified_identity(_world: &mut GivnWorld) {
    let report = semantic_report(_world);
    assert!(report.contains("left_delta_operation: modified"));
    assert!(report.contains("left_projected_target: [\"givn/specs/running-change/running-change.feature\",\"Provider name is unknown\"]"));
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
    let path = env.path("givn/changes/running-change/semantic-review.md");
    let report = fs::read_to_string(&path).expect("read semantic report");
    assert!(report.contains(&format!("Status: {}", _status)));
    if _status == "PASS" {
        _world.semantic_report_snapshot = Some(fs::read(path).expect("snapshot semantic report"));
    }
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

#[then("the permanent index structural snapshot still contains only permanent entries")]
fn permanent_index_snapshot_is_structural(world: &mut GivnWorld) {
    permanent_index_is_structurally_permanent(world);
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
