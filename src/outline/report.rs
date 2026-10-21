//! Human and JSON rendering from one classified model.

use super::classify::{Assessment, Classification, ClassifiedUnit, Comparison, MetricValue};
use super::model::{
    AnalyzedFile, LineKind, MethodId, OutlineItem, OutlineKind, Resolution, SourceScope, TypeKind,
    UnitId, UnitKind, Visibility,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: u32 = 3;
pub const RULES_VERSION: u32 = 1;

pub fn render_human(assessment: &Assessment) -> String {
    if assessment.files.is_empty() {
        return "outline report — no production sources, 0 files, 0 units, 0 hints\nNo production sources found in the requested scope.\n"
            .to_string();
    }
    let hints = assessment
        .units
        .iter()
        .filter(|unit| unit.classification != Classification::WithinLimits)
        .count();
    let languages = languages_label(assessment);
    let mut out = format!(
        "outline report — {languages}, {} files, {} units, {} hints\n",
        assessment.files.len(),
        assessment.units.len(),
        hints
    );
    out.push_str(
        "outline first: method declarations, signatures, and internal calls; metrics are hints\n",
    );
    out.push_str("advisory report: this report does not block review or archive\n\n");

    let index = MethodIndex::from_assessment(assessment);
    let mut units: Vec<&ClassifiedUnit> = assessment.units.iter().collect();
    units.sort_by(|left, right| {
        left.classification
            .label()
            .cmp(right.classification.label())
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.unit_id.cmp(&right.unit_id))
    });

    for unit in units {
        let label = if unit.classification == Classification::WithinLimits {
            "WITHIN-LIMITS"
        } else {
            "HINT"
        };
        out.push_str(&format!(
            "{:<13} {:<7} {:<36} {:<44} {}\n",
            label,
            kind_label(unit.kind),
            unit.path.as_str(),
            unit.unit_id,
            unit.classification.label()
        ));
        out.push_str(&render_outline(unit, &index));
        out.push_str(&format!(
            "  metrics (hint): code lines {}; methods {}; public methods {}; fields {}; LCOM4 {}; TCC {}\n",
            unit.metrics.code_lines,
            unit.metrics.methods,
            unit.metrics.public_methods,
            unit.metrics.fields,
            unit.metrics.lcom4,
            unit.metrics
                .tcc_permille
                .map(|value| format!("{:.3}", value as f64 / 1000.0))
                .unwrap_or_else(|| "undefined".to_string()),
        ));
        if !unit.evidence.is_empty() {
            out.push_str("  evidence:\n");
            for evidence in &unit.evidence {
                if let (Some(observed), Some(threshold)) =
                    (evidence.observed.number(), evidence.threshold.number())
                {
                    out.push_str(&format!(
                        "    {} {} {} {}\n",
                        evidence.rule,
                        observed,
                        evidence.comparison.label(),
                        threshold
                    ));
                }
            }
        }
        out.push('\n');
    }

    if !assessment.diagnostics.is_empty() {
        out.push_str(&format!("failures ({}):\n", assessment.diagnostics.len()));
        for diagnostic in &assessment.diagnostics {
            out.push_str(&format!(
                "  {}:{}: {}: {}\n",
                diagnostic.path,
                diagnostic.line.unwrap_or(0),
                diagnostic.kind.label(),
                diagnostic.message
            ));
        }
    }
    if !assessment.unsupported.is_empty() {
        out.push_str(&format!(
            "unsupported ({}):\n",
            assessment.unsupported.len()
        ));
        for note in &assessment.unsupported {
            out.push_str(&format!(
                "  {}:{}: {}: {}\n",
                note.span.file,
                note.span.start_line,
                note.construct.label(),
                note.reason
            ));
        }
    }
    out
}

pub(crate) struct MethodIndex {
    targets: BTreeMap<MethodId, CallTarget>,
}

struct CallTarget {
    unit_id: UnitId,
    label: String,
    visibility: Visibility,
}

impl MethodIndex {
    pub(crate) fn from_assessment(assessment: &Assessment) -> Self {
        let mut targets = BTreeMap::new();
        for unit in &assessment.units {
            let label = match unit.kind {
                UnitKind::Module => "module".to_string(),
                UnitKind::Type(_) => unit.name.clone(),
            };
            for item in &unit.outline {
                if matches!(item.kind, OutlineKind::Method | OutlineKind::TraitMethod) {
                    targets.insert(
                        item.id.clone(),
                        CallTarget {
                            unit_id: unit.unit_id.clone(),
                            label: label.clone(),
                            visibility: item.visibility,
                        },
                    );
                }
            }
        }
        Self { targets }
    }

