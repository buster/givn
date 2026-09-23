//! Release-owned migration catalog and project upgrade orchestration.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub const MIGRATION_CATALOG_PATH: &str = "migrations/catalog.yaml";

const MIGRATION_CATALOG_SCHEMA_VERSION: u32 = 1;
const MIGRATION_GUIDE: &str = include_str!("../../docs/migrations/0.2.0-to-0.3.0.md");

/// A release-owned migration bundle. The companion assets must contain all
/// information needed by an LLM to perform that phase in a consumer project.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MigrationBundle {
    pub id: String,
    pub source_version: String,
    pub target_version: String,
    pub order: u32,
    pub proposal_asset: String,
    pub design_asset: String,
}

/// The embedded catalog of migration bundles shipped with the binary.
#[derive(Debug, Clone, Deserialize)]
pub struct MigrationCatalog {
    pub schema_version: u32,
    pub migrations: Vec<MigrationBundle>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ReleaseVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ReleaseVersion {
    fn parse(value: &str) -> Result<Self> {
        let core = value.split_once('-').map(|(v, _)| v).unwrap_or(value);
        let mut parts = core.split('.');
        let major = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("version '{value}' has no major component"))?
            .parse()
            .with_context(|| format!("version '{value}' has an invalid major component"))?;
        let minor = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("version '{value}' has no minor component"))?
            .parse()
            .with_context(|| format!("version '{value}' has an invalid minor component"))?;
        let patch = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("version '{value}' has no patch component"))?
            .parse()
            .with_context(|| format!("version '{value}' has an invalid patch component"))?;
        if parts.next().is_some() {
            bail!("version '{value}' has more than three numeric components");
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    fn same_minor(self, other: Self) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

/// Load and validate the release-owned migration catalog.
pub fn load_catalog() -> Result<MigrationCatalog> {
    let yaml = crate::config::embedded::Assets::get_str(MIGRATION_CATALOG_PATH)
        .ok_or_else(|| anyhow::anyhow!("embedded migration catalog not found"))?;
    let catalog: MigrationCatalog =
        serde_yaml_ng::from_str(&yaml).context("parse embedded migration catalog")?;
    if catalog.schema_version != MIGRATION_CATALOG_SCHEMA_VERSION {
        bail!(
            "unsupported migration catalog schema version {}",
            catalog.schema_version
        );
    }
    let mut ids = HashSet::new();
    let mut sources = HashSet::new();
    for bundle in &catalog.migrations {
        let source = ReleaseVersion::parse(&bundle.source_version)?;
        if bundle.target_version == "<target-version>" {
            if !ids.insert(bundle.id.clone()) {
                bail!(
                    "migration catalog contains duplicate bundle '{}'",
                    bundle.id
                );
            }
            if !sources.insert(source) {
                bail!(
                    "migration catalog contains multiple bundles from source version '{}'",
                    bundle.source_version
                );
            }
            validate_bundle_assets(bundle)?;
            continue;
        }
        let target = ReleaseVersion::parse(&bundle.target_version)?;
        if target <= source {
            bail!(
                "migration bundle '{}' does not advance from {} to {}",
                bundle.id,
                bundle.source_version,
                bundle.target_version
            );
        }
        if !ids.insert(bundle.id.clone()) {
            bail!(
                "migration catalog contains duplicate bundle '{}'",
                bundle.id
            );
        }
        if !sources.insert(source) {
            bail!(
                "migration catalog contains multiple bundles from source version '{}'",
                bundle.source_version
            );
        }
        if bundle.id.is_empty()
            || bundle.proposal_asset.is_empty()
            || bundle.design_asset.is_empty()
        {
            bail!("migration catalog contains an incomplete bundle definition");
        }
        validate_bundle_assets(bundle)?;
    }
    Ok(catalog)
}

fn validate_bundle_assets(bundle: &MigrationBundle) -> Result<()> {
    if crate::config::embedded::Assets::get(&bundle.proposal_asset).is_none() {
        bail!(
            "migration bundle '{}' references missing proposal asset '{}'",
            bundle.id,
            bundle.proposal_asset
        );
    }
    if crate::config::embedded::Assets::get(&bundle.design_asset).is_none() {
        bail!(
            "migration bundle '{}' references missing design asset '{}'",
            bundle.id,
            bundle.design_asset
        );
    }
    Ok(())
}

/// Resolve the declared migration bundles covering a version gap.
///
/// An empty result means that no migration route is declared for the
/// transition, so the caller should perform only a managed upgrade. Once a
/// route starts, an incomplete route is an error rather than a silent skip.
pub fn migration_path(source: &str, target: &str) -> Result<Vec<MigrationBundle>> {
    let source = ReleaseVersion::parse(source)?;
    let target = ReleaseVersion::parse(target)?;
    let catalog = load_catalog()?;
    resolve_migration_path(&catalog, source, target)
}

fn resolve_migration_path(
    catalog: &MigrationCatalog,
    source: ReleaseVersion,
    target: ReleaseVersion,
) -> Result<Vec<MigrationBundle>> {
    if source >= target {
        return Ok(Vec::new());
    }

    let mut current = source;
    let mut path = Vec::new();
    let mut seen = HashSet::new();

    loop {
        // A bundle targeting a minor release covers later compatible patch
        // releases in that minor line.
        if current >= target || (!path.is_empty() && current.same_minor(target)) {
            return Ok(path);
        }

        let mut candidates: Vec<&MigrationBundle> = catalog
            .migrations
            .iter()
            .filter(|bundle| {
                let Ok(bundle_source) = ReleaseVersion::parse(&bundle.source_version) else {
                    return false;
                };
                let Ok(bundle_target) = ReleaseVersion::parse(&bundle.target_version) else {
                    return false;
                };
                bundle_source == current && bundle_target > current && bundle_target <= target
            })
            .collect();
        candidates.sort_by(|left, right| {
            left.order
                .cmp(&right.order)
                .then_with(|| left.target_version.cmp(&right.target_version))
                .then_with(|| left.id.cmp(&right.id))
        });

        let Some(bundle) = candidates.first() else {
            if path.is_empty() {
                return Ok(path);
            }
            bail!(
                "migration catalog has no route from {} toward {}",
                format_version(current),
                format_version(target)
            );
        };

        if !seen.insert(bundle.id.clone()) {
            bail!(
                "migration catalog contains a cycle at bundle '{}'",
                bundle.id
            );
        }
        current = ReleaseVersion::parse(&bundle.target_version)?;
        path.push((*bundle).clone());
    }
}

fn format_version(version: ReleaseVersion) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

/// Create the deterministic aggregate maintenance-change id for a version gap.
pub fn aggregate_migration_id(source: &str, target: &str) -> String {
    format!(
        "migrate-{}-to-{}",
        sanitize_version_for_id(source),
        sanitize_version_for_id(target)
    )
}

fn sanitize_version_for_id(version: &str) -> String {
    version
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}

/// Whether a specific aggregate migration plan exists in active or archived
/// storage.
pub fn migration_exists(project_root: &Path, change_id: &str) -> bool {
    project_root.join("givn/changes").join(change_id).exists()
        || project_root.join("givn/archive").join(change_id).exists()
}

/// Find an active aggregate migration plan created by a previous upgrade.
///
/// The config marker is advanced by the managed preparation, so later binary
/// upgrades cannot identify a pending plan from the marker alone. The visible
/// `migration.yaml` file is the durable pending-work signal.
pub fn pending_migration_change(project_root: &Path) -> Option<String> {
    let changes_dir = project_root.join("givn/changes");
    let mut ids: Vec<String> = std::fs::read_dir(changes_dir)
        .ok()?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_dir() || !path.join("migration.yaml").is_file() {
                return None;
            }
            entry.file_name().to_str().map(str::to_string)
        })
        .collect();
    ids.sort();
    ids.into_iter().next()
}

