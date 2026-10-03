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

/// Scaffold `givn/{specs,changes,archive}` and write the documented,
/// versioned `givn/config.yaml`. Enables all addons by default (opt-out via
/// `--no-addons`). Generates skills. Idempotent. `AGENTS.md` is otherwise
/// user-owned: givn writes to it only inside its own fenced
/// `<!-- givn:begin:* -->` blocks (e.g. the `absolute_mode` addon), and only
/// when the corresponding addon is enabled.
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
            resolve_agents_flag(agents)
        };
        let target_print = targets.iter().map(|t| t.display().to_string()).collect();
        (targets, target_print)
    } else {
        (Vec::new(), Vec::new())
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
        println!("  wrote    givn/config.yaml");
    } else {
        println!("  exists   givn/config.yaml  (use --force to overwrite)");
    }

    if config_created {
        write_skills_block(&project_root, &targets)?;
        write_rules_policy(&project_root)?;
        println!("  skills will be installed to: {}", target_print.join(", "));
    }

    // Write givn/commands.yaml (test runner + coverage, separate from config).
    let commands_path = project_root.join("givn/commands.yaml");
    if commands_path.exists() && !force {
        println!("  exists   givn/commands.yaml  (use --force to overwrite)");
    } else {
        let commands_content = include_str!("../../assets/commands.yaml.tmpl");
        std::fs::write(&commands_path, commands_content)?;
        println!("  wrote    givn/commands.yaml");
    }

    // Enable every known addon (default) unless --no-addons. Each addon's
    // own scaffolding (arc42 docs, absolute_mode's AGENTS.md block) runs as
    // part of `addons::enable_scaffold`, so the effect is identical to running
    // `givn addons enable <name>` manually for every addon (skills are
    // generated once below instead of per addon).
    if !no_addons {
        println!("\nEnabling addons...\n");
        for (name, _) in crate::cli::addons::KNOWN_ADDONS {
            crate::cli::addons::enable_scaffold(name)?;
        }
    }

    // Generate skills and agent commands.
    println!("\nGenerating skills and agent commands...\n");
    match crate::skills::targets::sync(&project_root) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("  warning: skill generation failed: {e}");
            eprintln!("  Run `givn skills sync` after configuring verify.command.");
        }
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
    println!("  core skills and commands");
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
                _ => {}
            }
        }
    }
    if no_addons {
        println!("  addons:  disabled (--no-addons)");
    } else {
        println!("  addons:  enabled");
        for (name, description) in crate::cli::addons::KNOWN_ADDONS {
            println!("    - {name}: {description}");
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

/// Ask the user which agents to install skills/commands to.
///
/// On a real terminal, shows an arrow-key/space/enter checkbox list with
/// nothing marked; confirming nothing is refused. `dialoguer` needs a
/// controlling terminal, so with piped/closed stdin (automation, tests) this
/// instead reads one line of comma-separated names, same as before. EOF
/// without an `--agents` flag is refused. Unknown names are dropped with a
/// warning listing the valid agents.
fn prompt_for_targets() -> Result<Vec<AgentTarget>> {
    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        let items: Vec<&str> = AgentTarget::ALL.iter().map(|t| t.display()).collect();

        let selected = dialoguer::MultiSelect::new()
            .with_prompt(
                "Which agents should givn install skills to? (space to toggle, enter to confirm)",
            )
            .items(&items)
            .interact()?;

        return require_agent_target(selected.into_iter().map(|i| AgentTarget::ALL[i]).collect());
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
            "no agent target selected; pass --agents <opencode|agents|claude|all> \
             or select one in the prompt (valid targets: {})",
            crate::skills::targets::valid_names().join(", ")
        );
    }
    Ok(targets)
}

/// Resolve the `--agents` flag values into targets, skipping the prompt.
/// `all` (case-insensitive, alone or alongside other names) expands to
/// every known target.
fn resolve_agents_flag(agents: &[String]) -> Vec<AgentTarget> {
    if agents.iter().any(|a| a.trim().eq_ignore_ascii_case("all")) {
        return AgentTarget::ALL.to_vec();
    }
    resolve_target_names(agents.iter().map(String::as_str))
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
