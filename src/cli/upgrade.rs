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
/// - Auto-enable any KNOWN_ADDONS not yet present in the config.
pub fn run() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let existing = std::fs::read_to_string(&config_path)?;
    let present_addons = crate::cli::addons::parse_enabled_addons(&existing);

    let new_content = upgrade_config(&existing, env!("CARGO_PKG_VERSION"))?;

    let needs_write = existing != new_content;

    if needs_write {
        std::fs::write(&config_path, &new_content)?;
        println!(
            "Updated givn/config.yaml to version {}.",
            env!("CARGO_PKG_VERSION")
        );
        println!("Review changes with: git diff givn/config.yaml");
        println!("Then run: givn skills sync");
    } else {
        println!("givn/config.yaml is already up to date (no changes).");
    }

    // Auto-enable new known addons not present in the existing config.
    let mut new_addons_enabled = false;
    for (name, _) in crate::cli::addons::KNOWN_ADDONS {
        if !present_addons.contains_key(*name) {
            println!("\nEnabling newly-available addon '{name}'...");
            crate::cli::addons::run_enable(name)?;
            new_addons_enabled = true;
        }
    }

    if new_addons_enabled {
        println!("Run `givn skills sync` to regenerate skill files.");
    }

    Ok(())
}

/// Upgrade the config content:
/// 1. Extract user-owned (uncommented) lines, grouped by top-level key.
/// 2. Re-render the template.
/// 3. For each user group whose top-level key is already active (uncommented)
///    in the fresh scaffold (currently only `verify:`), REPLACE that block in
///    place rather than duplicating the key — a duplicate top-level key is
///    invalid YAML and breaks every downstream `givn` command.
/// 4. Any user group whose key is not active in the fresh scaffold (e.g.
///    `addons:`, which ships commented-out) is appended at the end.
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

    let groups = group_by_top_level_key(&user_lines);
    let mut result = new_scaffold;
    let mut appended: Vec<&(String, Vec<String>)> = Vec::new();

    for group in &groups {
        let (key, lines) = group;
        if let Some(replaced) = replace_active_top_level_block(&result, key, lines) {
            result = replaced;
        } else {
            appended.push(group);
        }
    }

    if !appended.is_empty() {
        result.push_str("\n# --- User overrides preserved from previous version ---\n");
        for (_, lines) in appended {
            for line in lines {
                result.push_str(line);
                result.push('\n');
            }
        }
        result.push_str("# ---\n");
    }

    Ok(result)
}

/// Group user-owned lines by their top-level YAML key. A top-level key line
/// has no leading whitespace and matches `<key>:` (with or without a value
/// on the same line). Every subsequent indented line belongs to that group.
fn group_by_top_level_key(lines: &[String]) -> Vec<(String, Vec<String>)> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();

    for line in lines {
        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        if is_top_level {
            let key = line.split(':').next().unwrap_or(line).trim().to_string();
            groups.push((key, vec![line.clone()]));
        } else if let Some((_, current)) = groups.last_mut() {
            current.push(line.clone());
        }
        // A stray indented line with no preceding top-level key is malformed
        // input; silently dropped rather than panicking on upgrade.
    }

    groups
}

/// If `key` appears as an active (uncommented) top-level key in `scaffold`,
/// replace its entire block (the key line plus every following indented
/// line) with `replacement_lines`. Returns `None` if the key is not active
/// in the scaffold (it ships commented-out, or absent), signalling the
/// caller to append the group instead.
fn replace_active_top_level_block(
    scaffold: &str,
    key: &str,
    replacement_lines: &[String],
) -> Option<String> {
    let lines: Vec<&str> = scaffold.lines().collect();
    let start = lines.iter().position(|l| {
        !l.starts_with(' ')
            && !l.starts_with('\t')
            && l.split(':').next().unwrap_or(l).trim() == key
    })?;

    let mut end = start + 1;
    while end < lines.len() && (lines[end].starts_with(' ') || lines[end].starts_with('\t')) {
        end += 1;
    }

    let mut result = String::new();
    for line in &lines[..start] {
        result.push_str(line);
        result.push('\n');
    }
    for line in replacement_lines {
        result.push_str(line);
        result.push('\n');
    }
    for line in &lines[end..] {
        result.push_str(line);
        result.push('\n');
    }

    Some(result)
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

    #[test]
    fn upgrade_produces_valid_yaml_with_custom_verify_command() {
        // Regression test: a duplicate top-level `verify:` key is invalid
        // YAML and broke every downstream `givn` command after upgrade.
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let user_config = fresh.replace(
            r#"command: "givn missing-testrunner"   # <-- replace this with your runner"#,
            r#"command: "./gradlew test""#,
        );
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&upgraded);
        assert!(
            parsed.is_ok(),
            "upgraded config.yaml must be valid YAML, got error: {:?}\n---\n{upgraded}",
            parsed.err()
        );

        assert_eq!(
            upgraded.matches("\nverify:").count() + usize::from(upgraded.starts_with("verify:")),
            1,
            "verify: must appear exactly once as a top-level key"
        );
    }

    #[test]
    fn upgrade_replaces_verify_block_in_place_not_at_end() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let user_config = fresh.replace(
            r#"command: "givn missing-testrunner"   # <-- replace this with your runner"#,
            r#"command: "./gradlew test""#,
        );
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        // The replaced verify: block should still be near the top (inside
        // the managed scaffold section), not appended after the "User
        // overrides preserved" marker with everything else.
        let verify_pos = upgraded.find("verify:").unwrap();
        let overrides_marker_pos = upgraded.find("User overrides preserved");
        if let Some(marker_pos) = overrides_marker_pos {
            assert!(
                verify_pos < marker_pos,
                "verify: block should be replaced in place, not appended after the overrides marker"
            );
        }
    }

    #[test]
    fn upgrade_preserves_addons_alongside_custom_verify_command() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let mut user_config = fresh.replace(
            r#"command: "givn missing-testrunner"   # <-- replace this with your runner"#,
            r#"command: "./gradlew test""#,
        );
        user_config.push_str("\naddons:\n  arc42: true\n");

        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&upgraded);
        assert!(
            parsed.is_ok(),
            "upgraded config.yaml must be valid YAML: {:?}",
            parsed.err()
        );
        assert!(upgraded.contains("./gradlew test"));
        assert!(upgraded.contains("arc42: true"));
    }
}
