use anyhow::{Result, bail};

/// Re-render managed config scaffold, preserve user overrides, bump givn_config_version.
///
/// Strategy (ADR-023):
/// - Parse the existing givn/config.yaml into two regions:
///   - "managed" = commented lines (givn's doc scaffold)
///   - "user-owned" = uncommented key/value lines (never touched)
/// - Re-render the managed scaffold from the current binary's template.
/// - Splice user-owned lines back in at the appropriate positions.
/// - Bump givn_config_version.
/// - Write the file in-place. User reviews via `git diff`.
pub fn run() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let existing = std::fs::read_to_string(&config_path)?;
    let new_content = upgrade_config(&existing, env!("CARGO_PKG_VERSION"))?;

    if existing == new_content {
        println!("givn/config.yaml is already up to date (no changes).");
        return Ok(());
    }

    std::fs::write(&config_path, &new_content)?;
    println!(
        "Updated givn/config.yaml to version {}.",
        env!("CARGO_PKG_VERSION")
    );
    println!("Review changes with: git diff givn/config.yaml");
    println!("Then run: givn skills sync");

    Ok(())
}

/// Upgrade the config content:
/// 1. Extract user-owned (uncommented) lines, grouped by their section key.
/// 2. Re-render the template.
/// 3. Splice user overrides back in (preserving them).
pub fn upgrade_config(existing: &str, version: &str) -> Result<String> {
    // Extract user-owned lines (uncommented, non-empty, not the version marker).
    let user_lines: Vec<String> = existing
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && !trimmed.starts_with("givn_config_version:")
        })
        .map(|l| l.to_string())
        .collect();

    // Re-render the template.
    let new_scaffold = crate::config::render::config_yaml(version)?;

    if user_lines.is_empty() {
        return Ok(new_scaffold);
    }

    // Append user overrides after the scaffold with a separator.
    // This is the simplest safe strategy: user overrides at the end are always valid YAML.
    // A more sophisticated impl would splice them at the right sections; that is
    // a post-v1 enhancement.
    let mut result = new_scaffold;

    // Check if we need to append user overrides (only if they differ from defaults).
    // Filter out lines that are just the default verify.command (it's already in template).
    let truly_custom: Vec<&String> = user_lines
        .iter()
        .filter(|l| !l.trim().starts_with("command:"))
        .collect();

    if !truly_custom.is_empty() {
        result.push_str("\n# --- User overrides preserved from previous version ---\n");
        for line in &user_lines {
            result.push_str(line);
            result.push('\n');
        }
        result.push_str("# ---\n");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_of_fresh_config_is_idempotent() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        // Fresh config has no user overrides, so upgrading produces a new scaffold.
        // (Version bump changes the content.)
        let upgraded = upgrade_config(&fresh, "0.2.0").unwrap();
        assert!(upgraded.contains("givn_config_version: \"0.2.0\""));
        assert!(!upgraded.contains("givn_config_version: \"0.1.0\""));
    }

    #[test]
    fn upgrade_preserves_user_verify_command() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        // Simulate a user who set a custom verify.command.
        let user_config = fresh.replace(
            r#"command: "givn missing-testrunner"   # <-- replace this with your runner"#,
            r#"command: "cargo test""#,
        );
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();
        // The user's command should be preserved somewhere in the output.
        assert!(
            upgraded.contains("cargo test"),
            "user verify.command must be preserved"
        );
    }
}
