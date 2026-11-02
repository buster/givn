#![allow(dead_code)]

//! Neutral data model shared by every analyzer.
//!
//! Nothing here imports `syn`. Metrics and classifications are computed above
//! this model; an adapter only supplies facts.

use std::fmt;
use std::path::PathBuf;

/// Project-relative, `/`-separated path.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepoPath(String);

impl RepoPath {
    pub fn new(path: impl Into<String>) -> Self {
        let mut value = path.into().replace('\\', "/");
        while let Some(rest) = value.strip_prefix("./") {
            value = rest.to_string();
        }
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    Java,
    TypeScript,
    JavaScript,
}

impl Language {
    pub fn label(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Java => "java",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceScope {
    pub languages: Vec<Language>,
    pub roots: Vec<PathBuf>,
    pub include_test_sources: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalysisRequest {
    pub project_root: PathBuf,
    pub scope: SourceScope,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Construct {
    Struct,
    Enum,
    Trait,
    Union,
    TypeAlias,
    InherentImpl,
    TraitImpl,
    InlineModule,
    FileModule,
    FreeFunction,
    Class,
    Interface,
    Function,
    ItemMacro,
    Record,
    Namespace,
}

impl Construct {
    pub fn label(self) -> &'static str {
        match self {
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Union => "union",
            Self::TypeAlias => "type_alias",
            Self::InherentImpl => "inherent_impl",
            Self::TraitImpl => "trait_impl",
            Self::InlineModule => "inline_module",
            Self::FileModule => "file_module",
            Self::FreeFunction => "free_function",
            Self::Class => "class",
            Self::Interface => "interface",
            Self::Function => "function",
            Self::ItemMacro => "item-macros",
            Self::Record => "record",
            Self::Namespace => "namespace",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Limitation {
    ItemMacros,
    IncludeMacro,
    NonUtf8File,
    PythonDynamicCalls,
    PythonTypeInference,
    JavaPackageScope,
    JavaRecordsMappedToClass,
    JavaAnonymousClasses,
    TypeScriptDialectSelection,
    TypeScriptTypeOnlyDeclarations,
    TypeScriptNamespaces,
    CrossLanguageNameResolution,
    StructuralCallOrderNotRuntimeTrace,
    UncertainCallEdges,
}

impl Limitation {
    pub fn label(self) -> &'static str {
        match self {
            Self::ItemMacros => "item-macros",
            Self::IncludeMacro => "include-macro",
            Self::NonUtf8File => "non-utf8-file",
            Self::PythonDynamicCalls => "python-dynamic-calls",
            Self::PythonTypeInference => "python-type-inference",
            Self::JavaPackageScope => "java-package-private-is-crate-scope",
            Self::JavaRecordsMappedToClass => "java-records-are-reported-as-classes",
            Self::JavaAnonymousClasses => "java-anonymous-classes-are-not-outlined",
            Self::TypeScriptDialectSelection => "typescript-tsx-dialect-selected-by-extension",
            Self::TypeScriptTypeOnlyDeclarations => "typescript-declaration-files-are-excluded",
            Self::TypeScriptNamespaces => "typescript-namespaces-are-not-expanded",
            Self::CrossLanguageNameResolution => "calls-never-resolve-across-languages",
            Self::StructuralCallOrderNotRuntimeTrace => {
                "structural call order is not a complete runtime trace"
            }
            Self::UncertainCallEdges => "uncertain or omitted call edges limit conclusions",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalyzerCapabilities {
    pub languages: Vec<Language>,
    pub supported_constructs: Vec<Construct>,
    pub limitations: Vec<Limitation>,
    pub resolution_labels: Vec<Resolution>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    SameFile,
    ProjectOtherFile,
    NameOnly,
    Ambiguous,
    Dynamic,
    External,
}

impl Resolution {
    pub fn label(self) -> &'static str {
        match self {
            Self::SameFile => "same_file",
            Self::ProjectOtherFile => "project_other_file",
            Self::NameOnly => "name_only",
            Self::Ambiguous => "ambiguous",
            Self::Dynamic => "dynamic",
            Self::External => "external",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineKind {
    Code,
    Comment,
    Blank,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileStatus {
    Analyzed,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalyzedFile {
    pub path: RepoPath,
    pub language: Language,
    pub physical_lines: u32,
    pub lines: Vec<LineKind>,
    pub status: FileStatus,
    pub unit_ids: Vec<UnitId>,
    /// Spans excluded from production accounting, such as `#[cfg(test)]`
    /// items in an otherwise production file.
    pub excluded_spans: Vec<SourceSpan>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    FileUnreadable,
    ParseFailed,
    UnsupportedConstruct,
    AmbiguousType,
    AnalyzerUnavailable,
}

impl DiagnosticKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::FileUnreadable => "file-unreadable",
            Self::ParseFailed => "parse-failed",
            Self::UnsupportedConstruct => "unsupported-construct",
            Self::AmbiguousType => "ambiguous-type",
            Self::AnalyzerUnavailable => "analyzer-unavailable",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalysisDiagnostic {
    pub kind: DiagnosticKind,
    pub path: RepoPath,
    pub line: Option<u32>,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Analysis {
    pub files: Vec<AnalyzedFile>,
    pub units: Vec<SourceUnit>,
    pub diagnostics: Vec<AnalysisDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId(pub String);

impl fmt::Display for UnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub type MethodId = String;
pub type FieldId = String;
pub type CallId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeKind {
    Struct,
    Enum,
    Trait,
    Union,
    TypeAlias,
    Class,
    Interface,
}

impl TypeKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Union => "union",
            Self::TypeAlias => "type_alias",
            Self::Class => "class",
            Self::Interface => "interface",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    Type(TypeKind),
    Module,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitOrigin {
    Production,
    Test,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Crate,
    Restricted,
    Private,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRelation {
    ImplementsUnit(UnitId),
    ImplementedBy(UnitId),
    ContainedIn(UnitId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsupportedNote {
    pub construct: Construct,
    pub span: SourceSpan,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceSpan {
    pub file: RepoPath,
    pub start_line: u32,
    pub end_line: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceUnit {
    pub id: UnitId,
    pub language: Language,
    pub kind: UnitKind,
    pub name: String,
    pub module_path: String,
    pub declaration_file: RepoPath,
    pub files: Vec<RepoPath>,
    pub spans: Vec<SourceSpan>,
    pub visibility: Visibility,
    pub origin: UnitOrigin,
    pub methods: Vec<Method>,
    pub fields: Vec<Field>,
    pub relations: Vec<TypeRelation>,
    pub outline: Vec<OutlineItem>,
    pub complete: bool,
    pub unsupported: Vec<UnsupportedNote>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub id: MethodId,
    pub name: String,
    pub visibility: Visibility,
    pub signature: Signature,
    pub span: SourceSpan,
    pub flow: Flow,
    pub field_accesses: Vec<FieldId>,
    pub calls: Vec<Call>,
    pub is_trait_default: bool,
    pub origin: UnitOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub id: FieldId,
    pub name: String,
    pub visibility: Visibility,
    pub span: SourceSpan,
    pub type_label: String,
    pub origin: UnitOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub order: u32,
    pub callee: String,
    pub shape: CallShape,
    pub resolution: Resolution,
    pub target: Option<MethodId>,
}

/// How a call was written, independent of any language's syntax.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallShape {
    /// `foo(...)` with an optional path before the callee (`Type::foo`,
    /// `crate::module::foo`).
    Function { qualifier: Option<String> },
    /// `receiver.foo(...)` with the receiver evidence the adapter can prove.
    Method { receiver: ReceiverEvidence },
}

/// What an adapter knows about a method call's receiver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReceiverEvidence {
    /// `self` or `cls`; the receiver is the current unit.
    SelfUnit,
    /// A named type or owner in the receiver position.
    Qualifier(String),
    /// An arbitrary expression, field chain, or inferred value.
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutlineKind {
    Type,
    Method,
    TraitMethod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutlineItem {
    pub id: MethodId,
    pub kind: OutlineKind,
    pub name: String,
    pub visibility: Visibility,
    pub signature: String,
    pub order: u32,
    pub calls: Vec<Call>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Flow {
    Block(Vec<Flow>),
    Branch(Box<Flow>),
    Loop(Box<Flow>),
    Match { arms: u32, bodies: Vec<Flow> },
    Try(Box<Flow>),
    ShortCircuit(Box<Flow>, Box<Flow>),
    Return(Option<Box<Flow>>),
    Call(CallId),
    Unavailable,
}
