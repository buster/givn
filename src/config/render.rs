use anyhow::Result;

/// Render the fully-commented, versioned `givn/config.yaml` scaffold.
///
/// The template lives at `assets/config.yaml.tmpl` in the embedded assets.
/// The `{{GIVN_VERSION}}` placeholder is replaced with the binary version.
pub fn config_yaml(version: &str) -> Result<String> {
    let template = crate::config::embedded::Assets::get_str("config.yaml.tmpl")
        .ok_or_else(|| anyhow::anyhow!("embedded config.yaml.tmpl not found"))?;
    Ok(template.replace("{{GIVN_VERSION}}", version))
}
