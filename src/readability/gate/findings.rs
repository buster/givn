//! Finding predicate, finding reasons, the post-analysis guard, and
//! unit-level decision accounting.

use super::markers::ReviewMarkers;
use super::rationale::{self, RationaleError};
use super::revision::RevisionTree;
use super::{GateFailure, GateRequest, Side};
use crate::readability::classify::{Assessment, Classification, ClassifiedUnit};
use crate::readability::model::{Analysis, FileStatus, RepoPath, UnitId, UnitKind};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingReason {
    NewType,
    NewModule,
    ExistingGrown,
    ExistingNotGrown,
    CrossedThreshold,
    MethodLength,
    Removed,
    Fixed,
}

impl FindingReason {
    pub fn label(self) -> &'static str {
        match self {
            Self::NewType => "new-type",
            Self::NewModule => "new-module",
            Self::ExistingGrown => "existing-grown",
            Self::ExistingNotGrown => "existing-not-grown",
            Self::CrossedThreshold => "crossed-threshold",
            Self::MethodLength => "method-length",
            Self::Removed => "removed",
            Self::Fixed => "fixed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decision {
    Missing,
    Fixed { marker_count: usize },
    Justified { rationale: PathBuf },
}

/// One touched unit that is, was, or becomes a gate finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateFinding {
    pub unit_id: UnitId,
    pub path: RepoPath,
    pub files: Vec<RepoPath>,
    pub kind: UnitKind,
    pub name: String,
    pub head_classification: Option<Classification>,
    pub base_code_lines: Option<u32>,
    pub head_code_lines: Option<u32>,
    pub public_methods: u32,
    pub method_findings: Vec<String>,
    pub reason: FindingReason,
    pub decision: Decision,
}

impl GateFinding {
    pub fn is_head_finding(&self) -> bool {
        matches!(
            self.reason,
            FindingReason::NewType
                | FindingReason::NewModule
                | FindingReason::ExistingGrown
                | FindingReason::ExistingNotGrown
                | FindingReason::CrossedThreshold
                | FindingReason::MethodLength
        )
    }

    fn head_label(&self) -> &'static str {
        self.head_classification
            .map(Classification::label)
            .unwrap_or("within-limits")
    }

    pub fn message(&self) -> String {
        let reason = match self.reason {
            FindingReason::NewType => format!(
                "new type: {} code lines, {} public methods; classification {}",
                self.head_code_lines.unwrap_or(0),
                self.public_methods,
                self.head_label()
            ),
            FindingReason::NewModule => format!(
                "new module: {} code lines; classification {}",
                self.head_code_lines.unwrap_or(0),
                self.head_label()
            ),
            FindingReason::ExistingGrown => format!(
                "existing finding grew: {} -> {} code lines; classification {}",
                self.base_code_lines.unwrap_or(0),
                self.head_code_lines.unwrap_or(0),
                self.head_label()
            ),
            FindingReason::ExistingNotGrown => format!(
                "existing finding did not grow: {} -> {} code lines; classification {}",
                self.base_code_lines.unwrap_or(0),
                self.head_code_lines.unwrap_or(0),
                self.head_label()
            ),
            FindingReason::CrossedThreshold => format!(
                "crossed threshold: {} -> {} code lines; classification {}",
                self.base_code_lines.unwrap_or(0),
                self.head_code_lines.unwrap_or(0),
                self.head_label()
            ),
            FindingReason::Removed => format!(
                "removed: base {} code lines; finding no longer fires",
                self.base_code_lines.unwrap_or(0)
            ),
            FindingReason::Fixed => format!(
                "fixed at head: {} -> {} code lines; classification {}",
                self.base_code_lines.unwrap_or(0),
                self.head_code_lines.unwrap_or(0),
                self.head_label()
            ),
            FindingReason::MethodLength => String::new(),
        };
        let mut parts = Vec::new();
        if !reason.is_empty() {
            parts.push(reason);
        }
        parts.extend(self.method_findings.iter().cloned());
        parts.join("; ")
    }
}

/// A gate finding is a size finding other than `data-only`, or any unit with a
/// method finding. A data-only unit is exempt from the size rule.
pub fn is_gate_finding(unit: &ClassifiedUnit) -> bool {
    (unit.classification != Classification::WithinLimits
        && unit.classification != Classification::DataOnly)
        || !unit.method_findings.is_empty()
}

