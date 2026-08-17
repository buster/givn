//! Regular black-box steps for the permanent scenario duplicate audit.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use cucumber::{given, then, when};

use super::{GivnWorld, output, stderr, stdout};
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
const NO_CANDIDATE_FEATURE: &str = r#"Feature: unrelated operations

  Scenario: Glacier replication completes
    Given a glacier archive is configured
    When replication checkpoints are flushed
    Then the archive reports completion

  Scenario: Orchard irrigation starts
    Given an orchard schedule is loaded
    When irrigation valves are opened
    Then the orchard reports watering
"#;
const PERMANENT_RELATED_FEATURE_PATH: &str = "givn/specs/provider/provider.feature";
const ACTIVE_CHANGE_FEATURE_PATH: &str =
    "givn/changes/active-provider/specs/provider/provider.feature";
const ACTIVE_CHANGE_SCENARIO_TITLE: &str = "Active provider is rejected";
const ACTIVE_CHANGE_FEATURE: &str = r#"Feature: active provider

  Scenario: Active provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
"#;
const ARCHIVE_MATCHING_FEATURE_PATH: &str = "givn/archive/old-change/specs/old/old.feature";
const ARCHIVE_PATH_PREFIX: &str = "givn/archive/old-change/";
const ARCHIVE_MATCHING_FEATURE: &str = r#"Feature: old

  Scenario: Archived provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
"#;
const ACTIVE_AUDIT_ROOT: &str = "givn/changes/audit-boundary";
const ARCHIVE_AUDIT_ROOT: &str = "givn/archive/audit-boundary";
const ACTIVE_AUDIT_FILES: [(&str, &str); 3] = [
    (
        "givn/changes/audit-boundary/specs/active/active.feature",
        r#"Feature: active audit boundary

  Scenario: Active provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
"#,
    ),
    (
        "givn/changes/audit-boundary/proposal.md",
        "# Active audit fixture\nThis file must remain unchanged.\n",
    ),
    (
        "givn/changes/audit-boundary/notes.txt",
        "active fixture bytes\n",
    ),
];
const ARCHIVE_AUDIT_FILES: [(&str, &str); 3] = [
    (
        "givn/archive/audit-boundary/specs/archived/archived.feature",
        r#"Feature: archived audit boundary

  Scenario: Archived provider is rejected
    Given a configured provider catalog
    When the user selects an unknown provider
    Then the command rejects the request
"#,
    ),
    (
        "givn/archive/audit-boundary/README.md",
        "# Archive audit fixture\nThis file must remain unchanged.\n",
    ),
    (
        "givn/archive/audit-boundary/record.txt",
        "archive fixture bytes\n",
    ),
];
const PERSISTENT_INDEX_PATH: &str = "givn/.spec-index.json";
const MALFORMED_INDEX_BYTES: &[u8] = b"not-json";
const MALFORMED_PERMANENT_FEATURE: &str =
    "this is not a Gherkin feature document\nwithout a structural keyword\n";
const UNREADABLE_PERMANENT_FEATURE: &str = r#"Feature: private

  Scenario: Private fixture is readable
    Given a private fixture
    When the private fixture is inspected
    Then the private fixture is readable
"#;

#[derive(Debug)]
pub(crate) struct UnreadableFeatureGuard {
    path: PathBuf,
    original_permissions: fs::Permissions,
    fallback_backup: Option<PathBuf>,
}

impl Drop for UnreadableFeatureGuard {
    fn drop(&mut self) {
        if let Some(backup) = self.fallback_backup.take() {
            let _ = fs::remove_dir_all(&self.path);
            let _ = fs::rename(backup, &self.path);
        }
        let _ = fs::set_permissions(&self.path, self.original_permissions.clone());
    }
}

