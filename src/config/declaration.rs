//! The project declaration of its retained Specification
//! (`givn/specification.yaml`).
//!
//! A read-only loader for the `ise` addon's strict schema: five behavior roles
//! plus the list of replaceable `source` roots. Every value is a
//! project-relative path. The loader never writes; the only managed write is
//! the one-time scaffold in `cli::addons`.
//!
//! Strictness has two layers. Duplicate top-level keys are invisible after
//! deserialization, so the schema is scanned line by line first: an unknown key
//! is named before any role comparison, which keeps a misspelled role from also
//! being reported as a missing one. The parse then holds the shape
//! (`deny_unknown_fields`, one string list per key).

use std::path::{Component, Path};

use serde::Deserialize;

use crate::config::manifest::Manifest;

/// Project-relative location of the declaration.
pub const DECLARATION_PATH: &str = "givn/specification.yaml";

/// The key holding the replaceable roots. It is not a behavior role.
const SOURCE_KEY: &str = "source";

/// The complete top-level schema, in declaration order.
const KEYS: [&str; 6] = [
    "behavior",
    "structure",
    "contracts",
    "limits",
    "e2e",
    SOURCE_KEY,
];

/// One of the five roles a declaration retains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Behavior,
    Structure,
    Contracts,
    Limits,
    E2e,
}

impl Role {
    /// Every role, in declaration order.
    pub const ALL: [Role; 5] = [
        Role::Behavior,
        Role::Structure,
        Role::Contracts,
        Role::Limits,
        Role::E2e,
    ];

    /// The role's key in `givn/specification.yaml`.
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Behavior => "behavior",
            Role::Structure => "structure",
            Role::Contracts => "contracts",
            Role::Limits => "limits",
            Role::E2e => "e2e",
        }
    }
}

/// A project's declared Specification: what stays, and what may be replaced.
#[derive(Debug, Clone)]
pub struct SpecificationDeclaration {
    behavior: Vec<String>,
    structure: Vec<String>,
    contracts: Vec<String>,
    limits: Vec<String>,
    e2e: Vec<String>,
    source: Vec<String>,
}

/// Why a declaration was refused. Every message names the offending key, role,
/// or path, and the file itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationError {
    UnknownKey { key: String },
    RepeatedKey { key: String },
    MissingRole { role: &'static str },
    MissingSource,
    OutsideProject { path: String },
    Malformed { reason: String },
}

impl DeclarationError {
    /// The refusal reason without the declaration-path prefix, for a caller
    /// whose finding already carries the file as its location.
    pub fn detail(&self) -> String {
        match self {
            DeclarationError::UnknownKey { key } => format!(
                "unknown key '{key}' — the declaration holds only behavior, structure, \
                 contracts, limits, e2e, and source"
            ),
            DeclarationError::RepeatedKey { key } => {
                format!("'{key}' is declared more than once")
            }
            DeclarationError::MissingRole { role } => format!("missing role '{role}'"),
            DeclarationError::MissingSource => {
                format!("missing the '{SOURCE_KEY}:' list of replaceable roots")
            }
            DeclarationError::OutsideProject { path } => {
                format!("'{path}' is outside the project")
            }
            DeclarationError::Malformed { reason } => reason.clone(),
        }
    }
}

impl std::fmt::Display for DeclarationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{DECLARATION_PATH}: {}", self.detail())
    }
}

impl std::error::Error for DeclarationError {}

impl SpecificationDeclaration {
    /// Read the project's declaration. `Ok(None)` means no declaration file is
    /// present; a present but unreadable file is an error.
    pub fn load(project_root: &Path) -> Result<Option<Self>, DeclarationError> {
        let content = match std::fs::read_to_string(project_root.join(DECLARATION_PATH)) {
            Ok(content) => content,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(DeclarationError::Malformed {
                    reason: error.to_string(),
                });
            }
        };
        Self::parse(&content).map(Some)
    }

    /// The declared paths of each role, in declaration order.
    pub fn roles(&self) -> [(Role, &[String]); 5] {
        [
            (Role::Behavior, self.behavior.as_slice()),
            (Role::Structure, self.structure.as_slice()),
            (Role::Contracts, self.contracts.as_slice()),
            (Role::Limits, self.limits.as_slice()),
            (Role::E2e, self.e2e.as_slice()),
        ]
    }

    /// The declared replaceable roots, in declaration order.
    pub fn source_roots(&self) -> &[String] {
        &self.source
    }

    /// Whether the declaration applies to this project. The file is read only
    /// when the `ise` addon is effective.
    pub fn is_effective(manifest: &Manifest) -> bool {
        manifest.addons.ise
    }

    fn parse(content: &str) -> Result<Self, DeclarationError> {
        let declared = scan_top_level_keys(content)?;
        for role in Role::ALL {
            if !declared.iter().any(|key| *key == role.as_str()) {
                return Err(DeclarationError::MissingRole {
                    role: role.as_str(),
                });
            }
        }
        if !declared.contains(&SOURCE_KEY) {
            return Err(DeclarationError::MissingSource);
        }

        let document: DeclarationDocument =
            serde_yaml_ng::from_str(content).map_err(|error| DeclarationError::Malformed {
                reason: error.to_string(),
            })?;
        let declaration = SpecificationDeclaration {
            behavior: document.behavior,
            structure: document.structure,
            contracts: document.contracts,
            limits: document.limits,
            e2e: document.e2e,
            source: document.source,
        };
        declaration.validate_paths()?;
        Ok(declaration)
    }

    fn validate_paths(&self) -> Result<(), DeclarationError> {
        for (_, paths) in self.roles() {
            validate_role_paths(paths)?;
        }
        validate_role_paths(&self.source)
    }
}

