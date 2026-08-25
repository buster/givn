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
    ];
    let bge_override = world.retrieval_review_extra.as_ref().is_some_and(|extra| {
        extra
            .iter()
            .any(|(key, _)| key == "GIVN_SPEC_BGE_DIRECTIONAL_SCORES")
    });
    if !bge_override {
        envs.push((
            "GIVN_SPEC_BGE_DIRECTIONAL_SCORES".to_string(),
            bge_scores.trim_end_matches(';').to_string(),
        ));
    }
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

#[when("the user runs the advisory semantic review for \"running-change\" with verbose output")]
fn run_review_verbose_plain(world: &mut GivnWorld) {
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

#[given(
    "a project prepared for semantic review whose second-stage evidence filters every candidate"
)]
fn review_filters_everything(world: &mut GivnWorld) {
    review_output_project(world);
    let mut nli_scores = String::new();
    for right in PERMANENT_ENDPOINTS {
        nli_scores.push_str(&format!(
            "{DELTA_ENDPOINT}->{right}=-0.4,1.4,0.2;{right}->{DELTA_ENDPOINT}=-0.4,1.4,0.2;"
        ));
    }
    let mut bge_scores = String::new();
    for right in PERMANENT_ENDPOINTS {
        bge_scores.push_str(&format!(
            "{DELTA_ENDPOINT}->{right}=1.0;{right}->{DELTA_ENDPOINT}=1.0;"
        ));
    }
    world.retrieval_review_extra = Some(vec![
        (
            "GIVN_SPEC_NLI_DIRECTIONAL_SCORES".to_string(),
            nli_scores.trim_end_matches(';').to_string(),
        ),
        (
            "GIVN_SPEC_BGE_DIRECTIONAL_SCORES".to_string(),
            bge_scores.trim_end_matches(';').to_string(),
        ),
    ]);
}

#[then("stdout reports the recommendation accounting for both stages with zero recommended pairs")]
fn zero_recommended_accounting(world: &mut GivnWorld) {
    let output = super::stdout(world);
    for label in ["BGE candidate accounting:", "NLI candidate accounting:"] {
        let line = output
            .lines()
            .find(|line| line.contains(label))
            .unwrap_or_else(|| panic!("missing {label} in stdout: {output}"));
        let recommended = line
            .split("recommended ")
            .nth(1)
            .and_then(|value| value.split(',').next())
            .and_then(|value| value.parse::<usize>().ok())
            .expect("recommended count");
        assert_eq!(recommended, 0, "{label}: {line}");
    }
    assert!(
        output.contains("combined recommended 0 of 3 E5 candidates"),
        "stdout: {output}"
    );
}

#[then("stderr names the flagged recommended pair count")]
fn stderr_flagged_count(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("semantic review flagged 3 recommended candidate(s)"),
        "stderr: {stderr}"
    );
}

#[then("stderr names the persisted findings path under the change directory")]
fn stderr_findings_path(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(stderr.contains("review-findings.json"), "stderr: {stderr}");
}

#[then("givn/changes/running-change/review-findings.json exists")]
fn findings_file_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project missing");
    assert!(
        env.path("givn/changes/running-change/review-findings.json")
            .exists(),
        "findings file missing"
    );
}

#[given(
    "a project prepared for semantic review with a similar active delta and second-stage evidence disabled"
)]
fn review_second_stage_disabled(world: &mut GivnWorld) {
    review_output_project(world);
}

#[when(
    "the user runs the advisory semantic review for \"running-change\" with second-stage evidence disabled"
)]
fn run_review_disabled(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("review output project missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_BGE", "disabled"),
            ("GIVN_SPEC_RERANKER", "disabled"),
        ],
    ));
}

#[then("stderr names the fixed-threshold gate failure")]
fn stderr_threshold_failure(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("similarity gate failed:") && stderr.contains("fixed threshold 0.9500"),
        "stderr: {stderr}"
    );
}

