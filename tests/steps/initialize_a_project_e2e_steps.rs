//! E2E step definitions for the `initialize-a-project` capability.
//!
//! E2E bindings drive the real `givn init` / `givn skills sync` interfaces,
//! including the interactive terminal prompt through a real PTY.

use crate::common;
use crate::steps::GivnWorld;
use cucumber::{given, then, when};

/// Human-authored guidance that must survive initialization/synchronization.
pub(crate) const HUMAN_INSTRUCTIONS: &str =
    "# Project house rules\n\nAlways run the release checklist before tagging.\n";

fn fixture(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("initialization fixture is missing")
}

fn report(world: &GivnWorld) -> String {
    assert_command_succeeded(world);
    crate::steps::stdout(world)
}

/// Slice the managed `givn-project` block out of the fixture's AGENTS.md.
pub(crate) fn managed_project_block(world: &GivnWorld) -> String {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let start = agents
        .find("<!-- givn:begin:givn-project -->")
        .expect("managed project-start block missing");
    let end = agents
        .find("<!-- givn:end:givn-project -->")
        .expect("managed project-start block end missing");
    agents[start..end + "<!-- givn:end:givn-project -->".len()].to_string()
}

pub(crate) fn assert_command_succeeded(world: &GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(
        output.status.success(),
        "CLI command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Generated command bytes from releases before the skills-only surface.
pub(crate) const LEGACY_GENERATED_COMMAND: &str = "---\ndescription: legacy givn command\n---\n\n**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.\n";

fn write_legacy_commands(env: &common::TestEnv, target: &str, names: &[&str]) {
    let dir = env.path(&format!("{target}/commands"));
    std::fs::create_dir_all(&dir).expect("create commands dir");
    for name in names {
        std::fs::write(dir.join(format!("{name}.md")), LEGACY_GENERATED_COMMAND)
            .expect("write legacy command");
    }
}

fn write_custom_command(env: &common::TestEnv, target: &str, name: &str) {
    let dir = env.path(&format!("{target}/commands"));
    std::fs::create_dir_all(&dir).expect("create commands dir");
    std::fs::write(
        dir.join(format!("{name}.md")),
        "---\ndescription: custom project command\n---\n\nKeep this custom command.\n",
    )
    .expect("write custom command");
}

fn no_generated_commands(env: &common::TestEnv) {
    for target in [".opencode", ".agents", ".claude", ".omp", ".maki"] {
        let dir = env.path(&format!("{target}/commands"));
        let generated: Vec<_> = std::fs::read_dir(&dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
                    .filter(|path| {
                        std::fs::read_to_string(path)
                            .map(|content| {
                                content.contains(
                                    "**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`",
                                )
                            })
                            .unwrap_or(false)
                    })
                    .collect()
            })
            .unwrap_or_default();
        assert!(
            generated.is_empty(),
            "recognizable generated Givn command files remain under {target}: {generated:?}"
        );
    }
}

#[given("human-authored agent instructions outside the managed block")]
fn human_instructions_outside_block(world: &mut GivnWorld) {
    let env = fixture(world);
    std::fs::write(env.path("AGENTS.md"), HUMAN_INSTRUCTIONS).expect("write AGENTS.md");
}

#[then("the command exits 0 and reports installed skills and disabled optional addons")]
fn exits_zero_reports_skills_and_disabled_addons(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains("disabled (--no-addons)")
            && !out.contains("synced command:")
            && !out.contains("core skills and commands"),
        "the report does not name installed skills and disabled addons: {out}"
    );
}

#[then("the installed givn-propose skill directs the author to `givn instructions proposal`")]
fn installed_propose_skill_directs_to_instruction(world: &mut GivnWorld) {
    let env = fixture(world);
    let path = env.path(".opencode/skills/givn-propose/SKILL.md");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        content.contains("givn instructions proposal"),
        "the installed givn-propose skill does not direct the author to the proposal instruction:\n{content}"
    );
}

#[then("no Givn command file is generated")]
fn no_givn_command_file(world: &mut GivnWorld) {
    no_generated_commands(fixture(world));
}

#[then(
    "the managed startup block requires loading the effective dialog policy before the first question in each context independently of addons"
)]
fn startup_block_requires_dialog_independent_of_addons(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("effective dialog policy from")
            && block.contains("load it once per context before the first")
            && block.contains("optional addons disabled"),
        "the managed startup block does not own the addon-independent dialog obligation: {block}"
    );
}

