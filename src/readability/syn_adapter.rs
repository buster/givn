//! `syn` adapter: the only module that imports `syn`.
//!
//! It supplies facts only: spans, line kinds, fields, methods, calls, and a
//! neutral flow tree. Metrics and classification stay above the port.

use super::analyzer::SourceAnalyzer;
use super::model::{
    Analysis, AnalysisDiagnostic, AnalysisRequest, AnalyzedFile, AnalyzerCapabilities, Call,
    CallId, Construct, DiagnosticKind, Field, FileStatus, Flow, Language, Limitation, Method,
    OutlineItem, OutlineKind, RepoPath, Resolution, Signature, SourceSpan, SourceUnit, TypeKind,
    TypeRelation, UnitId, UnitKind, UnitOrigin, UnsupportedNote, Visibility,
};
use super::scan::{self, ScannedSource};
use anyhow::Result;
use std::collections::BTreeMap;
use syn::visit::Visit;
use syn::{
    Attribute, Expr, ExprBinary, ExprCall, ExprField, ExprIf, ExprLoop, ExprMatch, ExprMethodCall,
    ExprTry, FnArg, ImplItem, Item, ItemEnum, ItemImpl, ItemStruct, ItemTrait, ItemType, ItemUnion,
    Member, Meta, Pat, ReturnType, TraitItem, Type, Visibility as SynVisibility,
};

pub struct SynAnalyzer;

impl SourceAnalyzer for SynAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            language: Language::Rust,
            supported_constructs: vec![
                Construct::Struct,
                Construct::Enum,
                Construct::Trait,
                Construct::Union,
                Construct::TypeAlias,
                Construct::InherentImpl,
                Construct::TraitImpl,
                Construct::InlineModule,
                Construct::FileModule,
                Construct::FreeFunction,
            ],
            limitations: vec![
                Limitation::ItemMacros,
                Limitation::IncludeMacro,
                Limitation::NonUtf8File,
            ],
            resolution_labels: vec![
                Resolution::SameFile,
                Resolution::ProjectOtherFile,
                Resolution::NameOnly,
                Resolution::Ambiguous,
                Resolution::Dynamic,
                Resolution::External,
            ],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        let sources = scan::discover(&request.project_root, &request.scope.roots)?;
        let mut analysis = Analysis::default();
        let mut impls = ImplCollector::default();

        for source in sources {
            analyze_file(source, &mut analysis, &mut impls);
        }

        attach_impls(&mut analysis, impls);
        analysis.units.sort_by(|a, b| a.id.cmp(&b.id));
        analysis.diagnostics.sort_by(|a, b| {
            (a.path.clone(), a.line, a.message.clone()).cmp(&(
                b.path.clone(),
                b.line,
                b.message.clone(),
            ))
        });
        Ok(analysis)
    }
}

