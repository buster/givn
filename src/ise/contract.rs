//! The generated `ise-verification/v1` contract.
//!
//! The contract carries the commands, feature paths, declared execution-unit
//! counts, and coverage settings a trial verifier needs. It is generated from
//! the effective manifest and the Specification at the selected revision, so
//! the builder reads the commands that govern the trial without the project's
//! working tree.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::config::declaration::SpecificationDeclaration;
use crate::config::manifest::Manifest;
use crate::scm::EntryKind;

use super::manifest;
use super::partition;

/// The fixed project-relative path the rebuild writes the contract to.
pub(super) const CONTRACT_PATH: &str = "givn/ise-verification.yaml";

/// The contract's scope marker.
pub(super) const CONTENT_SCOPE: &str = "ise-verification/v1";

/// One trial's runnable verification contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Contract {
    pub scope: String,
    pub revision: String,
    pub component: Tier,
    pub end_to_end: Tier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<CoverageContract>,
}

/// One tier's command, feature path, and declared execution-unit count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Tier {
    pub command: String,
    pub features: String,
    /// Plain scenarios count once; a Scenario Outline counts once per example
    /// row, matching the runner's expanded count.
    pub execution_units: usize,
}

/// The coverage command and report paths when the addon is effective.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CoverageContract {
    pub measure_command: String,
    pub non_e2e_path: String,
    pub e2e_path: String,
    /// The End-to-end suite runs the product as a subprocess and cannot
    /// instrument it; a 0-line e2e report is accepted only when this is true.
    pub e2e_subprocess: bool,
}

impl Contract {
    /// Build the contract from the effective manifest and the feature blobs at
    /// the selected revision.
    pub(super) fn build(
        manifest: &Manifest,
        revision: &str,
        declaration: &SpecificationDeclaration,
        entries: &[crate::scm::RevisionEntry],
        blobs: &BTreeMap<String, Vec<u8>>,
    ) -> Result<Self> {
        let paths: Vec<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();
        let features = manifest.verify.features_path.clone();
        let (component_units, end_to_end_units) = execution_units(declaration, &paths, blobs)?;
        let coverage = manifest.addons.coverage.then(|| CoverageContract {
            measure_command: manifest.coverage.measure_command.clone(),
            non_e2e_path: manifest.coverage.non_e2e_path.clone(),
            e2e_path: manifest.coverage.e2e_path.clone(),
            e2e_subprocess: manifest.coverage.e2e_subprocess,
        });
        Ok(Self {
            scope: CONTENT_SCOPE.to_string(),
            revision: revision.to_string(),
            component: Tier {
                command: manifest.verify.command.clone(),
                features: features.clone(),
                execution_units: component_units,
            },
            end_to_end: Tier {
                command: manifest.verify.e2e_command.clone(),
                features,
                execution_units: end_to_end_units,
            },
            coverage,
        })
    }

    /// Serialize the contract with its fixed field order.
    fn yaml(&self) -> Result<String> {
        serde_yaml_ng::to_string(self).context("serialize the verification contract")
    }

    /// Write the contract and return its path checksum for the manifest.
    pub(super) fn write(&self, path: &Path) -> Result<String> {
        let text = self.yaml()?;
        std::fs::write(path, text.as_bytes())
            .with_context(|| format!("write the verification contract '{}'", path.display()))?;
        Ok(manifest::path_checksum(
            CONTRACT_PATH,
            EntryKind::Regular,
            false,
            text.as_bytes(),
        ))
    }

    /// Read one strict contract. A wrong scope or unknown field refuses by
    /// name; the reader never migrates an older shape.
    pub(super) fn read(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read the verification contract '{}'", path.display()))?;
        let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text).with_context(|| {
            format!(
                "the verification contract '{}' is not readable YAML",
                path.display()
            )
        })?;
        if let Some(recorded) = document
            .get("scope")
            .and_then(serde_yaml_ng::Value::as_str)
            .filter(|scope| *scope != CONTENT_SCOPE)
        {
            bail!(
                "the verification contract '{}' records the scope '{}'; the reader requires '{CONTENT_SCOPE}'",
                path.display(),
                recorded
            );
        }
        serde_yaml_ng::from_value(document)
            .with_context(|| {
                format!(
                    "the verification contract '{}' does not match ise-verification/v1",
                    path.display()
                )
            })
            .and_then(|contract: Self| {
                if let Some(coverage) = &contract.coverage {
                    require_relative_report("non_e2e_path", &coverage.non_e2e_path)?;
                    require_relative_report("e2e_path", &coverage.e2e_path)?;
                }
                Ok(contract)
            })
    }
}

/// A coverage report path must stay under the copy: relative and free of
/// parent-directory components.
fn require_relative_report(name: &str, path: &str) -> Result<()> {
    let parsed = Path::new(path);
    if parsed.is_absolute()
        || parsed
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        bail!(
            "the verification contract's coverage.{name} '{path}' must be relative to the spec copy and cannot contain '..'"
        );
    }
    Ok(())
}

