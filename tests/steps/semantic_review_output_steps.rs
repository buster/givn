//! Regular black-box steps for decision-first semantic review output.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

/// Projected identity of the single active delta scenario.
pub(crate) const DELTA_ENDPOINT: &str =
    "givn/specs/running-change/running-change.feature|Active provider request";
pub(crate) const PERMANENT_ENDPOINTS: [&str; 3] = [
    "givn/specs/provider.feature|Provider request is accepted",
    "givn/specs/provider.feature|Provider timeout is reported",
    "givn/specs/provider.feature|Provider warning is reported",
];
const DELTA_BODY: &str = "    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n";

/// Corpus where second-stage evidence recommends exactly the first
/// permanent pair and filters or token-caps the rest.
pub(crate) fn review_output_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    let mut permanent = String::from("Feature: provider\n");
    for title in PERMANENT_ENDPOINTS {
        let title = title.split('|').nth(1).expect("endpoint title");
        permanent.push_str(&format!("\n  Scenario: {title}\n{DELTA_BODY}"));
    }
    fs::write(specs.join("provider.feature"), permanent).expect("write permanent corpus");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create delta dir");
    fs::write(
        &delta,
        format!(
            "@givn.delta @running-change\nFeature: running change\n\n  @givn.added\n  Scenario: Active provider request\n{DELTA_BODY}"
        ),
    )
    .expect("write active delta");
    world.env = Some(env);
}

/// Deterministic score overrides: recommend the first permanent pair,
/// filter the others.
pub(crate) fn review_output_env(world: &GivnWorld) -> Vec<(String, String)> {
    let mut bge_scores = String::new();
    for (index, right) in PERMANENT_ENDPOINTS.iter().enumerate() {
        let score = if index == 0 { "4.0" } else { "1.0" };
        bge_scores.push_str(&format!("{DELTA_ENDPOINT}->{right}={score};"));
        bge_scores.push_str(&format!("{right}->{DELTA_ENDPOINT}={score};"));
    }
    let mut envs = vec![
        ("GIVN_SPEC_EMBEDDER".to_string(), "stub".to_string()),
        ("GIVN_SPEC_BGE".to_string(), "stub".to_string()),
        ("GIVN_SPEC_RERANKER".to_string(), "nli-stub".to_string()),
        (
            "GIVN_SPEC_BGE_DIRECTIONAL_SCORES".to_string(),
            bge_scores.trim_end_matches(';').to_string(),
        ),
    ];
    if let Some(nli_budget) = world.retrieval_nli_token_budget.as_deref() {
        envs.push((
            "GIVN_SPEC_NLI_TOKEN_BUDGET".to_string(),
            nli_budget.to_string(),
        ));
    }
    if let Some(extra) = world.retrieval_review_extra.as_ref() {
        for (key, value) in extra {
            envs.push((key.clone(), value.clone()));
        }
    }
    envs
}

pub(crate) fn run_review_with(
    world: &mut GivnWorld,
    extra_args: &[&str],
    extra_envs: &[(&str, &str)],
) {
    let env = world.env.as_ref().expect("review output project missing");
    let mut args: Vec<&str> = vec!["spec", "review", "--change", "running-change"];
    args.extend_from_slice(extra_args);
    let dynamic = review_output_env(world);
    let mut environments: Vec<(&str, &str)> = dynamic
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    environments.extend_from_slice(extra_envs);
    world.last_output = Some(common::run_givn_with_envs(env, &args, &environments));
}

#[given(
    "a project prepared for semantic review whose second-stage evidence recommends one pair and filters many"
)]
fn review_recommends_one_filters_many(world: &mut GivnWorld) {
    review_output_project(world);
    world.retrieval_nli_token_budget = Some("partial-512-513".to_string());
}

#[given(
    "a project prepared for semantic review whose second-stage evidence recommends at least one pair"
)]
fn review_recommends_at_least_one(world: &mut GivnWorld) {
    review_output_project(world);
    world.retrieval_nli_token_budget = Some("partial-512-513".to_string());
}

