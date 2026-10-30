use crate::config::overlay::load_effective;
use crate::skills::render::{
    SkillContext, default_skill_names, manifest_addon_enabled, render_template,
    resolve_skill_template,
};
use anyhow::{Context, Result};
/// Write rendered skills to the configured target locations:
/// - `.opencode/skills/<name>/SKILL.md` when `opencode` is selected
/// - `.agents/skills/<name>/SKILL.md` when `agents` is selected
/// - `.claude/skills/<name>/SKILL.md` when `claude` is selected
/// - `.omp/skills/<name>/SKILL.md` when `omp` is selected
/// - `.maki/skills/<name>/SKILL.md` when `maki` is selected
///
/// The target set comes from the `skills:` block of `givn/config.yaml`
/// (default: `[opencode]`). See the agent-integration architecture documentation.
///
/// `AGENTS.md` at the repo root is otherwise user-owned. The only exception:
/// givn manages its own fenced `<!-- givn:begin:* -->` blocks inside it:
/// - `givn-project` — always present, a short pointer to `givn instructions`.
/// - `communication-policy` — present only when that addon is enabled.
///
/// Both are kept current on every call to `sync`.
use std::path::Path;

/// A canonical agent target for generated skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentTarget {
    /// `.opencode/` — opencode skills mirror.
    Opencode,
    /// `.agents/` — the neutral skills location.
    Agents,
    /// `.claude/` — Claude Code skills mirror.
    Claude,
    /// `.omp/` — the omp (Oh My Pi) skills and hook location.
    Omp,
    /// `.maki/` — the Maki skills, reporter module, and its merged
    /// `plugin.toml` grants / `init.lua` fence.
    Maki,
}

impl AgentTarget {
    /// All known targets.
    pub const ALL: [AgentTarget; 5] = [
        AgentTarget::Opencode,
        AgentTarget::Agents,
        AgentTarget::Claude,
        AgentTarget::Omp,
        AgentTarget::Maki,
    ];

    /// Canonical config name (no leading dot).
    pub fn name(&self) -> &'static str {
        match self {
            AgentTarget::Opencode => "opencode",
            AgentTarget::Agents => "agents",
            AgentTarget::Claude => "claude",
            AgentTarget::Omp => "omp",
            AgentTarget::Maki => "maki",
        }
    }

    /// Directory name under the project root.
    pub fn dir(&self) -> &'static str {
        match self {
            AgentTarget::Opencode => ".opencode",
            AgentTarget::Agents => ".agents",
            AgentTarget::Claude => ".claude",
            AgentTarget::Omp => ".omp",
            AgentTarget::Maki => ".maki",
        }
    }

    /// Name shown in the init prompt (fully-qualified directory form).
    pub fn display(&self) -> &'static str {
        self.dir()
    }

    /// Resolve a (possibly dotted) name to a target.
    pub fn from_name(name: &str) -> Option<Self> {
        match name
            .trim()
            .trim_start_matches('.')
            .to_ascii_lowercase()
            .as_str()
        {
            "opencode" => Some(AgentTarget::Opencode),
            "agents" => Some(AgentTarget::Agents),
            "claude" => Some(AgentTarget::Claude),
            "omp" => Some(AgentTarget::Omp),
            "maki" => Some(AgentTarget::Maki),
            _ => None,
        }
    }
}

/// Resolve the configured agent targets from `givn/config.yaml`.
fn configured_targets(project_root: &Path) -> Result<Vec<AgentTarget>> {
    // load_skills_targets already defaults to [opencode] when the config has
    // no skills block or an empty/invalid list, so the result never needs a
    // second fallback here.
    let names = crate::config::overlay::load_skills_targets(project_root)?;
    let mut targets = Vec::new();
    for name in names {
        if let Some(t) = AgentTarget::from_name(&name) {
            targets.push(t);
        }
    }
    Ok(targets)
}

/// Parse target names (dotted or bare form), deduplicated in first-seen
/// order. Returns the resolved targets and any names that matched no known
/// target; the caller decides how to report/handle unknowns.
pub fn parse_target_names<'a>(
    parts: impl Iterator<Item = &'a str>,
) -> (Vec<AgentTarget>, Vec<String>) {
    let mut targets: Vec<AgentTarget> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    for part in parts {
        match AgentTarget::from_name(part) {
            Some(t) if !targets.contains(&t) => targets.push(t),
            Some(_) => {}
            None => {
                let name = part.trim();
                if !name.is_empty() {
                    unknown.push(name.to_string());
                }
            }
        }
    }
    (targets, unknown)
}

/// Display names of every known target, for warning/help messages.
pub fn valid_names() -> Vec<&'static str> {
    AgentTarget::ALL.iter().map(|t| t.display()).collect()
}

