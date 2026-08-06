use std::io::Write;

use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given(
    "the givn/config.yaml has the coverage addon enabled with valid commands"
)]
fn coverage_addon_enabled(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  non_e2e_command: \"true\"
  e2e_command: \"true\"
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
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  non_e2e_command: \"false\"
  e2e_command: \"true\"
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
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"\"
coverage:
  non_e2e_command: \"true\"
  e2e_command: \"true\"
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
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        if *file == "design-review.md" {
            writeln!(f, "DESIGN-REVIEW: PASS").ok();
        } else if *file == "review.md" {
            writeln!(f, "REVIEW: PASS").ok();
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
