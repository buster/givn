use crate::config::manifest::{
    Addons, Apply, Artifact, Coverage, HookEntry, Manifest, Run, Verify,
};
/// Apply the project overlay (from `givn/config.yaml`) onto the embedded default manifest.
///
/// Overlay semantics (ADR-010, ADR-024):
/// - Artifacts keyed by `id`. New id = append. Existing id = deep-merge field-by-field.
/// - `remove: true` on an artifact removes it from the effective graph.
/// - `optional: true` marks it skippable.
/// - Scalar fields (e.g. `verify.command`) in the overlay replace the default.
/// - `addons:` block: every flag is copied onto the effective manifest,
///   regardless of whether that addon has further side effects. `arc42: true`
///   additionally expands to the arc42-docs artifact definition.
/// - After merge, the effective graph is validated.
use anyhow::{Result, bail};

/// Parse the project overlay YAML and merge it onto `base`.
/// Artifact ids that an overlay may not remove. `givn/config.yaml` is a
/// tracked, agent-writable file — every gate in the workflow is a few YAML
/// lines away from being deleted by whoever edits it, deliberately or not.
/// `review` is the sign-off gate the whole workflow funnels into.
const PROTECTED_ARTIFACTS: &[&str] = &["review"];

/// Top-level keys owned by `givn/commands.yaml`.
const COMMAND_KEYS: &[&str] = &["verify", "run", "coverage"];

/// Top-level keys owned by `givn/config.yaml`.
const POLICY_KEYS: &[&str] = &[
    "artifacts",
    "apply",
    "addons",
    "skills",
    "rules",
    "extensions",
    "record",
    "givn_config_version",
];

pub fn apply(base: Manifest, overlay_yaml: &str) -> Result<Manifest> {
    validate_addon_keys(overlay_yaml)?;
    apply_parsed(base, overlay_yaml)
}

/// Apply `givn/config.yaml`, which owns policy, artifact, addon, target, and
/// extension settings. Executable command and runtime settings belong in
/// `givn/commands.yaml` and are refused here instead of being silently
/// applied from the wrong file.
pub fn apply_config(base: Manifest, overlay_yaml: &str) -> Result<Manifest> {
    validate_config_ownership(overlay_yaml)?;
    apply(base, overlay_yaml)
}

/// Apply `givn/commands.yaml` with its dedicated schema: only `verify`, `run`,
/// and `coverage` are accepted. Policy, addon, artifact, and apply keys are
/// refused before any value is applied.
pub fn apply_commands(base: Manifest, commands_yaml: &str) -> Result<Manifest> {
    validate_commands_ownership(commands_yaml)?;
    let overlay: CommandsDoc = serde_yaml_ng::from_str(commands_yaml)
        .map_err(|e| anyhow::anyhow!("failed to parse givn/commands.yaml: {e}"))?;
    let mut result = base;
    if let Some(v) = overlay.verify {
        result.verify = v;
    }
    if let Some(r) = overlay.run {
        result.run = r;
    }
    if let Some(c) = overlay.coverage {
        result.coverage = c;
    }
    Ok(result)
}

fn parse_overlay_value(overlay_yaml: &str) -> Option<serde_yaml_ng::Value> {
    serde_yaml_ng::from_str(overlay_yaml).ok()
}

