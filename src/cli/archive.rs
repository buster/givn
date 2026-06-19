use anyhow::{bail, Result};
use crate::config::overlay::load_effective;
use crate::gherkin::parse::collect_feature_files;
use crate::gherkin::merge::merge_file;
use crate::state::ChangeState;
use gherkin::GherkinEnv;

/// Archive a completed change.
///
/// Gate (ADR-028, arc42 ch.8.6):
/// 1. Check apply.requires: all required artifacts must exist.
/// 2. Check tasks complete (all [x]).
/// 3. Check review PASS (if review is in apply.requires).
/// 4. Run hooks of every apply.requires artifact.
/// 5. Merge delta .feature files into givn/specs/.
/// 6. Move change to givn/archive/<id>/.
///
/// All-or-nothing: if any step fails, abort with a clear error.
pub fn run(change: &str) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let change_dir = project_root.join("givn/changes").join(change);

    if !change_dir.exists() {
        bail!("change '{}' not found under givn/changes/", change);
    }

    // Check not already archived.
    let archive_target = project_root.join("givn/archive").join(change);
    if archive_target.exists() {
        bail!("change '{}' is already archived", change);
    }

    let manifest = load_effective()?;
    let state = ChangeState::from_path(&change_dir);

    println!("Archiving change '{change}'...");

    // === Gate checks ===

    // 1. Check apply.requires artifacts exist.
    for req in &manifest.apply.requires {
        let exists = match req.as_str() {
            "proposal" => state.proposal_exists,
            "specs" => state.specs_exist,
            "design" => state.design_exists,
            "tasks" => state.tasks_exists,
            "review" => state.review_exists,
            _ => false,
        };
        if !exists {
            bail!(
                "archive blocked: required artifact '{}' does not exist for change '{}'",
                req,
                change
            );
        }
    }

    // 2. Check tasks complete.
    if manifest.apply.requires.contains(&"tasks".to_string()) {
        match &state.tasks_progress {
            None => bail!("archive blocked: tasks.md not found"),
            Some(p) if !p.is_complete() => bail!(
                "archive blocked: tasks not complete ({}/{} done)",
                p.done,
                p.total
            ),
            _ => {}
        }
    }

    // 3. Check review PASS.
    if manifest.apply.requires.contains(&"review".to_string()) {
        if !state.review_pass {
            bail!(
                "archive blocked: review.md does not contain 'REVIEW: PASS'. \
                 Complete the review and add the sign-off marker."
            );
        }
    }

    // 4. Run hooks of all apply.requires artifacts.
    let features_path = change_dir
        .join("specs")
        .to_string_lossy()
        .into_owned();
    println!("Running archive gate hooks...");
    crate::hooks::run_archive_gate_hooks(&manifest, &features_path)?;
    println!("✓ Gate hooks passed");

    // === Merge delta features ===

    let delta_feature_dir = change_dir.join("specs");
    if delta_feature_dir.exists() {
        let delta_files = collect_feature_files(&delta_feature_dir);
        println!("Merging {} delta feature file(s)...", delta_files.len());

        for delta_path in &delta_files {
            let env = GherkinEnv::default();
            let delta_feature = gherkin::Feature::parse_path(delta_path, env).map_err(|e| {
                anyhow::anyhow!("failed to parse delta feature '{}': {}", delta_path.display(), e)
            })?;

            // Determine the permanent spec path: givn/specs/<cap>/<cap>.feature
            // Use the capability tag or the relative path of the delta file.
            let cap = crate::gherkin::delta::capability_tag(&delta_feature)
                .map(|t| t.to_string())
                .unwrap_or_else(|| {
                    delta_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string()
                });

            let permanent_dir = project_root.join("givn/specs").join(&cap);
            let permanent_path = permanent_dir.join(format!("{cap}.feature"));

            merge_file(&permanent_path, &delta_feature)?;
            println!("  merged → {}", permanent_path.display());
        }
    }

    // === Move change to archive ===

    std::fs::create_dir_all(&archive_target)?;
    copy_dir_all(&change_dir, &archive_target)?;
    std::fs::remove_dir_all(&change_dir)?;

    println!("✓ Archived '{change}' to {}", archive_target.display());
    println!("\nChange archived successfully.");
    println!("The delta specs have been merged into givn/specs/.");

    Ok(())
}

/// Recursively copy a directory tree.
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
