use anyhow::Result;

/// Regenerate skills and agent commands from current templates.
/// `AGENTS.md` is otherwise user-owned: givn writes to it only inside its
/// own fenced `<!-- givn:begin:* -->` blocks (e.g. the `absolute_mode`
/// addon), and only when the corresponding addon is enabled.
pub fn run_sync() -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        anyhow::bail!("givn/ not found. Run `givn init` first.");
    }

    println!("Syncing skills and agent commands...");
    crate::skills::targets::sync(&project_root)?;
    let targets = crate::config::overlay::load_skills_targets(&project_root)?;
    println!("\nDone. Generated:");
    for name in &targets {
        println!("  .{name}/skills/   (skills for {name})");
    }
    if targets.iter().any(|t| t == "opencode") {
        println!("  .opencode/commands/ (opencode slash commands)");
    }

    Ok(())
}
