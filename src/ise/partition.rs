//! Placement of one revision's tracked paths.
//!
//! The partition owns refusals 5 to 7: every declared retained entry must
//! match the revision, no replacement may be retained, and every replacement
//! must lie under a source root. Every check precedes the first write.

use std::collections::BTreeSet;

use anyhow::{Result, bail};

use crate::config::declaration::{Role, SpecificationDeclaration, normalize_project_path};
use crate::e2e_boundary::Glob;
use crate::scm::RevisionEntry;

use super::{Counts, ReplaceRequest};

/// Every tracked path at the revision, placed by the declaration.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Partition {
    retained: Vec<String>,
    kept_source: Vec<String>,
    replaced: Vec<String>,
    outside: Vec<String>,
    not_replaced: usize,
}

impl Partition {
    pub(super) fn counts(&self) -> Counts {
        Counts {
            retained: self.retained.len(),
            kept_source: self.kept_source.len(),
            replaced: self.replaced.len(),
            outside_declaration: self.outside.len(),
            not_replaced: self.not_replaced,
        }
    }

    /// The sorted union of retained and kept source paths: every file the
    /// trial copies from the revision.
    pub(super) fn copy_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self
            .retained
            .iter()
            .chain(&self.kept_source)
            .cloned()
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }

    /// The sorted retained paths: the trial's recorded retained set.
    pub(super) fn retained_paths(&self) -> &[String] {
        &self.retained
    }

    /// The sorted tracked paths this replacement removes.
    pub(super) fn replaced_paths(&self) -> &[String] {
        &self.replaced
    }
}

/// Place every tracked path at the revision. A retained role is tested first,
/// so a retained file under a source root stays in the trial under every form.
/// Every replacement refusal runs before the trial is created.
pub(super) fn partition(
    declaration: &SpecificationDeclaration,
    entries: &[RevisionEntry],
    request: &ReplaceRequest,
    revision: &str,
) -> Result<Partition> {
    let roles = retained_roles(declaration);
    require_declared_retained_entries(&roles, entries, revision)?;
    let source_roots: Vec<Glob> = declaration
        .source_roots()
        .iter()
        .map(|root| Glob::compile(root))
        .collect();
    let tracked: BTreeSet<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();

    let mut retained: Vec<String> = entries
        .iter()
        .filter(|entry| retained_role(&entry.path, &roles).is_some())
        .map(|entry| entry.path.clone())
        .collect();
    retained.sort();

    let candidates: BTreeSet<String> = if request.full {
        entries
            .iter()
            .filter(|entry| path_or_parent_matches(&source_roots, &entry.path))
            .map(|entry| entry.path.clone())
            .collect()
    } else {
        let candidates = named_candidates(&tracked, &request.paths)?;
        require_replaceable_paths(&candidates, &roles, &source_roots, revision)?;
        candidates
    };
    let retained_set: BTreeSet<&str> = retained.iter().map(String::as_str).collect();
    let mut replaced: Vec<String> = candidates
        .into_iter()
        .filter(|path| !retained_set.contains(path.as_str()))
        .collect();
    replaced.sort();
    let replaced_set: BTreeSet<&str> = replaced.iter().map(String::as_str).collect();

    let mut kept_source: Vec<String> = entries
        .iter()
        .map(|entry| &entry.path)
        .filter(|path| {
            !retained_set.contains(path.as_str())
                && !replaced_set.contains(path.as_str())
                && path_or_parent_matches(&source_roots, path)
        })
        .cloned()
        .collect();
    kept_source.sort();

    let mut outside: Vec<String> = entries
        .iter()
        .map(|entry| &entry.path)
        .filter(|path| {
            !retained_set.contains(path.as_str())
                && !replaced_set.contains(path.as_str())
                && !path_or_parent_matches(&source_roots, path)
        })
        .cloned()
        .collect();
    outside.sort();

    let not_replaced = direct_siblings(&tracked, &replaced_set);
    Ok(Partition {
        retained,
        kept_source,
        replaced,
        outside,
        not_replaced,
    })
}

/// The `.feature` paths the `behavior` role matches at the revision, sorted:
/// the files whose selected `@e2e` scenarios the record digests.
pub(super) fn behavior_features(
    declaration: &SpecificationDeclaration,
    paths: &[&str],
) -> Vec<String> {
    let globs: Vec<Glob> = declaration
        .roles()
        .into_iter()
        .filter(|(role, _)| *role == Role::Behavior)
        .flat_map(|(_, patterns)| patterns.iter().map(|pattern| Glob::compile(pattern)))
        .collect();
    let mut features: Vec<String> = paths
        .iter()
        .filter(|path| path.ends_with(".feature") && path_or_parent_matches(&globs, path))
        .map(|path| (*path).to_string())
        .collect();
    features.sort();
    features.dedup();
    features
}

