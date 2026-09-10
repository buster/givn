use std::io::Write;

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then};

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
            writeln!(f, "REVIEW: PASS\nREADME-IMPACT: none").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] all done").ok();
        }
    }
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .unwrap();
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    world.last_change_id = Some(id);
}

/// One fixture source report with explicit per-line data. `hits` maps the
/// line number to its hit count; the root attributes are arithmetically
/// consistent so `validate` accepts the report.
fn fixture_report_xml(hits: &[(u64, u64)]) -> String {
    let covered = hits.iter().filter(|(_, h)| *h > 0).count();
    let valid = hits.len();
    let lines = hits
        .iter()
        .map(|(number, count)| format!("            <line number=\"{number}\" hits=\"{count}\"/>"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r#"<?xml version="1.0"?>
<coverage line-rate="{rate}" branch-rate="0" lines-covered="{covered}" lines-valid="{valid}" branches-covered="0" branches-valid="0" timestamp="0" version="1">
  <sources>
    <source>.</source>
  </sources>
  <packages>
    <package line-rate="{rate}" branch-rate="0" complexity="0" name="src">
      <classes>
        <class branch-rate="0" complexity="0" filename="src/lib.rs" line-rate="{rate}" name="lib">
          <methods/>
          <lines>
{lines}
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#,
        rate = covered as f64 / valid as f64,
        covered = covered,
        valid = valid,
        lines = lines,
    )
}

/// A project with the legacy two-command coverage schema: the sentinel
/// measure/merge commands plus `non_e2e_command`/`e2e_command` scripts that
/// emit the two source reports with explicit per-line data. This is the
/// shape `write_legacy_merged_report` merges with the built-in merge.
fn legacy_coverage_project(world: &mut GivnWorld, non_e2e: &[(u64, u64)], e2e: &[(u64, u64)]) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"\"
coverage:
  measure_command: \"givn missing-coverage\"
  merge_command: \"givn missing-coverage\"
  non_e2e_command: \"./emit-non-e2e-report.sh\"
  e2e_command: \"./emit-e2e-report.sh\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
  output_path: \"coverage/cobertura-coverage.xml\"
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
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    for (script, env_var, hits) in [
        (
            "emit-non-e2e-report.sh",
            "GIVN_COVERAGE_NON_E2E_PATH",
            non_e2e,
        ),
        ("emit-e2e-report.sh", "GIVN_COVERAGE_E2E_PATH", e2e),
    ] {
        let path = env.path(script);
        std::fs::write(
            &path,
            format!(
                "#!/bin/sh\nmkdir -p coverage\ncat > \"${env_var}\" <<'XML'\n{}\nXML\n",
                fixture_report_xml(hits)
            ),
        )
        .expect("write report script");
        make_executable(&path);
    }
}

fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path)
        .expect("read script metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).expect("make script executable");
}

#[given("a givn project with legacy coverage commands whose reports measure the same lines")]
fn legacy_coverage_same_lines(world: &mut GivnWorld) {
    // Non-e2e: lines 1,2 hit, 3,4 measured but missed (2/4).
    // E2e: all four lines hit (4/4). Both reports measure the same four
    // lines and stay distinct; the union is 4/4, the defective sum 6/8.
    legacy_coverage_project(
        world,
        &[(1, 1), (2, 1), (3, 0), (4, 0)],
        &[(1, 1), (2, 1), (3, 1), (4, 1)],
    );
}

#[given(
    "a givn project with legacy coverage commands whose second report measures lines the first does not"
)]
fn legacy_coverage_disjoint_lines(world: &mut GivnWorld) {
    // Non-e2e: lines 1,2 hit, 3 measured but missed (2/3).
    // E2e: line 2 hit, line 4 measured but missed (1/2). The union counts
    // lines 1-4 once each and covers {1,2}: 2/4. The defective sum: 3/5.
    legacy_coverage_project(world, &[(1, 1), (2, 1), (3, 0)], &[(2, 1), (4, 0)]);
}

#[then(regex = r#"^the merged Cobertura report contains line coverage `(\d+)/(\d+)`$"#)]
fn merged_report_has_line_counters(world: &mut GivnWorld, covered: u64, valid: u64) {
    let env = world.env.as_ref().expect("environment is set");
    let report = std::fs::read_to_string(env.path("coverage/cobertura-coverage.xml"))
        .expect("read merged report");
    assert!(
        report.contains(&format!("lines-covered=\"{covered}\"")),
        "merged report does not claim lines-covered=\"{covered}\": {report}"
    );
    assert!(
        report.contains(&format!("lines-valid=\"{valid}\"")),
        "merged report does not claim lines-valid=\"{valid}\": {report}"
    );
}

#[given("a givn project with the coverage addon enabled and valid coverage commands")]
fn coverage_addon_enabled_alias(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_addon_enabled(world);
    with_run_declaration(world);
}

#[given("a givn project with the coverage addon enabled but the non-e2e coverage command exits non-zero")]
fn coverage_command_fails_alias(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    coverage_command_fails(world);
    with_run_declaration(world);
}

/// Append the complete run declaration to the fixture's commands file.
fn with_run_declaration(world: &GivnWorld) {
    let env = world.env.as_ref().expect("a project root is required first");
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated =
        common::replace_top_level_key(&content, "run", &common::complete_run_declaration("cli"));
    std::fs::write(&path, updated).expect("write commands.yaml");
}