/// Run a declared migration path after the managed upgrade commit.
pub fn prepare_legacy_upgrade(
    project_root: &Path,
    config_path: &Path,
    existing_config: &str,
    source_version: &str,
    target_version: &str,
    bundles: &[MigrationBundle],
) -> Result<()> {
    let change_id = aggregate_migration_id(source_version, target_version);
    let migration_dir = project_root.join("givn/changes").join(&change_id);

    if migration_exists(project_root, &change_id) {
        println!(
            "Migration change '{change_id}' already exists; preserving its proposal and design."
        );
        return Ok(());
    }

    let commit_a = prepare_managed_upgrade(
        project_root,
        config_path,
        existing_config,
        Some(source_version),
        target_version,
        &format!("chore(givn): prepare managed upgrade for {source_version} to {target_version}"),
    )?
    .ok_or_else(|| anyhow::anyhow!("managed migration preparation produced no commit"))?;

    write_aggregate_migration_plan(
        &migration_dir,
        &commit_a,
        source_version,
        target_version,
        bundles,
    )?;
    let commit_b = commit_migration_plan(project_root, &migration_dir, &change_id)?;

    println!("Prepared Givn {target_version} migration from {source_version}.");
    println!("  managed upgrade commit (A): {commit_a}");
    println!("  aggregate migration commit (B): {commit_b}");
    println!("  migration change: givn/changes/{change_id}");
    println!(
        "  migration phases: {}",
        bundles
            .iter()
            .map(|bundle| bundle.id.as_str())
            .collect::<Vec<_>>()
            .join(" -> ")
    );
    println!(
        "Next: run `givn status --change {change_id}` and complete the ordered LLM-guided migration."
    );

    Ok(())
}

