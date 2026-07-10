use crate::config::manifest::{Artifact, Manifest};
/// Filesystem-derived lifecycle state (ADR-014, arc42 ch.8.6).
///
/// There is no state file. State is computed entirely from which files exist,
/// driven by the effective manifest's artifact graph rather than hardcoded fields.
use std::path::Path;

/// The status of a single artifact within a change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactStatus {
    /// All `generates` paths exist (and for tasks: all checkboxes are checked;
    /// for review: REVIEW: PASS is present).
    Done,
    /// Some `generates` paths exist but the artifact is not fully complete.
    Partial,
    /// No `generates` paths exist.
    Pending,
    /// Intentionally not produced for this change (listed in `.givn-skip`).
    Skipped,
}

/// The state of a single artifact within a change, with context.
#[derive(Debug, Clone)]
pub struct ArtifactState {
    pub id: String,
    pub status: ArtifactStatus,
    pub optional: bool,
    pub gates_archive: bool,
    /// Task progress, if this artifact tracks checkboxes.
    pub tasks_progress: Option<TasksProgress>,
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

/// The full lifecycle state of a single change, manifest-aware.
#[derive(Debug, Clone)]
pub struct ChangeState {
    pub id: String,
    /// Artifacts in topological order with their current status.
    pub artifacts: Vec<ArtifactState>,
    pub verify_command: String,
    pub e2e_command: String,
}

impl ChangeState {
    /// Derive state by walking the manifest's artifact graph against the filesystem.
    pub fn from_manifest(change_dir: &Path, manifest: &Manifest) -> Self {
        let id = change_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let archive_requires: std::collections::HashSet<&str> =
            manifest.apply.requires.iter().map(|s| s.as_str()).collect();

        let ordered = topological_sort(&manifest.artifacts);

        let artifacts = ordered
            .iter()
            .map(|a| {
                let status = artifact_status(a, change_dir);
                let tasks_progress =
                    if a.id == "tasks" || a.generates.iter().any(|g| g.ends_with("tasks.md")) {
                        let tasks_path = change_dir.join("tasks.md");
                        if tasks_path.exists() {
                            Some(parse_tasks_progress(&tasks_path))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                ArtifactState {
                    id: a.id.clone(),
                    status,
                    optional: a.optional,
                    gates_archive: archive_requires.contains(a.id.as_str()),
                    tasks_progress,
                }
            })
            .collect();

        ChangeState {
            id,
            artifacts,
            verify_command: manifest.verify.command.clone(),
            e2e_command: manifest.verify.e2e_command.clone(),
        }
    }

    /// The first required artifact that is not yet Done or Skipped — the immediate next step.
    pub fn next_required(&self) -> Option<&ArtifactState> {
        self.artifacts.iter().find(|a| {
            !a.optional && a.status != ArtifactStatus::Done && a.status != ArtifactStatus::Skipped
        })
    }

    /// All optional artifacts that are still Pending.
    pub fn optional_pending(&self) -> Vec<&ArtifactState> {
        self.artifacts
            .iter()
            .filter(|a| a.optional && a.status == ArtifactStatus::Pending)
            .collect()
    }

    /// Whether all required artifacts are Done or Skipped (change is complete pending archive).
    pub fn all_required_done(&self) -> bool {
        self.artifacts
            .iter()
            .filter(|a| !a.optional)
            .all(|a| a.status == ArtifactStatus::Done || a.status == ArtifactStatus::Skipped)
    }

    /// Human-readable stage label derived from manifest-aware state.
    pub fn stage_label(&self) -> String {
        if self.all_required_done() {
            "ready-to-archive".to_string()
        } else if let Some(next) = self.next_required() {
            // Find the previous done artifact to describe current stage.
            let done_count = self
                .artifacts
                .iter()
                .filter(|a| !a.optional && a.status == ArtifactStatus::Done)
                .count();
            if done_count == 0 {
                "empty".to_string()
            } else if next.id == "tasks" {
                // Check if tasks is partial (in-progress).
                let tasks = self.artifacts.iter().find(|a| a.id == "tasks");
                if tasks
                    .map(|t| t.status == ArtifactStatus::Partial)
                    .unwrap_or(false)
                {
                    "in-progress".to_string()
                } else {
                    format!("{}-done", self.last_done_id())
                }
            } else {
                format!("{}-done", self.last_done_id())
            }
        } else {
            "empty".to_string()
        }
    }

    fn last_done_id(&self) -> &str {
        self.artifacts
            .iter()
            .filter(|a| !a.optional && a.status == ArtifactStatus::Done)
            .last()
            .map(|a| a.id.as_str())
            .unwrap_or("unknown")
    }
}

/// Determine whether an artifact's output exists in the change directory.
fn artifact_status(artifact: &Artifact, change_dir: &Path) -> ArtifactStatus {
    // Check the per-change skip file first. If the artifact id is listed,
    // the artifact was intentionally not produced for this change.
    if is_skipped(&artifact.id, change_dir) {
        return ArtifactStatus::Skipped;
    }

    if artifact.generates.is_empty() {
        return ArtifactStatus::Pending;
    }

    let mut any_exist = false;
    let mut all_exist = true;

    for pattern in &artifact.generates {
        let exists = if pattern.contains('*') {
            // Glob: check if any matching file exists.
            let base = pattern.split('*').next().unwrap_or("");
            let base_dir = change_dir.join(base.trim_end_matches('/'));
            crate::gherkin::parse::collect_feature_files(&base_dir).len() > 0
        } else {
            change_dir.join(pattern).exists()
        };

        if exists {
            any_exist = true;
        } else {
            all_exist = false;
        }
    }

    if !any_exist {
        return ArtifactStatus::Pending;
    }

    // For tasks: check if all checkboxes are done.
    if artifact.generates.iter().any(|g| g.ends_with("tasks.md")) {
        let tasks_path = change_dir.join("tasks.md");
        if tasks_path.exists() {
            let progress = parse_tasks_progress(&tasks_path);
            return if progress.is_complete() {
                ArtifactStatus::Done
            } else {
                ArtifactStatus::Partial
            };
        }
    }

    // For review: check for REVIEW: PASS marker.
    if artifact.generates.iter().any(|g| g.ends_with("review.md")) {
        let review_path = change_dir.join("review.md");
        if review_path.exists() {
            return if detect_review_pass(&review_path) {
                ArtifactStatus::Done
            } else {
                ArtifactStatus::Partial
            };
        }
    }

    // For design-review: check for DESIGN-REVIEW: PASS marker.
    if artifact
        .generates
        .iter()
        .any(|g| g.ends_with("design-review.md"))
    {
        let dr_path = change_dir.join("design-review.md");
        if dr_path.exists() {
            return if detect_design_review_pass(&dr_path) {
                ArtifactStatus::Done
            } else {
                ArtifactStatus::Partial
            };
        }
    }

    // For arc42-docs: check for STATUS: DONE marker in the change-level arc42.md.
    if artifact.generates.iter().any(|g| g.ends_with("arc42.md")) {
        let arc42_path = change_dir.join("arc42.md");
        if arc42_path.exists() {
            return if detect_arc42_done(&arc42_path) {
                ArtifactStatus::Done
            } else {
                ArtifactStatus::Partial
            };
        }
    }

    if all_exist {
        ArtifactStatus::Done
    } else {
        ArtifactStatus::Partial
    }
}

/// Topological sort of artifacts by their `requires` graph (Kahn's algorithm).
pub fn topological_sort(artifacts: &[Artifact]) -> Vec<&Artifact> {
    use std::collections::{HashMap, VecDeque};

    let index: HashMap<&str, usize> = artifacts
        .iter()
        .enumerate()
        .map(|(i, a)| (a.id.as_str(), i))
        .collect();

    let mut in_degree = vec![0usize; artifacts.len()];
    let mut dependents: Vec<Vec<usize>> = vec![vec![]; artifacts.len()];

    for (i, a) in artifacts.iter().enumerate() {
        for req in &a.requires {
            if let Some(&j) = index.get(req.as_str()) {
                dependents[j].push(i);
                in_degree[i] += 1;
            }
        }
    }

    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, d)| **d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut result = Vec::new();
    while let Some(i) = queue.pop_front() {
        result.push(&artifacts[i]);
        for &j in &dependents[i] {
            in_degree[j] -= 1;
            if in_degree[j] == 0 {
                queue.push_back(j);
            }
        }
    }

    result
}

/// Parse the number of `[x]` and `[ ]` checkboxes in a tasks.md file.
pub fn parse_tasks_progress(path: &Path) -> TasksProgress {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    let total = content.matches("[ ]").count() + content.matches("[x]").count();
    let done = content.matches("[x]").count();
    TasksProgress { total, done }
}

/// Detect if design-review.md contains the `DESIGN-REVIEW: PASS` marker.
pub fn detect_design_review_pass(path: &Path) -> bool {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    content.contains("DESIGN-REVIEW: PASS")
}

/// Detect if review.md contains the `REVIEW: PASS` marker.
pub fn detect_review_pass(path: &Path) -> bool {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    content.contains("REVIEW: PASS")
}

/// Detect if arc42.md (the change-level marker) contains `STATUS: DONE`.
pub fn detect_arc42_done(path: &Path) -> bool {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    content.contains("STATUS: DONE")
}

/// Per-change skip file name (one artifact-id per line).
pub const SKIP_FILE: &str = ".givn-skip";

/// Check whether an artifact id is listed in the change's `.givn-skip` file.
pub fn is_skipped(artifact_id: &str, change_dir: &Path) -> bool {
    let skip_path = change_dir.join(SKIP_FILE);
    if let Ok(content) = std::fs::read_to_string(&skip_path) {
        content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .any(|id| id == artifact_id)
    } else {
        false
    }
}

/// Read the list of skipped artifact ids for a change.
#[allow(dead_code)]
pub fn read_skip_list(change_dir: &Path) -> Vec<String> {
    let skip_path = change_dir.join(SKIP_FILE);
    std::fs::read_to_string(&skip_path)
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect()
}

/// Find all active changes under `givn/changes/`, manifest-aware.
pub fn list_changes(project_root: &Path, manifest: &Manifest) -> Vec<ChangeState> {
    let changes_dir = project_root.join("givn/changes");
    let mut changes = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&changes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                changes.push(ChangeState::from_manifest(&path, manifest));
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
    use crate::config::manifest::Manifest;

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

    #[test]
    fn detect_design_review_pass_finds_marker() {
        let dir = std::env::temp_dir().join("givn_state_dr_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("design-review.md");

        std::fs::write(&path, "Some content\n\n**DESIGN-REVIEW: PASS**\n").unwrap();
        assert!(detect_design_review_pass(&path));

        std::fs::write(&path, "Some content\n\n**DESIGN-REVIEW: FAIL**\n").unwrap();
        assert!(!detect_design_review_pass(&path));
    }

    #[test]
    fn topological_sort_respects_requires() {
        let manifest = Manifest::load_embedded().unwrap();
        let sorted = topological_sort(&manifest.artifacts);
        let ids: Vec<&str> = sorted.iter().map(|a| a.id.as_str()).collect();
        let pos = |id: &str| ids.iter().position(|&x| x == id).unwrap();
        assert!(pos("proposal") < pos("specs"));
        assert!(pos("specs") < pos("design"));
        assert!(pos("design") < pos("design-review"));
        assert!(pos("design-review") < pos("tasks"));
        assert!(pos("tasks") < pos("review"));
    }

    #[test]
    fn next_required_returns_first_pending() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_state_test3");
        std::fs::create_dir_all(&dir).unwrap();
        // Write only proposal.md.
        std::fs::write(dir.join("proposal.md"), "# Proposal").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        let next = state.next_required().unwrap();
        assert_eq!(next.id, "specs");
    }

    #[test]
    fn detect_arc42_done_finds_marker() {
        let dir = std::env::temp_dir().join("givn_state_arc42_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("arc42.md");

        std::fs::write(&path, "## Status\n\nSTATUS: DONE\n").unwrap();
        assert!(detect_arc42_done(&path));

        std::fs::write(&path, "## Status\n\nSTATUS: PARTIAL\n").unwrap();
        assert!(!detect_arc42_done(&path));
    }

    #[test]
    fn arc42_addon_artifact_is_required_and_requires_design() {
        use crate::config::overlay::apply;
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true").unwrap();
        let arc42 = result
            .artifacts
            .iter()
            .find(|a| a.id == "arc42-docs")
            .unwrap();
        assert!(!arc42.optional, "arc42-docs must be required (not optional)");
        assert!(
            arc42.requires.contains(&"design".to_string()),
            "arc42-docs must require design"
        );
        assert_eq!(arc42.generates, vec!["arc42.md"]);
    }

    #[test]
    fn arc42_addon_not_in_apply_requires_by_default() {
        use crate::config::overlay::apply;
        let base = Manifest::load_embedded().unwrap();
        let result = apply(base, "addons:\n  arc42: true").unwrap();
        assert!(
            !result.apply.requires.contains(&"arc42-docs".to_string()),
            "arc42-docs must not gate archive by default"
        );
    }

    #[test]
    fn is_skipped_reads_skip_file() {
        let dir = std::env::temp_dir().join("givn_state_skip_test");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(SKIP_FILE), "proposal\ndesign\n# comment\n\n").unwrap();

        assert!(is_skipped("proposal", &dir));
        assert!(is_skipped("design", &dir));
        assert!(!is_skipped("tasks", &dir));
        assert!(!is_skipped("review", &dir));
    }

    #[test]
    fn is_skipped_returns_false_when_no_skip_file() {
        let dir = std::env::temp_dir().join("givn_state_skip_test2");
        std::fs::create_dir_all(&dir).unwrap();
        // No .givn-skip file.
        assert!(!is_skipped("proposal", &dir));
    }

    #[test]
    fn skipped_artifact_is_not_next_required() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_state_skip_next_test");
        std::fs::create_dir_all(&dir).unwrap();
        // Skip proposal — no proposal.md, no specs, no design.
        std::fs::write(dir.join(SKIP_FILE), "proposal\n").unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        // proposal is skipped, so next_required should skip it and return specs.
        let next = state.next_required().unwrap();
        assert_eq!(next.id, "specs");
        // proposal itself should be Skipped.
        let proposal = state.artifacts.iter().find(|a| a.id == "proposal").unwrap();
        assert_eq!(proposal.status, ArtifactStatus::Skipped);
    }

    #[test]
    fn all_required_done_treats_skipped_as_satisfied() {
        let manifest = Manifest::load_embedded().unwrap();
        let dir = std::env::temp_dir().join("givn_state_skip_all_test");
        std::fs::create_dir_all(&dir).unwrap();
        // Skip everything except tasks and review.
        std::fs::write(dir.join(SKIP_FILE), "proposal\ndesign\ndesign-review\n").unwrap();
        // Write tasks.md (complete) and review.md (PASS).
        std::fs::write(dir.join("tasks.md"), "- [x] done\n").unwrap();
        std::fs::write(dir.join("review.md"), "REVIEW: PASS\n").unwrap();
        // Also need specs to exist for the specs artifact to be Done.
        std::fs::create_dir_all(dir.join("specs/cap")).unwrap();
        std::fs::write(
            dir.join("specs/cap/cap.feature"),
            "@givn.delta @cap\nFeature: cap\n  Scenario: s\n    Given x\n",
        )
        .unwrap();

        let state = ChangeState::from_manifest(&dir, &manifest);
        assert!(
            state.all_required_done(),
            "all required should be done or skipped"
        );
    }
}
