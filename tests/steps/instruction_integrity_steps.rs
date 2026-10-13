//! Step-definition skeleton for the instruction-integrity capability.
//!
//! RED scenarios intentionally remain undefined until each scenario enters its
//! own RED -> GREEN -> REFACTOR cycle.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};
use givn::skills::targets::AgentTarget;

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
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/design.md"))
            .expect("read design instruction");
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
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/review.md"))
            .expect("read review instruction");
    let template = std::fs::read_to_string(crate::common::repo_path("assets/templates/review.md"))
        .expect("read review template");
    world.additional_output = Some(format!("{instruction}\n{template}"));
}

#[given("the review instructions are requested")]
fn review_instructions_requested(world: &mut GivnWorld) {
    let instruction =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/review.md"))
            .expect("read review instruction");
    let template = std::fs::read_to_string(crate::common::repo_path("assets/templates/review.md"))
        .expect("read review template");
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
    let specs = std::fs::read_to_string(crate::common::repo_path("assets/instructions/specs.md"))
        .expect("read specs instruction");
    let command = std::fs::read_to_string(crate::common::repo_path("assets/commands/givn-spec.md"))
        .expect("read specs command");
    let skill = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-spec/SKILL.md.tmpl",
    ))
    .expect("read specs skill");
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

// --- instruction-integrity: served specs guidance through the CLI -----------

#[then("the returned guidance does not mention engine-backed retrieval commands")]
fn returned_guidance_omits_engine_commands(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for command in [
        "givn spec index",
        "givn spec search",
        "givn spec explore",
        "givn spec review",
    ] {
        assert!(
            !output.contains(command),
            "returned guidance still mentions engine-backed retrieval command `{command}`: {output}"
        );
    }
}

#[then("the returned guidance does not offer advisory or blocking retrieval paths")]
fn returned_guidance_omits_retrieval_paths(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "retrieval-unavailable",
        "Retrieval-aware authoring",
        "BGE_TOKEN_CAP",
        "NLI_TOKEN_CAP",
    ] {
        assert!(
            !output.contains(marker),
            "returned guidance still offers a retrieval path via `{marker}`: {output}"
        );
    }
}

#[then("the returned guidance keeps the deterministic spec toolchain wording")]
fn returned_guidance_keeps_deterministic_toolchain(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in ["@givn.delta", "givn lint", "givn/changes/"] {
        assert!(
            output.contains(marker),
            "returned guidance lost the deterministic toolchain wording `{marker}`: {output}"
        );
    }
}

#[then("the returned guidance carries the placement test")]
fn returned_guidance_carries_placement_test(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("placement test"),
        "returned guidance does not carry the capability placement test: {output}"
    );
}

#[then("the returned guidance rejects a channel as an answer")]
fn returned_guidance_rejects_channel_answer(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("Channel is never an answer"),
        "returned guidance does not reject a channel as an answer: {output}"
    );
}

#[then("the returned guidance requires goal-shaped capability names")]
fn returned_guidance_requires_goal_shaped_names(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("goal-shaped"),
        "returned guidance does not require goal-shaped capability names: {output}"
    );
}

// --- instruction-integrity: generated directions resolve through the CLI ----

/// Every installed skill and command file for the project's configured
/// targets, in deterministic order.
fn installed_guidance_files(world: &GivnWorld) -> Vec<std::path::PathBuf> {
    let root = env(world).root();
    let targets =
        givn::config::overlay::load_skills_targets(&root).expect("load configured skills targets");
    let mut files = Vec::new();
    for name in targets {
        let Some(target) = AgentTarget::from_name(&name) else {
            continue;
        };
        let skills_dir = root.join(target.dir()).join("skills");
        if skills_dir.is_dir() {
            let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(&skills_dir)
                .unwrap_or_else(|error| panic!("read {}: {error}", skills_dir.display()))
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .collect();
            entries.sort();
            for entry in entries {
                let file = entry.join("SKILL.md");
                if file.is_file() {
                    files.push(file);
                }
            }
        }
        if target.owns_commands() {
            let commands_dir = root.join(target.dir()).join("commands");
            if commands_dir.is_dir() {
                let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(&commands_dir)
                    .unwrap_or_else(|error| panic!("read {}: {error}", commands_dir.display()))
                    .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                    .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
                    .collect();
                entries.sort();
                files.extend(entries);
            }
        }
    }
    files
}

/// Every instruction id named by a `givn instructions <id>` direction in
/// `content`, in occurrence order.
fn named_instruction_ids(content: &str) -> Vec<String> {
    const NEEDLE: &str = "givn instructions ";
    let mut ids = Vec::new();
    let mut rest = content;
    while let Some(position) = rest.find(NEEDLE) {
        let after = &rest[position + NEEDLE.len()..];
        let token: String = after
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
            .collect();
        if token.chars().next().is_some_and(|c| c.is_ascii_lowercase()) {
            ids.push(token);
        }
        rest = after;
    }
    ids
}

