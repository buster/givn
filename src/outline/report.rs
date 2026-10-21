//! Human and JSON rendering from one classified model.

use super::classify::{Assessment, Classification, ClassifiedUnit, Comparison, MetricValue};
use super::model::{
    AnalyzedFile, LineKind, OutlineKind, SourceScope, TypeKind, UnitKind, Visibility,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const SCHEMA_VERSION: u32 = 2;
pub const RULES_VERSION: u32 = 1;

pub fn render_human(assessment: &Assessment) -> String {
    if assessment.files.is_empty() {
        return "outline report — rust, 0 files, 0 units, 0 hints\nNo production sources found in the requested scope.\n"
            .to_string();
    }
    let findings = assessment
        .units
        .iter()
        .filter(|unit| unit.classification != Classification::WithinLimits)
        .count();
    let mut out = format!(
        "outline report — rust, {} files, {} units, {} hints\n",
        assessment.files.len(),
        assessment.units.len(),
        findings
    );
    out.push_str(
        "outline first: method declarations, signatures, and internal calls; metrics are hints\n",
    );
    out.push_str("advisory report: this report does not block review or archive\n\n");

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
        out.push_str(&render_outline(unit));
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

pub(crate) fn render_outline(unit: &ClassifiedUnit) -> String {
    let mut out = String::from("  outline:\n");
    if unit.outline.is_empty() {
        out.push_str("    (none)\n");
        return out;
    }
    let internal: BTreeSet<&str> = unit.method_names.iter().map(String::as_str).collect();
    for item in &unit.outline {
        let kind = outline_kind_label(item.kind);
        let visibility = visibility_label(item.visibility);
        match item.kind {
            OutlineKind::Type => {
                out.push_str(&format!("    {kind} {} [{}]\n", item.name, visibility));
            }
            OutlineKind::Method | OutlineKind::TraitMethod => {
                out.push_str(&format!("    {kind} [{}] {}\n", visibility, item.signature))
            }
        }
        let calls: Vec<&str> = item
            .calls
            .iter()
            .map(|call| call.callee.as_str())
            .filter(|callee| internal.contains(callee))
            .collect();
        if !calls.is_empty() {
            out.push_str(&format!("      calls: {}\n", calls.join(" -> ")));
        }
    }
    out
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
        "findings": assessment.units.iter().filter(|unit| unit.classification != Classification::WithinLimits).count(),
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

    fn call(callee: &str, resolution: Resolution) -> Call {
        Call {
            order: 0,
            callee: callee.to_string(),
            resolution,
            target: None,
        }
    }

    #[test]
    fn the_human_report_leads_with_the_outline() {
        let path = RepoPath::new("src/engine.rs");
        let unit_id = UnitId("type:src/engine.rs::Engine".to_string());
        let unit = ClassifiedUnit {
            unit_id: unit_id.clone(),
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
                kind: OutlineKind::Method,
                name: "build".to_string(),
                visibility: Visibility::Public,
                signature: "fn build(&self) -> Engine".to_string(),
                order: 2,
                calls: vec![
                    call("new", Resolution::External),
                    call("build", Resolution::SameFile),
                ],
            }],
            evidence: vec![],
        };
        let assessment = Assessment {
            capabilities: AnalyzerCapabilities {
                language: Language::Rust,
                supported_constructs: vec![],
                limitations: vec![],
                resolution_labels: vec![],
            },
            files: vec![AnalyzedFile {
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
        assert!(text.contains("calls: build"));
        assert!(
            !text.contains("calls: new"),
            "calls outside the unit are not part of its outline"
        );
    }
}
