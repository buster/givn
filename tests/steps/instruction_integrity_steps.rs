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
    let content = std::fs::read_to_string("assets/instructions/specs.md")
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
        std::fs::read_to_string("assets/instructions/design.md").expect("read design instruction");
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
        std::fs::read_to_string("assets/instructions/review.md").expect("read review instruction");
    let template =
        std::fs::read_to_string("assets/templates/review.md").expect("read review template");
    world.additional_output = Some(format!("{instruction}\n{template}"));
}

#[given("the review instructions are requested")]
fn review_instructions_requested(world: &mut GivnWorld) {
    let instruction =
        std::fs::read_to_string("assets/instructions/review.md").expect("read review instruction");
    let template =
        std::fs::read_to_string("assets/templates/review.md").expect("read review template");
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
        std::fs::read_to_string("assets/instructions/specs.md").expect("read specs instruction");
    let command =
        std::fs::read_to_string("assets/commands/givn-spec.md").expect("read specs command");
    let skill =
        std::fs::read_to_string("assets/skills/givn-spec/SKILL.md.tmpl").expect("read specs skill");
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
    let content = std::fs::read_to_string("assets/instructions/dialog.md")
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
fn author_requests_unknown_instruction(_world: &mut GivnWorld) {
    unimplemented!()
}
