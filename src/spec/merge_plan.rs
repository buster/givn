//! Shared merge-text and file-writer seam.
//!
//! The projection computes merged feature text in memory; the archive writes
//! the same text to a permanent file. Both call `merge_feature_text`, which
//! wraps the one feature-level merge in `gherkin::merge`. There is no second
//! merge algorithm and no temporary specifications tree.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use gherkin::GherkinEnv;

use crate::gherkin::parse::collect_feature_files;

/// Produce the permanent feature text for one delta feature.
///
/// `retry` selects the idempotent mode used for an in-flight or legacy-unrecorded
/// archive merge. The function never writes.
pub fn merge_feature_text(
    permanent_text: Option<&str>,
    delta_feature: &gherkin::Feature,
    retry: bool,
) -> Result<String> {
    if retry {
        crate::gherkin::merge::apply_feature_deltas_retry(permanent_text, delta_feature)
    } else {
        crate::gherkin::merge::apply_feature_deltas(permanent_text, delta_feature)
    }
}

/// Merge one delta feature into a permanent file.
///
/// This is the thin read/write wrapper around `merge_feature_text`; it keeps
/// the `@givn.delta` guard and the atomic `.givn-tmp` rename. It is the only
/// production writer for archive and grouped-merge publication.
pub fn write_feature(permanent_path: &Path, delta_feature: &gherkin::Feature) -> Result<()> {
    write_feature_mode(permanent_path, delta_feature, false)
}

/// Retry an in-flight or legacy-unrecorded archive merge.
pub fn write_feature_retry(permanent_path: &Path, delta_feature: &gherkin::Feature) -> Result<()> {
    write_feature_mode(permanent_path, delta_feature, true)
}

fn write_feature_mode(
    permanent_path: &Path,
    delta_feature: &gherkin::Feature,
    retry: bool,
) -> Result<()> {
    if !crate::gherkin::delta::is_delta_feature(delta_feature) {
        bail!(
            "feature '{}' is not tagged @givn.delta — cannot merge",
            delta_feature.name
        );
    }

    let permanent_text = if permanent_path.exists() {
        Some(
            std::fs::read_to_string(permanent_path)
                .with_context(|| format!("cannot read '{}'", permanent_path.display()))?,
        )
    } else {
        None
    };
    let new_text = merge_feature_text(permanent_text.as_deref(), delta_feature, retry)
        .map_err(|error| anyhow::anyhow!("{}: {error}", permanent_path.display()))?;

    if let Some(parent) = permanent_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("cannot create '{}'", parent.display()))?;
    }
    let file_name = permanent_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "permanent.feature".to_string());
    let temporary = permanent_path.with_file_name(format!("{file_name}.givn-tmp"));
    std::fs::write(&temporary, new_text)
        .with_context(|| format!("cannot write '{}'", temporary.display()))?;
    std::fs::rename(&temporary, permanent_path)
        .with_context(|| format!("cannot replace '{}'", permanent_path.display()))?;
    Ok(())
}

/// One delta-to-permanent merge instruction.
#[derive(Debug, Clone)]
pub struct MergeEntry {
    pub relative: String,
    pub feature: gherkin::Feature,
    /// Absolute target for the archive writer.
    pub target: PathBuf,
    /// Project-relative target including the `givn/specs/` prefix.
    pub target_rel: PathBuf,
    pub retire: bool,
    pub cap: String,
}

