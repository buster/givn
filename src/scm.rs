//! SCM backend selection.
//!
//! One seam answers "which version-control system owns this project root?".
//! Consumers act on the returned [`Backend`] or receive a named refusal; they
//! never derive the backend themselves.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

/// The selected version-control backend.
#[derive(Debug, Clone)]
pub enum Backend {
    /// A Git worktree whose top level equals the project root.
    Git(crate::git::Worktree),
}

impl Backend {
    /// Modified tracked files plus untracked non-ignored files, as paths
    /// relative to the project root.
    pub fn dirty_entries(&self) -> Result<Vec<PathBuf>> {
        match self {
            Self::Git(worktree) => worktree.dirty_entries(),
        }
    }
}

/// Select the backend that owns `project_root`, or refuse by name.
pub fn select(project_root: &Path) -> Result<Backend> {
    let git = git_worktree(project_root)?;
    let fossil = fossil_marker(project_root);
    match (git, fossil) {
        (true, None) => Ok(Backend::Git(crate::git::Worktree::open(project_root)?)),
        (true, Some(marker)) => bail!(
            "ambiguous version control at '{}': both a Git worktree marker and a Fossil checkout marker ({marker}) are present",
            project_root.display()
        ),
        (false, Some(_)) => bail!(
            "Fossil checkout detected at '{}'; the Fossil backend is not available in this version",
            project_root.display()
        ),
        (false, None) => bail!(
            "no Git worktree or Fossil checkout found at '{}'; a version-controlled project is required",
            project_root.display()
        ),
    }
}

/// Whether the project root is inside a Git worktree.
fn git_worktree(project_root: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(project_root)
        .output()
        .with_context(|| format!("run `git rev-parse` in '{}'", project_root.display()))?;
    Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
}

/// The Fossil checkout marker at the project root, when present.
fn fossil_marker(project_root: &Path) -> Option<&'static str> {
    [".fslckout", "_FOSSIL_"]
        .into_iter()
        .find(|name| project_root.join(name).exists())
}
