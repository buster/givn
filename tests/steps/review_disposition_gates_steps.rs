use cucumber::{given, then, when};

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
        "README-IMPACT: none\n\n## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `{title_a}` | `{title_b}` | boundary |\n\nREVIEW: PASS\n"
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
        "README-IMPACT: none\n\n## Split-or-keep\n\n| Scenario | Decision | Reason |\n|---|---|---|\n| `{title}` | keep | one atomic import requires all setup rows |\n\nREVIEW: PASS\n"
    );
    std::fs::write(&path, content).expect("write review.md split-or-keep row");
}

#[given("the change's review.md has no disposition entries")]
fn review_md_no_dispositions(world: &mut GivnWorld) {
    let path = change_dir(world).join("review.md");
    std::fs::write(
        &path,
        "README-IMPACT: none\n\n# Review\n\nNo overlap dispositions recorded.\n\nREVIEW: PASS\n",
    )
    .expect("write review.md without dispositions");
}

// --- 0.9.10 review-gate fixtures -------------------------------------------

fn review_env(world: &GivnWorld) -> &crate::common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn review_root(world: &GivnWorld) -> std::path::PathBuf {
    review_env(world).root()
}

fn set_review_text(world: &GivnWorld, review: &str) {
    std::fs::write(change_dir(world).join("review.md"), review).expect("write review.md");
}

fn set_delta_feature(world: &GivnWorld, body: &str) {
    let path = change_dir(world).join("specs/demo-cap/demo-cap.feature");
    std::fs::create_dir_all(path.parent().expect("delta parent")).expect("create delta dir");
    std::fs::write(path, body).expect("write delta feature");
}

fn commit_and_refresh_receipt(world: &mut GivnWorld) {
    let id = world
        .last_change_id
        .clone()
        .expect("a review change id is required");
    crate::common::commit_fixture(review_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
}

fn review_ready(world: &mut GivnWorld) {
    if world.env.is_some() && world.last_change_id.as_deref() == Some("test-change") {
        return;
    }
    crate::steps::archive_proof_support::ready_change(world, "test-change", false, "pass");
    std::fs::write(
        review_root(world).join("givn/specs/demo-cap/demo-cap.feature"),
        "Feature: Demo Capability\n\n  Scenario: Existing permanent scenario\n    Given an existing thing\n    Then it works\n",
    )
    .expect("write permanent fixture feature");
    std::fs::write(
        review_root(world).join("givn/changes/test-change/proposal.md"),
        "# Proposal: fixture\n\n## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | demo-cap | EXTEND demo-cap | `EXTEND demo-cap` | fixture |\n",
    )
    .expect("write review fixture proposal");
    commit_and_refresh_receipt(world);
}

fn default_review() -> String {
    "README-IMPACT: none\n\nREVIEW: PASS\n".to_string()
}

fn short_delta(first: &str, second: Option<&str>) -> String {
    let second = second
        .map(|title| {
            format!(
                "\n  @givn.added\n  Scenario: {title}\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n"
            )
        })
        .unwrap_or_default();
    format!(
        "@givn.delta @demo-cap\nFeature: Demo Capability\n\n  @givn.added\n  Scenario: {first}\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n{second}"
    )
}

fn long_scenario(title: &str) -> String {
    let mut steps = String::new();
    for index in 1..=15 {
        steps.push_str(&format!("    And step {index} runs\n"));
    }
    format!(
        "@givn.delta @demo-cap\nFeature: Demo Capability\n\n  @givn.added\n  Scenario: {title}\n    Given a prepared workspace\n{steps}    When the action runs\n    Then the command exits 0\n"
    )
}

fn split_or_keep_review(title: &str, row: &str) -> String {
    format!(
        "README-IMPACT: none\n\n## Split-or-keep\n\n| Scenario | Decision | Reason |\n|---|---|---|\n| {title} | {row} |\n\nREVIEW: PASS\n"
    )
}

#[given("a change has two short added scenarios with matching shapes and distinct titles")]
fn two_short_added_scenarios(world: &mut GivnWorld) {
    review_ready(world);
    set_delta_feature(
        world,
        &short_delta("Short delta shape one", Some("Short delta shape two")),
    );
    set_review_text(world, &default_review());
    commit_and_refresh_receipt(world);
}

#[given("the completed review has no shape disposition table")]
fn review_has_no_shape_table(world: &mut GivnWorld) {
    let review = std::fs::read_to_string(change_dir(world).join("review.md")).unwrap_or_default();
    assert!(!review.contains("Overlap dispositions"));
}

#[given("applicable verification and integrity evidence passes")]
fn applicable_evidence_passes(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let receipt = change_dir(world).join("verification/tasks-verification.json");
    assert!(
        receipt.exists(),
        "fixture has no change verification receipt"
    );
    let _ = id;
}

#[given(
    regex = r#"^an otherwise valid completed change has a short delta scenario shape-matching (another delta scenario|a permanent scenario)$"#
)]
fn short_delta_shape_match(world: &mut GivnWorld, other: String) {
    review_ready(world);
    set_delta_feature(world, &short_delta("Short delta shape", None));
    if other == "a permanent scenario" {
        let permanent = review_root(world).join("givn/specs/demo-cap/demo-cap.feature");
        std::fs::write(
            permanent,
            "Feature: Demo Capability\n\n  Scenario: Permanent shape\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n",
        )
        .expect("write permanent shape scenario");
    }
    set_review_text(world, &default_review());
    commit_and_refresh_receipt(world);
}

#[given("its review has no shape disposition entries")]
fn review_no_shape_entries(world: &mut GivnWorld) {
    let review = std::fs::read_to_string(change_dir(world).join("review.md")).unwrap_or_default();
    assert!(!review.contains("Overlap dispositions"));
    assert!(!review.contains("| Scenario A |"));
}

