use anyhow::Result;

/// Regenerate AGENTS.md and skills from current templates.
pub fn run_sync() -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        anyhow::bail!("givn/ not found. Run `givn init` first.");
    }

    println!("Syncing skills and AGENTS.md...");
    crate::skills::targets::sync(&project_root)?;
    println!("\nDone. Generated:");
    println!("  AGENTS.md");
    println!("  .agents/skills/  (primary)");
    println!("  .claude/skills/  (Claude Code mirror)");
    println!("  .opencode/skills/ (opencode mirror)");

    Ok(())
}
