//! In-memory projection of the corpus a change would produce.
//!
//! The projection reads the permanent specifications into a `SpecCorpus`,
//! overlays the change's narratives, applies the merge plan with the shared
//! merge-text function, and runs the interaction gate. It never creates a
//! temporary directory, writes a file, or merges the permanent tree.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::gherkin::interaction::interaction_gate_findings;
use crate::overlap::analysis::Finding;
use crate::spec::corpus::SpecCorpus;
use crate::spec::merge_plan::{
    MergeEntry, MergePlanError, build_merge_plan, collect_narrative_documents, merge_feature_text,
    narrative_target,
};

/// The archive progress already recorded for a resumed candidate.
#[derive(Debug, Clone, Copy, Default)]
pub struct MergeProgress<'a> {
    pub merged: &'a [String],
    pub in_progress: Option<&'a str>,
    pub retry_unrecorded: bool,
}

impl MergeProgress<'_> {
    pub fn empty() -> Self {
        Self::default()
    }
}

/// Build the projected corpus from a plan and run the interaction gate.
pub fn interaction_findings_for_plan(
    project_root: &Path,
    change_specs: &Path,
    plan: &[MergeEntry],
    progress: MergeProgress<'_>,
) -> Result<Vec<Finding>> {
    let specs_root = project_root.join("givn/specs");
    let mut corpus = SpecCorpus::read(&specs_root)?;

    let mut documents = Vec::new();
    collect_narrative_documents(change_specs, &mut documents)?;
    for document in documents {
        let relative = document
            .strip_prefix(change_specs)
            .with_context(|| format!("narrative '{}' is outside the change", document.display()))?;
        let target = narrative_target(&specs_root, relative)?;
        let key = corpus_key(&specs_root, &target)?;
        let content = std::fs::read_to_string(&document)
            .with_context(|| format!("cannot read narrative '{}'", document.display()))?;
        corpus.overlay_narrative(&key, content);
    }

    for entry in plan {
        if progress
            .merged
            .iter()
            .any(|relative| relative == &entry.relative)
        {
            continue;
        }
        let retry =
            progress.retry_unrecorded || progress.in_progress == Some(entry.relative.as_str());
        if entry.retire {
            corpus.remove_capability(&entry.target_rel, &entry.cap);
            continue;
        }
        let permanent = corpus.features.get(&entry.target_rel).map(String::as_str);
        let merged = merge_feature_text(permanent, &entry.feature, retry)?;
        corpus.features.insert(entry.target_rel.clone(), merged);
    }

    Ok(interaction_gate_findings(&corpus))
}

/// Build the plan for an active change and project it.
pub fn interaction_findings(project_root: &Path, change: &str) -> Result<Vec<Finding>> {
    let change_dir = crate::state::resolve_change_dir(project_root, change)?;
    let source_specs = change_dir.join("specs");
    let plan = match build_merge_plan(project_root, &change_dir, change, &source_specs, false) {
        Ok(plan) => plan,
        Err(MergePlanError::UnplaceableCapability { capability, detail }) => {
            return Ok(vec![Finding {
                kind: "interaction_mismatch".to_string(),
                level: "error".to_string(),
                files: vec![source_specs.display().to_string()],
                titles: vec![format!(
                    "interaction inventory mismatch: capability '{capability}' cannot be placed \
                     in the projected corpus: {detail}"
                )],
                contradiction: false,
            }]);
        }
        Err(error) => return Err(error.into()),
    };
    interaction_findings_for_plan(project_root, &source_specs, &plan, MergeProgress::empty())
}

/// Run the same gate on the actual permanent corpus.
pub fn permanent_interaction_findings(project_root: &Path) -> Result<Vec<Finding>> {
    let specs_root = project_root.join("givn/specs");
    let corpus = SpecCorpus::read(&specs_root)?;
    Ok(interaction_gate_findings(&corpus))
}

