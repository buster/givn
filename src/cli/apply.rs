use anyhow::Result;
use crate::state::ChangeState;
use crate::config::overlay::load_effective;

/// Track task progress for a change (givn tracks, does not execute).
///
/// Reports progress and the next pending task group.
/// Enforces apply.requires: all required artifacts must exist before tracking.
pub fn run(change: &str) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let change_dir = project_root.join("givn/changes").join(change);

    if !change_dir.exists() {
        anyhow::bail!("change '{}' not found under givn/changes/", change);
    }

    let state = ChangeState::from_path(&change_dir);
    let manifest = load_effective()?;

    // Check apply.requires — all required artifacts must exist.
    let mut missing = Vec::new();
    for req in &manifest.apply.requires {
        let exists = match req.as_str() {
            "proposal" => state.proposal_exists,
            "specs" => state.specs_exist,
            "design" => state.design_exists,
            "tasks" => state.tasks_exists,
            "review" => state.review_exists,
            _ => false,
        };
        if !exists {
            missing.push(req.as_str());
        }
    }

    // Display progress.
    println!("Change: {change}");
    println!("Stage:  {}", state.stage_label());

    if let Some(ref p) = state.tasks_progress {
        println!("Tasks:  {}/{} complete", p.done, p.total);

        if p.total == 0 {
            println!("\nNo task checkboxes found in tasks.md.");
            println!("Add `- [ ] task description` lines to tasks.md to track progress.");
        } else if p.is_complete() {
            println!("\n✓ All tasks complete.");
        } else {
            println!("\nPending tasks: {}", p.total - p.done);
            // Show first few pending tasks from tasks.md.
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
    } else {
        println!("\nNo tasks.md found — create it to track progress.");
    }

    if !missing.is_empty() {
        println!(
            "\nMissing required artifacts for archive: {}",
            missing.join(", ")
        );
    }

    Ok(())
}
