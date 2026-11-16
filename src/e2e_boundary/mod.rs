//! The declared `e2e` acceptance boundary.
//!
//! One resolver owns the condition text. `cli::lint` renders the placement
//! conditions as advisory findings; `cli::check` refuses the acceptance
//! conditions before any runner executes. The specification-health pass and
//! the change check read the same declaration through this module, so the
//! declared path semantics have one implementation.

mod conditions;
mod hash;
mod imports;
mod manifest;
mod paths;

pub(crate) use conditions::BoundaryCondition;
pub(crate) use hash::hash_entry;
pub(crate) use paths::{Glob, ProjectPath, glob_matches_path_or_parent, walk_project};

use crate::config::declaration::{Role, SpecificationDeclaration, normalize_project_path};
use imports::import_edges;
use manifest::{
    CargoPackage, dependency_edges, is_cargo_manifest, nearest_manifest, parent_directory,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

/// The resolved declared boundary: the matched paths that resolved a package,
/// the matched paths that resolved none, and the conditions that pass found.
#[derive(Debug, Default)]
pub(crate) struct E2eBoundary {
    resolved: Vec<String>,
    import_paths: Vec<String>,
    conditions: Vec<BoundaryCondition>,
}

impl E2eBoundary {
    /// Resolve the declared `e2e` role against the project tree.
    pub(crate) fn resolve(
        project_root: &Path,
        declaration: &SpecificationDeclaration,
    ) -> E2eBoundary {
        Self::resolve_in(
            project_root,
            declaration,
            &paths::walk_project(project_root),
        )
    }

    /// Resolve against an already walked tree, so a caller that walked for its
    /// own findings does not walk twice.
    pub(crate) fn resolve_in(
        project_root: &Path,
        declaration: &SpecificationDeclaration,
        tree: &[ProjectPath],
    ) -> E2eBoundary {
        let declared = declaration
            .roles()
            .into_iter()
            .find(|(role, _)| *role == Role::E2e)
            .map(|(_, patterns)| patterns)
            .unwrap_or(&[]);
        if declared.is_empty() {
            return E2eBoundary::default();
        }
        let globs: Vec<(String, Glob)> = declared
            .iter()
            .map(|pattern| (pattern.clone(), Glob::compile(pattern)))
            .collect();

        let mut matched: Vec<(String, bool)> = Vec::new();
        if globs.iter().any(|(_, glob)| glob.matches("")) {
            matched.push((String::new(), true));
        }
        matched.extend(
            tree.iter()
                .filter(|entry| globs.iter().any(|(_, glob)| glob.matches(&entry.path)))
                .map(|entry| (entry.path.clone(), entry.is_dir)),
        );

        let mut conditions = Vec::new();
        if matched.is_empty() {
            for entry in declared {
                conditions.push(BoundaryCondition::missing_package(entry));
            }
            return E2eBoundary {
                conditions,
                ..E2eBoundary::default()
            };
        }

        let manifest_directories: BTreeSet<String> = tree
            .iter()
            .filter(|entry| !entry.is_dir && is_cargo_manifest(&entry.path))
            .map(|entry| parent_directory(&entry.path))
            .collect();
        let mut packages: BTreeMap<String, CargoPackage> = BTreeMap::new();
        let mut unusable: BTreeMap<String, String> = BTreeMap::new();
        for directory in &manifest_directories {
            let manifest = if directory.is_empty() {
                project_root.join("Cargo.toml")
            } else {
                project_root.join(directory).join("Cargo.toml")
            };
            match CargoPackage::read(directory, &manifest) {
                Ok(Some(package)) => {
                    packages.insert(directory.clone(), package);
                }
                Ok(None) => {
                    unusable.insert(
                        directory.clone(),
                        "the manifest declares neither a package nor a workspace".to_string(),
                    );
                }
                Err(reason) => {
                    unusable.insert(directory.clone(), reason);
                }
            }
        }

        let mut resolved: BTreeSet<String> = BTreeSet::new();
        let mut import_paths: BTreeSet<String> = BTreeSet::new();
        for (path, is_dir) in &matched {
            let candidate = if *is_dir {
                path.clone()
            } else {
                parent_directory(path)
            };
            match nearest_manifest(&manifest_directories, &candidate) {
                Some(directory) => {
                    resolved.insert(directory);
                }
                None => {
                    import_paths.insert(path.clone());
                    conditions.push(BoundaryCondition::no_package_manifest(path));
                }
            }
        }

        let source_roots: Vec<(String, Glob)> = declaration
            .source_roots()
            .iter()
            .map(|pattern| (pattern.clone(), Glob::compile(pattern)))
            .collect();
        let labelled = |package: &str| {
            packages
                .get(package)
                .map(|package_details| package_details.label(package))
                .unwrap_or_else(|| package.to_string())
        };

        for package in &resolved {
            if let Some((pattern, _)) = source_roots.iter().find(|(_, glob)| glob.matches(package))
            {
                conditions.push(BoundaryCondition::package_under_source(
                    package,
                    &labelled(package),
                    pattern,
                ));
            }
        }
        for path in &import_paths {
            if let Some((pattern, _)) = source_roots.iter().find(|(_, glob)| glob.matches(path)) {
                conditions.push(BoundaryCondition::path_under_source(path, pattern));
            }
        }

        for package in &resolved {
            for (pattern, glob) in &source_roots {
                if tree.iter().any(|entry| {
                    entry.path.as_str() != package.as_str()
                        && is_under(&entry.path, package)
                        && glob.matches(&entry.path)
                }) {
                    conditions.push(BoundaryCondition::source_inside_package(
                        package,
                        &labelled(package),
                        pattern,
                    ));
                }
            }
        }

        for package in &resolved {
            if packages.contains_key(package) {
                continue;
            }
            if let Some(reason) = unusable.get(package) {
                conditions.push(BoundaryCondition::malformed_manifest(package, reason));
            }
        }

        let edges = dependency_edges(&packages);
        let mut reported: BTreeSet<(String, String)> = BTreeSet::new();
        for start in &resolved {
            let Some(package) = packages.get(start) else {
                continue;
            };
            let mut visited: BTreeSet<String> = BTreeSet::new();
            visited.insert(start.clone());
            let mut queue: VecDeque<(String, usize)> = VecDeque::new();
            queue.push_back((start.clone(), 0));
            while let Some((current, distance)) = queue.pop_front() {
                for target in edges.get(&current).into_iter().flatten() {
                    if !visited.insert(target.clone()) {
                        continue;
                    }
                    let Some(pattern) = source_root_reached_by(target, &source_roots, tree) else {
                        queue.push_back((target.clone(), distance + 1));
                        continue;
                    };
                    if !reported.insert((start.clone(), target.clone())) {
                        continue;
                    }
                    let edge = if distance + 1 == 1 {
                        "dependency edge"
                    } else {
                        "transitive dependency edge"
                    };
                    conditions.push(BoundaryCondition::dependency_edge(
                        start,
                        &package.label(start),
                        &packages[target].label(target),
                        &pattern,
                        edge,
                    ));
                }
            }
        }

        for path in &import_paths {
            for (target, root) in import_edges(project_root, path, tree, &source_roots) {
                conditions.push(BoundaryCondition::import_edge(path, &target, &root));
            }
        }

        E2eBoundary {
            resolved: resolved.into_iter().collect(),
            import_paths: import_paths.into_iter().collect(),
            conditions,
        }
    }

    /// The digest of each resolved root's tracked tree, keyed by its
    /// project-relative directory (or its import-mode path). No declaration
    /// pattern and no file outside the resolved set is hashed.
    pub(crate) fn identity(&self, project_root: &Path) -> anyhow::Result<BTreeMap<String, String>> {
        let backend = crate::scm::select(project_root)?;
        let entries = backend.content_paths()?;
        let mut digests = BTreeMap::new();
        for root in self.roots() {
            let mut hasher = Sha256::new();
            for entry in entries
                .iter()
                .filter(|entry| is_under(&entry.path.to_string_lossy(), root))
            {
                let relative = entry.path.to_string_lossy().replace('\\', "/");
                hash_entry(
                    &mut hasher,
                    project_root,
                    &relative,
                    entry.gitlink.as_deref(),
                )?;
            }
            digests.insert(root.clone(), digest_hex(hasher));
        }
        Ok(digests)
    }

    /// Every resolved root the E2E runner command must name.
    pub(crate) fn roots(&self) -> impl Iterator<Item = &String> {
        self.resolved.iter().chain(self.import_paths.iter())
    }

    /// Whether a project-relative path is a resolved root or lies below one.
    /// The frozen-package refusal tests a trial path through this rule, so the
    /// containment semantics have one implementation beside the resolution.
    pub(crate) fn contains(&self, path: &str) -> bool {
        self.roots().any(|root| is_under(path, root))
    }

    /// The conditions that block the change check: the placement and
    /// acceptance conditions, plus `RunnerNamesNoPackage` when the command
    /// names no resolved root.
    pub(crate) fn conditions(&self, command: &str) -> Vec<BoundaryCondition> {
        let mut conditions = self
            .conditions
            .iter()
            .map(BoundaryCondition::clone)
            .collect::<Vec<_>>();
        let command = command.trim();
        if let Some(root) = self.roots().find(|root| !names_root(command, root)) {
            conditions.push(BoundaryCondition::runner_names_no_package(command, root));
        }
        conditions
    }
}

/// The tracked files of the declared `e2e` packages, resolved over a revision
/// tree instead of the working tree.
///
/// Each path the `e2e` role matches resolves to its nearest `Cargo.toml`
/// directory at or above it; every tracked file under a resolved root is in
/// the set. A matched path that resolves no manifest is its own root.
pub(crate) fn e2e_file_set(
    declaration: &SpecificationDeclaration,
    paths: &[&str],
) -> BTreeSet<String> {
    let declared = declaration
        .roles()
        .into_iter()
        .find(|(role, _)| *role == Role::E2e)
        .map(|(_, patterns)| patterns)
        .unwrap_or(&[]);
    if declared.is_empty() {
        return BTreeSet::new();
    }
    let globs: Vec<Glob> = declared
        .iter()
        .map(|pattern| Glob::compile(pattern))
        .collect();
    let manifest_directories: BTreeSet<String> = paths
        .iter()
        .filter(|path| is_cargo_manifest(path))
        .map(|path| parent_directory(path))
        .collect();

    let mut candidates: BTreeSet<(String, bool)> = BTreeSet::new();
    if globs.iter().any(|glob| glob.matches("")) {
        candidates.insert((String::new(), true));
    }
    let mut directories: BTreeSet<String> = BTreeSet::new();
    for path in paths {
        if globs.iter().any(|glob| glob.matches(path)) {
            candidates.insert((path.to_string(), false));
        }
        for (index, _) in path.match_indices('/') {
            directories.insert(path[..index].to_string());
        }
    }
    for directory in directories {
        if globs.iter().any(|glob| glob.matches(&directory)) {
            candidates.insert((directory, true));
        }
    }

    let mut roots: BTreeSet<String> = BTreeSet::new();
    for (path, is_dir) in candidates {
        let candidate = if is_dir {
            path.clone()
        } else {
            parent_directory(&path)
        };
        let Some(candidate) = candidate_path(&candidate) else {
            continue;
        };
        match nearest_manifest(&manifest_directories, &candidate) {
            Some(directory) => {
                roots.insert(directory);
            }
            None => {
                roots.insert(path);
            }
        }
    }
    paths
        .iter()
        .filter(|path| roots.iter().any(|root| is_under(path, root)))
        .map(|path| (*path).to_string())
        .collect()
}

/// The project-relative form of a candidate directory. The empty string is
/// the project root, which the declaration normalizer rejects but the E2E
/// boundary uses as its root package.
fn candidate_path(candidate: &str) -> Option<String> {
    if candidate.is_empty() {
        return Some(String::new());
    }
    normalize_project_path(candidate)
}

/// The `source` root a dependency target reaches. An edge is compared by
/// directory, not by the pattern's own match: a target is refused when it is
/// the root, lies inside the root's literal directory, or contains a path the
/// pattern names. `src/**/*.rs` therefore reaches `src` although the glob
/// matches no directory named `src`.
fn source_root_reached_by(
    directory: &str,
    source_roots: &[(String, Glob)],
    tree: &[ProjectPath],
) -> Option<String> {
    source_roots
        .iter()
        .find(|(_, glob)| {
            let anchor = glob.anchor();
            glob.matches(directory)
                || (!anchor.is_empty() && is_under(directory, &anchor))
                || tree.iter().any(|entry| {
                    entry.path != directory
                        && is_under(&entry.path, directory)
                        && glob.matches(&entry.path)
                })
        })
        .map(|(pattern, _)| pattern.clone())
}

/// Whether `candidate` is `root` or lies below it. The project root is `""`.
fn is_under(candidate: &str, root: &str) -> bool {
    root.is_empty()
        || candidate == root
        || candidate
            .strip_prefix(root)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// The lowercase hex rendering of a finished SHA-256 hash.
fn digest_hex(hasher: Sha256) -> String {
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Whether the command names `root`: as a whole token, or as a token carrying
/// `<root>/`. Quoting around a token is allowed.
///
/// A `--package <root>` value is such a token, so a declared E2E command names
/// its root through the flag value. A bare tier flag (`--e2e`) names no
/// package.
fn names_root(command: &str, root: &str) -> bool {
    let needle = if root.is_empty() { "." } else { root };
    let prefix = format!("{needle}/");
    command.split_whitespace().any(|raw| {
        let token = raw.trim_matches(|character: char| {
            matches!(character, '"' | '\'' | ',' | ';' | '(' | ')')
        });
        token == needle || !prefix.is_empty() && token.contains(&prefix)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::e2e_boundary::conditions::BoundaryConditionKind;

    #[test]
    fn a_whole_token_names_a_root() {
        assert!(names_root(
            "./run-tests.sh --package tests/e2e",
            "tests/e2e"
        ));
        assert!(names_root(
            "cargo test --manifest-path tests/e2e/Cargo.toml",
            "tests/e2e"
        ));
        assert!(names_root("./run.sh \"tests/e2e\"", "tests/e2e"));
    }

    #[test]
    fn a_path_with_the_root_as_its_prefix_does_not_name_it() {
        assert!(!names_root(
            "./run-tests.sh --package tests/e2e-extra",
            "tests/e2e"
        ));
        assert!(!names_root(
            "./run-tests.sh --bin other-target",
            "tests/e2e"
        ));
        assert!(!names_root("givn missing-e2e-runner", "tests/e2e"));
        assert!(!names_root("", "tests/e2e"));
    }

    #[test]
    fn the_first_unnamed_root_is_refused_by_name() {
        let boundary = E2eBoundary {
            resolved: vec!["tests/e2e".to_string(), "tests/acceptance".to_string()],
            import_paths: Vec::new(),
            conditions: Vec::new(),
        };
        let conditions = boundary.conditions("./run-tests.sh --package tests/e2e");
        assert_eq!(conditions.len(), 1);
        assert_eq!(
            conditions[0].kind,
            BoundaryConditionKind::RunnerNamesNoPackage
        );
        assert!(conditions[0].message.contains("tests/acceptance"));
    }

    #[test]
    fn a_named_root_adds_no_runner_condition() {
        let boundary = E2eBoundary {
            resolved: vec!["tests/e2e".to_string()],
            import_paths: Vec::new(),
            conditions: Vec::new(),
        };
        assert!(boundary.conditions("./run-tests.sh tests/e2e/").is_empty());
    }

    #[test]
    fn containment_covers_a_resolved_root_and_everything_below_it() {
        let boundary = E2eBoundary {
            resolved: vec!["tests/e2e".to_string()],
            import_paths: vec!["tests/imported-e2e".to_string()],
            conditions: Vec::new(),
        };
        assert!(boundary.contains("tests/e2e"));
        assert!(boundary.contains("tests/e2e/steps/keys.rs"));
        assert!(boundary.contains("tests/imported-e2e/tests/keys.rs"));
        assert!(!boundary.contains("tests/e2e-extra/steps/keys.rs"));
        assert!(!boundary.contains("src/keys/parser.rs"));
        assert!(!boundary.contains(""));
    }

    fn write(root: &std::path::Path, relative: &str, content: &str) {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().expect("a parent directory")).unwrap();
        std::fs::write(path, content).unwrap();
    }

    fn declaration(root: &std::path::Path, e2e: &str, source: &str) -> SpecificationDeclaration {
        write(
            root,
            "givn/specification.yaml",
            &format!(
                "behavior: []\nstructure: []\ncontracts: []\nlimits: []\ne2e: [{e2e}]\nsource: [{source}]\n"
            ),
        );
        SpecificationDeclaration::load(root)
            .expect("a readable declaration")
            .expect("a declaration")
    }

    fn dependency_conditions(boundary: &E2eBoundary) -> Vec<&BoundaryCondition> {
        boundary
            .conditions
            .iter()
            .filter(|condition| condition.kind == BoundaryConditionKind::DependencyEdge)
            .collect()
    }

    #[test]
    fn a_target_specific_dependency_into_source_is_refused() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let declaration = declaration(root, "\"tests/e2e\"", "\"src/**\"");
        write(
            root,
            "tests/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [target.'cfg(unix)'.dependencies]\nproduct = { path = \"../../src\" }\n",
        );
        write(
            root,
            "src/Cargo.toml",
            "[package]\nname = \"product\"\nversion = \"0.1.0\"\n",
        );

        let boundary = E2eBoundary::resolve(root, &declaration);
        let conditions = dependency_conditions(&boundary);
        assert_eq!(conditions.len(), 1, "{:?}", boundary.conditions);
        assert!(conditions[0].message.contains("through package 'product'"));
        assert!(conditions[0].message.contains("'src/**'"));
    }

    #[test]
    fn a_glob_shaped_source_root_reaches_the_directory_it_anchors_in() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let declaration = declaration(root, "\"tests/e2e\"", "\"src/**/*.rs\"");
        write(
            root,
            "tests/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\nproduct = { path = \"../../src\" }\n",
        );
        write(
            root,
            "src/Cargo.toml",
            "[package]\nname = \"product\"\nversion = \"0.1.0\"\n",
        );
        write(root, "src/lib.rs", "pub fn helper() {}\n");

        let boundary = E2eBoundary::resolve(root, &declaration);
        let conditions = dependency_conditions(&boundary);
        assert_eq!(conditions.len(), 1, "{:?}", boundary.conditions);
        assert!(conditions[0].message.contains("'src/**/*.rs'"));
    }

    #[test]
    fn a_dependency_that_contains_a_source_root_is_refused() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let declaration = declaration(root, "\"tests/e2e\"", "\"src/**\"");
        write(
            root,
            "Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
        );
        write(
            root,
            "tests/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\napp = { path = \"../..\" }\n",
        );
        write(root, "src/lib.rs", "pub fn helper() {}\n");

        let boundary = E2eBoundary::resolve(root, &declaration);
        let conditions = dependency_conditions(&boundary);
        assert_eq!(conditions.len(), 1, "{:?}", boundary.conditions);
        assert!(conditions[0].message.contains("through package 'app'"));
    }

    #[test]
    fn a_dependency_outside_every_source_root_is_admitted() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let declaration = declaration(root, "\"tests/e2e\"", "\"src/**/*.rs\"");
        write(
            root,
            "tests/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\nhelper = { path = \"../helper\" }\n",
        );
        write(
            root,
            "tests/helper/Cargo.toml",
            "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n",
        );
        write(root, "src/lib.rs", "pub fn helper() {}\n");

        let boundary = E2eBoundary::resolve(root, &declaration);
        assert!(
            dependency_conditions(&boundary).is_empty(),
            "{:?}",
            boundary.conditions
        );
    }
}
