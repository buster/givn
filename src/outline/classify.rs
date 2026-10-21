#![allow(dead_code)]

//! Shared classification and evidence.

use super::metrics::{self, MethodFinding, MethodMetrics, UnitMetrics};
use super::model::{
    Analysis, AnalysisDiagnostic, AnalyzedFile, AnalyzerCapabilities, Language, OutlineItem,
    RepoPath, SourceUnit, TypeKind, UnitId, UnitKind, UnsupportedNote,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    GodType,
    Interface,
    MultiTypeModule,
    Module,
    DataOnly,
    CohesiveLargeType,
    MethodLength,
    WithinLimits,
}

impl Classification {
    pub fn label(self) -> &'static str {
        match self {
            Self::GodType => "god-type",
            Self::Interface => "interface",
            Self::MultiTypeModule => "multi-type-module",
            Self::Module => "module",
            Self::DataOnly => "data-only",
            Self::CohesiveLargeType => "cohesive-large-type",
            Self::MethodLength => "method-length",
            Self::WithinLimits => "within-limits",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    AtLeast,
    AtMost,
    Equal,
}

impl Comparison {
    pub fn label(self) -> &'static str {
        match self {
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::AtLeast => ">=",
            Self::AtMost => "<=",
            Self::Equal => "=",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricValue {
    Count(u64),
    Permille(u32),
    Undefined,
}

impl MetricValue {
    pub fn number(self) -> Option<u64> {
        match self {
            Self::Count(value) => Some(value),
            Self::Permille(value) => Some(value as u64),
            Self::Undefined => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evidence {
    pub rule: &'static str,
    pub observed: MetricValue,
    pub threshold: MetricValue,
    pub comparison: Comparison,
    pub satisfied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassifiedUnit {
    pub unit_id: UnitId,
    pub language: Language,
    pub kind: UnitKind,
    pub name: String,
    pub path: RepoPath,
    pub files: Vec<RepoPath>,
    pub classification: Classification,
    pub metrics: UnitMetrics,
    pub method_findings: Vec<(String, MethodFinding)>,
    pub method_names: Vec<String>,
    pub methods: Vec<MethodMetrics>,
    pub outline: Vec<OutlineItem>,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assessment {
    pub capabilities: AnalyzerCapabilities,
    pub files: Vec<AnalyzedFile>,
    pub units: Vec<ClassifiedUnit>,
    pub diagnostics: Vec<AnalysisDiagnostic>,
    pub unsupported: Vec<UnsupportedNote>,
}

/// Run the shared metrics and classification over a neutral analysis.
pub fn assess(analysis: &Analysis, capabilities: AnalyzerCapabilities) -> Assessment {
    let metrics = metrics::compute(analysis);
    let units = analysis
        .units
        .iter()
        .filter(|unit| !unit.is_test())
        .map(|unit| {
            let unit_metrics = metrics
                .get(&unit.id)
                .cloned()
                .unwrap_or_else(|| metrics::compute_unit(unit, analysis));
            classify_unit(unit, &unit_metrics, analysis)
        })
        .collect();
    Assessment {
        capabilities,
        files: analysis.files.clone(),
        units,
        diagnostics: analysis.diagnostics.clone(),
        unsupported: analysis
            .units
            .iter()
            .flat_map(|unit| unit.unsupported.clone())
            .collect(),
    }
}

fn classify_unit(unit: &SourceUnit, metrics: &UnitMetrics, analysis: &Analysis) -> ClassifiedUnit {
    let method_metrics = metrics::compute_methods(unit, analysis);
    let method_findings: Vec<(String, MethodFinding)> = unit
        .methods
        .iter()
        .zip(method_metrics.iter())
        .filter_map(|(method, method_metrics)| {
            method_metrics
                .finding
                .map(|finding| (method.name.clone(), finding))
        })
        .collect();

    let (classification, evidence) = decide(unit, metrics);
    ClassifiedUnit {
        unit_id: unit.id.clone(),
        language: unit.language,
        kind: unit.kind,
        name: unit.name.clone(),
        path: unit.declaration_file.clone(),
        files: unit.files.clone(),
        classification,
        metrics: metrics.clone(),
        method_findings,
        method_names: unit
            .methods
            .iter()
            .map(|method| method.name.clone())
            .collect(),
        methods: method_metrics,
        outline: unit.outline.clone(),
        evidence,
    }
}

fn decide(unit: &SourceUnit, metrics: &UnitMetrics) -> (Classification, Vec<Evidence>) {
    let code_lines = MetricValue::Count(metrics.code_lines as u64);
    let over_type_limit = metrics.code_lines > 500;

    if let UnitKind::Type(type_kind) = unit.kind
        && let Some(decision) = classify_type(type_kind, metrics, code_lines, over_type_limit)
    {
        return decision;
    }

    if unit.kind == UnitKind::Module
        && let Some(decision) = classify_module(metrics, code_lines)
    {
        return decision;
    }

    if over_method_limit(metrics) {
        let mut items = Vec::new();
        if metrics.max_method_code_lines > 100 {
            items.push(evidence(
                "method-code-lines",
                MetricValue::Count(metrics.max_method_code_lines as u64),
                MetricValue::Count(100),
                Comparison::GreaterThan,
            ));
        }
        if metrics.max_method_complexity > 50 {
            items.push(evidence(
                "method-complexity",
                MetricValue::Count(metrics.max_method_complexity as u64),
                MetricValue::Count(50),
                Comparison::GreaterThan,
            ));
        }
        return (Classification::MethodLength, items);
    }

    (Classification::WithinLimits, Vec::new())
}

fn classify_type(
    type_kind: TypeKind,
    metrics: &UnitMetrics,
    code_lines: MetricValue,
    over_type_limit: bool,
) -> Option<(Classification, Vec<Evidence>)> {
    let trait_over_limit = type_kind == TypeKind::Trait && over_type_limit;
    let many_public_methods = metrics.public_methods > 50;
    if trait_over_limit || many_public_methods {
        let mut items = Vec::new();
        if trait_over_limit {
            items.push(evidence(
                "type-kind-trait",
                MetricValue::Count(1),
                MetricValue::Count(1),
                Comparison::Equal,
            ));
            items.push(evidence(
                "type-code-lines",
                code_lines,
                MetricValue::Count(500),
                Comparison::GreaterThan,
            ));
        }
        if many_public_methods {
            items.push(evidence(
                "type-public-methods",
                MetricValue::Count(metrics.public_methods as u64),
                MetricValue::Count(50),
                Comparison::GreaterThan,
            ));
        }
        return Some((Classification::Interface, items));
    }
    if matches!(type_kind, TypeKind::Struct | TypeKind::Enum)
        && metrics.methods <= 1
        && over_type_limit
    {
        return Some((
            Classification::DataOnly,
            vec![
                evidence(
                    "type-code-lines",
                    code_lines,
                    MetricValue::Count(500),
                    Comparison::GreaterThan,
                ),
                evidence(
                    "type-method-count",
                    MetricValue::Count(metrics.methods as u64),
                    MetricValue::Count(1),
                    Comparison::AtMost,
                ),
            ],
        ));
    }
    let low_cohesion_lcom4 = metrics.lcom4 > 2;
    let low_cohesion_tcc = metrics.tcc_permille.is_some_and(|tcc| tcc < 500);
    if over_type_limit && (low_cohesion_lcom4 || low_cohesion_tcc) {
        let mut items = vec![evidence(
            "type-code-lines",
            code_lines,
            MetricValue::Count(500),
            Comparison::GreaterThan,
        )];
        if low_cohesion_lcom4 {
            items.push(evidence(
                "type-lcom4",
                MetricValue::Count(metrics.lcom4 as u64),
                MetricValue::Count(2),
                Comparison::GreaterThan,
            ));
        }
        if let Some(tcc) = metrics.tcc_permille.filter(|tcc| *tcc < 500) {
            items.push(evidence(
                "type-tcc",
                MetricValue::Permille(tcc),
                MetricValue::Permille(500),
                Comparison::LessThan,
            ));
        }
        return Some((Classification::GodType, items));
    }
    if over_type_limit {
        return Some((
            Classification::CohesiveLargeType,
            vec![evidence(
                "type-code-lines",
                code_lines,
                MetricValue::Count(500),
                Comparison::GreaterThan,
            )],
        ));
    }
    None
}

fn classify_module(
    metrics: &UnitMetrics,
    code_lines: MetricValue,
) -> Option<(Classification, Vec<Evidence>)> {
    if metrics.code_lines > 1000 {
        return Some((
            Classification::Module,
            vec![evidence(
                "module-code-lines",
                code_lines,
                MetricValue::Count(1000),
                Comparison::GreaterThan,
            )],
        ));
    }
    if (500..=1000).contains(&metrics.code_lines) && metrics.declared_types >= 2 {
        return Some((
            Classification::MultiTypeModule,
            vec![evidence(
                "module-declared-types",
                MetricValue::Count(metrics.declared_types as u64),
                MetricValue::Count(2),
                Comparison::AtLeast,
            )],
        ));
    }
    if (500..=1000).contains(&metrics.code_lines) {
        return Some((
            Classification::Module,
            vec![evidence(
                "module-code-lines",
                code_lines,
                MetricValue::Count(500),
                Comparison::AtLeast,
            )],
        ));
    }
    None
}

fn over_method_limit(metrics: &UnitMetrics) -> bool {
    metrics.max_method_code_lines > 100
        || metrics.max_method_complexity > 50
        || !metrics.method_findings().is_empty()
}

impl UnitMetrics {
    fn method_findings(&self) -> Vec<MethodFinding> {
        let mut findings = Vec::new();
        if self.max_method_code_lines > 100 {
            findings.push(MethodFinding::MethodLength);
        }
        if self.max_method_complexity > 50 {
            findings.push(MethodFinding::Complexity);
        }
        findings
    }
}

impl SourceUnit {
    fn is_test(&self) -> bool {
        matches!(self.origin, super::model::UnitOrigin::Test)
    }
}

fn evidence(
    rule: &'static str,
    observed: MetricValue,
    threshold: MetricValue,
    comparison: Comparison,
) -> Evidence {
    Evidence {
        rule,
        observed,
        threshold,
        comparison,
        satisfied: true,
    }
}