#[given(
    "a change has current applicable verification evidence and passes all other applicable integrity checks"
)]
fn current_evidence_and_integrity(world: &mut GivnWorld) {
    review_ready(world);
    set_delta_feature(world, &short_delta("Complete delta scenario", None));
    set_review_text(world, &default_review());
    commit_and_refresh_receipt(world);
}

#[given("the change has no review.md and therefore no required review verdict evidence")]
fn no_review_file(world: &mut GivnWorld) {
    let path = change_dir(world).join("review.md");
    if path.exists() {
        std::fs::remove_file(path).expect("remove review.md");
    }
}

#[given("a change contains a scenario exceeding the scenario-length limit")]
fn long_scenario_fixture(world: &mut GivnWorld) {
    review_ready(world);
    set_delta_feature(world, &long_scenario("The long scenario"));
    set_review_text(world, &default_review());
    commit_and_refresh_receipt(world);
}

#[given(
    regex = r#"^its review explicitly records "keep" for that scenario with the brief reason "([^"]+)"$"#
)]
fn review_records_keep(world: &mut GivnWorld, reason: String) {
    set_review_text(
        world,
        &split_or_keep_review("The long scenario", &format!("keep | {reason}")),
    );
    commit_and_refresh_receipt(world);
}

#[given("all other applicable review evidence passes")]
fn other_review_evidence_passes(world: &mut GivnWorld) {
    let review = std::fs::read_to_string(change_dir(world).join("review.md")).unwrap_or_default();
    assert!(review.contains("REVIEW: PASS"));
    commit_and_refresh_receipt(world);
}

#[given(regex = r#"^its review contains (.+) for that scenario$"#)]
fn review_contains_entry(world: &mut GivnWorld, entry: String) {
    let row = match entry.as_str() {
        "a scenario name without a split or keep decision" => "The long scenario | | ".to_string(),
        "a keep decision without a brief reason" => "The long scenario | keep |".to_string(),
        other => panic!("unknown split-or-keep entry {other}"),
    };
    set_review_text(
        world,
        &format!(
            "README-IMPACT: none\n\n## Split-or-keep\n\n| Scenario | Decision | Reason |\n|---|---|---|\n| {row}\n\nREVIEW: PASS\n"
        ),
    );
    commit_and_refresh_receipt(world);
}

#[given("its review records a split decision for that scenario")]
fn review_records_split(world: &mut GivnWorld) {
    set_review_text(world, &split_or_keep_review("The long scenario", "split"));
    commit_and_refresh_receipt(world);
}

#[given("the chosen split is not completed in the delta specs")]
fn split_not_completed(world: &mut GivnWorld) {
    let feature = change_dir(world).join("specs/demo-cap/demo-cap.feature");
    let content = std::fs::read_to_string(feature).expect("read delta feature");
    assert!(content.contains("The long scenario"));
}

#[given("a review records a split decision for a long scenario")]
fn review_records_split_long(world: &mut GivnWorld) {
    review_ready(world);
    set_delta_feature(world, &long_scenario("The long scenario"));
    set_review_text(world, &split_or_keep_review("The long scenario", "split"));
    commit_and_refresh_receipt(world);
}

#[given(
    "the delta specs complete that split into independently focused scenarios within the length limit"
)]
fn split_completed(world: &mut GivnWorld) {
    set_delta_feature(
        world,
        &short_delta("Focused split one", Some("Focused split two")),
    );
    commit_and_refresh_receipt(world);
}

#[then("the command succeeds without requiring a shape disposition")]
fn succeeds_without_shape_disposition(world: &mut GivnWorld) {
    assert_eq!(
        super::output(world).status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&super::output(world).stderr)
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&super::output(world).stdout),
        String::from_utf8_lossy(&super::output(world).stderr)
    );
    assert!(!text.contains("Overlap dispositions missing"), "{text}");
}

#[then("terminal output reports a net delta of 2 added scenarios")]
fn net_delta_two_added(world: &mut GivnWorld) {
    let stdout = String::from_utf8_lossy(&super::output(world).stdout);
    assert!(stdout.contains("net delta: 2 added"), "{stdout}");
}

#[then(
    "the command exits non-zero and names review.md and the missing required review verdict evidence"
)]
fn missing_review_message(world: &mut GivnWorld) {
    let out = super::output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("review.md"), "{stderr}");
    assert!(
        stderr.contains("missing required review verdict evidence"),
        "{stderr}"
    );
}

#[then("no shape disposition is required")]
fn no_shape_disposition_required(world: &mut GivnWorld) {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&super::output(world).stdout),
        String::from_utf8_lossy(&super::output(world).stderr)
    );
    assert!(!text.contains("Overlap disposition"), "{text}");
}

#[then("the command exits non-zero and explains the missing split-or-keep evidence")]
fn missing_split_or_keep(world: &mut GivnWorld) {
    let out = super::output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("split-or-keep"), "{stderr}");
}

#[then("the command exits non-zero and names the incomplete split")]
fn incomplete_split(world: &mut GivnWorld) {
    let out = super::output(world);
    assert_ne!(out.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("incomplete split"), "{stderr}");
    assert!(stderr.contains("The long scenario"), "{stderr}");
}

#[when("the consumer invokes `givn check review --change test-change`")]
fn consumer_invokes_review(world: &mut GivnWorld) {
    let env = review_env(world);
    world.last_output = Some(crate::common::run_givn(
        env,
        &["check", "review", "--change", "test-change"],
    ));
}

#[then("the command succeeds without requiring a keep reason for the completed split")]
fn succeeds_without_keep_reason(world: &mut GivnWorld) {
    assert_eq!(
        super::output(world).status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&super::output(world).stderr)
    );
}
