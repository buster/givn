//! E2E step definitions for E5 passage-cap evidence.

use std::fs;

use cucumber::{given, then, when};
use serde_json::Value;

use super::GivnWorld;
use crate::common;

const FITTING_TITLE: &str = "Fitting E5 scenario";
const CAPPED_TITLE: &str = "Over-cap E5 scenario";

fn duplicate_json(world: &GivnWorld) -> Value {
    serde_json::from_slice(&super::output(world).stdout).expect("valid duplicate JSON")
}

fn index(world: &GivnWorld) -> Value {
    let env = world.env.as_ref().expect("E5 index project is missing");
    let bytes = fs::read(env.path("givn/.spec-index.json")).expect("read permanent E5 index");
    serde_json::from_slice(&bytes).expect("valid permanent E5 index JSON")
}

#[given(
    "an end-to-end permanent corpus with one fitting scenario and one complete E5 passage above 512 tokens"
)]
fn e2e_partial_permanent_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    fs::create_dir_all(specs.join("e5-cap")).expect("create E5 cap specs");
    fs::write(
        env.path("givn/specs/e5-cap/e5-cap.feature"),
        "Feature: E5 cap fixture\n\n  Scenario: Fitting E5 scenario\n    Given a fitting context\n    When the fitting action runs\n    Then the fitting result is complete\n\n  Scenario: Over-cap E5 scenario\n    Given an over-cap context\n    When the over-cap action runs\n    Then the over-cap result is complete\n",
    )
    .expect("write E5 cap corpus");
    world.env = Some(env);
}

#[given(
    "an end-to-end permanent corpus with a fitting scenario and one complete E5 passage above 512 tokens"
)]
fn e2e_fitting_permanent_corpus(world: &mut GivnWorld) {
    e2e_partial_permanent_corpus(world);
}

#[given(
    "deterministic E5 token-budget fixtures identify the over-cap scenario as requiring 550 tokens"
)]
fn e2e_over_cap_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the E5 cap corpus is missing");
    world.retrieval_e5_token_budget = Some("partial-512-550".to_string());
}

#[given(
    "deterministic E5 token-budget fixtures identify the over-cap permanent scenario as requiring 550 tokens"
)]
fn e2e_permanent_over_cap_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the E5 cap corpus is missing");
    world.retrieval_e5_token_budget = Some("partial-512-550".to_string());
}

#[given("a complete fitting draft scenario is available for exploration")]
fn e2e_fitting_draft(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 cap corpus is missing");
    fs::write(
        env.path("draft.feature"),
        "Feature: fitting draft\n\n  Scenario: Fitting draft scenario\n    Given a fitting context\n    When the fitting action runs\n    Then the fitting result is complete\n",
    )
    .expect("write fitting draft");
}

#[when("the end-to-end user explores the draft with JSON output through the givn CLI")]
fn e2e_explore_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 cap corpus is missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &[
            "spec",
            "explore",
            "--file",
            "draft.feature",
            "--format",
            "json",
        ],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_E5_TOKEN_BUDGET",
                world
                    .retrieval_e5_token_budget
                    .as_deref()
                    .expect("E5 token fixture"),
            ),
        ],
    ));
}

#[then("stdout is valid exploration JSON")]
fn explore_stdout_is_json(world: &mut GivnWorld) {
    let _: Value = serde_json::from_slice(&super::output(world).stdout)
        .expect("valid exploration JSON response");
}

#[then("the fitting permanent result retains its normal E5 rank and score fields")]
fn explore_fitting_result_is_normal(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let result = document["results"]
        .as_array()
        .expect("exploration results")
        .iter()
        .find(|result| result["title"] == FITTING_TITLE)
        .expect("fitting exploration result");
    assert!(result["rank"].is_u64());
    assert!(result["score"].is_number());
    assert!(result["file"].is_string());
}

#[then("the normal exploration `results` collection contains no over-cap endpoint")]
fn explore_results_exclude_cap(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let results = document["results"].as_array().expect("exploration results");
    assert!(!results.iter().any(|result| result["title"] == CAPPED_TITLE));
}

