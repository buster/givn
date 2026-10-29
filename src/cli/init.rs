use crate::skills::targets::AgentTarget;
use anyhow::{Result, bail};
use std::io::IsTerminal;
use std::path::Path;

const BANNER: &str = r#"
  ________  .__
 /  _____/  |__|__  __ ____
/   \  ___  |  \  \/ //    \
\    \_\  \ |  |\   /|   |  \
 \______  / |__| \_/ |___|  /
        \/                \/

  from Given to done.
"#;

/// The pre-marked entry in the interactive agent-target prompt.
///
/// `AgentTarget::ALL[0]`, so the checkbox list highlights it first; it is the
/// only pre-marked entry, not a fallback for a missing selection.
const DEFAULT_PROMPT_TARGET: AgentTarget = AgentTarget::Opencode;

/// Scaffold `givn/{specs,changes,archive}` and write the documented,
/// versioned `givn/config.yaml`. A fresh project enables every addon; an
/// existing project keeps its explicit choices, and `--no-addons` opts every
/// addon out. Generates skills. Idempotent. `AGENTS.md` is otherwise
/// user-owned: givn writes to it only inside its own fenced
/// `<!-- givn:begin:* -->` blocks (for example the `communication_policy`
/// addon), and only when the corresponding addon is enabled.
pub fn run(force: bool, no_addons: bool, agents: &[String]) -> Result<()> {
    let project_root = std::env::current_dir()?;

    print!("{BANNER}");
    println!("  version {}\n", env!("CARGO_PKG_VERSION"));
    println!("{}", "─".repeat(52));

    // A fresh config is written unless one already exists and `--force` is
    // absent. Resolve the agent-target selection before the first write, so a
    // refused initialization leaves the project unchanged. `--agents` skips
    // the prompt; otherwise the prompt (terminal) or one answer line (piped
    // stdin) decides, and an empty set is refused.
    let config_path = project_root.join("givn/config.yaml");
    let config_created = !(config_path.exists() && !force);
    let (targets, target_print) = if config_created {
        let targets = if agents.is_empty() {
            prompt_for_targets()?
        } else {
            resolve_agents_flag(agents)?
        };
        let target_print = targets.iter().map(|t| t.display().to_string()).collect();
        (targets, target_print)
    } else {
        (Vec::new(), Vec::new())
    };

    // Re-rendering an existing managed scaffold must not discard the
    // project's explicit addon choices. Read and validate them before the
    // first write so a retired or malformed block is refused unchanged.
    let preserved_addons = if config_created && config_path.exists() && !no_addons {
        let existing = std::fs::read_to_string(&config_path)?;
        Some(crate::cli::addons::explicit_addon_settings(&existing)?)
    } else {
        None
    };

    // Create directory scaffolding.
    let dirs = ["givn/specs", "givn/changes", "givn/archive"];
    println!("\nScaffolding project...\n");
    for dir in &dirs {
        let path = project_root.join(dir);
        std::fs::create_dir_all(&path)?;
        println!("  created  {dir}");
    }

    // Write givn/config.yaml (documented + versioned).
    if config_created {
        let content = crate::config::render::config_yaml(env!("CARGO_PKG_VERSION"))?;
        std::fs::write(&config_path, content)?;
        if let Some(settings) = &preserved_addons {
            let block = crate::cli::addons::addons_block(settings);
            if !block.is_empty() {
                let mut content = std::fs::read_to_string(&config_path)?;
                if !content.ends_with('\n') {
                    content.push('\n');
                }
                content.push('\n');
                content.push_str(&block);
                std::fs::write(&config_path, content)?;
            }
        }
        println!("  wrote    givn/config.yaml");
    } else {
        println!("  exists   givn/config.yaml  (use --force to overwrite)");
    }

    if config_created {
        write_skills_block(&project_root, &targets)?;
        write_rules_policy(&project_root)?;
        println!("  skills will be installed to: {}", target_print.join(", "));
    }

    // givn never invents a project runner script. The commands template ships
    // an explicit unconfigured sentinel that explains setup and exits non-zero.
    let commands_path = project_root.join("givn/commands.yaml");
    if commands_path.exists() && !force {
        println!("  exists   givn/commands.yaml  (use --force to overwrite)");
    } else {
        let commands_content = include_str!("../../assets/commands.yaml.tmpl");
        std::fs::write(&commands_path, commands_content)?;
        println!("  wrote    givn/commands.yaml");
    }

    // Resolve addons exactly once. A fresh scaffold enables every addon; a
    // re-rendered existing scaffold scaffolds only the choices restored from
    // the project. A plain re-init leaves the stored choices untouched.
    if no_addons {
        crate::cli::addons::disable_all(&project_root)?;
        println!("\nDisabling addons...\n");
        println!("  explicit false written for every addon");
    } else if config_created {
        println!("\nEnabling addons...\n");
        if preserved_addons.is_some() {
            let manifest = crate::config::overlay::load_effective()?;
            for addon in crate::cli::addons::KNOWN_ADDONS {
                if manifest
                    .addons
                    .get(addon.id)
                    .unwrap_or(addon.default_enabled)
                {
                    crate::cli::addons::scaffold_enabled(addon.id)?;
                }
            }
        } else {
            for addon in crate::cli::addons::KNOWN_ADDONS {
                if addon.default_enabled {
                    crate::cli::addons::enable_scaffold(addon.id)?;
                }
            }
        }
    }

    // Generate skills and agent integrations. A failure here leaves a partial
    // initialization: the scaffold remains for retry, and the command exits
    // non-zero without declaring the project Ready.
    println!("\nGenerating skills and agent integrations...\n");
    if let Err(error) = crate::skills::targets::sync(&project_root) {
        let destination = failed_skill_destination(&project_root);
        eprintln!("\n{}", "─".repeat(52));
        eprintln!("\nInitialization partial: required skill synchronization failed.");
        eprintln!("  failed destination: {destination}");
        eprintln!("  reported error: {error}");
        eprintln!("  successfully written scaffold files remain available for retry");
        eprintln!("  no rollback was attempted");
        eprintln!("  retry after correcting the destination: givn skills sync");
        eprintln!();
        return Err(anyhow::anyhow!(
            "initialization partial: required skill synchronization failed at {destination}: \
             {error}; retry after correcting the destination: givn skills sync"
        ));
    }

    // Backend awareness: name the selected backend's limits instead of
    // staying silent about commands that cannot work here (Q2, Q7).
    match crate::scm::select(&project_root) {
        Ok(crate::scm::Backend::Fossil(_)) => {}
        Err(_) => {
            println!(
                "\n  warning: no Git worktree or Fossil checkout found at '{}'",
                project_root.display()
            );
            println!(
                "  warning: archive and managed upgrade are unavailable until the project is version-controlled"
            );
        }
        Ok(_) => {}
    }

    // Summary: what got installed/initialized, so the user doesn't have to
    // manually enable addons afterwards (or wonder what just happened).
    println!("\n{}", "─".repeat(52));
    println!("\nInstalled:\n");
    println!("  givn/config.yaml, givn/{{specs,changes,archive}}");
    println!("  core skills and agent integrations");
    match crate::scm::select(&project_root) {
        Ok(crate::scm::Backend::Fossil(_)) => {
            println!(
                "  commit attribution: givn commit path only (raw fossil commit is not stamped)"
            )
        }
        _ => println!("  commit attribution: .githooks/commit-msg (best effort)"),
    }
    if let Ok(targets) = crate::config::overlay::load_skills_targets(&project_root) {
        for target in targets {
            match target.as_str() {
                "opencode" => {
                    println!("  attribution integration: .opencode/plugins/givn-attribution.js")
                }
                "claude" => println!(
                    "  attribution integration: .claude/hooks/givn-attribution.sh + .claude/settings.local.json"
                ),
                "omp" => {
                    println!("  attribution integration: .omp/hooks/pre/givn-attribution.js")
                }
                "maki" => println!(
                    "  development reporter: .maki/lua/givn-timeline.lua + .maki/plugin.toml + .maki/init.lua"
                ),
                _ => {}
            }
        }
    }
    if no_addons {
        println!("  addons:  disabled (--no-addons)");
        println!("           explicit false written for every addon");
    } else {
        let manifest = crate::config::overlay::load_effective()?;
        let states: Vec<(&str, bool)> = crate::cli::addons::KNOWN_ADDONS
            .iter()
            .map(|addon| {
                (
                    addon.id,
                    manifest
                        .addons
                        .get(addon.id)
                        .unwrap_or(addon.default_enabled),
                )
            })
            .collect();
        let enabled = states.iter().filter(|(_, enabled)| *enabled).count();
        if enabled == states.len() {
            println!("  addons:  enabled");
        } else if enabled == 0 {
            println!("  addons:  disabled");
        } else {
            println!("  addons:  effective selection");
        }
        for (id, enabled) in states {
            println!(
                "    - {id}: {}",
                if enabled { "enabled" } else { "disabled" }
            );
        }
    }

    println!("\n{}", "─".repeat(52));
    println!("\nReady. Three steps to your first change:\n");
    println!("  1. Start a change    givn new <change-id>");
    println!("  2. Plan it           /givn-propose  (in your AI agent)");
    println!("  3. Implement it      /givn-implement");
    println!();

    Ok(())
}

