//! Per-file item walk and neutral unit construction.

use super::super::model::{
    Analysis, AnalysisDiagnostic, AnalyzedFile, Construct, DiagnosticKind, Field, FileStatus,
    Language, Method, OutlineItem, OutlineKind, RepoPath, SourceSpan, SourceUnit, TypeKind,
    TypeRelation, UnitId, UnitKind, UnitOrigin, UnsupportedNote, Visibility,
};
use super::facts::{method_from_fn, method_without_body, outline_method};
use super::impl_index::{ImplCollector, ImplRecord, PendingImpl};
use super::scan::ScannedSource;
use super::spans::{
    field_span, impl_span_of, item_span, line_span, macro_line, span_for_ident, span_with_attrs,
};
use super::syntax::{attrs, has_cfg_test, impl_target_name, type_label, visibility};
use std::collections::BTreeMap;
use syn::{
    ImplItem, Item, ItemEnum, ItemImpl, ItemStruct, ItemTrait, ItemType, ItemUnion, TraitItem,
};

pub(super) fn analyze_file(
    source: ScannedSource,
    analysis: &mut Analysis,
    impls: &mut ImplCollector,
) {
    let path = source.path.clone();
    let physical_lines = source.lines.len() as u32;
    let parsed = syn::parse_file(&source.content);
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(error) => {
            analysis.files.push(AnalyzedFile {
                language: Language::Rust,
                path: path.clone(),
                physical_lines,
                lines: source.lines,
                status: FileStatus::Failed,
                unit_ids: Vec::new(),
                excluded_spans: Vec::new(),
            });
            analysis.diagnostics.push(AnalysisDiagnostic {
                kind: DiagnosticKind::ParseFailed,
                path,
                line: Some(error.span().start().line as u32),
                message: error.to_string(),
            });
            return;
        }
    };

    let module_path = module_path(&path);
    let mut file = FileBuilder {
        path: path.clone(),
        module_path: module_path.clone(),
        content: source.content,
        spans: Vec::new(),
        units: Vec::new(),
        unit_by_name: BTreeMap::new(),
        free_functions: Vec::new(),
        outline: Vec::new(),
        diagnostics: Vec::new(),
        unsupported: Vec::new(),
        pending_impls: Vec::new(),
        impl_records: Vec::new(),
        excluded_spans: Vec::new(),
    };
    file.walk_items(&parsed.items, &module_path);
    file.outline.sort_by(|left, right| {
        left.order
            .cmp(&right.order)
            .then_with(|| left.name.cmp(&right.name))
    });
    impls.pending.append(&mut file.pending_impls);
    impls.records.append(&mut file.impl_records);

    let mut unit_ids: Vec<UnitId> = file.units.iter().map(|unit| unit.id.clone()).collect();
    unit_ids.push(UnitId(format!("module:{}", path)));
    analysis.files.push(AnalyzedFile {
        language: Language::Rust,
        path: path.clone(),
        physical_lines,
        lines: source.lines,
        status: FileStatus::Analyzed,
        unit_ids: unit_ids.clone(),
        excluded_spans: file.excluded_spans,
    });
    analysis.units.extend(file.units);
    analysis.diagnostics.extend(file.diagnostics);

    let mut module = SourceUnit {
        language: Language::Rust,
        id: UnitId(format!("module:{}", path)),
        kind: UnitKind::Module,
        name: path.to_string(),
        module_path,
        declaration_file: path.clone(),
        files: vec![path.clone()],
        spans: file.spans,
        visibility: Visibility::Private,
        origin: UnitOrigin::Production,
        methods: file.free_functions,
        fields: Vec::new(),
        relations: Vec::new(),
        outline: file.outline,
        complete: file.unsupported.is_empty(),
        unsupported: file.unsupported,
    };
    module.methods.sort_by(|a, b| a.name.cmp(&b.name));
    analysis.units.push(module);
}

struct FileBuilder {
    path: RepoPath,
    module_path: String,
    content: String,
    spans: Vec<SourceSpan>,
    units: Vec<SourceUnit>,
    unit_by_name: BTreeMap<String, usize>,
    free_functions: Vec<Method>,
    outline: Vec<OutlineItem>,
    diagnostics: Vec<AnalysisDiagnostic>,
    unsupported: Vec<UnsupportedNote>,
    pending_impls: Vec<PendingImpl>,
    impl_records: Vec<ImplRecord>,
    excluded_spans: Vec<SourceSpan>,
}

