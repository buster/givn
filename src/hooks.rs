use crate::config::manifest::{HookEntry, HookFull, Manifest};
/// Hook execution engine (ADR-028, arc42 ch.8.12).
///
/// Resolves and executes artifact `post` hooks:
/// - Named built-ins: `verify` (run `verify.command`), `lint` (run `givn lint`).
/// - Shell command strings.
/// - Object form with `allow_failure` / `name`.
///
/// Hooks run with `GIVN_FEATURES` set to the appropriate features path.
/// Failure blocks the gate unless `allow_failure: true`.
use anyhow::Result;
use std::path::Path;

/// Run all post hooks for a given artifact.
///
/// The `verify` built-in is resolved to `manifest.verify.command` — never to the
/// sentinel. This is the single correct implementation; use it everywhere.
///
/// Returns Ok(()) if all required hooks pass.
/// Returns Err if any required hook fails (allow_failure hooks only warn).
pub fn run_artifact_hooks(
    artifact_id: &str,
    manifest: &Manifest,
    features_path: &str,
) -> Result<()> {
    let artifact = manifest
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .ok_or_else(|| anyhow::anyhow!("artifact '{}' not found in manifest", artifact_id))?;

    if artifact.post.is_empty() {
        return Ok(());
    }

    println!("Running hooks for artifact '{artifact_id}'...");
    let mut failed = false;

    for hook in &artifact.post {
        let (run, allow_failure, name) = normalise_hook(hook);
        let display = name.as_deref().unwrap_or(&run);
        println!("  hook: {display}");

        let result = match run.as_str() {
            "verify" => run_verify_hook(manifest, features_path),
            "verify-e2e" => run_verify_e2e_hook(manifest, features_path),
            "lint" => run_shell_command(
                &format!(
                    "{} lint",
                    std::env::current_exe().unwrap_or_default().display()
                ),
                features_path,
            ),
            "integrity" => crate::integrity_hook::run_integrity_hook(
                features_path,
                manifest.addons.arc42,
                manifest.addons.coverage,
                manifest.addons.readme,
            ),
            other => run_shell_command(other, features_path),
        };

        match result {
            Ok(()) => {
                println!("  ✓ {display}");
            }
            Err(e) if allow_failure => {
                eprintln!("  ⚠ {display} (failed, allow_failure=true): {e}");
            }
            Err(e) => {
                eprintln!("  ✗ {display}: {e}");
                failed = true;
            }
        }
    }

    if failed {
        anyhow::bail!("one or more required hooks failed for artifact '{artifact_id}'");
    }
    Ok(())
}

/// Run hooks for all `apply.requires` artifacts as the archive gate.
pub fn run_archive_gate_hooks(manifest: &Manifest, features_path: &str) -> Result<()> {
    for req in &manifest.apply.requires {
        run_artifact_hooks(req, manifest, features_path)?;
    }
    Ok(())
}

/// Normalise a HookEntry into (command_or_builtin, allow_failure, display_name).
///
/// Built-in hook names (`verify`, `verify-e2e`, `lint`, `integrity`) always
/// ignore `allow_failure` — the user may customise the hook list but may not
/// degrade these gates.
fn normalise_hook(entry: &HookEntry) -> (String, bool, Option<String>) {
    match entry {
        HookEntry::Short(s) => (s.clone(), false, None),
        HookEntry::Full(HookFull {
            run,
            allow_failure,
            name,
        }) => {
            let is_builtin = matches!(run.as_str(), "verify" | "verify-e2e" | "lint" | "integrity");
            (
                run.clone(),
                if is_builtin { false } else { *allow_failure },
                name.clone(),
            )
        }
    }
}

/// Resolve the `verify` built-in to `manifest.verify.command` and run it.
pub(crate) fn run_verify_hook(manifest: &Manifest, features_path: &str) -> Result<()> {
    let command = &manifest.verify.command;
    println!("  Running: {command}");
    run_shell_command(command, features_path)
}

