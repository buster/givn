use crate::gherkin::delta::is_removed_placeholder;
use crate::gherkin::parse::{LintKind, collect_feature_files, parse_file};
use crate::overlap::analysis::Finding;
use anyhow::Result;

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes:
///   0 = clean
///   1 = parse error
///   2 = @wip / PENDING
///   3 = duplicate scenario title (overlap hard error)
///
/// Parse errors (1) outrank duplicate titles (3), which outrank @wip (2).
/// `@givn.removed` scenarios suppress @wip warnings (they contain placeholder steps).
///
/// Returns the exit code; the caller (`main`) maps non-zero codes to
/// `std::process::exit`.
pub fn run(change: Option<&str>, paths: Vec<String>, json: bool) -> Result<i32> {
    let project_root = std::env::current_dir()?;
    let feature_files = if !paths.is_empty() {
        paths.iter().map(std::path::PathBuf::from).collect()
    } else if let Some(id) = change {
        // Scope to givn/changes/<id>/specs/
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        if !change_dir.exists() {
            anyhow::bail!("change '{id}' not found");
        }
        let specs_dir = change_dir.join("specs");
        let files = collect_feature_files(&specs_dir);
        if files.is_empty() {
            if json {
                println!("[]");
            } else {
                println!("givn lint: no .feature files found under givn/changes/{id}/specs/");
            }
            return Ok(0);
        }
        files
    } else {
        // Default: check all .feature files under givn/ in the project,
        // excluding the archive tree (historical, no longer executable).
        let archive_root = project_root.join("givn/archive");
        let files = collect_feature_files(&project_root.join("givn"))
            .into_iter()
            .filter(|p| !p.starts_with(&archive_root))
            .collect::<Vec<_>>();
        if files.is_empty() {
            if json {
                println!("[]");
            } else {
                println!("givn lint: no .feature files found under givn/");
            }
            return Ok(0);
        }
        files
    };

    let mut has_parse_error = false;
    let mut has_wip = false;
    let mut has_duplicate = false;
    let mut total_files = 0;
    let mut total_errors = 0;
    let mut findings: Vec<Finding> = Vec::new();

    for path in &feature_files {
        total_files += 1;
        let result = parse_file(path);

        // Filter out @wip findings from @givn.removed scenarios.
        let errors: Vec<_> = result
            .errors
            .iter()
            .filter(|e| {
                if e.kind == LintKind::Wip {
                    // Check if it belongs to a @givn.removed scenario — suppress if so.
                    // We do this by checking if the feature parsed and has a removed scenario
                    // at the same line.
                    if let Some(ref feature) = result.feature {
                        let wip_line = e.line.unwrap_or(0);
                        let is_in_removed = feature
                            .scenarios
                            .iter()
                            .any(|s| is_removed_placeholder(s) && s.position.line == wip_line);
                        if is_in_removed {
                            return false; // suppress
                        }
                    }
                }
                true
            })
            .collect();

        for error in &errors {
            total_errors += 1;
            let kind = match error.kind {
                LintKind::ParseError => {
                    has_parse_error = true;
                    "parse_error"
                }
                LintKind::Wip => {
                    has_wip = true;
                    "wip"
                }
            };
            findings.push(Finding {
                kind: kind.to_string(),
                level: if kind == "parse_error" {
                    "error"
                } else {
                    "warning"
                }
                .to_string(),
                files: vec![path.display().to_string()],
                titles: vec![error.message.clone()],
                contradiction: false,
            });
        }
    }

    // Cross-reference tasks.md for @e2e intent: if tasks.md plans e2e but no
    // @e2e-tagged scenarios exist in the specs, the tags may have been removed.
    if let Some(id) = change
        && !has_parse_error
    {
        let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
        let tasks_path = change_dir.join("tasks.md");
        if tasks_path.exists() {
            let tasks_content = std::fs::read_to_string(&tasks_path).unwrap_or_default();
            if crate::hooks::has_e2e_intent_in_tasks(&tasks_content) {
                let specs_dir = change_dir.join("specs");
                let has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
                if !has_e2e {
                    let message = "@e2e scenarios referenced in tasks.md but none found in specs/";
                    if json {
                        findings.push(Finding {
                            kind: "e2e_intent".to_string(),
                            level: "warning".to_string(),
                            files: vec![specs_dir.display().to_string()],
                            titles: vec![message.to_string()],
                            contradiction: false,
                        });
                    } else {
                        eprintln!("[WARN] {}  {}", specs_dir.display(), message);
                        eprintln!(
                            "       Restore @e2e tags or remove the e2e tasks from tasks.md."
                        );
                    }
                    total_errors += 1;
                    has_wip = true;
                }
            }
        }
    }

    // Tree-wide overlap scan (default whole-tree scope only; explicit paths
    // and --change keep their existing scoped semantics).
    if paths.is_empty() && change.is_none() {
        for finding in crate::overlap::analysis::scan_tree(&project_root) {
            if finding.kind == "duplicate_title" {
                total_errors += 1;
                has_duplicate = true;
            }
            findings.push(finding);
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&findings)?);
    } else {
        for finding in &findings {
            render_prose(finding);
        }
        // Summary.
        if total_errors == 0 {
            println!("givn lint: {total_files} file(s) checked — clean");
        } else {
            eprintln!("givn lint: {total_files} file(s) checked, {total_errors} finding(s)");
        }
    }

    // Exit code: parse errors (1) outrank duplicate titles (3), which
    // outrank @wip/PENDING (2).
    let exit_code = if has_parse_error {
        1
    } else if has_duplicate {
        3
    } else if has_wip {
        2
    } else {
        0
    };
    Ok(exit_code)
}

/// Render one finding in the human-readable format on stderr.
fn render_prose(finding: &Finding) {
    let level = match finding.kind.as_str() {
        "duplicate_title" => "DUPL ",
        "shape_match" => "SHAPE",
        "subset" => "SUBST",
        "long_scenario" => "LONG ",
        "parse_error" => "ERROR",
        "wip" => "WIP  ",
        _ => "WARN ",
    };
    let location = if finding.files.len() > 1 {
        format!("{} & {}", finding.files[0], finding.files[1])
    } else {
        finding.files.first().cloned().unwrap_or_default()
    };
    match finding.kind.as_str() {
        "duplicate_title" => {
            eprintln!(
                "[{level}] {location}  duplicate scenario title '{}'",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        "shape_match" => {
            let (a, b) = (finding.titles[0].as_str(), finding.titles[1].as_str());
            if finding.contradiction {
                eprintln!(
                    "[{level}] {location}  related_scenario: '{a}' and '{b}' share a shape but their facts contradict"
                );
            } else {
                eprintln!(
                    "[{level}] {location}  scenarios '{a}' and '{b}' share a shape — consider merging"
                );
            }
        }
        "subset" => {
            eprintln!(
                "[{level}] {location}  subset: scenario '{a}' is a subset of '{b}'",
                a = finding.titles[0],
                b = finding.titles[1]
            );
        }
        "long_scenario" => {
            eprintln!(
                "[{level}] {location}  long scenario: '{title}' has more than 14 steps",
                title = finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        _ => {
            eprintln!(
                "[{level}] {location}  {}",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
    }
}
