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
    pub usecase: String,
    pub score: f64,
    pub evidence: Vec<String>,
}

/// The confidence of the routed recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteConfidence {
    Signal,
    NoSignal,
}

/// The routed recommendation for a proposal.
#[derive(Debug, Clone, Serialize)]
pub struct RouteReport {
    pub candidates: Vec<RouteCandidate>,
    /// Best-matching use case when no capability scores meaningfully.
    pub suggested_usecase: Option<String>,
    /// Explicit confidence: no signal on a tie or below-threshold overlap.
    pub confidence: RouteConfidence,
    /// Human-readable reason when the confidence is no signal.
    pub note: Option<String>,
}

struct CapabilityInfo {
    name: String,
    usecase: String,
    /// subcommand -> invocation count.
    invocations: BTreeMap<String, usize>,
    text: String,
    /// The owning use case's Goal + Rules text, lowercased (empty when the
    /// use case document is missing or unreadable).
    goal_rules: String,
    /// The capability's interaction consumer actions, lowercased.
    interactions: String,
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

fn usecase_ownership_text(usecase_dir: &Path) -> (String, String) {
    let path = usecase_dir.join("usecase.md");
    let Ok(content) = std::fs::read_to_string(&path) else {
        return (String::new(), String::new());
    };
    let mut goal_rules = String::new();
    let mut interactions = String::new();
    let mut section = "";
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("## ") {
            section = match heading {
                "Goal" | "Rules" => "goal_rules",
                "Interactions" => "interactions",
                _ => "",
            };
            continue;
        }
        match section {
            "goal_rules" => {
                goal_rules.push_str(trimmed);
                goal_rules.push('\n');
            }
            "interactions" => {
                interactions.push_str(trimmed);
                interactions.push('\n');
            }
            _ => {}
        }
    }
    (goal_rules.to_lowercase(), interactions.to_lowercase())
}

fn load_capabilities(root: &Path) -> Result<Vec<CapabilityInfo>> {
    let specs_root = root.join("givn/specs");
    let mut caps = Vec::new();
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
            let (goal_rules, interactions) = usecase_ownership_text(&usecase_dir);
            caps.push(CapabilityInfo {
                name: file
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
                    .to_string(),
                usecase: usecase.clone(),
                invocations,
                text: text.to_lowercase(),
                goal_rules,
                interactions,
            });
        }
    }
    Ok(caps)
}

