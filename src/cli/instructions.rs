use anyhow::{Result, bail};

/// Show instructions for an artifact (text or --json).
///
/// Resolves the instruction via the cascade (project override wins over embedded default).
/// The effective manifest (base + overlay) is used to validate that the artifact exists.
pub fn run(artifact: &str, _change: Option<&str>, json: bool) -> Result<()> {
    // Load effective manifest to validate the artifact id exists.
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.artifacts.iter().any(|a| a.id == artifact) {
        let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        bail!(
            "unknown artifact '{}'. Known artifacts: {}",
            artifact,
            known.join(", ")
        );
    }

    let content = crate::config::cascade::resolve_instruction(artifact)?;

    if json {
        let out = serde_json::json!({
            "artifact": artifact,
            "instruction": content,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{content}");
    }

    Ok(())
}
