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

    // This legacy fixture verifies structural dispositions and net-delta output.
    // Keep its two added scenarios structurally present but make the semantic
    // seed obsolete so the remediation gate does not change that contract.
    let delta_path = change_dir(world).join("specs/cap/cap.feature");
    let delta = std::fs::read_to_string(&delta_path).expect("read disposition delta");
    let delta = delta
        .replace(
            "Scenario: Reviewed shape one",
            "Scenario: Reviewed shape one distinct audit alpha",
        )
        .replace(
            "Scenario: Reviewed shape two",
            "Scenario: Reviewed shape two distinct audit beta",
        )
        .replace(
            "Then the command exits 0",
            "Then the semantic fixture records distinct audit evidence",
        );
    std::fs::write(delta_path, delta).expect("rewrite disposition delta");
    let updated_review = std::fs::read_to_string(&path)
        .expect("read disposition review")
        .replace(&title_a, "Reviewed shape one distinct audit alpha")
        .replace(&title_b, "Reviewed shape two distinct audit beta");
    std::fs::write(path, updated_review).expect("rewrite disposition review");
    let semantic_report = change_dir(world).join("semantic-review.md");
    if semantic_report.exists() {
        std::fs::remove_file(semantic_report).expect("remove obsolete semantic seed");
    }
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
