//! `givn spec tree` — deterministic discovery over the permanent corpus
//! (Phase 4.1). No embeddings, no network, no score. Walks
//! `givn/specs/<group>/<cap>.feature`, reports per-group Actor/Goal from
//! `group.md`, per-capability scenario and `@e2e` counts, and the per-group
//! phrasing reuse ratio (the measurement approach from the retirement plan,
//! reported only — never gated).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use gherkin::GherkinEnv;
use serde::Serialize;

/// One capability's row in the tree output.
#[derive(Debug, Clone, Serialize)]
pub struct CapabilityRow {
    pub name: String,
    pub scenarios: usize,
    pub e2e: usize,
}

/// One group's summary in the tree output.
#[derive(Debug, Clone, Serialize)]
pub struct GroupRow {
    pub name: String,
    pub actor: Option<String>,
    pub goal: Option<String>,
    pub capabilities: Vec<CapabilityRow>,
    /// pooled phrasing reuse ratio for the group (steps / distinct), 0.0
    /// when there are no steps.
    pub reuse_ratio: f64,
}

/// Normalise a step line for phrasing comparison: strip quoted spans →
/// `"X"`, digits → `N`, collapse whitespace, lowercase.
fn normalize_phrasing(step: &str) -> String {
    let mut out = String::with_capacity(step.len());
    let mut chars = step.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            for n in chars.by_ref() {
                if n == '"' {
                    break;
                }
            }
            out.push_str("\"x\"");
        } else if c.is_ascii_digit() {
            for n in chars.by_ref() {
                if !n.is_ascii_digit() {
                    break;
                }
            }
            out.push('N');
        } else {
            out.push(c);
        }
    }
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Pooled phrasing reuse: total steps / distinct normalized phrasings.
pub fn reuse_ratio(step_phrasings: &[String]) -> f64 {
    if step_phrasings.is_empty() {
        return 0.0;
    }
    let distinct: std::collections::BTreeSet<&String> = step_phrasings.iter().collect();
    step_phrasings.len() as f64 / distinct.len() as f64
}

/// Walk the permanent corpus and build the tree report.
pub fn build_tree(root: &Path) -> Result<Vec<GroupRow>> {
    let specs_root = root.join("givn/specs");
    let mut groups: BTreeMap<String, GroupRow> = BTreeMap::new();

    for entry in std::fs::read_dir(&specs_root)? {
        let group_dir = entry?.path();
        if !group_dir.is_dir() {
            continue;
        }
        let group_name = group_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let group = groups
            .entry(group_name.clone())
            .or_insert_with(|| GroupRow {
                name: group_name.clone(),
                actor: None,
                goal: None,
                capabilities: Vec::new(),
                reuse_ratio: 0.0,
            });

        // Parse group.md for Actor/Goal when present.
        let group_md_path = group_dir.join("group.md");
        if group_md_path.exists() {
            let text = std::fs::read_to_string(&group_md_path)?;
            if let Ok(doc) = crate::gherkin::group_md::parse_group_md(&text) {
                if !doc.actor.is_empty() {
                    group.actor = Some(doc.actor);
                }
                if !doc.goal.is_empty() {
                    group.goal = Some(doc.goal);
                }
            }
        }

        // Per-capability rows.
        let mut phrasings: Vec<String> = Vec::new();
        let mut caps: Vec<CapabilityRow> = Vec::new();
        for file in std::fs::read_dir(&group_dir)? {
            let file = file?.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", file.display()))?;
            let name = file
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            for scenario in &feature.scenarios {
                for step in &scenario.steps {
                    phrasings.push(normalize_phrasing(&step.value));
                }
            }
            let e2e = feature
                .scenarios
                .iter()
                .filter(|s| s.tags.iter().any(|t| t == "e2e"))
                .count();
            caps.push(CapabilityRow {
                name,
                scenarios: feature.scenarios.len(),
                e2e,
            });
        }
        caps.sort_by(|a, b| a.name.cmp(&b.name));
        group.capabilities = caps;
        group.reuse_ratio = (reuse_ratio(&phrasings) * 100.0).round() / 100.0;
    }

    Ok(groups.into_values().collect())
}

/// Print the tree in human-readable text.
pub fn print_tree(rows: &[GroupRow]) {
    for group in rows {
        let total: usize = group.capabilities.iter().map(|c| c.scenarios).sum();
        let e2e: usize = group.capabilities.iter().map(|c| c.e2e).sum();
        println!(
            "group '{}' — {} capabilities, {} scenarios, {} @e2e, reuse {:.2}x",
            group.name,
            group.capabilities.len(),
            total,
            e2e,
            group.reuse_ratio
        );
        if let Some(actor) = &group.actor {
            println!("  actor: {actor}");
        }
        if let Some(goal) = &group.goal {
            println!("  goal:  {goal}");
        }
        for cap in &group.capabilities {
            println!(
                "  - {} ({} scenarios, {} @e2e)",
                cap.name, cap.scenarios, cap.e2e
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_phrasing_masks_quotes_digits_and_whitespace() {
        assert_eq!(
            normalize_phrasing("  the user   runs \"a thing\" with 3  tries"),
            "the user runs \"x\" with n tries"
        );
    }

    #[test]
    fn reuse_ratio_counts_pooled_reuse() {
        let steps = vec![
            "the user runs \"x\"".to_string(),
            "the user runs \"y\"".to_string(),
            "the user runs \"x\"".to_string(),
        ];
        assert!((reuse_ratio(&steps) - 1.5).abs() < 1e-9);
        assert_eq!(reuse_ratio(&[]), 0.0);
    }

    #[test]
    fn build_tree_reports_groups_capabilities_and_e2e_counts() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("workers")).unwrap();
        std::fs::write(
            base.join("workers/group.md"),
            "# Group: workers\n\n## Actor\n\nA builder.\n\n## Goal\n\nBuild things.\n\n## Interactions\n\n- build\n",
        )
        .unwrap();
        std::fs::write(
            base.join("workers/digging.feature"),
            "Feature: Digging\n\n  Scenario: Dig\n    Given x\n    Then y\n\n  @e2e\n  Scenario: Shovel\n    Given x\n    Then y\n",
        )
        .unwrap();
        let rows = build_tree(root.path()).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "workers");
        assert_eq!(rows[0].actor.as_deref(), Some("A builder."));
        assert_eq!(rows[0].capabilities.len(), 1);
        assert_eq!(rows[0].capabilities[0].scenarios, 2);
        assert_eq!(rows[0].capabilities[0].e2e, 1);
        assert!(rows[0].reuse_ratio > 0.0);
    }
}
