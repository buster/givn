//! Regular black-box steps for the permanent scenario duplicate audit.

use std::fs;

use cucumber::{given, then};

use super::{GivnWorld, stdout};
use crate::common;

const EXACT_DUPLICATE_FEATURE: &str = "Feature: provider\n\n  Scenario: Provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";
const CROSS_FEATURE_PROVIDER: &str = "Feature: provider\n\n  Scenario: Provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";
const CROSS_FEATURE_AUTH: &str = "Feature: auth\n\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n";

#[given("a permanent feature contains two scenarios with identical behaviour")]
fn duplicate_feature(world: &mut GivnWorld) {
    duplicate_project(
        world,
        &[(
            "givn/specs/provider/provider.feature",
            EXACT_DUPLICATE_FEATURE,
        )],
    );
}

#[given("two permanent features contain scenarios with identical behaviour")]
fn duplicate_features(world: &mut GivnWorld) {
    duplicate_project(
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

fn duplicate_project(world: &mut GivnWorld, features: &[(&str, &str)]) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    if specs.exists() {
        fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    }
    for (path, content) in features {
        let feature = env.path(path);
        fs::create_dir_all(feature.parent().expect("duplicate feature parent"))
            .expect("create permanent feature directory");
        fs::write(feature, content).expect("write exact duplicate fixture");
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
