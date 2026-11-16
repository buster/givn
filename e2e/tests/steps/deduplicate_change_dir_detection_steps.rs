use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given(
    regex = r#"^a givn project with a change "([^"]+)" that has design\.md with a valid Step Definitions table and tasks\.md where every COMMIT hash is a valid 40-character hex string$"#
)]
fn e2e_given_change_with_design_and_valid_hashes(world: &mut GivnWorld, id: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::harness::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::write(
        change_dir.join("design.md"),
        "| Step | File |\n| --- | --- |\n| Given x | steps/x.rs |\n",
    )
    .expect("write design.md");
    std::fs::write(
        change_dir.join("tasks.md"),
        "- [x] task: `a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0`\n\
         - [x] COMMIT: `a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0`\n",
    )
    .expect("write tasks.md");
    // Override verify.command so the integrity hook passes.
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  measure_command: \"./measure-coverage.sh\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_fixture_policy(env);
    write_permanent_example(env);
    common::write_fixture_delta(env, &id, "example", "Integrity fixture behavior");
    common::ensure_review_evidence(env, &id, true);
    world.last_change_id = Some(id.clone());
    let head = String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
        .trim()
        .to_string();
    common::proof_fixture::write_change_receipt(&change_dir, &id, &head, "true");
}

#[given(
    regex = r#"^a givn project with a change "([^"]+)" that has design\.md with a valid Step Definitions table and tasks\.md containing a placeholder COMMIT hash "([^"]+)"$"#
)]
fn e2e_given_change_with_placeholder_hash(world: &mut GivnWorld, id: String, hash: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::harness::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::write(
        change_dir.join("design.md"),
        "| Step | File |\n| --- | --- |\n| Given x | steps/x.rs |\n",
    )
    .expect("write design.md");
    std::fs::write(
        change_dir.join("tasks.md"),
        format!("- [x] COMMIT: `{}`\n", hash),
    )
    .expect("write tasks.md");
    // Override verify.command so the integrity hook runs.
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  measure_command: \"./measure-coverage.sh\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_fixture_policy(env);
    world.last_change_id = Some(id);
}

fn configure_fixture_policy(env: &crate::harness::TestEnv) {
    common::replace_config_addons(
        env,
        "addons:\n  arc42: true\n  readme: true\n  coverage: true\n  domain_modeling: true\n  dev_principles: true\n  outline: false\n",
    );
    common::write_fixture_coverage_measure(
        env,
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    );
}

fn write_permanent_example(env: &crate::harness::TestEnv) {
    let dir = env.path("givn/specs/example");
    std::fs::create_dir_all(&dir).expect("create permanent usecase dir");
    std::fs::write(
        dir.join("example.feature"),
        "Feature: Example\n\n  Scenario: Existing behavior\n    Given a fixture step\n",
    )
    .expect("write permanent feature");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", "example")
        .replace("<capability-id>", "example")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "No E2E interaction")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| example | exercise the capability | No E2E interaction |\n",
            "- none\n",
        );
    std::fs::write(dir.join("usecase.md"), document).expect("write permanent usecase");
}
