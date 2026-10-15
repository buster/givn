//! `givn spec tree` — deterministic discovery over the permanent corpus.
//! No embeddings, no network, no score. Walks
//! `givn/specs/<usecase>/<capability>.feature`, reports per-use-case Actor/Goal from
//! `usecase.md`, per-capability scenario and `@e2e` counts, and the per-use-case
//! phrasing reuse ratio (reported only — never gated).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use gherkin::GherkinEnv;
use serde::Serialize;

/// One capability's row in the tree output.
#[derive(Debug, Clone, Serialize)]
pub struct CapabilityRow {
    pub name: String,
    pub scenarios: usize,
    pub e2e: usize,
}

/// One use case's summary in the tree output.
#[derive(Debug, Clone, Serialize)]
pub struct UsecaseRow {
    pub name: String,
    pub actor: Option<String>,
    pub goal: Option<String>,
    pub capabilities: Vec<CapabilityRow>,
    /// pooled phrasing reuse ratio for the use case (steps / distinct), 0.0
    /// when there are no steps.
    pub reuse_ratio: f64,
}

/// Normalise a step line for phrasing comparison: strip quoted spans →
/// `"X"`, digits → `N`, collapse whitespace, lowercase.
fn normalize_phrasing(step: &str) -> String {
    let mut out = String::with_capacity(step.len());
    let mut chars = step.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            for n in chars.by_ref() {
                if n == '"' {
                    break;
                }
            }
            out.push_str("\"x\"");
        } else if c.is_ascii_digit() {
            for n in chars.by_ref() {
                if !n.is_ascii_digit() {
                    break;
                }
            }
            out.push('N');
        } else {
            out.push(c);
        }
    }
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Pooled phrasing reuse: total steps / distinct normalized phrasings.
pub fn reuse_ratio(step_phrasings: &[String]) -> f64 {
    if step_phrasings.is_empty() {
        return 0.0;
    }
    let distinct: std::collections::BTreeSet<&String> = step_phrasings.iter().collect();
    step_phrasings.len() as f64 / distinct.len() as f64
}

/// Walk the permanent corpus and build the tree report.
pub fn build_tree(root: &Path) -> Result<Vec<UsecaseRow>> {
    let specs_root = root.join("givn/specs");
    let mut usecases: BTreeMap<String, UsecaseRow> = BTreeMap::new();

    for entry in std::fs::read_dir(&specs_root)? {
        let usecase_dir = entry?.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        let usecase_name = usecase_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let usecase = usecases
            .entry(usecase_name.clone())
            .or_insert_with(|| UsecaseRow {
                name: usecase_name.clone(),
                actor: None,
                goal: None,
                capabilities: Vec::new(),
                reuse_ratio: 0.0,
            });

        // Parse usecase.md for Actors/Goal when present.
        let usecase_md_path = usecase_dir.join("usecase.md");
        if usecase_md_path.exists() {
            let text = std::fs::read_to_string(&usecase_md_path)?;
            if let Ok(doc) = crate::gherkin::usecase_md::parse_usecase_md(&text) {
                if !doc.actors.is_empty() {
                    usecase.actor = Some(doc.actors.join(", "));
                }
                if !doc.goal.is_empty() {
                    usecase.goal = Some(doc.goal);
                }
            }
        }

        // Per-capability rows.
        let mut phrasings: Vec<String> = Vec::new();
        let mut caps: Vec<CapabilityRow> = Vec::new();
        for file in std::fs::read_dir(&usecase_dir)? {
            let file = file?.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", file.display()))?;
            let name = file
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            for scenario in &feature.scenarios {
                for step in &scenario.steps {
                    phrasings.push(normalize_phrasing(&step.value));
                }
            }
            let e2e = feature
                .scenarios
                .iter()
                .filter(|s| s.tags.iter().any(|t| t == "e2e"))
                .count();
            caps.push(CapabilityRow {
                name,
                scenarios: feature.scenarios.len(),
                e2e,
            });
        }
        caps.sort_by(|a, b| a.name.cmp(&b.name));
        usecase.capabilities = caps;
        usecase.reuse_ratio = (reuse_ratio(&phrasings) * 100.0).round() / 100.0;
    }

    Ok(usecases.into_values().collect())
}

