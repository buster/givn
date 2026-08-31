//! Step definitions for the unified semantic rulebook capability.
//!
//! Fixtures build deterministic stub-score candidate sets with lexically
//! disjoint corpora (the stub embedder is a real bag-of-words vectorizer, so
//! only explicitly overridden pairs may admit at the 0.90 boundary).
//! Assertions reuse the shared subprocess harness (`tests/common`).

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

/// Distinct filler vocabularies so raw stub cosines stay far below the
/// admission boundary; overrides control every admitted pair by title.
const VOCABULARIES: [&str; 7] = [
    "alpha beacon canopy",
    "bravo dossier envelope",
    "charlie fountain granite",
    "delta harbor island",
    "echo junction kernel",
    "foxtrot lantern meadow",
    "gambit nomadic outpost",
];

fn scenario_body(index: usize, title: &str) -> String {
    let words = VOCABULARIES[index % VOCABULARIES.len()];
    format!(
        "\n  Scenario: {title}\n    Given the {words} ledger is loaded\n    When the operator reads the {words} entry\n    Then the {words} ledger answers\n"
    )
}

/// Assemble a semantic-review fixture: a lexically disjoint permanent corpus,
/// delta scenarios, overlap dispositions, and stub pair scores covering every
/// admitted pair. `admitted` maps (delta index, permanent index, score).
pub(crate) fn rulebook_project(
    world: &mut GivnWorld,
    delta_titles: &[&str],
    permanent_titles: &[&str],
    admitted: &[(usize, usize, &str)],
) {
    let env = common::make_init_root();
    std::fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write rulebook commands");
    let specs = env.path("givn/specs");
    if specs.exists() {
        std::fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    std::fs::create_dir_all(&specs).expect("create permanent specs");
    let mut corpus = String::from("Feature: rulebook corpus\n");
    for (index, title) in permanent_titles.iter().enumerate() {
        corpus.push_str(&scenario_body(index, title));
    }
    std::fs::write(specs.join("corpus.feature"), corpus).expect("write permanent corpus");

    let delta_dir = env.path("givn/changes/running-change/specs");
    std::fs::create_dir_all(&delta_dir).expect("create delta specs");
    let mut delta = String::from("@givn.delta @running-change\nFeature: running change\n");
    for (index, title) in delta_titles.iter().enumerate() {
        delta.push_str(&scenario_body(index, title));
    }
    std::fs::write(delta_dir.join("delta.feature"), delta).expect("write active delta");

    let mut dispositions = String::from(
        "REVIEW: PASS\nREADME-IMPACT: none\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n",
    );
    for (d, p, _) in admitted {
        dispositions.push_str(&format!(
            "| `{}` | `{}` | related |\n",
            delta_titles[*d], permanent_titles[*p]
        ));
    }
    std::fs::write(
        env.path("givn/changes/running-change/review.md"),
        dispositions,
    )
    .expect("write overlap dispositions");

    let scores: Vec<String> = admitted
        .iter()
        .map(|(d, p, score)| format!("{}|{}={}", delta_titles[*d], permanent_titles[*p], score))
        .collect();
    // Deterministic second-stage evidence: every admitted pair is
    // BGE_RECOMMENDED (score 5 >= 3.0) and NLI_RECOMMENDED (entailment logit
    // 5 -> probability ~1 >= 0.5) in both directions, so the candidates stay
    // blocking regardless of the stub backends' lexical fallbacks.
    const DELTA_PREFIX: &str = "givn/changes/running-change/specs/delta.feature|";
    const CORPUS_PREFIX: &str = "givn/specs/corpus.feature|";
    let mut bge_scores: Vec<String> = Vec::new();
    let mut nli_scores: Vec<String> = Vec::new();
    for (d, p, _) in admitted {
        let left = format!("{DELTA_PREFIX}{}", delta_titles[*d]);
        let right = format!("{CORPUS_PREFIX}{}", permanent_titles[*p]);
        bge_scores.push(format!("{left}->{right}=5.0000"));
        bge_scores.push(format!("{right}->{left}=5.0000"));
        nli_scores.push(format!("{left}->{right}=5,0,0"));
        nli_scores.push(format!("{right}->{left}=5,0,0"));
    }
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_stub_scores = Some(scores.join(";"));
    world.retrieval_bge_scores = Some(bge_scores.join(";"));
    world.retrieval_nli_scores = Some(nli_scores.join(";"));
}

/// The two-delta fixture from the semantic-review capability, reconfigured
/// for the combined second-stage stubs the rulebook group runs with.
pub(crate) fn reconfigure_combined_stubs(world: &mut GivnWorld) {
    world.retrieval_bge = Some("stub".to_string());
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("a rulebook review project with a single admitted candidate")]
pub(crate) fn single_admitted_candidate(world: &mut GivnWorld) {
    rulebook_project(
        world,
        &["Delta needs one"],
        &["Corpus supplies one"],
        &[(0, 0, "0.9600")],
    );
    reconfigure_combined_stubs(world);
}

#[given("a rulebook review project with two similar active deltas")]
pub(crate) fn two_similar_active_deltas_rulebook(world: &mut GivnWorld) {
    // Reuse the proven fixture from the semantic-review capability; the
    // rulebook scenarios only add combined second-stage stubs plus
    // deterministic recommended-directional evidence for all three pairs
    // (two delta-to-permanent, one delta-to-delta).
    super::semantic_review_steps::two_similar_active_deltas(world);
    reconfigure_combined_stubs(world);
    const D1: &str = "givn/changes/running-change/specs/first.feature|Unknown provider is rejected";
    const D2: &str =
        "givn/changes/running-change/specs/second.feature|Unknown provider selection is rejected";
    const P: &str = "givn/specs/retrieval.feature|Provider name is unknown";
    world.retrieval_bge_scores = Some(format!(
        "{D1}->{P}=5.0000;{P}->{D1}=5.0000;{D2}->{P}=5.0000;{P}->{D2}=5.0000;{D1}->{D2}=5.0000;{D2}->{D1}=5.0000"
    ));
    world.retrieval_nli_scores = Some(format!(
        "{D1}->{P}=5,0,0;{P}->{D1}=5,0,0;{D2}->{P}=5,0,0;{P}->{D2}=5,0,0;{D1}->{D2}=5,0,0;{D2}->{D1}=5,0,0"
    ));
}

#[given("a rulebook review project with more candidate pairs than the pair cap")]
pub(crate) fn more_pairs_than_cap(world: &mut GivnWorld) {
    let delta_titles: Vec<String> = (0..5).map(|i| format!("Delta needed {i}")).collect();
    let permanent_titles: Vec<String> = (0..5)
        .flat_map(|i| (0..5).map(move |j| format!("Corpus supplies {i} and {j}")))
        .collect();
    let delta_refs: Vec<&str> = delta_titles.iter().map(String::as_str).collect();
    let permanent_refs: Vec<&str> = permanent_titles.iter().map(String::as_str).collect();
    let admitted: Vec<(usize, usize, &str)> = (0..5)
        .flat_map(|d| (0..5).map(move |p| (d, p, "0.9600")))
        .collect();
    rulebook_project(world, &delta_refs, &permanent_refs, &admitted);
    reconfigure_combined_stubs(world);
}

#[given("a rulebook review project with one active delta and more neighbours than the neighbour count")]
pub(crate) fn one_delta_more_neighbours(world: &mut GivnWorld) {
    let permanent_titles: Vec<String> = (0..7)
        .map(|i| format!("Corpus supplies neighbour {i}"))
        .collect();
    let permanent_refs: Vec<&str> = permanent_titles.iter().map(String::as_str).collect();
    let admitted: Vec<(usize, usize, &str)> = (0..7).map(|p| (0usize, p, "0.9600")).collect();
    rulebook_project(world, &["Delta needs one"], &permanent_refs, &admitted);
    reconfigure_combined_stubs(world);
}

#[given("a rulebook review project with two similar active deltas and an unrelated permanent corpus")]
pub(crate) fn two_deltas_unrelated_corpus(world: &mut GivnWorld) {
    // Two near-identical delta scenarios over a lexically unrelated corpus:
    // only the delta-to-delta pair admits (override), matching the stub
    // embedder's title-keyed score table.
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        std::fs::remove_dir_all(&specs).expect("remove generated specs");
    }
    std::fs::create_dir_all(&specs).expect("create permanent specs");
    std::fs::write(
        specs.join("corpus.feature"),
        "Feature: rulebook corpus\n\n  Scenario: Recipe ledger stores amounts\n    Given the recipe ledger is loaded\n    When the cook reads the recipe entry\n    Then the recipe ledger answers\n",
    )
    .expect("write unrelated permanent corpus");
    let delta_dir = env.path("givn/changes/running-change/specs");
    std::fs::create_dir_all(&delta_dir).expect("create delta specs");
    std::fs::write(
        delta_dir.join("delta.feature"),
        "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  @givn.added\n  Scenario: Unknown provider selection is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write active delta");
    std::fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Unknown provider selection is rejected` | related |\n",
    )
    .expect("write overlap dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Unknown provider is rejected|Unknown provider selection is rejected=0.9600".to_string());
    world.retrieval_bge_scores = Some(
        "givn/changes/running-change/specs/delta.feature|Unknown provider is rejected->givn/changes/running-change/specs/delta.feature|Unknown provider selection is rejected=5.0000;givn/changes/running-change/specs/delta.feature|Unknown provider selection is rejected->givn/changes/running-change/specs/delta.feature|Unknown provider is rejected=5.0000".to_string(),
    );
    world.retrieval_nli_scores = Some(
        "givn/changes/running-change/specs/delta.feature|Unknown provider is rejected->givn/changes/running-change/specs/delta.feature|Unknown provider selection is rejected=5,0,0;givn/changes/running-change/specs/delta.feature|Unknown provider selection is rejected->givn/changes/running-change/specs/delta.feature|Unknown provider is rejected=5,0,0".to_string(),
    );
    reconfigure_combined_stubs(world);
}

#[given("an emergency pair budget below the selected pair count")]
pub(crate) fn emergency_budget(world: &mut GivnWorld) {
    world.retrieval_max_rerank_pairs = Some("10".to_string());
}

#[then(regex = r#"^the semantic review report candidate count is ([1-9][0-9]*)$"#)]
fn semantic_report_candidate_count(world: &mut GivnWorld, count: usize) {
    let env = world.env.as_ref().expect("project root is required");
    let report = std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    assert!(
        report.contains(&format!("Candidate count: {count}")),
        "expected candidate count {count} in report:\n{report}"
    );
}

#[then("the semantic review report records no candidate as `BGE_UNSCORED`")]
fn semantic_report_no_bge_unscored(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root is required");
    let report = std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    assert!(
        !report.contains("BGE_UNSCORED"),
        "no BGE_UNSCORED expected in report:\n{report}"
    );
}

#[then("the semantic review report records no candidate as `NLI_UNSCORED`")]
fn semantic_report_no_nli_unscored(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root is required");
    let report = std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic review report");
    assert!(
        !report.contains("NLI_UNSCORED"),
        "no NLI_UNSCORED expected in report:\n{report}"
    );
}

/// Snapshot the current semantic report so a later rerun can be compared
/// candidate-by-candidate.
#[when("the semantic review report is snapshotted")]
pub(crate) fn snapshot_semantic_report(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root is required");
    let report =
        std::fs::read(env.path("givn/changes/running-change/semantic-review.md"))
            .expect("read semantic review report");
    world.semantic_report_snapshot = Some(report);
}
