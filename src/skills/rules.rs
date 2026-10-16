//! Generated-rules policy application: ignore entries, the local-only
//! transition, and the cleanliness report.
//!
//! Under `local-only` the strict rules stay on the developer's machine. Their
//! paths are kept out of the repository through two destinations: a fenced
//! block in the tracked ignore file and a local exclude. A rule that is
//! already tracked becomes local-only without deleting its file. Under
//! `versioned` both fenced blocks are removed and nothing is transitioned.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::overlay::RulesPolicy;

const BLOCK_BEGIN: &str = "# givn:begin:rules";
const BLOCK_END: &str = "# givn:end:rules";

/// One managed ignore block: the fence that opens and closes its entries. The
/// rules policy and the development record each own a block, so neither writer
/// edits the other's entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Fence {
    /// The line that opens the block.
    pub(crate) begin: &'static str,
    /// The line that closes the block.
    pub(crate) end: &'static str,
}

/// The rules policy's block.
pub(crate) const RULES_FENCE: Fence = Fence {
    begin: BLOCK_BEGIN,
    end: BLOCK_END,
};

/// The development record's own block: the record is data, not a generated
/// rule, so it never shares the rules block.
pub(crate) const HISTORY_FENCE: Fence = Fence {
    begin: crate::history::HISTORY_BLOCK_BEGIN,
    end: crate::history::HISTORY_BLOCK_END,
};

/// The backend state at the project root, decided by the markers the root
/// itself carries — never by `scm::select` succeeding: selection also refuses
/// a nested root, both markers, or a missing executable, and a select-keyed
/// branch would write a versioned file while a worktree exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RootBackend {
    /// `.git` at the project root.
    Git,
    /// `.fslckout` or `_FOSSIL_` at the project root.
    Fossil,
    /// Neither marker: no repository exists yet (`givn init` in a bare
    /// directory).
    Bare,
    /// Both markers at the root: ambiguous, and neither writer touches it.
    Ambiguous,
}

/// The backend state the project root's own markers describe.
pub(crate) fn root_backend(project_root: &Path) -> RootBackend {
    let git = project_root.join(".git").exists();
    let fossil = project_root.join(".fslckout").exists() || project_root.join("_FOSSIL_").exists();
    match (git, fossil) {
        (true, false) => RootBackend::Git,
        (false, true) => RootBackend::Fossil,
        (false, false) => RootBackend::Bare,
        (true, true) => RootBackend::Ambiguous,
    }
}

/// One ignore destination: a file that carries a fenced block, or Fossil's
/// repository-level `ignore-glob` setting (a deduplicated list, never a file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IgnoreDestination {
    /// A file whose entries live inside one fenced block.
    Fenced(PathBuf),
    /// The repository-level `ignore-glob` setting of a Fossil checkout.
    FossilSetting,
}

/// One planned write: where the entries go, the fence they live under, and the
/// entries themselves.
#[derive(Debug, Clone)]
pub(crate) struct IgnoreWrite {
    pub(crate) destination: IgnoreDestination,
    pub(crate) fence: Fence,
    pub(crate) entries: Vec<String>,
}

