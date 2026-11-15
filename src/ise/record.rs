//! The `ise-trial/v1` record.
//!
//! One record carries the placement counts, one identity per retained and
//! `e2e` path, the per-feature `@e2e` scenario digest, and one rollup per
//! identity. The identity rule lives here so preparation and verification
//! hash the same shape.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::Counts;
use crate::config::declaration::SpecificationDeclaration;
use crate::scm::{EntryKind, RevisionEntry};

/// The record schema version.
pub(super) const SCHEMA: u32 = 1;
/// The record content scope; it names the record's shape, not the change
/// verification receipt.
pub(super) const CONTENT_SCOPE: &str = "ise-trial/v1";

/// One prepared trial's recorded evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TrialRecord {
    pub schema: u32,
    pub content_scope: String,
    pub backend: String,
    pub revision: String,
    pub named_paths: Vec<String>,
    pub replaced_paths: Vec<String>,
    pub counts: Counts,
    pub retained_digest: String,
    pub e2e_digest: String,
    pub replaced_digest: String,
    pub retained_files: BTreeMap<String, String>,
    pub e2e_files: BTreeMap<String, String>,
    pub e2e_scenarios: BTreeMap<String, String>,
    pub written_path: String,
    pub givn_version: String,
}

impl TrialRecord {
    /// Serialize the record with the field order the `ise-trial/v1` scope
    /// names.
    pub(super) fn yaml(&self) -> Result<String> {
        serde_yaml_ng::to_string(self).context("serialize the trial record")
    }

    /// Write the record, refusing to replace an existing file.
    pub(super) fn write(&self, path: &Path) -> Result<()> {
        let yaml = self.yaml()?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .with_context(|| format!("write the trial record '{}'", path.display()))?;
        file.write_all(yaml.as_bytes())
            .with_context(|| format!("write the trial record '{}'", path.display()))
    }
}

/// One identity per named path, keyed by the project-relative path.
pub(super) fn identities(
    paths: &[String],
    entries: &[RevisionEntry],
    blobs: &BTreeMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, String>> {
    let by_path: BTreeMap<&str, &RevisionEntry> = entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect();
    let mut identities = BTreeMap::new();
    for path in paths {
        let entry = by_path.get(path.as_str()).ok_or_else(|| {
            anyhow::anyhow!("the revision does not list '{path}', so no identity exists")
        })?;
        let content = blobs
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("the revision returned no content for '{path}'"))?;
        identities.insert(
            path.clone(),
            path_digest(path, entry.kind, entry.executable, content),
        );
    }
    Ok(identities)
}

/// The per-feature digest of the scenarios tagged `@e2e`, selected from the
/// `.feature` files the `behavior` role matches at the revision.
pub(super) fn scenario_digests(
    declaration: &SpecificationDeclaration,
    entries: &[RevisionEntry],
    blobs: &BTreeMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, String>> {
    let paths: Vec<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();
    let mut digests = BTreeMap::new();
    for path in super::partition::behavior_features(declaration, &paths) {
        let content = blobs
            .get(&path)
            .ok_or_else(|| anyhow::anyhow!("the revision returned no content for '{path}'"))?;
        let source = std::str::from_utf8(content)
            .with_context(|| format!("read the feature at '{path}' as UTF-8"))?;
        if let Some(digest) = crate::gherkin::content::e2e_scenario_digest(&path, source)
            .with_context(|| format!("digest the selected @e2e scenarios in '{path}'"))?
        {
            digests.insert(path, digest);
        }
    }
    Ok(digests)
}

/// One path's identity: the path, its entry kind, the executable marker, and
/// its bytes or symlink target, NUL-separated.
pub(super) fn path_digest(path: &str, kind: EntryKind, executable: bool, content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update(b"\0");
    hasher.update(kind_marker(kind, executable));
    hasher.update(b"\0");
    hasher.update(if executable { b"1" } else { b"0" });
    hasher.update(b"\0");
    hasher.update(content);
    digest_hex(hasher)
}

/// The rollup over sorted `path NUL digest` pairs.
pub(super) fn rollup(entries: &BTreeMap<String, String>) -> String {
    rollup_pairs(
        entries
            .iter()
            .map(|(path, digest)| (path.as_str(), digest.as_str())),
    )
}

/// The `e2e` rollup over the package files and the scenario digests together.
/// The two sets are disjoint by construction, so no path hashes twice.
pub(super) fn e2e_rollup(
    files: &BTreeMap<String, String>,
    scenarios: &BTreeMap<String, String>,
) -> String {
    let merged: BTreeMap<&str, &str> = files
        .iter()
        .chain(scenarios.iter())
        .map(|(path, digest)| (path.as_str(), digest.as_str()))
        .collect();
    rollup_pairs(merged.into_iter())
}

fn rollup_pairs<'a>(pairs: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    let mut hasher = Sha256::new();
    for (path, digest) in pairs {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
        hasher.update(digest.as_bytes());
        hasher.update(b"\n");
    }
    digest_hex(hasher)
}

/// The report's abbreviation: the first 12 lowercase hex characters and an
/// ellipsis. The record keeps the full digest.
pub(crate) fn abbreviate(digest: &str) -> String {
    format!("{}…", digest.chars().take(12).collect::<String>())
}

/// The record's kind marker: `l` symlink, `x` executable regular, `f` regular.
fn kind_marker(kind: EntryKind, executable: bool) -> &'static [u8] {
    match kind {
        EntryKind::Symlink => b"l",
        EntryKind::Regular if executable => b"x",
        EntryKind::Regular | EntryKind::Submodule => b"f",
    }
}

fn digest_hex(hasher: Sha256) -> String {
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
