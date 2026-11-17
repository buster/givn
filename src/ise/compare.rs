//! `givn ise compare`: the spec copy against the manifest.
//!
//! The add-on precondition runs before the manifest is read. The recorded
//! manifest is the authority: its Specification checksums are compared with
//! the copy's bytes, the copy's declaration bounds the added-file refusal,
//! and the recorded kept paths must still be present. A recorded Specification
//! path is also compared with the checkout, so an edit made after the copy was
//! written is named once, by the subject that owns the path.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::config::declaration::{DECLARATION_PATH, SpecificationDeclaration};
use crate::config::overlay::load_effective_at;
use crate::e2e_boundary::{Glob, glob_matches_path_or_parent};
use crate::scm::EntryKind;

use super::manifest;
use super::require_addons;

/// The sentence a checkout Specification edit prints after its path line.
const CHECKOUT_CHANGED: &str = "Specification changed after the copy was written";
/// The refusal names at most this many added paths.
const ADDED_REPORT_LIMIT: usize = 20;

/// `givn ise compare`'s parsed request.
#[derive(Debug, Clone)]
pub struct CompareRequest {
    /// The manifest the preparation wrote.
    pub manifest: PathBuf,
    /// The spec copy the manifest describes.
    pub copy: PathBuf,
}

/// One non-zero difference kind, in the fixed report order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DifferenceKind {
    Changed,
    Missing,
    Added,
}

/// One path's difference, with the sentence a checkout edit adds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    pub kind: DifferenceKind,
    pub path: String,
    pub note: Option<&'static str>,
}

/// One comparison's two subjects; an empty vector prints `match`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompareReport {
    pub specification: Vec<Difference>,
    pub end_to_end: Vec<Difference>,
}

impl CompareReport {
    /// The comparison exits zero only when both subjects match.
    pub fn is_clean(&self) -> bool {
        self.specification.is_empty() && self.end_to_end.is_empty()
    }
}

/// The manifest fields the comparison consumes. The strict shape is a later
/// batch; this reader defaults every field so a missing or foreign field is
/// not refused here.
#[derive(Debug, Default, Deserialize)]
struct Recorded {
    #[serde(default)]
    specification_files: Vec<String>,
    #[serde(default)]
    source_kept_files: Vec<String>,
    #[serde(default)]
    checksums: Checksums,
}

#[derive(Debug, Default, Deserialize)]
struct Checksums {
    #[serde(default)]
    specification: BTreeMap<String, String>,
    #[serde(default)]
    scenarios: BTreeMap<String, String>,
}

/// Compare a prepared spec copy with the manifest that recorded it.
pub fn compare(project_root: &Path, request: &CompareRequest) -> Result<CompareReport> {
    let effective = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&effective)?;

    let recorded = read_recorded(&request.manifest)?;
    let copy = request
        .copy
        .canonicalize()
        .with_context(|| format!("resolve the spec copy '{}'", request.copy.display()))?;
    let declaration = copy_declaration(&copy)?;
    require_no_added_specification_paths(&copy, &recorded, &declaration)?;
    require_kept_paths_present(&copy, &recorded)?;

    let mut specification: BTreeMap<String, Difference> = BTreeMap::new();
    for (path, checksum) in &recorded.checksums.specification {
        match local_checksum(&copy, path)? {
            None => observe(&mut specification, path, DifferenceKind::Missing, None),
            Some(observed) if observed != *checksum => {
                observe(&mut specification, path, DifferenceKind::Changed, None);
            }
            Some(_) => {}
        }
    }

    let backend = crate::scm::select(project_root)?;
    let content: BTreeSet<String> = backend
        .content_paths()
        .with_context(|| format!("read the project tree at '{}'", project_root.display()))?
        .into_iter()
        .map(|entry| entry.path.to_string_lossy().replace('\\', "/"))
        .collect();
    for (path, checksum) in &recorded.checksums.specification {
        if recorded.checksums.scenarios.contains_key(path) {
            continue;
        }
        if !content.contains(path) {
            observe(&mut specification, path, DifferenceKind::Missing, None);
            continue;
        }
        match local_checksum(project_root, path)? {
            None => observe(&mut specification, path, DifferenceKind::Missing, None),
            Some(observed) if observed != *checksum => observe(
                &mut specification,
                path,
                DifferenceKind::Changed,
                Some(CHECKOUT_CHANGED),
            ),
            Some(_) => {}
        }
    }

    Ok(CompareReport {
        specification: ordered(specification),
        end_to_end: Vec::new(),
    })
}

/// The recorded manifest fields, read permissively for this batch.
fn read_recorded(path: &Path) -> Result<Recorded> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read the manifest '{}'", path.display()))?;
    serde_yaml_ng::from_str(&text)
        .with_context(|| format!("read the manifest '{}'", path.display()))
}

/// The spec copy's declaration: the added-file refusal is read with the
/// declaration the copy carries, not the checkout's.
fn copy_declaration(copy: &Path) -> Result<SpecificationDeclaration> {
    let path = copy.join(DECLARATION_PATH);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("read the spec copy's declaration '{}'", path.display()))?;
    SpecificationDeclaration::from_content(&content).map_err(|error| anyhow::anyhow!("{error}"))
}

