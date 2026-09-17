//! Step-definition skeleton for the instruction-integrity capability.
//!
//! RED scenarios intentionally remain undefined until each scenario enters its
//! own RED -> GREEN -> REFACTOR cycle.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn policy(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("canonical specs guidance was not loaded")
}

#[given("the canonical specs guidance is requested")]
fn canonical_specs_guidance_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("assets/instructions/specs.md"))
        .expect("read canonical specs instruction");
    world.additional_output = Some(content);
}

#[when("an author reads the interaction and E2E policy")]
fn author_reads_interaction_policy(world: &mut GivnWorld) {
    assert!(
        !policy(world).is_empty(),
        "canonical policy must not be empty"
    );
}

#[then("the guidance requires primary assertions on the named real interface")]
fn guidance_requires_primary_interface_assertions(world: &mut GivnWorld) {
    let content = policy(world);
    assert!(
        content.contains("primary assertion is on what")
            && content.contains("the named real interface"),
        "canonical policy does not require primary interface assertions"
    );
}

#[then("the guidance rejects repository-only evidence for a higher-level interface")]
fn guidance_rejects_repository_only_evidence(world: &mut GivnWorld) {
    assert!(
        policy(world).contains("Repository/database checks are secondary"),
        "canonical policy does not reject repository-only evidence"
    );
}

#[then("the guidance requires a real browser for browser UI interactions")]
fn guidance_requires_real_browser(world: &mut GivnWorld) {
    let content = policy(world);
    assert!(
        content.contains("for a browser-UI capability, drive the actual")
            && content.contains("browser through a real browser driver"),
        "canonical policy does not require a real browser"
    );
}

#[then("the guidance treats an unresolved interface obstacle as a design gap")]
fn guidance_treats_obstacle_as_design_gap(world: &mut GivnWorld) {
    assert!(
        policy(world).contains("an interface obstacle is a design gap"),
        "canonical policy does not define the interface obstacle as a design gap"
    );
}

fn design_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("design guidance was not loaded")
}

#[given("the design instructions are requested")]
fn design_instructions_requested(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/design.md")).expect("read design instruction");
    world.additional_output = Some(content);
}

#[when("an author prepares the technical design")]
fn author_prepares_design(world: &mut GivnWorld) {
    assert!(
        !design_guidance(world).is_empty(),
        "design guidance must not be empty"
    );
}

#[then("the guidance requires the E2E runner and its tag isolation")]
fn design_requires_e2e_runner(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("verify.e2e_command"));
    assert!(content.contains("filtered to"));
    assert!(content.contains("`@e2e`"));
}

#[then("the guidance requires the step-definition location and driving mechanism")]
fn design_requires_step_location(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("E2E step definition location"));
    assert!(content.contains("driving mechanism"));
}

#[then("the guidance requires local infrastructure and framework decisions")]
fn design_requires_infrastructure(world: &mut GivnWorld) {
    let content = design_guidance(world);
    assert!(content.contains("Local test infrastructure"));
    assert!(content.contains("E2E framework choice and justification"));
}

#[then("the guidance requires the interaction coverage matrix")]
fn design_requires_coverage_matrix(world: &mut GivnWorld) {
    assert!(design_guidance(world).contains("Interaction Coverage Matrix"));
}

fn review_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("review guidance was not loaded")
}

// Steps kept for @givn.removed placeholders — the scenarios below were
// removed from the permanent spec together with the semantic gate, but the
// removal deltas still parse and must match step definitions until the
// change is archived.

#[given("a semantic review contains classified candidates")]
fn semantic_review_contains_candidates(world: &mut GivnWorld) {
    let instruction =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/review.md")).expect("read review instruction");
    let template =
        std::fs::read_to_string(crate::common::repo_path("assets/templates/review.md")).expect("read review template");
    world.additional_output = Some(format!("{instruction}\n{template}"));
}

#[given("the review instructions are requested")]
fn review_instructions_requested(world: &mut GivnWorld) {
    let instruction =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/review.md")).expect("read review instruction");
    let template =
        std::fs::read_to_string(crate::common::repo_path("assets/templates/review.md")).expect("read review template");
    world.additional_output = Some(format!("{instruction}\n{template}"));
}

