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
        "givn-explore",
        "givn-design-review",
        "givn-reverse-engineer",
        "givn-characterize",
    ]
}
