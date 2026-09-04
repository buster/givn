//! Parsing and checking of the proposal's `## Capability Routing` table
//! (Phase 4.7). The gate compares the *recorded* decision against the
//! *built* delta — never against a fresh `spec route` run, whose ranking
//! may legitimately drift as the corpus changes.

use std::path::Path;

use anyhow::{Result, bail};
use gherkin::GherkinEnv;

/// Exact heading the parser matches (copy-pasted into
/// `assets/templates/proposal.md`; a test asserts the two agree).
pub const HEADING_CAPABILITY_ROUTING: &str = "## Capability Routing";

/// One recorded routing decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingRow {
    pub capability: String,
    pub decision: Decision,
}

/// A recorded routing decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// `EXTEND <cap>` — delta extends an existing capability.
    Extend(String),
    /// `NEW in <group>` — delta creates a new capability in a group.
    NewIn(String),
}

/// Parse the Capability Routing table from a proposal. Exact-heading
/// match (the disposition parser's prefix match is what let its headings
/// drift); rows without a Decision column value are skipped.
pub fn parse_routing_table(proposal_text: &str) -> Vec<RoutingRow> {
    let mut rows = Vec::new();
    let mut in_section = false;
    for line in proposal_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            in_section = trimmed.trim() == HEADING_CAPABILITY_ROUTING;
            continue;
        }
        if !in_section || !trimmed.starts_with('|') || trimmed.starts_with("|-") {
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().trim_matches('`').to_string())
            .collect();
        if cells.len() < 4 || cells[0].is_empty() || cells[0] == "Proposed capability" {
            continue;
        }
        let decision_raw = cells[2].trim();
        if decision_raw.is_empty() {
            continue;
        }
        let decision = if let Some(cap) = decision_raw
            .strip_prefix("EXTEND")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Decision::Extend(cap.to_string())
        } else if let Some(group) = decision_raw
            .strip_prefix("NEW in")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Decision::NewIn(group.to_string())
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

/// A delta capability found under the change's specs dir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaCapability {
    pub capability: String,
    /// Group directory the delta file lives in (relative to the change's
    /// specs dir).
    pub group: Option<String>,
}

/// Collect the change's delta capabilities from its specs tree.
pub fn collect_delta_capabilities(specs_dir: &Path) -> Result<Vec<DeltaCapability>> {
    let mut out = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(specs_dir) {
        let feature = gherkin::Feature::parse_path(&path, GherkinEnv::default())
            .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", path.display()))?;
        if crate::gherkin::delta::is_retired_feature(&feature) {
            continue;
        }
        let capability = crate::gherkin::delta::capability_tag(&feature)
            .map(str::to_string)
            .or_else(|| {
                path.file_stem()
                    .and_then(|n| n.to_str())
                    .map(str::to_string)
            });
        let Some(capability) = capability else {
            continue;
        };
        let group = path
            .parent()
            .filter(|p| p != &specs_dir)
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(str::to_string);
        out.push(DeltaCapability { capability, group });
    }
    out.sort_by(|a, b| a.capability.cmp(&b.capability));
    out.dedup_by(|a, b| a.capability == b.capability);
    Ok(out)
}

/// Resolve which group an existing permanent capability lives in, by
/// scanning `givn/specs/*/<cap>.feature`.
pub fn resolve_permanent_group(specs_root: &Path, cap: &str) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(specs_root).ok()?;
    for entry in entries.flatten() {
        let group_dir = entry.path();
        if !group_dir.is_dir() {
            continue;
        }
        if group_dir.join(format!("{cap}.feature")).exists() {
            return Some(group_dir);
        }
    }
    None
}

fn content_words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect()
}

