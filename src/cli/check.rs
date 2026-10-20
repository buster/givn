use crate::config::overlay::load_effective;
use anyhow::{Result, bail};
use std::collections::HashSet;

/// Run an artifact's post hooks (e.g. verify.command, lint).
///
/// `givn check <artifact>` — runs that artifact's hooks.
/// `givn check` (no arg) — runs hooks for all artifacts with existing output.
pub fn run(artifact: Option<&str>, change: Option<&str>) -> Result<()> {
    let project_root = std::env::current_dir()?;
    if artifact == Some("migration") {
        let migration_id = change.ok_or_else(|| {
            anyhow::anyhow!("givn check migration requires --change <migration-id>")
        })?;
        return crate::spec::coverage::check_migration(&project_root, migration_id);
    }
    if artifact == Some("rules") {
        let report = crate::skills::rules::cleanliness(&project_root)?;
        crate::skills::rules::print_report("check rules", &report)?;
        if report.policy == Some(crate::config::overlay::RulesPolicy::Versioned) {
            println!("  check rules: the rules are versioned by policy; nothing to purge");
            return Ok(());
        }
        if !report.is_clean() {
            bail!(
                "rules check failed: tracked or reachable-history rules exist; clean them before publishing"
            );
        }
        return Ok(());
    }
    let manifest = load_effective()?;

    // Determine features_path: active change's features, or default specs.
    let features_path = if let Some(id) = change {
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        if crate::state::is_skipped("specs", &change_dir) {
            manifest.verify.features_path.clone()
        } else {
            change_dir.join("specs").to_string_lossy().into_owned()
        }
    } else {
        manifest.verify.features_path.clone()
    };

    if let Some(artifact_id) = artifact {
        // Validate against manifest before running checks.
        if !manifest.artifacts.iter().any(|a| a.id == artifact_id) {
            let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
            bail!(
                "unknown artifact '{}'. Known artifacts: {}",
                artifact_id,
                known.join(", ")
            );
        }
        // Run mandatory checks first, then custom hooks. The mandatory-check
        // boundary is one `phase` entry, parented to this command's entry.
        let phase = crate::history::current().map(|recorder| {
            recorder
                .span("mandatory checks")
                .kind("phase")
                .label(artifact_id)
        });
        let checks = run_mandatory_checks(
            artifact_id,
            &manifest,
            &features_path,
            &project_root,
            change,
        );
        if let Some(phase) = phase {
            phase.mark(if checks.is_ok() { "ok" } else { "failed" });
        }
        checks?;
        crate::hooks::run_artifact_hooks(artifact_id, &manifest, &features_path)
    } else {
        // Run hooks for all artifacts whose output exists.
        let change_dir = change
            .map(|id| crate::state::resolve_change_dir(&project_root, id))
            .transpose()?;

        let mut ran_any = false;
        for artifact in &manifest.artifacts {
            if artifact.post.is_empty() {
                continue;
            }

            let output_exists = match change_dir.as_ref() {
                Some(dir) => artifact_output_exists(artifact, dir),
                None => true, // global check: run all
            };

            if output_exists {
                ran_any = true;
                crate::hooks::run_artifact_hooks(&artifact.id, &manifest, &features_path)?;
            }
        }
        if !ran_any {
            println!("No artifact outputs found to check.");
        }
        Ok(())
    }
}

/// Run mandatory built-in checks for a given artifact.
///
/// These are not user-configurable post hooks — they are required gates
/// baked into the binary. For `tasks`: verify (test suite). For `review`:
/// integrity. Scenario execution is proven once per archive by the
/// verification receipt; review does not re-run the suite. All other
/// artifacts have no mandatory checks; their behavior comes from post hooks
/// only.
fn run_mandatory_checks(
    artifact_id: &str,
    manifest: &crate::config::manifest::Manifest,
    features_path: &str,
    project_root: &std::path::Path,
    change: Option<&str>,
) -> anyhow::Result<()> {
    match artifact_id {
        "tasks" => {
            println!("Mandatory checks for 'tasks':");
            // When running against delta specs (under givn/changes/<id>/specs/),
            // block if any non-removed scenario is still tagged @wip.
            if features_path.contains("givn/changes/") {
                let specs_path = std::path::Path::new(features_path);
                for path in crate::gherkin::parse::collect_feature_files(specs_path) {
                    let env = gherkin::GherkinEnv::default();
                    let feature = gherkin::Feature::parse_path(&path, env).map_err(|e| {
                        anyhow::anyhow!("failed to parse '{}': {}", path.display(), e)
                    })?;
                    if let Some(msg) = crate::gherkin::delta::find_wip_in_feature(&feature) {
                        anyhow::bail!(
                            "{} in '{}' — remove @wip before marking tasks complete",
                            msg,
                            path.display(),
                        );
                    }
                }
            }
            if let Some(id) = change {
                run_change_verification(manifest, project_root, id)?;
            } else {
                crate::hooks::run_verify_hook(manifest, features_path)?;
            }
            println!("  ✓ verify passed");
        }
        "review" => {
            println!("Mandatory checks for 'review':");
            crate::integrity_hook::run_integrity_hook(
                features_path,
                manifest.addons.arc42,
                manifest.addons.coverage,
                manifest.addons.readme,
            )?;
            println!("  ✓ integrity passed");
            let run_findings = crate::cli::run_block::check_run(manifest);
            if !run_findings.is_empty() {
                for f in &run_findings {
                    println!("  [{}] {}", crate::cli::run_block::label(f.kind), f.message);
                }
                anyhow::bail!(
                    "run declaration incomplete — configure `run:` in givn/commands.yaml"
                );
            }
            println!("  ✓ run declaration passed");
            if let Some(id) = change {
                crate::overlap::analysis::run_review_overlap_gate(project_root, id)?;
                check_retirement_disclosed(project_root, id)?;
                crate::spec::routing_table::check_routing_consistency(project_root, id)?;
                check_change_receipt(manifest, project_root, id)?;
                if manifest.addons.readability {
                    crate::readability::gate::enforce(project_root, id)?;
                }
            } else {
                println!(
                    "  ⚠ no --change given — overlap, retirement, routing, and receipt checks were skipped \
                     (they require a specific change to scope to)"
                );
            }
        }
        _ => {}
    }
    Ok(())
}

