//! E2E step definitions for the `instruction-extension` capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess: the
//! extension is project-owned configuration, so the fixture writes it and the
//! request path serves it. Step bindings are added scenario by scenario at
//! RED; this module exists so the capability owns exactly one e2e step file
//! (never one shared file).

use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::instruction_extension_steps::write_extensions;
use crate::steps::serve_extended_rules_steps::{
    marked_extension_section, record_builtin_content, record_expected_rule,
    recorded_builtin_content, recorded_stdout, request_instruction, without_skill_route,
};
use cucumber::{given, then, when};

const UPGRADE_EXTENSION_RULE: &str = "Keep proposals under 500 words";

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

#[given(
    "a committed Givn project with an active change and an extension for the proposal instruction"
)]
fn committed_project_with_proposal_extension(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let change = common::run_givn(&env, &["new", "instruction-extension"]);
    assert!(
        change.status.success(),
        "givn new instruction-extension failed: {}",
        String::from_utf8_lossy(&change.stderr)
    );
    write_extensions(
        &env,
        &format!("  proposal:\n    - {UPGRADE_EXTENSION_RULE}\n"),
    );
    record_expected_rule(&env, "proposal", UPGRADE_EXTENSION_RULE);
    // The extension is user-owned configuration; the fixture commits it so
    // the upgrade starts from a clean checkout.
    common::commit_fixture(&env);
    world.env = Some(env);
}

#[when("the consumer upgrades the project through the Givn CLI")]
fn consumer_upgrades_the_project(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.last_output = Some(common::run_givn(env, &["upgrade"]));
}

#[then("the served content carries the project's rule beside the current built-in rules")]
fn served_content_carries_rule_beside_builtin(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = crate::steps::serve_extended_rules_steps::expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let (section, _) = marked_extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing {rule:?}: {content}"
    );
    assert!(
        content.contains("proposed by"),
        "the served proposal content is missing the current built-in rules \
         beside {rule:?}: {content}"
    );
}