#[then("every generated skill and command names a served instruction")]
fn every_generated_file_names_instruction(world: &mut GivnWorld) {
    let files = installed_guidance_files(world);
    assert!(
        !files.is_empty(),
        "the fixture has no generated skills or commands to check"
    );
    for path in &files {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let ids = named_instruction_ids(&content);
        assert!(
            !ids.is_empty(),
            "{} names no served instruction (no `givn instructions <id>` direction)",
            path.display()
        );
    }
}

#[then("every named instruction id resolves through the Givn CLI")]
fn every_named_instruction_id_resolves(world: &mut GivnWorld) {
    let files = installed_guidance_files(world);
    let project = env(world);
    let mut checked: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for path in &files {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for id in named_instruction_ids(&content) {
            if !checked.insert(id.clone()) {
                continue;
            }
            let output = crate::common::run_givn(project, &["instructions", &id]);
            assert!(
                output.status.success(),
                "`givn instructions {id}` (named by {}) failed: {}",
                path.display(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    assert!(
        !checked.is_empty(),
        "the generated guidance names no instruction ids at all"
    );
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
    world.last_output = Some(crate::common::run_givn(env, &["instructions", "dialog"]));
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

// --- decision-ready questions: canonical policy contract --------------------

#[then(
    "the returned policy applies to every user-facing question inside and outside a named workflow"
)]
fn returned_policy_is_universal(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("every user-facing question")
            && output.contains("with or without a named workflow"),
        "returned policy does not state its universal scope: {output}"
    );
}

#[then(
    "the returned policy assumes project familiarity without assuming knowledge of every referenced artifact"
)]
fn returned_policy_assumes_project_familiarity(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("familiar with the project")
            && output.contains("referenced artifact in mind"),
        "returned policy does not state its audience: {output}"
    );
}

#[then(
    "the returned policy requires the decision subject brief sourced context and concrete consequences for each answer"
)]
fn returned_policy_requires_decision_subject(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "Show the decision subject",
        "State the context briefly",
        "Point to a source",
        "Make every answer consequential",
    ] {
        assert!(
            output.contains(marker),
            "returned policy is missing the {marker:?} obligation: {output}"
        );
    }
}

#[then(
    "the returned policy requires material later-change costs and the benefit purchased by difficult commitments"
)]
fn returned_policy_requires_material_cost_disclosure(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("Disclose material later-change costs")
            && output.contains("benefit purchased by the hard-to-change commitment"),
        "returned policy does not require material-cost disclosure: {output}"
    );
}

#[then(
    "the returned policy directs the author to settle discoverable facts and confirmed constraints before asking"
)]
fn returned_policy_requires_research_before_asking(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("discoverable facts and confirmed constraints before asking"),
        "returned policy does not require research before asking: {output}"
    );
}

#[then("the returned policy distinguishes authoring obligations from runtime enforcement")]
fn returned_policy_distinguishes_authoring(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("authoring obligation, not runtime enforcement"),
        "returned policy does not distinguish authoring from enforcement: {output}"
    );
}

// --- decision-ready questions: the full question record ---------------------

#[given("an initialized Givn project")]
fn initialized_givn_project(world: &mut GivnWorld) {
    world.env = Some(crate::common::make_init_root());
}

#[then(
    "the decision-record guidance keeps the complete context alternatives consequences and resolution"
)]
fn record_guidance_keeps_complete_record(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "- Blocks:",
        "- Context:",
        "- Question:",
        "- Options:",
        "- Resolution:",
    ] {
        assert!(
            output.contains(marker),
            "record guidance is missing the {marker:?} field: {output}"
        );
    }
}

#[then("the chosen and rejected alternatives retain their reasons after the question closes")]
fn record_guidance_retains_reasons(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("Chosen:") && output.contains("Rejected:"),
        "record guidance does not retain chosen and rejected reasons: {output}"
    );
}

#[then("decision-relevant cross-references carry their meaning rather than only their identifier")]
fn record_guidance_glosses_cross_references(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("one-line gloss on first use"),
        "record guidance does not require cross-reference glosses: {output}"
    );
}

#[then("routine information requests are not forced into invented alternatives or tradeoffs")]
fn record_guidance_keeps_clarification_proportional(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("does not invent alternatives or trade-offs"),
        "policy forces routine information requests into invented alternatives: {output}"
    );
}

#[then(
    "confirmations disclose both acceptance and revision consequences without treating consent as a rule waiver"
)]
fn record_guidance_bounds_confirmation_consent(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("Confirmations show both accept and revise outcomes")
            && output.contains("does not waive"),
        "policy does not bound what confirmation consent authorizes: {output}"
    );
}

// --- decision-ready questions: teaching contrasts ---------------------------

/// Slice one worked-contrast section out of the returned policy.
fn example_section(policy: &str, n: usize) -> &str {
    let marker = format!("### Example {n} decision record");
    let start = policy
        .find(&marker)
        .unwrap_or_else(|| panic!("policy is missing {marker:?}"));
    let rest = &policy[start..];
    let end = rest[marker.len()..]
        .find("### Example ")
        .map(|offset| marker.len() + offset)
        .unwrap_or(rest.len());
    &rest[..end]
}

fn all_example_sections(world: &GivnWorld) -> Vec<String> {
    let output = cli_stdout(world);
    (1..=5)
        .map(|n| example_section(&output, n).to_string())
        .collect()
}

