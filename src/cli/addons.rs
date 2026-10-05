/// `givn addons` subcommands: list, enable, disable.
///
/// Addons are pre-packaged artifact sets embedded in the binary and enabled
/// via the `addons:` key in `givn/config.yaml`. See arc42 ch.8.11.
use anyhow::{Result, bail};

/// Registry of all known addons (id, one-line description).
pub const KNOWN_ADDONS: &[(&str, &str)] = &[
    (
        "arc42",
        "Arc42 architecture documentation — creates and maintains durable docs/arc42/ chapters",
    ),
    (
        "absolute_mode",
        "Absolute Mode communication policy — manages a fenced block in AGENTS.md",
    ),
    (
        "readme",
        "README scaffolding — writes an initial README.md to the project root",
    ),
    (
        "coverage",
        "Archive-level coverage gate — runs project measurement and merge scripts, updates README from the merged Cobertura XML",
    ),
    (
        "domain_modeling",
        "Skill — model a change's domain (ubiquitous language, aggregates, invariants) before design",
    ),
    (
        "dev_principles",
        "Skill — apply the team's engineering principles during implementation",
    ),
    (
        "grill_me",
        "Command — Socratically grill the current plan/design, one question at a time",
    ),
    (
        "grill_with_docs",
        "Command — grill the plan/design, citing a source in the project's own docs for every challenge",
    ),
];

/// `givn addons list` — print all known addons and their enabled/disabled status.
pub fn run_list() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    // Read enabled state from config (if it exists).
    let enabled = if config_path.exists() {
        let yaml = std::fs::read_to_string(&config_path)?;
        parse_enabled_addons(&yaml)
    } else {
        std::collections::HashMap::new()
    };

    println!("Available addons:\n");
    for (id, description) in KNOWN_ADDONS {
        let state = if *enabled.get(*id).unwrap_or(&false) {
            "enabled "
        } else {
            "disabled"
        };
        println!("  [{state}]  {id}");
        println!("            {description}");
        println!();
    }

    if !config_path.exists() {
        println!("Tip: run `givn init` first to create givn/config.yaml, then");
        println!("     `givn addons enable <name>` to enable an addon.");
    } else {
        println!("Use `givn addons enable <name>` or `givn addons disable <name>`");
        println!("to toggle an addon. Changes are written to givn/config.yaml.");
    }

    Ok(())
}

/// `givn addons enable <name>` — set addon to true in givn/config.yaml, run
/// any one-time scaffolding for that addon, then regenerate skills/commands so
/// generated guidance always matches the new configuration.
pub fn run_enable(name: &str) -> Result<()> {
    enable_scaffold(name)?;
    crate::skills::targets::sync(&std::env::current_dir()?)?;
    Ok(())
}

/// The scaffolding half of enabling an addon (writes config, runs one-time
/// materialization, but does NOT regenerate skills/commands). `givn init`
/// uses this per addon and syncs once at the end.
pub fn enable_scaffold(name: &str) -> Result<()> {
    validate_addon_name(name)?;
    set_addon(name, true)?;
    println!("enabled addon '{name}' in givn/config.yaml");
    println!();

    // Addon-specific scaffolding.
    if name == "arc42" {
        scaffold_arc42()?;
        println!("The '{name}' artifact is now part of the effective workflow.");
        println!("It will appear after 'design' in `givn status`.");
        println!();
        println!("Next steps:");
        println!("  givn status              — see the updated workflow");
        println!("  givn eject {name}:instruction   — customise the instruction (optional)");
    } else if name == "absolute_mode" {
        let project_root = std::env::current_dir()?;
        crate::cli::absolute_mode::apply(&project_root)?;
        println!("Wrote the Absolute Mode policy block to AGENTS.md.");
        println!("Only the fenced <!-- givn:begin:absolute-mode --> block was added;");
        println!("the rest of AGENTS.md is untouched.");
    } else if name == "readme" {
        let count = crate::cli::readme::scaffold_readme()?;
        if count > 0 {
            println!("Scaffolded project README.md.");
        } else {
            println!("README.md already exists (content preserved).");
        }
        println!();
        println!("The 'readme' addon does not modify the workflow graph.");
        println!("To customise the template, eject and edit it: givn eject readme:template");
    } else if name == "coverage" {
        println!("The '{name}' gate is now enabled.");
        println!("Coverage is measured at archive time (not per-change).");
        println!();
        println!("Next steps:");
        println!("  Edit givn/commands.yaml to set coverage.measure_command");
        println!("  and coverage.merge_command to use your project scripts.");
    }

    Ok(())
}

