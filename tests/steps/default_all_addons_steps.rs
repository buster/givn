//! Steps for the all-addons-enabled default and effective addon policy.

use crate::common;
use crate::steps::{GivnWorld, stdout};
use cucumber::{given, then, when};
use givn::cli::addons::KNOWN_ADDONS;
use givn::config::manifest::Manifest;
use givn::config::overlay::{apply_commands, apply_config};

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn effective_manifest(world: &GivnWorld) -> Manifest {
    let environment = env(world);
    let mut manifest = Manifest::load_embedded().expect("load embedded manifest");
    let config = environment.path("givn/config.yaml");
    if config.exists() {
        let yaml = std::fs::read_to_string(&config).expect("read givn/config.yaml");
        manifest = apply_config(manifest, &yaml).expect("apply givn/config.yaml");
    }
    let commands = environment.path("givn/commands.yaml");
    if commands.exists() {
        let yaml = std::fs::read_to_string(&commands).expect("read givn/commands.yaml");
        manifest = apply_commands(manifest, &yaml).expect("apply givn/commands.yaml");
    }
    manifest
}

fn replace_addons(world: &GivnWorld, block: &str) {
    common::remove_addon_locks(env(world));
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let mut kept = Vec::new();
    let mut in_addons = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "addons:" {
            in_addons = true;
            continue;
        }
        if in_addons {
            if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                in_addons = false;
                kept.push(line.to_string());
            }
            continue;
        }
        kept.push(line.to_string());
    }
    while kept.last().is_some_and(|line| line.trim().is_empty()) {
        kept.pop();
    }
    kept.push(block.trim_end().to_string());
    std::fs::write(path, kept.join("\n") + "\n").expect("write givn/config.yaml");
}

#[when(regex = r#"^the user writes the addons block "(.+)"$"#)]
fn user_writes_addons_block(world: &mut GivnWorld, block: String) {
    replace_addons(world, &block.replace("\\n", "\n"));
}

fn assert_all_addons(world: &GivnWorld, expected: bool) {
    let manifest = effective_manifest(world);
    for addon in KNOWN_ADDONS {
        assert_eq!(
            manifest.addons.get(addon.id),
            Some(expected),
            "addon '{}' did not resolve to {expected}",
            addon.id
        );
    }
}

fn addon_free(world: &mut GivnWorld, agents: &str) {
    world.env = Some(common::make_bare_root());
    let environment = env(world);
    let output = common::run_givn(environment, &["init", "--no-addons", "--agents", agents]);
    assert!(
        output.status.success(),
        "givn init --no-addons failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[given("an initialized addon-free project selecting opencode")]
fn initialized_addon_free_project(world: &mut GivnWorld) {
    addon_free(world, "opencode");
}

#[given("an initialized addon-free project")]
fn initialized_addon_free_project_default(world: &mut GivnWorld) {
    addon_free(world, "opencode");
}

#[given("an initialized addon-free project with outline disabled")]
fn initialized_addon_free_project_outline_disabled(world: &mut GivnWorld) {
    addon_free(world, "opencode");
    replace_addons(world, "addons:\n  outline: false\n  challenge: false");
}

#[given("an initialized project without an addons block")]
fn initialized_project_without_addons_block(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let mut kept = Vec::new();
    let mut in_addons = false;
    for line in content.lines() {
        if line.trim() == "addons:" {
            in_addons = true;
            continue;
        }
        if in_addons && !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
            in_addons = false;
        }
        if !in_addons {
            kept.push(line.to_string());
        }
    }
    std::fs::write(path, kept.join("\n") + "\n").expect("write givn/config.yaml");
}

#[given("an initialized project whose only explicit addon setting disables outline")]
fn initialized_project_with_only_outline_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(world, "addons:\n  outline: false");
}

#[given("an initialized project with challenge enabled")]
fn initialized_project_with_challenge_enabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(world, "addons:\n  challenge: true");
}

#[given("a project with challenge enabled")]
fn project_with_challenge_enabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(world, "addons:\n  challenge: true");
}

