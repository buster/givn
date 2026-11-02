//! Java and TypeScript adapters backed by Tree-sitter.
//!
//! Rust and Python keep their existing analyzers. These adapters supply the
//! same neutral facts for the added languages: units, methods, fields, calls,
//! spans, visibility, signatures, and explicit unsupported constructs.

use super::analyzer::SourceAnalyzer;
use super::model::{
    Analysis, AnalysisDiagnostic, AnalysisRequest, AnalyzedFile, AnalyzerCapabilities, Call,
    CallShape, Construct, DiagnosticKind, Field, FileStatus, Flow, Language, Limitation, Method,
    OutlineItem, OutlineKind, ReceiverEvidence, RepoPath, Resolution, Signature, SourceSpan,
    SourceUnit, TypeKind, TypeRelation, UnitId, UnitKind, UnitOrigin, Visibility,
};
use super::scan;
use anyhow::{Context, Result};
use tree_sitter::{Node, Parser};

pub struct JavaAnalyzer;
pub struct TypeScriptAnalyzer;
pub struct JavaScriptAnalyzer;

impl SourceAnalyzer for JavaAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            languages: vec![Language::Java],
            supported_constructs: vec![
                Construct::Class,
                Construct::Interface,
                Construct::Enum,
                Construct::Record,
                Construct::Function,
                Construct::FileModule,
            ],
            limitations: vec![
                Limitation::JavaPackageScope,
                Limitation::JavaRecordsMappedToClass,
                Limitation::JavaAnonymousClasses,
                Limitation::NonUtf8File,
                Limitation::StructuralCallOrderNotRuntimeTrace,
                Limitation::UncertainCallEdges,
                Limitation::CrossLanguageNameResolution,
            ],
            resolution_labels: vec![
                Resolution::SameFile,
                Resolution::ProjectOtherFile,
                Resolution::NameOnly,
                Resolution::Ambiguous,
                Resolution::External,
            ],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        if !request.scope.languages.contains(&Language::Java) {
            return Ok(Analysis::default());
        }
        let sources = scan::discover(
            &request.project_root,
            &request.scope.roots,
            &[Language::Java],
        )?;
        let mut analysis = Analysis::default();
        for source in sources {
            analyze_java_file(source, &mut analysis)?;
        }
        sort_analysis(&mut analysis);
        Ok(analysis)
    }
}

impl SourceAnalyzer for TypeScriptAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            languages: vec![Language::TypeScript],
            supported_constructs: vec![
                Construct::Class,
                Construct::Interface,
                Construct::Enum,
                Construct::TypeAlias,
                Construct::Function,
                Construct::FileModule,
            ],
            limitations: vec![
                Limitation::TypeScriptDialectSelection,
                Limitation::TypeScriptTypeOnlyDeclarations,
                Limitation::TypeScriptNamespaces,
                Limitation::NonUtf8File,
                Limitation::StructuralCallOrderNotRuntimeTrace,
                Limitation::UncertainCallEdges,
                Limitation::CrossLanguageNameResolution,
            ],
            resolution_labels: vec![
                Resolution::SameFile,
                Resolution::ProjectOtherFile,
                Resolution::NameOnly,
                Resolution::Ambiguous,
                Resolution::External,
            ],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        if !request.scope.languages.contains(&Language::TypeScript) {
            return Ok(Analysis::default());
        }
        let sources = scan::discover(
            &request.project_root,
            &request.scope.roots,
            &[Language::TypeScript],
        )?;
        let mut analysis = Analysis::default();
        for source in sources {
            analyze_typescript_file(source, &mut analysis)?;
        }
        sort_analysis(&mut analysis);
        Ok(analysis)
    }
}

impl SourceAnalyzer for JavaScriptAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            languages: vec![Language::JavaScript],
            supported_constructs: vec![
                Construct::Class,
                Construct::Function,
                Construct::FileModule,
            ],
            limitations: vec![
                Limitation::NonUtf8File,
                Limitation::StructuralCallOrderNotRuntimeTrace,
                Limitation::UncertainCallEdges,
                Limitation::CrossLanguageNameResolution,
            ],
            resolution_labels: vec![
                Resolution::SameFile,
                Resolution::ProjectOtherFile,
                Resolution::NameOnly,
                Resolution::Ambiguous,
                Resolution::External,
            ],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        if !request.scope.languages.contains(&Language::JavaScript) {
            return Ok(Analysis::default());
        }
        let sources = scan::discover(
            &request.project_root,
            &request.scope.roots,
            &[Language::JavaScript],
        )?;
        let mut analysis = Analysis::default();
        for source in sources {
            analyze_javascript_file(source, &mut analysis)?;
        }
        sort_analysis(&mut analysis);
        Ok(analysis)
    }
}

