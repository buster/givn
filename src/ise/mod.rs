//! Clean-room trial preparation and verification.
//!
//! This module owns the admission checks that run before any trial content is
//! read or written, the partition of one revision, and the plain-directory
//! write that a verifier inspects. The CLI adapter in `crate::cli::ise`
//! formats the outcome; only the adapter knows about exit codes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::declaration::{
    DECLARATION_PATH, Role, SpecificationDeclaration, normalize_project_path,
};
use crate::config::manifest::Manifest;
use crate::config::overlay::load_effective_at;
use crate::e2e_boundary::Glob;
use crate::scm::{Backend, EntryKind, RevisionEntry};

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

/// One successful preparation's fixed report lines, before the record
/// identities and the written path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub backend: &'static str,
    pub revision: String,
    pub counts: Counts,
}

/// The five placement counts the report prints.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    pub retained: usize,
    pub kept_source: usize,
    pub replaced: usize,
    pub outside_declaration: usize,
    pub not_replaced: usize,
}

/// Every tracked path at the revision, placed by the declaration.
#[derive(Debug, Default, PartialEq, Eq)]
struct Partition {
    retained: Vec<String>,
    kept_source: Vec<String>,
    replaced: Vec<String>,
    outside: Vec<String>,
    not_replaced: usize,
}

impl Partition {
    fn counts(&self) -> Counts {
        Counts {
            retained: self.retained.len(),
            kept_source: self.kept_source.len(),
            replaced: self.replaced.len(),
            outside_declaration: self.outside.len(),
            not_replaced: self.not_replaced,
        }
    }

    /// The sorted union of retained and kept source paths: every file the
    /// trial copies from the revision.
    fn copy_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self
            .retained
            .iter()
            .chain(&self.kept_source)
            .cloned()
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }
}

/// Materialize a trial from one recorded revision and report its placement.
pub fn replace(project_root: &Path, request: &ReplaceRequest) -> Result<Report> {
    let admission = admit(project_root, request.at.as_deref())?;
    let partition = partition(&admission.declaration, &admission.entries, request);
    let report = Report {
        backend: admission.backend.name(),
        revision: admission.revision.clone(),
        counts: partition.counts(),
    };
    if request.dry_run {
        return Ok(report);
    }

    let copy_paths = partition.copy_paths();
    let blobs = admission
        .backend
        .revision_blobs(&admission.revision, &copy_paths)
        .with_context(|| format!("read the copy set at revision '{}'", admission.revision))?;
    let files = trial_files(&admission.entries, &copy_paths, blobs)?;
    let outcome = write_trial(&request.into, &files, &FileWriter)
        .with_context(|| format!("write revision '{}'", admission.revision))?;
    if let WriteOutcome::Mismatched { paths } = outcome {
        bail!(
            "the written trial differs from revision '{}' at {}",
            admission.revision,
            paths.join(", ")
        );
    }
    Ok(report)
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

/// Place every tracked path at the revision. A retained role is tested first,
/// so a retained file under a source root stays in the trial under every form.
fn partition(
    declaration: &SpecificationDeclaration,
    entries: &[RevisionEntry],
    request: &ReplaceRequest,
) -> Partition {
    let roles = retained_roles(declaration);
    let source_roots: Vec<Glob> = declaration
        .source_roots()
        .iter()
        .map(|root| Glob::compile(root))
        .collect();
    let tracked: BTreeSet<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();

    let mut retained: Vec<String> = entries
        .iter()
        .filter(|entry| retained_role(&entry.path, &roles).is_some())
        .map(|entry| entry.path.clone())
        .collect();
    retained.sort();

    let candidates: BTreeSet<String> = if request.full {
        entries
            .iter()
            .filter(|entry| path_or_parent_matches(&source_roots, &entry.path))
            .map(|entry| entry.path.clone())
            .collect()
    } else {
        named_candidates(&tracked, &request.paths)
    };
    let retained_set: BTreeSet<&str> = retained.iter().map(String::as_str).collect();
    let mut replaced: Vec<String> = candidates
        .into_iter()
        .filter(|path| !retained_set.contains(path.as_str()))
        .collect();
    replaced.sort();
    let replaced_set: BTreeSet<&str> = replaced.iter().map(String::as_str).collect();

    let mut kept_source: Vec<String> = entries
        .iter()
        .map(|entry| &entry.path)
        .filter(|path| {
            !retained_set.contains(path.as_str())
                && !replaced_set.contains(path.as_str())
                && path_or_parent_matches(&source_roots, path)
        })
        .cloned()
        .collect();
    kept_source.sort();

    let mut outside: Vec<String> = entries
        .iter()
        .map(|entry| &entry.path)
        .filter(|path| {
            !retained_set.contains(path.as_str())
                && !replaced_set.contains(path.as_str())
                && !path_or_parent_matches(&source_roots, path)
        })
        .cloned()
        .collect();
    outside.sort();

    let not_replaced = direct_siblings(&tracked, &replaced_set);
    Partition {
        retained,
        kept_source,
        replaced,
        outside,
        not_replaced,
    }
}

/// The retained role patterns in declaration order. The first matching role
/// wins, so the order is part of the contract.
fn retained_roles(declaration: &SpecificationDeclaration) -> Vec<(Role, Glob)> {
    declaration
        .roles()
        .into_iter()
        .flat_map(|(role, patterns)| {
            patterns
                .iter()
                .map(move |pattern| (role, Glob::compile(pattern)))
        })
        .collect()
}

fn retained_role(path: &str, roles: &[(Role, Glob)]) -> Option<Role> {
    roles
        .iter()
        .find(|(_, glob)| glob_matches_path_or_parent(glob, path))
        .map(|(role, _)| *role)
}

/// The `--path` values that resolve to tracked content. A tracked file
/// contributes itself, a directory contributes every tracked file beneath it,
/// a glob-shaped value contributes nothing, and repeats collapse.
fn named_candidates(tracked: &BTreeSet<&str>, values: &[String]) -> BTreeSet<String> {
    let mut directories = BTreeSet::new();
    for path in tracked {
        for (index, _) in path.match_indices('/') {
            directories.insert(path[..index].to_string());
        }
    }

    let mut candidates = BTreeSet::new();
    for value in values {
        let Some(value) = normalize_project_path(value) else {
            continue;
        };
        if tracked.contains(value.as_str()) {
            candidates.insert(value);
            continue;
        }
        if directories.contains(&value) {
            let prefix = format!("{value}/");
            candidates.extend(
                tracked
                    .iter()
                    .filter(|path| path.starts_with(&prefix))
                    .map(|path| (*path).to_string()),
            );
        }
    }
    candidates
}

/// Whether a role or source pattern matches the path or every tracked file
/// under a directory it matches.
fn path_or_parent_matches(globs: &[Glob], path: &str) -> bool {
    globs
        .iter()
        .any(|glob| glob_matches_path_or_parent(glob, path))
}

fn glob_matches_path_or_parent(glob: &Glob, path: &str) -> bool {
    glob.matches(path)
        || path
            .match_indices('/')
            .any(|(index, _)| glob.matches(&path[..index]))
}

/// The direct tracked files in the replaced files' parent directories that
/// were not themselves replaced.
fn direct_siblings(tracked: &BTreeSet<&str>, replaced: &BTreeSet<&str>) -> usize {
    let mut parents = BTreeSet::new();
    for path in replaced {
        if let Some((parent, _)) = path.rsplit_once('/') {
            parents.insert(parent);
        }
    }
    tracked
        .iter()
        .filter(|path| {
            !replaced.contains(**path)
                && path
                    .rsplit_once('/')
                    .is_some_and(|(parent, _)| parents.contains(parent))
        })
        .count()
}

/// One file the trial copies from the revision.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TrialFile {
    path: String,
    kind: EntryKind,
    executable: bool,
    bytes: Vec<u8>,
}