#[given("a project with challenge enabled and dev_principles and architecture_principles disabled")]
fn project_with_principle_addons_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(
        world,
        "addons:\n  challenge: true\n  dev_principles: false\n  architecture_principles: false",
    );
}

#[given("a project with challenge explicitly disabled")]
fn project_with_challenge_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(world, "addons:\n  challenge: false");
}

#[given("a project with domain_modeling, arc42, dev_principles, and challenge enabled")]
fn project_with_skill_addons_enabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(
        world,
        "addons:\n  arc42: true\n  domain_modeling: true\n  dev_principles: true\n  challenge: true",
    );
}

#[given("a project with domain_modeling and challenge disabled")]
fn project_with_skill_addons_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    replace_addons(
        world,
        "addons:\n  arc42: true\n  domain_modeling: false\n  challenge: false",
    );
}

#[given("an initialized project without legacy addon settings")]
fn initialized_project_without_legacy_addon_settings(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[given(regex = r#"^a project using "([^"]+)" YAML with unrelated custom settings$"#)]
fn project_with_addon_yaml_presentation(world: &mut GivnWorld, presentation: String) {
    world.env = Some(common::make_init_root());
    let base = "givn_config_version: \"0.9.10\"\ncustom:\n  keep: value\n";
    let content = match presentation.as_str() {
        "block mapping with comments" => {
            format!("{base}addons:\n  challenge: true # keep project choice\n  outline: true\n")
        }
        "flow mapping" => format!("{base}addons: {{challenge: true, outline: true}}\n"),
        "anchors and aliases" => {
            format!("{base}addons: &defaults\n  challenge: true\n  outline: true\n")
        }
        other => panic!("unknown YAML presentation: {other}"),
    };
    std::fs::write(env(world).path("givn/config.yaml"), &content).expect("write givn/config.yaml");
    world.additional_output = Some(content);
}

#[given(regex = r#"^an initialized project with "([^"]+)" misplaced in "([^"]+)"$"#)]
fn initialized_project_with_misplaced_field(world: &mut GivnWorld, field: String, file: String) {
    world.env = Some(common::make_init_root());
    let path = env(world).path(&file);
    let mut content = std::fs::read_to_string(&path).expect("read configuration file");
    let block = match field.as_str() {
        "addons" => "addons:\n  outline: false\n",
        "workflow" => "workflow:\n  mode: manual\n",
        "verify.command" => "verify:\n  command: \"cargo test\"\n",
        "run.command" => "run:\n  command: \"true\"\n",
        other => panic!("unknown misplaced field: {other}"),
    };
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(block);
    std::fs::write(path, content).expect("write configuration file");
    world.additional_output = Some(field);
}

#[when("the user enables domain_modeling and arc42")]
fn enables_domain_modeling_and_arc42(world: &mut GivnWorld) {
    for addon in ["domain_modeling", "arc42"] {
        let output = common::run_givn(env(world), &["addons", "enable", addon]);
        assert!(
            output.status.success(),
            "givn addons enable {addon} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[when("the user synchronizes skills and lists addons")]
fn synchronizes_skills_and_lists_addons(world: &mut GivnWorld) {
    let sync = common::run_givn(env(world), &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "givn skills sync failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let list = common::run_givn(env(world), &["addons", "list"]);
    assert!(
        list.status.success(),
        "givn addons list failed: {}",
        String::from_utf8_lossy(&list.stderr)
    );
    world.last_output = Some(list);
}

#[when("the user requests effective addon state")]
fn requests_effective_addon_state(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    world.last_output = Some(output);
}

#[then("all optional addons including communication_policy and challenge are explicitly disabled")]
fn all_addons_explicitly_disabled(world: &mut GivnWorld) {
    assert_all_addons(world, false);
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    for addon in KNOWN_ADDONS {
        assert!(
            config.contains(&format!("  {}: false", addon.id)),
            "{} lacks an explicit false setting:\n{config}",
            addon.id
        );
    }
}

#[then("every current addon has an explicit false setting")]
fn every_addon_has_explicit_false(world: &mut GivnWorld) {
    all_addons_explicitly_disabled(world);
}

#[then("every current addon including communication_policy and challenge is effectively enabled")]
fn every_addon_effectively_enabled(world: &mut GivnWorld) {
    assert_all_addons(world, true);
}

#[then("`givn addons list` reports every addon disabled")]
fn addons_list_reports_every_addon_disabled(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    assert!(
        output.status.success(),
        "givn addons list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for addon in KNOWN_ADDONS {
        assert!(
            stdout.contains(&format!("[disabled]  {}", addon.id)),
            "addons list does not report {} disabled:\n{stdout}",
            addon.id
        );
    }
}

#[then("every current addon including communication_policy and challenge is reported enabled")]
fn every_addon_reported_enabled(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    assert!(
        output.status.success(),
        "givn addons list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for addon in KNOWN_ADDONS {
        assert!(
            stdout.contains(&format!("[enabled ]  {}", addon.id)),
            "addons list does not report {} enabled:\n{stdout}",
            addon.id
        );
    }
    for retired in ["absolute_mode", "grill_me", "grill_with_docs"] {
        assert!(
            !stdout.contains(retired),
            "addons list still names retired addon {retired}:\n{stdout}"
        );
    }
}

#[then("the effective outline state is enabled")]
fn effective_outline_enabled(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[enabled ]  outline"),
        "outline is not effective enabled:\n{stdout}"
    );
}

#[then("unrelated addon opt-outs remain unchanged")]
fn unrelated_addon_opt_outs_remain(world: &mut GivnWorld) {
    let manifest = effective_manifest(world);
    assert_eq!(manifest.addons.get("challenge"), Some(false));
    assert_eq!(manifest.addons.get("outline"), Some(true));
}

#[then("outline is disabled in listing, guidance, artifacts, and addon execution")]
fn outline_disabled_everywhere(world: &mut GivnWorld) {
    let manifest = effective_manifest(world);
    assert_eq!(manifest.addons.get("outline"), Some(false));
    let output = common::run_givn(env(world), &["addons", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[disabled]  outline"), "{stdout}");
}

#[then(
    "omitted addons including challenge and communication_policy use the same enabled defaults everywhere"
)]
fn omitted_addons_use_enabled_defaults(world: &mut GivnWorld) {
    let manifest = effective_manifest(world);
    assert_eq!(manifest.addons.get("challenge"), Some(true));
    assert_eq!(manifest.addons.get("communication_policy"), Some(true));
}

#[then("the command refuses the unknown addon before writing")]
fn command_refuses_unknown_addon_before_writing(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    assert!(
        !output.status.success(),
        "the addon command must be refused"
    );
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml")).expect("read config");
    assert!(
        !config.contains("grill_me") && !config.contains("grill_with_docs"),
        "a retired addon must not be written:\n{config}"
    );
}

#[then(
    "the diagnostic names challenge as the manual replacement without enabling it automatically"
)]
fn diagnostic_names_manual_challenge(world: &mut GivnWorld) {
    let error = crate::steps::stderr(world);
    assert!(error.contains("challenge"), "{error}");
    assert!(error.contains("manual"), "{error}");
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    assert!(
        !config.contains("grill_me") && !config.contains("grill_with_docs"),
        "{config}"
    );
}

#[then(
    "`givn addons enable absolute_mode` is refused as an unknown addon with manual rename guidance"
)]
fn absolute_mode_alias_refused(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "enable", "absolute_mode"]);
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("absolute_mode"), "{error}");
    assert!(error.contains("communication_policy"), "{error}");
    assert!(error.contains("manual"), "{error}");
}