/// Resolve the `verify-e2e` built-in to `manifest.verify.e2e_command` and run it.
///
/// Skips silently ONLY when:
///   - the in-scope `.feature` files genuinely contain no `@e2e`-tagged scenario, AND
///   - there is no evidence in `tasks.md` that `@e2e` scenarios were planned for this change.
///
/// If @e2e scenarios exist but `verify.e2e_command` is empty/unconfigured, this is a hard
/// failure: a project with browser/CLI/API smoke-test scenarios that has no configured e2e
/// runner has an unenforced testing gap, not an optional one.
///
/// If @e2e scenarios do not exist but `tasks.md` references e2e (section heading, task
/// items with @e2e scenario tags), this is also a hard failure: @e2e tags were planned
/// then removed to bypass the gate.
pub(crate) fn run_verify_e2e_hook(manifest: &Manifest, features_path: &str) -> Result<()> {
    let command = &manifest.verify.e2e_command;
    let features_path = Path::new(features_path);
    let has_e2e = crate::gherkin::parse::has_e2e_scenarios(features_path);

    if command == "givn missing-e2e-runner" {
        if has_e2e {
            anyhow::bail!(
                "'{}' contains @e2e-tagged scenario(s) but verify.e2e_command is not \
                 configured in givn/config.yaml. An @e2e scenario without a configured \
                 e2e runner is an unenforced gap, not an optional one — set verify.e2e_command.",
                features_path.display()
            );
        }

        // Cross-reference tasks.md: if tasks.md references @e2e scenarios but the delta
        // specs have none, the @e2e tags were removed — a gate bypass attempt.
        if let Some(change_dir) = crate::state::find_change_dir(&features_path.to_string_lossy()) {
            let tasks_path = change_dir.join("tasks.md");
            if tasks_path.exists() {
                let tasks_content = std::fs::read_to_string(&tasks_path).unwrap_or_default();
                if has_e2e_intent_in_tasks(&tasks_content) {
                    anyhow::bail!(
                        "'{}' references @e2e scenarios but no @e2e-tagged scenario exists \
                         under '{}'. @e2e coverage was planned and then removed — restore \
                         the @e2e tags and configure verify.e2e_command in givn/config.yaml. \
                         Removing @e2e tags to bypass the verify-e2e gate is a procedure \
                         violation.",
                        tasks_path.display(),
                        features_path.display()
                    );
                }
            }
        }

        println!("  (skipped — no @e2e scenarios in scope, no verify.e2e_command configured)");
        return Ok(());
    }
    println!("  Running: {command}");
    run_shell_command(command, &features_path.to_string_lossy())
}

/// Check whether `tasks.md` content indicates that `@e2e` scenarios were planned.
///
/// Matches section headings and task-item patterns produced by the tasks.md template:
///   - `E2E Smoke Tests` (section heading)
///   - `@e2e Scenario:` (task item in checkbox list)
///   - `## E2e` or `## E2E` (any E2E section heading)
pub(crate) fn has_e2e_intent_in_tasks(content: &str) -> bool {
    let lower = content.to_lowercase();
    lower.contains("e2e smoke tests")
        || lower.contains("@e2e scenario:")
        || lower.contains("## @e2e")
        || lower.contains("## e2e")
}

/// Execute a shell command with GIVN_FEATURES set.
fn run_shell_command(command: &str, features_path: &str) -> Result<()> {
    let status = crate::subprocess::sh_streamed(
        command,
        &[("GIVN_FEATURES", features_path)],
        None,
        "gate hook",
    )?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "command '{}' exited with status: {}",
            command,
            status.code().unwrap_or(-1)
        )
    }
}

#[cfg(test)]
mod verify_e2e_hook_tests {
    use super::*;
    use crate::config::manifest::{Addons, Apply, Coverage, Manifest, Run, Verify};
    use std::fs;

    fn manifest_with(e2e_command: &str) -> Manifest {
        Manifest {
            artifacts: vec![],
            apply: Apply {
                requires: vec![],
                tracks: None,
            },
            verify: Verify {
                command: "true".to_string(),
                e2e_command: e2e_command.to_string(),
                features_path: "givn/specs".to_string(),
            },
            run: Run::default(),
            addons: Addons::default(),
            coverage: Coverage::default(),
            givn_config_version: None,
        }
    }