/// Pair the copy set with its revision metadata and blobs.
fn trial_files(
    entries: &[RevisionEntry],
    paths: &[String],
    blobs: Vec<(String, Vec<u8>)>,
) -> Result<Vec<TrialFile>> {
    let by_path: BTreeMap<&str, &RevisionEntry> = entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect();
    let mut bytes: BTreeMap<String, Vec<u8>> = blobs.into_iter().collect();
    paths
        .iter()
        .map(|path| {
            let entry = by_path.get(path.as_str()).copied().ok_or_else(|| {
                anyhow::anyhow!("the revision does not list '{path}', so the trial cannot write it")
            })?;
            let bytes = bytes
                .remove(path)
                .ok_or_else(|| anyhow::anyhow!("the revision returned no content for '{path}'"))?;
            Ok(TrialFile {
                path: path.clone(),
                kind: entry.kind,
                executable: entry.executable,
                bytes,
            })
        })
        .collect()
}

/// The write outcome: either every written file matches the revision, or the
/// mismatching paths were found after the trial was removed.
#[derive(Debug, Clone, PartialEq, Eq)]
enum WriteOutcome {
    Written,
    Mismatched { paths: Vec<String> },
}

/// Writes one trial file. Injectable so the re-read comparison can be proven
/// against a writer that corrupts a path between the write and the re-read.
trait TrialWriter {
    fn write(&self, trial: &Path, file: &TrialFile) -> Result<()>;
}

/// The real writer: a plain directory, no version-control command.
struct FileWriter;

impl TrialWriter for FileWriter {
    fn write(&self, trial: &Path, file: &TrialFile) -> Result<()> {
        let target = trial.join(&file.path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create '{}'", parent.display()))?;
        }
        match file.kind {
            EntryKind::Regular => {
                std::fs::write(&target, &file.bytes)
                    .with_context(|| format!("write '{}'", target.display()))?;
                set_executable(&target, file.executable)?;
            }
            EntryKind::Symlink => create_symlink(&target, &file.bytes)?,
            EntryKind::Submodule => bail!("'{}' is a repository link", file.path),
        }
        Ok(())
    }
}

