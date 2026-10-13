//! E2E step definitions for the `instruction-extension` capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess: the
//! extension is project-owned configuration, so the fixture writes it and the
//! request path serves it. Step bindings are added scenario by scenario at
//! RED; this module exists so the capability owns exactly one e2e step file
//! (never one shared file).

use cucumber::{given, then};

use crate::steps::GivnWorld;
use crate::steps::instruction_extension_steps::write_extensions;
use crate::steps::serve_extended_rules_steps::{
    record_builtin_content, record_expected_rule, recorded_builtin_content, recorded_stdout,
    request_instruction, without_skill_route,
};

/// The Given fixture for "A developer records an extension and it governs the
/// next guidance request": the developer writes the rule into the
/// `extensions:` map of `givn/config.yaml` (no recording command exists — D2).
/// The built-in served content is captured through a real
/// `givn instructions proposal` subprocess first, so the "stay unchanged"
/// assertion compares against the same request path.
#[given(
    regex = r#"^the developer records the rule "([^"]+)" for the proposal instruction in the project's configuration$"#
)]
fn developer_records_proposal_rule(world: &mut GivnWorld, rule: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let builtin = request_instruction(env, "proposal");
    assert!(
        builtin.status.success(),
        "givn instructions proposal failed before the extension: {}",
        String::from_utf8_lossy(&builtin.stderr)
    );
    record_builtin_content(env, "proposal", &builtin.stdout);
    write_extensions(env, &format!("  proposal:\n    - {rule}\n"));
    record_expected_rule(env, "proposal", &rule);
}

/// The built-in proposal rules are unmodified by the extension: the served
/// content's pre-extension prefix is byte-identical to the built-in capture,
/// and the built-in marker still appears beside the marked section.
#[then("the built-in proposal rules stay unchanged")]
fn builtin_proposal_rules_stay_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let builtin = without_skill_route(&recorded_builtin_content(env, "proposal"));
    let served = without_skill_route(&recorded_stdout(env, "proposal"));
    assert!(
        builtin.contains("proposed by"),
        "the built-in capture does not carry the built-in proposal rules: {builtin}"
    );
    assert!(
        served.starts_with(&builtin),
        "the served content does not keep the built-in proposal rules as its \
         unchanged prefix: {served}"
    );
}