    #[test]
    fn hard_fails_when_e2e_scenarios_exist_but_command_unconfigured() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(
            dir.path().join("thing.feature"),
            "Feature: thing\n\n  @e2e\n  Scenario: real interaction\n    Given a browser\n",
        )
        .unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, dir.path().to_str().unwrap());

        assert!(
            result.is_err(),
            "an @e2e scenario with no verify.e2e_command must hard fail, not skip silently"
        );
    }

    #[test]
    fn skips_silently_when_no_e2e_scenarios_and_command_unconfigured() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(
            dir.path().join("thing.feature"),
            "Feature: thing\n\n  Scenario: plain\n    Given a thing\n",
        )
        .unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, dir.path().to_str().unwrap());

        assert!(
            result.is_ok(),
            "no @e2e scenarios in scope must skip silently, not fail"
        );
    }

    #[test]
    fn runs_configured_command_when_e2e_scenarios_exist() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(
            dir.path().join("thing.feature"),
            "Feature: thing\n\n  @e2e\n  Scenario: real interaction\n    Given a browser\n",
        )
        .unwrap();

        let manifest = manifest_with("true");
        let result = run_verify_e2e_hook(&manifest, dir.path().to_str().unwrap());

        assert!(result.is_ok(), "a passing configured command must succeed");
    }

    #[test]
    fn fails_when_tasks_md_has_e2e_intent_but_delta_has_no_e2e() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Create path structure: tempdir/givn/changes/test/specs/
        let change_dir = dir.path().join("givn/changes/test");
        let specs_dir = change_dir.join("specs");
        fs::create_dir_all(&specs_dir).unwrap();

        // Feature file with NO @e2e tag.
        fs::write(
            specs_dir.join("thing.feature"),
            "Feature: thing\n\n  Scenario: plain\n    Given a thing\n",
        )
        .unwrap();

        // Tasks.md with e2e section heading (from template).
        fs::write(
            change_dir.join("tasks.md"),
            "## E2E Smoke Tests\n\n- [ ] @e2e Scenario: browser test\n",
        )
        .unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, specs_dir.to_str().unwrap());

        assert!(
            result.is_err(),
            "tasks.md with e2e intent but no @e2e scenarios must hard fail"
        );
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("procedure violation"),
            "error must mention procedure violation: {err}"
        );
    }

    #[test]
    fn fails_when_tasks_md_has_e2e_task_items_but_delta_has_no_e2e() {
        let dir = tempfile::tempdir().expect("tempdir");
        let change_dir = dir.path().join("givn/changes/test");
        let specs_dir = change_dir.join("specs");
        fs::create_dir_all(&specs_dir).unwrap();

        fs::write(
            specs_dir.join("thing.feature"),
            "Feature: thing\n\n  Scenario: plain\n    Given a thing\n",
        )
        .unwrap();

        // Tasks.md with @e2e Scenario: task items but no section heading.
        fs::write(
            change_dir.join("tasks.md"),
            "- [ ] @e2e Scenario: browser test\n",
        )
        .unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, specs_dir.to_str().unwrap());

        assert!(
            result.is_err(),
            "tasks.md with @e2e Scenario: items but no @e2e scenarios must hard fail"
        );
    }

    #[test]
    fn skips_when_tasks_md_has_no_e2e_intent_and_delta_has_no_e2e() {
        let dir = tempfile::tempdir().expect("tempdir");
        let change_dir = dir.path().join("givn/changes/test");
        let specs_dir = change_dir.join("specs");
        fs::create_dir_all(&specs_dir).unwrap();

        fs::write(
            specs_dir.join("thing.feature"),
            "Feature: thing\n\n  Scenario: plain\n    Given a thing\n",
        )
        .unwrap();

        // Tasks.md without any e2e references.
        fs::write(
            change_dir.join("tasks.md"),
            "- [ ] Set up step definitions\n- [ ] Scenario: plain\n",
        )
        .unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, specs_dir.to_str().unwrap());

        assert!(
            result.is_ok(),
            "no e2e intent in tasks.md must skip silently"
        );
    }

    #[test]
    fn skips_when_change_dir_not_detectable_has_no_e2e() {
        // Plain temp path without "givn/changes/" structure should skip tasks.md check.
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(
            dir.path().join("thing.feature"),
            "Feature: thing\n\n  Scenario: plain\n    Given a thing\n",
        )
        .unwrap();

        // Write a tasks.md here (plain tempdir won't be detected as a change dir).
        fs::write(dir.path().join("tasks.md"), "## E2E Smoke Tests\n").unwrap();

        let manifest = manifest_with("givn missing-e2e-runner");
        let result = run_verify_e2e_hook(&manifest, dir.path().to_str().unwrap());

        // Should skip because dir is not inside "givn/changes/".
        assert!(
            result.is_ok(),
            "a non-change path should skip the tasks.md cross-reference, not fail"
        );
    }

    #[test]
    fn has_e2e_intent_in_tasks_detects_patterns() {
        assert!(
            super::has_e2e_intent_in_tasks("## E2E Smoke Tests"),
            "must detect E2E Smoke Tests section heading"
        );
        assert!(
            super::has_e2e_intent_in_tasks("- [ ] @e2e Scenario: x"),
            "must detect @e2e Scenario: task item"
        );
        assert!(
            super::has_e2e_intent_in_tasks("## E2E"),
            "must detect E2E section heading"
        );
        assert!(
            super::has_e2e_intent_in_tasks("## @e2e browser smoke"),
            "must detect @e2e section heading"
        );
        assert!(
            !super::has_e2e_intent_in_tasks("Some other content without e2e"),
            "must not detect plain content"
        );
        assert!(
            !super::has_e2e_intent_in_tasks("- [ ] Set up step definitions"),
            "must not detect setup tasks"
        );
    }

    #[test]
    fn propagates_failure_of_configured_command() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(
            dir.path().join("thing.feature"),
            "Feature: thing\n\n  @e2e\n  Scenario: real interaction\n    Given a browser\n",
        )
        .unwrap();

        let manifest = manifest_with("false");
        let result = run_verify_e2e_hook(&manifest, dir.path().to_str().unwrap());

        assert!(result.is_err(), "a failing configured command must fail");
    }
}
