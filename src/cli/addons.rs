/// `givn addons` subcommands: list, enable, disable.
///
/// Addons are pre-packaged artifact sets embedded in the binary and enabled
/// via the `addons:` key in `givn/config.yaml`. See arc42 ch.8.11.
use anyhow::{Result, bail};
use std::io::IsTerminal;

/// One registered addon.
pub struct AddonSpec {
    pub id: &'static str,
    pub description: &'static str,
    pub default_enabled: bool,
}

/// Registry of all known addons.
pub const KNOWN_ADDONS: &[AddonSpec] = &[
    AddonSpec {
        id: "arc42",
        description: "Arc42 architecture documentation — creates and maintains durable docs/arc42/ chapters",
        default_enabled: true,
    },
    AddonSpec {
        id: "communication_policy",
        description: "Communication policy — manages a fenced presentation and documentation block in AGENTS.md; the dialog obligation is independent of this addon",
        default_enabled: true,
    },
    AddonSpec {
        id: "readme",
        description: "README scaffolding — writes an initial README.md to the project root",
        default_enabled: true,
    },
    AddonSpec {
        id: "coverage",
        description: "Archive-level coverage gate — runs project measurement and merge scripts, updates README from the merged Cobertura XML",
        default_enabled: true,
    },
    AddonSpec {
        id: "domain_modeling",
        description: "Skill — maintain the standalone glossary and structural domain model before design; does not require arc42",
        default_enabled: true,
    },
    AddonSpec {
        id: "dev_principles",
        description: "Skill — apply the team's engineering principles during implementation",
        default_enabled: true,
    },
    AddonSpec {
        id: "challenge",
        description: "Skill — research proposals, tests, specs, design, code, and whole changes before asking unresolved decisions; no lifecycle gate",
        default_enabled: true,
    },
    AddonSpec {
        id: "architecture_principles",
        description: "Architecture-principles guidance — last-responsible-moment decision timing, consumer contracts, component splits and port/adapter boundaries; enabled by default",
        default_enabled: true,
    },
    AddonSpec {
        id: "outline",
        description: "Outline report — enabled by default; disable with `givn addons disable outline`",
        default_enabled: true,
    },
];

/// `givn addons list` — print all known addons and their effective enabled state.
///
/// The listing resolves the same effective manifest every other command uses:
/// the embedded defaults, then the project overlay. An omitted addon in a
/// partial block therefore reports its resolved default, not "absent".
pub fn run_list() -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");
    let manifest = crate::config::overlay::load_effective()?;

    println!("Available addons:\n");
    for addon in KNOWN_ADDONS {
        let id = addon.id;
        let description = addon.description;
        let state = if manifest.addons.get(id).unwrap_or(addon.default_enabled) {
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
        println!("These are effective values: omitted keys use the enabled default.");
        println!("Use `givn addons enable <name>` or `givn addons disable <name>`");
        println!("to toggle an addon. Changes are written to givn/config.yaml.");
    }

    Ok(())
}

/// `givn addons enable <name>` — set addon to true in givn/config.yaml, run
/// any one-time scaffolding for that addon, then regenerate skills/commands so
/// generated guidance always matches the new configuration.
pub fn run_enable(
    name: &str,
    operator: bool,
    reason: Option<&str>,
    change: Option<&str>,
) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let before = crate::config::overlay::load_effective()?
        .addons
        .get(name)
        .unwrap_or(true);
    let target = guard_addon_mutation(&project_root, name, operator, reason, change)?;
    enable_scaffold(name)?;
    if let Some(change_dir) = target {
        crate::config::lock::record_override(
            &change_dir,
            name,
            before,
            true,
            reason.unwrap_or_default(),
        )?;
    }
    crate::skills::targets::sync(&project_root)?;
    Ok(())
}

/// The scaffolding half of enabling an addon (writes config, runs one-time
/// materialization, but does NOT regenerate skills/commands). `givn init`
/// uses this per addon and syncs once at the end.
pub fn enable_scaffold(name: &str) -> Result<()> {
    validate_addon_name(name)?;
    set_addon(&std::env::current_dir()?, name, true)?;
    println!("enabled addon '{name}' in givn/config.yaml");
    println!();
    scaffold_enabled(name)
}