/// The writes the rules policy makes in this backend state: the
/// `# givn:begin:rules` block in the tracked ignore file and in the
/// repository-local destination the backend offers. Nothing before a
/// repository exists and nothing in an ambiguous root: the policy keeps rules
/// out of a repository, and neither state has one it owns.
fn rules_writes(project_root: &Path, entries: &[String]) -> Vec<IgnoreWrite> {
    let fenced = |path: PathBuf, entries: Vec<String>| IgnoreWrite {
        destination: IgnoreDestination::Fenced(path),
        fence: RULES_FENCE,
        entries,
    };
    match root_backend(project_root) {
        RootBackend::Git => {
            // The worktree-local file is resolved through Git; without it the
            // Git destinations are not written at all, the same refusal
            // selection makes for a worktree without its executable.
            let Ok(exclude) = git_path(project_root, "info/exclude") else {
                return Vec::new();
            };
            vec![
                fenced(project_root.join(".gitignore"), entries.to_vec()),
                fenced(exclude, entries.to_vec()),
            ]
        }
        RootBackend::Fossil => {
            if !fossil_executable() {
                return Vec::new();
            }
            // Fossil's `*` crosses `/`, so the trailing slash is cosmetic;
            // the stripped form is kept for `.fossil-settings` readability.
            // The managed file's entry list is project content, so the
            // record's entry joins it here: a committed file overrides the
            // repository-level setting.
            let mut fossil_entries: Vec<String> = entries
                .iter()
                .map(|entry| entry.trim_end_matches('/').to_string())
                .collect();
            if !fossil_entries
                .iter()
                .any(|entry| entry == crate::history::FOSSIL_IGNORE_ENTRY)
            {
                fossil_entries.push(crate::history::FOSSIL_IGNORE_ENTRY.to_string());
            }
            vec![
                fenced(
                    project_root.join(".fossil-settings/ignore-glob"),
                    fossil_entries,
                ),
                IgnoreWrite {
                    destination: IgnoreDestination::FossilSetting,
                    fence: RULES_FENCE,
                    entries: entries.to_vec(),
                },
            ]
        }
        RootBackend::Bare | RootBackend::Ambiguous => Vec::new(),
    }
}

/// The writes the record makes, re-asserted on every record write: the
/// record's own `# givn:begin:history` block in a destination no command can
/// dirty. Never a versioned file while a repository exists.
pub(crate) fn record_writes(project_root: &Path) -> Vec<IgnoreWrite> {
    let git_entries = || vec![crate::history::GIT_IGNORE_ENTRY.to_string()];
    match root_backend(project_root) {
        RootBackend::Git => git_path(project_root, "info/exclude")
            .map(|path| {
                vec![IgnoreWrite {
                    destination: IgnoreDestination::Fenced(path),
                    fence: HISTORY_FENCE,
                    entries: git_entries(),
                }]
            })
            .unwrap_or_default(),
        // A managed ignore file overrides the repository-level setting, so the
        // record's entry reaches it through the owning commands
        // (`givn init`, `givn skills sync`, `givn upgrade`); nothing is
        // written at plain-command time.
        RootBackend::Fossil if project_root.join(".fossil-settings/ignore-glob").exists() => {
            Vec::new()
        }
        RootBackend::Fossil => vec![IgnoreWrite {
            destination: IgnoreDestination::FossilSetting,
            fence: HISTORY_FENCE,
            entries: vec![crate::history::FOSSIL_IGNORE_ENTRY.to_string()],
        }],
        RootBackend::Bare => vec![IgnoreWrite {
            destination: IgnoreDestination::Fenced(project_root.join(".gitignore")),
            fence: HISTORY_FENCE,
            entries: git_entries(),
        }],
        RootBackend::Ambiguous => Vec::new(),
    }
}

/// Apply one planned write: a fenced block is replaced in place (an empty
/// entry list removes it); Fossil's setting is merged as a deduplicated list.
pub(crate) fn apply_write(project_root: &Path, write: &IgnoreWrite) -> Result<()> {
    match &write.destination {
        IgnoreDestination::Fenced(path) => upsert_block(path, write.fence, &write.entries),
        IgnoreDestination::FossilSetting => {
            upsert_fossil_setting(project_root, &write.entries);
            Ok(())
        }
    }
}