#[then(
    "the returned teaching examples identify their records as hypothetical rather than current product behavior"
)]
fn teaching_examples_are_hypothetical(world: &mut GivnWorld) {
    for (index, section) in all_example_sections(world).iter().enumerate() {
        assert!(
            section.contains("Hypothetical"),
            "example {} does not label its record hypothetical: {section}",
            index + 1
        );
    }
}

#[then(
    "the goal-confirmation contrast displays every proposed goal and bounds what acceptance authorizes"
)]
fn goal_confirmation_contrast_is_bounded(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[0];
    for goal in [
        "Reserve an available copy",
        "Cancel one's own reservation",
        "View one's queue position",
    ] {
        assert!(
            section.contains(goal),
            "example 1 does not display the proposed goal {goal:?}"
        );
    }
    let goal_rows = section
        .lines()
        .map(|line| line.trim_start_matches('>').trim())
        .filter(|line| line.starts_with("| "))
        .filter(|line| !line.contains("Proposed goal") && !line.contains("|---"))
        .count();
    assert_eq!(
        goal_rows, 3,
        "example 1 must display exactly the three supplied goal rows: {section}"
    );
    assert!(
        section.contains("does not approve implementation")
            && section.contains("Adopt this cut")
            && section.contains("Revise the cut first"),
        "example 1 does not bound what acceptance authorizes: {section}"
    );
}

#[then(
    "the clarification contrast asks only the missing human intent and shows how the answer scopes subsequent work"
)]
fn clarification_contrast_shows_use(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[1];
    assert!(
        section.contains("Which project root should subsequent commands use?")
            && section.contains("limits the work to the catalog project"),
        "example 2 does not ask the missing intent and show its use: {section}"
    );
    assert!(
        !section.contains("Gain:") && !section.contains("Cost:"),
        "example 2 invents gains or costs for a short clarification: {section}"
    );
}

#[then(
    "the behavior-choice contrast shows the observable outcome of both answers rather than implementation labels"
)]
fn behavior_choice_contrast_shows_outcomes(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[2];
    assert!(
        section.contains("Allow general guidance without a task")
            && section.contains("Require a task for both")
            && section.contains(
                "the collaboration rules; task-specific instructions still require a task"
            )
            && section.contains("No active task"),
        "example 3 does not show both observable outcomes: {section}"
    );
}

#[then(
    "the commitment contrast shows the saved-information benefit compatibility restriction and later-change work"
)]
fn commitment_contrast_shows_material_cost(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[3];
    assert!(
        section.contains("8080")
            && section.contains("Benefit:")
            && section.contains("Cost:")
            && section.contains("not just reinstalling the old release"),
        "example 4 does not show the material commitment: {section}"
    );
}

#[then(
    "the diagnostic contrast reports a researched assertion defect rather than asking the person to inspect unseen scenarios"
)]
fn diagnostic_contrast_reports_research(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[4];
    assert!(
        section.contains("inactive project when an unrelated label contains active")
            && section.contains("I will make the assertion distinguish the actual project status"),
        "example 5 does not report the researched defect: {section}"
    );
}

#[then("every good contrast links to its own supplied decision record")]
fn every_contrast_links_its_record(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    for (index, section) in sections.iter().enumerate() {
        let link = format!("(#example-{}-decision-record)", index + 1);
        assert!(
            section.contains(&link),
            "example {} does not link its own decision record ({link})",
            index + 1
        );
    }
}

#[then(
    "the diagnostic contrast excludes an answer that violates the confirmed one-answer constraint"
)]
fn diagnostic_contrast_excludes_invalid_answer(world: &mut GivnWorld) {
    let sections = all_example_sections(world);
    let section = &sections[4];
    assert!(
        section.contains("violate the confirmed one-answer rule"),
        "example 5 does not exclude the invariant-violating answer: {section}"
    );
}

// --- decision-ready questions: material commitments -------------------------

#[when(
    "the author reads the material-commitment teaching contrast from the returned dialog instructions"
)]
fn author_reads_material_commitment_contrast(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("dialog fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["instructions", "dialog"]));
    let output = cli_stdout(world);
    assert!(
        output.contains("### Example 4 decision record"),
        "returned dialog instructions do not carry the material-commitment contrast"
    );
}

#[then(
    "the contrast distinguishes required recovery verification from already implemented recovery support"
)]
fn commitment_contrast_requires_recovery_verification(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    let section = example_section(&output, 4);
    assert!(
        section.contains("required work, not demonstrated support")
            && section.contains("separately verified reverse conversion"),
        "example 4 does not distinguish required recovery from implemented support: {section}"
    );
}

#[then(
    "the less-committing answer leaves saved projects unchanged rather than promising a reversible upgrade"
)]
fn commitment_contrast_keeps_saved_projects(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    let section = example_section(&output, 4);
    assert!(
        section.contains("still calls it Backend Port")
            && section.contains("does not introduce that format-related downgrade"),
        "example 4 does not describe the less-committing answer: {section}"
    );
    assert!(
        !section.contains("reversible") && !section.contains("recovery already works"),
        "example 4 promises an unverified reversible upgrade: {section}"
    );
}

