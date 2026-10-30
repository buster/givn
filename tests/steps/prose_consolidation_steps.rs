use std::fs;

use cucumber::{given, then, when};
use givn::skills::targets::AgentTarget;

// --- decision-ready questions: delegated question preparation ---------------

pub(crate) const DELEGATING_SKILLS: [&str; 6] = [
    "givn-implement",
    "givn-design-review",
    "givn-review",
    "givn-characterize",
    "givn-ideation",
    "givn-reverse-engineer",
];

// --- decision-ready questions: project-owned replacements -------------------

pub(crate) const REPLACEMENT_SKILL: &str = "---\nname: givn-propose\ndescription: Project-owned propose skill.\n---\n\n# givn-propose\n\nProject-owned propose guidance.\n";

pub(crate) const REPLACEMENT_COMMAND: &str =
    "---\ndescription: Project-owned propose command\n---\n\nProject-owned propose command body.\n";

const MANAGED_POLICY_REFERENCE: &str = "follow the effective dialog policy from";
const MANAGED_HANDOFF_REFERENCE: &str = "when work is handed to another context";

#[given("a Givn project with all agent skill targets selected")]
fn project_with_all_skill_targets(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let selection = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(",");
    let output = crate::common::run_givn(&env, &["init", "--agents", selection.as_str()]);
    assert!(
        output.status.success(),
        "init with all targets failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[given(
    "a valid project-owned skill replacement and legacy command intentionally omit the bundled dialog obligation"
)]
fn project_replacements_omit_obligation(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let skill_path = env.path("givn/skills/givn-propose/SKILL.md.tmpl");
    std::fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("create skills dir");
    std::fs::write(&skill_path, REPLACEMENT_SKILL).expect("write replacement skill");
    let command_path = env.path("givn/commands/givn-propose.md");
    std::fs::create_dir_all(command_path.parent().expect("command parent"))
        .expect("create commands dir");
    std::fs::write(&command_path, REPLACEMENT_COMMAND).expect("write replacement command");
    assert!(
        !REPLACEMENT_SKILL.contains("givn instructions dialog")
            && !REPLACEMENT_COMMAND.contains("givn instructions dialog"),
        "the replacement fixture must intentionally omit the bundled obligation"
    );
}

#[then("the project-owned skill source and legacy command bytes remain unchanged")]
fn replacement_sources_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let skill = std::fs::read_to_string(env.path("givn/skills/givn-propose/SKILL.md.tmpl"))
        .expect("replacement skill missing");
    assert_eq!(
        skill, REPLACEMENT_SKILL,
        "the project-owned skill template bytes changed"
    );
    let command = std::fs::read_to_string(env.path("givn/commands/givn-propose.md"))
        .expect("replacement command missing");
    assert_eq!(
        command, REPLACEMENT_COMMAND,
        "the project-owned command template bytes changed"
    );
}

#[then(
    "every selected skill consumer receives the authoritative rendered replacement without a bundled prefix"
)]
fn skill_consumers_receive_replacement(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let targets = givn::config::overlay::load_skills_targets(&env.root())
        .expect("load configured skills targets");
    for target in targets {
        let rel = format!(".{target}/skills/givn-propose/SKILL.md");
        let delivered =
            std::fs::read_to_string(env.path(&rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert_eq!(
            delivered, REPLACEMENT_SKILL,
            "{rel} is not the authoritative replacement (a bundled prefix or suffix was injected)"
        );
    }
}

#[then("the legacy command remains user-owned without generated replacement")]
fn legacy_command_remains_user_owned(world: &mut GivnWorld) {
    user_owned_legacy_command_unchanged(world);
}

#[then("the managed project-start guidance still references the effective dialog policy")]
fn managed_guidance_references_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let start = agents
        .find("<!-- givn:begin:givn-project -->")
        .expect("managed block start missing");
    let end = agents
        .find("<!-- givn:end:givn-project -->")
        .expect("managed block end missing");
    let block = &agents[start..end];
    let normalized = block.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains(MANAGED_POLICY_REFERENCE)
            && normalized.contains(MANAGED_HANDOFF_REFERENCE),
        "managed project-start guidance lost the effective-policy reference: {block}"
    );
}