/// Add the given targets to the `skills: targets:` list in
/// `givn/config.yaml`, preserving the rest of the file untouched. Returns
/// the full resulting target list.
pub fn add_targets(project_root: &Path, additions: &[AgentTarget]) -> Result<Vec<AgentTarget>> {
    let config_path = project_root.join("givn/config.yaml");
    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;

    validate_skills_yaml(&content)?;

    let mut merged = configured_targets(project_root)?;
    for t in additions {
        if !merged.contains(t) {
            merged.push(*t);
        }
    }
    let list = merged
        .iter()
        .map(|t| t.name())
        .collect::<Vec<_>>()
        .join(", ");
    let targets_line = format!("  targets: [{list}]");

    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();

    // Case 1: an uncommented `targets:` line inside an uncommented `skills:`
    // block already exists — replace it in place.
    let mut in_skills = false;
    for line in &mut lines {
        if in_skills {
            if !line.starts_with(' ') {
                in_skills = false;
            } else if line.trim_start().starts_with("targets:") {
                *line = targets_line.clone();
                std::fs::write(&config_path, lines.join("\n") + "\n")?;
                return Ok(merged);
            }
        }
        if line.trim() == "skills:" {
            in_skills = true;
        }
    }

    // Case 2: `skills:` block exists but has no `targets:` line — insert one.
    for i in 0..lines.len() {
        if lines[i].trim() == "skills:" {
            lines.insert(i + 1, targets_line.clone());
            std::fs::write(&config_path, lines.join("\n") + "\n")?;
            return Ok(merged);
        }
    }

    // Case 3: no `skills:` block at all — append a new one.
    let mut content = content;
    if !content.ends_with('\n') {
        content.push('\n');
    }
    if !content.ends_with("\n\n") {
        content.push('\n');
    }
    content.push_str(&format!("skills:\n{targets_line}\n"));
    std::fs::write(&config_path, content)?;
    Ok(merged)
}

/// Refuse YAML presentations this line-preserving editor cannot update safely.
fn validate_skills_yaml(content: &str) -> Result<()> {
    let mut skills_sections = 0usize;
    let mut in_skills = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if trimmed == "skills:" {
            skills_sections += 1;
            in_skills = true;
            if skills_sections > 1 {
                anyhow::bail!(
                    "unsupported YAML: givn/config.yaml contains more than one active skills section; edit it by hand"
                );
            }
            continue;
        }
        if trimmed.starts_with("skills:") {
            anyhow::bail!(
                "unsupported YAML: the skills section is not a block mapping; edit givn/config.yaml by hand"
            );
        }
        if !in_skills {
            continue;
        }
        if !line.starts_with(' ') {
            in_skills = false;
            continue;
        }
        if let Some(value) = trimmed.strip_prefix("targets:") {
            let value = value.trim();
            if value.is_empty()
                || value.starts_with('&')
                || value.starts_with('*')
                || value.starts_with('{')
            {
                anyhow::bail!(
                    "unsupported YAML: the skills targets value cannot be updated safely; edit givn/config.yaml by hand"
                );
            }
        }
    }
    Ok(())
}

/// Regenerate all skills in the project.
pub fn sync(project_root: &Path) -> Result<()> {
    crate::cli::git::install(project_root)?;

    let manifest = load_effective()?;
    let verify_command = &manifest.verify.command;
    let features_path = &manifest.verify.features_path;
    let version = env!("CARGO_PKG_VERSION");
    let targets = configured_targets(project_root)?;

    let addons = addon_flags(&manifest);
    let domain_skill = domain_modeling_present(&manifest);
    let mut skill_names = default_skill_names();
    if domain_skill {
        skill_names.push("givn-domain-modeling");
    }
    if manifest_addon_enabled(&manifest, "dev_principles") {
        skill_names.push("givn-dev-principles");
    }
    if manifest_addon_enabled(&manifest, "challenge") {
        skill_names.push("givn-challenge");
    }

    // Render and write each skill to the configured targets.
    let ctx = SkillContext {
        change_id: "<change-id>",
        verify_command,
        features_path,
        givn_version: version,
    };

    sync_agent_integrations(project_root, &targets)?;

    for name in &skill_names {
        let template = resolve_skill_template_for(name)?;
        let rendered = render_template(&template, &ctx, &addons)
            .with_context(|| format!("failed to render skill template for '{name}'"))?;

        // Validate frontmatter.
        validate_frontmatter(&rendered, name)?;

        write_skill_to_targets(project_root, name, &rendered, &targets)?;
        println!("  synced skill: {name}");
    }

    // The generated invocation surface is skills-only. Remove recognizable
    // Givn command files from earlier releases and leave custom files alone.
    reconcile_legacy_commands(project_root, &targets)?;

    // Self-reconcile: remove managed generated files that are no longer valid
    // for the current configuration.
    reconcile_addon_artifacts(project_root, &manifest, &targets)?;

    // Always-on blocks: unconditional, refreshed every sync.
    crate::cli::project_hint::apply(project_root)?;

    // Self-healing AGENTS.md block: keep it present/absent in sync with the
    // addon flag on every init/sync, independent of how it got out of sync.
    sync_communication_policy(
        project_root,
        manifest_addon_enabled(&manifest, "communication_policy"),
    )?;

    // Generated-rules policy: keep the ignore destinations current and make
    // tracked rules local-only. `AGENTS.md` is never staged by Givn.
    let policy = crate::config::overlay::load_rules_policy(project_root)?;
    crate::skills::rules::write_ignores(project_root, policy)?;
    let removed = crate::skills::rules::reconcile_tracked_rules(project_root, policy)?;
    if !removed.is_empty() {
        println!(
            "  rules policy: {} tracked rule(s) removed from the current revision (files kept)",
            removed.len()
        );
        for path in &removed {
            println!("    - {path}");
        }
    }
    let report = crate::skills::rules::cleanliness(project_root)?;
    crate::skills::rules::print_report("sync", &report)?;

    Ok(())
}

