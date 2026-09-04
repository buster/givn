//! Deterministic spec-discovery step definitions (Phase 4 rebuild).
//!
//! These steps drive the compiled givn CLI as a real subprocess against
//! fixture projects whose permanent corpus uses the grouped layout. They
//! back the rewritten `spec-duplicates`, `spec-explore-permanent`,
//! `spec-explore-change`, and `spec-progress-output` capabilities — all
//! deterministic: no embeddings, no index, no model downloads.

use std::path::PathBuf;

use crate::common::{make_bare_root, run_givn, TestEnv};
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

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

#[given("a spec discovery project whose corpus contains two features with one identical scenario each")]
fn corpus_with_cross_feature_pair(world: &mut GivnWorld) {
    let env = make_bare_root();
    let one = duplicate_pair_feature("The thing happens", "Unrelated other");
    write_feature(&env, "givn/specs/audit/one.feature", &one);
    let two = "Feature: Second\n\n  Scenario: The thing happens\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/audit/two.feature", two);
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose corpus contains two scenarios with identical assertions but different actions$"#)]
fn corpus_with_same_shape(world: &mut GivnWorld) {
    let env = make_bare_root();
    let body = "Feature: Shapes\n\n  Scenario: First shape\n    Given a prepared workspace\n    When the first action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n\n  Scenario: Second shape\n    Given a prepared workspace\n    When the second action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/shapes/shapes.feature", body);
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose corpus contains one scenario whose assertions are a subset of another's$"#)]
fn corpus_with_subset(world: &mut GivnWorld) {
    let env = make_bare_root();
    let body = "Feature: Subsets\n\n  Scenario: Small\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n\n  Scenario: Large\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/subsets/subsets.feature", body);
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose corpus contains a malformed permanent feature$"#)]
fn corpus_with_malformed_feature(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/broken/broken.feature",
        "Feature: Broken\n\n  Scenario: Unclosed docstring\n    Given a workspace\n      \"\"\"\n      never closed\n",
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project with no permanent specifications directory$"#)]
fn project_without_specs_dir(world: &mut GivnWorld) {
    let env = make_bare_root();
    std::fs::create_dir_all(env.root().join("givn")).unwrap();
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose archive contains a duplicate pair and whose corpus has none$"#)]
fn archive_holds_duplicate_pair(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/archive/old-change/old.feature",
        &duplicate_pair_feature("The thing happens", "The thing happens"),
    );
    write_feature(
        &env,
        "givn/specs/audit/audit.feature",
        "Feature: Clean\n\n  Scenario: Only one\n    Given a prepared workspace\n    Then the command exits 0\n",
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

#[given(regex = r#"^a spec discovery project whose corpus embeds literal Gherkin with a "Scenario:" line inside a docstring$"#)]
fn corpus_with_docstring_gherkin(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/docs/docs.feature",
        "Feature: Docs\n\n  Scenario: Real scenario only\n    Given a workspace\n    Then the report shows \"\"\"\n      Scenario: fake embedded title\n      \"\"\"\n",
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project with a non-empty corpus$"#)]
fn corpus_non_empty(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/audit/audit.feature",
        "Feature: Audit\n\n  Scenario: The thing happens\n    Given a prepared workspace\n    Then the command exits 0\n",
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project with two groups and a group\.md narrative$"#)]
fn two_groups_with_narrative(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/diggers/digging.feature",
        "Feature: Digging\n\n  Scenario: Dig\n    Given a workspace\n    Then the pit exists\n\n  @e2e\n  Scenario: Shovel\n    Given a workspace\n    Then the pit exists\n",
    );
    std::fs::write(
        env.root().join("givn/specs/diggers/group.md"),
        "# Group: diggers\n\n## Actor\n\nA builder digging holes.\n\n## Goal\n\nDig reproducible pits.\n\n## Interactions\n\n- dig a pit\n",
    )
    .unwrap();
    write_feature(
        &env,
        "givn/specs/fillers/filling.feature",
        "Feature: Filling\n\n  Scenario: Fill\n    Given a pit\n    Then the ground is level\n",
    );
    world.env = Some(env);
}

#[given(regex = r#"^a spec discovery project whose corpus contains a capability that invokes `givn lint`$"#)]
fn corpus_with_lint_capability(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/gates/lint-tool.feature",
        "Feature: Lint tool\n\n  Scenario: One\n    When the user runs `givn lint`\n    And the user runs `givn lint`\n    Then the command exits 0\n",
    );
    std::fs::write(
        env.root().join("givn/specs/gates/group.md"),
        "# Group: gates\n\n## Actor\n\nA checker.\n\n## Goal\n\nCheck things.\n\n## Interactions\n\n- run lint checks\n",
    )
    .unwrap();
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

#[given(regex = r"^an active change whose proposal describes internal widget plumbing with no command surface$")]
fn change_with_internal_proposal(world: &mut GivnWorld) {
    let env = env_of(world);
    let change = env.root().join("givn/changes/route-me");
    std::fs::create_dir_all(&change).unwrap();
    std::fs::write(
        change.join("proposal.md"),
        "Internal widget plumbing refactor.\n\nThe serialization of the widget graph internals changes with no user\nvisible surface or command invocation.\n",
    )
    .unwrap();
}

#[given(regex = r"^a spec discovery project with a non-empty corpus and an active change$")]
fn corpus_and_change(world: &mut GivnWorld) {
    corpus_non_empty(world);
    let env = env_of(world);
    let change = env.root().join("givn/changes/route-me");
    std::fs::create_dir_all(&change).unwrap();
    std::fs::write(change.join("proposal.md"), "Anything at all.\n").unwrap();
}

#[when(regex = r#"^the user audits the permanent corpus with `givn spec duplicates( --format json)?`$"#)]
fn run_duplicates(world: &mut GivnWorld, json: String) {
    let args = if json.trim().is_empty() {
        vec!["spec", "duplicates"]
    } else {
        vec!["spec", "duplicates", "--format", "json"]
    };
    world.last_output = Some(run_givn(env_of(world), &args));
}

#[when(regex = r#"^the user audits the permanent corpus twice with `givn spec duplicates`$"#)]
fn run_duplicates_twice(world: &mut GivnWorld) {
    let first = run_givn(env_of(world), &["spec", "duplicates"]);
    let second = run_givn(env_of(world), &["spec", "duplicates"]);
    world.additional_output = Some(String::from_utf8_lossy(&first.stdout).into_owned());
    world.last_output = Some(second);
}

#[when(regex = r#"^the user searches the permanent corpus for "(.*)" with `givn spec find`$"#)]
fn run_find(world: &mut GivnWorld, query: String) {
    world.last_output = Some(run_givn(env_of(world), &["spec", "find", &query]));
}

#[when(regex = r#"^the user searches the permanent corpus for the docstring-embedded title with `givn spec find`$"#)]
fn run_find_docstring_title(world: &mut GivnWorld) {
    world.last_output = Some(run_givn(
        env_of(world),
        &["spec", "find", "fake embedded title"],
    ));
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

#[when(regex = r#"^the user routes the change with both `--change` and `--file`$"#)]
fn run_route_both(world: &mut GivnWorld) {
    world.last_output = Some(run_givn(
        env_of(world),
        &[
            "spec",
            "route",
            "--change",
            "route-me",
            "--file",
            "givn/changes/route-me/proposal.md",
        ],
    ));
}

#[then(regex = r#"^stdout contains the audit endpoint titles "(.*)" and "(.*)"$"#)]
fn stdout_has_both_titles(world: &mut GivnWorld, a: String, b: String) {
    let out = stdout(world);
    assert!(out.contains(&a) || !out.contains(&a), "titles are identical by construction: {a}");
    assert!(out.contains(a.trim()) && out.contains(b.trim()), "{out}");
}

#[then("stdout is a JSON array")]
fn stdout_is_json_array(world: &mut GivnWorld) {
    let out = stdout(world);
    let value: serde_json::Value =
        serde_json::from_str(&out).expect("stdout is valid JSON");
    assert!(value.is_array(), "{value}");
}

#[then(regex = r#"^every JSON match carries kind left_file left_title right_file and right_title$"#)]
fn json_matches_have_fields(world: &mut GivnWorld) {
    let out = stdout(world);
    let value: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let items = value.as_array().expect("array");
    assert!(!items.is_empty(), "fixture must produce at least one match");
    for item in items {
        for field in ["kind", "left_file", "left_title", "right_file", "right_title"] {
            assert!(
                item.get(field).is_some_and(|v| v.is_string()),
                "missing {field} in {item}"
            );
        }
    }
}

#[then(regex = r#"^stdout contains the matching scenario title$"#)]
fn stdout_has_scenario_title(world: &mut GivnWorld) {
    assert!(stdout(world).contains("The shovelling scenario"));
}

#[then(regex = r#"^stdout identifies the owning group and capability$"#)]
fn stdout_has_group_and_capability(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(out.contains("workers"), "{out}");
    assert!(out.contains("work"), "{out}");
}

#[then(regex = r#"^stdout contains both group names$"#)]
fn tree_has_both_groups(world: &mut GivnWorld) {
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

#[then(regex = r#"^stdout contains the group actor line$"#)]
fn tree_has_actor(world: &mut GivnWorld) {
    assert!(stdout(world).contains("actor:"));
}

#[then(regex = r#"^stdout reports exactly one matching scenario$"#)]
fn find_reports_single_hit(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(out.contains("1 scenario(s) match"), "{out}");
}

#[then(regex = r#"^stdout contains the capability that invokes `givn lint`$"#)]
fn route_has_lint_capability(world: &mut GivnWorld) {
    assert!(stdout(world).contains("lint-tool"));
}

#[then(regex = r#"^stdout contains an evidence line$"#)]
fn route_has_evidence(world: &mut GivnWorld) {
    assert!(stdout(world).contains("both invoke `givn lint`"));
}

#[then("both audit reports are identical")]
fn audit_reports_identical(world: &mut GivnWorld) {
    let first = world
        .additional_output
        .as_ref()
        .expect("first audit output missing");
    let second = stdout(world);
    assert_eq!(first.trim(), second.trim(), "audit output must be deterministic");
}


#[then(regex = r#"^stdout stays free of "(.*)" and "(.*)" wording$"#)]
fn stdout_free_of_two(world: &mut GivnWorld, a: String, b: String) {
    let out = stdout(world);
    assert!(!out.contains(&a), "stdout contains {a}: {out}");
    assert!(!out.contains(&b), "stdout contains {b}: {out}");
}

#[then("stdout stays free of model wording")]
fn stdout_free_of_model_wording(world: &mut GivnWorld) {
    let out = stdout(world);
    assert!(!out.contains("model"), "stdout contains model: {out}");
}
