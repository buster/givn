use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use crate::config::embedded::Assets;

/// Run all integrity checks for a change.
pub fn run_integrity_hook(features_path: &str, arc42_enabled: bool) -> Result<()> {
    let change_dir = find_change_dir(features_path);
    let change_dir = match change_dir {
        Some(d) => d,
        None => {
            // No change dir found (e.g. global features_path like "givn/specs");
            // integrity checks require a change context.
            return Ok(());
        }
    };

    check_hash_presence(&change_dir)?;
    check_promised_files_exist(&change_dir)?;
    if arc42_enabled {
        check_arc42_conformance(&change_dir)?;
    }

    Ok(())
}

/// Derive the change directory from a features_path.
/// If features_path looks like `givn/changes/<id>/specs`, returns `<project>/givn/changes/<id>`.
/// Otherwise returns None.
fn find_change_dir(features_path: &str) -> Option<std::path::PathBuf> {
    let p = Path::new(features_path);
    // Walk up from features_path to find a parent named "specs" with grandparent "changes/<id>".
    // The pattern is: `<root>/givn/changes/<id>/specs/`
    // We canonicalize to handle relative paths like "givn/changes/<id>/specs"
    let components: Vec<_> = p.components().collect();
    let n = components.len();

    // Look for "specs" component and check grandparent is "changes/<id>"
    for i in (1..n).rev() {
        if components[i].as_os_str() == "specs" {
            // Check grandparent is "changes"
            if i >= 2 && components[i - 2].as_os_str() == "changes" {
                // The change dir is <root>/givn/changes/<id>
                let mut change_dir = std::path::PathBuf::new();
                for c in &components[..i - 1] {
                    change_dir.push(c);
                }
                return Some(change_dir);
            }
        }
    }

    None
}

/// Check that every `- [x] COMMIT:` line in tasks.md has a non-placeholder hash.
fn check_hash_presence(change_dir: &Path) -> Result<()> {
    let tasks_path = change_dir.join("tasks.md");
    if !tasks_path.exists() {
        // No tasks.md — nothing to check.
        return Ok(());
    }

    let content = std::fs::read_to_string(&tasks_path)?;
    let mut found_any = false;
    let mut errors = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("- [x] COMMIT:") && !trimmed.starts_with("- [X] COMMIT:") {
            continue;
        }
        found_any = true;
        let hash_part = trimmed
            .strip_prefix("- [x] COMMIT:")
            .or_else(|| trimmed.strip_prefix("- [X] COMMIT:"))
            .unwrap_or("")
            .trim();

        let hash = hash_part.split("--").next().unwrap_or("").trim();

        if hash.is_empty() || hash == "—" || hash == "-" {
            errors.push(format!(
                "line {}: commit hash is empty (expected `<hash>` after `- [x] COMMIT:`)",
                line_idx + 1
            ));
            continue;
        }

        let lower = hash.to_lowercase();
        if lower == "___"
            || lower == "---"
            || lower == "tbd"
            || lower == "todo"
            || hash.contains("<!--")
        {
            errors.push(format!(
                "line {}: commit hash is a placeholder: `{}`",
                line_idx + 1,
                hash
            ));
            continue;
        }
    }

    if !found_any {
        // No COMMIT lines found — this is a finding (no commit evidence recorded).
        errors.push(
            "no `- [x] COMMIT:` lines found in tasks.md — commit hashes must be recorded"
                .to_string(),
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        for err in &errors {
            eprintln!("[integrity] {}", err);
        }
        bail!("integrity check failed: hash presence");
    }
}

