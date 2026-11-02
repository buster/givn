//! Deterministic production-source discovery.

use super::model::{Language, LineKind, RepoPath};
use anyhow::{Context, Result};
use std::path::{Component, Path, PathBuf};

/// Test and tooling directories excluded for every language.
const EXCLUDED_DIRS: &[&str] = &["tests", "benches", "examples", "target", ".git"];

/// Python-only tooling directories; a Rust source tree may legitimately use
/// names such as `build/`, so the exclusion is language-specific.
const PYTHON_EXCLUDED_DIRS: &[&str] = &[
    "__pycache__",
    ".venv",
    "venv",
    ".tox",
    ".mypy_cache",
    ".pytest_cache",
    "build",
    "dist",
    "node_modules",
];

/// Java build, generated, and test roots.
const JAVA_EXCLUDED_DIRS: &[&str] = &["build", "out", "generated", "test"];

/// TypeScript and JavaScript build, dependency, and test roots.
const ECMASCRIPT_EXCLUDED_DIRS: &[&str] = &[
    "node_modules",
    "dist",
    "build",
    ".next",
    "coverage",
    "out",
    "__tests__",
];

/// Source extensions that are production code but outside the supported
/// supported analyzers.
const UNSUPPORTED_SOURCE_EXTENSIONS: &[&str] = &["go", "jsx"];

#[derive(Clone, Debug)]
pub struct ScannedSource {
    pub path: RepoPath,
    pub content: String,
    pub lines: Vec<LineKind>,
}

/// Discover production sources for the requested languages under the roots.
///
/// Roots are project-root-relative (already validated by the CLI). Test and
/// tooling directories are excluded wherever they occur. Paths are sorted and
/// deduplicated, so overlapping roots analyze each file once.
pub fn discover(
    project_root: &Path,
    roots: &[PathBuf],
    languages: &[Language],
) -> Result<Vec<ScannedSource>> {
    let canonical_root = project_root
        .canonicalize()
        .with_context(|| format!("resolve project root {}", project_root.display()))?;
    let mut files = Vec::new();

    for root in roots {
        let path = project_root.join(root);
        if !path.exists() {
            continue;
        }
        walk(&canonical_root, &path, languages, &mut files)?;
    }

    files.sort_by(|a, b| a.absolute.cmp(&b.absolute));
    files.dedup_by(|a, b| a.absolute == b.absolute);

    let mut sources = Vec::with_capacity(files.len());
    for path in files {
        let content = match std::fs::read_to_string(&path.absolute) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let lines = content
            .lines()
            .map(|line| classify_line(line, path.language))
            .collect();
        let relative = path
            .absolute
            .strip_prefix(&canonical_root)
            .unwrap_or(&path.absolute);
        sources.push(ScannedSource {
            path: RepoPath::new(relative.to_string_lossy().into_owned()),
            content,
            lines,
        });
    }
    Ok(sources)
}

/// Discover unsupported production source files under the requested roots.
///
/// Used only to distinguish an empty scope from a scope that contains sources
/// no analyzer can read; it never feeds analysis.
pub fn unsupported_production_sources(
    project_root: &Path,
    roots: &[PathBuf],
) -> Result<Vec<RepoPath>> {
    let canonical_root = project_root
        .canonicalize()
        .with_context(|| format!("resolve project root {}", project_root.display()))?;
    let mut files = Vec::new();
    for root in roots {
        let path = project_root.join(root);
        if path.exists() {
            walk_unsupported(&canonical_root, &path, &mut files)?;
        }
    }
    files.sort();
    files.dedup();
    Ok(files
        .iter()
        .map(|path| {
            let relative = path.strip_prefix(&canonical_root).unwrap_or(path);
            RepoPath::new(relative.to_string_lossy().into_owned())
        })
        .collect())
}

fn walk_unsupported(project_root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let canonical = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => return Ok(()),
    };
    if !canonical.starts_with(project_root) {
        return Ok(());
    }
    if canonical.is_file() {
        if is_unsupported_source(&canonical) {
            out.push(canonical);
        }
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&canonical)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if EXCLUDED_DIRS.contains(&name.as_ref()) || name.as_ref() == "node_modules" {
                continue;
            }
            walk_unsupported(project_root, &path, out)?;
        } else if is_unsupported_source(&path) {
            walk_unsupported(project_root, &path, out)?;
        }
    }
    Ok(())
}

