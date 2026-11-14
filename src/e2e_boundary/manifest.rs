//! Static Cargo package reading for the declared `e2e` boundary.
//!
//! The reader never invokes `cargo`: it parses each manifest that the bounded
//! project walk found and resolves path and workspace-inherited edges. A
//! manifest the reader cannot use is reported with its reason, because the
//! boundary cannot be proven from it.

/// Whether a project-relative path is a Cargo manifest.
pub(crate) fn is_cargo_manifest(path: &str) -> bool {
    path.rsplit('/').next() == Some("Cargo.toml")
}

/// The nearest manifest directory at or above `path`, bounded at the project
/// root (`""`).
pub(crate) fn nearest_manifest(
    manifest_directories: &std::collections::BTreeSet<String>,
    path: &str,
) -> Option<String> {
    let mut current = Some(path.to_string());
    while let Some(directory) = current {
        if manifest_directories.contains(&directory) {
            return Some(directory);
        }
        current = if directory.is_empty() {
            None
        } else {
            Some(parent_directory(&directory))
        };
    }
    None
}

/// A package manifest read statically, without invoking `cargo`. A virtual
/// workspace root carries no `[package]` and is still read, because a member's
/// `workspace = true` dependency resolves through it.
#[derive(Debug)]
pub(crate) struct CargoPackage {
    /// `[package].name`; `None` for a virtual workspace root.
    name: Option<String>,
    dependencies: Vec<Dependency>,
    workspace_dependencies: std::collections::BTreeMap<String, String>,
    has_workspace: bool,
}

/// One dependency edge declared by a manifest. A registry dependency carries
/// no project path and is not an edge into `source`.
#[derive(Debug)]
enum Dependency {
    /// An explicit `path = "..."` dependency, normalized to a project-relative
    /// directory.
    Path(String),
    /// `workspace = true`; the path comes from the workspace root.
    Workspace(String),
}

