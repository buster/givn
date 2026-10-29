use anyhow::{Result, bail};

/// Refresh managed project files and, when required, prepare the migration plan.
///
/// Strategy (ADR-023):
/// - Parse the existing givn/config.yaml into two regions:
///   - "managed" = commented lines (givn's doc scaffold)
///   - "user-owned" = uncommented key/value lines (never touched)
/// - Re-render the managed scaffold from the current binary's template.
/// - Splice user-owned lines back in at the appropriate positions.
/// - Bump givn_config_version.
/// - Synchronize generated agent integrations through the managed path.
/// - Commit only the managed path; a declared migration is written separately.
pub fn run() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let existing = std::fs::read_to_string(&config_path)?;
    validate_upgrade_config(&existing)?;
    let source_version = active_config_version(&existing);
    let target_version = env!("CARGO_PKG_VERSION");

    if let Some(change_id) = crate::cli::migration::pending_migration_change(&project_root) {
        println!(
            "Migration change '{change_id}' already exists; project is already prepared for migration."
        );
        return Ok(());
    }

    if let Some(source) = source_version.as_deref() {
        let bundles = crate::cli::migration::migration_path(source, target_version)?;
        if !bundles.is_empty() {
            let change_id = crate::cli::migration::aggregate_migration_id(source, target_version);
            if crate::cli::migration::migration_exists(&project_root, &change_id) {
                println!(
                    "Migration change '{change_id}' already exists; project is already prepared for migration."
                );
                return Ok(());
            }
            return crate::cli::migration::prepare_legacy_upgrade(
                &project_root,
                &config_path,
                &existing,
                source,
                target_version,
                &bundles,
            );
        }
    }

    let commit_message = format!(
        "chore(givn): update managed files from {} to {target_version}",
        source_version.as_deref().unwrap_or("unknown")
    );
    match crate::cli::migration::prepare_managed_upgrade(
        &project_root,
        &config_path,
        &existing,
        source_version.as_deref(),
        target_version,
        &commit_message,
    )? {
        Some(hash) => println!(
            "Refreshed commit attribution plumbing and managed Givn files for {target_version} in commit {hash}; no migration change was required."
        ),
        None => {
            println!("Managed Givn files are already up to date for {target_version} (no changes).")
        }
    }
    Ok(())
}

/// Read the active version marker without interpreting commented scaffold
/// examples as project configuration.
pub(crate) fn active_config_version(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            return None;
        }
        let value = trimmed.strip_prefix("givn_config_version:")?;
        let value = value.split('#').next().unwrap_or(value).trim();
        let value = value.trim_matches(['"', '\'']);
        (!value.is_empty()).then(|| value.to_string())
    })
}

