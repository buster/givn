//! Regular black-box steps for the permanent scenario duplicate audit.

use std::fs;
use std::process::Output;

use cucumber::{given, then, when};

use super::{GivnWorld, stdout};
use crate::common;

const EXACT_DUPLICATE_FEATURE: &str = "Feature: provider\n\n  Scenario: Provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";
const CROSS_FEATURE_PROVIDER: &str = "Feature: provider\n\n  Scenario: Provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";
const CROSS_FEATURE_AUTH: &str = "Feature: auth\n\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";
const STRUCTURAL_OVERLAP_FEATURE: &str = r#"Feature: provider

  Scenario: Provider exits successfully
    Given a configured provider catalog
    When the provider command is run normally
    Then the command exits 0 and output "contains" "ready"
      | provider |
      | aws      |

  Scenario: [DONE] Provider exits with a warning
    Given a configured provider catalog
    When the provider command is run in TTY mode
    Then the command exits 2 and output "not contains" "ready"
      | provider |
      | gcp      |
"#;
const STRUCTURAL_FACTS: &str = r#"facts={"left":{"exit_code":"0","contains_polarity":{"0":"positive"},"tables":{"provider":"aws"},"done":false,"tty":false},"right":{"exit_code":"2","contains_polarity":{"0":"negative"},"tables":{"provider":"gcp"},"done":true,"tty":true}}"#;
const SHORT_ASSERTION_FEATURE: &str = r#"Feature: provider assertions

  Scenario: Short provider assertion
    Given a configured provider catalog
    When the provider command is run
    Then the command exits 0
"#;
const LONG_ASSERTION_FEATURE: &str = r#"Feature: provider assertions

  Scenario: Long provider assertion
    Given a configured provider catalog
    When the provider command is run
    Then the command exits 0
    And the output contains 1
"#;
const SEMANTIC_AUTH_FEATURE: &str = r#"Feature: auth

  Scenario: Token is required
    Given a client request
    When the client sends a request without credentials
    Then the server responds with unauthorized status
"#;
const SEMANTIC_TOKEN_FEATURE: &str = r#"Feature: token

  Scenario: Authentication needs a token
    Given an identity service
    When the client authenticates without a token
    Then access is denied with message "token required"
"#;
const SEMANTIC_ACCOUNT_FEATURE: &str = r#"Feature: account

  Scenario: Account access needs credentials
    Given an account gateway
    When the client sends a request without a login
    Then the account service denies access with message "credentials required"
"#;
const SEMANTIC_SESSION_FEATURE: &str = r#"Feature: session

  Scenario: Session credentials are needed
    Given a session gateway
    When the client connects without a login
    Then the session service refuses access with message "credentials required"
"#;
const MANY_DUPLICATE_FEATURES: [(&str, &str); 6] = [
    (
        "givn/specs/alpha/alpha.feature",
        r#"Feature: alpha

  Scenario: Alpha behavior
    Given alpha setup
    When alpha action
    Then alpha result is stable

  Scenario: Alpha behavior copy
    Given alpha setup
    When alpha action
    Then alpha result is stable
"#,
    ),
    (
        "givn/specs/bravo/bravo.feature",
        r#"Feature: bravo

  Scenario: Bravo behavior
    Given bravo setup
    When bravo action
    Then bravo result is stable

  Scenario: Bravo behavior copy
    Given bravo setup
    When bravo action
    Then bravo result is stable
"#,
    ),
    (
        "givn/specs/charlie/charlie.feature",
        r#"Feature: charlie

  Scenario: Charlie behavior
    Given charlie setup
    When charlie action
    Then charlie result is stable

  Scenario: Charlie behavior copy
    Given charlie setup
    When charlie action
    Then charlie result is stable
"#,
    ),
    (
        "givn/specs/delta/delta.feature",
        r#"Feature: delta

  Scenario: Delta behavior
    Given delta setup
    When delta action
    Then delta result is stable

  Scenario: Delta behavior copy
    Given delta setup
    When delta action
    Then delta result is stable
"#,
    ),
    (
        "givn/specs/echo/echo.feature",
        r#"Feature: echo

  Scenario: Echo behavior
    Given echo setup
    When echo action
    Then echo result is stable

  Scenario: Echo behavior copy
    Given echo setup
    When echo action
    Then echo result is stable
"#,
    ),
    (
        "givn/specs/foxtrot/foxtrot.feature",
        r#"Feature: foxtrot

  Scenario: Foxtrot behavior
    Given foxtrot setup
    When foxtrot action
    Then foxtrot result is stable

  Scenario: Foxtrot behavior copy
    Given foxtrot setup
    When foxtrot action
    Then foxtrot result is stable
"#,
    ),
];