#[then("`e5_token_cap_count` equals the `e5_token_cap_candidates` collection length of 1")]
fn explore_cap_count(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    assert_eq!(document["e5_token_cap_count"], 1);
    assert_eq!(
        document["e5_token_cap_candidates"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[then(
    "the JSON contains a visible `E5_TOKEN_CAP` refactoring record for the over-cap permanent scenario in `e5_token_cap_candidates`"
)]
fn explore_cap_record_is_visible(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let cap = &document["e5_token_cap_candidates"][0];
    assert_eq!(cap["status"], "E5_TOKEN_CAP");
    assert_eq!(cap["title"], CAPPED_TITLE);
    assert_eq!(cap["file"], "givn/specs/e5-cap/e5-cap.feature");
}

#[then(
    "that cap record has its file, title, complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and guidance to shorten or split the scenario and rerun exploration"
)]
fn explore_cap_record_is_complete(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let cap = &document["e5_token_cap_candidates"][0];
    let hash = cap["content_hash"].as_str().expect("cap hash");
    assert_eq!(hash.len(), 16);
    assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
    assert_eq!(cap["token_count"], 550);
    assert_eq!(cap["limit"], 512);
    assert_eq!(cap["model"], "intfloat/multilingual-e5-large");
    assert_eq!(
        cap["tokenizer"],
        "intfloat/multilingual-e5-large/tokenizer.json"
    );
    assert_eq!(cap["refactoring_required"], true);
    let guidance = cap["guidance"].as_str().expect("cap guidance");
    assert!(guidance.contains("shorten"));
    assert!(guidance.contains("split"));
    assert!(guidance.contains("rerun"));
}

#[given(
    "an end-to-end project with one fitting permanent scenario and one active complete E5 passage above 512 tokens"
)]
fn e2e_advisory_cap_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    fs::create_dir_all(specs.join("permanent")).expect("create permanent advisory specs");
    fs::write(
        env.path("givn/specs/permanent/permanent.feature"),
        "Feature: fitting permanent\n\n  Scenario: Fitting permanent scenario\n    Given a fitting permanent context\n    When the fitting permanent action runs\n    Then the fitting permanent result is complete\n",
    )
    .expect("write fitting permanent scenario");
    let active = env.path("givn/changes/running-change/specs/running-change");
    fs::create_dir_all(&active).expect("create active advisory specs");
    fs::write(
        active.join("running-change.feature"),
        "@givn.delta @running-change\nFeature: active E5 cap\n\n  @givn.added\n  Scenario: Active over-cap scenario\n    Given an active over-cap context\n    When the active over-cap action runs\n    Then the active over-cap result is complete\n",
    )
    .expect("write active advisory scenario");
    fs::write(
        env.path("givn/commands.yaml"),
        "verify:\n  command: \"true\"\n  e2e_command: \"true\"\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n  non_e2e_path: coverage/non.xml\n  e2e_path: coverage/e2e.xml\n  output_path: coverage/all.xml\n",
    )
    .expect("write blocking review commands");
    fs::write(
        env.path("givn/changes/running-change/review.md"),
        "REVIEW: PASS\n",
    )
    .expect("write blocking review dispositions");
    world.env = Some(env);
}

#[given(
    "deterministic E5 token-budget fixtures identify the active scenario as requiring 550 tokens"
)]
fn e2e_active_over_cap_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the advisory cap project is missing");
    world.retrieval_e5_token_budget = Some("active-550".to_string());
}

#[when(
    "the end-to-end user runs advisory semantic change review for \"running-change\" through the givn CLI"
)]
fn e2e_run_advisory_review(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("advisory cap project is missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "review", "--change", "running-change"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_E5_TOKEN_BUDGET",
                world
                    .retrieval_e5_token_budget
                    .as_deref()
                    .expect("E5 token fixture"),
            ),
        ],
    ));
}

#[then("stdout remains an advisory retrieval result")]
fn advisory_stdout_remains_advisory(world: &mut GivnWorld) {
    assert!(super::stdout(world).contains("retrieval advisory"));
}

#[then(
    "stdout exposes an additive `e5_token_cap_candidates` collection with count 1 for the active file and title"
)]
fn advisory_stdout_exposes_cap_collection(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(stdout.contains("e5_token_cap_count: 1"));
    assert!(stdout.contains("e5_token_cap_candidates:"));
    assert!(stdout.contains("Active over-cap scenario"));
    assert!(
        stdout.contains("givn/changes/running-change/specs/running-change/running-change.feature")
    );
}

#[then(
    "stdout reports the cap record's complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and shorten-or-split guidance"
)]
fn advisory_stdout_cap_record_is_complete(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(stdout.contains("token_count\":550"));
    assert!(stdout.contains("limit\":512"));
    assert!(stdout.contains("intfloat/multilingual-e5-large"));
    assert!(stdout.contains("intfloat/multilingual-e5-large/tokenizer.json"));
    assert!(stdout.contains("refactoring_required\":true"));
    assert!(stdout.contains("shorten or split"));
    assert!(stdout.contains("rerun semantic review"));
}