// --- initialize-a-project: resident rules in the project-start block --------

const RESIDENT_RULES_BEGIN: &str = "<!-- givn:resident-rules:begin -->";
const RESIDENT_RULES_END: &str = "<!-- givn:resident-rules:end -->";

/// The managed project-start block of the fixture's AGENTS.md.
fn project_start_block(world: &GivnWorld) -> String {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let agents = fs::read_to_string(env.path("AGENTS.md")).expect("read AGENTS.md");
    let start = agents
        .find("<!-- givn:begin:givn-project -->")
        .expect("managed block start missing");
    let end = agents
        .find("<!-- givn:end:givn-project -->")
        .expect("managed block end missing");
    agents[start..end].to_string()
}

#[then("the project-start block carries the resident rules section of the tasks instruction")]
fn project_start_block_carries_resident_rules(world: &mut GivnWorld) {
    let served = {
        let env = world
            .env
            .as_ref()
            .expect("a project root is required first");
        crate::steps::serve_extended_rules_steps::served_instruction(env, "tasks")
    };
    let begin = served.find(RESIDENT_RULES_BEGIN).unwrap_or_else(|| {
        panic!("the served tasks instruction is missing the {RESIDENT_RULES_BEGIN} marker")
    });
    let end = served.find(RESIDENT_RULES_END).unwrap_or_else(|| {
        panic!("the served tasks instruction is missing the {RESIDENT_RULES_END} marker")
    });
    let section = served[begin + RESIDENT_RULES_BEGIN.len()..end].trim();
    assert!(
        !section.is_empty(),
        "the resident rules section of the tasks instruction is empty"
    );
    let block = project_start_block(world);
    assert!(
        block.contains(section),
        "the project-start block does not carry the resident rules section of the tasks instruction:\
         \nblock:\n{block}\nsection:\n{section}"
    );
}

#[then("the project-start block requires the effective dialog policy")]
fn project_start_block_requires_dialog_policy(world: &mut GivnWorld) {
    let block = project_start_block(world);
    assert!(
        block.contains("effective dialog policy") && block.contains("givn instructions dialog"),
        "the project-start block does not require the effective dialog policy:\n{block}"
    );
}

