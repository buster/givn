//! Human and JSON rendering from one classified model.

use super::classify::{
    Action, Assessment, Classification, ClassifiedUnit, Comparison, MetricValue,
};
use super::model::{
    AnalyzedFile, LineKind, OutlineKind, SourceScope, TypeKind, UnitKind, Visibility,
};
use serde_json::{Value, json};

pub const SCHEMA_VERSION: u32 = 1;
pub const RULES_VERSION: u32 = 1;

pub fn render_human(assessment: &Assessment) -> String {
    let findings = assessment
        .units
        .iter()
        .filter(|unit| unit.classification != Classification::WithinLimits)
        .count();
    let mut out = format!(
        "readability report — rust, {} files, {} units, {} findings\n",
        assessment.files.len(),
        assessment.units.len(),
        findings
    );
    out.push_str("advisory: this report does not block review or archive\n\n");

    let mut units: Vec<&ClassifiedUnit> = assessment.units.iter().collect();
    units.sort_by(|left, right| {
        left.action
            .severity()
            .cmp(&right.action.severity())
            .then_with(|| {
                left.classification
                    .label()
                    .cmp(right.classification.label())
            })
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.unit_id.cmp(&right.unit_id))
    });

    for unit in units {
        out.push_str(&format!(
            "{:<11} {:<7} {:<36} {}\n",
            unit.action.label().to_uppercase(),
            kind_label(unit.kind),
            format!("{}::{}", unit.path, unit.name),
            unit.classification.label()
        ));
        out.push_str(&format!(
            "            code lines {}; methods {}; public methods {}; fields {}; LCOM4 {}; TCC {}\n",
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
        for evidence in &unit.evidence {
            if let (Some(observed), Some(threshold)) =
                (evidence.observed.number(), evidence.threshold.number())
            {
                out.push_str(&format!(
                    "            {} {} {} {}\n",
                    evidence.rule,
                    observed,
                    evidence.comparison.label(),
                    threshold
                ));
            }
        }
        out.push_str(&format!(
            "            recommendation: {}\n\n",
            unit.recommendation
        ));
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

pub fn render_json(assessment: &Assessment, scope: &SourceScope) -> String {
    let files: Vec<Value> = assessment
        .files
        .iter()
        .map(|file| {
            json!({
                "path": file.path.as_str(),
                "status": match file.status {
                    super::model::FileStatus::Analyzed => "analyzed",
                    super::model::FileStatus::Failed => "failed",
                },
                "physical_lines": file.physical_lines,
                "code_lines": file.lines.iter().filter(|line| **line == LineKind::Code).count(),
                "units": file.unit_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            })
        })
        .collect();

    let units: Vec<Value> = assessment
        .units
        .iter()
        .map(|unit| {
            json!({
                "id": unit.unit_id.to_string(),
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
                "action": unit.action.label(),
                "recommendation": unit.recommendation,
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
                "methods": unit.method_names.iter().zip(unit.methods.iter()).map(|(name, method)| json!({
                    "name": name,
                    "visibility": visibility_label(method.visibility),
                    "code_lines": method.code_lines,
                    "complexity": method.complexity,
                    "nesting": method.nesting,
                    "finding": method.finding.map(|finding| finding.label()),
                })).collect::<Vec<_>>(),
                "outline": unit.outline.iter().map(|item| json!({
                    "kind": match item.kind {
                        OutlineKind::Type => "type",
                        OutlineKind::Field => "field",
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
                })).collect::<Vec<_>>(),
            })
        })
        .collect();

    let failures: Vec<Value> = assessment
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
        .collect();

    let unsupported: Vec<Value> = assessment
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
        .collect();

    let document = json!({
        "report": "readability",
        "schema_version": SCHEMA_VERSION,
        "rules_version": RULES_VERSION,
        "scope": {
            "language": "rust",
            "roots": scope.roots.iter().map(|root| root.to_string_lossy().replace('\\', "/")).collect::<Vec<_>>(),
            "production_only": true,
        },
        "capabilities": {
            "language": "rust",
            "supported_constructs": assessment.capabilities.supported_constructs.iter().map(|construct| construct.label()).collect::<Vec<_>>(),
            "limitations": assessment.capabilities.limitations.iter().map(|limitation| limitation.label()).collect::<Vec<_>>(),
            "resolution_labels": assessment.capabilities.resolution_labels.iter().map(|resolution| resolution.label()).collect::<Vec<_>>(),
        },
        "summary": {
            "files_scanned": assessment.files.len(),
            "files_analyzed": assessment.files.iter().filter(|file: &&AnalyzedFile| matches!(file.status, super::model::FileStatus::Analyzed)).count(),
            "files_failed": assessment.files.iter().filter(|file: &&AnalyzedFile| matches!(file.status, super::model::FileStatus::Failed)).count(),
            "units": assessment.units.len(),
            "units_classified": assessment.units.iter().filter(|unit| unit.classification != Classification::WithinLimits).count() + assessment.units.iter().filter(|unit| unit.classification == Classification::WithinLimits).count(),
            "units_unsupported": assessment.unsupported.len(),
            "findings": assessment.units.iter().filter(|unit| unit.classification != Classification::WithinLimits).count(),
            "unsupported": assessment.unsupported.len(),
        },
        "files": files,
        "units": units,
        "failures": failures,
        "unsupported": unsupported,
    });
    document.to_string()
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
fn _action_label(action: Action) -> &'static str {
    action.label()
}

#[allow(dead_code)]
fn _comparison(_comparison: Comparison) {}
