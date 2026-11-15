use crate::config::overlay::load_effective;
use crate::state::{ChangeState, list_archived, list_changes};
use anyhow::{Result, bail};
use std::path::Path;

/// Instruction ids served outside the artifact manifest. They resolve through
/// the same cascade and are named in the unknown-id diagnostic.
const POLICY_INSTRUCTION_IDS: &[&str] = &[
    "start",
    "dialog",
    "dialog-examples",
    "challenge",
    "dev-principles",
    "domain-modeling",
    "architecture-principles",
    "review-reverse",
    "specs-reverse",
    "tasks-reverse",
    "ise",
    "steps-rust",
    "steps-python",
    "steps-node",
    "steps-java",
    "steps-go",
    "steps-ruby",
];

/// Policy instruction ids served only while a named addon is effective. The
/// pair is (instruction id, addon id); an effective-off addon refuses the
/// request by name before the cascade runs.
const ADDON_BOUND_INSTRUCTION_IDS: &[(&str, &str)] = &[("ise", "ise")];

/// Instruction ids retired from the callable surface. They are refused before
/// the cascade so a project-owned override cannot restore an old entry point.
const RETIRED_INSTRUCTION_IDS: &[(&str, &str)] = &[("explore", "givn instructions start")];

/// Show instructions.
///
/// `givn instructions <artifact>` — unchanged: resolves the instruction for
/// that artifact via the cascade (project override wins over embedded
/// default). Requires a valid `givn/` project.
///
/// `givn instructions` (no artifact) — NEW: context-aware. Inspects the
/// current project/change state and returns generic, actionable guidance:
/// what givn is, and specifically which command or skill to run next given
/// the state right now. This is the entry point an agent runs at the start
/// of a session, or whenever unsure how to continue.
pub fn run(artifact: Option<&str>, change: Option<&str>, json: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    match artifact {
        Some(artifact) => run_artifact(artifact, change, &project_root, json),
        None => run_generic(change, json),
    }
}

