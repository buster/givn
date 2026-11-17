//! The `ise-manifest/v1` manifest.
//!
//! One manifest carries the placement counts, the three path inventories, the
//! checksum maps, and the two rollups. The checksum rule lives here so
//! preparation and comparison hash the same shape.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::declaration::SpecificationDeclaration;
use crate::scm::{EntryKind, RevisionEntry};

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

    /// Read the one `ise-manifest/v1` shape. The scope is exact, every listed
    /// field is required, no unknown field is accepted, and every checksum and
    /// rollup is 64 lowercase hex characters. An older scope is refused by
    /// name and never migrated.
    pub(super) fn read(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read the manifest '{}'", path.display()))?;
        let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
            .with_context(|| format!("the manifest '{}' is not readable YAML", path.display()))?;
        let manifest: Self = serde_yaml_ng::from_value(document).with_context(|| {
            format!(
                "the manifest '{}' does not match ise-manifest/v1",
                path.display()
            )
        })?;
        if manifest.scope != CONTENT_SCOPE {
            bail!(
                "the manifest '{}' records the scope '{}'; the reader requires '{CONTENT_SCOPE}'",
                path.display(),
                manifest.scope
            );
        }
        manifest.require_checksums(path)?;
        Ok(manifest)
    }

    /// Refuse a checksum or rollup that is not 64 lowercase hex characters.
    fn require_checksums(&self, path: &Path) -> Result<()> {
        for (recorded_path, checksum) in &self.checksums.specification {
            require_checksum(
                path,
                &format!("the Specification checksum of '{recorded_path}'"),
                checksum,
            )?;
        }
        for (recorded_path, checksum) in &self.checksums.end_to_end {
            require_checksum(
                path,
                &format!("the End-to-end checksum of '{recorded_path}'"),
                checksum,
            )?;
        }
        for (recorded_path, checksum) in &self.checksums.scenarios {
            require_checksum(
                path,
                &format!("the scenario checksum of '{recorded_path}'"),
                checksum,
            )?;
        }
        require_checksum(path, "specification_checksum", &self.specification_checksum)?;
        require_checksum(path, "end_to_end_checksum", &self.end_to_end_checksum)
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

/// Refuse one recorded checksum value that is not 64 lowercase hex characters.
fn require_checksum(manifest: &Path, what: &str, value: &str) -> Result<()> {
    let is_hex = value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if is_hex {
        return Ok(());
    }
    bail!(
        "the manifest '{}' records {what} '{value}'; every checksum is 64 lowercase hex characters",
        manifest.display()
    )
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

/// One path's checksum: `sha256(path NUL kind NUL marker NUL content)`.
///
/// The kind is `f` (regular), `x` (executable regular), `l` (symlink), or `g`
/// (repository link); the marker is `1` or `0`. A repository link's content is
/// the recorded commit id, so the link and a regular file holding that text
/// differ.
pub(super) fn path_checksum(
    path: &str,
    kind: EntryKind,
    executable: bool,
    content: &[u8],
) -> String {
    let kind = match kind {
        EntryKind::Regular if executable => "x",
        EntryKind::Regular => "f",
        EntryKind::Symlink => "l",
        EntryKind::Submodule => "g",
    };
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update(b"\0");
    hasher.update(kind.as_bytes());
    hasher.update(b"\0");
    hasher.update(if executable { b"1" } else { b"0" });
    hasher.update(b"\0");
    hasher.update(content);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The rollup over sorted `path NUL checksum LF` pairs.
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

/// The End-to-end rollup: the union of the content map and the scenario map,
/// keyed by path, with the scenario checksum winning when one path is in both.
pub(super) fn end_to_end_rollup(
    end_to_end: &BTreeMap<String, String>,
    scenarios: &BTreeMap<String, String>,
) -> String {
    let mut union = end_to_end.clone();
    union.extend(
        scenarios
            .iter()
            .map(|(path, digest)| (path.clone(), digest.clone())),
    );
    rollup(&union)
}

/// The report's abbreviation: the first 12 lowercase hex characters and an
/// ellipsis. The manifest keeps the full value.
pub(crate) fn abbreviate(digest: &str) -> String {
    format!("{}…", digest.chars().take(12).collect::<String>())
}