/// Whether the domain-modeling skill should be generated. It follows the
/// domain-modeling addon and does not require arc42.
fn domain_modeling_present(manifest: &crate::config::manifest::Manifest) -> bool {
    manifest.addons.domain_modeling
}

/// Build the addon-name/enabled map passed into the template renderer.
fn addon_flags(manifest: &crate::config::manifest::Manifest) -> Vec<(&'static str, bool)> {
    crate::skills::render::KNOWN_ADDON_KEYS
        .iter()
        .map(|key| (*key, manifest_addon_enabled(manifest, key)))
        .collect()
}

/// The two managed AGENTS.md block names: the current communication policy and
/// the retired absolute-mode block replaced during synchronization.
const COMMUNICATION_BLOCK: &str = "communication-policy";
const LEGACY_COMMUNICATION_BLOCK: &str = "absolute-mode";

/// Keep the communication-policy block current, remove the retired
/// absolute-mode block while preserving everything outside both blocks, and
/// report the replacement so an author can review the outside content.
fn sync_communication_policy(project_root: &Path, enabled: bool) -> Result<()> {
    let agents_path = project_root.join("AGENTS.md");
    let had_legacy = std::fs::read_to_string(&agents_path)
        .map(|content| content.contains("<!-- givn:begin:absolute-mode -->"))
        .unwrap_or(false);

    if enabled {
        let policy =
            crate::config::embedded::Assets::get_str("addons/communication-policy/policy.md")
                .ok_or_else(|| anyhow::anyhow!("embedded communication policy asset not found"))?;
        crate::cli::agents_block::apply(&agents_path, COMMUNICATION_BLOCK, &policy)?;
    } else {
        crate::cli::agents_block::remove(&agents_path, COMMUNICATION_BLOCK)?;
    }
    crate::cli::agents_block::remove(&agents_path, LEGACY_COMMUNICATION_BLOCK)?;

    if had_legacy {
        println!(
            "  communication policy: replaced the retired absolute-mode AGENTS.md block; review the preserved surrounding content"
        );
    }
    Ok(())
}

/// Remove addon-conditional generated skills whose addon is no longer enabled.
/// Only skills that givn itself manages are touched.
fn reconcile_addon_artifacts(
    project_root: &Path,
    manifest: &crate::config::manifest::Manifest,
    targets: &[AgentTarget],
) -> Result<()> {
    let disabled = [
        ("givn-domain-modeling", !manifest.addons.domain_modeling),
        (
            "givn-dev-principles",
            !manifest_addon_enabled(manifest, "dev_principles"),
        ),
        (
            "givn-challenge",
            !manifest_addon_enabled(manifest, "challenge"),
        ),
    ];
    for (skill, remove) in disabled {
        if !remove {
            continue;
        }
        for target in targets {
            let dir = project_root.join(target.dir()).join("skills").join(skill);
            if dir.exists() {
                std::fs::remove_dir_all(&dir)?;
            }
        }
    }
    Ok(())
}

/// Names of the command files Givn generated before the skills-only surface.
fn legacy_command_names() -> Vec<&'static str> {
    let mut names = default_skill_names();
    names.push("grill-me");
    names.push("grill-with-docs");
    names
}

/// Whether file bytes are a recognizable Givn-generated command from an
/// earlier release. A matching filename is never sufficient.
fn is_generated_legacy_command(content: &str) -> bool {
    let body = content.trim_start();
    if !body.starts_with("---") {
        return false;
    }
    let Some(frontmatter_end) = body[3..].find("\n---") else {
        return false;
    };
    let frontmatter = &body[3..3 + frontmatter_end];
    if !frontmatter.contains("description:") {
        return false;
    }
    let body = &body[3 + frontmatter_end + 4..];
    body.contains(
        "**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`",
    ) || body.contains("this file only triggers the step")
        || body.contains("This file triggers the step and carries no rules of its own.")
        || body.contains("When work is handed to another context, pass this block")
        || body.contains("<!-- givn:generated")
        || body.contains("Generated by givn")
}

