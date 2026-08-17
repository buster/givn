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
const PERSISTENT_INDEX_PATH: &str = "givn/.spec-index.json";
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

#[given("the persistent retrieval index is built and snapshotted")]
fn build_and_snapshot_persistent_retrieval_index(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let output =
        common::run_givn_with_envs(env, &["spec", "index"], &[("GIVN_SPEC_EMBEDDER", "stub")]);
    assert!(
        output.status.success(),
        "persistent retrieval index build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.permanent_index_snapshot = Some(persistent_index_bytes(env));
}

#[then("the persistent retrieval index is unchanged byte-for-byte")]
fn persistent_retrieval_index_unchanged_byte_for_byte(world: &mut GivnWorld) {
    let output = super::output(world);
    assert!(
        output.status.success(),
        "duplicate audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = duplicate_audit_report(output);
    assert_eq!(report.scanned_scenarios, 8);
    assert_eq!(report.candidate_count, 4);

    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let before = world
        .permanent_index_snapshot
        .as_ref()
        .expect("persistent retrieval index was not snapshotted");
    let after = persistent_index_bytes(env);
    assert_eq!(
        &after, before,
        "duplicate audit changed the persistent index"
    );
}

fn persistent_index_bytes(env: &common::TestEnv) -> Vec<u8> {
    fs::read(env.path(PERSISTENT_INDEX_PATH)).expect("read persistent retrieval index")
}

#[then("the permanent specifications are unchanged byte-for-byte")]
fn permanent_specs_unchanged_byte_for_byte(world: &mut GivnWorld) {
    let out = super::output(world);
    assert!(
        out.status.success(),
        "duplicate audit failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        stdout(world).starts_with("Givn spec duplicates\n"),
        "duplicate audit did not produce its report header: {}",
        stdout(world)
    );

    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let before = world
        .permanent_spec_snapshot
        .as_ref()
        .expect("permanent specifications were not snapshotted");
    let after = permanent_feature_snapshot(env);
    assert_eq!(
        &after, before,
        "permanent feature bytes changed after duplicate audit"
    );
}

fn permanent_feature_snapshot(env: &common::TestEnv) -> Vec<(String, Vec<u8>)> {
    let specs = env.path("givn/specs");
    givn::gherkin::parse::collect_feature_files(&specs)
        .into_iter()
        .map(|path| {
            let relative = path
                .strip_prefix(env.root())
                .expect("permanent feature under project root")
                .to_string_lossy()
                .replace('\\', "/");
            (
                relative,
                fs::read(path).expect("read permanent feature after audit"),
            )
        })
        .collect()
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

#[then("stdout is a valid JSON audit document")]
fn json_audit_document(world: &mut GivnWorld) {
    let document = json_document(world);
    assert!(
        document.is_object(),
        "duplicate audit JSON must be an object"
    );
}

#[then("the JSON schema_version is exactly 1")]
fn json_schema_version(world: &mut GivnWorld) {
    assert_eq!(json_document(world)["schema_version"], serde_json::json!(1));
}

#[then(
    "the JSON top-level keys are exactly `schema_version,scanned_scenarios,candidate_count,candidates` in that order"
)]
fn json_top_level_keys(world: &mut GivnWorld) {
    assert_json_object_key_order(
        &stdout(world),
        r#"{"schema_version":"#,
        &[
            "schema_version",
            "scanned_scenarios",
            "candidate_count",
            "candidates",
        ],
    );
}

#[then("every JSON candidate has keys exactly `rank,kind,left,right,evidence` in that order")]
fn json_candidate_keys(world: &mut GivnWorld) {
    assert_json_object_key_order(
        &stdout(world),
        r#"{"rank":"#,
        &["rank", "kind", "left", "right", "evidence"],
    );
}

#[then("every JSON endpoint has keys exactly `file,title` in that order")]
fn json_endpoint_keys(world: &mut GivnWorld) {
    assert_json_object_key_order(&stdout(world), r#"{"file":"#, &["file", "title"]);
}

#[then(
    "every JSON evidence object has keys exactly `relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score` in that order"
)]
fn json_evidence_keys(world: &mut GivnWorld) {
    assert_json_object_key_order(
        &stdout(world),
        r#"{"relation":"#,
        &[
            "relation",
            "assertion_shape",
            "facts",
            "distinguishing_facts",
            "fact_contradiction",
            "score",
        ],
    );
}

