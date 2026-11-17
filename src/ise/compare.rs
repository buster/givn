//! `givn ise compare`: the spec copy against the manifest.
//!
//! The add-on precondition runs before the manifest is read. The manifest is
//! read as one strict `ise-manifest/v1` document: the scope, the field set,
//! and every checksum and rollup must match, and the manifest must belong to
//! the given copy and lie outside it. The recorded manifest is then the
//! authority: its Specification checksums are compared with the copy's bytes,
//! the copy's declaration bounds the added-file refusal, and the recorded kept
//! paths must still be present. The recorded Specification and End-to-end
//! paths are also compared with the checkout's content paths, and a content
//! path the manifest does not record is added when the checkout's declaration
//! matches it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::declaration::{DECLARATION_PATH, Role, SpecificationDeclaration};
use crate::config::overlay::load_effective_at;
use crate::e2e_boundary::{Glob, e2e_file_set, glob_matches_path_or_parent};
use crate::gherkin::content::e2e_scenario_digest;
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

/// Compare a prepared spec copy with the manifest that recorded it.
pub fn compare(project_root: &Path, request: &CompareRequest) -> Result<CompareReport> {
    let effective = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&effective)?;

    let recorded = manifest::Manifest::read(&request.manifest)?;
    let copy = request
        .copy
        .canonicalize()
        .with_context(|| format!("resolve the spec copy '{}'", request.copy.display()))?;
    require_recorded_copy(&request.manifest, &recorded, &copy)?;
    require_manifest_outside_the_copy(&request.manifest, &copy)?;
    if let Some(declaration) = copy_declaration(&copy) {
        require_no_added_specification_paths(&copy, &recorded, &declaration)?;
    }
    require_kept_paths_present(&copy, &recorded)?;

    let mut specification: BTreeMap<String, Difference> = BTreeMap::new();
    let mut end_to_end: BTreeMap<String, Difference> = BTreeMap::new();

    let checkout = Checkout::read(project_root)?;
    for (path, checksum) in &recorded.checksums.end_to_end {
        if path == DECLARATION_PATH {
            continue;
        }
        if let Some(kind) =
            checkout.observed(path, checksum, recorded.checksums.scenarios.get(path))?
        {
            observe(&mut end_to_end, path, kind, None);
        }
    }
    for (path, checksum) in &recorded.checksums.specification {
        if recorded.checksums.end_to_end.contains_key(path) {
            continue;
        }
        let Some(kind) = checkout.observed(path, checksum, None)? else {
            continue;
        };
        let note = (kind == DifferenceKind::Changed).then_some(CHECKOUT_CHANGED);
        observe(&mut specification, path, kind, note);
    }
    for (path, owned_by_end_to_end) in checkout.added(&recorded)? {
        let subject = if owned_by_end_to_end {
            &mut end_to_end
        } else {
            &mut specification
        };
        observe(subject, &path, DifferenceKind::Added, None);
    }

    for (path, checksum) in &recorded.checksums.specification {
        let kind = match local_checksum(&copy, path)? {
            None => DifferenceKind::Missing,
            Some(observed) if observed != *checksum => DifferenceKind::Changed,
            Some(_) => continue,
        };
        if end_to_end.contains_key(path) {
            observe(&mut end_to_end, path, kind, None);
        } else {
            observe(&mut specification, path, kind, None);
        }
    }

    Ok(CompareReport {
        specification: ordered(specification),
        end_to_end: ordered(end_to_end),
    })
}

/// The checkout side: every content path with its backend identity, the
/// declaration the working tree carries, and the bytes read from disk.
struct Checkout {
    root: PathBuf,
    entries: BTreeMap<String, Option<String>>,
    declaration: Option<SpecificationDeclaration>,
}

impl Checkout {
    /// Read the selected backend's content paths and the current declaration.
    /// A declaration the loader refuses contributes no roles: the recorded
    /// declaration path is already a `changed` or `missing` difference.
    fn read(root: &Path) -> Result<Self> {
        let backend = crate::scm::select(root)?;
        let mut entries = BTreeMap::new();
        for entry in backend
            .content_paths()
            .with_context(|| format!("read the project tree at '{}'", root.display()))?
        {
            let path = entry.path.to_string_lossy().replace('\\', "/");
            entries.insert(path, entry.gitlink);
        }
        let declaration = SpecificationDeclaration::load(root).ok().flatten();
        Ok(Self {
            root: root.to_path_buf(),
            entries,
            declaration,
        })
    }

    /// The content paths, sorted and deduplicated.
    fn paths(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }

    /// Whether the checkout still holds a working-tree entry at `path`. The
    /// index keeps listing a file deleted from disk.
    fn presents(&self, path: &str) -> bool {
        std::fs::symlink_metadata(self.root.join(path)).is_ok()
    }

    /// One content path's checksum. A repository link hashes its recorded
    /// commit id, which is the only content a gitlink carries.
    fn checksum(&self, path: &str) -> Result<Option<String>> {
        if let Some(Some(gitlink)) = self.entries.get(path) {
            return Ok(Some(manifest::path_checksum(
                path,
                EntryKind::Submodule,
                false,
                gitlink.as_bytes(),
            )));
        }
        local_checksum(&self.root, path)
    }

