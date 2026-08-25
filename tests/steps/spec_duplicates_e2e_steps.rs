//! E2E black-box steps for the permanent scenario duplicate audit.

use std::fs;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

const AUTH_FEATURE: &str = r#"Feature: auth

  Scenario: Unknown provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Token is required
    Given a client request
    When the client sends a request without credentials
    Then the server responds with unauthorized status
"#;

const PROVIDER_FEATURE: &str = r#"Feature: provider

  Scenario: Provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request

  Scenario: Provider exits successfully
    Given a configured provider catalog
    When the provider command is run normally
    Then the command exits 0 and output "contains" "ready"
      | provider |
      | aws      |

  Scenario: Provider exits with a warning
    Given a configured provider catalog
    When the provider command is run in TTY mode
    Then the command exits 2 and output "not contains" "ready"
      | provider |
      | gcp      |
"#;

const TOKEN_FEATURE: &str = r#"Feature: token

  Scenario: Authentication needs a token
    Given an identity service
    When the client authenticates without a token
    Then access is denied with message "token required"
"#;

const DUPLICATE_SCORE_OVERRIDE: &str = "givn/specs/auth/auth.feature|Token is required::givn/specs/token/token.feature|Authentication needs a token=0.9600";

const EXPECTED_TEXT_FIELDS: [&str; 9] = [
    "kind=",
    "left=",
    "right=",
    "relation=",
    "assertion_shape=",
    "facts=",
    "distinguishing_facts=",
    "fact_contradiction=",
    "score=",
];

#[given(
    "an end-to-end project with permanent scenarios containing duplicate and related behaviour"
)]
fn e2e_duplicate_audit_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }

    for (path, content) in [
        ("givn/specs/auth/auth.feature", AUTH_FEATURE),
        ("givn/specs/provider/provider.feature", PROVIDER_FEATURE),
        ("givn/specs/token/token.feature", TOKEN_FEATURE),
    ] {
        let feature = env.path(path);
        fs::create_dir_all(feature.parent().expect("feature parent"))
            .expect("create feature directory");
        fs::write(feature, content).expect("write permanent feature");
    }

    world.env = Some(env);
}

#[when("the end-to-end user runs `givn spec duplicates`")]
fn e2e_run_duplicate_audit(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit project is missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "duplicates"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            ("GIVN_SPEC_RERANKER", "stub"),
            ("GIVN_SPEC_DUPLICATES_TEST_SCORES", DUPLICATE_SCORE_OVERRIDE),
        ],
    ));
}

#[then("stdout starts with the exact duplicate-audit header and counts")]
fn e2e_duplicate_audit_header(world: &mut GivnWorld) {
    let lines = report_lines(world);
    assert_eq!(lines[0], "Givn spec duplicates");
    assert_eq!(lines[1], "Scanned scenarios: 6");
    assert_eq!(lines[2], "Candidate count: 3");
}

#[then(
    "stdout uses candidate fields in the exact order `rank,kind,left,right,relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score`"
)]
fn e2e_duplicate_audit_field_order(world: &mut GivnWorld) {
    for line in report_lines(world).iter().skip(3) {
        assert!(
            line.starts_with("rank "),
            "candidate is missing rank: {line}"
        );
        let mut cursor = line
            .find(": ")
            .expect("candidate rank has a field separator")
            + 2;
        for field in EXPECTED_TEXT_FIELDS {
            let relative = line[cursor..]
                .find(field)
                .unwrap_or_else(|| panic!("candidate is missing field {field}: {line}"));
            cursor += relative + field.len();
        }
        assert_eq!(
            line.matches("kind=").count(),
            1,
            "candidate has unexpected kind fields: {line}"
        );
    }
}