#[then(
    "every JSON candidate has an exact classification, both scenario identities, a relation, and a rank"
)]
fn json_candidate_fields(world: &mut GivnWorld) {
    let candidates = json_candidates(world);
    let expected = [
        (
            1,
            "exact_duplicate",
            "givn/specs/provider/provider.feature",
            "Provider is rejected",
            "givn/specs/provider/provider.feature",
            "Unknown provider is rejected",
            "identical_behavior",
        ),
        (
            2,
            "structural_overlap",
            "givn/specs/overlap/overlap.feature",
            "Provider exits successfully",
            "givn/specs/overlap/overlap.feature",
            "Provider exits with a warning",
            "same_assertion_shape",
        ),
        (
            3,
            "semantic_candidate",
            "givn/specs/account/account.feature",
            "Account access needs credentials",
            "givn/specs/session/session.feature",
            "Session credentials are needed",
            "semantic_similarity",
        ),
        (
            4,
            "semantic_candidate",
            "givn/specs/auth/auth.feature",
            "Token is required",
            "givn/specs/token/token.feature",
            "Authentication needs a token",
            "semantic_similarity",
        ),
    ];
    assert_eq!(candidates.len(), expected.len());
    for (candidate, (rank, kind, left_file, left_title, right_file, right_title, relation)) in
        candidates.iter().zip(expected)
    {
        assert_eq!(candidate["rank"], serde_json::json!(rank));
        assert_eq!(candidate["kind"], serde_json::json!(kind));
        assert_eq!(candidate["left"]["file"], serde_json::json!(left_file));
        assert_eq!(candidate["left"]["title"], serde_json::json!(left_title));
        assert_eq!(candidate["right"]["file"], serde_json::json!(right_file));
        assert_eq!(candidate["right"]["title"], serde_json::json!(right_title));
        assert_eq!(
            candidate["evidence"]["relation"],
            serde_json::json!(relation)
        );
    }

    let exact_evidence = &candidates[0]["evidence"];
    assert_eq!(exact_evidence["assertion_shape"], serde_json::Value::Null);
    assert_eq!(exact_evidence["facts"], serde_json::Value::Null);
    assert_eq!(exact_evidence["score"], serde_json::Value::Null);

    let structural_evidence = &candidates[1]["evidence"];
    assert!(structural_evidence["assertion_shape"].is_object());
    assert!(structural_evidence["facts"].is_object());
    assert_eq!(structural_evidence["score"], serde_json::Value::Null);

    for candidate in &candidates[2..] {
        let evidence = &candidate["evidence"];
        assert_eq!(evidence["assertion_shape"], serde_json::Value::Null);
        assert_eq!(evidence["facts"], serde_json::Value::Null);
        assert_eq!(evidence["distinguishing_facts"], serde_json::json!([]));
        assert_eq!(evidence["fact_contradiction"], serde_json::json!(false));
        assert_eq!(evidence["score"], serde_json::json!("0.9600"));
    }
}

#[then("the JSON candidates follow the deterministic audit ordering")]
fn json_candidate_order(world: &mut GivnWorld) {
    let candidates = json_candidates(world);
    let kinds: Vec<&str> = candidates
        .iter()
        .map(|candidate| {
            candidate["kind"]
                .as_str()
                .expect("candidate kind is a string")
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            "exact_duplicate",
            "structural_overlap",
            "semantic_candidate",
            "semantic_candidate",
        ]
    );
    assert_eq!(
        (
            candidates[2]["left"]["file"].as_str(),
            candidates[2]["left"]["title"].as_str(),
            candidates[2]["right"]["file"].as_str(),
            candidates[2]["right"]["title"].as_str(),
        ),
        (
            Some("givn/specs/account/account.feature"),
            Some("Account access needs credentials"),
            Some("givn/specs/session/session.feature"),
            Some("Session credentials are needed"),
        )
    );
    assert_eq!(
        (
            candidates[3]["left"]["file"].as_str(),
            candidates[3]["left"]["title"].as_str(),
            candidates[3]["right"]["file"].as_str(),
            candidates[3]["right"]["title"].as_str(),
        ),
        (
            Some("givn/specs/auth/auth.feature"),
            Some("Token is required"),
            Some("givn/specs/token/token.feature"),
            Some("Authentication needs a token"),
        )
    );
}

