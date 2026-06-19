/// Write rendered skills to all target locations:
/// - `.agents/skills/<name>/SKILL.md`   (primary neutral)
/// - `.claude/skills/<name>/SKILL.md`   (Claude Code mirror)
/// - `.opencode/skills/<name>/SKILL.md` (opencode mirror)
/// - `AGENTS.md` at repo root
use anyhow::Result;
use std::path::Path;
use crate::skills::render::{SkillContext, render_skill, render_agents_md, resolve_skill_template, default_skill_names};
use crate::config::overlay::load_effective;

/// Regenerate all skills and AGENTS.md in the project.
pub fn sync(project_root: &Path) -> Result<()> {
    let manifest = load_effective()?;
    let verify_command = &manifest.verify.command;
    let features_path = &manifest.verify.features_path;
    let version = env!("CARGO_PKG_VERSION");

    let skill_names = default_skill_names();

    // Render and write each skill to all targets.
    let ctx = SkillContext {
        change_id: "<change-id>",
        verify_command,
        features_path,
        givn_version: version,
    };

    for name in &skill_names {
        let template = resolve_skill_template(name)?;
        let rendered = render_skill(&template, &ctx);

        // Validate frontmatter.
        validate_frontmatter(&rendered, name)?;

        write_skill_to_targets(project_root, name, &rendered)?;
        println!("  synced skill: {name}");
    }

    // Render and write AGENTS.md.
    let skill_refs: Vec<&str> = skill_names.clone();
    let agents_content = render_agents_md(verify_command, features_path, version, &skill_refs)?;
    let agents_path = project_root.join("AGENTS.md");
    std::fs::write(&agents_path, &agents_content)?;
    println!("  synced AGENTS.md");

    Ok(())
}

/// Write a rendered SKILL.md to all three target locations.
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

/// Validate SKILL.md frontmatter (name matches dir, description present and bounded).
fn validate_frontmatter(content: &str, expected_name: &str) -> Result<()> {
    if !content.starts_with("---") {
        anyhow::bail!("skill template for '{expected_name}' must start with YAML frontmatter (---)");
    }

    let fm_end = content[3..].find("\n---").unwrap_or(0) + 3;
    let frontmatter = &content[3..fm_end];

    if !frontmatter.contains(&format!("name: {expected_name}")) {
        anyhow::bail!(
            "skill template frontmatter name must match directory name '{expected_name}'"
        );
    }

    if !frontmatter.contains("description:") {
        anyhow::bail!("skill template for '{expected_name}' must have a 'description' in frontmatter");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
