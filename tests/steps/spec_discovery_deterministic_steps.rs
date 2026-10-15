//! Deterministic spec-discovery step definitions.
//!
//! These steps drive the compiled givn CLI as a real subprocess against
//! fixture projects whose permanent corpus uses use-case ownership. They
//! back the rewritten `spec-duplicates`, `spec-explore-permanent`,
//! `spec-explore-change`, and `spec-progress-output` capabilities — all
//! deterministic: no embeddings, no index, no model downloads.

use std::path::PathBuf;

use crate::common::{TestEnv, make_bare_root, run_givn};
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

fn write_usecase(
    env: &TestEnv,
    usecase: &str,
    capability: &str,
    actor: &str,
    goal: &str,
    scenario: &str,
) {
    let text = givn::gherkin::usecase_md::usecase_md_template()
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

#[given(
    "a spec discovery project whose corpus contains two features with one identical scenario each"
)]
fn corpus_with_cross_feature_pair(world: &mut GivnWorld) {
    let env = make_bare_root();
    let one = duplicate_pair_feature("The thing happens", "Unrelated other");
    write_feature(&env, "givn/specs/audit/one.feature", &one);
    let two = "Feature: Second\n\n  Scenario: The thing happens\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/audit/two.feature", two);
    world.env = Some(env);
}

#[given(
    regex = r#"^a spec discovery project whose corpus contains two scenarios with identical assertions but different actions$"#
)]
fn corpus_with_same_shape(world: &mut GivnWorld) {
    let env = make_bare_root();
    let body = "Feature: Shapes\n\n  Scenario: First shape\n    Given a prepared workspace\n    When the first action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n\n  Scenario: Second shape\n    Given a prepared workspace\n    When the second action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/shapes/shapes.feature", body);
    world.env = Some(env);
}

#[given(
    regex = r#"^a spec discovery project whose corpus contains one scenario whose assertions are a subset of another's$"#
)]
fn corpus_with_subset(world: &mut GivnWorld) {
    let env = make_bare_root();
    let body = "Feature: Subsets\n\n  Scenario: Small\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n\n  Scenario: Large\n    Given a prepared workspace\n    When the action runs\n    Then the command exits 0\n    And stdout contains \"done\"\n";
    write_feature(&env, "givn/specs/subsets/subsets.feature", body);
    world.env = Some(env);
}

#[given(
    regex = r#"^a spec discovery project whose corpus contains a malformed permanent feature$"#
)]
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

#[given(
    regex = r#"^a spec discovery project whose archive contains a duplicate pair and whose corpus has none$"#
)]
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

#[given(
    regex = r#"^a spec discovery project whose corpus embeds literal Gherkin with a "Scenario:" line inside a docstring$"#
)]
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

#[given(
    regex = r"^an active change whose proposal describes internal widget plumbing with no command surface$"
)]
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

#[when(
    regex = r#"^the user searches the permanent corpus for the docstring-embedded title with `givn spec find`$"#
)]
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
    assert!(
        out.contains(&a) || !out.contains(&a),
        "titles are identical by construction: {a}"
    );
    assert!(out.contains(a.trim()) && out.contains(b.trim()), "{out}");
}

#[then("stdout is a JSON array")]
fn stdout_is_json_array(world: &mut GivnWorld) {
    let out = stdout(world);
    let value: serde_json::Value = serde_json::from_str(&out).expect("stdout is valid JSON");
    assert!(value.is_array(), "{value}");
}

#[then(
    regex = r#"^every JSON match carries kind left_file left_title right_file and right_title$"#
)]
fn json_matches_have_fields(world: &mut GivnWorld) {
    let out = stdout(world);
    let value: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    let items = value.as_array().expect("array");
    assert!(!items.is_empty(), "fixture must produce at least one match");
    for item in items {
        for field in [
            "kind",
            "left_file",
            "left_title",
            "right_file",
            "right_title",
        ] {
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
    assert_eq!(
        first.trim(),
        second.trim(),
        "audit output must be deterministic"
    );
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

#[given("the permanent specifications are snapshotted")]
fn snapshot_specs(world: &mut GivnWorld) {
    let env = env_of(world);
    let specs = env.root().join("givn/specs");
    let mut contents = String::new();
    if specs.exists() {
        collect_tree_text(&specs, &mut contents);
    }
    world.additional_output = Some(contents);
}

fn collect_tree_text(dir: &std::path::Path, out: &mut String) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("read dir")
        .flatten()
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_tree_text(&path, out);
        } else {
            out.push_str(&path.display().to_string());
            out.push('\n');
            out.push_str(&std::fs::read_to_string(&path).unwrap_or_default());
        }
    }
}

