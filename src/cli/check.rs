use crate::config::overlay::load_effective;
use anyhow::{Result, bail};

/// Run an artifact's post hooks (e.g. verify.command, lint).
///
/// `givn check <artifact>` — runs that artifact's hooks.
/// `givn check` (no arg) — runs hooks for all artifacts with existing output.
pub fn run(artifact: Option<&str>, change: Option<&str>) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let manifest = load_effective()?;

    // Determine features_path: active change's features, or default specs.
    let features_path = if let Some(id) = change {
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        change_dir.join("specs").to_string_lossy().into_owned()
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
            )?;
            println!("  ✓ integrity passed");
            if let Some(id) = change {
                crate::overlap::analysis::run_review_overlap_gate(project_root, id)?;
                run_advisory_retrieval(project_root, id);
            }
        }
        _ => {}
    }
    Ok(())
}

/// Retrieval is advisory: setup failures are visible but never alter the
/// deterministic review gate's exit status.
fn run_advisory_retrieval(project_root: &std::path::Path, change_id: &str) {
    if let Err(error) = crate::spec::retrieval::review(project_root, change_id) {
        println!("retrieval advisory setup unavailable: {error:#}");
    }
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
