use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{given, then};

#[given("a givn project with verify.e2e_command unconfigured in givn/commands.yaml")]
fn e2e_given_unconfigured_e2e_command(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"givn missing-e2e-runner\"
coverage:
  measure_command: \"./measure-coverage.sh\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_fixture_policy(env);
}

#[then("stderr says the e2e runner is not configured for permanent specs with @e2e coverage")]
fn e2e_stderr_says_not_configured(world: &mut GivnWorld) {
    let s = crate::steps::stderr(world);
    assert!(
        s.contains("not configured"),
        "expected stderr to mention 'not configured', got: {}",
        s
    );
}

#[then("stderr names givn/commands.yaml as the e2e runner's configuration file")]
fn e2e_stderr_names_commands_yaml(world: &mut GivnWorld) {
    let s = crate::steps::stderr(world);
    assert!(
        s.contains("givn/commands.yaml"),
        "expected stderr to name givn/commands.yaml, got: {}",
        s
    );
}

#[given("the permanent spec directory givn/specs/ has no @e2e scenario")]
fn e2e_given_permanent_specs_no_e2e(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let specs_dir = env.path("givn/specs/existing");
    std::fs::create_dir_all(&specs_dir).expect("create permanent specs dir");
    std::fs::write(
        specs_dir.join("existing.feature"),
        "Feature: existing\n  Scenario: existing\n    Given a placeholder step\n",
    )
    .unwrap();
    write_permanent_existing(env);
}

#[given("the change directory still exists under givn/changes/")]
fn change_dir_still_exists(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let id = world.last_change_id.clone().expect("change id required");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    assert!(
        change_dir.exists(),
        "change dir {:?} should still exist (archive should have rolled back)",
        change_dir
    );
}

#[given("a committed givn project with verify.e2e_command unconfigured in givn/commands.yaml")]
fn committed_unconfigured_e2e(world: &mut GivnWorld) {
    e2e_given_unconfigured_e2e_command(world);
}

#[given(
    regex = r#"^a change "([^"]+)" with all artifacts done, whose delta spec has an @e2e scenario$"#
)]
fn e2e_given_change_with_e2e_delta(world: &mut GivnWorld, id: String) {
    use std::io::Write;
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
    write_permanent_usecase(env, "example", true);
    let specs_dir = change_dir.join("specs/example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n\n  @e2e\n  Scenario: e2e test\n    Given a placeholder step\n",
    )
    .unwrap();
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id);
}

fn write_permanent_usecase(env: &crate::harness::TestEnv, usecase: &str, e2e: bool) {
    let path = env.path(&format!("givn/specs/{usecase}/usecase.md"));
    std::fs::create_dir_all(path.parent().expect("usecase parent"))
        .expect("create permanent usecase dir");
    let mut document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", usecase)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "e2e test");
    if !e2e {
        let table = format!(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| {usecase} | exercise the capability | e2e test |\n"
        );
        document = document.replace(&table, "- none\n");
    }
    std::fs::write(path, document).expect("write permanent usecase");
}

fn write_permanent_existing(env: &crate::harness::TestEnv) {
    let path = env.path("givn/specs/existing/usecase.md");
    std::fs::create_dir_all(path.parent().expect("usecase parent"))
        .expect("create permanent usecase dir");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", "existing")
        .replace("<capability-id>", "existing")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "existing")
        .replace(
            "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| existing | exercise the capability | existing |\n",
            "- none\n",
        );
    std::fs::write(path, document).expect("write permanent usecase");
}

fn configure_fixture_policy(env: &crate::harness::TestEnv) {
    common::replace_config_addons(
        env,
        "addons:\n  arc42: true\n  readme: true\n  coverage: true\n  domain_modeling: true\n  dev_principles: true\n  outline: false\n",
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
