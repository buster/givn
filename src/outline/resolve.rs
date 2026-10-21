//! Resolve calls against the neutral method inventory and the call's shape.
//!
//! An adapter supplies callee names and whether the call is a free function
//! or a method, plus any receiver evidence. This pass only claims
//! `same_file`/`project_other_file` when the candidate's owner matches that
//! evidence. A method call on an unknown receiver stays `name_only`.

use std::collections::BTreeMap;

use super::model::{
    Analysis, Call, CallShape, Language, MethodId, ReceiverEvidence, RepoPath, Resolution,
    TypeKind, UnitKind, UnitOrigin,
};

#[derive(Clone)]
struct Target {
    unit_index: usize,
    method_id: MethodId,
    file: RepoPath,
    language: Language,
    unit_kind: UnitKind,
    unit_name: String,
    module_path: String,
}

pub(crate) fn resolve_calls(analysis: &mut Analysis) {
    let mut by_name: BTreeMap<String, Vec<Target>> = BTreeMap::new();
    for (unit_index, unit) in analysis.units.iter().enumerate() {
        if unit.origin != UnitOrigin::Production {
            continue;
        }
        for method in &unit.methods {
            by_name
                .entry(method.name.clone())
                .or_default()
                .push(Target {
                    unit_index,
                    method_id: method.id.clone(),
                    file: method.span.file.clone(),
                    language: unit.language,
                    unit_kind: unit.kind,
                    unit_name: unit.name.clone(),
                    module_path: unit.module_path.clone(),
                });
        }
    }

    let mut updates = Vec::new();
    for (unit_index, unit) in analysis.units.iter().enumerate() {
        for (method_index, method) in unit.methods.iter().enumerate() {
            for (call_index, call) in method.calls.iter().enumerate() {
                let (resolution, target) = resolve(
                    call,
                    unit_index,
                    unit.language,
                    &method.span.file,
                    &by_name,
                );
                updates.push((unit_index, method_index, call_index, resolution, target));
            }
        }
    }
    for (unit_index, method_index, call_index, resolution, target) in updates {
        let call = &mut analysis.units[unit_index].methods[method_index].calls[call_index];
        call.resolution = resolution;
        call.target = target;
    }

    // Outline items copy their method's calls before this pass runs. Keep the
    // two views identical so the report sees the resolved targets.
    for unit in &mut analysis.units {
        let calls_by_id: BTreeMap<MethodId, Vec<Call>> = unit
            .methods
            .iter()
            .map(|method| (method.id.clone(), method.calls.clone()))
            .collect();
        for item in &mut unit.outline {
            if let Some(calls) = calls_by_id.get(&item.id) {
                item.calls = calls.clone();
            }
        }
    }
}

