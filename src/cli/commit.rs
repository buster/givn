//! SCM-neutral commit path (`givn commit`).
//!
//! One command creates one attributed revision on either backend: Git stages
//! the named paths (or every pending change) and commits; Fossil records the
//! named paths (or records all changes) and checks in. The managed
//! attribution trailer is stamped when the harness provides context.
//!
//! Under the `local-only` rules policy the stage-all path excludes
//! `AGENTS.md` and every strict rule: project content only.

use anyhow::{Context, Result, bail};
use std::path::PathBuf;

/// Create one revision with `message` (plus the managed attribution trailer
/// when attribution context is active), recording `paths` — or every pending
/// change when no paths are given.
pub fn run(message: &str, paths: &[String]) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let backend = crate::scm::select(&project_root)?;
    let policy = crate::config::overlay::load_rules_policy(&project_root)?;

    if paths.is_empty() {
        let excluded: Vec<String> = std::iter::once("AGENTS.md".to_string())
            .chain(if policy.is_local_only() {
                crate::skills::rules::strict_rules(&project_root)?
            } else {
                Vec::new()
            })
            .collect();
        let include: Vec<PathBuf> = backend
            .dirty_entries()?
            .into_iter()
            .filter(|path| {
                let relative = path.to_string_lossy().replace('\\', "/");
                !excluded.iter().any(|entry| entry == &relative)
            })
            .collect();
        if include.is_empty() {
            bail!("nothing to commit: the checkout has no pending project content");
        }
        let staged_paths: Vec<String> = include
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();
        backend.stage_paths(&include, false)?;
        let stamped = crate::cli::attribution::stamped_message(message);
        return commit(backend, &staged_paths, &stamped);
    }

    let stamped = crate::cli::attribution::stamped_message(message);
    match &backend {
        crate::scm::Backend::Git(worktree) => {
            let relative: Vec<std::path::PathBuf> =
                paths.iter().map(std::path::PathBuf::from).collect();
            worktree.stage_paths(&relative, false)?;
        }
        crate::scm::Backend::Fossil(checkout) => {
            let relative: Vec<std::path::PathBuf> =
                paths.iter().map(std::path::PathBuf::from).collect();
            checkout.stage_paths(&relative, false)?;
        }
    }

    commit(backend, paths, &stamped)
}

/// Commit the staged set, or exactly `paths` when the backend has no index.
fn commit(backend: crate::scm::Backend, paths: &[String], message: &str) -> Result<()> {
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
                .commit_paths(message, &relative)
                .with_context(|| "the revision was rejected".to_string())?;
        }
        _ => backend
            .commit_staged(message)
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