    /// The path's difference against a recorded checksum, or `None` when it
    /// matches. The selected-scenario checksum is compared only after the
    /// bytes match, so a content edit is already `changed`.
    fn observed(
        &self,
        path: &str,
        checksum: &str,
        scenario: Option<&String>,
    ) -> Result<Option<DifferenceKind>> {
        let Some(observed) = self.checksum(path)? else {
            return Ok(Some(DifferenceKind::Missing));
        };
        if observed != *checksum {
            return Ok(Some(DifferenceKind::Changed));
        }
        let Some(scenario) = scenario else {
            return Ok(None);
        };
        let matches = self.scenario_digest(path)?.as_ref() == Some(scenario);
        Ok((!matches).then_some(DifferenceKind::Changed))
    }

    /// The selected `@e2e` scenario digest of one checkout feature.
    fn scenario_digest(&self, path: &str) -> Result<Option<String>> {
        let target = self.root.join(path);
        let bytes =
            std::fs::read(&target).with_context(|| format!("read '{}'", target.display()))?;
        let Ok(source) = String::from_utf8(bytes) else {
            return Ok(None);
        };
        Ok(e2e_scenario_digest(path, &source).ok().flatten())
    }

    /// The content paths the manifest does not record but the checkout's
    /// declaration matches, each with whether `End-to-end tests` owns it.
    fn added(&self, recorded: &manifest::Manifest) -> Result<Vec<(String, bool)>> {
        let Some(declaration) = &self.declaration else {
            return Ok(Vec::new());
        };
        let roles: Vec<Glob> = declaration
            .roles()
            .into_iter()
            .flat_map(|(_, patterns)| patterns.iter())
            .map(|pattern| Glob::compile(pattern))
            .collect();
        let behavior: Vec<Glob> = declaration
            .roles()
            .into_iter()
            .filter(|(role, _)| *role == Role::Behavior)
            .flat_map(|(_, patterns)| patterns.iter())
            .map(|pattern| Glob::compile(pattern))
            .collect();
        let content = self.paths();
        let end_to_end_set = e2e_file_set(declaration, &content);
        let recorded_paths: BTreeSet<&str> = recorded
            .specification_files
            .iter()
            .chain(&recorded.source_kept_files)
            .chain(&recorded.end_to_end_files)
            .map(String::as_str)
            .collect();
        let mut added = Vec::new();
        for path in self.entries.keys() {
            if recorded_paths.contains(path.as_str()) || !self.presents(path) {
                continue;
            }
            if !roles
                .iter()
                .any(|glob| glob_matches_path_or_parent(glob, path))
            {
                continue;
            }
            let owned_by_end_to_end = (path != DECLARATION_PATH && end_to_end_set.contains(path))
                || (path.ends_with(".feature")
                    && behavior
                        .iter()
                        .any(|glob| glob_matches_path_or_parent(glob, path))
                    && self.scenario_digest(path)?.is_some());
            added.push((path.clone(), owned_by_end_to_end));
        }
        Ok(added)
    }
}

/// The manifest must belong to the copy it describes: its recorded
/// `copy_directory` resolves to the canonical `--copy` argument.
fn require_recorded_copy(
    manifest: &Path,
    recorded: &manifest::Manifest,
    copy: &Path,
) -> Result<()> {
    let recorded_copy = Path::new(&recorded.copy_directory);
    let matches = recorded_copy
        .canonicalize()
        .map(|resolved| resolved == copy)
        .unwrap_or(false);
    if matches {
        return Ok(());
    }
    bail!(
        "the manifest '{}' records the copy directory '{}' but the copy is '{}'; the manifest must belong to the copy it describes",
        manifest.display(),
        recorded_copy.display(),
        copy.display()
    )
}

/// The manifest path must lie outside the copy: the manifest is evidence
/// about the copy, never content of it.
fn require_manifest_outside_the_copy(manifest: &Path, copy: &Path) -> Result<()> {
    let resolved = manifest
        .canonicalize()
        .with_context(|| format!("resolve the manifest '{}'", manifest.display()))?;
    if !resolved.starts_with(copy) {
        return Ok(());
    }
    bail!(
        "the manifest '{}' is inside the spec copy '{}'; the manifest is evidence outside the copy",
        manifest.display(),
        copy.display()
    )
}

/// The spec copy's declaration: the added-file refusal is read with the
/// declaration the copy carries, not the checkout's. A declaration the copy
/// does not carry, or one the loader refuses, contributes no roles: the
/// recorded declaration checksum already reports it as missing or changed.
fn copy_declaration(copy: &Path) -> Option<SpecificationDeclaration> {
    std::fs::read_to_string(copy.join(DECLARATION_PATH))
        .ok()
        .and_then(|content| SpecificationDeclaration::from_content(&content).ok())
}

/// Refuse a copy that holds a file under any declared Specification role the
/// manifest does not record. The `e2e` role bounds the refusal even though the
/// copy never carries an `e2e` match.
fn require_no_added_specification_paths(
    copy: &Path,
    recorded: &manifest::Manifest,
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
fn require_kept_paths_present(copy: &Path, recorded: &manifest::Manifest) -> Result<()> {
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
            existing.note = if merged == DifferenceKind::Changed {
                note.or(existing.note)
            } else {
                None
            };
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