fn sort_analysis(analysis: &mut Analysis) {
    analysis.units.sort_by(|left, right| left.id.cmp(&right.id));
    analysis.diagnostics.sort_by(|left, right| {
        (left.path.clone(), left.line, left.message.clone()).cmp(&(
            right.path.clone(),
            right.line,
            right.message.clone(),
        ))
    });
}

fn failed_file(source: &scan::ScannedSource, language: Language, message: String) -> Analysis {
    let mut analysis = Analysis::default();
    analysis.diagnostics.push(AnalysisDiagnostic {
        kind: DiagnosticKind::ParseFailed,
        path: source.path.clone(),
        line: Some(1),
        message,
    });
    analysis.files.push(AnalyzedFile {
        path: source.path.clone(),
        language,
        physical_lines: source.content.lines().count() as u32,
        lines: source.lines.clone(),
        status: FileStatus::Failed,
        unit_ids: Vec::new(),
        excluded_spans: Vec::new(),
    });
    analysis
}

fn span(file: &RepoPath, node: Node<'_>) -> SourceSpan {
    SourceSpan {
        file: file.clone(),
        start_line: node.start_position().row as u32 + 1,
        end_line: node.end_position().row as u32 + 1,
    }
}

fn text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

fn name_text(node: Node<'_>, source: &str, field: &str) -> Option<String> {
    node.child_by_field_name(field)
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn signature_text(node: Node<'_>, source: &str) -> String {
    let end = node
        .child_by_field_name("body")
        .map(|body| body.start_byte())
        .unwrap_or_else(|| node.end_byte());
    let raw = &source[node.start_byte()..end.max(node.start_byte())];
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn modifiers_text(node: Node<'_>, source: &str) -> String {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| matches!(child.kind(), "modifiers" | "accessibility_modifier"))
        .map(|modifiers| text(modifiers, source).to_string())
        .unwrap_or_default()
}

fn java_visibility(node: Node<'_>, source: &str) -> Visibility {
    let modifiers = modifiers_text(node, source);
    if modifiers.contains("public") {
        Visibility::Public
    } else if modifiers.contains("protected") {
        Visibility::Restricted
    } else if modifiers.contains("private") {
        Visibility::Private
    } else {
        Visibility::Crate
    }
}

fn ecmascript_visibility(node: Node<'_>, source: &str, name: &str) -> Visibility {
    if name.starts_with('#') {
        return Visibility::Private;
    }
    let modifiers = modifiers_text(node, source);
    if modifiers.contains("private") {
        Visibility::Private
    } else if modifiers.contains("protected") {
        Visibility::Restricted
    } else {
        Visibility::Public
    }
}

fn call_shape_java(node: Node<'_>, source: &str) -> Option<(String, CallShape)> {
    match node.kind() {
        "method_invocation" => {
            let callee = name_text(node, source, "name")?;
            let shape = node
                .child_by_field_name("object")
                .map(|object| {
                    let object = text(object, source).trim();
                    if object == "this" || object == "super" {
                        CallShape::Method {
                            receiver: ReceiverEvidence::SelfUnit,
                        }
                    } else if object.chars().next().is_some_and(char::is_uppercase) {
                        CallShape::Method {
                            receiver: ReceiverEvidence::Qualifier(object.to_string()),
                        }
                    } else {
                        CallShape::Method {
                            receiver: ReceiverEvidence::Unknown,
                        }
                    }
                })
                .unwrap_or(CallShape::Method {
                    receiver: ReceiverEvidence::SelfUnit,
                });
            Some((callee, shape))
        }
        "object_creation_expression" => {
            let type_name = name_text(node, source, "type")?;
            let callee = type_name.rsplit('.').next()?.to_string();
            Some((
                callee,
                CallShape::Function {
                    qualifier: Some(type_name),
                },
            ))
        }
        _ => None,
    }
}

fn call_shape_ecmascript(node: Node<'_>, source: &str) -> Option<(String, CallShape)> {
    match node.kind() {
        "call_expression" => {
            let function = node.child_by_field_name("function")?;
            match function.kind() {
                "identifier" => Some((
                    text(function, source).to_string(),
                    CallShape::Function { qualifier: None },
                )),
                "member_expression" => {
                    let callee = function
                        .child_by_field_name("property")
                        .and_then(|property| property.utf8_text(source.as_bytes()).ok())?
                        .to_string();
                    let object = function
                        .child_by_field_name("object")
                        .map(|object| text(object, source).trim().to_string())
                        .unwrap_or_default();
                    let receiver = if object == "this" {
                        ReceiverEvidence::SelfUnit
                    } else if object.chars().next().is_some_and(char::is_uppercase) {
                        ReceiverEvidence::Qualifier(object)
                    } else {
                        ReceiverEvidence::Unknown
                    };
                    Some((callee, CallShape::Method { receiver }))
                }
                _ => None,
            }
        }
        "new_expression" => {
            let constructor = node.child_by_field_name("constructor")?;
            let type_name = text(constructor, source).trim().to_string();
            let callee = type_name.rsplit('.').next()?.to_string();
            Some((
                callee,
                CallShape::Function {
                    qualifier: Some(type_name),
                },
            ))
        }
        _ => None,
    }
}

fn calls_in<F>(node: Node<'_>, source: &str, mut shape: F) -> Vec<Call>
where
    F: FnMut(Node<'_>, &str) -> Option<(String, CallShape)>,
{
    let mut calls = Vec::new();
    let mut cursor = node.walk();
    collect_calls(node, source, &mut shape, &mut calls);
    let _ = &mut cursor;
    calls
}

fn collect_calls<F>(node: Node<'_>, source: &str, shape: &mut F, calls: &mut Vec<Call>)
where
    F: FnMut(Node<'_>, &str) -> Option<(String, CallShape)>,
{
    if let Some((callee, shape)) = shape(node, source) {
        calls.push(Call {
            order: calls.len() as u32,
            callee,
            shape,
            resolution: Resolution::External,
            target: None,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_calls(child, source, shape, calls);
    }
}

fn field_from_java(node: Node<'_>, source: &str, file: &RepoPath) -> Option<Field> {
    let declarator = node.child_by_field_name("declarator")?;
    let name = name_text(declarator, source, "name")?;
    let type_label = name_text(node, source, "type").unwrap_or_default();
    Some(Field {
        id: name.clone(),
        name,
        visibility: java_visibility(node, source),
        span: span(file, node),
        type_label,
        origin: UnitOrigin::Production,
    })
}

fn field_from_ecmascript(node: Node<'_>, source: &str, file: &RepoPath) -> Option<Field> {
    let name = name_text(node, source, "name").or_else(|| name_text(node, source, "property"))?;
    let type_label = node
        .child_by_field_name("type")
        .map(|type_node| {
            text(type_node, source)
                .trim()
                .trim_start_matches(':')
                .trim()
                .to_string()
        })
        .unwrap_or_default();
    Some(Field {
        id: name.clone(),
        name: name.clone(),
        visibility: ecmascript_visibility(node, source, &name),
        span: span(file, node),
        type_label,
        origin: UnitOrigin::Production,
    })
}

#[allow(clippy::too_many_arguments)]
fn method_from(
    node: Node<'_>,
    source: &str,
    _file: &RepoPath,
    module_path: &str,
    name: String,
    visibility: Visibility,
    calls: Vec<Call>,
    trait_method: bool,
    span: SourceSpan,
) -> Method {
    Method {
        id: format!("{module_path}::{name}#{}", span.start_line),
        name,
        visibility,
        signature: Signature {
            text: signature_text(node, source),
        },
        span,
        flow: Flow::Unavailable,
        field_accesses: Vec::new(),
        calls,
        is_trait_default: trait_method,
        origin: UnitOrigin::Production,
    }
}

fn outline_method(method: &Method, trait_method: bool) -> OutlineItem {
    OutlineItem {
        id: method.id.clone(),
        kind: if trait_method {
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

struct FileBuilder {
    path: RepoPath,
    module_path: String,
    language: Language,
    module: SourceUnit,
    units: Vec<SourceUnit>,
    unsupported: Vec<super::model::UnsupportedNote>,
}

impl FileBuilder {
    fn new(path: RepoPath, language: Language) -> Self {
        let module_id = UnitId(format!("module:{}", path));
        let module = SourceUnit {
            id: module_id,
            language,
            kind: UnitKind::Module,
            name: path.as_str().to_string(),
            module_path: path
                .as_str()
                .trim_end_matches(".java")
                .trim_end_matches(".ts")
                .trim_end_matches(".tsx")
                .trim_end_matches(".js")
                .trim_end_matches(".mjs")
                .trim_end_matches(".cjs")
                .to_string(),
            declaration_file: path.clone(),
            files: vec![path.clone()],
            spans: vec![SourceSpan {
                file: path.clone(),
                start_line: 1,
                end_line: 1,
            }],
            visibility: Visibility::Public,
            origin: UnitOrigin::Production,
            methods: Vec::new(),
            fields: Vec::new(),
            relations: Vec::new(),
            outline: Vec::new(),
            complete: true,
            unsupported: Vec::new(),
        };
        Self {
            module_path: module.module_path.clone(),
            path,
            module,
            units: Vec::new(),
            language,
            unsupported: Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_type(
        &mut self,
        name: String,
        qualified: String,
        type_kind: TypeKind,
        visibility: Visibility,
        span: SourceSpan,
        fields: Vec<Field>,
        methods: Vec<Method>,
        parent: Option<UnitId>,
    ) -> UnitId {
        let id = UnitId(format!("type:{}::{qualified}", self.path));
        let mut relations = vec![TypeRelation::ContainedIn(UnitId(format!(
            "module:{}",
            self.path
        )))];
        if let Some(parent) = parent {
            relations.push(TypeRelation::ContainedIn(parent));
        }
        let mut outline = vec![OutlineItem {
            id: id.to_string(),
            kind: OutlineKind::Type,
            name: name.clone(),
            visibility,
            signature: String::new(),
            order: span.start_line,
            calls: Vec::new(),
        }];
        outline.extend(
            methods
                .iter()
                .map(|method| outline_method(method, matches!(type_kind, TypeKind::Interface))),
        );
        self.module.outline.push(outline[0].clone());
        self.units.push(SourceUnit {
            id: id.clone(),
            language: self.language,
            kind: UnitKind::Type(type_kind),
            name,
            module_path: self.module_path.clone(),
            declaration_file: self.path.clone(),
            files: vec![self.path.clone()],
            spans: vec![span],
            visibility,
            origin: UnitOrigin::Production,
            methods,
            fields,
            relations,
            outline,
            complete: true,
            unsupported: Vec::new(),
        });
        id
    }

    fn finish(mut self, source: &scan::ScannedSource) -> Analysis {
        self.module.outline.sort_by(|left, right| {
            left.order
                .cmp(&right.order)
                .then_with(|| left.name.cmp(&right.name))
        });
        self.module.unsupported = self.unsupported;
        let mut unit_ids = vec![self.module.id.clone()];
        unit_ids.extend(self.units.iter().map(|unit| unit.id.clone()));
        self.units.push(self.module);
        let mut analysis = Analysis {
            units: self.units,
            ..Analysis::default()
        };
        analysis.files.push(AnalyzedFile {
            path: self.path,
            language: self.language,
            physical_lines: source.content.lines().count() as u32,
            lines: source.lines.clone(),
            status: FileStatus::Analyzed,
            unit_ids,
            excluded_spans: Vec::new(),
        });
        analysis
    }
}

fn analyze_java_file(source: scan::ScannedSource, analysis: &mut Analysis) -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .context("load the Java Tree-sitter grammar")?;
    let tree = parser
        .parse(&source.content, None)
        .context("parse the Java source with Tree-sitter")?;
    if tree.root_node().has_error() {
        analysis.diagnostics.push(AnalysisDiagnostic {
            kind: DiagnosticKind::ParseFailed,
            path: source.path.clone(),
            line: Some(1),
            message: "Tree-sitter reported error nodes in the Java source".to_string(),
        });
        let failed = failed_file(&source, Language::Java, "parse failed".to_string());
        analysis.files.extend(failed.files);
        return Ok(());
    }
    let mut builder = FileBuilder::new(source.path.clone(), Language::Java);
    walk_java(tree.root_node(), &source.content, &mut builder, None);
    let file_analysis = builder.finish(&source);
    merge(analysis, file_analysis);
    Ok(())
}

fn walk_java<'a>(
    node: Node<'a>,
    source: &'a str,
    builder: &mut FileBuilder,
    parent: Option<UnitId>,
) {
    match node.kind() {
        "class_declaration"
        | "interface_declaration"
        | "enum_declaration"
        | "record_declaration" => {
            let Some(name) = name_text(node, source, "name") else {
                return;
            };
            let type_kind = match node.kind() {
                "interface_declaration" => TypeKind::Interface,
                "enum_declaration" => TypeKind::Enum,
                _ => TypeKind::Class,
            };
            let mut fields = Vec::new();
            let mut methods = Vec::new();
            let mut nested = Vec::new();
            if let Some(body) = node.child_by_field_name("body") {
                let mut cursor = body.walk();
                for child in body.children(&mut cursor) {
                    match child.kind() {
                        "field_declaration" | "constant_declaration" => {
                            if let Some(field) = field_from_java(child, source, &builder.path) {
                                fields.push(field);
                            }
                        }
                        "method_declaration" | "constructor_declaration" => {
                            if let Some(method_name) = name_text(child, source, "name") {
                                let calls = calls_in(child, source, call_shape_java);
                                let visibility = java_visibility(child, source);
                                methods.push(method_from(
                                    child,
                                    source,
                                    &builder.path,
                                    &builder.module_path,
                                    method_name,
                                    visibility,
                                    calls,
                                    false,
                                    span(&builder.path, child),
                                ));
                            }
                        }
                        "class_declaration"
                        | "interface_declaration"
                        | "enum_declaration"
                        | "record_declaration" => nested.push(child),
                        "enum_constant" => {
                            if let Some(name) = name_text(child, source, "name") {
                                fields.push(Field {
                                    id: name.clone(),
                                    name,
                                    visibility: Visibility::Public,
                                    span: span(&builder.path, child),
                                    type_label: String::new(),
                                    origin: UnitOrigin::Production,
                                });
                            }
                        }
                        "object_creation_expression"
                            if child.child_by_field_name("body").is_some() =>
                        {
                            builder.unsupported.push(super::model::UnsupportedNote {
                                construct: Construct::Class,
                                span: span(&builder.path, child),
                                reason: "anonymous class".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
            }
            if node.kind() == "record_declaration"
                && let Some(parameters) = node.child_by_field_name("parameters")
            {
                let mut cursor = parameters.walk();
                for child in parameters.children(&mut cursor) {
                    if child.kind() == "formal_parameter"
                        && let Some(name) = name_text(child, source, "name")
                    {
                        fields.push(Field {
                            id: name.clone(),
                            name,
                            visibility: Visibility::Public,
                            span: span(&builder.path, child),
                            type_label: name_text(child, source, "type").unwrap_or_default(),
                            origin: UnitOrigin::Production,
                        });
                    }
                }
            }
            let qualified = parent
                .as_ref()
                .map(|parent| {
                    format!(
                        "{}::{name}",
                        parent.0.rsplit("::").next().unwrap_or(&parent.0)
                    )
                })
                .unwrap_or_else(|| name.clone());
            let id = builder.add_type(
                name.clone(),
                qualified,
                type_kind,
                java_visibility(node, source),
                span(&builder.path, node),
                fields,
                methods,
                parent.clone(),
            );
            if node.kind() == "record_declaration" {
                builder.unsupported.push(super::model::UnsupportedNote {
                    construct: Construct::Record,
                    span: span(&builder.path, node),
                    reason: "record components are reported as class fields".to_string(),
                });
            }
            for child in nested {
                walk_java(child, source, builder, Some(id.clone()));
            }
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                walk_java(child, source, builder, parent.clone());
            }
        }
    }
}

fn analyze_typescript_file(source: scan::ScannedSource, analysis: &mut Analysis) -> Result<()> {
    let mut parser = Parser::new();
    let language = if source.path.as_str().ends_with(".tsx") {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };
    parser
        .set_language(&language)
        .context("load the TypeScript Tree-sitter grammar")?;
    let tree = parser
        .parse(&source.content, None)
        .context("parse the TypeScript source with Tree-sitter")?;
    if tree.root_node().has_error() {
        analysis.diagnostics.push(AnalysisDiagnostic {
            kind: DiagnosticKind::ParseFailed,
            path: source.path.clone(),
            line: Some(1),
            message: "Tree-sitter reported error nodes in the TypeScript source".to_string(),
        });
        let failed = failed_file(&source, Language::TypeScript, "parse failed".to_string());
        analysis.files.extend(failed.files);
        return Ok(());
    }
    let mut builder = FileBuilder::new(source.path.clone(), Language::TypeScript);
    walk_ecmascript(tree.root_node(), &source.content, &mut builder, None);
    let file_analysis = builder.finish(&source);
    merge(analysis, file_analysis);
    Ok(())
}

fn analyze_javascript_file(source: scan::ScannedSource, analysis: &mut Analysis) -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .context("load the JavaScript Tree-sitter grammar")?;
    let tree = parser
        .parse(&source.content, None)
        .context("parse the JavaScript source with Tree-sitter")?;
    if tree.root_node().has_error() {
        analysis.diagnostics.push(AnalysisDiagnostic {
            kind: DiagnosticKind::ParseFailed,
            path: source.path.clone(),
            line: Some(1),
            message: "Tree-sitter reported error nodes in the JavaScript source".to_string(),
        });
        let failed = failed_file(&source, Language::JavaScript, "parse failed".to_string());
        analysis.files.extend(failed.files);
        return Ok(());
    }
    let mut builder = FileBuilder::new(source.path.clone(), Language::JavaScript);
    walk_ecmascript(tree.root_node(), &source.content, &mut builder, None);
    let file_analysis = builder.finish(&source);
    merge(analysis, file_analysis);
    Ok(())
}

fn walk_ecmascript<'a>(
    node: Node<'a>,
    source: &'a str,
    builder: &mut FileBuilder,
    parent: Option<UnitId>,
) {
    match node.kind() {
        "class_declaration"
        | "interface_declaration"
        | "enum_declaration"
        | "type_alias_declaration" => {
            let Some(name) = name_text(node, source, "name") else {
                return;
            };
            let type_kind = match node.kind() {
                "interface_declaration" => TypeKind::Interface,
                "enum_declaration" => TypeKind::Enum,
                "type_alias_declaration" => TypeKind::TypeAlias,
                _ => TypeKind::Class,
            };
            let mut fields = Vec::new();
            let mut methods = Vec::new();
            let mut nested = Vec::new();
            if let Some(body) = node.child_by_field_name("body") {
                let mut cursor = body.walk();
                for child in body.children(&mut cursor) {
                    match child.kind() {
                        "public_field_definition" | "property_signature" | "field_definition" => {
                            if let Some(field) = field_from_ecmascript(child, source, &builder.path)
                            {
                                fields.push(field);
                            }
                        }
                        "method_definition" | "method_signature" | "abstract_method_signature" => {
                            if let Some(method_name) = name_text(child, source, "name") {
                                let calls = calls_in(child, source, call_shape_ecmascript);
                                let visibility = ecmascript_visibility(child, source, &method_name);
                                methods.push(method_from(
                                    child,
                                    source,
                                    &builder.path,
                                    &builder.module_path,
                                    method_name,
                                    visibility,
                                    calls,
                                    matches!(type_kind, TypeKind::Interface),
                                    span(&builder.path, child),
                                ));
                            }
                        }
                        "class_declaration"
                        | "interface_declaration"
                        | "enum_declaration"
                        | "type_alias_declaration" => nested.push(child),
                        "namespace_declaration" | "module" => {
                            builder.unsupported.push(super::model::UnsupportedNote {
                                construct: Construct::Namespace,
                                span: span(&builder.path, child),
                                reason: "namespace is not expanded into units".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
            }
            let qualified = parent
                .as_ref()
                .map(|parent| {
                    format!(
                        "{}::{name}",
                        parent.0.rsplit("::").next().unwrap_or(&parent.0)
                    )
                })
                .unwrap_or_else(|| name.clone());
            let id = builder.add_type(
                name.clone(),
                qualified,
                type_kind,
                ecmascript_visibility(node, source, &name),
                span(&builder.path, node),
                fields,
                methods,
                parent.clone(),
            );
            for child in nested {
                walk_ecmascript(child, source, builder, Some(id.clone()));
            }
        }
        "function_declaration" | "generator_function_declaration" => {
            if let Some(name) = name_text(node, source, "name") {
                let calls = calls_in(node, source, call_shape_ecmascript);
                let method = method_from(
                    node,
                    source,
                    &builder.path,
                    &builder.module_path,
                    name,
                    ecmascript_visibility(node, source, ""),
                    calls,
                    false,
                    span(&builder.path, node),
                );
                builder.module.outline.push(outline_method(&method, false));
                builder.module.methods.push(method);
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                walk_ecmascript(child, source, builder, parent.clone());
            }
        }
        "lexical_declaration" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() != "variable_declarator" {
                    continue;
                }
                let Some(name) = name_text(child, source, "name") else {
                    continue;
                };
                let Some(value) = child.child_by_field_name("value") else {
                    continue;
                };
                match value.kind() {
                    "arrow_function" => {
                        let calls = calls_in(value, source, call_shape_ecmascript);
                        let method = method_from(
                            value,
                            source,
                            &builder.path,
                            &builder.module_path,
                            name,
                            Visibility::Public,
                            calls,
                            false,
                            span(&builder.path, value),
                        );
                        builder.module.outline.push(outline_method(&method, false));
                        builder.module.methods.push(method);
                    }
                    "class" if builder.language == Language::JavaScript => {
                        builder.unsupported.push(super::model::UnsupportedNote {
                            construct: Construct::Class,
                            span: span(&builder.path, value),
                            reason: "class expression is not a unit".to_string(),
                        });
                    }
                    "function_expression" | "generator_function"
                        if builder.language == Language::JavaScript =>
                    {
                        builder.unsupported.push(super::model::UnsupportedNote {
                            construct: Construct::Function,
                            span: span(&builder.path, value),
                            reason: "function expression is not a unit".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
        "class" if builder.language == Language::JavaScript => {
            builder.unsupported.push(super::model::UnsupportedNote {
                construct: Construct::Class,
                span: span(&builder.path, node),
                reason: "class expression is not a unit".to_string(),
            });
        }
        "function_expression" | "generator_function"
            if builder.language == Language::JavaScript =>
        {
            if name_text(node, source, "name").is_none() {
                builder.unsupported.push(super::model::UnsupportedNote {
                    construct: Construct::Function,
                    span: span(&builder.path, node),
                    reason: "function expression is not a unit".to_string(),
                });
            }
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                walk_ecmascript(child, source, builder, parent.clone());
            }
        }
    }
}

fn merge(analysis: &mut Analysis, file: Analysis) {
    analysis.files.extend(file.files);
    analysis.units.extend(file.units);
    analysis.diagnostics.extend(file.diagnostics);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyze(
        analyzer: &dyn SourceAnalyzer,
        language: Language,
        files: &[(&str, &str)],
    ) -> Analysis {
        let root = tempfile::tempdir().expect("temp project");
        for (path, content) in files {
            let target = root.path().join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, content).unwrap();
        }
        let request = AnalysisRequest {
            project_root: root.path().to_path_buf(),
            scope: super::super::model::SourceScope {
                languages: vec![language],
                roots: vec![std::path::PathBuf::from("src")],
                include_test_sources: false,
            },
        };
        analyzer.analyze(&request).expect("analyze")
    }

    #[test]
    fn java_extracts_class_methods_fields_visibility_and_calls() {
        let analysis = analyze(
            &JavaAnalyzer,
            Language::Java,
            &[(
                "src/com/example/Widget.java",
                "package com.example;\n\
                 public class Widget {\n\
                     private String name;\n\
                     public int area() { return 1; }\n\
                     private void helper() { area(); }\n\
                 }\n",
            )],
        );
        let widget = analysis
            .units
            .iter()
            .find(|unit| unit.name == "Widget")
            .expect("Widget unit");
        assert_eq!(widget.kind, UnitKind::Type(TypeKind::Class));
        assert_eq!(widget.visibility, Visibility::Public);
        assert!(widget.fields.iter().any(|field| field.name == "name"));
        let area = widget
            .methods
            .iter()
            .find(|method| method.name == "area")
            .expect("area");
        assert_eq!(area.visibility, Visibility::Public);
        let helper = widget
            .methods
            .iter()
            .find(|method| method.name == "helper")
            .expect("helper");
        assert_eq!(helper.visibility, Visibility::Private);
        assert!(helper.calls.iter().any(|call| call.callee == "area"));
    }

    #[test]
    fn typescript_extracts_interface_and_module_function() {
        let analysis = analyze(
            &TypeScriptAnalyzer,
            Language::TypeScript,
            &[(
                "src/widget.ts",
                "export interface Drawable { draw(): void; }\n\
                 export function makeWidget(): Drawable { return { draw() {} }; }\n",
            )],
        );
        let drawable = analysis
            .units
            .iter()
            .find(|unit| unit.name == "Drawable")
            .expect("Drawable unit");
        assert_eq!(drawable.kind, UnitKind::Type(TypeKind::Interface));
        assert!(drawable.methods.iter().any(|method| method.name == "draw"));
        let module = analysis
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Module)
            .expect("module unit");
        assert!(module.outline.iter().any(|item| item.name == "makeWidget"));
    }

    #[test]
    fn typescript_tsx_selects_the_tsx_grammar() {
        let analysis = analyze(
            &TypeScriptAnalyzer,
            Language::TypeScript,
            &[(
                "src/view.tsx",
                "export function View(): JSX.Element { return <div>ok</div>; }\n",
            )],
        );
        assert_eq!(analysis.files.len(), 1);
        assert_eq!(analysis.files[0].status, FileStatus::Analyzed);
        assert!(
            analysis
                .units
                .iter()
                .any(|unit| unit.kind == UnitKind::Module)
        );
    }

    #[test]
    fn javascript_extracts_class_methods_fields_functions_and_arrows() {
        let analysis = analyze(
            &JavaScriptAnalyzer,
            Language::JavaScript,
            &[(
                "src/widget.js",
                "export class Widget {\n\
                     count = 0;\n\
                     area() { return this.count; }\n\
                 }\n\
                 export function makeWidget() { return new Widget(); }\n\
                 export const clone = (value) => value;\n",
            )],
        );
        let widget = analysis
            .units
            .iter()
            .find(|unit| unit.name == "Widget")
            .expect("Widget unit");
        assert_eq!(widget.kind, UnitKind::Type(TypeKind::Class));
        assert_eq!(widget.language, Language::JavaScript);
        assert!(widget.fields.iter().any(|field| field.name == "count"));
        assert!(widget.methods.iter().any(|method| method.name == "area"));
        let module = analysis
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Module)
            .expect("module unit");
        assert!(module.outline.iter().any(|item| item.name == "makeWidget"));
        assert!(module.outline.iter().any(|item| item.name == "clone"));
        let make_widget = module
            .methods
            .iter()
            .find(|method| method.name == "makeWidget")
            .expect("makeWidget");
        assert!(make_widget.calls.iter().any(|call| call.callee == "Widget"));
    }

    #[test]
    fn javascript_module_extensions_trim_the_module_path() {
        for path in ["src/queue.js", "src/queue.mjs", "src/queue.cjs"] {
            let analysis = analyze(
                &JavaScriptAnalyzer,
                Language::JavaScript,
                &[(path, "function drain() {}\n")],
            );
            let module = analysis
                .units
                .iter()
                .find(|unit| unit.kind == UnitKind::Module)
                .expect("module unit");
            assert_eq!(module.name, path);
            assert_eq!(module.module_path, "src/queue");
            assert!(module.outline.iter().any(|item| item.name == "drain"));
        }
    }

    #[test]
    fn javascript_class_expression_is_unsupported_and_not_a_unit() {
        let analysis = analyze(
            &JavaScriptAnalyzer,
            Language::JavaScript,
            &[(
                "src/widget.js",
                "const Widget = class { area() { return 1; } };\n",
            )],
        );
        let module = analysis
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Module)
            .expect("module unit");
        assert!(
            module
                .unsupported
                .iter()
                .any(|note| note.construct == Construct::Class)
        );
        assert!(!analysis.units.iter().any(|unit| unit.name == "Widget"));
    }

    #[test]
    fn malformed_javascript_is_a_failed_file_without_units() {
        let analysis = analyze(
            &JavaScriptAnalyzer,
            Language::JavaScript,
            &[("src/Broken.js", "export function broken( {\n")],
        );
        assert_eq!(analysis.files.len(), 1);
        assert_eq!(analysis.files[0].status, FileStatus::Failed);
        assert_eq!(analysis.files[0].language, Language::JavaScript);
        assert!(analysis.units.is_empty());
        assert!(!analysis.diagnostics.is_empty());
    }

    #[test]
    fn malformed_java_is_a_failed_file_without_units() {
        let analysis = analyze(
            &JavaAnalyzer,
            Language::Java,
            &[("src/Broken.java", "public class Broken {\n")],
        );
        assert_eq!(analysis.files.len(), 1);
        assert_eq!(analysis.files[0].status, FileStatus::Failed);
        assert!(analysis.units.is_empty());
        assert!(!analysis.diagnostics.is_empty());
    }
}