#[then("only challenge is disabled or unsupported YAML is refused before any write")]
fn only_challenge_disabled_or_refused(world: &mut GivnWorld) {
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml")).expect("read config");
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    if output.status.success() {
        assert!(config.contains("challenge: false"), "{config}");
        assert!(config.contains("custom:\n  keep: value"), "{config}");
    } else {
        let original = world
            .additional_output
            .as_deref()
            .expect("the original configuration is required");
        assert_eq!(config, original, "a refused edit changed the configuration");
    }
}

#[then("no duplicate addons section or unrelated edit occurs")]
fn no_duplicate_addons_or_unrelated_edit(world: &mut GivnWorld) {
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml")).expect("read config");
    assert_eq!(config.matches("\naddons:").count(), 1, "{config}");
    assert!(config.contains("custom:\n  keep: value"), "{config}");
}

#[then("on refusal all configuration and generated files remain unchanged")]
fn refusal_leaves_everything_unchanged(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    if !output.status.success() {
        let config =
            std::fs::read_to_string(env(world).path("givn/config.yaml")).expect("read config");
        assert_eq!(config, world.additional_output.as_deref().unwrap());
    }
}

#[then(regex = r#"^the request fails clearly naming the field and its owning file "([^"]+)"$"#)]
fn request_names_field_and_owner(world: &mut GivnWorld, owner: String) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    assert!(!output.status.success(), "the request must fail");
    let error = String::from_utf8_lossy(&output.stderr);
    let field = world
        .additional_output
        .as_deref()
        .expect("the misplaced field is required");
    let key = field.split('.').next().unwrap_or(field);
    assert!(error.contains(key), "error does not name {key}: {error}");
    assert!(
        error.contains(&owner),
        "error does not name {owner}: {error}"
    );
}

