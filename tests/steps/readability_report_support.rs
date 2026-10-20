//! Fixture helpers for the readability report scenarios.
//!
//! Helpers here write production Rust sources into a temporary project and
//! enable the `readability` addon; they contain no step bindings.

use crate::common::{self, TestEnv};
use givn::readability::model::{
    Analysis, AnalyzedFile, FileStatus, LineKind, RepoPath, SourceSpan, SourceUnit, TypeKind,
    TypeRelation, UnitId, UnitKind, UnitOrigin, Visibility,
};
use givn::readability::{self, AnalysisRequest, SourceAnalyzer, SourceScope, model::Language};
use std::path::PathBuf;

pub fn enable_readability(env: &TestEnv) {
    let out = common::run_givn(env, &["addons", "enable", "readability"]);
    assert!(
        out.status.success(),
        "givn addons enable readability failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

pub fn write_data_only_enum(env: &TestEnv, name: &str, variants: usize) {
    let mut source = format!("pub enum {name} {{\n");
    for index in 0..variants {
        source.push_str(&format!("    Variant{index},\n"));
    }
    source.push_str("}\n\n");
    source.push_str(&format!(
        "impl {name} {{\n    pub fn variant_count(&self) -> usize {{\n        {variants}\n    }}\n}}\n"
    ));
    write_source(env, "src/big_enum.rs", &source);
}

pub fn write_large_trait(env: &TestEnv, name: &str, methods: usize) {
    let mut source = format!("pub trait {name} {{\n");
    for index in 0..methods {
        source.push_str(&format!("    fn method_{index}(&self) -> usize;\n"));
    }
    source.push_str("}\n");
    write_source(env, "src/big_trait.rs", &source);
}

pub fn write_cohesive_type(env: &TestEnv, name: &str, lines_per_method: usize) {
    let mut source = format!("pub struct {name} {{\n    value: u64,\n}}\n\n");
    source.push_str(&format!("impl {name} {{\n"));
    for method in ["first", "second"] {
        source.push_str(&format!("    pub fn {method}(&self) -> u64 {{\n"));
        for _ in 0..lines_per_method {
            source.push_str("        let _ = self.value;\n");
        }
        source.push_str("        self.value\n    }\n");
    }
    source.push_str("}\n");
    write_source(env, "src/cohesive.rs", &source);
}

pub fn write_conditional_unit(env: &TestEnv) {
    write_source(
        env,
        "src/conditional.rs",
        "pub struct Conditional { value: u64 }\nimpl Conditional {\n    #[cfg(unix)]\n    pub fn poll_unix(&self) -> u64 { self.value }\n    #[cfg(windows)]\n    pub fn poll_windows(&self) -> u64 { self.value }\n}\n",
    );
}

pub fn write_fixture_corpus(env: &TestEnv) {
    write_god_type(env, "GodType");
    write_data_only_enum(env, "BigEnum", 520);
    write_large_trait(env, "BigTrait", 520);
    write_cohesive_type(env, "Cohesive", 260);
    write_item_macro(env);
    write_source(env, "src/malformed.rs", "pub struct Broken {\n");
}

pub fn assessment_counts(env: &TestEnv) -> (usize, usize, usize) {
    let request = AnalysisRequest {
        project_root: env.root(),
        scope: SourceScope {
            language: Language::Rust,
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = readability::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = readability::analyze(&request, analyzer.as_ref()).expect("analyze corpus");
    let assessment = readability::assess(&analysis, capabilities);
    (
        assessment.units.len(),
        assessment
            .units
            .iter()
            .filter(|unit| !unit.classification.label().is_empty())
            .count(),
        assessment
            .units
            .iter()
            .filter(|unit| !unit.action.label().is_empty())
            .count(),
    )
}

pub fn write_small_unit(env: &TestEnv) {
    write_source(env, "src/small.rs", "pub struct Small;\n");
}

pub fn write_item_macro(env: &TestEnv) {
    write_source(
        env,
        "src/macro_items.rs",
        "macro_rules! make_items { () => {} }\nmake_items!();\npub struct Real { pub value: u64 }\n",
    );
}

pub fn write_god_type(env: &TestEnv, name: &str) {
    let mut source = format!(
        "pub struct {name} {{\n    a: u64,\n    b: u64,\n    c: u64,\n}}\n\nimpl {name} {{\n"
    );
    for (method, field) in [("first", "a"), ("second", "b"), ("third", "c")] {
        source.push_str(&format!("    pub fn {method}(&self) -> u64 {{\n"));
        for _ in 0..168 {
            source.push_str(&format!("        let _ = self.{field};\n"));
        }
        source.push_str(&format!("        self.{field}\n    }}\n"));
    }
    source.push_str("}\n");
    write_source(env, "src/god_type.rs", &source);
}

pub fn write_boundary_type(env: &TestEnv, total_code_lines: usize) {
    let mut source;
    if total_code_lines == 501 {
        source = String::from("pub struct Boundary {\n    value: u64,\n}\n\nimpl Boundary {\n");
        for index in 0..2 {
            source.push_str(&format!("    pub fn method_{index}(&self) -> u64 {{\n"));
            for _ in 0..245 {
                source.push_str("        let _ = self.value;\n");
            }
            source.push_str("        self.value\n    }\n");
        }
    } else {
        let fields = total_code_lines - 12;
        source = String::from("pub struct Boundary {\n    value: u64,\n");
        for index in 0..fields.saturating_sub(1) {
            source.push_str(&format!("    field_{index}: u64,\n"));
        }
        source.push_str("}\n\nimpl Boundary {\n");
        for index in 0..2 {
            source.push_str(&format!(
                "    pub fn method_{index}(&self) -> u64 {{\n        let _ = self.value;\n        self.value\n    }}\n"
            ));
        }
    }
    source.push_str("}\n");
    write_source(env, "src/boundary.rs", &source);
}

pub fn assess_report(env: &TestEnv) -> String {
    let request = AnalysisRequest {
        project_root: env.root(),
        scope: SourceScope {
            language: Language::Rust,
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = readability::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = readability::analyze(&request, analyzer.as_ref()).expect("analyze fixture");
    let assessment = readability::assess(&analysis, capabilities);
    readability::report::render_human(&assessment)
}

pub fn write_source(env: &TestEnv, relative: &str, source: &str) {
    let path = env.path(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create source parent");
    }
    std::fs::write(path, source).expect("write source fixture");
}

struct FakeAnalyzer {
    analysis: Analysis,
    capabilities: givn::readability::model::AnalyzerCapabilities,
}

impl SourceAnalyzer for FakeAnalyzer {
    fn capabilities(&self) -> givn::readability::model::AnalyzerCapabilities {
        self.capabilities.clone()
    }

    fn analyze(&self, _request: &AnalysisRequest) -> anyhow::Result<Analysis> {
        Ok(self.analysis.clone())
    }
}

fn fake_analysis() -> Analysis {
    let path = RepoPath::new("src/small.rs");
    let span = SourceSpan {
        file: path.clone(),
        start_line: 1,
        end_line: 1,
    };
    let module_id = UnitId("module:src/small.rs".to_string());
    let type_id = UnitId("type:small::Small".to_string());
    let file = AnalyzedFile {
        path: path.clone(),
        physical_lines: 1,
        lines: vec![LineKind::Code],
        status: FileStatus::Analyzed,
        unit_ids: vec![type_id.clone(), module_id.clone()],
    };
    let type_unit = SourceUnit {
        id: type_id,
        kind: UnitKind::Type(TypeKind::Struct),
        name: "Small".to_string(),
        module_path: "small".to_string(),
        declaration_file: path.clone(),
        files: vec![path.clone()],
        spans: vec![span.clone()],
        visibility: Visibility::Public,
        origin: UnitOrigin::Production,
        methods: Vec::new(),
        fields: Vec::new(),
        relations: vec![TypeRelation::ContainedIn(module_id.clone())],
        outline: Vec::new(),
        complete: true,
        unsupported: Vec::new(),
    };
    let module = SourceUnit {
        id: module_id,
        kind: UnitKind::Module,
        name: "src/small.rs".to_string(),
        module_path: "small".to_string(),
        declaration_file: path.clone(),
        files: vec![path],
        spans: vec![span],
        visibility: Visibility::Private,
        origin: UnitOrigin::Production,
        methods: Vec::new(),
        fields: Vec::new(),
        relations: Vec::new(),
        outline: Vec::new(),
        complete: true,
        unsupported: Vec::new(),
    };
    Analysis {
        files: vec![file],
        units: vec![module, type_unit],
        diagnostics: Vec::new(),
    }
}

pub fn engine_reports(env: &TestEnv) -> (String, String) {
    let request = AnalysisRequest {
        project_root: env.root(),
        scope: SourceScope {
            language: Language::Rust,
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let syn = readability::rust_analyzer();
    let capabilities = syn.capabilities();
    let syn_analysis = readability::analyze(&request, syn.as_ref()).expect("syn analysis");
    let fake = FakeAnalyzer {
        analysis: fake_analysis(),
        capabilities: capabilities.clone(),
    };
    let fake_analysis = readability::analyze(&request, &fake).expect("fake analysis");
    assert_eq!(
        syn_analysis, fake_analysis,
        "the fake and syn analyses must be the same neutral model"
    );
    let syn_report = readability::report::render_human(&readability::assess(
        &syn_analysis,
        capabilities.clone(),
    ));
    let fake_report =
        readability::report::render_human(&readability::assess(&fake_analysis, capabilities));
    (syn_report, fake_report)
}
