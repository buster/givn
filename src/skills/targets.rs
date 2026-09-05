use crate::config::overlay::load_effective;
use crate::skills::render::{
    SkillContext, default_skill_names, render_template, resolve_skill_template,
};
use anyhow::{Context, Result};
/// Write rendered skills and commands to the configured target locations:
/// - `.opencode/skills/<name>/SKILL.md` when `opencode` is selected
/// - `.agents/skills/<name>/SKILL.md` when `agents` is selected
/// - `.claude/skills/<name>/SKILL.md` when `claude` is selected
/// - `.opencode/commands/<name>.md` when `opencode` is selected
///
/// The target set comes from the `skills:` block of `givn/config.yaml`
/// (default: `[opencode]`). See the agent-integration architecture documentation.
///
/// `AGENTS.md` at the repo root is otherwise user-owned. The only exception:
/// givn manages its own fenced `<!-- givn:begin:* -->` blocks inside it:
/// - `givn-project` — always present, a short pointer to `givn instructions`.
/// - `absolute-mode` — present only when that addon is enabled.
///
/// Both are kept current on every call to `sync`.
use std::path::Path;

/// A canonical agent target for generated skills/commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentTarget {
    /// `.opencode/` — opencode skills mirror + slash commands.
    Opencode,
    /// `.agents/` — the neutral skills location.
    Agents,
    /// `.claude/` — Claude Code skills mirror.
    Claude,
}

impl AgentTarget {
    /// All known targets.
    pub const ALL: [AgentTarget; 3] = [
        AgentTarget::Opencode,
        AgentTarget::Agents,
        AgentTarget::Claude,
    ];

    /// Canonical config name (no leading dot).
    pub fn name(&self) -> &'static str {
        match self {
            AgentTarget::Opencode => "opencode",
            AgentTarget::Agents => "agents",
            AgentTarget::Claude => "claude",
        }
    }

    /// Directory name under the project root.
    pub fn dir(&self) -> &'static str {
        match self {
            AgentTarget::Opencode => ".opencode",
            AgentTarget::Agents => ".agents",
            AgentTarget::Claude => ".claude",
        }
    }

    /// Name shown in the init prompt (fully-qualified directory form).
    pub fn display(&self) -> &'static str {
        self.dir()
    }

    /// Resolve a (possibly dotted) name to a target.
    pub fn from_name(name: &str) -> Option<Self> {
        match name
            .trim()
            .trim_start_matches('.')
            .to_ascii_lowercase()
            .as_str()
        {
            "opencode" => Some(AgentTarget::Opencode),
            "agents" => Some(AgentTarget::Agents),
            "claude" => Some(AgentTarget::Claude),
            _ => None,
        }
    }
}

/// Resolve the configured agent targets from `givn/config.yaml`.
fn configured_targets(project_root: &Path) -> Result<Vec<AgentTarget>> {
    // load_skills_targets already defaults to [opencode] when the config has
    // no skills block or an empty/invalid list, so the result never needs a
    // second fallback here.
    let names = crate::config::overlay::load_skills_targets(project_root)?;
    let mut targets = Vec::new();
    for name in names {
        if let Some(t) = AgentTarget::from_name(&name) {
            targets.push(t);
        }
    }
    Ok(targets)
}