/// Name the selected skill destination that is obviously unwritable (for
/// example a regular file where the target directory belongs). Falls back to
/// the selected destination list so the partial report never hides where the
/// required synchronization was headed.
fn failed_skill_destination(project_root: &Path) -> String {
    let targets = crate::config::overlay::load_skills_targets(project_root).unwrap_or_default();
    for name in &targets {
        let Some(target) = crate::skills::targets::AgentTarget::from_name(name) else {
            continue;
        };
        for path in [
            project_root.join(target.dir()),
            project_root.join(target.dir()).join("skills"),
        ] {
            if path.exists() && !path.is_dir() {
                return path.display().to_string();
            }
        }
    }
    let directories: Vec<String> = targets
        .iter()
        .filter_map(|name| crate::skills::targets::AgentTarget::from_name(name))
        .map(|target| target.display().to_string())
        .collect();
    if directories.is_empty() {
        "the selected agent target".to_string()
    } else {
        directories.join(", ")
    }
}

/// Ask the user which agents to install skills/commands to.
///
/// On a real terminal, shows an arrow-key/space/enter checkbox list with
/// `.opencode` marked; a confirmation that marks nothing reports the
/// requirement and shows the list again (Ctrl-C aborts). `dialoguer` needs a
/// controlling terminal, so with piped/closed stdin (automation, tests) this
/// instead reads one line of comma-separated names, same as before. EOF
/// without an `--agents` flag is refused. Unknown names are dropped with a
/// warning listing the valid agents.
fn prompt_for_targets() -> Result<Vec<AgentTarget>> {
    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        let items: Vec<&str> = AgentTarget::ALL.iter().map(|t| t.display()).collect();
        let defaults: Vec<bool> = AgentTarget::ALL
            .iter()
            .map(|target| *target == DEFAULT_PROMPT_TARGET)
            .collect();

        // The dialog stays open until a target is marked. Each iteration
        // rebuilds the builder — `interact(self)` consumes it and seeds the
        // checkboxes from `defaults` at entry, so the pre-marked entry
        // reappears after a re-ask.
        loop {
            let selected = dialoguer::MultiSelect::new()
                .with_prompt(
                    "Which agents should givn install skills to? (space to toggle, enter to confirm)",
                )
                .items(&items)
                .defaults(&defaults)
                .interact()?;

            if !selected.is_empty() {
                return Ok(selected.into_iter().map(|i| AgentTarget::ALL[i]).collect());
            }
            eprintln!(
                "error: no agent target selected; mark at least one agent target ({}) and confirm",
                crate::skills::targets::valid_names().join(", ")
            );
        }
    }

    let mut line = String::new();
    let n = stdin.read_line(&mut line)?;
    if n == 0 {
        return require_agent_target(Vec::new());
    }

    require_agent_target(resolve_target_names(line.split(',')))
}