#[then("the misplaced setting is not silently ignored or applied")]
fn misplaced_setting_not_ignored(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    assert!(
        !output.status.success(),
        "a misplaced setting must fail the request"
    );
    assert!(
        !stdout(world).contains("[enabled ]  outline"),
        "the request must not silently use the misplaced addons block"
    );
}

#[when("the user requests the next lifecycle instruction and synchronizes skills")]
fn requests_lifecycle_instruction_and_syncs(world: &mut GivnWorld) {
    let instruction = common::run_givn(env(world), &["instructions"]);
    assert!(
        instruction.status.success(),
        "givn instructions failed: {}",
        String::from_utf8_lossy(&instruction.stderr)
    );
    let sync = common::run_givn(env(world), &["skills", "sync"]);
    assert!(
        sync.status.success(),
        "givn skills sync failed: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    world.additional_output = Some(String::from_utf8_lossy(&instruction.stdout).into_owned());
}

#[then("the normal workflow remains available without a challenge verdict")]
fn normal_workflow_available(world: &mut GivnWorld) {
    let instruction = world
        .additional_output
        .as_deref()
        .expect("the lifecycle instruction is required");
    assert!(!instruction.trim().is_empty(), "{instruction}");
}

#[then("no givn-challenge skill is installed")]
fn no_challenge_skill_installed(world: &mut GivnWorld) {
    assert!(
        !env(world).path(".opencode/skills/givn-challenge").exists(),
        "the givn-challenge skill must not be installed while the addon is disabled"
    );
}

#[when(regex = r#"^the author requests challenge guidance for "([^"]+)"$"#)]
fn requests_challenge_guidance(world: &mut GivnWorld, _subject: String) {
    let skill =
        std::fs::read_to_string(env(world).path(".opencode/skills/givn-challenge/SKILL.md"))
            .expect("read the givn-challenge skill");
    let output = common::run_givn(env(world), &["instructions", "challenge"]);
    assert!(
        output.status.success(),
        "givn instructions challenge failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let instruction = String::from_utf8_lossy(&output.stdout).into_owned();
    world.additional_output = Some(format!("{skill}\n---INSTRUCTION---\n{instruction}"));
}

fn challenge_guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("challenge guidance is required")
}

#[then("givn-challenge directs the author to the canonical challenge instruction")]
fn challenge_skill_directs_to_instruction(world: &mut GivnWorld) {
    assert!(
        challenge_guidance(world).contains("givn instructions challenge"),
        "the givn-challenge skill does not direct the author to the served instruction"
    );
}

#[then("that instruction requires research and evidence before questions")]
fn challenge_requires_research(world: &mut GivnWorld) {
    let text = challenge_guidance(world).to_lowercase();
    assert!(text.contains("investigate first"), "{text}");
    assert!(text.contains("evidence"), "{text}");
}

#[then(
    "discoverable defects are reported as bounded actionable findings and hypotheses are labeled"
)]
fn challenge_reports_bounded_findings(world: &mut GivnWorld) {
    let text = challenge_guidance(world).to_lowercase();
    assert!(text.contains("bounded"), "{text}");
    assert!(text.contains("hypotheses"), "{text}");
}

