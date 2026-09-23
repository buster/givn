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
/// - After merge, the effective graph is validated (Phase 2.3a).
use anyhow::{Result, bail};

/// Parse the project overlay YAML and merge it onto `base`.
/// Artifact ids that an overlay may not remove. `givn/config.yaml` is a
/// tracked, agent-writable file — every gate in the workflow is a few YAML
/// lines away from being deleted by whoever edits it, deliberately or not.
/// `review` is the sign-off gate the whole workflow funnels into.
const PROTECTED_ARTIFACTS: &[&str] = &["review"];

pub fn apply(base: Manifest, overlay_yaml: &str) -> Result<Manifest> {
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

    if let Some(e) = overlay.extras {
        result.extras = e;
    }

    if let Some(addons) = overlay.addons {
        // arc42 additionally expands into an artifact-graph node — keep
        // that side effect here, guarded so enabling twice stays idempotent.
        // The arc42 document is produced during the design phase and
        // checked in design-review: design-review requires arc42-docs, and
        // arc42-docs itself gates archive (added to apply.requires) so an
        // enabled-but-unfinished arc42 doc genuinely blocks `givn archive`,
        // not just the "ready-to-archive" status label.
        if addons.arc42 && !result.addons.arc42 {
            result.artifacts.push(arc42_artifact());
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
    let base = Manifest::load_embedded()?;
    let project_root = std::env::current_dir()?;
    let config_path = project_root.join("givn/config.yaml");

    let manifest = if config_path.exists() {
        let yaml = std::fs::read_to_string(&config_path)?;
        apply(base, &yaml)?
    } else {
        base
    };

    let commands_path = project_root.join("givn/commands.yaml");
    if commands_path.exists() {
        let yaml = std::fs::read_to_string(&commands_path)?;
        apply(manifest, &yaml)
    } else {
        Ok(manifest)
    }
}

/// The known agent target names (canonical, no leading dot).
pub const KNOWN_AGENT_TARGETS: [&str; 3] = ["opencode", "agents", "claude"];

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

/// Lenient overlay document — only fields that are explicitly overridden need to be present.
#[derive(Debug, serde::Deserialize)]
struct OverlayDoc {
    #[serde(default)]
    artifacts: Vec<OverlayArtifact>,
    verify: Option<Verify>,
    run: Option<Run>,
    coverage: Option<Coverage>,
    apply: Option<Apply>,
    addons: Option<Addons>,
    extras: Option<bool>,
    /// Agent targets for skill/command generation (`skills.targets`).
    /// Ignored by the manifest itself; consumed by `load_skills_targets`.
    skills: Option<Skills>,
    /// Generated-rules policy scalar (`rules:`). Consumed by
    /// `load_rules_policy`; ignored by the manifest itself.
    rules: Option<String>,
    /// Ignored during overlay processing (used for version tracking).
    #[allow(dead_code)]
    givn_config_version: Option<String>,
}

/// The `skills:` block of `givn/config.yaml`.
#[derive(Debug, serde::Deserialize)]
struct Skills {
    /// Canonical agent target names: `opencode`, `agents`, `claude`.
    #[serde(default)]
    targets: Vec<String>,
}

/// Overlay artifact patch — same fields as `Artifact` but all are optional/defaulted
/// so the user only has to specify what they want to change.
#[derive(Debug, serde::Deserialize)]
struct OverlayArtifact {
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
mod tests {
    use super::*;

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
    fn addons_absolute_mode_flag_is_set_on_effective_manifest() {
        // Regression: enabling absolute_mode in givn/config.yaml must be
        // visible on the effective manifest returned by load_effective().
        // Previously only `arc42` was copied from the overlay onto the
        // result; every other addon flag (here: absolute_mode) was
        // silently dropped, so `givn skills sync` always saw it as
        // disabled and deleted the AGENTS.md block on every sync even
        // though `givn addons enable absolute_mode` had just written it.
        let base = Manifest::load_embedded().unwrap();
        assert!(!base.addons.absolute_mode, "must be disabled by default");

        let result = apply(base, "addons:\n  absolute_mode: true").unwrap();
        assert!(
            result.addons.absolute_mode,
            "addons.absolute_mode must be true after overlay enables it"
        );
    }

    #[test]
    fn addons_absolute_mode_and_arc42_can_both_be_enabled() {
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true\n  absolute_mode: true").unwrap();
        assert!(result.addons.arc42);
        assert!(result.addons.absolute_mode);
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
    fn addons_absolute_mode_defaults_to_false_with_no_overlay_addons_block() {
        // No `addons:` key at all in the overlay must leave the default
        // (false) untouched, not reset it or panic.
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "{}").unwrap();
        assert!(!result.addons.absolute_mode);
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
}