/// Refuse an initialization that selected no agent target.
///
/// Givn's workflow runs from the generated skills, so a project initialized
/// without an agent target is unusable; the caller resolves this before the
/// first write, leaving the project unchanged.
fn require_agent_target(targets: Vec<AgentTarget>) -> Result<Vec<AgentTarget>> {
    if targets.is_empty() {
        bail!(
            "no agent target selected; pass --agents <opencode|agents|claude|omp|maki|all> \
             or select one in the prompt (valid targets: {})",
            crate::skills::targets::valid_names().join(", ")
        );
    }
    Ok(targets)
}

/// Resolve the `--agents` flag values into targets, skipping the prompt.
/// `all` (case-insensitive, alone or alongside other names) expands to
/// every known target. An empty result is refused.
fn resolve_agents_flag(agents: &[String]) -> Result<Vec<AgentTarget>> {
    if agents.iter().any(|a| a.trim().eq_ignore_ascii_case("all")) {
        return require_agent_target(AgentTarget::ALL.to_vec());
    }
    require_agent_target(resolve_target_names(agents.iter().map(String::as_str)))
}

/// Resolve comma-split or flag-provided names into targets, deduplicated,
/// in first-seen order. Unknown names are dropped with a warning listing
/// the valid agents. An empty result is the caller's refusal case
/// (`require_agent_target`).
fn resolve_target_names<'a>(parts: impl Iterator<Item = &'a str>) -> Vec<AgentTarget> {
    let (targets, unknown) = crate::skills::targets::parse_target_names(parts);

    if !unknown.is_empty() {
        let valid = crate::skills::targets::valid_names();
        println!(
            "Unknown agent(s): {}. Valid agents: {}",
            unknown.join(", "),
            valid.join(", ")
        );
    }

    targets
}

