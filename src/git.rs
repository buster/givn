//! Root-bound Git worktree wrapper for the archive publication boundary.
//!
//! The archive requires the givn project to be the Git repository root: the
//! candidate and finalization commits, the clean-state witness, and the
//! evidence-path guard all operate on repository-root-relative paths.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A Git worktree whose top level equals the project root.
#[derive(Debug, Clone)]
pub struct Worktree {
    root: PathBuf,
}

impl Worktree {
    /// Construct the handle for a root already validated by the selection
    /// seam (`scm::select`), without repeating the prerequisite checks.
    pub fn at(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    /// Open the worktree, refusing a non-repository and a project nested
    /// inside a larger repository.
    /// The current `HEAD` commit id.
    pub fn head(&self) -> Result<String> {
        let output = run_git(&self.root, &["rev-parse", "HEAD"])?;
        if !output.status.success() {
            bail!(
                "cannot resolve the current revision in '{}': {}",
                self.root.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Whether `path` is present in the current `HEAD` revision.
    pub fn revision_contains(&self, path: &str) -> Result<bool> {
        let output = run_git(&self.root, &["cat-file", "-e", &format!("HEAD:{path}")])?;
        Ok(output.status.success())
    }

    /// Remove exactly `paths` from the index while keeping the working-tree
    /// files. Paths that are not tracked are ignored.
    pub fn remove_from_index(&self, paths: &[PathBuf]) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let mut args = vec![
            "rm".to_string(),
            "--cached".to_string(),
            "--ignore-unmatch".to_string(),
            "--".to_string(),
        ];
        args.extend(paths.iter().map(|path| path.to_string_lossy().into_owned()));
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let output = run_git(&self.root, &refs)?;
        if !output.status.success() {
            bail!(
                "failed to remove paths from the revision index: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(())
    }

    /// Reset the index to the current revision without touching the working
    /// tree. Used to undo staged transitions after a failed managed upgrade.
    pub fn restore_index(&self) -> Result<()> {
        let output = run_git(&self.root, &["reset", "--mixed", "-q", "HEAD"])?;
        if !output.status.success() {
            bail!(
                "failed to restore the revision index: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(())
    }

    /// Count the reachable revisions that touched each path.
    pub fn history_occurrences(&self, paths: &[String]) -> Result<Vec<(String, usize)>> {
        let mut out = Vec::new();
        for path in paths {
            let output = run_git(&self.root, &["log", "--all", "--format=%H", "--", path])?;
            if !output.status.success() {
                bail!(
                    "failed to scan history for '{path}': {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                );
            }
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
            if count > 0 {
                out.push((path.clone(), count));
            }
        }
        Ok(out)
    }

    /// Modified tracked files plus untracked non-ignored files, as paths
    /// relative to the repository root.
    pub fn dirty_entries(&self) -> Result<Vec<PathBuf>> {
        let output = run_git(&self.root, &["status", "--porcelain", "-z", "-uall"])?;
        if !output.status.success() {
            bail!(
                "cannot read the checkout state: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(parse_porcelain_z(&output.stdout))
    }

    /// Stage exactly `paths` relative to the repository root. `force` is
    /// reserved for the calculated managed allowlist; it is never widened to
    /// a directory or the complete worktree.
    pub fn stage_paths(&self, paths: &[PathBuf], force: bool) -> Result<()> {
        if paths.is_empty() {
            bail!("refusing to stage with no paths");
        }
        let mut args = vec!["add".to_string()];
        if force {
            args.push("-f".to_string());
        }
        args.push("--".to_string());
        args.extend(paths.iter().map(|path| path.to_string_lossy().into_owned()));
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let output = run_git(&self.root, &refs)?;
        if !output.status.success() {
            bail!(
                "failed to stage paths: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(())
    }

    /// Whether the index holds staged changes.
    pub fn has_staged_changes(&self) -> Result<bool> {
        let output = run_git(&self.root, &["diff", "--cached", "--quiet", "--exit-code"])?;
        match output.status.code() {
            Some(0) => Ok(false),
            Some(1) => Ok(true),
            _ => bail!(
                "could not inspect staged changes: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        }
    }

    /// Commit the staged changes.
    pub fn commit_staged(&self, message: &str) -> Result<()> {
        let output = run_git(&self.root, &["commit", "-m", message])?;
        if !output.status.success() {
            bail!(
                "the commit was rejected: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(())
    }

    /// Stage exactly `paths` and create one commit. Returns the commit id.
    pub fn commit(&self, message: &str, paths: &[String]) -> Result<String> {
        if paths.is_empty() {
            bail!("archive blocked: refusing to commit with no paths");
        }
        // Evidence may legitimately be gitignored (for example coverage
        // reports). Stage only paths Git can stage; a deleted tracked path
        // (the moved change directory) stays stageable.
        let mut stageable = Vec::new();
        for path in paths {
            let ignore = run_git(&self.root, &["check-ignore", "-q", "--", path])?;
            if ignore.status.success() {
                continue;
            }
            stageable.push(PathBuf::from(path));
        }
        if stageable.is_empty() {
            bail!(
                "archive blocked: no stageable evidence paths (all missing or gitignored): {}",
                paths.join(", ")
            );
        }
        self.stage_paths(&stageable, false).map_err(|error| {
            anyhow::anyhow!("archive blocked: failed to stage the archive paths: {error}")
        })?;
        self.commit_staged(message)
            .map_err(|error| anyhow::anyhow!("archive blocked: {error}"))?;
        self.head()
    }
}

fn run_git(root: &Path, args: &[&str]) -> Result<Output> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| format!("failed to run `git {}`", args.join(" ")))
}

/// Parse `git status --porcelain -z` output. Each NUL-separated field is
/// `XY <path>`; rename/copy entries carry the original path in the next
/// field. Both sides of a rename are reported so a rename touching an
/// evidence path can be classified.
fn parse_porcelain_z(bytes: &[u8]) -> Vec<PathBuf> {
    let text = String::from_utf8_lossy(bytes);
    let mut fields = text.split('\0').filter(|f| !f.is_empty()).peekable();
    let mut paths = Vec::new();
    while let Some(entry) = fields.next() {
        if entry.len() < 4 {
            continue;
        }
        let (status, path) = entry.split_at(3);
        paths.push(PathBuf::from(path));
        if (status.starts_with('R') || status.starts_with('C'))
            && let Some(original) = fields.next()
        {
            paths.push(PathBuf::from(original));
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_modified_and_untracked_entries() {
        let bytes = b" M src/lib.rs\0?? notes.txt\0";
        let paths = parse_porcelain_z(bytes);
        assert_eq!(
            paths,
            vec![PathBuf::from("src/lib.rs"), PathBuf::from("notes.txt")]
        );
    }

    #[test]
    fn parses_rename_both_sides() {
        let bytes = b"R  new.md\0old.md\0";
        let paths = parse_porcelain_z(bytes);
        assert_eq!(
            paths,
            vec![PathBuf::from("new.md"), PathBuf::from("old.md")]
        );
    }

    #[test]
    fn commit_skips_gitignored_evidence_paths() {
        let root = tempfile::TempDir::new().unwrap();
        let run = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(root.path())
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "git {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr)
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@example.com"]);
        run(&["config", "user.name", "T"]);
        std::fs::write(root.path().join(".gitignore"), "coverage/\n").unwrap();
        std::fs::write(root.path().join("README.md"), "readme\n").unwrap();
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "init"]);

        std::fs::create_dir_all(root.path().join("coverage")).unwrap();
        std::fs::write(root.path().join("coverage/report.xml"), "x").unwrap();
        std::fs::write(root.path().join("README.md"), "updated\n").unwrap();

        let worktree = Worktree::at(root.path());
        worktree
            .commit(
                "chore: evidence",
                &["README.md".to_string(), "coverage/report.xml".to_string()],
            )
            .unwrap();

        let names = run_git(root.path(), &["show", "--name-only", "--format=", "HEAD"]).unwrap();
        let names = String::from_utf8_lossy(&names.stdout);
        assert!(names.contains("README.md"));
        assert!(
            !names.contains("coverage/report.xml"),
            "ignored evidence must not be committed: {names}"
        );
    }
}
