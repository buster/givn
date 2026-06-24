/// `givn extras` subcommands: install, uninstall.
///
/// Extras are a curated set of optional, project-level skills and commands
/// independent of the workflow artifact graph. They are enabled via the
/// top-level `extras:` key in `givn/config.yaml`. See arc42 ch.8.11.
use anyhow::{bail, Result};

/// Write `extras: <value>` into givn/config.yaml.
///
/// Line-based edit (mirrors `set_addon` in `src/cli/addons.rs`) that preserves
/// all other content and comments:
///   1. If an uncommented `extras:` line exists → replace the whole line.
///   2. If a commented `# extras:` line exists → replace it with the active line.
///   3. If no `extras:` line exists at all → append one at the end.
pub fn set_extras(enabled: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let content = std::fs::read_to_string(&config_path)?;
    let value_str = if enabled { "true" } else { "false" };
    let target_line = format!("extras: {value_str}");

    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();

    // Case 1: an uncommented `extras:` line exists → replace the whole line.
    for line in &mut lines {
        if is_extras_key_line(line) && !line.trim_start().starts_with('#') {
            *line = target_line.clone();
            std::fs::write(&config_path, lines.join("\n") + "\n")?;
            return Ok(());
        }
    }

    // Case 2: a commented `# extras:` line exists → replace it with the active line.
    for line in &mut lines {
        if line.trim_start().starts_with('#') && is_extras_key_line(line) {
            *line = target_line.clone();
            std::fs::write(&config_path, lines.join("\n") + "\n")?;
            return Ok(());
        }
    }

    // Case 3: no `extras:` line at all → append one (after a blank line if needed).
    if !lines.last().map(|l| l.is_empty()).unwrap_or(true) {
        lines.push(String::new());
    }
    lines.push(target_line);
    std::fs::write(&config_path, lines.join("\n") + "\n")?;
    Ok(())
}

/// `givn extras install` — enable `extras` in givn/config.yaml, then sync
/// skills+commands so the extras set is materialised on disk.
///
/// Bails if the project hasn't been initialised (no `givn/` directory).
pub fn run_install() -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        bail!("givn/ not found. Run `givn init` first.");
    }

    println!("Enabling extras and syncing skills...");
    set_extras(true)?;
    crate::skills::targets::sync(&project_root)?;
    println!("\nDone. Extras skills and commands are now installed.");
    Ok(())
}

/// `givn extras uninstall` — disable `extras` in givn/config.yaml, then remove
/// the generated extras skills and commands from every target location.
///
/// Removal is best-effort: missing files/dirs are silently skipped, making
/// uninstall idempotent. Bails if the project hasn't been initialised.
pub fn run_uninstall() -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        bail!("givn/ not found. Run `givn init` first.");
    }

    println!("Disabling extras and removing generated files...");
    set_extras(false)?;
    remove_extras_files(&project_root);
    println!("\nDone. Extras skills and commands have been removed.");
    Ok(())
}

/// Delete every extras skill directory and extras command file from all
/// target locations. Best-effort: absent paths are no-ops.
fn remove_extras_files(project_root: &std::path::Path) {
    let skill_targets = [".agents/skills", ".claude/skills", ".opencode/skills"];
    for name in crate::skills::targets::extras_skill_names() {
        for base in &skill_targets {
            let dir = project_root.join(format!("{base}/{name}"));
            if dir.exists() {
                let _ = std::fs::remove_dir_all(&dir);
            }
        }
    }
    for name in crate::skills::targets::extras_command_names() {
        let file = project_root.join(format!(".opencode/commands/{name}.md"));
        if file.exists() {
            let _ = std::fs::remove_file(&file);
        }
    }
}

/// True if the line, once leading `#` and spaces are stripped, begins with `extras:`.
fn is_extras_key_line(line: &str) -> bool {
    line.trim_start_matches([' ', '#'])
        .trim_start()
        .starts_with("extras:")
}
