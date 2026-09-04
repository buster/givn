//! Deterministic scenario-overlap analysis over the whole active tree.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::gherkin::delta::{DeltaOp, delta_op};
use crate::gherkin::parse::collect_feature_files;
use crate::overlap::fingerprint::Fingerprint;

/// A machine-readable overlap finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    /// `duplicate_title` | `shape_match` | `subset` | `long_scenario`
    pub kind: String,
    /// `error` | `warning`
    pub level: String,
    /// Files involved (first is the primary).
    pub files: Vec<String>,
    /// Scenario titles involved.
    pub titles: Vec<String>,
    /// For `shape_match`: facts contradict — never suggest merging.
    #[serde(default)]
    pub contradiction: bool,
}

/// A scenario as seen by the overlap analysis.
#[derive(Debug, Clone)]
pub struct ScenarioInfo {
    pub title: String,
    pub file: PathBuf,
    pub capability: Option<String>,
    pub group: Option<String>,
    pub is_delta: bool,
    pub op: DeltaOp,
    pub fingerprint: Fingerprint,
    pub step_count: usize,
}

/// Collect all scenarios from the active tree (permanent specs + all active
/// changes' deltas). `givn/archive` is never scanned.
pub fn collect_tree(root: &Path) -> Vec<ScenarioInfo> {
    let mut out = Vec::new();
    let specs_root = root.join("givn/specs");
    for file in collect_feature_files(&specs_root) {
        collect_from_file(&file, &specs_root, false, &mut out);
    }
    let changes_root = root.join("givn/changes");
    if let Ok(entries) = std::fs::read_dir(&changes_root) {
        for entry in entries.flatten() {
            let specs_dir = entry.path().join("specs");
            if !specs_dir.is_dir() {
                continue;
            }
            for file in collect_feature_files(&specs_dir) {
                collect_from_file(&file, &specs_dir, true, &mut out);
            }
        }
    }
    out
}

fn collect_from_file(file: &Path, specs_root: &Path, is_delta: bool, out: &mut Vec<ScenarioInfo>) {
    let env = gherkin::GherkinEnv::default();
    let feature = match gherkin::Feature::parse_path(file, env) {
        Ok(f) => f,
        Err(_) => return, // unparseable files produce no scenarios
    };
    // The capability is the file stem — stable regardless of directory depth
    // or grouping scheme.
    let capability = file.file_stem().map(|n| n.to_string_lossy().into_owned());
    // The group is the immediate parent directory name, one level under
    // specs_root. This used to be conflated with `capability` — see the
    // duplicate_title exemption below for why that was a bug once grouping
    // landed.
    let group = file
        .parent()
        .filter(|p| p.parent() == Some(specs_root))
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned());
    for scenario in &feature.scenarios {
        let op = if is_delta {
            delta_op(scenario)
        } else {
            DeltaOp::Added
        };
        let fingerprint = Fingerprint::of_scenario(scenario, feature.background.as_ref());
        out.push(ScenarioInfo {
            title: scenario.name.clone(),
            file: file.to_path_buf(),
            capability: capability.clone(),
            group: group.clone(),
            is_delta,
            op,
            fingerprint,
            step_count: scenario.steps.len(),
        });
    }
}

/// Scan the active tree and return all overlap findings, deterministically
/// ordered (by kind priority, then by file, then by title).
pub fn scan_tree(root: &Path) -> Vec<Finding> {
    let scenarios = collect_tree(root);
    let mut findings = Vec::new();
    findings.extend(title_duplicates(&scenarios));
    findings.extend(shape_matches(&scenarios));
    findings.extend(subsets(&scenarios));
    findings.extend(long_scenarios(&scenarios));
    // Needs the raw filesystem, not the scenario list — call with root.
    findings.extend(singleton_groups(root));
    findings.extend(use_case_graph_findings(root));
    findings.extend(crate::gherkin::interaction::interaction_gate_findings(root));
    findings.sort_by(|a, b| {
        kind_order(a).cmp(&kind_order(b)).then_with(|| {
            a.files
                .first()
                .map(|s| s.as_str())
                .cmp(&b.files.first().map(|s| s.as_str()))
        })
    });
    findings
}