fn corpus_key(specs_root: &Path, target: &Path) -> Result<PathBuf> {
    let relative = target.strip_prefix(specs_root).with_context(|| {
        format!(
            "narrative target '{}' is outside '{}'",
            target.display(),
            specs_root.display()
        )
    })?;
    Ok(PathBuf::from("givn/specs").join(relative))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAGMENT: &str = "# Fragment: support\n\n## Capabilities\n\n- place-order\n\n## Interactions\n\n| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| place-order | submit an order | Consumer submits an order |\n\n## Diagram\n\n```mermaid\nflowchart LR\n A --> B\n```\n";

    fn project(with_row: bool) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let specs = root.path().join("givn/specs/fragments");
        std::fs::create_dir_all(&specs).unwrap();
        std::fs::write(specs.join("fragment.md"), FRAGMENT).unwrap();
        std::fs::write(
            specs.join("place-order.feature"),
            "Feature: Place order\n  @e2e\n  Scenario: Consumer submits an order\n    Given x\n",
        )
        .unwrap();
        let change = root.path().join("givn/changes/demo/specs/fragments");
        std::fs::create_dir_all(&change).unwrap();
        std::fs::write(
            change.join("place-order.feature"),
            "@givn.delta @place-order\nFeature: Place order\n  @givn.added @place-order @e2e\n  Scenario: Consumer cancels an order\n    Given y\n",
        )
        .unwrap();
        if with_row {
            let document = FRAGMENT.replace(
                "| place-order | submit an order | Consumer submits an order |",
                "| place-order | submit an order | Consumer submits an order |\n| place-order | cancel an order | Consumer cancels an order |",
            );
            std::fs::write(change.join("fragment.md"), document).unwrap();
        }
        root
    }

    #[test]
    fn a_consistent_projection_passes() {
        let root = project(true);
        let findings = interaction_findings(root.path(), "demo").unwrap();
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn an_unlisted_projected_scenario_fails() {
        let root = project(false);
        let findings = interaction_findings(root.path(), "demo").unwrap();
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "interaction_mismatch");
        assert!(
            findings[0].titles[0].contains("unlisted @e2e scenario 'Consumer cancels an order'"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_pre_existing_mismatch_is_not_subtracted() {
        let root = project(true);
        let specs = root.path().join("givn/specs/fragments");
        std::fs::write(
            specs.join("place-order.feature"),
            "Feature: Place order\n  @e2e\n  Scenario: Consumer submits an order\n    Given x\n  @e2e\n  Scenario: Consumer refunds an order\n    Given y\n",
        )
        .unwrap();
        let findings = interaction_findings(root.path(), "demo").unwrap();
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(
            findings[0].titles[0].contains("unlisted @e2e scenario 'Consumer refunds an order'"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_merged_entry_is_skipped_by_the_projection() {
        let root = project(false);
        let change = root.path().join("givn/changes/demo");
        let specs = change.join("specs");
        let plan = build_merge_plan(root.path(), &change, "demo", &specs, false).unwrap();
        assert_eq!(plan.len(), 1);
        let merged = vec![plan[0].relative.clone()];
        let progress = MergeProgress {
            merged: &merged,
            in_progress: None,
            retry_unrecorded: false,
        };
        let findings = interaction_findings_for_plan(root.path(), &specs, &plan, progress).unwrap();
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn the_projection_writes_no_file() {
        let root = project(true);
        let before = snapshot_tree(root.path());
        let findings = interaction_findings(root.path(), "demo").unwrap();
        assert!(findings.is_empty(), "{findings:?}");
        let after = snapshot_tree(root.path());
        assert_eq!(before, after);
    }

    fn snapshot_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        fn walk(root: &Path, dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(root, &path, out);
                } else if let Ok(bytes) = std::fs::read(&path) {
                    out.push((path.strip_prefix(root).unwrap_or(&path).to_path_buf(), bytes));
                }
            }
        }
        let mut out = Vec::new();
        walk(root, root, &mut out);
        out.sort_by(|left, right| left.0.cmp(&right.0));
        out
    }

    #[test]
    fn a_retired_capability_is_removed_from_the_projection() {
        let root = tempfile::TempDir::new().unwrap();
        let specs = root.path().join("givn/specs/orders");
        std::fs::create_dir_all(&specs).unwrap();
        let document = crate::gherkin::usecase_md::usecase_md_template()
            .replace("<usecase-id>", "orders")
            .replace("<capability-id>", "order")
            .replace("<actor role>", "Operator")
            .replace("<the domain goal>", "Run an order.")
            .replace("<consumer action>", "run an order")
            .replace("<scenario title>", "No E2E interaction");
        std::fs::write(specs.join("usecase.md"), document).unwrap();
        std::fs::write(
            specs.join("order.feature"),
            "Feature: Order\n  Scenario: Internal\n    Given x\n",
        )
        .unwrap();
        let change = root.path().join("givn/changes/demo/specs/orders");
        std::fs::create_dir_all(&change).unwrap();
        std::fs::write(
            change.join("order.feature"),
            "@givn.delta @order\n@givn.retired\nFeature: Order\n",
        )
        .unwrap();
        let findings = interaction_findings(root.path(), "demo").unwrap();
        assert!(findings.is_empty(), "{findings:?}");
    }
}
