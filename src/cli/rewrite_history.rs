//! The explicit, opt-in history rewrite: remove generated rules from the
//! project's reachable history through `git-filter-repo`, keeping every
//! working-tree file. Never automatic.

use anyhow::{Context, Result, bail};
use std::io::IsTerminal;
use std::process::Command;

/// Run the rewrite command. `yes` skips the interactive confirmation.
pub fn run(yes: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let policy = crate::config::overlay::load_rules_policy(&project_root)?;
    if !policy.is_local_only() {
        bail!("the rules are versioned by policy; nothing to purge");
    }
    let backend = crate::scm::select(&project_root)?;
    if matches!(backend, crate::scm::Backend::Fossil(_)) {
        bail!(
            "history rewriting is not available on the Fossil backend; the cleanliness report still names the affected revisions"
        );
    }
    if !backend.dirty_entries()?.is_empty() {
        bail!(
            "history rewriting requires a clean checkout; commit or stash the pending changes first"
        );
    }

    let report = crate::skills::rules::cleanliness(&project_root)?;
    if report.history.is_empty() {
        println!("no generated rule in reachable history; nothing to rewrite");
        return Ok(());
    }
    let paths: Vec<String> = report
        .history
        .iter()
        .map(|(path, _)| path.clone())
        .collect();
    let revisions: usize = report.history.iter().map(|(_, count)| count).sum();
    println!("history rewriting will purge {} rule path(s):", paths.len());
    for path in &paths {
        println!("  - {path}");
    }
    println!("  affected reachable revisions: {revisions}");

    if !yes {
        if !std::io::stdin().is_terminal() {
            bail!("confirmation required: re-run with --yes");
        }
        let confirmed = dialoguer::Confirm::new()
            .with_prompt("Rewrite reachable history now?")
            .interact()?;
        if !confirmed {
            bail!("history rewrite cancelled");
        }
    }

    require_filter_repo(&project_root)?;

    let mut args: Vec<String> = vec![
        "filter-repo".to_string(),
        "--invert-paths".to_string(),
        "--force".to_string(),
    ];
    for path in &paths {
        args.push("--path".to_string());
        args.push(path.clone());
    }
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = Command::new("git")
        .args(&refs)
        .current_dir(&project_root)
        .output()
        .context("run git filter-repo")?;
    if !output.status.success() {
        bail!(
            "git filter-repo failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let after = crate::skills::rules::cleanliness(&project_root)?;
    if !after.history.is_empty() {
        bail!(
            "the reachable history still contains a generated rule after the rewrite; nothing was published"
        );
    }
    let revision = backend.revision()?;
    println!("history rewritten; new revision {revision}");
    println!(
        "rule files are still on disk; force-push is required to publish the rewritten history"
    );
    println!("remote repositories and forks are outside the guarantee");
    Ok(())
}

fn require_filter_repo(project_root: &std::path::Path) -> Result<()> {
    let output = Command::new("git")
        .args(["filter-repo", "--version"])
        .current_dir(project_root)
        .output();
    match output {
        Ok(output) if output.status.success() => Ok(()),
        _ => bail!(
            "history rewriting requires git-filter-repo (https://github.com/newren/git-filter-repo); install it and re-run"
        ),
    }
}