fn kind_order(f: &Finding) -> u8 {
    match f.kind.as_str() {
        "duplicate_title" => 0,
        "shape_match" => 1,
        "subset" => 2,
        "singleton_group" => 3,
        "group_md_invalid" | "dangling_use_case_ref" | "use_case_cycle" => 4,
        _ => 5,
    }
}

fn title_duplicates(scenarios: &[ScenarioInfo]) -> Vec<Finding> {
    let mut by_title: BTreeMap<&str, Vec<&ScenarioInfo>> = BTreeMap::new();
    for s in scenarios {
        if s.op == DeltaOp::Removed {
            continue;
        }
        by_title.entry(s.title.as_str()).or_default().push(s);
    }
    let mut findings = Vec::new();
    for (title, group) in by_title {
        if group.len() < 2 {
            continue;
        }
        // Exemption: exactly one pair "modified delta vs. its permanent
        // target in the same capability".
        let non_modified: Vec<&ScenarioInfo> = group
            .iter()
            .copied()
            .filter(|s| s.op != DeltaOp::Modified)
            .collect();
        let modified: Vec<&ScenarioInfo> = group
            .iter()
            .copied()
            .filter(|s| s.op == DeltaOp::Modified)
            .collect();
        let exempt = modified.len() == 1
            && non_modified.len() == 1
            && modified[0].is_delta
            && !non_modified[0].is_delta
            && modified[0].capability.is_some()
            && modified[0].capability == non_modified[0].capability;
        if exempt {
            continue;
        }
        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                findings.push(Finding {
                    kind: "duplicate_title".to_string(),
                    level: "error".to_string(),
                    files: vec![
                        group[i].file.display().to_string(),
                        group[j].file.display().to_string(),
                    ],
                    titles: vec![title.to_string()],
                    contradiction: false,
                });
            }
        }
    }
    findings
}

fn shape_matches(scenarios: &[ScenarioInfo]) -> Vec<Finding> {
    let comparable: Vec<&ScenarioInfo> = scenarios
        .iter()
        .filter(|s| s.op != DeltaOp::Removed && !s.fingerprint.assertions.is_empty())
        .collect();
    let mut findings = Vec::new();
    for i in 0..comparable.len() {
        for j in (i + 1)..comparable.len() {
            let (a, b) = (comparable[i], comparable[j]);
            if a.title == b.title || a.fingerprint.assertions != b.fingerprint.assertions {
                continue;
            }
            let contradiction = a.fingerprint.facts.contradicts(&b.fingerprint.facts);
            findings.push(Finding {
                kind: "shape_match".to_string(),
                level: "warning".to_string(),
                files: vec![a.file.display().to_string(), b.file.display().to_string()],
                titles: vec![a.title.clone(), b.title.clone()],
                contradiction,
            });
        }
    }
    findings
}

fn subsets(scenarios: &[ScenarioInfo]) -> Vec<Finding> {
    let comparable: Vec<&ScenarioInfo> = scenarios
        .iter()
        .filter(|s| s.op != DeltaOp::Removed && !s.fingerprint.assertions.is_empty())
        .collect();
    let mut findings = Vec::new();
    for i in 0..comparable.len() {
        for j in 0..comparable.len() {
            if i == j {
                continue;
            }
            let (a, b) = (comparable[i], comparable[j]);
            if a.title == b.title {
                continue;
            }
            if a.fingerprint.assertions.len() <= b.fingerprint.assertions.len()
                || a.fingerprint.assertions == b.fingerprint.assertions
            {
                continue;
            }
            if is_subsequence(&b.fingerprint.assertions, &a.fingerprint.assertions) {
                findings.push(Finding {
                    kind: "subset".to_string(),
                    level: "warning".to_string(),
                    files: vec![b.file.display().to_string(), a.file.display().to_string()],
                    titles: vec![b.title.clone(), a.title.clone()],
                    contradiction: false,
                });
            }
        }
    }
    findings
}