#[then("questions are reserved for unresolved human decisions")]
fn challenge_questions_reserved(world: &mut GivnWorld) {
    assert!(
        challenge_guidance(world)
            .to_lowercase()
            .contains("unresolved human"),
        "the challenge instruction does not reserve questions for unresolved human decisions"
    );
}

#[then(
    "evidence includes counterexamples, discriminating assertions, root causes, compatibility and failure recovery where relevant"
)]
fn challenge_evidence_includes(world: &mut GivnWorld) {
    let text = challenge_guidance(world).to_lowercase();
    for needle in [
        "counterexample",
        "discriminating assertion",
        "root cause",
        "compatibility",
        "recovery",
    ] {
        assert!(
            text.contains(needle),
            "challenge guidance lacks {needle}: {text}"
        );
    }
}

#[then("performance claims require measurements")]
fn challenge_performance_requires_measurement(world: &mut GivnWorld) {
    let normalized = challenge_guidance(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        normalized.contains("Performance claims require an actual measurement"),
        "the challenge instruction does not require measurements for performance claims"
    );
}

#[then("the challenge instruction names the canonical engineering principles")]
fn challenge_names_canonical_principles(world: &mut GivnWorld) {
    let text = challenge_guidance(world);
    assert!(text.contains("givn instructions dev-principles"), "{text}");
    assert!(
        text.contains("givn instructions architecture-principles"),
        "{text}"
    );
}

#[then("the challenge instruction includes the condensed engineering principles")]
fn challenge_includes_condensed_principles(world: &mut GivnWorld) {
    let text = challenge_guidance(world);
    for needle in [
        "Correctness first; optimize for next maintainer.",
        "Answer decisions at last responsible moment.",
    ] {
        assert!(
            text.contains(needle),
            "challenge guidance lacks {needle}: {text}"
        );
    }
}

#[then("the challenge instruction requires the quality lenses")]
fn challenge_requires_quality_lenses(world: &mut GivnWorld) {
    let text = challenge_guidance(world).to_lowercase();
    for needle in [
        "quality lenses",
        "steelman",
        "test quality",
        "predicate domain",
    ] {
        assert!(
            text.contains(needle),
            "challenge guidance lacks {needle}: {text}"
        );
    }
}

#[then("the challenge instruction omits the disabled engineering principles")]
fn challenge_omits_disabled_principles(world: &mut GivnWorld) {
    let text = challenge_guidance(world);
    for needle in [
        "Correctness first; optimize for next maintainer.",
        "Answer decisions at last responsible moment.",
    ] {
        assert!(
            !text.contains(needle),
            "challenge guidance unexpectedly contains {needle}: {text}"
        );
    }
}

#[then(
    "accepted modularity and fix-forward choices are not overridden by blanket compatibility or rollback rules"
)]
fn challenge_respects_accepted_choices(world: &mut GivnWorld) {
    let text = challenge_guidance(world);
    for needle in ["modularity", "fix-forward", "blanket"] {
        assert!(
            text.contains(needle),
            "challenge guidance lacks {needle}: {text}"
        );
    }
}

#[given("recognizable generated grill-me and grill-with-docs commands beside custom commands")]
fn legacy_grill_commands_beside_custom(world: &mut GivnWorld) {
    let environment = env(world);
    for (dir, name) in [(".opencode", "grill-me"), (".opencode", "grill-with-docs")] {
        let path = environment.path(&format!("{dir}/commands/{name}.md"));
        std::fs::create_dir_all(path.parent().unwrap()).expect("create command directory");
        std::fs::write(
            &path,
            format!(
                "---\nname: {name}\ndescription: generated by an earlier Givn version\n---\n\n**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.\n"
            ),
        )
        .expect("write legacy command");
    }
    let custom = environment.path(".opencode/commands/team-release.md");
    std::fs::write(&custom, "# team-release\n\nProject-owned command.\n")
        .expect("write custom command");
}