/// `givn instructions <artifact>` — prints artifact writing instructions.
/// When `change` is not provided and exactly one active change exists,
/// auto-selects it.
fn run_artifact(
    artifact: &str,
    change: Option<&str>,
    project_root: &std::path::Path,
    json: bool,
) -> Result<()> {
    if let Some((_, replacement)) = RETIRED_INSTRUCTION_IDS
        .iter()
        .find(|(retired, _)| *retired == artifact)
    {
        bail!(
            "the '{}' entry point is retired; use `{replacement}` instead",
            artifact
        );
    }
    let manifest = load_effective()?;
    if let Some((_, addon)) = ADDON_BOUND_INSTRUCTION_IDS
        .iter()
        .find(|(id, _)| *id == artifact)
        && !crate::skills::render::manifest_addon_enabled(&manifest, addon)
    {
        bail!(
            "instruction '{artifact}' requires the '{addon}' addon; enable it with `givn addons enable {addon}`"
        );
    }
    let artifact_definition = manifest.artifacts.iter().find(|a| a.id == artifact);
    let extension_rules = extension_rules(project_root, artifact);
    let addons = addon_flags(&manifest);

    let Some(artifact_definition) = artifact_definition else {
        // A non-artifact id may be a policy instruction (e.g. `dialog`). It is
        // resolved through the same instruction cascade before any change
        // context is required, so a canonical policy is reachable in any
        // initialized project. An unresolvable id keeps the artifact error.
        let content = crate::config::cascade::resolve_instruction(artifact).map_err(|_| {
            let mut known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
            known.extend(POLICY_INSTRUCTION_IDS);
            anyhow::anyhow!(
                "unknown artifact '{}'. Known artifacts: {}",
                artifact,
                known.join(", ")
            )
        })?;
        let content = crate::skills::render::render_conditionals(&content, &addons)?;
        let content = crate::config::extensions::append(&content, &extension_rules);
        let skill_to_load = domain_skill_to_load(manifest.addons.domain_modeling);
        if json {
            let mut out = serde_json::json!({
                "id": artifact,
                "generates": Vec::<String>::new(),
                "requires": Vec::<String>::new(),
                "instruction": content,
            });
            if let Some(skill) = skill_to_load {
                out["skill_to_load"] = serde_json::json!(skill);
            }
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else {
            println!("{content}");
            if let Some(skill) = skill_to_load {
                println!("Skill to load now: `{skill}`");
            }
        }
        return Ok(());
    };

    // Resolve change: use explicit --change, or auto-select single active.
    let _change = match change {
        Some(id) => {
            crate::state::resolve_change_dir(project_root, id)?;
            id.to_string()
        }
        None => {
            let changes = list_changes(project_root, &manifest);
            match changes.len() {
                0 => bail!("No active changes. Create one with `givn new <id>` first."),
                1 => changes[0].id.clone(),
                _ => bail!("Multiple active changes. Use `--change <id>` to specify one."),
            }
        }
    };
    // --change is validated/resolved; artifact instructions are not change-specific.
    let content = crate::config::cascade::resolve_instruction(artifact)?;
    let content = crate::skills::render::render_conditionals(&content, &addons)?;
    let content = crate::config::extensions::append(&content, &extension_rules);
    let skill_to_load = domain_skill_to_load(manifest.addons.domain_modeling);

    if json {
        let mut out = serde_json::json!({
            "id": artifact_definition.id,
            "generates": artifact_definition.generates,
            "requires": artifact_definition.requires,
            "instruction": content,
        });
        if let Some(skill) = skill_to_load {
            out["skill_to_load"] = serde_json::json!(skill);
        }
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{content}");
        if let Some(skill) = skill_to_load {
            println!("Skill to load now: `{skill}`");
        }
    }

    Ok(())
}

/// The project's extension rules for `artifact`, if recorded in
/// `givn/config.yaml`.
fn extension_rules(project_root: &std::path::Path, artifact: &str) -> Vec<String> {
    crate::config::extensions::load(project_root)
        .into_iter()
        .find(|(id, _)| id == artifact)
        .map(|(_, rules)| rules)
        .unwrap_or_default()
}

/// The addon-name/enabled map the instruction renderer resolves conditional
/// blocks against. Mirrors the sync render path's flags; the names match
/// `render::KNOWN_ADDON_KEYS`.
fn addon_flags(manifest: &crate::config::manifest::Manifest) -> Vec<(&'static str, bool)> {
    crate::skills::render::KNOWN_ADDON_KEYS
        .iter()
        .map(|key| {
            (
                *key,
                crate::skills::render::manifest_addon_enabled(manifest, key),
            )
        })
        .collect()
}

/// `givn instructions` with no artifact — the generic, context-aware entry point.
fn run_generic(change: Option<&str>, json: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        return report_not_initialised(json);
    }

    let manifest = load_effective()?;

    // If a specific change was named, report on that one.
    if let Some(id) = change {
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        if !change_dir.exists() {
            let archived_dir = project_root.join("givn/archive").join(id);
            if archived_dir.exists() {
                return report_archived_change(id, json);
            }
            bail!("change '{}' not found", id);
        }
        let state = ChangeState::from_manifest(&change_dir, &manifest);
        let readme_hint = readme_nudge(&project_root, &manifest);
        return report_change(&state, manifest.addons.domain_modeling, json, readme_hint);
    }

    // No change named: derive from current project state.
    let changes = list_changes(&project_root, &manifest);
    let archived = list_archived(&project_root);
    let unproven = crate::state::list_unproven_archives(&project_root);
    let readme_hint = readme_nudge(&project_root, &manifest);

    match changes.len() {
        0 => report_no_active_changes(
            &archived,
            &unproven,
            capability_state(&project_root),
            json,
            readme_hint,
        ),
        1 => report_change(
            &changes[0],
            manifest.addons.domain_modeling,
            json,
            readme_hint,
        ),
        _ => report_multiple_changes(&changes, json),
    }
}

/// The README advisory line, when the readme addon is enabled and the README
/// check reports findings. Text-only: the JSON report shape is unchanged.
fn readme_nudge(
    project_root: &std::path::Path,
    manifest: &crate::config::manifest::Manifest,
) -> Option<String> {
    if !manifest.addons.readme {
        return None;
    }
    let findings = crate::cli::readme::check_readme(project_root);
    if findings.is_empty() {
        return None;
    }
    Some(format!(
        "README: {} finding(s) — run `givn readme check` to see them.",
        findings.len()
    ))
}

// ─── Report builders ────────────────────────────────────────────────────────

