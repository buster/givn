use anyhow::Result;
use crate::state::{list_changes, list_archived, ChangeState};

/// Show lifecycle state derived from the filesystem.
pub fn run(change: Option<&str>) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        anyhow::bail!("givn/ not found. Run `givn init` first.");
    }

    if let Some(id) = change {
        // Show state for a specific change.
        let change_dir = project_root.join("givn/changes").join(id);
        if !change_dir.exists() {
            // Check archived.
            let archived_dir = project_root.join("givn/archive").join(id);
            if archived_dir.exists() {
                println!("Change '{id}': archived");
                return Ok(());
            }
            anyhow::bail!("change '{}' not found (checked givn/changes/ and givn/archive/)", id);
        }
        let state = ChangeState::from_path(&change_dir);
        print_change_status(&state);
    } else {
        // Show all active changes.
        let changes = list_changes(&project_root);
        let archived = list_archived(&project_root);

        if changes.is_empty() && archived.is_empty() {
            println!("No changes found. Run `givn new <id>` to create one.");
            return Ok(());
        }

        if !changes.is_empty() {
            println!("Active changes:");
            for change in &changes {
                print_change_status(change);
            }
        }

        if !archived.is_empty() {
            println!("\nArchived:");
            for id in &archived {
                println!("  {id}  [archived]");
            }
        }
    }

    Ok(())
}

fn print_change_status(state: &ChangeState) {
    let stage = state.stage_label();

    // Task progress.
    let progress_str = if let Some(ref p) = state.tasks_progress {
        format!("  tasks: {}/{}", p.done, p.total)
    } else {
        String::new()
    };

    let review_str = if state.review_exists {
        if state.review_pass {
            "  review: PASS"
        } else {
            "  review: pending/FAIL"
        }
    } else {
        ""
    };

    println!("  {}  [{}]{}{}", state.id, stage, progress_str, review_str);

    // Show artifact checklist.
    println!("    proposal:  {}", if state.proposal_exists { "✓" } else { "·" });
    println!("    specs:     {}", if state.specs_exist { "✓" } else { "·" });
    println!("    design:    {}", if state.design_exists { "✓" } else { "·" });
    println!("    tasks:     {}", if state.tasks_exists { "✓" } else { "·" });
    println!("    review:    {}", if state.review_exists { "✓" } else { "·" });
}