#[then("the generated givn-challenge skill and recognizable generated legacy commands are removed")]
fn challenge_skill_and_legacy_commands_removed(world: &mut GivnWorld) {
    let environment = env(world);
    assert!(
        !environment.path(".opencode/skills/givn-challenge").exists(),
        "the givn-challenge skill was not removed"
    );
    for name in ["grill-me", "grill-with-docs"] {
        assert!(
            !environment
                .path(&format!(".opencode/commands/{name}.md"))
                .exists(),
            "the generated {name} command was not removed"
        );
    }
}

#[then("custom commands remain byte-for-byte unchanged")]
fn custom_commands_unchanged(world: &mut GivnWorld) {
    let custom = std::fs::read_to_string(env(world).path(".opencode/commands/team-release.md"))
        .expect("read custom command");
    assert_eq!(custom, "# team-release\n\nProject-owned command.\n");
}

#[then("the selected target receives the domain-modeling skill")]
fn selected_target_receives_domain_skill(world: &mut GivnWorld) {
    assert!(
        env(world)
            .path(".opencode/skills/givn-domain-modeling/SKILL.md")
            .is_file(),
        "the selected target did not receive the domain-modeling skill"
    );
}

#[then("no generated command files are written")]
fn no_generated_command_files(world: &mut GivnWorld) {
    for name in ["givn-propose", "grill-me", "grill-with-docs"] {
        assert!(
            !env(world)
                .path(&format!(".opencode/commands/{name}.md"))
                .exists(),
            "generated command {name} must not be written"
        );
    }
}

#[then("unselected targets receive no skills")]
fn unselected_targets_receive_no_skills(world: &mut GivnWorld) {
    for target in [".agents", ".claude", ".omp", ".maki"] {
        assert!(
            !env(world)
                .path(&format!("{target}/skills/givn-domain-modeling"))
                .exists(),
            "unselected target {target} received a skill"
        );
    }
}

#[then("the report names the domain-modeling, dev-principles, and givn-challenge skills")]
fn report_names_skill_addons(world: &mut GivnWorld) {
    let out = report_text(world);
    for name in [
        "givn-domain-modeling",
        "givn-dev-principles",
        "givn-challenge",
    ] {
        assert!(out.contains(&format!("synced skill: {name}")), "{out}");
    }
}

#[then("the report names no generated commands")]
fn report_names_no_commands(world: &mut GivnWorld) {
    let out = report_text(world);
    assert!(!out.contains("synced command:"), "{out}");
}

#[then("no separate grill-me or grill-with-docs guidance is generated")]
fn no_grill_guidance(world: &mut GivnWorld) {
    for path in [
        ".opencode/commands/grill-me.md",
        ".opencode/commands/grill-with-docs.md",
        ".opencode/skills/grill-me",
    ] {
        assert!(
            !env(world).path(path).exists(),
            "retired guidance exists: {path}"
        );
    }
}

#[then("no generated domain-modeling or givn-challenge skill remains")]
fn no_disabled_addon_skills(world: &mut GivnWorld) {
    for path in [
        ".opencode/skills/givn-domain-modeling",
        ".opencode/skills/givn-challenge",
    ] {
        assert!(
            !env(world).path(path).exists(),
            "disabled skill remains: {path}"
        );
    }
}

#[then("no addon command is generated or reported as synchronized")]
fn no_addon_commands_synced(world: &mut GivnWorld) {
    let out = report_text(world);
    assert!(!out.contains("synced command:"), "{out}");
}

fn report_text(world: &GivnWorld) -> String {
    world
        .last_output
        .as_ref()
        .map(|output| {
            format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        })
        .unwrap_or_default()
}

#[then(
    "the managed communication block contains concrete presentation rules without cognitive-rebuilding rhetoric"
)]
fn communication_block_rules(world: &mut GivnWorld) {
    let agents = std::fs::read_to_string(env(world).path("AGENTS.md")).expect("read AGENTS.md");
    assert!(
        agents.contains("Communication & Documentation Policy"),
        "{agents}"
    );
    assert!(agents.contains("Eliminate emojis"), "{agents}");
    assert!(
        !agents.to_lowercase().contains("cognitive rebuild"),
        "{agents}"
    );
}