#[given(
    "a project prepared for semantic review whose recommended pairs rank differently by confidence than by embedding score"
)]
fn review_confidence_order_fixture(world: &mut GivnWorld) {
    review_output_project(world);
    let mut bge_scores = String::new();
    let mut nli_scores = String::new();
    let confidences = [
        (PERMANENT_ENDPOINTS[0], "3.5", "0.9,0,0"),
        (PERMANENT_ENDPOINTS[1], "4.5", "1.6,0,0"),
        (PERMANENT_ENDPOINTS[2], "6.5", "2.6,0,0"),
    ];
    for (right, bge, nli) in confidences {
        bge_scores.push_str(&format!(
            "{DELTA_ENDPOINT}->{right}={bge};{right}->{DELTA_ENDPOINT}={bge};"
        ));
        nli_scores.push_str(&format!(
            "{DELTA_ENDPOINT}->{right}={nli};{right}->{DELTA_ENDPOINT}={nli};"
        ));
    }
    world.retrieval_review_extra = Some(vec![
        (
            "GIVN_SPEC_BGE_DIRECTIONAL_SCORES".to_string(),
            bge_scores.trim_end_matches(';').to_string(),
        ),
        (
            "GIVN_SPEC_NLI_DIRECTIONAL_SCORES".to_string(),
            nli_scores.trim_end_matches(';').to_string(),
        ),
    ]);
}

fn recommended_nli_entailments(world: &GivnWorld) -> Vec<f32> {
    super::stdout(world)
        .lines()
        .filter_map(|line| {
            if line.starts_with("nli candidate [") && line.contains("symmetric [entailment ") {
                line.split("symmetric [entailment ")
                    .nth(1)?
                    .split(',')
                    .next()?
                    .trim()
                    .parse::<f32>()
                    .ok()
            } else {
                None
            }
        })
        .collect()
}

fn recommended_bge_raws(world: &GivnWorld) -> Vec<f32> {
    super::stdout(world)
        .lines()
        .filter(|line| line.starts_with("BGE_RECOMMENDED:"))
        .filter_map(|line| {
            line.split("symmetric ")
                .nth(1)?
                .split(';')
                .next()?
                .trim()
                .parse::<f32>()
                .ok()
        })
        .collect()
}

#[then("the recommended NLI pairs are listed strongest symmetric entailment first")]
fn nli_recommended_order(world: &mut GivnWorld) {
    let values = recommended_nli_entailments(world);
    assert!(values.len() >= 2, "stdout: {}", super::stdout(world));
    let mut sorted = values.clone();
    sorted.sort_by(|left, right| right.total_cmp(left));
    assert_eq!(values, sorted, "stdout: {}", super::stdout(world));
}

#[then("the recommended BGE pairs are listed highest symmetric raw score first")]
fn bge_recommended_order(world: &mut GivnWorld) {
    let values = recommended_bge_raws(world);
    assert!(values.len() >= 2, "stdout: {}", super::stdout(world));
    let mut sorted = values.clone();
    sorted.sort_by(|left, right| right.total_cmp(left));
    assert_eq!(values, sorted, "stdout: {}", super::stdout(world));
}

#[then("neither listing follows embedding-score order")]
fn listings_differ_from_embedding_order(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let first_bge_title = output
        .lines()
        .find(|line| line.starts_with("BGE_RECOMMENDED:"))
        .and_then(|line| line.split('\'').nth(1))
        .expect("first BGE recommended title");
    assert_ne!(
        first_bge_title, "Provider request is accepted",
        "embedding-order title listed first: {output}"
    );
}

#[then(
    "stdout keeps the exact stage status line naming the model, pair budget, considered pairs, and diagnostic-only disclaimer"
)]
fn exact_stage_status_line(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output
            .lines()
            .any(|line| line.starts_with(
                "nli: model MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7, top_k 5, considered 3 candidate pairs; evidence is diagnostic only"
            )),
        "stdout: {output}"
    );
    assert!(
        output
            .lines()
            .any(|line| line.starts_with("BGE: model rozgo/bge-reranker-v2-m3, top_k 5,")),
        "stdout: {output}"
    );
}

#[then(
    "stdout keeps the exact accounting line naming raw, maximum, recommended, filtered, and unscored counts"
)]
fn exact_accounting_line(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output.lines().any(|line| line.starts_with(
            "NLI candidate accounting: raw 3, max pairs 2048, recommended 1, NLI filtered 0, unscored 2, bounded pool"
        )),
        "stdout: {output}"
    );
    assert!(
        output.lines().any(|line| line.starts_with(
            "BGE candidate accounting: recommended 1, BGE filtered 2, unscored 0, budget <= 2048"
        )),
        "stdout: {output}"
    );
}

