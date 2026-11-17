//! Fossil checkout operations behind the SCM seam.
//!
//! Fossil has no staging index: `publish` records all changes and commits the
//! witnessed dirty set, validated against the resolved publication paths.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use anyhow::{Context, Result, bail};

/// A Fossil checkout whose root is the project root.
#[derive(Debug, Clone)]
pub struct FossilCheckout {
    root: PathBuf,
}

impl FossilCheckout {
    /// Construct a handle without probing; the seam already detected the
    /// checkout and verified the boundary.
    pub fn at(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    /// The displayed 40-hex identifier of the current checkout revision.
    pub fn revision(&self) -> Result<String> {
        let output = self.run(&["info"])?;
        if !output.status.success() {
            bail!(
                "`fossil info` failed in '{}': {}",
                self.root.display(),
                stderr(&output)
            );
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout
            .lines()
            .find(|line| line.starts_with("checkout:"))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "the Fossil checkout at '{}' has no current revision",
                    self.root.display()
                )
            })?;
        let id = line["checkout:".len()..]
            .split_whitespace()
            .next()
            .unwrap_or_default();
        if id.is_empty() {
            bail!(
                "the Fossil checkout at '{}' reports an empty revision",
                self.root.display()
            );
        }
        Ok(id.to_string())
    }

    /// Tracked paths plus untracked (`EXTRA`) paths, relative to the root.
    pub fn content_paths(&self) -> Result<Vec<crate::scm::ContentEntry>> {
        let mut paths = Vec::new();
        let listed = self.run(&["ls"])?;
        if !listed.status.success() {
            bail!(
                "`fossil ls` failed in '{}': {}",
                self.root.display(),
                stderr(&listed)
            );
        }
        for line in String::from_utf8_lossy(&listed.stdout).lines() {
            let path = line.trim();
            if !path.is_empty() {
                paths.push(PathBuf::from(path));
            }
        }
        let extras = self.run(&["extras"])?;
        if extras.status.success() {
            for line in String::from_utf8_lossy(&extras.stdout).lines() {
                let path = line.trim().trim_start_matches("EXTRA ").trim();
                if !path.is_empty() {
                    paths.push(PathBuf::from(path));
                }
            }
        }
        paths.sort();
        paths.dedup();
        Ok(paths
            .into_iter()
            .map(|path| crate::scm::ContentEntry {
                path,
                gitlink: None,
            })
            .collect())
    }

    /// Changed paths relative to the root, including `EXTRA` entries.
    pub fn dirty_entries(&self) -> Result<Vec<PathBuf>> {
        let output = self.run(&["changes", "--differ"])?;
        if !output.status.success() {
            bail!(
                "`fossil changes --differ` failed in '{}': {}",
                self.root.display(),
                stderr(&output)
            );
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut entries: Vec<PathBuf> = stdout.lines().flat_map(parse_change_paths).collect();
        entries.sort();
        entries.dedup();
        Ok(entries)
    }

    /// Publish exactly the witnessed dirty set, refusing anything outside
    /// `paths`. Returns the new revision identifier.
    pub fn publish(&self, paths: &[String], message: &str) -> Result<String> {
        if paths.is_empty() {
            bail!("archive blocked: refusing to publish with no paths");
        }
        for entry in self.dirty_entries()? {
            if !paths.iter().any(|allowed| entry.starts_with(allowed)) {
                bail!(
                    "archive blocked: refusing to publish '{}' — it is outside the resolved publication paths",
                    entry.display()
                );
            }
        }
        self.run_checked(&["addremove"])?;
        let output = self.run(&["commit", "-m", message])?;
        if !output.status.success() {
            bail!("the commit was rejected: {}", stderr(&output));
        }
        self.revision()
    }

    /// Record exactly `paths` for the next commit. `force` is reserved for
    /// the calculated managed allowlist: `fossil add` on an ignored path
    /// exits 0 after an interactive prompt, so `--force` is passed whenever
    /// the caller allows it rather than detected from a refusal.
    pub fn stage_paths(&self, paths: &[PathBuf], force: bool) -> Result<()> {
        for path in paths {
            // A managed path that does not exist in this checkout (for
            // example a Git-only hook file) has nothing to record.
            if !self.root.join(path).exists() {
                continue;
            }
            let mut args: Vec<&str> = vec!["add"];
            if force {
                args.push("--force");
            }
            args.push("--");
            let path = path.to_string_lossy();
            args.push(&path);
            self.run_checked(&args)?;
        }
        Ok(())
    }

    /// Remove exactly `paths` from the current revision while keeping the
    /// working-tree files. `--soft` is pinned so the user's `mv-rm-files`
    /// setting cannot delete them.
    pub fn remove_from_index(&self, paths: &[PathBuf]) -> Result<()> {
        for path in paths {
            if !self.root.join(path).exists() {
                continue;
            }
            let path = path.to_string_lossy();
            self.run_checked(&["rm", "--soft", "--", &path])?;
        }
        Ok(())
    }

    /// Count the reachable revisions that touched each path.
    pub fn history_occurrences(&self, paths: &[String]) -> Result<Vec<(String, usize)>> {
        let mut out = Vec::new();
        for path in paths {
            let output = self.run(&["finfo", "-b", path])?;
            // A path that never existed in history is not a violation.
            if !output.status.success() {
                continue;
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

    /// Whether `revision` names a check-in.
    pub fn commit_exists(&self, revision: &str) -> Result<bool> {
        let output = self.run(&["info", revision])?;
        Ok(output.status.success())
    }

    /// The comment of `revision`, or `None` when it does not exist.
    pub fn commit_message(&self, revision: &str) -> Result<Option<String>> {
        let output = self.run(&["info", revision])?;
        if !output.status.success() {
            return Ok(None);
        }
        let text = String::from_utf8_lossy(&output.stdout);
        let mut message = String::new();
        let mut in_comment = false;
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("comment:") {
                in_comment = true;
                message.push_str(rest.trim_start());
                continue;
            }
            if in_comment {
                if line.starts_with(char::is_whitespace) {
                    message.push('\n');
                    message.push_str(line.trim_start());
                } else {
                    break;
                }
            }
        }
        Ok(Some(message))
    }

    /// Fossil exposes check-in ancestry through `info`; existence is the
    /// available proof for this gate.
    pub fn is_ancestor(&self, revision: &str) -> Result<bool> {
        self.commit_exists(revision)
    }

    /// Whether the checkout holds pending changes. Fossil has no index; the
    /// clean-state preflight bounds the pending set to the staged writes.
    pub fn has_staged_changes(&self) -> Result<bool> {
        Ok(!self.dirty_entries()?.is_empty())
    }

    /// Read a file from the current checkout revision. Returns `None` when the
    /// path is absent.
    pub fn revision_file(&self, path: &str) -> Result<Option<String>> {
        let output = self.run(&["cat", path, "-r", "current"])?;
        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()))
        } else {
            Ok(None)
        }
    }

    pub fn commit_selected(&self, message: &str, paths: &[String]) -> Result<String> {
        if paths.is_empty() {
            bail!("explicit paths are required");
        }
        let conflicts = self.run(&["changes", "--classify"])?;
        if !conflicts.status.success()
            || String::from_utf8_lossy(&conflicts.stdout)
                .lines()
                .any(|line| line.starts_with("CONFLICT") || line.starts_with("MERGED"))
        {
            bail!("cannot isolate selected paths while Fossil has pending merge changes");
        }
        let tracked = self.run(&["ls"])?;
        if !tracked.status.success() {
            bail!("cannot inspect tracked Fossil paths");
        }
        let tracked = String::from_utf8_lossy(&tracked.stdout);
        for path in paths {
            if !self.root.join(path).exists()
                && !tracked
                    .lines()
                    .any(|entry| Path::new(entry) == Path::new(path))
            {
                bail!("unknown absent explicit path '{path}'");
            }
        }
        for entry in tracked.lines() {
            if paths.iter().any(|path| Path::new(entry).starts_with(path))
                && !self.root.join(entry).exists()
            {
                self.run_checked(&["rm", "--soft", "--", entry])?;
            }
        }
        for path in paths {
            if self.root.join(path).exists() {
                self.run_checked(&["add", "--", path])?;
            }
        }
        let relative: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
        self.commit_paths(message, &relative)?;
        self.revision()
    }

    /// Commit exactly `paths` (already recorded), leaving other pending
    /// changes untouched. Returns an error when none of them changed.
    pub fn commit_paths(&self, message: &str, paths: &[PathBuf]) -> Result<()> {
        let mut args: Vec<String> = vec![
            "commit".to_string(),
            "-m".to_string(),
            message.to_string(),
            "--".to_string(),
        ];
        for path in paths {
            args.push(path.to_string_lossy().into_owned());
        }
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let output = self.run(&refs)?;
        if !output.status.success() {
            bail!("the commit was rejected: {}", stderr(&output));
        }
        Ok(())
    }

    /// Commit the pending changes.
    pub fn commit_staged(&self, message: &str) -> Result<()> {
        let output = self.run(&["commit", "-m", message])?;
        if !output.status.success() {
            bail!("the commit was rejected: {}", stderr(&output));
        }
        Ok(())
    }

    /// The oldest reachable revision that touched `path`.
    pub fn first_touching_revision(&self, path: &str) -> Result<Option<String>> {
        let output = self.run(&[
            "timeline",
            "-n",
            "0",
            "-t",
            "ci",
            "-F",
            "%H",
            "--reverse",
            "ancestors",
            "current",
            "-p",
            path,
        ])?;
        if !output.status.success() {
            return Ok(None);
        }
        Ok(first_revision_line(&String::from_utf8_lossy(
            &output.stdout,
        )))
    }

    /// The first parent of `revision`, or `None` for a root revision.
    pub fn parent_revision(&self, revision: &str) -> Result<Option<String>> {
        let output = self.run(&["info", revision])?;
        if !output.status.success() {
            return Ok(None);
        }
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Some(parent) = line.trim().strip_prefix("parent:") {
                // `fossil info` prints `<hash> <date>`; the revision is the
                // first token. `merged-from:` is never the first parent.
                return Ok(parent.split_whitespace().next().map(ToString::to_string));
            }
        }
        Ok(None)
    }

    /// Tracked files under `root` in `revision`, with their entry kind.
    ///
    /// The check-in manifest is the read: `fossil ls` reports neither the
    /// executable bit nor a symlink, while each `F` card carries both.
    pub fn revision_files(
        &self,
        revision: &str,
        root: &str,
    ) -> Result<Vec<crate::scm::RevisionEntry>> {
        let output = self.run(&["artifact", revision_arg(revision)])?;
        if !output.status.success() {
            bail!(
                "read the Fossil manifest at revision '{revision}' in '{}': {}",
                self.root.display(),
                stderr(&output)
            );
        }
        let manifest = String::from_utf8_lossy(&output.stdout);
        Ok(parse_manifest(&manifest, revision)?
            .into_iter()
            .filter(|entry| root.is_empty() || under_root(&entry.path, root))
            .collect())
    }

    /// One path's bytes at `revision`, or `None` when the checkout holds no
    /// blob for it.
    pub fn revision_blob(&self, revision: &str, path: &str) -> Result<Option<Vec<u8>>> {
        let output = self.run(&["cat", "-r", revision_arg(revision), path])?;
        Ok(output.status.success().then_some(output.stdout))
    }

    /// Changed paths under `root` from `base` to the current revision.
    pub fn changed_paths(&self, base: Option<&str>, root: &str) -> Result<Vec<String>> {
        let mut paths = Vec::new();
        if let Some(base) = base {
            let output = self.run(&["diff", "--from", base, "--to", "current", "--brief"])?;
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    for path in parse_change_paths(line) {
                        let path = path.to_string_lossy().replace('\\', "/");
                        if under_root(&path, root) && path.ends_with(".rs") {
                            paths.push(path);
                        }
                    }
                }
            }
        } else {
            let output = self.run(&["ls"])?;
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if under_root(line, root) && line.ends_with(".rs") {
                    paths.push(line.to_string());
                }
            }
        }
        let extras = self.run(&["extras"])?;
        for line in String::from_utf8_lossy(&extras.stdout).lines() {
            if under_root(line, root) && line.ends_with(".rs") {
                paths.push(line.to_string());
            }
        }
        paths.sort();
        paths.dedup();
        Ok(paths)
    }

    /// Whether `path` is present in the current revision.
    pub fn revision_contains(&self, path: &str) -> Result<bool> {
        let output = self.run(&["cat", path])?;
        Ok(output.status.success())
    }

    fn run(&self, args: &[&str]) -> Result<Output> {
        Command::new("fossil")
            .args(args)
            .current_dir(&self.root)
            .output()
            .with_context(|| {
                format!(
                    "run `fossil {}` in '{}'",
                    args.join(" "),
                    self.root.display()
                )
            })
    }

    fn run_checked(&self, args: &[&str]) -> Result<()> {
        let output = self.run(args)?;
        if !output.status.success() {
            bail!(
                "`fossil {}` failed in '{}': {}",
                args.join(" "),
                self.root.display(),
                stderr(&output)
            );
        }
        Ok(())
    }
}