fn communication_block(world: &GivnWorld) -> String {
    let agents = std::fs::read_to_string(env(world).path("AGENTS.md")).expect("read AGENTS.md");
    let start = agents
        .find("<!-- givn:begin:communication-policy -->")
        .expect("communication block start marker");
    let end = agents
        .find("<!-- givn:end:communication-policy -->")
        .expect("communication block end marker");
    agents[start..end].to_string()
}

#[then("the managed communication block requires tables for command, flag, and key mappings")]
fn communication_block_tables(world: &mut GivnWorld) {
    let block = communication_block(world);
    assert!(block.contains("**Tabulate mappings.**"), "{block}");
    assert!(block.contains("go into tables"), "{block}");
    assert!(block.contains("never narrate key sequences"), "{block}");
}

#[then("the managed communication block sends fallback and error-string rules to specs and help")]
fn communication_block_specs_boundary(world: &mut GivnWorld) {
    let block = communication_block(world);
    assert!(block.contains("**No Gherkin in docs.**"), "{block}");
    assert!(block.contains("fallback chains"), "{block}");
    assert!(
        block.contains("belong in spec files and `--help`"),
        "{block}"
    );
}

#[then("the managed communication block keeps quick start free of validation rules and exit codes")]
fn communication_block_quick_start(world: &mut GivnWorld) {
    let block = communication_block(world);
    assert!(block.contains("**Setup stays runnable.**"), "{block}");
    assert!(block.contains("exit codes"), "{block}");
    assert!(
        block.contains("edge-case flags belong in reference material"),
        "{block}"
    );
}

#[then("the managed communication block names no documentation file or README line budget")]
fn communication_block_no_layout_policy(world: &mut GivnWorld) {
    let block = communication_block(world);
    for needle in ["README.md", "docs/", "lines max", "150 lines"] {
        assert!(
            !block.contains(needle),
            "block must not name {needle}: {block}"
        );
    }
}

#[then("the managed communication block is at most 300 words")]
fn communication_block_word_budget(world: &mut GivnWorld) {
    let block = communication_block(world);
    let words = block.split_whitespace().count();
    assert!(
        words <= 300,
        "communication block is {words} words; budget is 300"
    );
}

#[then("the mandatory dialog policy remains independent of the communication addon")]
fn dialog_policy_independent(world: &mut GivnWorld) {
    let agents = std::fs::read_to_string(env(world).path("AGENTS.md")).expect("read AGENTS.md");
    assert!(agents.contains("effective dialog policy"), "{agents}");
}

#[then("no addon skills or generated commands exist")]
fn no_addon_skills_or_commands(world: &mut GivnWorld) {
    for name in [
        "givn-domain-modeling",
        "givn-dev-principles",
        "givn-challenge",
    ] {
        assert!(
            !env(world)
                .path(&format!(".opencode/skills/{name}"))
                .exists(),
            "addon skill {name} must not exist"
        );
    }
    let commands = env(world).path(".opencode/commands");
    assert!(
        !commands.exists() || std::fs::read_dir(&commands).unwrap().next().is_none(),
        "generated commands must not exist"
    );
}

#[then("the selected target receives addon skills without generated commands")]
fn selected_target_receives_addon_skills(world: &mut GivnWorld) {
    for name in [
        "givn-domain-modeling",
        "givn-dev-principles",
        "givn-challenge",
    ] {
        assert!(
            env(world)
                .path(&format!(".opencode/skills/{name}/SKILL.md"))
                .is_file(),
            "the selected target lacks {name}"
        );
    }
    no_generated_command_files(world);
}

#[then("no retired addon name is listed")]
fn no_retired_addon_name_listed(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    for retired in ["absolute_mode", "grill_me", "grill_with_docs"] {
        assert!(
            !stdout.contains(retired),
            "retired addon {retired} listed: {stdout}"
        );
    }
}

