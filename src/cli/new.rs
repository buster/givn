use anyhow::{bail, Result};

/// Create a new change scaffold under `givn/changes/<id>/`.
pub fn run(id: &str) -> Result<()> {
    validate_id(id)?;
    let project_root = std::env::current_dir()?;
    let change_dir = project_root.join("givn/changes").join(id);

    if change_dir.exists() {
        bail!("change '{}' already exists at {}", id, change_dir.display());
    }

    // Check givn/ is initialised.
    if !project_root.join("givn").exists() {
        bail!("givn/ not found. Run `givn init` first.");
    }

    std::fs::create_dir_all(&change_dir)?;

    // Write proposal stub from cascade.
    let proposal = crate::config::cascade::resolve_template("proposal", "proposal.md")?;
    let proposal_path = change_dir.join("proposal.md");
    std::fs::write(&proposal_path, proposal)?;

    println!("Created change '{id}'");
    println!("  {}", proposal_path.display());
    println!("\nNext: fill in proposal.md, then run:");
    println!("  givn instructions proposal --change {id}");

    Ok(())
}

fn validate_id(id: &str) -> Result<()> {
    if id.is_empty() {
        bail!("change id must not be empty");
    }
    if !id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        bail!("change id must be kebab-case (lowercase letters, digits, hyphens): '{id}'");
    }
    if id.starts_with('-') || id.ends_with('-') {
        bail!("change id must not start or end with a hyphen: '{id}'");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_id;

    #[test]
    fn valid_ids() {
        assert!(validate_id("add-user-auth").is_ok());
        assert!(validate_id("fix-123").is_ok());
        assert!(validate_id("feature").is_ok());
    }

    #[test]
    fn invalid_ids() {
        assert!(validate_id("").is_err());
        assert!(validate_id("-starts-with-hyphen").is_err());
        assert!(validate_id("ends-with-hyphen-").is_err());
        assert!(validate_id("has SPACES").is_err());
        assert!(validate_id("has_underscores").is_err());
        assert!(validate_id("HasUppercase").is_err());
    }
}