    fn get(&self, id: &str) -> Option<&CallTarget> {
        self.targets.get(id)
    }
}

pub(crate) fn render_outline(unit: &ClassifiedUnit, index: &MethodIndex) -> String {
    let mut out = String::from("  outline:\n");
    if unit.outline.is_empty() {
        out.push_str("    (none)\n");
        return out;
    }
    let local: BTreeMap<&str, &OutlineItem> = unit
        .outline
        .iter()
        .filter(|item| matches!(item.kind, OutlineKind::Method | OutlineKind::TraitMethod))
        .map(|item| (item.id.as_str(), item))
        .collect();
    for item in &unit.outline {
        let kind = outline_kind_label(item.kind);
        let visibility = visibility_label(item.visibility);
        match item.kind {
            OutlineKind::Type => {
                out.push_str(&format!("    {kind} {} [{}]\n", item.name, visibility));
            }
            OutlineKind::Method | OutlineKind::TraitMethod => {
                out.push_str(&format!("    {kind} [{}] {}\n", visibility, item.signature));
                let mut stack = vec![item.id.clone()];
                render_calls(&mut out, item, unit, index, &local, 3, &mut stack);
            }
        }
    }
    out
}

fn render_calls(
    out: &mut String,
    item: &OutlineItem,
    unit: &ClassifiedUnit,
    index: &MethodIndex,
    local: &BTreeMap<&str, &OutlineItem>,
    depth: usize,
    stack: &mut Vec<MethodId>,
) {
    let indent = "  ".repeat(depth);
    let mut seen: Vec<&str> = Vec::new();
    for call in &item.calls {
        let Some(target_id) = call.target.as_deref() else {
            if call.resolution == Resolution::Ambiguous {
                let key = call.callee.as_str();
                if seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                out.push_str(&format!("{indent}-> [ambiguous] {}\n", call.callee));
            }
            continue;
        };
        if seen.contains(&target_id) {
            continue;
        }
        seen.push(target_id);
        let Some(target) = index.get(target_id) else {
            continue;
        };
        let tag = format!("{}, {}", visibility_label(target.visibility), target.label);
        out.push_str(&format!("{indent}-> [{tag}] {}\n", call.callee));
        if target.unit_id != unit.unit_id {
            continue;
        }
        let Some(target_item) = local.get(target_id) else {
            continue;
        };
        if stack.iter().any(|id| id.as_str() == target_id) {
            out.push_str(&format!("{indent}  [cycle]\n"));
            continue;
        }
        stack.push(target_id.to_string());
        render_calls(out, target_item, unit, index, local, depth + 1, stack);
        stack.pop();
    }
}

fn outline_kind_label(kind: OutlineKind) -> &'static str {
    match kind {
        OutlineKind::Type => "type",
        OutlineKind::Method => "method",
        OutlineKind::TraitMethod => "trait method",
    }
}

pub fn render_json(assessment: &Assessment, scope: &SourceScope) -> String {
    let document = json!({
        "report": "outline",
        "schema_version": SCHEMA_VERSION,
        "rules_version": RULES_VERSION,
        "scope": {
            "languages": scope.languages.iter().map(|language| language.label()).collect::<Vec<_>>(),
            "roots": scope.roots.iter().map(|root| root.to_string_lossy().replace('\\', "/")).collect::<Vec<_>>(),
            "production_only": true,
        },
        "capabilities": {
            "languages": assessment.capabilities.languages.iter().map(|language| language.label()).collect::<Vec<_>>(),
            "supported_constructs": assessment.capabilities.supported_constructs.iter().map(|construct| construct.label()).collect::<Vec<_>>(),
            "limitations": assessment.capabilities.limitations.iter().map(|limitation| limitation.label()).collect::<Vec<_>>(),
            "resolution_labels": assessment.capabilities.resolution_labels.iter().map(|resolution| resolution.label()).collect::<Vec<_>>(),
        },
        "summary": summary_json(assessment),
        "files": files_json(assessment),
        "units": units_json(assessment),
        "failures": failures_json(assessment),
        "unsupported": unsupported_json(assessment),
    });
    document.to_string()
}

fn files_json(assessment: &Assessment) -> Vec<Value> {
    assessment.files.iter().map(file_json).collect()
}