impl CargoPackage {
    /// Read one manifest. `Ok(None)` means the file parses but declares
    /// neither a `[package]` nor a `[workspace]`; `Err` carries the reason the
    /// manifest cannot be used.
    pub(crate) fn read(
        dir: &str,
        manifest: &std::path::Path,
    ) -> std::result::Result<Option<Self>, String> {
        let content = std::fs::read_to_string(manifest).map_err(|error| error.to_string())?;
        let document: toml::Value = toml::from_str(&content).map_err(|error| error.to_string())?;
        let name = document
            .get("package")
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_string);
        let has_workspace = document.get("workspace").is_some();
        if name.is_none() && !has_workspace {
            return Ok(None);
        }
        let mut dependencies = Vec::new();
        for table in ["dependencies", "dev-dependencies", "build-dependencies"] {
            let Some(entries) = document.get(table).and_then(toml::Value::as_table) else {
                continue;
            };
            for (dependency, entry) in entries {
                let Some(entry) = entry.as_table() else {
                    continue;
                };
                if let Some(path) = entry.get("path").and_then(toml::Value::as_str) {
                    if let Some(target) = normalize_relative(dir, path) {
                        dependencies.push(Dependency::Path(target));
                    }
                } else if entry.get("workspace").and_then(toml::Value::as_bool) == Some(true) {
                    dependencies.push(Dependency::Workspace(dependency.clone()));
                }
            }
        }
        let workspace_dependencies = document
            .get("workspace")
            .and_then(|workspace| workspace.get("dependencies"))
            .and_then(toml::Value::as_table)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|(name, entry)| {
                        let path = entry.get("path")?.as_str()?;
                        Some((name.clone(), normalize_relative(dir, path)?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(Some(CargoPackage {
            name,
            dependencies,
            workspace_dependencies,
            has_workspace,
        }))
    }

    /// The package name for a finding. A virtual workspace root has no
    /// `[package]`, so its manifest directory names it.
    pub(crate) fn label(&self, dir: &str) -> String {
        self.name.clone().unwrap_or_else(|| {
            if dir.is_empty() {
                ".".to_string()
            } else {
                dir.to_string()
            }
        })
    }
}

/// Resolve `path` (relative to the package at `base`) to a project-relative
/// directory, refusing an escape above the project root.
pub(crate) fn normalize_relative(base: &str, path: &str) -> Option<String> {
    let mut segments: Vec<&str> = base
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    for component in std::path::Path::new(path).components() {
        match component {
            std::path::Component::Normal(segment) => segments.push(segment.to_str()?),
            std::path::Component::ParentDir => {
                segments.pop()?;
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir | std::path::Component::Prefix(_) => return None,
        }
    }
    Some(segments.join("/"))
}

/// The direct dependency edges between the parsed packages.
pub(crate) fn dependency_edges(
    packages: &std::collections::BTreeMap<String, CargoPackage>,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut edges: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (dir, package) in packages {
        let mut targets: Vec<String> = Vec::new();
        for dependency in &package.dependencies {
            let target = match dependency {
                Dependency::Path(target) => Some(target.clone()),
                Dependency::Workspace(name) => workspace_root(packages, dir)
                    .and_then(|root| root.workspace_dependencies.get(name).cloned()),
            };
            if let Some(target) = target
                && packages.contains_key(&target)
                && !targets.contains(&target)
            {
                targets.push(target);
            }
        }
        edges.insert(dir.clone(), targets);
    }
    edges
}

/// The nearest package whose manifest declares `[workspace]`, at or above
/// `dir`.
fn workspace_root<'a>(
    packages: &'a std::collections::BTreeMap<String, CargoPackage>,
    dir: &str,
) -> Option<&'a CargoPackage> {
    let mut current = Some(dir.to_string());
    while let Some(directory) = current {
        if let Some(package) = packages.get(&directory)
            && package.has_workspace
        {
            return Some(package);
        }
        current = if directory.is_empty() {
            None
        } else {
            Some(parent_directory(&directory))
        };
    }
    None
}

/// The parent directory of a project-relative path; the project root is `""`.
pub(crate) fn parent_directory(path: &str) -> String {
    path.rsplit_once('/')
        .map_or(String::new(), |(parent, _)| parent.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn write(root: &std::path::Path, relative: &str, content: &str) {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().expect("a parent directory")).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// Read the named manifest directories into the package map keyed by their
    /// project-relative directory.
    fn packages(root: &std::path::Path, directories: &[&str]) -> BTreeMap<String, CargoPackage> {
        directories
            .iter()
            .map(|directory| {
                let manifest = if directory.is_empty() {
                    root.join("Cargo.toml")
                } else {
                    root.join(directory).join("Cargo.toml")
                };
                (
                    (*directory).to_string(),
                    CargoPackage::read(directory, &manifest)
                        .expect("a readable manifest")
                        .expect("a package"),
                )
            })
            .collect()
    }

    #[test]
    fn a_virtual_root_is_read_and_a_workspace_dependency_resolves_through_it() {
        let root = tempfile::tempdir().unwrap();
        write(
            root.path(),
            "Cargo.toml",
            "[workspace]\nmembers = [\"crates/app\", \"e2e\"]\n\n\
             [workspace.dependencies]\napp = { path = \"crates/app\" }\n",
        );
        write(
            root.path(),
            "crates/app/Cargo.toml",
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.path(),
            "e2e/Cargo.toml",
            "[package]\nname = \"e2e-tests\"\nversion = \"0.1.0\"\n\n\
             [dependencies]\napp = { workspace = true }\n",
        );

        let packages = packages(root.path(), &["", "crates/app", "e2e"]);
        assert_eq!(packages[""].label(""), ".");
        assert_eq!(packages["e2e"].label("e2e"), "e2e-tests");

        let edges = dependency_edges(&packages);
        assert_eq!(edges["e2e"], vec!["crates/app".to_string()]);
        assert!(edges["crates/app"].is_empty());
    }

    #[test]
    fn a_manifest_with_neither_a_package_nor_a_workspace_is_skipped() {
        let root = tempfile::tempdir().unwrap();
        write(
            root.path(),
            "Cargo.toml",
            "[dependencies]\napp = { path = \"app\" }\n",
        );
        assert!(
            CargoPackage::read("", &root.path().join("Cargo.toml"))
                .expect("a readable manifest")
                .is_none()
        );
    }

    #[test]
    fn a_malformed_manifest_names_the_parse_failure() {
        let root = tempfile::tempdir().unwrap();
        write(root.path(), "Cargo.toml", "[package\nname = \"app\"\n");
        let reason = CargoPackage::read("", &root.path().join("Cargo.toml"))
            .expect_err("a malformed manifest is refused");
        assert!(reason.contains("TOML"), "{reason}");
    }
}
