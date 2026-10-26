//! E2E step definitions for the archive-proof capability.

use cucumber::{given, then};

use super::archive_proof_support as support;
use crate::common;
use crate::steps::GivnWorld;

#[given(regex = r#"^a committed givn project with a completed change "([^"]+)" ready to archive$"#)]
fn committed_ready_change(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "pass");
}

#[given(
    regex = r#"^a committed (Git|Fossil) project with a completed change "([^"]+)" ready to archive$"#
)]
fn committed_backend_ready_change(world: &mut GivnWorld, backend: String, id: String) {
    support::ready_change_on(world, support::backend_kind(&backend), &id, false, "pass");
}

#[given("the project reports scenario counts for every verification scope")]
fn project_reports_counts(world: &mut GivnWorld) {
    support::ensure_env(world);
    let env = support::env(world);
    support::write_result_runner(env, "pass");
    common::commit_fixture(env);
    if let Some(id) = world.last_change_id.clone() {
        support::refresh_change_receipt(world, &id);
    }
}

#[given(regex = r#"^a committed givn project with an unproven archive candidate "([^"]+)"$"#)]
fn committed_unproven_candidate(world: &mut GivnWorld, id: String) {
    support::ready_change(world, &id, false, "fail");
    let out = support::run_archive(world, &id);
    assert!(
        !out.status.success(),
        "fixture archive unexpectedly succeeded: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    world.last_change_id = Some(id);
}

#[then(regex = r"^the archive at givn/archive/([^/]+)/ carries a proven verification receipt$")]
fn carries_proven_receipt(world: &mut GivnWorld, id: String) {
    let receipt = support::receipt(world, &id);
    assert_eq!(receipt["status"], "proven");
    assert!(
        receipt["tested_revision"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "proven receipt does not name the tested commit: {receipt}"
    );
}

fn invocation_log(world: &GivnWorld) -> String {
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    std::fs::read_to_string(
        support::root(world).join(format!("givn/archive/{id}/results/invocations.log")),
    )
    .unwrap_or_default()
}

#[given(
    "the developer commits only proposal, design, design-review, tasks, review, arc42, use-case narrative, README, and evidence edits after that revision"
)]
fn commit_archive_documentation_edits(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("a change id");
    let root = support::root(world);
    let change = root.join(format!("givn/changes/{id}"));
    for name in [
        "proposal.md",
        "design.md",
        "design-review.md",
        "review.md",
        "arc42.md",
    ] {
        let path = change.join(name);
        let mut content = std::fs::read_to_string(&path).unwrap_or_default();
        let marker = match name {
            "review.md" => Some("REVIEW: PASS"),
            "design-review.md" => Some("DESIGN-REVIEW: PASS"),
            "arc42.md" => Some("STATUS: DONE"),
            _ => None,
        };
        if let Some(marker) = marker {
            content = content.replacen(
                marker,
                &format!("Documentation-only archive edit.\n\n{marker}"),
                1,
            );
        } else {
            content.push_str("\nDocumentation-only archive edit.\n");
        }
        std::fs::write(path, content).unwrap();
    }
    let head = support::current_revision(world);
    std::fs::write(
        change.join("tasks.md"),
        format!("- [x] COMMIT: {head} -- Documentation-only archive edit\n"),
    )
    .unwrap();
    let usecase = change.join("specs").join(&id).join("usecase.md");
    let content = std::fs::read_to_string(&usecase).unwrap();
    std::fs::write(
        usecase,
        content.replace(
            "- Structural failures outrank other findings.",
            "- Structural failures outrank other findings.\n- Documentation-only archive edit.",
        ),
    )
    .unwrap();
    let readme = root.join("README.md");
    let mut content = std::fs::read_to_string(&readme).unwrap();
    content.push_str("\nDocumentation-only archive edit.\n");
    std::fs::write(readme, content).unwrap();
    let evidence = change.join("evidence/visual/fixture/transcript.txt");
    std::fs::create_dir_all(evidence.parent().unwrap()).unwrap();
    std::fs::write(evidence, "fixture evidence\n").unwrap();
    common::commit_fixture(support::env(world));
    world.skip_receipt_autowrite = true;
    world.verification_log_before = Some(invocation_log(world));
}

#[then("the merged regular scope ran once with features path \"givn/specs\"")]
fn merged_regular_scope_ran_once(world: &mut GivnWorld) {
    let before = world
        .verification_log_before
        .clone()
        .expect("the fixture must snapshot the invocation log");
    let after = invocation_log(world);
    let count = |text: &str| {
        text.lines()
            .filter(|line| {
                line.starts_with("regular givn/specs") || line.contains(" regular givn/specs")
            })
            .count()
    };
    assert_eq!(
        count(&after) - count(&before),
        1,
        "merged regular scope did not run exactly once: before={before} after={after}"
    );
}

#[then("the archive terminal transcript is captured for this scenario")]
fn capture_archive_transcript(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("the scenario must run the real CLI first");
    let title =
        "Consumer archives a change after documentation-only review edits and the proof is reused";
    let slug = givn::integrity_hook::scenario_slug(title);
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let active = root.join("givn/changes/review-friction-hardening");
    let dir = if active.exists() {
        active.join("evidence/visual").join(&slug)
    } else {
        root.join("target/review-friction-hardening-evidence")
            .join(&slug)
    };
    std::fs::create_dir_all(&dir).expect("create transcript directory");
    let cwd = support::root(world).display().to_string();
    let text = format!(
        "Scenario: {title}\nargv: givn archive --change review-proof-demo\ncwd (fixture project root): {cwd}\nexit code: {}\n--- stdout ---\n{}\n--- stderr ---\n{}\n",
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    std::fs::write(dir.join("transcript.txt"), text).expect("write transcript");
}
