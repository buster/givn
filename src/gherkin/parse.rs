#[cfg(test)]
use gherkin::ParseError;
use gherkin::{Feature, GherkinEnv, ParseFileError};
use std::path::Path;

/// Result of parsing a single .feature file for lint purposes.
#[derive(Debug)]
pub struct ParseResult {
    pub feature: Option<Feature>,
    pub errors: Vec<LintError>,
}

/// A lint finding from a .feature file.
#[derive(Debug)]
pub struct LintError {
    pub kind: LintKind,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintKind {
    /// Parse error (highest severity — exit code 1).
    ParseError,
    /// @wip or PENDING (exit code 2).
    Wip,
}

/// Parse a .feature file and return lint results.
pub fn parse_file(path: &Path) -> ParseResult {
    let env = GherkinEnv::default();
    match Feature::parse_path(path, env) {
        Ok(feature) => {
            let errors = collect_wip(&feature);
            ParseResult {
                feature: Some(feature),
                errors,
            }
        }
        Err(e) => {
            let (message, line) = build_parse_error(path, &e);
            ParseResult {
                feature: None,
                errors: vec![LintError {
                    kind: LintKind::ParseError,
                    message,
                    line,
                }],
            }
        }
    }
}

/// Build a human-friendly parse-error message with the offending source line
/// and a hint when we can detect a common mistake.
fn build_parse_error(path: &Path, err: &ParseFileError) -> (String, Option<usize>) {
    let ParseFileError::Parsing { source, .. } = err else {
        // I/O error — nothing extra to show.
        return (format!("{err}"), None);
    };

    // ParseError::Display gives us "Error at <line>:<col>: {expected:?}".
    let inner = format!("{source}");

    // Try to extract the line number from the display string.
    let error_line = parse_error_line(&inner);

    // Try to read the offending line from the file.
    let source_line = error_line.and_then(|n| read_source_line(path, n));

    let mut msg = inner.clone();

    if let Some(ref text) = source_line {
        msg.push_str(&format!("\n       | {}", text.trim()));
    }

    // Detect common mistakes and add a targeted hint.
    if let Some(hint) = detect_hint(error_line, path) {
        msg.push_str(&format!("\n  hint: {hint}"));
    }

    (msg, error_line)
}

/// Extract the line number from a ParseError display string like
/// "Error at 11:5: {...}".
fn parse_error_line(display: &str) -> Option<usize> {
    let rest = display.strip_prefix("Error at ")?;
    let colon = rest.find(':')?;
    rest[..colon].parse().ok()
}

/// Read a single source line from a file (1-indexed).
fn read_source_line(path: &Path, line_number: usize) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    content
        .lines()
        .nth(line_number.saturating_sub(1))
        .map(String::from)
}

/// Read all lines from a file, returning them 0-indexed.
fn read_all_lines(path: &Path) -> Option<Vec<String>> {
    let content = std::fs::read_to_string(path).ok()?;
    Some(content.lines().map(String::from).collect())
}

/// Detect common Gherkin mistakes and return a targeted hint.
///
/// The parser often reports the error on the line *after* the actual problem
/// (e.g. a misplaced tag causes the next step line to fail). We scan the
/// neighbourhood of the error line for patterns we recognise.
fn detect_hint(error_line: Option<usize>, path: &Path) -> Option<String> {
    let line_num = error_line?;
    let lines = read_all_lines(path)?;

    // Scan backwards from the error line to find a Scenario: keyword, looking
    // for any @-prefixed line sitting between Scenario: and the error.
    for i in (0..line_num.saturating_sub(1)).rev() {
        let trimmed = lines[i].trim();

        if trimmed.starts_with("Scenario:") || trimmed.starts_with("Scenario Outline:") {
            // We hit the Scenario line. Check if any line between it and the
            // error line starts with '@'.
            let has_misplaced_tag = lines[(i + 1)..line_num.saturating_sub(1)]
                .iter()
                .any(|l| l.trim().starts_with('@'));
            if has_misplaced_tag {
                return Some(
                    "tags must appear on the line BEFORE `Scenario:`, not after it".to_string(),
                );
            }
            break;
        }

        // If the error line itself is a @-tag right after Scenario:
        if trimmed.starts_with('@') && i > 0 {
            let prev = lines[i - 1].trim();
            if prev.starts_with("Scenario:") || prev.starts_with("Scenario Outline:") {
                return Some(
                    "tags must appear on the line BEFORE `Scenario:`, not after it".to_string(),
                );
            }
        }

        // Stop scanning if we hit another structural keyword.
        if trimmed.starts_with("Feature:")
            || trimmed.starts_with("Background:")
            || trimmed.starts_with("Rule:")
        {
            break;
        }
    }

    None
}

/// Parse a .feature string (for testing).
#[cfg(test)]
pub fn parse_str(input: &str) -> Result<Feature, ParseError> {
    let env = GherkinEnv::default();
    Feature::parse(input, env)
}

/// Collect @wip scenarios (and PENDING steps) as lint findings.
fn collect_wip(feature: &Feature) -> Vec<LintError> {
    let mut errors = Vec::new();
    for scenario in &feature.scenarios {
        if scenario.tags.iter().any(|t| t == "wip") {
            errors.push(LintError {
                kind: LintKind::Wip,
                message: format!("scenario '{}' is tagged @wip", scenario.name),
                line: Some(scenario.position.line),
            });
        }
        for step in &scenario.steps {
            if step.value.trim() == "PENDING" {
                errors.push(LintError {
                    kind: LintKind::Wip,
                    message: format!(
                        "scenario '{}' has a PENDING step: '{}'",
                        scenario.name, step.value
                    ),
                    line: Some(step.position.line),
                });
            }
        }
    }
    errors
}

/// Collect all .feature files under a directory recursively.
pub fn collect_feature_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if !dir.exists() {
        return files;
    }
    collect_recursive(dir, &mut files);
    files.sort();
    files
}

fn collect_recursive(dir: &Path, acc: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_recursive(&path, acc);
            } else if path.extension().map(|e| e == "feature").unwrap_or(false) {
                acc.push(path);
            }
        }
    }
}