fn has_size_classification(classification: Classification) -> bool {
    matches!(
        classification,
        Classification::GodType
            | Classification::Interface
            | Classification::MultiTypeModule
            | Classification::Module
            | Classification::CohesiveLargeType
    )
}

pub fn collect(
    touched: &BTreeSet<UnitId>,
    base: &Assessment,
    head: &Assessment,
) -> Vec<GateFinding> {
    let base_by_id: BTreeMap<&UnitId, &ClassifiedUnit> = base
        .units
        .iter()
        .map(|unit| (&unit.unit_id, unit))
        .collect();
    let head_by_id: BTreeMap<&UnitId, &ClassifiedUnit> = head
        .units
        .iter()
        .map(|unit| (&unit.unit_id, unit))
        .collect();

    let mut findings: Vec<GateFinding> = touched
        .iter()
        .filter_map(|id| finding_for(id, base_by_id.get(id).copied(), head_by_id.get(id).copied()))
        .collect();
    findings.sort_by(|left, right| {
        (left.path.clone(), left.unit_id.clone()).cmp(&(right.path.clone(), right.unit_id.clone()))
    });
    findings
}

fn finding_for(
    id: &UnitId,
    base: Option<&ClassifiedUnit>,
    head: Option<&ClassifiedUnit>,
) -> Option<GateFinding> {
    let base_finding = base.is_some_and(is_gate_finding);
    let head_finding = head.is_some_and(is_gate_finding);

    let reason = match (base, head) {
        (_, None) => {
            if base_finding {
                FindingReason::Removed
            } else {
                return None;
            }
        }
        (None, Some(unit)) => {
            if !head_finding {
                return None;
            }
            if unit.kind == UnitKind::Module {
                FindingReason::NewModule
            } else {
                FindingReason::NewType
            }
        }
        (Some(base_unit), Some(head_unit)) => {
            if head_finding {
                let method_only = !has_size_classification(head_unit.classification)
                    && !head_unit.method_findings.is_empty()
                    && !(base_finding && has_size_classification(base_unit.classification));
                if method_only {
                    FindingReason::MethodLength
                } else if base_finding {
                    if head_unit.metrics.code_lines > base_unit.metrics.code_lines {
                        FindingReason::ExistingGrown
                    } else {
                        FindingReason::ExistingNotGrown
                    }
                } else {
                    FindingReason::CrossedThreshold
                }
            } else if base_finding {
                FindingReason::Fixed
            } else {
                return None;
            }
        }
    };

    let source = head.or(base)?;
    Some(GateFinding {
        unit_id: id.clone(),
        path: source.path.clone(),
        files: source.files.clone(),
        kind: source.kind,
        name: source.name.clone(),
        head_classification: head.map(|unit| unit.classification),
        base_code_lines: base.map(|unit| unit.metrics.code_lines),
        head_code_lines: head.map(|unit| unit.metrics.code_lines),
        public_methods: head.map(|unit| unit.metrics.public_methods).unwrap_or(0),
        method_findings: head.map(method_messages).unwrap_or_default(),
        reason,
        decision: Decision::Missing,
    })
}

fn method_messages(unit: &ClassifiedUnit) -> Vec<String> {
    unit.methods
        .iter()
        .filter_map(|method| {
            method.finding.map(|_| {
                format!(
                    "long method: {} code lines, complexity {}; classification {}",
                    method.code_lines,
                    method.complexity,
                    unit.classification.label()
                )
            })
        })
        .collect()
}

