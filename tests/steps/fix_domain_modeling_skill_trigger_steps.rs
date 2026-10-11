//! Step-definition skeleton for the domain-skill-trigger capability.
//!
//! Regular steps inspect the `givn-domain-modeling` skill template asset and
//! drive the CLI (addon-off and JSON variants) via `crate::common::run_givn`.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn skill_template(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("domain-modeling skill template was not loaded")
}

fn stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(output.status.success(), "CLI command failed");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn active_project(world: &GivnWorld) -> &crate::common::TestEnv {
    world.env.as_ref().expect("fixture project is missing")
}

const FIXTURE_CHANGE_ID: &str = "domain-skill-trigger";

#[given("the domain-modeling skill template is inspected")]
fn domain_modeling_template_inspected(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-domain-modeling/SKILL.md.tmpl",
    ))
    .expect("read domain-modeling skill template");
    world.additional_output = Some(content);
}

#[given("a Givn project with the domain model addon enabled and a change awaiting specs")]
fn addon_enabled_project_with_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let enable = crate::common::run_givn(&env, &["addons", "enable", "domain_modeling"]);
    assert!(
        enable.status.success(),
        "addons enable domain_modeling failed"
    );
    let created = crate::common::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[given("a Givn project with the domain model addon disabled and a change awaiting specs")]
fn addon_disabled_project_with_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let disable = crate::common::run_givn(&env, &["addons", "disable", "domain_modeling"]);
    assert!(
        disable.status.success(),
        "addons disable domain_modeling failed"
    );
    let created = crate::common::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[given("a Givn project with the domain model addon enabled and a change awaiting review")]
fn addon_enabled_project_awaiting_review(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let enable = crate::common::run_givn(&env, &["addons", "enable", "domain_modeling"]);
    assert!(
        enable.status.success(),
        "addons enable domain_modeling failed"
    );
    let created = crate::common::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    let change = env.path(&format!("givn/changes/{FIXTURE_CHANGE_ID}"));
    std::fs::create_dir_all(change.join("specs/domain-skill-trigger")).unwrap();
    std::fs::write(
        change.join("specs/domain-skill-trigger/example.feature"),
        "Feature: fixture\n",
    )
    .unwrap();
    std::fs::write(change.join("design.md"), "# Design\n").unwrap();
    std::fs::write(
        change.join("design-review.md"),
        "# Design review\n\nDESIGN-REVIEW: PASS\n",
    )
    .unwrap();
    std::fs::write(change.join("tasks.md"), "- [x] done\n").unwrap();
    world.env = Some(env);
}

#[given(
    "a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs"
)]
fn addon_enabled_without_arc42_project_with_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let disable = crate::common::run_givn(&env, &["addons", "disable", "arc42"]);
    assert!(disable.status.success(), "addons disable arc42 failed");
    let enable = crate::common::run_givn(&env, &["addons", "enable", "domain_modeling"]);
    assert!(
        enable.status.success(),
        "addons enable domain_modeling failed"
    );
    let created = crate::common::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("an agent reads the skill trigger text")]
fn agent_reads_skill_trigger_text(world: &mut GivnWorld) {
    assert!(
        !skill_template(world).is_empty(),
        "skill template must not be empty"
    );
}

#[when("an agent reads the skill body")]
fn agent_reads_skill_body(world: &mut GivnWorld) {
    assert!(
        !skill_template(world).is_empty(),
        "skill template must not be empty"
    );
}

#[when("an agent reads the lifecycle guidance")]
fn agent_reads_lifecycle_guidance(world: &mut GivnWorld) {
    assert!(
        !skill_template(world).is_empty(),
        "skill template must not be empty"
    );
}

#[when("the agent requests the specs instructions through the Givn CLI")]
fn agent_requests_specs_instructions(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(
            env,
            &["instructions", "specs", "--change", FIXTURE_CHANGE_ID],
        )
    };
    world.last_output = Some(output);
}

#[when("the agent requests the next-step guidance as JSON through the Givn CLI")]
fn agent_requests_next_step_json(world: &mut GivnWorld) {
    let output = {
        let env = active_project(world);
        crate::common::run_givn(env, &["instructions", "--json"])
    };
    world.last_output = Some(output);
}

