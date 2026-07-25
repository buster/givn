/// The `absolute_mode` addon: manages the `absolute-mode` fenced block in
/// AGENTS.md (see `crate::cli::agents_block` for the generic block mechanism).
use anyhow::Result;

const BLOCK_NAME: &str = "absolute-mode";

/// Ensure the absolute-mode block is present in AGENTS.md with current
/// policy content. Creates AGENTS.md if it does not exist. Idempotent.
pub fn apply(project_root: &std::path::Path) -> Result<()> {
    let policy = crate::config::embedded::Assets::get_str("addons/absolute-mode/policy.md")
        .ok_or_else(|| anyhow::anyhow!("embedded addons/absolute-mode/policy.md not found"))?;
    let agents_path = project_root.join("AGENTS.md");
    crate::cli::agents_block::apply(&agents_path, BLOCK_NAME, &policy)
}

/// Remove the absolute-mode block from AGENTS.md. Leaves all other content,
/// including any other named block, untouched.
pub fn remove(project_root: &std::path::Path) -> Result<()> {
    let agents_path = project_root.join("AGENTS.md");
    crate::cli::agents_block::remove(&agents_path, BLOCK_NAME)
}
