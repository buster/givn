use anyhow::{bail, Result};
use crate::config::overlay::load_effective;

/// Create a new change scaffold under `givn/changes/<id>/`.
///
/// If `skip` is non-empty, each named artifact id is validated against the
/// effective manifest, written to `.givn-skip` (one per line), and its
/// `generates` output is not scaffolded. `givn status` shows skipped artifacts
/// as ⊘ skipped (not pending), so `next_required` skips them.
pub fn run(id: &str, skip: &[String]) -> Result<()> {
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

    // Validate skip ids against the effective manifest (if any skip requested).
    if !skip.is_empty() {
        let manifest = load_effective()?;
        let known: std::collections::HashSet<&str> =
            manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        for s in skip {
            if !known.contains(s.as_str()) {
                let mut ids: Vec<&str> = known.iter().copied().collect();
                ids.sort();
                bail!(
                    "unknown artifact '{}' in --skip. Known artifacts: {}",
                    s,
                    ids.join(", ")
                );
            }
        }
    }

    std::fs::create_dir_all(&change_dir)?;

    // Write proposal stub from cascade, unless skipped.
    let skip_set: std::collections::HashSet<&str> =
        skip.iter().map(|s| s.as_str()).collect();

    if !skip_set.contains("proposal") {
        let proposal = crate::config::cascade::resolve_template("proposal", "proposal.md")?;
        let proposal_path = change_dir.join("proposal.md");
        std::fs::write(&proposal_path, proposal)?;
        println!("Created change '{id}'");
        println!("  {}", proposal_path.display());
        println!("\nNext: fill in proposal.md, then run:");
        println!("  givn instructions proposal --change {id}");
    } else {
        println!("Created change '{id}' (proposal skipped)");
    }

    // Write the skip file if any artifacts were skipped.
    if !skip.is_empty() {
        let skip_path = change_dir.join(crate::state::SKIP_FILE);
        let content = skip.join("\n") + "\n";
        std::fs::write(&skip_path, content)?;
        println!("  {} (skipped: {})", skip_path.display(), skip.join(", "));
    }

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