#[then("human-authored instructions remain unchanged")]
fn human_instructions_remain_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert!(
        agents.contains(HUMAN_INSTRUCTIONS),
        "human-authored instructions were altered: {agents}"
    );
}

#[given("an initialized project selecting opencode and claude")]
fn initialized_project_with_opencode_and_claude(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", "opencode,claude"]);
    assert!(
        out.status.success(),
        "init for opencode and claude failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.env = Some(env);
}

#[given("an older managed startup block beside human-authored instructions")]
fn older_managed_startup_block(world: &mut GivnWorld) {
    let env = fixture(world);
    let stale = format!(
        "{HUMAN_INSTRUCTIONS}\n<!-- givn:begin:givn-project -->\nThis project uses **givn**. Old block without the dialog obligation.\n<!-- givn:end:givn-project -->\n"
    );
    std::fs::write(env.path("AGENTS.md"), stale).expect("write stale AGENTS.md");
}

#[given("recognizable Givn-generated legacy commands beside custom commands")]
fn legacy_commands_beside_custom_commands(world: &mut GivnWorld) {
    let env = fixture(world);
    write_legacy_commands(env, ".opencode", &["givn-propose", "givn-review"]);
    write_legacy_commands(env, ".claude", &["givn-propose"]);
    write_custom_command(env, ".opencode", "team-release");
}

#[then("the command exits 0 and reports the selected skill destinations")]
fn exits_zero_reports_skill_destinations(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains(".opencode/skills")
            && out.contains(".claude/skills")
            && !out.contains("synced command:"),
        "the report does not name the selected skill destinations: {out}"
    );
}

#[then("only claude receives generated Givn skills")]
fn only_claude_receives_skills(world: &mut GivnWorld) {
    let env = fixture(world);
    assert!(
        env.path(".claude/skills/givn-propose/SKILL.md").is_file(),
        "claude did not receive the givn-propose skill"
    );
    for target in [".opencode", ".agents", ".omp", ".maki"] {
        assert!(
            !env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .exists(),
            "unselected target {target} received a skill"
        );
    }
}

#[then("only selected targets receive skills")]
fn only_selected_targets_receive_skills(world: &mut GivnWorld) {
    let env = fixture(world);
    for target in [".opencode", ".claude"] {
        assert!(
            env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .is_file(),
            "{target} did not receive the givn-propose skill"
        );
    }
    for target in [".agents", ".omp", ".maki"] {
        assert!(
            !env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .exists(),
            "unselected target {target} received a skill"
        );
    }
}

#[then("recognizable generated legacy commands are removed without generating replacements")]
fn legacy_commands_removed_without_replacements(world: &mut GivnWorld) {
    let env = fixture(world);
    for rel in [
        ".opencode/commands/givn-propose.md",
        ".opencode/commands/givn-review.md",
        ".claude/commands/givn-propose.md",
    ] {
        assert!(
            !env.path(rel).exists(),
            "recognizable legacy command was not removed: {rel}"
        );
    }
    no_generated_commands(env);
}

#[then("custom commands and human-authored instructions remain byte-for-byte unchanged")]
fn custom_commands_and_human_instructions_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let custom = std::fs::read_to_string(env.path(".opencode/commands/team-release.md"))
        .expect("custom command missing");
    assert_eq!(
        custom, "---\ndescription: custom project command\n---\n\nKeep this custom command.\n",
        "the custom command was altered"
    );
    human_instructions_remain_unchanged(world);
}

#[then(
    "the startup block is refreshed once without changing the persisted selection or unrelated configuration"
)]
fn startup_block_refreshed_selection_unchanged(world: &mut GivnWorld) {
    managed_block_not_duplicated(world);
    persisted_selection_unchanged(world);
}

#[given("an initialized project with a customized legacy givn-propose command")]
fn initialized_project_with_custom_legacy_command(world: &mut GivnWorld) {
    let env = common::make_init_root();
    write_custom_command(&env, ".opencode", "givn-propose");
    world.env = Some(env);
}

