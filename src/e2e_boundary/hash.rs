//! The shared content-entry hash rule.
//!
//! The tested-content digest and the declared `e2e` identity hash the same
//! shape for one path: its project-relative name, its executable marker, and
//! its bytes (or gitlink id / symlink target). The caller owns ordering.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Hash one content entry into `hasher`. A gitlink entry hashes its recorded
/// object id; a symlink hashes its target; a directory hashes a marker; every
/// other entry hashes its bytes.
pub(crate) fn hash_entry(
    hasher: &mut Sha256,
    project_root: &Path,
    relative: &str,
    gitlink: Option<&str>,
) -> Result<()> {
    hasher.update(relative.as_bytes());
    hasher.update(b"\0");
    let absolute = project_root.join(relative);
    let metadata = std::fs::symlink_metadata(&absolute)
        .with_context(|| format!("cannot read tested content path '{relative}'"))?;
    #[cfg(unix)]
    let executable = {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    };
    #[cfg(not(unix))]
    let executable = false;
    hasher.update(if executable { b"1" } else { b"0" });
    hasher.update(b"\0");
    if let Some(object) = gitlink {
        hasher.update(b"gitlink\0");
        hasher.update(object.as_bytes());
    } else if metadata.file_type().is_symlink() {
        let target = std::fs::read_link(&absolute)
            .with_context(|| format!("cannot read symlink target '{relative}'"))?;
        hasher.update(target.to_string_lossy().as_bytes());
    } else if metadata.is_dir() {
        hasher.update(b"directory\0");
    } else {
        let bytes = std::fs::read(&absolute)
            .with_context(|| format!("cannot read tested content path '{relative}'"))?;
        hasher.update(&bytes);
    }
    hasher.update(b"\0");
    Ok(())
}
