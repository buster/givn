//! `givn ise verify`: the re-check of one prepared trial.
//!
//! The add-on precondition runs before the record is read. The record trust
//! anchor, the retained identities, and the current `e2e` identities land in
//! the verify batch; this module owns the command's one entry point.

use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::config::overlay::load_effective_at;

use super::require_addons;

/// Re-check a prepared trial's frozen retained and `e2e` inputs.
pub fn verify(project_root: &Path, _trial: &Path, _at: &Path) -> Result<()> {
    let manifest = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&manifest)?;
    bail!("the trial re-check is not implemented yet")
}