struct ScenarioCommit {
    title: String,
    checked: bool,
    hash: Option<String>,
}

fn scenario_title(line: &str) -> Option<(bool, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("- [") {
        return None;
    }
    let checked = trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]");
    let index = trimmed.find("Scenario:")?;
    let title = trimmed[index + "Scenario:".len()..]
        .trim()
        .trim_matches('*')
        .trim_matches('`')
        .trim();
    if title.is_empty() {
        return None;
    }
    Some((checked, title.to_string()))
}

fn commit_line(line: &str) -> Option<(bool, Option<String>)> {
    let trimmed = line.trim();
    if !trimmed.starts_with("- [") || !trimmed.contains("COMMIT:") {
        return None;
    }
    let checked = trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]");
    let hash = trimmed
        .split_once("COMMIT:")
        .map(|(_, rest)| rest)
        .unwrap_or("")
        .split("--")
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('`');
    Some((checked, (!hash.is_empty()).then(|| hash.to_string())))
}

fn parse_scenario_commits(content: &str) -> Vec<ScenarioCommit> {
    let mut scenarios: Vec<ScenarioCommit> = Vec::new();
    for line in content.lines() {
        if let Some((checked, title)) = scenario_title(line) {
            scenarios.push(ScenarioCommit {
                title,
                checked,
                hash: None,
            });
        } else if let Some((checked, hash)) = commit_line(line)
            && let Some(current) = scenarios.last_mut()
            && current.hash.is_none()
        {
            current.hash = hash;
            if checked {
                current.checked = true;
            }
        }
    }
    scenarios
}

