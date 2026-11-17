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
use serde::{Deserialize, Serialize};

use crate::config::declaration::{DECLARATION_PATH, Role, SpecificationDeclaration};
use crate::config::manifest::Manifest;
use crate::config::overlay::load_effective_at;
use crate::scm::{Backend, EntryKind, RevisionEntry};

mod partition;
mod record;
/// The retired verifier body; the comparison slice reuses its drift reads
/// and replaces the module.
#[allow(dead_code)]
mod verify;

pub(crate) use record::abbreviate;

/// What one rebuild command replaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RebuildScope {
    /// Every tracked file under the declared source roots.
    All,
    /// A tracked file or a directory's tracked files; the command requires
    /// at least one path.
    Paths(Vec<String>),
}

/// `givn ise rebuild-all`'s and `givn ise rebuild-component`'s parsed
/// request.
#[derive(Debug, Clone)]
pub struct RebuildRequest {
    /// Revision to read; `None` and `HEAD` both resolve the current revision.
    pub at: Option<String>,
    /// The files this rebuild replaces.
    pub scope: RebuildScope,
    /// Spec copy directory; required and outside the checkout.
    pub into: PathBuf,
    /// Optional manifest path; written only when supplied.
    pub manifest: Option<PathBuf>,
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

/// One successful preparation's report: the fixed lines, the two digest
/// rollups, the frozen package's role and size, the write targets, and the
/// record when it went to stdout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub backend: &'static str,
    pub revision: String,
    pub counts: Counts,
    pub retained_digest: String,
    pub e2e_digest: String,
    /// The roles whose files the trial does not hold; empty when nothing is
    /// frozen, so the report prints no frozen line.
    pub excluded_roles: Vec<&'static str>,
    /// The frozen paths the trial omits, counted once per frozen path.
    pub frozen_files: usize,
    pub written_path: PathBuf,
    /// The sorted paths a dry run would write; `None` outside a dry run.
    pub would_write: Option<Vec<String>>,
    /// The YAML record printed after a successful rebuild.
    pub record: Option<String>,
}

/// The five placement counts the report prints.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub retained: usize,
    pub kept_source: usize,
    pub replaced: usize,
    pub outside_declaration: usize,
    pub not_replaced: usize,
}

/// Materialize a trial from one recorded revision and report its placement.
pub fn rebuild(project_root: &Path, request: &RebuildRequest) -> Result<Report> {
    let admission = admit(project_root, request.at.as_deref())?;
    let placement = partition::partition(
        &admission.declaration,
        &admission.entries,
        request,
        &admission.revision,
    )?;
    let trial = require_write_targets(project_root, request)?;
    require_copy_set(&copy_set(&placement), &admission.entries)?;

    let copy_paths = placement.copy_paths();
    let revision_paths: Vec<&str> = admission
        .entries
        .iter()
        .map(|entry| entry.path.as_str())
        .collect();
    let e2e_paths: Vec<String> =
        crate::e2e_boundary::e2e_file_set(&admission.declaration, &revision_paths)
            .into_iter()
            .collect();
    let replaced_paths = placement.replaced_paths().to_vec();
    let read_paths = identity_paths(&copy_paths, &e2e_paths, &replaced_paths);
    let blobs: BTreeMap<String, Vec<u8>> = admission
        .backend
        .revision_blobs(&admission.revision, &read_paths)
        .with_context(|| format!("read the trial inputs at revision '{}'", admission.revision))?
        .into_iter()
        .collect();

    let counts = placement.counts();
    let excluded_roles: Vec<&'static str> = if placement.frozen_paths().is_empty() {
        Vec::new()
    } else {
        vec![Role::E2e.as_str()]
    };
    let retained_files =
        record::identities(placement.retained_paths(), &admission.entries, &blobs)?;
    let e2e_files = record::identities(&e2e_paths, &admission.entries, &blobs)?;
    let e2e_scenarios =
        record::scenario_digests(&admission.declaration, &admission.entries, &blobs)?;
    let replaced_files = record::identities(&replaced_paths, &admission.entries, &blobs)?;
    let record = record::TrialRecord {
        schema: record::SCHEMA,
        content_scope: record::CONTENT_SCOPE.to_string(),
        backend: admission.backend.name().to_string(),
        revision: admission.revision.clone(),
        named_paths: named_paths(request),
        replaced_paths,
        counts,
        excluded_roles: excluded_roles
            .iter()
            .map(|role| (*role).to_string())
            .collect(),
        retained_digest: record::rollup(&retained_files),
        e2e_digest: record::e2e_rollup(&e2e_files, &e2e_scenarios),
        replaced_digest: record::rollup(&replaced_files),
        retained_files,
        e2e_files,
        e2e_scenarios,
        written_path: trial.to_string_lossy().into_owned(),
        givn_version: env!("CARGO_PKG_VERSION").to_string(),
    };
    let mut report = Report {
        backend: admission.backend.name(),
        revision: admission.revision.clone(),
        counts,
        retained_digest: record.retained_digest.clone(),
        e2e_digest: record.e2e_digest.clone(),
        excluded_roles,
        frozen_files: placement.frozen_paths().len(),
        written_path: trial.clone(),
        would_write: request.dry_run.then(|| copy_paths.clone()),
        record: None,
    };
    if request.dry_run {
        return Ok(report);
    }

    let files = trial_files(&admission.entries, &copy_paths, &blobs)?;
    let outcome = write_trial(&trial, &files, &FileWriter)
        .with_context(|| format!("write revision '{}'", admission.revision))?;
    if let WriteOutcome::Mismatched { paths } = outcome {
        bail!(
            "the written trial differs from revision '{}' at {}",
            admission.revision,
            paths.join(", ")
        );
    }
    report.record = Some(record.yaml()?);
    Ok(report)
}