#[then("the custom command remains byte-for-byte unchanged")]
fn custom_legacy_command_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let content = std::fs::read_to_string(env.path(".opencode/commands/givn-propose.md"))
        .expect("custom legacy command missing");
    assert_eq!(
        content, "---\ndescription: custom project command\n---\n\nKeep this custom command.\n",
        "the custom command was altered"
    );
}

#[then("the report names the custom command requiring manual transition")]
fn report_names_manual_transition(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("manual transition") && out.contains(".opencode/commands/givn-propose.md"),
        "the report does not name the custom command needing manual transition: {out}"
    );
}

#[then("a matching filename alone is not treated as evidence of Givn ownership")]
fn filename_is_not_ownership(world: &mut GivnWorld) {
    custom_legacy_command_unchanged(world);
}

#[then("the omp givn-propose skill and attribution integration exist")]
fn omp_skill_and_attribution_exist(world: &mut GivnWorld) {
    let env = fixture(world);
    assert!(
        env.path(".omp/skills/givn-propose/SKILL.md").is_file(),
        "the omp givn-propose skill is missing"
    );
    assert!(
        env.path(".omp/hooks/pre/givn-attribution.js").is_file(),
        "the omp attribution integration is missing"
    );
}

#[then("no target receives generated Givn commands")]
fn no_target_receives_generated_commands(world: &mut GivnWorld) {
    no_generated_commands(fixture(world));
}

#[then("no omp Givn command file exists")]
fn no_omp_command_file(world: &mut GivnWorld) {
    let env = fixture(world);
    let commands = env.path(".omp/commands");
    let files: Vec<_> = std::fs::read_dir(&commands)
        .map(|entries| entries.flatten().map(|entry| entry.path()).collect())
        .unwrap_or_default();
    assert!(files.is_empty(), "omp command files exist: {files:?}");
}

#[then("the guidance explains invocation as `/skill:givn-propose`")]
fn guidance_explains_omp_invocation(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let out = common::run_givn(env, &["instructions"]);
    let overview = String::from_utf8_lossy(&out.stdout);
    assert!(
        agents.contains("/skill:givn-<step>") && overview.contains("/skill:givn-<step>"),
        "the guidance does not explain the OMP skill invocation"
    );
}

#[then("no unselected target receives skills")]
fn no_unselected_target_receives_skills(world: &mut GivnWorld) {
    let env = fixture(world);
    for target in [".opencode", ".agents", ".claude", ".maki"] {
        assert!(
            !env.path(&format!("{target}/skills/givn-propose/SKILL.md"))
                .exists(),
            "unselected target {target} received a skill"
        );
    }
}

#[given(regex = r#"^an initialized project selecting only "?([a-z]+)"?$"#)]
fn initialized_project_selecting_only(world: &mut GivnWorld, target: String) {
    let env = common::make_bare_root();
    let out = common::run_givn(&env, &["init", "--agents", &target]);
    assert!(
        out.status.success(),
        "init for {target} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.env = Some(env);
}

#[then(
    regex = r#"^"([^"]+)" can discover the givn-propose skill and its change argument through "([^"]+)"$"#
)]
fn target_discovers_skill(world: &mut GivnWorld, target: String, invocation: String) {
    assert!(
        !invocation.trim().is_empty(),
        "no invocation was documented"
    );
    let env = fixture(world);
    let path = env.path(&format!(".{target}/skills/givn-propose/SKILL.md"));
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        content.contains("<change-id>"),
        "{target} cannot pass the change argument to givn-propose: {content}"
    );
}

#[then("the skill contains trigger metadata and the direction to `givn instructions proposal`")]
fn skill_has_metadata_and_direction(world: &mut GivnWorld) {
    let env = fixture(world);
    let content = [".opencode", ".agents", ".claude", ".omp", ".maki"]
        .iter()
        .map(|target| env.path(&format!("{target}/skills/givn-propose/SKILL.md")))
        .find(|path| path.is_file())
        .map(|path| std::fs::read_to_string(&path).expect("read givn-propose skill"))
        .expect("no selected target received the givn-propose skill");
    assert!(
        content.starts_with("---\nname: givn-propose\ndescription:")
            && content.contains("givn instructions proposal"),
        "the skill lacks trigger metadata or its instruction direction: {content}"
    );
}

