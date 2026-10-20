//! Fixture helpers for the readability report scenarios.
//!
//! Helpers here write production Rust sources into a temporary project and
//! enable the `readability` addon; they contain no step bindings.

use crate::common::{self, TestEnv};
use givn::readability::{self, AnalysisRequest, SourceScope, model::Language};
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