#[then(
    "stdout identifies rank 1 as the exact duplicate between `givn/specs/auth/auth.feature` scenario \"Unknown provider is rejected\" and `givn/specs/provider/provider.feature` scenario \"Provider is rejected\""
)]
fn e2e_exact_duplicate_rank_one(world: &mut GivnWorld) {
    let line = candidate_line(world, 1);
    assert_candidate_endpoints(
        &line,
        "givn/specs/auth/auth.feature",
        "Unknown provider is rejected",
        "givn/specs/provider/provider.feature",
        "Provider is rejected",
    );
    assert!(line.contains("kind=exact_duplicate"));
    assert!(line.contains("relation=identical_behavior"));
    assert!(line.contains("assertion_shape=null facts=null"));
    assert!(line.ends_with("score=null"));
}

#[then(
    "stdout identifies rank 2 as the structural overlap between `givn/specs/provider/provider.feature` scenario \"Provider exits successfully\" and `givn/specs/provider/provider.feature` scenario \"Provider exits with a warning\""
)]
fn e2e_structural_overlap_rank_two(world: &mut GivnWorld) {
    let line = candidate_line(world, 2);
    assert_candidate_endpoints(
        &line,
        "givn/specs/provider/provider.feature",
        "Provider exits successfully",
        "givn/specs/provider/provider.feature",
        "Provider exits with a warning",
    );
    assert!(line.contains("kind=structural_overlap"));
    assert!(line.contains("relation=same_assertion_shape"));
    assert!(line.contains("assertion_shape={"));
    assert!(line.contains("facts={"));
    assert!(line.ends_with("score=null"));
}

#[then(
    "stdout identifies rank 3 as the semantic candidate between `givn/specs/auth/auth.feature` scenario \"Token is required\" and `givn/specs/token/token.feature` scenario \"Authentication needs a token\""
)]
fn e2e_semantic_candidate_rank_three(world: &mut GivnWorld) {
    let line = candidate_line(world, 3);
    assert_candidate_endpoints(
        &line,
        "givn/specs/auth/auth.feature",
        "Token is required",
        "givn/specs/token/token.feature",
        "Authentication needs a token",
    );
    assert!(line.contains("kind=semantic_candidate"));
    assert!(line.contains("relation=semantic_similarity"));
    assert!(line.contains("assertion_shape=null facts=null"));
    assert!(line.ends_with("score=0.9600"));
}

#[then("every text candidate rank is contiguous from 1 through the reported candidate count")]
fn e2e_duplicate_audit_ranks_are_contiguous(world: &mut GivnWorld) {
    let lines = report_lines(world);
    let candidate_count = lines[2]
        .strip_prefix("Candidate count: ")
        .expect("candidate count header")
        .parse::<usize>()
        .expect("candidate count is an integer");
    let ranks = lines
        .iter()
        .skip(3)
        .map(|line| {
            line.strip_prefix("rank ")
                .and_then(|line| line.split_once(": "))
                .and_then(|(rank, _)| rank.parse::<usize>().ok())
                .expect("candidate rank is an integer")
        })
        .collect::<Vec<_>>();
    assert_eq!(ranks.len(), candidate_count);
    assert_eq!(ranks, (1..=candidate_count).collect::<Vec<_>>());
}

fn report_lines(world: &GivnWorld) -> Vec<String> {
    let output = world
        .last_output
        .as_ref()
        .expect("duplicate audit output is missing");
    assert!(
        output.status.success(),
        "duplicate audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = super::stdout(world);
    assert!(
        stdout.ends_with('\n'),
        "duplicate audit is missing its final newline"
    );
    stdout.lines().map(str::to_string).collect()
}

fn candidate_line(world: &GivnWorld, rank: usize) -> String {
    report_lines(world)
        .into_iter()
        .find(|line| line.starts_with(&format!("rank {rank}: ")))
        .unwrap_or_else(|| panic!("duplicate audit rank {rank} is missing"))
}

fn assert_candidate_endpoints(
    line: &str,
    left_file: &str,
    left_title: &str,
    right_file: &str,
    right_title: &str,
) {
    let left = serde_json::json!({"file": left_file, "title": left_title});
    let right = serde_json::json!({"file": right_file, "title": right_title});
    assert!(
        line.contains(&format!("left={left}")),
        "left endpoint mismatch: {line}"
    );
    assert!(
        line.contains(&format!("right={right}")),
        "right endpoint mismatch: {line}"
    );
}
