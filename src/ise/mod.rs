//! Clean-room trial preparation and verification.
//!
//! This module owns the admission checks that run before any trial content is
//! read or written. The CLI adapter in `crate::cli::ise` formats the outcome;
//! only the adapter knows about exit codes.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::declaration::{DECLARATION_PATH, SpecificationDeclaration};
use crate::config::manifest::Manifest;
use crate::config::overlay::load_effective_at;
use crate::scm::{Backend, RevisionEntry};

/// `givn ise replace`'s parsed request.
#[derive(Debug, Clone)]
pub struct ReplaceRequest {
    /// Revision to read; `None` and `HEAD` both resolve the current revision.
    pub at: Option<String>,
    /// Replace every tracked file under the declared source roots.
    pub full: bool,
    /// Replace a tracked file or a directory's tracked files (repeatable).
    pub paths: Vec<String>,
    /// Trial directory; required and outside the checkout.
    pub into: PathBuf,
    /// Optional trial-record path, written only when supplied.
    pub receipt: Option<PathBuf>,
    /// Perform every read and refusal without writing.
    pub dry_run: bool,
}

/// The state the partition consumes after admission.
pub struct Admission {
    pub backend: Backend,
    pub revision: String,
    pub declaration: SpecificationDeclaration,
    pub entries: Vec<RevisionEntry>,
}

/// Materialize a trial. The admission checks are implemented; the partition
/// and write land in the next behavior batch.
pub fn replace(project_root: &Path, request: &ReplaceRequest) -> Result<()> {
    let _admission = admit(project_root, request.at.as_deref())?;
    bail!("the clean-room trial write is not implemented yet")
}

/// Re-check a prepared trial. The add-on precondition is implemented; the
/// record trust anchor and the per-path checks land in the verify batch.
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

/// The ordered admission checks: add-ons, declaration, backend, dirty
/// checkout, revision. Every check precedes the first write and none writes.
fn admit(project_root: &Path, at: Option<&str>) -> Result<Admission> {
    let manifest = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&manifest)?;

    let declaration = SpecificationDeclaration::load(project_root)
        .map_err(|error| anyhow::anyhow!("{error}"))?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{DECLARATION_PATH} is missing; the clean-room trial requires a declared Specification"
            )
        })?;

    let backend = crate::scm::select(project_root)?;

    let dirty = backend.dirty_entries()?;
    if !dirty.is_empty() {
        let paths = dirty
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        bail!("the checkout is dirty at {paths}; commit or restore these paths first");
    }

    let revision = match at {
        None => backend.revision()?,
        Some("HEAD") => backend.revision()?,
        Some(value) => {
            if !backend.commit_exists(value)? {
                bail!("revision '{value}' does not exist in this repository");
            }
            value.to_string()
        }
    };
    let entries = backend
        .revision_files(&revision, "")
        .with_context(|| format!("read revision '{revision}'"))?;

    Ok(Admission {
        backend,
        revision,
        declaration,
        entries,
    })
}

/// `ise` and its `architecture_principles` prerequisite gate both commands.
fn require_addons(manifest: &Manifest) -> Result<()> {
    if !manifest.addons.ise {
        bail!("ise is not enabled; run `givn addons enable ise` first");
    }
    if !manifest.addons.architecture_principles {
        bail!(
            "architecture_principles is not enabled; run `givn addons enable architecture_principles` first"
        );
    }
    Ok(())
}
