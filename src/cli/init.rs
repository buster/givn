use anyhow::Result;
use crate::config::render;

/// Scaffold `givn/{specs,changes,archive}` and write the documented,
/// versioned `givn/config.yaml`. Generates AGENTS.md and skills. Idempotent.
pub fn run(force: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    // Create directory scaffolding.
    let dirs = [
        "givn/specs",
        "givn/changes",
        "givn/archive",
    ];
    for dir in &dirs {
        let path = project_root.join(dir);
        std::fs::create_dir_all(&path)?;
        println!("  created  {}", path.display());
    }

    // Write givn/config.yaml (documented + versioned).
    let config_path = project_root.join("givn/config.yaml");
    if config_path.exists() && !force {
        println!("  exists   {} (use --force to overwrite)", config_path.display());
    } else {
        let content = render::config_yaml(env!("CARGO_PKG_VERSION"))?;
        std::fs::write(&config_path, content)?;
        println!("  wrote    {}", config_path.display());
    }

    // Generate skills and AGENTS.md.
    println!("\nGenerating skills and AGENTS.md...");
    match crate::skills::targets::sync(&project_root) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("  warning: skill generation failed: {e}");
            eprintln!("  Run `givn skills sync` after configuring verify.command.");
        }
    }

    println!("\ngivn initialised. Next step:");
    println!("  1. Set verify.command in givn/config.yaml");
    println!("  2. Run `givn skills sync` to refresh skills and commands");
    println!("  3. Run `givn new <change-id>` to start a change");

    Ok(())
}