/// Refresh and commit managed files for a migration-free or migration-bearing
/// upgrade. Returns `None` when the refresh produced no diff.
pub fn prepare_managed_upgrade(
    project_root: &Path,
    config_path: &Path,
    existing_config: &str,
    source_version: Option<&str>,
    target_version: &str,
    commit_message: &str,
) -> Result<Option<String>> {
    let backend = crate::scm::select(project_root)?;
    if !backend.dirty_entries()?.is_empty() {
        bail!("automatic managed upgrade requires a clean checkout; no files were changed");
    }

    let managed_paths = crate::skills::targets::managed_generated_paths(project_root)?;
    let commands_path = project_root.join("givn/commands.yaml");
    let snapshot = FileSnapshot::capture(
        std::iter::once(config_path.to_path_buf())
            .chain(std::iter::once(commands_path.clone()))
            .chain(managed_paths.iter().cloned()),
    )?;

    let upgraded = crate::cli::upgrade::upgrade_config(existing_config, target_version)?;
    if let Err(error) = apply_managed_upgrade(config_path, &upgraded, project_root) {
        snapshot.restore().ok();
        return Err(error);
    }

    match commit_managed_upgrade(
        project_root,
        &backend,
        config_path,
        &managed_paths,
        commit_message,
    ) {
        Ok(hash) => Ok(hash),
        Err(error) => {
            snapshot.restore().ok();
            // The transition may have staged index removals before the
            // failure; reset the index to the restored revision state.
            backend.restore_index().ok();
            let source = source_version.unwrap_or("unknown");
            Err(error.context(format!(
                "managed upgrade from {source} to {target_version} was not committed"
            )))
        }
    }
}

fn apply_managed_upgrade(config_path: &Path, upgraded: &str, project_root: &Path) -> Result<()> {
    std::fs::write(config_path, upgraded)
        .with_context(|| format!("write managed config '{}'", config_path.display()))?;

    crate::skills::targets::sync(project_root)
        .context("synchronize generated Givn skills and commands")?;

    // Run backfill: an existing project may predate the run declaration.
    // Idempotent — a commands file that already declares `run:` is left
    // untouched, so a repeated upgrade produces no diff and no commit.
    let commands_path = project_root.join("givn/commands.yaml");
    if commands_path.exists() {
        let existing = std::fs::read_to_string(&commands_path)
            .with_context(|| format!("read managed commands file '{}'", commands_path.display()))?;
        if let Some(backfilled) = crate::cli::upgrade::backfill_commands(&existing) {
            std::fs::write(&commands_path, backfilled).with_context(|| {
                format!("write managed commands file '{}'", commands_path.display())
            })?;
            println!(
                "  wrote    run: scaffold to givn/commands.yaml — fill it in; `givn archive` is blocked until you do"
            );
        }
    }

    let report = crate::skills::rules::cleanliness(project_root)?;
    crate::skills::rules::print_report("upgrade", &report)?;

    Ok(())
}