fn make_unreadable_feature_guard(path: PathBuf) -> UnreadableFeatureGuard {
    let original_permissions = fs::metadata(&path)
        .expect("read unreadable feature metadata")
        .permissions();
    let permission_is_enforced = set_unreadable_permissions(&path);
    let fallback_backup = if permission_is_enforced {
        None
    } else {
        // Elevated test users can bypass mode 000. A directory at the same
        // .feature path makes Feature::parse_path fail with an I/O error
        // (EISDIR) instead of exercising malformed-Gherkin parsing.
        let backup = path.with_extension("readable");
        fs::rename(&path, &backup).expect("move readable feature for I/O fallback");
        if let Err(error) = fs::set_permissions(&backup, original_permissions.clone()) {
            let _ = fs::rename(&backup, &path);
            panic!("restore readable feature permissions for I/O fallback: {error}");
        }
        if let Err(error) = fs::create_dir(&path) {
            let _ = fs::rename(&backup, &path);
            panic!("create unreadable feature I/O fixture directory: {error}");
        }
        Some(backup)
    };

    UnreadableFeatureGuard {
        path,
        original_permissions,
        fallback_backup,
    }
}

#[cfg(unix)]
fn set_unreadable_permissions(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    if fs::set_permissions(path, fs::Permissions::from_mode(0o000)).is_err() {
        return false;
    }
    fs::read_to_string(path).is_err()
}

#[cfg(not(unix))]
fn set_unreadable_permissions(_path: &Path) -> bool {
    false
}

#[given(regex = r##"^a malformed permanent feature at "([^"]+)"$"##)]
fn malformed_permanent_feature(world: &mut GivnWorld, path: String) {
    assert_eq!(path, "givn/specs/broken/broken.feature");
    permanent_project(world, &[(path.as_str(), MALFORMED_PERMANENT_FEATURE)]);
}

#[given(regex = r##"^an unreadable permanent feature at "([^"]+)"$"##)]
fn unreadable_permanent_feature(world: &mut GivnWorld, path: String) {
    assert_eq!(path, "givn/specs/private/private.feature");
    permanent_project(world, &[(path.as_str(), UNREADABLE_PERMANENT_FEATURE)]);
    let feature_path = world
        .env
        .as_ref()
        .expect("unreadable feature project is initialized")
        .path(&path);
    world.unreadable_feature = Some(make_unreadable_feature_guard(feature_path));
}

#[given("a project without a `givn/specs/` directory")]
fn missing_permanent_specs_directory(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    assert!(
        specs.is_dir(),
        "givn init did not create the specs directory"
    );
    fs::remove_dir_all(&specs).expect("remove initialized permanent specs directory");
    assert!(!specs.exists(), "permanent specs directory still exists");

    let index = env.path(PERSISTENT_INDEX_PATH);
    if index.exists() {
        fs::remove_file(&index).expect("remove pre-existing persistent retrieval index");
    }
    assert!(
        !index.exists(),
        "persistent retrieval index unexpectedly exists"
    );

    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
}

#[then(regex = r##"^stderr contains the project-relative path "([^"]+)"$"##)]
fn stderr_contains_project_relative_path(world: &mut GivnWorld, path: String) {
    let message = stderr(world);
    assert!(
        message.contains(&path),
        "stderr missing project-relative path {path:?}: {message}"
    );
}

#[then("stdout is empty")]
fn stdout_is_empty(world: &mut GivnWorld) {
    assert!(
        output(world).stdout.is_empty(),
        "duplicate audit emitted stdout: {}",
        stdout(world)
    );
    drop(world.unreadable_feature.take());
}

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

#[given("a permanent scenario corpus with no duplicate or related behaviour")]
fn no_candidate_fixture(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[(
            "givn/specs/unrelated/unrelated.feature",
            NO_CANDIDATE_FEATURE,
        )],
    );
}

#[given("active change files and archive contents are snapshotted")]
fn snapshot_active_change_and_archive(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    write_fixture_files(env, &ACTIVE_AUDIT_FILES);
    write_fixture_files(env, &ARCHIVE_AUDIT_FILES);
    world.active_change_snapshot = Some(snapshot_fixture_tree(env, ACTIVE_AUDIT_ROOT));
    world.archive_snapshot = Some(snapshot_fixture_tree(env, ARCHIVE_AUDIT_ROOT));
}