/// The decision-consistency check body: verify every recorded decision
/// matches what was actually built, and every built delta is declared.
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
        // Legacy/foreign proposals predate the Capability Routing section.
        // Nothing is recorded, so there is no decision to check against —
        // warn loudly instead of failing (the template always scaffolds the
        // table, and a present table is enforced strictly).
        println!(
            "  ⚠ proposal.md has no filled '{HEADING_CAPABILITY_ROUTING}' table — \
             the delta capabilities [{}] are undeclared; run `givn spec route` and \
             record decisions (EXTEND <cap> / NEW in <group>)",
            deltas
                .iter()
                .map(|d| d.capability.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }

    let specs_root = project_root.join("givn/specs");
    let mut errors = Vec::new();

    for delta in &deltas {
        let row = rows.iter().find(|r| r.capability == delta.capability);
        let Some(row) = row else {
            errors.push(format!(
                "delta capability '{}' is not declared in the Capability Routing table — \
                 drift (building what the table does not say) is not allowed",
                delta.capability
            ));
            continue;
        };
        match &row.decision {
            Decision::Extend(cap) => {
                match resolve_permanent_group(&specs_root, cap) {
                    Some(permanent_group) => {
                        let permanent_group_name = permanent_group
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?");
                        if let Some(delta_group) = &delta.group
                            && delta_group != permanent_group_name
                        {
                            errors.push(format!(
                                "decision EXTEND {cap}: the delta lives in group '{delta_group}' \
                                 but the capability lives in group '{permanent_group_name}' — \
                                 they must match"
                            ));
                        }
                    }
                    None => {
                        errors.push(format!(
                            "decision EXTEND {cap}: capability '{cap}' does not exist in the \
                             permanent corpus — record 'NEW in <group>' instead"
                        ));
                    }
                }
                if delta.capability != *cap {
                    errors.push(format!(
                        "decision EXTEND {cap}: the delta's capability is '{}' — they must match",
                        delta.capability
                    ));
                }
            }
            Decision::NewIn(group) => {
                if resolve_permanent_group(&specs_root, &delta.capability).is_some() {
                    errors.push(format!(
                        "decision NEW in {group}: capability '{}' already exists in the \
                         permanent corpus — record 'EXTEND {}' instead",
                        delta.capability, delta.capability
                    ));
                }
                match &delta.group {
                    Some(delta_group) if delta_group == group => {}
                    other => {
                        errors.push(format!(
                            "decision NEW in {group}: the delta file lives in '{}' — \
                             it must live under specs/{group}/",
                            other.as_deref().unwrap_or("(ungrouped)")
                        ));
                    }
                }
                // Every NEW capability must have a matching Interactions
                // entry in the group's group.md — the e2e-per-interaction
                // inventory is the durable record (Phase 3).
                let group_md = specs_root.join(group).join("group.md");
                let group_md_delta = specs_dir.join(group).join("group.md");
                let interactions_path = if group_md_delta.exists() {
                    group_md_delta
                } else if group_md.exists() {
                    group_md
                } else {
                    errors.push(format!(
                        "decision NEW in {group}: no group.md exists for that group — \
                         create one with an Interactions entry covering '{}'",
                        delta.capability
                    ));
                    continue;
                };
                let text = std::fs::read_to_string(&interactions_path)?;
                match crate::gherkin::group_md::parse_group_md(&text) {
                    Ok(doc) => {
                        let cap_words = content_words(&delta.capability);
                        let covers = doc.interactions.iter().any(|interaction| {
                            let words = content_words(interaction);
                            cap_words
                                .iter()
                                .any(|cw| words.iter().any(|iw| iw == cw || cw.contains(iw)))
                        }) || text.contains(&delta.capability);
                        if !covers {
                            errors.push(format!(
                                "decision NEW in {group}: group.md's Interactions section has \
                                 no entry plausibly covering '{}' — add one (every NEW \
                                 capability must have a matching Interactions entry)",
                                delta.capability
                            ));
                        }
                    }
                    Err(e) => {
                        errors.push(format!(
                            "decision NEW in {group}: group.md failed to parse: {e}"
                        ));
                    }
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

    fn temp_project() -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::TempDir::new().unwrap();
        let path = root.path().to_path_buf();
        (root, path)
    }

    fn seed_grouped_permanent(project: &std::path::Path) {
        let base = project.join("givn/specs");
        std::fs::create_dir_all(base.join("gates")).unwrap();
        std::fs::write(
            base.join("gates/gate-a.feature"),
            "Feature: Gate A\n\n  Scenario: S\n    Given x\n    Then y\n",
        )
        .unwrap();
        std::fs::write(
            base.join("gates/group.md"),
            "# Group: gates\n\n## Actor\n\nA.\n\n## Goal\n\nG.\n\n## Interactions\n\n- run gate-a checks\n",
        )
        .unwrap();
    }

    fn seed_change(project: &std::path::Path, proposal: &str, delta_rel: &str, delta_text: &str) {
        let change = project.join("givn/changes/my-change");
        std::fs::create_dir_all(change.join("specs")).unwrap();
        std::fs::write(change.join("proposal.md"), proposal).unwrap();
        let delta_path = change.join("specs").join(delta_rel);
        std::fs::create_dir_all(delta_path.parent().unwrap()).unwrap();
        std::fs::write(delta_path, delta_text).unwrap();
    }

    #[test]
    fn parser_reads_template_heading_and_rows() {
        let template = crate::config::embedded::Assets::get_str("templates/proposal.md")
            .expect("embedded proposal template");
        let filled = template.replace(
            "| | | `EXTEND <cap>` / `NEW in <group>` | |",
            "| gate-a | EXTEND gate-a | EXTEND gate-a | |",
        );
        let rows = parse_routing_table(&filled);
        assert_eq!(
            rows,
            vec![RoutingRow {
                capability: "gate-a".to_string(),
                decision: Decision::Extend("gate-a".to_string()),
            }]
        );
    }

    #[test]
    fn extend_decision_matching_delta_and_group_passes() {
        let (_root, project) = temp_project();
        seed_grouped_permanent(&project);
        seed_change(
            &project,
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-a | EXTEND gate-a | EXTEND gate-a | |\n",
            "gates/gate-a.feature",
            "@givn.delta @gate-a\nFeature: Gate A\n\n  @givn.added\n  Scenario: S2\n    Given x\n    Then y\n",
        );
        check_routing_consistency(&project, "my-change").unwrap();
    }

    #[test]
    fn extend_decision_but_delta_creates_new_capability_fails() {
        let (_root, project) = temp_project();
        seed_grouped_permanent(&project);
        seed_change(
            &project,
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-a | EXTEND gate-a | EXTEND gate-a | |\n",
            "other-group/brand-new.feature",
            "@givn.delta @brand-new\nFeature: Brand new\n\n  @givn.added\n  Scenario: S\n    Given x\n    Then y\n",
        );
        let err = check_routing_consistency(&project, "my-change").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not declared"), "{msg}");
        assert!(msg.contains("drift"), "{msg}");
    }

    #[test]
    fn new_decision_outside_recorded_group_fails() {
        let (_root, project) = temp_project();
        seed_grouped_permanent(&project);
        seed_change(
            &project,
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| fresh-cap | NEW in gates | NEW in gates | |\n",
            "elsewhere/fresh-cap.feature",
            "@givn.delta @fresh-cap\nFeature: Fresh\n\n  @givn.added\n  Scenario: S\n    Given x\n    Then y\n",
        );
        let err = check_routing_consistency(&project, "my-change").unwrap_err();
        assert!(
            err.to_string().contains("must live under specs/gates/"),
            "{err}"
        );
    }

    #[test]
    fn new_decision_without_group_md_interactions_entry_fails() {
        let (_root, project) = temp_project();
        let base = project.join("givn/specs");
        std::fs::create_dir_all(base.join("gates")).unwrap();
        std::fs::write(
            base.join("gates/gate-a.feature"),
            "Feature: Gate A\n\n  Scenario: S\n    Given x\n    Then y\n",
        )
        .unwrap();
        // No group.md at all.
        seed_change(
            &project,
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-b | NEW in gates | NEW in gates | |\n",
            "gates/gate-b.feature",
            "@givn.delta @gate-b\nFeature: Gate B\n\n  @givn.added\n  Scenario: S\n    Given x\n    Then y\n",
        );
        let err = check_routing_consistency(&project, "my-change").unwrap_err();
        assert!(err.to_string().contains("no group.md exists"), "{err}");
    }

    #[test]
    fn new_decision_with_matching_interactions_entry_passes() {
        let (_root, project) = temp_project();
        seed_grouped_permanent(&project);
        seed_change(
            &project,
            "## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| gate-b | NEW in gates | NEW in gates | |\n",
            "gates/gate-b.feature",
            "@givn.delta @gate-b\nFeature: Gate B\n\n  @givn.added\n  Scenario: S\n    Given x\n    Then y\n",
        );
        // Delta-side group.md update carrying the new interaction.
        let change = project.join("givn/changes/my-change");
        std::fs::write(
            change.join("specs/gates/group.md"),
            "# Group: gates\n\n## Actor\n\nA.\n\n## Goal\n\nG.\n\n## Interactions\n\n- run gate-a checks\n- run gate-b checks\n",
        )
        .unwrap();
        check_routing_consistency(&project, "my-change").unwrap();
    }

    #[test]
    fn delta_without_any_routing_table_warns_but_passes() {
        // Legacy proposals predate the table: nothing recorded, nothing to
        // compare — a loud warning prints and the check passes.
        let (_root, project) = temp_project();
        seed_grouped_permanent(&project);
        seed_change(
            &project,
            "# Proposal without routing\n",
            "gates/gate-a.feature",
            "@givn.delta @gate-a\nFeature: Gate A\n\n  @givn.added\n  Scenario: S2\n    Given x\n    Then y\n",
        );
        check_routing_consistency(&project, "my-change").unwrap();
    }

    #[test]
    fn change_without_delta_specs_passes_trivially() {
        let (_root, project) = temp_project();
        let change = project.join("givn/changes/my-change");
        std::fs::create_dir_all(&change).unwrap();
        check_routing_consistency(&project, "my-change").unwrap();
    }
}
