/// CSS-style cascade: resolve a facet for an artifact.
///
/// Resolution order (later wins):
///   1. Embedded binary defaults (under `assets/`)
///   2. Project override files (under `givn/artifacts/<artifact_id>/`)
///
/// The smallest overridable unit is a single facet (template, instruction),
/// not the whole artifact bundle.
use anyhow::Result;

/// Resolve the template for an artifact.
/// Returns the file contents as a string.
pub fn resolve_template(artifact_id: &str, filename: &str) -> Result<String> {
    let project_root = std::env::current_dir()?;

    // 1. Project override wins.
    let project_path = project_root
        .join("givn/artifacts")
        .join(artifact_id)
        .join(filename);
    if project_path.exists() {
        return Ok(std::fs::read_to_string(&project_path)?);
    }

    // 2. Embedded default fallback.
    let embedded_key = format!("templates/{filename}");
    crate::config::embedded::Assets::get_str(&embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no template found for artifact '{}' (tried project override '{}' and embedded '{}')",
            artifact_id,
            project_path.display(),
            embedded_key
        )
    })
}

/// Resolve the instruction for an artifact.
/// Returns the instruction content as a string.
pub fn resolve_instruction(artifact_id: &str) -> Result<String> {
    let project_root = std::env::current_dir()?;

    // 1. Project override wins.
    let project_path = project_root
        .join("givn/artifacts")
        .join(artifact_id)
        .join("instructions.md");
    if project_path.exists() {
        return Ok(std::fs::read_to_string(&project_path)?);
    }

    // 2. Embedded default fallback.
    let embedded_key = format!("instructions/{artifact_id}.md");
    crate::config::embedded::Assets::get_str(&embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no instruction found for artifact '{}' (tried project override '{}' and embedded '{}')",
            artifact_id,
            project_path.display(),
            embedded_key
        )
    })
}

/// Resolve an arbitrary asset by its embedded key, with an optional project override.
/// `embedded_key` is relative to `assets/`.
/// `project_key` is relative to the project root.
pub fn resolve_asset(embedded_key: &str, project_key: &str) -> Result<String> {
    let project_root = std::env::current_dir()?;
    let project_path = project_root.join(project_key);

    if project_path.exists() {
        return Ok(std::fs::read_to_string(&project_path)?);
    }

    crate::config::embedded::Assets::get_str(embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "asset not found: tried '{}' and embedded '{}'",
            project_path.display(),
            embedded_key
        )
    })
}