#[then("active change files are unchanged byte-for-byte")]
fn active_change_files_unchanged_byte_for_byte(world: &mut GivnWorld) {
    let out = output(world);
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
    let report = duplicate_audit_report(out);
    assert_eq!(report.scanned_scenarios, 8);
    assert_eq!(report.candidate_count, 4);

    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let before = world
        .active_change_snapshot
        .as_ref()
        .expect("active change files were not snapshotted");
    assert_fixture_tree_unchanged(env, ACTIVE_AUDIT_ROOT, before);
}

#[then("archive contents are unchanged byte-for-byte")]
fn archive_contents_unchanged_byte_for_byte(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let before = world
        .archive_snapshot
        .as_ref()
        .expect("archive contents were not snapshotted");
    assert_fixture_tree_unchanged(env, ARCHIVE_AUDIT_ROOT, before);
}

#[then(r#"the text report is exactly `Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n`"#)]
fn empty_duplicate_text_report(world: &mut GivnWorld) {
    let output = super::output(world);
    assert_eq!(
        output.stdout,
        b"Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n"
    );
    assert_eq!(
        duplicate_audit_report(output),
        DuplicateAuditReport {
            scanned_scenarios: 2,
            candidate_count: 0,
            candidates: Vec::new(),
        }
    );
}

#[then("the JSON report has scanned_scenarios 2, candidate_count 0, and an empty candidates array")]
fn empty_duplicate_json_report(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("no-candidate fixture is required");
    let output = common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    );
    assert!(
        output.status.success(),
        "empty duplicate JSON audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = duplicate_json_document(&output);
    assert_eq!(document["scanned_scenarios"], serde_json::json!(2));
    assert_eq!(document["candidate_count"], serde_json::json!(0));
    assert_eq!(document["candidates"], serde_json::json!([]));
}

#[then("the JSON report has scanned_scenarios 0, candidate_count 0, and an empty candidates array")]
fn missing_specs_json_report(world: &mut GivnWorld) {
    let output = super::output(world);
    assert!(
        output.status.success(),
        "missing specs JSON audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = duplicate_json_document(output);
    assert_eq!(document["schema_version"], serde_json::json!(1));
    assert_eq!(document["scanned_scenarios"], serde_json::json!(0));
    assert_eq!(document["candidate_count"], serde_json::json!(0));
    assert_eq!(document["candidates"], serde_json::json!([]));

    let env = world
        .env
        .as_ref()
        .expect("missing specs fixture is required");
    assert!(
        !env.path("givn/specs").exists(),
        "duplicate audit recreated the permanent specs directory"
    );
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

fn write_fixture_files(env: &common::TestEnv, files: &[(&str, &str)]) {
    for (relative, content) in files {
        let path = env.path(relative);
        fs::create_dir_all(path.parent().expect("fixture file parent"))
            .expect("create fixture file directory");
        fs::write(path, content).expect("write fixture file");
    }
}

fn snapshot_fixture_tree(env: &common::TestEnv, relative_root: &str) -> Vec<(String, Vec<u8>)> {
    let root = env.path(relative_root);
    let project_root = env.root();
    let mut snapshot = Vec::new();
    collect_fixture_files(&root, &project_root, &mut snapshot);
    snapshot.sort_by(|left, right| left.0.cmp(&right.0));
    snapshot
}

fn collect_fixture_files(
    directory: &Path,
    project_root: &Path,
    snapshot: &mut Vec<(String, Vec<u8>)>,
) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
        Err(error) => panic!("read fixture tree {}: {error}", directory.display()),
    };
    let mut paths = entries
        .map(|entry| entry.expect("read fixture tree entry").path())
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        if path.is_dir() {
            collect_fixture_files(&path, project_root, snapshot);
            continue;
        }
        let relative = path
            .strip_prefix(project_root)
            .expect("fixture path under project root")
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path)
            .unwrap_or_else(|error| panic!("read fixture file {}: {error}", path.display()));
        snapshot.push((relative, bytes));
    }
}