/// Materialize one addon's one-time files without changing its stored setting.
/// `givn init --force` uses this after restoring the project's explicit addon
/// choices, so a disabled addon is never scaffolded.
pub(crate) fn scaffold_enabled(name: &str) -> Result<()> {
    // Addon-specific scaffolding.
    if name == "arc42" {
        scaffold_arc42()?;
        scaffold_glossary()?;
        println!("The '{name}' artifact is now part of the effective workflow.");
        println!("It will appear after 'design' in `givn status`.");
        println!();
        println!("Next steps:");
        println!("  givn status              — see the updated workflow");
        println!("  givn eject {name}:instruction   — customise the instruction (optional)");
    } else if name == "domain_modeling" {
        scaffold_glossary()?;
        scaffold_domain_model()?;
        println!("The standalone domain documents follow the `domain_modeling` addon.");
        println!("The generated skill is refreshed on the next `givn skills sync`.");
        println!();
    } else if name == "communication_policy" {
        println!(
            "The managed communication policy block follows this setting on the next `givn skills sync`."
        );
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

/// Write the standalone glossary when it is missing. The scaffold never
/// overwrites an existing file.
fn scaffold_glossary() -> Result<()> {
    let project_root = std::env::current_dir()?;
    scaffold_document_at(
        &project_root,
        "docs/glossary.md",
        "templates/glossary.md",
        "glossary",
    )
}

/// Write the standalone structural model when it is missing. The scaffold
/// never overwrites an existing file.
fn scaffold_domain_model() -> Result<()> {
    let project_root = std::env::current_dir()?;
    scaffold_document_at(
        &project_root,
        "docs/domain-model.md",
        "templates/domain-model.md",
        "domain model",
    )
}

/// Materialize one embedded documentation template when the destination does
/// not exist. Existing content is project-owned and preserved byte-for-byte.
fn scaffold_document_at(
    project_root: &std::path::Path,
    relative_path: &str,
    asset_key: &str,
    label: &str,
) -> Result<()> {
    let dest = project_root.join(relative_path);
    if dest.exists() {
        println!("  exists   {relative_path}  (content preserved)");
        return Ok(());
    }

    let content = crate::config::embedded::Assets::get_str(asset_key)
        .ok_or_else(|| anyhow::anyhow!("embedded {label} template missing: {asset_key}"))?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, content)?;
    println!("  created  {relative_path}");
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
pub fn run_disable(
    name: &str,
    operator: bool,
    reason: Option<&str>,
    change: Option<&str>,
) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let before = crate::config::overlay::load_effective()?
        .addons
        .get(name)
        .unwrap_or(true);
    let target = guard_addon_mutation(&project_root, name, operator, reason, change)?;
    disable_scaffold(name)?;
    if let Some(change_dir) = target {
        crate::config::lock::record_override(
            &change_dir,
            name,
            before,
            false,
            reason.unwrap_or_default(),
        )?;
    }
    crate::skills::targets::sync(&project_root)?;
    Ok(())
}

/// Refuse an add-on mutation while a locked change is active. The operator
/// override requires an interactive terminal and a recorded reason, so it is
/// not an ordinary autonomous command.
fn guard_addon_mutation(
    project_root: &std::path::Path,
    addon: &str,
    operator: bool,
    reason: Option<&str>,
    change: Option<&str>,
) -> Result<Option<std::path::PathBuf>> {
    let locked = crate::config::lock::locked_active_changes(project_root)?;
    if locked.is_empty() {
        return Ok(None);
    }
    if !operator {
        bail!(
            "add-on configuration is operator-owned and is locked by active change(s): {}.              `givn addons` cannot change `{addon}` during a change. Install the missing dependency              in an isolated environment or report an environment failure; do not disable its gate.              An operator can override on an interactive terminal with `--operator --reason \"<reason>\"`.",
            locked
                .iter()
                .map(|(id, _)| id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if !std::io::stdin().is_terminal() {
        bail!(
            "the add-on operator override requires an interactive terminal; no configuration was changed"
        );
    }
    let reason = reason.unwrap_or_default().trim();
    if reason.is_empty() {
        bail!(
            "the add-on operator override requires a non-empty --reason; no configuration was changed"
        );
    }
    let change_dir = match change {
        Some(id) => {
            let dir = crate::state::resolve_change_dir(project_root, id)?;
            if crate::config::lock::read(&dir)?.is_none() {
                bail!("change '{id}' has no add-on lock to override");
            }
            dir
        }
        None if locked.len() == 1 => locked[0].1.clone(),
        None => bail!(
            "several locked changes are active; select one with --change <id>: {}",
            locked
                .iter()
                .map(|(id, _)| id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    Ok(Some(change_dir))
}

/// The scaffold/cleanup half of disabling an addon (writes config, removes
/// addon-owned files, but does NOT regenerate skills/commands).
pub fn disable_scaffold(name: &str) -> Result<()> {
    validate_addon_name(name)?;
    set_addon(&std::env::current_dir()?, name, false)?;
    println!("disabled addon '{name}' in givn/config.yaml");
    println!();

    if name == "arc42" {
        println!("The '{name}' artifact is no longer part of the workflow.");
        println!("Existing docs/arc42/ files are not deleted.");
    } else if name == "communication_policy" {
        println!("The managed communication block is removed on the next `givn skills sync`.");
        println!("The mandatory dialog obligation is unchanged.");
        println!("Any other content in AGENTS.md was left untouched.");
    } else if name == "readme" {
        println!("The 'readme' addon is disabled. The existing README.md is not deleted.");
    } else if name == "coverage" {
        println!("The 'coverage' gate is now disabled.");
        println!("The coverage fence in README.md is preserved.");
    } else if is_skill_addon(name) {
        remove_skill_addon_files(name);
        println!("Removed the '{name}' skill files from every agent target.");
    }

    Ok(())
}

// ── Internals ────────────────────────────────────────────────────────────────

fn validate_addon_name(name: &str) -> Result<()> {
    if KNOWN_ADDONS.iter().any(|addon| addon.id == name) {
        return Ok(());
    }
    if let Some(replacement) = crate::config::manifest::Addons::replacement_for_retired(name) {
        bail!(
            "retired addon '{name}' is not supported. Rename it manually to '{replacement}' in \
             givn/config.yaml — givn 0.9.10 does not convert retired configuration or accept a \
             compatibility alias. The supported migration baseline is 1.0; review the \
             communication_policy, challenge, and skills-only transitions by hand. \
             No files were changed."
        );
    }
    let known: Vec<&str> = KNOWN_ADDONS.iter().map(|addon| addon.id).collect();
    bail!("unknown addon '{name}'. Known addons: {}", known.join(", "))
}

/// The project's explicit addon choices, in registry order. Used by
/// `givn init --force` to restore those choices after re-rendering the
/// managed scaffold. Retired or non-boolean settings are refused before any
/// write instead of being silently dropped.
pub(crate) fn explicit_addon_settings(content: &str) -> Result<Vec<(String, bool)>> {
    let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(content)
        .map_err(|e| anyhow::anyhow!("givn/config.yaml is not valid YAML: {e}"))?;
    let Some(root) = document.as_mapping() else {
        return Ok(Vec::new());
    };
    let Some(addons) = root.get("addons") else {
        return Ok(Vec::new());
    };
    let Some(addons) = addons.as_mapping() else {
        bail!(
            "the addons setting in givn/config.yaml must be a block mapping; no files were changed"
        );
    };
    for key in addons.keys() {
        let Some(key) = key.as_str() else {
            bail!("addon settings must use string keys; no files were changed");
        };
        validate_addon_name(key)?;
    }
    let mut settings = Vec::new();
    for addon in KNOWN_ADDONS {
        let Some(value) = addons.get(addon.id) else {
            continue;
        };
        let Some(enabled) = value.as_bool() else {
            bail!(
                "addon '{}' must be true or false in givn/config.yaml; no files were changed",
                addon.id
            );
        };
        settings.push((addon.id.to_string(), enabled));
    }
    Ok(settings)
}

/// Render explicit addon choices as an ordinary block mapping.
pub(crate) fn addons_block(settings: &[(String, bool)]) -> String {
    if settings.is_empty() {
        return String::new();
    }
    let mut block = String::from("addons:\n");
    for (name, enabled) in settings {
        block.push_str(&format!("  {name}: {enabled}\n"));
    }
    block
}

/// Write an explicit `false` for every known add-on. `givn init --no-addons`
/// uses this so the opt-out is visible in the project's config instead of
/// depending on an embedded default.
pub(crate) fn disable_all(project_root: &std::path::Path) -> Result<()> {
    for addon in KNOWN_ADDONS {
        set_addon(project_root, addon.id, false)?;
    }
    Ok(())
}

/// Write `addons.<name>: <value>` into givn/config.yaml without changing any
/// other setting. Ordinary block mappings are edited in place; flow-style or
/// otherwise ambiguous structures are refused before the file is written.
fn set_addon(project_root: &std::path::Path, name: &str, enabled: bool) -> Result<()> {
    let config_path = project_root.join("givn/config.yaml");
    if !config_path.exists() {
        bail!("givn/config.yaml not found. Run `givn init` first.");
    }
    let content = std::fs::read_to_string(&config_path)?;
    let updated = edit_addons_block(&content, name, enabled)?;
    let parsed: serde_yaml_ng::Value = serde_yaml_ng::from_str(&updated)
        .map_err(|e| anyhow::anyhow!("refusing to write invalid givn/config.yaml: {e}"))?;
    let actual = parsed
        .get("addons")
        .and_then(|value| value.as_mapping())
        .and_then(|addons| addons.get(name))
        .and_then(|value| value.as_bool());
    if actual != Some(enabled) {
        bail!(
            "refusing to write givn/config.yaml: the edited addon '{name}' did not resolve to \
             the requested value; no files were changed"
        );
    }
    std::fs::write(&config_path, updated)?;
    Ok(())
}

fn active_addons_lines(lines: &[&str]) -> Vec<usize> {
    let mut found = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() || line.starts_with([' ', '\t', '#']) {
            continue;
        }
        let Some((key, _)) = line.split_once(':') else {
            continue;
        };
        if key.trim() == "addons" {
            found.push(index);
        }
    }
    found
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

fn edit_addons_block(content: &str, name: &str, enabled: bool) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let addons_lines = active_addons_lines(&lines);
    if addons_lines.len() > 1 {
        bail!(
            "givn/config.yaml has multiple active addons blocks; refusing to edit. Keep exactly \
             one addons block and edit givn/config.yaml manually. No files were changed."
        );
    }

    let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(content)
        .map_err(|e| anyhow::anyhow!("givn/config.yaml is not valid YAML: {e}"))?;
    let root = document
        .as_mapping()
        .ok_or_else(|| anyhow::anyhow!("givn/config.yaml must be a YAML mapping"))?;
    let value = if enabled { "true" } else { "false" };

    let Some(start) = addons_lines.first().copied() else {
        if root.get("addons").is_some() {
            bail!(
                "the addons setting in givn/config.yaml is not an ordinary block mapping; \
                 refusing to edit. Edit givn/config.yaml manually. No files were changed."
            );
        }
        let mut updated = content.trim_end().to_string();
        updated.push_str(&format!("\n\naddons:\n  {name}: {value}\n"));
        return Ok(updated);
    };

    let colon = lines[start]
        .find(':')
        .ok_or_else(|| anyhow::anyhow!("malformed addons line in givn/config.yaml"))?;
    let rest = lines[start][colon + 1..].trim();
    if !rest.is_empty() && !rest.starts_with('#') {
        bail!(
            "the addons setting in givn/config.yaml is not an ordinary block mapping (flow \
             style, anchor, or inline value); refusing to edit. Rewrite it as an indented block \
             and edit givn/config.yaml manually. No files were changed."
        );
    }
    match root.get("addons").and_then(|value| value.as_mapping()) {
        Some(_) => {}
        None => {
            bail!(
                "the addons setting in givn/config.yaml is not a block mapping; refusing to edit. \
                 Edit givn/config.yaml manually. No files were changed."
            )
        }
    }

    let mut end = start + 1;
    while end < lines.len() {
        let line = lines[end];
        if line.is_empty()
            || line.trim_start().starts_with('#')
            || line.starts_with(' ')
            || line.starts_with('\t')
        {
            end += 1;
        } else {
            break;
        }
    }

    let child_indent = (start + 1..end)
        .find_map(|index| {
            let line = lines[index];
            let trimmed = line.trim_start();
            (!trimmed.is_empty() && !trimmed.starts_with('#'))
                .then(|| leading_spaces(line))
                .filter(|indent| *indent > 0)
        })
        .unwrap_or(2);

    let mut matches = Vec::new();
    for (index, line) in lines.iter().enumerate().take(end).skip(start + 1) {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') || leading_spaces(line) != child_indent {
            continue;
        }
        if let Some((key, _)) = trimmed.split_once(':')
            && key.trim() == name
        {
            matches.push(index);
        }
    }
    if matches.len() > 1 {
        bail!(
            "givn/config.yaml sets addon '{name}' more than once; refusing to edit. Keep one \
             setting and edit givn/config.yaml manually. No files were changed."
        );
    }

    let mut updated = lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    if let Some(index) = matches.first().copied() {
        let line = lines[index];
        let indent = " ".repeat(child_indent);
        let Some((_, after_colon)) = line.trim_start().split_once(':') else {
            bail!("malformed addon line for '{name}' in givn/config.yaml");
        };
        let (before_comment, comment) = match after_colon.find('#') {
            Some(hash) => (&after_colon[..hash], &after_colon[hash..]),
            None => (after_colon, ""),
        };
        let current = before_comment.trim();
        if current != "true" && current != "false" {
            bail!(
                "addon '{name}' in givn/config.yaml does not use a true/false value; refusing to \
                 edit. Edit givn/config.yaml manually. No files were changed."
            );
        }
        let comment = if comment.is_empty() {
            String::new()
        } else {
            format!(" {}", comment)
        };
        updated[index] = format!("{indent}{name}: {value}{comment}");
    } else {
        if root
            .get("addons")
            .and_then(|value| value.as_mapping())
            .is_some_and(|mapping| mapping.contains_key(name))
        {
            bail!(
                "addon '{name}' in givn/config.yaml uses a shape this tool cannot safely edit; \
                 refusing to edit. Edit givn/config.yaml manually. No files were changed."
            );
        }
        updated.insert(
            start + 1,
            format!("{}{name}: {value}", " ".repeat(child_indent)),
        );
    }

    Ok(format!("{}\n", updated.join("\n")))
}

/// Map from addon name to the generated skill name (or None if not a skill addon).
fn addon_skill_name(name: &str) -> Option<&'static str> {
    match name {
        "domain_modeling" => Some("givn-domain-modeling"),
        "dev_principles" => Some("givn-dev-principles"),
        "challenge" => Some("givn-challenge"),
        _ => None,
    }
}

fn is_skill_addon(name: &str) -> bool {
    addon_skill_name(name).is_some()
}

/// Delete generated skill directories from all target locations for a skill addon.
fn remove_skill_addon_files(addon: &str) {
    let Some(skill_name) = addon_skill_name(addon) else {
        return;
    };
    let project_root = std::env::current_dir().expect("current dir");
    for base in &[".agents", ".claude", ".opencode", ".omp", ".maki"] {
        let dir = project_root.join(format!("{base}/skills/{skill_name}"));
        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(validate_addon_name("architecture_principles").is_ok());
        assert!(validate_addon_name("communication_policy").is_ok());
        assert!(validate_addon_name("challenge").is_ok());
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
        assert_eq!(addon_skill_name("challenge"), Some("givn-challenge"));
        assert_eq!(addon_skill_name("arc42"), None);
    }

    fn config_fixture(content: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let givn = directory.path().join("givn");
        std::fs::create_dir(&givn).unwrap();
        let path = givn.join("config.yaml");
        std::fs::write(&path, content).unwrap();
        (directory, path)
    }

    #[test]
    fn addon_edits_refuse_flow_style_addons_before_writing() {
        let content = "givn_config_version: \"0.9.9\"\naddons: {challenge: true}\n";
        let (directory, path) = config_fixture(content);
        let error = set_addon(directory.path(), "challenge", false).unwrap_err();
        assert!(error.to_string().contains("block"), "{error}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), content);
    }

    #[test]
    fn addon_edits_refuse_anchored_addons_before_writing() {
        let content = "addons: &defaults\n  challenge: true\n";
        let (directory, path) = config_fixture(content);
        let error = set_addon(directory.path(), "challenge", false).unwrap_err();
        assert!(error.to_string().contains("block"), "{error}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), content);
    }

    #[test]
    fn addon_edits_refuse_duplicate_addons_sections_before_writing() {
        let content = "addons:\n  challenge: true\naddons:\n  outline: false\n";
        let (directory, path) = config_fixture(content);
        let error = set_addon(directory.path(), "challenge", false).unwrap_err();
        assert!(error.to_string().contains("addons"), "{error}");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), content);
    }

    #[test]
    fn addon_edits_change_only_the_key_inside_the_addons_block() {
        let content = "\
givn_config_version: \"0.9.9\"
other:
  challenge: true
addons:
  challenge: true # keep project choice
  outline: false
";
        let (directory, _) = config_fixture(content);
        set_addon(directory.path(), "challenge", false).unwrap();
        let updated = std::fs::read_to_string(directory.path().join("givn/config.yaml")).unwrap();
        assert!(
            updated.contains("other:\n  challenge: true"),
            "a same-named key outside addons must not change: {updated}"
        );
        assert!(
            updated.contains("  challenge: false # keep project choice"),
            "the intended addon key must change and keep its comment: {updated}"
        );
        assert!(updated.contains("  outline: false"), "{updated}");
        assert_eq!(updated.matches("challenge:").count(), 2, "{updated}");
    }

    #[test]
    fn addon_edits_leave_the_commented_template_untouched() {
        let content = "\
givn_config_version: \"0.9.9\"
# addons:
#   challenge: true
";
        let (directory, _) = config_fixture(content);
        set_addon(directory.path(), "challenge", false).unwrap();
        let updated = std::fs::read_to_string(directory.path().join("givn/config.yaml")).unwrap();
        assert!(
            updated.contains("# addons:\n#   challenge: true"),
            "commented documentation must stay commented: {updated}"
        );
        assert!(
            updated.contains("addons:\n  challenge: false"),
            "a fresh active addons block must be appended: {updated}"
        );
    }

    #[test]
    fn disable_all_writes_an_explicit_false_for_every_current_addon() {
        let content = "givn_config_version: \"0.9.9\"\n";
        let (directory, path) = config_fixture(content);
        disable_all(directory.path()).unwrap();
        let updated = std::fs::read_to_string(&path).unwrap();
        for addon in KNOWN_ADDONS {
            assert!(
                updated.contains(&format!("  {}: false", addon.id)),
                "{} must be explicitly disabled: {updated}",
                addon.id
            );
        }
        assert!(
            updated.contains("  communication_policy: false"),
            "{updated}"
        );
        assert!(updated.contains("  challenge: false"), "{updated}");
    }

    #[test]
    fn retired_addon_names_name_their_manual_replacement() {
        for (retired, replacement) in [
            ("absolute_mode", "communication_policy"),
            ("grill_me", "challenge"),
            ("grill_with_docs", "challenge"),
        ] {
            let error = validate_addon_name(retired).expect_err("retired addon must be refused");
            let message = error.to_string();
            assert!(message.contains(retired), "{message}");
            assert!(message.contains(replacement), "{message}");
            assert!(message.contains("manual"), "{message}");
        }
    }
    #[test]
    fn documentation_templates_are_embedded() {
        for key in ["templates/glossary.md", "templates/domain-model.md"] {
            assert!(
                crate::config::embedded::Assets::get(key).is_some(),
                "embedded documentation template missing: {key}"
            );
        }
    }

    #[test]
    fn scaffold_document_preserves_existing_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let docs = directory.path().join("docs");
        std::fs::create_dir_all(&docs).unwrap();
        std::fs::write(docs.join("glossary.md"), "# Project glossary\n").unwrap();

        scaffold_document_at(
            directory.path(),
            "docs/glossary.md",
            "templates/glossary.md",
            "glossary",
        )
        .unwrap();

        assert_eq!(
            std::fs::read_to_string(docs.join("glossary.md")).unwrap(),
            "# Project glossary\n"
        );
    }

    #[test]
    fn scaffold_document_creates_a_missing_file_from_the_embedded_template() {
        let directory = tempfile::tempdir().unwrap();

        scaffold_document_at(
            directory.path(),
            "docs/domain-model.md",
            "templates/domain-model.md",
            "domain model",
        )
        .unwrap();

        let created = std::fs::read_to_string(directory.path().join("docs/domain-model.md"))
            .expect("domain model scaffold");
        assert!(
            created.contains("## Boundaries and responsibilities"),
            "{created}"
        );
        assert!(created.contains("layout: elk"), "{created}");
    }

    #[test]
    fn addon_registry_matches_the_manifest_schema_and_defaults() {
        let manifest = crate::config::manifest::Manifest::load_embedded().unwrap();
        assert_eq!(
            KNOWN_ADDONS.len(),
            crate::config::manifest::Addons::NAMES.len()
        );
        for addon in KNOWN_ADDONS {
            assert_eq!(
                manifest.addons.get(addon.id),
                Some(addon.default_enabled),
                "registry entry '{}' does not match the manifest schema",
                addon.id
            );
        }
        for name in crate::config::manifest::Addons::NAMES {
            assert!(
                KNOWN_ADDONS.iter().any(|addon| addon.id == name),
                "manifest addon '{name}' is missing from the registry"
            );
        }
    }
}
