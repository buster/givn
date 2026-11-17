//! Deterministic spec-discovery step definitions.
//!
//! These steps drive the compiled givn CLI as a real subprocess against
//! fixture projects whose permanent corpus uses use-case ownership. They
//! back the rewritten `spec-duplicates`, `spec-explore-permanent`,
//! `spec-explore-change`, and `spec-progress-output` capabilities — all
//! deterministic: no embeddings, no index, no model downloads.

use crate::harness::{TestEnv, make_bare_root, run_givn};
use crate::steps::GivnWorld;
use cucumber::{given, then, when};
use std::path::PathBuf;

fn stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(output.status.success(), "CLI command failed");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn env_of(world: &GivnWorld) -> &TestEnv {
    world.env.as_ref().expect("fixture project is missing")
}

fn write_feature(env: &TestEnv, rel: &str, body: &str) {
    let path: PathBuf = env.root().join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

fn write_usecase(
    env: &TestEnv,
    usecase: &str,
    capability: &str,
    actor: &str,
    goal: &str,
    scenario: &str,
) {
    let text = crate::harness::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", capability)
        .replace("<actor role>", actor)
        .replace("<the domain goal>", goal)
        .replace("<consumer action>", "perform the capability")
        .replace("<scenario title>", scenario);
    let path = env.root().join(format!("givn/specs/{usecase}/usecase.md"));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

fn duplicate_pair_feature(title_a: &str, title_b: &str) -> String {
    format!(
        "Feature: Audit fixture\n\n  Scenario: {title_a}\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n\n  Scenario: {title_b}\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n"
    )
}

#[given("a spec discovery project whose corpus contains one feature with two identical scenarios")]
fn corpus_with_exact_pair(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/audit/audit.feature",
        &duplicate_pair_feature("The thing happens", "The thing happens"),
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose corpus contains a scenario mentioning "(.*)"$"#)]
fn corpus_with_token_scenario(world: &mut GivnWorld, token: String) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/workers/work.feature",
        &format!(
            "Feature: Work\n\n  Scenario: The shovelling scenario\n    Given a workspace\n    Then the report mentions {token}\n"
        ),
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project with two use cases and a usecase\.md narrative$"#)]
fn two_usecases_with_narrative(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/diggers/digging.feature",
        "Feature: Digging\n\n  Scenario: Dig\n    Given a workspace\n    Then the pit exists\n\n  @e2e\n  Scenario: Shovel\n    Given a workspace\n    Then the pit exists\n",
    );
    write_usecase(
        &env,
        "diggers",
        "digging",
        "A builder digging holes.",
        "Dig reproducible pits.",
        "Shovel",
    );
    write_feature(
        &env,
        "givn/specs/fillers/filling.feature",
        "Feature: Filling\n\n  Scenario: Fill\n    Given a pit\n    Then the ground is level\n",
    );
    world.env = Some(env);
}

#[given(
    regex = r#"^a spec discovery project whose corpus contains a capability that invokes `givn lint`$"#
)]
fn corpus_with_lint_capability(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/gates/lint-tool.feature",
        "Feature: Lint tool\n\n  Scenario: One\n    When the user runs `givn lint`\n    And the user runs `givn lint`\n    Then the command exits 0\n",
    );
    write_usecase(
        &env,
        "gates",
        "lint-tool",
        "A checker.",
        "Check things.",
        "Lint runs",
    );
    world.env = Some(env);
}

#[given(regex = r"^an active change whose proposal describes extending the lint command$")]
fn change_with_lint_proposal(world: &mut GivnWorld) {
    let env = env_of(world);
    let change = env.root().join("givn/changes/route-me");
    std::fs::create_dir_all(&change).unwrap();
    std::fs::write(
        change.join("proposal.md"),
        "Extend the lint command.\n\nThe change improves `givn lint` output. When a user runs `givn lint --json`\nthe lint findings print as JSON. Lint exits stay unchanged.\n",
    )
    .unwrap();
}

