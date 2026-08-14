//! Deterministic scenario-overlap analysis over the whole active tree.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::gherkin::delta::{delta_op, DeltaOp};
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
    let capability = file
        .parent()
        .filter(|p| p.parent() == Some(specs_root))
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned());
    for scenario in &feature.scenarios {
        let op = if is_delta { delta_op(scenario) } else { DeltaOp::Added };
        let fingerprint = Fingerprint::of_scenario(scenario, feature.background.as_ref());
        out.push(ScenarioInfo {
            title: scenario.name.clone(),
            file: file.to_path_buf(),
            capability: capability.clone(),
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
        _ => 3,
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
        let non_modified: Vec<&ScenarioInfo> =
            group.iter().copied().filter(|s| s.op != DeltaOp::Modified).collect();
        let modified: Vec<&ScenarioInfo> =
            group.iter().copied().filter(|s| s.op == DeltaOp::Modified).collect();
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

fn long_scenarios(scenarios: &[ScenarioInfo]) -> Vec<Finding> {
    scenarios
        .iter()
        .filter(|s| s.op != DeltaOp::Removed && s.step_count > 14)
        .map(|s| Finding {
            kind: "long_scenario".to_string(),
            level: "warning".to_string(),
            files: vec![s.file.display().to_string()],
            titles: vec![s.title.clone()],
            contradiction: false,
        })
        .collect()
}
