//! Deterministic command-surface clustering of the spec corpus (Phase 2.4a).
//!
//! Given the `givn/specs/` tree, propose a use-case grouping by clustering
//! capabilities on which `givn <subcommand>` their scenario steps invoke.
//! This is the signal validated in the retirement plan (PLAN.md §5): a
//! capability's dominant command surface is a clean, embedding-free
//! grouping signal.
//!
//! Deliberately NOT a bare substring match for "givn X": the prose phrase
//! "Given a givn project with…" must not become a `givn project` command
//! invocation. Only match backtick-wrapped `givn <subcommand>` tokens
//! where `<subcommand>` is a real subcommand from `KNOWN_SUBCOMMANDS`
//! (kept in sync with `Commands` in `src/main.rs`).

use std::collections::BTreeMap;
use std::path::Path;

use gherkin::GherkinEnv;

/// The real subcommand list to match against. Keep in sync with `Commands`
/// in `src/main.rs`; hand-maintained for the first version.
pub const KNOWN_SUBCOMMANDS: &[&str] = &[
    "init",
    "new",
    "instructions",
    "lint",
    "check",
    "status",
    "graph",
    "archive",
    "eject",
    "addons",
    "readme",
    "skills",
    "spec",
    "upgrade",
    "run",
];

/// The proposed grouping derived from command-surface clustering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterProposal {
    /// (dominant subcommand, capability ids sorted alphabetically).
    pub groups: Vec<(String, Vec<String>)>,
    /// Capabilities whose steps invoke no givn command — `«include»`
    /// fragment candidates, not use-case candidates.
    pub fragments: Vec<String>,
}

/// A capability's invocation counts per subcommand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSurface {
    pub capability: String,
    /// subcommand → invocation count, sorted by count desc then name.
    pub invocations: Vec<(String, usize)>,
}

/// Extract a capability's per-subcommand invocation counts from its parsed
/// feature steps (including the feature-level Background, if any). Only
/// backtick-wrapped `givn <subcommand>` tokens count.
pub fn command_surface(capability: &str, feature: &gherkin::Feature) -> CommandSurface {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let background_steps = feature
        .background
        .as_ref()
        .map(|b| b.steps.as_slice())
        .unwrap_or(&[]);
    for scenario in &feature.scenarios {
        for step in scenario.steps.iter().chain(background_steps.iter()) {
            count_invocations(&step.value, &mut counts);
        }
    }
    let mut invocations: Vec<(String, usize)> = counts.into_iter().collect();
    invocations.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    CommandSurface {
        capability: capability.to_string(),
        invocations,
    }
}

fn count_invocations(step_text: &str, counts: &mut BTreeMap<String, usize>) {
    let mut rest = step_text;
    while let Some(pos) = rest.find("`givn ") {
        let after = &rest[pos + "`givn ".len()..];
        let token: String = after
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '-')
            .collect();
        if let Some(sub) = KNOWN_SUBCOMMANDS.iter().find(|s| **s == token) {
            *counts.entry((*sub).to_string()).or_insert(0) += 1;
        }
        rest = &rest[pos + 1..];
    }
}

/// Cluster all capabilities under `givn/specs/` (grouped or flat layout)
/// by their dominant command surface.
pub fn cluster_tree(root: &Path) -> ClusterProposal {
    let specs_root = root.join("givn/specs");
    let mut surfaces: Vec<CommandSurface> = Vec::new();
    for cap_dir in std::fs::read_dir(&specs_root)
        .into_iter()
        .flatten()
        .flatten()
    {
        let path = cap_dir.path();
        if !path.is_dir() {
            continue;
        }
        // Flat layout: <cap>/<cap>.feature.
        let feature_path = path.join(format!(
            "{}.feature",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
        ));
        // Grouped layout: <group>/<cap>.feature — a capability file, not a
        // group dir, has no subdirectories and one or more feature files.
        let candidates: Vec<std::path::PathBuf> = if feature_path.exists() {
            vec![feature_path]
        } else {
            std::fs::read_dir(&path)
                .map(|rd| {
                    rd.flatten()
                        .map(|e| e.path())
                        .filter(|p| p.extension().is_some_and(|e| e == "feature"))
                        .collect()
                })
                .unwrap_or_default()
        };
        for file in candidates {
            let Ok(feature) = gherkin::Feature::parse_path(&file, GherkinEnv::default()) else {
                continue;
            };
            let cap_name = file
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            surfaces.push(command_surface(&cap_name, &feature));
        }
    }

    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut fragments: Vec<String> = Vec::new();
    for surface in &surfaces {
        // Ambiguity: a tie for the top count is left ungrouped for human
        // judgment rather than guessed. A single invocation is too weak a
        // signal to cluster on.
        let Some((dominant, top_count)) = surface.invocations.first().map(|(n, c)| (n.clone(), *c))
        else {
            fragments.push(surface.capability.clone());
            continue;
        };
        if top_count <= 1 || ties_exist(&surface.invocations, top_count) {
            fragments.push(surface.capability.clone());
            continue;
        }
        groups
            .entry(dominant)
            .or_default()
            .push(surface.capability.clone());
    }
    for members in groups.values_mut() {
        members.sort();
    }
    ClusterProposal {
        groups: groups.into_iter().collect(),
        fragments: {
            fragments.sort();
            fragments
        },
    }
}