/// Post-analysis guard: every changed production `.rs` path present at head
/// must be analyzed or named by a diagnostic. A deletion needs no head entry.
pub fn guard(
    changed: &BTreeSet<String>,
    base_tree: &RevisionTree,
    head_tree: &RevisionTree,
    base: &Analysis,
    head: &Analysis,
) -> Vec<GateFailure> {
    let mut failures = Vec::new();
    for path in changed {
        if !path.ends_with(".rs") {
            continue;
        }
        if let Some(kind) = head_tree.kind(path) {
            match kind {
                crate::scm::EntryKind::Regular => {
                    if head_tree.unreadable.iter().any(|entry| entry == path) {
                        failures.push(GateFailure::AnalysisFailed {
                            side: Side::Head,
                            path: path.clone(),
                            detail: String::new(),
                        });
                    } else {
                        check_analyzed(head, path, Side::Head, true, &mut failures);
                    }
                }
                crate::scm::EntryKind::Symlink => failures.push(GateFailure::AnalysisFailed {
                    side: Side::Head,
                    path: path.clone(),
                    detail: "symlink".to_string(),
                }),
                crate::scm::EntryKind::Submodule => failures.push(GateFailure::AnalysisFailed {
                    side: Side::Head,
                    path: path.clone(),
                    detail: "submodule".to_string(),
                }),
            }
        }
        if let Some(kind) = base_tree.kind(path) {
            match kind {
                crate::scm::EntryKind::Regular => {
                    if base_tree.unreadable.iter().any(|entry| entry == path) {
                        failures.push(GateFailure::AnalysisFailed {
                            side: Side::Base,
                            path: path.clone(),
                            detail: String::new(),
                        });
                    } else {
                        check_analyzed(base, path, Side::Base, false, &mut failures);
                    }
                }
                crate::scm::EntryKind::Symlink => failures.push(GateFailure::AnalysisFailed {
                    side: Side::Base,
                    path: path.clone(),
                    detail: "symlink".to_string(),
                }),
                crate::scm::EntryKind::Submodule => failures.push(GateFailure::AnalysisFailed {
                    side: Side::Base,
                    path: path.clone(),
                    detail: "submodule".to_string(),
                }),
            }
        }
    }
    failures
}

fn check_analyzed(
    analysis: &Analysis,
    path: &str,
    side: Side,
    head_guard: bool,
    failures: &mut Vec<GateFailure>,
) {
    if let Some(file) = analysis
        .files
        .iter()
        .find(|file| file.path.as_str() == path)
    {
        if file.status == FileStatus::Failed {
            failures.push(GateFailure::AnalysisFailed {
                side,
                path: path.to_string(),
                detail: String::new(),
            });
        }
        return;
    }
    if analysis
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.path.as_str() == path)
    {
        return;
    }
    if head_guard {
        failures.push(GateFailure::UnanalyzedFile(path.to_string()));
    }
}

