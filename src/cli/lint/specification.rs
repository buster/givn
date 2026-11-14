use super::manifest::{
    CargoPackage, dependency_edges, is_cargo_manifest, nearest_manifest, parent_directory,
};
use crate::config::declaration::{DECLARATION_PATH, Role, SpecificationDeclaration};
use crate::overlap::analysis::Finding;
use anyhow::Result;

/// The outcome of the default-scope specification pass. A refused declaration
/// is the only finding; otherwise the four advisory finding kinds are computed
/// from the complete role set.
pub(super) enum SpecificationHealth {
    Blocking(Finding),
    Advisories(Vec<Finding>),
}

/// Read the declaration once and report its health. Runs only in the default
/// project scope, and only while `ise` is effective: nothing is read while the
/// addon is off, whatever sits on disk.
pub(super) fn specification_health(project_root: &std::path::Path) -> Result<SpecificationHealth> {
    let manifest = crate::config::overlay::load_effective_at(project_root)?;
    if !SpecificationDeclaration::is_effective(&manifest) {
        return Ok(SpecificationHealth::Advisories(Vec::new()));
    }
    let declaration = match SpecificationDeclaration::load(project_root) {
        Ok(Some(declaration)) => declaration,
        Ok(None) => {
            return Ok(SpecificationHealth::Blocking(declaration_finding(
                "the declaration is absent; the declaration checks were not performed",
            )));
        }
        Err(error) => {
            return Ok(SpecificationHealth::Blocking(declaration_finding(
                &error.detail(),
            )));
        }
    };
    Ok(SpecificationHealth::Advisories(specification_findings(
        project_root,
        &declaration,
    )))
}

/// The blocking declaration finding. Its location is the declaration file the
/// developer must write or repair.
fn declaration_finding(message: &str) -> Finding {
    Finding {
        kind: "specification_declaration".to_string(),
        level: "error".to_string(),
        files: vec![DECLARATION_PATH.to_string()],
        titles: vec![message.to_string()],
        contradiction: false,
    }
}

/// One project-relative declaration finding, located at the declaration and
/// the path or package it is about.
fn declaration_advisory(kind: &str, subject: &str, message: String) -> Finding {
    Finding {
        kind: kind.to_string(),
        level: "warning".to_string(),
        files: vec![DECLARATION_PATH.to_string(), subject.to_string()],
        titles: vec![message],
        contradiction: false,
    }
}