fn analyze_file(source: ScannedSource, analysis: &mut Analysis, impls: &mut ImplCollector) {
    let path = source.path.clone();
    let physical_lines = source.lines.len() as u32;
    let parsed = syn::parse_file(&source.content);
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(error) => {
            analysis.files.push(AnalyzedFile {
                path: path.clone(),
                physical_lines,
                lines: source.lines,
                status: FileStatus::Failed,
                unit_ids: Vec::new(),
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
    };
    file.walk_items(&parsed.items, &module_path);
    impls.pending.append(&mut file.pending_impls);
    impls.records.append(&mut file.impl_records);

    let mut unit_ids: Vec<UnitId> = file.units.iter().map(|unit| unit.id.clone()).collect();
    unit_ids.push(UnitId(format!("module:{}", path)));
    analysis.files.push(AnalyzedFile {
        path: path.clone(),
        physical_lines,
        lines: source.lines,
        status: FileStatus::Analyzed,
        unit_ids: unit_ids.clone(),
    });
    analysis.units.extend(file.units);
    analysis.diagnostics.extend(file.diagnostics);

    let mut module = SourceUnit {
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
}

impl FileBuilder {
    fn walk_items(&mut self, items: &[Item], module_path: &str) {
        for item in items {
            if has_cfg_test(attrs(item)) {
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
                        &self.module_path,
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
        let mut unit = SourceUnit {
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
        let outline = fields
            .iter()
            .enumerate()
            .map(|(index, field)| OutlineItem {
                kind: OutlineKind::Field,
                name: field.name.clone(),
                visibility: field.visibility,
                signature: field.type_label.clone(),
                order: index as u32,
                calls: Vec::new(),
            })
            .collect();
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        self.push_type(
            item.ident.to_string(),
            TypeKind::Struct,
            visibility(&item.vis),
            span,
            fields,
            Vec::new(),
            outline,
        );
    }

    fn push_enum(&mut self, item: &ItemEnum, _module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        let outline = item
            .variants
            .iter()
            .enumerate()
            .map(|(index, variant)| OutlineItem {
                kind: OutlineKind::Field,
                name: variant.ident.to_string(),
                visibility: Visibility::Public,
                signature: String::new(),
                order: index as u32,
                calls: Vec::new(),
            })
            .collect();
        self.push_type(
            item.ident.to_string(),
            TypeKind::Enum,
            visibility(&item.vis),
            span,
            Vec::new(),
            Vec::new(),
            outline,
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

    fn push_trait(&mut self, item: &ItemTrait, _module_path: &str) {
        let span = span_for_ident(&item.ident, &self.content, &self.path);
        self.spans.push(span.clone());
        let mut methods = Vec::new();
        let mut outline = Vec::new();
        for trait_item in &item.items {
            if let TraitItem::Fn(function) = trait_item {
                let method = if let Some(default) = &function.default {
                    method_from_fn(
                        &function.sig,
                        default,
                        &self.path,
                        &self.module_path,
                        true,
                        Visibility::Public,
                        &self.content,
                    )
                } else {
                    method_without_body(
                        &function.sig,
                        &self.path,
                        &self.module_path,
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
        let mut methods = Vec::new();
        let mut outline = Vec::new();
        for impl_item in &item.items {
            if let ImplItem::Fn(function) = impl_item {
                if has_cfg_test(&function.attrs) {
                    continue;
                }
                let method = method_from_fn(
                    &function.sig,
                    &function.block,
                    &self.path,
                    &self.module_path,
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

/// Impls whose type declaration was not in the same file, plus the trait
/// relation facts collected from every attached impl.
#[derive(Default)]
struct ImplCollector {
    pending: Vec<PendingImpl>,
    records: Vec<ImplRecord>,
}

struct PendingImpl {
    file: RepoPath,
    module_path: String,
    target: String,
    trait_name: Option<String>,
    spans: Vec<SourceSpan>,
    methods: Vec<Method>,
    outline: Vec<OutlineItem>,
    line: Option<u32>,
}

struct ImplRecord {
    implementor_id: UnitId,
    trait_name: String,
    file: RepoPath,
    line: Option<u32>,
}

/// Attach cross-file impls to the unique type with the same name, then resolve
/// trait/implementor relations by type name.
///
/// A name declared more than once is ambiguous: the impl is never merged and
/// an `AmbiguousType` diagnostic is recorded. Same-file impls already attached
/// by `FileBuilder` are left untouched.
fn attach_impls(analysis: &mut Analysis, collector: ImplCollector) {
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

fn method_from_fn(
    sig: &syn::Signature,
    block: &syn::Block,
    file: &RepoPath,
    module_path: &str,
    trait_method: bool,
    visibility: Visibility,
    content: &str,
) -> Method {
    let name = sig.ident.to_string();
    let mut visitor = FactsVisitor::default();
    visitor.visit_block(block);
    let id = format!("{module_path}::{name}");
    Method {
        id,
        name,
        visibility,
        signature: signature(sig),
        span: span_for_ident(&sig.ident, content, file),
        flow: Flow::Block(visitor.flow),
        field_accesses: visitor.fields.into_iter().collect(),
        calls: visitor
            .calls
            .into_iter()
            .enumerate()
            .map(|(index, callee)| Call {
                order: index as u32,
                callee,
                resolution: Resolution::SameFile,
                target: None,
            })
            .collect(),
        is_trait_default: trait_method,
        origin: UnitOrigin::Production,
    }
}

fn method_without_body(
    sig: &syn::Signature,
    file: &RepoPath,
    module_path: &str,
    visibility: Visibility,
    content: &str,
) -> Method {
    let name = sig.ident.to_string();
    Method {
        id: format!("{module_path}::{name}"),
        name,
        visibility,
        signature: signature(sig),
        span: span_for_ident(&sig.ident, content, file),
        flow: Flow::Block(Vec::new()),
        field_accesses: Vec::new(),
        calls: Vec::new(),
        is_trait_default: false,
        origin: UnitOrigin::Production,
    }
}

fn outline_method(method: &Method, default: bool) -> OutlineItem {
    OutlineItem {
        kind: if default {
            OutlineKind::TraitMethod
        } else {
            OutlineKind::Method
        },
        name: method.name.clone(),
        visibility: method.visibility,
        signature: method.signature.text.clone(),
        order: method.span.start_line,
        calls: method.calls.clone(),
    }
}

#[derive(Default)]
struct FactsVisitor {
    fields: std::collections::BTreeSet<String>,
    calls: Vec<String>,
    flow: Vec<Flow>,
}

impl<'ast> Visit<'ast> for FactsVisitor {
    fn visit_expr_field(&mut self, node: &'ast ExprField) {
        if is_self(&node.base) {
            match &node.member {
                Member::Named(name) => {
                    self.fields.insert(name.to_string());
                }
                Member::Unnamed(index) => {
                    self.fields.insert(index.index.to_string());
                }
            }
        }
        syn::visit::visit_expr_field(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        self.calls.push(node.method.to_string());
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = &*node.func
            && let Some(segment) = path.path.segments.last()
        {
            self.calls.push(segment.ident.to_string());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.flow
            .push(Flow::Branch(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_if(self, node);
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_loop(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_while(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast ExprMatch) {
        self.flow.push(Flow::Match {
            arms: node.arms.len() as u32,
            bodies: Vec::new(),
        });
        syn::visit::visit_expr_match(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast ExprTry) {
        self.flow.push(Flow::Try(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_try(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        if matches!(node.op, syn::BinOp::And(_) | syn::BinOp::Or(_)) {
            self.flow.push(Flow::ShortCircuit(
                Box::new(Flow::Block(Vec::new())),
                Box::new(Flow::Block(Vec::new())),
            ));
        }
        syn::visit::visit_expr_binary(self, node);
    }
}

fn is_self(expr: &Expr) -> bool {
    matches!(expr, Expr::Path(path) if path.path.is_ident("self"))
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

fn impl_target_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn span_for_ident(ident: &syn::Ident, content: &str, file: &RepoPath) -> SourceSpan {
    line_span(content, file, ident.span().start().line as u32)
}

fn field_span(field: &syn::Field, file: &RepoPath) -> SourceSpan {
    let line = field
        .ident
        .as_ref()
        .map(|ident| ident.span().start().line as u32)
        .unwrap_or(1);
    SourceSpan {
        file: file.clone(),
        start_line: line,
        end_line: line,
    }
}

fn macro_line(item: &syn::ItemMacro) -> u32 {
    item.mac
        .path
        .segments
        .last()
        .map(|segment| segment.ident.span().start().line as u32)
        .unwrap_or(1)
}

fn impl_span_of(item: &ItemImpl, content: &str, file: &RepoPath) -> SourceSpan {
    let first_method = item
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(function) => Some(function.sig.ident.span().start().line as u32),
            _ => None,
        })
        .unwrap_or(1);
    let lines: Vec<&str> = content.lines().collect();
    let start = (1..=first_method)
        .rev()
        .find(|line| {
            lines
                .get(*line as usize - 1)
                .is_some_and(|text| text.trim_start().starts_with("impl"))
        })
        .unwrap_or(first_method);
    line_span(content, file, start)
}

fn line_span(content: &str, file: &RepoPath, start_line: u32) -> SourceSpan {
    let lines: Vec<&str> = content.lines().collect();
    let start_index = start_line.saturating_sub(1) as usize;
    let mut depth = 0i32;
    let mut seen = false;
    let mut end_line = start_line;
    for (index, line) in lines.iter().enumerate().skip(start_index) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    seen = true;
                }
                '}' => {
                    depth -= 1;
                    if seen && depth <= 0 {
                        end_line = index as u32 + 1;
                        return SourceSpan {
                            file: file.clone(),
                            start_line,
                            end_line,
                        };
                    }
                }
                _ => {}
            }
        }
        end_line = index as u32 + 1;
    }
    SourceSpan {
        file: file.clone(),
        start_line,
        end_line: if seen { end_line } else { start_line },
    }
}

fn signature(sig: &syn::Signature) -> Signature {
    let mut text = String::new();
    if sig.constness.is_some() {
        text.push_str("const ");
    }
    if sig.asyncness.is_some() {
        text.push_str("async ");
    }
    if matches!(sig.safety, syn::Safety::Unsafe(_)) {
        text.push_str("unsafe ");
    }
    text.push_str("fn ");
    text.push_str(&sig.ident.to_string());
    text.push('(');
    for (index, arg) in sig.inputs.iter().enumerate() {
        if index > 0 {
            text.push_str(", ");
        }
        match arg {
            FnArg::Receiver(receiver) => {
                match &receiver.kind {
                    syn::ReceiverKind::Reference(_, _, mutable) => {
                        text.push('&');
                        if mutable.is_some() {
                            text.push_str("mut ");
                        }
                    }
                    syn::ReceiverKind::Value => {
                        if receiver.mutability.is_some() {
                            text.push_str("mut ");
                        }
                    }
                    syn::ReceiverKind::Typed(_, ty) => {
                        text.push_str("self: ");
                        text.push_str(&type_label(ty));
                        continue;
                    }
                    _ => {}
                }
                text.push_str("self");
            }
            FnArg::Typed(pat) => {
                if let Pat::Ident(ident) = &*pat.pat {
                    text.push_str(&ident.ident.to_string());
                } else {
                    text.push('_');
                }
                text.push_str(": ");
                text.push_str(&type_label(&pat.ty));
            }
        }
    }
    text.push(')');
    if let ReturnType::Type(_, ty) = &sig.output {
        text.push_str(" -> ");
        text.push_str(&type_label(ty));
    }
    Signature { text }
}

fn type_label(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_else(|| "type".to_string()),
        Type::Reference(reference) => format!("&{}", type_label(&reference.elem)),
        Type::Tuple(tuple) => format!("tuple{}", tuple.elems.len()),
        Type::Slice(slice) => format!("[{}]", type_label(&slice.elem)),
        Type::Array(array) => format!("[{}; _]", type_label(&array.elem)),
        _ => "type".to_string(),
    }
}

fn visibility(vis: &SynVisibility) -> Visibility {
    match vis {
        SynVisibility::Public(_) => Visibility::Public,
        SynVisibility::Restricted(restricted) => {
            if restricted.path.is_ident("crate") {
                Visibility::Crate
            } else {
                Visibility::Restricted
            }
        }
        SynVisibility::Inherited => Visibility::Private,
    }
}

fn attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Struct(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        _ => &[],
    }
}

fn has_cfg_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        match &attr.meta {
            Meta::List(list) => list.tokens.to_string().trim() == "test",
            _ => false,
        }
    })
}

#[allow(dead_code)]
fn _call_id(_id: CallId) {}

#[cfg(test)]
mod tests {
    use super::super::model::SourceScope;
    use super::*;

    fn analyze_sources(files: &[(&str, &str)]) -> Analysis {
        let root = tempfile::tempdir().expect("temp project");
        for (path, content) in files {
            let target = root.path().join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, content).unwrap();
        }
        let request = AnalysisRequest {
            project_root: root.path().to_path_buf(),
            scope: SourceScope {
                language: Language::Rust,
                roots: vec![std::path::PathBuf::from("src")],
                include_test_sources: false,
            },
        };
        SynAnalyzer.analyze(&request).expect("analyze")
    }

    #[test]
    fn type_ids_use_the_declaration_file_and_impls_merge_across_files() {
        let analysis = analyze_sources(&[
            (
                "src/engine.rs",
                "pub struct Engine { a: u64, b: u64 }\nimpl Engine {\n    pub fn first(&self) -> u64 { self.a }\n}\n",
            ),
            (
                "src/engine/parts.rs",
                "impl Engine {\n    pub fn second(&self) -> u64 { self.b }\n    pub fn third(&self) -> u64 { self.a }\n}\n",
            ),
        ]);

        let engine = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/engine.rs::Engine".to_string()))
            .expect("the type is identified by its declaration file");
        assert_eq!(engine.name, "Engine");
        assert_eq!(
            engine.files,
            vec![
                RepoPath::new("src/engine.rs"),
                RepoPath::new("src/engine/parts.rs")
            ]
        );
        let methods: Vec<&str> = engine
            .methods
            .iter()
            .map(|method| method.name.as_str())
            .collect();
        assert_eq!(methods, vec!["first", "second", "third"]);
        assert!(
            engine
                .spans
                .iter()
                .any(|span| span.file == RepoPath::new("src/engine/parts.rs"))
        );
        assert!(
            analysis
                .units
                .iter()
                .any(|unit| unit.id == UnitId("module:src/engine/parts.rs".to_string()))
        );
        assert!(
            analysis
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.kind != DiagnosticKind::AmbiguousType)
        );
    }

    #[test]
    fn an_ambiguous_type_name_is_not_merged() {
        let analysis = analyze_sources(&[
            ("src/a.rs", "pub struct C;\n"),
            ("src/b.rs", "pub struct C;\n"),
            (
                "src/c.rs",
                "impl C {\n    pub fn only(&self) -> u64 { 1 }\n}\n",
            ),
        ]);

        assert!(analysis.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == DiagnosticKind::AmbiguousType
                && diagnostic.path == RepoPath::new("src/c.rs")
        }));
        for unit in analysis
            .units
            .iter()
            .filter(|unit| unit.name == "C" && matches!(unit.kind, UnitKind::Type(_)))
        {
            assert!(
                unit.files
                    .iter()
                    .all(|file| file != &RepoPath::new("src/c.rs")),
                "an ambiguous impl was merged into {}",
                unit.id
            );
        }
    }

    #[test]
    fn a_duplicate_name_resolves_by_the_impl_module_tree() {
        let analysis = analyze_sources(&[
            (
                "src/a_b.rs",
                "pub struct C;\nimpl C {\n    pub fn one(&self) {}\n}\n",
            ),
            (
                "src/a_b/parts.rs",
                "impl C {\n    pub fn two(&self) {}\n}\n",
            ),
            (
                "src/a-b.rs",
                "pub struct C;\nimpl C {\n    pub fn one(&self) {}\n}\n",
            ),
            (
                "src/a-b/parts.rs",
                "impl C {\n    pub fn two(&self) {}\n}\n",
            ),
        ]);
        assert!(
            analysis
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.kind != DiagnosticKind::AmbiguousType),
            "each impl belongs to its own module tree"
        );
        for id in ["type:src/a_b.rs::C", "type:src/a-b.rs::C"] {
            let unit = analysis
                .units
                .iter()
                .find(|unit| unit.id == UnitId(id.to_string()))
                .expect("type unit");
            let methods: Vec<&str> = unit
                .methods
                .iter()
                .map(|method| method.name.as_str())
                .collect();
            assert_eq!(methods, vec!["one", "two"]);
            assert_eq!(unit.files.len(), 2);
        }
    }

    #[test]
    fn a_cross_file_trait_impl_records_both_relations() {
        let analysis = analyze_sources(&[
            (
                "src/api.rs",
                "pub trait Api {\n    fn call(&self) -> u64;\n}\n",
            ),
            (
                "src/engine.rs",
                "pub struct Engine;\nimpl Api for Engine {\n    fn call(&self) -> u64 { 1 }\n}\n",
            ),
        ]);

        let api = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/api.rs::Api".to_string()))
            .expect("trait unit");
        assert!(api.relations.contains(&TypeRelation::ImplementedBy(UnitId(
            "type:src/engine.rs::Engine".to_string()
        ))));
        let engine = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/engine.rs::Engine".to_string()))
            .expect("implementor unit");
        assert!(
            engine
                .relations
                .contains(&TypeRelation::ImplementsUnit(UnitId(
                    "type:src/api.rs::Api".to_string()
                )))
        );
    }
}