fn validate_addon_keys(overlay_yaml: &str) -> Result<()> {
    let Some(document) = parse_overlay_value(overlay_yaml) else {
        return Ok(());
    };
    let Some(root) = document.as_mapping() else {
        return Ok(());
    };
    let Some(addons) = root.get("addons") else {
        return Ok(());
    };
    let Some(addons) = addons.as_mapping() else {
        return Ok(());
    };
    for key in addons.keys() {
        let Some(key) = key.as_str() else {
            bail!("addons settings must use string keys");
        };
        if Addons::NAMES.contains(&key) {
            continue;
        }
        if let Some(replacement) = Addons::replacement_for_retired(key) {
            bail!(
                "retired addon setting 'addons.{key}' is not supported. Rename it manually to \
                 'addons.{replacement}' — givn 0.9.10 does not convert retired configuration or \
                 accept a compatibility alias. The supported migration baseline is 1.0; review \
                 the communication_policy, challenge, and skills-only transitions by hand. \
                 No files were changed."
            );
        }
        bail!(
            "unknown addon '{key}' in the addons block. Known addons: {}. Retired addons \
             requiring manual transition: {}. No files were changed.",
            Addons::NAMES.join(", "),
            Addons::RETIRED
                .iter()
                .map(|(retired, replacement)| format!("{retired} -> {replacement}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

fn validate_config_ownership(overlay_yaml: &str) -> Result<()> {
    let Some(document) = parse_overlay_value(overlay_yaml) else {
        return Ok(());
    };
    let Some(root) = document.as_mapping() else {
        return Ok(());
    };
    for key in COMMAND_KEYS {
        if root.contains_key(*key) {
            bail!(
                "'{key}' is executable configuration and belongs in givn/commands.yaml, not \
                 givn/config.yaml. Move the '{key}' block manually; the supported migration \
                 baseline is 1.0. No files were changed."
            );
        }
    }
    Ok(())
}

fn validate_commands_ownership(commands_yaml: &str) -> Result<()> {
    let Some(document) = parse_overlay_value(commands_yaml) else {
        return Ok(());
    };
    let Some(root) = document.as_mapping() else {
        return Ok(());
    };
    for key in root.keys() {
        let Some(key) = key.as_str() else {
            bail!("givn/commands.yaml must use string top-level keys");
        };
        if COMMAND_KEYS.contains(&key) {
            continue;
        }
        if POLICY_KEYS.contains(&key) {
            bail!(
                "'{key}' is policy, artifact, or addon configuration and belongs in \
                 givn/config.yaml, not givn/commands.yaml. Move the '{key}' block manually. \
                 No files were changed."
            );
        }
        bail!(
            "unknown key '{key}' in givn/commands.yaml. This file only accepts verify, run, \
             and coverage; policy, addon, artifact, and apply settings belong in givn/config.yaml. \
             No files were changed."
        );
    }
    Ok(())
}

fn apply_parsed(base: Manifest, overlay_yaml: &str) -> Result<Manifest> {
    // Parse the overlay as a Manifest (using lenient defaults for missing fields).
    let overlay: OverlayDoc = serde_yaml_ng::from_str(overlay_yaml)
        .map_err(|e| anyhow::anyhow!("failed to parse givn/config.yaml: {e}"))?;

    let mut result = base;

    // Merge verify block.
    if let Some(v) = overlay.verify {
        result.verify = v;
    }

    // Merge run block (replace-whole, same semantics as verify).
    if let Some(r) = overlay.run {
        result.run = r;
    }

    // Merge coverage block.
    if let Some(c) = overlay.coverage {
        result.coverage = c;
    }

    // Merge apply block.
    if let Some(a) = overlay.apply {
        result.apply = a;
    }

    if let Some(addons) = overlay.addons {
        // arc42 additionally expands into an artifact-graph node — keep
        // that side effect here, guarded so enabling twice stays idempotent.
        // The arc42 document is produced during the design phase and
        // checked in design-review: design-review requires arc42-docs, and
        // arc42-docs itself gates archive (added to apply.requires) so an
        // enabled-but-unfinished arc42 doc genuinely blocks `givn archive`,
        // not just the "ready-to-archive" status label.
        if addons.arc42 {
            if !result.artifacts.iter().any(|a| a.id == "arc42-docs") {
                result.artifacts.push(arc42_artifact());
            }
            if !result.apply.requires.contains(&"arc42-docs".to_string()) {
                result.apply.requires.push("arc42-docs".to_string());
            }
            if let Some(design_review) = result
                .artifacts
                .iter_mut()
                .find(|a| a.id == "design-review")
                && !design_review.requires.contains(&"arc42-docs".to_string())
            {
                design_review.requires.push("arc42-docs".to_string());
            }
        } else if result.addons.arc42 {
            // The default now enables Arc42, so an explicit opt-out must
            // remove the artifact and its dependencies instead of leaving a
            // gate the project did not ask for.
            result.artifacts.retain(|a| a.id != "arc42-docs");
            result.apply.requires.retain(|id| id != "arc42-docs");
            if let Some(design_review) = result
                .artifacts
                .iter_mut()
                .find(|a| a.id == "design-review")
            {
                design_review.requires.retain(|id| id != "arc42-docs");
            }
        }
        result.addons = addons;
    }

    // Merge artifact patches.
    for patch in overlay.artifacts {
        if patch.remove {
            if PROTECTED_ARTIFACTS.contains(&patch.id.as_str()) {
                bail!(
                    "cannot remove artifact '{}' via overlay — it is a protected \
                     gate. Use `optional: true` if you need to make it skippable, \
                     or edit apply.requires directly if you understand the \
                     consequences.",
                    patch.id
                );
            }
            result.artifacts.retain(|a| a.id != patch.id);
            continue;
        }
        if let Some(existing) = result.artifacts.iter_mut().find(|a| a.id == patch.id) {
            // Deep merge: only apply fields that are explicitly set in the patch.
            merge_artifact(existing, patch);
        } else {
            // New artifact: append (convert from patch type to full Artifact).
            result.artifacts.push(overlay_to_artifact(patch));
        }
    }

    // Validate the effective graph.
    validate_graph(&result)?;

    Ok(result)
}

/// Load the effective manifest: embedded default + project overlay (if present).
pub fn load_effective() -> Result<Manifest> {
    load_effective_at(&std::env::current_dir()?)
}

/// `load_effective` against an explicit project root, for a caller that
/// resolves the root itself instead of relying on the working directory.
pub fn load_effective_at(project_root: &std::path::Path) -> Result<Manifest> {
    let base = Manifest::load_embedded()?;
    let config_path = project_root.join("givn/config.yaml");

    let manifest = if config_path.exists() {
        let yaml = std::fs::read_to_string(&config_path)?;
        apply_config(base, &yaml)?
    } else {
        base
    };

    let commands_path = project_root.join("givn/commands.yaml");
    if commands_path.exists() {
        let yaml = std::fs::read_to_string(&commands_path)?;
        apply_commands(manifest, &yaml)
    } else {
        Ok(manifest)
    }
}

/// The known agent target names (canonical, no leading dot).
pub const KNOWN_AGENT_TARGETS: [&str; 5] = ["opencode", "agents", "claude", "omp", "maki"];

/// The resolved generated-rules policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RulesPolicy {
    /// Givn rules stay on the developer's machine, out of the repository.
    LocalOnly,
    /// Givn rules are repository content, staged and committed like source.
    Versioned,
}

impl RulesPolicy {
    /// The scalar label used in config and reports.
    pub fn label(self) -> &'static str {
        match self {
            RulesPolicy::LocalOnly => "local-only",
            RulesPolicy::Versioned => "versioned",
        }
    }

    pub fn is_local_only(self) -> bool {
        matches!(self, RulesPolicy::LocalOnly)
    }
}

/// Load the generated-rules policy from the `rules:` scalar of
/// `givn/config.yaml`.
///
/// An absent or empty value resolves to `local-only` (the default for new
/// and existing projects). An unrecognized value warns on stderr, names the
/// valid values, and continues as `local-only`.
pub fn load_rules_policy(project_root: &std::path::Path) -> Result<RulesPolicy> {
    let config_path = project_root.join("givn/config.yaml");
    if !config_path.exists() {
        return Ok(RulesPolicy::LocalOnly);
    }
    let yaml = std::fs::read_to_string(&config_path)?;
    let doc: OverlayDoc = serde_yaml_ng::from_str(&yaml)
        .map_err(|e| anyhow::anyhow!("failed to parse givn/config.yaml: {e}"))?;
    let value = doc.rules.as_deref().map(str::trim).unwrap_or("");
    match value {
        "" | "local-only" => Ok(RulesPolicy::LocalOnly),
        "versioned" => Ok(RulesPolicy::Versioned),
        other => {
            eprintln!(
                "warning: unrecognized rules policy '{other}'; valid values: local-only, versioned — continuing with local-only"
            );
            Ok(RulesPolicy::LocalOnly)
        }
    }
}

/// Load the agent targets from the `skills:` block of `givn/config.yaml`.
///
/// Canonical names (no leading dot), deduplicated, unknown names dropped.
/// An absent block, an empty list, or a list with no known names falls back
/// to `["opencode"]`.
pub fn load_skills_targets(project_root: &std::path::Path) -> Result<Vec<String>> {
    let config_path = project_root.join("givn/config.yaml");
    let mut targets: Vec<String> = Vec::new();
    if config_path.exists() {
        let yaml = std::fs::read_to_string(&config_path)?;
        let doc: OverlayDoc = serde_yaml_ng::from_str(&yaml)
            .map_err(|e| anyhow::anyhow!("failed to parse givn/config.yaml: {e}"))?;
        if let Some(skills) = doc.skills {
            for raw in skills.targets {
                let name = raw.trim().trim_start_matches('.').to_string();
                let name = name.to_ascii_lowercase();
                if KNOWN_AGENT_TARGETS.contains(&name.as_str()) && !targets.contains(&name) {
                    targets.push(name);
                }
            }
        }
    }
    if targets.is_empty() {
        targets.push("opencode".to_string());
    }
    Ok(targets)
}

/// Convert an overlay patch to a full Artifact (for new artifacts added via overlay).
fn overlay_to_artifact(patch: OverlayArtifact) -> Artifact {
    Artifact {
        id: patch.id,
        generates: patch.generates,
        requires: patch.requires,
        template: patch.template,
        instruction: patch.instruction,
        optional: patch.optional,
        remove: false,
        post: patch.post,
    }
}

/// Merge non-default patch fields onto the existing artifact in place.
fn merge_artifact(existing: &mut Artifact, patch: OverlayArtifact) {
    if !patch.generates.is_empty() {
        existing.generates = patch.generates;
    }
    if !patch.requires.is_empty() {
        existing.requires = patch.requires;
    }
    if patch.template.is_some() {
        existing.template = patch.template;
    }
    if patch.instruction.is_some() {
        existing.instruction = patch.instruction;
    }
    if patch.optional {
        existing.optional = true;
    }
    if !patch.post.is_empty() {
        existing.post = patch.post;
    }
}

/// Validate the effective artifact graph:
/// - No cycles in `requires`.
/// - No dangling `requires` references.
/// - `apply.requires` may not reference a removed non-optional artifact.
pub fn validate_graph(manifest: &Manifest) -> Result<()> {
    let ids: std::collections::HashSet<&str> =
        manifest.artifacts.iter().map(|a| a.id.as_str()).collect();

    // Check for dangling requires.
    for artifact in &manifest.artifacts {
        for dep in &artifact.requires {
            if !ids.contains(dep.as_str()) {
                bail!(
                    "artifact '{}' requires '{}' which does not exist in the effective manifest",
                    artifact.id,
                    dep
                );
            }
        }
    }

    // Check apply.requires references exist.
    for req in &manifest.apply.requires {
        if !ids.contains(req.as_str()) {
            bail!(
                "apply.requires references '{}' which does not exist in the effective manifest",
                req
            );
        }
    }

    // Check for cycles using DFS.
    if let Some(cycle) = find_cycle(&manifest.artifacts) {
        bail!("dependency graph contains a cycle: {}", cycle.join(" → "));
    }

    Ok(())
}

/// Returns the cycle path if one exists, else None.
fn find_cycle(artifacts: &[Artifact]) -> Option<Vec<String>> {
    use std::collections::HashMap;

    // Build adjacency list.
    let adj: HashMap<&str, &[String]> = artifacts
        .iter()
        .map(|a| (a.id.as_str(), a.requires.as_slice()))
        .collect();

    let mut visited = std::collections::HashSet::new();
    let mut rec_stack = std::collections::HashSet::new();

    for artifact in artifacts {
        if !visited.contains(artifact.id.as_str())
            && let Some(cycle) = dfs(
                &artifact.id,
                &adj,
                &mut visited,
                &mut rec_stack,
                &mut vec![],
            )
        {
            return Some(cycle);
        }
    }
    None
}

fn dfs<'a>(
    node: &'a str,
    adj: &std::collections::HashMap<&'a str, &'a [String]>,
    visited: &mut std::collections::HashSet<&'a str>,
    rec_stack: &mut std::collections::HashSet<&'a str>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    visited.insert(node);
    rec_stack.insert(node);
    path.push(node.to_string());

    if let Some(deps) = adj.get(node) {
        for dep in *deps {
            if !visited.contains(dep.as_str()) {
                if let Some(cycle) = dfs(dep, adj, visited, rec_stack, path) {
                    return Some(cycle);
                }
            } else if rec_stack.contains(dep.as_str()) {
                // Found a cycle: build the cycle path.
                let mut cycle_path = path.clone();
                cycle_path.push(dep.clone());
                return Some(cycle_path);
            }
        }
    }

    path.pop();
    rec_stack.remove(node);
    None
}

/// The default arc42-docs artifact definition (used when addon is enabled).
///
/// `generates` is the change-level marker file (`arc42.md` in the change dir).
/// The actual durable docs live under `docs/arc42/` (not tracked as generates).
/// Template and instruction are resolved via the cascade (addon fallback layer).
fn arc42_artifact() -> Artifact {
    Artifact {
        id: "arc42-docs".to_string(),
        generates: vec!["arc42.md".to_string()],
        requires: vec!["design".to_string()],
        template: Some("impact-assessment-template.md".to_string()),
        instruction: Some("arc42-docs".to_string()),
        optional: false,
        remove: false,
        post: vec![],
    }
}

/// Dedicated `givn/commands.yaml` schema. Its authority is limited to the
/// executable command and runtime settings; policy and artifact settings are
/// refused before deserialization.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CommandsDoc {
    verify: Option<Verify>,
    run: Option<Run>,
    coverage: Option<Coverage>,
}

/// Lenient overlay document — only fields that are explicitly overridden need to be present.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct OverlayDoc {
    #[serde(default)]
    pub(crate) artifacts: Vec<OverlayArtifact>,
    pub(crate) verify: Option<Verify>,
    pub(crate) run: Option<Run>,
    pub(crate) coverage: Option<Coverage>,
    pub(crate) apply: Option<Apply>,
    pub(crate) addons: Option<Addons>,
    /// Agent targets for skill/command generation (`skills.targets`).
    /// Ignored by the manifest itself; consumed by `load_skills_targets`.
    pub(crate) skills: Option<Skills>,
    /// Generated-rules policy scalar (`rules:`). Consumed by
    /// `load_rules_policy`; ignored by the manifest itself.
    pub(crate) rules: Option<String>,
    /// Project-owned instruction extensions (`extensions:`). Consumed by
    /// `config: extensions::load`; ignored by the manifest itself.
    pub(crate) extensions: Option<ExtensionEntries>,
    /// The development record's project-owned block (`record:`), read as a raw
    /// value so an invalid `mode` — a non-string, an unknown word, or a block
    /// that is not a mapping — is the record resolver's one-warning fallback
    /// rather than a whole-config parse failure. Consumed by
    /// `history::record_setting`; ignored by the manifest itself.
    pub(crate) record: Option<serde_yaml_ng::Value>,
    /// Ignored during overlay processing (used for version tracking).
    #[allow(dead_code)]
    pub(crate) givn_config_version: Option<String>,
}

