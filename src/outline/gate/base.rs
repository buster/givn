//! Change-base resolution.
//!
//! The base is the parent of the first reachable commit that touched
//! `givn/changes/<change-id>/`. There is no fallback to `HEAD`, the working
//! tree, or a merge-base.

use super::{ChangeBase, GateFailure, Side};
use crate::scm::Backend;
use anyhow::Result;

pub enum Resolution {
    Resolved(ChangeBase),
    NoCommit,
    Shallow,
}

pub fn resolve(backend: &Backend, change_id: &str) -> Result<Resolution> {
    let change_path = format!("givn/changes/{change_id}/");
    let first_touch = match backend.first_touching_revision(&change_path) {
        Ok(Some(revision)) if !revision.trim().is_empty() => revision,
        Ok(_) => return Ok(Resolution::NoCommit),
        Err(error)
            if error
                .to_string()
                .starts_with("cannot resolve change base: repository is shallow") =>
        {
            return Ok(Resolution::Shallow);
        }
        Err(error) => return Err(error),
    };
    let parent = backend.parent_revision(&first_touch)?;
    Ok(Resolution::Resolved(ChangeBase {
        first_touch,
        parent,
    }))
}

/// Map a revision-tree path error to the gate's named failure.
pub fn path_failure(error: &anyhow::Error, side: Side) -> GateFailure {
    let message = error.to_string();
    if let Some(path) = message.strip_prefix("revision path is not valid UTF-8: ") {
        return GateFailure::NonUtf8Path(path.to_string());
    }
    if let Some(path) = message.strip_prefix("revision path contains a line break: ") {
        return GateFailure::NewlinePath(path.to_string());
    }
    GateFailure::AnalysisFailed {
        side,
        path: "revision tree".to_string(),
        detail: message,
    }
}
