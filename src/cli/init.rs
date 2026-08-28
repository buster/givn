use crate::skills::targets::AgentTarget;
use anyhow::Result;
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

/// Default agent target when the user gives no selection.
const DEFAULT_TARGETS: [AgentTarget; 1] = [AgentTarget::Opencode];

/// Scaffold `givn/{specs,changes,archive}` and write the documented,
/// versioned `givn/config.yaml`. Enables all addons by default (opt-out via
/// `--no-addons`). Generates skills. Idempotent. `AGENTS.md` is otherwise
/// user-owned: givn writes to it only inside its own fenced
/// `<!-- givn:begin:* -->` blocks (e.g. the `absolute_mode` addon), and only
/// when the corresponding addon is enabled.
pub fn run(force: bool, no_addons: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    print!("{BANNER}");
    println!("  version {}\n", env!("CARGO_PKG_VERSION"));
    println!("{}", "─".repeat(52));

    // Create directory scaffolding.
    let dirs = ["givn/specs", "givn/changes", "givn/archive"];
    println!("\nScaffolding project...\n");
    for dir in &dirs {
        let path = project_root.join(dir);
        std::fs::create_dir_all(&path)?;
        println!("  created  {dir}");
    }

    // Write givn/config.yaml (documented + versioned).
    let config_path = project_root.join("givn/config.yaml");
    let config_created = if config_path.exists() && !force {
        println!("  exists   givn/config.yaml  (use --force to overwrite)");
        false
    } else {
        let content = crate::config::render::config_yaml(env!("CARGO_PKG_VERSION"))?;
        std::fs::write(&config_path, content)?;
        println!("  wrote    givn/config.yaml");
        true
    };

    // Ask which agents to install skills/commands to (fresh config only).
    // Non-interactive stdin (EOF) falls back to the default: opencode.
    let mut target_print = Vec::new();
    if config_created {
        let targets = prompt_for_targets()?;
        for t in &targets {
            target_print.push(t.display().to_string());
        }
        write_skills_block(&project_root, &targets)?;
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

    // Summary: what got installed/initialized, so the user doesn't have to
    // manually enable addons afterwards (or wonder what just happened).
    println!("\n{}", "─".repeat(52));
    println!("\nInstalled:\n");
    println!("  givn/config.yaml, givn/{{specs,changes,archive}}");
    println!("  core skills and commands");
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
/// Reads one line from stdin. On a terminal the numbered prompt is shown;
/// with piped/closed stdin the line is still read if available. An empty
/// answer, EOF, or an answer with no known agent name falls back to the
/// default (`opencode`). Unknown names are dropped with a warning listing
/// the valid agents.
fn prompt_for_targets() -> Result<Vec<AgentTarget>> {
    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        println!("\nWhich agents should givn install skills to?");
        for (i, t) in AgentTarget::ALL.iter().enumerate() {
            let marker = if *t == AgentTarget::Opencode {
                " (default)"
            } else {
                ""
            };
            println!("  [{}] {}{}", i + 1, t.display(), marker);
        }
        println!(
            "Enter numbers or names, comma-separated. Press Enter for the default (opencode): "
        );
    }

    let mut line = String::new();
    let n = stdin.read_line(&mut line)?;
    if n == 0 {
        return Ok(DEFAULT_TARGETS.to_vec());
    }

    let mut targets: Vec<AgentTarget> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    for part in line.split(',') {
        match AgentTarget::from_name(part) {
            Some(t) if !targets.contains(&t) => targets.push(t),
            Some(_) => {}
            None => {
                let name = part.trim();
                if !name.is_empty() {
                    unknown.push(name.to_string());
                }
            }
        }
    }

    if !unknown.is_empty() {
        let valid: Vec<&str> = AgentTarget::ALL.iter().map(|t| t.display()).collect();
        println!(
            "Unknown agent(s): {}. Valid agents: {}",
            unknown.join(", "),
            valid.join(", ")
        );
    }

    if targets.is_empty() {
        return Ok(DEFAULT_TARGETS.to_vec());
    }

    Ok(targets)
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
