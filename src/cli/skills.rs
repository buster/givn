use anyhow::{Result, bail};

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

/// Add agent target(s) to the existing `skills: targets:` selection in
/// `givn/config.yaml` and sync. Only that one line is touched — addons,
/// comments, and everything else in the file are left as-is.
pub fn run_targets_add(names: &[String]) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn/config.yaml").exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let expand_all = names.iter().any(|n| n.trim().eq_ignore_ascii_case("all"));
    let (additions, unknown) = if expand_all {
        (crate::skills::targets::AgentTarget::ALL.to_vec(), Vec::new())
    } else {
        crate::skills::targets::parse_target_names(names.iter().map(String::as_str))
    };

    let valid = crate::skills::targets::valid_names();
    if !unknown.is_empty() {
        println!(
            "Unknown agent(s): {}. Valid agents: {}",
            unknown.join(", "),
            valid.join(", ")
        );
    }
    if additions.is_empty() {
        bail!("No valid agent name given. Valid agents: {}", valid.join(", "));
    }

    let updated = crate::skills::targets::add_targets(&project_root, &additions)?;
    println!(
        "skills targets: {}",
        updated
            .iter()
            .map(|t| t.display())
            .collect::<Vec<_>>()
            .join(", ")
    );

    println!("\nSyncing skills and agent commands...");
    crate::skills::targets::sync(&project_root)?;

    Ok(())
}
