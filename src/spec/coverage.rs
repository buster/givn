//! Machine-checkable specification and migration coverage evidence.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use gherkin::GherkinEnv;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoverageEntry {
    pub feature_path: String,
    pub usecase: String,
    pub capability: String,
    pub scenario: String,
    pub behavior_hash: String,
    pub e2e: bool,
    pub interaction: Option<String>,
    pub personas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoverageSnapshot {
    pub schema: u32,
    pub entries: Vec<CoverageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoverageComparison {
    pub schema: u32,
    pub before_entries: usize,
    pub after_entries: usize,
    pub before_e2e: usize,
    pub after_e2e: usize,
    pub before_interactions: usize,
    pub after_interactions: usize,
    pub missing_behavior: Vec<String>,
    pub lost_e2e: Vec<String>,
    pub lower_interaction_coverage: bool,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceCoverageComparison {
    pub schema: u32,
    pub before: SourceCoverage,
    pub after: SourceCoverage,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SourceCoverage {
    pub lines_covered: u64,
    pub lines_valid: u64,
    pub branches_covered: u64,
    pub branches_valid: u64,
}

pub fn snapshot(root: &Path) -> Result<CoverageSnapshot> {
    let specs_root = root.join("givn/specs");
    let mut entries = Vec::new();
    for feature_path in crate::gherkin::parse::collect_feature_files(&specs_root) {
        let feature = gherkin::Feature::parse_path(&feature_path, GherkinEnv::default()).map_err(
            |error| anyhow::anyhow!("failed to parse {}: {error}", feature_path.display()),
        )?;
        let relative = feature_path.strip_prefix(&specs_root)?;
        let components: Vec<_> = relative.components().collect();
        let (usecase, capability, personas, interactions) = if components
            .first()
            .is_some_and(|part| part.as_os_str() == "fragments")
        {
            let fragment_id = parse_fragment_id(&specs_root.join("fragments/fragment.md"))?;
            (
                format!("fragment:{fragment_id}"),
                feature_path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string(),
                Vec::new(),
                BTreeMap::new(),
            )
        } else {
            let usecase_name = components
                .first()
                .and_then(|part| part.as_os_str().to_str())
                .unwrap_or_default()
                .to_string();
            let usecase_path = specs_root.join(&usecase_name).join("usecase.md");
            let doc = crate::gherkin::usecase_md::parse_usecase_md(
                &std::fs::read_to_string(&usecase_path).with_context(|| {
                    format!("missing use-case document {}", usecase_path.display())
                })?,
            )?;
            let interactions = doc
                .interactions
                .into_iter()
                .map(|interaction| (interaction.e2e_scenario, interaction.capability))
                .collect();
            (
                usecase_name,
                feature_path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string(),
                doc.personas,
                interactions,
            )
        };
        let feature_path = relative.to_string_lossy().replace('\\', "/");
        for scenario in feature.scenarios {
            let behavior_hash = behavior_hash(&scenario);
            let e2e = scenario.tags.iter().any(|tag| tag == "e2e");
            let interaction = interactions.get(&scenario.name).cloned();
            entries.push(CoverageEntry {
                feature_path: feature_path.clone(),
                usecase: usecase.clone(),
                capability: capability.clone(),
                scenario: scenario.name,
                behavior_hash,
                e2e,
                interaction,
                personas: personas.clone(),
            });
        }
    }
    entries.sort_by(|left, right| {
        left.behavior_hash
            .cmp(&right.behavior_hash)
            .then_with(|| left.scenario.cmp(&right.scenario))
            .then_with(|| left.feature_path.cmp(&right.feature_path))
    });
    Ok(CoverageSnapshot { schema: 1, entries })
}

pub fn compare(before: &CoverageSnapshot, after: &CoverageSnapshot) -> CoverageComparison {
    let mut after_by_identity: BTreeMap<(&str, &str), Vec<&CoverageEntry>> = BTreeMap::new();
    for entry in &after.entries {
        after_by_identity
            .entry((&entry.behavior_hash, entry.scenario.as_str()))
            .or_default()
            .push(entry);
    }
    let mut missing_behavior = Vec::new();
    let mut lost_e2e = Vec::new();
    let mut used: BTreeSet<(&str, &str, &str, usize)> = BTreeSet::new();
    for entry in &before.entries {
        let identity = (entry.behavior_hash.as_str(), entry.scenario.as_str());
        let Some(candidates) = after_by_identity.get(&identity) else {
            missing_behavior.push(format!("{} — {}", entry.feature_path, entry.scenario));
            continue;
        };
        let Some((index, current)) = candidates.iter().enumerate().find(|(index, current)| {
            !used.contains(&(
                entry.behavior_hash.as_str(),
                entry.scenario.as_str(),
                current.feature_path.as_str(),
                *index,
            ))
        }) else {
            missing_behavior.push(format!("{} — {}", entry.feature_path, entry.scenario));
            continue;
        };
        used.insert((
            entry.behavior_hash.as_str(),
            entry.scenario.as_str(),
            current.feature_path.as_str(),
            index,
        ));
        if entry.e2e && !current.e2e {
            lost_e2e.push(format!("{} — {}", entry.feature_path, entry.scenario));
        }
    }
    let before_interactions = before
        .entries
        .iter()
        .filter(|entry| entry.interaction.is_some())
        .count();
    let after_interactions = after
        .entries
        .iter()
        .filter(|entry| entry.interaction.is_some())
        .count();
    let before_e2e = before.entries.iter().filter(|entry| entry.e2e).count();
    let after_e2e = after.entries.iter().filter(|entry| entry.e2e).count();
    let passed = missing_behavior.is_empty()
        && lost_e2e.is_empty()
        && after_e2e >= before_e2e
        && after_interactions >= before_interactions;
    CoverageComparison {
        schema: 1,
        before_entries: before.entries.len(),
        after_entries: after.entries.len(),
        before_e2e,
        after_e2e,
        before_interactions,
        after_interactions,
        missing_behavior,
        lost_e2e,
        lower_interaction_coverage: after_interactions < before_interactions,
        passed,
    }
}

pub fn parse_source_coverage(path: &Path) -> Result<SourceCoverage> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read source coverage evidence {}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .with_context(|| format!("source coverage evidence is not JSON: {}", path.display()))?;
    Ok(SourceCoverage {
        lines_covered: number(&value, "lines_covered")?,
        lines_valid: number(&value, "lines_valid")?,
        branches_covered: number(&value, "branches_covered")?,
        branches_valid: number(&value, "branches_valid")?,
    })
}

pub fn compare_source(before: SourceCoverage, after: SourceCoverage) -> SourceCoverageComparison {
    let passed = after.lines_covered * before.lines_valid
        >= before.lines_covered * after.lines_valid
        && after.branches_covered * before.branches_valid
            >= before.branches_covered * after.branches_valid;
    SourceCoverageComparison {
        schema: 1,
        before,
        after,
        passed,
    }
}

pub fn check_migration(root: &Path, migration_id: &str) -> Result<()> {
    let migration_dir = root.join("givn/changes").join(migration_id);
    if !migration_dir.exists() {
        bail!("migration change '{migration_id}' not found");
    }
    let before_path = migration_dir.join("coverage-before.json");
    let after_path = migration_dir.join("coverage-after.json");
    let before: CoverageSnapshot = serde_json::from_str(&std::fs::read_to_string(&before_path)?)?;
    let after: CoverageSnapshot = serde_json::from_str(&std::fs::read_to_string(&after_path)?)?;
    let retired = read_retired_entries(&migration_dir)?;
    let mut comparison = compare(&before, &after);
    comparison
        .missing_behavior
        .retain(|entry| !retired.contains(entry));
    for entry in &before.entries {
        let identity = format!("{} — {}", entry.feature_path, entry.scenario);
        if retired.contains(&identity) {
            if entry.e2e {
                comparison.before_e2e = comparison.before_e2e.saturating_sub(1);
            }
            if entry.interaction.is_some() {
                comparison.before_interactions = comparison.before_interactions.saturating_sub(1);
            }
        }
    }
    comparison.lost_e2e.retain(|entry| !retired.contains(entry));
    comparison.passed = comparison.missing_behavior.is_empty()
        && comparison.lost_e2e.is_empty()
        && comparison.after_e2e >= comparison.before_e2e
        && comparison.after_interactions >= comparison.before_interactions;
    std::fs::write(
        migration_dir.join("coverage-comparison.json"),
        serde_json::to_string_pretty(&comparison)?,
    )?;

    let source_comparison = match (
        find_source_evidence(&migration_dir, "before"),
        find_source_evidence(&migration_dir, "after"),
    ) {
        (Some(before), Some(after)) => Some(compare_source(
            parse_source_coverage(&before)?,
            parse_source_coverage(&after)?,
        )),
        _ => None,
    };
    if let Some(comparison) = &source_comparison {
        std::fs::write(
            migration_dir.join("source-coverage-comparison.json"),
            serde_json::to_string_pretty(comparison)?,
        )?;
    }
    if !comparison.passed
        || source_comparison
            .as_ref()
            .is_some_and(|comparison| !comparison.passed)
    {
        bail!("migration coverage regression; see coverage-comparison.json");
    }
    println!(
        "migration coverage passed: {} behavior entries, {} @e2e",
        after.entries.len(),
        after.entries.iter().filter(|entry| entry.e2e).count()
    );
    Ok(())
}

fn find_source_evidence(dir: &Path, suffix: &str) -> Option<PathBuf> {
    [
        dir.join(format!("source-coverage-{suffix}.json")),
        dir.join(format!("coverage-source-{suffix}.json")),
    ]
    .into_iter()
    .find(|path| path.exists())
}

fn read_retired_entries(dir: &Path) -> Result<BTreeSet<String>> {
    let path = dir.join("usecase-migration.md");
    if !path.exists() {
        return Ok(BTreeSet::new());
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read migration ledger {}", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|line| {
            let columns: Vec<_> = line
                .split('|')
                .map(str::trim)
                .filter(|column| !column.is_empty())
                .collect();
            (columns.len() >= 3 && columns[2] == "RETIRE").then(|| columns[0].to_string())
        })
        .collect())
}