impl FileBuilder {
    fn walk_items(&mut self, items: &[Item], module_path: &str) {
        for item in items {
            if has_cfg_test(attrs(item)) {
                self.excluded_spans
                    .push(item_span(item, &self.content, &self.path));
                continue;
            }
            match item {
                Item::Struct(item) => {
                    self.push_struct(item, module_path);
                }
                Item::Enum(item) => {
                    self.push_enum(item, module_path);
                }
                Item::Union(item) => {
                    self.push_union(item, module_path);
                }
                Item::Trait(item) => {
                    self.push_trait(item, module_path);
                }
                Item::Type(item) => {
                    self.push_type_alias(item, module_path);
                }
                Item::Impl(item) => {
                    self.push_impl(item, module_path);
                }
                Item::Fn(item) => {
                    let method = method_from_fn(
                        &item.sig,
                        &item.block,
                        &self.path,
                        module_path,
                        false,
                        visibility(&item.vis),
                        &self.content,
                    );
                    self.spans.push(method.span.clone());
                    self.free_functions.push(method.clone());
                    self.outline.push(outline_method(&method, false));
                }
                Item::Mod(item) if item.content.is_some() => {
                    let nested = format!("{module_path}::{}", item.ident);
                    if let Some((_, items)) = &item.content {
                        self.walk_items(items, &nested);
                    }
                }
                Item::Macro(item) => {
                    let span = line_span(&self.content, &self.path, macro_line(item));
                    self.unsupported.push(UnsupportedNote {
                        construct: Construct::ItemMacro,
                        span: span.clone(),
                        reason: "item-position macro is not expanded".to_string(),
                    });
                    self.diagnostics.push(AnalysisDiagnostic {
                        kind: DiagnosticKind::UnsupportedConstruct,
                        path: self.path.clone(),
                        line: Some(span.start_line),
                        message: "item-position macro is not expanded".to_string(),
                    });
                }
                _ => {}
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_type(
        &mut self,
        name: String,
        type_kind: TypeKind,
        visibility: Visibility,
        span: SourceSpan,
        fields: Vec<Field>,
        methods: Vec<Method>,
        outline: Vec<OutlineItem>,
    ) {
        // The stable unit id is the declaration file plus the type name. The
        // module path stays available for method ids and outline ordering, but
        // it is not part of the public identity: a type implemented in another
        // file is still one unit (see `attach_impls`).
        let id = UnitId(format!("type:{}::{name}", self.path));
        self.outline.push(OutlineItem {
            id: id.to_string(),
            kind: OutlineKind::Type,
            name: name.clone(),
            visibility,
            signature: String::new(),
            order: span.start_line,
            calls: Vec::new(),
        });
        let mut unit = SourceUnit {
            language: Language::Rust,
            id: id.clone(),
            kind: UnitKind::Type(type_kind),
            name: name.clone(),
            module_path: self.module_path.clone(),
            declaration_file: self.path.clone(),
            files: vec![self.path.clone()],
            spans: vec![span],
            visibility,
            origin: UnitOrigin::Production,
            methods,
            fields,
            relations: vec![TypeRelation::ContainedIn(UnitId(format!(
                "module:{}",
                self.path
            )))],
            outline,
            complete: true,
            unsupported: Vec::new(),
        };
        unit.methods.sort_by(|a, b| a.name.cmp(&b.name));
        self.unit_by_name.insert(name, self.units.len());
        self.units.push(unit);
    }

    fn push_struct(&mut self, item: &ItemStruct, _module_path: &str) {
        let fields = item
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| Field {
                id: field
                    .ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| index.to_string()),
                name: field
                    .ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| index.to_string()),
                visibility: visibility(&field.vis),
                span: field_span(field, &self.path),
                type_label: type_label(&field.ty),
                origin: UnitOrigin::Production,
            })
            .collect::<Vec<_>>();
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        self.push_type(
            item.ident.to_string(),
            TypeKind::Struct,
            visibility(&item.vis),
            span,
            fields,
            Vec::new(),
            Vec::new(),
        );
    }