#[given("a Givn project with delegated authoring and review guidance enabled")]
fn project_with_delegated_guidance(world: &mut GivnWorld) {
    let env = crate::common::make_bare_root();
    let output = crate::common::run_givn(&env, &["init", "--agents", "opencode,agents,claude"]);
    assert!(
        output.status.success(),
        "init for delegated guidance failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

#[when("the author reads the synchronized delegation instructions")]
fn author_reads_delegation_instructions(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    world.last_output = Some(crate::common::run_givn(env, &["skills", "sync"]));
    assert!(
        world
            .last_output
            .as_ref()
            .expect("sync ran")
            .status
            .success(),
        "skills sync failed"
    );
}

#[then("every question-preparing skill stays thin and directs the agent to served instructions")]
fn delegating_skills_stay_thin(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("delegation fixture is missing");
    for skill in DELEGATING_SKILLS {
        let rel = format!(".opencode/skills/{skill}/SKILL.md");
        let content = std::fs::read_to_string(env.path(&rel))
            .unwrap_or_else(|e| panic!("{rel} missing: {e}"));
        assert!(
            content.contains("givn instructions")
                && !content.contains("effective policy from `givn instructions dialog`")
                && !content.contains("does not inherit the parent's loaded policy"),
            "{rel} is not a thin pointer to served instructions: {content}"
        );
    }
}

#[then("the bootstrap owns the effective dialog policy without relying on skill inheritance")]
fn bootstrap_owns_dialog_policy(world: &mut GivnWorld) {
    let block = project_start_block(world);
    assert!(
        block.contains("effective dialog policy")
            && block.contains("givn instructions dialog")
            && block.contains("load it once per context"),
        "the bootstrap does not own the dialog policy: {block}"
    );
}
use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn target_skill_paths(world: &GivnWorld, skill: &str) -> Vec<std::path::PathBuf> {
    let targets = givn::config::overlay::load_skills_targets(&env(world).root())
        .unwrap_or_else(|error| panic!("load skills targets from givn/config.yaml: {error}"));
    targets
        .into_iter()
        .map(|target| env(world).path(&format!(".{target}/skills/givn-{skill}/SKILL.md")))
        .collect()
}

fn generated_skill_content(world: &GivnWorld, skill: &str) -> String {
    target_skill_paths(world, skill)
        .into_iter()
        .map(|path| {
            fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read generated skill {}: {error}", path.display()))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[given("a givn project with all agent skill targets enabled")]
fn project_with_all_targets(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let selection = AgentTarget::ALL
        .iter()
        .map(|target| target.name())
        .collect::<Vec<_>>()
        .join(", ");
    let output = common::run_givn_with_stdin(&env, &["init"], &format!("{selection}\n"));
    assert!(output.status.success(), "givn init failed: {:?}", output);
    world.env = Some(env);
}

#[given("a givn project with all agent skill targets enabled and an active change")]
fn project_with_all_targets_and_active_change(world: &mut GivnWorld) {
    project_with_all_targets(world);
    let env = world.env.as_ref().expect("init created the fixture");
    let output = common::run_givn(env, &["new", "add-user-auth"]);
    assert!(
        output.status.success(),
        "givn new add-user-auth failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[then(
    regex = r"^the generated givn-([a-z-]+) command directs the [a-z]+ to the ([a-z-]+) instruction$"
)]
fn generated_command_directs_to_instruction(
    world: &mut GivnWorld,
    command: String,
    instruction: String,
) {
    let path = env(world).path(&format!(".opencode/commands/givn-{command}.md"));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read generated command {}: {error}", path.display()));
    let needle = format!("givn instructions {instruction}");
    assert!(
        content.contains(&needle),
        "givn-{command} does not direct to the {instruction} instruction \
         (missing {needle:?}): {content}"
    );
}

#[then(
    regex = r"^the generated givn-([a-z-]+) guidance directs the ([a-z]+(?: [a-z]+)?) to the ([a-z-]+) instruction$"
)]
fn generated_guidance_directs_to_instruction(
    world: &mut GivnWorld,
    skill: String,
    _actor: String,
    instruction: String,
) {
    let content = generated_skill_content(world, &skill);
    let needle = format!("givn instructions {instruction}");
    assert!(
        content.contains(&needle),
        "givn-{skill} guidance does not direct to the {instruction} instruction \
         (missing {needle:?}): {content}"
    );
}

#[given(regex = r##"^a project override for specs instructions containing "([^"]+)"$"##)]
fn specs_instruction_override(world: &mut GivnWorld, text: String) {
    common::write_override(env(world), "specs", "instructions.md", &text);
}

#[when("the user reads the design and review instructions for prose-consolidation")]
fn read_design_and_review_instructions(world: &mut GivnWorld) {
    let design = common::run_givn(
        env(world),
        &["instructions", "design", "--change", "prose-consolidation"],
    );
    let review = common::run_givn(
        env(world),
        &["instructions", "review", "--change", "prose-consolidation"],
    );
    assert!(
        design.status.success(),
        "design instructions failed: {:?}",
        design
    );
    assert!(
        review.status.success(),
        "review instructions failed: {:?}",
        review
    );
    let mut combined = String::from_utf8_lossy(&design.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&review.stdout));
    world.additional_output = Some(combined);
}

#[then(regex = r##"^combined instruction output contains "([^"]+)"$"##)]
fn combined_output_contains(world: &mut GivnWorld, text: String) {
    let output = world
        .additional_output
        .as_ref()
        .expect("combined instruction output was not captured");
    assert!(
        output.contains(&text),
        "combined output missing {text:?}: {output}"
    );
}

#[then(regex = r##"^combined instruction output does not contain "([^"]+)"$"##)]
fn combined_output_does_not_contain(world: &mut GivnWorld, text: String) {
    let output = world
        .additional_output
        .as_ref()
        .expect("combined instruction output was not captured");
    assert!(
        !output.contains(&text),
        "combined output contains forbidden {text:?}: {output}"
    );
}

#[then(regex = r##"^the generated givn-([a-z-]+) guidance contains "([^"]+)"$"##)]
fn generated_guidance_contains(world: &mut GivnWorld, skill: String, text: String) {
    let content = generated_skill_content(world, &skill);
    assert!(
        content.contains(&text),
        "givn-{skill} guidance missing {text:?}: {content}"
    );
}

#[then(regex = r##"^the generated givn-([a-z-]+) command contains "([^"]+)"$"##)]
fn generated_command_contains(world: &mut GivnWorld, command: String, text: String) {
    let path = env(world).path(&format!(".opencode/commands/givn-{command}.md"));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read generated command {}: {error}", path.display()));
    assert!(
        content.contains(&text),
        "command missing {text:?}: {content}"
    );
}

#[then(regex = r##"^the generated grill-([a-z-]+) command contains "([^"]+)"$"##)]
fn generated_grill_command_contains(world: &mut GivnWorld, command: String, text: String) {
    let path = env(world).path(&format!(".opencode/commands/grill-{command}.md"));
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read generated command {}: {error}", path.display()));
    assert!(
        content.contains(&text),
        "command missing {text:?}: {content}"
    );
}

#[then(regex = r##"^generated guidance does not contain "([^"]+)"$"##)]
fn generated_guidance_does_not_contain(world: &mut GivnWorld, text: String) {
    let mut content = String::new();
    for skill in [
        "spec",
        "design",
        "review",
        "steps",
        "tasks",
        "implement",
        "design-review",
    ] {
        content.push_str(&generated_skill_content(world, skill));
    }
    let command_dir = env(world).path(".opencode/commands");
    if let Ok(entries) = fs::read_dir(command_dir) {
        for entry in entries.flatten() {
            if let Ok(value) = fs::read_to_string(entry.path()) {
                content.push_str(&value);
            }
        }
    }
    assert!(
        !content.contains(&text),
        "generated guidance contains forbidden {text:?}"
    );
}

// --- prepare-0-9-10: canonical E2E, seed stamps and thin guidance -----------

fn normalized_asset(relative: &str) -> String {
    let content = std::fs::read_to_string(crate::common::repo_path(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"));
    content.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn loaded_guidance(world: &GivnWorld) -> String {
    let content = match world.additional_output.as_deref() {
        Some(content) => content.to_string(),
        None => crate::steps::stdout(world),
    };
    content.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[given("a project whose product is a library without a service or browser")]
fn project_with_library_product(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    world.additional_output = Some(normalized_asset("assets/instructions/specs.md"));
}

#[when("the author reads the canonical E2E guidance")]
fn author_reads_canonical_e2e_guidance(world: &mut GivnWorld) {
    assert!(!loaded_guidance(world).is_empty(), "E2E guidance is empty");
}

#[then("consumer calls to the public library API may run in process as real-interface evidence")]
fn library_api_calls_may_run_in_process(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content.contains("public library API"),
        "E2E guidance does not accept a public library API"
    );
    assert!(
        content.contains("may run in process"),
        "E2E guidance does not accept in-process library calls"
    );
}

#[then("the policy does not require artificial services containers or browser infrastructure")]
fn policy_requires_no_artificial_infrastructure(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content
            .contains("do not require artificial services, containers, or browser infrastructure"),
        "E2E guidance demands infrastructure the product does not have"
    );
}

#[given("regular and E2E verification share one runner command")]
fn shared_verification_runner(world: &mut GivnWorld) {
    let combined = [
        "assets/instructions/specs.md",
        "assets/instructions/design.md",
        "assets/instructions/tasks.md",
        "assets/instructions/review.md",
    ]
    .iter()
    .map(|relative| normalized_asset(relative))
    .collect::<Vec<_>>()
    .join("\n");
    world.additional_output = Some(combined);
}

#[when("the author reads the canonical verification guidance")]
fn author_reads_canonical_verification_guidance(world: &mut GivnWorld) {
    assert!(
        !loaded_guidance(world).is_empty(),
        "verification guidance is empty"
    );
}

#[then("explicit scope selection such as GIVN_RESULT_SCOPE distinguishes the executions")]
fn explicit_scope_selection_distinguishes_executions(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world).contains("GIVN_RESULT_SCOPE"),
        "verification guidance does not name explicit scope selection"
    );
}

#[then("equal test counts or command strings do not invalidate honestly isolated scopes")]
fn equal_counts_do_not_invalidate_scopes(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content.contains("Equal test counts or identical command strings do not"),
        "verification guidance still infers isolation from command shape"
    );
}

#[then(
    "required executions and zero failed or skipped tests remain required without an extra isolation run"
)]
fn required_executions_remain_without_extra_run(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content.contains("zero failed or skipped tests"),
        "verification guidance drops the zero-failure requirement"
    );
    assert!(
        content.contains("no extra isolation run")
            || content.contains("without an extra isolation run")
            || content.contains("without adding an extra isolation run"),
        "verification guidance adds an isolation run"
    );
}