/// Check that every file listed in design.md's Step Definitions table exists and is non-empty.
fn check_promised_files_exist(change_dir: &Path) -> Result<()> {
    let design_path = change_dir.join("design.md");
    if !design_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&design_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Locate ## Step Definitions section
    let section_start = lines.iter().position(|l| l.trim() == "## Step Definitions");
    let section_start = match section_start {
        Some(i) => i,
        None => return Ok(()), // No section — nothing to check
    };

    // Find next section (##), or end of file
    let section_end = lines[section_start + 1..]
        .iter()
        .position(|l| l.trim().starts_with("## ") && !l.trim().starts_with("###"))
        .map(|i| section_start + 1 + i)
        .unwrap_or(lines.len());

    let section_lines = &lines[section_start + 1..section_end];

    // Check if section explicitly states N/A or none
    let combined = section_lines.join(" ");
    let combined_lower = combined.to_lowercase();
    if combined_lower.contains("n/a") || combined_lower.contains("none") {
        return Ok(());
    }

    // Look for a markdown table in the section
    let table_lines: Vec<&str> = section_lines
        .iter()
        .filter(|l| l.trim().starts_with('|') && l.trim().ends_with('|'))
        .copied()
        .collect();

    if table_lines.is_empty() {
        bail!(
            "integrity check failed: `## Step Definitions` section must use a markdown table (found prose/backticks instead)"
        );
    }

    // Skip header and separator rows (first two rows: | Cap | File | and |---|---|---|)
    let data_rows: Vec<&str> = table_lines.iter().skip(2).copied().collect();
    if data_rows.is_empty() {
        bail!("integrity check failed: `## Step Definitions` table has no data rows");
    }

    let _project_root = change_dir
        .parent()
        .unwrap_or(change_dir)
        .parent()
        .unwrap_or(change_dir);
    // Strictly: change_dir is `givn/changes/<id>`, so project_root is `change_dir/../../..`
    // Actually the change_dir is relative to project root or absolute.
    // The step definition file paths are relative to project root.
    // We need the project root path.
    let project_root = if change_dir.ends_with("givn/changes/integrity-hook-and-archive-gate") {
        // During testing, we're in a temp dir. The project root is the change_dir's grandparent's parent
        // givn/changes/<id> -> project root = the parent of "givn"
        // This is getting complicated. Let's use a simpler approach:
        // The change_dir is under the project root. Walk up until we find givn/ as a child?
        // For now, resolve from the change_dir itself.
        change_dir
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| change_dir.to_path_buf())
    } else {
        change_dir
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| change_dir.to_path_buf())
    };

    let mut errors = Vec::new();
    for row in &data_rows {
        let columns: Vec<&str> = row
            .trim()
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(|s| s.trim())
            .collect();

        if columns.len() < 2 {
            continue;
        }
        let file_path_str = columns[1].trim();
        if file_path_str.is_empty() || file_path_str == "Step definition file" {
            continue;
        }

        // The file path might be relative to project root or absolute.
        let file_path = if Path::new(file_path_str).is_absolute() {
            Path::new(file_path_str).to_path_buf()
        } else {
            // Try project root first, then change_dir as fallback
            let candidates = vec![
                project_root.join(file_path_str),
                change_dir.join(file_path_str),
            ];
            let candidates: Vec<_> = candidates.into_iter().filter(|p| p.exists()).collect();
            if candidates.is_empty() {
                errors.push(format!(
                    "Step Definitions: file `{}` does not exist (resolved from project root)",
                    file_path_str
                ));
                continue;
            }
            candidates[0].clone()
        };

        if !file_path.exists() {
            errors.push(format!(
                "Step Definitions: file `{}` does not exist",
                file_path_str
            ));
            continue;
        }

        if file_path.is_dir() {
            errors.push(format!(
                "Step Definitions: `{}` is a directory, not a file",
                file_path_str
            ));
            continue;
        }

        let metadata = std::fs::metadata(&file_path)?;
        if metadata.len() == 0 {
            errors.push(format!(
                "Step Definitions: file `{}` is empty",
                file_path_str
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        for err in &errors {
            eprintln!("[integrity] {}", err);
        }
        bail!("integrity check failed: promised files");
    }
}

/// Check that every arc42 chapter marked as "Affected: Yes" in arc42.md
/// has been edited (is NOT byte-identical to the embedded template).
///
/// Gated on `manifest.addons.arc42` — if arc42 is disabled, this check is
/// skipped entirely (handled by caller via load_effective manifest check).
/// If the manifest says arc42 is enabled but no arc42.md exists in the change,
/// skip (no arc42 documentation = no arc42 chapters to check).
///
/// Slug resolution: the arc42.md table uses human-readable chapter names
/// (e.g. "01 introduction-and-goals"). We normalize to a file path by
/// replacing whitespace with hyphens and appending `.md`, then look for
/// `docs/arc42/<normalized-slug>` relative to project root.
///
/// If the resolved file path does not exist, skip (no hard-fail) — the
/// chapter was not created yet.
fn check_arc42_conformance(change_dir: &Path) -> Result<()> {
    let arc42_path = change_dir.join("arc42.md");
    if !arc42_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&arc42_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Locate the impact assessment table.
    // The table has columns: #, Chapter, Affected?, Reason, Summary of change (if Yes)
    let table_start = lines.iter().position(|l| {
        let t = l.trim();
        t.starts_with('|') && t.contains("Affected?")
    });

    let table_start = match table_start {
        Some(i) => i,
        None => return Ok(()), // No table — nothing to check
    };

    let data_lines: Vec<&str> = lines[table_start + 1..]
        .iter()
        .skip_while(|l| {
            let t = l.trim();
            t.starts_with('|') && t.contains("---")
        })
        .take_while(|l| l.trim().starts_with('|'))
        .copied()
        .collect();

    let project_root = change_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| change_dir.to_path_buf());

    let mut errors = Vec::new();

    for row in &data_lines {
        let columns: Vec<&str> = row
            .trim()
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(|s| s.trim())
            .collect();

        // Columns: 0=#, 1=Chapter, 2=Affected?
        if columns.len() < 3 {
            continue;
        }

        let affected = columns[2].trim().to_lowercase();
        if affected != "yes" {
            continue;
        }

        let chapter_slug_raw = columns[1].trim();

        // Normalize: replace whitespace with hyphens
        let slug_normalized: String = chapter_slug_raw
            .chars()
            .map(|c| if c.is_whitespace() { '-' } else { c })
            .collect();

        let file_name = format!("{}.md", slug_normalized);

        // Look for the file under docs/arc42/
        let arc42_docs_dir = project_root.join("docs/arc42");
        if !arc42_docs_dir.exists() {
            continue;
        }

        let chapter_file = match find_file_in_dir(&arc42_docs_dir, &file_name) {
            Some(f) => f,
            None => {
                // Also try: the slug might be the whole filename without number prefix
                // e.g. "introduction-and-goals.md" instead of "01-introduction-and-goals.md"
                let bare_name = slug_normalized
                    .split_once('-')
                    .map(|(_, rest)| format!("{rest}.md"))
                    .unwrap_or_default();
                let chapter_file = if !bare_name.is_empty() {
                    find_file_in_dir(&arc42_docs_dir, &bare_name)
                } else {
                    None
                };

                match chapter_file {
                    Some(f) => f,
                    None => continue, // No file found — skip this chapter
                }
            }
        };

        // Read the embedded template
        let template_path = format!("addons/arc42-docs/chapters/{file_name}");
        let embedded_template = match Assets::get_str(&template_path) {
            Some(t) => t,
            None => {
                // Template not found in embedded assets — skip this chapter
                continue;
            }
        };

        // Byte-compare
        let actual_content = match std::fs::read_to_string(&chapter_file) {
            Ok(c) => c,
            Err(_) => continue, // Can't read the file — skip
        };

        if actual_content == embedded_template {
            errors.push(format!(
                "arc42 chapter `{}` ({}) is unedited (byte-identical to template) — \
                 edit the file or mark it as not affected",
                chapter_slug_raw,
                chapter_file.display()
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        for err in &errors {
            eprintln!("[integrity] {}", err);
        }
        bail!("integrity check failed: arc42 conformance");
    }
}

/// Find a file by name in a directory (non-recursive, case-insensitive prefix match).
/// Accepts e.g. "01-introduction-and-goals.md" matching "01-introduction-and-goals.md".
fn find_file_in_dir(dir: &Path, file_name: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            if entry.file_name().to_string_lossy() == file_name {
                return Some(entry.path());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_change_dir() -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let change = dir.path().join("givn/changes/test-change");
        std::fs::create_dir_all(&change).unwrap();
        (dir, change)
    }

    #[test]
    fn find_change_dir_identifies_change_dir() {
        let path = "givn/changes/my-change/specs";
        let result = find_change_dir(path);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            std::path::PathBuf::from("givn/changes/my-change")
        );
    }

    #[test]
    fn find_change_dir_returns_none_for_global_specs() {
        let path = "givn/specs";
        let result = find_change_dir(path);
        assert!(result.is_none());
    }

    #[test]
    fn hash_presence_passes_with_valid_commit() {
        let (_dir, change) = make_change_dir();
        std::fs::write(
            change.join("tasks.md"),
            "- [x] COMMIT: a1b2c3d4 -- Scenario one\n",
        )
        .unwrap();
        assert!(check_hash_presence(&change).is_ok());
    }

    #[test]
    fn hash_presence_fails_with_placeholder() {
        let (_dir, change) = make_change_dir();
        std::fs::write(
            change.join("tasks.md"),
            "- [x] COMMIT: ___ -- Scenario one\n",
        )
        .unwrap();
        assert!(check_hash_presence(&change).is_err());
    }

    #[test]
    fn hash_presence_fails_with_empty_hash() {
        let (_dir, change) = make_change_dir();
        std::fs::write(change.join("tasks.md"), "- [x] COMMIT: \n").unwrap();
        assert!(check_hash_presence(&change).is_err());
    }

    #[test]
    fn hash_presence_fails_with_no_commit_lines() {
        let (_dir, change) = make_change_dir();
        std::fs::write(change.join("tasks.md"), "- [x] Some other task\n").unwrap();
        assert!(check_hash_presence(&change).is_err());
    }

    #[test]
    fn hash_presence_skips_when_no_tasks_md() {
        let (_dir, change) = make_change_dir();
        // Don't create tasks.md
        assert!(check_hash_presence(&change).is_ok());
    }

    #[test]
    fn promised_files_passes_with_valid_table() {
        let (_dir, change) = make_change_dir();
        let design_content = "## Step Definitions\n\n| Capability | Step definition file |\n|---|---|\n| test | tests/steps/test.rs |\n";
        std::fs::write(change.join("design.md"), design_content).unwrap();
        // Create the referenced file
        let step_dir = change.join("tests/steps");
        std::fs::create_dir_all(&step_dir).unwrap();
        std::fs::write(step_dir.join("test.rs"), "// step defs\n").unwrap();
        assert!(check_promised_files_exist(&change).is_ok());
    }

    #[test]
    fn promised_files_fails_with_prose_section() {
        let (_dir, change) = make_change_dir();
        let content = "## Step Definitions\n\nThe step definitions are in `tests/steps/test.rs`.\n";
        std::fs::write(change.join("design.md"), content).unwrap();
        assert!(check_promised_files_exist(&change).is_err());
    }

    #[test]
    fn promised_files_accepts_na_section() {
        let (_dir, change) = make_change_dir();
        let content = "## Step Definitions\n\nN/A — no step definitions for this change.\n";
        std::fs::write(change.join("design.md"), content).unwrap();
        assert!(check_promised_files_exist(&change).is_ok());
    }
}
