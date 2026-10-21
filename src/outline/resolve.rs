//! Resolve call names against the neutral method inventory.
//!
//! An adapter supplies callee names. This pass decides which declared method a
//! call belongs to, preferring the same unit, then the same file, then a
//! unique project method. The renderer uses the target to nest same-unit calls
//! and to label calls that leave the component.

use std::collections::BTreeMap;

use super::model::{Analysis, Call, Language, MethodId, RepoPath, Resolution, UnitOrigin};

#[derive(Clone)]
struct Target {
    unit_index: usize,
    method_id: MethodId,
    file: RepoPath,
    language: Language,
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
                });
        }
    }

    let mut updates = Vec::new();
    for (unit_index, unit) in analysis.units.iter().enumerate() {
        for (method_index, method) in unit.methods.iter().enumerate() {
            for (call_index, call) in method.calls.iter().enumerate() {
                let (resolution, target) = resolve(
                    &call.callee,
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
    callee: &str,
    unit_index: usize,
    language: Language,
    file: &RepoPath,
    by_name: &BTreeMap<String, Vec<Target>>,
) -> (Resolution, Option<MethodId>) {
    let Some(candidates) = by_name.get(callee) else {
        return (Resolution::External, None);
    };
    let candidates: Vec<&Target> = candidates
        .iter()
        .filter(|target| target.language == language)
        .collect();
    if candidates.is_empty() {
        return (Resolution::External, None);
    }

    let same_unit: Vec<&Target> = candidates
        .iter()
        .copied()
        .filter(|target| target.unit_index == unit_index)
        .collect();
    if same_unit.len() == 1 {
        return (Resolution::SameFile, Some(same_unit[0].method_id.clone()));
    }
    if same_unit.len() > 1 {
        return (Resolution::Ambiguous, None);
    }

    let same_file: Vec<&Target> = candidates
        .iter()
        .copied()
        .filter(|target| &target.file == file)
        .collect();
    if same_file.len() == 1 {
        return (Resolution::SameFile, Some(same_file[0].method_id.clone()));
    }
    if same_file.len() > 1 {
        return (Resolution::Ambiguous, None);
    }

    if candidates.len() == 1 {
        return (
            Resolution::ProjectOtherFile,
            Some(candidates[0].method_id.clone()),
        );
    }
    (Resolution::Ambiguous, None)
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
                    vec![call("is_skipped"), call("is_complete")],
                ),
                method("state::is_skipped", "is_skipped", Vec::new()),
            ],
            vec![
                outline(
                    "state::artifact_status",
                    "artifact_status",
                    vec![call("is_skipped"), call("is_complete")],
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
