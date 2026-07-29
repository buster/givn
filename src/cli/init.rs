use crate::config::render;
use anyhow::Result;

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
    if config_path.exists() && !force {
        println!("  exists   givn/config.yaml  (use --force to overwrite)");
    } else {
        let content = render::config_yaml(env!("CARGO_PKG_VERSION"))?;
        std::fs::write(&config_path, content)?;
        println!("  wrote    givn/config.yaml");
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
    // part of `addons::run_enable`, so the effect is identical to running
    // `givn addons enable <name>` manually for every addon.
    if !no_addons {
        println!("\nEnabling addons...\n");
        for (name, _) in crate::cli::addons::KNOWN_ADDONS {
            crate::cli::addons::run_enable(name)?;
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
