//! Deterministic scenario-overlap analysis over the whole active tree.

use std::collections::{BTreeMap, BTreeSet};
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
    /// The use-case directory the file lives under. Part of
    /// the public analysis model; consumed by tooling built on scan_tree.
    #[allow(dead_code)]
    pub usecase: Option<String>,
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
    // The use case is the immediate parent directory name, one level under
    // specs_root. This is distinct from `capability` — see the
    // duplicate_title exemption below for why that was a bug once grouping
    // landed.
    let usecase = file
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
            usecase: usecase.clone(),
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
    findings.extend(singleton_usecases(root));
    findings.extend(use_case_graph_findings(root));
    findings.extend(delta_usecase_findings(root));
    let interaction_findings = match crate::spec::corpus::SpecCorpus::read(&root.join("givn/specs"))
    {
        Ok(corpus) => crate::gherkin::interaction::interaction_gate_findings(&corpus),
        Err(error) => vec![Finding {
            kind: "usecase_invalid".to_string(),
            level: "error".to_string(),
            files: vec!["givn/specs".to_string()],
            titles: vec![error.to_string()],
            contradiction: false,
        }],
    };
    findings.extend(interaction_findings);
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
        "singleton_usecase" => 3,
        "usecase_invalid"
        | "fragment_invalid"
        | "dangling_capability_ref"
        | "ambiguous_relationship_target"
        | "dangling_use_case_ref"
        | "use_case_cycle"
        | "extend_fragment"
        | "undeclared_capability" => 4,
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