/// Remove recognizable legacy command files from every selected target. A
/// custom file with a known command name is preserved and reported for manual
/// transition; unrelated custom commands are left untouched.
fn reconcile_legacy_commands(project_root: &Path, targets: &[AgentTarget]) -> Result<()> {
    let known = legacy_command_names();
    let mut removed = 0usize;
    let mut custom = Vec::new();
    for target in targets {
        let commands = project_root.join(target.dir()).join("commands");
        if !commands.is_dir() {
            continue;
        }
        for entry in std::fs::read_dir(&commands)? {
            let path = entry?.path();
            if path.extension().is_none_or(|extension| extension != "md") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let content = std::fs::read_to_string(&path)?;
            if is_generated_legacy_command(&content) {
                std::fs::remove_file(&path)?;
                removed += 1;
            } else if known.contains(&stem) {
                custom.push(path);
            }
        }
        if std::fs::read_dir(&commands)?.next().is_none() {
            std::fs::remove_dir(&commands)?;
        }
    }
    if removed > 0 {
        println!("  removed {removed} recognizable legacy Givn command file(s)");
    }
    for path in custom {
        println!(
            "  manual transition: preserved custom command {}; Givn no longer generates commands",
            path.strip_prefix(project_root).unwrap_or(&path).display()
        );
    }
    Ok(())
}

/// Return the project paths that `sync` may write for the current effective
/// configuration. The upgrade migration uses this list as its Git staging
/// allowlist; it must never stage an entire generated directory.
pub fn managed_generated_paths(project_root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let manifest = load_effective()?;
    let targets = configured_targets(project_root)?;
    let mut paths = vec![
        project_root.join("AGENTS.md"),
        project_root.join(".githooks/commit-msg"),
    ];

    let mut skill_names = default_skill_names();
    if domain_modeling_present(&manifest) {
        skill_names.push("givn-domain-modeling");
    }
    if manifest_addon_enabled(&manifest, "dev_principles") {
        skill_names.push("givn-dev-principles");
    }
    if manifest_addon_enabled(&manifest, "challenge") {
        skill_names.push("givn-challenge");
    }

    for target in &targets {
        for name in &skill_names {
            paths.push(
                project_root
                    .join(target.dir())
                    .join("skills")
                    .join(name)
                    .join("SKILL.md"),
            );
        }
    }

    // The integration files come from the same per-target table the installer
    // uses, and a target's registration file from its own registration row, so
    // this allowlist can never drift from what `sync` writes — and the target
    // list decides every row.
    for target in &targets {
        for integration in integrations_of(*target) {
            paths.push(project_root.join(integration.relative));
        }
        if let Some(file) = managed_registration_file(*target) {
            paths.push(project_root.join(file));
        }
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// One generated agent integration: the asset shipped under
/// `assets/agent-integrations/` and the project path it is installed at.
struct AgentIntegration {
    /// The project-relative install path.
    relative: &'static str,
    /// The asset path under `assets/`.
    asset: &'static str,
    /// Whether the installed file is executable.
    executable: bool,
}

/// The integrations the opencode target receives.
const OPENCODE_INTEGRATIONS: &[AgentIntegration] = &[
    AgentIntegration {
        relative: ".opencode/plugins/givn-attribution.js",
        asset: "agent-integrations/opencode/givn-attribution.js",
        executable: false,
    },
    AgentIntegration {
        relative: ".opencode/plugins/givn-timeline.js",
        asset: "agent-integrations/opencode/givn-timeline.js",
        executable: false,
    },
];

/// The integrations the omp target receives.
const OMP_INTEGRATIONS: &[AgentIntegration] = &[
    AgentIntegration {
        relative: ".omp/hooks/pre/givn-attribution.js",
        asset: "agent-integrations/omp/givn-attribution.js",
        executable: false,
    },
    AgentIntegration {
        relative: ".omp/hooks/pre/givn-timeline.js",
        asset: "agent-integrations/omp/givn-timeline.js",
        executable: false,
    },
];

/// The integrations the claude target receives.
const CLAUDE_INTEGRATIONS: &[AgentIntegration] = &[
    AgentIntegration {
        relative: ".claude/hooks/givn-attribution.sh",
        asset: "agent-integrations/claude/givn-attribution.sh",
        executable: true,
    },
    AgentIntegration {
        relative: ".claude/hooks/givn-timeline.sh",
        asset: "agent-integrations/claude/givn-timeline.sh",
        executable: true,
    },
];

/// The integrations the maki target receives: the reporter module, copied
/// verbatim. Its two merged files (the `plugin.toml` grants and the fence in
/// `init.lua`) are user-owned and never generated paths, so they are not rows
/// here.
const MAKI_INTEGRATIONS: &[AgentIntegration] = &[AgentIntegration {
    relative: ".maki/lua/givn-timeline.lua",
    asset: "agent-integrations/maki/givn-timeline.lua",
    executable: false,
}];

/// The integrations one target receives: the table is the target's install
/// set, so no branch here decides what a target gets.
fn integrations_of(target: AgentTarget) -> &'static [AgentIntegration] {
    match target {
        AgentTarget::Opencode => OPENCODE_INTEGRATIONS,
        AgentTarget::Omp => OMP_INTEGRATIONS,
        AgentTarget::Claude => CLAUDE_INTEGRATIONS,
        AgentTarget::Maki => MAKI_INTEGRATIONS,
        AgentTarget::Agents => &[],
    }
}

/// The merged registration file a target owns that is also a managed generated
/// path: the Claude Code settings file, which `sync` merges and `givn upgrade`
/// stages. A target's other merges are user-owned files and are never listed
/// here.
fn managed_registration_file(target: AgentTarget) -> Option<&'static str> {
    match target {
        AgentTarget::Claude => Some(".claude/settings.local.json"),
        _ => None,
    }
}