/// The declared execution-unit counts for the component and End-to-end tiers.
///
/// A feature-level `@e2e` tag selects every scenario in that file. A Scenario
/// Outline expands to one unit per example row, so the declared count matches
/// the runner's scenario events rather than the file's outline headings.
fn execution_units(
    declaration: &SpecificationDeclaration,
    paths: &[&str],
    blobs: &BTreeMap<String, Vec<u8>>,
) -> Result<(usize, usize)> {
    let mut component = 0usize;
    let mut end_to_end = 0usize;
    for path in partition::behavior_features(declaration, paths) {
        let content = blobs
            .get(&path)
            .ok_or_else(|| anyhow::anyhow!("the revision returned no content for '{path}'"))?;
        let source = std::str::from_utf8(content)
            .with_context(|| format!("read the feature at '{path}' as UTF-8"))?;
        let feature = gherkin::Feature::parse(source, gherkin::GherkinEnv::default())
            .with_context(|| format!("parse the feature at '{path}'"))?;
        let feature_e2e = feature.tags.iter().any(|tag| tag == "e2e");
        for scenario in &feature.scenarios {
            let units = scenario_execution_units(scenario);
            if feature_e2e || scenario.tags.iter().any(|tag| tag == "e2e") {
                end_to_end += units;
            } else {
                component += units;
            }
        }
    }
    Ok((component, end_to_end))
}

/// One scenario's expanded execution units. A plain scenario is one unit; an
/// outline is one per example row.
fn scenario_execution_units(scenario: &gherkin::Scenario) -> usize {
    if scenario.examples.is_empty() {
        return 1;
    }
    scenario
        .examples
        .iter()
        .map(|examples| {
            examples
                .table
                .as_ref()
                .map_or(0, |table| table.rows.len().saturating_sub(1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scenario(source: &str) -> gherkin::Scenario {
        let feature = gherkin::Feature::parse(source, gherkin::GherkinEnv::default()).unwrap();
        feature.scenarios[0].clone()
    }

    #[test]
    fn a_plain_scenario_is_one_execution_unit() {
        let scenario = scenario("Feature: demo\n  Scenario: one\n    Given nothing\n");
        assert_eq!(scenario_execution_units(&scenario), 1);
    }

    #[test]
    fn an_outline_counts_each_example_row() {
        let scenario = scenario(
            "Feature: demo\n  Scenario Outline: one\n    Given <value>\n    Examples:\n      | value |\n      | a     |\n      | b     |\n",
        );
        assert_eq!(scenario_execution_units(&scenario), 2);
    }

    #[test]
    fn an_outline_with_two_example_blocks_sums_their_rows() {
        let scenario = scenario(
            "Feature: demo\n  Scenario Outline: one\n    Given <value>\n    Examples:\n      | value |\n      | a     |\n    Examples:\n      | value |\n      | b     |\n      | c     |\n",
        );
        assert_eq!(scenario_execution_units(&scenario), 3);
    }

    #[test]
    fn a_contract_round_trips_and_refuses_an_unknown_field() {
        let contract = Contract {
            scope: CONTENT_SCOPE.to_string(),
            revision: "abc".to_string(),
            component: Tier {
                command: "./verify.sh".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 10,
            },
            end_to_end: Tier {
                command: "./e2e.sh".to_string(),
                features: "givn/specs".to_string(),
                execution_units: 1,
            },
            coverage: None,
        };
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("contract.yaml");
        contract.write(&path).unwrap();
        assert_eq!(Contract::read(&path).unwrap(), contract);

        let mut text = std::fs::read_to_string(&path).unwrap();
        text.push_str("unknown: true\n");
        std::fs::write(&path, text).unwrap();
        let error = Contract::read(&path).unwrap_err();
        assert!(format!("{error:#}").contains("unknown"), "{error:#}");
    }

    #[test]
    fn a_coverage_report_path_cannot_leave_the_copy() {
        let directory = tempfile::TempDir::new().unwrap();
        let path = directory.path().join("contract.yaml");
        std::fs::write(
            &path,
            "scope: ise-verification/v1\nrevision: abc\ncomponent:\n  command: \"true\"\n  features: givn/specs\n  execution_units: 1\nend_to_end:\n  command: \"true\"\n  features: givn/specs\n  execution_units: 0\ncoverage:\n  measure_command: \"true\"\n  non_e2e_path: \"../outside.xml\"\n  e2e_path: coverage/e2e.xml\n  e2e_subprocess: false\n",
        )
        .unwrap();
        let error = Contract::read(&path).expect_err("a parent path refuses");
        assert!(
            format!("{error:#}").contains("cannot contain '..'"),
            "{error:#}"
        );
    }
}