#[then("the command exits 0 and reports that outline is enabled")]
fn command_reports_outline_enabled(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("a command output is required");
    assert!(output.status.success(), "the addon command failed");
    let out = String::from_utf8_lossy(&output.stdout);
    assert!(out.contains("enabled addon 'outline'"), "{out}");
}

#[when("the user requests effective addon state or executes the configured workflow")]
fn requests_effective_addon_state_or_workflow(world: &mut GivnWorld) {
    requests_effective_addon_state(world);
}

#[then("the mandatory dialog-policy startup obligation remains installed")]
fn mandatory_dialog_obligation_installed(world: &mut GivnWorld) {
    let agents = std::fs::read_to_string(env(world).path("AGENTS.md")).expect("read AGENTS.md");
    assert!(agents.contains("effective dialog policy"), "{agents}");
}

#[then("no grill_me, grill_with_docs, or absolute_mode setting is generated")]
fn no_retired_addon_setting_generated(world: &mut GivnWorld) {
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml")).expect("read config");
    for retired in ["grill_me", "grill_with_docs", "absolute_mode"] {
        assert!(
            !config.contains(retired),
            "retired setting {retired} generated:\n{config}"
        );
    }
}

#[then("`givn addons list` reports the effective outline state as enabled")]
fn addons_list_effective_outline(world: &mut GivnWorld) {
    let output = common::run_givn(env(world), &["addons", "list"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[enabled ]  outline"), "{stdout}");
}

// --- Standalone domain documentation scaffolding ---------------------------

#[given(regex = r#"^a project file "([^"]+)" with content "([^"]+)"$"#)]
fn project_file_with_content(world: &mut GivnWorld, rel: String, content: String) {
    let path = env(world).path(&rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent directories");
    }
    std::fs::write(&path, content).expect("write project file");
}

#[then(regex = r#"^the file "([^"]+)" does not contain "([^"]+)"$"#)]
fn file_does_not_contain(world: &mut GivnWorld, rel: String, text: String) {
    let path = env(world).path(&rel);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        !content.contains(&text),
        "expected {} not to contain {:?}:\n{content}",
        rel,
        text
    );
}

// --- add-on configuration lock ---------------------------------------------

#[then(regex = r#"^the givn config keeps addon "([^"]+)" (enabled|disabled)$"#)]
fn config_keeps_addon(world: &mut GivnWorld, name: String, state: String) {
    let content = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    let parsed: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&content).expect("parse givn/config.yaml");
    let value = parsed
        .get("addons")
        .and_then(|addons| addons.get(&name))
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    assert_eq!(
        value,
        state == "enabled",
        "addon {name} is not {state}: {content}"
    );
}

#[when(regex = r#"^the project configuration sets addon "([^"]+)" to "([^"]+)"$"#)]
fn project_configuration_sets_addon(world: &mut GivnWorld, name: String, value: String) {
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let target = format!("  {name}: {value}");
    let mut found = false;
    let lines = content
        .lines()
        .map(|line| {
            if !found
                && !line.trim_start().starts_with('#')
                && line.trim_start().starts_with(&format!("{name}:"))
            {
                found = true;
                target.clone()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>();
    let mut updated = lines.join("\n");
    if !found {
        updated.push_str(&format!("\naddons:\n  {name}: {value}\n"));
    } else {
        updated.push('\n');
    }
    std::fs::write(&path, updated).expect("write givn/config.yaml");
}

#[when("the operator disables coverage with an override in the real terminal")]
fn operator_overrides_in_terminal(world: &mut GivnWorld) {
    let env = env(world);
    let command = format!(
        "{} addons disable coverage --operator --reason 'missing dependency'",
        env!("CARGO_BIN_EXE_givn")
    );
    world.last_output = Some(common::run_command_in_terminal(env, &command));
}

#[then("the change lock records the operator override reason")]
fn lock_records_override(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(env(world).path("givn/changes/addon-lock/config-lock.yaml"))
            .expect("read config-lock.yaml");
    assert!(content.contains("coverage: false"), "{content}");
    assert!(content.contains("reason: missing dependency"), "{content}");
    assert!(content.contains("from: true"), "{content}");
    assert!(content.contains("to: false"), "{content}");
}
