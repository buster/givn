use crate::config::overlay::load_effective;
use crate::skills::render::{
    SkillContext, default_skill_names, render_skill, resolve_skill_template,
};
/// Write rendered skills and commands to all target locations:
/// - `.agents/skills/<name>/SKILL.md`   (primary neutral)
/// - `.claude/skills/<name>/SKILL.md`   (Claude Code mirror)
/// - `.opencode/skills/<name>/SKILL.md` (opencode mirror)
/// - `.opencode/commands/<name>.md`     (opencode slash commands)
///
/// `AGENTS.md` at the repo root is otherwise user-owned. The only exception:
/// givn manages its own fenced `<!-- givn:begin:* -->` blocks inside it:
/// - `givn-project` — always present, a short pointer to `givn instructions`.
/// - `absolute-mode` — present only when that addon is enabled.
///
/// Both are kept current on every call to `sync`.
use anyhow::Result;
use std::path::Path;

/// Regenerate all skills and commands in the project.
pub fn sync(project_root: &Path) -> Result<()> {
    let manifest = load_effective()?;
    let verify_command = &manifest.verify.command;
    let features_path = &manifest.verify.features_path;
    let version = env!("CARGO_PKG_VERSION");

    let mut skill_names = default_skill_names();
    if manifest.addons.domain_modeling {
        skill_names.push("givn-domain-modeling");
    }
    if manifest.addons.dev_principles {
        skill_names.push("givn-dev-principles");
    }

    // Render and write each skill to all targets.
    let ctx = SkillContext {
        change_id: "<change-id>",
        verify_command,
        features_path,
        givn_version: version,
    };

    for name in &skill_names {
        let template = resolve_skill_template_for(name)?;
        let rendered = render_skill(&template, &ctx);

        // Validate frontmatter.
        validate_frontmatter(&rendered, name)?;

        write_skill_to_targets(project_root, name, &rendered)?;
        println!("  synced skill: {name}");
    }

    // Write opencode slash commands.
    let mut command_names = default_command_names();
    if manifest.addons.grill_me {
        command_names.push("grill-me");
    }
    if manifest.addons.grill_with_docs {
        command_names.push("grill-with-docs");
    }
    for name in &command_names {
        let content = resolve_command_template_for(name)?;
        write_opencode_command(project_root, name, &content)?;
        println!("  synced command: /{name}");
    }

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

/// Write a rendered SKILL.md to all three skill target locations.
fn write_skill_to_targets(project_root: &Path, name: &str, content: &str) -> Result<()> {
    let targets = [
        format!(".agents/skills/{name}/SKILL.md"),
        format!(".claude/skills/{name}/SKILL.md"),
        format!(".opencode/skills/{name}/SKILL.md"),
    ];

    for target in &targets {
        let path = project_root.join(target);
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
