use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::given;
use std::io::Write;

#[given(
    "a givn project with coverage enabled and valid coverage commands that produce Cobertura XML"
)]
fn e2e_given_coverage_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"\"
coverage:
  measure_command: \"./measure-coverage.sh\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_fixture_policy(env);
}

#[given(regex = r#"^a completed change "([^"]+)" with all required artifacts$"#)]
fn e2e_given_completed_change(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::harness::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    for file in &[
        "proposal.md",
        "design.md",
        "design-review.md",
        "tasks.md",
        "review.md",
    ] {
        if *file == "proposal.md" {
            std::fs::write(
                change_dir.join(file),
                crate::steps::fixture_routing_proposal("example"),
            )
            .expect("write proposal");
            continue;
        }
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        if *file == "design-review.md" {
            writeln!(f, "DESIGN-REVIEW: PASS").ok();
        } else if *file == "review.md" {
            writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] COMMIT: a1b2c3d4 -- all done").ok();
        }
    }
    let permanent = env.path("givn/specs/example");
    std::fs::create_dir_all(&permanent).expect("create permanent usecase dir");
    std::fs::write(
        permanent.join("example.feature"),
        "Feature: example\n  Scenario: Existing stub\n    Given a placeholder step\n",
    )
    .expect("write permanent feature");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", "example")
        .replace("<capability-id>", "example")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "stub")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| example | exercise the capability | stub |\n",
            "- none\n",
        );
    std::fs::write(permanent.join("usecase.md"), document).expect("write permanent usecase");
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .unwrap();
    crate::harness::declare_fixture_routing(env, &id);
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id.clone());
    crate::harness::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
}

fn configure_fixture_policy(env: &crate::harness::TestEnv) {
    common::replace_config_addons(
        env,
        "addons:\n  arc42: true\n  readme: true\n  coverage: true\n  domain_modeling: true\n  dev_principles: true\n",
    );
    common::write_fixture_coverage_measure(env, NON_E2E_COVERAGE, E2E_COVERAGE);
}

const NON_E2E_COVERAGE: &str = r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#;

const E2E_COVERAGE: &str = r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#;
