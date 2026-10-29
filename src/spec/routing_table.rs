//! Parsing and checking of the proposal's `## Capability Routing` table.
//!
//! Routing decisions identify a capability's use-case owner. The decision is
//! recorded by the author; review never silently reruns advisory routing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use gherkin::GherkinEnv;

use crate::gherkin::usecase_md::{
    FragmentDoc, UsecaseDiagnostic, UsecaseDiagnostics, UsecaseDoc, parse_fragment_md_diagnostics,
    parse_usecase_md_diagnostics,
};

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

fn usecase_document_path(specs_dir: &Path, usecase: &str) -> PathBuf {
    specs_dir.join(usecase).join("usecase.md")
}

fn content_words(text: &str) -> Vec<String> {
    text.split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocumentKind {
    Usecase,
    Fragment,
}

impl DocumentKind {
    fn label(self) -> &'static str {
        match self {
            Self::Usecase => "usecase",
            Self::Fragment => "fragment",
        }
    }
}

#[derive(Default)]
struct CachedDocument {
    kind: Option<DocumentKind>,
    usecase: Option<UsecaseDoc>,
    fragment: Option<FragmentDoc>,
    diagnostics: UsecaseDiagnostics,
}

fn collect_narrative_paths(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_narrative_paths(&path, out);
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "usecase.md" || name == "fragment.md")
        {
            out.push(path);
        }
    }
}

fn relative_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn cache_document(cache: &mut BTreeMap<PathBuf, CachedDocument>, path: &Path, project_root: &Path) {
    if cache.contains_key(path) {
        return;
    }
    let kind = if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "fragment.md")
    {
        DocumentKind::Fragment
    } else {
        DocumentKind::Usecase
    };
    let Ok(content) = std::fs::read_to_string(path) else {
        cache.insert(
            path.to_path_buf(),
            CachedDocument {
                kind: Some(kind),
                usecase: None,
                fragment: None,
                diagnostics: UsecaseDiagnostics(vec![UsecaseDiagnostic {
                    line: None,
                    message: format!("cannot read '{}'", relative_path(project_root, path)),
                    suggestion: None,
                }]),
            },
        );
        return;
    };
    let (usecase, fragment, diagnostics) = match kind {
        DocumentKind::Usecase => {
            let (doc, diagnostics) = parse_usecase_md_diagnostics(&content);
            (doc, None, diagnostics)
        }
        DocumentKind::Fragment => {
            let (doc, diagnostics) = parse_fragment_md_diagnostics(&content);
            (None, doc, diagnostics)
        }
    };
    cache.insert(
        path.to_path_buf(),
        CachedDocument {
            kind: Some(kind),
            usecase,
            fragment,
            diagnostics,
        },
    );
}