    fn push_enum(&mut self, item: &ItemEnum, _module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        self.push_type(
            item.ident.to_string(),
            TypeKind::Enum,
            visibility(&item.vis),
            span,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
    }

    fn push_union(&mut self, item: &ItemUnion, _module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        let fields = item
            .fields
            .named
            .iter()
            .enumerate()
            .map(|(index, field)| Field {
                id: field
                    .ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| index.to_string()),
                name: field
                    .ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| index.to_string()),
                visibility: visibility(&field.vis),
                span: field_span(field, &self.path),
                type_label: type_label(&field.ty),
                origin: UnitOrigin::Production,
            })
            .collect();
        self.push_type(
            item.ident.to_string(),
            TypeKind::Union,
            visibility(&item.vis),
            span,
            fields,
            Vec::new(),
            Vec::new(),
        );
    }

    fn push_trait(&mut self, item: &ItemTrait, module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        let method_path = format!("{module_path}::{}", item.ident);
        let mut methods = Vec::new();
        let mut outline = Vec::new();
        for trait_item in &item.items {
            if let TraitItem::Fn(function) = trait_item {
                let method = if let Some(default) = &function.default {
                    method_from_fn(
                        &function.sig,
                        default,
                        &self.path,
                        &method_path,
                        true,
                        Visibility::Public,
                        &self.content,
                    )
                } else {
                    method_without_body(
                        &function.sig,
                        &self.path,
                        &method_path,
                        Visibility::Public,
                        &self.content,
                    )
                };
                outline.push(outline_method(&method, function.default.is_some()));
                methods.push(method);
            }
        }
        self.push_type(
            item.ident.to_string(),
            TypeKind::Trait,
            visibility(&item.vis),
            span,
            Vec::new(),
            methods,
            outline,
        );
    }

    fn push_type_alias(&mut self, item: &ItemType, _module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        self.push_type(
            item.ident.to_string(),
            TypeKind::TypeAlias,
            visibility(&item.vis),
            span,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
    }

    fn push_impl(&mut self, item: &ItemImpl, module_path: &str) {
        let Some(name) = impl_target_name(&item.self_ty) else {
            return;
        };
        let trait_name = item.trait_.as_ref().and_then(|(path, _)| {
            path.segments
                .last()
                .map(|segment| segment.ident.to_string())
        });
        let impl_span = impl_span_of(item, &self.content, &self.path);
        self.spans.push(impl_span.clone());
        let method_path = format!("{module_path}::{name}");
        let mut methods = Vec::new();
        let mut outline = Vec::new();
        for impl_item in &item.items {
            if let ImplItem::Fn(function) = impl_item {
                if has_cfg_test(&function.attrs) {
                    self.excluded_spans.push(span_with_attrs(
                        &function.attrs,
                        &function.sig.ident,
                        &self.content,
                        &self.path,
                    ));
                    continue;
                }
                let method = method_from_fn(
                    &function.sig,
                    &function.block,
                    &self.path,
                    &method_path,
                    false,
                    visibility(&function.vis),
                    &self.content,
                );
                outline.push(outline_method(&method, false));
                methods.push(method);
            }
        }
        let method_spans: Vec<SourceSpan> =
            methods.iter().map(|method| method.span.clone()).collect();
        let start_line = impl_span.start_line;
        let mut spans = method_spans;
        spans.push(impl_span);

        if let Some(index) = self.unit_by_name.get(&name).copied() {
            let unit = &mut self.units[index];
            unit.spans.extend(spans);
            unit.methods.extend(methods);
            unit.methods.sort_by(|a, b| a.name.cmp(&b.name));
            if unit.files.iter().all(|file| file != &self.path) {
                unit.files.push(self.path.clone());
            }
            unit.outline.extend(outline);
            unit.outline.sort_by_key(|item| item.order);
            if let Some(trait_name) = trait_name {
                self.impl_records.push(ImplRecord {
                    implementor_id: unit.id.clone(),
                    trait_name,
                    file: self.path.clone(),
                    line: Some(start_line),
                });
            }
            return;
        }

        // The declaration may live in another file. Keep the impl until every
        // file has been visited, then attach it to the unique type with this
        // name (or refuse an ambiguous name).
        self.pending_impls.push(PendingImpl {
            file: self.path.clone(),
            module_path: module_path.to_string(),
            target: name,
            trait_name,
            spans,
            methods,
            outline,
            line: Some(start_line),
        });
    }
}

fn module_path(path: &RepoPath) -> String {
    let mut value = path.as_str().trim_end_matches(".rs").to_string();
    for prefix in ["src/", "crates/"] {
        if let Some(rest) = value.strip_prefix(prefix) {
            value = rest.to_string();
            break;
        }
    }
    value = value.replace("/mod", "");
    value.replace('/', "::")
}