/// Assign a decision to every gate finding and report every invalid or missing
/// decision. The record-missing case suppresses per-unit `unaccounted`
/// failures because the missing record is the actionable failure.
pub fn decide(
    findings: &mut [GateFinding],
    markers: &ReviewMarkers,
    request: &GateRequest,
    record_missing: bool,
) -> Vec<GateFailure> {
    let mut failures = Vec::new();

    let mut conflicted = BTreeSet::new();
    for unit in markers.fixed.keys() {
        if markers.justified.contains_key(unit) {
            conflicted.insert(unit.clone());
            failures.push(GateFailure::ConflictingDecisions {
                unit: unit.to_string(),
            });
        }
    }

    let mut by_slug: BTreeMap<String, Vec<UnitId>> = BTreeMap::new();
    for finding in findings.iter() {
        by_slug
            .entry(rationale::slug(&finding.unit_id.to_string()))
            .or_default()
            .push(finding.unit_id.clone());
    }
    let mut colliding = BTreeSet::new();
    for units in by_slug.values_mut().filter(|units| units.len() > 1) {
        units.sort();
        units.dedup();
        for pair in units.windows(2) {
            failures.push(GateFailure::SlugCollision {
                first: pair[0].to_string(),
                second: pair[1].to_string(),
            });
            colliding.insert(pair[0].clone());
            colliding.insert(pair[1].clone());
        }
    }

    for finding in findings.iter_mut() {
        let fixed = markers.fixed.get(&finding.unit_id).copied().unwrap_or(0);
        let justified = markers
            .justified
            .get(&finding.unit_id)
            .copied()
            .unwrap_or(0);

        if conflicted.contains(&finding.unit_id) {
            finding.decision = Decision::Missing;
            continue;
        }

        if fixed > 0 {
            if finding.is_head_finding() {
                failures.push(GateFailure::InvalidFixed {
                    unit: finding.unit_id.to_string(),
                    classification: finding.head_label().to_string(),
                    head: finding.head_code_lines.unwrap_or(0),
                    base: finding.base_code_lines.unwrap_or(0),
                });
            }
            finding.decision = Decision::Fixed {
                marker_count: fixed,
            };
            continue;
        }

        if justified > 0 {
            let relative = rationale::relative_path(&request.change_id, &finding.unit_id);
            if !finding.is_head_finding() {
                failures.push(GateFailure::JustifiedWithoutFinding {
                    unit: finding.unit_id.to_string(),
                });
                finding.decision = Decision::Justified {
                    rationale: PathBuf::from(&relative),
                };
            } else if colliding.contains(&finding.unit_id) {
                finding.decision = Decision::Justified {
                    rationale: PathBuf::from(&relative),
                };
            } else {
                match rationale::validate(&request.project_root, &relative, finding) {
                    Ok(_) => {
                        finding.decision = Decision::Justified {
                            rationale: PathBuf::from(&relative),
                        };
                    }
                    Err(RationaleError::Missing) => {
                        failures.push(GateFailure::MissingRationale {
                            unit: finding.unit_id.to_string(),
                            path: relative,
                        });
                        finding.decision = Decision::Missing;
                    }
                    Err(RationaleError::Invalid(message)) => {
                        failures.push(GateFailure::InvalidRationale {
                            path: relative,
                            message,
                        });
                        finding.decision = Decision::Missing;
                    }
                }
            }
            continue;
        }

        finding.decision = Decision::Missing;
        if !record_missing && markers.unknown.is_empty() {
            failures.push(GateFailure::Unaccounted {
                unit: finding.unit_id.to_string(),
            });
        }
    }

    failures
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readability::metrics::UnitMetrics;

    fn metrics(code_lines: u32, public_methods: u32) -> UnitMetrics {
        UnitMetrics {
            physical_lines: code_lines,
            code_lines,
            methods: 3,
            public_methods,
            fields: 3,
            declared_types: 1,
            public_surface: public_methods,
            lcom4: 3,
            tcc_permille: Some(100),
            lcc_permille: Some(100),
            max_method_code_lines: 10,
            max_method_complexity: 1,
            files: 1,
        }
    }

    fn unit(id: &str, classification: Classification, code_lines: u32) -> ClassifiedUnit {
        ClassifiedUnit {
            unit_id: UnitId(id.to_string()),
            kind: UnitKind::Type(crate::readability::model::TypeKind::Struct),
            name: "Engine".to_string(),
            path: RepoPath::new("src/engine.rs"),
            files: vec![RepoPath::new("src/engine.rs")],
            classification,
            metrics: metrics(code_lines, 4),
            method_findings: Vec::new(),
            method_names: Vec::new(),
            methods: Vec::new(),
            outline: Vec::new(),
            evidence: Vec::new(),
        }
    }

    #[test]
    fn data_only_is_exempt_unless_a_method_finding_exists() {
        let data_only = unit("type:src/status.rs::Status", Classification::DataOnly, 620);
        assert!(!is_gate_finding(&data_only));

        let mut with_method = data_only.clone();
        with_method.method_findings.push((
            "long".to_string(),
            crate::readability::metrics::MethodFinding::MethodLength,
        ));
        assert!(is_gate_finding(&with_method));
    }

    #[test]
    fn finding_reasons_match_the_design_messages() {
        let base = unit("type:src/engine.rs::Engine", Classification::GodType, 620);
        let mut head = base.clone();
        head.metrics = metrics(700, 4);
        let id = UnitId("type:src/engine.rs::Engine".to_string());
        let finding = finding_for(&id, Some(&base), Some(&head)).expect("finding");
        assert_eq!(finding.reason, FindingReason::ExistingGrown);
        assert_eq!(
            finding.message(),
            "existing finding grew: 620 -> 700 code lines; classification god-type"
        );

        head.classification = Classification::WithinLimits;
        head.metrics = metrics(480, 4);
        let fixed = finding_for(&id, Some(&base), Some(&head)).expect("fixed");
        assert_eq!(fixed.reason, FindingReason::Fixed);
        assert_eq!(
            fixed.message(),
            "fixed at head: 620 -> 480 code lines; classification within-limits"
        );
    }

    #[test]
    fn invalid_fixed_message_is_exact() {
        let failure = GateFailure::InvalidFixed {
            unit: "type:src/engine.rs::Engine".to_string(),
            classification: "god-type".to_string(),
            head: 620,
            base: 620,
        };
        assert_eq!(
            failure.message(),
            "FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 620 code lines (base 620)"
        );
    }
}
