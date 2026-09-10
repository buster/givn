use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then};

#[given("a givn project with verify.e2e_command unconfigured in givn/config.yaml")]
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
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");
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
}

#[given("the givn/config.yaml has e2e_command unconfigured")]
fn unconfigured_e2e_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"givn missing-e2e-runner\"
coverage:
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");
}

#[given("the givn/config.yaml has e2e_command set to \"true\"")]
fn configured_e2e_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"./emit-result.sh e2e\"
coverage:
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");
}

#[given(regex = r#"^a completed change "([^"]+)" whose delta contains an @e2e-tagged scenario$"#)]
fn completed_change_with_e2e_delta(world: &mut GivnWorld, id: String) {
    use std::io::Write;
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(out.status.success(), "givn new {} failed", id);
    }
    // Write all required artifacts.
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
            writeln!(f, "REVIEW: PASS\nREADME-IMPACT: none").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] all done").ok();
        }
    }
    // Create specs dir with an @e2e-tagged scenario.
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n\n  @e2e\n  Scenario: e2e test\n    Given a placeholder step\n",
    )
    .unwrap();
    // Write arc42 marker.
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose delta contains zero @e2e-tagged scenarios$"#
)]
fn completed_change_no_e2e_delta(world: &mut GivnWorld, id: String) {
    use std::io::Write;
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(out.status.success(), "givn new {} failed", id);
    }
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
            writeln!(f, "REVIEW: PASS\nREADME-IMPACT: none").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] all done").ok();
        }
    }
    // Create specs dir with no @e2e scenario.
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n\n  Scenario: unit test\n    Given a placeholder step\n",
    )
    .unwrap();
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id);
}

#[given("the permanent specs currently have no @e2e scenarios")]
fn permanent_specs_no_e2e(world: &mut GivnWorld) {
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

#[then("stderr mentions the e2e runner is not configured")]
fn stderr_mentions_e2e_not_configured(world: &mut GivnWorld) {
    let s = crate::steps::stderr(world);
    assert!(
        s.contains("not configured"),
        "expected stderr to mention 'not configured', got: {}",
        s
    );
}

// ─── forward-only archive fixtures ────────────────────────────────────────

/// A project whose verify and e2e commands emit reconciling results, with
/// valid Cobertura coverage commands.
fn e2e_project(world: &mut GivnWorld, e2e_command: &str) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = format!(
        "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"{e2e_command}\"
coverage:
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
"
    );
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");
}

#[given("a givn project whose e2e_command reports scenario counts")]
fn project_with_counting_e2e(world: &mut GivnWorld) {
    e2e_project(world, "./emit-result.sh e2e");
}

#[given("a givn project with the e2e_command set to \"false\"")]
fn project_with_failing_e2e(world: &mut GivnWorld) {
    e2e_project(world, "false");
}

#[given("a committed givn project with verify.e2e_command unconfigured in givn/config.yaml")]
fn committed_unconfigured_e2e(world: &mut GivnWorld) {
    e2e_given_unconfigured_e2e_command(world);
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose delta contains an @e2e-tagged scenario and is ready to archive$"#
)]
fn completed_change_with_e2e_ready(world: &mut GivnWorld, id: String) {
    completed_change_with_e2e_delta(world, id);
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::git_commit_all(env, "fixture: e2e change artifacts");
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
    let out = crate::common::run_givn(env, &["new", &id]);
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
            writeln!(f, "REVIEW: PASS\nREADME-IMPACT: none").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] all done").ok();
        }
    }
    let specs_dir = change_dir.join("specs/e2e-check");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n\n  @e2e\n  Scenario: e2e test\n    Given a placeholder step\n",
    )
    .unwrap();
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id);
}