#[given("an existing E2E scenario is shown to be a variant rather than a distinct action")]
fn existing_e2e_scenario_is_a_variant(world: &mut GivnWorld) {
    let combined = [
        "assets/instructions/specs.md",
        "assets/instructions/tasks.md",
    ]
    .iter()
    .map(|relative| normalized_asset(relative))
    .collect::<Vec<_>>()
    .join("\n");
    world.additional_output = Some(combined);
}

#[when("the author reads the canonical reclassification guidance")]
fn author_reads_reclassification_guidance(world: &mut GivnWorld) {
    assert!(
        !loaded_guidance(world).is_empty(),
        "reclassification guidance is empty"
    );
}

#[then(
    "reclassification requires a reason and aligned interaction inventory tasks and review records"
)]
fn reclassification_requires_reason_and_alignment(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content.contains(
            "record the reason and align the interaction inventory, tasks, and review records"
        ),
        "reclassification does not require a reason and aligned records"
    );
}

#[then("every retained real user action still has exactly one E2E smoke scenario")]
fn retained_actions_keep_one_smoke(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world)
            .contains("every retained real user action still has exactly one `@e2e` smoke"),
        "reclassification loses one-smoke coverage"
    );
}

#[then("tags cannot be removed merely to evade verification")]
fn tags_cannot_be_removed_to_evade(world: &mut GivnWorld) {
    let content = loaded_guidance(world);
    assert!(
        content.contains("tags are never removed merely to evade verification")
            || content.contains("Do not remove an `@e2e` tag merely to bypass a gate"),
        "guidance permits removing tags to evade verification"
    );
}

