//! Where a scenario writes its terminal transcript.
//!
//! One convention for every terminal-evidence step. A run from a delta feature
//! (`GIVN_FEATURES=<repo>/givn/changes/<id>/specs`) writes into the change's
//! `evidence/visual/<scenario-slug>/`, the path the visual gate reads. A run
//! from the permanent corpus (`GIVN_FEATURES=<repo>/givn/specs`, which is what
//! the archive's own verification passes) writes into the gitignored
//! `target/terminal-evidence/<scenario-slug>/`: the change directory has moved
//! to `givn/archive/`, so recreating it would leave untracked work that blocks
//! the next archive.
//!
//! The project runner canonicalizes `GIVN_FEATURES` (the package's working
//! directory is `e2e/`), so the marker is matched anywhere in the path rather
//! than at its start.

use std::path::PathBuf;

use super::binary;
use super::timing::scenario_slug;

/// The directory holding one scenario's `transcript.txt`.
pub fn transcript_dir(scenario: &str) -> PathBuf {
    let slug = scenario_slug(scenario);
    let features = std::env::var("GIVN_FEATURES").unwrap_or_default();
    let change = features
        .rfind("givn/changes/")
        .map(|at| &features[at + "givn/changes/".len()..])
        .and_then(|rest| rest.split('/').next())
        .filter(|id| !id.is_empty());
    let root = binary::product_root();
    match change {
        Some(change) => root
            .join("givn/changes")
            .join(change)
            .join("evidence/visual")
            .join(slug),
        None => root.join("target/terminal-evidence").join(slug),
    }
}