#[then(
    "the skill contains no duplicated rules, orchestration, delegation, or repeated dialog-policy block"
)]
fn skill_is_thin(world: &mut GivnWorld) {
    let env = fixture(world);
    for target in [".opencode", ".agents", ".claude", ".omp", ".maki"] {
        let path = env.path(&format!("{target}/skills/givn-propose/SKILL.md"));
        if !path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&path).expect("read givn-propose skill");
        for forbidden in [
            "givn instructions dialog",
            "When work is handed to another context",
            "**Dialog policy:**",
            "## Rule ",
            "commit each finished scenario",
        ] {
            assert!(
                !content.contains(forbidden),
                "{target} givn-propose skill repeats {forbidden:?}:\n{content}"
            );
        }
        assert!(
            content.matches("givn instructions ").count() == 1,
            "{target} givn-propose skill has more than one instruction direction: {content}"
        );
    }
}

#[given("an initialized project selecting maki")]
fn initialized_project_selecting_maki(world: &mut GivnWorld) {
    initialized_project_selecting_only(world, "maki".to_string());
}

#[then("the guidance explains explicit requests and agent-selected skills")]
fn guidance_explains_maki_skills(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let normalized = agents.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("Maki uses an explicit request or an agent-selected skill"),
        "the guidance does not explain Maki skill selection: {agents}"
    );
}

#[then("the guidance does not promise a generated `/givn-propose` command")]
fn guidance_does_not_promise_command(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    let out = common::run_givn(env, &["instructions"]);
    let overview = String::from_utf8_lossy(&out.stdout);
    for guidance in [agents.as_str(), overview.as_ref()] {
        assert!(
            !guidance.contains("`/givn-propose`")
                && !guidance.contains("generated /givn-propose")
                && !guidance.contains("synced command"),
            "the guidance promises a generated `/givn-propose` command: {guidance}"
        );
    }
}

#[given(
    "an initialized project with outline and challenge explicitly disabled and other addon settings preserved"
)]
fn initialized_project_with_outline_and_challenge_disabled(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let path = env.path("givn/config.yaml");
    let mut config = std::fs::read_to_string(&path).expect("read config");
    if !config.contains("\naddons:") {
        config.push_str("\naddons:\n");
    }
    for addon in ["outline", "challenge"] {
        if config.contains(&format!("  {addon}:")) {
            config = config
                .lines()
                .map(|line| {
                    if line.trim_start().starts_with(&format!("{addon}:")) {
                        format!("  {addon}: false")
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
        } else {
            config.push_str(&format!("  {addon}: false\n"));
        }
    }
    std::fs::write(&path, config).expect("write config");
    world.env = Some(env);
}

#[then(
    "outline and challenge remain disabled in configuration, installed guidance, and the initialization report"
)]
fn disabled_addons_remain_disabled(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    for addon in ["outline", "challenge"] {
        assert!(
            config
                .lines()
                .any(|line| line.trim() == format!("{addon}: false")),
            "{addon} was re-enabled: {config}"
        );
    }
    assert!(
        !env.path(".opencode/skills/givn-challenge/SKILL.md")
            .exists(),
        "the challenge skill was installed while disabled"
    );
    let out = report(world);
    assert!(
        out.contains("challenge") && out.contains("disabled"),
        "the report does not name the disabled addon: {out}"
    );
}

#[then("unrelated addon choices remain unchanged")]
fn unrelated_addon_choices_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains("arc42: true") && config.contains("domain_modeling: true"),
        "unrelated addon choices changed: {config}"
    );
}

#[given("a writable new project whose required opencode skill destination cannot be written")]
fn unwritable_opencode_skill_destination(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let skills = env.path(".opencode/skills");
    std::fs::create_dir_all(&skills).expect("create skills dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&skills)
            .expect("stat skills dir")
            .permissions();
        permissions.set_mode(0o555);
        std::fs::set_permissions(&skills, permissions).expect("make skills dir read-only");
    }
    world.env = Some(env);
}

#[then("the command exits nonzero")]
fn command_exits_nonzero(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("no CLI output");
    assert!(
        !output.status.success(),
        "the command succeeded unexpectedly: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[then("the report identifies initialization as partial and names the failed destination")]
fn report_identifies_partial(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("no CLI output");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.to_lowercase().contains("partial") && text.contains(".opencode"),
        "the report does not identify a partial initialization and the failed destination: {text}"
    );
}

#[then("the report names `givn skills sync` as the retry after correcting that destination")]
fn report_names_retry(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("no CLI output");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains("givn skills sync"),
        "the report does not name the retry: {text}"
    );
}

