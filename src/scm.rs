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
    /// A Fossil checkout whose root equals the project root.
    Fossil(crate::fossil::FossilCheckout),
}

impl Backend {
    /// The current revision identifier of the project.
    pub fn revision(&self) -> Result<String> {
        match self {
            Self::Git(worktree) => worktree.head(),
            Self::Fossil(checkout) => checkout.revision(),
        }
    }

    /// Modified tracked files plus untracked non-ignored files, as paths
    /// relative to the project root.
    pub fn dirty_entries(&self) -> Result<Vec<PathBuf>> {
        match self {
            Self::Git(worktree) => worktree.dirty_entries(),
            Self::Fossil(checkout) => checkout.dirty_entries(),
        }
    }

    /// Stage exactly `paths`; `force` is reserved for the calculated managed
    /// allowlist.
    pub fn stage_paths(&self, paths: &[PathBuf], force: bool) -> Result<()> {
        match self {
            Self::Git(worktree) => worktree.stage_paths(paths, force),
            Self::Fossil(checkout) => checkout.stage_paths(paths, force),
        }
    }

    /// Whether the backend holds staged changes. Fossil has no index, so the
    /// pending checkout changes stand in for the staged set.
    pub fn has_staged_changes(&self) -> Result<bool> {
        match self {
            Self::Git(worktree) => worktree.has_staged_changes(),
            Self::Fossil(checkout) => checkout.has_staged_changes(),
        }
    }

    /// Commit the staged changes.
    pub fn commit_staged(&self, message: &str) -> Result<()> {
        match self {
            Self::Git(worktree) => worktree.commit_staged(message),
            Self::Fossil(checkout) => checkout.commit_staged(message),
        }
    }

    /// Publish exactly `paths` as one revision. Returns the new revision.
    pub fn publish(&self, paths: &[String], message: &str) -> Result<String> {
        match self {
            Self::Git(worktree) => worktree.commit(message, paths),
            Self::Fossil(checkout) => checkout.publish(paths, message),
        }
    }

    /// Whether `path` is present in the current revision.
    pub fn revision_contains(&self, path: &str) -> Result<bool> {
        match self {
            Self::Git(worktree) => worktree.revision_contains(path),
            Self::Fossil(checkout) => checkout.revision_contains(path),
        }
    }
}

/// Select the backend that owns `project_root`, or refuse by name.
pub fn select(project_root: &Path) -> Result<Backend> {
    let git = git_worktree(project_root)?;
    let fossil = fossil_marker(project_root);
    match (git, fossil) {
        (true, None) => {
            let top = git_top_level(project_root)?;
            if top != canonical(project_root) {
                bail!(
                    "the project root '{}' must be the worktree top level; it is nested inside '{}'",
                    project_root.display(),
                    top.display()
                );
            }
            Ok(Backend::Git(crate::git::Worktree::at(project_root)))
        }
        (true, Some(marker)) => bail!(
            "ambiguous version control at '{}': both a Git worktree marker and a Fossil checkout marker ({marker}) are present",
            project_root.display()
        ),
        (false, Some(_)) => {
            fossil_executable(project_root)?;
            Ok(Backend::Fossil(crate::fossil::FossilCheckout::at(
                project_root,
            )))
        }
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
        .output();
    match output {
        Ok(output) => {
            Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if project_root.join(".git").exists() {
                bail!(
                    "Git worktree detected at '{}' but the `git` executable was not found on PATH",
                    project_root.display()
                );
            }
            Ok(false)
        }
        Err(error) => Err(error)
            .with_context(|| format!("run `git rev-parse` in '{}'", project_root.display())),
    }
}

/// The canonicalized worktree top level for a project inside a Git worktree.
fn git_top_level(project_root: &Path) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(project_root)
        .output()
        .with_context(|| format!("run `git rev-parse` in '{}'", project_root.display()))?;
    if !output.status.success() {
        bail!(
            "cannot resolve the Git worktree top level from '{}'",
            project_root.display()
        );
    }
    let top = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(canonical(Path::new(&top)))
}

/// Refuse a Fossil checkout when its executable is not on PATH.
fn fossil_executable(project_root: &Path) -> Result<()> {
    match Command::new("fossil").arg("version").output() {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => bail!(
            "Fossil checkout detected at '{}' but the `fossil` executable was not found on PATH",
            project_root.display()
        ),
        Err(error) => Err(error)
            .with_context(|| format!("run `fossil version` in '{}'", project_root.display())),
    }
}

fn canonical(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// The Fossil checkout marker at the project root, when present.
fn fossil_marker(project_root: &Path) -> Option<&'static str> {
    [".fslckout", "_FOSSIL_"]
        .into_iter()
        .find(|name| project_root.join(name).exists())
}