/// Whether the `fossil` executable is available: the Fossil destinations are
/// written through it, and a checkout without it is left alone — the same
/// refusal `scm::select` makes.
fn fossil_executable() -> bool {
    Command::new("fossil")
        .arg("version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Apply the ignore entries for `policy` to the active backend's
/// destinations and return the entries written (empty under `versioned`).
pub fn write_ignores(project_root: &Path, policy: RulesPolicy) -> Result<Vec<String>> {
    let entries = if policy.is_local_only() {
        ignore_entries()
    } else {
        Vec::new()
    };
    for write in rules_writes(project_root, &entries) {
        apply_write(project_root, &write)?;
    }
    Ok(entries)
}

/// The project-relative paths of every strict rule: all generated paths
/// except the `AGENTS.md` fence (the confirmed single exception).
pub fn strict_rules(project_root: &Path) -> Result<Vec<String>> {
    let mut entries: Vec<String> = crate::skills::targets::managed_generated_paths(project_root)?
        .into_iter()
        .filter_map(|path| path.strip_prefix(project_root).ok().map(Path::to_path_buf))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .filter(|path| path != "AGENTS.md")
        .collect();
    entries.sort();
    entries.dedup();
    Ok(entries)
}

/// The fenced ignore entries. They are prefix patterns, not per-file paths,
/// so the tracked block never enumerates individual skill or command names.
pub fn ignore_entries() -> Vec<String> {
    vec![
        ".opencode/skills/givn-*/".to_string(),
        ".agents/skills/givn-*/".to_string(),
        ".claude/skills/givn-*/".to_string(),
        ".omp/skills/givn-*/".to_string(),
        ".opencode/commands/givn-*.md".to_string(),
        ".opencode/commands/grill-*.md".to_string(),
        ".omp/commands/givn-*.md".to_string(),
        ".omp/commands/grill-*.md".to_string(),
        ".opencode/plugins/givn-attribution.js".to_string(),
        ".claude/hooks/givn-attribution.sh".to_string(),
        ".claude/settings.local.json".to_string(),
        ".omp/hooks/pre/givn-attribution.js".to_string(),
        ".githooks/commit-msg".to_string(),
    ]
}

/// Whether a project-relative path is a Givn rule, independent of the
/// current allowlist. Used to discover tracked rules that left the current
/// configuration (a disabled addon, a removed target).
pub fn is_rule_path(relative: &str) -> bool {
    if relative == ".githooks/commit-msg"
        || relative == ".opencode/plugins/givn-attribution.js"
        || relative == ".claude/hooks/givn-attribution.sh"
        || relative == ".claude/settings.local.json"
        || relative == ".omp/hooks/pre/givn-attribution.js"
    {
        return true;
    }
    for prefix in [".opencode/commands/", ".omp/commands/"] {
        if let Some(name) = relative.strip_prefix(prefix) {
            return name.starts_with("givn-") || name.starts_with("grill-");
        }
    }
    for prefix in [
        ".opencode/skills/",
        ".agents/skills/",
        ".claude/skills/",
        ".omp/skills/",
    ] {
        if let Some(rest) = relative.strip_prefix(prefix) {
            return rest.starts_with("givn-");
        }
    }
    false
}

/// The rules tracked at the current revision, discovered by the path
/// predicate so rules from a disabled addon or a removed target are found.
pub fn tracked_rule_paths(project_root: &Path) -> Result<Vec<String>> {
    let tracked = match crate::scm::select(project_root) {
        Ok(crate::scm::Backend::Git(_)) => git_listing(project_root, &["ls-files"])?,
        Ok(crate::scm::Backend::Fossil(_)) => {
            let output = Command::new("fossil")
                .args(["ls"])
                .current_dir(project_root)
                .output()
                .context("run fossil ls")?;
            if !output.status.success() {
                Vec::new()
            } else {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(str::to_string)
                    .collect()
            }
        }
        Err(_) => Vec::new(),
    };
    let mut rules: Vec<String> = tracked
        .into_iter()
        .map(|path| path.replace('\\', "/"))
        .filter(|path| is_rule_path(path))
        .collect();
    rules.sort();
    rules.dedup();
    Ok(rules)
}

fn git_listing(project_root: &Path, args: &[&str]) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(project_root)
        .output()
        .context("run git ls-files")?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect())
}

