use crate::config::embedded::Assets;
use anyhow::{Result, bail};

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
    // Use the effective manifest so addon artifacts (e.g. arc42-docs) are ejectable.
    let manifest = crate::config::overlay::load_effective()?;
    let artifact = manifest
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no artifact '{artifact_id}' in the effective manifest. \
             If this is an addon artifact, enable it in givn/config.yaml first."
            )
        })?;

    let template_name = artifact.template.as_deref().unwrap_or(artifact_id);

    // Cascade: core templates/ then addons/<id>/template.md.
    let core_key = format!("templates/{template_name}");
    let addon_key = format!("addons/{artifact_id}/template.md");
    let content = Assets::get_str(&core_key)
        .or_else(|| Assets::get_str(&addon_key))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no embedded template for artifact '{artifact_id}' \
                 (tried '{core_key}' and '{addon_key}')"
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
    // Validate against the effective manifest before looking up assets.
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.artifacts.iter().any(|a| a.id == artifact_id) {
        let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        anyhow::bail!(
            "unknown artifact '{}'. Known artifacts: {}",
            artifact_id,
            known.join(", ")
        );
    }

    // Cascade: core instructions/ then addons/<id>/instructions.md.
    let core_key = format!("instructions/{artifact_id}.md");
    let addon_key = format!("addons/{artifact_id}/instructions.md");
    let content = Assets::get_str(&core_key)
        .or_else(|| Assets::get_str(&addon_key))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no embedded instruction for artifact '{artifact_id}' \
                 (tried '{core_key}' and '{addon_key}')"
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
        anyhow::anyhow!("no embedded skill template for '{skill_name}' (tried '{embedded_key}')")
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
    println!(
        "Note: givn/manifest.yaml is for reference only. Use givn/config.yaml overlay to patch it."
    );
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