#[when("the reviewer reads the deterministic gate guidance")]
fn reviewer_reads_deterministic_gate_guidance(world: &mut GivnWorld) {
    assert!(
        !review_guidance(world).is_empty(),
        "review guidance must not be empty"
    );
}

#[then("the guidance requires an overlap disposition for every shape match")]
fn guidance_requires_overlap_dispositions(world: &mut GivnWorld) {
    let content = review_guidance(world);
    assert!(
        content.contains("## Deterministic gate dispositions"),
        "review guidance missing deterministic gate dispositions section"
    );
    assert!(
        content.contains("Overlap") && content.contains("dispositions"),
        "review guidance missing overlap dispositions table requirement"
    );
    assert!(
        content.contains("shape match"),
        "review guidance does not tie dispositions to shape matches"
    );
}

#[then("the guidance requires a split-or-keep decision for every long scenario")]
fn guidance_requires_split_or_keep(world: &mut GivnWorld) {
    assert!(
        review_guidance(world).contains("split-or-keep"),
        "review guidance missing split-or-keep decision requirement"
    );
}

fn instruction_json(world: &GivnWorld) -> serde_json::Value {
    let output = world
        .last_output
        .as_ref()
        .expect("instruction command was not run");
    assert!(output.status.success(), "instruction command failed");
    serde_json::from_slice(&output.stdout).expect("instruction output is not JSON")
}

#[given("the effective specs artifact metadata is available")]
fn effective_specs_metadata_available(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "instruction-integrity"]);
    assert!(output.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("an agent inspects the specs instructions as JSON")]
fn agent_inspects_specs_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("instruction fixture is missing");
    world.last_output = Some(crate::common::run_givn(
        env,
        &[
            "instructions",
            "specs",
            "--change",
            "instruction-integrity",
            "--json",
        ],
    ));
}

#[then("the response identifies the artifact")]
fn response_identifies_artifact(world: &mut GivnWorld) {
    assert_eq!(instruction_json(world)["id"], "specs");
}

#[then("the response lists the generated paths and prerequisites")]
fn response_lists_paths_and_prerequisites(world: &mut GivnWorld) {
    let value = instruction_json(world);
    assert!(value["generates"].is_array());
    assert!(value["requires"].is_array());
}

#[then("the response contains the resolved instruction text")]
fn response_contains_instruction_text(world: &mut GivnWorld) {
    assert!(instruction_json(world)["instruction"].is_string());
}

#[when("an agent requests the specs instructions for an unsafe change context")]
fn agent_requests_unsafe_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("instruction fixture is missing");
    world.last_output = Some(crate::common::run_givn(
        env,
        &["instructions", "specs", "--change", "../unsafe"],
    ));
}

#[then("the Givn CLI rejects the request with a clear kebab-case error")]
fn unsafe_change_is_rejected(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("unsafe-change command was not run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("kebab-case"), "{stderr}");
}

const OVERRIDE_SENTINEL: &str = "PROJECT SPECS OVERRIDE SENTINEL";

#[given("a project specs instruction override is prepared")]
fn project_specs_override_prepared(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "instruction-integrity"]);
    assert!(output.status.success(), "create fixture change failed");
    crate::common::write_override(
        &env,
        "specs",
        "instructions.md",
        &format!("# {OVERRIDE_SENTINEL}\n"),
    );
    world.env = Some(env);
}

#[when("the agent synchronizes skills and requests the specs instructions")]
fn agent_syncs_and_requests_specs(world: &mut GivnWorld) {
    let sync_output = {
        let env = world.env.as_ref().expect("override fixture is missing");
        crate::common::run_givn(env, &["skills", "sync"])
    };
    assert!(sync_output.status.success(), "skill sync failed");
    world.previous_output = Some(sync_output);
    let instruction_output = {
        let env = world.env.as_ref().expect("override fixture is missing");
        crate::common::run_givn(
            env,
            &["instructions", "specs", "--change", "instruction-integrity"],
        )
    };
    world.last_output = Some(instruction_output);
}