/// The merges a target runs over its own files: the Claude Code settings merge,
/// and Maki's two merged files (the `plugin.toml` grants and the fence in
/// `init.lua`), which are user-owned and therefore never generated paths.
fn target_merges(target: AgentTarget) -> &'static [fn(&Path) -> Result<()>] {
    match target {
        AgentTarget::Claude => &[merge_claude_settings],
        AgentTarget::Maki => &[merge_maki_plugin, merge_maki_init],
        _ => &[],
    }
}

fn sync_agent_integrations(project_root: &Path, targets: &[AgentTarget]) -> Result<()> {
    let mut installed_reporter = false;
    for target in targets {
        sync_target_integrations(project_root, *target)?;
        installed_reporter |= !integrations_of(*target).is_empty();
    }
    if installed_reporter {
        crate::history::warn_tracked_recordings(project_root);
    }
    Ok(())
}

fn sync_target_integrations(project_root: &Path, target: AgentTarget) -> Result<()> {
    for integration in integrations_of(target) {
        write_embedded_file(
            project_root,
            integration.relative,
            integration.asset,
            integration.executable,
        )?;
        println!("  integration: {}", integration.relative);
    }
    if let Some(file) = managed_registration_file(target) {
        println!("  integration: {file}");
    }
    for merge in target_merges(target) {
        merge(project_root)?;
    }
    Ok(())
}

fn write_embedded_file(
    project_root: &Path,
    relative: &str,
    asset_path: &str,
    executable: bool,
) -> Result<()> {
    let path = project_root.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = crate::config::embedded::Assets::get_str(asset_path)
        .ok_or_else(|| anyhow::anyhow!("embedded agent integration missing: {asset_path}"))?;
    std::fs::write(&path, content)?;
    if executable {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&path)?.permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&path, permissions)?;
        }
    }
    Ok(())
}

/// The command the Claude development reporter's per-event registration runs:
/// the installed shim, which pipes its payload to the binary-side mapper.
const CLAUDE_TIMELINE_COMMAND: &str = "${CLAUDE_PROJECT_DIR}/.claude/hooks/givn-timeline.sh";

/// One event the Claude development reporter registers for.
struct ClaudeTimelineHook {
    /// The Claude Code event name.
    event: &'static str,
    /// The matcher group's pattern: `*` matches every occurrence of the event.
    matcher: &'static str,
    /// Whether the hook runs in the background: the tool-level hooks never
    /// slow the agent.
    asynchronous: bool,
    /// The synchronous flush points' own timeout in seconds, so a hung flush
    /// cannot stall the turn (`SessionEnd` keeps the harness's 1.5 s budget).
    timeout: Option<f64>,
}

/// The events the generated shim maps: the tool-level events stamp and close
/// the tool spans, `PermissionRequest`/`Notification` stamp the waits the
/// harness does not time, `SubagentStart`/`SubagentStop` span a subagent,
/// `Stop` is the turn's usage flush point, and `SessionEnd` prunes the
/// session's pending state.
const CLAUDE_TIMELINE_HOOKS: &[ClaudeTimelineHook] = &[
    ClaudeTimelineHook {
        event: "PreToolUse",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "PostToolUse",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "PostToolUseFailure",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "PermissionRequest",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "Notification",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "SubagentStart",
        matcher: "*",
        asynchronous: true,
        timeout: None,
    },
    ClaudeTimelineHook {
        event: "SubagentStop",
        matcher: "*",
        asynchronous: false,
        timeout: Some(5.0),
    },
    ClaudeTimelineHook {
        event: "Stop",
        matcher: "*",
        asynchronous: false,
        timeout: Some(5.0),
    },
    ClaudeTimelineHook {
        event: "SessionEnd",
        matcher: "*",
        asynchronous: false,
        timeout: Some(1.5),
    },
];

/// Whether one settings entry carries a command hook running `command`.
fn points_at(entry: &serde_json::Value, command: &str) -> bool {
    entry
        .get("hooks")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|hooks| {
            hooks.iter().any(|hook| {
                hook.get("command").and_then(serde_json::Value::as_str) == Some(command)
            })
        })
}

