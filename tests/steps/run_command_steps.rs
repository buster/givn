//! Unit-level step definitions for the run-command declaration capability.
//!
//! These steps work directly on the manifest/overlay layer
//! (`givn::config`) and the validation/render/backfill functions — no
//! subprocess is started.

use cucumber::{given, then, when};
use gherkin::Step;

use super::GivnWorld;

// --- Given: declarations on disk ---------------------------------------------

#[given(regex = r#"^a commands file with a run declaration:$"#)]
fn commands_file_with_run_declaration(world: &mut GivnWorld, step: &Step) {
    let env = world.env.get_or_insert_with(crate::common::make_bare_root);
    let content = step
        .docstring
        .as_ref()
        .expect("docstring content for run declaration");
    let path = env.path("givn/commands.yaml");
    std::fs::create_dir_all(path.parent().expect("givn dir")).expect("create givn dir");
    std::fs::write(&path, content).expect("write commands.yaml");
}

// --- When: load ---------------------------------------------------------------

#[when("the commands file is loaded")]
fn commands_file_is_loaded(world: &mut GivnWorld) {
    let env = world.env.get_or_insert_with(crate::common::make_bare_root);
    let base = givn::config::manifest::Manifest::load_embedded().expect("load default manifest");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    match givn::config::overlay::apply(base, &content) {
        Ok(m) => {
            world.overlay_manifest = Some(m);
            world.overlay_error = None;
        }
        Err(e) => {
            world.overlay_error = Some(e.to_string());
        }
    }
}

// --- Then ---------------------------------------------------------------------

#[then(regex = r#"^loading fails mentioning "(.*)"$"#)]
fn loading_fails_mentioning(world: &mut GivnWorld, needle: String) {
    let err = world
        .overlay_error
        .as_ref()
        .expect("expected loading to fail");
    assert!(
        err.contains(&needle),
        "load error {err:?} does not mention {needle:?}"
    );
}

// --- Given: declaration under validation -------------------------------------

#[given(regex = r#"^a run declaration:$"#)]
fn a_run_declaration(world: &mut GivnWorld, step: &Step) {
    let yaml = step
        .docstring
        .as_ref()
        .expect("docstring content for run declaration")
        .clone();
    let base = givn::config::manifest::Manifest::load_embedded().expect("load default manifest");
    world.overlay_manifest =
        Some(givn::config::overlay::apply(base, &yaml).expect("parse run declaration"));
    world.run_findings.clear();
}

// --- When: validate ------------------------------------------------------------

#[when("the run declaration is validated")]
fn the_run_declaration_is_validated(world: &mut GivnWorld) {
    let manifest = world
        .overlay_manifest
        .as_ref()
        .expect("a run declaration is required first");
    world.run_findings = givn::cli::run_block::check_run(manifest);
}

// --- Then: findings ------------------------------------------------------------

#[then(regex = r#"^the check reports "(.*)"$"#)]
fn the_check_reports(world: &mut GivnWorld, needle: String) {
    assert!(
        world
            .run_findings
            .iter()
            .any(|f| f.message.contains(&needle)),
        "no finding mentions {needle:?}: {:?}",
        world.run_findings,
    );
}

#[then("the check reports no findings")]
fn the_check_reports_no_findings(world: &mut GivnWorld) {
    assert!(
        world.run_findings.is_empty(),
        "expected no findings: {:?}",
        world.run_findings,
    );
}

#[then("the check reports no port or readiness findings")]
fn the_check_reports_no_port_or_readiness_findings(world: &mut GivnWorld) {
    use givn::cli::run_block::RunFindingKind;
    let offending: Vec<_> = world
        .run_findings
        .iter()
        .filter(|f| {
            matches!(
                f.kind,
                RunFindingKind::MissingField | RunFindingKind::ForbiddenField
            )
        })
        .collect();
    assert!(
        offending.is_empty(),
        "expected no port or readiness findings: {:?}",
        offending,
    );
}

// --- Change-status inspection fixtures (0.9.10) ----------------------------

