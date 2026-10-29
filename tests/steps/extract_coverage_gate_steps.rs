use std::io::Write;

use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given("the givn/config.yaml has the coverage addon enabled with valid commands")]
fn coverage_addon_enabled(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
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

#[given(
    "the givn/config.yaml has the coverage addon enabled but the non-e2e coverage command exits non-zero"
)]
fn coverage_command_fails(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"\"
coverage:
  measure_command: \"false\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_fixture_policy(env);
}

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
            writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] COMMIT: a1b2c3d4 -- all done").ok();
        }
    }
    let permanent = env.path("givn/specs/example");
    std::fs::create_dir_all(&permanent).expect("create permanent usecase dir");
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", "example")
        .replace("<capability-id>", "example")
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "stub");
    std::fs::write(permanent.join("usecase.md"), document).expect("write permanent usecase");
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .unwrap();
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id.clone());
    crate::common::commit_fixture(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
}

#[given("a givn project with the coverage addon enabled and valid coverage commands")]
fn coverage_addon_enabled_alias(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_addon_enabled(world);
    with_run_declaration(world);
}

#[given(
    "a givn project with the coverage addon enabled but the non-e2e coverage command exits non-zero"
)]
fn coverage_command_fails_alias(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_command_fails(world);
    with_run_declaration(world);
}

/// Append the complete run declaration to the fixture's commands file.
fn with_run_declaration(world: &GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated =
        common::replace_top_level_key(&content, "run", &common::complete_run_declaration("cli"));
    std::fs::write(&path, updated).expect("write commands.yaml");
}

use std::os::unix::fs::PermissionsExt;

use crate::steps::archive_proof_support as support;
use cucumber::then;

fn coverage_fixture(world: &mut GivnWorld, id: &str, body: &str) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    let env = world.env.as_ref().unwrap();
    support::write_result_runner(env, "pass");
    support::configure_commands(env);
    support::enable_coverage(env);
    let log = format!("givn/archive/{id}/results/coverage-invocations.log");
    let measure = format!("#!/bin/sh\nmkdir -p coverage\necho run >> \"{log}\"\n{body}\n");
    let merge = "#!/bin/sh\nmkdir -p \"$(dirname \"$GIVN_COVERAGE_OUTPUT_PATH\")\"\ncp \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_OUTPUT_PATH\"\n";
    for (name, content) in [
        ("emit-coverage.sh", measure),
        ("merge-coverage.sh", merge.to_string()),
    ] {
        let path = env.path(name);
        std::fs::write(&path, content).expect("write coverage script");
        let mut permissions = std::fs::metadata(&path)
            .expect("script metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&path, permissions).expect("make script executable");
    }
    support::write_change_artifacts(env, id, false);
}

fn valid_reports(covered: u64) -> String {
    let report = format!(
        r#"<?xml version="1.0"?>
<coverage line-rate="{}" branch-rate="0" lines-covered="{covered}" lines-valid="4" branches-covered="0" branches-valid="0" timestamp="0" version="1">
  <packages />
</coverage>"#,
        covered as f64 / 4.0
    );
    format!(
        "cat > \"$GIVN_COVERAGE_NON_E2E_PATH\" <<'XML'\n{report}\nXML\ncat > \"$GIVN_COVERAGE_E2E_PATH\" <<'XML'\n{report}\nXML",
    )
}

fn commit_and_refresh(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::commit_fixture(env);
    support::refresh_change_receipt(world, id);
    world.last_change_id = Some(id.to_string());
}

