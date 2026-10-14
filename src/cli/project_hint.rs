/// The always-on `givn-project` block in AGENTS.md: a short pointer telling
/// any agent working in this project that it uses givn, and to run
/// `givn instructions` for context-aware next steps. Unlike the
/// `absolute_mode` addon, this is unconditional — always applied on
/// `givn init` and kept current on every `givn skills sync`, independent of
/// any addon flag.
use anyhow::{Context, Result};

const BLOCK_NAME: &str = "givn-project";

/// The markers delimiting the resident rules section of the tasks
/// instruction. That section is the single home of the task-checkoff and
/// commit-message obligations the project-start block carries.
const RESIDENT_RULES_BEGIN: &str = "<!-- givn:resident-rules:begin -->";
const RESIDENT_RULES_END: &str = "<!-- givn:resident-rules:end -->";

/// Extract the marked resident-rules section of the embedded tasks
/// instruction. A missing marker is a shipped-asset defect: the caller must
/// fail naming `instructions/tasks.md` instead of writing a block without the
/// obligations.
fn resident_rules(tasks: &str) -> Result<&str> {
    let begin = tasks.find(RESIDENT_RULES_BEGIN).with_context(|| {
        format!("instructions/tasks.md is missing the {RESIDENT_RULES_BEGIN} marker")
    })?;
    let end = tasks.find(RESIDENT_RULES_END).with_context(|| {
        format!("instructions/tasks.md is missing the {RESIDENT_RULES_END} marker")
    })?;
    let section = tasks[begin + RESIDENT_RULES_BEGIN.len()..end].trim();
    anyhow::ensure!(
        !section.is_empty(),
        "instructions/tasks.md carries an empty resident-rules section"
    );
    Ok(section)
}

/// Ensure the givn-project hint block is present in AGENTS.md, current with
/// the embedded hint text. Creates AGENTS.md if it does not exist. Idempotent.
/// The resident-rules section is rendered from the same embedded
/// `instructions/tasks.md` the request path serves.
pub fn apply(project_root: &std::path::Path) -> Result<()> {
    let hint = crate::config::embedded::Assets::get_str("agents-project-hint.md")
        .ok_or_else(|| anyhow::anyhow!("embedded agents-project-hint.md not found"))?;
    let tasks = crate::config::embedded::Assets::get_str("instructions/tasks.md")
        .ok_or_else(|| anyhow::anyhow!("embedded instructions/tasks.md not found"))?;
    let resident = resident_rules(&tasks)?;
    let hint = hint.replace("{{RESIDENT_RULES}}", resident);
    let agents_path = project_root.join("AGENTS.md");
    crate::cli::agents_block::apply(&agents_path, BLOCK_NAME, &hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resident_rules_extracts_the_marked_section() {
        let tasks = "before\n<!-- givn:resident-rules:begin -->\n- rule\n<!-- givn:resident-rules:end -->\nafter\n";
        assert_eq!(resident_rules(tasks).unwrap(), "- rule");
    }

    #[test]
    fn resident_rules_fails_closed_naming_the_asset() {
        let error = resident_rules("no markers here").unwrap_err().to_string();
        assert!(error.contains("instructions/tasks.md"), "{error}");
    }

    #[test]
    fn hint_renders_the_resident_rules() {
        let hint = "x\n{{RESIDENT_RULES}}\ny\n";
        let rendered = hint.replace(
            "{{RESIDENT_RULES}}",
            resident_rules(
                "<!-- givn:resident-rules:begin -->\n- rule\n<!-- givn:resident-rules:end -->\n",
            )
            .unwrap(),
        );
        assert_eq!(rendered, "x\n- rule\ny\n");
    }
}