/// Merge one development-reporter hook entry idempotently per
/// `(event, matcher, command)`: an entry of this reporter whose matcher no
/// longer matches is dropped first (a changed matcher is re-applied), the
/// current entry's handler options are re-applied, and a user's own entries —
/// and the attribution entry — are never touched.
fn merge_claude_timeline_hook(
    hooks: &mut serde_json::Map<String, serde_json::Value>,
    hook: &ClaudeTimelineHook,
) -> Result<()> {
    let entries = hooks
        .entry(hook.event)
        .or_insert_with(|| serde_json::json!([]))
        .as_array_mut()
        .ok_or_else(|| anyhow::anyhow!("Claude {} hooks must be an array", hook.event))?;
    entries.retain(|entry| {
        !points_at(entry, CLAUDE_TIMELINE_COMMAND)
            || entry.get("matcher").and_then(serde_json::Value::as_str) == Some(hook.matcher)
    });
    let mut handler = serde_json::Map::new();
    handler.insert("type".to_string(), serde_json::json!("command"));
    handler.insert(
        "command".to_string(),
        serde_json::json!(CLAUDE_TIMELINE_COMMAND),
    );
    if hook.asynchronous {
        handler.insert("async".to_string(), serde_json::json!(true));
    }
    if let Some(timeout) = hook.timeout {
        handler.insert("timeout".to_string(), serde_json::json!(timeout));
    }
    let current = entries.iter_mut().find(|entry| {
        entry.get("matcher").and_then(serde_json::Value::as_str) == Some(hook.matcher)
            && points_at(entry, CLAUDE_TIMELINE_COMMAND)
    });
    match current {
        Some(entry) => {
            entry.as_object_mut().ok_or_else(|| {
                anyhow::anyhow!("Claude {} hook entries must be JSON objects", hook.event)
            })?["hooks"] = serde_json::json!([serde_json::Value::Object(handler)]);
        }
        None => entries.push(serde_json::json!({
            "matcher": hook.matcher,
            "hooks": [serde_json::Value::Object(handler)],
        })),
    }
    Ok(())
}

/// The fence Givn owns inside `.maki/init.lua`: the exact fail-soft body the
/// design reproduced (the bare `pcall(require, …).setup()` form indexes the
/// boolean status, raises, and kills the rest of the user's `init.lua`).
const MAKI_FENCE_BEGIN: &str = "-- givn:begin:timeline";
const MAKI_FENCE_END: &str = "-- givn:end:timeline";
const MAKI_FENCE_LINES: [&str; 6] = [
    MAKI_FENCE_BEGIN,
    "local ok, reporter = pcall(require, \"givn-timeline\")",
    "if ok and type(reporter) == \"table\" and reporter.setup then",
    "  reporter.setup()",
    "end",
    MAKI_FENCE_END,
];

/// The permission keys the Maki reporter calls: `maki.fs.read`,
/// `maki.fs.write`/`rm`, and `maki.fn.jobstart`/`jobwait`. maki adds nothing
/// else to a file the user already owns.
const MAKI_REPORTER_GRANTS: [&str; 3] = ["fs_read", "fs_write", "run"];

/// The key a TOML line assigns, when the line is a key/value pair.
fn toml_key(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('[') || line.starts_with('#') {
        return None;
    }
    let (key, _) = line.split_once('=')?;
    Some(key.trim())
}