pub fn check_routing_consistency(project_root: &Path, change_id: &str) -> Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let specs_dir = change_dir.join("specs");
    let deltas = collect_delta_capabilities(&specs_dir)?;

    let mut narrative_paths = Vec::new();
    collect_narrative_paths(&specs_dir, &mut narrative_paths);
    narrative_paths.sort();

    if deltas.is_empty() && narrative_paths.is_empty() {
        return Ok(());
    }

    let mut cache = BTreeMap::new();
    for path in &narrative_paths {
        cache_document(&mut cache, path, project_root);
    }

    let proposal_path = change_dir.join("proposal.md");
    let proposal_text = std::fs::read_to_string(&proposal_path).unwrap_or_default();
    let rows = parse_routing_table(&proposal_text);

    if deltas.is_empty() {
        emit_or_return(project_root, &cache, BTreeSet::new())?;
        return Ok(());
    }

    if rows.is_empty() {
        return emit_or_return(
            project_root,
            &cache,
            BTreeSet::from([format!(
                "proposal.md has no filled '{HEADING_CAPABILITY_ROUTING}' table — \
             the delta capabilities [{}] are undeclared; record decisions \
             (EXTEND <capability> / NEW in <usecase-id>)",
                deltas
                    .iter()
                    .map(|delta| delta.capability.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )]),
        );
    }

    let permanent_root = project_root.join("givn/specs");
    let mut errors = BTreeSet::new();
    for delta in &deltas {
        let Some(row) = rows.iter().find(|row| row.capability == delta.capability) else {
            errors.insert(format!(
                "delta capability '{}' is not declared in the Capability Routing table — the proposal records no 'NEW in <usecase-id>' or 'EXTEND <capability>' decision for it",
                delta.capability
            ));
            continue;
        };

        let owner = match crate::spec::owner::resolve_capability_owner(
            &permanent_root,
            &delta.capability,
        ) {
            Ok(owner) => owner,
            Err(error) => {
                errors.insert(error.to_string());
                continue;
            }
        };
        if let Some(owner) = &owner {
            println!(
                "capability '{}' already owned by {}",
                delta.capability,
                owner.display()
            );
        }
        if matches!(row.decision, Decision::NewIn(_)) && owner.is_some() {
            errors.insert(format!(
                "decision NEW: capability '{}' already exists in the permanent corpus; record 'EXTEND {}' instead",
                delta.capability, delta.capability
            ));
            continue;
        }
        match &row.decision {
            Decision::Extend(capability) => {
                let Some(permanent_usecase) = owner else {
                    errors.insert(format!(
                        "decision EXTEND {capability}: capability does not exist in the permanent corpus; record 'NEW in <usecase-id>' instead"
                    ));
                    continue;
                };
                if delta.capability != *capability {
                    errors.insert(format!(
                        "decision EXTEND {capability}: the delta capability is '{}'",
                        delta.capability
                    ));
                }
                let permanent_usecase_name = permanent_usecase
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("?");
                if delta.usecase.as_deref() != Some(permanent_usecase_name) {
                    errors.insert(format!(
                        "decision EXTEND {capability}: the delta must live under specs/{permanent_usecase_name}/"
                    ));
                }
            }
            Decision::NewIn(usecase) if usecase == "fragments" => {
                let delta_path = specs_dir.join("fragments/fragment.md");
                let fragment_path = if delta_path.exists() {
                    delta_path
                } else {
                    permanent_root.join("fragments/fragment.md")
                };
                cache_document(&mut cache, &fragment_path, project_root);
                let fragment = cache
                    .get(&fragment_path)
                    .and_then(|doc| doc.fragment.as_ref());
                match fragment {
                    Some(doc) if doc.capabilities.contains(&delta.capability) => {}
                    Some(_) => {
                        errors.insert(format!(
                            "decision NEW in fragments: fragment.md does not declare capability '{}'",
                            delta.capability
                        ));
                    }
                    None => {}
                }
                if delta.usecase.as_deref() != Some("fragments") {
                    errors.insert(
                        "decision NEW in fragments: the delta must live under specs/fragments/"
                            .to_string(),
                    );
                }
            }
            Decision::NewIn(usecase) => {
                if delta.usecase.as_deref() != Some(usecase.as_str()) {
                    errors.insert(format!(
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
                    errors.insert(format!(
                        "decision NEW in {usecase}: no usecase.md exists; create one with '{}' in Capabilities",
                        delta.capability
                    ));
                    continue;
                };
                cache_document(&mut cache, &document_path, project_root);
                let doc = cache
                    .get(&document_path)
                    .and_then(|document| document.usecase.as_ref());
                let Some(doc) = doc else {
                    continue;
                };
                if !doc
                    .capabilities
                    .iter()
                    .any(|capability| capability == &delta.capability)
                {
                    errors.insert(format!(
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
                    errors.insert(format!(
                        "decision NEW in {usecase}: usecase.md has no interaction for capability '{}'",
                        delta.capability
                    ));
                }
            }
        }
    }

    emit_or_return(project_root, &cache, errors)
}

fn emit_or_return(
    project_root: &Path,
    cache: &BTreeMap<PathBuf, CachedDocument>,
    errors: BTreeSet<String>,
) -> Result<()> {
    let mut lines = Vec::new();
    let mut seen = BTreeSet::new();
    for (path, document) in cache {
        if document.diagnostics.is_empty() {
            continue;
        }
        let kind = document
            .kind
            .map(DocumentKind::label)
            .unwrap_or("narrative");
        lines.push(format!(
            "  {kind} document '{}':",
            relative_path(project_root, path)
        ));
        for diagnostic in &document.diagnostics.0 {
            let rendered = diagnostic.rendered();
            if seen.insert(rendered.clone()) {
                lines.push(format!("    {rendered}"));
            }
        }
    }
    for error in errors {
        if seen.insert(error.clone()) {
            lines.push(format!("  {error}"));
        }
    }
    if !lines.is_empty() {
        bail!(
            "capability routing decisions do not match the built delta:\n{}",
            lines.join("\n")
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
    fn fragment_ownership_routes_and_conflicts_are_checked() {
        for (decision, duplicate, expected) in [
            ("EXTEND gate-a", false, None),
            ("NEW in fragments", false, Some("already exists")),
            ("EXTEND gate-a", true, Some("ambiguous")),
            ("NEW in fragments", true, Some("ambiguous")),
        ] {
            let (_root, project) = temp_project();
            let fragments = project.join("givn/specs/fragments");
            std::fs::create_dir_all(&fragments).unwrap();
            std::fs::write(fragments.join("gate-a.feature"), "Feature: Gate A\n").unwrap();
            std::fs::write(
                fragments.join("fragment.md"),
                crate::gherkin::usecase_md::fragment_md_template()
                    .replace("<fragment-id>", "support")
                    .replace("<capability-id>", "gate-a"),
            )
            .unwrap();
            if !duplicate {
                std::fs::remove_file(project.join("givn/specs/gates/gate-a.feature")).unwrap();
            }
            let change = project.join("givn/changes/my-change");
            std::fs::create_dir_all(change.join("specs/fragments")).unwrap();
            std::fs::write(change.join("proposal.md"), format!("## Capability Routing\n\n| Proposed capability | Recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-a | | {decision} | |\n")).unwrap();
            std::fs::write(change.join("specs/fragments/gate-a.feature"), "@givn.delta @gate-a\nFeature: Gate A\n  @givn.added\n  Scenario: Added\n    Given x\n").unwrap();
            let result = check_routing_consistency(&project, "my-change");
            match expected {
                Some(message) => assert!(result.unwrap_err().to_string().contains(message)),
                None => result.unwrap(),
            }
        }
    }

    #[test]
    fn new_fragment_can_use_its_delta_narrative() {
        let (_root, project) = temp_project();
        let change = project.join("givn/changes/my-change");
        let dir = change.join("specs/fragments");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(change.join("proposal.md"), "## Capability Routing\n\n| Proposed capability | Recommendation | Decision | Rationale |\n|---|---|---|---|\n| help | | NEW in fragments | |\n").unwrap();
        std::fs::write(
            dir.join("fragment.md"),
            crate::gherkin::usecase_md::fragment_md_template()
                .replace("<fragment-id>", "support")
                .replace("<capability-id>", "help"),
        )
        .unwrap();
        std::fs::write(
            dir.join("help.feature"),
            "@givn.delta @help\nFeature: Help\n  @givn.added\n  Scenario: Help\n    Given x\n",
        )
        .unwrap();
        check_routing_consistency(&project, "my-change").unwrap();
        std::fs::remove_file(change.join("proposal.md")).unwrap();
        assert!(
            check_routing_consistency(&project, "my-change")
                .unwrap_err()
                .to_string()
                .contains("undeclared")
        );
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
            change.join("proposal.md"),
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