#[then(
    "the policy requires unknown compatibility facts to be established or consent narrowed before rollout approval"
)]
fn commitment_contrast_requires_established_facts(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    let section = example_section(&output, 4);
    assert!(
        section.contains("establish them or narrow approval to investigation"),
        "example 4 does not require unknown facts to be established or consent narrowed: {section}"
    );
}

// --- decision-ready questions: general policy without a change --------------

#[then(
    "the returned general policy governs question preparation without requiring a selected change"
)]
fn general_policy_governs_question_preparation(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("every user-facing question")
            && output.contains("does not invent alternatives or trade-offs"),
        "returned general policy is not the universal/proportional question contract: {output}"
    );
}

#[then(
    "the returned policy distinguishes routine clarification from decisions that require alternatives and consequences"
)]
fn general_policy_distinguishes_clarification(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("A short information request")
            && output.contains("A substantive choice receives the full treatment"),
        "policy does not distinguish routine clarification from substantive decisions: {output}"
    );
}

// --- decision-ready questions: design guidance migration --------------------

#[when("the author requests the design instructions through the Givn CLI")]
fn author_requests_design_instructions_cli(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("design-guidance fixture is missing");
    world.last_output = Some(crate::common::run_givn(
        env,
        &[
            "instructions",
            "design",
            "--change",
            "instruction-integrity",
        ],
    ));
}

#[then(
    "the returned design guidance requires the runner scope strictness proof separate bindings real driver and local infrastructure"
)]
fn design_guidance_requires_setup_record(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "verify.e2e_command",
        "E2E strict-mode proof",
        "separate from unit/integration steps",
        "Local test infrastructure",
        "named real interface",
    ] {
        assert!(
            output.contains(marker),
            "returned design guidance is missing {marker:?}: {output}"
        );
    }
}

#[then(
    "the returned design guidance requires framework justification and a matching interaction coverage matrix"
)]
fn design_guidance_requires_framework_and_matrix(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    for marker in [
        "E2E framework choice and justification",
        "Interaction Coverage Matrix",
    ] {
        assert!(
            output.contains(marker),
            "returned design guidance is missing {marker:?}: {output}"
        );
    }
}

#[then(
    "the returned design guidance distinguishes terminal transcripts from Web UI viewport evidence"
)]
fn design_guidance_distinguishes_terminal_evidence(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("screenshot per viewport")
            && output.contains("terminal interface")
            && output.contains("transcript.txt"),
        "returned design guidance does not distinguish terminal from Web UI evidence: {output}"
    );
}

// --- decision-ready questions: review guidance migration --------------------

#[when("the reviewer requests the review instructions through the Givn CLI")]
fn reviewer_requests_review_instructions_cli(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("review-guidance fixture is missing");
    world.last_output = Some(crate::common::run_givn(
        env,
        &[
            "instructions",
            "review",
            "--change",
            "instruction-integrity",
        ],
    ));
}

#[then(
    "the returned review guidance requires a disposition for every reported overlap pair with a reason"
)]
fn review_guidance_requires_pair_dispositions(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("## Overlap dispositions")
            && output.contains("one row per finding pair")
            && output.contains("an empty or unrecognised word does not"),
        "returned review guidance does not require reasoned pair dispositions: {output}"
    );
}

#[then(
    "the returned review guidance requires a split-or-keep decision for every reported long scenario"
)]
fn review_guidance_requires_split_or_keep(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("## Split-or-keep")
            && output.contains("one row per long scenario")
            && output.contains("either `split` or `keep`"),
        "returned review guidance does not require split-or-keep decisions: {output}"
    );
}

#[then(
    "the returned review guidance distinguishes an advisory finding from completed verification evidence"
)]
fn review_guidance_distinguishes_advisory_evidence(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("advisory evidence until its disposition is recorded")
            && output.contains("completed verification evidence"),
        "returned review guidance does not distinguish advisory from completed evidence: {output}"
    );
}

// --- decision-ready questions: machine-readable startup ---------------------

#[given("an initialized Givn project with an active change awaiting specifications")]
fn initialized_project_awaiting_specs(world: &mut GivnWorld) {
    let env = crate::common::make_init_root();
    let output = crate::common::run_givn(&env, &["new", "decision-ready-json"]);
    assert!(output.status.success(), "create fixture change failed");
    world.env = Some(env);
}

