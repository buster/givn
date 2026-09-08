use crate::config::overlay::load_effective;
use anyhow::{Result, bail};

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
        // Run mandatory checks first, then custom hooks.
        run_mandatory_checks(
            artifact_id,
            &manifest,
            &features_path,
            &project_root,
            change,
        )?;
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
/// verify, verify-e2e, and integrity. All other artifacts have no
/// mandatory checks; their behavior comes from post hooks only.
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
            crate::hooks::run_verify_hook(manifest, features_path)?;
            println!("  ✓ verify passed");
        }
        "review" => {
            println!("Mandatory checks for 'review':");
            crate::hooks::run_verify_hook(manifest, features_path)?;
            println!("  ✓ verify passed");
            crate::hooks::run_verify_e2e_hook(manifest, features_path)?;
            println!("  ✓ verify-e2e passed");
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
            } else {
                println!(
                    "  ⚠ no --change given — overlap, retirement, and routing checks were skipped \
                     (they require a specific change to scope to)"
                );
            }
        }
        _ => {}
    }
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
}