/// Print the tree in human-readable text.
pub fn print_tree(rows: &[UsecaseRow]) {
    for usecase in rows {
        let total: usize = usecase.capabilities.iter().map(|c| c.scenarios).sum();
        let e2e: usize = usecase.capabilities.iter().map(|c| c.e2e).sum();
        println!(
            "use case '{}' — {} capabilities, {} scenarios, {} @e2e, reuse {:.2}x",
            usecase.name,
            usecase.capabilities.len(),
            total,
            e2e,
            usecase.reuse_ratio
        );
        if let Some(actor) = &usecase.actor {
            println!("  actors: {actor}");
        }
        if let Some(goal) = &usecase.goal {
            println!("  goal:  {goal}");
        }
        for cap in &usecase.capabilities {
            println!(
                "  - {} ({} scenarios, {} @e2e)",
                cap.name, cap.scenarios, cap.e2e
            );
        }
    }
}

/// One lexical search hit: a scenario whose title or step text
/// contains the (lowercased) query.
#[derive(Debug, Clone, Serialize)]
pub struct FindHit {
    pub usecase: String,
    pub capability: String,
    pub file: String,
    pub scenario: String,
    pub e2e: bool,
}

/// A deterministic duplicate/structural match. Exact
/// duplicates, shared assertion shapes, and assertion subsets only — no
/// embeddings, no score.
#[derive(Debug, Clone, Serialize)]
pub struct DuplicateMatch {
    pub kind: String,
    pub left_file: String,
    pub left_title: String,
    pub right_file: String,
    pub right_title: String,
}

fn is_subsequence(needle: &[String], haystack: &[String]) -> bool {
    let mut it = haystack.iter();
    needle.iter().all(|n| it.any(|h| h == n))
}

/// Walk the permanent corpus and report exact/structural duplicate
/// scenarios. Behavior text is the joined step values (normalized);
/// structure comes from the overlap fingerprint's assertion sequence.
pub fn duplicate_matches(root: &Path) -> Result<Vec<DuplicateMatch>> {
    use crate::overlap::fingerprint::Fingerprint;

    let specs_root = root.join("givn/specs");
    struct Entry {
        file: String,
        title: String,
        behavior: String,
        assertions: Vec<String>,
    }
    let mut entries: Vec<Entry> = Vec::new();
    // A missing specs directory is an empty corpus, not an error.
    let usecase_entries = match std::fs::read_dir(&specs_root) {
        Ok(rd) => rd,
        Err(_) => return Ok(Vec::new()),
    };
    for entry in usecase_entries {
        let usecase_dir = entry?.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        for file in std::fs::read_dir(&usecase_dir)
            .into_iter()
            .flatten()
            .flatten()
        {
            let file = file.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", file.display()))?;
            let file_label = file.display().to_string();
            for scenario in &feature.scenarios {
                let mut steps: Vec<&gherkin::Step> = scenario.steps.iter().collect();
                if let Some(background) = &feature.background {
                    steps.extend(background.steps.iter());
                }
                let behavior = steps
                    .iter()
                    .map(|s| normalize_phrasing(&s.value))
                    .collect::<Vec<_>>()
                    .join("\n");
                let fingerprint = Fingerprint::of_scenario(scenario, feature.background.as_ref());
                entries.push(Entry {
                    file: file_label.clone(),
                    title: scenario.name.clone(),
                    behavior,
                    assertions: fingerprint.assertions,
                });
            }
        }
    }

    let mut matches = Vec::new();
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            let (a, b) = (&entries[i], &entries[j]);
            if a.behavior == b.behavior {
                matches.push(DuplicateMatch {
                    kind: "exact_duplicate".to_string(),
                    left_file: a.file.clone(),
                    left_title: a.title.clone(),
                    right_file: b.file.clone(),
                    right_title: b.title.clone(),
                });
            } else if !a.assertions.is_empty() && a.assertions == b.assertions {
                matches.push(DuplicateMatch {
                    kind: "same_assertion_shape".to_string(),
                    left_file: a.file.clone(),
                    left_title: a.title.clone(),
                    right_file: b.file.clone(),
                    right_title: b.title.clone(),
                });
            } else {
                // Subset check must consider both directions: each
                // unordered pair is visited once with a fixed order.
                let (short_e, long_e) = if a.assertions.len() <= b.assertions.len() {
                    (a, b)
                } else {
                    (b, a)
                };
                if !short_e.assertions.is_empty()
                    && long_e.assertions.len() > short_e.assertions.len()
                    && is_subsequence(&short_e.assertions, &long_e.assertions)
                {
                    matches.push(DuplicateMatch {
                        kind: "assertion_subset".to_string(),
                        left_file: short_e.file.clone(),
                        left_title: short_e.title.clone(),
                        right_file: long_e.file.clone(),
                        right_title: long_e.title.clone(),
                    });
                }
            }
        }
    }
    Ok(matches)
}

