//! Shared Git fixtures for the llm-attribution-installation capability.

use std::path::Path;
use std::process::{Command, Output};

pub(crate) fn git(root: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git")
}