#[then("the trigger loads while naming or discussing domain concepts")]
fn trigger_loads_while_naming_domain_concepts(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("naming or") && content.contains("discussing domain concepts"),
        "trigger text does not cover naming or discussing domain concepts"
    );
}

#[then("the trigger loads before writing feature specs or the design")]
fn trigger_loads_before_specs_or_design(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("before writing .feature specs or design.md"),
        "trigger text does not cover the specs and design phases"
    );
}

#[then("the trigger loads when spec and code terms disagree")]
fn trigger_loads_on_spec_code_disagreement(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("when spec and code terms disagree"),
        "trigger text does not cover spec/code disagreement"
    );
}

#[then("the body challenges generic and overloaded terms")]
fn body_challenges_generic_terms(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Challenge generic or overloaded terms"),
        "body does not challenge generic or overloaded terms"
    );
}

#[then("the body names aggregates with domain nouns")]
fn body_names_aggregates_with_domain_nouns(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Name aggregates with domain nouns"),
        "body does not name aggregates with domain nouns"
    );
}

#[then("the body is updated inline as the vocabulary resolves")]
fn body_updated_inline(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("the glossary inline"),
        "body does not require inline glossary updates"
    );
}

#[then("the lifecycle describes the domain's own observable states")]
fn lifecycle_describes_domain_states(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("the *domain's own*") && content.contains("trip reset"),
        "lifecycle guidance does not describe a domain's own observable states"
    );
}

#[then("the lifecycle does not describe the givn change lifecycle")]
fn lifecycle_does_not_describe_givn_cycle(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        !content.contains("created -> planned -> implemented -> archived"),
        "lifecycle guidance still describes the givn change lifecycle"
    );
}

#[then("the body requires a per-change ubiquitous-language table")]
fn body_requires_ul_table(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Ubiquitous language"),
        "body does not require a ubiquitous-language table"
    );
}

#[then("the body records rejected synonyms as anti-terms")]
fn body_records_anti_terms(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Anti-terms"),
        "body does not record rejected synonyms as anti-terms"
    );
}

#[then("the guidance says to load the domain-modeling skill now")]
fn guidance_says_load_domain_skill(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        output.contains("Skill to load now: `givn-domain-modeling`"),
        "guidance does not say to load the domain-modeling skill now: {output}"
    );
}

#[then("the JSON response names the domain-modeling skill as the skill to load")]
fn json_response_names_domain_skill(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("JSON next-step command was not run");
    assert!(output.status.success(), "next-step JSON command failed");
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("next-step output is not JSON");
    assert_eq!(
        value["skill_to_load"], "givn-domain-modeling",
        "JSON response does not name the skill to load"
    );
}

#[then("the guidance does not mention the domain-modeling skill")]
fn guidance_does_not_mention_domain_skill(world: &mut GivnWorld) {
    let output = stdout(world);
    assert!(
        !output.contains("Skill to load now"),
        "guidance mentions the domain-modeling skill with the addon disabled: {output}"
    );
}

#[given("the design-review skill template is inspected")]
fn design_review_template_inspected(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-design-review/SKILL.md.tmpl",
    ))
    .expect("read design-review skill template");
    world.additional_output = Some(content);
}

#[given("the review skill template is inspected")]
fn review_template_inspected(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-review/SKILL.md.tmpl",
    ))
    .expect("read review skill template");
    world.additional_output = Some(content);
}

#[given("a placeholder removed scenario")]
fn placeholder_removed_scenario(_world: &mut GivnWorld) {
    let template = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-domain-modeling/SKILL.md.tmpl",
    ))
    .expect("domain-modeling skill template must exist for the placeholder step");
    assert!(
        template.contains("givn-domain-modeling"),
        "skill template asset is not loadable"
    );
}

#[when("an agent reads the check list")]
fn agent_reads_check_list(world: &mut GivnWorld) {
    assert!(
        !skill_template(world).is_empty(),
        "skill template must not be empty"
    );
}

#[then("the trigger loads during every workflow step of a change")]
fn trigger_loads_during_every_workflow_step(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("during every workflow step of a change"),
        "trigger text does not say it loads for every workflow step"
    );
}

#[then("the body instructs maintaining the project's arc42 glossary")]
fn body_instructs_maintaining_glossary(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("maintaining the project's arc42 glossary"),
        "body does not instruct maintaining the arc42 glossary"
    );
}