#[when(
    regex = r#"^the user audits the permanent corpus with `givn spec duplicates( --format json)?`$"#
)]
fn run_duplicates(world: &mut GivnWorld, json: String) {
    let args = if json.trim().is_empty() {
        vec!["spec", "duplicates"]
    } else {
        vec!["spec", "duplicates", "--format", "json"]
    };
    world.last_output = Some(run_givn(env_of(world), &args));
}

#[when(regex = r#"^the user searches the permanent corpus for "(.*)" with `givn spec find`$"#)]
fn run_find(world: &mut GivnWorld, query: String) {
    world.last_output = Some(run_givn(env_of(world), &["spec", "find", &query]));
}

#[when(regex = r#"^the user inspects the corpus with `givn spec tree`$"#)]
fn run_tree(world: &mut GivnWorld) {
    world.last_output = Some(run_givn(env_of(world), &["spec", "tree"]));
}

#[when(regex = r#"^the user routes the change with `givn spec route --change`$"#)]
fn run_route_change(world: &mut GivnWorld) {
    world.last_output = Some(run_givn(
        env_of(world),
        &["spec", "route", "--change", "route-me"],
    ));
}

#[then("stdout is a JSON array")]
fn stdout_is_json_array(world: &mut GivnWorld) {
    let out = stdout(world);
    let value: serde_json::Value = serde_json::from_str(&out).expect("stdout is valid JSON");
    assert!(value.is_array(), "{value}");
}

#[then(regex = r#"^stdout contains the matching scenario title$"#)]
fn stdout_has_scenario_title(world: &mut GivnWorld) {
    assert!(stdout(world).contains("The shovelling scenario"));
}

#[then(regex = r#"^stdout identifies the owning use case and capability$"#)]
fn stdout_has_usecase_and_capability(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(out.contains("workers"), "{out}");
    assert!(out.contains("work"), "{out}");
}

#[then(regex = r#"^stdout contains both use-case names$"#)]
fn tree_has_both_usecases(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(out.contains("diggers"), "{out}");
    assert!(out.contains("fillers"), "{out}");
}

#[then(regex = r#"^stdout contains the scenario count and the e2e count per capability$"#)]
fn tree_has_counts(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(out.contains("2 scenarios"), "{out}");
    assert!(out.contains("1 @e2e"), "{out}");
}

#[then(regex = r#"^stdout contains the use-case actor line$"#)]
fn tree_has_actor(world: &mut GivnWorld) {
    assert!(stdout(world).contains("actors:"));
}

#[then(regex = r#"^stdout contains the capability that invokes `givn lint`$"#)]
fn route_has_lint_capability(world: &mut GivnWorld) {
    assert!(stdout(world).contains("lint-tool"));
}

#[then(regex = r#"^stdout contains an evidence line$"#)]
fn route_has_evidence(world: &mut GivnWorld) {
    assert!(stdout(world).contains("both invoke `givn lint`"));
}

#[then(regex = r#"^stdout stays free of "(.*)" and "(.*)" wording$"#)]
fn stdout_free_of_two(world: &mut GivnWorld, a: String, b: String) {
    let out = stdout(world);
    assert!(!out.contains(&a), "stdout contains {a}: {out}");
    assert!(!out.contains(&b), "stdout contains {b}: {out}");
}

#[given(
    "a spec discovery project whose corpus contains one feature with two scenarios having literally equal supported executable content"
)]
fn corpus_literal_equal_in_one_feature(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/audit/audit.feature",
        "Feature: Literal\n\n  Scenario: Literal first\n    Given a workspace named \"alpha\"\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n\n  Scenario: Literal second\n    Given a workspace named \"alpha\"\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n",
    );
    world.env = Some(env);
}

fn audit_text(world: &GivnWorld) -> String {
    String::from_utf8_lossy(&world.last_output.as_ref().expect("audit output").stdout).into_owned()
}

#[then("stdout contains both audit endpoints")]
fn stdout_has_both_endpoints(world: &mut GivnWorld) {
    let out = audit_text(world);
    assert!(out.contains("Literal first"), "{out}");
    assert!(out.contains("Literal second"), "{out}");
    assert_eq!(out.matches("audit.feature").count(), 2, "{out}");
}