fn status_env(world: &GivnWorld) -> &crate::common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn write_complete_change(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = crate::common::run_givn(env, &["new", id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let change = env.path(&format!("givn/changes/{id}"));
    std::fs::write(
        change.join("proposal.md"),
        crate::steps::fixture_routing_proposal("example"),
    )
    .expect("write proposal");
    std::fs::write(change.join("design.md"), "# Design\n").expect("write design");
    std::fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design-review");
    std::fs::write(change.join("tasks.md"), "- [x] done\n").expect("write tasks");
    std::fs::write(
        change.join("review.md"),
        "README-IMPACT: none\n\nREVIEW: PASS\n",
    )
    .expect("write review");
    std::fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").expect("write arc42");
    let specs = change.join("specs/example");
    std::fs::create_dir_all(&specs).expect("create specs");
    std::fs::write(specs.join("example.feature"), "Feature: example\n").expect("write feature");
}

fn write_archive_receipt(world: &GivnWorld, id: &str, status: &str, matching: bool) {
    let env = status_env(world);
    let archive = env.path(&format!("givn/archive/{id}"));
    std::fs::create_dir_all(&archive).expect("create archive dir");
    let (candidate, tested) = if matching {
        ("fixture-revision", "fixture-revision")
    } else {
        ("fixture-candidate", "older-revision")
    };
    let receipt = serde_json::json!({
        "schema": 1,
        "change": id,
        "status": status,
        "candidate_revision": candidate,
        "tested_revision": tested,
        "scopes": {"regular": {"status": "passed", "total": 2, "passed": 2, "failed": 0, "skipped": 0, "seconds": 0.0}},
        "merge_completed": true,
    });
    std::fs::write(archive.join("verification.json"), format!("{receipt}\n"))
        .expect("write archive receipt");
}

#[given(regex = r#"^a change "([^"]+)" has a proven archive with a matching archive receipt$"#)]
fn change_has_proven_archive(world: &mut GivnWorld, id: String) {
    write_archive_receipt(world, &id, "proven", true);
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a change "([^"]+)" exists in the archive directory$"#)]
fn change_exists_in_archive_directory(world: &mut GivnWorld, id: String) {
    let env = world.env.get_or_insert_with(crate::common::make_init_root);
    std::fs::create_dir_all(env.path(&format!("givn/archive/{id}"))).expect("create archive dir");
    world.last_change_id = Some(id);
}

#[given("the archive receipt is missing")]
fn archive_receipt_missing(_world: &mut GivnWorld) {}

#[given("the archive receipt is stale")]
fn archive_receipt_stale(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("an archive id");
    write_archive_receipt(world, &id, "proven", false);
}

#[given(
    regex = r#"^a project with complete active artifacts for change "([^"]+)" and a pending archive candidate "([^"]+)"$"#
)]
fn project_with_active_and_candidate(world: &mut GivnWorld, active: String, candidate: String) {
    world.env = Some(crate::common::make_init_root());
    write_complete_change(world, &active);
    change_exists_in_archive_directory(world, candidate.clone());
    world.last_change_id = Some(active);
}

#[given("verification commands record every invocation and fail if invoked")]
fn verification_commands_record_invocation(world: &mut GivnWorld) {
    let env = status_env(world);
    let marker = env.path("verification-invoked");
    let script = format!(
        "#!/bin/sh\nprintf 'invoked' > \"{}\"\nexit 9\n",
        marker.display()
    );
    let runner = env.path("forbidden-runner.sh");
    std::fs::write(&runner, script).expect("write forbidden runner");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&runner, std::fs::Permissions::from_mode(0o755))
            .expect("chmod forbidden runner");
    }
    let commands = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&commands).expect("read commands.yaml");
    let updated = crate::common::replace_top_level_key(
        &content,
        "verify",
        "verify:\n  command: \"./forbidden-runner.sh\"\n  e2e_command: \"./forbidden-runner.sh\"\n",
    );
    std::fs::write(commands, updated).expect("write commands.yaml");
}

fn snapshot_status_artifacts(world: &GivnWorld) -> String {
    let env = status_env(world);
    let mut entries = std::collections::BTreeMap::new();
    for root in ["givn/changes", "givn/archive"] {
        let base = env.path(root);
        let mut stack = vec![base];
        while let Some(dir) = stack.pop() {
            let Ok(entries_in_dir) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries_in_dir.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Ok(bytes) = std::fs::read(&path) {
                    entries.insert(
                        path.display().to_string(),
                        givn::proof::digest_text(&String::from_utf8_lossy(&bytes)),
                    );
                }
            }
        }
    }
    serde_json::to_string(&entries).expect("snapshot json")
}

#[given("the change artifacts and receipts are snapshotted")]
fn snapshot_change_artifacts(world: &mut GivnWorld) {
    world.additional_output = Some(snapshot_status_artifacts(world));
}

#[then("no verification command was invoked")]
fn no_verification_command_invoked(world: &mut GivnWorld) {
    let env = status_env(world);
    assert!(
        !env.path("verification-invoked").exists(),
        "status executed a configured verification command"
    );
}

#[then("the change artifacts and receipts are unchanged")]
fn change_artifacts_unchanged(world: &mut GivnWorld) {
    let before = world.additional_output.as_ref().expect("artifact snapshot");
    assert_eq!(before, &snapshot_status_artifacts(world));
}