/// Lexical overlap of the proposal with each usecase.md's Goal + Interactions
/// text, used for the `NEW in <usecase-id>` fallback.
fn usecase_affinity(root: &Path, proposal_words: &[String]) -> Result<BTreeMap<String, f64>> {
    let specs_root = root.join("givn/specs");
    let mut affinity = BTreeMap::new();
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
        let usecase_md = usecase_dir.join("usecase.md");
        let mut score = 0.0;
        if usecase_md.exists() {
            let text = std::fs::read_to_string(&usecase_md)?.to_lowercase();
            for word in proposal_words {
                if text.contains(word.as_str()) {
                    score += 1.0;
                }
            }
        }
        affinity.insert(usecase, score);
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
        let mut command_overlap = false;

        // 1. Command surface (strongest signal).
        for (sub, count) in &proposal_subcommands {
            if let Some(cap_count) = cap.invocations.get(sub) {
                command_overlap = true;
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

        // 3. Ownership evidence: the owning use case's Goal/Rules text.
        let mut ownership_hits = 0usize;
        for word in &proposal_words {
            if cap.goal_rules.contains(word.as_str()) {
                ownership_hits += 1;
                if ownership_hits >= 3 {
                    break;
                }
            }
        }
        if ownership_hits > 0 {
            score += ownership_hits as f64 * 2.0;
            evidence.push(format!(
                "{ownership_hits} goal or rules words match the owning use case"
            ));
        }

        // 4. Ownership evidence: the capability name and its owning use case.
        let name_lower = cap.name.to_lowercase();
        let usecase_lower = cap.usecase.to_lowercase();
        let named = proposal_words
            .iter()
            .any(|word| *word == name_lower || *word == usecase_lower)
            || proposal_tokens.iter().any(|token| {
                token.to_lowercase() == name_lower || token.to_lowercase() == usecase_lower
            });
        if named {
            score += 2.0;
            evidence.push(format!("proposal names `{}` or its use case", cap.name));
        }

        // 5. Ownership evidence: the capability's interaction consumer actions.
        let mut interaction_hits = 0usize;
        for word in &proposal_words {
            if cap.interactions.contains(word.as_str()) {
                interaction_hits += 1;
                if interaction_hits >= 3 {
                    break;
                }
            }
        }
        if interaction_hits > 0 {
            score += interaction_hits as f64;
            evidence.push(format!(
                "{interaction_hits} interaction words match the capability"
            ));
        }

        // 6. Lexical overlap with capability text (capped).
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

        // Scatter penalty: no goal ownership and no command overlap.
        if ownership_hits == 0 && !command_overlap {
            score *= 0.5;
            evidence.push("no goal ownership for this proposal".to_string());
        }

        if score > 0.0 {
            candidates.push(RouteCandidate {
                capability: cap.name.clone(),
                usecase: cap.usecase.clone(),
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

    // 7. Use-case affinity for the NEW fallback.
    let affinity = usecase_affinity(root, &proposal_words)?;
    let mut affinity_ranked: Vec<(String, f64)> = affinity.into_iter().collect();
    affinity_ranked.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    let top_affinity = affinity_ranked.first().cloned();
    let affinity_tied = match (&top_affinity, affinity_ranked.get(1)) {
        (Some((_, a)), Some((_, b))) => (a - b).abs() < 0.01,
        _ => false,
    };
    let suggested_usecase = top_affinity
        .clone()
        .filter(|(_, score)| *score > 0.0)
        .map(|(usecase, _)| usecase);

    // 8. Confidence.
    let (confidence, note) = if candidates.is_empty() {
        (
            RouteConfidence::NoSignal,
            Some("no command, token, or lexical overlap".to_string()),
        )
    } else if candidates.len() >= 2 && (candidates[0].score - candidates[1].score).abs() < 0.01 {
        (
            RouteConfidence::NoSignal,
            Some(format!("the top candidates tie at {:.2}", candidates[0].score)),
        )
    } else if candidates[0].score >= 3.0 {
        (RouteConfidence::Signal, None)
    } else if let Some((_, affinity_score)) = &top_affinity {
        if *affinity_score >= 2.0 && !affinity_tied {
            (RouteConfidence::Signal, None)
        } else {
            (
                RouteConfidence::NoSignal,
                Some("no use-case affinity clears the signal floor".to_string()),
            )
        }
    } else {
        (
            RouteConfidence::NoSignal,
            Some("no use-case affinity signal".to_string()),
        )
    };

    Ok(RouteReport {
        candidates,
        suggested_usecase,
        confidence,
        note,
    })
}

/// Print the report with an EXTEND / NEW recommendation and its confidence.
pub fn print_report(report: &RouteReport) {
    if report.confidence == RouteConfidence::NoSignal {
        let reason = report.note.as_deref().unwrap_or("no clear winner");
        println!("recommendation: no signal — {reason}");
        println!(
            "\nno ranked list is printed without signal; record the decision in the \
proposal's Capability Routing table with its rationale."
        );
        return;
    }

    let top = report.candidates.first();
    let extend = top.filter(|c| c.score >= 3.0);
    match extend {
        Some(top) => {
            println!(
                "recommendation: EXTEND {} (use case '{}', score {:.2}) — confidence: signal",
                top.capability, top.usecase, top.score
            );
        }
        None => match &report.suggested_usecase {
            Some(usecase) => {
                println!("recommendation: NEW in {usecase} — confidence: signal");
            }
            None => println!("recommendation: NEW (no use-case affinity signal)"),
        },
    }
    println!("\nranked candidates:");
    for c in &report.candidates {
        println!(
            "  {:.2}  {} (use case {})",
            c.score, c.capability, c.usecase
        );
        for e in c.evidence.iter().take(3) {
            println!("         - {e}");
        }
    }
    println!("\nrecord the decision in the proposal's Capability Routing table.");
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
        assert_eq!(report.candidates[0].usecase, "linters");
        assert!(report.candidates[0].score >= 3.0, "extend bar");
    }

    #[test]
    fn goal_aligned_capability_ranks_above_lexically_similar_candidate() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("alignment")).unwrap();
        std::fs::write(
            base.join("alignment/coverage-tool.feature"),
            "Feature: Coverage tool\n\n  Scenario: One\n    Given a coverage report exists\n    Then the command exits 0\n",
        )
        .unwrap();
        std::fs::write(
            base.join("alignment/usecase.md"),
            "# Use case: alignment\n\n## Level\n\n!\n\n## Goal\n\nValidate coverage reports.\n\n## Rules\n\n- Reports stay current.\n",
        )
        .unwrap();
        std::fs::create_dir_all(base.join("gates")).unwrap();
        std::fs::write(
            base.join("gates/lint-tool.feature"),
            "Feature: Lint tool\n\n  Scenario: One\n    Given a coverage report exists\n    And a coverage report is stale\n    Then the command exits 0\n",
        )
        .unwrap();
        std::fs::write(
            base.join("gates/usecase.md"),
            "# Use case: gates\n\n## Level\n\n!\n\n## Goal\n\nCheck the tool.\n\n## Rules\n\n- Keep it deterministic.\n",
        )
        .unwrap();
        let report = route(root.path(), "Add coverage report validation.").unwrap();
        assert_eq!(report.confidence, RouteConfidence::Signal);
        assert_eq!(report.candidates[0].capability, "coverage-tool");
        assert!(
            report.candidates[0].score >= 3.0,
            "aligned candidate must clear the extend bar: {:?}",
            report.candidates
        );
        let mismatched = report
            .candidates
            .iter()
            .find(|c| c.capability == "lint-tool")
            .expect("mismatched candidate present");
        assert!(
            mismatched.score < report.candidates[0].score,
            "goal-aligned candidate must outrank the lexically similar one"
        );
    }

    #[test]
    fn tied_top_candidates_report_no_signal() {
        let root = tempfile::TempDir::new().unwrap();
        let base = root.path().join("givn/specs");
        std::fs::create_dir_all(base.join("gates")).unwrap();
        for capability in ["lint-tool", "lint-check"] {
            std::fs::write(
                base.join(format!("gates/{capability}.feature")),
                "Feature: Lint\n\n  Scenario: One\n    When the user runs `givn lint`\n",
            )
            .unwrap();
        }
        std::fs::write(
            base.join("gates/usecase.md"),
            "# Use case: gates\n\n## Level\n\n!\n\n## Goal\n\nCheck the tool.\n\n## Rules\n\n- Keep it deterministic.\n",
        )
        .unwrap();
        let report = route(root.path(), "Extend the `givn lint` command.").unwrap();
        assert_eq!(report.confidence, RouteConfidence::NoSignal);
        assert!(
            report.note.as_deref().unwrap_or_default().contains("tie"),
            "note={:?}",
            report.note
        );
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