fn commit_managed_upgrade(
    project_root: &Path,
    backend: &crate::scm::Backend,
    config_path: &Path,
    managed_paths: &[PathBuf],
    commit_message: &str,
) -> Result<Option<String>> {
    let policy = crate::config::overlay::load_rules_policy(project_root)?;
    let mut paths = vec![
        config_path.to_path_buf(),
        project_root.join("givn/commands.yaml"),
    ];
    for extra in [".gitignore", ".fossil-settings/ignore-glob"] {
        let path = project_root.join(extra);
        if path.exists() {
            paths.push(path);
        }
    }
    if !policy.is_local_only() {
        // The versioned policy stages the generated rules through the
        // calculated allowlist; `AGENTS.md` is never staged (Q3).
        paths.extend(
            managed_paths
                .iter()
                .filter(|path| path.strip_prefix(project_root).ok() != Some(Path::new("AGENTS.md")))
                .cloned(),
        );
    }
    let relative = relative_managed_paths(project_root, paths)?;
    backend
        .stage_paths(&relative, !policy.is_local_only())
        .context("the selected backend could not stage the managed upgrade files")?;

    if !backend.has_staged_changes()? {
        return Ok(None);
    }

    backend
        .commit_staged(commit_message)
        .context("the selected backend could not create the managed upgrade revision")?;
    Ok(Some(backend.revision()?))
}

/// Existing managed paths, relative to the project root. Path filtering is
/// project knowledge and stays here; staging is the backend's operation.
fn relative_managed_paths<I>(project_root: &Path, paths: I) -> Result<Vec<PathBuf>>
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
        bail!("automatic upgrade produced no managed files to commit");
    }
    Ok(relative_paths)
}

fn write_aggregate_migration_plan(
    migration_dir: &Path,
    commit_a: &str,
    source_version: &str,
    target_version: &str,
    bundles: &[MigrationBundle],
) -> Result<()> {
    let parent = migration_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("migration directory has no parent"))?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("create migration parent '{}'", parent.display()))?;
    let change_id = migration_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("migration");
    let temporary_dir = parent.join(format!(".{change_id}.tmp-{}", std::process::id()));
    if temporary_dir.exists() {
        std::fs::remove_dir_all(&temporary_dir).with_context(|| {
            format!(
                "remove stale migration staging directory '{}'",
                temporary_dir.display()
            )
        })?;
    }
    std::fs::create_dir(&temporary_dir).with_context(|| {
        format!(
            "create migration staging directory '{}'",
            temporary_dir.display()
        )
    })?;

    let result = write_aggregate_migration_files(
        &temporary_dir,
        change_id,
        commit_a,
        source_version,
        target_version,
        bundles,
    );
    if let Err(error) = result {
        std::fs::remove_dir_all(&temporary_dir).ok();
        return Err(error);
    }

    std::fs::rename(&temporary_dir, migration_dir)
        .with_context(|| format!("publish migration directory '{}'", migration_dir.display()))?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct AggregateMigrationMetadata<'a> {
    schema_version: u32,
    source_version: &'a str,
    target_version: &'a str,
    tool_version: &'a str,
    upgrade_commit: &'a str,
    bundles: Vec<AggregateBundleMetadata<'a>>,
}

#[derive(Debug, Serialize)]
struct AggregateBundleMetadata<'a> {
    sequence: usize,
    id: &'a str,
    source_version: &'a str,
    target_version: &'a str,
    order: u32,
}