#[then("the resolved instruction contains the project override")]
fn resolved_instruction_contains_override(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("override instruction command was not run");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(OVERRIDE_SENTINEL));
}

#[then("the project override file remains unchanged")]
fn project_override_remains_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("override fixture is missing");
    let content = std::fs::read_to_string(env.path("givn/artifacts/specs/instructions.md"))
        .expect("read project specs override");
    assert_eq!(content, format!("# {OVERRIDE_SENTINEL}\n"));
}

fn specs_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("specs guidance was not loaded")
}

#[given("the synchronized agent guidance for the specs workflow")]
fn synchronized_specs_guidance(world: &mut GivnWorld) {
    let specs =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/specs.md")).expect("read specs instruction");
    let command =
        std::fs::read_to_string(crate::common::repo_path("assets/commands/givn-spec.md")).expect("read specs command");
    let skill =
        std::fs::read_to_string(crate::common::repo_path("assets/skills/givn-spec/SKILL.md.tmpl")).expect("read specs skill");
    world.additional_output = Some(format!("{specs}\n{command}\n{skill}"));
}

#[when("an author reads the guidance")]
fn author_reads_guidance(world: &mut GivnWorld) {
    assert!(
        !specs_guidance(world).is_empty(),
        "guidance must not be empty"
    );
}

#[then("the guidance does not mention engine-backed retrieval commands")]
fn guidance_omits_engine_commands(world: &mut GivnWorld) {
    let content = specs_guidance(world);
    for command in [
        "givn spec index",
        "givn spec search",
        "givn spec explore",
        "givn spec review",
    ] {
        assert!(
            !content.contains(command),
            "guidance still mentions retired engine command `{command}`"
        );
    }
}

#[then("the guidance does not offer advisory or blocking retrieval paths")]
fn guidance_omits_retrieval_paths(world: &mut GivnWorld) {
    let content = specs_guidance(world);
    for marker in [
        "retrieval-unavailable",
        "Retrieval-aware authoring",
        "BGE_TOKEN_CAP",
        "NLI_TOKEN_CAP",
    ] {
        assert!(
            !content.contains(marker),
            "guidance still offers a retrieval path via `{marker}`"
        );
    }
}

#[then("the guidance keeps the deterministic spec toolchain wording")]
fn guidance_keeps_deterministic_toolchain(world: &mut GivnWorld) {
    let content = specs_guidance(world);
    for marker in ["@givn.delta", "givn lint", "givn/changes/"] {
        assert!(
            content.contains(marker),
            "guidance lost the deterministic toolchain wording `{marker}`"
        );
    }
}

// --- dialog-question-quality: dialog policy steps --------------------------

fn cli_stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(
        output.status.success(),
        "CLI command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[given("a Givn project with an active change for the dialog policy")]
fn dialog_policy_project(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "dialog-question-quality"]);
    assert!(output.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("the author requests the dialog policy through the Givn CLI")]
fn author_requests_dialog_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dialog fixture is missing");
    world.last_output = Some(crate::common::run_givn(
        env,
        &[
            "instructions",
            "dialog",
            "--change",
            "dialog-question-quality",
        ],
    ));
}

#[then("the CLI output contains the named-artifact rule")]
fn cli_contains_named_artifact_rule(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("domain language") && output.contains("identifier in parentheses"),
        "dialog policy missing the named-artifact rule: {output}"
    );
}

#[then("the CLI output contains the option treatment rule")]
fn cli_contains_option_treatment_rule(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "what it means",
        "what it gains",
        "what it costs",
        "concrete example",
    ] {
        assert!(
            output.contains(marker),
            "dialog policy missing option treatment marker {marker:?}: {output}"
        );
    }
}

#[then("the CLI output contains the standalone-answerability rule")]
fn cli_contains_answerability_rule(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("answerable standalone") && output.contains("without opening another file"),
        "dialog policy missing the standalone-answerability rule: {output}"
    );
}

#[then("the CLI output contains the rejected-alternative rule")]
fn cli_contains_rejected_alternative_rule(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("rejected alternatives") && output.contains("resolution"),
        "dialog policy missing the rejected-alternative rule: {output}"
    );
}

fn dialog_policy(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("dialog policy was not loaded")
}

