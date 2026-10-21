//! Shared metric computation above the analyzer port.

use super::model::{
    Analysis, AnalyzedFile, Flow, Method, Resolution, SourceUnit, UnitId, UnitKind, Visibility,
};
use super::scan;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitMetrics {
    pub physical_lines: u32,
    pub code_lines: u32,
    pub methods: u32,
    pub public_methods: u32,
    pub fields: u32,
    pub declared_types: u32,
    pub public_surface: u32,
    pub lcom4: u32,
    pub tcc_permille: Option<u32>,
    pub lcc_permille: Option<u32>,
    pub max_method_code_lines: u32,
    pub max_method_complexity: u32,
    pub files: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodMetrics {
    pub code_lines: u32,
    pub complexity: u32,
    pub nesting: u32,
    pub visibility: Visibility,
    pub finding: Option<MethodFinding>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodFinding {
    MethodLength,
    Complexity,
}

impl MethodFinding {
    pub fn label(self) -> &'static str {
        match self {
            Self::MethodLength => "method-length",
            Self::Complexity => "complexity",
        }
    }
}

pub fn compute(analysis: &Analysis) -> HashMap<UnitId, UnitMetrics> {
    analysis
        .units
        .iter()
        .map(|unit| (unit.id.clone(), compute_unit(unit, analysis)))
        .collect()
}

pub fn compute_methods(unit: &SourceUnit, analysis: &Analysis) -> Vec<MethodMetrics> {
    unit.methods
        .iter()
        .map(|method| method_metrics(method, analysis))
        .collect()
}

pub fn compute_unit(unit: &SourceUnit, analysis: &Analysis) -> UnitMetrics {
    let code_lines = unit
        .files
        .iter()
        .map(|file| {
            analysis
                .files
                .iter()
                .find(|analyzed| &analyzed.path == file)
                .map(|analyzed| {
                    scan::code_lines_excluding(
                        &analyzed.lines,
                        &unit.spans,
                        &analyzed.excluded_spans,
                        file,
                    )
                })
                .unwrap_or(0)
        })
        .sum::<u32>();
    let physical_lines = unit
        .files
        .iter()
        .map(|file| {
            analysis
                .files
                .iter()
                .find(|analyzed| &analyzed.path == file)
                .map(|analyzed| {
                    scan::span_lines_excluding(
                        analyzed.lines.len(),
                        &unit.spans,
                        &analyzed.excluded_spans,
                        file,
                    )
                })
                .unwrap_or(0)
        })
        .sum::<u32>();

    let method_metrics: Vec<MethodMetrics> = unit
        .methods
        .iter()
        .map(|method| method_metrics(method, analysis))
        .collect();
    let max_method_code_lines = method_metrics
        .iter()
        .map(|method| method.code_lines)
        .max()
        .unwrap_or(0);
    let max_method_complexity = method_metrics
        .iter()
        .map(|method| method.complexity)
        .max()
        .unwrap_or(0);

    let (lcom4, tcc_permille, lcc_permille) = cohesion(unit);
    let public_methods = unit
        .methods
        .iter()
        .filter(|method| method.visibility == Visibility::Public)
        .count() as u32;
    let public_fields = unit
        .fields
        .iter()
        .filter(|field| field.visibility == Visibility::Public)
        .count() as u32;
    let declared_types = match unit.kind {
        UnitKind::Type(_) => 1,
        UnitKind::Module => analysis
            .units
            .iter()
            .filter(|candidate| {
                matches!(candidate.kind, UnitKind::Type(_))
                    && candidate.declaration_file == unit.declaration_file
            })
            .count() as u32,
    };

    UnitMetrics {
        physical_lines,
        code_lines,
        methods: unit.methods.len() as u32,
        public_methods,
        fields: unit.fields.len() as u32,
        declared_types,
        public_surface: public_methods + public_fields,
        lcom4,
        tcc_permille,
        lcc_permille,
        max_method_code_lines,
        max_method_complexity,
        files: unit.files.len() as u32,
    }
}

fn method_metrics(method: &Method, analysis: &Analysis) -> MethodMetrics {
    let code_lines = analysis
        .files
        .iter()
        .find(|analyzed: &&AnalyzedFile| analyzed.path == method.span.file)
        .map(|analyzed| {
            scan::code_lines_excluding(
                &analyzed.lines,
                std::slice::from_ref(&method.span),
                &analyzed.excluded_spans,
                &method.span.file,
            )
        })
        .unwrap_or(0);
    let complexity = flow_complexity(&method.flow);
    let nesting = flow_nesting(&method.flow);
    let finding = if code_lines > 100 {
        Some(MethodFinding::MethodLength)
    } else if complexity > 50 {
        Some(MethodFinding::Complexity)
    } else {
        None
    };
    MethodMetrics {
        code_lines,
        complexity,
        nesting,
        visibility: method.visibility,
        finding,
    }
}

/// LCOM4 over production methods and fields, plus TCC/LCC in permille.
fn cohesion(unit: &SourceUnit) -> (u32, Option<u32>, Option<u32>) {
    if unit.methods.is_empty() {
        return (0, None, None);
    }
    if unit.methods.len() == 1 {
        let tcc = if unit.methods[0].field_accesses.is_empty() {
            None
        } else {
            Some(1000)
        };
        return (1, tcc, tcc);
    }

    let mut nodes = unit.methods.len() + unit.fields.len();
    let mut parent: Vec<usize> = (0..nodes).collect();
    let field_index: HashMap<&str, usize> = unit
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| (field.id.as_str(), unit.methods.len() + index))
        .collect();

    let mut method_fields: Vec<HashSet<usize>> = Vec::with_capacity(unit.methods.len());
    for (method_index, method) in unit.methods.iter().enumerate() {
        let mut set = HashSet::new();
        for field_id in &method.field_accesses {
            if let Some(&index) = field_index.get(field_id.as_str()) {
                set.insert(index);
                union(&mut parent, method_index, index);
            }
        }
        method_fields.push(set);
    }
    for (index, method) in unit.methods.iter().enumerate() {
        for call in &method.calls {
            if !matches!(
                call.resolution,
                Resolution::SameFile | Resolution::ProjectOtherFile
            ) {
                continue;
            }
            if let Some(target_index) = call
                .target
                .as_ref()
                .and_then(|target| unit.methods.iter().position(|m| &m.id == target))
            {
                union(&mut parent, index, target_index);
            }
        }
    }
    for index in 0..nodes {
        parent[index] = find(&mut parent, index);
    }
    nodes = parent.iter().copied().collect::<HashSet<_>>().len();
    let lcom4 = if nodes == 0 { 0 } else { nodes as u32 };

    let with_fields: Vec<usize> = method_fields
        .iter()
        .enumerate()
        .filter(|(_, fields)| !fields.is_empty())
        .map(|(index, _)| index)
        .collect();
    let pairs = with_fields.len() * with_fields.len().saturating_sub(1) / 2;
    if pairs == 0 {
        return (lcom4, None, None);
    }
    let mut direct = 0usize;
    for (left, right) in with_fields.iter().enumerate() {
        for other in with_fields.iter().skip(left + 1) {
            if !method_fields[*right].is_disjoint(&method_fields[*other]) {
                direct += 1;
            }
        }
    }
    let permille = (direct * 1000 / pairs) as u32;
    (lcom4, Some(permille), Some(permille))
}

