use anyhow::Result;
use crate::config::render;

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
/// versioned `givn/config.yaml`. Generates skills. Idempotent.
/// `AGENTS.md` is left untouched (user-owned).
pub fn run(force: bool, no_extras: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    print!("{BANNER}");
    println!("  version {}\n", env!("CARGO_PKG_VERSION"));
    println!("{}", "─".repeat(52));

    // Create directory scaffolding.
    let dirs = [
        "givn/specs",
        "givn/changes",
        "givn/archive",
    ];
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

    // Enable extras (default) unless --no-extras. Runs after the config write so
    // a --force overwrite can't wipe the flag; sync then reads the result.
    crate::cli::extras::set_extras(!no_extras)?;

    // Generate skills and agent commands.
    println!("\nGenerating skills and agent commands...\n");
    match crate::skills::targets::sync(&project_root) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("  warning: skill generation failed: {e}");
            eprintln!("  Run `givn skills sync` after configuring verify.command.");
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
