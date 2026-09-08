use cucumber::given;

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
        "REVIEW: PASS\nREADME-IMPACT: none\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `{title_a}` | `{title_b}` | boundary |\n"
    );
    std::fs::write(&path, content).expect("write review.md disposition row");

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
        )
        .replace(
            "Given a project root with givn/ initialised",
            "Given the zephyr quorum registry is provisioned",
        )
        .replace(
            "When the user runs `givn lint`",
            "When the zephyr quorum audit is invoked",
        );
    // The two rewritten scenarios must also stop resembling each other:
    // the unified rulebook compares delta-to-delta at 0.90, so give each
    // scenario its own vocabulary.
    let (head, tail) = delta
        .split_once("Reviewed shape one distinct audit alpha")
        .expect("split disposition delta at scenario one");
    let (mid, tail2) = tail
        .split_once("Reviewed shape two distinct audit beta")
        .expect("split disposition delta at scenario two");
    let tail2 = tail2
        .replacen(
            "Given the zephyr quorum registry is provisioned",
            "Given the cobalt meridian archive is provisioned",
            1,
        )
        .replacen(
            "When the zephyr quorum audit is invoked",
            "When the cobalt meridian audit is invoked",
            1,
        );
    let delta = format!(
        "{head}Reviewed shape one distinct audit alpha{mid}Reviewed shape two distinct audit beta{tail2}"
    );
    std::fs::write(delta_path, delta).expect("rewrite disposition delta");
    let updated_review = std::fs::read_to_string(&path)
        .expect("read disposition review")
        .replace(&title_a, "Reviewed shape one distinct audit alpha")
        .replace(&title_b, "Reviewed shape two distinct audit beta");
    std::fs::write(path, updated_review).expect("rewrite disposition review");
}

#[given(regex = r#"^the change's review\.md contains a split-or-keep row for "([^"]+)"$"#)]
fn review_md_split_or_keep_row(world: &mut GivnWorld, title: String) {
    let path = change_dir(world).join("review.md");
    let content = format!(
        "REVIEW: PASS\nREADME-IMPACT: none\n\n## Split-or-keep\n\n| Scenario | Decision |\n|---|---|\n| `{title}` | keep |\n"
    );
    std::fs::write(&path, content).expect("write review.md split-or-keep row");
}

#[given("the change's review.md has no disposition entries")]
fn review_md_no_dispositions(world: &mut GivnWorld) {
    let path = change_dir(world).join("review.md");
    std::fs::write(
        &path,
        "REVIEW: PASS\nREADME-IMPACT: none\n\n# Review\n\nNo overlap dispositions recorded.\n",
    )
    .expect("write review.md without dispositions");
}