fn normalize_commit_text(text: &str) -> String {
    text.replace(['`', '*'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn placeholder_hash(hash: &str) -> bool {
    let lower = hash.to_lowercase();
    lower == "___"
        || lower == "---"
        || lower == "xxx"
        || lower.chars().all(|c| c == 'x')
        || lower == "tbd"
        || lower == "todo"
        || lower.starts_with('<')
}

/// Block unless every checked scenario has one unique, real commit whose
/// message body contains the scenario title. A change with no checked
/// scenario has not started implementation; it is not blocked here.
fn check_scenario_commits(
    backend: &crate::scm::Backend,
    change_dir: &std::path::Path,
) -> Result<()> {
    let tasks = change_dir.join("tasks.md");
    let Ok(content) = std::fs::read_to_string(&tasks) else {
        return Ok(());
    };
    let scenarios = parse_scenario_commits(&content);
    if scenarios.is_empty() || !scenarios.iter().any(|scenario| scenario.checked) {
        return Ok(());
    }
    let mut errors = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for scenario in &scenarios {
        if !scenario.checked {
            errors.push(format!(
                "scenario '{}' is not checked; a scenario is done only when its commit is recorded",
                scenario.title
            ));
            continue;
        }
        let Some(hash) = scenario.hash.as_deref() else {
            errors.push(format!(
                "scenario '{}' has no `COMMIT:` line",
                scenario.title
            ));
            continue;
        };
        if placeholder_hash(hash) {
            errors.push(format!(
                "scenario '{}' uses a placeholder revision `{hash}`",
                scenario.title
            ));
            continue;
        }
        if !seen.insert(hash.to_string()) {
            errors.push(format!(
                "scenario '{}' reuses revision `{hash}`; each scenario needs its own commit",
                scenario.title
            ));
            continue;
        }
        if !backend.commit_exists(hash)? {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` which does not exist",
                scenario.title
            ));
            continue;
        }
        if !backend.is_ancestor(hash)? {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` which is not reachable from HEAD",
                scenario.title
            ));
            continue;
        }
        let Some(message) = backend.commit_message(hash)? else {
            errors.push(format!(
                "scenario '{}' names revision `{hash}` with no readable message",
                scenario.title
            ));
            continue;
        };
        if !normalize_commit_text(&message).contains(&normalize_commit_text(&scenario.title)) {
            errors.push(format!(
                "scenario '{}' commit `{hash}` does not name the scenario in its message",
                scenario.title
            ));
        }
    }
    if !errors.is_empty() {
        bail!("scenario commit check failed:\n  {}", errors.join("\n  "));
    }
    println!(
        "  ✓ scenario commits passed ({} scenario(s))",
        scenarios.len()
    );
    Ok(())
}

/// Report a missing README-impact decision before review. The review integrity
/// gate remains the blocking authority.
fn preflight_readme_impact(change_dir: &std::path::Path) {
    let review = change_dir.join("review.md");
    match std::fs::read_to_string(&review) {
        Ok(content) if content.contains("README-IMPACT:") => {}
        Ok(_) => println!(
            "  ⚠ task preflight: review.md has no README-IMPACT decision (none or updated - <sections>)"
        ),
        Err(_) => println!("  ⚠ task preflight: review.md is missing"),
    }
}

/// Run the change-scoped verification and persist its receipt.
/// The clean-tree requirement keeps the receipt bound to one revision; the
/// agent commits implementation and evidence before this command runs.
fn run_change_verification(
    manifest: &crate::config::manifest::Manifest,
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    preflight_readme_impact(&change_dir);
    let backend = crate::scm::select(project_root)?;
    check_scenario_commits(&backend, &change_dir)?;
    let offending: Vec<String> = backend
        .dirty_entries()?
        .iter()
        .filter(|path| !crate::proof::is_evidence_path(project_root, change_id, manifest, path))
        .map(|path| path.display().to_string())
        .collect();
    if !offending.is_empty() {
        anyhow::bail!(
            "change verification requires a clean committed tree: {}",
            offending.join(", ")
        );
    }
    let rules = project_root
        .join("givn/changes")
        .join(change_id)
        .join("specs");
    let features = rules.to_string_lossy().into_owned();
    let result_file = change_dir.join(crate::proof::CHANGE_RESULT_FILE);
    if let Some(parent) = result_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let result_path = result_file.to_string_lossy().into_owned();
    let status = crate::subprocess::sh_streamed(
        &manifest.verify.command,
        &[
            ("GIVN_FEATURES", &features),
            ("GIVN_RESULT_FILE", &result_path),
            ("GIVN_RESULT_SCOPE", "change"),
        ],
        Some(project_root),
        "change verification",
    )?;
    if !status.success() {
        anyhow::bail!(
            "change verification failed (exit code {}); repair the change and rerun \
             `givn check tasks --change {change_id}`",
            status.code().unwrap_or(-1)
        );
    }
    let result = crate::proof::RunnerResult::read_counts(
        std::path::Path::new(&result_path),
        "the change verification",
    )?;
    let head = backend.revision()?;
    crate::proof::write_change_receipt(
        project_root,
        &change_dir,
        change_id,
        &head,
        &manifest.verify.command,
        result.clone(),
        crate::proof::unix_now(),
    )?;
    println!(
        "  ✓ change verification receipt: {} scenario(s), 0 failed, 0 skipped at {}",
        result.total, head
    );
    Ok(())
}

/// Refuse review unless a current, green change-scoped receipt exists.
fn check_change_receipt(
    manifest: &crate::config::manifest::Manifest,
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let receipt = crate::proof::ChangeReceipt::read(&change_dir).map_err(|error| {
        anyhow::anyhow!("{error}. Run `givn check tasks --change {change_id}` before review.")
    })?;
    let backend = crate::scm::select(project_root)?;
    check_scenario_commits(&backend, &change_dir)?;
    let head = backend.revision()?;
    if !receipt.is_current(&change_dir, &head, &manifest.verify.command)? {
        let features = crate::proof::digest_features(&change_dir.join("specs"))?;
        anyhow::bail!(
            "change verification receipt is stale for '{}': receipt head {}, current head {head}; \
             receipt features {}, current features {features}; receipt command {} ({:?}), current command {} ({:?}). \
             Run `givn check tasks --change {change_id}` again.",
            change_id,
            receipt.head,
            receipt.features_digest,
            receipt.command_digest,
            receipt.command,
            crate::proof::digest_text(&manifest.verify.command),
            manifest.verify.command
        );
    }
    println!(
        "  ✓ change verification receipt passed ({} scenario(s) at {})",
        receipt.result.total, receipt.head
    );
    Ok(())
}

/// If this change's delta retires any capability (`@givn.retired`),
/// require review.md to contain a `RETIRE: <capability>` line for each
/// one. A plain substring check, not a table parser — deliberately
/// simple so there is no heading-name mismatch to get wrong (see the
/// disposition-table parser in overlap/disposition.rs for what that
/// class of bug looks like).
fn check_retirement_disclosed(
    project_root: &std::path::Path,
    change_id: &str,
) -> anyhow::Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let specs_dir = change_dir.join("specs");
    let retired_caps: Vec<String> = crate::gherkin::parse::collect_feature_files(&specs_dir)
        .into_iter()
        .filter_map(|path| {
            let feature =
                gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default()).ok()?;
            if crate::gherkin::delta::is_retired_feature(&feature) {
                crate::gherkin::delta::capability_tag(&feature).map(|c| c.to_string())
            } else {
                None
            }
        })
        .collect();
    if retired_caps.is_empty() {
        return Ok(());
    }
    let review_path = change_dir.join("review.md");
    let review_text = std::fs::read_to_string(&review_path).unwrap_or_default();
    let missing: Vec<&String> = retired_caps
        .iter()
        .filter(|cap| !review_text.contains(&format!("RETIRE: {cap}")))
        .collect();
    if !missing.is_empty() {
        anyhow::bail!(
            "review.md is missing a 'RETIRE: <capability>' line for: {}. \
             Retiring a capability must be disclosed and justified in review.md.",
            missing
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

/// Check if an artifact's output files exist in the change directory.
fn artifact_output_exists(
    artifact: &crate::config::manifest::Artifact,
    change_dir: &std::path::Path,
) -> bool {
    for pattern in &artifact.generates {
        let path = change_dir.join(pattern);
        if path.exists() {
            return true;
        }
        // Handle glob patterns like "specs/**/*.feature"
        if pattern.contains('*')
            && !crate::gherkin::parse::collect_feature_files(&change_dir.join("specs")).is_empty()
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Temp project with a change whose delta retires `dead-cap`.
    fn temp_change(review_text: &str) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        let change = project.join("givn/changes/retire-thing");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        std::fs::write(
            change.join("specs/retire-thing.feature"),
            "@givn.delta @givn.retired @dead-cap\nFeature: Dead capability\n",
        )
        .unwrap();
        std::fs::write(change.join("review.md"), review_text).unwrap();
        root
    }

    #[test]
    fn retirement_without_disclosure_fails() {
        let root = temp_change("# Review\n\nREVIEW: PASS\n");
        let project = root.path().to_path_buf();
        let err = check_retirement_disclosed(&project, "retire-thing").unwrap_err();
        assert!(err.to_string().contains("RETIRE: <capability>"), "{err}");
        assert!(err.to_string().contains("dead-cap"), "{err}");
    }

    #[test]
    fn disclosed_retirement_passes() {
        let root = temp_change("# Review\n\nRETIRE: dead-cap — obsolete engine\n");
        let project = root.path().to_path_buf();
        check_retirement_disclosed(&project, "retire-thing").unwrap();
    }

    #[test]
    fn no_retirement_delta_means_no_requirement() {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        let change = project.join("givn/changes/plain-change");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        std::fs::write(
            change.join("specs/plain-change.feature"),
            "@givn.delta @some-cap\nFeature: Some capability\n\n  Scenario: One\n    Given x\n    Then y\n",
        )
        .unwrap();
        check_retirement_disclosed(&project, "plain-change").unwrap();
    }
    #[test]
    fn parses_two_scenarios_with_the_same_revision() {
        let content = "- [x] Scenario: First\n  - [x] COMMIT: abc -- first\n\
                       - [x] Scenario: Second\n  - [x] COMMIT: abc -- second\n";
        let scenarios = parse_scenario_commits(content);
        assert_eq!(scenarios.len(), 2);
        assert_eq!(scenarios[0].hash.as_deref(), Some("abc"));
        assert_eq!(scenarios[1].hash.as_deref(), Some("abc"));
    }

    #[test]
    fn normalizes_commit_messages_for_scenario_matching() {
        let message = normalize_commit_text("feat: **Scenario with a title**");
        assert!(message.contains(&normalize_commit_text("Scenario with a title")));
        assert!(!message.contains(&normalize_commit_text("Different scenario")));
    }

    #[test]
    fn parses_a_scenario_with_its_own_revision() {
        let content = "- [x] Scenario: One scenario\n  - [x] COMMIT: abc -- One scenario\n";
        let scenarios = parse_scenario_commits(content);
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].title, "One scenario");
        assert!(scenarios[0].checked);
        assert_eq!(scenarios[0].hash.as_deref(), Some("abc"));
    }
}
