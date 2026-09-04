//! Interaction/e2e coverage gate (Phase 3.2 + 3.4).
//!
//! The `## Interactions` section of each group's `group.md` is the durable
//! inventory of distinct user actions (Phase 3.1). This module gates the
//! `@e2e` scenario count of the group's capabilities against that
//! inventory — count-based, never fuzzy title matching (an interaction
//! description will not be word-for-word identical to a scenario title,
//! and building a soft similarity comparison to pretend otherwise is
//! exactly what this initiative removed).
//!
//! Counting rules (docstring-aware, Phase 3.3): scenarios are counted from
//! the parsed AST only — never a text search, so Gherkin embedded as
//! docstring payload cannot produce phantom scenarios. A tagged Scenario
//! Outline counts ONCE (Phase 3.4): the `@e2e` tag marks that the outline
//! as a whole is the e2e evidence for one interaction; Examples rows are
//! input variation, not additional distinct interactions. This rule is
//! documented in `assets/instructions/specs.md` next to the e2e policy.

use std::path::Path;

use anyhow::Result;
use gherkin::GherkinEnv;
use serde::Serialize;

use crate::overlap::analysis::Finding;

/// Per-group coverage comparison.
#[derive(Debug, Clone, Serialize)]
pub struct GroupCoverage {
    pub group: String,
    pub declared_interactions: usize,
    pub e2e_scenarios: usize,
}

/// True if a scenario (or outline) carries the `@e2e` tag.
fn is_e2e(scenario: &gherkin::Scenario) -> bool {
    scenario.tags.iter().any(|t| t == "e2e")
}

/// Count the e2e evidence in a feature: one per tagged scenario, one per
/// tagged Outline (regardless of its Examples row count).
pub fn count_e2e(feature: &gherkin::Feature) -> usize {
    feature.scenarios.iter().filter(|s| is_e2e(s)).count()
}

/// Build the per-group coverage comparison over the permanent corpus.
pub fn group_coverages(root: &Path) -> Result<Vec<GroupCoverage>> {
    let specs_root = root.join("givn/specs");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&specs_root)? {
        let group_dir = entry?.path();
        if !group_dir.is_dir() {
            continue;
        }
        let group = group_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let mut declared = 0usize;
        let group_md = group_dir.join("group.md");
        if group_md.exists() {
            let text = std::fs::read_to_string(&group_md)?;
            match crate::gherkin::group_md::parse_group_md(&text) {
                Ok(doc) => declared = doc.interactions.len(),
                Err(_) => {} // group_md_invalid is reported by the graph check
            }
        }

        let mut e2e = 0usize;
        for file in std::fs::read_dir(&group_dir)? {
            let file = file?.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", file.display()))?;
            e2e += count_e2e(&feature);
        }
        out.push(GroupCoverage {
            group,
            declared_interactions: declared,
            e2e_scenarios: e2e,
        });
    }
    out.sort_by(|a, b| a.group.cmp(&b.group));
    Ok(out)
}