#[then("JSON ranks are contiguous from 1 through candidate_count")]
fn json_contiguous_ranks(world: &mut GivnWorld) {
    let document = json_document(world);
    let candidates = document["candidates"]
        .as_array()
        .expect("JSON candidates is an array");
    assert_eq!(document["candidate_count"], serde_json::json!(4));
    assert_eq!(
        document["candidate_count"].as_u64(),
        Some(candidates.len() as u64)
    );
    let ranks: Vec<u64> = candidates
        .iter()
        .map(|candidate| {
            candidate["rank"]
                .as_u64()
                .expect("candidate rank is an integer")
        })
        .collect();
    assert_eq!(ranks, (1..=candidates.len() as u64).collect::<Vec<_>>());
}

#[then("JSON scanned_scenarios equals the number of eligible non-removed permanent scenarios")]
fn json_scanned_scenarios(world: &mut GivnWorld) {
    let document = json_document(world);
    assert_eq!(document["scanned_scenarios"], serde_json::json!(8));
}

fn json_document(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_str(&stdout(world)).expect("duplicate audit stdout must be valid JSON")
}

fn json_candidates(world: &GivnWorld) -> Vec<serde_json::Value> {
    json_document(world)["candidates"]
        .as_array()
        .expect("JSON candidates is an array")
        .clone()
}

fn assert_json_object_key_order(raw: &str, marker: &str, expected: &[&str]) {
    let starts: Vec<usize> = raw.match_indices(marker).map(|(index, _)| index).collect();
    assert!(
        !starts.is_empty(),
        "JSON object marker `{marker}` is missing"
    );
    for start in starts {
        let actual = json_object_keys(raw, start);
        assert_eq!(
            actual, expected,
            "JSON object key order changed at byte {start}"
        );
    }
}

fn json_object_keys(raw: &str, start: usize) -> Vec<String> {
    let bytes = raw.as_bytes();
    assert_eq!(
        bytes[start], b'{',
        "JSON object marker must start with an object"
    );
    let end = json_container_end(raw, start);
    let mut index = start + 1;
    let mut keys = Vec::new();
    while index < end {
        index = skip_json_whitespace(bytes, index);
        if index == end {
            break;
        }
        let key_end = json_string_end(raw, index);
        let key: String =
            serde_json::from_str(&raw[index..=key_end]).expect("JSON key is a string");
        keys.push(key);
        index = skip_json_whitespace(bytes, key_end + 1);
        assert_eq!(bytes[index], b':', "JSON object key is missing its colon");
        index = skip_json_value(raw, index + 1);
        index = skip_json_whitespace(bytes, index);
        if index < end {
            assert_eq!(
                bytes[index], b',',
                "JSON object member is missing its comma"
            );
            index += 1;
        }
    }
    keys
}

fn json_container_end(raw: &str, start: usize) -> usize {
    let bytes = raw.as_bytes();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (index, byte) in bytes.iter().copied().enumerate().skip(start) {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            _ => {}
        }
    }
    panic!("JSON container starting at byte {start} is not closed");
}

fn json_string_end(raw: &str, start: usize) -> usize {
    let bytes = raw.as_bytes();
    assert_eq!(bytes[start], b'"', "JSON key must start with a quote");
    let mut escaped = false;
    for (index, byte) in bytes.iter().copied().enumerate().skip(start + 1) {
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            return index;
        }
    }
    panic!("JSON string starting at byte {start} is not closed");
}

fn skip_json_value(raw: &str, start: usize) -> usize {
    let bytes = raw.as_bytes();
    let start = skip_json_whitespace(bytes, start);
    match bytes[start] {
        b'"' => json_string_end(raw, start) + 1,
        b'{' | b'[' => json_container_end(raw, start) + 1,
        _ => {
            let mut index = start;
            while index < bytes.len() && bytes[index] != b',' && bytes[index] != b'}' {
                index += 1;
            }
            while index > start && bytes[index - 1].is_ascii_whitespace() {
                index -= 1;
            }
            index
        }
    }
}

fn skip_json_whitespace(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

fn parse_report_count(line: Option<&&str>, prefix: &str) -> usize {
    line.and_then(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("duplicate audit is missing `{prefix}`"))
        .parse()
        .expect("duplicate audit count is an integer")
}
