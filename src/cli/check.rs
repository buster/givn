use anyhow::Result;

/// Run an artifact's post hooks (e.g. verify.command).
pub fn run(artifact: Option<&str>, _change: Option<&str>) -> Result<()> {
    println!("givn check: not implemented yet");
    let _ = artifact;
    Ok(())
}