/// Failure to build a merge plan that the projection must report as a finding.
#[derive(Debug, thiserror::Error)]
pub enum MergePlanError {
    #[error("{detail}")]
    UnplaceableCapability { capability: String, detail: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl MergePlanError {
    fn other(message: String) -> Self {
        Self::Other(anyhow::anyhow!(message))
    }
}

pub(crate) fn delta_capability(
    delta_path: &Path,
    delta_feature: &gherkin::Feature,
) -> Result<String> {
    let cap = crate::gherkin::delta::capability_tag(delta_feature)
        .map(|tag| tag.to_string())
        .unwrap_or_else(|| {
            delta_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string()
        });
    crate::gherkin::delta::validate_capability_tag(&cap)
        .map_err(|e| anyhow::anyhow!("{e} (from '{}')", delta_path.display()))?;
    Ok(cap)
}

/// Resolve the permanent directory a capability's spec lives in: the use-case
/// directory it already exists in, or the use case its proposal's
/// `NEW in <usecase-id>` routing decision declares.
pub(crate) fn resolve_permanent_dir(
    project_root: &Path,
    change_root: &Path,
    change_id: &str,
    cap: &str,
) -> Result<PathBuf, MergePlanError> {
    let specs_root = project_root.join("givn/specs");
    if let Some(dir) = crate::spec::owner::resolve_capability_owner(&specs_root, cap)
        .map_err(MergePlanError::Other)?
    {
        return Ok(dir);
    }
    let usecase = routing_usecase_for_new_capability(change_root, cap).ok_or_else(|| {
        MergePlanError::UnplaceableCapability {
            capability: cap.to_string(),
            detail: format!(
                "capability '{cap}' exists nowhere in the permanent corpus and its proposal \
                 records no 'NEW in <usecase-id>' routing decision (change '{change_id}')"
            ),
        }
    })?;
    Ok(specs_root.join(usecase))
}

fn routing_usecase_for_new_capability(change_root: &Path, cap: &str) -> Option<String> {
    let proposal = std::fs::read_to_string(change_root.join("proposal.md")).ok()?;
    crate::spec::routing_table::parse_routing_table(&proposal)
        .into_iter()
        .find(|row| row.capability == cap)
        .and_then(|row| match row.decision {
            crate::spec::routing_table::Decision::NewIn(usecase) => Some(usecase),
            _ => None,
        })
}

pub(crate) fn build_merge_plan(
    project_root: &Path,
    change_root: &Path,
    change_id: &str,
    source_specs: &Path,
    validate: bool,
) -> Result<Vec<MergeEntry>, MergePlanError> {
    let specs_root = project_root.join("givn/specs");
    let mut plan = Vec::new();
    let mut seen = BTreeSet::new();
    if !source_specs.exists() {
        return Ok(plan);
    }
    for delta_path in collect_feature_files(source_specs) {
        let feature =
            gherkin::Feature::parse_path(&delta_path, GherkinEnv::default()).map_err(|e| {
                MergePlanError::other(format!(
                    "failed to parse delta feature '{}': {e}",
                    delta_path.display()
                ))
            })?;
        if validate && let Some(wip_msg) = crate::gherkin::delta::find_wip_in_feature(&feature) {
            return Err(MergePlanError::other(format!(
                "archive blocked: {wip_msg} in '{}' — remove @wip from completed scenarios",
                delta_path.display()
            )));
        }
        let rel = delta_path
            .strip_prefix(source_specs)
            .unwrap_or(&delta_path)
            .to_string_lossy()
            .replace('\\', "/");
        let cap = delta_capability(&delta_path, &feature).map_err(MergePlanError::Other)?;
        if !seen.insert(cap.clone()) {
            return Err(MergePlanError::other(format!(
                "archive blocked: capability '{cap}' appears in more than one delta file; \
                 a change may merge each capability once"
            )));
        }
        let retire = crate::gherkin::delta::is_retired_feature(&feature);
        if retire {
            if !feature.scenarios.is_empty() {
                return Err(MergePlanError::other(format!(
                    "archive blocked: '{}' is tagged @givn.retired but contains {} \
                     scenario(s) — a retirement delta must have zero scenarios",
                    delta_path.display(),
                    feature.scenarios.len()
                )));
            }
            let Some(usecase_dir) = crate::spec::owner::resolve_capability_owner(&specs_root, &cap)
                .map_err(MergePlanError::Other)?
            else {
                return Err(MergePlanError::other(if validate {
                    format!(
                        "archive blocked: '{}' retires capability '{cap}' but no such \
                         capability exists in the active use-case tree",
                        delta_path.display()
                    )
                } else {
                    format!(
                        "archive blocked: the retirement target for '{cap}' is no longer \
                         present; rebuild the plan from the receipt"
                    )
                }));
            };
            if validate && !usecase_dir.join(format!("{cap}.feature")).exists() {
                return Err(MergePlanError::other(format!(
                    "archive blocked: '{}' retires capability '{cap}' but '{}' does not exist",
                    delta_path.display(),
                    usecase_dir.join(format!("{cap}.feature")).display()
                )));
            }
            let target_rel = target_rel(&specs_root, &usecase_dir)?;
            plan.push(MergeEntry {
                relative: rel,
                feature,
                target: usecase_dir,
                target_rel,
                retire: true,
                cap,
            });
            continue;
        }

        let permanent_dir = resolve_permanent_dir(project_root, change_root, change_id, &cap)?;
        let target = permanent_dir.join(format!("{cap}.feature"));
        if validate {
            let permanent_text = std::fs::read_to_string(&target).ok();
            crate::gherkin::merge::validate_feature_merge(permanent_text.as_deref(), &feature)
                .map_err(|error| {
                    MergePlanError::other(format!("{}: {error}", delta_path.display()))
                })?;
        }
        let target_rel = target_rel(&specs_root, &target)?;
        plan.push(MergeEntry {
            relative: rel,
            feature,
            target,
            target_rel,
            retire: false,
            cap,
        });
    }
    Ok(plan)
}

fn target_rel(specs_root: &Path, target: &Path) -> Result<PathBuf, MergePlanError> {
    let relative = target.strip_prefix(specs_root).map_err(|_| {
        MergePlanError::other(format!(
            "merge target '{}' is outside '{}'",
            target.display(),
            specs_root.display()
        ))
    })?;
    Ok(PathBuf::from("givn/specs").join(relative))
}

pub(crate) fn collect_narrative_documents(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_narrative_documents(&path, out)?;
        } else if path.file_name().is_some_and(|name| name == "usecase.md")
            || (path.file_name().is_some_and(|name| name == "fragment.md")
                && path.parent().and_then(|parent| parent.file_name())
                    == Some(std::ffi::OsStr::new("fragments")))
        {
            out.push(path);
        }
    }
    Ok(())
}