#[derive(Debug, PartialEq, Eq)]
struct DuplicateAuditReport {
    scanned_scenarios: usize,
    candidate_count: usize,
    candidates: Vec<DuplicateAuditCandidate>,
}

#[derive(Debug, PartialEq, Eq)]
struct DuplicateAuditCandidate {
    rank: usize,
    evidence: String,
}

#[given(
    regex = r##"^the permanent scenario "([^"]+)" at "([^"]+)" has the assertion sequence of "([^"]+)" at "([^"]+)"$"##
)]
fn structural_subset_fixture(
    world: &mut GivnWorld,
    short_title: String,
    short_file: String,
    long_title: String,
    long_file: String,
) {
    assert_eq!(short_title, "Short provider assertion");
    assert_eq!(short_file, "givn/specs/a/a.feature");
    assert_eq!(long_title, "Long provider assertion");
    assert_eq!(long_file, "givn/specs/b/b.feature");
    permanent_project(
        world,
        &[
            ("givn/specs/a/a.feature", SHORT_ASSERTION_FEATURE),
            ("givn/specs/b/b.feature", LONG_ASSERTION_FEATURE),
        ],
    );
}

#[given(
    "permanent scenarios share the same masked assertion shape but have different fingerprint facts"
)]
fn structural_overlap_fixture(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[(
            "givn/specs/provider/provider.feature",
            STRUCTURAL_OVERLAP_FEATURE,
        )],
    );
}

#[then("candidate rank 1 is classified as `structural_overlap`")]
fn structural_overlap_kind(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("kind=structural_overlap"));
}

#[then("candidate rank 1 has relation `same_assertion_shape`")]
fn structural_overlap_relation(world: &mut GivnWorld) {
    let line = candidate_line(world);
    assert!(line.contains("relation=same_assertion_shape"));
    assert!(line.contains(
        r#"assertion_shape={"left":["the command exits * and output \"*\" \"*\""],"right":["the command exits * and output \"*\" \"*\""]}"#
    ));
}

#[then(r#"candidate rank 1 has left fingerprint facts `{"exit_code":"0","contains_polarity":{"0":"positive"},"tables":{"provider":"aws"},"done":false,"tty":false}`"#)]
fn structural_overlap_left_facts(world: &mut GivnWorld) {
    let line = candidate_line(world);
    assert!(line.contains(STRUCTURAL_FACTS));
}

#[then(r#"candidate rank 1 has right fingerprint facts `{"exit_code":"2","contains_polarity":{"0":"negative"},"tables":{"provider":"gcp"},"done":true,"tty":true}`"#)]
fn structural_overlap_right_facts(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains(STRUCTURAL_FACTS));
}

#[then(r#"candidate rank 1 has distinguishing facts `[{"field":"exit_code","left":"0","right":"2"},{"field":"contains_polarity[0]","left":"positive","right":"negative"},{"field":"tables[provider]","left":"aws","right":"gcp"},{"field":"done","left":false,"right":true},{"field":"tty","left":false,"right":true}]`"#)]
fn structural_overlap_distinguishing_facts(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains(
        r#"distinguishing_facts=[{"field":"exit_code","left":"0","right":"2"},{"field":"contains_polarity[0]","left":"positive","right":"negative"},{"field":"tables[provider]","left":"aws","right":"gcp"},{"field":"done","left":false,"right":true},{"field":"tty","left":false,"right":true}]"#
    ));
}

#[then("candidate rank 1 has fact contradiction `true`")]
fn structural_overlap_contradiction(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("fact_contradiction=true"));
}

#[then("candidate rank 1 has no Given/When action-structure evidence")]
fn structural_overlap_no_action_evidence(world: &mut GivnWorld) {
    let line = candidate_line(world);
    assert!(!line.contains("Given"));
    assert!(!line.contains("When"));
    assert!(!line.contains("action-structure"));
}

#[then("candidate rank 1 has relation `left_assertions_subset_of_right`")]
fn structural_subset_relation(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("relation=left_assertions_subset_of_right"));
}