fn resolve(
    call: &Call,
    unit_index: usize,
    language: Language,
    file: &RepoPath,
    by_name: &BTreeMap<String, Vec<Target>>,
) -> (Resolution, Option<MethodId>) {
    let Some(candidates) = by_name.get(&call.callee) else {
        return (Resolution::External, None);
    };
    let candidates: Vec<&Target> = candidates
        .iter()
        .filter(|target| target.language == language)
        .collect();
    if candidates.is_empty() {
        return (Resolution::External, None);
    }

    let compatible: Vec<&Target> = match &call.shape {
        CallShape::Function { qualifier: None } => candidates
            .iter()
            .copied()
            .filter(|target| matches!(target.unit_kind, UnitKind::Module))
            .collect(),
        CallShape::Function {
            qualifier: Some(qualifier),
        } => candidates
            .iter()
            .copied()
            .filter(|target| function_qualifier_matches(qualifier, unit_index, target))
            .collect(),
        CallShape::Method {
            receiver: ReceiverEvidence::SelfUnit,
        } => {
            let matching: Vec<&Target> = candidates
                .iter()
                .copied()
                .filter(|target| target.unit_index == unit_index)
                .collect();
            if matching.is_empty() {
                return name_only_or_ambiguous(&candidates);
            }
            matching
        }
        CallShape::Method {
            receiver: ReceiverEvidence::Qualifier(qualifier),
        } => candidates
            .iter()
            .copied()
            .filter(|target| {
                matches!(target.unit_kind, UnitKind::Type(_))
                    && (target.unit_name == *qualifier
                        || (qualifier == "Self" && target.unit_index == unit_index))
            })
            .collect(),
        CallShape::Method {
            receiver: ReceiverEvidence::Unknown,
        } => return name_only_or_ambiguous(&candidates),
    };

    if compatible.is_empty() {
        // A named receiver that only matches a trait may still be implemented
        // by that owner; the relation is not proven, so it is a name-only
        // guess rather than an external call.
        if matches!(
            &call.shape,
            CallShape::Method {
                receiver: ReceiverEvidence::Qualifier(_)
            }
        ) && candidates
            .iter()
            .any(|target| matches!(target.unit_kind, UnitKind::Type(TypeKind::Trait)))
        {
            return name_only_or_ambiguous(&candidates);
        }
        return (Resolution::External, None);
    }

    let same_unit: Vec<&Target> = compatible
        .iter()
        .copied()
        .filter(|target| target.unit_index == unit_index)
        .collect();
    if !same_unit.is_empty() {
        return one_target(same_unit.as_slice(), file);
    }

    let same_file: Vec<&Target> = compatible
        .iter()
        .copied()
        .filter(|target| &target.file == file)
        .collect();
    if !same_file.is_empty() {
        return one_target(same_file.as_slice(), file);
    }

    one_target(compatible.as_slice(), file)
}

fn one_target(candidates: &[&Target], caller_file: &RepoPath) -> (Resolution, Option<MethodId>) {
    match candidates {
        [target] => (
            if &target.file == caller_file {
                Resolution::SameFile
            } else {
                Resolution::ProjectOtherFile
            },
            Some(target.method_id.clone()),
        ),
        _ => (Resolution::Ambiguous, None),
    }
}

fn name_only_or_ambiguous(candidates: &[&Target]) -> (Resolution, Option<MethodId>) {
    match candidates {
        [target] => (
            Resolution::NameOnly,
            Some(target.method_id.clone()),
        ),
        _ => (Resolution::Ambiguous, None),
    }
}

fn function_qualifier_matches(qualifier: &str, unit_index: usize, target: &Target) -> bool {
    if qualifier == "Self" {
        return target.unit_index == unit_index;
    }
    let Some(owner) = qualifier.rsplit("::").next() else {
        return false;
    };
    match target.unit_kind {
        UnitKind::Type(_) => target.unit_name == owner,
        UnitKind::Module => {
            module_name_matches(&target.unit_name, owner)
                || target
                    .module_path
                    .rsplit("::")
                    .next()
                    .is_some_and(|name| name == owner)
        }
    }
}