#[then("the permanent specifications are unchanged")]
fn specs_unchanged(world: &mut GivnWorld) {
    let env = env_of(world);
    let specs = env.root().join("givn/specs");
    let mut contents = String::new();
    if specs.exists() {
        collect_tree_text(&specs, &mut contents);
    }
    let before = world.additional_output.as_ref().expect("snapshot missing");
    assert_eq!(*before, contents, "permanent specifications changed");
}

// --- use-case-model-scoping: canonical use-case contract steps -------------

fn model_document(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("use-case model document was not loaded")
}

#[given("the durable use-case model document is inspected")]
fn durable_usecase_model_inspected(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("docs/use-case-model.md"))
        .expect("read docs/use-case-model.md");
    world.additional_output = Some(content);
}

#[when("a maintainer reads the definition and the scoping rules")]
fn maintainer_reads_definition_and_rules(world: &mut GivnWorld) {
    assert!(
        !model_document(world).is_empty(),
        "use-case model must not be empty"
    );
}

#[then("the document defines a use case as a contract between one primary actor and the system")]
fn document_defines_use_case(world: &mut GivnWorld) {
    assert!(
        model_document(world).contains("contract between one primary actor and the system"),
        "document does not carry the canonical definition"
    );
}

#[then("the document carries the promotion rule for a leaf with three or more actor goals")]
fn document_carries_promotion_rule(world: &mut GivnWorld) {
    let content = model_document(world);
    assert!(
        content.contains("Promotion rule")
            && content.contains("three or more distinct actor goals"),
        "document does not carry the promotion rule"
    );
}

#[then("the document carries the placement test for a capability")]
fn document_carries_placement_test(world: &mut GivnWorld) {
    let content = model_document(world);
    assert!(
        content.contains("Placement test")
            && content.contains("which actor goal does completing this"),
        "document does not carry the placement test"
    );
}

#[then("the document carries the ten review rejection criteria")]
fn document_carries_ten_criteria(world: &mut GivnWorld) {
    let content = model_document(world);
    for title in [
        "Goal conjunction",
        "Missing primary actor",
        "Placement failure",
        "Mutation under an observing goal",
        "Extension without a step reference",
        "Inconsistent preconditions",
        "Mechanism vocabulary",
        "Component- or channel-shaped name",
        "Oversized leaf",
        "Relationship misuse",
    ] {
        assert!(
            content.contains(title),
            "document is missing the rejection criterion {title:?}"
        );
    }
}

#[then("the document carries the worked split of an observing mega-leaf")]
fn document_carries_observing_split(world: &mut GivnWorld) {
    let content = model_document(world);
    for marker in [
        "follow-the-run",
        "inspect-project-state",
        "follow-run-events",
        "manage-project-identity",
        "web-interface-substrate",
    ] {
        assert!(
            content.contains(marker),
            "document is missing the observing split marker {marker:?}"
        );
    }
}

#[then(
    "the document carries the promotion of a conjunction-goal leaf into a summary and its split"
)]
fn document_carries_conjunction_promotion(world: &mut GivnWorld) {
    let content = model_document(world);
    assert!(
        content.contains("run-a-project")
            && content.contains("promotes it to a summary")
            && content.contains("splits it"),
        "document does not carry the conjunction-goal promotion example"
    );
}

#[then("the worked observing case is tied to the mutation-containment and component-name criteria")]
fn observing_case_tied_to_criteria(world: &mut GivnWorld) {
    assert!(
        model_document(world).contains(
            "fails R4 (mutation under an observing goal) and R8 (component- or channel-shaped name)"
        ),
        "observing example is not tied to R4 and R8"
    );
}

#[then("the worked conjunction case is tied to the goal-conjunction and oversized-leaf criteria")]
fn conjunction_case_tied_to_criteria(world: &mut GivnWorld) {
    assert!(
        model_document(world).contains("fails R1 (goal conjunction) and R9 (oversized leaf)"),
        "conjunction example is not tied to R1 and R9"
    );
}

#[then("the workflow document points at the use-case model")]
fn workflow_points_at_model(_world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("docs/workflow.md"))
        .expect("read docs/workflow.md");
    assert!(
        content.contains("use-case-model.md"),
        "workflow document does not point at the use-case model"
    );
}

