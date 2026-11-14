use crate::overlap::analysis::Finding;

/// The parsed status of a change seed.
enum SeedStatus {
    /// A known status that needs no change lookup.
    Valid,
    /// No `Status:` line, an unrecognised value, or an empty reference.
    Missing,
    /// `proposed by <change-id>` — the referenced change must be resolved.
    Proposed(String),
}

/// Advisory seed-status hygiene for `givn/ideation/*/changes/*.md`.
///
/// Tolerant by design: only the first `Status:` line is read; prose before and
/// after it is ignored. Findings are advisory and never affect the exit code.
pub(super) fn check_seed_status(project_root: &std::path::Path) -> Vec<Finding> {
    let ideation_root = project_root.join("givn/ideation");
    if !ideation_root.is_dir() {
        return Vec::new();
    }

    let mut topic_dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&ideation_root)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect()
        })
        .unwrap_or_default();
    topic_dirs.sort();

    let mut seeds: Vec<std::path::PathBuf> = Vec::new();
    for topic in topic_dirs {
        let changes_dir = topic.join("changes");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&changes_dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.is_file()
                            && path
                                .extension()
                                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
                    })
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        seeds.extend(files);
    }

    let mut findings = Vec::new();
    for seed in seeds {
        let content = std::fs::read_to_string(&seed).unwrap_or_default();
        let message = match parse_seed_status(&content) {
            SeedStatus::Valid => None,
            SeedStatus::Missing => Some(
                "change seed has no valid status line (expected ready, proposed by <change-id>, \
                 implemented in <change-id>, superseded by <reference>, or withdrawn)"
                    .to_string(),
            ),
            SeedStatus::Proposed(id) => {
                if project_root.join("givn/changes").join(&id).is_dir() {
                    None
                } else if project_root.join("givn/archive").join(&id).is_dir() {
                    Some(format!(
                        "change seed is proposed by '{id}', but that change is already archived"
                    ))
                } else {
                    Some(format!(
                        "change seed is proposed by '{id}', but no such change exists"
                    ))
                }
            }
        };
        if let Some(message) = message {
            findings.push(Finding {
                kind: "seed_status".to_string(),
                level: "warning".to_string(),
                files: vec![seed.display().to_string()],
                titles: vec![message],
                contradiction: false,
            });
        }
    }
    findings
}

/// Advisory persona-reference hygiene for active change use-case documents
/// and ideation use cases. A `## Personas` entry resolves when the ID names a
/// file under `givn/personas/` or any topic's `personas/`. Matching is
/// ID-only; the finding never affects the exit code.
pub(super) fn check_persona_references(
    project_root: &std::path::Path,
    change: Option<&str>,
) -> Vec<Finding> {
    let mut documents: Vec<std::path::PathBuf> = Vec::new();

    let changes_root = project_root.join("givn/changes");
    let change_ids: Vec<String> = match change {
        Some(id) => vec![id.to_string()],
        None => read_dir_names(&changes_root),
    };
    for id in change_ids {
        let specs_dir = changes_root.join(&id).join("specs");
        for usecase in read_dir_names(&specs_dir) {
            let path = specs_dir.join(&usecase).join("usecase.md");
            if path.is_file() {
                documents.push(path);
            }
        }
    }

    if change.is_none() {
        let ideation_root = project_root.join("givn/ideation");
        for topic in read_dir_names(&ideation_root) {
            let use_cases = ideation_root.join(&topic).join("use-cases");
            for entry in read_dir_names(&use_cases) {
                let path = use_cases.join(format!("{entry}.md"));
                if path.is_file() {
                    documents.push(path);
                }
            }
        }
    }

    let project_personas = project_root.join("givn/personas");
    let ideation_root = project_root.join("givn/ideation");

    let mut findings = Vec::new();
    for document in documents {
        let content = std::fs::read_to_string(&document).unwrap_or_default();
        for persona in persona_entries(&content) {
            if resolves_persona(&project_personas, &ideation_root, &persona) {
                continue;
            }
            findings.push(Finding {
                kind: "persona_ref".to_string(),
                level: "warning".to_string(),
                files: vec![document.display().to_string()],
                titles: vec![format!(
                    "persona reference '{persona}' matches no project or topic persona file"
                )],
                contradiction: false,
            });
        }
    }
    findings
}

/// Names of the immediate subdirectories of `dir`, sorted; empty when absent.
fn read_dir_names(dir: &std::path::Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().is_dir())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The `## Personas` bullet entries of a use-case document, tolerantly.
fn persona_entries(content: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut in_section = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            in_section = trimmed == "## Personas";
            continue;
        }
        if !in_section {
            continue;
        }
        let Some(value) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        else {
            continue;
        };
        let value = value.trim();
        if !value.is_empty() && !value.eq_ignore_ascii_case("none") {
            entries.push(value.to_string());
        }
    }
    entries
}

fn resolves_persona(
    project_personas: &std::path::Path,
    ideation_root: &std::path::Path,
    persona: &str,
) -> bool {
    if project_personas.join(format!("{persona}.md")).is_file() {
        return true;
    }
    for topic in read_dir_names(ideation_root) {
        if ideation_root
            .join(&topic)
            .join("personas")
            .join(format!("{persona}.md"))
            .is_file()
        {
            return true;
        }
    }
    false
}

/// Read the first `Status:` line of a seed and classify its value.
fn parse_seed_status(content: &str) -> SeedStatus {
    let Some(value) = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("Status:").map(str::trim))
    else {
        return SeedStatus::Missing;
    };

    if value == "ready"
        || value.starts_with("ready ")
        || value.starts_with("withdrawn")
        || value.starts_with("implemented in ")
        || value.starts_with("superseded by ")
    {
        return SeedStatus::Valid;
    }
    if let Some(id) = value
        .strip_prefix("proposed by ")
        .map(str::trim)
        .filter(|id| !id.is_empty())
    {
        return SeedStatus::Proposed(id.to_string());
    }
    SeedStatus::Missing
}
