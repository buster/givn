//! Add-on configuration lock recorded when a change starts.
//!
//! New changes record the effective add-on values in
//! `givn/changes/<id>/config-lock.yaml`. Static review and archive compare the
//! current project configuration with that record. Changes created before the
//! lock existed have no file and are left unaffected.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const LOCK_FILE: &str = "config-lock.yaml";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct AddonLock {
    pub version: u32,
    pub addons: BTreeMap<String, bool>,
    #[serde(default)]
    pub overrides: Vec<AddonOverride>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AddonOverride {
    pub addon: String,
    pub from: bool,
    pub to: bool,
    pub reason: String,
    pub timestamp: String,
}

/// Write a new lock for a freshly created change.
pub fn write_for_change(
    change_dir: &Path,
    manifest: &crate::config::manifest::Manifest,
) -> Result<()> {
    let lock = AddonLock {
        version: 1,
        addons: effective_addons(manifest),
        overrides: Vec::new(),
    };
    write(change_dir, &lock)
}

/// The effective add-on values in registry order.
pub fn effective_addons(manifest: &crate::config::manifest::Manifest) -> BTreeMap<String, bool> {
    crate::config::manifest::Addons::NAMES
        .iter()
        .map(|name| {
            (
                (*name).to_string(),
                manifest.addons.get(name).unwrap_or(true),
            )
        })
        .collect()
}

/// The add-on values a fresh init or a forced re-init would write: each
/// addon's declared default, or `false` for every addon under `--no-addons`.
pub fn proposed_addons(
    no_addons: bool,
    preserved: Option<&[(String, bool)]>,
) -> BTreeMap<String, bool> {
    let mut proposed: BTreeMap<String, bool> = crate::config::manifest::Addons::NAMES
        .iter()
        .map(|name| {
            (
                (*name).to_string(),
                !no_addons
                    && crate::config::addons::AddonSpec::declared_default(name).unwrap_or(false),
            )
        })
        .collect();
    if let Some(preserved) = preserved {
        for (name, enabled) in preserved {
            proposed.insert(name.clone(), *enabled);
        }
    }
    proposed
}

/// Differences between a recorded baseline and the current effective values.
pub fn differences(
    recorded: &BTreeMap<String, bool>,
    current: &BTreeMap<String, bool>,
) -> Vec<String> {
    let mut changes = Vec::new();
    for (name, before) in recorded {
        let after = current.get(name).copied().unwrap_or(*before);
        if after != *before {
            changes.push(format!(
                "  {name}: {} -> {}",
                if *before { "enabled" } else { "disabled" },
                if after { "enabled" } else { "disabled" }
            ));
        }
    }
    changes
}

/// Read a change's lock, or `None` for a change created before locking.
pub fn read(change_dir: &Path) -> Result<Option<AddonLock>> {
    let path = change_dir.join(LOCK_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("read add-on lock '{}'", path.display()))?;
    let lock: AddonLock = serde_yaml_ng::from_str(&content)
        .with_context(|| format!("parse add-on lock '{}'", path.display()))?;
    if lock.version != 1 {
        bail!(
            "unsupported add-on lock version {} in '{}'",
            lock.version,
            path.display()
        );
    }
    Ok(Some(lock))
}

/// Whether the change's recorded add-on lock has `ise` effective.
///
/// The change's lock is the authority, not the current project manifest: a
/// change with no lock, or with `ise: false` in its lock, gains no boundary
/// gate even if the project later enables `ise`.
pub fn ise_effective_for_change(change_dir: &Path) -> Result<bool> {
    Ok(read(change_dir)?.is_some_and(|lock| lock.addons.get("ise").copied().unwrap_or(false)))
}

fn write(change_dir: &Path, lock: &AddonLock) -> Result<()> {
    let path = change_dir.join(LOCK_FILE);
    let content = serde_yaml_ng::to_string(lock)
        .with_context(|| format!("serialize add-on lock '{}'", path.display()))?;
    std::fs::write(&path, content)
        .with_context(|| format!("write add-on lock '{}'", path.display()))
}