fn assert_fixture_tree_unchanged(
    env: &common::TestEnv,
    relative_root: &str,
    before: &[(String, Vec<u8>)],
) {
    let after = snapshot_fixture_tree(env, relative_root);
    assert_eq!(
        &after,
        before,
        "duplicate audit changed fixture tree rooted at {}",
        env.path(relative_root).display()
    );
}

#[given("a permanent scenario corpus with one related scenario")]
fn permanent_related_scenario_fixture(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[(PERMANENT_RELATED_FEATURE_PATH, EXACT_DUPLICATE_FEATURE)],
    );
}

#[given("an active change contains another scenario with related behaviour")]
fn active_related_scenario_fixture(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("permanent duplicate fixture is required");
    let path = env.path(ACTIVE_CHANGE_FEATURE_PATH);
    fs::create_dir_all(path.parent().expect("active feature parent"))
        .expect("create active feature directory");
    fs::write(path, ACTIVE_CHANGE_FEATURE).expect("write active feature fixture");
}

#[then("the audit reports only relationships between permanent scenarios")]
fn permanent_only_audit(world: &mut GivnWorld) {
    let out = super::output(world);
    assert!(
        out.status.success(),
        "duplicate audit failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let report = duplicate_audit_report(out);
    assert_eq!(report.scanned_scenarios, 2);
    assert_eq!(report.candidate_count, 1);
    assert_eq!(report.candidates[0].rank, 1);
    assert!(
        report.candidates[0]
            .evidence
            .contains("kind=exact_duplicate")
    );
    assert_eq!(
        candidate_endpoint_file(&report.candidates[0].evidence, "left"),
        PERMANENT_RELATED_FEATURE_PATH
    );
    assert_eq!(
        candidate_endpoint_file(&report.candidates[0].evidence, "right"),
        PERMANENT_RELATED_FEATURE_PATH
    );
}

#[then("the active change scenario identity is absent from every candidate")]
fn active_identity_absent(world: &mut GivnWorld) {
    let identity = format!(
        "\"file\":\"{ACTIVE_CHANGE_FEATURE_PATH}\",\"title\":\"{ACTIVE_CHANGE_SCENARIO_TITLE}\""
    );
    assert!(
        !stdout(world).contains(&identity),
        "active change identity appeared in the audit: {}",
        stdout(world)
    );
}

#[given("a permanent corpus with one candidate")]
fn archived_audit_fixture(world: &mut GivnWorld) {
    permanent_project(
        world,
        &[(
            "givn/specs/provider/provider.feature",
            EXACT_DUPLICATE_FEATURE,
        )],
    );
    let env = world
        .env
        .as_ref()
        .expect("archived audit fixture is initialized");
    let output = common::run_givn_with_envs(
        env,
        &["spec", "duplicates", "--format", "json"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    );
    assert!(
        output.status.success(),
        "baseline archived audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = duplicate_json_document(&output);
    assert_eq!(document["candidate_count"], serde_json::json!(1));
    world.previous_output = Some(output);
}

#[given(regex = r##"^the archive contains a matching scenario at \"([^\"]+)\"$"##)]
fn archived_matching_scenario_fixture(world: &mut GivnWorld, path: String) {
    assert_eq!(path, ARCHIVE_MATCHING_FEATURE_PATH);
    let env = world
        .env
        .as_ref()
        .expect("archived audit fixture is initialized");
    let feature_path = env.path(&path);
    fs::create_dir_all(
        feature_path
            .parent()
            .expect("archive matching feature parent"),
    )
    .expect("create archive matching feature directory");
    fs::write(feature_path, ARCHIVE_MATCHING_FEATURE).expect("write archive matching feature");
}

#[then("every JSON candidate endpoint file starts with `givn/specs/`")]
fn json_candidate_endpoints_are_permanent(world: &mut GivnWorld) {
    let candidates = json_candidates(world);
    assert!(
        !candidates.is_empty(),
        "archive audit produced no candidates"
    );
    for candidate in candidates {
        for side in ["left", "right"] {
            let file = candidate[side]["file"]
                .as_str()
                .unwrap_or_else(|| panic!("JSON {side} endpoint file is not a string"));
            assert!(
                file.starts_with("givn/specs/"),
                "JSON {side} candidate endpoint is not permanent: {file}"
            );
        }
    }
}

#[then(regex = r##"^no JSON candidate identity contains `([^`]+)`$"##)]
fn no_json_candidate_archive_identity(world: &mut GivnWorld, path: String) {
    assert_eq!(path, ARCHIVE_PATH_PREFIX);
    for candidate in json_candidates(world) {
        for side in ["left", "right"] {
            let endpoint = &candidate[side];
            let file = endpoint["file"]
                .as_str()
                .unwrap_or_else(|| panic!("JSON {side} endpoint file is not a string"));
            let title = endpoint["title"]
                .as_str()
                .unwrap_or_else(|| panic!("JSON {side} endpoint title is not a string"));
            assert!(
                !file.contains(&path) && !title.contains(&path),
                "JSON {side} candidate identity contains {path}: {file}::{title}"
            );
        }
    }
}

#[then("the archived matching scenario does not change candidate_count")]
fn archived_scenario_does_not_change_candidate_count(world: &mut GivnWorld) {
    assert!(
        output(world).status.success(),
        "archived audit failed: {}",
        stderr(world)
    );
    let before = world
        .previous_output
        .as_ref()
        .expect("baseline archived audit output is missing");
    let before_document = duplicate_json_document(before);
    let after_document = json_document(world);
    assert_eq!(before_document["candidate_count"], serde_json::json!(1));
    assert_eq!(
        after_document["candidate_count"],
        before_document["candidate_count"]
    );
    assert_eq!(after_document["candidate_count"], serde_json::json!(1));
}

#[then("every candidate endpoint file starts with `givn/specs/`")]
fn candidate_endpoints_are_permanent(world: &mut GivnWorld) {
    let report = duplicate_audit_report(super::output(world));
    for candidate in report.candidates {
        for side in ["left", "right"] {
            let file = candidate_endpoint_file(&candidate.evidence, side);
            assert!(
                file.starts_with("givn/specs/"),
                "{side} candidate endpoint is not permanent: {file}"
            );
        }
    }
}

fn candidate_endpoint_file<'a>(evidence: &'a str, side: &str) -> &'a str {
    let marker = match side {
        "left" => "left={\"file\":\"",
        "right" => "right={\"file\":\"",
        _ => panic!("unsupported duplicate candidate endpoint side: {side}"),
    };
    let start = evidence
        .find(marker)
        .map(|index| index + marker.len())
        .unwrap_or_else(|| panic!("duplicate candidate is missing its {side} endpoint"));
    evidence[start..]
        .split_once("\",\"title\":\"")
        .map(|(file, _)| file)
        .expect("duplicate candidate endpoint has no title")
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

#[given("a permanent corpus with duplicate and related-scenario candidates")]
fn audited_boundary_fixture(world: &mut GivnWorld) {
    repeated_audit_fixture(world);
}

#[given("no persistent retrieval index exists")]
fn no_persistent_retrieval_index(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let index = env.path(PERSISTENT_INDEX_PATH);
    if index.exists() {
        fs::remove_file(&index).expect("remove pre-existing persistent retrieval index");
    }
    assert!(
        !index.exists(),
        "persistent retrieval index unexpectedly exists before audit"
    );
}

#[given("the persistent retrieval index contains the bytes \"not-json\"")]
fn malformed_persistent_retrieval_index(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let index = env.path(PERSISTENT_INDEX_PATH);
    fs::write(&index, MALFORMED_INDEX_BYTES).expect("write malformed persistent retrieval index");
    let snapshot = fs::read(&index).expect("read malformed persistent retrieval index snapshot");
    assert_eq!(
        snapshot.as_slice(),
        MALFORMED_INDEX_BYTES,
        "malformed persistent retrieval index fixture changed before audit"
    );
    world.malformed_index_snapshot = Some(snapshot);
}

#[then("the audit uses current permanent files and transient embeddings")]
fn audit_uses_current_permanent_files_and_transient_embeddings(world: &mut GivnWorld) {
    let out = super::output(world);
    assert!(
        out.status.success(),
        "duplicate audit failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let document = duplicate_json_document(out);
    assert_eq!(document["schema_version"], serde_json::json!(1));
    assert_eq!(document["scanned_scenarios"], serde_json::json!(8));
    assert_eq!(document["candidate_count"], serde_json::json!(4));

    let candidates = document["candidates"]
        .as_array()
        .expect("duplicate audit candidates must be an array");
    let kinds = candidates
        .iter()
        .map(|candidate| {
            candidate["kind"]
                .as_str()
                .expect("duplicate candidate kind must be a string")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        vec![
            "exact_duplicate",
            "structural_overlap",
            "semantic_candidate",
            "semantic_candidate",
        ]
    );
    assert!(
        candidates.iter().any(|candidate| {
            candidate["evidence"]["relation"] == serde_json::json!("semantic_similarity")
                && candidate["evidence"]["score"] == serde_json::json!("0.9600")
        }),
        "duplicate audit did not report a transient semantic candidate"
    );
    for candidate in candidates {
        for side in ["left", "right"] {
            let file = candidate[side]["file"]
                .as_str()
                .unwrap_or_else(|| panic!("JSON {side} endpoint file is not a string"));
            assert!(
                file.starts_with("givn/specs/"),
                "JSON {side} candidate endpoint is not a current permanent file: {file}"
            );
        }
    }
}

#[then("the audit report is valid JSON with the permanent candidates")]
fn malformed_index_audit_report(world: &mut GivnWorld) {
    audit_uses_current_permanent_files_and_transient_embeddings(world);
}

#[then("the persistent retrieval index still contains the bytes \"not-json\"")]
fn malformed_persistent_retrieval_index_is_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let before = world
        .malformed_index_snapshot
        .as_ref()
        .expect("malformed persistent retrieval index was not snapshotted");
    assert_eq!(
        before.as_slice(),
        MALFORMED_INDEX_BYTES,
        "malformed persistent retrieval index snapshot is not the required byte sequence"
    );
    let after = fs::read(env.path(PERSISTENT_INDEX_PATH))
        .expect("read malformed persistent retrieval index after audit");
    assert_eq!(
        after.as_slice(),
        before.as_slice(),
        "duplicate audit changed the malformed persistent retrieval index"
    );
    assert_eq!(
        after.as_slice(),
        MALFORMED_INDEX_BYTES,
        "persistent retrieval index was not left as the malformed byte sequence"
    );
}

#[when("the user runs `givn spec duplicates` through a build without spec retrieval")]
fn duplicates_without_retrieval_feature(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("duplicate audit fixture is required");
    let manifest_path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let target_dir = env.path(".cargo-target-no-spec-retrieval");
    let output = Command::new("cargo")
        .current_dir(env.root())
        .args([
            "run",
            "--quiet",
            "--manifest-path",
            manifest_path.as_str(),
            "--no-default-features",
            "--bin",
            "givn",
            "--",
            "spec",
            "duplicates",
        ])
        .env("CARGO_TARGET_DIR", target_dir)
        .stdin(Stdio::null())
        .output()
        .expect("run no-feature givn process");
    world.last_output = Some(output);
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

#[then("the persistent retrieval index remains absent")]
fn persistent_retrieval_index_remains_absent(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("missing specs fixture is required");
    assert!(
        !env.path(PERSISTENT_INDEX_PATH).exists(),
        "duplicate audit created the persistent retrieval index"
    );
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
    duplicate_json_document(super::output(world))
}

fn duplicate_json_document(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).expect("duplicate audit stdout must be valid JSON")
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