/// Refuse configuration that needs manual repair before any managed write.
///
/// A 0.x version marker alone is not a configuration-format defect: 0.9.10
/// accepts manually prepared current-format configuration with that marker.
/// Malformed YAML, retired addon keys, and executable settings misplaced in
/// config.yaml are refused with the 1.0 baseline and manual-transition paths.
pub(crate) fn validate_upgrade_config(existing: &str) -> Result<()> {
    let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(existing).map_err(|error| {
        anyhow::anyhow!(
            "givn upgrade refused: givn/config.yaml is malformed and must be repaired manually \
             before upgrading ({error}). This is not the version marker: 0.9.10 accepts current-format \
             configuration carrying a 0.x marker. The supported migration \
             baseline is 1.0. Repair the YAML and run `givn upgrade` again. No files were changed."
        )
    })?;
    let Some(root) = document.as_mapping() else {
        anyhow::bail!(
            "givn upgrade refused: givn/config.yaml must be a YAML mapping. This is not caused \
             by the 0.x version marker; the supported migration baseline is 1.0. Repair it \
             manually. No files were changed."
        );
    };

    for key in ["verify", "run", "coverage"] {
        if root.contains_key(key) {
            anyhow::bail!(
                "givn upgrade refused: '{key}' is executable configuration and belongs in \
                 givn/commands.yaml, not givn/config.yaml. Move the '{key}' block manually; the \
                 supported migration baseline is 1.0. No files were changed."
            );
        }
    }

    let mut retired = Vec::new();
    for key in root.keys() {
        if let Some(key) = key.as_str()
            && crate::config::manifest::Addons::replacement_for_retired(key).is_some()
        {
            retired.push(key.to_string());
        }
    }
    if let Some(addons) = root.get("addons").and_then(|value| value.as_mapping()) {
        for key in addons.keys() {
            if let Some(key) = key.as_str()
                && crate::config::manifest::Addons::replacement_for_retired(key).is_some()
            {
                retired.push(key.to_string());
            }
        }
    }
    if let Some(retired) = retired.first() {
        let replacement = crate::config::manifest::Addons::replacement_for_retired(retired)
            .unwrap_or("the current setting");
        anyhow::bail!(
            "givn upgrade refused: the retired configuration key '{retired}' requires a manual \
             transition to '{replacement}'. Givn 0.9.10 does not automatically migrate retired \
             configuration and accepts no compatibility alias. The supported migration baseline \
             is 1.0; review the communication_policy, challenge, and skills-only transitions by \
             hand, then run `givn upgrade` again. No files were changed."
        );
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
    validate_upgrade_config(existing)?;
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

/// The run scaffold appended by the backfill — the same text the config
/// template embeds (`commands.yaml.tmpl` includes `run.yaml.tmpl` verbatim).
pub const RUN_SCAFFOLD: &str = include_str!("../../assets/run.yaml.tmpl");

/// Append the run scaffold to `existing` unless a top-level `run:` key is
/// already present (line-based scan; a commented example is not a key).
/// Returns `None` when nothing changed, which makes the backfill idempotent.
pub fn backfill_commands(existing: &str) -> Option<String> {
    let has_run = existing.lines().any(|line| line.starts_with("run:"));
    if has_run {
        return None;
    }
    Some(format!("{}\n\n{RUN_SCAFFOLD}", existing.trim_end()))
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
    fn upgrade_preserves_user_apply_as_override() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let mut user_config = fresh.clone();
        user_config.push_str("\napply:\n  requires: [tasks]\n");
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();
        assert!(
            upgraded.contains("requires: [tasks]"),
            "user apply override must be preserved, got:\n{upgraded}"
        );
    }

    #[test]
    fn upgrade_produces_valid_yaml_with_user_overrides() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let mut user_config = fresh.clone();
        user_config.push_str("\napply:\n  requires: [tasks]\n");
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&upgraded);
        assert!(
            parsed.is_ok(),
            "upgraded config.yaml must be valid YAML, got error: {:?}\n---\n{upgraded}",
            parsed.err()
        );
        assert!(
            upgraded.contains("requires: [tasks]"),
            "user override must be preserved"
        );
    }

    #[test]
    fn upgrade_preserves_user_rules_override() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let mut user_config = fresh.clone();
        user_config.push_str("\nrules: versioned\n");
        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        assert!(
            upgraded.contains("rules: versioned"),
            "user rules override must be in the upgraded output"
        );
    }

    #[test]
    fn upgrade_preserves_addons_with_user_overrides() {
        let fresh = crate::config::render::config_yaml("0.1.0").unwrap();
        let mut user_config = fresh.clone();
        user_config.push_str("\naddons:\n  arc42: true\n");

        let upgraded = upgrade_config(&user_config, "0.2.0").unwrap();

        let parsed: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&upgraded);
        assert!(
            parsed.is_ok(),
            "upgraded config.yaml must be valid YAML: {:?}",
            parsed.err()
        );
        assert!(upgraded.contains("arc42: true"));
    }

    #[test]
    fn active_config_version_ignores_commented_examples() {
        let config = "# givn_config_version: \"0.1.0\"\n\
                       givn_config_version: \"0.2.0\" # managed\n";
        assert_eq!(active_config_version(config).as_deref(), Some("0.2.0"));
    }

    #[test]
    fn active_config_version_accepts_unquoted_values() {
        assert_eq!(
            active_config_version("givn_config_version: 0.2.0\n").as_deref(),
            Some("0.2.0")
        );
    }

    #[test]
    fn upgrade_accepts_a_0_x_version_marker_on_current_format_configuration() {
        let existing = "\
givn_config_version: \"0.7.0\"
addons:
  outline: false
";
        let upgraded = upgrade_config(existing, "0.9.10").unwrap();
        let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(&upgraded).unwrap();
        assert_eq!(
            parsed.get("givn_config_version").and_then(|v| v.as_str()),
            Some("0.9.10")
        );
        assert!(upgraded.contains("outline: false"), "{upgraded}");
    }

    #[test]
    fn upgrade_refuses_retired_addon_configuration_before_writing() {
        for (retired, replacement) in [
            ("absolute_mode", "communication_policy"),
            ("grill_me", "challenge"),
            ("grill_with_docs", "challenge"),
        ] {
            let existing = format!("givn_config_version: \"0.7.0\"\naddons:\n  {retired}: true\n");
            let error = upgrade_config(&existing, "0.9.10")
                .expect_err("retired configuration must require manual transition");
            let message = error.to_string();
            assert!(message.contains(retired), "{message}");
            assert!(message.contains(replacement), "{message}");
            assert!(message.contains("manual"), "{message}");
            assert!(message.contains("1.0"), "{message}");
        }
    }

    #[test]
    fn upgrade_refuses_malformed_configuration_before_writing() {
        let existing = "givn_config_version: \"0.7.0\"\naddons: [unclosed\n";
        let error = upgrade_config(existing, "0.9.10")
            .expect_err("malformed configuration must be refused");
        let message = error.to_string();
        assert!(message.contains("malformed"), "{message}");
        assert!(message.contains("not the version marker"), "{message}");
    }

    #[test]
    fn upgrade_refuses_executable_configuration_misplaced_in_config_yaml() {
        let existing = "\
givn_config_version: \"0.7.0\"
verify:
  command: \"cargo test\"
";
        let error = upgrade_config(existing, "0.9.10")
            .expect_err("command configuration belongs in commands.yaml");
        let message = error.to_string();
        assert!(message.contains("verify"), "{message}");
        assert!(message.contains("givn/commands.yaml"), "{message}");
        assert!(message.contains("manual"), "{message}");
    }
}