/// The deserialization shape: six required string lists, nothing else.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeclarationDocument {
    behavior: Vec<String>,
    structure: Vec<String>,
    contracts: Vec<String>,
    limits: Vec<String>,
    e2e: Vec<String>,
    source: Vec<String>,
}

/// The top-level keys of the declaration, refusing an unknown or repeated one.
/// The schema is flat, so a line-initial `<identifier>:` is exact.
fn scan_top_level_keys(content: &str) -> Result<Vec<&str>, DeclarationError> {
    let mut keys: Vec<&str> = Vec::new();
    for line in content.lines() {
        let Some((candidate, _)) = line.split_once(':') else {
            continue;
        };
        if !is_key(candidate) {
            continue;
        }
        if !KEYS.contains(&candidate) {
            return Err(DeclarationError::UnknownKey {
                key: candidate.to_string(),
            });
        }
        if keys.contains(&candidate) {
            return Err(DeclarationError::RepeatedKey {
                key: candidate.to_string(),
            });
        }
        keys.push(candidate);
    }
    Ok(keys)
}

/// Whether a line prefix is a top-level key: an identifier at column zero.
fn is_key(candidate: &str) -> bool {
    let mut characters = candidate.chars();
    match characters.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn validate_role_paths(paths: &[String]) -> Result<(), DeclarationError> {
    for path in paths {
        if !is_project_relative(path) {
            return Err(DeclarationError::OutsideProject { path: path.clone() });
        }
    }
    Ok(())
}

/// Whether every declared value stays inside the project: non-empty, not
/// absolute (POSIX or Windows drive), and no `..` above the project root.
fn is_project_relative(value: &str) -> bool {
    if value.is_empty() || value.starts_with('/') || has_windows_prefix(value) {
        return false;
    }
    let mut depth = 0usize;
    for component in Path::new(value).components() {
        match component {
            Component::Normal(_) => depth += 1,
            Component::ParentDir if depth > 0 => depth -= 1,
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return false,
            Component::CurDir => {}
        }
    }
    true
}

fn has_windows_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration(entries: &[(&str, &str)]) -> String {
        entries
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn valid() -> Vec<(&'static str, &'static str)> {
        vec![
            ("behavior", r#"["givn/specs/**"]"#),
            ("structure", r#"["docs/glossary.md"]"#),
            ("contracts", r#"["docs/reference.md"]"#),
            ("limits", r#"["docs/arc42/02-architecture-constraints.md"]"#),
            ("e2e", r#"["tests/e2e"]"#),
            ("source", r#"["src/**"]"#),
        ]
    }

    #[test]
    fn every_role_and_the_source_list_are_required() {
        for index in 0..6 {
            let mut entries = valid();
            let (key, _) = entries.remove(index);
            let error = SpecificationDeclaration::parse(&declaration(&entries))
                .expect_err("a missing key is refused");
            if key == SOURCE_KEY {
                assert_eq!(error, DeclarationError::MissingSource);
            } else {
                assert_eq!(error, DeclarationError::MissingRole { role: key });
            }
        }
    }

    #[test]
    fn a_value_that_escapes_the_project_is_refused() {
        for escape in ["../other/crates/**", "/opt/specs/**", "C:/specs/**", ""] {
            let mut lines: Vec<String> = valid()
                .iter()
                .map(|(key, value)| format!("{key}: {value}"))
                .collect();
            lines[5] = format!("source: [\"{escape}\"]");
            let error = SpecificationDeclaration::parse(&lines.join("\n"))
                .expect_err("an outside-project value is refused");
            assert!(
                matches!(error, DeclarationError::OutsideProject { .. }),
                "{escape} produced {error}"
            );
        }
    }

    #[test]
    fn a_relative_value_with_inner_parent_segments_stays_inside() {
        assert!(is_project_relative("src/../src/lib.rs"));
        assert!(is_project_relative("givn/specs/**"));
        assert!(!is_project_relative("src/../../outside"));
    }

    #[test]
    fn the_refusal_message_names_the_file_and_the_offending_element() {
        let error = DeclarationError::MissingRole { role: "limits" };
        assert_eq!(
            error.to_string(),
            "givn/specification.yaml: missing role 'limits'"
        );
        assert_eq!(error.detail(), "missing role 'limits'");
    }
}
