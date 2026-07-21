use crate::config::overlay::load_effective;
use crate::state::{ArtifactStatus, ChangeState, list_archived, list_changes};
use anyhow::Result;

/// Show lifecycle state derived from the filesystem.
///
/// Without --change: lists all active changes with a brief summary.
/// With --change <id>: shows the full artifact checklist, current stage,
/// next required step, and any optional steps still pending.
/// With --json: machine-readable output for use by agents/commands.
pub fn run(change: Option<&str>, json: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        anyhow::bail!("givn/ not found. Run `givn init` first.");
    }

    let manifest = load_effective()?;

    if let Some(id) = change {
        let change_dir = project_root.join("givn/changes").join(id);
        if !change_dir.exists() {
            let archived_dir = project_root.join("givn/archive").join(id);
            if archived_dir.exists() {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "id": id,
                            "archived": true,
                        }))?
                    );
                } else {
                    println!("Change '{id}': archived");
                }
                return Ok(());
            }
            anyhow::bail!("change '{}' not found", id);
        }
        let state = ChangeState::from_manifest(&change_dir, &manifest);
        if json {
            print_json(&state)?;
        } else {
            print_detail(&state);
        }
    } else {
        let changes = list_changes(&project_root, &manifest);
        let archived = list_archived(&project_root);

        // Auto-default to detail view when exactly one active change exists.
        if changes.len() == 1 {
            if json {
                print_json(&changes[0])?;
            } else {
                print_detail(&changes[0]);
            }
            return Ok(());
        }

        if changes.is_empty() && archived.is_empty() {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "active": [],
                        "archived": [],
                    }))?
                );
            } else {
                println!("No changes found. Run `givn new <id>` to create one.");
            }
            return Ok(());
        }

        if json {
            let active: Vec<serde_json::Value> = changes.iter().map(summary_json).collect();
            let out = serde_json::json!({
                "active": active,
                "archived": archived,
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        } else {
            if !changes.is_empty() {
                println!("Active changes:\n");
                for change in &changes {
                    print_summary(change);
                }
            }
            if !archived.is_empty() {
                println!("\nArchived: {}", archived.join(", "));
            }
        }
    }

    Ok(())
}

// ─── Human-readable output ───────────────────────────────────────────────────

fn print_summary(state: &ChangeState) {
    let next = state.next_required().map(|a| a.id.as_str()).unwrap_or("—");
    let stage = state.stage_label();
    println!("  {}  [{}]  next: {}", state.id, stage, next);
}

fn print_detail(state: &ChangeState) {
    println!("Change: {}", state.id);
    println!("Stage:  {}", state.stage_label());
    println!();

    // Artifact checklist.
    println!("Artifacts:");
    for a in &state.artifacts {
        let marker = match a.status {
            ArtifactStatus::Done => "✓",
            ArtifactStatus::Partial => "~",
            ArtifactStatus::Pending => "·",
            ArtifactStatus::Skipped => "⊘",
        };
        let tags = {
            let mut t = Vec::new();
            if a.gates_archive {
                t.push("gates archive");
            }
            if a.optional {
                t.push("optional");
            }
            if t.is_empty() {
                String::new()
            } else {
                format!("  ({})", t.join(", "))
            }
        };
        let extra = if let Some(ref p) = a.tasks_progress {
            format!("  {}/{} tasks", p.done, p.total)
        } else {
            String::new()
        };
        println!("  {marker} {}{tags}{extra}", a.id);
    }

    // Pending task descriptions when tasks are Partial (in-progress).
    if let Some(tasks) = state.artifacts.iter().find(|a| a.id == "tasks")
        && tasks.status == ArtifactStatus::Partial
    {
        let tasks_path = std::env::current_dir()
            .unwrap_or_default()
            .join("givn/changes")
            .join(&state.id)
            .join("tasks.md");
        if let Ok(content) = std::fs::read_to_string(&tasks_path) {
            let pending: Vec<&str> = content
                .lines()
                .filter(|l| l.contains("[ ]"))
                .take(3)
                .collect();
            if !pending.is_empty() {
                println!();
                println!("Pending tasks:");
                for task in &pending {
                    println!("  {}", task.trim());
                }
            }
        }
        println!();
        println!("Check off each task in tasks.md immediately after completing it.");
        println!("An unchecked box means that scenario is not done.");
    }

    println!();

    // Next required step.
    match state.next_required() {
        None => {
            println!("All required artifacts complete. Ready to archive.");
            println!("  Run: givn archive --change {}", state.id);
        }
        Some(next) => {
            println!("Next required:  {}", next.id);
            println!("  Run: givn instructions {} --change {}", next.id, state.id);
        }
    }

    // Optional steps still pending.
    let optional = state.optional_pending();
    if !optional.is_empty() {
        println!();
        println!("Optional (pending):");
        for a in optional {
            println!("  · {}", a.id);
        }
    }
}

// ─── JSON output ─────────────────────────────────────────────────────────────

fn summary_json(state: &ChangeState) -> serde_json::Value {
    let next = state.next_required().map(|a| a.id.as_str());
    serde_json::json!({
        "id": state.id,
        "stage": state.stage_label(),
        "next_required": next,
        "all_required_done": state.all_required_done(),
    })
}

fn print_json(state: &ChangeState) -> Result<()> {
    let artifacts: Vec<serde_json::Value> = state
        .artifacts
        .iter()
        .map(|a| {
            let status_str = match a.status {
                ArtifactStatus::Done => "done",
                ArtifactStatus::Partial => "partial",
                ArtifactStatus::Pending => "pending",
                ArtifactStatus::Skipped => "skipped",
            };
            let mut v = serde_json::json!({
                "id": a.id,
                "status": status_str,
                "optional": a.optional,
                "gates_archive": a.gates_archive,
            });
            if let Some(ref p) = a.tasks_progress {
                v["tasks_total"] = serde_json::json!(p.total);
                v["tasks_done"] = serde_json::json!(p.done);
            }
            v
        })
        .collect();

    let next = state.next_required().map(|a| {
        serde_json::json!({
            "id": a.id,
            "optional": a.optional,
        })
    });

    let optional_pending: Vec<&str> = state
        .optional_pending()
        .iter()
        .map(|a| a.id.as_str())
        .collect();

    let mut out = serde_json::json!({
        "id": state.id,
        "stage": state.stage_label(),
        "all_required_done": state.all_required_done(),
        "next_required": next,
        "optional_pending": optional_pending,
        "artifacts": artifacts,
        "verify_command": state.verify_command,
    });
    if !state.e2e_command.is_empty() {
        out["e2e_command"] = serde_json::json!(state.e2e_command);
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
