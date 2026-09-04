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
///
/// Resolution order:
///   1. Project override: `givn/artifacts/<id>/<filename>`
///   2. Embedded default (core): `templates/<filename>`
///   3. Embedded addon: `addons/<id>/template.md`
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

    // 2. Embedded core template.
    let embedded_key = format!("templates/{filename}");
    if let Some(content) = crate::config::embedded::Assets::get_str(&embedded_key) {
        return Ok(content);
    }

    // 3. Embedded addon template (addons/<artifact_id>/<filename>).
    let addon_key = format!("addons/{artifact_id}/{filename}");
    crate::config::embedded::Assets::get_str(&addon_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no template found for artifact '{}' (tried project override '{}', \
             embedded 'templates/{filename}', and addon '{addon_key}')",
            artifact_id,
            project_path.display(),
        )
    })
}

/// Resolve the instruction for an artifact.
/// Returns the instruction content as a string.
///
/// Resolution order:
///   1. Project override: `givn/artifacts/<id>/instructions.md`
///   2. Embedded default (core): `instructions/<id>.md`
///   3. Embedded addon: `addons/<id>/instructions.md`
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

    // 2. Embedded core instruction.
    let embedded_key = format!("instructions/{artifact_id}.md");
    if let Some(content) = crate::config::embedded::Assets::get_str(&embedded_key) {
        return Ok(content);
    }

    // 3. Embedded addon instruction (addons/<artifact_id>/instructions.md).
    let addon_key = format!("addons/{artifact_id}/instructions.md");
    crate::config::embedded::Assets::get_str(&addon_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no instruction found for artifact '{}' (tried project override '{}', \
             embedded 'instructions/{artifact_id}.md', and addon '{addon_key}')",
            artifact_id,
            project_path.display(),
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

#[cfg(test)]
mod tests {
    fn embedded(asset: &str) -> String {
        crate::config::embedded::Assets::get_str(asset)
            .unwrap_or_else(|| panic!("embedded asset missing: {asset}"))
    }

    #[test]
    fn specs_instruction_requires_regular_scenarios_to_differ_from_e2e() {
        let text = embedded("instructions/specs.md");
        assert!(
            text.contains("Non-e2e scenarios must differ from their action's `@e2e` scenario.")
        );
        assert!(text.contains("the layer is a driver, not an invariant"));
    }
}
