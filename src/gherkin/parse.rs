use gherkin::{Feature, GherkinEnv};
#[cfg(test)]
use gherkin::ParseError;
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
            let message = format!("{e}");
            ParseResult {
                feature: None,
                errors: vec![LintError {
                    kind: LintKind::ParseError,
                    message,
                    line: None,
                }],
            }
        }
    }
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
                message: format!(
                    "scenario '{}' is tagged @wip",
                    scenario.name
                ),
                line: Some(scenario.position.line),
            });
        }
        for step in &scenario.steps {
            if step.value.trim() == "PENDING" {
                errors.push(LintError {
                    kind: LintKind::Wip,
                    message: format!(
                        "scenario '{}' has a PENDING step: '{}'",
                        scenario.name,
                        step.value
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_feature() {
        let input = r#"
Feature: Hello world
  Scenario: Greet
    Given a user
    When they say hello
    Then they receive a greeting
"#;
        let result = parse_str(input);
        assert!(result.is_ok(), "valid feature should parse: {:?}", result);
        let feature = result.unwrap();
        assert_eq!(feature.name, "Hello world");
        assert_eq!(feature.scenarios.len(), 1);
    }

    #[test]
    fn detect_wip_scenario() {
        let input = r#"
Feature: Wip test
  @wip
  Scenario: Not done yet
    Given something
    When it happens
    Then nothing
"#;
        let feature = parse_str(input).expect("should parse");
        let errors = collect_wip(&feature);
        assert!(!errors.is_empty(), "should detect @wip");
        assert_eq!(errors[0].kind, LintKind::Wip);
    }
}