/// Scaffold `docs/arc42/` chapter files from the embedded templates.
///
/// Each file is written only if it does not already exist — existing content
/// (including fully-filled chapters) is never overwritten.
fn scaffold_arc42() -> Result<()> {
    use crate::config::embedded::Assets;

    let project_root = std::env::current_dir()?;
    let out_dir = project_root.join("docs/arc42");
    std::fs::create_dir_all(&out_dir)?;

    // All embedded chapter files live under addons/arc42-docs/chapters/.
    let prefix = "addons/arc42-docs/chapters/";
    let mut written = Vec::new();
    let mut skipped = Vec::new();

    // Iterate embedded assets whose path starts with the chapters prefix.
    for path in Assets::iter() {
        let path: &str = &path;
        let Some(filename) = path.strip_prefix(prefix) else {
            continue;
        };
        let dest = out_dir.join(filename);
        if dest.exists() {
            skipped.push(filename.to_string());
        } else {
            let content = Assets::get_str(path)
                .ok_or_else(|| anyhow::anyhow!("embedded asset missing: {path}"))?;
            // Nested chapter templates (e.g. adr/adr-template.md) keep their
            // relative subdirectory under docs/arc42/.
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dest, content)?;
            written.push(filename.to_string());
        }
    }

    if !written.is_empty() {
        println!("Scaffolded docs/arc42/:");
        for f in &written {
            println!("  created  docs/arc42/{f}");
        }
    }
    if !skipped.is_empty() {
        println!("Skipped (already exist — content preserved):");
        for f in &skipped {
            println!("  exists   docs/arc42/{f}");
        }
    }
    println!();

    Ok(())
}

/// `givn addons disable <name>` — set addon to false in givn/config.yaml,
/// run any cleanup for that addon, then regenerate skills/commands so
/// generated guidance matches the new configuration.
pub fn run_disable(name: &str) -> Result<()> {
    disable_scaffold(name)?;
    crate::skills::targets::sync(&std::env::current_dir()?)?;
    Ok(())
}

/// The scaffold/cleanup half of disabling an addon (writes config, removes
/// addon-owned files, but does NOT regenerate skills/commands).
pub fn disable_scaffold(name: &str) -> Result<()> {
    validate_addon_name(name)?;
    set_addon(name, false)?;
    println!("disabled addon '{name}' in givn/config.yaml");
    println!();

    if name == "arc42" {
        println!("The '{name}' artifact is no longer part of the workflow.");
        println!("Existing docs/arc42/ files are not deleted.");
    } else if name == "absolute_mode" {
        let project_root = std::env::current_dir()?;
        crate::cli::absolute_mode::remove(&project_root)?;
        println!("Removed the Absolute Mode block from AGENTS.md.");
        println!("Any other content in AGENTS.md was left untouched.");
    } else if name == "readme" {
        println!("The 'readme' addon is disabled. The existing README.md is not deleted.");
    } else if name == "coverage" {
        println!("The 'coverage' gate is now disabled.");
        println!("The coverage fence in README.md is preserved.");
    } else if is_skill_addon(name) {
        remove_skill_addon_files(name);
        println!("Removed the '{name}' skill files from .agents/, .claude/, .opencode/.");
    } else if is_command_addon(name) {
        remove_command_addon_files(name);
        println!("Removed the '{name}' command file from .opencode/commands/.");
    }

    Ok(())
}

// ── Internals ────────────────────────────────────────────────────────────────