#[then("regular scenarios protect requirements distinct from the retained smoke")]
fn regular_scenarios_remain_distinct(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world).contains(
            "regular scenarios still protect requirements distinct from the retained smoke"
        ),
        "reclassification collapses regular requirements into the smoke"
    );
}

#[then(
    "the managed bootstrap still requires loading the effective dialog policy before the context's first question"
)]
fn bootstrap_still_requires_dialog_policy(world: &mut GivnWorld) {
    let block = project_start_block(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        block.contains("load it once per context before the first question")
            || block.contains(
                "loading the effective dialog policy before the context's first question"
            ),
        "bootstrap lost the dialog obligation: {block}"
    );
    assert!(
        block.contains("givn instructions dialog"),
        "bootstrap does not name the effective dialog policy: {block}"
    );
}

#[then("core skills contain no repeated policy-loading boilerplate")]
fn core_skills_contain_no_policy_boilerplate(world: &mut GivnWorld) {
    let root = env(world).root();
    let targets = givn::config::overlay::load_skills_targets(&root).expect("load skills targets");
    for target in targets {
        let dir = root.join(format!(".{target}/skills"));
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path().join("SKILL.md");
            let content = fs::read_to_string(&path).unwrap_or_default();
            let normalized = content.split_whitespace().collect::<Vec<_>>().join(" ");
            assert!(
                !normalized.contains("givn instructions dialog"),
                "{} repeats the dialog-loading policy instead of pointing to the instruction",
                path.display()
            );
            assert!(
                !normalized.contains("presenting agent remains responsible"),
                "{} repeats the delegation policy",
                path.display()
            );
        }
    }
}

#[given("valid project-owned skill replacements intentionally omit bundled dialog-loading text")]
fn replacements_omit_bundled_dialog_text(world: &mut GivnWorld) {
    project_replacements_omit_obligation(world);
}

#[then("project-owned skill source bytes remain unchanged")]
fn project_owned_skill_bytes_unchanged(world: &mut GivnWorld) {
    replacement_sources_unchanged(world);
}

