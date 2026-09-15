//! SCM-neutral commit path (`givn commit`).
//!
//! One command creates one attributed revision on either backend: Git stages
//! the named paths (or every pending change) and commits; Fossil records the
//! named paths (or records all changes) and checks in. The managed
//! attribution trailer is stamped when the harness provides context.

use anyhow::{Context, Result, bail};

/// Create one revision with `message` (plus the managed attribution trailer
/// when attribution context is active), recording `paths` — or every pending
/// change when no paths are given.
pub fn run(message: &str, paths: &[String]) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let backend = crate::scm::select(&project_root)?;

    let stamped = crate::cli::attribution::stamped_message(message);
    match &backend {
        crate::scm::Backend::Git(worktree) => {
            if paths.is_empty() {
                worktree.stage_all()?;
            } else {
                let relative: Vec<std::path::PathBuf> =
                    paths.iter().map(std::path::PathBuf::from).collect();
                worktree.stage_paths(&relative, false)?;
            }
        }
        crate::scm::Backend::Fossil(checkout) => {
            if paths.is_empty() {
                checkout.record_all_changes()?;
            } else {
                let relative: Vec<std::path::PathBuf> =
                    paths.iter().map(std::path::PathBuf::from).collect();
                checkout.stage_paths(&relative, false)?;
            }
        }
    }

    if !backend.has_staged_changes()? {
        bail!("nothing to commit: the checkout has no pending changes");
    }
    match &backend {
        // Fossil has no index: a bare commit would include every pending
        // change, so named paths are committed explicitly.
        crate::scm::Backend::Fossil(checkout) if !paths.is_empty() => {
            let relative: Vec<std::path::PathBuf> =
                paths.iter().map(std::path::PathBuf::from).collect();
            checkout
                .commit_paths(&stamped, &relative)
                .with_context(|| "the revision was rejected".to_string())?;
        }
        _ => backend
            .commit_staged(&stamped)
            .with_context(|| "the revision was rejected".to_string())?,
    }
    println!(
        "[{}] committed revision on the {} backend",
        crate::subprocess::local_timestamp(),
        match backend {
            crate::scm::Backend::Git(_) => "Git",
            crate::scm::Backend::Fossil(_) => "Fossil",
        }
    );
    Ok(())
}