#[then("stdout does not report the capped endpoint as a normal ranked E5 pair candidate")]
fn advisory_stdout_excludes_cap_pair(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(
        !stdout
            .lines()
            .any(|line| line.starts_with("rank ") && line.contains("Active over-cap scenario"))
    );
    assert!(!stdout.contains("resembles 'Active over-cap scenario'"));
}

fn semantic_e5_report(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("blocking cap project is missing");
    fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("read E5 semantic review report")
}

#[then("the command exits non-zero because the E5 token-cap finding is unresolved")]
fn blocking_cap_exits_nonzero(world: &mut GivnWorld) {
    assert!(!super::output(world).status.success());
}

#[then(
    "the semantic review report has normal `Candidate count: 0` and an `e5_token_cap_count` of 1"
)]
fn blocking_report_counts_caps_separately(world: &mut GivnWorld) {
    let report = semantic_e5_report(world);
    assert!(report.contains("Candidate count: 0"));
    assert!(report.contains("e5_token_cap_count: 1"));
}

#[then(
    "the report's additive `e5_token_cap_candidates` collection contains an `E5_TOKEN_CAP` record with the active file and title"
)]
fn blocking_report_contains_active_cap(world: &mut GivnWorld) {
    let report = semantic_e5_report(world);
    assert!(report.contains("e5_token_cap_candidates:"));
    assert!(report.contains("status\":\"E5_TOKEN_CAP"));
    assert!(
        report.contains("givn/changes/running-change/specs/running-change/running-change.feature")
    );
    assert!(report.contains("Active over-cap scenario"));
}

#[then(
    "that cap record has its complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, and `refactoring_required: true`"
)]
fn blocking_report_cap_record_is_complete(world: &mut GivnWorld) {
    let report = semantic_e5_report(world);
    assert!(report.contains("content_hash:"));
    assert!(report.contains("token_count: 550"));
    assert!(report.contains("limit: 512"));
    assert!(report.contains("model: intfloat/multilingual-e5-large"));
    assert!(report.contains("tokenizer: intfloat/multilingual-e5-large/tokenizer.json"));
    assert!(report.contains("refactoring_required: true"));
}

#[then("the report tells the author to shorten or split the scenario before rerunning review")]
fn blocking_report_has_review_guidance(world: &mut GivnWorld) {
    let report = semantic_e5_report(world);
    assert!(report.contains("shorten or split the scenario, then rerun semantic review"));
}

#[then(
    "the cap record has `classification: null` and `rationale: null` and no automatic `DUPLICATE`, `VARIANT`, or `VALID-BOUNDARY` classification"
)]
fn blocking_report_cap_is_unresolved(world: &mut GivnWorld) {
    let report = semantic_e5_report(world);
    assert!(report.contains("- classification: null"));
    assert!(report.contains("- rationale: null"));
    assert!(!report.contains("DUPLICATE"));
    assert!(!report.contains("VARIANT"));
    assert!(!report.contains("VALID-BOUNDARY"));
}

#[when("the end-to-end user runs `givn spec index` through the givn CLI")]
fn e2e_run_index(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("E5 cap corpus is missing");
    world.last_output = Some(common::run_givn_with_envs(
        env,
        &["spec", "index"],
        &[
            ("GIVN_SPEC_EMBEDDER", "stub"),
            (
                "GIVN_SPEC_E5_TOKEN_BUDGET",
                world
                    .retrieval_e5_token_budget
                    .as_deref()
                    .expect("E5 token fixture"),
            ),
        ],
    ));
}

#[then("the fitting scenario has a vector in the permanent index")]
fn fitting_scenario_has_vector(world: &mut GivnWorld) {
    let document = index(world);
    let entries = document["entries"].as_array().expect("index entries");
    let entry = entries
        .iter()
        .find(|entry| entry["title"] == FITTING_TITLE)
        .expect("fitting index entry");
    assert!(
        entry["embedding"]
            .as_array()
            .is_some_and(|values| !values.is_empty())
    );
}

#[then("the over-cap scenario has no truncated vector in the permanent index")]
fn over_cap_scenario_has_no_vector(world: &mut GivnWorld) {
    let document = index(world);
    let entries = document["entries"].as_array().expect("index entries");
    assert!(!entries.iter().any(|entry| entry["title"] == CAPPED_TITLE));
}