fn validate_addon_name(name: &str) -> Result<()> {
    if KNOWN_ADDONS.iter().any(|(id, _)| *id == name) {
        Ok(())
    } else {
        let known: Vec<&str> = KNOWN_ADDONS.iter().map(|(id, _)| *id).collect();
        bail!("unknown addon '{name}'. Known addons: {}", known.join(", "))
    }
}

/// Write `addons.<name>: <value>` into givn/config.yaml.
///
/// Strategy (line-based, preserves all other content):
///   1. If `addons:` block already exists with the key → replace the value line.
///   2. If `addons:` block exists without the key → insert the key after `addons:`.
///   3. If no `addons:` block exists → append a new block at the end.
fn set_addon(name: &str, enabled: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }

    let content = std::fs::read_to_string(&config_path)?;
    let value_str = if enabled { "true" } else { "false" };
    let key_line = format!("  {name}: {value_str}");

    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();

    // Case 1: key already exists (uncommented) — replace its value in place.
    for line in &mut lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with(&format!("{name}:"))
            && !line.trim_start_matches(' ').starts_with('#')
        {
            // We're inside addons block if the key is indented.
            if line.starts_with("  ") && !line.starts_with("   ") {
                *line = key_line.clone();
                std::fs::write(&config_path, lines.join("\n") + "\n")?;
                return Ok(());
            }
        }
    }

    // Case 2: `addons:` block exists (uncommented) — insert/replace key inside it.
    for i in 0..lines.len() {
        let trimmed = lines[i].trim();
        if trimmed == "addons:" {
            // Check if the key exists commented-out on the next few lines.
            // Uncomment it, or insert after `addons:`.
            let mut inserted = false;
            for (_, line) in lines.iter_mut().enumerate().skip(i + 1) {
                // Stop if we hit a new top-level key.
                if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                    break;
                }
                // Uncomment or replace a commented version of this key.
                if line.contains(&format!("{name}:")) {
                    *line = key_line.clone();
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                lines.insert(i + 1, key_line.clone());
            }
            std::fs::write(&config_path, lines.join("\n") + "\n")?;
            return Ok(());
        }
    }

    // Case 3: no `addons:` block at all — look for commented-out addons block
    // and uncomment it, or append a new one.
    for i in 0..lines.len() {
        if lines[i].trim_start_matches('#').trim() == "addons:" {
            // Found a commented `# addons:` — uncomment the block.
            lines[i] = "addons:".to_string();
            // Uncomment or insert the specific key on the next line.
            let mut key_set = false;
            for (_, line) in lines.iter_mut().enumerate().skip(i + 1) {
                if line
                    .trim_start_matches([' ', '#'])
                    .starts_with(&format!("{name}:"))
                {
                    *line = key_line.clone();
                    key_set = true;
                    break;
                }
                // Stop at next non-comment, non-blank, non-indented line.
                if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                    break;
                }
            }
            if !key_set {
                lines.insert(i + 1, key_line.clone());
            }
            std::fs::write(&config_path, lines.join("\n") + "\n")?;
            return Ok(());
        }
    }

    // Append a fresh addons block.
    if !lines.last().map(|l| l.is_empty()).unwrap_or(true) {
        lines.push(String::new());
    }
    lines.push("addons:".to_string());
    lines.push(key_line);
    std::fs::write(&config_path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Parse which addons are enabled from config YAML (simple line scan,
/// no full YAML parse — avoids clobbering comments).
pub(crate) fn parse_enabled_addons(yaml: &str) -> std::collections::HashMap<String, bool> {
    let mut result = std::collections::HashMap::new();
    let mut in_addons = false;

    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if trimmed == "addons:" {
            in_addons = true;
            continue;
        }
        if in_addons {
            // Stop if we hit a new top-level key (non-indented, non-blank).
            if !line.is_empty() && !line.starts_with(' ') {
                break;
            }
            // Parse `  <name>: true|false`.
            if let Some(rest) = trimmed.strip_suffix(": true") {
                result.insert(rest.to_string(), true);
            } else if let Some(rest) = trimmed.strip_suffix(": false") {
                result.insert(rest.to_string(), false);
            }
        }
    }

    result
}

/// Map from addon name to the generated skill name (or None if not a skill addon).
fn addon_skill_name(name: &str) -> Option<&'static str> {
    match name {
        "domain_modeling" => Some("givn-domain-modeling"),
        "dev_principles" => Some("givn-dev-principles"),
        _ => None,
    }
}

