use crate::config::overlay::load_effective;
use crate::state::{ChangeState, list_archived, list_changes};
use anyhow::{Result, bail};

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
    // Resolve change: use explicit --change, or auto-select single active.
    let _change = match change {
        Some(id) => {
            crate::state::resolve_change_dir(project_root, id)?;
            id.to_string()
        }
        None => {
            let manifest = load_effective()?;
            let changes = list_changes(project_root, &manifest);
            match changes.len() {
                0 => bail!("No active changes. Create one with `givn new <id>` first."),
                1 => changes[0].id.clone(),
                _ => bail!("Multiple active changes. Use `--change <id>` to specify one."),
            }
        }
    };
    // --change is validated/resolved; artifact instructions are not change-specific.
    let manifest = load_effective()?;
    let artifact_definition = match manifest.artifacts.iter().find(|a| a.id == artifact) {
        Some(definition) => definition,
        None => {
            let known: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
            bail!(
                "unknown artifact '{}'. Known artifacts: {}",
                artifact,
                known.join(", ")
            );
        }
    };

    let content = crate::config::cascade::resolve_instruction(artifact)?;

    if json {
        let out = serde_json::json!({
            "id": artifact_definition.id,
            "generates": artifact_definition.generates,
            "requires": artifact_definition.requires,
            "instruction": content,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{content}");
    }

    Ok(())
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
        return report_change(&state, manifest.addons.domain_modeling, json);
    }

    // No change named: derive from current project state.
    let changes = list_changes(&project_root, &manifest);
    let archived = list_archived(&project_root);

    match changes.len() {
        0 => report_no_active_changes(&archived, json),
        1 => report_change(
            &changes[0],
            manifest.addons.domain_modeling,
            json,
        ),
        _ => report_multiple_changes(&changes, json),
    }
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

fn report_no_active_changes(archived: &[String], json: bool) -> Result<()> {
    let next_command = "givn new <change-id>".to_string();
    let summary = if archived.is_empty() {
        "No changes exist yet.".to_string()
    } else {
        format!(
            "No active changes. {} archived change(s): {}.",
            archived.len(),
            archived.join(", ")
        )
    };

    if json {
        print_json(
            "no-active-changes",
            &summary,
            &next_command,
            None,
            &generic_overview(),
        )
    } else {
        println!("{}", generic_overview());
        println!();
        println!("Status: {summary}");
        println!();
        println!("Next: run `{next_command}` to start one, or `/givn-propose <description>`");
        println!("(as a skill) to plan and scaffold it in one pass.");
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
    let summary = format!("Change '{id}' is archived. Nothing further to do on it.");
    if json {
        print_json(
            "archived",
            &summary,
            "givn new <id>",
            None,
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

fn report_change(state: &ChangeState, domain_modeling: bool, json: bool) -> Result<()> {
    let (summary, next_command, next_skill) = describe_next_step(state);
    let skill_to_load = state
        .next_required()
        .and_then(|a| domain_skill_to_load(Some(a.id.as_str()), domain_modeling));

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
        if let Some(skill) = skill_to_load {
            println!("Skill to load now: `{skill}`");
        }
        println!("Next: run `{next_command}`");
        if let Some(skill) = next_skill {
            println!("  (as a skill in your AI agent: `/{skill}`)");
        }
    }
    Ok(())
}

/// The domain-modeling skill to route to at the given phase, if any.
///
/// Routed only at the specs and design phases — the two artifacts the skill's
/// trigger text names ("before writing .feature specs or design.md") — and
/// only when the addon is enabled. When off, behaviour is identical to before.
fn domain_skill_to_load(
    next_artifact_id: Option<&str>,
    domain_modeling: bool,
) -> Option<&'static str> {
    if !domain_modeling {
        return None;
    }
    match next_artifact_id {
        Some("specs") | Some("design") => Some("givn-domain-modeling"),
        _ => None,
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

/// Map a manifest artifact id to its conventional slash-command/skill name.
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
development: the `.feature` file is the executable specification. Workflow: \
propose → spec → design → design-review → tasks → implement → review → \
archive. Skills/slash-commands are named `/givn-<step>` (e.g. `/givn-propose`, \
`/givn-implement`). Run `givn status` or `givn status --change <id>` for a \
full checklist view with pending tasks, or `givn instructions <artifact>` for \
that artifact's writing instructions."
        .to_string()
}

fn print_json(
    status: &str,
    summary: &str,
    next_command: &str,
    change_id: Option<&str>,
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
    }

    #[test]
    fn domain_skill_to_load_routes_specs_and_design_when_addon_enabled() {
        assert_eq!(
            domain_skill_to_load(Some("specs"), true),
            Some("givn-domain-modeling")
        );
        assert_eq!(
            domain_skill_to_load(Some("design"), true),
            Some("givn-domain-modeling")
        );
    }

    #[test]
    fn domain_skill_to_load_is_empty_without_addon_or_other_phases() {
        assert_eq!(domain_skill_to_load(Some("specs"), false), None);
        assert_eq!(domain_skill_to_load(Some("review"), true), None);
        assert_eq!(domain_skill_to_load(Some("tasks"), true), None);
        assert_eq!(domain_skill_to_load(None, true), None);
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