fn finalization_hook(env: &common::TestEnv, rejecting: bool) {
    let hooks = env.path(".git/hooks");
    std::fs::create_dir_all(&hooks).expect("create hooks dir");
    let script = if rejecting {
        "#!/bin/sh\nif grep -q 'prove archive' \"$1\"; then\n  echo 'rejected finalization: prove archive' >&2\n  exit 1\nfi\nexit 0\n"
    } else {
        "#!/bin/sh\nexit 0\n"
    };
    let hook = hooks.join("commit-msg");
    std::fs::write(&hook, script).expect("write finalization hook");
    let mut permissions = std::fs::metadata(&hook)
        .expect("hook metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&hook, permissions).expect("make hook executable");
}

fn invocation_count(world: &GivnWorld, id: &str) -> usize {
    std::fs::read_to_string(support::root(world).join(format!(
        "givn/archive/{id}/results/coverage-invocations.log"
    )))
    .map(|content| content.lines().count())
    .unwrap_or(0)
}

#[given("an unproven archive candidate whose verification scopes passed but coverage failed")]
fn unproven_candidate_failed_coverage(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_fixture(world, "cov-repair", "");
    let env = world.env.as_ref().unwrap();
    let script = "#!/bin/sh\nmkdir -p coverage\necho run >> \"givn/archive/cov-repair/results/coverage-invocations.log\"\nif [ -f \"givn/archive/cov-repair/evidence/coverage-fixed\" ]; then\n  cat > \"$GIVN_COVERAGE_NON_E2E_PATH\" <<'XML'\n<?xml version=\"1.0\"?>\n<coverage line-rate=\"0.5\" branch-rate=\"0\" lines-covered=\"2\" lines-valid=\"4\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\">\n  <packages />\n</coverage>\nXML\n  cp \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_E2E_PATH\"\nelse\n  exit 1\nfi\n";
    std::fs::write(env.path("emit-coverage.sh"), script).expect("write marker coverage script");
    let mut permissions = std::fs::metadata(env.path("emit-coverage.sh"))
        .expect("script metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(env.path("emit-coverage.sh"), permissions).expect("chmod");
    let _ = env;
    commit_and_refresh(world, "cov-repair");
    let out = support::run_archive(world, "cov-repair");
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_output = Some(out);
    world.last_change_id = Some("cov-repair".to_string());
}

#[given("the coverage failure is repaired without changing the scopes' relevant inputs")]
fn coverage_failure_repaired(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let log = format!("givn/archive/{id}/results/invocations.log");
    world.verification_log_before =
        Some(std::fs::read_to_string(support::root(world).join(&log)).unwrap_or_default());
    let marker = support::root(world).join(format!("givn/archive/{id}/evidence/coverage-fixed"));
    std::fs::create_dir_all(marker.parent().unwrap()).expect("create evidence dir");
    std::fs::write(&marker, "fixed\n").expect("write repair marker");
}

#[then("coverage is measured again and fresh valid reports allow the archive to become proven")]
fn coverage_remeasured_and_proven(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(
        receipt["status"],
        "proven",
        "stderr: {}",
        crate::steps::stderr(world)
    );
    assert_eq!(receipt["coverage"]["status"], "passed");
    assert!(invocation_count(world, &id) >= 2);
}

#[then("the passed verification scopes are reused without another suite execution")]
fn passed_scopes_reused(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let before = world
        .verification_log_before
        .clone()
        .expect("fixture log snapshot");
    let after = std::fs::read_to_string(
        support::root(world).join(format!("givn/archive/{id}/results/invocations.log")),
    )
    .unwrap_or_default();
    assert_eq!(before, after, "a verification scope ran again");
}

#[given("a completed change is ready to archive with coverage enabled")]
fn completed_change_with_coverage(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_fixture(world, "cov-demo", &valid_reports(2));
    commit_and_refresh(world, "cov-demo");
}

#[given(
    "configured regular and end-to-end coverage executions each write fresh valid reports covering identical source lines"
)]
fn equal_coverage_reports(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("emit-coverage.sh");
    let content = std::fs::read_to_string(&path).expect("read measure script");
    assert!(content.contains("GIVN_COVERAGE_NON_E2E_PATH"));
    assert_eq!(invocation_count(world, "cov-demo"), 0);
}

#[then("coverage equality does not block the proven archive")]
fn equal_coverage_does_not_block(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(
        support::receipt_status(world, &id),
        "proven",
        "stderr: {}",
        crate::steps::stderr(world)
    );
    assert_eq!(support::receipt(world, &id)["coverage"]["status"], "passed");
}

#[then("no percentage threshold or additional provenance requirement replaces the equality gate")]
fn no_threshold_or_provenance(world: &mut GivnWorld) {
    let stdout = crate::steps::stdout(world);
    assert!(!stdout.to_lowercase().contains("threshold"));
    assert!(!stdout.to_lowercase().contains("identical"));
}

#[then("each configured coverage measurement executes once without an extra isolation run")]
fn coverage_measurement_once(world: &mut GivnWorld) {
    assert_eq!(invocation_count(world, "cov-demo"), 1);
}

#[given("an archive candidate has old valid coverage reports")]
fn old_valid_coverage_reports(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    support::write_result_runner(env, "pass");
    support::configure_commands(env);
    support::enable_coverage(env);
    for path in [
        "coverage/non-e2e-cobertura.xml",
        "coverage/e2e-cobertura.xml",
    ] {
        let target = env.path(path);
        std::fs::create_dir_all(target.parent().unwrap()).expect("create coverage dir");
        std::fs::write(
            target,
            "<?xml version=\"1.0\"?>\n<coverage line-rate=\"0.5\" branch-rate=\"0\" lines-covered=\"2\" lines-valid=\"4\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\">\n  <packages />\n</coverage>\n",
        )
        .expect("write old report");
    }
    support::write_change_artifacts(env, "cov-demo", false);
    commit_and_refresh(world, "cov-demo");
}