/// Regenerate all skills and commands in the project.
pub fn sync(project_root: &Path) -> Result<()> {
    let manifest = load_effective()?;
    let verify_command = &manifest.verify.command;
    let features_path = &manifest.verify.features_path;
    let version = env!("CARGO_PKG_VERSION");
    let targets = configured_targets(project_root)?;

    let addons = addon_flags(&manifest);
    let domain_skill = domain_modeling_present(&manifest);
    let mut skill_names = default_skill_names();
    if domain_skill {
        skill_names.push("givn-domain-modeling");
    }
    if manifest.addons.dev_principles {
        skill_names.push("givn-dev-principles");
    }

    // Render and write each skill to the configured targets.
    let ctx = SkillContext {
        change_id: "<change-id>",
        verify_command,
        features_path,
        givn_version: version,
    };

    for name in &skill_names {
        let template = resolve_skill_template_for(name)?;
        let rendered = render_template(&template, &ctx, &addons)
            .with_context(|| format!("failed to render skill template for '{name}'"))?;

        // Validate frontmatter.
        validate_frontmatter(&rendered, name)?;

        write_skill_to_targets(project_root, name, &rendered, &targets)?;
        println!("  synced skill: {name}");
    }

    // Write opencode slash commands (opencode target only).
    if targets.contains(&AgentTarget::Opencode) {
        let mut command_names = default_command_names();
        if manifest.addons.grill_me {
            command_names.push("grill-me");
        }
        if manifest.addons.grill_with_docs {
            command_names.push("grill-with-docs");
        }
        for name in &command_names {
            let template = resolve_command_template_for(name)?;
            let content = render_template(&template, &ctx, &addons)
                .with_context(|| format!("failed to render command template for '{name}'"))?;
            write_opencode_command(project_root, name, &content)?;
            println!("  synced command: /{name}");
        }
    }

    // Self-reconcile: remove managed generated files that are no longer valid
    // for the current configuration.
    reconcile_addon_artifacts(project_root, &manifest, &targets)?;

    // Always-on blocks: unconditional, refreshed every sync.
    crate::cli::project_hint::apply(project_root)?;

    // Self-healing AGENTS.md block: keep it present/absent in sync with the
    // addon flag on every init/sync, independent of how it got out of sync.
    if manifest.addons.absolute_mode {
        crate::cli::absolute_mode::apply(project_root)?;
    } else {
        crate::cli::absolute_mode::remove(project_root)?;
    }

    Ok(())
}

/// Whether the domain-modeling skill should be generated. It requires the
/// arc42 addon (the project's glossary), mirroring the router's coupling.
fn domain_modeling_present(manifest: &crate::config::manifest::Manifest) -> bool {
    manifest.addons.domain_modeling && manifest.addons.arc42
}

/// Build the addon-name/enabled map passed into the template renderer.
fn addon_flags(manifest: &crate::config::manifest::Manifest) -> Vec<(&'static str, bool)> {
    vec![
        ("arc42", manifest.addons.arc42),
        ("domain_modeling", manifest.addons.domain_modeling),
        ("dev_principles", manifest.addons.dev_principles),
        ("grill_me", manifest.addons.grill_me),
        ("grill_with_docs", manifest.addons.grill_with_docs),
        ("absolute_mode", manifest.addons.absolute_mode),
        ("readme", manifest.addons.readme),
        ("coverage", manifest.addons.coverage),
    ]
}

/// Remove addon-conditional generated files whose addon is no longer enabled.
/// Only skills/commands that givn itself manages are touched.
fn reconcile_addon_artifacts(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
    targets: &[AgentTarget],
) -> Result<()> {
    if !manifest.addons.domain_modeling || !manifest.addons.arc42 {
        for target in targets {
            let dir = project_root
                .join(target.dir())
                .join("skills/givn-domain-modeling");
            if dir.exists() {
                std::fs::remove_dir_all(&dir)?;
            }
        }
    }
    if !manifest.addons.dev_principles {
        for target in targets {
            let dir = project_root
                .join(target.dir())
                .join("skills/givn-dev-principles");
            if dir.exists() {
                std::fs::remove_dir_all(&dir)?;
            }
        }
    }
    if !manifest.addons.grill_me {
        let file = project_root.join(".opencode/commands/grill-me.md");
        if file.exists() {
            std::fs::remove_file(&file)?;
        }
    }
    if !manifest.addons.grill_with_docs {
        let file = project_root.join(".opencode/commands/grill-with-docs.md");
        if file.exists() {
            std::fs::remove_file(&file)?;
        }
    }
    Ok(())
}

