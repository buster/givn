//! Shared run-declaration fixture helpers.
//!
//! Once the run-declaration gate is sharp, every fixture project that
//! reaches `givn check review` or `givn archive` must carry a complete run
//! declaration. These helpers rewrite the top-level `run:` key in the
//! current fixture project's `givn/commands.yaml` while preserving all other
//! content, so they compose with `verify commands are no-ops` in either
//! order. The declared commands are never executed by the gates — validation
//! only reads them.

use cucumber::given;

use super::GivnWorld;
use crate::common;

/// The run declaration block for a product type, as YAML text.
fn declaration_text(product_type: &str) -> String {
    common::complete_run_declaration(product_type)
}

/// Rewrite (or append) the top-level `run:` key in the fixture project's
/// commands file, preserving every other line.
pub(crate) fn declare_run(world: &mut GivnWorld, product_type: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let replacement = declaration_text(product_type);
    let updated = common::replace_top_level_key(&content, "run", &replacement);
    std::fs::write(&path, updated).expect("write commands.yaml");
}

/// Remove the top-level `run:` key (and its body) from the commands file.
pub(crate) fn strip_run(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = common::remove_top_level_key(&content, "run");
    std::fs::write(&path, updated).expect("write commands.yaml");
}

#[given("the project declares a complete run declaration")]
fn declares_complete_run_declaration(world: &mut GivnWorld) {
    declare_run(world, "cli");
}

#[given(regex = r#"^the project declares a complete run declaration for a (\w+) product$"#)]
fn declares_complete_run_declaration_for_product(world: &mut GivnWorld, product: String) {
    declare_run(world, &product);
}

#[given("the commands file has no run declaration")]
fn commands_file_has_no_run_declaration(world: &mut GivnWorld) {
    strip_run(world);
}