/// Parse one `fossil changes --differ` line into its paths. A hard rename
/// prints `<STATUS> old -> new`; both sides are covered so a move out of a
/// production root is still a dirty production path.
fn parse_change_paths(line: &str) -> Vec<PathBuf> {
    let trimmed = line.trim_end();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let Some((_, rest)) = trimmed.split_once(char::is_whitespace) else {
        return Vec::new();
    };
    let rest = rest.trim_start();
    if let Some((source, destination)) = rest.split_once("->") {
        [source.trim(), destination.trim()]
            .into_iter()
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .collect()
    } else if rest.is_empty() {
        Vec::new()
    } else {
        vec![PathBuf::from(rest)]
    }
}

/// Fossil calls the checked-out head `current`; the gate names it `HEAD`
/// because that is the SCM-neutral contract.
fn revision_arg(revision: &str) -> &str {
    if revision == "HEAD" {
        "current"
    } else {
        revision
    }
}

/// Fossil's timeline ends with `+++ end of timeline (n) +++`; only a real
/// hash is a revision. With no match the command still exits successfully.
fn first_revision_line(output: &str) -> Option<String> {
    output
        .lines()
        .map(str::trim)
        .find(|line| {
            !line.is_empty()
                && !line.starts_with("+++")
                && line.chars().all(|character| character.is_ascii_hexdigit())
        })
        .map(ToString::to_string)
}

