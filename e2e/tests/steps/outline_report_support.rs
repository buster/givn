//! Fixture helpers for the `assess-code-structure` scenario.
//!
//! Writes production Rust sources into a temporary project and enables the
//! `outline` addon. The scenario then runs `givn outline report` and asserts on
//! what the binary printed; nothing here analyzes the fixture in process.

use crate::harness::{self, TestEnv};

pub fn enable_outline(env: &TestEnv) {
    let out = harness::run_givn(env, &["addons", "enable", "outline"]);
    assert!(
        out.status.success(),
        "givn addons enable outline failed: {}",
        String::from_utf8_lossy(&out.stderr)
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

pub fn write_source(env: &TestEnv, relative: &str, source: &str) {
    let path = env.path(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create source parent");
    }
    std::fs::write(path, source).expect("write source fixture");
}