#[then(
    "stdout keeps the exact distribution line naming entailment, neutral, and contradiction counts"
)]
fn exact_distribution_line(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output
            .lines()
            .any(|line| line == "nli distribution: entailment 1, neutral 0, contradiction 0"),
        "stdout: {output}"
    );
}

#[then(
    "each reported pair keeps the exact established detail line including its advisory disclaimer"
)]
fn exact_pair_detail(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let detail = output
        .lines()
        .find(|line| line.starts_with("nli candidate ["))
        .expect("NLI detail line");
    for fragment in [
        "; forward [entailment ",
        "], reverse [entailment ",
        "], symmetric [entailment ",
        "strongest ",
        "; evidence is advisory and no Givn classification is assigned",
    ] {
        assert!(detail.contains(fragment), "{detail}");
    }
    let bge_detail = output
        .lines()
        .find(|line| line.starts_with("BGE_RECOMMENDED:"))
        .expect("BGE detail line");
    for fragment in [
        " E5 ",
        ", forward ",
        ", reverse ",
        ", symmetric ",
        "; evidence is advisory and no Givn classification is assigned",
    ] {
        assert!(bge_detail.contains(fragment), "{bge_detail}");
    }
}

#[then("stdout lists every filtered pair in embedding-score order after the recommended pairs")]
fn filtered_after_recommended(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let bge_titles: Vec<String> = output
        .lines()
        .filter(|line| line.starts_with("BGE_"))
        .filter_map(|line| {
            let title = line.split('\u{27}').nth(1)?.to_string();
            Some(title)
        })
        .collect();
    assert_eq!(bge_titles.len(), 3, "stdout: {output}");
    assert_eq!(
        bge_titles.first().map(String::as_str),
        Some("Provider request is accepted"),
        "recommended first: {output}"
    );
    let mut filtered_titles = bge_titles[1..].to_vec();
    filtered_titles.sort();
    assert_eq!(
        filtered_titles,
        vec![
            "Provider timeout is reported".to_string(),
            "Provider warning is reported".to_string(),
        ],
        "filtered in embedding order after recommended: {output}"
    );
    let nli_detail_lines: Vec<&str> = output
        .lines()
        .filter(|line| line.starts_with("nli candidate ["))
        .collect();
    assert_eq!(nli_detail_lines.len(), 1, "stdout: {output}");
    let caps = output
        .lines()
        .filter(|line| line.contains("token-cap: NLI_TOKEN_CAP"))
        .count();
    assert_eq!(caps, 2, "capped pairs listed as guidance: {output}");
    let first_nli_title = nli_detail_lines[0].split('\u{27}').nth(1).expect("title");
    assert_eq!(
        first_nli_title, "Provider request is accepted",
        "recommended NLI pair first: {output}"
    );
}

#[then("stdout lists any unscored or token-capped pair with its admission status")]
fn cap_pairs_listed(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let caps = output
        .lines()
        .filter(|line| line.contains("token-cap: NLI_TOKEN_CAP"))
        .count();
    assert_eq!(caps, 2, "token-cap guidance present when verbose: {output}");
}

#[then("no listing limit applies to any stage")]
fn no_listing_limits(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let accounting = output
        .lines()
        .find(|line| line.contains("NLI candidate accounting:"))
        .expect("NLI accounting");
    let raw: usize = accounting
        .split("raw ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse().ok())
        .expect("raw");
    let unscored: usize = accounting
        .split("unscored ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let details = output
        .lines()
        .filter(|line| line.starts_with("nli candidate ["))
        .count();
    assert_eq!(
        details + unscored,
        raw,
        "every pair accounted for in verbose listing: {output}"
    );
    assert!(
        !output.contains("nli diagnostic: reported"),
        "no truncation disclaimer"
    );
}

#[given(
    "a project prepared for semantic review whose stubbed stages emit repeated resolution notices"
)]
fn review_resolution_seam(world: &mut GivnWorld) {
    review_recommends_one_filters_many(world);
    world
        .retrieval_review_extra
        .get_or_insert_with(Vec::new)
        .push((
            "GIVN_SPEC_RESOLUTION_NOTICES".to_string(),
            "repeat".to_string(),
        ));
}