fn under_root(path: &str, root: &str) -> bool {
    path == root
        || path
            .strip_prefix(root)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// Parse the `F` cards of a Fossil check-in manifest. Each card is
/// `F <path> <40-hex-or-64-hex blob> [flags]`; `x` marks an executable
/// regular file and `l` a symlink. A manifest is the recorded revision, so a
/// malformed card refuses instead of being skipped.
fn parse_manifest(manifest: &str, revision: &str) -> Result<Vec<crate::scm::RevisionEntry>> {
    let mut entries = Vec::new();
    for line in manifest.lines() {
        let Some(rest) = line.strip_prefix("F ") else {
            continue;
        };
        let mut fields = rest.split_whitespace();
        let Some(raw_path) = fields.next() else {
            bail!(
                "the Fossil manifest at revision '{revision}' has an F line without a path: '{line}'"
            );
        };
        let Some(hash) = fields.next() else {
            bail!(
                "the Fossil manifest at revision '{revision}' has an F line without a blob hash: '{line}'"
            );
        };
        if !is_blob_hash(hash) {
            bail!(
                "the Fossil manifest at revision '{revision}' has an F line without a 40-hex or 64-hex blob hash: '{line}'"
            );
        }
        let mut kind = crate::scm::EntryKind::Regular;
        let mut executable = false;
        for flag in fields {
            match flag {
                "x" => executable = true,
                "l" => kind = crate::scm::EntryKind::Symlink,
                other => bail!(
                    "the Fossil manifest at revision '{revision}' has an F line with an unknown flag '{other}': '{line}'"
                ),
            }
        }
        entries.push(crate::scm::RevisionEntry {
            path: decode_manifest_path(raw_path),
            kind,
            executable,
            gitlink: None,
        });
    }
    Ok(entries)
}

/// A Fossil manifest hash is a 40-hex SHA-1 or a 64-hex SHA-256 artifact id.
fn is_blob_hash(value: &str) -> bool {
    (value.len() == 40 || value.len() == 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// Fossil escapes a space in a manifest path as `\s`.
fn decode_manifest_path(raw: &str) -> String {
    raw.replace("\\s", " ")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_paths_include_tracked_and_extra_files() {
        use std::process::Command;
        let available = Command::new("fossil")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
        if !available {
            return;
        }
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();
        let repository = dir.path().join("fossil-repo");
        let run = |args: &[&str]| {
            let output = Command::new("fossil")
                .args(args)
                .current_dir(root)
                .env("FOSSIL_USER", "fixture")
                .env("USER", "fixture")
                .output()
                .expect("run fossil");
            assert!(
                output.status.success(),
                "fossil {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        run(&["init", repository.to_str().unwrap()]);
        run(&["open", repository.to_str().unwrap()]);
        std::fs::write(root.join("tracked.txt"), "tracked\n").unwrap();
        run(&["add", "tracked.txt"]);
        run(&["commit", "-m", "fixture"]);
        std::fs::write(root.join("extra.txt"), "extra\n").unwrap();
        let checkout = FossilCheckout::at(root);
        let paths: Vec<String> = checkout
            .content_paths()
            .unwrap()
            .into_iter()
            .map(|entry| entry.path.to_string_lossy().replace('\\', "/"))
            .collect();
        assert!(paths.contains(&"tracked.txt".to_string()), "{paths:?}");
        assert!(paths.contains(&"extra.txt".to_string()), "{paths:?}");
    }

    #[test]
    fn parses_status_lines_including_extra_and_renames() {
        assert_eq!(
            parse_change_paths("ADDED      givn/specs/a.feature"),
            vec![PathBuf::from("givn/specs/a.feature")]
        );
        assert_eq!(
            parse_change_paths("EXTRA      notes.txt"),
            vec![PathBuf::from("notes.txt")]
        );
        assert_eq!(
            parse_change_paths("MISSING    givn/changes/x/proposal.md"),
            vec![PathBuf::from("givn/changes/x/proposal.md")]
        );
        assert_eq!(
            parse_change_paths("RENAMED    old.txt  ->  new.txt"),
            vec![PathBuf::from("old.txt"), PathBuf::from("new.txt")]
        );
        assert!(parse_change_paths("").is_empty());
        assert!(parse_change_paths("EDITED").is_empty());
    }

    #[test]
    fn parses_manifest_entry_kinds_and_decodes_a_space() {
        let manifest = "\
C abcdef\n\
P 0123456789abcdef0123456789abcdef01234567\n\
F dir/file.txt 0123456789abcdef0123456789abcdef01234567\n\
F a\\sb.txt fedcba9876543210fedcba9876543210fedcba98\n\
F run.sh 1111111111111111111111111111111111111111111111111111111111111111 x\n\
F link 2222222222222222222222222222222222222222222222222222222222222222 l\n\
F sub 3333333333333333333333333333333333333333 l x\n\
";
        let entries = parse_manifest(manifest, "current").unwrap();
        assert_eq!(entries.len(), 5, "{entries:?}");
        assert_eq!(entries[0].path, "dir/file.txt");
        assert_eq!(entries[0].kind, crate::scm::EntryKind::Regular);
        assert!(!entries[0].executable);
        assert_eq!(entries[1].path, "a b.txt");
        assert_eq!(entries[2].path, "run.sh");
        assert!(entries[2].executable);
        assert_eq!(entries[3].kind, crate::scm::EntryKind::Symlink);
        assert_eq!(entries[4].kind, crate::scm::EntryKind::Symlink);
        assert!(entries[4].executable);
    }

    #[test]
    fn refuses_a_manifest_line_without_a_blob_hash() {
        let manifest = "F dir/file.txt not-a-hash\n";
        let error = parse_manifest(manifest, "abc123")
            .expect_err("a short hash is refused")
            .to_string();
        assert!(error.contains("abc123"), "{error}");
        assert!(error.contains("F dir/file.txt not-a-hash"), "{error}");
        assert!(error.contains("40-hex or 64-hex"), "{error}");
    }

    #[test]
    fn refuses_a_manifest_line_with_an_unknown_flag() {
        let manifest = "F run.sh 0123456789abcdef0123456789abcdef01234567 z\n";
        let error = parse_manifest(manifest, "current")
            .expect_err("an unknown flag is refused")
            .to_string();
        assert!(error.contains("unknown flag 'z'"), "{error}");
        assert!(error.contains("current"), "{error}");
    }

    #[test]
    fn a_timeline_end_marker_is_not_a_revision() {
        assert_eq!(first_revision_line("+++ end of timeline (0) +++\n"), None);
        assert_eq!(
            first_revision_line("abc123\ndef456\n+++ end of timeline (2) +++\n"),
            Some("abc123".to_string())
        );
    }
}
