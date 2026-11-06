//! Capability-aware interaction/E2E coverage checks for use-case documents.
//!
//! Each interaction belongs to one capability and names exactly one `@e2e`
//! scenario. Summary use cases do not duplicate child inventories; fragments
//! retain their own behavior and are not copied into including use cases.
//! The gate reads a `SpecCorpus`, so the same check runs on the permanent
//! corpus and on a change's projected corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use gherkin::GherkinEnv;
use serde::Serialize;

use crate::overlap::analysis::Finding;
use crate::spec::corpus::SpecCorpus;

#[derive(Debug, Clone, Serialize)]
pub struct UsecaseCoverage {
    pub usecase: String,
    pub capability: String,
    pub declared_interactions: usize,
    pub e2e_scenarios: usize,
    pub declared_e2e_scenarios: Vec<String>,
    pub actual_e2e_scenarios: Vec<String>,
}

struct UsecaseDeclaration {
    narrative: PathBuf,
    capabilities: Vec<String>,
    interactions: Vec<crate::gherkin::usecase_md::Interaction>,
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

/// Parse every narrative in the corpus into its declared capabilities and
/// interactions. Parse errors are returned so the gate can report one
/// `usecase_interaction_invalid` finding.
fn usecase_declarations(corpus: &SpecCorpus) -> Result<BTreeMap<String, UsecaseDeclaration>> {
    let mut declarations = BTreeMap::new();
    for (key, content) in &corpus.narratives {
        let Some(usecase) = key
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
        else {
            continue;
        };
        let is_fragment = key.file_name().is_some_and(|name| name == "fragment.md");
        let (capabilities, interactions) = if is_fragment {
            let doc = crate::gherkin::usecase_md::parse_fragment_md(content)
                .map_err(|error| anyhow!("invalid fragment '{}': {error}", key.display()))?;
            (doc.capabilities, doc.interactions)
        } else {
            let doc = crate::gherkin::usecase_md::parse_usecase_md(content)
                .map_err(|error| anyhow!("invalid use case '{}': {error}", key.display()))?;
            (doc.capabilities, doc.interactions)
        };
        declarations.insert(
            usecase.to_string(),
            UsecaseDeclaration {
                narrative: key.clone(),
                capabilities,
                interactions,
            },
        );
    }
    Ok(declarations)
}

/// Build coverage rows for every capability declared by a leaf or summary use
/// case. A declared capability whose feature is absent is an error; the gate
/// maps it to a blocking `usecase_interaction_invalid` finding.
pub fn usecase_coverages(corpus: &SpecCorpus) -> Result<Vec<UsecaseCoverage>> {
    let declarations = usecase_declarations(corpus)?;
    let mut out = Vec::new();
    for (usecase, declaration) in declarations {
        for capability in declaration.capabilities {
            let feature_key = PathBuf::from("givn/specs")
                .join(&usecase)
                .join(format!("{capability}.feature"));
            let Some(text) = corpus.features.get(&feature_key) else {
                return Err(anyhow!(
                    "declared capability '{capability}' has no feature at '{}'",
                    feature_key.display()
                ));
            };
            let feature = gherkin::Feature::parse(text, GherkinEnv::default())
                .map_err(|error| anyhow!("failed to parse {}: {error}", feature_key.display()))?;
            let declared: Vec<String> = declaration
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

/// The one interaction-inventory gate. It reports missing/unlisted E2E
/// interactions and features outside the declared capability list. The gate is
/// infallible after `SpecCorpus::read`; document/feature parse errors become
/// blocking `usecase_interaction_invalid` findings.
pub fn interaction_gate_findings(corpus: &SpecCorpus) -> Vec<Finding> {
    let declarations = match usecase_declarations(corpus) {
        Ok(declarations) => declarations,
        Err(error) => return vec![usecase_invalid(error.to_string())],
    };
    let coverages = match usecase_coverages(corpus) {
        Ok(coverages) => coverages,
        Err(error) => return vec![usecase_invalid(error.to_string())],
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
        let mut title = format!(
            "interaction inventory mismatch: use case '{}' capability '{}' declares {} interaction(s) and has {} @e2e scenario(s)",
            coverage.usecase,
            coverage.capability,
            coverage.declared_interactions,
            coverage.e2e_scenarios
        );
        for scenario in missing {
            title.push_str(&format!("; missing @e2e scenario '{scenario}'"));
        }
        for scenario in extra {
            title.push_str(&format!("; unlisted @e2e scenario '{scenario}'"));
        }
        findings.push(Finding {
            kind: "interaction_mismatch".to_string(),
            level: "error".to_string(),
            files: vec![
                narrative_key(&coverage.usecase).display().to_string(),
                feature_key(&coverage.usecase, &coverage.capability)
                    .display()
                    .to_string(),
            ],
            titles: vec![title],
            contradiction: false,
        });
    }

    findings.extend(undeclared_findings(corpus, &declarations));
    findings
}

/// Report every direct feature file whose capability is absent from the owning
/// document's `## Capabilities` list, including a feature directory with no
/// document at all.
fn undeclared_findings(
    corpus: &SpecCorpus,
    declarations: &BTreeMap<String, UsecaseDeclaration>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for feature in corpus.features.keys() {
        let Some((usecase, capability)) = direct_capability(feature) else {
            continue;
        };
        let declared = declarations
            .get(&usecase)
            .is_some_and(|declaration| declaration.capabilities.contains(&capability));
        if declared {
            continue;
        }
        let Some(text) = corpus.features.get(feature) else {
            continue;
        };
        let e2e_titles = match gherkin::Feature::parse(text, GherkinEnv::default()) {
            Ok(parsed) => parsed
                .scenarios
                .iter()
                .filter(|scenario| is_e2e(scenario))
                .map(|scenario| scenario.name.clone())
                .collect::<Vec<_>>(),
            Err(error) => {
                return vec![usecase_invalid(format!(
                    "failed to parse {}: {error}",
                    feature.display()
                ))];
            }
        };
        let mut title = format!(
            "interaction inventory mismatch: use case '{usecase}' has undeclared capability '{capability}'"
        );
        if !e2e_titles.is_empty() {
            title.push_str(&format!(
                " with @e2e scenario(s) {}",
                e2e_titles
                    .iter()
                    .map(|title| format!("'{title}'"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        let mut files = vec![feature.display().to_string()];
        if let Some(declaration) = declarations.get(&usecase) {
            files.insert(0, declaration.narrative.display().to_string());
        }
        findings.push(Finding {
            kind: "undeclared_capability".to_string(),
            level: "error".to_string(),
            files,
            titles: vec![title],
            contradiction: false,
        });
    }
    findings
}

fn direct_capability(feature: &Path) -> Option<(String, String)> {
    let relative = feature.strip_prefix("givn/specs").ok()?;
    let mut components = relative.components();
    let usecase = components.next()?.as_os_str().to_str()?.to_string();
    let file = components.next()?.as_os_str().to_str()?;
    if components.next().is_some() {
        return None;
    }
    let capability = file.strip_suffix(".feature")?.to_string();
    Some((usecase, capability))
}

fn narrative_key(usecase: &str) -> PathBuf {
    PathBuf::from("givn/specs")
        .join(usecase)
        .join(if usecase == "fragments" {
            "fragment.md"
        } else {
            "usecase.md"
        })
}

fn feature_key(usecase: &str, capability: &str) -> PathBuf {
    PathBuf::from("givn/specs")
        .join(usecase)
        .join(format!("{capability}.feature"))
}

fn usecase_invalid(message: String) -> Finding {
    Finding {
        kind: "usecase_interaction_invalid".to_string(),
        level: "error".to_string(),
        files: vec!["givn/specs".to_string()],
        titles: vec![message],
        contradiction: false,
    }
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

    fn corpus(root: &Path) -> SpecCorpus {
        SpecCorpus::read(&root.join("givn/specs")).unwrap()
    }

    #[test]
    fn fragment_interactions_match_capability_and_title() {
        let root = tempfile::TempDir::new().unwrap();
        let dir = root.path().join("givn/specs/fragments");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("fragment.md"), "# Fragment: support\n\n## Capabilities\n\n- place-order\n- cancel-order\n\n## Interactions\n\n| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| place-order | submit | Consumer submits an order |\n\n## Diagram\n\n```mermaid\nflowchart LR\n A --> B\n```\n").unwrap();
        std::fs::write(
            dir.join("place-order.feature"),
            "Feature: Place\n  Scenario: Internal\n    Given x\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("cancel-order.feature"),
            "Feature: Cancel\n  @e2e\n  Scenario: Consumer submits an order\n    Given x\n",
        )
        .unwrap();
        let findings = interaction_gate_findings(&corpus(root.path()));
        assert_eq!(findings.len(), 2, "{findings:?}");
        assert!(
            findings
                .iter()
                .all(|finding| finding.files[0].ends_with("fragment.md"))
        );
        std::fs::write(
            dir.join("place-order.feature"),
            "Feature: Place\n  @e2e\n  Scenario: Consumer submits an order\n    Given x\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("cancel-order.feature"),
            "Feature: Cancel\n  Scenario: Internal\n    Given x\n",
        )
        .unwrap();
        assert!(interaction_gate_findings(&corpus(root.path())).is_empty());
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
        assert!(interaction_gate_findings(&corpus(root.path())).is_empty());
        let rows = usecase_coverages(&corpus(root.path())).unwrap();
        assert_eq!(rows[0].usecase, "project-run");
        assert_eq!(rows[0].capability, "project-run");
    }

    #[test]
    fn missing_interaction_mapping_is_an_error() {
        let root = temp_usecase(
            "Feature: Run\n\n  @e2e\n  Scenario: Other\n    Given x\n    Then y\n",
            USECASE,
        );
        let findings = interaction_gate_findings(&corpus(root.path()));
        assert_eq!(findings.len(), 1);
        assert!(
            findings[0].titles[0].contains("missing @e2e scenario 'Consumer starts a project'"),
            "{findings:?}"
        );
        assert!(
            findings[0].titles[0].contains("unlisted @e2e scenario 'Other'"),
            "{findings:?}"
        );
    }

    #[test]
    fn tagged_outline_counts_once() {
        let root = temp_usecase(
            "Feature: Run\n\n  @e2e\n  Scenario Outline: Consumer starts a project\n    Given <kind>\n    Then y\n\n    Examples:\n      | kind |\n      | clean |\n      | dirty |\n",
            USECASE,
        );
        let rows = usecase_coverages(&corpus(root.path())).unwrap();
        assert_eq!(rows[0].e2e_scenarios, 1);
    }

    #[test]
    fn undeclared_feature_is_reported() {
        let root = temp_usecase(
            "Feature: Run\n  @e2e\n  Scenario: Consumer starts a project\n    Given x\n",
            USECASE,
        );
        let dir = root.path().join("givn/specs/project-run");
        std::fs::write(
            dir.join("cancellations.feature"),
            "Feature: Cancel\n  @e2e\n  Scenario: Consumer cancels an order\n    Given x\n",
        )
        .unwrap();
        let findings = interaction_gate_findings(&corpus(root.path()));
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "undeclared_capability");
        assert!(
            findings[0].titles[0].contains("undeclared capability 'cancellations'"),
            "{findings:?}"
        );
        assert!(
            findings[0].titles[0].contains("Consumer cancels an order"),
            "{findings:?}"
        );
    }

    #[test]
    fn feature_directory_without_document_is_reported() {
        let root = tempfile::TempDir::new().unwrap();
        let dir = root.path().join("givn/specs/orphan");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("orphan.feature"),
            "Feature: Orphan\n  @e2e\n  Scenario: Hidden\n    Given x\n",
        )
        .unwrap();
        let findings = interaction_gate_findings(&corpus(root.path()));
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "undeclared_capability");
        assert!(
            findings[0].titles[0].contains("use case 'orphan'"),
            "{findings:?}"
        );
    }

    #[test]
    fn declared_capability_without_feature_is_a_gate_error() {
        let root = temp_usecase("Feature: Run\n  Scenario: Internal\n    Given x\n", USECASE);
        let dir = root.path().join("givn/specs/project-run");
        std::fs::remove_file(dir.join("project-run.feature")).unwrap();
        let findings = interaction_gate_findings(&corpus(root.path()));
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "usecase_interaction_invalid");
    }
}