/// Create the trial, write every file, then re-read and compare each one. A
/// mismatch removes the trial, so no partial evidence survives.
fn write_trial(
    trial: &Path,
    files: &[TrialFile],
    writer: &dyn TrialWriter,
) -> Result<WriteOutcome> {
    if let Some(parent) = trial
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create '{}'", parent.display()))?;
    }
    std::fs::create_dir(trial)
        .with_context(|| format!("create the trial at '{}'", trial.display()))?;
    for file in files {
        writer
            .write(trial, file)
            .with_context(|| format!("write '{}'", file.path))?;
    }

    let mut mismatched: Vec<String> = files
        .iter()
        .filter(|file| !written_matches(trial, file))
        .map(|file| file.path.clone())
        .collect();
    if mismatched.is_empty() {
        return Ok(WriteOutcome::Written);
    }
    mismatched.sort();
    std::fs::remove_dir_all(trial)
        .with_context(|| format!("remove the mismatching trial at '{}'", trial.display()))?;
    Ok(WriteOutcome::Mismatched { paths: mismatched })
}

fn written_matches(trial: &Path, file: &TrialFile) -> bool {
    let target = trial.join(&file.path);
    let Ok(metadata) = std::fs::symlink_metadata(&target) else {
        return false;
    };
    match file.kind {
        EntryKind::Regular => {
            metadata.file_type().is_file()
                && executable_matches(&metadata, file.executable)
                && std::fs::read(&target).is_ok_and(|bytes| bytes == file.bytes)
        }
        EntryKind::Symlink => {
            metadata.file_type().is_symlink()
                && std::fs::read_link(&target)
                    .is_ok_and(|link| link.to_string_lossy().as_bytes() == file.bytes.as_slice())
        }
        EntryKind::Submodule => false,
    }
}

#[cfg(unix)]
fn set_executable(path: &Path, executable: bool) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path)
        .with_context(|| format!("read the mode of '{}'", path.display()))?
        .permissions();
    let mode = permissions.mode() & 0o777;
    permissions.set_mode(if executable { mode | 0o111 } else { mode });
    std::fs::set_permissions(path, permissions)
        .with_context(|| format!("set the mode of '{}'", path.display()))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path, _executable: bool) -> Result<()> {
    Ok(())
}

#[cfg(unix)]
fn create_symlink(path: &Path, target: &[u8]) -> Result<()> {
    let target = std::str::from_utf8(target).context("a symlink target is not UTF-8")?;
    std::os::unix::fs::symlink(target, path)
        .with_context(|| format!("create the symlink at '{}'", path.display()))
}

#[cfg(not(unix))]
fn create_symlink(path: &Path, _target: &[u8]) -> Result<()> {
    bail!(
        "'{}' is a symlink; this platform cannot write one",
        path.display()
    )
}

#[cfg(unix)]
fn executable_matches(metadata: &std::fs::Metadata, executable: bool) -> bool {
    use std::os::unix::fs::PermissionsExt;
    (metadata.permissions().mode() & 0o111 != 0) == executable
}

#[cfg(not(unix))]
fn executable_matches(_metadata: &std::fs::Metadata, _executable: bool) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the file the real writer would, then corrupts its content.
    struct CorruptingWriter;

    impl TrialWriter for CorruptingWriter {
        fn write(&self, trial: &Path, file: &TrialFile) -> Result<()> {
            FileWriter.write(trial, file)?;
            std::fs::write(trial.join(&file.path), b"corrupted\n")?;
            Ok(())
        }
    }

    fn one_file() -> TrialFile {
        TrialFile {
            path: "src/kept.rs".to_string(),
            kind: EntryKind::Regular,
            executable: false,
            bytes: b"pub fn kept() {}\n".to_vec(),
        }
    }

    #[test]
    fn a_re_read_mismatch_names_the_path_and_removes_the_trial() {
        let container = tempfile::TempDir::new().unwrap();
        let trial = container.path().join("trial");
        let outcome = write_trial(&trial, &[one_file()], &CorruptingWriter)
            .expect("the corrupting writer succeeds");
        assert_eq!(
            outcome,
            WriteOutcome::Mismatched {
                paths: vec!["src/kept.rs".to_string()]
            }
        );
        assert!(!trial.exists(), "the mismatching trial must be removed");
    }

    #[test]
    fn a_matching_write_keeps_the_trial() {
        let container = tempfile::TempDir::new().unwrap();
        let trial = container.path().join("trial");
        let outcome =
            write_trial(&trial, &[one_file()], &FileWriter).expect("the trial is written");
        assert_eq!(outcome, WriteOutcome::Written);
        assert_eq!(
            std::fs::read(trial.join("src/kept.rs")).unwrap(),
            one_file().bytes
        );
    }
}
