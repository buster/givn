//! Parsing and checking of the proposal's `## Capability Routing` table.
//!
//! Routing decisions identify a capability's use-case owner. The decision is
//! recorded by the author; review never silently reruns advisory routing.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use gherkin::GherkinEnv;

use crate::gherkin::usecase_md::parse_usecase_md;

pub const HEADING_CAPABILITY_ROUTING: &str = "## Capability Routing";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingRow {
    pub capability: String,
    pub decision: Decision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Extend(String),
    NewIn(String),
}

pub fn parse_routing_table(proposal_text: &str) -> Vec<RoutingRow> {
    let mut rows = Vec::new();
    let mut in_section = false;
    for line in proposal_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            in_section = trimmed == HEADING_CAPABILITY_ROUTING;
            continue;
        }
        if !in_section || !trimmed.starts_with('|') || trimmed.starts_with("|-") {
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().trim_matches('`').to_string())
            .collect();
        if cells.len() < 4 || cells[0].is_empty() || cells[0] == "Proposed capability" {
            continue;
        }
        let decision_raw = cells[2].trim();
        let decision = if let Some(capability) = decision_raw
            .strip_prefix("EXTEND")
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            Decision::Extend(capability.to_string())
        } else if let Some(usecase) = decision_raw
            .strip_prefix("NEW in")
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            Decision::NewIn(usecase.to_string())
        } else {
            continue;
        };
        rows.push(RoutingRow {
            capability: cells[0].clone(),
            decision,
        });
    }
    rows
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaCapability {
    pub capability: String,
    pub usecase: Option<String>,
}

pub fn collect_delta_capabilities(specs_dir: &Path) -> Result<Vec<DeltaCapability>> {
    let mut out = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(specs_dir) {
        let feature = gherkin::Feature::parse_path(&path, GherkinEnv::default())
            .map_err(|error| anyhow::anyhow!("failed to parse {}: {error}", path.display()))?;
        if crate::gherkin::delta::is_retired_feature(&feature) {
            continue;
        }
        let capability = crate::gherkin::delta::capability_tag(&feature)
            .map(str::to_string)
            .or_else(|| {
                path.file_stem()
                    .and_then(|name| name.to_str())
                    .map(str::to_string)
            });
        let Some(capability) = capability else {
            continue;
        };
        let usecase = path
            .parent()
            .filter(|parent| *parent != specs_dir)
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .map(str::to_string);
        out.push(DeltaCapability {
            capability,
            usecase,
        });
    }
    out.sort_by(|left, right| left.capability.cmp(&right.capability));
    out.dedup_by(|left, right| left.capability == right.capability);
    Ok(out)
}

pub fn resolve_permanent_usecase(specs_root: &Path, capability: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(specs_root).ok()?;
    for entry in entries.flatten() {
        let usecase_dir = entry.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        if usecase_dir.join(format!("{capability}.feature")).exists() {
            return Some(usecase_dir);
        }
    }
    None
}

fn usecase_document_path(specs_dir: &Path, usecase: &str) -> PathBuf {
    specs_dir.join(usecase).join("usecase.md")
}