fn is_unsupported_source(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return false;
    };
    if !UNSUPPORTED_SOURCE_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        return false;
    }
    !path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("_test.go"))
}

fn walk(
    project_root: &Path,
    dir: &Path,
    languages: &[Language],
    out: &mut Vec<ScannedSourcePath>,
) -> Result<()> {
    let canonical = match dir.canonicalize() {
        Ok(path) => path,
        Err(_) => return Ok(()),
    };
    if !canonical.starts_with(project_root) {
        return Ok(());
    }
    if canonical.is_file() {
        let relative = canonical.strip_prefix(project_root).unwrap_or(&canonical);
        if let Some(language) = language_for_path(relative, languages) {
            out.push(ScannedSourcePath {
                absolute: canonical,
                language,
            });
        }
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&canonical)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if languages
                .iter()
                .all(|language| excluded_dir(&name, *language))
            {
                continue;
            }
            walk(project_root, &path, languages, out)?;
        } else if language_for_path(
            path.strip_prefix(project_root).unwrap_or(path.as_path()),
            languages,
        )
        .is_some()
        {
            walk(project_root, &path, languages, out)?;
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct ScannedSourcePath {
    absolute: PathBuf,
    language: Language,
}

fn excluded_dir(name: &str, language: Language) -> bool {
    EXCLUDED_DIRS.contains(&name)
        || (language == Language::Python && PYTHON_EXCLUDED_DIRS.contains(&name))
        || (language == Language::Java && JAVA_EXCLUDED_DIRS.contains(&name))
        || (matches!(language, Language::TypeScript | Language::JavaScript)
            && ECMASCRIPT_EXCLUDED_DIRS.contains(&name))
}

/// Classify a source file that is relative to the project root.
///
/// Excluded directory names are matched against the path relative to the
/// project root only, so an absolute checkout path such as `/build/...` never
/// hides otherwise valid sources.
fn language_for_path(path: &Path, languages: &[Language]) -> Option<Language> {
    let extension = path.extension()?.to_str()?;
    let name = path.file_name()?.to_str()?;
    let language = match extension {
        "rs" if languages.contains(&Language::Rust) => Language::Rust,
        "py" if languages.contains(&Language::Python) => {
            if name.starts_with("test_") || name.ends_with("_test.py") {
                return None;
            }
            Language::Python
        }
        "java" if languages.contains(&Language::Java) => {
            if name.ends_with("Test.java") || name.ends_with("Tests.java") {
                return None;
            }
            Language::Java
        }
        "ts" | "tsx" if languages.contains(&Language::TypeScript) => {
            if name.ends_with(".d.ts")
                || name.ends_with(".test.ts")
                || name.ends_with(".spec.ts")
                || name.ends_with(".test.tsx")
                || name.ends_with(".spec.tsx")
            {
                return None;
            }
            Language::TypeScript
        }
        "js" | "mjs" | "cjs" if languages.contains(&Language::JavaScript) => {
            if ["test", "spec"]
                .iter()
                .any(|kind| name.ends_with(&format!(".{kind}.{extension}")))
            {
                return None;
            }
            Language::JavaScript
        }
        _ => return None,
    };
    if path.components().any(|component| match component {
        Component::Normal(name) => excluded_dir(&name.to_string_lossy(), language),
        _ => false,
    }) {
        return None;
    }
    Some(language)
}

fn classify_line(line: &str, language: Language) -> LineKind {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return LineKind::Blank;
    }
    match language {
        Language::Rust => {
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                LineKind::Comment
            } else {
                LineKind::Code
            }
        }
        Language::Python => {
            if trimmed.starts_with('#') {
                LineKind::Comment
            } else {
                LineKind::Code
            }
        }
        Language::Java | Language::TypeScript | Language::JavaScript => {
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                LineKind::Comment
            } else {
                LineKind::Code
            }
        }
    }
}

/// Count code lines in a span union, excluding recorded non-production spans.
pub fn code_lines_excluding(
    lines: &[LineKind],
    spans: &[super::model::SourceSpan],
    excluded: &[super::model::SourceSpan],
    file: &RepoPath,
) -> u32 {
    marked_lines(lines.len(), spans, excluded, file)
        .iter()
        .enumerate()
        .filter(|(index, marked)| **marked && lines.get(*index) == Some(&LineKind::Code))
        .count() as u32
}