/// Merge the Maki reporter's permission grants into `.maki/plugin.toml`,
/// idempotently per key and never clobbering the file: a key the user already
/// wrote is never removed or flipped, and surrounding content is untouched.
/// When the file is absent it is created from the asset, which names all six
/// keys, so the created file states the whole grant.
fn merge_maki_plugin(project_root: &Path) -> Result<()> {
    let path = project_root.join(".maki/plugin.toml");
    if !path.exists() {
        return write_embedded_file(
            project_root,
            ".maki/plugin.toml",
            "agent-integrations/maki/plugin.toml",
            false,
        );
    }
    let content = std::fs::read_to_string(&path)?;
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    match lines.iter().position(|line| line.trim() == "[permissions]") {
        Some(start) => {
            // The table ends at the next header (or the end of the file); an
            // existing key — whatever value the user gave it — is left alone.
            let end = lines[start + 1..]
                .iter()
                .position(|line| line.trim().starts_with('['))
                .map(|offset| start + 1 + offset)
                .unwrap_or(lines.len());
            let missing: Vec<&str> = MAKI_REPORTER_GRANTS
                .into_iter()
                .filter(|key| {
                    !lines[start + 1..end]
                        .iter()
                        .any(|line| toml_key(line) == Some(key))
                })
                .collect();
            if missing.is_empty() {
                return Ok(());
            }
            for (offset, key) in missing.into_iter().enumerate() {
                lines.insert(end + offset, format!("{key} = true"));
            }
        }
        None => {
            if lines.iter().any(|line| !line.trim().is_empty()) {
                lines.push(String::new());
            }
            lines.push("[permissions]".to_string());
            for key in MAKI_REPORTER_GRANTS {
                lines.push(format!("{key} = true"));
            }
        }
    }
    std::fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Merge the managed fence into `.maki/init.lua`: created when the file is
/// absent, re-applied when the fence is already there (so a stale body is
/// replaced), appended otherwise — and the rest of the user's file is never
/// touched.
fn merge_maki_init(project_root: &Path) -> Result<()> {
    let path = project_root.join(".maki/init.lua");
    let fence = MAKI_FENCE_LINES.join("\n");
    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, format!("{fence}\n"))?;
        return Ok(());
    }
    let content = std::fs::read_to_string(&path)?;
    let Some(begin) = content.find(MAKI_FENCE_BEGIN) else {
        let mut merged = content;
        if !merged.is_empty() && !merged.ends_with('\n') {
            merged.push('\n');
        }
        if !merged.is_empty() {
            merged.push('\n');
        }
        merged.push_str(&fence);
        merged.push('\n');
        std::fs::write(&path, merged)?;
        return Ok(());
    };
    let end = content[begin..]
        .find(MAKI_FENCE_END)
        .map(|offset| begin + offset + MAKI_FENCE_END.len())
        .ok_or_else(|| anyhow::anyhow!("the .maki/init.lua fence has no end marker"))?;
    let merged = format!("{}{}", &content[..begin], fence);
    let merged = format!("{}{}", merged, &content[end..]);
    if merged != content {
        std::fs::write(&path, merged)?;
    }
    Ok(())
}

fn merge_claude_settings(project_root: &Path) -> Result<()> {
    let path = project_root.join(".claude/settings.local.json");
    let mut settings = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str::<serde_json::Value>(&content)
            .with_context(|| format!("parse {}", path.display()))?
    } else {
        serde_json::json!({})
    };

    let root = settings
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("Claude settings must contain a JSON object"))?;
    let hooks = root.entry("hooks").or_insert_with(|| serde_json::json!({}));
    let hooks = hooks
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("Claude settings hooks must contain a JSON object"))?;
    let pre_tool_use = hooks
        .entry("PreToolUse")
        .or_insert_with(|| serde_json::json!([]));
    let entries = pre_tool_use
        .as_array_mut()
        .ok_or_else(|| anyhow::anyhow!("Claude PreToolUse hooks must be an array"))?;

    let command = "${CLAUDE_PROJECT_DIR}/.claude/hooks/givn-attribution.sh";
    let already_present = entries.iter().any(|entry| points_at(entry, command));
    if !already_present {
        entries.push(serde_json::json!({
            "matcher": "Bash",
            "hooks": [{"type": "command", "command": command}]
        }));
    }

    // The development reporter's per-event registration, merged beside the
    // attribution entry and beside any user entry.
    for hook in CLAUDE_TIMELINE_HOOKS {
        merge_claude_timeline_hook(hooks, hook)?;
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(&settings)? + "\n")?;
    Ok(())
}

/// Write a rendered SKILL.md to the configured skill target locations.
fn write_skill_to_targets(
    project_root: &Path,
    name: &str,
    content: &str,
    targets: &[AgentTarget],
) -> Result<()> {
    for target in targets {
        let path = project_root
            .join(target.dir())
            .join("skills")
            .join(name)
            .join("SKILL.md");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content)?;
    }

    Ok(())
}

/// Resolve a skill template by name.
fn resolve_skill_template_for(name: &str) -> Result<String> {
    resolve_skill_template(name)
}

