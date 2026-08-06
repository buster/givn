use crate::gherkin::delta::is_removed_placeholder;
use crate::gherkin::parse::{LintKind, collect_feature_files, parse_file};
use anyhow::Result;

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes:
///   0 = clean
///   1 = parse error
///   2 = @wip / PENDING
///
/// Parse errors outrank wip/PENDING.
/// `@givn.removed` scenarios suppress @wip warnings (they contain placeholder steps).
///
/// Returns the exit code; the caller (`main`) maps non-zero codes to
/// `std::process::exit`.
pub fn run(change: Option<&str>, paths: Vec<String>) -> Result<i32> {
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
            println!("givn lint: no .feature files found under givn/changes/{id}/specs/");
            return Ok(0);
        }
        files
    } else {
        // Default: check all .feature files under givn/ in the project.
        let files = collect_feature_files(&project_root.join("givn"));
        if files.is_empty() {
            println!("givn lint: no .feature files found under givn/");
            return Ok(0);
        }
        files
    };

    let mut has_parse_error = false;
    let mut has_wip = false;
    let mut total_files = 0;
    let mut total_errors = 0;

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
            match error.kind {
                LintKind::ParseError => has_parse_error = true,
                LintKind::Wip => has_wip = true,
            }

            let location = error.line.map(|l| format!(":{l}")).unwrap_or_default();
            let level = match error.kind {
                LintKind::ParseError => "ERROR",
                LintKind::Wip => "WIP  ",
            };
            eprintln!(
                "[{level}] {}{}  {}",
                path.display(),
                location,
                error.message
            );
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
                    eprintln!(
                        "[WARN] {}  @e2e scenarios referenced in tasks.md but none found in specs/",
                        specs_dir.display()
                    );
                    eprintln!("       Restore @e2e tags or remove the e2e tasks from tasks.md.");
                    total_errors += 1;
                    has_wip = true;
                }
            }
        }
    }

    // Summary.
    if total_errors == 0 {
        println!("givn lint: {total_files} file(s) checked — clean");
    } else {
        eprintln!("givn lint: {total_files} file(s) checked, {total_errors} finding(s)");
    }

    // Exit code: parse errors (1) outrank @wip/PENDING (2).
    let exit_code = if has_parse_error {
        1
    } else if has_wip {
        2
    } else {
        0
    };
    Ok(exit_code)
}
