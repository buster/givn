use crate::common;
use crate::steps::GivnWorld;
use cucumber::given;

#[given(
    regex = r#"^a givn project with a change "([^"]+)" that has design\.md with a valid Step Definitions table and tasks\.md where every COMMIT hash is a valid 40-character hex string$"#
)]
fn e2e_given_change_with_design_and_valid_hashes(world: &mut GivnWorld, id: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::common::run_givn(env, &["new", &id]);
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
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a givn project with a change "([^"]+)" that has design\.md with a valid Step Definitions table and tasks\.md containing a placeholder COMMIT hash "([^"]+)"$"#
)]
fn e2e_given_change_with_placeholder_hash(world: &mut GivnWorld, id: String, hash: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().unwrap();
    let change_dir = env.path(&format!("givn/changes/{}", id));
    let out = crate::common::run_givn(env, &["new", &id]);
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
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a change "([^"]+)" exists with specs under "([^"]+)"$"#)]
fn change_with_specs(world: &mut GivnWorld, id: String, specs_rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let specs_dir = env.path(&format!("givn/changes/{id}/{specs_rel}example"));
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .unwrap();
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" exists with design\.md containing a valid Step Definitions table$"#
)]
fn change_with_design_md(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(out.status.success(), "givn new {} failed", id);
    }
    std::fs::write(
        change_dir.join("design.md"),
        "| Step | File |\n| --- | --- |\n| Given x | steps/x.rs |\n",
    )
    .expect("write design.md");
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" exists with tasks\.md where every COMMIT hash is a valid 40-character hex string$"#
)]
fn change_with_valid_hashes(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(out.status.success(), "givn new {} failed", id);
    }
    std::fs::write(
        change_dir.join("tasks.md"),
        "- [x] task: `a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0`\n\
         - [x] COMMIT: `a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0`\n",
    )
    .expect("write tasks.md");
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" exists with specs under "([^"]+)" containing zero @e2e scenarios$"#
)]
fn change_with_specs_no_e2e(world: &mut GivnWorld, id: String, specs_rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let specs_dir = env.path(&format!("givn/changes/{id}/{specs_rel}example"));
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .unwrap();
    world.last_change_id = Some(id);
}