#[given("the canonical dialog policy is requested")]
fn canonical_dialog_policy_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("assets/instructions/dialog.md"))
        .expect("read canonical dialog instruction");
    world.additional_output = Some(content);
}

#[when("an author reads the option treatment and the record shape")]
fn author_reads_option_treatment(world: &mut GivnWorld) {
    assert!(
        !dialog_policy(world).is_empty(),
        "dialog policy must not be empty"
    );
}

#[then("the policy requires every option's meaning gain cost and concrete example")]
fn policy_requires_option_treatment(world: &mut GivnWorld) {
    let content = dialog_policy(world);
    for marker in ["Meaning:", "In practice:", "Gain:", "Cost:"] {
        assert!(
            content.contains(marker),
            "dialog policy missing option treatment marker {marker:?}: {content}"
        );
    }
}

#[then("the policy requires the rejected alternatives with their reasons")]
fn policy_requires_rejected_alternatives(world: &mut GivnWorld) {
    let content = dialog_policy(world);
    assert!(
        content.contains("Rejected:") && content.contains("Resolution:"),
        "dialog policy does not record rejected alternatives with the resolution: {content}"
    );
}

#[then("the policy rejects an outcome-only resolution")]
fn policy_rejects_outcome_only_resolution(world: &mut GivnWorld) {
    assert!(
        dialog_policy(world).contains("An outcome-only resolution is not a"),
        "dialog policy does not reject an outcome-only resolution"
    );
}

#[then("the policy requires a gloss for every cross-reference")]
fn policy_requires_cross_reference_gloss(world: &mut GivnWorld) {
    assert!(
        dialog_policy(world).contains("one-line gloss on first use"),
        "dialog policy does not require a cross-reference gloss"
    );
}

#[when("the author requests the dialog policy through the Givn CLI without a change")]
fn author_requests_dialog_policy_without_change(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dialog fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["instructions", "dialog"]));
}

#[when("the author requests an unknown instruction through the Givn CLI")]
fn author_requests_unknown_instruction(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dialog fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["instructions", "not-an-artifact"]));
}

// --- use-case-model-scoping: rejection criteria and placement test steps ---

#[given("the design-review instructions are requested")]
fn design_review_instructions_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("assets/instructions/design-review.md"))
        .expect("read design-review instruction");
    world.additional_output = Some(content);
}

#[when("the reviewer reads the use-case branch")]
fn reviewer_reads_use_case_branch(world: &mut GivnWorld) {
    assert!(
        !world
            .additional_output
            .as_deref()
            .expect("design-review guidance was not loaded")
            .is_empty(),
        "design-review guidance must not be empty"
    );
}

#[then("the guidance points at the use-case scope criteria")]
fn guidance_points_at_scope_criteria(world: &mut GivnWorld) {
    let content = world
        .additional_output
        .as_deref()
        .expect("design-review guidance was not loaded");
    assert!(
        content.contains("use-case scope rejection criteria"),
        "design-review guidance does not point at the use-case scope criteria"
    );
}

#[when("the reviewer reads the use-case scope criteria")]
fn reviewer_reads_scope_criteria(world: &mut GivnWorld) {
    assert!(
        !review_guidance(world).is_empty(),
        "review guidance must not be empty"
    );
}

fn scope_criteria(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("review guidance was not loaded")
}

#[then("the guidance rejects a goal conjunction")]
fn rejects_goal_conjunction(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Goal conjunction"),
        "review guidance does not reject Goal conjunction"
    );
}

#[then("the guidance rejects a missing primary actor")]
fn rejects_missing_primary_actor(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Missing primary actor"),
        "review guidance does not reject Missing primary actor"
    );
}

#[then("the guidance rejects a capability that fails the placement test")]
fn rejects_placement_failure(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Placement failure"),
        "review guidance does not reject Placement failure"
    );
}

#[then("the guidance rejects a mutating scenario under an observing goal")]
fn rejects_mutation_under_observing_goal(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Mutation under an observing goal"),
        "review guidance does not reject Mutation under an observing goal"
    );
}

#[then("the guidance rejects an extension without a step reference")]
fn rejects_extension_without_step(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Extension without a step reference"),
        "review guidance does not reject Extension without a step reference"
    );
}

