use anyhow::{Context, Result, bail};
use std::path::{Component, Path, PathBuf};

pub fn run(message: &str, paths: &[String]) -> Result<()> {
    if paths.is_empty() {
        bail!("explicit paths are required: givn commit <path>... -m <message>");
    }
    let project_root = std::env::current_dir()?.canonicalize()?;
    let paths = validate_paths(&project_root, paths)?;
    let backend = crate::scm::select(&project_root)?;
    let stamped = crate::cli::attribution::stamped_message(message);
    let revision = match &backend {
        crate::scm::Backend::Git(worktree) => worktree.commit_selected(&stamped, &paths),
        crate::scm::Backend::Fossil(checkout) => checkout.commit_selected(&stamped, &paths),
    }
    .context("the selected revision was rejected")?;
    println!(
        "[{}] committed revision {} on the {} backend",
        crate::subprocess::local_timestamp(),
        revision,
        match backend {
            crate::scm::Backend::Git(_) => "Git",
            crate::scm::Backend::Fossil(_) => "Fossil",
        }
    );
    Ok(())
}

fn validate_paths(root: &Path, paths: &[String]) -> Result<Vec<String>> {
    paths
        .iter()
        .map(|path| {
            let relative = PathBuf::from(path);
            if path.is_empty()
                || path.starts_with(':')
                || relative.components().any(|part| {
                    matches!(
                        part,
                        Component::ParentDir | Component::RootDir | Component::Prefix(_)
                    )
                })
            {
                bail!("unsafe explicit path '{path}': select paths inside the checkout");
            }
            let full = root.join(&relative);
            let mut ancestor = full.as_path();
            while !ancestor.exists() {
                ancestor = ancestor
                    .parent()
                    .context("explicit path has no existing parent")?;
            }
            if !ancestor.canonicalize()?.starts_with(root) {
                bail!("unsafe explicit path '{path}': selection escapes the checkout");
            }
            Ok(path.clone())
        })
        .collect()
}
