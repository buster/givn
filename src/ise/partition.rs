//! Placement of one revision's tracked paths.
//!
//! The partition owns refusals 5 to 7: every declared Specification entry
//! must match the revision, no replacement may be retained, and every
//! replacement must lie under a source root. Every check precedes the first
//! write.

use std::collections::BTreeSet;

use anyhow::{Result, bail};

use crate::config::declaration::{
    DECLARATION_PATH, Role, SpecificationDeclaration, normalize_project_path,
};
use crate::e2e_boundary::{Glob, glob_matches_path_or_parent};
use crate::scm::RevisionEntry;

use super::{Counts, RebuildRequest, RebuildScope};

/// Every tracked path at the revision, placed by the declaration.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Partition {
    retained: Vec<String>,
    kept_source: Vec<String>,
    replaced: Vec<String>,
    outside: Vec<String>,
    frozen: Vec<String>,
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

    /// The sorted frozen paths: the resolved `e2e` package minus the
    /// declaration. The trial never copies one.
    pub(super) fn frozen_paths(&self) -> &[String] {
        &self.frozen
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
    request: &RebuildRequest,
    revision: &str,
) -> Result<Partition> {
    let roles = retained_roles(declaration);
    require_declaration_entries(&roles, entries, revision)?;
    let source_roots: Vec<Glob> = declaration
        .source_roots()
        .iter()
        .map(|root| Glob::compile(root))
        .collect();
    let tracked: BTreeSet<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();

    let revision_paths: Vec<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();
    let frozen: Vec<String> = crate::e2e_boundary::e2e_file_set(declaration, &revision_paths)
        .into_iter()
        .filter(|path| path != DECLARATION_PATH)
        .collect();
    let frozen_set: BTreeSet<&str> = frozen.iter().map(String::as_str).collect();

    let mut retained_paths: BTreeSet<String> = BTreeSet::new();
    retained_paths.insert(DECLARATION_PATH.to_string());
    retained_paths.extend(
        entries
            .iter()
            .filter(|entry| retained_role(&entry.path, &roles).is_some())
            .map(|entry| entry.path.clone()),
    );
    retained_paths.retain(|path| !frozen_set.contains(path.as_str()));
    let retained: Vec<String> = retained_paths.into_iter().collect();

    let candidates: BTreeSet<String> = match &request.scope {
        RebuildScope::All => entries
            .iter()
            .filter(|entry| path_or_parent_matches(&source_roots, &entry.path))
            .map(|entry| entry.path.clone())
            .collect(),
        RebuildScope::Paths(paths) => {
            let candidates = named_candidates(&tracked, paths)?;
            require_replaceable_paths(&candidates, &roles, &source_roots, revision)?;
            candidates
        }
    };
    let retained_set: BTreeSet<&str> = retained.iter().map(String::as_str).collect();
    let mut replaced: Vec<String> = candidates
        .into_iter()
        .filter(|path| !retained_set.contains(path.as_str()) && !frozen_set.contains(path.as_str()))
        .collect();
    replaced.sort();
    let replaced_set: BTreeSet<&str> = replaced.iter().map(String::as_str).collect();

    let mut kept_source: Vec<String> = entries
        .iter()
        .map(|entry| &entry.path)
        .filter(|path| {
            !retained_set.contains(path.as_str())
                && !replaced_set.contains(path.as_str())
                && !frozen_set.contains(path.as_str())
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
                && !frozen_set.contains(path.as_str())
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
        frozen,
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

/// Refusal 5: every declared Specification entry must match a tracked path or
/// a derived directory at the revision; an entry that matches nothing names
/// itself and the revision.
fn require_declaration_entries(
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
                "the declared Specification entry '{pattern}' in the {} role matches nothing at revision '{revision}'",
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
        if path == DECLARATION_PATH {
            bail!(
                "'{path}' is a Specification file at revision '{revision}'; the declaration is never removed"
            );
        }
    }
    for path in candidates {
        if let Some(role) = retained_role(path, roles) {
            bail!(
                "'{path}' is a Specification file through the {} role at revision '{revision}'; a Specification file is never removed",
                role.as_str()
            );
        }
    }
    for path in candidates {
        if !path_or_parent_matches(source_roots, path) {
            bail!(
                "'{path}' is outside every declared source root at revision '{revision}'; rebuild-all writes a copy that removes every declared source root"
            );
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::*;
    use crate::scm::EntryKind;

    fn declaration_in(root: &std::path::Path, content: &str) -> SpecificationDeclaration {
        std::fs::create_dir_all(root.join("givn")).expect("create the fixture givn/");
        std::fs::write(root.join(DECLARATION_PATH), content)
            .expect("write the fixture declaration");
        SpecificationDeclaration::load(root)
            .expect("the fixture declaration loads")
            .expect("the fixture declaration is present")
    }

    fn full_replacement() -> RebuildRequest {
        RebuildRequest {
            at: None,
            scope: RebuildScope::All,
            into: PathBuf::from("/tmp/ise-trial"),
            manifest: None,
            dry_run: true,
        }
    }

    /// A role matching the declaration must not count it twice: the retained
    /// count, the retained path list, and the record's retained map agree.
    #[test]
    fn the_retained_count_equals_the_recorded_retained_entries() {
        let container = tempfile::TempDir::new().expect("create the fixture project");
        let declaration = declaration_in(
            container.path(),
            "behavior: []\nstructure: []\ncontracts: [\"givn/**\"]\nlimits: []\ne2e: []\nsource: [\"src/**\"]\n",
        );
        let entries = [
            RevisionEntry {
                path: DECLARATION_PATH.to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
            RevisionEntry {
                path: "givn/specs/a.md".to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
        ];
        let placement = partition(&declaration, &entries, &full_replacement(), "abc123")
            .expect("the fixture revision partitions");
        let counts = placement.counts();
        assert_eq!(
            counts.retained, 2,
            "the declaration and its role match are two entries"
        );
        assert_eq!(counts.retained, placement.retained_paths().len());
        assert!(
            placement
                .retained_paths()
                .iter()
                .any(|path| path == DECLARATION_PATH),
            "the retained set does not hold the declaration"
        );
        let blobs: BTreeMap<String, Vec<u8>> = entries
            .iter()
            .map(|entry| (entry.path.clone(), Vec::new()))
            .collect();
        let identities =
            super::super::record::identities(placement.retained_paths(), &entries, &blobs)
                .expect("every retained path is listed at the revision");
        assert_eq!(counts.retained, identities.len());
    }

    /// A frozen path is in no bucket: not retained, not kept source, not replaced,
    /// and not outside, even when it sits under a source root a full replacement
    /// covers.
    #[test]
    fn a_frozen_path_under_a_source_root_is_in_no_bucket() {
        let container = tempfile::TempDir::new().expect("create the fixture project");
        let declaration = declaration_in(
            container.path(),
            "behavior: []\nstructure: []\ncontracts: []\nlimits: []\ne2e: [\"src/e2e\"]\nsource: [\"src/**\"]\n",
        );
        let entries = [
            RevisionEntry {
                path: DECLARATION_PATH.to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
            RevisionEntry {
                path: "src/e2e/Cargo.toml".to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
            RevisionEntry {
                path: "src/e2e/tests/keys.rs".to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
            RevisionEntry {
                path: "src/kept.rs".to_string(),
                kind: EntryKind::Regular,
                executable: false,
            },
        ];
        let request = full_replacement();
        let placement = partition(&declaration, &entries, &request, "abc123")
            .expect("the fixture revision partitions");
        assert_eq!(
            placement.frozen_paths(),
            ["src/e2e/Cargo.toml", "src/e2e/tests/keys.rs"],
            "the resolved package is not the frozen set"
        );
        assert_eq!(
            placement.retained_paths(),
            [DECLARATION_PATH.to_string()],
            "the frozen package is retained"
        );
        assert_eq!(placement.copy_paths(), vec![DECLARATION_PATH.to_string()]);
        assert_eq!(
            placement.replaced_paths(),
            ["src/kept.rs".to_string()],
            "a frozen path under a source root is not replaced"
        );
        assert_eq!(
            placement.counts(),
            Counts {
                retained: 1,
                kept_source: 0,
                replaced: 1,
                outside_declaration: 0,
                not_replaced: 0,
            },
            "a frozen path deeper than a replaced file's directory is no sibling"
        );
    }
}
