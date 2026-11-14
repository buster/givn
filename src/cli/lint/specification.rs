use crate::config::declaration::{DECLARATION_PATH, Role, SpecificationDeclaration};
use crate::e2e_boundary::{BoundaryCondition, E2eBoundary, Glob, walk_project};
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

    let boundary = E2eBoundary::resolve_in(project_root, declaration, &tree);
    for condition in boundary
        .conditions("")
        .into_iter()
        .filter(BoundaryCondition::placement)
    {
        findings.push(declaration_advisory(
            "specification_e2e_boundary",
            &condition.subject,
            condition.message,
        ));
    }
    findings
}

/// Whether a project-relative path names a `.feature` file.
fn is_feature_file(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "feature")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &std::path::Path, relative: &str, content: &str) {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().expect("a parent directory")).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// The six-key declaration with three lists this module's findings read.
    fn declaration(behavior: &[&str], e2e: &[&str], source: &[&str]) -> String {
        let list = |paths: &[&str]| {
            paths
                .iter()
                .map(|path| format!("\"{path}\""))
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!(
            "behavior: [{}]\nstructure: []\ncontracts: []\nlimits: []\ne2e: [{}]\nsource: [{}]\n",
            list(behavior),
            list(e2e),
            list(source)
        )
    }

    fn findings(root: &std::path::Path) -> Vec<Finding> {
        let declaration = SpecificationDeclaration::load(root)
            .expect("a readable declaration")
            .expect("a declaration");
        specification_findings(root, &declaration)
    }

    fn reports(found: &[Finding], title: &str) -> bool {
        found
            .iter()
            .any(|finding| finding.titles.iter().any(|candidate| candidate == title))
    }

    #[test]
    fn an_e2e_package_under_a_source_root_is_reported() {
        let root = tempfile::tempdir().unwrap();
        write(
            root.path(),
            "Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.path(),
            "src/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\napp = { path = \"../..\" }\n",
        );
        write(
            root.path(),
            "givn/specification.yaml",
            &declaration(&[], &["src/e2e"], &["src/**"]),
        );

        let found = findings(root.path());
        assert!(
            reports(
                &found,
                "e2e package 'e2e-tests' sits under a source root ('src/**')"
            ),
            "{found:#?}"
        );
    }

    #[test]
    fn a_workspace_inherited_dependency_is_reported_as_an_e2e_edge() {
        let root = tempfile::tempdir().unwrap();
        write(
            root.path(),
            "Cargo.toml",
            "[workspace]\nmembers = [\"crates/app\", \"tests/e2e\"]\n\n\
             [workspace.dependencies]\napp = { path = \"crates/app\" }\n",
        );
        write(
            root.path(),
            "crates/app/Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.path(),
            "tests/e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\napp = { workspace = true }\n",
        );
        write(
            root.path(),
            "givn/specification.yaml",
            &declaration(&[], &["tests/e2e"], &["crates/**"]),
        );

        let found = findings(root.path());
        assert!(
            reports(
                &found,
                "e2e package 'e2e-tests' has a dependency edge into source (root 'crates/**') \
                 through package 'app'"
            ),
            "{found:#?}"
        );
    }
}