fn find(parent: &mut [usize], node: usize) -> usize {
    if parent[node] != node {
        parent[node] = find(parent, parent[node]);
    }
    parent[node]
}

fn union(parent: &mut [usize], left: usize, right: usize) {
    let left_root = find(parent, left);
    let right_root = find(parent, right);
    if left_root != right_root {
        parent[right_root] = left_root;
    }
}

pub fn flow_complexity(flow: &Flow) -> u32 {
    1 + flow_decision_points(flow)
}

fn flow_decision_points(flow: &Flow) -> u32 {
    match flow {
        Flow::Block(nodes) => nodes.iter().map(flow_decision_points).sum(),
        Flow::Branch(inner) => 1 + flow_decision_points(inner),
        Flow::Loop(inner) => 1 + flow_decision_points(inner),
        Flow::Match { arms, bodies } => {
            arms.saturating_sub(1) + bodies.iter().map(flow_decision_points).sum::<u32>()
        }
        Flow::Try(inner) => 1 + flow_decision_points(inner),
        Flow::ShortCircuit(left, right) => {
            1 + flow_decision_points(left) + flow_decision_points(right)
        }
        Flow::Return(inner) => inner.as_deref().map(flow_decision_points).unwrap_or(0),
        Flow::Call(_) => 0,
    }
}

pub fn flow_nesting(flow: &Flow) -> u32 {
    match flow {
        Flow::Block(nodes) => nodes.iter().map(flow_nesting).max().unwrap_or(0),
        Flow::Branch(inner) | Flow::Loop(inner) | Flow::Try(inner) => 1 + flow_nesting(inner),
        Flow::Match { bodies, .. } => 1 + bodies.iter().map(flow_nesting).max().unwrap_or(0),
        Flow::ShortCircuit(left, right) => 1 + flow_nesting(left).max(flow_nesting(right)),
        Flow::Return(inner) => inner.as_deref().map(flow_nesting).unwrap_or(0),
        Flow::Call(_) => 0,
    }
}