#[then(r#"candidate rank 1 has assertion shapes `{"left":["the command exits *"],"right":["the command exits *","the output contains *"]}`"#)]
fn structural_subset_shapes(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains(
        r#"assertion_shape={"left":["the command exits *"],"right":["the command exits *","the output contains *"]}"#
    ));
}

#[given(
    regex = r##"^the permanent scenario "([^"]+)" at "([^"]+)" and "([^"]+)" at "([^"]+)" describe closely related behaviour with different wording$"##
)]
fn semantic_candidate_fixture(
    world: &mut GivnWorld,
    left_title: String,
    left_file: String,
    right_title: String,
    right_file: String,
) {
    assert_eq!(left_title, "Token is required");
    assert_eq!(left_file, "givn/specs/auth/auth.feature");
    assert_eq!(right_title, "Authentication needs a token");
    assert_eq!(right_file, "givn/specs/token/token.feature");
    permanent_project(
        world,
        &[
            ("givn/specs/auth/auth.feature", SEMANTIC_AUTH_FEATURE),
            ("givn/specs/token/token.feature", SEMANTIC_TOKEN_FEATURE),
        ],
    );
    world.retrieval_duplicate_scores = Some(
        "givn/specs/auth/auth.feature|Token is required::givn/specs/token/token.feature|Authentication needs a token=0.9600".to_string(),
    );
}

#[then("candidate rank 1 is classified as `semantic_candidate`")]
fn semantic_candidate_kind(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("kind=semantic_candidate"));
}

#[then("candidate rank 1 has relation `semantic_similarity`")]
fn semantic_candidate_relation(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("relation=semantic_similarity"));
}

#[then("candidate rank 1 has score `0.9600`")]
fn semantic_candidate_score(world: &mut GivnWorld) {
    assert!(candidate_line(world).ends_with("score=0.9600"));
}

#[then("candidate rank 1 has null assertion shape and null fingerprint facts")]
fn semantic_candidate_null_structural_evidence(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("assertion_shape=null facts=null"));
}

#[then("candidate rank 1 has empty distinguishing facts and fact contradiction `false`")]
fn semantic_candidate_non_contradictory(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("distinguishing_facts=[] fact_contradiction=false"));
}

#[then("the semantic candidate is advisory and is not an automatic duplicate decision")]
fn semantic_candidate_advisory(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(output.contains("kind=semantic_candidate"));
    assert!(!output.contains("kind=exact_duplicate"));
    assert!(!output.contains("kind=structural_overlap"));
    assert!(!output.contains("automatic duplicate"));
}

#[given("a permanent feature contains two scenarios with identical behaviour")]
fn duplicate_feature(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[(
            "givn/specs/provider/provider.feature",
            EXACT_DUPLICATE_FEATURE,
        )],
    );
}

#[given("two permanent features contain scenarios with identical behaviour")]
fn duplicate_features(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[
            (
                "givn/specs/provider/provider.feature",
                CROSS_FEATURE_PROVIDER,
            ),
            ("givn/specs/auth/auth.feature", CROSS_FEATURE_AUTH),
        ],
    );
}

#[given("the permanent corpus contains more than five duplicate and related-scenario candidates")]
fn many_duplicate_candidates_fixture(world: &mut GivnWorld) {
    permanent_project(world, &MANY_DUPLICATE_FEATURES);
}

#[then("the audit includes the candidate at rank 6")]
fn candidate_rank_six(world: &mut GivnWorld) {
    assert!(
        stdout(world)
            .lines()
            .any(|line| line.starts_with("rank 6: ")),
        "duplicate audit rank 6 is missing:\n{}",
        stdout(world)
    );
}

#[then("candidate ranks are exactly `1,2,3,4,5,6`")]
fn candidate_ranks_are_contiguous(world: &mut GivnWorld) {
    assert_eq!(candidate_ranks(world), vec![1, 2, 3, 4, 5, 6]);
}

#[then("the highest rank equals the candidate count")]
fn highest_rank_equals_candidate_count(world: &mut GivnWorld) {
    let highest_rank = candidate_ranks(world)
        .into_iter()
        .max()
        .expect("duplicate audit has no candidate ranks");
    let candidate_count = stdout(world)
        .lines()
        .find_map(|line| line.strip_prefix("Candidate count: "))
        .expect("duplicate audit candidate count is missing")
        .parse::<usize>()
        .expect("duplicate audit candidate count is not an integer");
    assert_eq!(highest_rank, candidate_count);
}

