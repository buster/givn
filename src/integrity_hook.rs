use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use crate::config::embedded::Assets;
use crate::config::overlay::load_effective;

/// Run all integrity checks for a change.
pub fn run_integrity_hook(
    features_path: &str,
    arc42_enabled: bool,
    coverage_enabled: bool,
    readme_enabled: bool,
) -> Result<()> {
    let change_dir = crate::state::find_change_dir(features_path);
    let change_dir = match change_dir {
        Some(d) => d,
        None => {
            return Ok(());
        }
    };

    check_hash_presence(&change_dir)?;
    check_promised_files_exist(&change_dir)?;
    check_visual_review(&change_dir)?;
    if arc42_enabled {
        check_arc42_conformance(&change_dir)?;
    }
    if coverage_enabled {
        check_coverage_config()?;
    }
    if readme_enabled {
        check_readme_impact(&change_dir)?;
    }

    Ok(())
}

/// Check that `review.md` records a machine-checkable README impact decision.
///
/// Gated on `manifest.addons.readme` — with the addon off, the decision is not
/// required. A valid decision is exactly one line per occurrence:
/// `README-IMPACT: none` or `README-IMPACT: updated - <sections>`.
fn check_readme_impact(change_dir: &Path) -> Result<()> {
    let review_path = change_dir.join("review.md");
    if !review_path.exists() {
        // No review.md — nothing to check (matching the other integrity checks).
        return Ok(());
    }
    let content = std::fs::read_to_string(&review_path)?;
    let mut found_any = false;
    for line in content.lines() {
        let Some(value) = line
            .trim_start()
            .strip_prefix("README-IMPACT:")
            .map(str::trim)
        else {
            continue;
        };
        found_any = true;
        let valid = value == "none"
            || (value.starts_with("updated - ")
                && value
                    .strip_prefix("updated - ")
                    .is_some_and(|sections| !sections.trim().is_empty()));
        if !valid {
            bail!(
                "README-IMPACT value '{value}' is not a valid README impact decision — \
                 use 'README-IMPACT: none' or 'README-IMPACT: updated - <sections>'"
            );
        }
    }
    if !found_any {
        bail!(
            "review.md is missing the README impact decision — add a line \
             'README-IMPACT: none' or 'README-IMPACT: updated - <sections>'"
        );
    }
    Ok(())
}

/// A parsed `## Visual Design Contract` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualContract {
    pub interface: VisualInterface,
    pub viewports: Vec<String>,
    pub states: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualInterface {
    Web,
    Terminal,
    None,
}

/// Parse the `## Visual Design Contract` section from a design document.
///
/// Fenced code blocks are removed before scanning so an example contract
/// inside a fence never triggers the gate. Returns `None` when no section
/// exists. The interface defaults to `None` when the section carries no
/// recognized `- Interface:` line.
pub fn parse_visual_contract(design: &str) -> Option<VisualContract> {
    let without_fences = strip_fenced_blocks(design);
    let lines: Vec<&str> = without_fences.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == "## Visual Design Contract")?;
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.trim_start().starts_with("## "))
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());

    let mut interface = VisualInterface::None;
    let mut viewports = Vec::new();
    let mut states = Vec::new();
    for line in &lines[start + 1..end] {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("- Interface:") {
            interface = match value.trim().to_lowercase().as_str() {
                "web" => VisualInterface::Web,
                "terminal" => VisualInterface::Terminal,
                _ => VisualInterface::None,
            };
        } else if let Some(value) = trimmed.strip_prefix("- Viewports:") {
            viewports = value
                .split(',')
                .map(|entry| {
                    entry
                        .split('(')
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_lowercase()
                })
                .filter(|name| !name.is_empty())
                .collect();
        } else if let Some(value) = trimmed.strip_prefix("- States:") {
            states = value
                .split(',')
                .map(|entry| entry.trim().to_lowercase())
                .filter(|name| !name.is_empty())
                .collect();
        }
    }
    Some(VisualContract {
        interface,
        viewports,
        states,
    })
}

/// Deterministic scenario-title slug: lowercase ASCII; every run of
/// non-alphanumeric characters becomes one `-`; leading/trailing `-` removed.
/// "Consumer opens the dashboard" -> "consumer-opens-the-dashboard".
pub fn scenario_slug(title: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for character in title.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(character);
        } else {
            pending_dash = true;
        }
    }
    slug
}

