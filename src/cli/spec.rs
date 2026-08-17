use std::path::PathBuf;

use anyhow::{Result, bail};

pub fn run_index() -> Result<()> {
    crate::spec::retrieval::index(&std::env::current_dir()?)
}

pub fn run_search(query: &str) -> Result<()> {
    crate::spec::retrieval::search(&std::env::current_dir()?, query)
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

pub fn run_review(change_id: &str) -> Result<()> {
    crate::spec::retrieval::review(&std::env::current_dir()?, change_id)
}

pub fn run_duplicates(json: bool) -> Result<()> {
    crate::spec::retrieval::duplicates(&std::env::current_dir()?, json)
}
