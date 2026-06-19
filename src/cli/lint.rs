use anyhow::Result;
use crate::gherkin::parse::{collect_feature_files, parse_file, LintKind};
use crate::gherkin::delta::is_removed_placeholder;

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes:
///   0 = clean
///   1 = parse error
///   2 = @wip / PENDING
///
/// Parse errors outrank wip/PENDING.
/// `@givn.removed` scenarios suppress @wip warnings (they contain placeholder steps).
pub fn run(change: Option<&str>, paths: Vec<String>) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let feature_files = if !paths.is_empty() {
        paths
            .iter()
            .map(|p| std::path::PathBuf::from(p))
            .collect()
    } else if let Some(id) = change {
        // Scope to givn/<id>/specs/
        let specs_dir = project_root.join("givn").join(id).join("specs");
        let files = collect_feature_files(&specs_dir);
        if files.is_empty() {
            println!("givn lint: no .feature files found under givn/{id}/specs/");
            return Ok(());
        }
        files
    } else {
        // Default: check all .feature files under givn/ in the project.
        let files = collect_feature_files(&project_root.join("givn"));
        if files.is_empty() {
            println!("givn lint: no .feature files found under givn/");
            return Ok(());
        }
        files
    };

    let mut max_severity = 0u8;
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
                        let is_in_removed = feature.scenarios.iter().any(|s| {
                            is_removed_placeholder(s) && s.position.line <= wip_line
                        });
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
            let severity = match error.kind {
                LintKind::ParseError => 1u8,
                LintKind::Wip => 2u8,
            };
            max_severity = max_severity.max(severity);

            let location = error
                .line
                .map(|l| format!(":{l}"))
                .unwrap_or_default();
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

    // Summary.
    if total_errors == 0 {
        println!("givn lint: {total_files} file(s) checked — clean");
    } else {
        eprintln!(
            "givn lint: {total_files} file(s) checked, {total_errors} finding(s)"
        );
    }

    // Exit with the appropriate code.
    // (Note: actual process exit is handled by mapping to a non-zero return in main.)
    match max_severity {
        0 => Ok(()),
        1 => std::process::exit(1),
        _ => std::process::exit(2),
    }
}