/// Remove fenced code blocks (``` or ~~~) so examples are never parsed.
fn strip_fenced_blocks(text: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Check the visual review verdict for a change whose design declares a
/// visual interface. A change without a contract section is exempt.
fn check_visual_review(change_dir: &Path) -> Result<()> {
    let design_path = change_dir.join("design.md");
    if !design_path.exists() {
        return Ok(());
    }
    let design = std::fs::read_to_string(&design_path)?;
    let Some(contract) = parse_visual_contract(&design) else {
        return Ok(());
    };
    if contract.interface == VisualInterface::None {
        return Ok(());
    }
    if contract.interface == VisualInterface::Web && contract.viewports.is_empty() {
        bail!(
            "visual design contract declares a web interface without viewports — \
             name at least one viewport (default: mobile, desktop)"
        );
    }

    let review_path = change_dir.join("review.md");
    if !review_path.exists() {
        bail!(
            "review.md is missing the visual verdict — add \
             'VISUAL-REVIEW: PASS' or 'VISUAL-REVIEW: UNVERIFIED'"
        );
    }
    let review = std::fs::read_to_string(&review_path)?;
    match visual_verdict(&review) {
        VisualVerdict::Pass => {}
        VisualVerdict::Ambiguous => bail!(
            "review.md carries both VISUAL-REVIEW: PASS and VISUAL-REVIEW: UNVERIFIED — \
             record exactly one visual verdict"
        ),
        VisualVerdict::Unverified => bail!(
            "visual review is VISUAL-REVIEW: UNVERIFIED — a human sign-off must resolve it \
             to VISUAL-REVIEW: PASS before review can pass"
        ),
        VisualVerdict::Missing => bail!(
            "review.md is missing the visual verdict — add \
             'VISUAL-REVIEW: PASS' for a declared visual interface"
        ),
    }

    check_persona_disposition(&review)?;
    check_visual_evidence(change_dir, &contract)
}

/// A declared visual interface requires a `PERSONAS:` line naming at least
/// one confirmed Persona. Fenced examples in the review template do not
/// satisfy the check.
fn check_persona_disposition(review: &str) -> Result<()> {
    let stripped = strip_fenced_blocks(review);
    let Some(value) = persona_disposition(&stripped) else {
        bail!(
            "visual review is missing the persona disposition — add \
             'PERSONAS: <confirmed persona>' to review.md"
        );
    };
    if value.is_empty() || value.eq_ignore_ascii_case("none") {
        bail!(
            "a declared visual interface requires at least one confirmed Persona — \
             'PERSONAS: {value}' is not a confirmed persona"
        );
    }
    Ok(())
}

/// Extract the value of the first `PERSONAS:` line.
fn persona_disposition(review: &str) -> Option<String> {
    review.lines().find_map(|line| {
        line.trim()
            .strip_prefix("PERSONAS:")
            .map(|value| value.trim().to_string())
    })
}

/// Collect every delta `@e2e` scenario title that still needs visual evidence.
pub fn collect_e2e_scenario_titles(specs_dir: &Path) -> Result<Vec<String>> {
    let mut titles = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(specs_dir) {
        let feature = gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default())
            .map_err(|error| anyhow::anyhow!("failed to parse {}: {error}", path.display()))?;
        for scenario in &feature.scenarios {
            let is_e2e = scenario.tags.iter().any(|tag| tag == "e2e");
            let is_removed = scenario.tags.iter().any(|tag| tag == "givn.removed");
            if is_e2e && !is_removed {
                titles.push(scenario.name.clone());
            }
        }
    }
    Ok(titles)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisualVerdict {
    Pass,
    Unverified,
    Ambiguous,
    Missing,
}

fn visual_verdict(review: &str) -> VisualVerdict {
    match (
        review.contains("VISUAL-REVIEW: PASS"),
        review.contains("VISUAL-REVIEW: UNVERIFIED"),
    ) {
        (true, true) => VisualVerdict::Ambiguous,
        (false, true) => VisualVerdict::Unverified,
        (true, false) => VisualVerdict::Pass,
        (false, false) => VisualVerdict::Missing,
    }
}

/// A zero-byte evidence file counts as missing.
fn is_non_empty_file(path: &Path) -> bool {
    path.metadata().map(|meta| meta.is_file() && meta.len() > 0).unwrap_or(false)
}

/// Check that every delta `@e2e` scenario has the declared evidence.
fn check_visual_evidence(change_dir: &Path, contract: &VisualContract) -> Result<()> {
    let titles = collect_e2e_scenario_titles(&change_dir.join("specs"))?;
    let evidence_root = change_dir.join("evidence/visual");
    let mut errors = Vec::new();
    for title in &titles {
        let slug = scenario_slug(title);
        match contract.interface {
            VisualInterface::Web => {
                for viewport in &contract.viewports {
                    let path = evidence_root.join(&slug).join(format!("{viewport}.png"));
                    if !is_non_empty_file(&path) {
                        errors.push(format!(
                            "visual evidence missing for scenario '{title}' at viewport \
                             '{viewport}' (expected {})",
                            path.display()
                        ));
                    }
                }
                if !titles.is_empty() {
                    for state in &contract.states {
                        for viewport in &contract.viewports {
                            let found = titles.iter().any(|candidate| {
                                is_non_empty_file(
                                    &evidence_root
                                        .join(scenario_slug(candidate))
                                        .join(format!("{viewport}--{state}.png")),
                                )
                            });
                            if !found {
                                errors.push(format!(
                                    "visual state evidence missing for state '{state}' at viewport \
                                     '{viewport}' (expected a non-empty {viewport}--{state}.png \
                                     under evidence/visual/<scenario-slug>/)"
                                ));
                            }
                        }
                    }
                }
            }
            VisualInterface::Terminal => {
                let path = evidence_root.join(&slug).join("transcript.txt");
                if !is_non_empty_file(&path) {
                    errors.push(format!(
                        "visual evidence missing for scenario '{title}' (expected {})",
                        path.display()
                    ));
                }
            }
            VisualInterface::None => {}
        }
    }
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("[integrity] {error}");
        }
        bail!("integrity check failed: visual evidence");
    }
    Ok(())
}

