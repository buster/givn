//! The `givn addons list` materialization report (ADR-076).
//!
//! Compares each addon's stored setting against the filesystem effect its
//! registry entry declares, and prints one literal suggestion per difference.
//! This module never writes: it has no `std::fs::write`, no removal call, and
//! no `set_addon`. The suggestions are commands the developer runs.

use std::path::Path;

use anyhow::Result;

use crate::cli::addons::{KNOWN_ADDONS, Materialization};

/// What the filesystem shows for one addon's declared materialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observed {
    /// Every declared path and managed block is present.
    Present,
    /// The first absent declared path, or the damaged block's file.
    Missing(String),
    /// The skill is present in some selected targets but not in these.
    Partial(Vec<String>),
    /// The addon declares nothing that can be checked.
    None,
}

/// One addon's report line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: &'static str,
    pub enabled: bool,
    pub observed: Observed,
    pub suggestion: Option<String>,
}

/// The whole report.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    pub entries: Vec<Entry>,
    pub retired: Vec<(&'static str, &'static str)>,
}

impl Report {
    /// True when an enabled addon's materialization is broken.
    pub fn has_failure(&self) -> bool {
        self.entries.iter().any(|entry| {
            entry.enabled && matches!(entry.observed, Observed::Missing(_) | Observed::Partial(_))
        })
    }
}

/// Build the report for `project_root` from the effective manifest and the
/// selected agent targets, both resolved by the caller.
pub fn build(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
    selected_targets: &[String],
) -> Result<Report> {
    let mut report = Report::default();

    for addon in KNOWN_ADDONS {
        let enabled = manifest
            .addons
            .get(addon.id)
            .unwrap_or(addon.default_enabled);
        let observed = observe(project_root, addon.materialization, selected_targets);
        let suggestion = suggestion_for(addon.id, enabled, &observed);
        report.entries.push(Entry {
            id: addon.id,
            enabled,
            observed,
            suggestion,
        });
    }

    report.retired = retired_keys(project_root)?;
    Ok(report)
}

/// Append the lock note to every suggestion while a change holds the add-on
/// lock, because the suggested commands are refused in that state.
pub fn annotate_lock(report: &mut Report, locked: &[String]) {
    if locked.is_empty() {
        return;
    }
    let suffix = format!(
        "  (refused while change {} holds the add-on lock)",
        locked.join(", ")
    );
    for entry in &mut report.entries {
        if let Some(suggestion) = entry.suggestion.as_mut() {
            suggestion.push_str(&suffix);
        }
    }
}

fn observe(project_root: &Path, materialization: Materialization, targets: &[String]) -> Observed {
    match materialization {
        Materialization::None => Observed::None,
        Materialization::Files(paths) => {
            match paths.iter().find(|path| !project_root.join(path).exists()) {
                Some(path) => Observed::Missing((*path).to_string()),
                None => Observed::Present,
            }
        }
        Materialization::ManagedBlock { file, block } => {
            match managed_block_intact(project_root, file, block) {
                Ok(true) => Observed::Present,
                Ok(false) => Observed::Missing(file.to_string()),
                Err(_) => Observed::Missing(file.to_string()),
            }
        }
        Materialization::FilesAndSkill { files, skill } => {
            // A declared file is checked before the skill: losing it is the
            // stronger failure, and the skill directories alone cannot show it.
            if let Some(path) = own_files(files).find(|path| !project_root.join(path).exists()) {
                return Observed::Missing(path.to_string());
            }
            let absent: Vec<String> = targets
                .iter()
                .map(|target| skill_path(target, skill))
                .filter(|path| !project_root.join(path).exists())
                .collect();
            if absent.is_empty() {
                Observed::Present
            } else if absent.len() == targets.len() {
                Observed::Missing(skill_path(&targets[0], skill))
            } else {
                Observed::Partial(absent)
            }
        }
    }
}

/// The declared paths this addon reports on: every path it declares, minus
/// the ones another registry entry also declares. A shared document belongs to
/// the addon that scaffolds it first, so a project that kept it for that addon
/// is not reported as broken for the other.
fn own_files(files: &'static [&'static str]) -> impl Iterator<Item = &'static str> {
    files.iter().copied().filter(|path| {
        KNOWN_ADDONS
            .iter()
            .filter(|addon| declares(addon, path))
            .count()
            == 1
    })
}