#[when("the author requests the normal Givn startup instructions as JSON")]
fn author_requests_startup_json(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("startup fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["instructions", "--json"]));
}

fn startup_json(world: &GivnWorld) -> serde_json::Value {
    let output = world
        .last_output
        .as_ref()
        .expect("startup JSON was not requested");
    assert!(
        output.status.success(),
        "startup JSON command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("startup output is not JSON")
}

#[then("the response preserves the active change identity and next required artifact")]
fn startup_json_preserves_identity_and_next(world: &mut GivnWorld) {
    let value = startup_json(world);
    assert_eq!(
        value["id"], "decision-ready-json",
        "startup JSON lost the active change identity: {value}"
    );
    let next_command = value["next_command"].as_str().unwrap_or_default();
    assert!(
        next_command.contains("specs") && next_command.contains("decision-ready-json"),
        "startup JSON does not request specs for the prepared change: {value}"
    );
    assert_eq!(
        value["next_skill"], "givn-spec",
        "startup JSON does not carry the specs skill for the prepared change: {value}"
    );
}

#[then(
    "the response includes the same universal effective-dialog loading obligation in its guidance"
)]
fn startup_json_carries_dialog_obligation(world: &mut GivnWorld) {
    let value = startup_json(world);
    let overview = value["overview"].as_str().unwrap_or_default();
    assert!(
        overview.contains("load it once per context before the first question")
            && overview.contains("pass the obligation explicitly to fresh delegated contexts"),
        "startup JSON overview does not carry the dialog obligation: {overview}"
    );
}

#[then("the response does not embed a second complete dialog policy")]
fn startup_json_does_not_embed_full_policy(world: &mut GivnWorld) {
    let value = startup_json(world);
    let overview = value["overview"].as_str().unwrap_or_default();
    assert!(
        !overview.contains("### Example 1 decision record")
            && !overview.contains("## Rule 1 — Ask only necessary"),
        "startup JSON embeds a second complete dialog policy: {overview}"
    );
}

// --- decision-ready questions: addon-independent obligations ----------------

pub(crate) const CORE_SKILL_FAMILIES: [&str; 13] = [
    "givn-propose",
    "givn-spec",
    "givn-design",
    "givn-tasks",
    "givn-implement",
    "givn-review",
    "givn-archive",
    "givn-explore",
    "givn-design-review",
    "givn-steps",
    "givn-reverse-engineer",
    "givn-characterize",
    "givn-ideation",
];

pub(crate) const CORE_COMMANDS: [&str; 13] = CORE_SKILL_FAMILIES;

#[given("a Givn project with all agent skill targets selected and every optional addon disabled")]
fn project_with_all_targets_and_no_addons(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let selection = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(",");
    let output = crate::common::run_givn(
        &env,
        &["init", "--no-addons", "--agents", selection.as_str()],
    );
    assert!(
        output.status.success(),
        "addon-free init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[when("the author synchronizes the selected guidance through the Givn CLI")]
fn author_synchronizes_selected_guidance(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["skills", "sync"]));
}

#[then(
    "every available core skill and command still requires the effective dialog policy for every user-facing question"
)]
fn core_entries_require_dialog_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    let targets = givn::config::overlay::load_skills_targets(&env.root())
        .expect("load configured skills targets");
    for skill in CORE_SKILL_FAMILIES {
        for target in &targets {
            let rel = format!(".{target}/skills/{skill}/SKILL.md");
            let content = std::fs::read_to_string(env.path(&rel))
                .unwrap_or_else(|e| panic!("{rel} missing after sync: {e}"));
            assert!(
                content.contains("for every user-facing question")
                    && content.contains("effective policy from `givn instructions dialog`"),
                "{rel} does not carry the dialog obligation: {content}"
            );
        }
    }
    for command in CORE_COMMANDS {
        let rel = format!(".opencode/commands/{command}.md");
        let content = std::fs::read_to_string(env.path(&rel))
            .unwrap_or_else(|e| panic!("{rel} missing after sync: {e}"));
        assert!(
            content.contains("for every user-facing question")
                && content.contains("effective policy from `givn instructions dialog`"),
            "{rel} does not carry the dialog obligation: {content}"
        );
    }
}

#[then("guidance for disabled addons is not installed")]
fn disabled_addon_guidance_is_absent(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    for absent in [
        ".opencode/skills/givn-domain-modeling",
        ".opencode/skills/givn-dev-principles",
        ".opencode/commands/grill-me.md",
        ".opencode/commands/grill-with-docs.md",
    ] {
        assert!(
            !env.path(absent).exists(),
            "disabled-addon guidance was installed at {absent}"
        );
    }
    let content = std::fs::read_to_string(env.path(".opencode/skills/givn-propose/SKILL.md"))
        .expect("core skill missing");
    for vocabulary in [
        "arc42",
        "Absolute Mode",
        "domain-modeling",
        "dev-principles",
    ] {
        assert!(
            !content.contains(vocabulary),
            "core guidance leaks disabled-addon vocabulary {vocabulary:?}: {content}"
        );
    }
}

#[then("the core dialog obligation does not depend on the optional tone policy")]
fn core_obligation_is_addon_independent(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    let content = std::fs::read_to_string(env.path(".opencode/skills/givn-propose/SKILL.md"))
        .expect("core skill missing");
    assert!(
        content.contains("effective policy from `givn instructions dialog`")
            && !content.contains("Absolute Mode"),
        "core obligation depends on the optional tone policy: {content}"
    );
}

#[then("the selected targets and addon choices remain unchanged")]
fn selected_targets_and_addons_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("config missing");
    // Managed scaffold comments mention the addon keys; only live keys count.
    let active: String = config
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let expected = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(", ");
    assert!(
        active.contains(&format!("targets: [{expected}]")),
        "selected targets changed after sync: {active}"
    );
    for addon in [
        "arc42",
        "absolute_mode",
        "readme",
        "coverage",
        "domain_modeling",
        "dev_principles",
        "grill_me",
        "grill_with_docs",
    ] {
        assert!(
            !active.contains(&format!("{addon}: true")),
            "optional addon {addon} was enabled without a user decision: {active}"
        );
    }
}

