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

    println!("\ngivn initialised. Next step: set verify.command in givn/config.yaml.");
    println!("Run `givn skills sync` to generate AGENTS.md and skills.");

    Ok(())
}