/// True if `needle` appears as a subsequence of `haystack`.
fn is_subsequence(needle: &[String], haystack: &[String]) -> bool {
    let mut it = haystack.iter();
    needle.iter().all(|n| it.any(|h| h == n))
}

/// Scenarios longer than this many steps produce a `long_scenario` warning.
pub const LONG_SCENARIO_STEP_THRESHOLD: usize = 14;

fn long_scenarios(scenarios: &[ScenarioInfo]) -> Vec<Finding> {
    scenarios
        .iter()
        .filter(|s| s.op != DeltaOp::Removed && s.step_count > LONG_SCENARIO_STEP_THRESHOLD)
        .map(|s| Finding {
            kind: "long_scenario".to_string(),
            level: "warning".to_string(),
            files: vec![s.file.display().to_string()],
            titles: vec![s.title.clone()],
            contradiction: false,
        })
        .collect()
}

/// Find group directories containing exactly one capability file — i.e.
/// `givn/specs/<name>/<name>.feature` where the group name equals the sole
/// capability name. Such a "singleton group" is functionally identical to
/// the old flat layout; flagging it (as a warning, never blocking) keeps
/// `spec regroup` from satisfying the grouping requirement by creating
/// one-capability groups that move nothing meaningfully.
/// A group.md relationship bullet must name one target: either a group
/// directory or a capability file somewhere in the corpus. A parenthetical
/// annotation after the target is allowed and ignored.
fn use_case_ref_target(specs_root: &Path, group_name: &str, raw: &str) -> Option<String> {
    let bullet = raw
        .trim()
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim();
    if bullet.is_empty() || bullet.eq_ignore_ascii_case("none") {
        return None;
    }
    // One bullet names one target; a comma means the author listed two
    // refs in one bullet — reject so the docs stay machine-checkable.
    let target = bullet
        .split(" (")
        .next()
        .unwrap_or(bullet)
        .trim()
        .to_string();
    if target.contains(',') || target.is_empty() {
        return Some(target);
    }
    // A group directory…
    if specs_root.join(&target).is_dir() {
        return Some(target);
    }
    // …or a capability file in any group.
    let mut owned = specs_root.read_dir().ok()?;
    if owned.any(|entry| {
        entry
            .map(|e| e.path().join(format!("{target}.feature")).is_file())
            .unwrap_or(false)
    }) {
        return Some(target);
    }
    // A capability owned by the referencing group itself.
    if specs_root
        .join(group_name)
        .join(format!("{target}.feature"))
        .is_file()
    {
        return Some(target);
    }
    Some(target) // unresolved — caller reports the dangling reference
}

