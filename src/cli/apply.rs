use anyhow::Result;
use crate::state::{ArtifactStatus, ChangeState};
use crate::config::overlay::load_effective;

/// Track task progress for a change (givn tracks, does not execute).
pub fn run(change: &str) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let change_dir = project_root.join("givn/changes").join(change);

    if !change_dir.exists() {
        anyhow::bail!("change '{}' not found under givn/changes/", change);
    }

    let manifest = load_effective()?;
    let state = ChangeState::from_manifest(&change_dir, &manifest);

    // Find the tasks artifact state.
    let tasks = state.artifacts.iter().find(|a| a.id == "tasks");

    println!("Change: {change}");
    println!("Stage:  {}", state.stage_label());

    if let Some(tasks_artifact) = tasks {
        if let Some(ref p) = tasks_artifact.tasks_progress {
            println!("Tasks:  {}/{} complete", p.done, p.total);

            if p.total == 0 {
                println!("\nNo task checkboxes found in tasks.md.");
                println!("Add `- [ ] task description` lines to tasks.md to track progress.");
            } else if p.is_complete() {
                println!("\n✓ All tasks complete.");
            } else {
                println!("\nPending tasks: {}", p.total - p.done);
                if let Ok(content) = std::fs::read_to_string(change_dir.join("tasks.md")) {
                    let pending: Vec<_> = content
                        .lines()
                        .filter(|l| l.contains("[ ]"))
                        .take(3)
                        .collect();
                    if !pending.is_empty() {
                        println!("\nNext up:");
                        for task in pending {
                            println!("  {}", task.trim());
                        }
                    }
                }
            }
        } else if tasks_artifact.status == ArtifactStatus::Pending {
            println!("\nNo tasks.md found — create it to track progress.");
        }
    }

    // Report missing apply.requires artifacts (Pending, but not Skipped).
    let missing: Vec<&str> = manifest.apply.requires.iter()
        .filter(|req| {
            state.artifacts.iter()
                .find(|a| &a.id == *req)
                .map(|a| a.status == ArtifactStatus::Pending)
                .unwrap_or(true)
        })
        .map(|s| s.as_str())
        .collect();

    if !missing.is_empty() {
        println!("\nMissing required artifacts for archive: {}", missing.join(", "));
    }

    Ok(())
}