fn report_not_initialised(json: bool) -> Result<()> {
    let next_command = "givn init".to_string();
    let summary = "This project has not run `givn init` yet.".to_string();

    if json {
        print_json(
            "not-initialised",
            &summary,
            &next_command,
            None,
            None,
            &generic_overview(),
        )
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Status: not initialised.");
        println!();
        println!("Next: run `{next_command}` to scaffold this project for givn.");
        Ok(())
    }
}

/// Count permanent use cases and feature files under `givn/specs`. The routing
/// step needs this to tell a modelled capability area from a greenfield one
/// without inspecting the tree itself.
fn capability_state(project_root: &Path) -> (usize, usize) {
    let specs_root = project_root.join("givn/specs");
    let mut usecases = 0;
    let mut features = 0;
    let Ok(entries) = std::fs::read_dir(&specs_root) else {
        return (0, 0);
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let narrative = if path.file_name().and_then(|name| name.to_str()) == Some("fragments") {
            path.join("fragment.md")
        } else {
            path.join("usecase.md")
        };
        if narrative.exists() {
            usecases += 1;
        }
        features += count_feature_files(&path);
    }
    (usecases, features)
}

fn count_feature_files(dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                count_feature_files(&path)
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("feature") {
                1
            } else {
                0
            }
        })
        .sum()
}

/// One sentence naming the capability model's state. The greenfield wording
/// also names the routing consequence, because a bare count does not tell the
/// reader what to do with it.
fn capability_summary((usecases, features): (usize, usize)) -> String {
    if usecases == 0 && features == 0 {
        "Capabilities: none — `givn/specs` holds no use case, so no capability area is modelled; ideation is the default start for new work.".to_string()
    } else {
        format!(
            "Capabilities: {usecases} use case(s), {features} feature file(s) under `givn/specs`."
        )
    }
}

fn report_no_active_changes(
    archived: &[String],
    unproven: &[String],
    capabilities: (usize, usize),
    json: bool,
    readme_hint: Option<String>,
) -> Result<()> {
    let next_command = if let Some(id) = unproven.first() {
        format!("givn archive --change {id}")
    } else {
        "givn new <change-id>".to_string()
    };
    let next_skill = if unproven.is_empty() {
        "givn-start"
    } else {
        "givn-archive"
    };
    let summary = if !unproven.is_empty() {
        format!(
            "No active changes. {} unproven archive candidate(s): {}. {}",
            unproven.len(),
            unproven.join(", "),
            capability_summary(capabilities)
        )
    } else if archived.is_empty() {
        format!("No changes exist yet. {}", capability_summary(capabilities))
    } else {
        format!(
            "No active changes. {} archived change(s): {}. {}",
            archived.len(),
            archived.join(", "),
            capability_summary(capabilities)
        )
    };

    if json {
        print_json(
            "no-active-changes",
            &summary,
            &next_command,
            None,
            Some(next_skill),
            &generic_overview(),
        )
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Status: {summary}");
        println!();
        for id in unproven {
            println!(
                "Unproven archive candidate '{id}' — resume with `givn archive --change {id}`."
            );
        }
        if !unproven.is_empty() {
            println!();
        }
        if let Some(hint) = readme_hint {
            println!("{hint}");
            println!();
        }
        if unproven.is_empty() {
            println!(
                "Next: for new or unclassified work, load the `givn-start` skill (OMP: `/skill:givn-start`) to investigate and choose the next action."
            );
            println!(
                "For a change whose goal and scope are already clear, `{next_command}` remains the direct alternative."
            );
        } else {
            println!(
                "Next: run `{next_command}` to resume the candidate, or load the `givn-archive` skill."
            );
        }
        Ok(())
    }
}