#[then("the body does not require a per-change domain artifact")]
fn body_does_not_require_domain_artifact(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        !content.contains("domain.md"),
        "body still requires a per-change domain.md artifact"
    );
}

#[then("the body instructs enabling the arc42 addon first when the glossary is missing")]
fn body_instructs_enabling_arc42(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Requires the arc42 addon"),
        "body does not state the arc42 addon requirement"
    );
    assert!(
        content.contains("enable it first when the glossary is missing"),
        "body does not instruct enabling the arc42 addon first"
    );
}

#[then(
    "a check requires confirming the ubiquitous language is used consistently and recorded in the arc42 glossary"
)]
fn design_review_checks_ubiquitous_language(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("ubiquitous language is used consistently"),
        "design-review template has no ubiquitous-language check"
    );
    assert!(
        content.contains("arc42 glossary"),
        "design-review check does not reference the arc42 glossary"
    );
}

#[then(
    "a check requires confirming the ubiquitous language is used consistently everywhere and recorded in the arc42 glossary"
)]
fn review_checks_ubiquitous_language(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("ubiquitous language is used consistently"),
        "review template has no ubiquitous-language check"
    );
    assert!(
        content.contains("arc42 glossary"),
        "review check does not reference the arc42 glossary"
    );
}

// --- term-introduction-discipline: term introduction test steps ------------

#[when("an agent reads the term introduction test")]
fn agent_reads_term_introduction_test(world: &mut GivnWorld) {
    assert!(
        !skill_template(world).is_empty(),
        "skill template must not be empty"
    );
}

#[then("the body requires a genus and differentia definition")]
fn body_requires_genus_and_differentia(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Genus + differentia") && content.contains("is a `<kind>` that"),
        "body does not require a genus and differentia definition"
    );
}

#[then("the body requires a contrast pair")]
fn body_requires_contrast_pair(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Contrast pair") && content.contains("not `<near neighbour>`"),
        "body does not require a contrast pair"
    );
}

#[then("the body requires one concrete domain sentence")]
fn body_requires_concrete_sentence(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("concrete domain sentence"),
        "body does not require one concrete domain sentence"
    );
}

#[then("the body demonstrates a failure when the genus slot cannot be filled")]
fn body_demonstrates_genus_failure(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        content.contains("Substrate") && content.contains("has no referent"),
        "body does not demonstrate the genus-slot failure"
    );
}

#[then("the body no longer tells the agent to choose the name alone")]
fn body_no_longer_chooses_alone(world: &mut GivnWorld) {
    let content = skill_template(world);
    assert!(
        !content.contains("Propose it and justify it")
            && content.contains("do not choose it alone"),
        "body still tells the agent to choose and justify the name alone"
    );
}

#[then("the body describes the concept before proposing names")]
fn body_describes_concept_first(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("the concept in one sentence"),
        "body does not describe the concept before proposing names"
    );
}

#[then("the body offers up to three candidate names")]
fn body_offers_candidate_names(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("up to three candidate names"),
        "body does not offer up to three candidate names"
    );
}

#[then("the body states what each candidate evokes and any collision it carries")]
fn body_states_candidate_collisions(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("any collision it carries"),
        "body does not state what each candidate evokes and any collision it carries"
    );
}

#[then("the body accepts the user's own name")]
fn body_accepts_own_name(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("accept the user's own name"),
        "body does not accept the user's own name"
    );
}

// --- user-verified-term-naming: naming-question example steps ---------------

#[then(regex = r#"^the body shows the naming question for "([^"]+)"$"#)]
fn body_shows_naming_question(world: &mut GivnWorld, term: String) {
    let content = skill_template(world);
    assert!(
        content.contains(term.as_str()) && content.contains("Candidates:"),
        "body does not show the naming question with its candidate names"
    );
}

#[then(regex = r#"^the body shows the colliding candidate "([^"]+)"$"#)]
fn body_shows_colliding_candidate(world: &mut GivnWorld, candidate: String) {
    let content = skill_template(world);
    assert!(
        content.contains(candidate.as_str()) && content.contains("collides"),
        "body does not show the candidate and its collision"
    );
}

#[then("the body asks which name the record should use")]
fn body_asks_which_name(world: &mut GivnWorld) {
    assert!(
        skill_template(world).contains("should the record use, or what is yours"),
        "body does not ask which name the record should use"
    );
}