/// Gate findings for the whole tree. Level is `warning`: groups whose
/// narratives predate the inventory rule (or legitimately declare no
/// interactions) must not fail every unrelated lint run — the binding
/// enforcement for changes is the routing gate's NEW-in-group check plus
/// review-time scrutiny.
pub fn interaction_gate_findings(root: &Path) -> Vec<Finding> {
    let coverages = match group_coverages(root) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut findings = Vec::new();
    for c in coverages {
        if c.declared_interactions == c.e2e_scenarios {
            continue;
        }
        let (direction, detail) = if c.e2e_scenarios > c.declared_interactions {
            (
                "over-coverage",
                format!(
                    "more @e2e scenarios than declared interactions — every @e2e must map to \
                     one inventory entry; drop the tag or declare the interaction"
                ),
            )
        } else {
            (
                "under-coverage",
                format!(
                    "declared interactions without a matching @e2e scenario — the spec is \
                     incomplete; add the @e2e scenario or fix the inventory"
                ),
            )
        };
        findings.push(Finding {
            kind: "interaction_mismatch".to_string(),
            level: "warning".to_string(),
            files: vec![
                root.join("givn/specs")
                    .join(&c.group)
                    .join("group.md")
                    .display()
                    .to_string(),
            ],
            titles: vec![format!(
                "{direction}: {} declared interaction(s), {} @e2e scenario(s) — {detail}",
                c.declared_interactions, c.e2e_scenarios
            )],
            contradiction: false,
        });
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_group(feature_text: &str, interactions: &str) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let group = root.path().join("givn/specs/workers");
        std::fs::create_dir_all(&group).unwrap();
        std::fs::write(group.join("group.md"), interactions).unwrap();
        std::fs::write(group.join("work.feature"), feature_text).unwrap();
        root
    }

    const TWO_INTERACTIONS: &str = "# G\n\n## Actor\n\nA.\n\n## Goal\n\nG.\n\n## Interactions\n\n- build the thing\n- inspect the thing\n";

    #[test]
    fn two_declared_two_e2e_passes() {
        let root = temp_group(
            "Feature: W\n\n  @e2e\n  Scenario: Build\n    Given x\n    Then y\n\n  @e2e\n  Scenario: Inspect\n    Given x\n    Then y\n",
            TWO_INTERACTIONS,
        );
        let findings = interaction_gate_findings(root.path());
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn three_e2e_for_two_interactions_is_over_coverage() {
        let root = temp_group(
            "Feature: W\n\n  @e2e\n  Scenario: Build\n    Given x\n    Then y\n\n  @e2e\n  Scenario: Inspect\n    Given x\n    Then y\n\n  @e2e\n  Scenario: Extra\n    Given x\n    Then y\n",
            TWO_INTERACTIONS,
        );
        let findings = interaction_gate_findings(root.path());
        assert_eq!(findings.len(), 1);
        assert!(findings[0].titles[0].contains("over-coverage"));
        assert!(findings[0].titles[0].contains("2 declared interaction(s), 3 @e2e"));
    }

    #[test]
    fn one_e2e_for_two_interactions_is_under_coverage() {
        let root = temp_group(
            "Feature: W\n\n  @e2e\n  Scenario: Build\n    Given x\n    Then y\n",
            TWO_INTERACTIONS,
        );
        let findings = interaction_gate_findings(root.path());
        assert_eq!(findings.len(), 1);
        assert!(findings[0].titles[0].contains("under-coverage"));
    }

    #[test]
    fn tagged_outline_counts_once_regardless_of_examples_rows() {
        // Phase 3.4: the outline as a whole is the e2e evidence for ONE
        // interaction; Examples rows are input variation.
        let root = temp_group(
            "Feature: W\n\n  @e2e\n  Scenario Outline: Build variants\n    Given a <material>\n    Then the thing is built\n\n    Examples:\n      | material |\n      | wood     |\n      | steel    |\n      | stone    |\n",
            TWO_INTERACTIONS,
        );
        let coverages = group_coverages(root.path()).unwrap();
        assert_eq!(coverages[0].e2e_scenarios, 1, "outline counts once");
        // One interaction genuinely uncovered → under-coverage, not parity.
        let findings = interaction_gate_findings(root.path());
        assert_eq!(findings.len(), 1);
        assert!(findings[0].titles[0].contains("under-coverage"));
    }

    #[test]
    fn untagged_outline_rows_never_count() {
        let root = temp_group(
            "Feature: W\n\n  Scenario Outline: Build variants\n    Given a <material>\n    Then the thing is built\n\n    Examples:\n      | material |\n      | wood     |\n      | steel    |\n",
            TWO_INTERACTIONS,
        );
        let coverages = group_coverages(root.path()).unwrap();
        assert_eq!(coverages[0].e2e_scenarios, 0);
    }
}
