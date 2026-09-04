use std::path::PathBuf;

use anyhow::{Result, bail};

pub fn run_index() -> Result<()> {
    crate::spec::retrieval::index(&std::env::current_dir()?)
}

pub fn run_search(query: &str, evidence: bool) -> Result<()> {
    crate::spec::retrieval::search(&std::env::current_dir()?, query, evidence)
}

pub fn run_explore(
    file: Option<PathBuf>,
    stdin: bool,
    change: Option<&str>,
    json: bool,
) -> Result<()> {
    if file.is_some() == stdin {
        bail!("spec explore requires exactly one input: use --file or --stdin");
    }

    crate::spec::retrieval::explore(
        &std::env::current_dir()?,
        file.as_deref(),
        stdin,
        change,
        json,
    )
}

#[cfg(feature = "spec-retrieval")]
pub fn run_review(change_id: &str, verbose: bool, json: bool) -> Result<()> {
    crate::spec::retrieval::review_with_options(
        &std::env::current_dir()?,
        change_id,
        crate::spec::retrieval::ReviewOptions {
            verbose,
            json,
            findings: true,
        },
    )
}

#[cfg(not(feature = "spec-retrieval"))]
pub fn run_review(change_id: &str, _verbose: bool, _json: bool) -> Result<()> {
    let _ = change_id;
    anyhow::bail!(
        "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
    )
}

pub fn run_duplicates(json: bool) -> Result<()> {
    crate::spec::retrieval::duplicates(&std::env::current_dir()?, json)
}

pub fn run_bge_parity() -> Result<()> {
    crate::spec::retrieval::bge_parity(&std::env::current_dir()?)
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

/// `givn spec regroup` — print the command-surface clustering proposal.
/// Never modifies files; a human reviews, confirms, and applies it via the
/// migration step (retirement plan task 2.5).
pub fn run_regroup(json: bool) -> Result<()> {
    let proposal = crate::spec::cluster::cluster_tree(&std::env::current_dir()?);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "groups": proposal
                    .groups
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
    println!("Proposed use-case grouping (command-surface clustering)");
    println!("Rename groups to use-case-flavoured labels before applying.");
    println!("This command never modifies files.\n");
    for (sub, caps) in &proposal.groups {
        println!("group '{sub}' ({} capabilities):", caps.len());
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
