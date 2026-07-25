/// The always-on `givn-project` block in AGENTS.md: a short pointer telling
/// any agent working in this project that it uses givn, and to run
/// `givn instructions` for context-aware next steps. Unlike the
/// `absolute_mode` addon, this is unconditional — always applied on
/// `givn init` and kept current on every `givn skills sync`, independent of
/// any addon flag.
use anyhow::Result;

const BLOCK_NAME: &str = "givn-project";

/// Ensure the givn-project hint block is present in AGENTS.md, current with
/// the embedded hint text. Creates AGENTS.md if it does not exist. Idempotent.
pub fn apply(project_root: &std::path::Path) -> Result<()> {
    let hint = crate::config::embedded::Assets::get_str("agents-project-hint.md")
        .ok_or_else(|| anyhow::anyhow!("embedded agents-project-hint.md not found"))?;
    let agents_path = project_root.join("AGENTS.md");
    crate::cli::agents_block::apply(&agents_path, BLOCK_NAME, &hint)
}