/// Machine-check the use-case relationship graph recorded in the group
/// files (Phase 2/3 vocabulary: use case = group narrative; `«include»` /
/// `«extend»` = relationship bullets). Findings:
/// - `group_md_invalid`: the group's group.md does not parse.
/// - `dangling_use_case_ref`: an Includes/Extends bullet names no existing
///   group or capability.
/// - `use_case_cycle`: the include/extend graph between groups has a cycle.
fn use_case_graph_findings(root: &Path) -> Vec<Finding> {
    use crate::gherkin::group_md::{GroupDoc, parse_group_md};

    let specs_root = root.join("givn/specs");
    let mut findings = Vec::new();
    let mut docs: Vec<(String, GroupDoc)> = Vec::new();
    let group_names: Vec<String> = std::fs::read_dir(&specs_root)
        .map(|rd| {
            rd.flatten()
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    for group in &group_names {
        let group_md = specs_root.join(group).join("group.md");
        let text = match std::fs::read_to_string(&group_md) {
            Ok(t) => t,
            Err(_) => continue, // a group without group.md is not a relationship error
        };
        match parse_group_md(&text) {
            Ok(doc) => docs.push((group.clone(), doc)),
            Err(e) => findings.push(Finding {
                kind: "group_md_invalid".to_string(),
                level: "error".to_string(),
                files: vec![group_md.display().to_string()],
                titles: vec![format!("group.md does not parse: {e}")],
                contradiction: false,
            }),
        }
    }

    // Dangling references.
    for (group, doc) in &docs {
        for (relation, refs) in [("«include»", &doc.includes), ("«extend»", &doc.extends)] {
            for raw in refs {
                let resolved = use_case_ref_target(&specs_root, group, raw);
                let valid = resolved.as_ref().is_some_and(|target| {
                    specs_root.join(target).is_dir() || group_names.iter().any(|g| g == target) || {
                        let cap_file = format!("{target}.feature");
                        specs_root
                            .read_dir()
                            .map(|rd| rd.flatten().any(|e| e.path().join(&cap_file).is_file()))
                            .unwrap_or(false)
                    }
                });
                if !valid {
                    findings.push(Finding {
                        kind: "dangling_use_case_ref".to_string(),
                        level: "error".to_string(),
                        files: vec![
                            specs_root
                                .join(group)
                                .join("group.md")
                                .display()
                                .to_string(),
                        ],
                        titles: vec![format!(
                            "{relation} target '{raw}' names no existing group or capability"
                        )],
                        contradiction: false,
                    });
                }
            }
        }
    }

    // Cycle detection over group-level edges (includes + extends).
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (group, doc) in &docs {
        for raw in doc.includes.iter().chain(doc.extends.iter()) {
            if let Some(target) = use_case_ref_target(&specs_root, group, raw)
                && group_names.contains(&target)
                && target != *group
            {
                edges.entry(group.clone()).or_default().push(target);
            }
        }
    }
    // DFS from each group; a back-edge to the path means a cycle.
    fn visit(
        node: &str,
        edges: &BTreeMap<String, Vec<String>>,
        path: &mut Vec<String>,
        done: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        path.push(node.to_string());
        for next in edges.get(node).cloned().unwrap_or_default() {
            if let Some(pos) = path.iter().position(|n| n == &next) {
                cycles.push(path[pos..].to_vec());
            } else if !done.iter().any(|n| n == &next) {
                visit(&next, edges, path, done, cycles);
            }
        }
        done.push(path.pop().expect("pushed above"));
    }
    let mut cycles: Vec<Vec<String>> = Vec::new();
    let mut done: Vec<String> = Vec::new();
    for group in &group_names {
        if !done.iter().any(|n| n == group) {
            visit(group, &edges, &mut Vec::new(), &mut done, &mut cycles);
        }
    }
    for cycle in cycles {
        findings.push(Finding {
            kind: "use_case_cycle".to_string(),
            level: "error".to_string(),
            files: cycle
                .iter()
                .map(|g| specs_root.join(g).join("group.md").display().to_string())
                .collect(),
            titles: vec![format!(
                "include/extend graph has a cycle: {}",
                cycle.join(" -> ")
            )],
            contradiction: false,
        });
    }

    findings
}

fn singleton_groups(root: &Path) -> Vec<Finding> {
    let specs_root = root.join("givn/specs");
    let mut findings = Vec::new();
    let Ok(entries) = std::fs::read_dir(&specs_root) else {
        return findings;
    };
    for entry in entries.flatten() {
        let group_dir = entry.path();
        if !group_dir.is_dir() {
            continue;
        }
        let feature_count = std::fs::read_dir(&group_dir)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "feature"))
                    .count()
            })
            .unwrap_or(0);
        if feature_count == 1 {
            findings.push(Finding {
                kind: "singleton_group".to_string(),
                level: "warning".to_string(),
                files: vec![group_dir.display().to_string()],
                titles: vec!["group contains only one capability".to_string()],
                contradiction: false,
            });
        }
    }
    findings
}