/// Lexical search over the parsed AST — never a text grep, so docstring
/// payload cannot produce phantom scenarios. Substring match, lowercased,
/// over scenario titles and step text.
pub fn find_scenarios(root: &Path, query: &str) -> Result<Vec<FindHit>> {
    let needle = query.to_lowercase();
    let specs_root = root.join("givn/specs");
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(&specs_root)? {
        let usecase_dir = entry?.path();
        if !usecase_dir.is_dir()
            || usecase_dir
                .file_name()
                .is_some_and(|name| name == "fragments")
        {
            continue;
        }
        let usecase = usecase_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        for file in std::fs::read_dir(&usecase_dir)? {
            let file = file?.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())?;
            let capability = file
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            for scenario in &feature.scenarios {
                let e2e = scenario.tags.iter().any(|t| t == "e2e");
                let title_match = scenario.name.to_lowercase().contains(&needle);
                let step_match = scenario
                    .steps
                    .iter()
                    .any(|s| s.value.to_lowercase().contains(&needle));
                if title_match || step_match {
                    hits.push(FindHit {
                        usecase: usecase.clone(),
                        capability: capability.clone(),
                        file: file.display().to_string(),
                        scenario: scenario.name.clone(),
                        e2e,
                    });
                }
            }
        }
    }
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_phrasing_masks_quotes_digits_and_whitespace() {
        assert_eq!(
            normalize_phrasing("  the user   runs \"a thing\" with 3  tries"),
            "the user runs \"x\" with n tries"
        );
    }

    #[test]
    fn reuse_ratio_counts_pooled_reuse() {
        let steps = vec![
            "the user runs \"x\"".to_string(),
            "the user runs \"y\"".to_string(),
            "the user runs \"x\"".to_string(),
        ];
        assert!((reuse_ratio(&steps) - 1.5).abs() < 1e-9);
        assert_eq!(reuse_ratio(&[]), 0.0);
    }

    #[test]
    fn find_scenarios_matches_titles_and_steps_only_via_ast() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("workers")).unwrap();
        // A docstring containing a literal "Scenario:" must NOT create a
        // phantom scenario (AST-based, not text grep).
        std::fs::write(
            base.join("workers/digging.feature"),
            "Feature: Digging\n\n  Scenario: Unique shovelling title\n    Given x\n    Then the report says \"shovel-unique-token\"\n      \"\"\"\n      Scenario: fake in docstring shovel-unique-token\n      \"\"\"\n",
        )
        .unwrap();
        let by_title = find_scenarios(root.path(), "Unique shovelling title").unwrap();
        assert_eq!(by_title.len(), 1, "{by_title:?}");
        assert_eq!(by_title[0].usecase, "workers");
        assert_eq!(by_title[0].capability, "digging");
        assert!(!by_title[0].e2e);
        let by_step = find_scenarios(root.path(), "shovel-unique-token").unwrap();
        assert_eq!(
            by_title.len(),
            1,
            "docstring must not multiply hits: {by_step:?}"
        );
        assert_eq!(by_step[0].scenario, "Unique shovelling title");
        assert!(
            find_scenarios(root.path(), "no-such-token-xyz")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn build_tree_reports_usecases_capabilities_and_e2e_counts() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("workers")).unwrap();
        std::fs::write(
            base.join("workers/usecase.md"),
            "# Use case: workers\n\n## Level\n\n!\n\n## Actors\n\n- A builder.\n\n## Goal\n\nBuild things.\n\n## Trigger\n\nA request.\n\n## Preconditions\n\n- A project exists.\n\n## Main flow\n\n1. Build.\n\n## Extensions\n\n- none\n\n## Rules\n\n- Build once.\n\n## Examples\n\n- A project.\n\n## Minimal guarantee\n\nThe project remains.\n\n## Success guarantee\n\nThe project is built.\n\n## Personas\n\n- none\n\n## Capabilities\n\n- digging\n\n## Interactions\n\n| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| digging | shovel | Shovel |\n\n## Includes\n\n- none\n\n## Extends\n\n- none\n\n## Out of scope\n\nNothing.\n\n## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n",
        )
        .unwrap();
        std::fs::write(
            base.join("workers/digging.feature"),
            "Feature: Digging\n\n  Scenario: Dig\n    Given x\n    Then y\n\n  @e2e\n  Scenario: Shovel\n    Given x\n    Then y\n",
        )
        .unwrap();
        let rows = build_tree(root.path()).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "workers");
        assert_eq!(rows[0].actor.as_deref(), Some("A builder."));
        assert_eq!(rows[0].capabilities.len(), 1);
        assert_eq!(rows[0].capabilities[0].scenarios, 2);
        assert_eq!(rows[0].capabilities[0].e2e, 1);
        assert!(rows[0].reuse_ratio > 0.0);
    }
}