/// The raw `record:` block of `givn/config.yaml`, when the file exists and
/// parses. `None` for a missing file, an unreadable file, or a config the
/// overlay parser refuses: the record then resolves its default mode.
///
/// The value's meaning belongs to the record resolver, never to the overlay:
/// the parse stays lenient so a broken `record.mode` never stops the work it
/// would record.
pub fn load_record_block(project_root: &std::path::Path) -> Option<serde_yaml_ng::Value> {
    let yaml = std::fs::read_to_string(project_root.join("givn/config.yaml")).ok()?;
    let doc: OverlayDoc = serde_yaml_ng::from_str(&yaml).ok()?;
    doc.record
}

/// Lenient `extensions:` entry list from `givn/config.yaml`.
///
/// The pinned YAML parser rejects duplicate keys inside a
/// `serde_yaml_ng::Value` mapping, so the map is read as an entry list of raw
/// values in document order. A repeated instruction key keeps its last
/// occurrence (recorded behavior). Every other shape parses leniently: the
/// serving path validates the entries and warns at request time, so no
/// command fails because of a bad extension entry.
#[derive(Debug, Default)]
pub(crate) struct ExtensionEntries {
    /// `(instruction id, raw value)` pairs in document order, a repeated
    /// key keeping its last occurrence.
    pub(crate) entries: Vec<(String, serde_yaml_ng::Value)>,
}