/// The retained role patterns in declaration order, each with its declared
/// text. The first matching role wins, so the order is part of the contract.
fn retained_roles(declaration: &SpecificationDeclaration) -> Vec<(Role, &str, Glob)> {
    declaration
        .roles()
        .into_iter()
        .flat_map(|(role, patterns)| {
            patterns
                .iter()
                .map(move |pattern| (role, pattern.as_str(), Glob::compile(pattern)))
        })
        .collect()
}

fn retained_role(path: &str, roles: &[(Role, &str, Glob)]) -> Option<Role> {
    roles
        .iter()
        .find(|(_, _, glob)| glob_matches_path_or_parent(glob, path))
        .map(|(role, _, _)| *role)
}

/// The `--path` values that resolve to tracked content. A tracked file
/// contributes itself, a directory contributes every tracked file beneath it,
/// a glob-shaped value contributes nothing, and repeats collapse. A value the
/// declaration's normalizer rejects is outside the project and refuses.
fn named_candidates(tracked: &BTreeSet<&str>, values: &[String]) -> Result<BTreeSet<String>> {
    let mut directories = BTreeSet::new();
    for path in tracked {
        for (index, _) in path.match_indices('/') {
            directories.insert(path[..index].to_string());
        }
    }

    let mut candidates = BTreeSet::new();
    for value in values {
        let Some(value) = normalize_project_path(value) else {
            bail!(
                "'{value}' is outside the project; a replacement path is a project-relative tracked file or directory"
            );
        };
        if tracked.contains(value.as_str()) {
            candidates.insert(value);
            continue;
        }
        if directories.contains(&value) {
            let prefix = format!("{value}/");
            candidates.extend(
                tracked
                    .iter()
                    .filter(|path| path.starts_with(&prefix))
                    .map(|path| (*path).to_string()),
            );
        }
    }
    Ok(candidates)
}

/// Refusal 5: every declared retained entry must match a tracked path or a
/// derived directory at the revision; an entry that matches nothing names
/// itself and the revision.
fn require_declared_retained_entries(
    roles: &[(Role, &str, Glob)],
    entries: &[RevisionEntry],
    revision: &str,
) -> Result<()> {
    for (role, pattern, glob) in roles {
        let matches = entries
            .iter()
            .any(|entry| glob_matches_path_or_parent(glob, &entry.path));
        if !matches {
            bail!(
                "the declared {} entry '{pattern}' matches nothing at revision '{revision}'",
                role.as_str()
            );
        }
    }
    Ok(())
}

/// Refusals 6 and 7: no candidate may be retained, and every candidate must
/// lie under a declared source root. The retained test runs first over the
/// whole candidate set, so the reported condition is the role, not the root.
fn require_replaceable_paths(
    candidates: &BTreeSet<String>,
    roles: &[(Role, &str, Glob)],
    source_roots: &[Glob],
    revision: &str,
) -> Result<()> {
    for path in candidates {
        if let Some(role) = retained_role(path, roles) {
            bail!(
                "'{path}' is retained by the {} role at revision '{revision}'; a retained path is never replaced",
                role.as_str()
            );
        }
    }
    for path in candidates {
        if !path_or_parent_matches(source_roots, path) {
            bail!("'{path}' is outside every declared source root at revision '{revision}'");
        }
    }
    Ok(())
}

/// Whether a role or source pattern matches the path or every tracked file
/// under a directory it matches.
fn path_or_parent_matches(globs: &[Glob], path: &str) -> bool {
    globs
        .iter()
        .any(|glob| glob_matches_path_or_parent(glob, path))
}

fn glob_matches_path_or_parent(glob: &Glob, path: &str) -> bool {
    glob.matches(path)
        || path
            .match_indices('/')
            .any(|(index, _)| glob.matches(&path[..index]))
}

/// The direct tracked files in the replaced files' parent directories that
/// were not themselves replaced.
fn direct_siblings(tracked: &BTreeSet<&str>, replaced: &BTreeSet<&str>) -> usize {
    let mut parents = BTreeSet::new();
    for path in replaced {
        if let Some((parent, _)) = path.rsplit_once('/') {
            parents.insert(parent);
        }
    }
    tracked
        .iter()
        .filter(|path| {
            !replaced.contains(**path)
                && path
                    .rsplit_once('/')
                    .is_some_and(|(parent, _)| parents.contains(parent))
        })
        .count()
}