/// Validate SKILL.md frontmatter (name matches dir, description present and bounded).
fn validate_frontmatter(content: &str, expected_name: &str) -> Result<()> {
    if !content.starts_with("---") {
        anyhow::bail!(
            "skill template for '{expected_name}' must start with YAML frontmatter (---)"
        );
    }

    let fm_end = content[3..].find("\n---").unwrap_or(0) + 3;
    let frontmatter = &content[3..fm_end];

    if !frontmatter.contains(&format!("name: {expected_name}")) {
        anyhow::bail!(
            "skill template frontmatter name must match directory name '{expected_name}'"
        );
    }

    if !frontmatter.contains("description:") {
        anyhow::bail!(
            "skill template for '{expected_name}' must have a 'description' in frontmatter"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::embedded::Assets;

    #[test]
    fn validate_frontmatter_requires_name_match() {
        let content = "---\nname: givn-propose\ndescription: Test skill\n---\n\nContent";
        assert!(validate_frontmatter(content, "givn-propose").is_ok());
        assert!(validate_frontmatter(content, "givn-spec").is_err());
    }

    #[test]
    fn validate_frontmatter_requires_description() {
        let content = "---\nname: givn-propose\n---\n\nContent";
        assert!(validate_frontmatter(content, "givn-propose").is_err());
    }

    #[test]
    fn domain_skill_follows_its_own_addon_without_arc42() {
        let mut manifest = crate::config::manifest::Manifest::load_embedded().unwrap();
        manifest.addons.domain_modeling = true;
        manifest.addons.arc42 = false;
        assert!(domain_modeling_present(&manifest));

        manifest.addons.domain_modeling = false;
        assert!(!domain_modeling_present(&manifest));
    }

    #[test]
    fn all_skill_templates_are_embedded() {
        let mut names = default_skill_names();
        names.extend([
            "givn-domain-modeling",
            "givn-dev-principles",
            "givn-challenge",
        ]);
        for name in names {
            let key = format!("skills/{name}/SKILL.md.tmpl");
            assert!(
                Assets::get(&key).is_some(),
                "embedded skill template missing: {key}"
            );
        }
    }

    #[test]
    fn no_command_templates_are_embedded() {
        assert!(
            Assets::iter().all(|path| !path.starts_with("commands/")),
            "command templates are still shipped"
        );
    }

    #[test]
    fn generated_legacy_command_detection_requires_generated_bytes() {
        let generated = "---\ndescription: demo\n---\n\n**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.\n";
        assert!(is_generated_legacy_command(generated));
        let custom = "---\ndescription: house rule\n---\n\nDo the thing.\n";
        assert!(!is_generated_legacy_command(custom));
        assert!(!is_generated_legacy_command("plain text"));
    }

    #[test]
    fn target_edits_refuse_flow_mapping_and_anchors_before_writing() {
        for content in [
            "skills: {targets: [opencode]}\n",
            "skills: &defaults\n  targets: [opencode]\n",
            "skills:\n  targets: *defaults\n",
            "skills:\n  targets:\n    - opencode\n",
            "skills:\n  targets: [opencode]\nskills:\n  targets: [claude]\n",
        ] {
            assert!(
                validate_skills_yaml(content).is_err(),
                "unsupported YAML was accepted: {content}"
            );
        }
        assert!(validate_skills_yaml("skills:\n  targets: [opencode]\nother: kept\n").is_ok());
    }

    #[test]
    fn communication_policy_replaces_legacy_block_and_preserves_outside_content() {
        let dir = tempfile::tempdir().unwrap();
        let agents = dir.path().join("AGENTS.md");
        std::fs::write(
            &agents,
            "Human intro\n\n<!-- givn:begin:absolute-mode -->\nOLD\n<!-- givn:end:absolute-mode -->\n\nHuman tail\n",
        )
        .unwrap();
        sync_communication_policy(dir.path(), true).unwrap();
        let content = std::fs::read_to_string(&agents).unwrap();
        assert!(content.contains("Human intro") && content.contains("Human tail"));
        assert!(content.contains("<!-- givn:begin:communication-policy -->"));
        assert!(!content.contains("OLD"));
        assert!(!content.contains("absolute-mode"));
    }

    #[test]
    fn legacy_command_reconciliation_removes_generated_and_preserves_custom() {
        let dir = tempfile::tempdir().unwrap();
        let commands = dir.path().join(".opencode/commands");
        std::fs::create_dir_all(&commands).unwrap();
        let generated = "---\ndescription: legacy\n---\n\n**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.\n";
        std::fs::write(commands.join("givn-propose.md"), generated).unwrap();
        std::fs::write(
            commands.join("givn-review.md"),
            "---\ndescription: custom\n---\n\nTeam-specific review command.\n",
        )
        .unwrap();
        std::fs::write(
            commands.join("team-release.md"),
            "---\ndescription: custom\n---\n\nTeam release command.\n",
        )
        .unwrap();
        reconcile_legacy_commands(dir.path(), &[AgentTarget::Opencode]).unwrap();
        assert!(!commands.join("givn-propose.md").exists());
        assert!(commands.join("givn-review.md").exists());
        assert!(commands.join("team-release.md").exists());
    }

    /// The development reporter set is keyed to the targets that execute the
    /// agent's work: `opencode`, `claude`, and `omp` each receive one reporter,
    /// the neutral `agents` skills mirror receives none, and every reporter
    /// row's asset is really embedded — an installed reporter is never a
    /// dangling path.
    #[test]
    fn only_executing_targets_receive_a_development_reporter() {
        let mut reporters: Vec<(&str, &str)> = Vec::new();
        for target in AgentTarget::ALL {
            for integration in integrations_of(target) {
                if integration.asset.contains("givn-timeline") {
                    reporters.push((target.name(), integration.asset));
                }
            }
        }
        let mut names: Vec<&str> = reporters.iter().map(|(name, _)| *name).collect();
        names.sort_unstable();
        assert_eq!(
            names,
            ["claude", "maki", "omp", "opencode"],
            "the reporter set is not keyed to the executing targets: {reporters:#?}"
        );
        for (name, asset) in &reporters {
            assert!(
                Assets::get(asset).is_some(),
                "the {name} reporter's asset is not embedded: {asset}"
            );
        }
    }
}