fn write_aggregate_migration_files(
    migration_dir: &Path,
    change_id: &str,
    commit_a: &str,
    source_version: &str,
    target_version: &str,
    bundles: &[MigrationBundle],
) -> Result<()> {
    std::fs::write(migration_dir.join(".givn-skip"), "specs\n")
        .context("write migration skip marker")?;

    let metadata = AggregateMigrationMetadata {
        schema_version: MIGRATION_CATALOG_SCHEMA_VERSION,
        source_version,
        target_version,
        tool_version: target_version,
        upgrade_commit: commit_a,
        bundles: bundles
            .iter()
            .enumerate()
            .map(|(index, bundle)| AggregateBundleMetadata {
                sequence: index + 1,
                id: &bundle.id,
                source_version: &bundle.source_version,
                target_version: &bundle.target_version,
                order: bundle.order,
            })
            .collect(),
    };
    std::fs::write(
        migration_dir.join("migration.yaml"),
        serde_yaml_ng::to_string(&metadata).context("render migration metadata")?,
    )
    .context("write migration metadata")?;

    std::fs::write(
        migration_dir.join("proposal.md"),
        aggregate_migration_proposal(change_id, source_version, target_version, commit_a, bundles)?,
    )
    .context("write migration proposal")?;
    std::fs::write(
        migration_dir.join("design.md"),
        aggregate_migration_design(change_id, source_version, target_version, commit_a, bundles)?,
    )
    .context("write migration design")?;
    Ok(())
}

fn aggregate_migration_proposal(
    change_id: &str,
    source_version: &str,
    target_version: &str,
    commit_a: &str,
    bundles: &[MigrationBundle],
) -> Result<String> {
    let mut output = format!(
        "# Proposal: {change_id}\n\n\
## Aggregate migration\n\n\
This maintenance change migrates a project from Givn {source_version} to the \
installed Givn {target_version} contract. Managed preparation was recorded in \
Commit A `{commit_a}`. The migration phases below are one ordered plan; work \
each phase in order and follow the completion rules in its bundle.\n\n\
## Scope\n\n\
The LLM must inspect project evidence and derive project-specific tasks. It must \
preserve project-owned overrides and must not invent domain behavior, tests,\
architecture decisions, or provenance.\n\n\
## Bundle proposals\n\n"
    );
    for (index, bundle) in bundles.iter().enumerate() {
        output.push_str(&format!(
            "### Phase {}: {} ({} -> {})\n\n",
            index + 1,
            bundle.id,
            bundle.source_version,
            bundle.target_version
        ));
        output.push_str(&render_bundle_asset(
            &bundle.proposal_asset,
            source_version,
            target_version,
            commit_a,
        )?);
        output.push_str("\n\n");
    }
    Ok(output)
}

fn aggregate_migration_design(
    change_id: &str,
    source_version: &str,
    target_version: &str,
    commit_a: &str,
    bundles: &[MigrationBundle],
) -> Result<String> {
    let mut output = format!(
        "# Design: {change_id}\n\n\
## Migration metadata\n\n\
- source_version: `{source_version}`\n\
- target_version: `{target_version}`\n\
- upgrade_commit: `{commit_a}`\n\
- migration_type: aggregate maintenance change without a product specification\n\n\
## Execution order\n\n\
Work the following phases strictly in sequence. Follow each bundle's own \
completion rules; a bundle may direct that the repairs run as named follow-up \
changes. The complete sequence is also recorded in `migration.yaml`.\n\n"
    );
    for (index, bundle) in bundles.iter().enumerate() {
        output.push_str(&format!(
            "### Phase {}: {} ({} -> {})\n\n",
            index + 1,
            bundle.id,
            bundle.source_version,
            bundle.target_version
        ));
        output.push_str(&render_bundle_asset(
            &bundle.design_asset,
            source_version,
            target_version,
            commit_a,
        )?);
        output.push_str("\n\n");
    }
    output.push_str(
        "## Completion boundary\n\nThis generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood, including any follow-up changes a bundle names.\n",
    );
    Ok(output)
}

fn render_bundle_asset(
    asset_path: &str,
    source_version: &str,
    target_version: &str,
    commit_a: &str,
) -> Result<String> {
    let content = crate::config::embedded::Assets::get_str(asset_path)
        .ok_or_else(|| anyhow::anyhow!("embedded migration asset not found: {asset_path}"))?;
    Ok(content
        .replace("{{SOURCE_VERSION}}", source_version)
        .replace("{{TARGET_VERSION}}", target_version)
        .replace("{{UPGRADE_COMMIT}}", commit_a)
        .replace("{{MIGRATION_GUIDE}}", MIGRATION_GUIDE))
}

