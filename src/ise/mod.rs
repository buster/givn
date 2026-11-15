//! Clean-room trial preparation and verification.
//!
//! This module owns the admission checks that run before any trial content is
//! read or written, the partition of one revision, and the plain-directory
//! write that a verifier inspects. The CLI adapter in `crate::cli::ise`
//! formats the outcome; only the adapter knows about exit codes.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::declaration::{DECLARATION_PATH, SpecificationDeclaration};
use crate::config::manifest::Manifest;
use crate::config::overlay::load_effective_at;
use crate::scm::{Backend, EntryKind, RevisionEntry};

mod partition;

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

/// Materialize a trial from one recorded revision and report its placement.
pub fn replace(project_root: &Path, request: &ReplaceRequest) -> Result<Report> {
    let admission = admit(project_root, request.at.as_deref())?;
    let partition = partition::partition(
        &admission.declaration,
        &admission.entries,
        request,
        &admission.revision,
    )?;
    let report = Report {
        backend: admission.backend.name(),
        revision: admission.revision.clone(),
        counts: partition.counts(),
    };
    require_write_targets(project_root, request)?;
    require_copy_set(&partition.copy_paths(), &admission.entries)?;
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

/// Refusal 8: both write targets lie outside the checkout and are created new.
/// The trial must not exist; the receipt must not exist, lie inside the trial,
/// or be an ancestor of it. Every path is resolved through its deepest
/// existing ancestor, so a symlinked parent cannot alias a checkout path.
fn require_write_targets(project_root: &Path, request: &ReplaceRequest) -> Result<()> {
    let checkout = project_root
        .canonicalize()
        .with_context(|| format!("resolve the checkout '{}'", project_root.display()))?;
    let trial = resolved_target(&request.into, &checkout)?;
    if trial.starts_with(&checkout) {
        bail!(
            "the target '{}' is inside the checkout '{}'",
            request.into.display(),
            checkout.display()
        );
    }
    if trial.exists() {
        bail!(
            "the target '{}' already exists; a trial is written only into a new path",
            request.into.display()
        );
    }
    let Some(receipt_request) = request.receipt.as_deref() else {
        return Ok(());
    };
    let receipt = resolved_target(receipt_request, &checkout)?;
    if receipt.starts_with(&checkout) {
        bail!(
            "the receipt '{}' is inside the checkout '{}'",
            receipt_request.display(),
            checkout.display()
        );
    }
    if receipt.exists() {
        bail!(
            "the receipt '{}' already exists; a record is never overwritten",
            receipt_request.display()
        );
    }
    if receipt.starts_with(&trial) {
        bail!(
            "the receipt '{}' lies inside the trial '{}'",
            receipt_request.display(),
            trial.display()
        );
    }
    if trial.starts_with(&receipt) {
        bail!(
            "the receipt '{}' is an ancestor of the trial '{}'",
            receipt_request.display(),
            trial.display()
        );
    }
    Ok(())
}

/// The absolute form of a write target: every existing ancestor is
/// canonicalized and the missing components are appended.
fn resolved_target(path: &Path, checkout: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        checkout.join(path)
    };
    let mut missing: Vec<OsString> = Vec::new();
    let mut current = absolute.as_path();
    let existing = loop {
        match current.canonicalize() {
            Ok(canonical) => break canonical,
            Err(error) => {
                let name = current.file_name().ok_or_else(|| {
                    anyhow::anyhow!(
                        "the path '{}' has no existing ancestor: {error}",
                        path.display()
                    )
                })?;
                missing.push(name.to_os_string());
                current = current.parent().ok_or_else(|| {
                    anyhow::anyhow!(
                        "the path '{}' has no existing ancestor: {error}",
                        path.display()
                    )
                })?;
            }
        }
    };
    let mut resolved = existing;
    for name in missing.iter().rev() {
        resolved.push(name);
    }
    Ok(resolved)
}

/// Refusal 9: the copy set carries no repository link and no version-control
/// marker at any depth. The check runs before any content is read or written.
fn require_copy_set(paths: &[String], entries: &[RevisionEntry]) -> Result<()> {
    for path in paths {
        if path
            .split('/')
            .any(|component| matches!(component, ".git" | ".fslckout" | "_FOSSIL_"))
        {
            bail!(
                "'{path}' is a version-control marker; a clean-room trial copies no repository metadata"
            );
        }
        let is_link = entries
            .iter()
            .any(|entry| entry.path == *path && entry.kind == EntryKind::Submodule);
        if is_link {
            bail!("'{path}' is a repository link; a clean-room trial copies no repository link");
        }
    }
    Ok(())
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
