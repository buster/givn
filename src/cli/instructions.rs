use anyhow::Result;

/// Show instructions for an artifact (text or --json).
pub fn run(artifact: &str, _change: Option<&str>, json: bool) -> Result<()> {
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
