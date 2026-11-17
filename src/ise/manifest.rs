//! The `ise-manifest/v1` manifest.
//!
//! One manifest carries the placement counts, the three path inventories, the
//! checksum maps, and the two rollups. The checksum rule lives here so
//! preparation and comparison hash the same shape.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::declaration::SpecificationDeclaration;
use crate::scm::RevisionEntry;

/// The manifest content scope; it names the manifest's shape, not the change
/// verification receipt.
pub(super) const CONTENT_SCOPE: &str = "ise-manifest/v1";

/// One prepared spec copy's recorded evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Manifest {
    pub scope: String,
    pub revision: String,
    pub removed_paths: Vec<String>,
    pub specification_files: Vec<String>,
    pub source_kept_files: Vec<String>,
    pub end_to_end_files: Vec<String>,
    pub counts: Counts,
    pub checksums: Checksums,
    pub specification_checksum: String,
    pub end_to_end_checksum: String,
    pub copy_directory: String,
    pub givn_version: String,
}

impl Manifest {
    /// Serialize the manifest with the field order the `ise-manifest/v1`
    /// scope names.
    fn yaml(&self) -> Result<String> {
        serde_yaml_ng::to_string(self).context("serialize the manifest")
    }

    /// Write the manifest to a new path. An existing or unwritable target is
    /// an error naming the path; the writer never replaces a file.
    pub(super) fn write(&self, path: &Path) -> Result<()> {
        let text = self.yaml()?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .with_context(|| format!("write the manifest '{}'", path.display()))?;
        file.write_all(text.as_bytes())
            .with_context(|| format!("write the manifest '{}'", path.display()))
    }
}

/// The five placement counts the manifest records.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Counts {
    pub specification_files: usize,
    pub source_kept: usize,
    pub source_removed: usize,
    pub outside_spec_copy: usize,
    pub files_beside_removed_paths: usize,
}

/// The three checksum maps, one checksum per recorded path.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Checksums {
    pub specification: BTreeMap<String, String>,
    pub end_to_end: BTreeMap<String, String>,
    pub scenarios: BTreeMap<String, String>,
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

/// The rollup over sorted `path NUL checksum` pairs.
pub(super) fn rollup(entries: &BTreeMap<String, String>) -> String {
    let mut hasher = Sha256::new();
    for (path, digest) in entries {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
        hasher.update(digest.as_bytes());
        hasher.update(b"\n");
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The report's abbreviation: the first 12 lowercase hex characters and an
/// ellipsis. The manifest keeps the full value.
pub(crate) fn abbreviate(digest: &str) -> String {
    format!("{}…", digest.chars().take(12).collect::<String>())
}