#[then("stderr shows exactly one artifact resolution summary line per participating stage")]
fn one_resolution_summary_per_stage(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    let summaries: Vec<&str> = stderr
        .lines()
        .filter(|line| line.contains(" ready ("))
        .collect();
    assert_eq!(summaries.len(), 3, "stderr: {stderr}");
    assert!(
        summaries
            .iter()
            .any(|l| l.contains("(E5)") && l.ends_with("/stub/e5-cache)")),
        "E5 summary missing: {stderr}"
    );
    assert!(
        summaries
            .iter()
            .any(|l| l.contains("(BGE)") && l.ends_with("/stub/bge-cache)")),
        "BGE summary missing: {stderr}"
    );
    assert!(
        summaries
            .iter()
            .any(|l| l.contains("NLI") && l.ends_with("/stub/nli-cache)")),
        "NLI summary missing: {stderr}"
    );
}

#[then("stderr contains no repeated resolution lines for the same stage")]
fn no_repeated_resolution_lines(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for stage in ["(E5)", "(BGE)", "NLI"] {
        let count = stderr
            .lines()
            .filter(|line| line.contains(stage) && line.contains(" ready ("))
            .count();
        assert!(count <= 1, "{stage} summarized {count} times: {stderr}");
    }
    assert!(
        !stderr.contains("resolving "),
        "resolution start lines suppressed by default: {stderr}"
    );
}

#[then("stderr contains the full repeated resolution lines for every participating stage")]
fn verbose_repeated_resolution_lines(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    for (stage, detail) in [
        ("retrieval model artifacts (E5)", "/stub/e5-cache"),
        ("reranker model artifacts (BGE)", "/stub/bge-cache"),
        ("NLI model artifacts", "/stub/nli-cache"),
    ] {
        let starts = stderr
            .lines()
            .filter(|line| *line == format!("resolving {stage} into {detail}"))
            .count();
        let dones = stderr
            .lines()
            .filter(|line| *line == format!("{stage} resolved"))
            .count();
        assert_eq!(starts, 2, "{stage}: {stderr}");
        assert_eq!(dones, 2, "{stage}: {stderr}");
    }
}

#[then("stdout contains no truncation disclaimer for either second-stage stage")]
fn no_truncation_disclaimer(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        !output.contains("nli diagnostic: reported"),
        "NLI truncation disclaimer present: {output}"
    );
    assert!(
        !output.contains("reported at most"),
        "truncation disclaimer present: {output}"
    );
}

#[then("stdout still contains no truncation disclaimer")]
fn still_no_truncation_disclaimer(world: &mut GivnWorld) {
    no_truncation_disclaimer(world);
}

#[given("the deprecated report-limit environment variable is set to \"1\"")]
fn deprecated_report_limit_set(world: &mut GivnWorld) {
    world
        .retrieval_review_extra
        .get_or_insert_with(Vec::new)
        .push(("GIVN_SPEC_NLI_REPORT_LIMIT".to_string(), "1".to_string()));
}

fn normalized_lines(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|line| !line.contains("runtime ms"))
        .collect()
}

#[then("every line except elapsed-runtime values matches the same run without the variable")]
fn output_matches_run_without_variable(world: &mut GivnWorld) {
    let with_var = super::stdout(world);
    let extra = world.retrieval_review_extra.as_mut().expect("extra envs");
    extra.retain(|(key, _)| key != "GIVN_SPEC_NLI_REPORT_LIMIT");
    run_review_with(world, &[], &[]);
    let without_var = super::stdout(world);
    assert_eq!(
        normalized_lines(&with_var),
        normalized_lines(&without_var),
        "deprecated variable changed output"
    );
}

#[then("no warning or error names the deprecated variable")]
fn nothing_names_deprecated_variable(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        !stderr.contains("GIVN_SPEC_NLI_REPORT_LIMIT") && !stderr.contains("REPORT_LIMIT"),
        "stderr: {stderr}"
    );
}