/// Make every tracked strict rule local-only: remove exactly those paths from
/// the current revision while keeping the files. Returns the paths removed.
/// Under `versioned` this is a no-op.
pub fn reconcile_tracked_rules(project_root: &Path, policy: RulesPolicy) -> Result<Vec<String>> {
    if !policy.is_local_only() {
        return Ok(Vec::new());
    }
    let Ok(backend) = crate::scm::select(project_root) else {
        // A project without a backend has no tracked rules.
        return Ok(Vec::new());
    };
    let tracked = tracked_rule_paths(project_root)?;
    if !tracked.is_empty() {
        let paths: Vec<PathBuf> = tracked.iter().map(PathBuf::from).collect();
        backend
            .remove_from_index(&paths)
            .context("the selected backend could not remove tracked rules from the revision")?;
    }
    Ok(tracked)
}

/// A cleanliness scan over the strict rules: tracked at the current
/// revision, and present in reachable history.
#[derive(Debug, Default)]
pub struct Cleanliness {
    pub policy: Option<RulesPolicy>,
    pub tracked: Vec<String>,
    pub history: Vec<(String, usize)>,
}

impl Cleanliness {
    pub fn is_clean(&self) -> bool {
        self.tracked.is_empty() && self.history.is_empty()
    }
}

/// Scan the strict rules for tracked and reachable-history occurrences.
pub fn cleanliness(project_root: &Path) -> Result<Cleanliness> {
    let policy = crate::config::overlay::load_rules_policy(project_root)?;
    let tracked = tracked_rule_paths(project_root)?;
    let mut entries = strict_rules(project_root)?;
    for path in &tracked {
        if !entries.contains(path) {
            entries.push(path.clone());
        }
    }
    entries.sort();
    let Ok(backend) = crate::scm::select(project_root) else {
        return Ok(Cleanliness {
            policy: Some(policy),
            tracked,
            history: Vec::new(),
        });
    };
    let history = backend.history_occurrences(&entries)?;
    Ok(Cleanliness {
        policy: Some(policy),
        tracked,
        history,
    })
}

/// Print the report shared by init, upgrade, sync, and the dedicated check.
/// Reporting never blocks.
pub fn print_report(prefix: &str, report: &Cleanliness) -> Result<()> {
    let policy = report
        .policy
        .ok_or_else(|| anyhow::anyhow!("cleanliness report without a resolved policy"))?;
    println!("  {prefix}: rules: {}", policy.label());
    if report.tracked.is_empty() {
        println!("  {prefix}: no tracked rule in the current revision");
    } else {
        println!(
            "  {prefix}: {} tracked rule(s) in the current revision:",
            report.tracked.len()
        );
        for path in &report.tracked {
            println!("    - {path}");
        }
    }
    let historical: usize = report.history.iter().map(|(_, count)| count).sum();
    if historical == 0 {
        println!("  {prefix}: no rule in reachable history");
    } else {
        println!("  {prefix}: {historical} reachable revision(s) still contain a rule:");
        for (path, count) in &report.history {
            println!("    - {path}: {count} revision(s)");
        }
    }
    println!(
        "  {prefix}: local reachable refs (including remote-tracking); remote repositories and forks are outside the guarantee"
    );
    Ok(())
}