/// Refuse a copy that holds a file under any declared Specification role the
/// manifest does not record. The `e2e` role bounds the refusal even though the
/// copy never carries an `e2e` match.
fn require_no_added_specification_paths(
    copy: &Path,
    recorded: &Recorded,
    declaration: &SpecificationDeclaration,
) -> Result<()> {
    let globs: Vec<Glob> = declaration
        .roles()
        .into_iter()
        .flat_map(|(_, patterns)| patterns.iter())
        .map(|pattern| Glob::compile(pattern))
        .collect();
    let listed: BTreeSet<&str> = recorded
        .specification_files
        .iter()
        .map(String::as_str)
        .collect();
    let added: Vec<String> = copy_paths(copy)?
        .into_iter()
        .filter(|path| !listed.contains(path.as_str()))
        .filter(|path| {
            globs
                .iter()
                .any(|glob| glob_matches_path_or_parent(glob, path))
        })
        .collect();
    if added.is_empty() {
        return Ok(());
    }
    let shown: Vec<&str> = added
        .iter()
        .take(ADDED_REPORT_LIMIT)
        .map(String::as_str)
        .collect();
    let remainder = added.len() - shown.len();
    let more = if remainder == 0 {
        String::new()
    } else {
        format!(" (and {remainder} more)")
    };
    let noun = if added.len() == 1 { "file" } else { "files" };
    bail!(
        "the spec copy adds {} {noun} under a declared Specification path; Specification files are not the product: {}{more}",
        added.len(),
        shown.join(", ")
    )
}

/// Refuse a copy that lost a recorded kept path. Kept content is not verified,
/// so an edit is allowed and only a deletion refuses.
fn require_kept_paths_present(copy: &Path, recorded: &Recorded) -> Result<()> {
    for path in &recorded.source_kept_files {
        if std::fs::symlink_metadata(copy.join(path)).is_err() {
            bail!(
                "the spec copy lost the kept Source code '{path}'; the agent may edit kept Source code but not delete a file the copy was written with"
            );
        }
    }
    Ok(())
}

/// One path's observation, merged so the copy and the checkout contribute one
/// difference: a missing side wins, else a changed side, and the checkout
/// sentence prints only for a changed checkout.
fn observe(
    differences: &mut BTreeMap<String, Difference>,
    path: &str,
    kind: DifferenceKind,
    note: Option<&'static str>,
) {
    match differences.get_mut(path) {
        None => {
            differences.insert(
                path.to_string(),
                Difference {
                    kind,
                    path: path.to_string(),
                    note,
                },
            );
        }
        Some(existing) => {
            let merged =
                if existing.kind == DifferenceKind::Missing || kind == DifferenceKind::Missing {
                    DifferenceKind::Missing
                } else {
                    DifferenceKind::Changed
                };
            existing.kind = merged;
            if merged == DifferenceKind::Changed && note.is_some() {
                existing.note = note;
            }
        }
    }
}

/// The report order: changed, missing, added, and sorted paths within a kind.
fn ordered(differences: BTreeMap<String, Difference>) -> Vec<Difference> {
    let mut differences: Vec<Difference> = differences.into_values().collect();
    differences.sort_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then_with(|| left.path.cmp(&right.path))
    });
    differences
}

/// Every non-directory path under `copy`, relative to it and sorted, without
/// following symlinks.
fn copy_paths(copy: &Path) -> Result<Vec<String>> {
    let mut paths: Vec<String> = Vec::new();
    let mut directories = vec![copy.to_path_buf()];
    while let Some(directory) = directories.pop() {
        let entries = std::fs::read_dir(&directory)
            .with_context(|| format!("read the spec copy '{}'", directory.display()))?;
        for entry in entries {
            let entry =
                entry.with_context(|| format!("read an entry of '{}'", directory.display()))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .with_context(|| format!("read the kind of '{}'", path.display()))?;
            if file_type.is_dir() {
                directories.push(path);
                continue;
            }
            let relative = path
                .strip_prefix(copy)
                .expect("every walked path is inside the spec copy")
                .to_string_lossy()
                .replace('\\', "/");
            paths.push(relative);
        }
    }
    paths.sort();
    Ok(paths)
}

/// One project-relative path's checksum under `root`, read by entry kind,
/// executable bit, and bytes or symlink target.
fn local_checksum(root: &Path, path: &str) -> Result<Option<String>> {
    let target = root.join(path);
    let metadata = match std::fs::symlink_metadata(&target) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("read '{}'", target.display()));
        }
    };
    let file_type = metadata.file_type();
    if file_type.is_symlink() {
        let link = std::fs::read_link(&target)
            .with_context(|| format!("read the symlink target of '{}'", target.display()))?;
        return Ok(Some(manifest::path_checksum(
            path,
            EntryKind::Symlink,
            false,
            link.to_string_lossy().as_bytes(),
        )));
    }
    if !file_type.is_file() {
        return Ok(None);
    }
    let bytes = std::fs::read(&target).with_context(|| format!("read '{}'", target.display()))?;
    Ok(Some(manifest::path_checksum(
        path,
        EntryKind::Regular,
        is_executable(&metadata),
        &bytes,
    )))
}

#[cfg(unix)]
fn is_executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_metadata: &std::fs::Metadata) -> bool {
    false
}
