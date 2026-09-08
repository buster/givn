//! Capability-aware interaction/E2E coverage checks for use-case documents.
//!
//! Each interaction belongs to one capability and names exactly one `@e2e`
//! scenario. Summary use cases do not duplicate child inventories; fragments
//! retain their own behavior and are not copied into including use cases.

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use gherkin::GherkinEnv;
use serde::Serialize;

use crate::overlap::analysis::Finding;

#[derive(Debug, Clone, Serialize)]
pub struct UsecaseCoverage {
    pub usecase: String,
    pub capability: String,
    pub declared_interactions: usize,
    pub e2e_scenarios: usize,
    pub declared_e2e_scenarios: Vec<String>,
    pub actual_e2e_scenarios: Vec<String>,
}

fn is_e2e(scenario: &gherkin::Scenario) -> bool {
    scenario.tags.iter().any(|tag| tag == "e2e")
}

#[allow(dead_code)]
pub fn count_e2e(feature: &gherkin::Feature) -> usize {
    feature
        .scenarios
        .iter()
        .filter(|scenario| is_e2e(scenario))
        .count()
}

/// Build coverage rows for every capability declared by a leaf or summary use
/// case. A missing use-case document is reported by the relationship/lint
/// layer, not treated as a zero-interaction document here.
pub fn usecase_coverages(root: &Path) -> Result<Vec<UsecaseCoverage>> {
    let specs_root = root.join("givn/specs");
    let mut out = Vec::new();
    let entries = match std::fs::read_dir(&specs_root) {
        Ok(entries) => entries,
        Err(_) => return Ok(out),
    };

    for entry in entries {
        let usecase_dir = entry?.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        let usecase = usecase_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let usecase_path = usecase_dir.join("usecase.md");
        if !usecase_path.exists() {
            continue;
        }
        let doc =
            crate::gherkin::usecase_md::parse_usecase_md(&std::fs::read_to_string(&usecase_path)?)?;

        for capability in doc.capabilities {
            let feature_path = usecase_dir.join(format!("{capability}.feature"));
            let feature = gherkin::Feature::parse_path(&feature_path, GherkinEnv::default())
                .map_err(|error| {
                    anyhow::anyhow!("failed to parse {}: {error}", feature_path.display())
                })?;
            let declared: Vec<String> = doc
                .interactions
                .iter()
                .filter(|interaction| interaction.capability == capability)
                .map(|interaction| interaction.e2e_scenario.clone())
                .collect();
            let actual: Vec<String> = feature
                .scenarios
                .iter()
                .filter(|scenario| is_e2e(scenario))
                .map(|scenario| scenario.name.clone())
                .collect();
            out.push(UsecaseCoverage {
                usecase: usecase.clone(),
                capability,
                declared_interactions: declared.len(),
                e2e_scenarios: actual.len(),
                declared_e2e_scenarios: declared,
                actual_e2e_scenarios: actual,
            });
        }
    }
    out.sort_by(|left, right| {
        left.usecase
            .cmp(&right.usecase)
            .then_with(|| left.capability.cmp(&right.capability))
    });
    Ok(out)
}

/// Compatibility-free active API. The old group coverage function is gone;
/// callers must use the use-case vocabulary.
pub fn interaction_gate_findings(root: &Path) -> Vec<Finding> {
    let coverages = match usecase_coverages(root) {
        Ok(coverages) => coverages,
        Err(error) => {
            return vec![Finding {
                kind: "usecase_interaction_invalid".to_string(),
                level: "error".to_string(),
                files: vec![root.join("givn/specs").display().to_string()],
                titles: vec![error.to_string()],
                contradiction: false,
            }];
        }
    };

    let mut findings = Vec::new();
    for coverage in coverages {
        let declared: BTreeSet<&str> = coverage
            .declared_e2e_scenarios
            .iter()
            .map(String::as_str)
            .collect();
        let actual: BTreeSet<&str> = coverage
            .actual_e2e_scenarios
            .iter()
            .map(String::as_str)
            .collect();
        if declared == actual {
            continue;
        }
        let missing: Vec<&str> = declared.difference(&actual).copied().collect();
        let extra: Vec<&str> = actual.difference(&declared).copied().collect();
        let detail = format!(
            "missing declared scenarios [{}]; unlisted @e2e scenarios [{}]",
            missing.join(", "),
            extra.join(", ")
        );
        findings.push(Finding {
            kind: "interaction_mismatch".to_string(),
            level: "error".to_string(),
            files: vec![
                root.join("givn/specs")
                    .join(&coverage.usecase)
                    .join("usecase.md")
                    .display()
                    .to_string(),
                root.join("givn/specs")
                    .join(&coverage.usecase)
                    .join(format!("{}.feature", coverage.capability))
                    .display()
                    .to_string(),
            ],
            titles: vec![format!(
                "use case '{}' capability '{}' declares {} interaction(s) and has {} @e2e scenario(s) — {detail}",
                coverage.usecase,
                coverage.capability,
                coverage.declared_interactions,
                coverage.e2e_scenarios,
            )],
            contradiction: false,
        });
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_usecase(feature_text: &str, interactions: &str) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let usecase = root.path().join("givn/specs/project-run");
        std::fs::create_dir_all(&usecase).unwrap();
        std::fs::write(usecase.join("usecase.md"), interactions).unwrap();
        std::fs::write(usecase.join("project-run.feature"), feature_text).unwrap();
        root
    }

    const USECASE: &str = r#"# Use case: project-run

## Level

!

## Actors

- Operator

## Goal

Run a project.

## Trigger

The operator starts it.

## Preconditions

- The project exists.

## Main flow

1. Start.

## Extensions

- none

## Rules

- The run has one authority.

## Examples

- A clean run.

## Minimal guarantee

The project remains inspectable.

## Success guarantee

The run starts.

## Personas

- none

## Capabilities

- project-run

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| project-run | start a project | Consumer starts a project |

## Includes

- none

## Extends

- none

## Out of scope

Scheduling.

## Diagram

```mermaid
flowchart LR
  A --> B
```
"#;

    #[test]
    fn matching_capability_interaction_passes() {
        let root = temp_usecase(
            "Feature: Run\n\n  @e2e\n  Scenario: Consumer starts a project\n    Given x\n    Then y\n",
            USECASE,
        );
        assert!(interaction_gate_findings(root.path()).is_empty());
        let rows = usecase_coverages(root.path()).unwrap();
        assert_eq!(rows[0].usecase, "project-run");
        assert_eq!(rows[0].capability, "project-run");
    }

    #[test]
    fn missing_interaction_mapping_is_an_error() {
        let root = temp_usecase(
            "Feature: Run\n\n  @e2e\n  Scenario: Other\n    Given x\n    Then y\n",
            USECASE,
        );
        let findings = interaction_gate_findings(root.path());
        assert_eq!(findings.len(), 1);
        assert!(findings[0].titles[0].contains("missing declared scenarios"));
    }

    #[test]
    fn tagged_outline_counts_once() {
        let root = temp_usecase(
            "Feature: Run\n\n  @e2e\n  Scenario Outline: Consumer starts a project\n    Given <kind>\n    Then y\n\n    Examples:\n      | kind |\n      | clean |\n      | dirty |\n",
            USECASE,
        );
        let rows = usecase_coverages(root.path()).unwrap();
        assert_eq!(rows[0].e2e_scenarios, 1);
    }
}