/// Count lines in a span union, excluding recorded non-production spans.
pub fn span_lines_excluding(
    line_count: usize,
    spans: &[super::model::SourceSpan],
    excluded: &[super::model::SourceSpan],
    file: &RepoPath,
) -> u32 {
    marked_lines(line_count, spans, excluded, file)
        .iter()
        .filter(|marked| **marked)
        .count() as u32
}

/// Count code lines outside recorded non-production spans.
pub fn production_code_lines(
    lines: &[LineKind],
    excluded: &[super::model::SourceSpan],
    file: &RepoPath,
) -> u32 {
    let all = [super::model::SourceSpan {
        file: file.clone(),
        start_line: 1,
        end_line: lines.len().max(1) as u32,
    }];
    code_lines_excluding(lines, &all, excluded, file)
}

fn marked_lines(
    line_count: usize,
    spans: &[super::model::SourceSpan],
    excluded: &[super::model::SourceSpan],
    file: &RepoPath,
) -> Vec<bool> {
    let mut marked = vec![false; line_count];
    for span in spans.iter().filter(|span| &span.file == file) {
        let start = span.start_line.saturating_sub(1) as usize;
        let end = span.end_line.saturating_sub(1) as usize;
        for marked_line in marked
            .iter_mut()
            .take(end.min(line_count.saturating_sub(1)) + 1)
            .skip(start)
        {
            *marked_line = true;
        }
    }
    for span in excluded.iter().filter(|span| &span.file == file) {
        let start = span.start_line.saturating_sub(1) as usize;
        let end = span.end_line.saturating_sub(1) as usize;
        for marked_line in marked
            .iter_mut()
            .take(end.min(line_count.saturating_sub(1)) + 1)
            .skip(start)
        {
            *marked_line = false;
        }
    }
    marked
}

#[cfg(test)]
mod tests {
    use super::*;

    fn language(path: &str, languages: &[Language]) -> Option<Language> {
        language_for_path(Path::new(path), languages)
    }

    #[test]
    fn production_scope_is_language_specific() {
        assert_eq!(
            language("src/build/engine.rs", &[Language::Rust]),
            Some(Language::Rust)
        );
        assert_eq!(language("src/build/engine.py", &[Language::Python]), None);
        assert_eq!(
            language("src/.venv/lib/helper.py", &[Language::Python]),
            None
        );
        assert_eq!(language("src/test_helper.py", &[Language::Python]), None);
        assert_eq!(language("src/helper_test.py", &[Language::Python]), None);
        assert_eq!(
            language("src/helper.py", &[Language::Python]),
            Some(Language::Python)
        );
        assert_eq!(
            language("src/helper.js", &[Language::JavaScript]),
            Some(Language::JavaScript)
        );
        assert_eq!(
            language("src/helper.mjs", &[Language::JavaScript]),
            Some(Language::JavaScript)
        );
        assert_eq!(
            language("src/helper.cjs", &[Language::JavaScript]),
            Some(Language::JavaScript)
        );
        assert_eq!(
            language("src/helper.test.js", &[Language::JavaScript]),
            None
        );
        assert_eq!(
            language("src/helper.spec.mjs", &[Language::JavaScript]),
            None
        );
        assert_eq!(
            language("node_modules/helper.js", &[Language::JavaScript]),
            None
        );
        assert_eq!(language("src/helper.js", &[Language::TypeScript]), None);
        assert_eq!(language("tests/helper.rs", &[Language::Rust]), None);
    }

    #[test]
    fn discovery_ignores_excluded_names_above_the_project_root() {
        let sandbox = tempfile::tempdir().expect("temp dir");
        let project_root = sandbox.path().join("build").join("project");
        for (path, content) in [
            ("src/widget.ts", "export function widget() {}\n"),
            ("src/Widget.java", "public class Widget {}\n"),
        ] {
            let target = project_root.join(path);
            std::fs::create_dir_all(target.parent().unwrap()).expect("source dir");
            std::fs::write(target, content).expect("source file");
        }
        let discovered = discover(
            &project_root,
            &[PathBuf::from("src")],
            &[Language::Java, Language::TypeScript],
        )
        .expect("discover");
        let mut paths: Vec<_> = discovered
            .iter()
            .map(|source| source.path.as_str().to_string())
            .collect();
        paths.sort();
        assert_eq!(paths, vec!["src/Widget.java", "src/widget.ts"]);
    }
}
