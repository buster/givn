//! Path-resolving references under a matched path that resolves no package.
//!
//! The reader is deliberately shallow: it recognizes `mod`, `use`, `require`,
//! `import`, and `from ... import` declarations, and it refuses a reference
//! only when it reduces to a project path under a declared `source` root.
//! External package names, standard-library names, and syntax the reader does
//! not recognize are admitted (recorded as R-139).

use super::manifest::{normalize_relative, parent_directory};
use super::paths::{Glob, ProjectPath};

/// Read every file under `path` and return the distinct `(target, root)` pairs
/// whose reference reduces into a declared `source` root.
pub(crate) fn import_edges(
    project_root: &std::path::Path,
    path: &str,
    tree: &[ProjectPath],
    source_roots: &[(String, Glob)],
) -> Vec<(String, String)> {
    let mut edges: std::collections::BTreeSet<(String, String)> = std::collections::BTreeSet::new();
    for entry in tree
        .iter()
        .filter(|entry| !entry.is_dir && is_under(&entry.path, path))
    {
        let Ok(content) = std::fs::read_to_string(project_root.join(&entry.path)) else {
            continue;
        };
        for line in content.lines() {
            for reference in references(line) {
                if let Some(edge) = target_of(&reference, &entry.path, tree, source_roots) {
                    edges.insert(edge);
                }
            }
        }
    }
    edges.into_iter().collect()
}

/// Whether `candidate` is `root` or lies below it. The project root is `""`.
fn is_under(candidate: &str, root: &str) -> bool {
    root.is_empty()
        || candidate == root
        || candidate
            .strip_prefix(root)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// The reference texts one source line carries.
fn references(line: &str) -> Vec<String> {
    let trimmed = line.trim_start();
    let declaration = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
    let mut found = Vec::new();
    if let Some(rest) = declaration.strip_prefix("mod ") {
        let token = rest
            .split(|character: char| {
                character.is_whitespace() || character == ';' || character == '{'
            })
            .next()
            .unwrap_or_default();
        if !token.is_empty() {
            found.push(token.to_string());
        }
    }
    if let Some(rest) = declaration.strip_prefix("use ") {
        let path = rest.split(';').next().unwrap_or(rest).trim();
        if !path.is_empty() {
            found.push(path.to_string());
        }
    }
    if let Some(rest) = trimmed.strip_prefix("from ") {
        let module = rest.split(" import").next().unwrap_or(rest).trim();
        if !module.is_empty() {
            found.push(module.to_string());
        }
    }
    if (trimmed.contains("require") || trimmed.contains("import "))
        && let Some(quoted) = quoted_string(trimmed)
    {
        found.push(quoted);
    }
    if let Some(rest) = declaration.strip_prefix("import ") {
        let head = rest
            .split(|character: char| {
                character.is_whitespace() || character == ',' || character == ';'
            })
            .next()
            .unwrap_or_default();
        if !head.is_empty() && !head.starts_with(['"', '\'', '{', '*']) && !head.contains("from") {
            found.push(head.to_string());
        }
    }
    found
}

/// The first quoted string on the line, without its quotes.
fn quoted_string(line: &str) -> Option<String> {
    let start = line.find(['"', '\''])?;
    let quote = line[start..].chars().next()?;
    let rest = &line[start + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

/// Reduce one reference to a `(target, source_root)` pair, or `None` when it
/// names no project path, names one outside every `source` root, or uses
/// syntax the reader does not recognize.
fn target_of(
    reference: &str,
    file: &str,
    tree: &[ProjectPath],
    source_roots: &[(String, Glob)],
) -> Option<(String, String)> {
    let reference = reference.split('{').next().unwrap_or(reference).trim();
    if reference.is_empty() {
        return None;
    }
    if reference.starts_with('.') {
        return relative_target(reference, file, source_roots);
    }
    let segments: Vec<&str> = reference
        .trim_start_matches("::")
        .split("::")
        .flat_map(|part| part.split('.'))
        .filter(|segment| !segment.is_empty())
        .collect();
    let first = *segments.first()?;
    if matches!(first, "self" | "super") {
        let mut parts: Vec<&str> = Vec::new();
        for segment in &segments {
            match *segment {
                "self" => {}
                "super" => parts.push(".."),
                other => parts.push(other),
            }
        }
        if parts.is_empty() {
            return None;
        }
        return relative_target(&parts.join("/"), file, source_roots);
    }
    if first == "crate" {
        return None;
    }
    for end in (1..=segments.len()).rev() {
        let candidate = segments[..end].join("/");
        if !tree.iter().any(|entry| entry.path == candidate) {
            continue;
        }
        let (pattern, _) = source_roots
            .iter()
            .find(|(_, glob)| glob.matches(&candidate))?;
        return Some((candidate, pattern.clone()));
    }
    None
}

/// Resolve a relative reference against the file's directory and refuse it
/// when the resolved path lies under a `source` root.
fn relative_target(
    reference: &str,
    file: &str,
    source_roots: &[(String, Glob)],
) -> Option<(String, String)> {
    let resolved = normalize_relative(&parent_directory(file), reference)?;
    let (pattern, _) = source_roots
        .iter()
        .find(|(_, glob)| glob.matches(&resolved))?;
    Some((resolved, pattern.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roots() -> Vec<(String, Glob)> {
        vec![("src/**".to_string(), Glob::compile("src/**"))]
    }

    fn tree(paths: &[(&str, bool)]) -> Vec<ProjectPath> {
        paths
            .iter()
            .map(|(path, is_dir)| ProjectPath {
                path: (*path).to_string(),
                is_dir: *is_dir,
            })
            .collect()
    }

    #[test]
    fn a_relative_reference_that_escapes_into_source_is_refused() {
        let entries = tree(&[("tests/e2e", true), ("tests/e2e/steps.rs", false)]);
        let target = target_of(
            "../../src/helpers",
            "tests/e2e/steps.rs",
            &entries,
            &roots(),
        );
        assert_eq!(
            target,
            Some(("src/helpers".to_string(), "src/**".to_string()))
        );
    }

    #[test]
    fn a_qualified_external_reference_is_admitted() {
        let entries = tree(&[("givn", true), ("tests/e2e", true)]);
        assert_eq!(
            target_of(
                "givn::proof::ChangeReceipt",
                "tests/e2e/steps.rs",
                &entries,
                &roots()
            ),
            None
        );
    }

    #[test]
    fn a_qualified_reference_naming_a_source_path_is_refused() {
        let entries = tree(&[("src", true), ("tests/e2e", true)]);
        assert_eq!(
            target_of("src::helpers", "tests/e2e/steps.rs", &entries, &roots()),
            Some(("src".to_string(), "src/**".to_string()))
        );
    }

    #[test]
    fn a_module_declaration_is_read() {
        assert_eq!(
            references("mod ../../src/helpers;"),
            vec!["../../src/helpers"]
        );
        assert!(references("pub use givn::proof;").contains(&"givn::proof".to_string()));
    }
}
