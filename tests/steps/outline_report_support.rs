//! Fixture helpers for the outline report scenarios.
//!
//! Helpers here write production Rust sources into a temporary project and
//! enable the `outline` addon; they contain no step bindings.

use crate::common::{self, TestEnv};
use givn::outline::model::{
    Analysis, AnalyzedFile, FileStatus, LineKind, OutlineItem, OutlineKind, RepoPath, SourceSpan,
    SourceUnit, TypeKind, TypeRelation, UnitId, UnitKind, UnitOrigin, Visibility,
};
use givn::outline::{self, AnalysisRequest, SourceAnalyzer, SourceScope, model::Language};
use std::path::PathBuf;

pub fn enable_outline(env: &TestEnv) {
    let out = common::run_givn(env, &["addons", "enable", "outline"]);
    assert!(
        out.status.success(),
        "givn addons enable outline failed: {}",
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
            languages: vec![Language::Rust],
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = outline::analyzer::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = outline::analyze(&request, analyzer.as_ref()).expect("analyze corpus");
    let assessment = outline::assess(&analysis, capabilities);
    (
        assessment.units.len(),
        assessment
            .units
            .iter()
            .filter(|unit| !unit.classification.label().is_empty())
            .count(),
        assessment.units.len(),
    )
}

pub fn write_long_method(env: &TestEnv) {
    let mut source = String::from(
        "pub struct Long {\n    value: u64,\n}\n\nimpl Long {\n    pub fn long(&self) -> u64 {\n",
    );
    for _ in 0..110 {
        source.push_str("        let _ = self.value;\n");
    }
    source.push_str("        self.value\n    }\n}\n");
    write_source(env, "src/long_method.rs", &source);
}

pub fn write_large_module(env: &TestEnv) {
    let mut source = String::new();
    for index in 0..21 {
        source.push_str(&format!("pub fn function_{index}() -> u64 {{\n"));
        for _ in 0..50 {
            source.push_str("    let _value = 1;\n");
        }
        source.push_str("    0\n}\n\n");
    }
    write_source(env, "src/large_module.rs", &source);
}

pub fn write_inspect_band_type(env: &TestEnv) {
    let mut source =
        String::from("pub struct Boundary {\n    a: u64,\n    b: u64,\n}\n\nimpl Boundary {\n");
    for (method, fields) in [
        ("first", &["a", "b"][..]),
        ("second", &["a"][..]),
        ("third", &["b"][..]),
    ] {
        source.push_str(&format!("    pub fn {method}(&self) -> u64 {{\n"));
        for _ in 0..170 {
            source.push_str(&format!("        let _ = self.{};\n", fields[0]));
        }
        for field in fields.iter().skip(1) {
            source.push_str(&format!("        let _ = self.{field};\n"));
        }
        source.push_str(&format!("        self.{}\n    }}\n", fields[0]));
    }
    source.push_str("}\n");
    write_source(env, "src/inspect_band.rs", &source);
}

pub fn write_many_public_methods(env: &TestEnv) {
    let mut source = String::from("pub struct ManyPublic;\nimpl ManyPublic {\n");
    for index in 0..51 {
        source.push_str(&format!(
            "    pub fn method_{index}(&self) -> u64 {{ {index} }}\n"
        ));
    }
    source.push_str("}\n");
    write_source(env, "src/many_public.rs", &source);
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
    write_god_type_at(env, name, "src/god_type.rs");
}

pub fn write_god_type_at(env: &TestEnv, name: &str, relative: &str) {
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
    write_source(env, relative, &source);
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

pub fn assess_json(env: &TestEnv) -> String {
    let request = AnalysisRequest {
        project_root: env.root(),
        scope: SourceScope {
            languages: vec![Language::Rust],
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = outline::analyzer::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = outline::analyze(&request, analyzer.as_ref()).expect("analyze fixture");
    let assessment = outline::assess(&analysis, capabilities);
    outline::report::render_json(&assessment, &request.scope)
}

pub fn assess_report(env: &TestEnv) -> String {
    let request = AnalysisRequest {
        project_root: env.root(),
        scope: SourceScope {
            languages: vec![Language::Rust],
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = outline::analyzer::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = outline::analyze(&request, analyzer.as_ref()).expect("analyze fixture");
    let assessment = outline::assess(&analysis, capabilities);
    outline::report::render_human(&assessment)
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
    capabilities: givn::outline::model::AnalyzerCapabilities,
}

impl SourceAnalyzer for FakeAnalyzer {
    fn capabilities(&self) -> givn::outline::model::AnalyzerCapabilities {
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
    let type_id = UnitId("type:src/small.rs::Small".to_string());
    let file = AnalyzedFile {
        language: Language::Rust,
        path: path.clone(),
        physical_lines: 1,
        lines: vec![LineKind::Code],
        status: FileStatus::Analyzed,
        unit_ids: vec![type_id.clone(), module_id.clone()],
        excluded_spans: Vec::new(),
    };
    let type_unit = SourceUnit {
        language: Language::Rust,
        id: type_id.clone(),
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
        language: Language::Rust,
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
        outline: vec![OutlineItem {
            id: type_id.to_string(),
            kind: OutlineKind::Type,
            name: "Small".to_string(),
            visibility: Visibility::Public,
            signature: String::new(),
            order: 1,
            calls: Vec::new(),
        }],
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
            languages: vec![Language::Rust],
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let syn = outline::analyzer::rust_analyzer();
    let capabilities = syn.capabilities();
    let syn_analysis = outline::analyze(&request, syn.as_ref()).expect("syn analysis");
    let fake = FakeAnalyzer {
        analysis: fake_analysis(),
        capabilities: capabilities.clone(),
    };
    let fake_analysis = outline::analyze(&request, &fake).expect("fake analysis");
    assert_eq!(
        syn_analysis, fake_analysis,
        "the fake and syn analyses must be the same neutral model"
    );
    let syn_report =
        outline::report::render_human(&outline::assess(&syn_analysis, capabilities.clone()));
    let fake_report = outline::report::render_human(&outline::assess(&fake_analysis, capabilities));
    (syn_report, fake_report)
}

pub fn write_python_class(env: &TestEnv) {
    write_source(
        env,
        "src/tracker.py",
        "class OrderBook:\n    \"\"\"Track incoming orders.\"\"\"\n\n    def __init__(self) -> None:\n        self.orders: list[str] = []\n\n    def add(self, order: str) -> int:\n        self.orders.append(order)\n        return len(self.orders)\n\n    def fill(self, order: str) -> int:\n        return self.add(order)\n",
    );
}

pub fn write_cross_component_calls(env: &TestEnv) {
    write_source(
        env,
        "src/component.rs",
        "pub fn normalize() -> u64 {\n    helper()\n}\n\nfn helper() -> u64 {\n    1\n}\n\npub struct Engine {}\n\nimpl Engine {\n    pub fn build(&self) -> u64 {\n        normalize()\n    }\n}\n",
    );
}

pub fn write_python_module(env: &TestEnv) {
    write_source(
        env,
        "src/store.py",
        "def load(path: str) -> str:\n    return path\n\n\ndef save(path: str, value: str) -> int:\n    return len(value)\n",
    );
}

pub fn write_python3_stub(env: &TestEnv) {
    let stub_dir = env.path(".python3-stub");
    std::fs::create_dir_all(&stub_dir).expect("create python3 stub dir");
    let marker = stub_dir.join("invoked");
    let script = format!(
        "#!/bin/sh\nprintf invoked > '{}'\nexit 1\n",
        marker.display()
    );
    let path = stub_dir.join("python3");
    std::fs::write(&path, script).expect("write python3 stub");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("make python3 stub executable");
    }
}

pub fn python3_stub_invoked(env: &TestEnv) -> bool {
    env.path(".python3-stub/invoked").exists()
}
pub fn write_unknown_receiver_call(env: &TestEnv) {
    write_source(
        env,
        "src/tier.rs",
        "pub struct TierReasoning;\n\nimpl TierReasoning {\n    pub fn check(&self, value: &ExternalValue) -> bool {\n        value.is_empty()\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.missing()\n    }\n}\n\npub struct First;\n\nimpl First {\n    pub fn missing(&self) {}\n}\n\npub struct Second;\n\nimpl Second {\n    pub fn missing(&self) {}\n}\n",
    );
}

pub fn write_self_receiver_call(env: &TestEnv) {
    write_source(
        env,
        "src/engine.rs",
        "pub struct Engine {\n    value: u64,\n}\n\nimpl Engine {\n    pub fn build(&self) -> u64 {\n        self.validate();\n        self.check()\n    }\n\n    pub fn validate(&self) -> u64 {\n        self.value\n    }\n\n    pub fn check(&self) -> u64 {\n        self.value\n    }\n}\n",
    );
}

pub fn write_named_receiver_call(env: &TestEnv) {
    write_source(
        env,
        "src/line.rs",
        "pub struct Line;\n\npub struct Consumer;\n\nimpl Line {\n    pub fn from(value: u64) -> Self {\n        let _ = value;\n        Self\n    }\n\n    pub fn other(&self) {}\n}\n\nimpl Consumer {\n    pub fn make(&self) -> Line {\n        Line::from(1)\n    }\n}\n",
    );
}

pub fn write_ambiguous_call(env: &TestEnv) {
    write_source(
        env,
        "src/refresh.rs",
        "pub struct First;\n\nimpl First {\n    pub fn refresh(&self) {}\n}\n\npub struct Second;\n\nimpl Second {\n    pub fn refresh(&self) {}\n}\n\npub struct Caller;\n\nimpl Caller {\n    pub fn poll(&self, value: &ExternalValue) {\n        value.refresh();\n    }\n}\n",
    );
}

pub fn write_contradictory_qualifier(env: &TestEnv) {
    write_source(
        env,
        "src/external.rs",
        "pub struct Error;\n\nimpl Error {\n    pub fn from(value: u64) -> Self {\n        let _ = value;\n        Self\n    }\n}\n\npub struct Caller;\n\nimpl Caller {\n    pub fn make(&self) -> Error {\n        Line::from(1)\n    }\n}\n",
    );
}

pub fn write_tcc_god_type(env: &TestEnv) {
    let mut source = String::from("pub struct Cohesion {\n    a: u64,\n    b: u64,\n}\n\nimpl Cohesion {\n");
    for (method, field, other) in [("first", "a", "second"), ("second", "b", "first")] {
        source.push_str(&format!("    pub fn {method}(&self) -> u64 {{\n"));
        for _ in 0..255 {
            source.push_str(&format!("        let _ = self.{field};\n"));
        }
        if method == "first" {
            source.push_str(&format!("        self.{other}();\n"));
        }
        source.push_str(&format!("        self.{field}\n    }}\n"));
    }
    source.push_str("}\n");
    write_source(env, "src/cohesion_evidence.rs", &source);
}

pub fn write_complexity_method(env: &TestEnv) {
    let mut source = String::from("pub struct Long;\n\nimpl Long {\n    pub fn branchy(&self, value: u64) -> u64 {\n");
    for index in 0..60 {
        source.push_str(&format!(
            "        if value == {index} {{ return {index}; }}\n"
        ));
    }
    source.push_str("        value\n    }\n}\n");
    write_source(env, "src/complex.rs", &source);
}

pub fn write_module_with_types(env: &TestEnv) {
    write_source(
        env,
        "src/module.rs",
        "pub struct First;\npub struct Second;\npub struct Third;\n\npub fn load() -> u64 {\n    1\n}\n",
    );
}

pub fn write_interleaved_module(env: &TestEnv) {
    write_source(
        env,
        "src/interleave.rs",
        "pub fn alpha() -> u64 {\n    1\n}\n\npub struct Beta;\n\npub fn gamma() -> u64 {\n    2\n}\n\npub struct Delta;\n",
    );
}

pub fn write_python_classes_module(env: &TestEnv) {
    write_source(
        env,
        "src/ledger.py",
        "class OrderBook:\n    def add(self, order: str) -> int:\n        return len(order)\n\n\nclass Ledger:\n    def record(self, value: str) -> int:\n        return len(value)\n\n\ndef load(path: str) -> str:\n    return path\n",
    );
}

pub fn write_cross_file_impl(env: &TestEnv) {
    write_source(
        env,
        "src/engine_core.rs",
        "pub struct Engine;\n\nimpl Engine {\n    pub fn check(&self) -> bool {\n        self.validate()\n    }\n}\n",
    );
    write_source(
        env,
        "src/engine_parts.rs",
        "impl Engine {\n    pub fn validate(&self) -> bool {\n        self.extra();\n        true\n    }\n\n    pub fn extra(&self) {}\n}\n",
    );
}

pub fn write_resolution_report(env: &TestEnv) {
    write_source(
        env,
        "src/engine_view.rs",
        "pub struct Engine;\n\nimpl Engine {\n    pub fn build(&self, value: &ExternalValue) -> bool {\n        self.validate();\n        normalize();\n        value.is_empty()\n    }\n\n    pub fn validate(&self) -> bool {\n        true\n    }\n\n    pub fn is_empty(&self) -> bool {\n        false\n    }\n}\n",
    );
    write_source(
        env,
        "src/normalizer.rs",
        "pub fn normalize() -> bool {\n    true\n}\n",
    );
}

pub fn write_inline_test_module(env: &TestEnv) {
    write_source(
        env,
        "src/counted.rs",
        "pub struct Counted {\n    value: u64,\n}\n\nimpl Counted {\n    pub fn value(&self) -> u64 {\n        self.value\n    }\n}\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn reads_value() {\n        assert_eq!(Counted { value: 1 }.value(), 1);\n    }\n}\n",
    );
}

