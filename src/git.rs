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

    /// Read a file from the current revision. Returns `None` when the path is
    /// absent from `HEAD`.
    pub fn revision_file(&self, path: &str) -> Result<Option<String>> {
        let output = run_git(&self.root, &["show", &format!("HEAD:{path}")])?;
        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()))
        } else {
            Ok(None)
        }
    }

    /// Tracked paths plus untracked non-ignored paths, relative to the
    /// worktree root. A gitlink carries its recorded object id.
    pub fn content_paths(&self) -> Result<Vec<crate::scm::ContentEntry>> {
        let listed = run_git(
            &self.root,
            &[
                "ls-files",
                "-z",
                "--cached",
                "--others",
                "--exclude-standard",
            ],
        )?;
        if !listed.status.success() {
            bail!(
                "`git ls-files` failed in '{}': {}",
                self.root.display(),
                String::from_utf8_lossy(&listed.stderr).trim()
            );
        }
        let staged = run_git(&self.root, &["ls-files", "-s", "-z"])?;
        let mut gitlinks = std::collections::BTreeMap::new();
        if staged.status.success() {
            for record in String::from_utf8_lossy(&staged.stdout).split('\0') {
                let mut fields = record.split_whitespace();
                let mode = fields.next().unwrap_or_default();
                let object = fields.next().unwrap_or_default();
                let path = fields.next().unwrap_or_default();
                if mode == "160000" && !path.is_empty() {
                    gitlinks.insert(PathBuf::from(path), object.to_string());
                }
            }
        }
        let mut entries = Vec::new();
        for path in String::from_utf8_lossy(&listed.stdout).split('\0') {
            if path.is_empty() {
                continue;
            }
            let relative = PathBuf::from(path);
            let gitlink = gitlinks.get(&relative).cloned();
            entries.push(crate::scm::ContentEntry {
                path: relative,
                gitlink,
            });
        }
        entries.sort_by(|left, right| left.path.cmp(&right.path));
        entries.dedup_by(|left, right| left.path == right.path);
        Ok(entries)
    }

    /// Whether `revision` names a commit in this repository.
    pub fn commit_exists(&self, revision: &str) -> Result<bool> {
        let output = run_git(
            &self.root,
            &["cat-file", "-e", &format!("{revision}^{{commit}}")],
        )?;
        Ok(output.status.success())
    }

    /// The full message of `revision`, or `None` when it does not exist.
    pub fn commit_message(&self, revision: &str) -> Result<Option<String>> {
        let output = run_git(&self.root, &["log", "-1", "--format=%B", revision])?;
        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()))
        } else {
            Ok(None)
        }
    }

    /// Whether `revision` is reachable from `HEAD`.
    pub fn is_ancestor(&self, revision: &str) -> Result<bool> {
        let output = run_git(
            &self.root,
            &["merge-base", "--is-ancestor", revision, "HEAD"],
        )?;
        match output.status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => bail!(
                "could not inspect ancestry of '{revision}': {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        }
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

    pub fn commit_selected(&self, message: &str, paths: &[String]) -> Result<String> {
        if paths.is_empty() {
            bail!("explicit paths are required");
        }
        let conflicts = run_git(&self.root, &["ls-files", "--unmerged", "-z"])?;
        if !conflicts.status.success() || !conflicts.stdout.is_empty() {
            bail!("cannot isolate selected paths with an unmerged index; resolve conflicts first");
        }
        let mut additions = Vec::new();
        for path in paths {
            let known = run_git(
                &self.root,
                &[
                    "--literal-pathspecs",
                    "ls-files",
                    "--cached",
                    "--others",
                    "--exclude-standard",
                    "-z",
                    "--",
                    path,
                ],
            )?;
            if !known.status.success() {
                bail!("cannot inspect explicit path '{path}'");
            }
            if known.stdout.is_empty() {
                let deleted = run_git(
                    &self.root,
                    &[
                        "--literal-pathspecs",
                        "diff",
                        "--cached",
                        "--name-only",
                        "--diff-filter=D",
                        "-z",
                        "--",
                        path,
                    ],
                )?;
                if !deleted.status.success() || deleted.stdout.is_empty() {
                    bail!("unknown or ignored explicit path '{path}'");
                }
            }
            let untracked = run_git(
                &self.root,
                &[
                    "--literal-pathspecs",
                    "ls-files",
                    "--others",
                    "--exclude-standard",
                    "-z",
                    "--",
                    path,
                ],
            )?;
            if !untracked.status.success() {
                bail!("cannot inspect explicit path '{path}'");
            }
            additions.extend(parse_nul_paths(&untracked.stdout)?);
        }
        if !additions.is_empty() {
            let mut args = vec!["--literal-pathspecs", "add", "--intent-to-add", "--"];
            args.extend(additions.iter().map(String::as_str));
            let output = run_git(&self.root, &args)?;
            if !output.status.success() {
                bail!(
                    "cannot record selected additions: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                );
            }
        }
        let mut args = vec![
            "--literal-pathspecs",
            "commit",
            "--only",
            "--message",
            message,
            "--",
        ];
        args.extend(paths.iter().map(String::as_str));
        let output = run_git(&self.root, &args)?;
        if !output.status.success() {
            bail!(
                "the selected commit was rejected: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        self.head()
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

    /// The oldest reachable revision that touched `path`.
    pub fn first_touching_revision(&self, path: &str) -> Result<Option<String>> {
        let shallow = run_git(&self.root, &["rev-parse", "--is-shallow-repository"])?;
        if shallow.status.success() && String::from_utf8_lossy(&shallow.stdout).trim() == "true" {
            bail!(
                "cannot resolve change base: repository is shallow; fetch the full history before review"
            );
        }
        let output = run_git(
            &self.root,
            &[
                "rev-list",
                "--reverse",
                "--full-history",
                "HEAD",
                "--",
                path,
            ],
        )?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .map(ToString::to_string))
    }

    /// The first parent of `revision`, or `None` for a root revision.
    pub fn parent_revision(&self, revision: &str) -> Result<Option<String>> {
        let output = run_git(
            &self.root,
            &["rev-parse", "--verify", &format!("{revision}^1")],
        )?;
        if !output.status.success() {
            return Ok(None);
        }
        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ))
    }

    /// Tracked files under `root` in `revision`, with their entry kind.
    pub fn revision_files(
        &self,
        revision: &str,
        root: &str,
    ) -> Result<Vec<crate::scm::RevisionEntry>> {
        let mut args = vec!["ls-tree", "-r", "-z", revision];
        if !root.is_empty() {
            args.push("--");
            args.push(root);
        }
        let output = run_git(&self.root, &args)?;
        if !output.status.success() {
            bail!(
                "`git ls-tree` failed in '{}': {}",
                self.root.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        let mut files = Vec::new();
        for entry in output.stdout.split(|byte| *byte == 0) {
            if entry.is_empty() {
                continue;
            }
            let Some(tab) = entry.iter().position(|byte| *byte == b'\t') else {
                continue;
            };
            let meta = String::from_utf8_lossy(&entry[..tab]);
            let path = revision_path(&entry[tab + 1..])?;
            let mut parts = meta.split_whitespace();
            let mode = parts.next().unwrap_or("");
            let kind = parts.next().unwrap_or("");
            let entry_kind = if kind == "commit" {
                crate::scm::EntryKind::Submodule
            } else if mode == "120000" {
                crate::scm::EntryKind::Symlink
            } else {
                crate::scm::EntryKind::Regular
            };
            files.push(crate::scm::RevisionEntry {
                path,
                kind: entry_kind,
                executable: entry_kind == crate::scm::EntryKind::Regular && mode == "100755",
            });
        }
        Ok(files)
    }

    /// Read the named blobs from `revision`.
    pub fn revision_blobs(
        &self,
        revision: &str,
        paths: &[String],
    ) -> Result<Vec<(String, Vec<u8>)>> {
        let mut blobs = Vec::new();
        for path in paths {
            let output = run_git(&self.root, &["show", &format!("{revision}:{path}")])?;
            if output.status.success() {
                blobs.push((path.clone(), output.stdout));
            }
        }
        Ok(blobs)
    }

    /// Changed paths under `root` from `base` to the current revision.
    pub fn changed_paths(&self, base: Option<&str>, root: &str) -> Result<Vec<String>> {
        let range = base.map(|base| format!("{base}..HEAD"));
        let args: Vec<&str> = match range.as_deref() {
            Some(range) => vec![
                "diff",
                "--name-only",
                "--no-renames",
                "-z",
                range,
                "--",
                root,
            ],
            None => vec!["ls-files", "-z", "--", root],
        };
        let output = run_git(&self.root, &args)?;
        if !output.status.success() {
            bail!(
                "`git {}` failed in '{}': {}",
                args.join(" "),
                self.root.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        let mut paths = parse_nul_paths(&output.stdout)?;
        let untracked = run_git(
            &self.root,
            &[
                "ls-files",
                "--others",
                "--exclude-standard",
                "-z",
                "--",
                root,
            ],
        )?;
        paths.extend(parse_nul_paths(&untracked.stdout)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

fn revision_path(bytes: &[u8]) -> Result<String> {
    let path = std::str::from_utf8(bytes).with_context(|| {
        format!(
            "revision path is not valid UTF-8: {}",
            String::from_utf8_lossy(bytes)
        )
    })?;
    if path.contains('\n') || path.contains('\r') {
        bail!(
            "revision path contains a line break: {}",
            path.escape_debug()
        );
    }
    Ok(path.to_string())
}

fn parse_nul_paths(bytes: &[u8]) -> Result<Vec<String>> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .map(revision_path)
        .collect()
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
    fn rejects_invalid_utf8_and_line_break_revision_paths() {
        assert_eq!(revision_path(b"src/engine.rs").unwrap(), "src/engine.rs");
        let line_break = revision_path(b"src/a\nb.rs").unwrap_err().to_string();
        assert!(line_break.contains("revision path contains a line break"));
        let invalid = revision_path(&[0xff, 0xfe]).unwrap_err().to_string();
        assert!(invalid.contains("revision path is not valid UTF-8"));
    }

    #[test]
    fn parses_nul_separated_paths() {
        assert_eq!(
            parse_nul_paths(b"src/a.rs\0src/b.rs\0").unwrap(),
            vec!["src/a.rs".to_string(), "src/b.rs".to_string()]
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
