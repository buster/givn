use cucumber::given;

use crate::common;
use crate::steps::GivnWorld;

fn change_dir(world: &GivnWorld) -> std::path::PathBuf {
    let root = world
        .env
        .as_ref()
        .map(|e| e.path(""))
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    root.join(format!("givn/changes/{id}"))
}

#[given(
    regex = r#"^the change's review\.md contains a disposition row naming "([^"]+)" and "([^"]+)"$"#
)]
fn review_md_disposition_row(world: &mut GivnWorld, title_a: String, title_b: String) {
    let path = change_dir(world).join("review.md");
    let content = format!(
        "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `{title_a}` | `{title_b}` | boundary |\n"
    );
    std::fs::write(&path, content).expect("write review.md disposition row");

    // The blocking semantic gate is part of `check review`; seed its report
    // classification so this legacy disposition fixture remains a complete pass.
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let output = common::run_givn_with_envs(
        env,
        &["check", "review", "--change", "test-change"],
        &[("GIVN_SPEC_EMBEDDER", "stub")],
    );
    assert!(
        !output.status.success(),
        "semantic seed review unexpectedly passed"
    );
    let report_path = change_dir(world).join("semantic-review.md");
    let report = std::fs::read_to_string(&report_path).expect("semantic seed report");
    let report = report
        .replace("- classification: UNCERTAIN", "- classification: DUPLIKAT")
        .replace(
            "- rationale: null",
            "- rationale: \"The existing disposition covers this behavior\"",
        );
    std::fs::write(report_path, report).expect("classify semantic seed report");
}

#[given(regex = r#"^the change's review\.md contains a split-or-keep row for "([^"]+)"$"#)]
fn review_md_split_or_keep_row(world: &mut GivnWorld, title: String) {
    let path = change_dir(world).join("review.md");
    let content =
        format!("## Split-or-keep\n\n| Scenario | Decision |\n|---|---|\n| `{title}` | keep |\n");
    std::fs::write(&path, content).expect("write review.md split-or-keep row");
}

#[given("the change's review.md has no disposition entries")]
fn review_md_no_dispositions(world: &mut GivnWorld) {
    let path = change_dir(world).join("review.md");
    std::fs::write(&path, "# Review\n\nNo overlap dispositions recorded.\n")
        .expect("write review.md without dispositions");
}