/// Check that coverage commands are configured when the addon is enabled.
fn check_coverage_config() -> Result<()> {
    let manifest = load_effective()?;
    let cov = &manifest.coverage;
    let sentinel = "givn missing-coverage";
    let legacy_configured = cov.non_e2e_command.is_some() && cov.e2e_command.is_some();
    if (cov.measure_command == sentinel || cov.merge_command == sentinel) && !legacy_configured {
        anyhow::bail!(
            "coverage addon is enabled but coverage commands are not configured. \
             Set coverage.measure_command and coverage.merge_command in givn/commands.yaml. \
             The measurement command must create both source reports and the merge command \
             must create the configured output report. Legacy projects may temporarily \
             keep both coverage.non_e2e_command and coverage.e2e_command while migrating. \
             Or disable the addon with: givn addons disable coverage"
        );
    }
    Ok(())
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
    let mut seen_hashes: std::collections::HashSet<String> = std::collections::HashSet::new();

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

        let hash = hash_part
            .split("--")
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('`');

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
            || lower == "xxx"
            || lower.chars().all(|c| c == 'x')
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

        // Check for duplicate hashes — each scenario requires its own atomic commit.
        if !seen_hashes.insert(hash.to_string()) {
            errors.push(format!(
                "line {}: commit hash `{}` is also used by another task — \
                 each scenario needs its own atomic commit",
                line_idx + 1,
                hash
            ));
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
    // Match standalone "n/a" or "none" as a distinct token (not inside a filename).
    if combined_lower.contains("n/a")
        || combined_lower
            .split(|c: char| c.is_whitespace() || c == ',' || c == '\n')
            .any(|w| w.trim_matches(|c: char| !c.is_alphanumeric()) == "none")
    {
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

    // change_dir is `<project_root>/givn/changes/<id>`, so project_root is 3 levels up.
    // For a relative path like `givn/changes/<id>`: 3 parent() calls yields `` (current dir).
    // For an absolute path: 3 parent() calls yields the absolute project root.
    let project_root = change_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| change_dir.to_path_buf());

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
        let file_path_str = columns[1].trim().trim_matches('`').trim();
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
        if entry.file_type().map(|t| t.is_file()).unwrap_or(false)
            && entry.file_name().to_string_lossy() == file_name
        {
            return Some(entry.path());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::find_change_dir;
    use tempfile::TempDir;

    fn make_change_dir() -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let change = dir.path().join("givn/changes/test-change");
        std::fs::create_dir_all(&change).unwrap();
        (dir, change)
    }

    #[test]
    fn fenced_contract_example_is_ignored() {
        let design = "# Design\n\n```\n## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780)\n```\n";
        assert_eq!(parse_visual_contract(design), None);
    }

    #[test]
    fn fenced_contract_example_with_states_is_ignored() {
        let design = "# Design\n\n```\n## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780)\n- States: empty\n```\n";
        assert_eq!(parse_visual_contract(design), None);
    }

    #[test]
    fn web_contract_parses_interface_and_viewports() {
        let design = "## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780), desktop (1920x1080)\n\nprose\n";
        let contract = parse_visual_contract(design).unwrap();
        assert_eq!(contract.interface, VisualInterface::Web);
        assert_eq!(contract.viewports, vec!["mobile", "desktop"]);
        assert!(contract.states.is_empty());
    }

    #[test]
    fn states_are_parsed_and_optional() {
        let with_states = "## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780)\n- States: empty, loading, error\n";
        let contract = parse_visual_contract(with_states).unwrap();
        assert_eq!(contract.states, vec!["empty", "loading", "error"]);

        let empty_line = "## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780)\n- States:\n";
        assert!(parse_visual_contract(empty_line).unwrap().states.is_empty());
    }

    fn write_web_contract_with_states(change: &std::path::Path, states: &str) {
        let design = format!(
            "## Visual Design Contract\n\n- Interface: web\n- Viewports: mobile (360x780), desktop (1920x1080)\n- States: {states}\n"
        );
        std::fs::write(change.join("design.md"), design).unwrap();
    }

    fn write_delta_e2e(change: &std::path::Path, title: &str) {
        let specs = change.join("specs/fixture");
        std::fs::create_dir_all(&specs).unwrap();
        let feature = format!(
            "@givn.delta @fixture\n\nFeature: Fixture\n\n  @givn.added @e2e\n  Scenario: {title}\n    Given a\n    When b\n    Then c\n"
        );
        std::fs::write(specs.join("fixture.feature"), feature).unwrap();
    }

    fn write_evidence(change: &std::path::Path, title: &str, file: &str) {
        let path = change
            .join("evidence/visual")
            .join(scenario_slug(title))
            .join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"bytes").unwrap();
    }

    #[test]
    fn declared_states_require_evidence_per_viewport() {
        let (_dir, change) = make_change_dir();
        write_web_contract_with_states(&change, "empty, loading");
        write_delta_e2e(&change, "Consumer opens the dashboard");
        write_evidence(&change, "Consumer opens the dashboard", "mobile.png");
        write_evidence(&change, "Consumer opens the dashboard", "desktop.png");
        let design = std::fs::read_to_string(change.join("design.md")).unwrap();
        let contract = parse_visual_contract(&design).unwrap();
        assert!(check_visual_evidence(&change, &contract).is_err());

        write_evidence(&change, "Consumer opens the dashboard", "mobile--empty.png");
        write_evidence(&change, "Consumer opens the dashboard", "desktop--empty.png");
        write_evidence(&change, "Consumer opens the dashboard", "mobile--loading.png");
        write_evidence(&change, "Consumer opens the dashboard", "desktop--loading.png");
        assert!(check_visual_evidence(&change, &contract).is_ok());
    }

    #[test]
    fn empty_state_evidence_file_counts_as_missing() {
        let (_dir, change) = make_change_dir();
        write_web_contract_with_states(&change, "empty");
        write_delta_e2e(&change, "Consumer opens the dashboard");
        write_evidence(&change, "Consumer opens the dashboard", "mobile.png");
        write_evidence(&change, "Consumer opens the dashboard", "desktop.png");
        write_evidence(&change, "Consumer opens the dashboard", "mobile--empty.png");
        let empty = change
            .join("evidence/visual")
            .join(scenario_slug("Consumer opens the dashboard"))
            .join("desktop--empty.png");
        std::fs::write(empty, b"").unwrap();
        let design = std::fs::read_to_string(change.join("design.md")).unwrap();
        let contract = parse_visual_contract(&design).unwrap();
        assert!(check_visual_evidence(&change, &contract).is_err());
    }

    #[test]
    fn persona_disposition_requires_a_confirmed_persona() {
        assert_eq!(
            persona_disposition("PERSONAS: shopper-anna\n"),
            Some("shopper-anna".to_string())
        );
        assert!(check_persona_disposition("PERSONAS: shopper-anna\n").is_ok());
        assert!(check_persona_disposition("PERSONAS: none\n").is_err());
        assert!(check_persona_disposition("PERSONAS:\n").is_err());
        assert!(check_persona_disposition("no marker\n").is_err());
    }

    #[test]
    fn fenced_persona_scaffold_does_not_count() {
        let review = "```\nPERSONAS: fixture-persona\n```\n";
        assert!(check_persona_disposition(review).is_err());
    }

    #[test]
    fn absent_section_is_exempt() {
        assert_eq!(parse_visual_contract("# Design\n\nNo contract here.\n"), None);
    }

    #[test]
    fn scenario_slug_matches_the_documented_rule() {
        assert_eq!(
            scenario_slug("Consumer opens the dashboard"),
            "consumer-opens-the-dashboard"
        );
        assert_eq!(scenario_slug("C001: Paused -- now!"), "c001-paused-now");
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
    fn hash_presence_fails_with_duplicate_hash() {
        let (_dir, change) = make_change_dir();
        std::fs::write(
            change.join("tasks.md"),
            "- [x] COMMIT: a1b2c3d4 -- Scenario one\n- [x] COMMIT: a1b2c3d4 -- Scenario two\n",
        )
        .unwrap();
        assert!(check_hash_presence(&change).is_err());
    }

    #[test]
    fn hash_presence_passes_with_distinct_hashes() {
        let (_dir, change) = make_change_dir();
        std::fs::write(
            change.join("tasks.md"),
            "- [x] COMMIT: a1b2c3d4 -- Scenario one\n- [x] COMMIT: e5f6g7h8 -- Scenario two\n",
        )
        .unwrap();
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
