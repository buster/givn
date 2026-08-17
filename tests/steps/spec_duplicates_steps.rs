//! Regular black-box steps for the permanent scenario duplicate audit.

use std::fs;

use cucumber::{given, then};

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
    assert_eq!(count, "1");
    assert!(stdout(world).contains("Candidate count: 1\n"));
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