#[then(
    "every selected skill consumer receives the authoritative replacement without a bundled prefix"
)]
fn every_skill_consumer_receives_replacement(world: &mut GivnWorld) {
    skill_consumers_receive_replacement(world);
}

#[then("user-owned legacy command files remain unchanged without being generated or synchronized")]
fn user_owned_legacy_command_unchanged(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("replacement fixture is missing");
    let path = env.path("givn/commands/givn-propose.md");
    let source = fs::read_to_string(&path).expect("user-owned legacy command missing");
    assert_eq!(
        source, REPLACEMENT_COMMAND,
        "the user-owned command changed"
    );
    assert!(
        !env.path(".opencode/commands/givn-propose.md").exists(),
        "a generated command was installed beside the user-owned legacy command"
    );
}

#[then(
    "browser E2E guidance accepts a real browser rendering static content as interface evidence"
)]
fn browser_guidance_accepts_static_content(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world)
            .contains("a real browser rendering static content is valid interface evidence"),
        "browser E2E guidance rejects static rendered content"
    );
}

#[then("it requires state changes only when the product interaction actually changes state")]
fn rendered_state_change_only_when_interaction_changes(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world).contains(
            "Require a rendered-state change only when the product interaction actually changes state"
        ),
        "browser E2E guidance requires a state change for every interaction"
    );
}

#[then("it does not require artificial clocks counters or asynchronous content")]
fn no_artificial_rendered_state(world: &mut GivnWorld) {
    assert!(
        loaded_guidance(world)
            .contains("do not require artificial clocks, counters, or asynchronous content"),
        "browser E2E guidance manufactures a rendered-state change"
    );
}

#[then("managed project-start guidance still references the effective dialog policy")]
fn managed_project_start_references_policy(world: &mut GivnWorld) {
    managed_guidance_references_policy(world);
}

// --- proportionate entry: retired explore output ---------------------------

const GENERATED_EXPLORE_SKILL: &str = "---\nname: givn-explore\ndescription: fixture\n---\n\n# givn-explore\n\nThe rules for this step are served by `givn instructions explore` — load the instruction and follow it.\n";

fn write_project_file(world: &GivnWorld, rel: &str, content: &str) {
    let path = env(world).path(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, content).expect("write project file");
}

#[given(regex = r#"^a recognizable generated explore skill at "([^"]+)"$"#)]
fn recognizable_generated_explore_skill(world: &mut GivnWorld, rel: String) {
    write_project_file(world, &rel, GENERATED_EXPLORE_SKILL);
}

#[given(regex = r#"^a custom explore skill at "([^"]+)" with content "([^"]+)"$"#)]
fn custom_explore_skill(world: &mut GivnWorld, rel: String, content: String) {
    write_project_file(world, &rel, &content);
}

#[given(regex = r#"^a recognizable generated explore command at "([^"]+)"$"#)]
fn generated_explore_command(world: &mut GivnWorld, rel: String) {
    write_project_file(
        world,
        &rel,
        "---\ndescription: fixture\n---\n\nGenerated by givn\n",
    );
}

#[given(
    "a committed project with a tracked generated explore skill and the versioned rules policy"
)]
fn committed_project_with_tracked_explore(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let init = common::run_givn(&env, &["init", "--no-addons", "--agents", "opencode"]);
    assert!(
        init.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    let skill = env.path(".opencode/skills/givn-explore/SKILL.md");
    if let Some(parent) = skill.parent() {
        fs::create_dir_all(parent).expect("create explore skill dir");
    }
    fs::write(&skill, GENERATED_EXPLORE_SKILL).expect("write generated explore skill");
    let config_path = env.path("givn/config.yaml");
    let config = fs::read_to_string(&config_path).expect("read config");
    let mut replaced = false;
    let lines: Vec<String> = config
        .lines()
        .map(|line| {
            if !replaced
                && !line.trim_start().starts_with('#')
                && line.trim_start().starts_with("rules:")
            {
                replaced = true;
                "rules: versioned".to_string()
            } else {
                line.to_string()
            }
        })
        .collect();
    assert!(replaced, "fixture config must declare rules");
    fs::write(&config_path, format!("{}\n", lines.join("\n"))).expect("write config");

    common::git_init(&env);
    world.env = Some(env);
}
