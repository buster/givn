//! E2E black-box steps for the semantic review report.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

#[given("an end-to-end project prepared for semantic review with a similar active delta")]
fn e2e_semantic_review_project(world: &mut GivnWorld) {
    super::semantic_review_steps::two_similar_active_deltas(world);
}

#[when(regex = r#"^the end-to-end user runs change review for "([^"]+)"$"#)]
fn e2e_run_change_review(world: &mut GivnWorld, change: String) {
    let env = world.env.as_ref().expect("semantic project is required");
    let scores = world
        .retrieval_stub_scores
        .as_deref()
        .expect("semantic score fixture");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", &change],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_STUB_PAIR_SCORES", scores),
        ],
    ));
}

#[then(
    "the semantic review report contains a candidate with both endpoints and an \"UNCERTAIN\" classification"
)]
fn e2e_report_contains_uncertain_candidate(world: &mut GivnWorld) {
    let report = e2e_report(world);
    assert!(report.contains("Candidate count: "));
    assert!(report.contains("- left_title: \"Unknown provider is rejected\""));
    assert!(report.contains("- right_title: \"Provider name is unknown\""));
    assert!(report.contains("- classification: UNCERTAIN"));
}

#[then(
    "the candidate has left scope \"current-change\" and title \"Unknown provider is rejected\""
)]
fn e2e_candidate_left_endpoint(world: &mut GivnWorld) {
    let report = e2e_report(world);
    assert!(report.contains("- left_scope: current-change"));
    assert!(report.contains("- left_title: \"Unknown provider is rejected\""));
}

#[then("the candidate has right scope \"permanent\" and title \"Provider name is unknown\"")]
fn e2e_candidate_right_endpoint(world: &mut GivnWorld) {
    let report = e2e_report(world);
    assert!(report.contains("- right_scope: permanent"));
    assert!(report.contains("- right_title: \"Provider name is unknown\""));
}

#[then(
    "both endpoint content hashes and evidence hashes are exactly 16 lowercase hexadecimal characters"
)]
fn e2e_endpoint_hashes_are_fixed(world: &mut GivnWorld) {
    let report = e2e_report(world);
    let hashes: Vec<&str> = report
        .lines()
        .filter_map(|line| {
            if line.starts_with("- left_content_hash:")
                || line.starts_with("- left_evidence_hash:")
                || line.starts_with("- right_content_hash:")
                || line.starts_with("- right_evidence_hash:")
            {
                line.split_once(':').map(|(_, value)| value.trim())
            } else {
                None
            }
        })
        .collect();
    assert!(!hashes.is_empty() && hashes.len().is_multiple_of(4));
    assert!(hashes.iter().all(|hash| {
        hash.len() == 16
            && hash
                .chars()
                .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    }));
}

#[then(
    "the candidate embedding score has exactly four decimals in the range \"0.9500\" through \"1.0000\""
)]
fn e2e_candidate_score_is_fixed(world: &mut GivnWorld) {
    let report = e2e_report(world);
    let scores: Vec<&str> = report
        .lines()
        .filter_map(|line| line.strip_prefix("- embedding_score: "))
        .collect();
    assert!(!scores.is_empty());
    assert!(scores.iter().all(|score| {
        score.len() == 6
            && score.as_bytes()[1] == b'.'
            && score[2..]
                .chars()
                .all(|character| character.is_ascii_digit())
            && (0.95..=1.0).contains(&score.parse::<f32>().expect("score"))
    }));
}

#[then(
    regex = r#"^the candidate embedding score has exactly four decimals in the range "([^"]+)" through "([^"]+)"$"#
)]
fn e2e_candidate_score_is_in_range(world: &mut GivnWorld, low: String, high: String) {
    let report = e2e_report(world);
    let scores: Vec<&str> = report
        .lines()
        .filter_map(|line| line.strip_prefix("- embedding_score: "))
        .collect();
    assert!(!scores.is_empty());
    let low = low.parse::<f32>().expect("range lower bound");
    let high = high.parse::<f32>().expect("range upper bound");
    assert!(scores.iter().all(|score| {
        score.len() == 6
            && score.as_bytes()[1] == b'.'
            && score[2..]
                .chars()
                .all(|character| character.is_ascii_digit())
            && (low..=high).contains(&score.parse::<f32>().expect("score"))
    }));
}

#[then("the candidate contains the concrete fact \"fact_contradiction: false\"")]
fn e2e_candidate_contains_fact(world: &mut GivnWorld) {
    assert!(e2e_report(world).contains("- fact_contradiction: false"));
}

fn e2e_report(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("semantic project is required");
    fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read semantic report")
}
