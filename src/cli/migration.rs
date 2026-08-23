//! Automatic preparation for the 0.2.0 to 0.3.0 project migration.

use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub const LEGACY_VERSION: &str = "0.2.0";
pub const TARGET_VERSION: &str = "0.3.0";
pub const MIGRATION_CHANGE_ID: &str = "migrate-0-2-0-to-0-3-0";

const MIGRATION_GUIDE: &str = include_str!("../../docs/migrations/0.2.0-to-0.3.0.md");

/// Whether the deterministic migration plan already exists in active or
/// archived storage.
pub fn migration_exists(project_root: &Path) -> bool {
    project_root
        .join("givn/changes")
        .join(MIGRATION_CHANGE_ID)
        .exists()
        || project_root
            .join("givn/archive")
            .join(MIGRATION_CHANGE_ID)
            .exists()
}

/// Prepare a legacy project and create the two commits that delimit the
/// machine-safe upgrade from the project-specific migration work.
pub fn prepare_legacy_upgrade(
    project_root: &Path,
    config_path: &Path,
    existing_config: &str,
) -> Result<()> {
    let migration_dir = project_root.join("givn/changes").join(MIGRATION_CHANGE_ID);

    if migration_exists(project_root) {
        println!(
            "Migration change '{MIGRATION_CHANGE_ID}' already exists; preserving its proposal and design."
        );
        return Ok(());
    }

    ensure_clean_git(project_root)?;

    let managed_paths = crate::skills::targets::managed_generated_paths(project_root)?;
    let snapshot = FileSnapshot::capture(
        std::iter::once(config_path.to_path_buf()).chain(managed_paths.iter().cloned()),
    )?;

    let upgraded = crate::cli::upgrade::upgrade_config(existing_config, TARGET_VERSION)?;
    if let Err(error) = apply_managed_upgrade(project_root, config_path, &upgraded) {
        snapshot.restore().ok();
        return Err(error);
    }

    let commit_a = match commit_managed_upgrade(project_root, config_path, &managed_paths) {
        Ok(hash) => hash,
        Err(error) => {
            snapshot.restore().ok();
            return Err(error);
        }
    };

    write_migration_plan(&migration_dir, &commit_a)?;
    if let Err(error) = commit_migration_plan(project_root, &migration_dir) {
        eprintln!(
            "Commit A ({commit_a}) succeeded. The migration plan remains visible at {} for manual review.",
            migration_dir.display()
        );
        return Err(error);
    }

    let commit_b = git_commit_hash(project_root)?;
    println!("Prepared Givn {TARGET_VERSION} migration from {LEGACY_VERSION}.");
    println!("  managed upgrade commit (A): {commit_a}");
    println!("  migration plan commit (B):  {commit_b}");
    println!("  migration change: givn/changes/{MIGRATION_CHANGE_ID}");
    println!(
        "Next: run `givn status --change {MIGRATION_CHANGE_ID}` and complete the LLM-guided migration."
    );

    Ok(())
}

fn apply_managed_upgrade(project_root: &Path, config_path: &Path, upgraded: &str) -> Result<()> {
    std::fs::write(config_path, upgraded)
        .with_context(|| format!("write managed config '{}'", config_path.display()))?;

    crate::skills::targets::sync(project_root)
        .context("synchronize generated Givn skills and commands")?;

    Ok(())
}

fn commit_managed_upgrade(
    project_root: &Path,
    config_path: &Path,
    managed_paths: &[PathBuf],
) -> Result<String> {
    let mut paths = vec![config_path.to_path_buf()];
    paths.extend(managed_paths.iter().cloned());
    stage_paths(project_root, paths)?;

    let output = git(
        project_root,
        [
            "commit",
            "-m",
            "chore(givn): prepare 0.2.0 to 0.3.0 migration",
        ],
    )?;
    if !output.status.success() {
        bail!(
            "Git could not create the managed upgrade commit: {}",
            command_output(&output)
        );
    }
    git_commit_hash(project_root)
}

fn write_migration_plan(migration_dir: &Path, commit_a: &str) -> Result<()> {
    std::fs::create_dir_all(migration_dir)
        .with_context(|| format!("create migration directory '{}'", migration_dir.display()))?;
    std::fs::write(migration_dir.join(".givn-skip"), "specs\n")
        .context("write migration skip marker")?;
    std::fs::write(
        migration_dir.join("proposal.md"),
        migration_proposal(commit_a),
    )
    .context("write migration proposal")?;
    std::fs::write(migration_dir.join("design.md"), migration_design(commit_a))
        .context("write migration design")?;
    Ok(())
}

fn commit_migration_plan(project_root: &Path, migration_dir: &Path) -> Result<()> {
    stage_paths(project_root, [migration_dir.to_path_buf()])?;
    let output = git(
        project_root,
        [
            "commit",
            "-m",
            "docs(upgrade): record 0.2.0 to 0.3.0 migration plan",
        ],
    )?;
    if !output.status.success() {
        bail!(
            "Git could not create the migration-plan commit: {}",
            command_output(&output)
        );
    }
    Ok(())
}

fn stage_paths<I>(project_root: &Path, paths: I) -> Result<()>
where
    I: IntoIterator<Item = PathBuf>,
{
    let relative_paths: Vec<PathBuf> = paths
        .into_iter()
        .filter(|path| path.exists())
        .map(|path| {
            path.strip_prefix(project_root)
                .map(Path::to_path_buf)
                .with_context(|| {
                    format!(
                        "managed path '{}' is outside project root '{}'",
                        path.display(),
                        project_root.display()
                    )
                })
        })
        .collect::<Result<Vec<_>>>()?;

    if relative_paths.is_empty() {
        bail!("automatic migration produced no managed files to commit");
    }

    let mut args = vec!["add".to_string(), "--".to_string()];
    args.extend(
        relative_paths
            .iter()
            .map(|path| path.to_string_lossy().into_owned()),
    );
    let output = git(project_root, args.iter().map(String::as_str))?;
    if !output.status.success() {
        bail!(
            "Git could not stage managed migration files: {}",
            command_output(&output)
        );
    }
    Ok(())
}