fn candidate_ranks(world: &GivnWorld) -> Vec<usize> {
    stdout(world)
        .lines()
        .filter_map(|line| {
            line.strip_prefix("rank ")
                .and_then(|line| line.split_once(':'))
                .and_then(|(rank, _)| rank.parse::<usize>().ok())
        })
        .collect()
}

fn permanent_project(world: &mut GivnWorld, features: &[(&str, &str)]) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    for (path, content) in features {
        let feature = env.path(path);
        fs::create_dir_all(feature.parent().expect("duplicate feature parent"))
            .expect("create permanent feature directory");
        fs::write(feature, content).expect("write permanent fixture");
    }
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[then(regex = r"^the audit reports candidate count (\d+)$")]
fn candidate_count(world: &mut GivnWorld, count: String) {
    assert!(stdout(world).contains(&format!("Candidate count: {count}\n")));
}

#[then("candidate rank 1 is classified as `exact_duplicate`")]
fn exact_duplicate_kind(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("kind=exact_duplicate"));
}

#[then(regex = r##"^candidate rank 1 has left identity \("([^\"]+)", "([^\"]+)"\)$"##)]
fn exact_duplicate_left_identity(world: &mut GivnWorld, file: String, title: String) {
    assert!(candidate_line(world).contains(&format!(
        "left={{\"file\":\"{file}\",\"title\":\"{title}\"}}"
    )));
}

#[then(regex = r##"^candidate rank 1 has right identity \("([^\"]+)", "([^\"]+)"\)$"##)]
fn exact_duplicate_right_identity(world: &mut GivnWorld, file: String, title: String) {
    assert!(candidate_line(world).contains(&format!(
        "right={{\"file\":\"{file}\",\"title\":\"{title}\"}}"
    )));
}

#[then("candidate rank 1 has relation `identical_behavior`")]
fn exact_duplicate_relation(world: &mut GivnWorld) {
    assert!(candidate_line(world).contains("relation=identical_behavior"));
}

#[then("candidate rank 1 has score `null`")]
fn exact_duplicate_score(world: &mut GivnWorld) {
    assert!(candidate_line(world).ends_with("score=null"));
}

fn candidate_line(world: &GivnWorld) -> String {
    stdout(world)
        .lines()
        .find(|line| line.starts_with("rank 1: "))
        .expect("duplicate audit rank 1 is missing")
        .to_string()
}

#[given("a permanent scenario corpus with duplicate and related-scenario candidates")]
fn repeated_audit_fixture(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[
            (
                "givn/specs/provider/provider.feature",
                EXACT_DUPLICATE_FEATURE,
            ),
            (
                "givn/specs/overlap/overlap.feature",
                STRUCTURAL_OVERLAP_FEATURE,
            ),
            ("givn/specs/auth/auth.feature", SEMANTIC_AUTH_FEATURE),
            ("givn/specs/token/token.feature", SEMANTIC_TOKEN_FEATURE),
            (
                "givn/specs/account/account.feature",
                SEMANTIC_ACCOUNT_FEATURE,
            ),
            (
                "givn/specs/session/session.feature",
                SEMANTIC_SESSION_FEATURE,
            ),
        ],
    );
    world.retrieval_duplicate_scores = Some(
        "givn/specs/auth/auth.feature|Token is required::givn/specs/token/token.feature|Authentication needs a token=0.9600;givn/specs/account/account.feature|Account access needs credentials::givn/specs/session/session.feature|Session credentials are needed=0.9600".to_string(),
    );
}

#[when("the user runs `givn spec duplicates` twice without changing the corpus")]
fn repeated_audit_runs(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("repeated audit fixture is required");
    let score_overrides = world
        .retrieval_duplicate_scores
        .as_deref()
        .expect("repeated audit score overrides are required");
    let environments = [
        ("GIVN_SPEC_EMBEDDER", "stub"),
        ("GIVN_SPEC_DUPLICATES_TEST_SCORES", score_overrides),
    ];
    let first = common::run_givn_with_envs(env, &["spec", "duplicates"], &environments);
    let second = common::run_givn_with_envs(env, &["spec", "duplicates"], &environments);
    world.previous_output = Some(first);
    world.last_output = Some(second);
}