// --- decision-ready questions: project-owned dialog override ----------------

pub(crate) const DIALOG_OVERRIDE: &str = "# Instruction: dialog\n\nProject-owned dialog policy: require human approval before broadening the\nselected project scope. Nothing else is delegated.\n";

#[given("an initialized Givn project with a project-owned dialog instruction override")]
fn initialized_project_with_dialog_override(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let output = crate::common::run_givn(&env, &["init", "--agents", "opencode,agents,claude"]);
    assert!(
        output.status.success(),
        "init for the override fixture failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    crate::common::write_override(&env, "dialog", "instructions.md", DIALOG_OVERRIDE);
    world.env = Some(env);
}

#[given("the override requires human approval before broadening the selected project scope")]
fn override_requires_human_approval(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("override fixture is missing");
    let content = std::fs::read_to_string(env.path("givn/artifacts/dialog/instructions.md"))
        .expect("dialog override missing");
    assert_eq!(
        content, DIALOG_OVERRIDE,
        "the dialog override fixture does not carry the approval requirement"
    );
}

#[when(
    "the author synchronizes guidance and requests the dialog instructions through the Givn CLI"
)]
fn synchronize_and_request_dialog(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("override fixture is missing");
    let sync = crate::common::run_givn(env, &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "skills sync failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let text = crate::common::run_givn(env, &["instructions", "dialog"]);
    assert!(
        text.status.success(),
        "text dialog retrieval failed: {}",
        String::from_utf8_lossy(&text.stderr)
    );
    world.additional_output = Some(String::from_utf8_lossy(&text.stdout).into_owned());
    world.last_output = Some(crate::common::run_givn(
        env,
        &["instructions", "dialog", "--json"],
    ));
}

#[then("the returned policy is the project-owned dialog override rather than the bundled default")]
fn returned_policy_is_the_override(world: &mut GivnWorld) {
    let text = dialog_policy(world).to_string();
    let output = cli_stdout(world);
    let value: serde_json::Value =
        serde_json::from_str(&output).expect("dialog instruction JSON expected");
    let instruction = value["instruction"].as_str().unwrap_or_default();
    for policy in [text.as_str(), instruction] {
        assert!(
            policy.contains("require human approval before broadening the"),
            "returned policy is not the project-owned override: {policy}"
        );
        assert!(
            !policy.contains("## Rule 1 — Ask only necessary"),
            "returned policy is the bundled default rather than the override: {policy}"
        );
    }
}

#[then("the project-owned override remains unchanged on disk")]
fn override_unchanged_on_disk(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("override fixture is missing");
    let content = std::fs::read_to_string(env.path("givn/artifacts/dialog/instructions.md"))
        .expect("dialog override missing");
    assert_eq!(
        content, DIALOG_OVERRIDE,
        "the project-owned override was rewritten by synchronization"
    );
}

#[then(
    "the synchronized entry guidance directs authors to the effective policy rather than a bundled policy copy"
)]
fn synchronized_entries_direct_to_effective_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("override fixture is missing");
    for rel in [
        ".opencode/skills/givn-propose/SKILL.md",
        ".claude/skills/givn-propose/SKILL.md",
        ".opencode/commands/givn-propose.md",
    ] {
        let content =
            std::fs::read_to_string(env.path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(
            content.contains("effective policy from `givn instructions dialog`"),
            "{rel} does not direct authors to the effective policy: {content}"
        );
        assert!(
            !content.contains("## Rule 1 — Ask only necessary")
                && !content.contains("### Example 1 decision record"),
            "{rel} embeds a bundled policy copy: {content}"
        );
    }
}

fn dialog_policy(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("dialog policy was not loaded")
}

#[given("the canonical dialog policy is requested")]
fn canonical_dialog_policy_requested(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/dialog.md"))
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
    world.last_output = Some(crate::common::run_givn(
        env,
        &["instructions", "not-an-artifact"],
    ));
}

// --- use-case-model-scoping: rejection criteria and placement test steps ---

#[given("the design-review instructions are requested")]
fn design_review_instructions_requested(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/instructions/design-review.md",
    ))
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
    let instruction =
        std::fs::read_to_string(crate::common::repo_path("assets/instructions/proposal.md"))
            .expect("read proposal instruction");
    let command =
        std::fs::read_to_string(crate::common::repo_path("assets/commands/givn-propose.md"))
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
    let design_review = std::fs::read_to_string(crate::common::repo_path(
        "assets/instructions/design-review.md",
    ))
    .expect("read design-review instruction");
    let command = std::fs::read_to_string(crate::common::repo_path(
        "assets/commands/givn-design-review.md",
    ))
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
fn review_rejects_unqualified_guarantee(world: &mut GivnWorld) {
    assert!(
        review_guidance(world).contains("(open: Q<N>)"),
        "review guidance does not reject an unqualified guarantee with an open prerequisite"
    );
}

