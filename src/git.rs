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
    pub fn open(root: &Path) -> Result<Self> {
        let inside = run_git(root, &["rev-parse", "--is-inside-work-tree"])?;
        if !inside.status.success() {
            bail!(
                "archive blocked: Git is required to archive a change — '{}' is not a Git work tree",
                root.display()
            );
        }

        let top = run_git(root, &["rev-parse", "--show-toplevel"])?;
        if !top.status.success() {
            bail!(
                "archive blocked: Git is required to archive a change — cannot resolve the repository root from '{}'",
                root.display()
            );
        }
        let top_level = String::from_utf8_lossy(&top.stdout).trim().to_string();
        let top_path =
            std::fs::canonicalize(&top_level).unwrap_or_else(|_| PathBuf::from(&top_level));
        let root_canonical = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
        if top_path != root_canonical {
            bail!(
                "archive blocked: the project root must be the repository root — '{}' is nested inside '{}'",
                root.display(),
                top_path.display()
            );
        }

        Ok(Self {
            root: root.to_path_buf(),
        })
    }

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

        let worktree = Worktree::open(root.path()).unwrap();
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