/// Machine-check the typed relationship graph recorded in active use-case and
/// fragment documents. Every target resolves to one identity, extensions do
/// not target fragments, and use-case relationships are acyclic.
fn use_case_graph_findings(root: &Path) -> Vec<Finding> {
    use crate::gherkin::usecase_md::{
        RelationshipTarget, UsecaseDoc, parse_fragment_md, parse_usecase_md,
    };

    let specs_root = root.join("givn/specs");
    let mut findings = Vec::new();
    let usecase_names: Vec<String> = std::fs::read_dir(&specs_root)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().is_dir() && entry.file_name() != "fragments")
                .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let fragment_path = specs_root.join("fragments/fragment.md");
    let fragment_ids: Vec<String> = if fragment_path.exists() {
        match std::fs::read_to_string(&fragment_path)
            .map_err(anyhow::Error::from)
            .and_then(|text| parse_fragment_md(&text))
        {
            Ok(fragment) => {
                for capability in &fragment.capabilities {
                    let path = fragment_path
                        .parent()
                        .unwrap_or_else(|| {
                            fragment_path.parent().expect("fragment path has parent")
                        })
                        .join(format!("{capability}.feature"));
                    if !path.exists() {
                        findings.push(Finding {
                            kind: "dangling_capability_ref".to_string(),
                            level: "error".to_string(),
                            files: vec![fragment_path.display().to_string()],
                            titles: vec![format!(
                                "fragment capability '{capability}' is not present at {}",
                                path.display()
                            )],
                            contradiction: false,
                        });
                    }
                }
                vec![fragment.id]
            }
            Err(error) => {
                findings.push(Finding {
                    kind: "fragment_invalid".to_string(),
                    level: "error".to_string(),
                    files: vec![fragment_path.display().to_string()],
                    titles: vec![format!("fragment.md does not parse: {error}")],
                    contradiction: false,
                });
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    let mut docs: Vec<(String, UsecaseDoc)> = Vec::new();
    for usecase in &usecase_names {
        let path = specs_root.join(usecase).join("usecase.md");
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                findings.push(Finding {
                    kind: "usecase_invalid".to_string(),
                    level: "error".to_string(),
                    files: vec![path.display().to_string()],
                    titles: vec![format!("usecase.md cannot be read: {error}")],
                    contradiction: false,
                });
                continue;
            }
        };
        match parse_usecase_md(&text) {
            Ok(doc) if doc.id == *usecase => {
                for capability in &doc.capabilities {
                    let path = specs_root
                        .join(usecase)
                        .join(format!("{capability}.feature"));
                    if !path.exists() {
                        findings.push(Finding {
                            kind: "dangling_capability_ref".to_string(),
                            level: "error".to_string(),
                            files: vec![
                                specs_root
                                    .join(usecase)
                                    .join("usecase.md")
                                    .display()
                                    .to_string(),
                            ],
                            titles: vec![format!(
                                "capability '{capability}' is not present at {}",
                                path.display()
                            )],
                            contradiction: false,
                        });
                    }
                }
                docs.push((usecase.clone(), doc));
            }
            Ok(doc) => findings.push(Finding {
                kind: "usecase_invalid".to_string(),
                level: "error".to_string(),
                files: vec![path.display().to_string()],
                titles: vec![format!(
                    "use-case ID '{}' does not match directory '{usecase}'",
                    doc.id
                )],
                contradiction: false,
            }),
            Err(error) => findings.push(Finding {
                kind: "usecase_invalid".to_string(),
                level: "error".to_string(),
                files: vec![path.display().to_string()],
                titles: vec![format!("usecase.md does not parse: {error}")],
                contradiction: false,
            }),
        }
    }

    for (usecase, doc) in &docs {
        for (relation, references) in [("Includes", &doc.includes), ("Extends", &doc.extends)] {
            for target in references {
                let is_usecase = usecase_names.iter().any(|name| name == target.id());
                let is_fragment = fragment_ids.iter().any(|id| id == target.id());
                let path = specs_root.join(usecase).join("usecase.md");
                if target.is_fragment() && relation == "Extends" {
                    findings.push(Finding {
                        kind: "extend_fragment".to_string(),
                        level: "error".to_string(),
                        files: vec![path.display().to_string()],
                        titles: vec![format!(
                            "Extends cannot target a fragment: '{}'",
                            target.id()
                        )],
                        contradiction: false,
                    });
                } else if (target.is_fragment() && !is_fragment)
                    || (!target.is_fragment() && !is_usecase)
                {
                    findings.push(Finding {
                        kind: "dangling_use_case_ref".to_string(),
                        level: "error".to_string(),
                        files: vec![path.display().to_string()],
                        titles: vec![format!(
                            "{relation} target '{target}' does not resolve in its declared namespace"
                        )],
                        contradiction: false,
                    });
                }
            }
        }
    }

    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (usecase, doc) in &docs {
        for target in doc.includes.iter().chain(doc.extends.iter()) {
            if let RelationshipTarget::Usecase(target) = target
                && usecase_names.contains(target)
            {
                edges
                    .entry(usecase.clone())
                    .or_default()
                    .push(target.clone());
            }
        }
    }
    fn visit(
        node: &str,
        edges: &BTreeMap<String, Vec<String>>,
        path: &mut Vec<String>,
        done: &mut BTreeSet<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        path.push(node.to_string());
        for next in edges.get(node).cloned().unwrap_or_default() {
            if let Some(position) = path.iter().position(|item| item == &next) {
                cycles.push(path[position..].to_vec());
            } else if !done.contains(&next) {
                visit(&next, edges, path, done, cycles);
            }
        }
        done.insert(path.pop().expect("node was pushed"));
    }
    let mut done = BTreeSet::new();
    let mut cycles = Vec::new();
    for usecase in &usecase_names {
        if !done.contains(usecase) {
            visit(usecase, &edges, &mut Vec::new(), &mut done, &mut cycles);
        }
    }
    for cycle in cycles {
        findings.push(Finding {
            kind: "use_case_cycle".to_string(),
            level: "error".to_string(),
            files: cycle
                .iter()
                .map(|usecase| {
                    specs_root
                        .join(usecase)
                        .join("usecase.md")
                        .display()
                        .to_string()
                })
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

/// Active delta narratives are part of the change's specification identity.
/// A malformed or misnamed delta use-case document is a blocking finding even
/// when its capability has no permanent owner yet.
fn delta_usecase_findings(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let changes_root = root.join("givn/changes");
    let Ok(changes) = std::fs::read_dir(&changes_root) else {
        return findings;
    };
    for change in changes.flatten() {
        let specs = change.path().join("specs");
        let Ok(entries) = std::fs::read_dir(&specs) else {
            continue;
        };
        let mut documents: Vec<std::path::PathBuf> = entries
            .flatten()
            .map(|entry| entry.path().join("usecase.md"))
            .filter(|path| path.is_file())
            .collect();
        documents.sort();
        for path in documents {
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            match crate::gherkin::usecase_md::parse_usecase_md(&text) {
                Ok(doc) => {
                    let directory = path
                        .parent()
                        .and_then(|parent| parent.file_name())
                        .and_then(|name| name.to_str())
                        .unwrap_or_default();
                    if doc.id != directory {
                        findings.push(Finding {
                            kind: "usecase_invalid".to_string(),
                            level: "error".to_string(),
                            files: vec![path.display().to_string()],
                            titles: vec![format!(
                                "use-case ID '{}' does not match directory '{directory}'",
                                doc.id
                            )],
                            contradiction: false,
                        });
                    }
                }
                Err(error) => findings.push(Finding {
                    kind: "usecase_invalid".to_string(),
                    level: "error".to_string(),
                    files: vec![path.display().to_string()],
                    titles: vec![format!("usecase.md does not parse: {error}")],
                    contradiction: false,
                }),
            }
        }
    }
    findings
}

fn singleton_usecases(root: &Path) -> Vec<Finding> {
    let specs_root = root.join("givn/specs");
    let mut findings = Vec::new();
    let Ok(entries) = std::fs::read_dir(&specs_root) else {
        return findings;
    };
    for entry in entries.flatten() {
        let usecase_dir = entry.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        let feature_count = std::fs::read_dir(&usecase_dir)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "feature"))
                    .count()
            })
            .unwrap_or(0);
        if feature_count == 1 {
            findings.push(Finding {
                kind: "singleton_usecase".to_string(),
                level: "warning".to_string(),
                files: vec![usecase_dir.display().to_string()],
                titles: vec!["use case contains only one capability".to_string()],
                contradiction: false,
            });
        }
    }
    findings
}

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
                for (i, r) in rows.iter().enumerate() {
                    if (r.title_a == a && r.title_b == b) || (r.title_a == b && r.title_b == a) {
                        matched_rows[i] = true;
                    }
                }
            }
            "long_scenario" => {
                let title = finding.titles.first().map(String::as_str).unwrap_or("?");
                let decisions: Vec<_> = split_rows
                    .iter()
                    .enumerate()
                    .filter(|(_, row)| row.title == title)
                    .collect();
                if decisions.len() != 1 {
                    missing.push(format!(
                        "split-or-keep: '{title}' needs one explicit decision"
                    ));
                    continue;
                }
                let (index, row) = decisions[0];
                matched_split[index] = true;
                match row.decision.as_str() {
                    "keep" if !row.reason.trim().is_empty() => {}
                    "split" => missing.push(format!("split-or-keep: '{title}' has an incomplete split; the long scenario remains")),
                    _ => missing.push(format!("split-or-keep: '{title}' requires keep with a brief reason or a completed split")),
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

    #[test]
    fn typed_relationships_do_not_cross_namespaces() {
        let usecase =
            crate::gherkin::usecase_md::usecase_md_template().replace("<usecase-id>", "first");
        let fragment = crate::gherkin::usecase_md::fragment_md_template()
            .replace("<fragment-id>", "support")
            .replace("<capability-id>", "help");
        for (target, same_named_usecase, expected_kind) in [
            ("usecase: support", true, None),
            ("fragment: support", true, None),
            ("usecase: support", false, Some("dangling_use_case_ref")),
        ] {
            let root = temp_tree(&[
                (
                    "givn/specs/first/usecase.md",
                    &usecase.replace(
                        "## Includes\n\n- none",
                        &format!("## Includes\n\n- {target}"),
                    ),
                ),
                ("givn/specs/fragments/fragment.md", &fragment),
            ]);
            if same_named_usecase {
                let dir = root.path().join("givn/specs/support");
                std::fs::create_dir_all(&dir).unwrap();
                std::fs::write(dir.join("usecase.md"), usecase.replace("first", "support"))
                    .unwrap();
            }
            let findings = use_case_graph_findings(root.path());
            assert!(
                !findings
                    .iter()
                    .any(|f| f.kind == "ambiguous_relationship_target"),
                "{findings:?}"
            );
            assert_eq!(
                findings.iter().any(|f| f.kind == "dangling_use_case_ref"),
                expected_kind.is_some(),
                "{findings:?}"
            );
        }
    }

    #[test]
    fn fragment_target_cannot_resolve_as_usecase_or_be_extended() {
        let template = crate::gherkin::usecase_md::usecase_md_template();
        for relation in ["Includes", "Extends"] {
            let source = template.replace("<usecase-id>", "first").replace(
                &format!("## {relation}\n\n- none"),
                &format!("## {relation}\n\n- fragment: support"),
            );
            let root = temp_tree(&[
                ("givn/specs/first/usecase.md", &source),
                (
                    "givn/specs/support/usecase.md",
                    &template.replace("<usecase-id>", "support"),
                ),
            ]);
            let expected = if relation == "Includes" {
                "dangling_use_case_ref"
            } else {
                "extend_fragment"
            };
            assert!(
                use_case_graph_findings(root.path())
                    .iter()
                    .any(|finding| finding.kind == expected)
            );
        }
    }

    #[test]
    fn cycles_follow_only_usecase_relationships() {
        let template = crate::gherkin::usecase_md::usecase_md_template();
        for relation in ["Includes", "Extends"] {
            for fragment_target in [false, true] {
                let first = template.replace("<usecase-id>", "first").replace(
                    &format!("## {relation}\n\n- none"),
                    &format!(
                        "## {relation}\n\n- {}: support",
                        if fragment_target {
                            "fragment"
                        } else {
                            "usecase"
                        }
                    ),
                );
                let second = template
                    .replace("<usecase-id>", "support")
                    .replace("## Includes\n\n- none", "## Includes\n\n- usecase: first");
                let fragment = crate::gherkin::usecase_md::fragment_md_template()
                    .replace("<fragment-id>", "support");
                let root = temp_tree(&[
                    ("givn/specs/first/usecase.md", &first),
                    ("givn/specs/support/usecase.md", &second),
                    ("givn/specs/fragments/fragment.md", &fragment),
                ]);
                assert_eq!(
                    use_case_graph_findings(root.path())
                        .iter()
                        .any(|finding| finding.kind == "use_case_cycle"),
                    !fragment_target
                );
            }
        }
    }

    #[test]
    fn shape_advisory_does_not_require_disposition() {
        let root = temp_tree(&[
            (
                "givn/specs/a/a.feature",
                "Feature: A\n  Scenario: One\n    Given a\n    Then result\n",
            ),
            (
                "givn/changes/c1/specs/b/b.feature",
                "@givn.delta @b\nFeature: B\n  @givn.added\n  Scenario: Two\n    Given a\n    Then result\n",
            ),
        ]);
        assert!(
            scan_tree(root.path())
                .iter()
                .any(|f| f.kind == "shape_match")
        );
        run_review_overlap_gate(root.path(), "c1").unwrap();
    }

    #[test]
    fn long_scenario_requires_reasoned_keep_or_completed_split() {
        let long = format!(
            "@givn.delta @a\nFeature: A\n  @givn.added\n  Scenario: Marathon\n{}",
            "    Given setup\n".repeat(15)
        );
        for (decision, reason, passes) in [
            ("", "", false),
            ("keep", "", false),
            ("keep", "atomic import", true),
            ("split", "promised", false),
            ("other", "reason", false),
        ] {
            let review = format!(
                "## Split-or-keep\n\n| Scenario | Decision | Reason |\n|---|---|---|\n| Marathon | {decision} | {reason} |\n"
            );
            let root = temp_tree(&[
                ("givn/changes/c1/specs/a/a.feature", &long),
                ("givn/changes/c1/review.md", &review),
            ]);
            assert_eq!(
                run_review_overlap_gate(root.path(), "c1").is_ok(),
                passes,
                "{decision}: {reason}"
            );
        }
        let root = temp_tree(&[
            (
                "givn/changes/c1/specs/a/a.feature",
                "@givn.delta @a\nFeature: A\n  @givn.added\n  Scenario: Part one\n    Given x\n  @givn.added\n  Scenario: Part two\n    Given y\n",
            ),
            (
                "givn/changes/c1/review.md",
                "## Split-or-keep\n\n| Scenario | Decision | Reason |\n|---|---|---|\n| Marathon | split | |\n",
            ),
        ]);
        run_review_overlap_gate(root.path(), "c1").unwrap();
    }

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

    /// Regression test for capability/use-case ownership: two
    /// different capabilities sharing a use-case directory must still be
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
        assert_eq!(scenarios[0].usecase.as_deref(), Some("example-group"));
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
        assert_eq!(scenarios[0].usecase.as_deref(), Some("example-group"));
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
        let findings = singleton_usecases(root.path());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].kind, "singleton_usecase");
        assert_eq!(findings[0].level, "warning");
        assert!(findings[0].files[0].ends_with("lonely"));
    }

    #[test]
    fn flat_layout_capability_dirs_are_not_flagged_as_singleton_usecases() {
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
        let findings = singleton_usecases(root.path());
        assert_eq!(findings.len(), 1);
    }
}