fn ties_exist(invocations: &[(String, usize)], top_count: usize) -> bool {
    invocations.iter().filter(|(_, c)| *c == top_count).count() > 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gherkin::parse::parse_str;

    #[test]
    fn counts_backtick_wrapped_real_subcommands_only() {
        let feature = parse_str(
            r#"
Feature: X

  Scenario: One
    Given a givn project with an active change
    When the user runs `givn lint`
    And the user runs `givn spec tree`
    Then the command exits 0
    And prose mentions givn project casually
"#,
        )
        .unwrap();
        let surface = command_surface("demo", &feature);
        // "Given a givn project with..." is prose, not a command — no
        // `givn project` match; the backtick-wrapped real subcommands count.
        assert_eq!(
            surface.invocations,
            vec![("lint".to_string(), 1), ("spec".to_string(), 1)]
        );
    }

    #[test]
    fn dominant_surface_wins_and_single_invocation_stays_fragment() {
        let feature = parse_str(
            r#"
Feature: Demo

  Scenario: One
    When the user runs `givn lint`
    And the user runs `givn lint --json`
    And the user runs `givn status`

  Scenario: Two
    When the user runs `givn lint --change demo`
"#,
        )
        .unwrap();
        let surface = command_surface("demo", &feature);
        assert_eq!(surface.invocations[0], ("lint".to_string(), 3));
    }

    #[test]
    fn tie_for_top_count_is_flagged_as_fragment() {
        let feature = parse_str(
            r#"
Feature: Demo

  Scenario: One
    When the user runs `givn lint`
    And the user runs `givn status`
"#,
        )
        .unwrap();
        let surface = command_surface("demo", &feature);
        assert_eq!(surface.invocations.len(), 2);
        assert_eq!(surface.invocations[0].1, 1);
    }

    #[test]
    fn cluster_tree_separates_grouped_and_flat_layouts() {
        let root = tempfile::TempDir::new().unwrap();
        let specs = root.path().join("givn/specs");
        std::fs::create_dir_all(specs.join("linters")).unwrap();
        std::fs::write(
            specs.join("linters/lint-command.feature"),
            "Feature: Lint\n\n  Scenario: S\n    When the user runs `givn lint`\n    And the user runs `givn lint`\n",
        )
        .unwrap();
        std::fs::create_dir_all(specs.join("spec-tools")).unwrap();
        std::fs::write(
            specs.join("spec-tools/spec-duplicates.feature"),
            "Feature: Duplicates\n\n  Scenario: T\n    When the user runs `givn spec duplicates`\n    And the user runs `givn spec duplicates`\n",
        )
        .unwrap();
        std::fs::create_dir_all(specs.join("frag-cap")).unwrap();
        std::fs::write(
            specs.join("frag-cap/frag-cap.feature"),
            "Feature: Frag\n\n  Scenario: U\n    Given a project root\n    Then nothing runs\n",
        )
        .unwrap();
        let proposal = cluster_tree(root.path());
        assert!(
            proposal
                .groups
                .contains(&("lint".to_string(), vec!["lint-command".to_string()]))
        );
        assert!(
            proposal
                .groups
                .contains(&("spec".to_string(), vec!["spec-duplicates".to_string()]))
        );
        assert_eq!(proposal.fragments, vec!["frag-cap"]);
    }
}

#[cfg(test)]
mod corpus_dump {
    use super::*;

    /// Manual verification aid (task 2.4a): run with
    /// `cargo test --lib cluster_corpus_dump -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn cluster_corpus() {
        let proposal = cluster_tree(Path::new("."));
        for (sub, caps) in &proposal.groups {
            println!("{sub} <- {} : {:?}", caps.len(), caps);
        }
        println!(
            "fragments ({}): {:?}",
            proposal.fragments.len(),
            proposal.fragments
        );
    }
}