fn declares(addon: &crate::cli::addons::AddonSpec, path: &str) -> bool {
    match addon.materialization {
        Materialization::Files(files) | Materialization::FilesAndSkill { files, .. } => {
            files.contains(&path)
        }
        Materialization::ManagedBlock { .. } | Materialization::None => false,
    }
}

fn skill_path(target: &str, skill: &str) -> String {
    format!(".{target}/skills/{skill}")
}

fn managed_block_intact(project_root: &Path, file: &str, block: &str) -> Result<bool> {
    let content = std::fs::read_to_string(project_root.join(file))?;
    let begin = format!("<!-- givn:begin:{block} -->");
    let end = format!("<!-- givn:end:{block} -->");
    let begins: Vec<usize> = content.match_indices(&begin).map(|(i, _)| i).collect();
    let ends: Vec<usize> = content.match_indices(&end).map(|(i, _)| i).collect();
    Ok(begins.len() == 1 && ends.len() == 1 && begins[0] < ends[0])
}

fn suggestion_for(id: &str, enabled: bool, observed: &Observed) -> Option<String> {
    match (enabled, observed) {
        (true, Observed::Missing(_)) => Some(format!("givn addons enable {id}")),
        (true, Observed::Partial(_)) => Some("givn skills sync".to_string()),
        (false, Observed::Present) => Some(format!("givn addons disable {id}")),
        _ => None,
    }
}

/// Retired addon keys that are still present in `givn/config.yaml`.
fn retired_keys(project_root: &Path) -> Result<Vec<(&'static str, &'static str)>> {
    let path = project_root.join("givn/config.yaml");
    let Ok(content) = std::fs::read_to_string(&path) else {
        return Ok(Vec::new());
    };
    let Ok(document) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&content) else {
        return Ok(Vec::new());
    };
    let Some(addons) = document.get("addons").and_then(|value| value.as_mapping()) else {
        return Ok(Vec::new());
    };
    Ok(crate::config::manifest::Addons::RETIRED
        .into_iter()
        .filter(|(retired, _)| addons.contains_key(*retired))
        .collect())
}

/// The `artifacts:` continuation line for one observed state.
pub fn observed_line(observed: &Observed) -> String {
    match observed {
        Observed::Present => "              artifacts: present".to_string(),
        Observed::Missing(path) => format!("              artifacts: missing {path}"),
        Observed::Partial(paths) => format!(
            "              artifacts: partial ({} missing)",
            paths.join(", ")
        ),
        Observed::None => "              artifacts: none".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declared_paths() -> Vec<&'static str> {
        KNOWN_ADDONS
            .iter()
            .flat_map(|addon| match addon.materialization {
                Materialization::Files(files) | Materialization::FilesAndSkill { files, .. } => {
                    files.to_vec()
                }
                Materialization::ManagedBlock { .. } | Materialization::None => Vec::new(),
            })
            .collect()
    }

    #[test]
    fn a_shared_path_belongs_to_exactly_one_addon() {
        // `docs/glossary.md` is scaffolded by both arc42 and domain_modeling;
        // the report must attribute it to one so the other is never `partial`.
        let paths = declared_paths();
        let mut shared: Vec<&str> = Vec::new();
        for (index, path) in paths.iter().enumerate() {
            if paths[index + 1..].contains(path) && !shared.contains(path) {
                shared.push(path);
            }
        }
        let glossary = own_files(&["docs/glossary.md"]).collect::<Vec<_>>();
        assert!(
            glossary.is_empty(),
            "shared paths must not count against an addon: {shared:?}"
        );
        let arc42 = KNOWN_ADDONS
            .iter()
            .find(|addon| addon.id == "arc42")
            .expect("arc42 registered");
        assert!(
            declares(arc42, "docs/glossary.md"),
            "arc42 still scaffolds and keeps the glossary"
        );
    }

    #[test]
    fn the_report_module_has_no_write_path() {
        // Only the production half: the assertions below name the same
        // symbols they forbid.
        let source = include_str!("addon_report.rs");
        let production: String = source
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default()
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("//")
            })
            .collect::<Vec<_>>()
            .join("\n");
        for symbol in ["fs::write", "remove_file", "remove_dir_all", "set_addon"] {
            assert!(
                !production.contains(symbol),
                "the report must not mutate the project: found {symbol}"
            );
        }
    }
}
