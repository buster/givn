//! `givn ise verify`: the re-check of one prepared trial.
//!
//! The add-on precondition runs before the record is read. The record is the
//! verifier's trust anchor only after its placement, backend, and written path
//! match the trial; the retained set is then re-read from the trial and the
//! `e2e` package and selected `@e2e` scenarios from the current checkout.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::declaration::{DECLARATION_PATH, SpecificationDeclaration};
use crate::config::overlay::load_effective_at;
use crate::gherkin::content::e2e_scenario_digest;
use crate::scm::{Backend, EntryKind};

use super::record::{self, TrialRecord};
use super::{partition, require_addons};

/// A recorded retained path is absent from the trial.
const TRIAL_MISSING_PATH: &str = "the path is missing from the trial";
/// The trial's content differs from the recorded identity.
const TRIAL_DIFFERS: &str = "the trial differs from the record";
/// A recorded `e2e` package file is absent from the current package.
const CHECKOUT_MISSING_PATH: &str = "the path is missing from the checkout's e2e package";
/// The current package holds a file the record does not.
const CHECKOUT_GAINED_PATH: &str = "the checkout's e2e package gained this path";
/// The current `e2e` package file differs from the recorded identity.
const CHECKOUT_DIFFERS: &str = "the checkout differs from the record";
/// The per-feature `@e2e` scenario digest differs, or the feature is missing.
const SCENARIOS_DIFFER: &str = "the selected @e2e scenarios differ in the checkout";

/// Which report line a drifting path is printed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DriftLine {
    Retained,
    E2e,
}

/// One drifting path's report line and reason, keyed so one run renders every
/// path in a stable order.
type Drift = BTreeMap<(DriftLine, String), &'static str>;

/// One re-check's outcome: the resolved trial and every drifting path with its
/// reason. An empty map is a clean trial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyReport {
    pub trial: PathBuf,
    pub drift: Drift,
}

/// Why one path carries no identity to compare.
enum IdentityGap {
    Missing,
    Differs,
}

/// Re-check a prepared trial's frozen retained and `e2e` inputs.
pub fn verify(project_root: &Path, trial: &Path, at: &Path) -> Result<VerifyReport> {
    let manifest = load_effective_at(project_root).with_context(|| {
        format!(
            "read the effective manifest at '{}'",
            project_root.display()
        )
    })?;
    require_addons(&manifest)?;

    let record = read_record(trial)?;
    let trial_directory = trial_directory(at)?;
    let backend = crate::scm::select(project_root)?;
    require_record_belongs(trial, &record, &trial_directory, backend.name())?;

    let mut drift = Drift::new();
    check_retained(&trial_directory, &record, &mut drift);
    check_checkout(project_root, &backend, &record, &mut drift)?;
    Ok(VerifyReport {
        trial: trial_directory,
        drift,
    })
}

/// The strict record load: an unreadable file and a malformed shape both name
/// the record and the reason. The schema, the content scope, and the replaced
/// digest are asserted before any identity is compared.
fn read_record(path: &Path) -> Result<TrialRecord> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read the trial record '{}'", path.display()))?;
    let record: TrialRecord = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("read the trial record '{}'", path.display()))?;
    if record.schema != record::SCHEMA {
        bail!(
            "the record '{}' has the schema {}; a trial record is schema {}",
            path.display(),
            record.schema,
            record::SCHEMA
        );
    }
    if record.content_scope != record::CONTENT_SCOPE {
        bail!(
            "the record '{}' has the content scope '{}'; expected '{}'",
            path.display(),
            record.content_scope,
            record::CONTENT_SCOPE
        );
    }
    if !is_sha256_hex(&record.replaced_digest) {
        bail!(
            "the record '{}' has no 64-hex replaced_digest; the record is no trial evidence",
            path.display()
        );
    }
    Ok(record)
}