fn commit_migration_plan(
    project_root: &Path,
    migration_dir: &Path,
    change_id: &str,
) -> Result<String> {
    let backend = crate::scm::select(project_root)?;
    let relative = relative_managed_paths(project_root, [migration_dir.to_path_buf()])?;
    backend
        .stage_paths(&relative, true)
        .context("the selected backend could not stage the aggregate migration plan")?;
    backend
        .commit_staged(&format!(
            "docs(upgrade): record aggregate migration plan {change_id}"
        ))
        .context("the selected backend could not create the aggregate migration-plan revision")?;
    backend
        .revision()
        .context("read the migration-plan revision")
}

struct FileSnapshot {
    files: Vec<(PathBuf, Option<Vec<u8>>)>,
}

impl FileSnapshot {
    fn capture<I>(paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = PathBuf>,
    {
        let mut seen = HashSet::new();
        let mut files = Vec::new();
        for path in paths {
            if !seen.insert(path.clone()) {
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
            files.push((path, content));
        }
        Ok(Self { files })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn bundle(id: &str, source: &str, target: &str, order: u32) -> MigrationBundle {
        MigrationBundle {
            id: id.to_string(),
            source_version: source.to_string(),
            target_version: target.to_string(),
            order,
            proposal_asset: "migrations/0.2.0-to-0.3.0/proposal.md".to_string(),
            design_asset: "migrations/0.2.0-to-0.3.0/design.md".to_string(),
        }
    }

    #[test]
    fn catalog_contains_the_0_2_to_0_3_bundle() {
        let catalog = load_catalog().unwrap();
        assert_eq!(catalog.schema_version, MIGRATION_CATALOG_SCHEMA_VERSION);
        assert!(catalog.migrations.iter().any(|migration| {
            migration.id == "migrate-0-2-0-to-0-3-0"
                && migration.source_version == "0.2.0"
                && migration.target_version == "0.3.0"
        }));
    }

    #[test]
    fn migration_path_routes_0_7_0_to_the_current_release() {
        let path = migration_path("0.7.0", env!("CARGO_PKG_VERSION")).unwrap();
        assert_eq!(
            path.iter()
                .map(|bundle| bundle.id.as_str())
                .collect::<Vec<_>>(),
            ["migrate-0-7-0-to-0-8-0"]
        );
    }

    #[test]
    fn aggregate_proposal_preserves_word_spacing() {
        let bundles = vec![bundle("migrate-0-2-0-to-0-3-0", "0.2.0", "0.3.0", 10)];
        let proposal = aggregate_migration_proposal(
            "migrate-0-2-0-to-0-3-0",
            "0.2.0",
            "0.3.0",
            "abc123",
            &bundles,
        )
        .unwrap();
        assert!(proposal.contains("to the installed Givn 0.3.0 contract"));
        assert!(proposal.contains("recorded in Commit A `abc123`"));
        assert!(proposal.contains("plan; work each phase in order"));
        assert!(proposal.contains("It must preserve project-owned overrides"));
        assert!(!proposal.contains("theinstalled"));
        assert!(!proposal.contains("inCommit"));
        assert!(!proposal.contains("workeach"));
        assert!(!proposal.contains("mustpreserve"));
    }

    #[test]
    fn migration_path_orders_multiple_bundles() {
        let catalog = MigrationCatalog {
            schema_version: MIGRATION_CATALOG_SCHEMA_VERSION,
            migrations: vec![
                bundle("second", "0.3.0", "0.4.0", 20),
                bundle("first", "0.2.0", "0.3.0", 10),
            ],
        };
        let path = resolve_migration_path(
            &catalog,
            ReleaseVersion::parse("0.2.0").unwrap(),
            ReleaseVersion::parse("0.4.0").unwrap(),
        )
        .unwrap();
        assert_eq!(
            path.iter()
                .map(|bundle| bundle.id.as_str())
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
    }

    #[test]
    fn migration_path_covers_a_skipped_compatible_patch_release() {
        let catalog = MigrationCatalog {
            schema_version: MIGRATION_CATALOG_SCHEMA_VERSION,
            migrations: vec![bundle("first", "0.2.0", "0.3.0", 10)],
        };
        let path = resolve_migration_path(
            &catalog,
            ReleaseVersion::parse("0.2.0").unwrap(),
            ReleaseVersion::parse("0.3.1").unwrap(),
        )
        .unwrap();
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn migration_free_patch_release_has_no_path() {
        let catalog = MigrationCatalog {
            schema_version: MIGRATION_CATALOG_SCHEMA_VERSION,
            migrations: vec![bundle("first", "0.2.0", "0.3.0", 10)],
        };
        let path = resolve_migration_path(
            &catalog,
            ReleaseVersion::parse("0.3.0").unwrap(),
            ReleaseVersion::parse("0.3.1").unwrap(),
        )
        .unwrap();
        assert!(path.is_empty());
    }

    #[test]
    fn aggregate_design_keeps_bundle_phases_in_order() {
        let bundles = vec![
            bundle("first", "0.2.0", "0.3.0", 10),
            bundle("second", "0.3.0", "0.4.0", 20),
        ];
        let design = aggregate_migration_design(
            "migrate-0-2-0-to-0-4-0",
            "0.2.0",
            "0.4.0",
            "commit-a",
            &bundles,
        )
        .unwrap();
        let first = design.find("### Phase 1: first").unwrap();
        let second = design.find("### Phase 2: second").unwrap();
        assert!(first < second, "{design}");
        assert!(design.contains("## Prompt 1: Inventory"));
    }

    #[test]
    fn aggregate_titles_use_the_change_id_not_the_staging_dir() {
        let root = tempfile::tempdir().unwrap();
        let staging = root.path().join(".migrate-0-6-0-to-0-7-0.tmp-1");
        std::fs::create_dir(&staging).unwrap();
        let bundles = vec![bundle("migrate-0-6-0-to-0-7-0", "0.6.0", "0.7.0", 50)];
        write_aggregate_migration_files(
            &staging,
            "migrate-0-6-0-to-0-7-0",
            "commit-a",
            "0.6.0",
            "0.7.0",
            &bundles,
        )
        .unwrap();
        let proposal = std::fs::read_to_string(staging.join("proposal.md")).unwrap();
        let design = std::fs::read_to_string(staging.join("design.md")).unwrap();
        assert!(
            proposal.starts_with("# Proposal: migrate-0-6-0-to-0-7-0"),
            "{proposal}"
        );
        assert!(
            design.starts_with("# Design: migrate-0-6-0-to-0-7-0"),
            "{design}"
        );
        assert!(!proposal.contains(".tmp-"), "{proposal}");
        assert!(!design.contains(".tmp-"), "{design}");
    }

    #[test]
    fn aggregate_id_is_stable_for_semver_values() {
        assert_eq!(
            aggregate_migration_id("0.2.0", "0.3.1"),
            "migrate-0-2-0-to-0-3-1"
        );
    }

    #[test]
    fn managed_upgrade_without_changes_does_not_create_empty_commit() {
        let directory = tempfile::tempdir().unwrap();
        let config = directory.path().join("givn/config.yaml");
        std::fs::create_dir_all(config.parent().unwrap()).unwrap();
        std::fs::write(&config, "givn_config_version: \"0.3.0\"\n").unwrap();

        for args in [
            ["init"].as_slice(),
            ["config", "user.name", "Givn Test"].as_slice(),
            ["config", "user.email", "givn@example.test"].as_slice(),
            ["add", "--", "givn/config.yaml"].as_slice(),
            ["commit", "-m", "initial"].as_slice(),
        ] {
            let output = Command::new("git")
                .current_dir(directory.path())
                .args(args)
                .output()
                .unwrap();
            assert!(output.status.success(), "git {:?} failed", args);
        }

        let backend = crate::scm::select(directory.path()).unwrap();
        let before = backend.revision().unwrap();
        let result = commit_managed_upgrade(
            directory.path(),
            &backend,
            &config,
            std::slice::from_ref(&config),
            "chore(givn): refresh managed files",
        )
        .unwrap();

        assert_eq!(result, None);
        assert_eq!(backend.revision().unwrap(), before);
    }
}