#[then("both audit reports are byte-for-byte identical")]
fn repeated_audit_bytes(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("first duplicate audit output is missing");
    let second = world
        .last_output
        .as_ref()
        .expect("second duplicate audit output is missing");
    assert!(
        first.status.success(),
        "first audit failed: {:?}",
        first.status
    );
    assert!(
        second.status.success(),
        "second audit failed: {:?}",
        second.status
    );
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(
        duplicate_audit_report(first),
        duplicate_audit_report(second)
    );
}

#[then("both reports have the same scanned scenario count, candidate count, ranks, and evidence")]
fn repeated_audit_evidence(world: &mut GivnWorld) {
    let first = world
        .previous_output
        .as_ref()
        .expect("first duplicate audit output is missing");
    let second = world
        .last_output
        .as_ref()
        .expect("second duplicate audit output is missing");
    let first_report = duplicate_audit_report(first);
    let second_report = duplicate_audit_report(second);

    assert_eq!(
        first_report.scanned_scenarios,
        second_report.scanned_scenarios
    );
    assert_eq!(first_report.candidate_count, second_report.candidate_count);
    assert_eq!(
        first_report
            .candidates
            .iter()
            .map(|candidate| candidate.rank)
            .collect::<Vec<_>>(),
        second_report
            .candidates
            .iter()
            .map(|candidate| candidate.rank)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        first_report
            .candidates
            .iter()
            .map(|candidate| candidate.evidence.as_str())
            .collect::<Vec<_>>(),
        second_report
            .candidates
            .iter()
            .map(|candidate| candidate.evidence.as_str())
            .collect::<Vec<_>>()
    );

    assert_eq!(first_report.scanned_scenarios, 8);
    assert_eq!(first_report.candidate_count, 4);
    assert_eq!(
        first_report
            .candidates
            .iter()
            .map(|candidate| candidate.rank)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert!(
        first_report.candidates[0]
            .evidence
            .starts_with("kind=exact_duplicate")
    );
    assert!(first_report.candidates[0].evidence.ends_with("score=null"));
    assert!(
        first_report.candidates[1]
            .evidence
            .starts_with("kind=structural_overlap")
    );
    assert!(first_report.candidates[1].evidence.ends_with("score=null"));
    assert!(first_report.candidates[2].evidence.starts_with(
        "kind=semantic_candidate left={\"file\":\"givn/specs/account/account.feature\""
    ));
    assert!(
        first_report.candidates[3]
            .evidence
            .starts_with("kind=semantic_candidate left={\"file\":\"givn/specs/auth/auth.feature\"")
    );
    assert!(
        first_report.candidates[2]
            .evidence
            .ends_with("score=0.9600")
    );
    assert!(
        first_report.candidates[3]
            .evidence
            .ends_with("score=0.9600")
    );
}

fn duplicate_audit_report(output: &Output) -> DuplicateAuditReport {
    let text = std::str::from_utf8(&output.stdout).expect("duplicate audit stdout is UTF-8");
    assert!(!text.contains('\r'), "duplicate audit uses CRLF endings");
    assert!(
        text.ends_with('\n'),
        "duplicate audit is missing its final newline"
    );
    assert!(
        !text.ends_with("\n\n"),
        "duplicate audit has multiple final newlines"
    );

    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.first().copied(), Some("Givn spec duplicates"));
    let scanned_scenarios = parse_report_count(lines.get(1), "Scanned scenarios: ");
    let candidate_count = parse_report_count(lines.get(2), "Candidate count: ");
    let candidates: Vec<DuplicateAuditCandidate> = lines
        .iter()
        .skip(3)
        .map(|line| {
            let value = line
                .strip_prefix("rank ")
                .expect("duplicate audit candidate line has rank prefix");
            let (rank, evidence) = value
                .split_once(": ")
                .expect("duplicate audit candidate line has field separator");
            DuplicateAuditCandidate {
                rank: rank.parse().expect("duplicate audit rank is an integer"),
                evidence: evidence.to_string(),
            }
        })
        .collect();
    assert_eq!(candidates.len(), candidate_count);

    DuplicateAuditReport {
        scanned_scenarios,
        candidate_count,
        candidates,
    }
}

fn parse_report_count(line: Option<&&str>, prefix: &str) -> usize {
    line.and_then(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("duplicate audit is missing `{prefix}`"))
        .parse()
        .expect("duplicate audit count is an integer")
}
