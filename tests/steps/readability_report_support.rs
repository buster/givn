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

pub fn write_source(env: &TestEnv, relative: &str, source: &str) {
    let path = env.path(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create source parent");
    }
    std::fs::write(path, source).expect("write source fixture");
}