impl<'de> serde::Deserialize<'de> for ExtensionEntries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EntriesVisitor;

        impl<'de> serde::de::Visitor<'de> for EntriesVisitor {
            type Value = ExtensionEntries;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of instruction id to rule list")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ExtensionEntries, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut entries: Vec<(String, serde_yaml_ng::Value)> = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<serde_yaml_ng::Value>()?;
                    entries.retain(|(existing, _)| existing != &key);
                    entries.push((key, value));
                }
                Ok(ExtensionEntries { entries })
            }

            // A `extensions:` value that is not a map is ignored here and
            // reported by the serving path; the command keeps working.
            fn visit_seq<A>(self, _seq: A) -> Result<ExtensionEntries, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(ExtensionEntries::default())
            }

            fn visit_bool<E: serde::de::Error>(self, _value: bool) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }

            fn visit_i64<E: serde::de::Error>(self, _value: i64) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }

            fn visit_u64<E: serde::de::Error>(self, _value: u64) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }

            fn visit_f64<E: serde::de::Error>(self, _value: f64) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }

            fn visit_str<E: serde::de::Error>(self, _value: &str) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }

            fn visit_unit<E: serde::de::Error>(self) -> Result<ExtensionEntries, E> {
                Ok(ExtensionEntries::default())
            }
        }

        deserializer.deserialize_any(EntriesVisitor)
    }
}

