//! Template rendering for generated skills and commands.
//!
//! Two layers:
//! 1. Conditional blocks — `{{#if addons.<name>}}` … `{{else}}` … `{{/if}}`
//!    resolved against the project's enabled addon set. Fail-closed: an
//!    unknown addon name or an unbalanced conditional is an error that aborts
//!    the whole sync.
//! 2. Variable substitution — the four `{{CHANGE_ID}}`, `{{VERIFY_COMMAND}}`,
//!    `{{FEATURES_PATH}}`, `{{GIVN_VERSION}}` slots.
//!
//! Directives are recognized on their own line; the directive lines themselves
//! are removed from the output, so surrounding Markdown/fenced content stays
//! intact.

use anyhow::{Result, bail};

/// Known addon ids accepted in `{{#if addons.<name>}}` conditions.
pub const KNOWN_ADDON_KEYS: &[&str] = &[
    "arc42",
    "domain_modeling",
    "dev_principles",
    "grill_me",
    "grill_with_docs",
    "absolute_mode",
    "readme",
    "coverage",
];

/// Context variables injected into skill templates.
pub struct SkillContext<'a> {
    pub change_id: &'a str,
    pub verify_command: &'a str,
    pub features_path: &'a str,
    pub givn_version: &'a str,
}

/// Render a SKILL.md.tmpl / command template by resolving conditional blocks
/// against `addons` and then substituting the context variables.
pub fn render_template(
    template: &str,
    ctx: &SkillContext<'_>,
    addons: &[(&str, bool)],
) -> Result<String> {
    let lines: Vec<&str> = template.lines().collect();
    let resolved = render_lines(&lines, addons)?;
    Ok(substitute_vars(&resolved, ctx))
}

/// Replace the four template variables. Plain string substitution, same as the
/// original renderer.
fn substitute_vars(template: &str, ctx: &SkillContext<'_>) -> String {
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
        "givn-steps",
        "givn-reverse-engineer",
        "givn-characterize",
        "givn-ideation",
    ]
}

/// Recursively render a slice of template lines, resolving conditional blocks.
fn render_lines(lines: &[&str], addons: &[(&str, bool)]) -> Result<String> {
    let mut out = String::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if let Some(key) = parse_open_condition(trimmed) {
            if !KNOWN_ADDON_KEYS.contains(&key) {
                bail!("unknown addon '{key}' in conditional directive {{#if addons.{key}}}");
            }
            let (close_idx, else_idx) = find_block_end(lines, i)?;
            let enabled = addon_enabled(addons, key);
            let (branch_start, branch_end) = if enabled {
                (i + 1, else_idx.unwrap_or(close_idx))
            } else {
                let start = else_idx.map_or(close_idx, |e| e + 1);
                (start, close_idx)
            };
            let inner = render_lines(&lines[branch_start..branch_end], addons)?;
            out.push_str(&inner);
            i = close_idx + 1;
        } else if trimmed == "{{else}}" || trimmed == "{{/if}}" {
            bail!("stray '{}' outside a conditional block", trimmed);
        } else {
            out.push_str(line);
            out.push('\n');
            i += 1;
        }
    }
    Ok(out)
}

fn addon_enabled(addons: &[(&str, bool)], key: &str) -> bool {
    addons
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, enabled)| *enabled)
        .unwrap_or(false)
}

/// Parse `{{#if addons.<name>}}` (trimmed) into the addon key.
fn parse_open_condition(trimmed: &str) -> Option<&str> {
    let inner = trimmed.strip_prefix("{{#if ")?;
    let inner = inner.strip_suffix("}}")?;
    let key = inner.trim();
    key.strip_prefix("addons.")
}

