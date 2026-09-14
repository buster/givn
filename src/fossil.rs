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
        let mut entries: Vec<PathBuf> = stdout.lines().filter_map(parse_change_line).collect();
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

    /// Whether the checkout holds pending changes. Fossil has no index; the
    /// clean-state preflight bounds the pending set to the staged writes.
    pub fn has_staged_changes(&self) -> Result<bool> {
        Ok(!self.dirty_entries()?.is_empty())
    }

    /// Commit the pending changes.
    pub fn commit_staged(&self, message: &str) -> Result<()> {
        let output = self.run(&["commit", "-m", message])?;
        if !output.status.success() {
            bail!("the commit was rejected: {}", stderr(&output));
        }
        Ok(())
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

/// Parse one `fossil changes --differ` line into its path. A hard rename
/// prints `<STATUS> old -> new`, so the destination wins.
fn parse_change_line(line: &str) -> Option<PathBuf> {
    let trimmed = line.trim_end();
    if trimmed.is_empty() {
        return None;
    }
    let rest = trimmed.split_once(char::is_whitespace)?.1.trim_start();
    if rest.is_empty() {
        return None;
    }
    let path = match rest.split_once("->") {
        Some((_, destination)) => destination.trim(),
        None => rest,
    };
    if path.is_empty() {
        return None;
    }
    Some(PathBuf::from(path))
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_lines_including_extra_and_renames() {
        assert_eq!(
            parse_change_line("ADDED      givn/specs/a.feature"),
            Some(PathBuf::from("givn/specs/a.feature"))
        );
        assert_eq!(
            parse_change_line("EXTRA      notes.txt"),
            Some(PathBuf::from("notes.txt"))
        );
        assert_eq!(
            parse_change_line("MISSING    givn/changes/x/proposal.md"),
            Some(PathBuf::from("givn/changes/x/proposal.md"))
        );
        assert_eq!(
            parse_change_line("RENAMED    old.txt  ->  new.txt"),
            Some(PathBuf::from("new.txt"))
        );
        assert_eq!(parse_change_line(""), None);
        assert_eq!(parse_change_line("EDITED"), None);
    }
}
