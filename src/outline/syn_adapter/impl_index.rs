//! Syn-free cross-file impl attachment and trait-relation facts.

use super::super::model::{
    Analysis, AnalysisDiagnostic, DiagnosticKind, Method, OutlineItem, RepoPath, SourceSpan,
    TypeKind, TypeRelation, UnitId, UnitKind,
};
use std::collections::BTreeMap;

/// Impls whose type declaration was not in the same file, plus the trait
/// relation facts collected from every attached impl.
#[derive(Default)]
pub(super) struct ImplCollector {
    pub(super) pending: Vec<PendingImpl>,
    pub(super) records: Vec<ImplRecord>,
}

pub(super) struct PendingImpl {
    pub(super) file: RepoPath,
    pub(super) module_path: String,
    pub(super) target: String,
    pub(super) trait_name: Option<String>,
    pub(super) spans: Vec<SourceSpan>,
    pub(super) methods: Vec<Method>,
    pub(super) outline: Vec<OutlineItem>,
    pub(super) line: Option<u32>,
}

pub(super) struct ImplRecord {
    pub(super) implementor_id: UnitId,
    pub(super) trait_name: String,
    pub(super) file: RepoPath,
    pub(super) line: Option<u32>,
}

/// Attach cross-file impls to the unique type with the same name, then resolve
/// trait/implementor relations by type name.
///
/// A name declared more than once is ambiguous: the impl is never merged and
/// an `AmbiguousType` diagnostic is recorded. Same-file impls already attached
/// by `FileBuilder` are left untouched.
pub(super) fn attach_impls(analysis: &mut Analysis, collector: ImplCollector) {
    let ImplCollector {
        pending,
        mut records,
    } = collector;
    attach_pending_impls(analysis, pending, &mut records);
    resolve_trait_relations(analysis, records);
}

/// Merge every pending cross-file impl into its unique declaration and collect
/// the trait relation facts for the second phase.
fn attach_pending_impls(
    analysis: &mut Analysis,
    pending: Vec<PendingImpl>,
    records: &mut Vec<ImplRecord>,
) {
    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, unit) in analysis.units.iter().enumerate() {
        if matches!(unit.kind, UnitKind::Type(_)) {
            by_name.entry(unit.name.clone()).or_default().push(index);
        }
    }

    for pending in pending {
        let named = by_name.get(&pending.target).map(Vec::as_slice);
        let candidates: Vec<usize> = match named {
            Some([index]) => vec![*index],
            Some(indices) => indices
                .iter()
                .copied()
                .filter(|index| {
                    let declared = &analysis.units[*index].module_path;
                    pending.module_path == *declared
                        || pending.module_path.starts_with(&format!("{declared}::"))
                })
                .collect(),
            None => Vec::new(),
        };
        match candidates.as_slice() {
            [index] => {
                let unit = &mut analysis.units[*index];
                unit.spans.extend(pending.spans);
                unit.methods.extend(pending.methods);
                unit.methods.sort_by(|a, b| a.name.cmp(&b.name));
                if unit.files.iter().all(|file| file != &pending.file) {
                    unit.files.push(pending.file.clone());
                }
                unit.outline.extend(pending.outline);
                unit.outline.sort_by_key(|item| item.order);
                if let Some(trait_name) = pending.trait_name {
                    records.push(ImplRecord {
                        implementor_id: unit.id.clone(),
                        trait_name,
                        file: pending.file,
                        line: pending.line,
                    });
                }
            }
            _ if named.is_some() => {
                // A duplicate type name is resolved only when the impl's
                // module tree identifies exactly one declaration. Anything
                // else stays an explicit ambiguity and is never merged.
                analysis.diagnostics.push(AnalysisDiagnostic {
                    kind: DiagnosticKind::AmbiguousType,
                    path: pending.file,
                    line: pending.line,
                    message: format!(
                        "impl target '{}' matches {} declared types; not merged",
                        pending.target,
                        named.map(|indices| indices.len()).unwrap_or(0)
                    ),
                });
            }
            _ => {}
        }
    }
}

/// Resolve each recorded trait name to its unique trait and add the
/// implementor relation on both sides.
fn resolve_trait_relations(analysis: &mut Analysis, records: Vec<ImplRecord>) {
    let mut traits_by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, unit) in analysis.units.iter().enumerate() {
        if matches!(unit.kind, UnitKind::Type(TypeKind::Trait)) {
            traits_by_name
                .entry(unit.name.clone())
                .or_default()
                .push(index);
        }
    }

    for record in records {
        let matches = traits_by_name.get(&record.trait_name).map(Vec::as_slice);
        let Some([trait_index]) = matches else {
            if let Some(ambiguous) = matches.filter(|matches| matches.len() > 1) {
                analysis.diagnostics.push(AnalysisDiagnostic {
                    kind: DiagnosticKind::AmbiguousType,
                    path: record.file,
                    line: record.line,
                    message: format!(
                        "trait '{}' matches {} declared traits; implementors not expanded",
                        record.trait_name,
                        ambiguous.len()
                    ),
                });
            }
            continue;
        };
        let Some(implementor_index) = analysis
            .units
            .iter()
            .position(|unit| unit.id == record.implementor_id)
        else {
            continue;
        };
        let trait_id = analysis.units[*trait_index].id.clone();
        let implementor_id = analysis.units[implementor_index].id.clone();
        let implementor = &mut analysis.units[implementor_index];
        if !implementor
            .relations
            .contains(&TypeRelation::ImplementsUnit(trait_id.clone()))
        {
            implementor
                .relations
                .push(TypeRelation::ImplementsUnit(trait_id));
        }
        let trait_unit = &mut analysis.units[*trait_index];
        if !trait_unit
            .relations
            .contains(&TypeRelation::ImplementedBy(implementor_id.clone()))
        {
            trait_unit
                .relations
                .push(TypeRelation::ImplementedBy(implementor_id));
        }
    }
}