#[then("the report does not declare the project Ready or claim rollback")]
fn report_does_not_claim_ready_or_rollback(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("no CLI output");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !text.contains("Ready.") && !text.to_lowercase().contains("rolled back"),
        "the report claims readiness or rollback: {text}"
    );
}

#[then("successfully written scaffold files remain available for retry")]
fn scaffold_remains_for_retry(world: &mut GivnWorld) {
    let env = fixture(world);
    assert!(
        env.path("givn/config.yaml").is_file() && env.path("givn/specs").is_dir(),
        "the scaffold files were not preserved for retry"
    );
}

#[given("a partially initialized project after required skill synchronization failed")]
fn partially_initialized_project(world: &mut GivnWorld) {
    unwritable_opencode_skill_destination(world);
    let out = common::run_givn(fixture(world), &["init", "--agents", "opencode"]);
    assert!(
        !out.status.success(),
        "the partial initialization unexpectedly succeeded"
    );
}

#[given("the destination has been made writable")]
fn destination_made_writable(world: &mut GivnWorld) {
    let env = fixture(world);
    let skills = env.path(".opencode/skills");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = permissions_for(&skills);
        permissions.set_mode(0o755);
        std::fs::set_permissions(&skills, permissions).expect("make skills dir writable");
    }
}

#[cfg(unix)]
fn permissions_for(path: &std::path::Path) -> std::fs::Permissions {
    std::fs::metadata(path).expect("stat path").permissions()
}

#[then("the command exits 0 and reports installed skills")]
fn exits_zero_reports_installed_skills(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose"),
        "the retry did not report installed skills: {out}"
    );
}

#[then("the persisted target selection and explicit addon opt-outs remain unchanged")]
fn selection_and_opt_outs_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains("targets: [opencode]"),
        "the persisted target selection changed: {config}"
    );
    assert_eq!(
        config.matches("\naddons:").count(),
        1,
        "skill sync duplicated the addons section: {config}"
    );
}

#[given("a project root without a givn/ directory or test scripts")]
fn bare_root_without_scripts(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    assert!(!env.path("run-tests.sh").exists());
    world.env = Some(env);
}

#[then("both verification scopes declare an explicit unconfigured runner sentinel")]
fn verification_scopes_are_unconfigured_sentinels(world: &mut GivnWorld) {
    let env = fixture(world);
    let commands = std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands");
    assert!(
        commands.contains("command: \"givn missing-testrunner\"")
            && commands.contains("e2e_command: \"givn missing-e2e-runner\""),
        "verification scopes do not declare unconfigured sentinels: {commands}"
    );
}

#[then(
    "verification configuration does not assume `./run-tests.sh` or `./run-tests.sh --e2e` exists"
)]
fn verification_does_not_assume_script(world: &mut GivnWorld) {
    let env = fixture(world);
    let commands = std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands");
    let active: String = commands
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !active.contains("command: \"./run-tests.sh\"")
            && !active.contains("e2e_command: \"./run-tests.sh --e2e\""),
        "verification configuration assumes a project script: {active}"
    );
}