/// Refuse when the current effective configuration differs from the change's
/// recorded lock.
pub fn check(project_root: &Path, change_id: &str) -> Result<()> {
    let change_dir = crate::state::resolve_change_dir(project_root, change_id)?;
    let Some(lock) = read(&change_dir)? else {
        return Ok(());
    };
    let manifest = crate::config::overlay::load_effective()?;
    let current = effective_addons(&manifest);
    let changes = differences(&lock.addons, &current);
    if !changes.is_empty() {
        bail!(
            "add-on configuration changed during change '{change_id}':\n{}\n\
             Recorded in {}.\n\
             Add-on configuration is operator-owned. Restore the recorded values, or run the \
             add-on command with `--operator --reason \"<reason>\"` on an interactive terminal. \
             A missing dependency must be installed in an isolated environment or reported as an \
             environment failure; do not disable its gate.",
            changes.join("\n"),
            change_dir.join(LOCK_FILE).display()
        );
    }
    Ok(())
}

/// Active changes that carry an add-on lock.
pub fn locked_active_changes(project_root: &Path) -> Result<Vec<(String, PathBuf)>> {
    let manifest = crate::config::overlay::load_effective()?;
    let mut locked = Vec::new();
    for change in crate::state::list_changes(project_root, &manifest) {
        let dir = project_root.join("givn/changes").join(&change.id);
        if read(&dir)?.is_some() {
            locked.push((change.id, dir));
        }
    }
    Ok(locked)
}

/// Record an operator override after the add-on setting was changed.
pub fn record_override(
    change_dir: &Path,
    addon: &str,
    from: bool,
    to: bool,
    reason: &str,
) -> Result<()> {
    let Some(mut lock) = read(change_dir)? else {
        bail!(
            "change '{}' has no add-on lock; the override cannot be recorded",
            change_dir.display()
        );
    };
    lock.overrides.push(AddonOverride {
        addon: addon.to_string(),
        from,
        to,
        reason: reason.to_string(),
        timestamp: crate::subprocess::local_timestamp(),
    });
    lock.addons.insert(addon.to_string(), to);
    write(change_dir, &lock)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (
        tempfile::TempDir,
        PathBuf,
        crate::config::manifest::Manifest,
    ) {
        let directory = tempfile::tempdir().unwrap();
        let change = directory.path().join("givn/changes/locked");
        std::fs::create_dir_all(&change).unwrap();
        std::fs::create_dir_all(directory.path().join("givn")).unwrap();
        std::fs::write(
            directory.path().join("givn/config.yaml"),
            "givn_config_version: \"0.9.16\"\n",
        )
        .unwrap();
        let manifest = crate::config::manifest::Manifest::load_embedded().unwrap();
        (directory, change, manifest)
    }

    #[test]
    fn a_new_lock_records_every_effective_addon() {
        let (_dir, change, manifest) = fixture();
        write_for_change(&change, &manifest).unwrap();
        let lock = read(&change).unwrap().expect("lock");
        assert_eq!(lock.version, 1);
        assert_eq!(lock.addons.len(), 10);
        assert_eq!(lock.addons.get("ise"), Some(&false));
        assert!(
            lock.addons
                .iter()
                .filter(|(name, _)| name.as_str() != "ise")
                .all(|(_, enabled)| *enabled)
        );
        assert!(lock.overrides.is_empty());
    }

    #[test]
    fn a_fresh_proposal_follows_each_declared_default() {
        let proposed = proposed_addons(false, None);
        assert_eq!(proposed.get("ise"), Some(&false));
        assert_eq!(proposed.get("outline"), Some(&true));
        let none = proposed_addons(true, None);
        assert_eq!(none.len(), 10);
        assert!(none.values().all(|enabled| !enabled));
    }

    #[test]
    fn a_lock_is_optional_for_old_changes() {
        let (_dir, change, _manifest) = fixture();
        assert!(read(&change).unwrap().is_none());
    }

    #[test]
    fn an_override_updates_the_baseline_and_records_the_reason() {
        let (_dir, change, manifest) = fixture();
        write_for_change(&change, &manifest).unwrap();
        record_override(&change, "coverage", true, false, "dependency unavailable").unwrap();
        let lock = read(&change).unwrap().expect("lock");
        assert_eq!(lock.addons.get("coverage"), Some(&false));
        let entry = lock.overrides.last().expect("override");
        assert_eq!(entry.addon, "coverage");
        assert!(entry.from);
        assert!(!entry.to);
        assert_eq!(entry.reason, "dependency unavailable");
    }
}