fn file_json(file: &AnalyzedFile) -> Value {
    json!({
        "path": file.path.as_str(),
        "language": file.language.label(),
        "status": match file.status {
            super::model::FileStatus::Analyzed => "analyzed",
            super::model::FileStatus::Failed => "failed",
        },
        "physical_lines": file.physical_lines,
        "code_lines": file.lines.iter().filter(|line| **line == LineKind::Code).count(),
        "units": file.unit_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
    })
}

fn units_json(assessment: &Assessment) -> Vec<Value> {
    assessment.units.iter().map(unit_json).collect()
}

fn unit_json(unit: &ClassifiedUnit) -> Value {
    json!({
        "id": unit.unit_id.to_string(),
        "language": unit.language.label(),
        "kind": match unit.kind {
            UnitKind::Type(_) => "type",
            UnitKind::Module => "module",
        },
        "type_kind": match unit.kind {
            UnitKind::Type(kind) => Some(type_kind_label(kind)),
            UnitKind::Module => None,
        },
        "name": unit.name,
        "declaration_file": unit.path.as_str(),
        "files": unit.files.iter().map(|file| file.as_str()).collect::<Vec<_>>(),
        "classification": unit.classification.label(),
        "metrics": {
            "physical_lines": unit.metrics.physical_lines,
            "code_lines": unit.metrics.code_lines,
            "methods": unit.metrics.methods,
            "public_methods": unit.metrics.public_methods,
            "fields": unit.metrics.fields,
            "declared_types": unit.metrics.declared_types,
            "public_surface": unit.metrics.public_surface,
            "lcom4": unit.metrics.lcom4,
            "tcc_permille": unit.metrics.tcc_permille,
            "lcc_permille": unit.metrics.lcc_permille,
            "max_method_code_lines": unit.metrics.max_method_code_lines,
            "max_method_complexity": unit.metrics.max_method_complexity,
            "files": unit.metrics.files,
        },
        "evidence": unit.evidence.iter().map(evidence_json).collect::<Vec<_>>(),
        "methods": methods_json(unit),
        "outline": outline_json(unit),
    })
}

fn methods_json(unit: &ClassifiedUnit) -> Vec<Value> {
    unit.method_names
        .iter()
        .zip(unit.methods.iter())
        .map(|(name, method)| {
            json!({
                "name": name,
                "visibility": visibility_label(method.visibility),
                "code_lines": method.code_lines,
                "complexity": method.complexity,
                "nesting": method.nesting,
                "finding": method.finding.map(|finding| finding.label()),
            })
        })
        .collect()
}