#[then(
    "the permanent index has one additive `e5_token_cap_candidates` record for the over-cap endpoint"
)]
fn index_has_one_e5_cap_record(world: &mut GivnWorld) {
    let document = index(world);
    let caps = document["e5_token_caps"]
        .as_array()
        .expect("index E5 token-cap metadata");
    assert_eq!(caps.len(), 1);
    assert_eq!(caps[0]["status"], "E5_TOKEN_CAP");
    assert_eq!(caps[0]["title"], CAPPED_TITLE);
}

#[then(
    "that cap record has its file, title, complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and guidance"
)]
fn index_cap_record_is_complete(world: &mut GivnWorld) {
    let cap = &index(world)["e5_token_caps"][0];
    let file = cap["file"].as_str().expect("cap file");
    assert_eq!(file, "givn/specs/e5-cap/e5-cap.feature");
    assert_eq!(cap["title"], CAPPED_TITLE);
    let hash = cap["content_hash"].as_str().expect("cap content hash");
    assert_eq!(hash.len(), 16);
    assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
    assert_eq!(cap["token_count"], 550);
    assert_eq!(cap["limit"], 512);
    assert_eq!(cap["model"], "intfloat/multilingual-e5-large");
    assert_eq!(
        cap["tokenizer"],
        "intfloat/multilingual-e5-large/tokenizer.json"
    );
    assert_eq!(cap["refactoring_required"], true);
    assert!(cap["guidance"].as_str().is_some_and(|guidance| {
        guidance.contains("shorten") && guidance.contains("split") && guidance.contains("rerun")
    }));
}

#[then("stdout identifies the over-cap scenario as an E5 token-cap refactoring finding")]
fn stdout_identifies_e5_cap(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    assert!(stdout.contains("E5_TOKEN_CAP"));
    assert!(stdout.contains(CAPPED_TITLE));
    assert!(stdout.contains("refactoring required: true"));
}

#[given(
    "an end-to-end permanent corpus with an exact duplicate, a structural overlap, and two complete E5 passages above 512 tokens"
)]
fn e2e_duplicate_cap_corpus(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let specs = env.path("givn/specs");
    fs::remove_dir_all(&specs).expect("remove generated permanent specs");
    for directory in [
        "a-over-cap",
        "b-over-cap",
        "c-exact",
        "d-exact",
        "e-structural",
        "f-structural",
    ] {
        fs::create_dir_all(specs.join(directory)).expect("create duplicate cap directory");
    }
    fs::write(
        env.path("givn/specs/a-over-cap/a-over-cap.feature"),
        "Feature: Over-cap A\n\n  Scenario: Over-cap A\n    Given alpha context\n    When alpha action runs\n    Then alpha result is complete\n",
    )
    .expect("write over-cap A");
    fs::write(
        env.path("givn/specs/b-over-cap/b-over-cap.feature"),
        "Feature: Over-cap B\n\n  Scenario: Over-cap B\n    Given beta context\n    When beta action runs\n    Then beta result is complete\n",
    )
    .expect("write over-cap B");
    for (directory, feature, title) in [
        ("c-exact", "Exact C", "Exact C"),
        ("d-exact", "Exact D", "Exact D"),
    ] {
        fs::write(
            env.path(&format!("givn/specs/{directory}/{directory}.feature")),
            format!(
                "Feature: {feature}\n\n  Scenario: {title}\n    Given identical context\n    When identical action runs\n    Then identical result is complete\n"
            ),
        )
        .expect("write exact duplicate fixture");
    }
    for (directory, feature, title, context) in [
        ("e-structural", "Structural E", "Structural E", "one"),
        ("f-structural", "Structural F", "Structural F", "two"),
    ] {
        fs::write(
            env.path(&format!("givn/specs/{directory}/{directory}.feature")),
            format!(
                "Feature: {feature}\n\n  Scenario: {title}\n    Given {context} structural context\n    When the structural action runs\n    Then the structural result is complete\n"
            ),
        )
        .expect("write structural overlap fixture");
    }
    world.env = Some(env);
}

#[given(
    "deterministic E5 token-budget fixtures identify the over-cap endpoints as requiring 550 and 551 tokens"
)]
fn e2e_duplicate_cap_fixture(world: &mut GivnWorld) {
    assert!(world.env.is_some(), "the duplicate cap corpus is missing");
    world.retrieval_e5_token_budget = Some("partial-550-551-512".to_string());
}

