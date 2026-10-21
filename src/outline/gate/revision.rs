//! Revision-tree export into a temporary project root.
//!
//! Both sides are materialized from the SCM backend: the gate never analyzes
//! the working copy. Symlinks and submodules are recorded but never written;
//! the post-analysis guard fails closed when a changed path is one of them.

use super::base::path_failure;
use super::{GateFailure, Side};
use crate::scm::{Backend, EntryKind};
use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

pub struct RevisionTree {
    pub root: tempfile::TempDir,
    pub entries: Vec<(String, EntryKind)>,
    pub unreadable: Vec<String>,
    pub failures: Vec<GateFailure>,
}

impl RevisionTree {
    pub fn kind(&self, path: &str) -> Option<EntryKind> {
        self.entries
            .iter()
            .find(|(entry, _)| entry == path)
            .map(|(_, kind)| *kind)
    }
}

pub fn empty(_side: Side) -> Result<RevisionTree> {
    Ok(RevisionTree {
        root: tempfile::tempdir().context("create revision-tree temp directory")?,
        entries: Vec::new(),
        unreadable: Vec::new(),
        failures: Vec::new(),
    })
}

pub fn materialize(
    backend: &Backend,
    revision: &str,
    side: Side,
    roots: &[String],
) -> Result<RevisionTree> {
    let root = tempfile::tempdir().context("create revision-tree temp directory")?;
    let mut failures = Vec::new();
    let mut entries: BTreeMap<String, EntryKind> = BTreeMap::new();

    for production_root in roots {
        let files = match backend.revision_files(revision, production_root) {
            Ok(files) => files,
            Err(error) => {
                failures.push(path_failure(&error, side));
                return Ok(RevisionTree {
                    root,
                    entries: Vec::new(),
                    unreadable: Vec::new(),
                    failures,
                });
            }
        };
        for (path, kind) in files {
            let path = path.replace('\\', "/");
            if let Err(failure) = validate_path(&path, roots, side) {
                failures.push(failure);
                continue;
            }
            entries.insert(path, kind);
        }
    }

    let regular: Vec<String> = entries
        .iter()
        .filter(|(_, kind)| **kind == EntryKind::Regular)
        .map(|(path, _)| path.clone())
        .collect();
    let blobs = match backend.revision_blobs(revision, &regular) {
        Ok(blobs) => blobs,
        Err(error) => {
            failures.push(GateFailure::AnalysisFailed {
                side,
                path: "revision tree".to_string(),
                detail: error.to_string(),
            });
            return Ok(RevisionTree {
                root,
                entries: entries.into_iter().collect(),
                unreadable: Vec::new(),
                failures,
            });
        }
    };

    let mut written = BTreeSet::new();
    for (path, bytes) in blobs {
        if !entries.contains_key(&path) {
            continue;
        }
        let target = root.path().join(&path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create revision parent for {path}"))?;
        }
        std::fs::write(&target, bytes).with_context(|| format!("write revision file {path}"))?;
        written.insert(path);
    }
    let unreadable = regular
        .into_iter()
        .filter(|path| !written.contains(path))
        .collect();

    Ok(RevisionTree {
        root,
        entries: entries.into_iter().collect(),
        unreadable,
        failures,
    })
}

fn validate_path(path: &str, roots: &[String], side: Side) -> std::result::Result<(), GateFailure> {
    if path.contains('\n') || path.contains('\r') {
        return Err(GateFailure::NewlinePath(path.to_string()));
    }
    let parsed = Path::new(path);
    if parsed.is_absolute() {
        return Err(GateFailure::SkippedEntryDependency {
            side,
            path: path.to_string(),
            kind: "absolute path".to_string(),
        });
    }
    if parsed
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(GateFailure::SkippedEntryDependency {
            side,
            path: path.to_string(),
            kind: "parent-directory path".to_string(),
        });
    }
    if !super::head::under_roots(path, roots) {
        return Err(GateFailure::SkippedEntryDependency {
            side,
            path: path.to_string(),
            kind: "path outside the production roots".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roots() -> Vec<String> {
        vec!["src".to_string()]
    }

    #[test]
    fn revision_paths_must_stay_inside_the_production_roots() {
        assert!(validate_path("src/engine.rs", &roots(), Side::Head).is_ok());
        assert!(matches!(
            validate_path("src/../etc/passwd", &roots(), Side::Head),
            Err(GateFailure::SkippedEntryDependency { .. })
        ));
        assert!(matches!(
            validate_path("/etc/passwd", &roots(), Side::Head),
            Err(GateFailure::SkippedEntryDependency { .. })
        ));
        assert!(matches!(
            validate_path("docs/engine.rs", &roots(), Side::Head),
            Err(GateFailure::SkippedEntryDependency { .. })
        ));
        assert!(matches!(
            validate_path("src/engine\n.rs", &roots(), Side::Head),
            Err(GateFailure::NewlinePath(_))
        ));
    }
}