#[then("the guidance rejects a precondition established by the flow")]
fn rejects_flow_precondition(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Inconsistent preconditions"),
        "review guidance does not reject Inconsistent preconditions"
    );
}

#[then("the guidance rejects mechanism vocabulary in the goal trigger or guarantees")]
fn rejects_mechanism_vocabulary(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Mechanism vocabulary"),
        "review guidance does not reject Mechanism vocabulary"
    );
}

#[then("the guidance rejects a component- or channel-shaped name")]
fn rejects_component_channel_name(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Component- or channel-shaped name"),
        "review guidance does not reject Component- or channel-shaped name"
    );
}

#[then("the guidance rejects an oversized leaf")]
fn rejects_oversized_leaf(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Oversized leaf"),
        "review guidance does not reject Oversized leaf"
    );
}

#[then("the guidance rejects relationship misuse")]
fn rejects_relationship_misuse(world: &mut GivnWorld) {
    assert!(
        scope_criteria(world).contains("Relationship misuse"),
        "review guidance does not reject Relationship misuse"
    );
}

#[when("an author reads the capability placement guidance")]
fn author_reads_placement_guidance(world: &mut GivnWorld) {
    assert!(
        !specs_guidance(world).is_empty(),
        "specs guidance must not be empty"
    );
}

#[then("the guidance carries the placement test")]
fn guidance_carries_placement_test(world: &mut GivnWorld) {
    assert!(
        specs_guidance(world).contains("placement test"),
        "specs guidance does not carry the placement test"
    );
}

#[then("the guidance rejects a channel as an answer")]
fn guidance_rejects_channel_answer(world: &mut GivnWorld) {
    assert!(
        specs_guidance(world).contains("Channel is never an answer"),
        "specs guidance does not reject a channel as an answer"
    );
}

#[then("the guidance requires goal-shaped capability names")]
fn guidance_requires_goal_shaped_names(world: &mut GivnWorld) {
    assert!(
        specs_guidance(world).contains("goal-shaped"),
        "specs guidance does not require goal-shaped capability names"
    );
}

// --- persona-identity-reuse: persona template steps ------------------------

#[given("the persona template is inspected")]
fn persona_template_inspected(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("assets/templates/persona.md"))
        .expect("read persona template");
    world.additional_output = Some(content);
}

#[when("an author reads the persona schema")]
fn author_reads_persona_schema(world: &mut GivnWorld) {
    assert!(
        !world
            .additional_output
            .as_deref()
            .expect("persona template was not loaded")
            .is_empty(),
        "persona template must not be empty"
    );
}

#[then("the template names the authority boundary and evaluation capability")]
fn template_names_authority_and_evaluation(world: &mut GivnWorld) {
    let content = world
        .additional_output
        .as_deref()
        .expect("persona template was not loaded");
    assert!(
        content.contains("Authority boundary") && content.contains("Evaluation capability"),
        "persona template does not name the authority boundary and evaluation capability"
    );
}

#[then("the template names the own vocabulary and success criterion")]
fn template_names_vocabulary_and_success(world: &mut GivnWorld) {
    let content = world
        .additional_output
        .as_deref()
        .expect("persona template was not loaded");
    assert!(
        content.contains("Own vocabulary") && content.contains("Success criterion"),
        "persona template does not name the own vocabulary and success criterion"
    );
}

#[then("the template carries an append-only running record")]
fn template_carries_running_record(world: &mut GivnWorld) {
    let content = world
        .additional_output
        .as_deref()
        .expect("persona template was not loaded");
    assert!(
        content.contains("Running record") && content.contains("append-only"),
        "persona template does not carry an append-only running record"
    );
}

#[then("the template carries a dated topic stances appendix")]
fn template_carries_topic_stances(world: &mut GivnWorld) {
    let content = world
        .additional_output
        .as_deref()
        .expect("persona template was not loaded");
    assert!(
        content.contains("Topic stances") && content.contains("<YYYY-MM-DD>"),
        "persona template does not carry a dated topic stances appendix"
    );
}

// --- guarantee-question-traceability: instruction steps --------------------

