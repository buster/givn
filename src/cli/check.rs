use crate::config::overlay::load_effective;
use anyhow::Result;

/// Run an artifact's post hooks (e.g. verify.command, lint).
///
/// `givn check <artifact>` — runs that artifact's hooks.
/// `givn check` (no arg) — runs hooks for all artifacts with existing output.
pub fn run(artifact: Option<&str>, change: Option<&str>) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let manifest = load_effective()?;

    // Determine features_path: active change's features, or default specs.
    let features_path = if let Some(id) = change {
        let change_dir = project_root.join("givn/changes").join(id);
        if change_dir.join("specs").exists() {
            change_dir.join("specs").to_string_lossy().into_owned()
        } else {
            manifest.verify.features_path.clone()
        }
    } else {
        manifest.verify.features_path.clone()
    };

    if let Some(artifact_id) = artifact {
        // Run hooks for a specific artifact.
        crate::hooks::run_artifact_hooks(artifact_id, &manifest, &features_path)
    } else {
        // Run hooks for all artifacts whose output exists.
        let change_dir = if let Some(id) = change {
            Some(project_root.join("givn/changes").join(id))
        } else {
            None
        };

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
        if pattern.contains('*') {
            if crate::gherkin::parse::collect_feature_files(&change_dir.join("specs")).len() > 0 {
                return true;
            }
        }
    }
    false
}
