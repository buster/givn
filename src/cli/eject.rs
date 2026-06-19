use anyhow::{bail, Result};
use crate::config::embedded::Assets;

/// Write an embedded default facet into the project directory for editing.
///
/// Eject targets:
///   `<artifact-id>:template`     → writes `givn/artifacts/<id>/template.<ext>`
///   `<artifact-id>:instruction`  → writes `givn/artifacts/<id>/instructions.md`
///   `skill:<skill-name>`         → writes `givn/skills/<name>/SKILL.md.tmpl`
///   `manifest`                   → writes `givn/artifacts/manifest.yaml` (the embedded manifest)
pub fn run(target: &str, force: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if target == "manifest" {
        eject_manifest(&project_root, force)?;
        return Ok(());
    }

    if let Some(skill_name) = target.strip_prefix("skill:") {
        eject_skill(&project_root, skill_name, force)?;
        return Ok(());
    }

    // Split "artifact_id:facet".
    if let Some((artifact_id, facet)) = target.split_once(':') {
        match facet {
            "template" => eject_template(&project_root, artifact_id, force)?,
            "instruction" => eject_instruction(&project_root, artifact_id, force)?,
            other => bail!(
                "unknown facet '{}'. Use 'template' or 'instruction'. \
                 Example: `givn eject design:template`",
                other
            ),
        }
        return Ok(());
    }

    bail!(
        "invalid eject target '{target}'. Examples:\n\
         \tgivn eject design:template\n\
         \tgivn eject review:instruction\n\
         \tgivn eject skill:givn-propose\n\
         \tgivn eject manifest"
    )
}

fn eject_template(project_root: &std::path::Path, artifact_id: &str, force: bool) -> Result<()> {
    // Determine the template filename for this artifact.
    let manifest = crate::config::manifest::Manifest::load_embedded()?;
    let artifact = manifest.artifacts.iter().find(|a| a.id == artifact_id).ok_or_else(|| {
        anyhow::anyhow!("no artifact '{artifact_id}' in the embedded manifest")
    })?;

    let template_name = artifact.template.as_deref().unwrap_or(artifact_id);

    // Try to find the embedded template.
    let embedded_key = format!("templates/{template_name}");
    let content = Assets::get_str(&embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no embedded template for artifact '{artifact_id}' (tried '{embedded_key}')"
        )
    })?;

    let dest_dir = project_root.join("givn/artifacts").join(artifact_id);
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join(template_name);

    write_file(&dest, &content, force)?;
    println!("ejected  {}", dest.display());
    println!("Edit this file to customise the template for '{artifact_id}'.");
    Ok(())
}

fn eject_instruction(project_root: &std::path::Path, artifact_id: &str, force: bool) -> Result<()> {
    let embedded_key = format!("instructions/{artifact_id}.md");
    let content = Assets::get_str(&embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no embedded instruction for artifact '{artifact_id}' (tried '{embedded_key}')"
        )
    })?;

    let dest_dir = project_root.join("givn/artifacts").join(artifact_id);
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join("instructions.md");

    write_file(&dest, &content, force)?;
    println!("ejected  {}", dest.display());
    println!("Edit this file to customise the instruction for '{artifact_id}'.");
    Ok(())
}

fn eject_skill(project_root: &std::path::Path, skill_name: &str, force: bool) -> Result<()> {
    let embedded_key = format!("skills/{skill_name}/SKILL.md.tmpl");
    let content = Assets::get_str(&embedded_key).ok_or_else(|| {
        anyhow::anyhow!(
            "no embedded skill template for '{skill_name}' (tried '{embedded_key}')"
        )
    })?;

    let dest_dir = project_root.join("givn/skills").join(skill_name);
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join("SKILL.md.tmpl");

    write_file(&dest, &content, force)?;
    println!("ejected  {}", dest.display());
    println!("Edit this file to customise the '{skill_name}' skill template.");
    println!("Run `givn skills sync` to regenerate.");
    Ok(())
}

fn eject_manifest(project_root: &std::path::Path, force: bool) -> Result<()> {
    let content = Assets::get_str("default-manifest.yaml")
        .ok_or_else(|| anyhow::anyhow!("embedded default-manifest.yaml not found"))?;

    let dest = project_root.join("givn/manifest.yaml");
    write_file(&dest, &content, force)?;
    println!("ejected  {}", dest.display());
    println!("Note: givn/manifest.yaml is for reference only. Use givn/config.yaml overlay to patch it.");
    Ok(())
}

fn write_file(path: &std::path::Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        bail!(
            "file already exists: {} (use --force to overwrite)",
            path.display()
        );
    }
    std::fs::write(path, content)?;
    Ok(())
}
