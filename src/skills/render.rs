use anyhow::Result;

/// Context variables injected into skill templates.
pub struct SkillContext<'a> {
    pub change_id: &'a str,
    pub verify_command: &'a str,
    pub features_path: &'a str,
    pub givn_version: &'a str,
}

/// Render a SKILL.md.tmpl by substituting context variables.
pub fn render_skill(template: &str, ctx: &SkillContext<'_>) -> String {
    template
        .replace("{{CHANGE_ID}}", ctx.change_id)
        .replace("{{VERIFY_COMMAND}}", ctx.verify_command)
        .replace("{{FEATURES_PATH}}", ctx.features_path)
        .replace("{{GIVN_VERSION}}", ctx.givn_version)
}

/// Render the AGENTS.md template.
pub fn render_agents_md(
    verify_command: &str,
    features_path: &str,
    givn_version: &str,
    skills: &[&str],
) -> Result<String> {
    let template = crate::config::cascade::resolve_asset("AGENTS.md.tmpl", "givn/AGENTS.md.tmpl")?;

    let skills_list = if skills.is_empty() {
        "  (none configured)".to_string()
    } else {
        skills
            .iter()
            .map(|s| format!("- `{s}`"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(template
        .replace("{{VERIFY_COMMAND}}", verify_command)
        .replace("{{FEATURES_PATH}}", features_path)
        .replace("{{GIVN_VERSION}}", givn_version)
        .replace("{{SKILLS_LIST}}", &skills_list))
}

/// Get the embedded skill template for a skill name.
/// Falls back to project override at `givn/skills/<name>/SKILL.md.tmpl`.
pub fn resolve_skill_template(skill_name: &str) -> Result<String> {
    crate::config::cascade::resolve_asset(
        &format!("skills/{skill_name}/SKILL.md.tmpl"),
        &format!("givn/skills/{skill_name}/SKILL.md.tmpl"),
    )
}

/// List all default skill names (from the embedded assets).
pub fn default_skill_names() -> Vec<&'static str> {
    vec![
        "givn-propose",
        "givn-spec",
        "givn-design",
        "givn-tasks",
        "givn-implement",
        "givn-review",
        "givn-archive",
    ]
}