fn content_words(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

pub fn check_routing_consistency(project_root: &Path, change_id: &str) -> Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let specs_dir = change_dir.join("specs");
    let deltas = collect_delta_capabilities(&specs_dir)?;
    if deltas.is_empty() {
        return Ok(());
    }

    let proposal_path = change_dir.join("proposal.md");
    let proposal_text = std::fs::read_to_string(&proposal_path).unwrap_or_default();
    let rows = parse_routing_table(&proposal_text);
    if rows.is_empty() {
        println!(
            "  ⚠ proposal.md has no filled '{HEADING_CAPABILITY_ROUTING}' table — \
             the delta capabilities [{}] are undeclared; record decisions \
             (EXTEND <capability> / NEW in <usecase-id>)",
            deltas
                .iter()
                .map(|delta| delta.capability.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }

    let permanent_root = project_root.join("givn/specs");
    let mut errors = Vec::new();
    for delta in &deltas {
        let Some(row) = rows.iter().find(|row| row.capability == delta.capability) else {
            errors.push(format!(
                "delta capability '{}' is not declared in the Capability Routing table",
                delta.capability
            ));
            continue;
        };

        match &row.decision {
            Decision::Extend(capability) => {
                let Some(permanent_usecase) =
                    resolve_permanent_usecase(&permanent_root, capability)
                else {
                    errors.push(format!(
                        "decision EXTEND {capability}: capability does not exist in the permanent corpus; record 'NEW in <usecase-id>' instead"
                    ));
                    continue;
                };
                if delta.capability != *capability {
                    errors.push(format!(
                        "decision EXTEND {capability}: the delta capability is '{}'",
                        delta.capability
                    ));
                }
                let permanent_usecase_name = permanent_usecase
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("?");
                if delta.usecase.as_deref() != Some(permanent_usecase_name) {
                    errors.push(format!(
                        "decision EXTEND {capability}: the delta must live under specs/{permanent_usecase_name}/"
                    ));
                }
            }
            Decision::NewIn(usecase) => {
                if resolve_permanent_usecase(&permanent_root, &delta.capability).is_some() {
                    errors.push(format!(
                        "decision NEW in {usecase}: capability '{}' already exists in the permanent corpus; record 'EXTEND {}' instead",
                        delta.capability, delta.capability
                    ));
                }
                if delta.usecase.as_deref() != Some(usecase.as_str()) {
                    errors.push(format!(
                        "decision NEW in {usecase}: the delta must live under specs/{usecase}/"
                    ));
                    continue;
                }
                let usecase_path = usecase_document_path(&specs_dir, usecase);
                let permanent_path = usecase_document_path(&permanent_root, usecase);
                let document_path = if usecase_path.exists() {
                    usecase_path
                } else if permanent_path.exists() {
                    permanent_path
                } else {
                    errors.push(format!(
                        "decision NEW in {usecase}: no usecase.md exists; create one with '{}' in Capabilities",
                        delta.capability
                    ));
                    continue;
                };
                match parse_usecase_md(&std::fs::read_to_string(&document_path)?) {
                    Ok(doc) => {
                        if !doc
                            .capabilities
                            .iter()
                            .any(|capability| capability == &delta.capability)
                        {
                            errors.push(format!(
                                "decision NEW in {usecase}: usecase.md does not declare capability '{}'",
                                delta.capability
                            ));
                        }
                        let capability_words = content_words(&delta.capability);
                        let has_interaction = doc.interactions.iter().any(|interaction| {
                            interaction.capability == delta.capability
                                || capability_words.iter().any(|word| {
                                    content_words(&interaction.consumer_action)
                                        .iter()
                                        .any(|interaction_word| interaction_word == word)
                                })
                        });
                        if !has_interaction {
                            errors.push(format!(
                                "decision NEW in {usecase}: usecase.md has no interaction for capability '{}'",
                                delta.capability
                            ));
                        }
                    }
                    Err(error) => errors.push(format!(
                        "decision NEW in {usecase}: usecase.md failed to parse: {error}"
                    )),
                }
            }
        }
    }

    if !errors.is_empty() {
        bail!(
            "capability routing decisions do not match the built delta:\n  {}",
            errors.join("\n  ")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const USECASE: &str = r#"# Use case: gates

## Level

!

## Actors

- Operator

## Goal

Run gates.

## Trigger

The operator asks.

## Preconditions

- A project exists.

## Main flow

1. Run.

## Extensions

- none

## Rules

- Run once.

## Examples

- A clean run.

## Minimal guarantee

The project is inspectable.

## Success guarantee

The gate result is shown.

## Personas

- none

## Capabilities

- gate-a

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| gate-a | run gate-a | Consumer runs gate-a |

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

    fn temp_project() -> (tempfile::TempDir, PathBuf) {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        std::fs::create_dir_all(project.join("givn/specs/gates")).unwrap();
        std::fs::write(project.join("givn/specs/gates/usecase.md"), USECASE).unwrap();
        std::fs::write(
            project.join("givn/specs/gates/gate-a.feature"),
            "Feature: Gate A\n\n  Scenario: S\n    Given x\n    Then y\n",
        )
        .unwrap();
        (root, project)
    }

    #[test]
    fn parser_reads_usecase_routing_rows() {
        let rows = parse_routing_table(
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-a | EXTEND gate-a | EXTEND gate-a | |\n| fresh | NEW in gates | NEW in gates | |\n",
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].decision, Decision::NewIn("gates".to_string()));
    }

    #[test]
    fn existing_capability_must_use_its_permanent_usecase() {
        let (_root, project) = temp_project();
        let change = project.join("givn/changes/my-change");
        std::fs::create_dir_all(change.join("specs/other")).unwrap();
        std::fs::write(
            &change.join("proposal.md"),
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-a | EXTEND gate-a | EXTEND gate-a | |\n",
        )
        .unwrap();
        std::fs::write(
            change.join("specs/other/gate-a.feature"),
            "@givn.delta @gate-a\nFeature: Gate A\n\n  @givn.added\n  Scenario: S2\n    Given x\n    Then y\n",
        )
        .unwrap();
        let error = check_routing_consistency(&project, "my-change").unwrap_err();
        assert!(error.to_string().contains("must live under specs/gates"));
    }
}