#[given(
    "a committed omp project with tracked recognizable generated legacy command files and generated skills"
)]
fn committed_omp_project_with_tracked_legacy(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let init = common::run_givn(&env, &["init", "--agents", "omp"]);
    assert!(
        init.status.success(),
        "omp init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    write_legacy_commands(&env, ".omp", &["givn-propose", "givn-review"]);
    for args in [vec!["init"], vec!["add", "-A"]] {
        let out = std::process::Command::new("git")
            .args(&args)
            .current_dir(env.root())
            .output()
            .expect("git command");
        assert!(
            out.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let commit = std::process::Command::new("git")
        .args([
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ])
        .current_dir(env.root())
        .output()
        .expect("git commit");
    assert!(
        commit.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
    world.env = Some(env);
}

#[then("recognizable generated legacy commands are removed")]
fn recognizable_legacy_commands_removed(world: &mut GivnWorld) {
    let env = fixture(world);
    assert!(
        !env.path(".omp/commands/givn-propose.md").exists()
            && !env.path(".omp/commands/givn-review.md").exists(),
        "recognizable omp legacy commands remain"
    );
}

#[then("installed generated skills and attribution integration are ignored and not tracked")]
fn generated_omp_rules_ignored_and_untracked(world: &mut GivnWorld) {
    let env = fixture(world);
    let listing = std::process::Command::new("git")
        .args(["ls-files", ".omp/skills", ".omp/hooks"])
        .current_dir(env.root())
        .output()
        .expect("git ls-files");
    assert!(listing.status.success(), "git ls-files failed");
    assert!(
        String::from_utf8_lossy(&listing.stdout).trim().is_empty(),
        "generated omp rules remain tracked: {}",
        String::from_utf8_lossy(&listing.stdout)
    );
    let ignore = std::fs::read_to_string(env.path(".gitignore")).expect("read .gitignore");
    assert!(
        ignore.contains(".omp/skills/givn-*/") && ignore.contains(".omp/hooks/pre/givn-"),
        "the omp generated rules are not ignored: {ignore}"
    );
}

#[then("no replacement command files are generated")]
fn no_replacement_command_files(world: &mut GivnWorld) {
    no_generated_commands(fixture(world));
}

#[given(regex = r#"^an initialized project with "([^"]+)" configuration and unrelated settings$"#)]
fn initialized_project_with_configuration_presentation(
    world: &mut GivnWorld,
    presentation: String,
) {
    let env = common::make_init_root();
    let path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&path).expect("read config");
    let mut lines: Vec<String> = Vec::new();
    let mut in_skills = false;
    for line in config.lines() {
        if in_skills {
            if !line.starts_with(' ') && !line.trim().is_empty() {
                in_skills = false;
            } else {
                continue;
            }
        }
        if line.trim() == "skills:" {
            in_skills = true;
            match presentation.as_str() {
                "flow mapping" => lines.push("skills: {targets: [opencode]}".to_string()),
                "anchors and aliases" => {
                    lines.push("skills: &defaults".to_string());
                    lines.push("  targets: [opencode]".to_string());
                }
                _ => {
                    lines.push("skills:".to_string());
                    lines.push("  targets: [opencode] # keep project comment".to_string());
                }
            }
            continue;
        }
        lines.push(line.to_string());
    }
    lines.push("custom_setting: keep-me".to_string());
    std::fs::write(&path, lines.join("\n") + "\n").expect("write config");
    world.additional_output = Some(presentation);
    world.env = Some(env);
}

#[then(
    "the target choice alone is updated or unsupported YAML is clearly refused before any write"
)]
fn target_choice_updated_or_refused(world: &mut GivnWorld) {
    let env = fixture(world);
    let presentation = world
        .additional_output
        .clone()
        .expect("configuration presentation missing");
    let output = world.last_output.as_ref().expect("no CLI output");
    let before = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    if presentation == "block mapping with comments" {
        assert!(
            output.status.success(),
            "supported block YAML was refused: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            before.contains("targets: [opencode, claude]"),
            "the target choice was not updated: {before}"
        );
    } else {
        assert!(
            !output.status.success(),
            "unsupported YAML was updated instead of refused"
        );
        assert!(
            before.contains(if presentation == "flow mapping" {
                "skills: {targets: [opencode]}"
            } else {
                "skills: &defaults"
            }),
            "unsupported YAML changed before the refusal: {before}"
        );
    }
}

#[then("unrelated settings and custom guidance are unchanged")]
fn unrelated_settings_and_guidance_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains("custom_setting: keep-me"),
        "an unrelated setting changed: {config}"
    );
}

#[then("no duplicate skills section is introduced")]
fn no_duplicate_skills_section(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    let sections = config
        .lines()
        .filter(|line| line.trim_start().starts_with("skills:"))
        .count();
    assert_eq!(sections, 1, "duplicate skills section: {config}");
}

// --- non-interactive initialization -----------------------------------------

#[given("the root contains human-authored agent instructions to preserve")]
fn root_contains_human_instructions(world: &mut GivnWorld) {
    let env = fixture(world);
    std::fs::write(env.path("AGENTS.md"), HUMAN_INSTRUCTIONS).expect("write AGENTS.md");
}

#[then("the initialization report names opencode and states that optional addons are disabled")]
fn init_report_names_selection_and_addons(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("skills will be installed to: .opencode")
            && out.contains("disabled (--no-addons)"),
        "the initialization report does not name the selection or the disabled addons: {out}"
    );
}