// --- finding-severity-routing-signal: routing rule steps -------------------

#[when("an author reads the routing rule")]
fn author_reads_routing_rule(world: &mut GivnWorld) {
    assert!(
        !traceability_guidance(world).is_empty(),
        "proposal guidance must not be empty"
    );
}

#[then("the guidance tells the author to omit the recommendation when route reports no signal")]
fn guidance_omits_recommendation_without_signal(world: &mut GivnWorld) {
    let content = traceability_guidance(world);
    assert!(
        content.contains("no signal") && content.contains("omit the"),
        "guidance does not tell the author to omit the recommendation without signal"
    );
}

#[then("the guidance keeps the recorded decision mandatory")]
fn guidance_keeps_decision_mandatory(world: &mut GivnWorld) {
    assert!(
        traceability_guidance(world).contains("decision stays mandatory"),
        "guidance does not keep the recorded decision mandatory"
    );
}

// --- validate-generated-references: consumer-view reference check ----------

fn env(world: &GivnWorld) -> &crate::common::TestEnv {
    world.env.as_ref().expect("project fixture is missing")
}

fn configured_skill_content(world: &GivnWorld, skill: &str) -> Vec<(std::path::PathBuf, String)> {
    let root = env(world).root();
    let targets =
        givn::config::overlay::load_skills_targets(&root).expect("load configured skills targets");
    targets
        .into_iter()
        .map(|target| {
            let path = root.join(format!(".{target}/skills/{skill}/SKILL.md"));
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            (path, content)
        })
        .collect()
}

#[when("the author requests every instruction Givn prints")]
fn author_requests_every_printed_instruction(world: &mut GivnWorld) {
    const EXTRA_INSTRUCTIONS: &[&str] =
        &["review-reverse", "specs-reverse", "tasks-reverse", "dialog"];
    let env = env(world);
    let graph = crate::common::run_givn(env, &["graph", "--json"]);
    assert!(
        graph.status.success(),
        "givn graph --json failed: {}",
        String::from_utf8_lossy(&graph.stderr)
    );
    let graph_json: serde_json::Value =
        serde_json::from_slice(&graph.stdout).expect("parse givn graph --json");
    let mut ids: Vec<String> = graph_json["artifacts"]
        .as_array()
        .expect("graph JSON carries an artifacts array")
        .iter()
        .map(|artifact| {
            artifact["id"]
                .as_str()
                .expect("artifact entry carries an id")
                .to_string()
        })
        .collect();
    ids.extend(EXTRA_INSTRUCTIONS.iter().map(|id| id.to_string()));

    let mut corpus = String::new();
    for id in ids {
        let output = crate::common::run_givn(
            env,
            &["instructions", &id, "--change", "instruction-integrity"],
        );
        assert!(
            output.status.success(),
            "givn instructions {id} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        corpus.push_str(&format!("\n### instructions/{id}\n\n"));
        corpus.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    world.additional_output = Some(corpus);
}

#[then(
    "every guidance reference resolves in the consumer project or is owned by a named creation step"
)]
fn installed_guidance_references_resolve(world: &mut GivnWorld) {
    let root = env(world).root();
    let sources = crate::common::reference_check::installed_guidance(&root);
    assert!(
        !sources.is_empty(),
        "the fixture has no installed guidance to check"
    );
    let findings = crate::common::reference_check::unresolved_references(&root, &sources);
    assert!(
        findings.is_empty(),
        "unresolved guidance references:\n{}",
        crate::common::reference_check::describe(&findings)
    );
    let uncovered = crate::common::reference_check::uncovered_targets(&root, &sources);
    assert!(
        uncovered.is_empty(),
        "configured targets without synchronized guidance: {}",
        uncovered.join(", ")
    );
}

#[then("every agent target Givn supports is covered by the synchronized guidance")]
fn every_agent_target_is_covered(world: &mut GivnWorld) {
    let root = env(world).root();
    let sources = crate::common::reference_check::installed_guidance(&root);
    let mut missing = Vec::new();
    for target in AgentTarget::ALL {
        if !crate::common::reference_check::recognizes_target_dir(target.dir()) {
            missing.push(format!("{} (not recognized by the check)", target.dir()));
            continue;
        }
        let prefix = format!("{}/", target.dir());
        if !sources
            .iter()
            .any(|source| source.label.starts_with(&prefix))
        {
            missing.push(format!("{} (no synchronized guidance)", target.dir()));
        }
    }
    assert!(
        missing.is_empty(),
        "agent targets not covered by the synchronized guidance: {}",
        missing.join(", ")
    );
}

#[then(
    "every printed guidance reference resolves in the consumer project or is owned by a named creation step"
)]
fn printed_guidance_references_resolve(world: &mut GivnWorld) {
    let root = env(world).root();
    let text = world
        .additional_output
        .clone()
        .expect("printed instructions were not collected");
    let sources = vec![crate::common::reference_check::GuidanceSource {
        label: "printed instructions".to_string(),
        text,
    }];
    let findings = crate::common::reference_check::unresolved_references(&root, &sources);
    assert!(
        findings.is_empty(),
        "unresolved printed guidance references:\n{}",
        crate::common::reference_check::describe(&findings)
    );
}