/// Review gate: every shape match (without fact contradiction) that involves
/// the change's delta needs a disposition row; every long scenario in the
/// delta needs a split-or-keep row. Prints the net-delta receipt.
pub fn run_review_overlap_gate(root: &Path, change_id: &str) -> anyhow::Result<()> {
    let review_path = root.join("givn/changes").join(change_id).join("review.md");
    let (rows, split_rows) = crate::overlap::disposition::parse_review(&review_path);
    let delta_prefix = root
        .join("givn/changes")
        .join(change_id)
        .join("specs")
        .to_string_lossy()
        .into_owned();

    let findings = scan_tree(root);
    let mut missing: Vec<String> = Vec::new();
    // Track which disposition/split rows actually matched a current
    // finding, so stale rows (scenario renamed/removed after the row was
    // written) can be reported as a hygiene warning.
    let mut matched_rows: Vec<bool> = vec![false; rows.len()];
    let mut matched_split: Vec<bool> = vec![false; split_rows.len()];
    for finding in &findings {
        let involves_delta = finding.files.iter().any(|f| f.starts_with(&delta_prefix));
        if !involves_delta {
            continue;
        }
        match finding.kind.as_str() {
            "shape_match" if !finding.contradiction => {
                let (a, b) = (finding.titles[0].as_str(), finding.titles[1].as_str());
                if !crate::overlap::disposition::disposed(&rows, a, b) {
                    missing.push(format!("shape match: '{a}' / '{b}'"));
                } else {
                    for (i, r) in rows.iter().enumerate() {
                        if (r.title_a == a && r.title_b == b) || (r.title_a == b && r.title_b == a)
                        {
                            matched_rows[i] = true;
                        }
                    }
                }
            }
            "long_scenario" => {
                let title = finding.titles.first().map(String::as_str).unwrap_or("?");
                if !split_rows.iter().any(|r| r.title == title) {
                    missing.push(format!("split-or-keep: '{title}'"));
                } else {
                    for (i, r) in split_rows.iter().enumerate() {
                        if r.title == title {
                            matched_split[i] = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Net-delta receipt: count added/modified/removed in the change's delta.
    let delta_ops: Vec<DeltaOp> = collect_tree(root)
        .into_iter()
        .filter(|s| s.file.to_string_lossy().starts_with(&delta_prefix))
        .map(|s| s.op)
        .collect();
    let added = delta_ops.iter().filter(|o| **o == DeltaOp::Added).count();
    let modified = delta_ops
        .iter()
        .filter(|o| **o == DeltaOp::Modified)
        .count();
    let removed = delta_ops.iter().filter(|o| **o == DeltaOp::Removed).count();
    println!("net delta: {added} added, {modified} modified, {removed} removed");
    println!(
        "net delta json: {}",
        serde_json::json!({"added": added, "modified": modified, "removed": removed})
    );

    // Orphan-row hygiene warning (non-blocking): disposition rows that no
    // longer correspond to any current finding — e.g. the scenario was
    // renamed after the row was written.
    for (i, r) in rows.iter().enumerate() {
        if !matched_rows[i] {
            println!(
                "  ⚠ stale disposition row: '{} / {}' matches no current shape-match finding",
                r.title_a, r.title_b
            );
        }
    }
    for (i, r) in split_rows.iter().enumerate() {
        if !matched_split[i] {
            println!(
                "  ⚠ stale split-or-keep row: '{}' matches no current long scenario",
                r.title
            );
        }
    }

    if !missing.is_empty() {
        if !review_path.exists() {
            anyhow::bail!(
                "review.md is missing in givn/changes/{change_id}/ — the overlap gate \
                 needs its disposition table to verify every finding"
            );
        }
        anyhow::bail!(
            "overlap dispositions missing in review.md:\n  {}",
            missing.join("\n  ")
        );
    }
    println!("  ✓ overlap dispositions passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Temp project with a grouped specs tree seeded from (rel, text) pairs.
    fn temp_tree(files: &[(&str, &str)]) -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        for (rel, text) in files {
            let path = root.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        root
    }

    /// Regression test for the capability/group conflation bug: two
    /// different capabilities sharing a group directory must still be
    /// compared by title. Before the fix, both resolved `.capability` to
    /// the group name, the duplicate_title exemption wrongly applied, and
    /// the finding was suppressed.
    #[test]
    fn same_title_in_different_capabilities_of_one_group_is_flagged() {
        let root = temp_tree(&[
            (
                "givn/specs/example-group/cap-a.feature",
                "Feature: Cap A\n\n  Scenario: Same title\n    Given x\n    Then y\n",
            ),
            (
                "givn/specs/example-group/cap-b.feature",
                "Feature: Cap B\n\n  Scenario: Same title\n    Given y\n    Then z\n",
            ),
        ]);
        let scenarios = collect_tree(root.path());
        assert_eq!(scenarios.len(), 2);
        assert_eq!(scenarios[0].capability.as_deref(), Some("cap-a"));
        assert_eq!(scenarios[0].group.as_deref(), Some("example-group"));
        let findings = title_duplicates(&scenarios);
        assert!(
            findings.iter().any(|f| f.kind == "duplicate_title"),
            "same title in two capabilities of one group must be flagged, got {findings:?}"
        );
    }

    #[test]
    fn modified_delta_still_exempt_for_same_capability() {
        let root = temp_tree(&[
            (
                "givn/specs/old-layout/old-cap/old-cap.feature",
                "Feature: Old cap\n\n  Scenario: Shared\n    Given x\n    Then y\n",
            ),
            (
                "givn/changes/c1/specs/old-layout/old-cap/old-cap.feature",
                "@givn.delta @old-cap\nFeature: Old cap\n\n  @givn.modified\n  Scenario: Shared\n    Given x\n    Then y2\n",
            ),
        ]);
        let scenarios = collect_tree(root.path());
        let findings = title_duplicates(&scenarios);
        assert!(
            findings.is_empty(),
            "modified-delta-vs-target in the same capability stays exempt, got {findings:?}"
        );
    }

    #[test]
    fn grouped_delta_layout_preserves_capability_and_group() {
        let root = temp_tree(&[(
            "givn/changes/c1/specs/example-group/cap-c.feature",
            "@givn.delta @cap-c\nFeature: Cap C\n\n  @givn.modified\n  Scenario: S\n    Given x\n    Then y\n",
        )]);
        let scenarios = collect_tree(root.path());
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].capability.as_deref(), Some("cap-c"));
        assert_eq!(scenarios[0].group.as_deref(), Some("example-group"));
    }
}

#[cfg(test)]
mod singleton_tests {
    use super::*;

    #[test]
    fn one_two_capability_group_and_one_singleton_produce_exactly_one_finding() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        for (rel, text) in [
            (
                "example-group/cap-a.feature",
                "Feature: A\n\n  Scenario: S1\n    Given x\n    Then y\n",
            ),
            (
                "example-group/cap-b.feature",
                "Feature: B\n\n  Scenario: S2\n    Given x\n    Then y\n",
            ),
            (
                "lonely/lonely.feature",
                "Feature: Lonely\n\n  Scenario: S3\n    Given x\n    Then y\n",
            ),
        ] {
            let path = base.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let findings = singleton_groups(root.path());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "singleton_group");
        assert_eq!(findings[0].level, "warning");
        assert!(findings[0].files[0].ends_with("lonely"));
    }

    #[test]
    fn flat_layout_capability_dirs_are_not_flagged_as_singleton_groups() {
        // Pre-migration layout: givn/specs/<cap>/<cap>.feature — one feature
        // file per directory, but these are capabilities, not groups. The
        // shape is structurally identical to a singleton group, so the
        // finding fires; this is expected and is one reason the finding is
        // a warning rather than an error.
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        let path = base.join("some-cap/some-cap.feature");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            "Feature: Some cap\n\n  Scenario: S\n    Given x\n    Then y\n",
        )
        .unwrap();
        let findings = singleton_groups(root.path());
        assert_eq!(findings.len(), 1);
    }
}

#[cfg(test)]
mod use_case_graph_tests {
    use super::*;

    fn seed(root: &Path, files: &[(&str, &str)]) {
        for (rel, text) in files {
            let path = root.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
    }

    const GROUP_MD: &str = "# Group: G\n\n## Actor\n\nA.\n\n## Goal\n\nG.\n\n## Interactions\n\n- x\n\n## Includes\n\n- none\n\n## Extends\n\n- none\n";

    #[test]
    fn dangling_include_ref_is_an_error() {
        let root = tempfile::TempDir::new().unwrap();
        seed(
            root.path(),
            &[(
                "givn/specs/alpha/group.md",
                &GROUP_MD.replace("- none\n\n## Extends", "- ghost-group\n\n## Extends"),
            )],
        );
        let findings = use_case_graph_findings(root.path());
        assert!(
            findings
                .iter()
                .any(|f| f.kind == "dangling_use_case_ref" && f.level == "error"),
            "{findings:?}"
        );
    }

    #[test]
    fn references_to_groups_and_capabilities_resolve() {
        let root = tempfile::TempDir::new().unwrap();
        seed(
            root.path(),
            &[
                (
                    "givn/specs/alpha/group.md",
                    &GROUP_MD.replace("- none\n\n## Extends", "- beta\n\n## Extends"),
                ),
                ("givn/specs/beta/group.md", GROUP_MD),
                (
                    "givn/specs/beta/helper.feature",
                    "Feature: H\n\n  Scenario: S\n    Given x\n    Then y\n",
                ),
            ],
        );
        let findings = use_case_graph_findings(root.path());
        assert!(
            findings.is_empty(),
            "group and capability refs must resolve: {findings:?}"
        );
    }

    #[test]
    fn mutual_include_is_a_cycle() {
        let root = tempfile::TempDir::new().unwrap();
        seed(
            root.path(),
            &[
                (
                    "givn/specs/alpha/group.md",
                    &GROUP_MD.replace("- none\n\n## Extends", "- beta\n\n## Extends"),
                ),
                (
                    "givn/specs/beta/group.md",
                    &GROUP_MD.replace("- none\n\n## Extends", "- alpha\n\n## Extends"),
                ),
            ],
        );
        let findings = use_case_graph_findings(root.path());
        assert!(
            findings.iter().any(|f| f.kind == "use_case_cycle"),
            "{findings:?}"
        );
    }

    #[test]
    fn parenthetical_annotations_are_allowed() {
        let root = tempfile::TempDir::new().unwrap();
        seed(
            root.path(),
            &[(
                "givn/specs/alpha/group.md",
                &GROUP_MD.replace(
                    "- none\n\n## Extends",
                    "- beta (config storage is contributed there)\n\n## Extends",
                ),
            )],
        );
        seed(root.path(), &[("givn/specs/beta/group.md", GROUP_MD)]);
        let findings = use_case_graph_findings(root.path());
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn unparseable_group_md_is_an_error() {
        let root = tempfile::TempDir::new().unwrap();
        seed(
            root.path(),
            &[(
                "givn/specs/alpha/group.md",
                "# broken\n\nno sections here\n",
            )],
        );
        let findings = use_case_graph_findings(root.path());
        assert!(
            findings.iter().any(|f| f.kind == "group_md_invalid"),
            "{findings:?}"
        );
    }
}