fn ensure_clean_git(project_root: &Path) -> Result<()> {
    let repository =
        git(project_root, ["rev-parse", "--is-inside-work-tree"]).map_err(|error| {
            anyhow::anyhow!("automatic 0.2.0 to 0.3.0 migration requires a Git repository: {error}")
        })?;
    if !repository.status.success() || String::from_utf8_lossy(&repository.stdout).trim() != "true"
    {
        bail!(
            "automatic 0.2.0 to 0.3.0 migration requires a Git repository; no files were changed"
        );
    }

    let status = git(
        project_root,
        ["status", "--porcelain", "--untracked-files=all"],
    )?;
    if !status.status.success() {
        bail!(
            "could not inspect Git worktree before migration: {}",
            command_output(&status)
        );
    }
    if !status.stdout.is_empty() {
        bail!(
            "automatic 0.2.0 to 0.3.0 migration requires a clean Git worktree; no files were changed"
        );
    }
    Ok(())
}

fn git<I, S>(project_root: &Path, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new("git")
        .current_dir(project_root)
        .args(args)
        .output()
        .context("run Git command for automatic migration")
}

fn git_commit_hash(project_root: &Path) -> Result<String> {
    let output = git(project_root, ["rev-parse", "HEAD"])?;
    if !output.status.success() {
        bail!(
            "could not read the automatic migration commit hash: {}",
            command_output(&output)
        );
    }
    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        bail!("Git returned an empty automatic migration commit hash");
    }
    Ok(hash)
}

fn command_output(output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    match (stdout.is_empty(), stderr.is_empty()) {
        (true, true) => format!("exit code {}", output.status.code().unwrap_or(-1)),
        (false, true) => stdout,
        (true, false) => stderr,
        (false, false) => format!("{stdout}; {stderr}"),
    }
}

fn migration_proposal(commit_a: &str) -> String {
    format!(
        "# Proposal: {MIGRATION_CHANGE_ID}\n\n\
## Problem / Opportunity\n\n\
This project was created from Givn {LEGACY_VERSION}, while the installed tool\
now implements the {TARGET_VERSION} contract. The machine-safe scaffold and\
generated guidance can be refreshed automatically, but project-specific\
specification, test, and architecture decisions require evidence from this\
repository.\n\n\
## Proposed Solution\n\n\
Bring this project to the Givn {TARGET_VERSION} contract through the ordered\
LLM prompts in `design.md`. Preserve project-owned overrides and user content,\
update only applicable active changes and durable documentation, and complete\
the normal Givn review and archive gates. The managed preparation was recorded\
in Commit A `{commit_a}`.\n\n\
## Out of Scope\n\n\
- Inventing domain behavior or business invariants.\n\
- Automatically editing domain specifications, step definitions, source code,\
  ADR decisions, Arc42 facts, ejected facets, or user-authored documents.\n\
- Adding a synthetic migration feature to `givn/specs/`.\n\n\
## Open Questions\n\n\
The LLM must resolve which of the 0.3.0 contracts apply to this project by\
inspecting its evidence before deriving tasks.\n"
    )
}

fn migration_design(commit_a: &str) -> String {
    format!(
        "# Design: {MIGRATION_CHANGE_ID}\n\n\
## Migration Metadata\n\n\
- source_version: `{LEGACY_VERSION}`\n\
- target_version: `{TARGET_VERSION}`\n\
- upgrade_commit: `{commit_a}`\n\
- migration_type: maintenance change without a product specification\n\n\
## Machine-safe Preparation\n\n\
Commit A `{commit_a}` contains the managed config scaffold and generated agent\
integrations selected by the project. Commit B will contain this migration\
plan. Inspect both commits before making project-specific edits. The clean Git\
preflight and path allowlist are part of the upgrade contract.\n\n\
## LLM Prompt Pack\n\n\
The following is the canonical migration guide shipped with Givn {TARGET_VERSION}.\
Execute its prompts in order, adapting only where the project inventory proves\
that a surface does not apply.\n\n\
{MIGRATION_GUIDE}\n\n\
## Completion Boundary\n\n\
This design intentionally does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`,\
or an implementation commit. The project LLM must derive tasks, perform review,\
and record actual evidence before the maintenance change is archived.\n"
    )
}

struct FileSnapshot {
    files: Vec<(PathBuf, Option<Vec<u8>>)>,
}

impl FileSnapshot {
    fn capture<I>(paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = PathBuf>,
    {
        let mut unique = HashMap::new();
        for path in paths {
            if unique.contains_key(&path) {
                continue;
            }
            let content = if path.exists() {
                Some(
                    std::fs::read(&path)
                        .with_context(|| format!("snapshot managed path '{}'", path.display()))?,
                )
            } else {
                None
            };
            unique.insert(path, content);
        }
        Ok(Self {
            files: unique.into_iter().collect(),
        })
    }

    fn restore(&self) -> Result<()> {
        for (path, content) in &self.files {
            match content {
                Some(bytes) => std::fs::write(path, bytes)
                    .with_context(|| format!("restore managed path '{}'", path.display()))?,
                None if path.exists() => std::fs::remove_file(path)
                    .with_context(|| format!("remove new managed path '{}'", path.display()))?,
                None => {}
            }
        }
        Ok(())
    }
}
