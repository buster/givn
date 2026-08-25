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
