//! Compiled-CLI subprocess steps for decision-first semantic review output.
//!
//! These bindings drive the real `givn` binary as a subprocess against a
//! prepared project — the same interface an external client uses.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

const DELTA_ENDPOINT: &str =
    "givn/specs/running-change/running-change.feature|Active provider request";
const PERMANENT_ENDPOINTS: [&str; 3] = [
    "givn/specs/provider.feature|Provider request is accepted",
    "givn/specs/provider.feature|Provider timeout is reported",
    "givn/specs/provider.feature|Provider warning is reported",
];
/// The delta body differs from the permanent corpus so the deterministic
/// exact-duplicate gate does not swallow the pairs; the stub pair scores
/// lift them above the boundary instead.
const PERMANENT_BODY: &str = "    Given a configured provider catalog\n    When the user selects a known provider\n    Then the command accepts the request\n";
const DELTA_BODY: &str = "    Given a configured provider catalogue\n    When the user picks a known provider\n    Then the request is accepted\n";

fn write_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated specs");
    fs::create_dir_all(&specs).expect("create permanent specs");
    let mut permanent = String::from("Feature: provider\n");
    for endpoint in PERMANENT_ENDPOINTS {
        let title = endpoint.split('|').nth(1).expect("endpoint title");
        permanent.push_str(&format!("\n  Scenario: {title}\n{PERMANENT_BODY}"));
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
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Provider request is accepted` | `Active provider request` | boundary |\n| `Provider timeout is reported` | `Active provider request` | boundary |\n| `Provider warning is reported` | `Active provider request` | boundary |\n",
    )
    .expect("write dispositions");
    // Neutralise the project-owned verify hooks for the blocking surface.
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write stub commands");
    world.env = Some(env);
}

fn base_envs(_world: &GivnWorld) -> Vec<(String, String)> {
    // The advisory pool keys the delta endpoint by its projected permanent
    // identity; the blocking report keys it by the source file.
    const DELTA_SOURCE: &str =
        "givn/changes/running-change/specs/running.feature|Active provider request";
    let mut bge_scores = String::new();
    let mut nli_scores = String::new();
    let confidences = [
        (PERMANENT_ENDPOINTS[0], "4.0", Some("2.2,-0.2,0.1")),
        (PERMANENT_ENDPOINTS[1], "1.0", Some("-0.4,1.4,0.2")),
        (PERMANENT_ENDPOINTS[2], "1.0", Some("-0.4,1.4,0.2")),
    ];
    for (right, bge, nli) in confidences {
        for delta_endpoint in [DELTA_ENDPOINT, DELTA_SOURCE] {
            bge_scores.push_str(&format!(
                "{delta_endpoint}->{right}={bge};{right}->{delta_endpoint}={bge};"
            ));
            if let Some(nli) = nli {
                nli_scores.push_str(&format!(
                    "{delta_endpoint}->{right}={nli};{right}->{delta_endpoint}={nli};"
                ));
            }
        }
    }
    vec![
        ("GIVN_SPEC_EMBEDDER".to_string(), "stub".to_string()),
        ("GIVN_SPEC_BGE".to_string(), "stub".to_string()),
        ("GIVN_SPEC_RERANKER".to_string(), "nli-stub".to_string()),
        (
            "GIVN_SPEC_BGE_DIRECTIONAL_SCORES".to_string(),
            bge_scores.trim_end_matches(';').to_string(),
        ),
        (
            "GIVN_SPEC_NLI_DIRECTIONAL_SCORES".to_string(),
            nli_scores.trim_end_matches(';').to_string(),
        ),
    ]
}

fn run_review(world: &mut GivnWorld, extra_args: &[&str]) {
    let env = world.env.as_ref().expect("e2e review project missing");
    let mut args: Vec<&str> = vec!["spec", "review", "--change", "running-change"];
    args.extend_from_slice(extra_args);
    let envs = base_envs(world);
    let environments: Vec<(&str, &str)> = envs
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    world.last_output = Some(common::run_givn_with_envs(env, &args, &environments));
}

#[given("an end-to-end project whose semantic review recommends one pair and filters many")]
fn e2e_review_output_project(world: &mut GivnWorld) {
    write_project(world);
}

#[given("an end-to-end project whose second-stage evidence recommends at least one pair")]
fn e2e_recommends_at_least_one(world: &mut GivnWorld) {
    write_project(world);
}

#[when(
    "the end-to-end user runs the advisory semantic review for \"running-change\" through the givn CLI"
)]
fn e2e_run_review_default(world: &mut GivnWorld) {
    run_review(world, &[]);
}

#[when(
    "the end-to-end user runs the advisory semantic review with verbose output through the givn CLI"
)]
fn e2e_run_review_verbose(world: &mut GivnWorld) {
    run_review(world, &["--verbose"]);
}

#[when(
    "the end-to-end user runs the advisory semantic review with structured JSON output through the givn CLI"
)]
fn e2e_run_review_json(world: &mut GivnWorld) {
    run_review(world, &["--format", "json"]);
}

#[when(
    "the end-to-end user runs the blocking workflow review for \"running-change\" through the givn CLI"
)]
fn e2e_run_blocking_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e review project missing");
    let mut envs = base_envs(world);
    // The blocking report derives its E5 candidates from the deterministic
    // semantic-report score fixture.
    let mut stub_scores = String::new();
    for right in PERMANENT_ENDPOINTS {
        let title = right.split('|').nth(1).expect("title");
        stub_scores.push_str(&format!("{title}|Active provider request=0.9600;"));
    }
    envs.push((
        "GIVN_SPEC_STUB_PAIR_SCORES".to_string(),
        stub_scores.trim_end_matches(';').to_string(),
    ));
    let environments: Vec<(&str, &str)> = envs
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "running-change"],
        &environments,
    ));
}

#[then("stdout shows the recommended pair's detail line")]
fn e2e_shows_recommended(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        output
            .lines()
            .any(|line| line.starts_with("BGE_RECOMMENDED:")
                && line.contains("Provider request is accepted")),
        "recommended pair not shown: {output}"
    );
}

#[then("stdout shows no FILTERED detail line")]
fn e2e_hides_filtered(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        !output.lines().any(|line| line.starts_with("BGE_FILTERED:")),
        "filtered detail leaked: {output}"
    );
}

#[then("stdout shows the FILTERED detail rows")]
fn e2e_verbose_shows_filtered(world: &mut GivnWorld) {
    let output = super::stdout(world);
    let filtered = output
        .lines()
        .filter(|line| line.starts_with("BGE_FILTERED:"))
        .count();
    assert_eq!(filtered, 2, "verbose filtered rows missing: {output}");
}

#[then("stdout shows no truncation disclaimer")]
fn e2e_no_truncation_disclaimer(world: &mut GivnWorld) {
    let output = super::stdout(world);
    assert!(
        !output.contains("nli diagnostic: reported"),
        "truncation disclaimer present: {output}"
    );
}

#[then("stdout parses as a schema-version-1 review document")]
fn e2e_json_parses(world: &mut GivnWorld) {
    let stdout = String::from_utf8_lossy(&super::output(world).stdout);
    assert!(
        stdout.trim_start().starts_with('{'),
        "not a pure JSON document: {stdout}"
    );
    let document: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("valid review JSON");
    assert_eq!(document["schema_version"], 1);
}

#[then("the JSON counters equal those of an equivalent text run")]
fn e2e_json_counter_parity(world: &mut GivnWorld) {
    let json_stdout = super::stdout(world);
    run_review(world, &[]);
    let text = super::stdout(world);
    let document: serde_json::Value = serde_json::from_str(&json_stdout).expect("review JSON");
    for (label, field, key) in [
        ("BGE candidate accounting:", "bge_run", "recommended"),
        ("NLI candidate accounting:", "nli_run", "recommended"),
    ] {
        let expected: u64 = text
            .lines()
            .find(|line| line.contains(label))
            .unwrap_or_else(|| panic!("missing {label}"))
            .split(if key == "filtered" {
                "filtered "
            } else {
                "recommended "
            })
            .nth(1)
            .and_then(|value| value.split(',').next())
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or_else(|| panic!("missing count in {label}"));
        assert_eq!(document[field][format!("{key}_count")], expected);
    }
}

#[then("givn/changes/running-change/review-findings.json exists beside the change")]
fn e2e_findings_exists(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e review project missing");
    assert!(
        env.path("givn/changes/running-change/review-findings.json")
            .exists(),
        "findings file missing"
    );
}

#[then("the persisted document matches the document emitted on stdout")]
fn e2e_persisted_matches_stdout(world: &mut GivnWorld) {
    run_review(world, &["--format", "json"]);
    let env = world.env.as_ref().expect("e2e review project missing");
    let findings_path = env.path("givn/changes/running-change/review-findings.json");
    let persisted: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(findings_path).expect("findings"))
            .expect("persisted parses");
    let emitted: serde_json::Value =
        serde_json::from_slice(&super::output(world).stdout).expect("emitted parses");
    assert_eq!(persisted["verdict"], emitted["verdict"]);
    assert_eq!(persisted["candidates"], emitted["candidates"]);
    assert_eq!(
        persisted["combined_recommended_candidates"],
        emitted["combined_recommended_candidates"]
    );
}

#[then("the command exits non-zero because the recommended candidate is unresolved")]
fn e2e_blocking_exits_nonzero(world: &mut GivnWorld) {
    assert!(
        !super::output(world).status.success(),
        "expected blocking failure, stdout: {}",
        super::stdout(world)
    );
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("semantic review blocked") || stderr.contains("semantic review flagged"),
        "stderr: {stderr}"
    );
}

#[then("stderr names the flagged recommendation failure")]
fn e2e_stderr_flagged(world: &mut GivnWorld) {
    let stderr = super::stderr(world);
    assert!(
        stderr.contains("semantic review flagged") || stderr.contains("semantic review blocked"),
        "stderr: {stderr}"
    );
}

#[then("the persisted semantic review report records the blocked status")]
fn e2e_report_blocked(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e review project missing");
    let report =
        std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
            .expect("report");
    assert!(
        report.contains("Status: BLOCKED"),
        "expected blocked report, got:\n{}",
        report.lines().take(8).collect::<Vec<_>>().join("\n")
    );
}

#[then("the command stdout names the complete saved semantic-review report path")]
fn e2e_stdout_report_path(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("e2e review project missing");
    let expected_suffix = "givn/changes/running-change/semantic-review.md";
    let output = super::stdout(world);
    let line = output
        .lines()
        .find(|line| line.starts_with("semantic review report saved: "))
        .unwrap_or_else(|| panic!("missing report-saved line in stdout: {output}"));
    let path = line.trim_start_matches("semantic review report saved: ").trim();
    assert!(
        path.ends_with(expected_suffix),
        "report path {path:?} does not end with {expected_suffix:?}"
    );
    assert!(
        env.path("givn/changes/running-change/semantic-review.md").exists(),
        "named report path does not exist on disk"
    );
}