/// Whether a recorded digest is a full SHA-256 value.
fn is_sha256_hex(digest: &str) -> bool {
    digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// The trial directory the record must name; a missing directory refuses
/// before any identity is compared.
fn trial_directory(at: &Path) -> Result<PathBuf> {
    at.canonicalize()
        .with_context(|| format!("the trial directory '{}' is missing", at.display()))
}

/// The record belongs to this trial only when it names the same directory,
/// records the selected backend, and lies outside the trial the builder owned.
fn require_record_belongs(
    record_path: &Path,
    record: &TrialRecord,
    trial: &Path,
    backend: &str,
) -> Result<()> {
    let written = Path::new(&record.written_path);
    let resolved_written = written
        .canonicalize()
        .unwrap_or_else(|_| written.to_path_buf());
    if resolved_written != trial {
        bail!(
            "the record '{}' names the written path '{}' but the trial is '{}'",
            record_path.display(),
            record.written_path,
            trial.display()
        );
    }
    if record.backend != backend {
        bail!(
            "the record '{}' records the backend '{}' but the selected backend is '{}'",
            record_path.display(),
            record.backend,
            backend
        );
    }
    let resolved_record = record_path
        .canonicalize()
        .unwrap_or_else(|_| record_path.to_path_buf());
    if resolved_record.starts_with(trial) {
        bail!(
            "the record '{}' lies inside the trial '{}'; the trial's builder must not own the record",
            record_path.display(),
            trial.display()
        );
    }
    Ok(())
}

/// Every recorded retained path: a missing path and a differing digest are the
/// two retained reasons. The recorded set is the trial's authority, not the
/// current declaration.
fn check_retained(trial: &Path, record: &TrialRecord, drift: &mut Drift) {
    for (path, recorded) in &record.retained_files {
        let reason = match identity(trial, path) {
            Err(IdentityGap::Missing) => TRIAL_MISSING_PATH,
            Err(IdentityGap::Differs) => TRIAL_DIFFERS,
            Ok(digest) if digest != *recorded => TRIAL_DIFFERS,
            Ok(_) => continue,
        };
        drift.insert((DriftLine::Retained, path.clone()), reason);
    }
}

/// The current checkout's `e2e` package and selected `@e2e` scenarios. The
/// retained set is not re-resolved: the trial is the recorded retained set,
/// while a new package file or a changed scenario is the project's drift.
fn check_checkout(
    project_root: &Path,
    backend: &Backend,
    record: &TrialRecord,
    drift: &mut Drift,
) -> Result<()> {
    let declaration = SpecificationDeclaration::load(project_root)
        .map_err(|error| anyhow::anyhow!("{error}"))?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "{DECLARATION_PATH} is missing; the clean-room trial requires a declared Specification"
            )
        })?;
    let content = backend
        .content_paths()
        .with_context(|| format!("read the project tree at '{}'", project_root.display()))?;
    let paths: Vec<String> = content
        .iter()
        .map(|entry| entry.path.to_string_lossy().into_owned())
        .collect();
    let current_paths: Vec<&str> = paths.iter().map(String::as_str).collect();

    let current_e2e = crate::e2e_boundary::e2e_file_set(&declaration, &current_paths);
    for (path, recorded) in &record.e2e_files {
        let reason = if !current_e2e.contains(path) {
            CHECKOUT_MISSING_PATH
        } else {
            match identity(project_root, path) {
                Err(_) => CHECKOUT_MISSING_PATH,
                Ok(digest) if digest != *recorded => CHECKOUT_DIFFERS,
                Ok(_) => continue,
            }
        };
        drift.insert((DriftLine::E2e, path.clone()), reason);
    }
    for path in &current_e2e {
        if !record.e2e_files.contains_key(path) {
            drift.insert((DriftLine::E2e, path.clone()), CHECKOUT_GAINED_PATH);
        }
    }
    check_scenarios(project_root, &declaration, &current_paths, record, drift)
}

/// The selected `@e2e` scenarios per `behavior` feature: a missing, an extra,
/// or a changed feature path all carry the scenario reason.
fn check_scenarios(
    project_root: &Path,
    declaration: &SpecificationDeclaration,
    current_paths: &[&str],
    record: &TrialRecord,
    drift: &mut Drift,
) -> Result<()> {
    let mut current: BTreeMap<String, String> = BTreeMap::new();
    for path in partition::behavior_features(declaration, current_paths) {
        let Ok(text) = std::fs::read_to_string(project_root.join(&path)) else {
            continue;
        };
        let digest = e2e_scenario_digest(&path, &text)
            .with_context(|| format!("digest the selected @e2e scenarios in '{path}'"))?;
        if let Some(digest) = digest {
            current.insert(path, digest);
        }
    }
    for (path, recorded) in &record.e2e_scenarios {
        if current.get(path) != Some(recorded) {
            drift.insert((DriftLine::E2e, path.clone()), SCENARIOS_DIFFER);
        }
    }
    for path in current.keys() {
        if !record.e2e_scenarios.contains_key(path) {
            drift.insert((DriftLine::E2e, path.clone()), SCENARIOS_DIFFER);
        }
    }
    Ok(())
}

/// One project-relative path's identity under `root`, read by entry kind,
/// executable bit, and bytes or symlink target.
fn identity(root: &Path, path: &str) -> std::result::Result<String, IdentityGap> {
    let target = root.join(path);
    let metadata = std::fs::symlink_metadata(&target).map_err(|_| IdentityGap::Missing)?;
    let file_type = metadata.file_type();
    if file_type.is_symlink() {
        let link = std::fs::read_link(&target).map_err(|_| IdentityGap::Differs)?;
        return Ok(record::path_digest(
            path,
            EntryKind::Symlink,
            false,
            link.to_string_lossy().as_bytes(),
        ));
    }
    if !file_type.is_file() {
        return Err(IdentityGap::Differs);
    }
    let bytes = std::fs::read(&target).map_err(|_| IdentityGap::Differs)?;
    Ok(record::path_digest(
        path,
        EntryKind::Regular,
        is_executable(&metadata),
        &bytes,
    ))
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
