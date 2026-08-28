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
    let content = std::fs::read_to_string("assets/skills/givn-domain-modeling/SKILL.md.tmpl")
        .expect("read domain-modeling skill template");
    world.additional_output = Some(content);
}

#[given("a Givn project with the domain model addon enabled and a change awaiting specs")]
fn addon_enabled_project_with_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let enable = crate::common::run_givn(&env, &["addons", "enable", "domain_modeling"]);
    assert!(enable.status.success(), "addons enable domain_modeling failed");
    let created = crate::common::run_givn(&env, &["new", FIXTURE_CHANGE_ID]);
    assert!(created.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[given("a Givn project with the domain model addon disabled and a change awaiting specs")]
fn addon_disabled_project_with_change(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
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
        content.contains("Update `domain.md` inline"),
        "body does not require inline domain.md updates"
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
    unimplemented!("guidance-says-load-domain-skill");
    let _ = world;
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
