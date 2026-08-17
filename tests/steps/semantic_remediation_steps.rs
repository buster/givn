//! Regular black-box steps for semantic remediation verification.

use std::fs;

use cucumber::given;

use super::GivnWorld;
use crate::common;

#[given(
    regex = r#"^a project prepared for semantic remediation with a \"(DUPLIKAT|VARIANTE|VALID-BOUNDARY)\" candidate$"#
)]
fn remediation_project(world: &mut GivnWorld, classification: String) {
    let env = common::make_init_root();
    let permanent = env.path("givn/specs");
    if permanent.exists() {
        fs::remove_dir_all(&permanent).expect("remove generated permanent specs");
    }
    fs::create_dir_all(&permanent).expect("create permanent specs");
    fs::write(
        permanent.join("provider.feature"),
        "Feature: Provider catalog\n\n  Scenario: Provider name is unknown\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write remediation permanent spec");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write remediation commands");
    let delta = env.path("givn/changes/running-change/specs/running.feature");
    fs::create_dir_all(delta.parent().expect("delta parent")).expect("create remediation delta");
    fs::write(
        &delta,
        "@givn.delta @running-change\nFeature: Provider change\n\n  @givn.added\n  Scenario: Unknown provider is rejected\n    Given a configured provider catalog\n    When the user selects an unknown provider\n    Then the command rejects the request\n",
    )
    .expect("write remediation delta");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Unknown provider is rejected` | `Provider name is unknown` | boundary |\n",
    )
    .expect("write remediation dispositions");
    world.env = Some(env);
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_reranker = Some("stub".to_string());
    world.retrieval_stub_scores =
        Some("Unknown provider is rejected|Provider name is unknown=0.9600".to_string());
    world.semantic_remediation_mode = Some(classification);
}
