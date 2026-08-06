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
    // Create specs dir with a valid feature file.
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(specs_dir.join("example.feature"), "Feature: example\n").unwrap();
    // Write STATUS: DONE marker for arc42-docs gate.
    std::fs::write(
        change_dir.join("arc42.md"),
        "## Status\n\nSTATUS: DONE\n",
    )
    .unwrap();
}