#[given("coverage is required and its command exits successfully without writing new reports")]
fn coverage_command_writes_nothing(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let measure = "#!/bin/sh\nfor p in \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_E2E_PATH\"; do\n  if [ -e \"$p\" ]; then echo present >> \"givn/archive/cov-demo/results/coverage-removal.log\"; fi\ndone\nexit 0\n";
    let merge = "#!/bin/sh\nexit 0\n";
    for (name, content) in [("emit-coverage.sh", measure), ("merge-coverage.sh", merge)] {
        let path = env.path(name);
        std::fs::write(&path, content).expect("write coverage script");
        let mut permissions = std::fs::metadata(&path)
            .expect("script metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&path, permissions).expect("chmod");
    }
    commit_and_refresh(world, "cov-demo");
}

#[then("the old measurement inputs were removed before coverage execution")]
fn old_inputs_removed(world: &mut GivnWorld) {
    let root = support::root(world);
    let log = root.join("givn/archive/cov-demo/results/coverage-removal.log");
    let content = std::fs::read_to_string(log).unwrap_or_default();
    assert!(
        !content.contains("present"),
        "stale coverage input was visible during measurement: {content}"
    );
    for path in [
        "coverage/non-e2e-cobertura.xml",
        "coverage/e2e-cobertura.xml",
    ] {
        assert!(
            !root.join(path).exists(),
            "old coverage input survived: {path}"
        );
    }
}

#[then("the candidate remains unproven because fresh coverage reports are missing")]
fn unproven_missing_coverage(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.to_lowercase().contains("coverage"),
        "stderr lacks coverage failure: {stderr}"
    );
}

#[given("an archive candidate is ready for coverage measurement")]
fn ready_for_coverage(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_fixture(world, "cov-demo", &valid_reports(2));
    commit_and_refresh(world, "cov-demo");
}

#[given(regex = r#"^the coverage command exits successfully but produces (.+)$"#)]
fn coverage_command_invalid(world: &mut GivnWorld, report: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let body = match report.as_str() {
        "malformed coverage output" => {
            "printf '<coverage line-rate=\"oops\"' > \"$GIVN_COVERAGE_NON_E2E_PATH\"\ncp \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_E2E_PATH\"".to_string()
        }
        "an empty report" => {
            ": > \"$GIVN_COVERAGE_NON_E2E_PATH\"\n: > \"$GIVN_COVERAGE_E2E_PATH\"".to_string()
        }
        other => panic!("unknown coverage report shape '{other}'"),
    };
    let measure = format!(
        "#!/bin/sh\nmkdir -p coverage\necho run >> \"givn/archive/cov-demo/results/coverage-invocations.log\"\n{body}\n"
    );
    let path = env.path("emit-coverage.sh");
    std::fs::write(&path, measure).expect("write invalid coverage script");
    let mut permissions = std::fs::metadata(&path)
        .expect("script metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&path, permissions).expect("chmod");
    commit_and_refresh(world, "cov-demo");
}

#[then("the command exits non-zero and names the invalid coverage output")]
fn names_invalid_coverage(world: &mut GivnWorld) {
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.to_lowercase().contains("coverage"),
        "stderr lacks coverage: {stderr}"
    );
    assert!(
        stderr.contains("parse") || stderr.contains("validation"),
        "stderr does not name invalid output: {stderr}"
    );
}

#[then("the candidate remains unproven")]
fn candidate_remains_unproven(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    assert_eq!(support::receipt_status(world, &id), "pending");
}

#[given("an archive candidate has fresh valid coverage and passed verification evidence")]
fn coverage_passed_finalization_failed(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_fixture(world, "cov-demo", &valid_reports(2));
    let env = world.env.as_ref().unwrap();
    finalization_hook(env, true);
    commit_and_refresh(world, "cov-demo");
    let out = support::run_archive(world, "cov-demo");
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_output = Some(out);
    world.last_change_id = Some("cov-demo".to_string());
    let receipt = support::receipt(world, "cov-demo");
    assert_eq!(
        receipt["status"],
        "pending",
        "stderr: {}",
        crate::steps::stderr(world)
    );
    assert_eq!(
        receipt["coverage"]["status"],
        "passed",
        "stderr: {}",
        crate::steps::stderr(world)
    );
    world.verification_log_before = Some(
        std::fs::read_to_string(
            support::root(world).join("givn/archive/cov-demo/results/invocations.log"),
        )
        .unwrap_or_default(),
    );
    world.verification_invocations_before = Some(invocation_count(world, "cov-demo"));
}

#[given("only finalization publication failed")]
fn only_finalization_failed(world: &mut GivnWorld) {
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("rejected finalization"),
        "stderr lacks finalization rejection: {stderr}"
    );
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    finalization_hook(env, false);
}

#[given("relevant measurement inputs remain unchanged")]
fn measurement_inputs_unchanged(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    world.verification_invocations_before = Some(invocation_count(world, &id));
}

#[then("the archive becomes proven using its recorded coverage")]
fn proven_with_recorded_coverage(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert_eq!(receipt["coverage"]["status"], "passed");
}

#[then("neither coverage nor a verification suite executes again")]
fn no_coverage_or_suite(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let coverage_before = world
        .verification_invocations_before
        .expect("coverage invocation snapshot");
    assert_eq!(invocation_count(world, &id), coverage_before);
    let suite_before = world
        .verification_log_before
        .as_deref()
        .expect("suite invocation snapshot");
    let suite_after = std::fs::read_to_string(
        support::root(world).join(format!("givn/archive/{id}/results/invocations.log")),
    )
    .unwrap_or_default();
    assert_eq!(suite_before, suite_after, "a verification suite ran again");
}

/// Addon policy belongs in `givn/config.yaml`, and the archive coverage gate
/// removes stale reports before measuring: the fixture measure command writes
/// both reports during the run.
fn configure_fixture_policy(env: &crate::common::TestEnv) {
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