fn parse_fragment_id(path: &Path) -> Result<String> {
    let doc = crate::gherkin::usecase_md::parse_fragment_md(&std::fs::read_to_string(path)?)?;
    Ok(doc.id)
}

fn behavior_hash(scenario: &gherkin::Scenario) -> String {
    let mut canonical = scenario.keyword.clone();
    canonical.push('|');
    canonical.push_str(&scenario.name);
    for step in &scenario.steps {
        canonical.push('|');
        canonical.push_str(step.keyword.trim());
        canonical.push('|');
        canonical.push_str(&step.value);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn number(value: &serde_json::Value, key: &str) -> Result<u64> {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| anyhow::anyhow!("source coverage evidence is missing numeric '{key}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moved_behavior_matches_without_path_identity() {
        let before = CoverageSnapshot {
            schema: 1,
            entries: vec![CoverageEntry {
                feature_path: "old/cap.feature".into(),
                usecase: "old".into(),
                capability: "cap".into(),
                scenario: "Runs".into(),
                behavior_hash: "same".into(),
                e2e: true,
                interaction: Some("Runs".into()),
                personas: vec![],
            }],
        };
        let mut after = before.clone();
        after.entries[0].feature_path = "new/cap.feature".into();
        after.entries[0].usecase = "new".into();
        assert!(compare(&before, &after).passed);
    }

    #[test]
    fn lost_behavior_and_e2e_block() {
        let before = CoverageSnapshot {
            schema: 1,
            entries: vec![CoverageEntry {
                feature_path: "old/cap.feature".into(),
                usecase: "old".into(),
                capability: "cap".into(),
                scenario: "Runs".into(),
                behavior_hash: "same".into(),
                e2e: true,
                interaction: Some("Runs".into()),
                personas: vec![],
            }],
        };
        let after = CoverageSnapshot {
            schema: 1,
            entries: vec![],
        };
        let comparison = compare(&before, &after);
        assert!(!comparison.passed);
        assert_eq!(comparison.missing_behavior.len(), 1);
    }

    #[test]
    fn lost_e2e_evidence_blocks_without_missing_behavior() {
        let before = CoverageSnapshot {
            schema: 1,
            entries: vec![CoverageEntry {
                feature_path: "old/cap.feature".into(),
                usecase: "old".into(),
                capability: "cap".into(),
                scenario: "Runs".into(),
                behavior_hash: "same".into(),
                e2e: true,
                interaction: Some("Runs".into()),
                personas: vec![],
            }],
        };
        let mut after = before.clone();
        after.entries[0].e2e = false;
        let comparison = compare(&before, &after);
        assert!(!comparison.passed);
        assert!(comparison.missing_behavior.is_empty());
        assert_eq!(comparison.lost_e2e.len(), 1);
    }

    #[test]
    fn lower_interaction_coverage_blocks_when_behavior_remains() {
        let before = CoverageSnapshot {
            schema: 1,
            entries: vec![CoverageEntry {
                feature_path: "old/cap.feature".into(),
                usecase: "old".into(),
                capability: "cap".into(),
                scenario: "Runs".into(),
                behavior_hash: "same".into(),
                e2e: true,
                interaction: Some("Runs".into()),
                personas: vec![],
            }],
        };
        let mut after = before.clone();
        after.entries[0].interaction = None;
        let comparison = compare(&before, &after);
        assert!(!comparison.passed);
        assert!(comparison.missing_behavior.is_empty());
        assert!(comparison.lower_interaction_coverage);
    }

    #[test]
    fn lower_line_coverage_blocks() {
        let comparison = compare_source(
            SourceCoverage {
                lines_covered: 90,
                lines_valid: 100,
                branches_covered: 9,
                branches_valid: 10,
            },
            SourceCoverage {
                lines_covered: 89,
                lines_valid: 100,
                branches_covered: 9,
                branches_valid: 10,
            },
        );
        assert!(!comparison.passed);
    }

    #[test]
    fn lower_branch_coverage_blocks() {
        let comparison = compare_source(
            SourceCoverage {
                lines_covered: 90,
                lines_valid: 100,
                branches_covered: 9,
                branches_valid: 10,
            },
            SourceCoverage {
                lines_covered: 90,
                lines_valid: 100,
                branches_covered: 8,
                branches_valid: 10,
            },
        );
        assert!(!comparison.passed);
    }

    #[test]
    fn migration_ledger_identifies_explicit_retirements() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(
            directory.path().join("usecase-migration.md"),
            "| Old entry | Old owner | Operation | New owner |\n|---|---|---|---|\n| old/capability.feature — Preserved behavior | old | RETIRE | none |\n",
        )
        .unwrap();
        let retired = read_retired_entries(directory.path()).unwrap();
        assert!(retired.contains("old/capability.feature — Preserved behavior"));
    }

    #[test]
    fn explicit_retirement_allows_missing_behavior() {
        let directory = tempfile::tempdir().unwrap();
        let migration = directory.path().join("givn/changes/migration-test");
        std::fs::create_dir_all(&migration).unwrap();
        let before = CoverageSnapshot {
            schema: 1,
            entries: vec![CoverageEntry {
                feature_path: "old/capability.feature".into(),
                usecase: "old".into(),
                capability: "capability".into(),
                scenario: "Preserved behavior".into(),
                behavior_hash: "behavior-hash".into(),
                e2e: true,
                interaction: Some("preserve behavior".into()),
                personas: vec![],
            }],
        };
        std::fs::write(
            migration.join("coverage-before.json"),
            serde_json::to_string(&before).unwrap(),
        )
        .unwrap();
        std::fs::write(
            migration.join("coverage-after.json"),
            r#"{"schema":1,"entries":[]}"#,
        )
        .unwrap();
        std::fs::write(
            migration.join("usecase-migration.md"),
            "| Old entry | Old owner | Operation | New owner |\n|---|---|---|---|\n| old/capability.feature — Preserved behavior | old | RETIRE | none |\n",
        )
        .unwrap();
        let result = check_migration(directory.path(), "migration-test");
        assert!(result.is_ok(), "migration check failed: {result:?}");
    }
}