/// The four advisory findings. Findings iterate declared non-empty lists only,
/// so the empty scaffold is silent, and `e2e` is excluded from the generic
/// path-missing finding: its absent package is reported once by the boundary
/// finding with the role entry and the reason.
fn specification_findings(
    project_root: &std::path::Path,
    declaration: &SpecificationDeclaration,
) -> Vec<Finding> {
    let roles = declaration.roles();
    if roles.iter().all(|(_, paths)| paths.is_empty()) {
        return Vec::new();
    }
    let tree = walk_project(project_root);
    let mut findings = Vec::new();

    for (role, declared) in declaration.roles() {
        if role == Role::E2e {
            continue;
        }
        for pattern in declared {
            let glob = Glob::compile(pattern);
            if !tree.iter().any(|entry| glob.matches(&entry.path)) {
                findings.push(declaration_advisory(
                    "specification_path_missing",
                    pattern,
                    format!(
                        "the '{}' role declares '{pattern}', which matches no path in the project",
                        role.as_str()
                    ),
                ));
            }
        }
    }

    // One finding per duplicated string, not one per role pair: three roles
    // declaring the same path is one ambiguous ownership, not three.
    let mut ownership: Vec<(&str, Vec<&str>)> = Vec::new();
    for (role, declared) in declaration.roles() {
        for pattern in declared {
            match ownership
                .iter_mut()
                .find(|(candidate, _)| *candidate == pattern.as_str())
            {
                Some((_, roles)) => {
                    if !roles.contains(&role.as_str()) {
                        roles.push(role.as_str());
                    }
                }
                None => ownership.push((pattern.as_str(), vec![role.as_str()])),
            }
        }
    }
    for (pattern, roles) in ownership {
        if roles.len() < 2 {
            continue;
        }
        let named = roles.join("', '");
        findings.push(declaration_advisory(
            "specification_role_duplicate",
            pattern,
            format!(
                "duplicate declaration: '{pattern}' is declared by '{named}'; a path belongs to one role"
            ),
        ));
    }

    // A behavior tree with no `.feature` file is a declared but empty role, not
    // an error: only a corpus with features can leave one unretained.
    let behavior = declaration
        .roles()
        .into_iter()
        .find(|(role, _)| *role == Role::Behavior)
        .map(|(_, paths)| paths)
        .unwrap_or(&[]);
    let behavior_globs: Vec<Glob> = behavior
        .iter()
        .map(|pattern| Glob::compile(pattern))
        .collect();
    if !behavior_globs.is_empty() {
        for entry in &tree {
            if entry.is_dir || !is_feature_file(&entry.path) {
                continue;
            }
            if behavior_globs.iter().any(|glob| glob.matches(&entry.path)) {
                continue;
            }
            findings.push(Finding {
                kind: "specification_feature_unretained".to_string(),
                level: "warning".to_string(),
                files: vec![entry.path.clone()],
                titles: vec![
                    "the .feature file matches no 'behavior' glob; it is not retained by the \
                     declared Specification"
                        .to_string(),
                ],
                contradiction: false,
            });
        }
    }

    let e2e = declaration
        .roles()
        .into_iter()
        .find(|(role, _)| *role == Role::E2e)
        .map(|(_, paths)| paths)
        .unwrap_or(&[]);
    let e2e_globs: Vec<Glob> = e2e.iter().map(|pattern| Glob::compile(pattern)).collect();
    let matched: Vec<&ProjectPath> = tree
        .iter()
        .filter(|entry| e2e_globs.iter().any(|glob| glob.matches(&entry.path)))
        .collect();
    if !e2e.is_empty() && matched.is_empty() {
        for pattern in e2e {
            findings.push(declaration_advisory(
                "specification_e2e_boundary",
                pattern,
                format!("the 'e2e' role entry '{pattern}' matches no path in the project"),
            ));
        }
    }
    if !matched.is_empty() {
        let manifest_directories: std::collections::BTreeSet<String> = tree
            .iter()
            .filter(|entry| !entry.is_dir && is_cargo_manifest(&entry.path))
            .map(|entry| parent_directory(&entry.path))
            .collect();
        let mut packages: std::collections::BTreeMap<String, CargoPackage> =
            std::collections::BTreeMap::new();
        for directory in &manifest_directories {
            let manifest = if directory.is_empty() {
                project_root.join("Cargo.toml")
            } else {
                project_root.join(directory).join("Cargo.toml")
            };
            if let Some(package) = CargoPackage::read(directory, &manifest) {
                packages.insert(directory.clone(), package);
            }
        }

        let mut resolved: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for entry in &matched {
            let candidate = if entry.is_dir {
                entry.path.clone()
            } else {
                parent_directory(&entry.path)
            };
            match nearest_manifest(&manifest_directories, &candidate) {
                Some(directory) => {
                    // A manifest that exists but does not parse (malformed, or
                    // carrying no [package]) skips the edge comparison for that
                    // package; no finding is invented for it.
                    if packages.contains_key(&directory) {
                        resolved.insert(directory);
                    }
                }
                None => findings.push(declaration_advisory(
                    "specification_e2e_boundary",
                    &entry.path,
                    format!(
                        "the matched path '{}' resolves no package manifest at or above it \
                         before the project root",
                        entry.path
                    ),
                )),
            }
        }

        let source_roots: Vec<(&String, Glob)> = declaration
            .source_roots()
            .iter()
            .map(|pattern| (pattern, Glob::compile(pattern)))
            .collect();
        let edges = dependency_edges(&packages);
        let mut reported: std::collections::BTreeSet<(String, String)> =
            std::collections::BTreeSet::new();
        for start in &resolved {
            let package = &packages[start];
            if let Some((pattern, _)) = source_roots.iter().find(|(_, glob)| glob.matches(start)) {
                findings.push(declaration_advisory(
                    "specification_e2e_boundary",
                    start,
                    format!(
                        "e2e package '{}' sits under a source root ('{pattern}')",
                        package.name
                    ),
                ));
            }
            let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            visited.insert(start.clone());
            let mut queue: std::collections::VecDeque<(String, usize)> =
                std::collections::VecDeque::new();
            queue.push_back((start.clone(), 0));
            while let Some((current, distance)) = queue.pop_front() {
                for target in edges.get(&current).into_iter().flatten() {
                    if !visited.insert(target.clone()) {
                        continue;
                    }
                    let Some((pattern, _)) =
                        source_roots.iter().find(|(_, glob)| glob.matches(target))
                    else {
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
                    findings.push(declaration_advisory(
                        "specification_e2e_boundary",
                        start,
                        format!(
                            "e2e package '{}' has a {edge} into source (root '{pattern}') through \
                             package '{}'",
                            package.name, packages[target].name
                        ),
                    ));
                }
            }
        }
    }
    findings
}

/// Whether a project-relative path names a `.feature` file.
fn is_feature_file(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "feature")
}

