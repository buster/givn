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
    println!("\nDone. Generated:");
    println!("  .agents/skills/   (primary neutral skills)");
    println!("  .claude/skills/   (Claude Code mirror)");
    println!("  .opencode/skills/ (opencode skills mirror)");
    println!("  .opencode/commands/ (opencode slash commands)");

    Ok(())
}
