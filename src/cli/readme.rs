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

    let project_name = detect_project_name(&project_root);
    let rendered = template.replace("{{PROJECT_NAME}}", &project_name);

    std::fs::write(&dest, rendered)?;
    Ok(1)
}

/// Detect the project name from package metadata or fall back to the
/// directory name.
fn detect_project_name(project_root: &std::path::Path) -> String {
    // 1. Cargo.toml → package.name
    if let Ok(content) = std::fs::read_to_string(project_root.join("Cargo.toml")) {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = trimmed
                .strip_prefix("name = \"")
                .and_then(|s| s.strip_suffix('"'))
            {
                return name.to_string();
            }
        }
    }
    // 2. package.json → "name"
    if let Ok(content) = std::fs::read_to_string(project_root.join("package.json"))
        && let Ok(json) = serde_json::from_str::<serde_json::Value>(&content)
        && let Some(name) = json.get("name").and_then(|v| v.as_str())
    {
        return name.to_string();
    }
    // 3. Fallback: directory name
    project_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project")
        .to_string()
}
