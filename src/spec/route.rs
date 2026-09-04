//! `givn spec route` — deterministic capability-routing candidates (Phase
//! 4.5). A narrowing tool, not a decision-maker: it emits ranked candidates
//! with evidence; a human/agent reads them and writes the decision in the
//! proposal's Capability Routing table. The review-time gate (Phase 4.7)
//! checks the recorded decision against what was built — it never reruns
//! this ranking.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use gherkin::GherkinEnv;
use serde::Serialize;

use crate::spec::cluster::KNOWN_SUBCOMMANDS;

/// A ranked routing candidate.
#[derive(Debug, Clone, Serialize)]
pub struct RouteCandidate {
    pub capability: String,
    pub group: String,
    pub score: f64,
    pub evidence: Vec<String>,
}

/// The routed recommendation for a proposal.
#[derive(Debug, Clone, Serialize)]
pub struct RouteReport {
    pub candidates: Vec<RouteCandidate>,
    /// Best-matching group when no capability scores meaningfully.
    pub suggested_group: Option<String>,
}

struct CapabilityInfo {
    name: String,
    group: String,
    file: std::path::PathBuf,
    /// subcommand -> invocation count.
    invocations: BTreeMap<String, usize>,
    text: String,
}

/// Extract `givn <subcommand>` invocations from text — only real
/// subcommands, never prose like "a givn project with".
fn count_subcommands(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut rest = text;
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
    counts
}

/// Backticked inline tokens from text (identifiers, paths, commands) —
/// without the leading/trailing backticks.
fn backticked_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('`') else {
            break;
        };
        let token = after[..end].trim().to_string();
        if !token.is_empty() && !tokens.contains(&token) {
            tokens.push(token);
        }
        rest = &after[end + 1..];
    }
    tokens
}

fn load_capabilities(root: &Path) -> Result<Vec<CapabilityInfo>> {
    let specs_root = root.join("givn/specs");
    let mut caps = Vec::new();
    for entry in std::fs::read_dir(&specs_root)? {
        let group_dir = entry?.path();
        if !group_dir.is_dir() {
            continue;
        }
        let group = group_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        for file in std::fs::read_dir(&group_dir)? {
            let file = file?.path();
            if file.extension().is_none_or(|e| e != "feature") {
                continue;
            }
            let feature = gherkin::Feature::parse_path(&file, GherkinEnv::default())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {e}", file.display()))?;
            let mut invocations = BTreeMap::new();
            let background_steps = feature
                .background
                .as_ref()
                .map(|b| b.steps.as_slice())
                .unwrap_or(&[]);
            let mut text = String::new();
            for scenario in &feature.scenarios {
                text.push_str(&scenario.name);
                text.push('\n');
                for step in scenario.steps.iter().chain(background_steps.iter()) {
                    text.push_str(&step.value);
                    text.push('\n');
                    crate::spec::cluster::count_invocations_for(&step.value, &mut invocations);
                }
            }
            caps.push(CapabilityInfo {
                name: file
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
                    .to_string(),
                group: group.clone(),
                file,
                invocations,
                text: text.to_lowercase(),
            });
        }
    }
    Ok(caps)
}

/// Lexical overlap of the proposal with each group.md's Goal + Interactions
/// text, used for the `NEW in <group>` fallback.
fn group_affinity(root: &Path, proposal_words: &[String]) -> Result<BTreeMap<String, f64>> {
    let specs_root = root.join("givn/specs");
    let mut affinity = BTreeMap::new();
    for entry in std::fs::read_dir(&specs_root)? {
        let group_dir = entry?.path();
        if !group_dir.is_dir() {
            continue;
        }
        let group = group_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let group_md = group_dir.join("group.md");
        let mut score = 0.0;
        if group_md.exists() {
            let text = std::fs::read_to_string(&group_md)?.to_lowercase();
            for word in proposal_words {
                if text.contains(word.as_str()) {
                    score += 1.0;
                }
            }
        }
        affinity.insert(group, score);
    }
    Ok(affinity)
}

fn content_words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| w.len() > 4)
        .map(|w| w.to_lowercase())
        .collect()
}