#[when("the user runs the advisory semantic review for \"running-change\"")]
fn run_review_default(world: &mut GivnWorld) {
    run_review_with(world, &[], &[]);
}

#[then(
    "stdout lists every recommended second-stage pair exactly once under its stage with its admission status"
)]
fn lists_recommended_once(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let bge_recommended = output
        .lines()
        .filter(|line| line.starts_with("BGE_RECOMMENDED:"))
        .count();
    let nli_recommended = output
        .lines()
        .filter(|line| *line == "NLI admission: NLI_RECOMMENDED")
        .count();
    assert_eq!(bge_recommended, 1, "stdout: {output}");
    assert_eq!(nli_recommended, 1, "stdout: {output}");
}

#[then("stdout contains no detail line for a filtered or unscored second-stage pair")]
fn hides_filtered_and_unscored(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for forbidden in [
        "BGE_FILTERED:",
        "BGE_UNSCORED:",
        "NLI admission: NLI_FILTERED",
        "NLI admission: NLI_UNSCORED",
    ] {
        assert!(
            !output.contains(forbidden),
            "unexpected {forbidden} in stdout: {output}"
        );
    }
}

#[then("token-capped pairs keep their guidance lines visible with pair and limit token counts")]
fn token_cap_guidance_visible(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let cap_lines = output
        .lines()
        .filter(|line| line.contains("token-cap: NLI_TOKEN_CAP"))
        .count();
    assert_eq!(cap_lines, 2, "stdout: {output}");
    assert!(
        output.contains("requires 513 tokens (limit 512)"),
        "stdout: {output}"
    );
    assert!(output.contains("left 257"), "stdout: {output}");
}

#[then("the stage status lines, accounting counters, and label distributions remain printed")]
fn status_lines_remain(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.contains("BGE status: BGE_ENABLED"),
        "stdout: {output}"
    );
    assert!(
        output.contains("BGE candidate accounting:"),
        "stdout: {output}"
    );
    assert!(
        output.contains("nli: model MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7"),
        "stdout: {output}"
    );
    assert!(
        output.contains("NLI candidate accounting:"),
        "stdout: {output}"
    );
    assert!(output.contains("nli distribution:"), "stdout: {output}");
}

#[when(
    "the user runs the advisory semantic review for \"running-change\" again with verbose output"
)]
fn run_review_verbose(world: &mut GivnWorld) {
    run_review_with(world, &["--verbose"], &[]);
}

#[then("stdout contains no raw E5 similarity-candidate detail line")]
fn hides_raw_e5_list(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        !output.contains("similarity candidate ["),
        "raw E5 detail leaked into default output: {output}"
    );
}

#[then("stdout still reports the E5 evaluated-pairs count")]
fn reports_evaluated_count(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let evaluated = output
        .lines()
        .find(|line| line.contains("retrieval advisory: embedding similarity evaluated "))
        .expect("evaluated-pairs line");
    assert!(evaluated.ends_with("candidate pairs"), "{evaluated}");
}

#[then("stdout lists every E5 similarity candidate above the threshold in its established format")]
fn lists_raw_e5_verbose(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let detail_lines = output
        .lines()
        .filter(|line| line.starts_with("similarity candidate ["))
        .count();
    assert!(detail_lines >= 1, "stdout: {output}");
    for line in output
        .lines()
        .filter(|l| l.starts_with("similarity candidate ["))
    {
        assert!(line.contains(", threshold 0.9500; remediation:"), "{line}");
    }
    let evaluated = output
        .lines()
        .find(|line| line.contains("embedding similarity evaluated "))
        .expect("evaluated-pairs line");
    let count: usize = evaluated
        .split("evaluated ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .and_then(|value| value.parse().ok())
        .expect("evaluated count");
    assert_eq!(
        detail_lines, count,
        "verbose lists every gate candidate: {output}"
    );
}