/// The `skills:` block of `givn/config.yaml`.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Skills {
    /// Canonical agent target names: `opencode`, `agents`, `claude`.
    #[serde(default)]
    targets: Vec<String>,
}

/// Overlay artifact patch — same fields as `Artifact` but all are optional/defaulted
/// so the user only has to specify what they want to change.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct OverlayArtifact {
    id: String,
    #[serde(default)]
    generates: Vec<String>,
    #[serde(default)]
    requires: Vec<String>,
    template: Option<String>,
    instruction: Option<String>,
    #[serde(default)]
    optional: bool,
    #[serde(default, rename = "remove")]
    remove: bool,
    #[serde(default)]
    post: Vec<HookEntry>,
}

#[cfg(test)]
pub(crate) fn test_cwd_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RestoreCwd(std::path::PathBuf);

    impl Drop for RestoreCwd {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }

    fn project_with(config: &str, commands: &str) -> (tempfile::TempDir, RestoreCwd) {
        let directory = tempfile::tempdir().unwrap();
        let givn = directory.path().join("givn");
        std::fs::create_dir(&givn).unwrap();
        std::fs::write(givn.join("config.yaml"), config).unwrap();
        std::fs::write(givn.join("commands.yaml"), commands).unwrap();
        let restore = RestoreCwd(std::env::current_dir().unwrap());
        std::env::set_current_dir(directory.path()).unwrap();
        (directory, restore)
    }

    #[test]
    fn empty_overlay_returns_base_unchanged() {
        let base = Manifest::load_embedded().unwrap();
        let base_count = base.artifacts.len();
        let result = apply(base, "{}").unwrap();
        assert_eq!(result.artifacts.len(), base_count);
    }

    #[test]
    fn overlay_replaces_verify_command() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, r#"verify: {command: "cargo test"}"#).unwrap();
        assert_eq!(result.verify.command, "cargo test");
    }

    #[test]
    fn overlay_cannot_remove_review_artifact() {
        let base = Manifest::load_embedded().unwrap();
        let overlay = r#"
artifacts:
  - id: review
    remove: true
apply:
  requires: [tasks]
"#;
        let err = apply(base, overlay).unwrap_err();
        assert!(err.to_string().contains("protected"), "{err}");
    }

    #[test]
    fn overlay_can_remove_non_protected_artifact() {
        let base = Manifest::load_embedded().unwrap();
        let original_count = base.artifacts.len();
        // design-review is not protected; removing it (and rewiring the
        // tasks artifact that requires it) must keep working.
        let overlay = r#"
artifacts:
  - id: design-review
    remove: true
  - id: tasks
    requires: [specs]
apply:
  requires: [specs]
"#;
        let result = apply(base, overlay).unwrap();
        assert_eq!(result.artifacts.len(), original_count - 1);
        assert!(!result.artifacts.iter().any(|a| a.id == "design-review"));
    }

    #[test]
    fn overlay_can_add_artifact() {
        let base = Manifest::load_embedded().unwrap();
        let original_count = base.artifacts.len();
        let overlay = r#"
artifacts:
  - id: refactor
    generates: ["refactor.md"]
    requires: ["tasks"]
"#;
        let result = apply(base, overlay).unwrap();
        assert_eq!(result.artifacts.len(), original_count + 1);
        assert!(result.artifacts.iter().any(|a| a.id == "refactor"));
    }

    #[test]
    fn overlay_can_make_artifact_optional() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "artifacts:\n  - id: review\n    optional: true").unwrap();
        let review = result.artifacts.iter().find(|a| a.id == "review").unwrap();
        assert!(review.optional);
    }

    #[test]
    fn validation_rejects_dangling_requires() {
        let base = Manifest::load_embedded().unwrap();
        let overlay = r#"
artifacts:
  - id: new-artifact
    generates: ["new.md"]
    requires: ["nonexistent"]
"#;
        let err = apply(base, overlay).unwrap_err();
        assert!(err.to_string().contains("nonexistent"));
    }

    #[test]
    fn validation_rejects_cycle() {
        let base = Manifest::load_embedded().unwrap();
        // Create a cycle by making design require tasks (which already requires design).
        let overlay = r#"
artifacts:
  - id: design
    requires: ["tasks"]
"#;
        let err = apply(base, overlay).unwrap_err();
        assert!(err.to_string().contains("cycle"));
    }

    #[test]
    fn addons_arc42_inserts_artifact() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true").unwrap();
        assert!(result.artifacts.iter().any(|a| a.id == "arc42-docs"));
    }

    #[test]
    fn addons_arc42_not_present_by_default() {
        let base = Manifest::load_embedded().unwrap();
        assert!(!base.artifacts.iter().any(|a| a.id == "arc42-docs"));
    }

    #[test]
    fn addons_arc42_idempotent() {
        // Enabling twice must not insert two arc42-docs entries.
        let base = Manifest::load_embedded().unwrap();
        let once = apply(base, "addons:\n  arc42: true").unwrap();
        let count = once
            .artifacts
            .iter()
            .filter(|a| a.id == "arc42-docs")
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn addons_arc42_flag_is_set_on_effective_manifest() {
        // Regression: the arc42-docs artifact being present is not the same
        // as addons.arc42 being true on the effective manifest. Anything
        // reading manifest.addons.arc42 directly (not just artifact
        // presence) must also see it enabled.
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true").unwrap();
        assert!(
            result.addons.arc42,
            "addons.arc42 must be true after overlay"
        );
    }

    #[test]
    fn addons_communication_policy_flag_is_set_on_effective_manifest() {
        let base = Manifest::load_embedded().unwrap();
        assert!(
            base.addons.communication_policy,
            "all addons are enabled by default"
        );

        let result = apply(base, "addons:\n  communication_policy: true").unwrap();
        assert!(
            result.addons.communication_policy,
            "addons.communication_policy must be true after overlay enables it"
        );
    }

    #[test]
    fn addons_architecture_principles_flag_is_set_on_effective_manifest() {
        let base = Manifest::load_embedded().unwrap();
        assert!(
            base.addons.architecture_principles,
            "all addons are enabled by default"
        );

        let result = apply(base, "addons:\n  architecture_principles: true").unwrap();
        assert!(
            result.addons.architecture_principles,
            "addons.architecture_principles must be true after overlay enables it"
        );
    }

    #[test]
    fn addons_communication_policy_and_arc42_can_both_be_enabled() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true\n  communication_policy: true").unwrap();
        assert!(result.addons.arc42);
        assert!(result.addons.communication_policy);
        assert!(result.artifacts.iter().any(|a| a.id == "arc42-docs"));
    }

    #[test]
    fn addons_coverage_flag_is_set_on_effective_manifest() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  coverage: true").unwrap();
        assert!(
            result.addons.coverage,
            "addons.coverage must be true after overlay"
        );
    }

    #[test]
    fn addons_coverage_does_not_insert_artifact() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  coverage: true").unwrap();
        assert!(
            !result.artifacts.iter().any(|a| a.id == "coverage"),
            "coverage addon must no longer create a change-level artifact"
        );
    }

    #[test]
    fn addons_coverage_and_arc42_can_both_be_enabled() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true\n  coverage: true").unwrap();
        assert!(result.addons.arc42);
        assert!(result.addons.coverage);
        assert!(result.artifacts.iter().any(|a| a.id == "arc42-docs"));
    }

    #[test]
    fn addons_keep_the_all_addons_default_with_no_overlay_addons_block() {
        // No `addons:` key at all in the overlay must leave the all-enabled
        // default untouched, not reset it or panic.
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "{}").unwrap();
        assert!(result.addons.communication_policy);
        assert!(result.addons.challenge);
    }

    #[test]
    fn addon_instruction_resolves_via_cascade() {
        // The embedded addon instruction must be findable via cascade.
        let instruction = crate::config::cascade::resolve_instruction("arc42-docs").unwrap();
        assert!(
            instruction.contains("arc42"),
            "arc42-docs instruction must reference arc42"
        );
    }

    #[test]
    fn addon_template_resolves_via_cascade() {
        // The embedded addon template must be findable via cascade.
        // arc42-docs template name is "impact-assessment-template.md".
        let template =
            crate::config::cascade::resolve_template("arc42-docs", "impact-assessment-template.md")
                .unwrap();
        assert!(
            template.contains("STATUS"),
            "arc42-docs template must contain STATUS marker"
        );
    }

    #[test]
    fn coverage_refuses_legacy_two_command_keys_and_accepts_the_surviving_schema() {
        // The strict `Coverage` deserializer (deny_unknown_fields) refuses
        // the retired legacy keys on load — a project still carrying them
        // fails to load its configuration (risk R-81), and a leniency
        // regression here would silently restore the removed acceptance.
        let base = Manifest::load_embedded().unwrap();

        let non_e2e = apply(base.clone(), "coverage:\n  non_e2e_command: \"x\"\n").unwrap_err();
        assert!(
            non_e2e.to_string().contains("non_e2e_command"),
            "refusal must name the offending key, got: {non_e2e}"
        );

        let e2e = apply(base.clone(), "coverage:\n  e2e_command: \"x\"\n").unwrap_err();
        assert!(
            e2e.to_string().contains("e2e_command"),
            "refusal must name the offending key, got: {e2e}"
        );

        // The surviving schema must still parse through the overlay.
        let result = apply(
            base,
            "coverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n",
        )
        .unwrap();
        assert_eq!(result.coverage.measure_command, "true");
        assert_eq!(result.coverage.merge_command, "true");
    }

    #[test]
    fn partial_addons_block_keeps_renamed_defaults_enabled() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  outline: false").unwrap();
        let yaml = serde_yaml_ng::to_string(&result).unwrap();
        assert!(!result.addons.outline);
        assert!(
            yaml.contains("communication_policy: true"),
            "omitted communication_policy must keep the all-enabled default: {yaml}"
        );
        assert!(
            yaml.contains("challenge: true"),
            "omitted challenge must keep the all-enabled default: {yaml}"
        );
    }

    #[test]
    fn retired_addon_keys_are_refused_with_manual_transition_guidance() {
        for (retired, replacement) in [
            ("absolute_mode", "communication_policy"),
            ("grill_me", "challenge"),
            ("grill_with_docs", "challenge"),
        ] {
            let base = Manifest::load_embedded().unwrap();
            let error = apply(base, &format!("addons:\n  {retired}: true"))
                .expect_err("a retired addon key must be refused");
            let message = error.to_string();
            assert!(message.contains(retired), "{message}");
            assert!(message.contains(replacement), "{message}");
            assert!(message.contains("manual"), "{message}");
        }
    }

    #[test]
    fn load_effective_refuses_executable_settings_owned_by_commands_yaml() {
        let _lock = test_cwd_lock().lock().unwrap();
        let (_directory, _restore) = project_with(
            "givn_config_version: \"0.9.9\"\nverify:\n  command: \"cargo test\"\n",
            "",
        );
        let error = load_effective().expect_err("config.yaml must not own verify");
        let message = error.to_string();
        assert!(message.contains("verify"), "{message}");
        assert!(message.contains("givn/commands.yaml"), "{message}");
    }

    #[test]
    fn load_effective_refuses_policy_settings_owned_by_config_yaml() {
        let _lock = test_cwd_lock().lock().unwrap();
        let (_directory, _restore) = project_with(
            "givn_config_version: \"0.9.9\"\n",
            "addons:\n  outline: false\n",
        );
        let error = load_effective().expect_err("commands.yaml must not own addons");
        let message = error.to_string();
        assert!(message.contains("addons"), "{message}");
        assert!(message.contains("givn/config.yaml"), "{message}");
    }

    #[test]
    fn load_effective_resolves_renamed_addon_defaults_after_the_config_overlay() {
        let _lock = test_cwd_lock().lock().unwrap();
        let (_directory, _restore) = project_with(
            "givn_config_version: \"0.9.9\"\naddons:\n  outline: false\n",
            "",
        );
        let manifest = load_effective().unwrap();
        let yaml = serde_yaml_ng::to_string(&manifest).unwrap();
        assert!(!manifest.addons.outline);
        assert!(yaml.contains("communication_policy: true"), "{yaml}");
        assert!(yaml.contains("challenge: true"), "{yaml}");
    }

    #[test]
    fn stray_extras_key_parses_and_has_no_effect() {
        // Lenient overlay tolerance (design D1): a document still carrying
        // the retired top-level `extras:` key must parse and change nothing —
        // the effective manifest is byte-identical to the base.
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base.clone(), "extras: true\n").unwrap();
        assert_eq!(
            serde_yaml_ng::to_string(&result).unwrap(),
            serde_yaml_ng::to_string(&base).unwrap(),
            "a stray extras key must leave the effective manifest unchanged"
        );
    }
    #[test]
    fn commands_overlay_accepts_only_inline_executable_settings() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply_commands(
            base,
            "verify:\n  command: \"cargo test\"\nrun:\n  command: \"./serve\"\n  product_type: cli\ncoverage:\n  measure_command: \"true\"\n  merge_command: \"true\"\n",
        )
        .unwrap();
        assert_eq!(result.verify.command, "cargo test");
        assert_eq!(result.run.command, "./serve");
        assert_eq!(result.coverage.measure_command, "true");
    }

    #[test]
    fn commands_overlay_refuses_policy_and_artifact_keys() {
        for key in [
            "addons",
            "artifacts",
            "apply",
            "skills",
            "rules",
            "extensions",
            "record",
            "givn_config_version",
        ] {
            let base = Manifest::load_embedded().unwrap();
            let error = apply_commands(base, &format!("{key}: {{}}"))
                .expect_err("policy keys must be refused in commands.yaml");
            let message = error.to_string();
            assert!(message.contains(key), "{message}");
            assert!(message.contains("givn/config.yaml"), "{message}");
        }
    }

    #[test]
    fn config_overlay_refuses_executable_keys_owned_by_commands_yaml() {
        for key in ["verify", "run", "coverage"] {
            let base = Manifest::load_embedded().unwrap();
            let error = apply_config(base, &format!("{key}: {{}}"))
                .expect_err("executable keys must be refused in config.yaml");
            let message = error.to_string();
            assert!(message.contains(key), "{message}");
            assert!(message.contains("givn/commands.yaml"), "{message}");
        }
    }

    #[test]
    fn config_template_examples_are_supported_by_the_overlay_parser() {
        let base = Manifest::load_embedded().unwrap();
        let custom = apply(
            base.clone(),
            "artifacts:\n  - id: refactor\n    remove: true\n",
        )
        .unwrap();
        assert!(
            !custom
                .artifacts
                .iter()
                .any(|artifact| artifact.id == "refactor")
        );

        let optional = apply(
            base.clone(),
            "artifacts:\n  - id: review\n    optional: true\n",
        )
        .unwrap();
        assert!(
            optional
                .artifacts
                .iter()
                .find(|artifact| artifact.id == "review")
                .unwrap()
                .optional
        );

        let added = apply(
            base,
            "artifacts:\n  - id: refactor\n    generates: [\"refactor.md\"]\n    requires: [tasks]\n    template: refactor.md\n",
        )
        .unwrap();
        assert!(
            added
                .artifacts
                .iter()
                .any(|artifact| artifact.id == "refactor")
        );
    }

    #[test]
    fn shipped_templates_pass_their_own_schemas() {
        let base = Manifest::load_embedded().unwrap();
        let config =
            include_str!("../../assets/config.yaml.tmpl").replace("{{GIVN_VERSION}}", "0.9.10");
        apply_config(base.clone(), &config).expect("config template must parse as config.yaml");
        apply_commands(base, include_str!("../../assets/commands.yaml.tmpl"))
            .expect("commands template must parse as commands.yaml");
    }
}