/// Route a proposal against the permanent corpus.
pub fn route(root: &Path, proposal_text: &str) -> Result<RouteReport> {
    let caps = load_capabilities(root)?;
    let proposal_subcommands = count_subcommands(proposal_text);
    let proposal_tokens = backticked_tokens(proposal_text);
    let proposal_words = content_words(proposal_text);

    let mut candidates: Vec<RouteCandidate> = Vec::new();
    for cap in &caps {
        let mut score = 0.0;
        let mut evidence = Vec::new();

        // 1. Command surface (strongest signal).
        for (sub, count) in &proposal_subcommands {
            if let Some(cap_count) = cap.invocations.get(sub) {
                let weight = 3.0 + (*count.min(cap_count) as f64 - 1.0) * 0.5;
                score += weight;
                evidence.push(format!(
                    "both invoke `givn {sub}` (proposal x{count}, capability x{cap_count})"
                ));
            }
        }

        // 2. Structural tokens (backticked identifiers/paths).
        for token in &proposal_tokens {
            let lower = token.to_lowercase();
            if cap.text.contains(&lower) {
                score += 1.0;
                evidence.push(format!("capability text contains `{token}`"));
            }
        }

        // 3. Lexical overlap with capability text (capped).
        let mut word_hits = 0usize;
        for word in &proposal_words {
            if cap.text.contains(word.as_str()) {
                word_hits += 1;
                if word_hits >= 15 {
                    break;
                }
            }
        }
        if word_hits > 0 {
            score += word_hits as f64 * 0.2;
            evidence.push(format!("{word_hits} content words overlap"));
        }

        if score > 0.0 {
            candidates.push(RouteCandidate {
                capability: cap.name.clone(),
                group: cap.group.clone(),
                score: (score * 100.0).round() / 100.0,
                evidence,
            });
        }
    }

    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.capability.cmp(&b.capability))
    });
    candidates.truncate(10);

    // 4. Group affinity for the NEW fallback.
    let affinity = group_affinity(root, &proposal_words)?;
    let suggested_group = affinity
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .filter(|(_, score)| *score > 0.0)
        .map(|(group, _)| group);

    Ok(RouteReport {
        candidates,
        suggested_group,
    })
}

/// Print the report with an EXTEND / NEW recommendation.
pub fn print_report(report: &RouteReport) {
    let top = report.candidates.first();
    let extend = top.filter(|c| c.score >= 3.0);
    match extend {
        Some(top) => {
            println!(
                "recommendation: EXTEND {} (group '{}', score {:.2})",
                top.capability, top.group, top.score
            );
        }
        None => {
            match &report.suggested_group {
                Some(group) => println!("recommendation: NEW in {group}"),
                None => println!("recommendation: NEW (no group affinity signal)"),
            };
            if let Some(top) = top {
                println!(
                    "  closest existing capability: {} (group '{}', score {:.2}) — below the extend bar",
                    top.capability, top.group, top.score
                );
            }
        }
    }
    println!("\nranked candidates:");
    for c in &report.candidates {
        println!("  {:.2}  {} (group {})", c.score, c.capability, c.group);
        for e in c.evidence.iter().take(3) {
            println!("         - {e}");
        }
    }
    if report.candidates.is_empty() {
        println!("  (no signal — the proposal has no command, token, or lexical overlap)");
    }
    println!(
        "\nrecord the decision in the proposal's Capability Routing table; \
this ranking is advisory only."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_subcommands_ignores_prose_and_unknown_verbs() {
        let counts = count_subcommands(
            "Given a givn project with changes\n`givn lint` and `givn status` and `givn not-real`",
        );
        assert_eq!(counts.get("lint"), Some(&1));
        assert_eq!(counts.get("status"), Some(&1));
        assert!(!counts.contains_key("project"));
        assert!(!counts.contains_key("not-real"));
    }

    #[test]
    fn backticked_tokens_are_extracted() {
        assert_eq!(
            backticked_tokens("a `foo/bar` and `givn lint` and bare ` tail"),
            vec!["foo/bar".to_string(), "givn lint".to_string()]
        );
    }

    #[test]
    fn routing_prefers_matching_command_surface() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("linters")).unwrap();
        std::fs::write(
            base.join("linters/lint-tool.feature"),
            "Feature: Lint\n\n  Scenario: One\n    When the user runs `givn lint`\n    And the user runs `givn lint`\n",
        )
        .unwrap();
        std::fs::create_dir_all(base.join("movers")).unwrap();
        std::fs::write(
            base.join("movers/mover.feature"),
            "Feature: Mover\n\n  Scenario: Two\n    When something unrelated happens\n    Then it is fine\n",
        )
        .unwrap();
        let report = route(
            root.path(),
            "Improve lint: `givn lint --json` should print JSON. Lint output. Lint.",
        )
        .unwrap();
        assert!(!report.candidates.is_empty());
        assert_eq!(report.candidates[0].capability, "lint-tool");
        assert_eq!(report.candidates[0].group, "linters");
        assert!(report.candidates[0].score >= 3.0, "extend bar");
    }

    #[test]
    fn routing_without_command_surface_suggests_new() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("linters")).unwrap();
        std::fs::write(
            base.join("linters/lint-tool.feature"),
            "Feature: Lint\n\n  Scenario: One\n    When the user runs `givn lint`\n",
        )
        .unwrap();
        let report = route(
            root.path(),
            "Refactor internal serialization of the widget graph plumbing.",
        )
        .unwrap();
        assert!(
            report.candidates.iter().all(|c| c.score < 3.0),
            "no candidate should clear the extend bar: {:?}",
            report.candidates
        );
    }
}
