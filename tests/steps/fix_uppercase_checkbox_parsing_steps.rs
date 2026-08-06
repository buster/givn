use cucumber::given;
use crate::steps::GivnWorld;

#[given(regex = r#"^a change "([^"]+)" exists with artifact "([^"]+)" containing "([^"]*)"$"#)]
fn change_with_artifact_containing(world: &mut GivnWorld, id: String, artifact: String, content: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(
            out.status.success(),
            "givn new {} failed: {}",
            id,
            String::from_utf8_lossy(&out.stderr),
        );
    }
    world.last_change_id = Some(id.clone());
    let content = content.replace("\\n", "\n");
    std::fs::write(change_dir.join(&artifact), content).expect("write artifact file");
}

#[given(regex = r#"^a change "([^"]+)" exists with all required artifacts complete and tasks\.md containing "([^"]*)" with a valid COMMIT hash$"#)]
fn change_all_complete_with_tasks_content(world: &mut GivnWorld, id: String, tasks_content: String) {
    use std::io::Write;
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_change_id = Some(id.clone());
    let change_dir = env.path(&format!("givn/changes/{}", id));
    if !change_dir.exists() {
        let out = crate::common::run_givn(env, &["new", &id]);
        assert!(
            out.status.success(),
            "givn new {} failed: {}",
            id,
            String::from_utf8_lossy(&out.stderr),
        );
    }
    // Write all required artifacts with proper completion markers.
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
            let content = tasks_content.replace("\\n", "\n");
            write!(f, "{}", content).ok();
        }
    }
    // Create specs dir with a valid @givn.delta-tagged feature file.
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(specs_dir.join("example.feature"), "@givn.delta\nFeature: example\n").unwrap();
    // Write STATUS: DONE marker for arc42-docs gate.
    std::fs::write(
        change_dir.join("arc42.md"),
        "## Status\n\nSTATUS: DONE\n",
    )
    .unwrap();
    // Override commands.yaml so verify and coverage gates pass in test environment.
    // The temp project root has no Rust project or coverage tooling, so we set
    // everything to simple `true` commands and write dummy Cobertura XML files.
    let commands_path = env.path("givn/commands.yaml");
    let commands_yaml = "\
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
    std::fs::write(&commands_path, commands_yaml).expect("write patched commands.yaml");
    // Write distinct dummy Cobertura XML files so the coverage gate passes.
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    let non_e2e_xml = r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#;
    let e2e_xml = r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#;
    std::fs::write(cov_dir.join("non-e2e-cobertura.xml"), non_e2e_xml).expect("write non-e2e xml");
    std::fs::write(cov_dir.join("e2e-cobertura.xml"), e2e_xml).expect("write e2e xml");
}