/// Replace `fence`'s block in `path` with the given entries. An empty entry
/// list removes the block. Surrounding user content — including the other
/// writer's block — is preserved.
fn upsert_block(path: &Path, fence: Fence, entries: &[String]) -> Result<()> {
    let content = if path.exists() {
        std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?
    } else {
        String::new()
    };
    let updated = render_block(&content, fence, entries);
    if updated == content {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    std::fs::write(path, updated).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

/// Return `content` with `fence`'s block replaced by `entries`. An empty
/// entry list removes the block; absent markers append it at the end.
fn render_block(content: &str, fence: Fence, entries: &[String]) -> String {
    let mut result = String::new();
    let mut in_block = false;
    for line in content.lines() {
        if line.trim() == fence.begin {
            in_block = true;
            continue;
        }
        if line.trim() == fence.end {
            in_block = false;
            continue;
        }
        if !in_block {
            result.push_str(line);
            result.push('\n');
        }
    }
    if !entries.is_empty() {
        let trimmed = result.trim_end();
        result = if trimmed.is_empty() {
            String::new()
        } else {
            format!("{trimmed}\n\n")
        };
        result.push_str(fence.begin);
        result.push('\n');
        for entry in entries {
            result.push_str(entry);
            result.push('\n');
        }
        result.push_str(fence.end);
        result.push('\n');
    }
    result
}

/// Resolve a repository-local file through Git, so linked worktrees write the
/// right file (the checkout's `.git/info/exclude`, resolved with
/// `git rev-parse --git-path`).
fn git_path(project_root: &Path, path: &str) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", path])
        .current_dir(project_root)
        .output()
        .context("run git rev-parse")?;
    if !output.status.success() {
        anyhow::bail!("git rev-parse --git-path {path} failed");
    }
    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let candidate = PathBuf::from(&raw);
    Ok(if candidate.is_absolute() {
        candidate
    } else {
        project_root.join(candidate)
    })
}

/// Fossil repository-local ignore: the repository's `ignore-glob` setting,
/// which is not versioned (a clone inherits it through the repository file).
/// Existing user globs are preserved. Best effort: a missing or failing
/// Fossil leaves the tracked block in charge.
fn upsert_fossil_setting(project_root: &Path, entries: &[String]) {
    let current = fossil(project_root, &["settings", "--value", "ignore-glob"]);
    let Ok(current) = current else {
        return;
    };
    let mut globs: Vec<String> = current
        .split([',', '\n'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();
    let before = globs.len();
    for entry in entries {
        if !globs.contains(entry) {
            globs.push(entry.clone());
        }
    }
    if globs.len() == before {
        return;
    }
    let joined = globs.join(",");
    let _ = fossil(project_root, &["settings", "ignore-glob", &joined]);
}

fn fossil(project_root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("fossil")
        .args(args)
        .current_dir(project_root)
        .output()
        .context("run fossil")?;
    if !output.status.success() {
        anyhow::bail!(
            "fossil {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches_entry(path: &str, entry: &str) -> bool {
        if let Some(prefix) = entry.strip_suffix("*/") {
            return path.starts_with(prefix)
                && path.len() > prefix.len()
                && path[prefix.len()..].contains('/');
        }
        if let Some((prefix, suffix)) = entry.split_once('*') {
            return path.starts_with(prefix) && path.ends_with(suffix);
        }
        path == entry
    }

    #[test]
    fn ignore_entries_cover_every_rule_shape() {
        let rules = [
            ".opencode/skills/givn-propose/SKILL.md",
            ".agents/skills/givn-spec/SKILL.md",
            ".claude/skills/givn-review/SKILL.md",
            ".opencode/commands/givn-design.md",
            ".opencode/commands/grill-me.md",
            ".opencode/plugins/givn-attribution.js",
            ".claude/hooks/givn-attribution.sh",
            ".claude/settings.local.json",
            ".githooks/commit-msg",
        ];
        for path in rules {
            assert!(
                ignore_entries()
                    .iter()
                    .any(|entry| matches_entry(path, entry)),
                "no ignore entry covers {path}"
            );
        }
    }

    #[test]
    fn ignore_entries_never_match_user_content() {
        for path in ["AGENTS.md", ".agents/skills/gh-stack/SKILL.md"] {
            assert!(
                !ignore_entries()
                    .iter()
                    .any(|entry| matches_entry(path, entry)),
                "an ignore entry unexpectedly covers {path}"
            );
        }
    }
}