#[given(regex = r##"^a givn-ideation skill override that names the missing file "([^"]+)"$"##)]
fn givn_ideation_skill_override_with_missing_file(world: &mut GivnWorld, path: String) {
    let dir = env(world).root().join("givn/skills/givn-ideation");
    std::fs::create_dir_all(&dir).expect("create the skill override directory");
    let template = format!(
        "---\nname: givn-ideation\ndescription: fixture override\n---\n\n# givn-ideation\n\nSee `{path}` for details.\n"
    );
    std::fs::write(dir.join("SKILL.md.tmpl"), template).expect("write the skill override");
}

#[given(regex = r##"^a specs instruction override that names the missing file "([^"]+)"$"##)]
fn specs_instruction_override_with_missing_file(world: &mut GivnWorld, path: String) {
    crate::common::write_override(
        env(world),
        "specs",
        "instructions.md",
        &format!("# Specs override\n\nSee `{path}` for details.\n"),
    );
}

#[then(regex = r##"^the installed reference check fails and names "([^"]+)"$"##)]
fn installed_reference_check_fails_naming(world: &mut GivnWorld, path: String) {
    let root = env(world).root();
    let sources = crate::common::reference_check::installed_guidance(&root);
    let findings = crate::common::reference_check::unresolved_references(&root, &sources);
    assert!(
        findings.iter().any(|finding| finding.reference == path),
        "expected a finding for {path}; got:\n{}",
        crate::common::reference_check::describe(&findings)
    );
}

#[then(regex = r##"^the printed reference check fails and names "([^"]+)"$"##)]
fn printed_reference_check_fails_naming(world: &mut GivnWorld, path: String) {
    let root = env(world).root();
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    let sources = vec![crate::common::reference_check::GuidanceSource {
        label: "instructions/specs".to_string(),
        text: String::from_utf8_lossy(&output.stdout).into_owned(),
    }];
    let findings = crate::common::reference_check::unresolved_references(&root, &sources);
    assert!(
        findings.iter().any(|finding| finding.reference == path),
        "expected a finding for {path}; got:\n{}",
        crate::common::reference_check::describe(&findings)
    );
}

#[when(regex = r##"^the skills of the agent target "([^"]+)" are removed$"##)]
fn skills_of_agent_target_are_removed(world: &mut GivnWorld, target: String) {
    assert!(
        AgentTarget::from_name(&target).is_some(),
        "unknown agent target '{target}'"
    );
    let dir = env(world).root().join(&target).join("skills");
    std::fs::remove_dir_all(&dir)
        .unwrap_or_else(|error| panic!("remove {}: {error}", dir.display()));
}

#[then(regex = r##"^the configured-target check fails and names the agent target "([^"]+)"$"##)]
fn configured_target_check_fails_naming(world: &mut GivnWorld, target: String) {
    assert!(
        AgentTarget::from_name(&target).is_some(),
        "unknown agent target '{target}'"
    );
    let root = env(world).root();
    let sources = crate::common::reference_check::installed_guidance(&root);
    let uncovered = crate::common::reference_check::uncovered_targets(&root, &sources);
    assert!(
        uncovered.contains(&target),
        "expected uncovered target {target}; got: {}",
        uncovered.join(", ")
    );
}

#[then("the generated givn-characterize guidance requests `givn instructions review-reverse`")]
fn characterize_guidance_requests_reverse_review(world: &mut GivnWorld) {
    let files = configured_skill_content(world, "givn-characterize");
    assert!(
        !files.is_empty(),
        "no configured agent target carries the givn-characterize skill"
    );
    for (path, content) in files {
        assert!(
            content.contains("givn instructions review-reverse"),
            "{} does not request the reverse review through the Givn CLI",
            path.display()
        );
    }
}

#[then("the generated givn-characterize command requests `givn instructions review-reverse`")]
fn characterize_command_requests_reverse_review(world: &mut GivnWorld) {
    let root = env(world).root();
    let targets =
        givn::config::overlay::load_skills_targets(&root).expect("load configured skills targets");
    let mut checked = 0;
    for name in targets {
        let Some(target) = AgentTarget::from_name(&name) else {
            continue;
        };
        if !target.owns_commands() {
            continue;
        }
        let path = root.join(format!(".{name}/commands/givn-characterize.md"));
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        assert!(
            content.contains("givn instructions review-reverse"),
            "{} does not request the reverse review through the Givn CLI",
            path.display()
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no command-owning target carries givn-characterize"
    );
}

#[then("every command-owning agent target receives the givn-characterize command")]
fn every_command_owning_target_receives_characterize(world: &mut GivnWorld) {
    let root = env(world).root();
    for target in AgentTarget::ALL
        .iter()
        .filter(|target| target.owns_commands())
    {
        let path = root
            .join(target.dir())
            .join("commands/givn-characterize.md");
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        assert!(
            content.contains("givn instructions review-reverse"),
            "{} does not request the reverse review through the Givn CLI",
            path.display()
        );
    }
}
