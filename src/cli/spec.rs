use anyhow::Result;

pub fn run_index() -> Result<()> {
    crate::spec::retrieval::index(&std::env::current_dir()?)
}

pub fn run_search(query: &str) -> Result<()> {
    crate::spec::retrieval::search(&std::env::current_dir()?, query)
}

pub fn run_review(change_id: &str) -> Result<()> {
    crate::spec::retrieval::review(&std::env::current_dir()?, change_id)
}
