//! Fixture helpers for the readability report scenarios.
//!
//! Helpers here write production Rust sources into a temporary project and
//! enable the `readability` addon; they contain no step bindings.

use crate::common::{self, TestEnv};

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

pub fn write_source(env: &TestEnv, relative: &str, source: &str) {
    let path = env.path(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create source parent");
    }
    std::fs::write(path, source).expect("write source fixture");
}