fn report_multiple_changes(changes: &[ChangeState], json: bool) -> Result<()> {
    let summary = format!("{} active changes — pick one to continue.", changes.len());

    if json {
        let list: Vec<serde_json::Value> = changes
            .iter()
            .map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "stage": c.stage_label(),
                    "next_required": c.next_required().map(|a| a.id.as_str()),
                })
            })
            .collect();
        let out = serde_json::json!({
            "status": "multiple-active-changes",
            "summary": summary,
            "changes": list,
            "next_command": "givn instructions --change <id>",
            "overview": generic_overview(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Status: {summary}");
        println!();
        for c in changes {
            let next = c.next_required().map(|a| a.id.as_str()).unwrap_or("—");
            println!("  {}  [{}]  next: {}", c.id, c.stage_label(), next);
        }
        println!();
        println!("Next: run `givn instructions --change <id>` for the one you want to continue.");
    }
    Ok(())
}

fn report_archived_change(id: &str, json: bool) -> Result<()> {
    let pending = std::env::current_dir().ok().is_some_and(|root| {
        matches!(
            crate::proof::read_status(&root, id),
            crate::proof::ArchiveProof::Pending | crate::proof::ArchiveProof::PendingWithActive
        )
    });
    if pending {
        let summary = format!("Change '{id}' has an unproven archive candidate.");
        let next_command = format!("givn archive --change {id}");
        if json {
            return print_json(
                "unproven-archive",
                &summary,
                &next_command,
                None,
                Some("givn-archive"),
                &generic_overview(),
            );
        }
        println!("{}", generic_overview());
        println!();
        println!("Status: {summary}");
        println!();
        println!("Next: run `{next_command}` to resume and prove it.");
        return Ok(());
    }

    let summary = format!("Change '{id}' is archived. Nothing further to do on it.");
    if json {
        print_json(
            "archived",
            &summary,
            "givn new <change-id>",
            None,
            Some("givn-start"),
            &generic_overview(),
        )
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Status: {summary}");
        println!();
        println!(
            "Next: run `givn new <id>` to start a new change, or `givn status` to see others."
        );
        Ok(())
    }
}

fn report_change(
    state: &ChangeState,
    domain_modeling: bool,
    json: bool,
    readme_hint: Option<String>,
) -> Result<()> {
    let (summary, next_command, next_skill) = describe_next_step(state);
    let skill_to_load = if state.next_required().is_some() {
        domain_skill_to_load(domain_modeling)
    } else {
        None
    };

    if json {
        let mut out = serde_json::json!({
            "status": "active-change",
            "id": state.id,
            "stage": state.stage_label(),
            "summary": summary,
            "next_command": next_command,
            "next_skill": next_skill,
            "overview": generic_overview(),
        });
        if let Some(skill) = skill_to_load {
            out["skill_to_load"] = serde_json::json!(skill);
        }
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Change: {}  [{}]", state.id, state.stage_label());
        println!("Status: {summary}");
        println!();
        if let Some(hint) = readme_hint {
            println!("{hint}");
            println!();
        }
        if let Some(skill) = skill_to_load {
            println!("Skill to load now: `{skill}`");
        }
        println!("Next: run `{next_command}`");
        if let Some(skill) = next_skill {
            println!("  (load the `{skill}` skill; in OMP: `/skill:{skill}`)");
        }
        println!(
            "This step applies when continuing this change; it does not authorize unrelated work."
        );
    }
    Ok(())
}

/// The domain-modeling skill to route to, if any.
///
/// Routed during every workflow step of a change (any phase), and only when
/// the domain-modeling addon is enabled. The standalone domain documents do not
/// depend on the arc42 addon.
fn domain_skill_to_load(domain_modeling: bool) -> Option<&'static str> {
    if domain_modeling {
        Some("givn-domain-modeling")
    } else {
        None
    }
}

/// Derive (summary, shell command, optional skill name) for the immediate
/// next step of a change, given its current lifecycle state.
fn describe_next_step(state: &ChangeState) -> (String, String, Option<String>) {
    if state.all_required_done() {
        return (
            "All required artifacts are complete. Ready to archive.".to_string(),
            format!("givn archive --change {}", state.id),
            Some("givn-archive".to_string()),
        );
    }

    match state.next_required() {
        Some(next) if next.id == "tasks" => {
            let in_progress = next
                .tasks_progress
                .as_ref()
                .map(|p| p.done > 0 && !p.is_complete())
                .unwrap_or(false);
            if in_progress {
                let p = next.tasks_progress.as_ref().unwrap();
                (
                    format!(
                        "Implementation in progress ({}/{} tasks done). \
                         Check off each task in tasks.md immediately after \
                         completing it — an unchecked box means that \
                         scenario is not done.",
                        p.done, p.total
                    ),
                    format!("givn instructions tasks --change {}", state.id),
                    Some("givn-implement".to_string()),
                )
            } else {
                (
                    "Task list not written yet.".to_string(),
                    format!("givn instructions tasks --change {}", state.id),
                    Some("givn-tasks".to_string()),
                )
            }
        }
        Some(next) => (
            format!("Next required artifact: {}.", next.id),
            format!("givn instructions {} --change {}", next.id, state.id),
            Some(artifact_to_skill(&next.id)),
        ),
        None => (
            "Nothing required is pending.".to_string(),
            format!("givn status --change {}", state.id),
            None,
        ),
    }
}