fn traceability_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("traceability guidance was not loaded")
}

#[given("the proposal instructions are requested")]
fn proposal_instructions_requested(world: &mut GivnWorld) {
    let instruction = std::fs::read_to_string(crate::common::repo_path("assets/instructions/proposal.md"))
        .expect("read proposal instruction");
    let command = std::fs::read_to_string(crate::common::repo_path("assets/commands/givn-propose.md"))
        .expect("read propose command");
    world.additional_output = Some(format!("{instruction}\n{command}"));
}

#[when("an author reads the question disposition rule")]
fn author_reads_question_disposition(world: &mut GivnWorld) {
    assert!(
        !traceability_guidance(world).is_empty(),
        "proposal guidance must not be empty"
    );
}

#[then("the guidance requires each intersecting seed question to be dispositioned")]
fn guidance_requires_seed_disposition(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    assert!(
        content.contains("intersecting seed question") && content.contains("dispositioned"),
        "guidance does not require dispositioning intersecting seed questions"
    );
}

#[then("the guidance names the answered carried and out-of-scope dispositions")]
fn guidance_names_dispositions(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    for marker in ["answered", "carried", "out of scope"] {
        assert!(
            content.contains(marker),
            "guidance is missing the {marker:?} disposition"
        );
    }
}

#[then("the guidance rejects a bare deferral to the design")]
fn guidance_rejects_bare_deferral(world: &mut GivnWorld) {
    assert!(
        traceability_guidance(world).contains("is not a disposition"),
        "guidance does not reject a bare deferral to the design"
    );
}

#[then("the guidance assigns stable question IDs")]
fn guidance_assigns_question_ids(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    assert!(
        content.contains("`D1`") && content.contains("D<n>"),
        "guidance does not assign stable question IDs"
    );
}

#[then("the guidance marks the disposition rule advisory without an ideation source")]
fn guidance_marks_advisory_without_source(world: &mut GivnWorld) {
    assert!(
        traceability_guidance(world).contains("advisory without an ideation source"),
        "guidance does not mark the rule advisory without an ideation source"
    );
}

#[given("the design and design-review instructions are requested")]
fn design_and_design_review_requested(world: &mut GivnWorld) {
    let design = std::fs::read_to_string(crate::common::repo_path("assets/instructions/design.md"))
        .expect("read design instruction");
    let design_review =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/design-review.md"))
            .expect("read design-review instruction");
    let command =
        std::fs::read_to_string(crate::common::repo_path("assets/commands/givn-design-review.md"))
            .expect("read design-review command");
    world.additional_output = Some(format!("{design}\n{design_review}\n{command}"));
}

#[when("a maintainer reads the ledger contract")]
fn maintainer_reads_ledger_contract(world: &mut GivnWorld) {
    assert!(
        !traceability_guidance(world).is_empty(),
        "ledger guidance must not be empty"
    );
}

#[then("the guidance requires a decisions ledger mapping each question ID")]
fn guidance_requires_ledger(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    assert!(
        content.contains("ledger") && content.contains("D<n>"),
        "guidance does not require a decisions ledger mapping question IDs"
    );
}

#[then("the guidance records decisions as \"Decides D\" with their realization")]
fn guidance_records_decides(world: &mut GivnWorld) {
    assert!(
        traceability_guidance(world).contains("Decides D"),
        "guidance does not record decisions as Decides D"
    );
}

#[then("the guidance records the seed question reference beside the ledger ID")]
fn guidance_records_seed_reference(world: &mut GivnWorld) {
    assert!(
        traceability_guidance(world).contains("seed Q"),
        "guidance does not record the seed question reference beside the ledger ID"
    );
}

#[then("the design-review guidance treats an unresolved unrecorded question as a finding")]
fn design_review_treats_unresolved_as_finding(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    assert!(
        content.contains("unresolved, unrecorded") && content.contains("finding"),
        "design-review guidance does not treat an unresolved question as a finding"
    );
}

#[then("the guidance rejects an unqualified guarantee whose prerequisite is open")]
fn review_rejects_unqualified_guarantee(_world: &mut GivnWorld) {
    unimplemented!()
}