#[then("the JSON retains the exact duplicate and structural overlap candidates")]
fn duplicate_json_retains_deterministic_candidates(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let candidates = document["candidates"]
        .as_array()
        .expect("normal candidates");
    assert!(
        candidates
            .iter()
            .any(|candidate| candidate["kind"] == "exact_duplicate")
    );
    assert!(
        candidates
            .iter()
            .any(|candidate| candidate["kind"] == "structural_overlap")
    );
}

#[then("the normal `candidates` collection contains no E5 token-cap record")]
fn duplicate_json_normal_candidates_exclude_caps(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let serialized = serde_json::to_string(&document["candidates"]).expect("serialize candidates");
    assert!(!serialized.contains("E5_TOKEN_CAP"));
}

#[then(
    "`candidate_count` equals the normal `candidates` collection length and excludes both cap records"
)]
fn duplicate_json_candidate_count_excludes_caps(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    assert_eq!(
        document["candidate_count"],
        document["candidates"].as_array().unwrap().len()
    );
    assert_ne!(document["candidate_count"], 0);
}

#[then("`e5_token_cap_count` equals the `e5_token_cap_candidates` collection length of 2")]
fn duplicate_json_cap_count(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    assert_eq!(document["e5_token_cap_count"], 2);
    assert_eq!(
        document["e5_token_cap_candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[then(
    "the `e5_token_cap_candidates` collection contains endpoint-level `E5_TOKEN_CAP` records for both over-cap files and titles"
)]
fn duplicate_json_cap_endpoints(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let caps = document["e5_token_cap_candidates"]
        .as_array()
        .expect("E5 cap candidates");
    assert_eq!(caps[0]["status"], "E5_TOKEN_CAP");
    assert_eq!(caps[1]["status"], "E5_TOKEN_CAP");
    assert_eq!(caps[0]["file"], "givn/specs/a-over-cap/a-over-cap.feature");
    assert_eq!(caps[1]["file"], "givn/specs/b-over-cap/b-over-cap.feature");
    assert_eq!(caps[0]["title"], "Over-cap A");
    assert_eq!(caps[1]["title"], "Over-cap B");
}

#[then(
    "each cap record reports its complete content hash, token count, limit 512, E5 model and tokenizer provenance, and `refactoring_required` true"
)]
fn duplicate_json_cap_records_are_complete(world: &mut GivnWorld) {
    for cap in duplicate_json(world)["e5_token_cap_candidates"]
        .as_array()
        .expect("E5 cap candidates")
    {
        let hash = cap["content_hash"].as_str().expect("cap hash");
        assert_eq!(hash.len(), 16);
        assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
        assert!(cap["token_count"] == 550 || cap["token_count"] == 551);
        assert_eq!(cap["limit"], 512);
        assert_eq!(cap["model"], "intfloat/multilingual-e5-large");
        assert_eq!(
            cap["tokenizer"],
            "intfloat/multilingual-e5-large/tokenizer.json"
        );
        assert_eq!(cap["refactoring_required"], true);
    }
}

#[then(
    "the cap records are ordered by normalized file and title and give guidance to shorten or split the scenario and rerun the audit"
)]
fn duplicate_json_caps_are_ordered_and_guided(world: &mut GivnWorld) {
    let document = duplicate_json(world);
    let caps = document["e5_token_cap_candidates"]
        .as_array()
        .expect("E5 cap candidates");
    assert!(caps[0]["file"].as_str().unwrap() < caps[1]["file"].as_str().unwrap());
    for cap in caps {
        let guidance = cap["guidance"].as_str().expect("cap guidance");
        assert!(guidance.contains("shorten"));
        assert!(guidance.contains("split"));
        assert!(guidance.contains("rerun"));
    }
    let report_guidance = document["guidance"].as_str().expect("report guidance");
    assert!(report_guidance.contains("audit"));
}

#[then("no E5 semantic candidate contains either over-cap endpoint as either endpoint")]
fn duplicate_json_semantic_candidates_exclude_caps(world: &mut GivnWorld) {
    for candidate in duplicate_json(world)["candidates"]
        .as_array()
        .expect("normal candidates")
        .iter()
        .filter(|candidate| candidate["kind"] == "semantic_candidate")
    {
        let left = candidate["left"]["file"].as_str().unwrap();
        let right = candidate["right"]["file"].as_str().unwrap();
        assert!(!left.contains("a-over-cap") && !left.contains("b-over-cap"));
        assert!(!right.contains("a-over-cap") && !right.contains("b-over-cap"));
    }
}