/// Map a manifest artifact id to its conventional skill name.
fn artifact_to_skill(artifact_id: &str) -> String {
    match artifact_id {
        "proposal" => "givn-propose".to_string(),
        "specs" => "givn-spec".to_string(),
        other => format!("givn-{other}"),
    }
}

/// The generic overview of givn: what it is, the lifecycle, and where to
/// look for more. This text is deliberately short — the point of
/// `givn instructions` is to say what to do NEXT, not to re-explain the
/// whole system every time.
fn generic_overview() -> String {
    "givn — from Given to done. This project uses givn for spec-driven \
 development: the `.feature` file is the executable specification, and its \
 owning use case supplies the domain contract. Consult `usecase.md` and any \
 linked ideation topic before changing a capability; consult confirmed \
 Personas as review lenses, not as Gherkin actors. Workflow: \
 propose → spec → design → design-review → tasks → implement → review → \
 archive. Generated guidance is skills-only; no `/givn-*` command files are \
 installed. Claude Code and OpenCode use their native skill invocation, the \
 neutral `.agents` target uses skill discovery, OMP uses `/skill:givn-<step>`, \
 and Maki uses an explicit request or an agent-selected skill. Run `givn status` \
 or `givn status --change <id>` for a full checklist view with pending tasks, \
 or `givn instructions <artifact>` for that artifact's writing instructions. \
 For every user-facing question, follow the effective dialog policy from \
 `givn instructions dialog`; load it once per context before the first \
 question (once per context rather than rereading before every question), \
 including clarification and confirmation outside a named workflow, and pass \
 the obligation explicitly when work is handed to another context. This \
 obligation applies with optional addons disabled and outside any named \
 skill. Before the first question of a policy-indexed type, read that type's companion examples \
 from `givn instructions dialog-examples`; the policy is not loaded \
 automatically. A reported next step applies when continuing that change; it \
 does not authorize unrelated work. For new or unclassified work, load \
 `givn-start` to choose answer-only, direct work, ideation, or a formal change; \
 `givn new <change-id>` remains the optional direct alternative. Direct entry to \
 ideation, proposal, and clearly authorized direct work remains available."
        .to_string()
}

