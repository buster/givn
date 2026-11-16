//! The development record's on-disk format, mirrored for fixtures.
//!
//! The package links no product crate, so a scenario that needs a recorded
//! history writes the record itself. This module is the record's shape as the
//! product stores it (`givn/history/events.jsonl`): one JSON object per line,
//! `schema`-versioned, ids minted at append time. The shape is a file-format
//! literal, not behaviour: the report under test reads the file, so a drift in
//! this mirror fails the scenario that asserts the report's content.
//!
//! The product owns the writer (`src/history.rs`); this mirror exists so the
//! acceptance package can seed a history without executing the product first.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The record's schema version.
pub const SCHEMA: u32 = 1;

/// The outcome a harness did not report.
pub const OUTCOME_UNKNOWN: &str = "unknown";

/// The bounded content field.
pub const CONTENT_FIELD_LIMIT: usize = 64 * 1024;

/// One record line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub schema: u32,
    pub id: u64,
    pub at: String,
    pub scope: Scope,
    pub source: String,
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// The one scope an entry belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Scope {
    Change { id: String },
    Ideation { id: String },
    Project { id: String },
}

impl HistoryEntry {
    /// One entry the agent intake reports: no `id` yet (the store mints it),
    /// `agent` as its source, and `unknown` until the caller sets the outcome.
    pub fn reported(scope: Scope, kind: &str, name: &str, at: &str) -> Self {
        HistoryEntry {
            schema: SCHEMA,
            id: 0,
            at: at.to_string(),
            scope,
            source: "agent".to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
            label: None,
            duration_ms: None,
            outcome: OUTCOME_UNKNOWN.to_string(),
            parent: None,
            session: None,
            data: None,
        }
    }
}

/// The project's `givn/history/events.jsonl`: append-only, ids minted at
/// append time from the current last line.
#[derive(Debug, Clone)]
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    pub fn at(path: PathBuf) -> Self {
        HistoryStore { path }
    }

    /// Append entries in order, minting each `id` and resolving `parent` from
    /// the batch index of the entry it is nested in.
    pub fn append_linked(
        &self,
        entries: &mut [HistoryEntry],
        parents: &[Option<usize>],
    ) -> std::io::Result<u64> {
        use std::io::{Read as _, Write as _};

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.path)?;
        file.lock()?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let first_id = last_id(&text) + 1;
        for (offset, entry) in entries.iter_mut().enumerate() {
            entry.id = first_id + offset as u64;
            if let Some(parent) = parents.get(offset).copied().flatten() {
                entry.parent = Some(first_id + parent as u64);
            }
            let line = serde_json::to_string(entry).map_err(std::io::Error::other)?;
            writeln!(file, "{line}")?;
        }
        Ok(first_id)
    }

    /// Append entries with no parent links.
    pub fn append(&self, entries: &mut [HistoryEntry]) -> std::io::Result<u64> {
        let parents = vec![None; entries.len()];
        self.append_linked(entries, &parents)
    }
}

/// The id of the store's current last line (`0` when empty).
fn last_id(text: &str) -> u64 {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| serde_json::from_str::<HistoryEntry>(line).ok())
        .map(|entry| entry.id)
        .unwrap_or(0)
}