/// Map from addon name to the generated command name (or None if not a command addon).
fn addon_command_name(name: &str) -> Option<&'static str> {
    match name {
        "grill_me" => Some("grill-me"),
        "grill_with_docs" => Some("grill-with-docs"),
        _ => None,
    }
}

fn is_skill_addon(name: &str) -> bool {
    addon_skill_name(name).is_some()
}

fn is_command_addon(name: &str) -> bool {
    addon_command_name(name).is_some()
}

/// Delete generated skill directories from all target locations for a skill addon.
fn remove_skill_addon_files(addon: &str) {
    let Some(skill_name) = addon_skill_name(addon) else {
        return;
    };
    let project_root = std::env::current_dir().expect("current dir");
    for base in &[".agents", ".claude", ".opencode", ".omp"] {
        let dir = project_root.join(format!("{base}/skills/{skill_name}"));
        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

/// Delete the generated command file for a command addon.
fn remove_command_addon_files(addon: &str) {
    let Some(cmd_name) = addon_command_name(addon) else {
        return;
    };
    let project_root = std::env::current_dir().expect("current dir");
    for base in &[".opencode", ".omp"] {
        let file = project_root.join(format!("{base}/commands/{cmd_name}.md"));
        if file.exists() {
            let _ = std::fs::remove_file(&file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_enabled_addons_finds_true() {
        let yaml = "addons:\n  arc42: true\n";
        let map = parse_enabled_addons(yaml);
        assert_eq!(map.get("arc42"), Some(&true));
    }

    #[test]
    fn parse_enabled_addons_finds_false() {
        let yaml = "addons:\n  arc42: false\n";
        let map = parse_enabled_addons(yaml);
        assert_eq!(map.get("arc42"), Some(&false));
    }

    #[test]
    fn parse_enabled_addons_ignores_comments() {
        let yaml = "# addons:\n#   arc42: true\nverify:\n  command: cargo test\n";
        let map = parse_enabled_addons(yaml);
        assert!(map.is_empty());
    }

    #[test]
    fn validate_addon_name_rejects_unknown() {
        assert!(validate_addon_name("nonexistent").is_err());
    }

    #[test]
    fn validate_addon_name_accepts_arc42() {
        assert!(validate_addon_name("arc42").is_ok());
    }

    #[test]
    fn validate_addon_name_accepts_readme() {
        assert!(validate_addon_name("readme").is_ok());
    }

    #[test]
    fn validate_addon_name_accepts_coverage() {
        assert!(validate_addon_name("coverage").is_ok());
    }

    #[test]
    fn validate_addon_name_accepts_new_addons() {
        assert!(validate_addon_name("domain_modeling").is_ok());
        assert!(validate_addon_name("dev_principles").is_ok());
        assert!(validate_addon_name("grill_me").is_ok());
        assert!(validate_addon_name("grill_with_docs").is_ok());
    }

    #[test]
    fn addon_skill_name_maps_correctly() {
        assert_eq!(
            addon_skill_name("domain_modeling"),
            Some("givn-domain-modeling")
        );
        assert_eq!(
            addon_skill_name("dev_principles"),
            Some("givn-dev-principles")
        );
        assert_eq!(addon_skill_name("grill_me"), None);
        assert_eq!(addon_skill_name("arc42"), None);
    }

    #[test]
    fn addon_command_name_maps_correctly() {
        assert_eq!(addon_command_name("grill_me"), Some("grill-me"));
        assert_eq!(
            addon_command_name("grill_with_docs"),
            Some("grill-with-docs")
        );
        assert_eq!(addon_command_name("domain_modeling"), None);
    }
}