/// Find the matching `{{/if}}` for the `{{#if …}}` at `open_idx`, tracking
/// nesting depth, and locate the `{{else}}` at depth 1 if present.
/// Returns `(close_idx, else_idx)`.
fn find_block_end(lines: &[&str], open_idx: usize) -> Result<(usize, Option<usize>)> {
    let mut depth = 1usize;
    let mut else_idx: Option<usize> = None;
    for (offset, raw) in lines.iter().enumerate().skip(open_idx + 1) {
        let trimmed = raw.trim();
        if trimmed.starts_with("{{#if ") {
            depth += 1;
        } else if trimmed == "{{/if}}" {
            depth -= 1;
            if depth == 0 {
                return Ok((offset, else_idx));
            }
        } else if depth == 1 && trimmed == "{{else}}" && else_idx.is_none() {
            else_idx = Some(offset);
        }
    }
    bail!("unterminated conditional block (no matching {{/if}}) in template");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> SkillContext<'static> {
        SkillContext {
            change_id: "demo",
            verify_command: "cargo test",
            features_path: "givn/specs",
            givn_version: "0.0.0",
        }
    }

    fn render(template: &str, addons: &[(&str, bool)]) -> Result<String> {
        render_template(template, &ctx(), addons)
    }

    #[test]
    fn keeps_block_when_addon_enabled() {
        let out = render(
            "a\n{{#if addons.arc42}}\non\n{{/if}}\nz\n",
            &[("arc42", true)],
        )
        .unwrap();
        assert_eq!(out, "a\non\nz\n");
    }

    #[test]
    fn drops_block_when_addon_disabled() {
        let out = render(
            "a\n{{#if addons.arc42}}\noff\n{{/if}}\nz\n",
            &[("arc42", false)],
        )
        .unwrap();
        assert_eq!(out, "a\nz\n");
    }

    #[test]
    fn else_branch_switches_on_disabled() {
        let out = render(
            "{{#if addons.domain_modeling}}\nyes\n{{else}}\nno\n{{/if}}\n",
            &[("domain_modeling", false)],
        )
        .unwrap();
        assert_eq!(out, "no\n");
    }

    #[test]
    fn else_branch_kept_when_enabled() {
        let out = render(
            "{{#if addons.domain_modeling}}\nyes\n{{else}}\nno\n{{/if}}\n",
            &[("domain_modeling", true)],
        )
        .unwrap();
        assert_eq!(out, "yes\n");
    }

    #[test]
    fn nested_blocks_require_both() {
        let template =
            "{{#if addons.arc42}}\n{{#if addons.domain_modeling}}\nboth\n{{/if}}\n{{/if}}\n";
        assert_eq!(
            render(template, &[("arc42", true), ("domain_modeling", true)]).unwrap(),
            "both\n"
        );
        // arc42 on, domain_modeling off → whole block dropped
        let out = render(template, &[("arc42", true), ("domain_modeling", false)]).unwrap();
        assert!(!out.contains("both"), "block should be dropped: {out:?}");
        // arc42 off → whole block dropped
        let out = render(template, &[("arc42", false), ("domain_modeling", true)]).unwrap();
        assert!(!out.contains("both"), "block should be dropped: {out:?}");
    }

    #[test]
    fn unknown_addon_is_error() {
        let err = render("{{#if addons.nope}}x{{/if}}\n", &[]);
        assert!(err.is_err());
        let msg = format!("{err:?}");
        assert!(msg.contains("unknown addon"), "{msg}");
    }

    #[test]
    fn unterminated_if_is_error() {
        let err = render("{{#if addons.arc42}}\nx\n", &[("arc42", true)]);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("unterminated"));
    }

    #[test]
    fn stray_closing_is_error() {
        let err = render("x\n{{/if}}\n", &[]);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("stray"));
    }

    #[test]
    fn variables_still_substituted() {
        let out = render(
            "{{CHANGE_ID}} {{VERIFY_COMMAND}} {{FEATURES_PATH}} {{GIVN_VERSION}}\n",
            &[],
        )
        .unwrap();
        assert_eq!(out, "demo cargo test givn/specs 0.0.0\n");
    }

    #[test]
    fn directives_vanish_inside_fence() {
        let template = "```\n{{#if addons.arc42}}\nkept\n{{/if}}\n```\n";
        let out = render(template, &[("arc42", true)]).unwrap();
        assert_eq!(out, "```\nkept\n```\n");
    }
}
