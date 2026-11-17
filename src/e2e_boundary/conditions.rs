//! The boundary conditions one resolver reports.
//!
//! `givn lint` renders the placement subset as advisory findings; `givn check
//! tasks` refuses the acceptance subset. Both mappings read the same message,
//! so a condition is worded once.

/// One boundary condition, in the order the resolver found it.
#[derive(Clone, Debug)]
pub(crate) struct BoundaryCondition {
    pub(crate) kind: BoundaryConditionKind,
    /// The package directory, matched path, role entry, or root the condition
    /// is about.
    pub(crate) subject: String,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundaryConditionKind {
    MissingPackage,
    NoPackageManifest,
    PackageUnderSource,
    SourceInsidePackage,
    DependencyEdge,
    ImportEdge,
    MalformedManifest,
    RunnerNamesNoPackage,
}

impl BoundaryCondition {
    /// Whether `givn check tasks` refuses the condition. `NoPackageManifest`
    /// is the one base condition the check admits, because it falls back to
    /// the import reader for that path.
    pub(crate) fn acceptance(&self) -> bool {
        !matches!(self.kind, BoundaryConditionKind::NoPackageManifest)
    }

    /// Whether `givn lint` renders the condition as an advisory finding.
    pub(crate) fn placement(&self) -> bool {
        matches!(
            self.kind,
            BoundaryConditionKind::MissingPackage
                | BoundaryConditionKind::NoPackageManifest
                | BoundaryConditionKind::PackageUnderSource
                | BoundaryConditionKind::DependencyEdge
        )
    }

    /// The refusal line the change check prints, naming the subject first.
    pub(crate) fn render(&self) -> String {
        if self.subject.is_empty() || self.message.contains(&self.subject) {
            self.message.clone()
        } else {
            format!("{}: {}", self.subject, self.message)
        }
    }

    pub(crate) fn missing_package(entry: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::MissingPackage,
            subject: entry.to_string(),
            message: format!("the 'e2e' role entry '{entry}' matches no path in the project"),
        }
    }

    pub(crate) fn no_package_manifest(path: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::NoPackageManifest,
            subject: path.to_string(),
            message: format!(
                "the matched path '{path}' resolves no package manifest at or above it before \
                 the project root"
            ),
        }
    }

    pub(crate) fn package_under_source(package: &str, label: &str, root: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::PackageUnderSource,
            subject: package.to_string(),
            message: format!("e2e package '{label}' sits under a source root ('{root}')"),
        }
    }

    pub(crate) fn path_under_source(path: &str, root: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::PackageUnderSource,
            subject: path.to_string(),
            message: format!("the e2e path '{path}' sits under a source root ('{root}')"),
        }
    }

    pub(crate) fn source_inside_package(package: &str, label: &str, root: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::SourceInsidePackage,
            subject: package.to_string(),
            message: format!("e2e package '{label}' contains the source root '{root}'"),
        }
    }

    pub(crate) fn dependency_edge(
        package: &str,
        label: &str,
        target: &str,
        root: &str,
        edge: &str,
    ) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::DependencyEdge,
            subject: package.to_string(),
            message: format!(
                "e2e package '{label}' has a {edge} into source (root '{root}') through package \
                 '{target}'"
            ),
        }
    }

    pub(crate) fn import_edge(path: &str, target: &str, root: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::ImportEdge,
            subject: path.to_string(),
            message: format!(
                "the e2e path '{path}' imports the source path '{target}' (root '{root}')"
            ),
        }
    }

    pub(crate) fn malformed_manifest(package: &str, reason: &str) -> Self {
        BoundaryCondition {
            kind: BoundaryConditionKind::MalformedManifest,
            subject: package.to_string(),
            message: format!("the e2e package '{package}' cannot be read: {reason}"),
        }
    }

    pub(crate) fn runner_names_no_package(command: &str, root: &str) -> Self {
        let label = if root.is_empty() { "." } else { root };
        BoundaryCondition {
            kind: BoundaryConditionKind::RunnerNamesNoPackage,
            subject: root.to_string(),
            message: format!(
                "the E2E runner command '{command}' does not name the e2e root '{label}'"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn condition(kind: BoundaryConditionKind, subject: &str) -> BoundaryCondition {
        BoundaryCondition {
            kind,
            subject: subject.to_string(),
            message: "message".to_string(),
        }
    }

    #[test]
    fn placement_marks_the_advisory_conditions_only() {
        assert!(condition(BoundaryConditionKind::MissingPackage, "x").placement());
        assert!(condition(BoundaryConditionKind::NoPackageManifest, "x").placement());
        assert!(condition(BoundaryConditionKind::PackageUnderSource, "x").placement());
        assert!(condition(BoundaryConditionKind::DependencyEdge, "x").placement());
        assert!(!condition(BoundaryConditionKind::SourceInsidePackage, "x").placement());
        assert!(!condition(BoundaryConditionKind::ImportEdge, "x").placement());
        assert!(!condition(BoundaryConditionKind::MalformedManifest, "x").placement());
        assert!(!condition(BoundaryConditionKind::RunnerNamesNoPackage, "x").placement());
    }

    #[test]
    fn the_missing_manifest_condition_is_advisory_for_the_change_check() {
        assert!(!condition(BoundaryConditionKind::NoPackageManifest, "x").acceptance());
        assert!(condition(BoundaryConditionKind::MissingPackage, "x").acceptance());
        assert!(condition(BoundaryConditionKind::ImportEdge, "x").acceptance());
        assert!(condition(BoundaryConditionKind::RunnerNamesNoPackage, "x").acceptance());
    }
}