fn module_name_matches(path: &str, owner: &str) -> bool {
    let stem = path.trim_end_matches(".rs");
    let stem = stem.strip_suffix("/mod").unwrap_or(stem);
    stem.rsplit('/').next().is_some_and(|name| name == owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outline::model::{
        Analysis, Flow, Method, OutlineItem, OutlineKind, Signature, SourceSpan, SourceUnit,
        TypeKind, UnitId, UnitKind, UnitOrigin, Visibility,
    };

    fn path() -> RepoPath {
        RepoPath::new("src/state.rs")
    }

    fn span() -> SourceSpan {
        SourceSpan {
            file: path(),
            start_line: 1,
            end_line: 1,
        }
    }

    fn method(id: &str, name: &str, calls: Vec<Call>) -> Method {
        Method {
            id: id.to_string(),
            name: name.to_string(),
            visibility: Visibility::Public,
            signature: Signature {
                text: format!("fn {name}()"),
            },
            span: span(),
            flow: Flow::Block(Vec::new()),
            field_accesses: Vec::new(),
            calls,
            is_trait_default: false,
            origin: UnitOrigin::Production,
        }
    }

    fn outline(id: &str, name: &str, calls: Vec<Call>) -> OutlineItem {
        OutlineItem {
            id: id.to_string(),
            kind: OutlineKind::Method,
            name: name.to_string(),
            visibility: Visibility::Public,
            signature: format!("fn {name}()"),
            order: 1,
            calls,
        }
    }

    fn unit(
        id: &str,
        kind: UnitKind,
        name: &str,
        methods: Vec<Method>,
        outline: Vec<OutlineItem>,
    ) -> SourceUnit {
        SourceUnit {
            id: UnitId(id.to_string()),
            language: Language::Rust,
            kind,
            name: name.to_string(),
            module_path: "state".to_string(),
            declaration_file: path(),
            files: vec![path()],
            spans: Vec::new(),
            visibility: Visibility::Public,
            origin: UnitOrigin::Production,
            methods,
            fields: Vec::new(),
            relations: Vec::new(),
            outline,
            complete: true,
            unsupported: Vec::new(),
        }
    }

    fn call(callee: &str) -> Call {
        Call {
            order: 0,
            callee: callee.to_string(),
            shape: CallShape::Function { qualifier: None },
            resolution: Resolution::External,
            target: None,
        }
    }

    fn method_call(callee: &str, receiver: &str) -> Call {
        Call {
            order: 0,
            callee: callee.to_string(),
            shape: CallShape::Method {
                receiver: ReceiverEvidence::Qualifier(receiver.to_string()),
            },
            resolution: Resolution::External,
            target: None,
        }
    }

    #[test]
    fn resolves_same_unit_then_same_file_and_syncs_the_outline() {
        let module = unit(
            "module:src/state.rs",
            UnitKind::Module,
            "src/state.rs",
            vec![
                method(
                    "state::artifact_status",
                    "artifact_status",
                    vec![call("is_skipped"), method_call("is_complete", "TasksProgress")],
                ),
                method("state::is_skipped", "is_skipped", Vec::new()),
            ],
            vec![
                outline(
                    "state::artifact_status",
                    "artifact_status",
                    vec![call("is_skipped"), method_call("is_complete", "TasksProgress")],
                ),
                outline("state::is_skipped", "is_skipped", Vec::new()),
            ],
        );
        let change = unit(
            "type:src/state.rs::ChangeState",
            UnitKind::Type(TypeKind::Struct),
            "ChangeState",
            vec![method(
                "state::ChangeState::from_manifest",
                "from_manifest",
                vec![call("artifact_status")],
            )],
            vec![outline(
                "state::ChangeState::from_manifest",
                "from_manifest",
                vec![call("artifact_status")],
            )],
        );
        let tasks = unit(
            "type:src/state.rs::TasksProgress",
            UnitKind::Type(TypeKind::Struct),
            "TasksProgress",
            vec![method(
                "type:src/state.rs::TasksProgress::is_complete",
                "is_complete",
                Vec::new(),
            )],
            vec![outline(
                "type:src/state.rs::TasksProgress::is_complete",
                "is_complete",
                Vec::new(),
            )],
        );
        let mut analysis = Analysis {
            files: Vec::new(),
            units: vec![module, change, tasks],
            diagnostics: Vec::new(),
        };

        resolve_calls(&mut analysis);

        assert_eq!(
            analysis.units[0].methods[0].calls[0].target.as_deref(),
            Some("state::is_skipped")
        );
        assert_eq!(
            analysis.units[0].methods[0].calls[1].target.as_deref(),
            Some("type:src/state.rs::TasksProgress::is_complete")
        );
        assert_eq!(
            analysis.units[1].methods[0].calls[0].target.as_deref(),
            Some("state::artifact_status")
        );
        assert_eq!(
            analysis.units[0].outline[0].calls[1].target.as_deref(),
            Some("type:src/state.rs::TasksProgress::is_complete")
        );
    }
}
