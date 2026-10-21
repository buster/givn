//! Committed-HEAD selection, the dirty-production guard, and changed paths.

use super::GateRequest;
use crate::scm::Backend;
use anyhow::Result;
use std::collections::BTreeSet;

/// Working-tree production paths that prevent analysis. Non-production paths
/// (`review.md`, rationales, docs, `givn/`) are deliberately ignored.
pub fn dirty_production(backend: &Backend, request: &GateRequest) -> Result<Vec<String>> {
    let roots = request.root_strings();
    let mut paths = BTreeSet::new();
    for path in backend.dirty_entries()? {
        let path = path.to_string_lossy().replace('\\', "/");
        if under_roots(&path, &roots) {
            paths.insert(path);
        }
    }
    Ok(paths.into_iter().collect())
}

/// Changed production paths from `base` to committed `HEAD`. A deleted path
/// stays in the set; it has no head entry.
pub fn changed_production_paths(
    backend: &Backend,
    base: Option<&str>,
    roots: &[String],
) -> Result<BTreeSet<String>> {
    let mut paths = BTreeSet::new();
    for root in roots {
        for path in backend.changed_paths(base, root)? {
            let path = path.replace('\\', "/");
            if under_roots(&path, roots) {
                paths.insert(path);
            }
        }
    }
    Ok(paths)
}

pub fn under_roots(path: &str, roots: &[String]) -> bool {
    roots.iter().any(|root| {
        path == root
            || path
                .strip_prefix(root)
                .is_some_and(|rest| rest.starts_with('/'))
    })
}
