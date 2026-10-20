//! `givn readability report`: the opt-in, advisory code-structure report.
//!
//! The command is always registered and gated at runtime so the disabled path
//! can name the reason. Findings never change the exit code.

use anyhow::{Result, bail};
use std::path::PathBuf;

/// Run the readability report. The `readability` addon must be enabled.
pub fn run_report(json: bool, out: Option<PathBuf>, paths: Vec<PathBuf>) -> Result<()> {
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.addons.readability {
        bail!(
            "readability experiment is disabled; enable it with givn addons enable readability"
        );
    }

    // The analysis pipeline is implemented by the following scenarios.
    let _ = (json, out, paths);
    bail!("readability report is not implemented yet")
}