#[then(
    "the generated project-start guidance requires the effective dialog policy before every context's first user-facing question"
)]
fn project_start_requires_policy(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("follow the effective dialog policy from")
            && block.contains("load it once per context before the first"),
        "managed project-start guidance does not require the policy load: {block}"
    );
}

#[then(
    "the obligation covers questions outside named skills and names the handoff to another context"
)]
fn obligation_covers_outside_skills_and_handoff(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    let normalized = block.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("outside any named")
            && normalized.contains("when work is handed to another context"),
        "managed obligation does not cover named-skill boundaries or the handoff: {block}"
    );
}

#[then(
    "the project-start guidance contains a policy reference rather than the worked dialog examples"
)]
fn project_start_has_reference_not_examples(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        !block.contains("### Example 1 decision record")
            && !block.contains("## Rule 1 — Ask only necessary"),
        "managed project-start guidance embeds the worked dialog policy: {block}"
    );
}

#[then("the human-authored agent instructions remain unchanged")]
fn human_instructions_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert!(
        agents.contains(HUMAN_INSTRUCTIONS),
        "human-authored agent instructions were altered: {agents}"
    );
}

#[then("the installed givn-propose guidance directs the author to the proposal instruction")]
fn installed_propose_guidance_directs_to_instruction(world: &mut GivnWorld) {
    let env = fixture(world);
    let path = env.path(".opencode/skills/givn-propose/SKILL.md");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        content.contains("givn instructions proposal"),
        "the installed givn-propose guidance does not direct the author to the proposal instruction:\n{content}"
    );
}

#[then("no optional tone addon is required for the obligation")]
fn no_tone_addon_required(world: &mut GivnWorld) {
    let env = fixture(world);
    let block = managed_project_block(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("config missing");
    let active: String = config
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !block.contains("Absolute Mode") && !active.contains("absolute_mode: true"),
        "the obligation depends on the optional tone addon: {block}"
    );
}

// --- interactive terminal initialization ------------------------------------

#[when("the consumer selects opencode and claude in the real terminal init prompt")]
fn consumer_selects_targets_in_terminal(world: &mut GivnWorld) {
    let env = fixture(world);
    // Option order is .opencode, .agents, .claude with only .opencode
    // premarked: two down keys move to .claude, space marks it, enter confirms.
    world.last_output = Some(common::run_init_in_terminal(env, b"\x1b[B\x1b[B\x20\n"));
}

#[then(
    "the terminal displays a labeled agent-target selection and confirms the selected integrations"
)]
fn terminal_shows_labeled_selection(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("Which agents should givn install skills to?")
            && out.contains("[x] .opencode")
            && out.contains("[ ] .agents")
            && out.contains("[x] .claude"),
        "the terminal stream does not display the labeled agent-target selection: {out}"
    );
    assert!(
        out.contains(".opencode, .claude"),
        "the terminal stream does not confirm the selected integrations: {out}"
    );
}

// --- resynchronization ------------------------------------------------------

#[given(
    "the project contains an older managed startup block beside human-authored agent instructions"
)]
fn project_contains_stale_managed_block(world: &mut GivnWorld) {
    let env = fixture(world);
    let stale = format!(
        "{HUMAN_INSTRUCTIONS}\n<!-- givn:begin:givn-project -->\n{stale_body}\n<!-- givn:end:givn-project -->\n",
        stale_body = "This project uses **givn**. Old block without the dialog obligation."
    );
    std::fs::write(env.path("AGENTS.md"), stale).expect("write stale AGENTS.md");
}

#[then("the synchronization report identifies the selected guidance destinations")]
fn sync_report_identifies_destinations(world: &mut GivnWorld) {
    let out = report(world);
    assert!(
        out.contains("synced skill: givn-propose")
            && out.contains("Done. Generated:")
            && !out.contains("synced command:"),
        "the synchronization report does not identify the selected guidance: {out}"
    );
}

#[then(
    "the managed project-start block now requires the effective dialog policy for every user-facing question"
)]
fn managed_block_now_requires_policy(world: &mut GivnWorld) {
    let block = managed_project_block(world);
    assert!(
        block.contains("every user-facing question")
            && block.contains("effective dialog policy from"),
        "the refreshed managed block does not require the policy: {block}"
    );
}

