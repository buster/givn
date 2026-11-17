//! SCM backend selection.
//!
//! One seam answers "which version-control system owns this project root?".
//! Consumers act on the returned [`Backend`] or receive a named refusal; they
//! never derive the backend themselves.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

/// How a path is represented in a revision tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Regular,
    Symlink,
    Submodule,
}

/// One tracked path at a revision, with the facts a clean-room write needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionEntry {
    pub path: String,
    pub kind: EntryKind,
    pub executable: bool,
}

/// One content path plus its backend-specific identity. `gitlink` carries a
/// submodule's recorded object id; other paths leave it empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentEntry {
    pub path: PathBuf,
    pub gitlink: Option<String>,
}

/// The selected version-control backend.
#[derive(Debug, Clone)]
pub enum Backend {
    /// A Git worktree whose top level equals the project root.
    Git(crate::git::Worktree),
    /// A Fossil checkout whose root equals the project root.
    Fossil(crate::fossil::FossilCheckout),
}

impl Backend {
    /// The backend's name in the trial report and record.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Git(_) => "git",
            Self::Fossil(_) => "fossil",
        }
    }

    /// The current revision identifier of the project.
    pub fn revision(&self) -> Result<String> {
        match self {
            Self::Git(worktree) => worktree.head(),
            Self::Fossil(checkout) => checkout.revision(),
        }
    }

    /// Read a file from the current revision, or `None` when it is absent.
    pub fn revision_file(&self, path: &str) -> Result<Option<String>> {
        match self {
            Self::Git(worktree) => worktree.revision_file(path),
            Self::Fossil(checkout) => checkout.revision_file(path),
        }
    }

    /// Tracked paths plus untracked non-ignored paths, relative to the
    /// project root. Gitlinks carry their recorded object id.
    pub fn content_paths(&self) -> Result<Vec<ContentEntry>> {
        match self {
            Self::Git(worktree) => worktree.content_paths(),
            Self::Fossil(checkout) => checkout.content_paths(),
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

    /// Remove exactly `paths` from the current revision while keeping their
    /// working-tree files (the local-only transition).
    pub fn remove_from_index(&self, paths: &[PathBuf]) -> Result<()> {
        match self {
            Self::Git(worktree) => worktree.remove_from_index(paths),
            Self::Fossil(checkout) => checkout.remove_from_index(paths),
        }
    }

    /// Restore the current revision's index state. Used after a failed
    /// managed upgrade rolled the working tree back, so staged transitions
    /// do not survive. Fossil has no index and is a no-op.
    pub fn restore_index(&self) -> Result<()> {
        match self {
            Self::Git(worktree) => worktree.restore_index(),
            Self::Fossil(_) => Ok(()),
        }
    }

    /// Count the reachable revisions that touched each path. Paths with no
    /// recorded occurrence are omitted.
    pub fn history_occurrences(&self, paths: &[String]) -> Result<Vec<(String, usize)>> {
        match self {
            Self::Git(worktree) => worktree.history_occurrences(paths),
            Self::Fossil(checkout) => checkout.history_occurrences(paths),
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

    /// Whether `revision` names a commit/check-in.
    pub fn commit_exists(&self, revision: &str) -> Result<bool> {
        match self {
            Self::Git(worktree) => worktree.commit_exists(revision),
            Self::Fossil(checkout) => checkout.commit_exists(revision),
        }
    }

    /// The full message of `revision`.
    pub fn commit_message(&self, revision: &str) -> Result<Option<String>> {
        match self {
            Self::Git(worktree) => worktree.commit_message(revision),
            Self::Fossil(checkout) => checkout.commit_message(revision),
        }
    }

    /// Whether `revision` is reachable from the current revision.
    pub fn is_ancestor(&self, revision: &str) -> Result<bool> {
        match self {
            Self::Git(worktree) => worktree.is_ancestor(revision),
            Self::Fossil(checkout) => checkout.is_ancestor(revision),
        }
    }

    /// Whether `path` is present in the current revision.
    pub fn revision_contains(&self, path: &str) -> Result<bool> {
        match self {
            Self::Git(worktree) => worktree.revision_contains(path),
            Self::Fossil(checkout) => checkout.revision_contains(path),
        }
    }

    /// The oldest reachable revision that touched `path`.
    pub fn first_touching_revision(&self, path: &str) -> Result<Option<String>> {
        match self {
            Self::Git(worktree) => worktree.first_touching_revision(path),
            Self::Fossil(checkout) => checkout.first_touching_revision(path),
        }
    }

    /// The first parent of `revision`, or `None` for a root revision.
    pub fn parent_revision(&self, revision: &str) -> Result<Option<String>> {
        match self {
            Self::Git(worktree) => worktree.parent_revision(revision),
            Self::Fossil(checkout) => checkout.parent_revision(revision),
        }
    }

    /// Tracked files under `root` in `revision`, with their entry kind.
    pub fn revision_files(&self, revision: &str, root: &str) -> Result<Vec<RevisionEntry>> {
        match self {
            Self::Git(worktree) => worktree.revision_files(revision, root),
            Self::Fossil(checkout) => checkout.revision_files(revision, root),
        }
    }

    /// One path's bytes at `revision`, or `None` when the backend holds no
    /// blob for it. The caller owns the refusal sentence, because which read
    /// refused (the declaration or a trial input) changes the words.
    pub fn revision_blob(&self, revision: &str, path: &str) -> Result<Option<Vec<u8>>> {
        match self {
            Self::Git(worktree) => worktree.revision_blob(revision, path),
            Self::Fossil(checkout) => checkout.revision_blob(revision, path),
        }
    }

    /// Read the named blobs from `revision`. A path the backend cannot read
    /// is a named refusal: a skipped path would silently shrink the trial.
    pub fn revision_blobs(
        &self,
        revision: &str,
        paths: &[String],
    ) -> Result<Vec<(String, Vec<u8>)>> {
        let mut blobs = Vec::with_capacity(paths.len());
        for path in paths {
            match self.revision_blob(revision, path)? {
                Some(bytes) => blobs.push((path.clone(), bytes)),
                None => bail!(
                    "'{path}' cannot be read from revision '{revision}'; the backend returned no blob for it"
                ),
            }
        }
        Ok(blobs)
    }

    /// Changed paths under `root` from `base` to the current revision, plus
    /// untracked non-ignored paths. With no base, every tracked path is
    /// returned.
    pub fn changed_paths(&self, base: Option<&str>, root: &str) -> Result<Vec<String>> {
        match self {
            Self::Git(worktree) => worktree.changed_paths(base, root),
            Self::Fossil(checkout) => checkout.changed_paths(base, root),
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
        (true, Some(marker)) => {
            // A Git worktree that merely *contains* the root (its top level
            // differs) does not own it; the root-local Fossil marker wins.
            // Only markers at the root itself are ambiguous.
            if git_top_level(project_root)? == canonical(project_root) {
                bail!(
                    "ambiguous version control at '{}': both a Git worktree marker and a Fossil checkout marker ({marker}) are present",
                    project_root.display()
                );
            }
            fossil_executable(project_root)?;
            Ok(Backend::Fossil(crate::fossil::FossilCheckout::at(
                project_root,
            )))
        }
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
