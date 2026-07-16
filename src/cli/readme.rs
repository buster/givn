use anyhow::{Context, Result};

/// Scaffold an initial README.md from the embedded template.
///
/// Idempotent — does nothing if README.md already exists.
/// Returns the number of files written (0 or 1).
pub fn scaffold_readme() -> Result<usize> {
    let project_root = std::env::current_dir()?;
    let dest = project_root.join("README.md");
    if dest.exists() {
        return Ok(0);
    }

    let template = crate::config::cascade::resolve_template("readme", "README.md.tmpl")
        .context("failed to resolve readme template from embedded assets")?;

    std::fs::write(&dest, template)?;
    Ok(1)
}