/// The `--path` values the record keeps: normalized, sorted, deduplicated.
/// A value the normalizer rejects already refused in the partition.
fn named_paths(request: &RebuildRequest) -> Vec<String> {
    let RebuildScope::Paths(paths) = &request.scope else {
        return Vec::new();
    };
    let mut paths: Vec<String> = paths
        .iter()
        .filter_map(|value| crate::config::declaration::normalize_project_path(value))
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// The union of the copy set and the frozen set: every revision path the trial
/// may copy or the record names, so a repository link in either set refuses by
/// name before any content is read.
fn copy_set(placement: &partition::Partition) -> Vec<String> {
    let mut paths: Vec<String> = placement
        .copy_paths()
        .into_iter()
        .chain(placement.frozen_paths().iter().cloned())
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// The union of the copy set, the `e2e` package files, and the replaced
/// paths: every revision entry an identity or a digest needs.
fn identity_paths(copy: &[String], e2e: &[String], replaced: &[String]) -> Vec<String> {
    let mut paths: Vec<String> = copy.iter().chain(e2e).chain(replaced).cloned().collect();
    paths.sort();
    paths.dedup();
    paths
}

/// The ordered admission checks: add-ons, backend, dirty checkout, revision,
/// then the declaration recorded at that revision. Every check precedes the
/// first write and none writes.
fn admit(project_root: &Path, at: Option<&str>) -> Result<Admission> {
    let manifest = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&manifest)?;

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
    let declaration = revision_declaration(&backend, &revision, &entries)?;

    Ok(Admission {
        backend,
        revision,
        declaration,
        entries,
    })
}

/// The declaration recorded at the selected revision. The four unreadable
/// cases are distinct refusals: the tree does not record the path; the tree
/// records it but the backend returns no blob; the blob is not UTF-8; the
/// blob is malformed. The working tree is never consulted, so the partition
/// and the trial use the roles and the content the revision declared.
fn revision_declaration(
    backend: &Backend,
    revision: &str,
    entries: &[RevisionEntry],
) -> Result<SpecificationDeclaration> {
    if !entries.iter().any(|entry| entry.path == DECLARATION_PATH) {
        bail!(
            "the revision '{revision}' does not record '{DECLARATION_PATH}'; a clean-room trial always carries the declaration"
        );
    }
    let bytes = backend
        .revision_blob(revision, DECLARATION_PATH)
        .with_context(|| format!("read '{DECLARATION_PATH}' at revision '{revision}'"))?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "'{DECLARATION_PATH}' is recorded at revision '{revision}' but the backend returned no blob for it"
            )
        })?;
    let content = std::str::from_utf8(&bytes)
        .map_err(|error| anyhow::anyhow!("{DECLARATION_PATH}: {error}"))?;
    SpecificationDeclaration::from_content(content).map_err(|error| anyhow::anyhow!("{error}"))
}

/// Refusal 8 (spec copy target): the trial lies outside the checkout and is
/// created new. The path is resolved through its deepest existing ancestor, so
/// a symlinked parent cannot alias a checkout path. The manifest target checks
/// land with the manifest slice. Returns the absolute trial path the report and
/// the record name.
fn require_write_targets(project_root: &Path, request: &RebuildRequest) -> Result<PathBuf> {
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
    Ok(trial)
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
    blobs: &BTreeMap<String, Vec<u8>>,
) -> Result<Vec<TrialFile>> {
    let by_path: BTreeMap<&str, &RevisionEntry> = entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect();
    paths
        .iter()
        .map(|path| {
            let entry = by_path.get(path.as_str()).copied().ok_or_else(|| {
                anyhow::anyhow!("the revision does not list '{path}', so the trial cannot write it")
            })?;
            let bytes = blobs
                .get(path)
                .ok_or_else(|| anyhow::anyhow!("the revision returned no content for '{path}'"))?
                .clone();
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

    #[test]
    fn a_submodule_entry_in_the_copy_set_is_refused_as_a_repository_link() {
        let entries = [RevisionEntry {
            path: "vendor/lib".to_string(),
            kind: EntryKind::Submodule,
            executable: false,
        }];
        let error = require_copy_set(&["vendor/lib".to_string()], &entries)
            .expect_err("a repository link is refused");
        assert_eq!(
            error.to_string(),
            "'vendor/lib' is a repository link; a clean-room trial copies no repository link"
        );
    }

    #[test]
    fn a_regular_entry_in_the_copy_set_passes_the_link_check() {
        let entries = [RevisionEntry {
            path: "src/kept.rs".to_string(),
            kind: EntryKind::Regular,
            executable: false,
        }];
        require_copy_set(&["src/kept.rs".to_string()], &entries)
            .expect("a regular file is copied, not refused");
    }
}