fn status_text(world: &GivnWorld) -> String {
    let out = crate::steps::output(world);
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn status_json(world: &GivnWorld) -> serde_json::Value {
    serde_json::from_str(&status_text(world)).expect("status stdout is JSON")
}

#[then(regex = r#"^stdout reports verification as (current|stale|not checked)$"#)]
fn stdout_reports_verification(world: &mut GivnWorld, state: String) {
    let text = status_text(world);
    let observed = if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        value["verification"]
            .as_str()
            .unwrap_or("missing")
            .to_string()
    } else {
        text.lines()
            .find_map(|line| line.strip_prefix("Verification: "))
            .map(|value| value.trim().to_string())
            .unwrap_or_else(|| "missing".to_string())
    };
    assert_eq!(observed, state, "{text}");
}

#[then("the status reports required artifacts as complete")]
fn status_reports_artifacts_complete(world: &mut GivnWorld) {
    let text = status_text(world);
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        assert_eq!(value["all_required_done"], true, "{text}");
    } else {
        assert!(text.contains("All required artifacts complete."), "{text}");
    }
}

#[then("the status does not claim that artifact completion establishes archive readiness")]
fn status_does_not_claim_readiness(world: &mut GivnWorld) {
    let text = status_text(world);
    assert!(!text.contains("ready-to-archive"), "{text}");
    assert!(!text.contains("Ready to archive"), "{text}");
}

#[then("stdout reports a proven archive")]
fn stdout_reports_proven_archive(world: &mut GivnWorld) {
    let text = status_text(world);
    assert!(text.contains("proven archive"), "{text}");
}

#[then("the JSON status reports a proven archive")]
fn json_reports_proven_archive(world: &mut GivnWorld) {
    let value = status_json(world);
    assert_eq!(value["archive_status"], "proven", "{value}");
}

#[then("the status reports a pending archive candidate")]
fn status_reports_pending_candidate(world: &mut GivnWorld) {
    let text = status_text(world);
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        assert_eq!(value["archive_status"], "pending-candidate", "{text}");
    } else {
        assert!(text.contains("pending archive candidate"), "{text}");
    }
}

#[then("the status does not report a proven archive")]
fn status_does_not_report_proven(world: &mut GivnWorld) {
    let text = status_text(world);
    assert!(!text.contains("proven archive"), "{text}");
}

fn status_change_id(world: &GivnWorld) -> String {
    if let Some(id) = world.last_change_id.clone() {
        return id;
    }
    let env = status_env(world);
    let mut ids: Vec<String> = std::fs::read_dir(env.path("givn/changes"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .collect();
    ids.sort();
    assert_eq!(
        ids.len(),
        1,
        "exactly one active change is required: {ids:?}"
    );
    ids.remove(0)
}

#[given("a green receipt matches the current relevant inputs")]
fn green_receipt_matches_inputs(world: &mut GivnWorld) {
    let id = status_change_id(world);
    crate::common::commit_fixture(status_env(world));
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    world.last_change_id = Some(id);
}

#[given("tested executable inputs changed after a green receipt")]
fn tested_inputs_changed_after_receipt(world: &mut GivnWorld) {
    green_receipt_matches_inputs(world);
    let id = status_change_id(world);
    let env = status_env(world);
    let feature = env.path(&format!("givn/changes/{id}/specs/example/example.feature"));
    let mut content = std::fs::read_to_string(&feature).expect("read delta feature");
    content.push_str("\n  @givn.added\n  Scenario: Late scenario\n    Given x\n    Then y\n");
    std::fs::write(&feature, content).expect("write changed delta feature");
    crate::common::commit_fixture(env);
}

#[given("no verification receipt exists")]
fn no_verification_receipt_exists(world: &mut GivnWorld) {
    let id = status_change_id(world);
    let env = status_env(world);
    let path = env.path(&format!(
        "givn/changes/{id}/verification/tasks-verification.json"
    ));
    if path.exists() {
        std::fs::remove_file(path).expect("remove verification receipt");
    }
}

#[then("stdout reports that required artifacts are complete")]
fn stdout_reports_required_complete(world: &mut GivnWorld) {
    status_reports_artifacts_complete(world);
}

#[then("stdout does not claim \"ready-to-archive\" or \"Ready to archive\"")]
fn stdout_does_not_claim_ready(world: &mut GivnWorld) {
    let text = status_text(world);
    assert!(!text.contains("ready-to-archive"), "{text}");
    assert!(!text.contains("Ready to archive"), "{text}");
}

#[then(regex = r#"^the status reports verification as (current|stale|not checked)$"#)]
fn status_reports_verification(world: &mut GivnWorld, state: String) {
    stdout_reports_verification(world, state);
}
