//! Deterministic production-source discovery.

use super::model::{LineKind, RepoPath};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const EXCLUDED_DIRS: &[&str] = &["tests", "benches", "examples", "target"];

#[derive(Clone, Debug)]
pub struct ScannedSource {
    pub path: RepoPath,
    pub content: String,
    pub lines: Vec<LineKind>,
}

/// Discover production `.rs` files under the requested roots.
///
/// Roots are project-root-relative (already validated by the CLI). Test and
/// tooling directories are excluded wherever they occur. Paths are sorted and
/// deduplicated, so overlapping roots analyze each file once.
pub fn discover(project_root: &Path, roots: &[PathBuf]) -> Result<Vec<ScannedSource>> {
    let canonical_root = project_root
        .canonicalize()
        .with_context(|| format!("resolve project root {}", project_root.display()))?;
    let mut files = Vec::new();

    for root in roots {
        let path = project_root.join(root);
        if !path.exists() {
            continue;
        }
        walk(&canonical_root, &path, &mut files)?;
    }

    files.sort_by(|a, b| a.absolute.cmp(&b.absolute));
    files.dedup_by(|a, b| a.absolute == b.absolute);

    let mut sources = Vec::with_capacity(files.len());
    for path in files {
        let content = match std::fs::read_to_string(&path.absolute) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let lines = content.lines().map(classify_line).collect();
        let relative = path
            .absolute
            .strip_prefix(&canonical_root)
            .unwrap_or(&path.absolute);
        sources.push(ScannedSource {
            path: RepoPath::new(relative.to_string_lossy().into_owned()),
            content,
            lines,
        });
    }
    Ok(sources)
}

fn walk(project_root: &Path, dir: &Path, out: &mut Vec<ScannedSourcePath>) -> Result<()> {
    let canonical = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => return Ok(()),
    };
    if !canonical.starts_with(project_root) {
        return Ok(());
    }
    if canonical.is_file() {
        if canonical.extension().is_some_and(|ext| ext == "rs") {
            out.push(ScannedSourcePath {
                absolute: canonical,
            });
        }
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&canonical)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if EXCLUDED_DIRS.contains(&name.as_ref()) {
                continue;
            }
            walk(project_root, &path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            walk(project_root, &path, out)?;
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct ScannedSourcePath {
    absolute: PathBuf,
}

fn classify_line(line: &str) -> LineKind {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        LineKind::Blank
    } else if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        LineKind::Comment
    } else {
        LineKind::Code
    }
}

/// Count code lines in a span union.
pub fn code_lines(lines: &[LineKind], spans: &[super::model::SourceSpan], file: &RepoPath) -> u32 {
    let mut marked = vec![false; lines.len()];
    for span in spans.iter().filter(|span| &span.file == file) {
        let start = span.start_line.saturating_sub(1) as usize;
        let end = span.end_line.saturating_sub(1) as usize;
        for marked_line in marked
            .iter_mut()
            .take(end.min(lines.len().saturating_sub(1)) + 1)
            .skip(start)
        {
            *marked_line = true;
        }
    }
    marked
        .iter()
        .enumerate()
        .filter(|(index, marked)| **marked && lines.get(*index) == Some(&LineKind::Code))
        .count() as u32
}