/// Return the project paths that `sync` may write for the current effective
/// configuration. The upgrade migration uses this list as its Git staging
/// allowlist; it must never stage an entire generated directory.
pub fn managed_generated_paths(project_root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let manifest = load_effective()?;
    let targets = configured_targets(project_root)?;
    let mut paths = vec![project_root.join("AGENTS.md")];

    let mut skill_names = default_skill_names();
    if domain_modeling_present(&manifest) {
        skill_names.push("givn-domain-modeling");
    }
    if manifest.addons.dev_principles {
        skill_names.push("givn-dev-principles");
    }

    for target in &targets {
        for name in &skill_names {
            paths.push(
                project_root
                    .join(target.dir())
                    .join("skills")
                    .join(name)
                    .join("SKILL.md"),
            );
        }
    }

    if targets.contains(&AgentTarget::Opencode) {
        let mut command_names = default_command_names();
        if manifest.addons.grill_me {
            command_names.push("grill-me");
        }
        if manifest.addons.grill_with_docs {
            command_names.push("grill-with-docs");
        }
        for name in command_names {
            paths.push(project_root.join(format!(".opencode/commands/{name}.md")));
        }
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// Write a rendered SKILL.md to the configured skill target locations.
fn write_skill_to_targets(
    project_root: &Path,
    name: &str,
    content: &str,
    targets: &[AgentTarget],
) -> Result<()> {
    for target in targets {
        let path = project_root
            .join(target.dir())
            .join("skills")
            .join(name)
            .join("SKILL.md");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content)?;
    }

    Ok(())
}

/// Write an opencode command file to `.opencode/commands/<name>.md`.
fn write_opencode_command(project_root: &Path, name: &str, content: &str) -> Result<()> {
    let path = project_root.join(format!(".opencode/commands/{name}.md"));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    Ok(())
}

/// Resolve a command template: project override wins over embedded default.
/// Project override path: `givn/commands/<name>.md`
/// Embedded path: `commands/<name>.md`
fn resolve_command_template(name: &str) -> Result<String> {
    crate::config::cascade::resolve_asset(
        &format!("commands/{name}.md"),
        &format!("givn/commands/{name}.md"),
    )
}

/// The default opencode command names (one per givn workflow action).
pub fn default_command_names() -> Vec<&'static str> {
    vec![
        "givn-propose",
        "givn-spec",
        "givn-design",
        "givn-tasks",
        "givn-implement",
        "givn-review",
        "givn-archive",
        "givn-explore",
        "givn-design-review",
        "givn-steps",
        "givn-reverse-engineer",
        "givn-characterize",
        "givn-ideation",
    ]
}

/// Resolve a skill template by name.
fn resolve_skill_template_for(name: &str) -> Result<String> {
    resolve_skill_template(name)
}

/// Resolve a command template by name.
fn resolve_command_template_for(name: &str) -> Result<String> {
    resolve_command_template(name)
}

/// Validate SKILL.md frontmatter (name matches dir, description present and bounded).
fn validate_frontmatter(content: &str, expected_name: &str) -> Result<()> {
    if !content.starts_with("---") {
        anyhow::bail!(
            "skill template for '{expected_name}' must start with YAML frontmatter (---)"
        );
    }

    let fm_end = content[3..].find("\n---").unwrap_or(0) + 3;
    let frontmatter = &content[3..fm_end];

    if !frontmatter.contains(&format!("name: {expected_name}")) {
        anyhow::bail!(
            "skill template frontmatter name must match directory name '{expected_name}'"
        );
    }

    if !frontmatter.contains("description:") {
        anyhow::bail!(
            "skill template for '{expected_name}' must have a 'description' in frontmatter"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::embedded::Assets;

    #[test]
    fn validate_frontmatter_requires_name_match() {
        let content = "---\nname: givn-propose\ndescription: Test skill\n---\n\nContent";
        assert!(validate_frontmatter(content, "givn-propose").is_ok());
        assert!(validate_frontmatter(content, "givn-spec").is_err());
    }

    #[test]
    fn validate_frontmatter_requires_description() {
        let content = "---\nname: givn-propose\n---\n\nContent";
        assert!(validate_frontmatter(content, "givn-propose").is_err());
    }

    #[test]
    fn all_command_templates_are_embedded() {
        for name in default_command_names() {
            let key = format!("commands/{name}.md");
            assert!(
                Assets::get(&key).is_some(),
                "embedded command template missing: {key}"
            );
        }
    }
}
