/// Filesystem-derived lifecycle state (ADR-014, arc42 ch.8.6).
///
/// There is no state file. State is computed entirely from which files exist.
use std::path::Path;

/// The lifecycle state of a single change.
#[derive(Debug, Clone)]
pub struct ChangeState {
    pub id: String,
    pub proposal_exists: bool,
    pub specs_exist: bool,
    pub design_exists: bool,
    pub tasks_exists: bool,
    pub review_exists: bool,
    pub tasks_progress: Option<TasksProgress>,
    pub review_pass: bool,
}

/// Progress derived from tasks.md checkbox parsing.
#[derive(Debug, Clone)]
pub struct TasksProgress {
    pub total: usize,
    pub done: usize,
}

impl TasksProgress {
    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.done == self.total
    }
}

impl ChangeState {
    /// Derive state from the filesystem for a change under `givn/changes/<id>/`.
    pub fn from_path(change_dir: &Path) -> Self {
        let id = change_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let proposal_exists = change_dir.join("proposal.md").exists();
        let design_exists = change_dir.join("design.md").exists();
        let tasks_exists = change_dir.join("tasks.md").exists();
        let review_exists = change_dir.join("review.md").exists();

        // Check if any .feature files exist under specs/.
        let specs_exist = {
            let specs_dir = change_dir.join("specs");
            specs_dir.exists()
                && crate::gherkin::parse::collect_feature_files(&specs_dir).len() > 0
        };

        // Parse tasks progress.
        let tasks_progress = if tasks_exists {
            Some(parse_tasks_progress(&change_dir.join("tasks.md")))
        } else {
            None
        };

        // Detect review PASS marker.
        let review_pass = if review_exists {
            detect_review_pass(&change_dir.join("review.md"))
        } else {
            false
        };

        ChangeState {
            id,
            proposal_exists,
            specs_exist,
            design_exists,
            tasks_exists,
            review_exists,
            tasks_progress,
            review_pass,
        }
    }

    /// Returns a human-readable summary of the current stage.
    pub fn stage_label(&self) -> &'static str {
        if self.review_exists && self.review_pass {
            "ready-to-archive"
        } else if self.tasks_exists {
            if self.tasks_progress.as_ref().map(|p| p.is_complete()).unwrap_or(false) {
                "implementation-done"
            } else {
                "in-progress"
            }
        } else if self.design_exists {
            "design-done"
        } else if self.specs_exist {
            "specs-done"
        } else if self.proposal_exists {
            "proposal-done"
        } else {
            "empty"
        }
    }
}

/// Parse the number of `[x]` and `[ ]` checkboxes in a tasks.md file.
pub fn parse_tasks_progress(path: &Path) -> TasksProgress {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    let total = content.matches("[ ]").count() + content.matches("[x]").count();
    let done = content.matches("[x]").count();
    TasksProgress { total, done }
}

/// Detect if review.md contains the `REVIEW: PASS` marker.
pub fn detect_review_pass(path: &Path) -> bool {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    content.contains("REVIEW: PASS")
}

/// Find all active changes under `givn/changes/`.
pub fn list_changes(project_root: &Path) -> Vec<ChangeState> {
    let changes_dir = project_root.join("givn/changes");
    let mut changes = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&changes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                changes.push(ChangeState::from_path(&path));
            }
        }
    }
    changes.sort_by(|a, b| a.id.cmp(&b.id));
    changes
}

/// Find all archived changes under `givn/archive/`.
pub fn list_archived(project_root: &Path) -> Vec<String> {
    let archive_dir = project_root.join("givn/archive");
    let mut ids = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&archive_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    ids.push(name.to_string());
                }
            }
        }
    }
    ids.sort();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tasks_progress_counts_checkboxes() {
        let content = "- [x] done\n- [ ] pending\n- [x] also done\n";
        let dir = std::env::temp_dir().join("givn_state_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tasks.md");
        std::fs::write(&path, content).unwrap();

        let progress = parse_tasks_progress(&path);
        assert_eq!(progress.total, 3);
        assert_eq!(progress.done, 2);
        assert!(!progress.is_complete());
    }

    #[test]
    fn detect_review_pass_finds_marker() {
        let dir = std::env::temp_dir().join("givn_state_test2");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("review.md");

        std::fs::write(&path, "Some content\n\n**REVIEW: PASS**\n").unwrap();
        assert!(detect_review_pass(&path));

        std::fs::write(&path, "Some content\n\n**REVIEW: FAIL**\n").unwrap();
        assert!(!detect_review_pass(&path));
    }
}