fn outline_json(unit: &ClassifiedUnit) -> Vec<Value> {
    unit.outline
        .iter()
        .map(|item| {
            json!({
                "kind": match item.kind {
                    OutlineKind::Type => "type",
                    OutlineKind::Method => "method",
                    OutlineKind::TraitMethod => "trait_method",
                },
                "name": item.name,
                "visibility": visibility_label(item.visibility),
                "signature": item.signature,
                "order": item.order,
                "calls": item.calls.iter().map(|call| json!({
                    "callee": call.callee,
                    "resolution": call.resolution.label(),
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn failures_json(assessment: &Assessment) -> Vec<Value> {
    assessment
        .diagnostics
        .iter()
        .map(|diagnostic| {
            json!({
                "kind": diagnostic.kind.label(),
                "path": diagnostic.path.as_str(),
                "line": diagnostic.line,
                "message": diagnostic.message,
            })
        })
        .collect()
}

fn unsupported_json(assessment: &Assessment) -> Vec<Value> {
    assessment
        .unsupported
        .iter()
        .map(|note| {
            json!({
                "kind": note.construct.label(),
                "path": note.span.file.as_str(),
                "line": note.span.start_line,
                "message": note.reason,
            })
        })
        .collect()
}

fn summary_json(assessment: &Assessment) -> Value {
    json!({
        "files_scanned": assessment.files.len(),
        "files_analyzed": assessment.files.iter().filter(|file: &&AnalyzedFile| matches!(file.status, super::model::FileStatus::Analyzed)).count(),
        "files_failed": assessment.files.iter().filter(|file: &&AnalyzedFile| matches!(file.status, super::model::FileStatus::Failed)).count(),
        "units": assessment.units.len(),
        "units_classified": assessment.units.len(),
        "units_unsupported": assessment.unsupported.len(),
        "hints": assessment.units.iter().filter(|unit| unit.classification != Classification::WithinLimits).count(),
        "unsupported": assessment.unsupported.len(),
    })
}

fn evidence_json(evidence: &super::classify::Evidence) -> Value {
    json!({
        "rule": evidence.rule,
        "observed": metric_json(evidence.observed),
        "threshold": metric_json(evidence.threshold),
        "comparison": evidence.comparison.label(),
        "satisfied": evidence.satisfied,
    })
}

fn metric_json(value: MetricValue) -> Value {
    match value {
        MetricValue::Count(value) => json!(value),
        MetricValue::Permille(value) => json!(value),
        MetricValue::Undefined => Value::Null,
    }
}

fn languages_label(assessment: &Assessment) -> String {
    let mut languages = std::collections::BTreeSet::new();
    for file in &assessment.files {
        languages.insert(file.language.label());
    }
    if languages.is_empty() {
        "no sources".to_string()
    } else {
        languages.into_iter().collect::<Vec<_>>().join("+")
    }
}

fn kind_label(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Type(_) => "type",
        UnitKind::Module => "module",
    }
}

fn type_kind_label(kind: TypeKind) -> &'static str {
    kind.label()
}

fn visibility_label(visibility: Visibility) -> &'static str {
    match visibility {
        Visibility::Public => "public",
        Visibility::Crate => "crate",
        Visibility::Restricted => "restricted",
        Visibility::Private => "private",
    }
}

#[allow(dead_code)]
fn _comparison(_comparison: Comparison) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outline::classify::Classification;
    use crate::outline::metrics::UnitMetrics;
    use crate::outline::model::{
        AnalyzerCapabilities, Call, FileStatus, Language, OutlineItem, RepoPath, Resolution, UnitId,
    };

    fn call(callee: &str, resolution: Resolution, target: Option<&str>) -> Call {
        Call {
            order: 0,
            callee: callee.to_string(),
            resolution,
            target: target.map(ToString::to_string),
        }
    }

    #[test]
    fn the_human_report_leads_with_the_outline() {
        let path = RepoPath::new("src/engine.rs");
        let unit_id = UnitId("type:src/engine.rs::Engine".to_string());
        let unit = ClassifiedUnit {
            unit_id: unit_id.clone(),
            language: Language::Rust,
            kind: UnitKind::Type(TypeKind::Struct),
            name: "Engine".to_string(),
            path: path.clone(),
            files: vec![path.clone()],
            classification: Classification::GodType,
            metrics: UnitMetrics {
                physical_lines: 700,
                code_lines: 620,
                methods: 1,
                public_methods: 1,
                fields: 1,
                declared_types: 1,
                public_surface: 2,
                lcom4: 1,
                tcc_permille: Some(1000),
                lcc_permille: Some(1000),
                max_method_code_lines: 20,
                max_method_complexity: 3,
                files: 1,
            },
            method_findings: vec![],
            method_names: vec!["build".to_string()],
            methods: vec![],
            outline: vec![OutlineItem {
                id: "type:src/engine.rs::Engine::build".to_string(),
                kind: OutlineKind::Method,
                name: "build".to_string(),
                visibility: Visibility::Public,
                signature: "fn build(&self) -> Engine".to_string(),
                order: 2,
                calls: vec![
                    call("new", Resolution::External, None),
                    call(
                        "build",
                        Resolution::SameFile,
                        Some("type:src/engine.rs::Engine::build"),
                    ),
                ],
            }],
            evidence: vec![],
        };
        let assessment = Assessment {
            capabilities: AnalyzerCapabilities {
                languages: vec![Language::Rust],
                supported_constructs: vec![],
                limitations: vec![],
                resolution_labels: vec![],
            },
            files: vec![AnalyzedFile {
                language: Language::Rust,
                path,
                physical_lines: 700,
                lines: vec![],
                status: FileStatus::Analyzed,
                unit_ids: vec![unit_id],
            }],
            units: vec![unit],
            diagnostics: vec![],
            unsupported: vec![],
        };

        let text = render_human(&assessment);
        let outline = text.find("outline:").expect("outline section");
        let metrics = text.find("metrics (hint):").expect("metric hints");
        assert!(outline < metrics, "outline must come before metrics");
        assert!(text.contains("method [public] fn build(&self) -> Engine"));
        assert!(
            !text.contains("field data"),
            "data is not part of the outline"
        );
        assert!(text.contains("-> [public, Engine] build"));
        assert!(text.contains("[cycle]"));
        assert!(
            !text.contains("-> new"),
            "calls outside the unit are not part of its outline"
        );
    }

    fn empty_metrics() -> UnitMetrics {
        UnitMetrics {
            physical_lines: 10,
            code_lines: 10,
            methods: 1,
            public_methods: 1,
            fields: 0,
            declared_types: 0,
            public_surface: 1,
            lcom4: 0,
            tcc_permille: None,
            lcc_permille: None,
            max_method_code_lines: 1,
            max_method_complexity: 1,
            files: 1,
        }
    }

    fn item(id: &str, name: &str, visibility: Visibility, calls: Vec<Call>) -> OutlineItem {
        OutlineItem {
            id: id.to_string(),
            kind: OutlineKind::Method,
            name: name.to_string(),
            visibility,
            signature: format!("fn {name}(&self)"),
            order: 0,
            calls,
        }
    }

    fn unit(
        unit_id: &str,
        kind: UnitKind,
        name: &str,
        path: &RepoPath,
        outline: Vec<OutlineItem>,
    ) -> ClassifiedUnit {
        ClassifiedUnit {
            unit_id: UnitId(unit_id.to_string()),
            language: Language::Rust,
            kind,
            name: name.to_string(),
            path: path.clone(),
            files: vec![path.clone()],
            classification: Classification::WithinLimits,
            metrics: empty_metrics(),
            method_findings: vec![],
            method_names: outline.iter().map(|item| item.name.clone()).collect(),
            methods: vec![],
            outline,
            evidence: vec![],
        }
    }

    #[test]
    fn the_report_renders_one_call_tree_per_component() {
        let path = RepoPath::new("src/state.rs");
        let module_id = "module:src/state.rs";
        let change_id = "type:src/state.rs::ChangeState";
        let tasks_id = "type:src/state.rs::TasksProgress";
        let module = unit(
            module_id,
            UnitKind::Module,
            "src/state.rs",
            &path,
            vec![
                item(
                    "state::artifact_status",
                    "artifact_status",
                    Visibility::Private,
                    vec![
                        call(
                            "is_skipped",
                            Resolution::SameFile,
                            Some("state::is_skipped"),
                        ),
                        call(
                            "is_complete",
                            Resolution::SameFile,
                            Some("type:src/state.rs::TasksProgress::is_complete"),
                        ),
                    ],
                ),
                item(
                    "state::is_skipped",
                    "is_skipped",
                    Visibility::Public,
                    vec![],
                ),
            ],
        );
        let change = unit(
            change_id,
            UnitKind::Type(TypeKind::Struct),
            "ChangeState",
            &path,
            vec![item(
                "state::ChangeState::from_manifest",
                "from_manifest",
                Visibility::Public,
                vec![call(
                    "artifact_status",
                    Resolution::SameFile,
                    Some("state::artifact_status"),
                )],
            )],
        );
        let tasks = unit(
            tasks_id,
            UnitKind::Type(TypeKind::Struct),
            "TasksProgress",
            &path,
            vec![item(
                "type:src/state.rs::TasksProgress::is_complete",
                "is_complete",
                Visibility::Public,
                vec![],
            )],
        );
        let assessment = Assessment {
            capabilities: AnalyzerCapabilities {
                languages: vec![Language::Rust],
                supported_constructs: vec![],
                limitations: vec![],
                resolution_labels: vec![],
            },
            files: vec![AnalyzedFile {
                language: Language::Rust,
                path: path.clone(),
                physical_lines: 10,
                lines: vec![],
                status: FileStatus::Analyzed,
                unit_ids: vec![
                    UnitId(module_id.to_string()),
                    UnitId(change_id.to_string()),
                    UnitId(tasks_id.to_string()),
                ],
            }],
            units: vec![module, change, tasks],
            diagnostics: vec![],
            unsupported: vec![],
        };

        let text = render_human(&assessment);
        let module_section = text
            .split(module_id)
            .nth(1)
            .expect("module component")
            .split(change_id)
            .next()
            .expect("module section");
        assert!(module_section.contains("-> [public, module] is_skipped"));
        assert!(module_section.contains("-> [public, TasksProgress] is_complete"));
        assert!(
            !module_section.contains("-> [public, module] is_complete"),
            "a cross-component call keeps its own component label"
        );

        let change_section = text
            .split(change_id)
            .nth(1)
            .expect("change component")
            .split(tasks_id)
            .next()
            .expect("change section");
        assert!(change_section.contains("-> [private, module] artifact_status"));
        assert!(
            !change_section.contains("-> [public, module] is_skipped"),
            "a cross-component call is a leaf; its callee's calls stay in its own component"
        );
    }
}
