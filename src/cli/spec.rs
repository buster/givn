use anyhow::{Result, bail};

pub fn run_coverage(baseline: Option<&std::path::Path>, json: bool) -> Result<()> {
    let root = std::env::current_dir()?;
    let current = crate::spec::coverage::snapshot(&root)?;
    if let Some(path) = baseline {
        let baseline_text = std::fs::read_to_string(path).map_err(|error| {
            anyhow::anyhow!(
                "failed to read coverage baseline {}: {error}",
                path.display()
            )
        })?;
        let before: crate::spec::coverage::CoverageSnapshot = serde_json::from_str(&baseline_text)?;
        let comparison = crate::spec::coverage::compare(&before, &current);
        if json {
            println!("{}", serde_json::to_string_pretty(&comparison)?);
        } else {
            println!(
                "specification coverage: {}",
                if comparison.passed {
                    "passed"
                } else {
                    "failed"
                }
            );
            println!(
                "  behavior entries: {} → {}",
                comparison.before_entries, comparison.after_entries
            );
            println!(
                "  @e2e evidence: {} → {}",
                comparison.before_e2e, comparison.after_e2e
            );
            println!(
                "  interactions: {} → {}",
                comparison.before_interactions, comparison.after_interactions
            );
            for missing in &comparison.missing_behavior {
                println!("  missing behavior: {missing}");
            }
            for lost in &comparison.lost_e2e {
                println!("  lost @e2e evidence: {lost}");
            }
        }
        if !comparison.passed {
            bail!("specification coverage regressed");
        }
    } else if json {
        println!("{}", serde_json::to_string_pretty(&current)?);
    } else {
        println!(
            "specification coverage: {} behavior entries, {} @e2e",
            current.entries.len(),
            current.entries.iter().filter(|entry| entry.e2e).count()
        );
    }
    Ok(())
}
pub fn run_duplicates(json: bool) -> Result<()> {
    // Deterministic path only (Phase 4.3): exact/structural matches from
    // the parsed AST + overlap fingerprint. No embeddings, no score.
    let root = std::env::current_dir()?;
    let matches = crate::spec::tree::duplicate_matches(&root)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&matches)?);
        return Ok(());
    }
    println!(
        "scanned permanent corpus: {} exact/structural match(es)",
        matches.len()
    );
    for m in &matches {
        println!(
            "  [{}] '{}' ({}) <> '{}' ({})",
            m.kind, m.left_title, m.left_file, m.right_title, m.right_file
        );
    }
    Ok(())
}

pub fn run_route(change: Option<&str>, file: Option<&std::path::Path>) -> Result<()> {
    if change.is_some() == file.is_some() {
        bail!("spec route requires exactly one input: use --change or --file");
    }
    let root = std::env::current_dir()?;
    let proposal_path = match (change, file) {
        (Some(id), _) => {
            let change_dir = crate::state::resolve_change_dir(&root, id)?;
            change_dir.join("proposal.md")
        }
        (_, Some(path)) => path.to_path_buf(),
        _ => bail!("unreachable"),
    };
    let proposal_text = std::fs::read_to_string(&proposal_path)
        .map_err(|e| anyhow::anyhow!("failed to read proposal {}: {e}", proposal_path.display()))?;
    let report = crate::spec::route::route(&root, &proposal_text)?;
    crate::spec::route::print_report(&report);
    Ok(())
}

/// `givn spec find <query>` — lexical search over the parsed AST (Phase 4.2).
pub fn run_find(query: &str) -> Result<()> {
    let hits = crate::spec::tree::find_scenarios(&std::env::current_dir()?, query)?;
    if hits.is_empty() {
        println!("no scenarios match '{query}'");
        return Ok(());
    }
    println!("{} scenario(s) match '{query}':", hits.len());
    for hit in &hits {
        println!(
            "  [{}] {} / {} — {}",
            if hit.e2e { "e2e" } else { "   " },
            hit.usecase,
            hit.capability,
            hit.scenario
        );
    }
    Ok(())
}

/// `givn spec tree` — deterministic corpus discovery (Phase 4.1).
pub fn run_tree(json: bool) -> Result<()> {
    let rows = crate::spec::tree::build_tree(&std::env::current_dir()?)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
    } else {
        crate::spec::tree::print_tree(&rows);
    }
    Ok(())
}

/// `givn spec propose-usecases` — print the command-surface proposal.
/// Never modifies files; a human reviews, confirms, and applies it via the
/// migration step (retirement plan task 2.5).
pub fn run_propose_usecases(json: bool) -> Result<()> {
    let proposal = crate::spec::cluster::cluster_tree(&std::env::current_dir()?);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "usecases": proposal
                    .usecases
                    .iter()
                    .map(|(sub, caps)| serde_json::json!({
                        "dominant_subcommand": sub,
                        "capabilities": caps,
                    }))
                    .collect::<Vec<_>>(),
                "fragments": proposal.fragments,
            }))?
        );
        return Ok(());
    }
    println!("Proposed use cases (command-surface clustering)");
    println!("Review and confirm use-case identities before applying.");
    println!("This command never modifies files.\n");
    for (sub, caps) in &proposal.usecases {
        println!("use case '{sub}' ({} capabilities):", caps.len());
        for cap in caps {
            println!("  - {cap}");
        }
    }
    if !proposal.fragments.is_empty() {
        println!(
            "\nfragments ({} capability with no dominant command surface —",
            proposal.fragments.len()
        );
        println!("«include» fragment candidates, not use-case candidates):");
        for f in &proposal.fragments {
            println!("  - {f}");
        }
    }
    Ok(())
}