#[then("the reference document points at the use-case model")]
fn reference_points_at_model(_world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("docs/reference.md"))
        .expect("read docs/reference.md");
    assert!(
        content.contains("use-case-model.md"),
        "reference document does not point at the use-case model"
    );
}

fn template_text(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("use-case document template was not loaded")
}

#[given("the canonical use-case document template is inspected")]
fn canonical_usecase_template_inspected(world: &mut GivnWorld) {
    world.additional_output = Some(givn::gherkin::usecase_md::usecase_md_template());
}

#[when("a maintainer reads the template guidance")]
fn maintainer_reads_template_guidance(world: &mut GivnWorld) {
    assert!(
        !template_text(world).is_empty(),
        "use-case document template must not be empty"
    );
}

#[then("the template names the primary-actor convention")]
fn template_names_primary_actor_convention(world: &mut GivnWorld) {
    assert!(
        template_text(world).contains("the first actor is the primary actor"),
        "template does not name the primary-actor convention"
    );
}

#[then("the template shows the extension format")]
fn template_shows_extension_format(world: &mut GivnWorld) {
    assert!(
        template_text(world).contains("protects <guarantee>"),
        "template does not show the extension format"
    );
}

#[then("the template numbers the main-flow example")]
fn template_numbers_main_flow(world: &mut GivnWorld) {
    let content = template_text(world);
    assert!(
        content.contains("1. ") && content.contains("System responsibility"),
        "template does not number the main-flow example"
    );
}

#[then("the template carries the promotion note")]
fn template_carries_promotion_note(world: &mut GivnWorld) {
    assert!(
        template_text(world).contains("promoted to `+`"),
        "template does not carry the promotion note"
    );
}

#[then("the template preserves the fixture placeholders")]
fn template_preserves_placeholders(world: &mut GivnWorld) {
    let content = template_text(world);
    for placeholder in [
        "<usecase-id>",
        "<capability-id>",
        "<actor role>",
        "<the domain goal>",
        "<consumer action>",
        "<scenario title>",
    ] {
        assert!(
            content.contains(placeholder),
            "template dropped the fixture placeholder {placeholder:?}"
        );
    }
}

// --- finding-severity-routing-signal: route confidence fixtures ------------

#[given("a spec discovery project whose corpus contains two capabilities that invoke `givn lint`")]
fn corpus_two_lint_capabilities(world: &mut GivnWorld) {
    let env = make_bare_root();
    for capability in ["lint-tool", "lint-check"] {
        write_feature(
            &env,
            &format!("givn/specs/gates/{capability}.feature"),
            "Feature: Lint\n\n  Scenario: One\n    When the user runs `givn lint`\n",
        );
    }
    write_usecase(
        &env,
        "gates",
        "lint-tool",
        "A checker.",
        "Check the tool.",
        "Lint runs",
    );
    world.env = Some(env);
}

#[given(
    "a spec discovery project whose corpus contains a goal-aligned and a goal-mismatched capability"
)]
fn corpus_goal_aligned_and_mismatched(world: &mut GivnWorld) {
    let env = make_bare_root();
    write_feature(
        &env,
        "givn/specs/alignment/coverage-tool.feature",
        "Feature: Coverage tool\n\n  Scenario: One\n    Given a coverage report exists\n    Then the command exits 0\n",
    );
    write_usecase(
        &env,
        "alignment",
        "coverage-tool",
        "A maintainer.",
        "Validate coverage reports.",
        "Coverage runs",
    );
    write_feature(
        &env,
        "givn/specs/gates/lint-tool.feature",
        "Feature: Lint tool\n\n  Scenario: One\n    Given a coverage report exists\n    And a coverage report is stale\n    Then the command exits 0\n",
    );
    write_usecase(
        &env,
        "gates",
        "lint-tool",
        "A checker.",
        "Check the tool.",
        "Lint runs",
    );
    world.env = Some(env);
}

#[given("an active change whose proposal describes coverage report validation")]
fn change_with_coverage_proposal(world: &mut GivnWorld) {
    let env = env_of(world);
    let change = env.root().join("givn/changes/route-me");
    std::fs::create_dir_all(&change).unwrap();
    std::fs::write(
        change.join("proposal.md"),
        "Add coverage report validation.\n\nThe change validates the coverage report before publication.\n",
    )
    .unwrap();
}