/// One entry of the bounded project walk: a project-relative path and whether
/// it is a directory.
struct ProjectPath {
    path: String,
    is_dir: bool,
}

/// Walk the project under the finding exclusions. Paths are project-relative
/// and use `/`; version-control, build, archive, and active-change trees are
/// skipped because they are not retained behavior.
fn walk_project(project_root: &std::path::Path) -> Vec<ProjectPath> {
    let mut entries = Vec::new();
    walk_directory(project_root, project_root, &mut entries);
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    entries
}

fn walk_directory(
    project_root: &std::path::Path,
    directory: &std::path::Path,
    entries: &mut Vec<ProjectPath>,
) {
    let Ok(read_dir) = std::fs::read_dir(directory) else {
        return;
    };
    let mut children: Vec<std::fs::DirEntry> = read_dir.flatten().collect();
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        let path = child.path();
        let Ok(relative) = path.strip_prefix(project_root) else {
            continue;
        };
        let relative = relative
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        if is_excluded_from_findings(&relative) {
            continue;
        }
        let Ok(file_type) = child.file_type() else {
            continue;
        };
        let is_dir = file_type.is_dir();
        entries.push(ProjectPath {
            path: relative,
            is_dir,
        });
        if is_dir {
            walk_directory(project_root, &path, entries);
        }
    }
}

/// Whether a project-relative path belongs to a tree the findings never scan.
fn is_excluded_from_findings(relative: &str) -> bool {
    if relative == "givn/archive"
        || relative.starts_with("givn/archive/")
        || relative == "givn/changes"
        || relative.starts_with("givn/changes/")
    {
        return true;
    }
    relative
        .split('/')
        .any(|segment| matches!(segment, ".git" | ".hg" | ".svn" | ".fossil" | "target"))
}

/// One compiled role pattern. `*` matches within one segment, `?` matches one
/// character, and a `**` segment matches zero or more segments. Everything else
/// is literal, so a pattern without wildcards is an existence check.
struct Glob {
    segments: Vec<String>,
}

impl Glob {
    fn compile(pattern: &str) -> Self {
        let segments = std::path::Path::new(pattern)
            .components()
            .filter_map(|component| match component {
                std::path::Component::Normal(segment) => {
                    Some(segment.to_string_lossy().into_owned())
                }
                _ => None,
            })
            .collect();
        Glob { segments }
    }

    fn matches(&self, path: &str) -> bool {
        self.matches_segments(0, path)
    }

    fn matches_segments(&self, pattern_index: usize, rest: &str) -> bool {
        let Some(segment) = self.segments.get(pattern_index) else {
            return rest.is_empty();
        };
        if segment == "**" {
            let mut remaining = rest;
            loop {
                if self.matches_segments(pattern_index + 1, remaining) {
                    return true;
                }
                if remaining.is_empty() {
                    return false;
                }
                remaining = match remaining.find('/') {
                    Some(slash) => &remaining[slash + 1..],
                    None => "",
                };
            }
        }
        let (head, tail) = match rest.find('/') {
            Some(slash) => (&rest[..slash], &rest[slash + 1..]),
            None => (rest, ""),
        };
        !head.is_empty()
            && segment_matches(segment, head)
            && self.matches_segments(pattern_index + 1, tail)
    }
}

/// Whether one path segment matches one pattern segment. Wildcards consume
/// whole characters, so a multi-byte character counts as one `?`.
fn segment_matches(pattern: &str, text: &str) -> bool {
    match pattern.chars().next() {
        None => text.is_empty(),
        Some('*') => {
            let rest = &pattern[1..];
            segment_matches(rest, text)
                || text.chars().next().is_some_and(|character| {
                    segment_matches(pattern, &text[character.len_utf8()..])
                })
        }
        Some('?') => text
            .chars()
            .next()
            .is_some_and(|character| segment_matches(&pattern[1..], &text[character.len_utf8()..])),
        Some(literal) => text.chars().next().is_some_and(|character| {
            character == literal
                && segment_matches(
                    &pattern[literal.len_utf8()..],
                    &text[character.len_utf8()..],
                )
        }),
    }
}