#[then("the managed block is not duplicated")]
fn managed_block_not_duplicated(world: &mut GivnWorld) {
    let env = fixture(world);
    let agents = std::fs::read_to_string(env.path("AGENTS.md")).expect("AGENTS.md missing");
    assert_eq!(
        agents.matches("<!-- givn:begin:givn-project -->").count(),
        1,
        "the managed project-start block was duplicated: {agents}"
    );
    assert_eq!(
        agents.matches("<!-- givn:end:givn-project -->").count(),
        1,
        "the managed project-start block end marker was duplicated: {agents}"
    );
}

#[then("the persisted agent selection and unrelated configuration remain unchanged")]
fn persisted_selection_unchanged(world: &mut GivnWorld) {
    let env = fixture(world);
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("config missing");
    assert!(
        config.contains("targets: ['opencode', 'claude']")
            || config.contains("targets: [opencode, claude]"),
        "the persisted agent selection changed: {config}"
    );
}

// --- served runner bootstrap guidance --------------------------------------

#[given("a project initialized with unconfigured verification runners")]
fn runner_fixture_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let commands = std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands");
    assert!(
        commands.contains("givn missing-testrunner"),
        "verification runners are not unconfigured: {commands}"
    );
    world.env = Some(env);
}

#[when("the user invokes the unconfigured test-runner sentinel")]
fn invoke_unconfigured_test_runner(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("runner fixture is missing");
    world.last_output = Some(common::run_givn(env, &["missing-testrunner"]));
}

#[then("the commands template requires selected GIVN_FEATURES and every GIVN_RESULT_SCOPE")]
fn template_requires_features_and_scopes(_world: &mut GivnWorld) {
    let template = std::fs::read_to_string(common::repo_path("assets/commands.yaml.tmpl"))
        .expect("read commands template");
    for required in [
        "execute GIVN_FEATURES",
        "GIVN_RESULT_SCOPE",
        "`change` executes every delta",
        "including @e2e",
        "regular",
        "e2e selects only @e2e",
    ] {
        assert!(
            template.contains(required),
            "commands template is missing {required:?}: {template}"
        );
    }
}

#[then("the commands template requires fresh actual counts and fail-closed outcomes")]
fn template_requires_counts_and_failure_handling(_world: &mut GivnWorld) {
    let template = std::fs::read_to_string(common::repo_path("assets/commands.yaml.tmpl"))
        .expect("read commands template");
    for required in [
        "fresh GIVN_RESULT_FILE",
        "actual total, passed, failed, and skipped",
        "failed, undefined, pending, ambiguous, or skipped scenario",
        "must fail the",
    ] {
        assert!(
            template.contains(required),
            "commands template is missing {required:?}: {template}"
        );
    }
}

#[then("the commands template points to the runner contract instead of a generic script")]
fn template_points_to_contract(_world: &mut GivnWorld) {
    let template = std::fs::read_to_string(common::repo_path("assets/commands.yaml.tmpl"))
        .expect("read commands template");
    assert!(
        template.contains("docs/reference.md#runner-contract")
            && !template.contains("#!/bin/sh")
            && !template.contains("regular) tags='not @e2e'"),
        "commands template still ships a generic runner script: {template}"
    );
}

#[then("the sentinel names runner choices and the adapter contract")]
fn sentinel_names_choices_and_contract(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("sentinel output is missing");
    let stderr = String::from_utf8_lossy(&output.stderr);
    for required in [
        "cargo llvm-cov",
        "Cucumber.js",
        "Behave",
        "pytest-bdd",
        "GIVN_FEATURES",
        "GIVN_RESULT_SCOPE",
        "`change` runs every delta scenario, including `@e2e`",
        "regular",
        "e2e selects only `@e2e`",
    ] {
        assert!(
            stderr.contains(required),
            "sentinel guidance is missing {required:?}: {stderr}"
        );
    }
}

#[then("the sentinel exits non-zero")]
fn sentinel_exits_nonzero(world: &mut GivnWorld) {
    let output = world
        .last_output
        .as_ref()
        .expect("sentinel output is missing");
    assert!(
        !output.status.success(),
        "the unconfigured sentinel unexpectedly succeeded"
    );
}