/// Append the `skills:` block with the chosen targets to `givn/config.yaml`.
/// Line-based append: preserves all other content and comments.
fn write_skills_block(project_root: &Path, targets: &[AgentTarget]) -> Result<()> {
    let config_path = project_root.join("givn/config.yaml");
    let mut content = std::fs::read_to_string(&config_path)?;
    if !content.ends_with('\n') {
        content.push('\n');
    }
    if !content.ends_with("\n\n") {
        content.push('\n');
    }
    let list: Vec<&str> = targets.iter().map(|t| t.name()).collect();
    content.push_str(&format!("skills:\n  targets: [{}]\n", list.join(", ")));
    std::fs::write(&config_path, content)?;
    Ok(())
}

/// Record the active default rules policy in a fresh config. A later edit is
/// a user-owned line that `givn upgrade` preserves.
fn write_rules_policy(project_root: &Path) -> Result<()> {
    let config_path = project_root.join("givn/config.yaml");
    let mut content = std::fs::read_to_string(&config_path)?;
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str("rules: local-only\n");
    std::fs::write(&config_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn cwd_lock() -> &'static std::sync::Mutex<()> {
        crate::config::overlay::test_cwd_lock()
    }

    struct RestoreCwd(PathBuf);

    impl Drop for RestoreCwd {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }

    #[test]
    fn required_skill_sync_failure_makes_init_partial_and_nonzero() {
        let _lock = cwd_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join(".opencode"), "not a directory").unwrap();
        let _restore = RestoreCwd(std::env::current_dir().unwrap());
        std::env::set_current_dir(directory.path()).unwrap();

        let error = run(false, true, &["opencode".to_string()])
            .expect_err("a required skill synchronization failure must fail init");
        let message = error.to_string();
        assert!(message.contains("partial"), "{message}");
        assert!(message.contains("givn skills sync"), "{message}");
        assert!(message.contains(".opencode"), "{message}");
        assert!(
            directory.path().join("givn/config.yaml").exists(),
            "successfully written scaffold files must remain for retry"
        );
    }

    #[test]
    fn init_scaffolds_unconfigured_runner_sentinels_without_a_script() {
        let _lock = cwd_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let directory = tempfile::tempdir().unwrap();
        let _restore = RestoreCwd(std::env::current_dir().unwrap());
        std::env::set_current_dir(directory.path()).unwrap();

        run(false, true, &["opencode".to_string()]).expect("init succeeds");
        assert!(
            !directory.path().join("run-tests.sh").exists(),
            "init must not invent a project runner script"
        );
        let commands =
            std::fs::read_to_string(directory.path().join("givn/commands.yaml")).unwrap();
        assert!(commands.contains("givn missing-testrunner"), "{commands}");
        let active: Vec<&str> = commands
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect();
        assert!(
            !active.iter().any(|line| line.contains("./run-tests.sh")),
            "{commands}"
        );
    }
    fn append_addons(project_root: &Path, settings: &[(&str, bool)]) {
        let config_path = project_root.join("givn/config.yaml");
        let content = std::fs::read_to_string(&config_path).unwrap();
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
        kept.push(
            crate::cli::addons::addons_block(
                &settings
                    .iter()
                    .map(|(name, enabled)| (name.to_string(), *enabled))
                    .collect::<Vec<_>>(),
            )
            .trim_end()
            .to_string(),
        );
        std::fs::write(config_path, kept.join("\n") + "\n").unwrap();
    }

    #[test]
    fn reentry_preserves_explicit_addon_opt_outs() {
        let _lock = cwd_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let directory = tempfile::tempdir().unwrap();
        let _restore = RestoreCwd(std::env::current_dir().unwrap());
        std::env::set_current_dir(directory.path()).unwrap();

        run(false, true, &["opencode".to_string()]).unwrap();
        append_addons(
            directory.path(),
            &[("outline", false), ("challenge", false)],
        );

        run(false, false, &[]).unwrap();
        let after_plain =
            std::fs::read_to_string(directory.path().join("givn/config.yaml")).unwrap();
        assert!(after_plain.contains("  outline: false"), "{after_plain}");
        assert!(after_plain.contains("  challenge: false"), "{after_plain}");

        run(true, false, &["opencode".to_string()]).unwrap();
        let after_force =
            std::fs::read_to_string(directory.path().join("givn/config.yaml")).unwrap();
        assert!(after_force.contains("  outline: false"), "{after_force}");
        assert!(after_force.contains("  challenge: false"), "{after_force}");
        assert_eq!(after_force.matches("\naddons:").count(), 1, "{after_force}");
        let manifest = crate::config::overlay::load_effective().unwrap();
        assert!(!manifest.addons.outline);
        assert!(!manifest.addons.challenge);
        assert!(manifest.addons.communication_policy);
    }

    #[test]
    fn no_addons_opts_out_every_current_addon_including_the_renamed_pair() {
        let _lock = cwd_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let directory = tempfile::tempdir().unwrap();
        let _restore = RestoreCwd(std::env::current_dir().unwrap());
        std::env::set_current_dir(directory.path()).unwrap();

        run(false, true, &["opencode".to_string()]).unwrap();
        let config = std::fs::read_to_string(directory.path().join("givn/config.yaml")).unwrap();
        assert!(config.contains("  communication_policy: false"), "{config}");
        assert!(config.contains("  challenge: false"), "{config}");
        assert!(
            !config.contains("absolute_mode") && !config.contains("grill_"),
            "{config}"
        );
    }
}