fn print_json(
    status: &str,
    summary: &str,
    next_command: &str,
    change_id: Option<&str>,
    next_skill: Option<&str>,
    overview: &str,
) -> Result<()> {
    let mut out = serde_json::json!({
        "status": status,
        "summary": summary,
        "next_command": next_command,
        "overview": overview,
    });
    if let Some(id) = change_id {
        out["id"] = serde_json::json!(id);
    }
    if let Some(skill) = next_skill {
        out["next_skill"] = serde_json::json!(skill);
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::manifest::Manifest;

    #[test]
    fn artifact_to_skill_maps_proposal_and_specs_specially() {
        assert_eq!(artifact_to_skill("proposal"), "givn-propose");
        assert_eq!(artifact_to_skill("specs"), "givn-spec");
    }

    #[test]
    fn artifact_to_skill_defaults_to_prefixed_id() {
        assert_eq!(artifact_to_skill("design"), "givn-design");
        assert_eq!(artifact_to_skill("design-review"), "givn-design-review");
        assert_eq!(artifact_to_skill("tasks"), "givn-tasks");
        assert_eq!(artifact_to_skill("review"), "givn-review");
    }

    #[test]
    fn generic_overview_mentions_the_lifecycle_and_status_command() {
        let overview = generic_overview();
        assert!(overview.contains("propose"));
        assert!(overview.contains("archive"));
        assert!(overview.contains("givn status"));
        assert!(overview.contains("use case"));
        assert!(overview.contains("Personas"));
    }

    #[test]
    fn capability_summary_calls_a_tree_without_specs_greenfield() {
        let dir = std::env::temp_dir().join("givn_instructions_test_greenfield");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(capability_state(&dir), (0, 0));

        let summary = capability_summary(capability_state(&dir));
        assert!(summary.contains("ideation is the default start"));
    }

    #[test]
    fn capability_summary_counts_use_cases_and_feature_files() {
        let dir = std::env::temp_dir().join("givn_instructions_test_capability_counts");
        let _ = std::fs::remove_dir_all(&dir);
        let usecase = dir.join("givn/specs/author-change");
        std::fs::create_dir_all(usecase.join("nested")).unwrap();
        std::fs::write(usecase.join("usecase.md"), "# Author change").unwrap();
        std::fs::write(usecase.join("behaviour.feature"), "Feature: x").unwrap();
        std::fs::write(usecase.join("nested/deep.feature"), "Feature: y").unwrap();

        assert_eq!(capability_state(&dir), (1, 2));
        let summary = capability_summary((1, 2));
        assert!(summary.contains("1 use case(s), 2 feature file(s)"));
        assert!(!summary.contains("ideation is the default start"));
    }

    #[test]
    fn domain_skill_to_load_routes_when_the_domain_addon_enabled() {
        assert_eq!(domain_skill_to_load(true), Some("givn-domain-modeling"));
    }

    #[test]
    fn domain_skill_to_load_is_empty_without_the_domain_addon() {
        assert_eq!(domain_skill_to_load(false), None);
    }

    #[test]
    fn describe_next_step_points_to_specs_after_proposal() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_instructions_test_specs_next");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("proposal.md"), "# Proposal").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        let (summary, command, skill) = describe_next_step(&state);
        assert!(summary.contains("specs"));
        assert!(command.contains("givn instructions specs --change"));
        assert_eq!(skill.as_deref(), Some("givn-spec"));
    }

    #[test]
    fn describe_next_step_recommends_writing_tasks_when_not_started() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_instructions_test_tasks_not_started");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("proposal.md"), "# Proposal").unwrap();
        std::fs::create_dir_all(dir.join("specs/cap")).unwrap();
        std::fs::write(
            dir.join("specs/cap/cap.feature"),
            "@givn.delta @cap\nFeature: cap\n  Scenario: s\n    Given x\n",
        )
        .unwrap();
        std::fs::write(dir.join("design.md"), "# Design").unwrap();
        std::fs::write(dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        let (summary, command, skill) = describe_next_step(&state);
        assert!(summary.contains("not written"));
        assert!(command.contains("givn instructions tasks --change"));
        assert_eq!(skill.as_deref(), Some("givn-tasks"));
    }

    #[test]
    fn describe_next_step_recommends_implement_when_tasks_in_progress() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_instructions_test_tasks_in_progress");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("proposal.md"), "# Proposal").unwrap();
        std::fs::create_dir_all(dir.join("specs/cap")).unwrap();
        std::fs::write(
            dir.join("specs/cap/cap.feature"),
            "@givn.delta @cap\nFeature: cap\n  Scenario: s\n    Given x\n",
        )
        .unwrap();
        std::fs::write(dir.join("design.md"), "# Design").unwrap();
        std::fs::write(dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
        std::fs::write(dir.join("tasks.md"), "- [x] done\n- [ ] pending\n").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        let (summary, _command, skill) = describe_next_step(&state);
        assert!(summary.contains("in progress"));
        assert!(summary.contains("1/2"));
        assert_eq!(skill.as_deref(), Some("givn-implement"));
    }

    #[test]
    fn describe_next_step_recommends_archive_when_all_required_done() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_instructions_test_all_done");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("proposal.md"), "# Proposal").unwrap();
        std::fs::create_dir_all(dir.join("specs/cap")).unwrap();
        std::fs::write(
            dir.join("specs/cap/cap.feature"),
            "@givn.delta @cap\nFeature: cap\n  Scenario: s\n    Given x\n",
        )
        .unwrap();
        std::fs::write(dir.join("design.md"), "# Design").unwrap();
        std::fs::write(dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n").unwrap();
        std::fs::write(dir.join("tasks.md"), "- [x] done\n").unwrap();
        std::fs::write(dir.join("review.md"), "REVIEW: PASS\n").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        let (summary, command, skill) = describe_next_step(&state);
        assert!(summary.contains("Ready to archive"));
        assert!(command.contains("givn archive --change"));
        assert_eq!(skill.as_deref(), Some("givn-archive"));
    }
}