pub(crate) fn narrative_target(specs_root: &Path, relative: &Path) -> Result<PathBuf> {
    if relative == Path::new("fragments/fragment.md") {
        return Ok(specs_root.join(relative));
    }
    if relative
        .file_name()
        .is_some_and(|name| name == "usecase.md")
    {
        let Some(usecase) = relative.parent().and_then(|parent| parent.file_name()) else {
            bail!("usecase.md must be under specs/<usecase-id>/");
        };
        return Ok(specs_root.join(usecase).join("usecase.md"));
    }
    bail!(
        "unsupported narrative path '{}'; expected usecase.md or fragments/fragment.md",
        relative.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> gherkin::Feature {
        crate::gherkin::parse::parse_str(text).unwrap()
    }

    #[test]
    fn writes_a_new_feature_and_keeps_background_and_tags() {
        let feature = parse(
            "@givn.delta @catalog @e2e\nFeature: Catalog\n  Background: Session\n    Given signed in\n  @givn.added @api\n  Scenario: Read\n    Then visible\n",
        );
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.feature");
        write_feature(&path, &feature).unwrap();
        let merged = parse(&std::fs::read_to_string(path).unwrap());
        assert!(merged.background.is_some());
        assert!(merged.tags.contains(&"e2e".into()));
        assert!(merged.scenarios[0].tags.contains(&"api".into()));
    }

    #[test]
    fn retry_accepts_an_already_applied_added_scenario() {
        let feature = parse(
            "@givn.delta @catalog\nFeature: Catalog\n  @givn.added\n  Scenario: Read\n    Then visible\n",
        );
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.feature");
        write_feature(&path, &feature).unwrap();
        let once = std::fs::read_to_string(&path).unwrap();
        write_feature_retry(&path, &feature).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), once);
    }

    #[test]
    fn refuses_a_non_delta_feature_without_writing() {
        let feature = parse("Feature: Catalog\n  Scenario: Read\n    Then visible\n");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.feature");
        assert!(write_feature(&path, &feature).is_err());
        assert!(!path.exists());
    }

    #[test]
    fn conflicting_background_refuses_without_writing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog.feature");
        let permanent = "Feature: Catalog\n  Background: Session\n    Given original session\n  Scenario: Read\n    Then visible\n  Scenario: Keep\n    Then unchanged\n";
        std::fs::write(&path, permanent).unwrap();
        let feature = parse(
            "@givn.delta @catalog\nFeature: Catalog\n  Background: Session\n    Given different session\n  @givn.modified\n  Scenario: Read\n    Then updated\n",
        );
        assert!(write_feature(&path, &feature).is_err());
        assert_eq!(std::fs::read_to_string(path).unwrap(), permanent);
    }

    #[test]
    fn unsupported_publication_refuses_without_creating_parent() {
        for source in [
            "@givn.delta @catalog\nFeature: Catalog\n  Rule: Restricted\n    Scenario: Read\n      Then visible\n",
            "# language: fr\n@givn.delta @catalog\nFonctionnalité: Catalogue\n  Scénario: Lire\n    Alors visible\n",
        ] {
            let feature = parse(source);
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("new/catalog.feature");
            assert!(
                write_feature(&path, &feature).is_err(),
                "unsupported feature accepted: {source}"
            );
            assert!(!path.parent().unwrap().exists());
        }
    }
}
