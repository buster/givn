mod hygiene;
mod manifest;
mod specification;

use crate::gherkin::delta::is_removed_placeholder;
use crate::gherkin::parse::{LintKind, collect_feature_files, parse_file};
use crate::overlap::analysis::Finding;
use anyhow::Result;
use hygiene::{check_persona_references, check_seed_status};
use specification::{SpecificationHealth, specification_health};

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes:
///   0 = clean
///   1 = parse error or a refused specification declaration
///   2 = @wip / PENDING
///   3 = duplicate scenario title (overlap hard error) or empty feature
///
/// Parse errors (1) outrank duplicate titles/empty features (3), which
/// outrank @wip (2).
/// `@givn.removed` scenarios suppress @wip warnings (they contain placeholder steps).
///
/// Returns the exit code; the caller (`main`) maps non-zero codes to
/// `std::process::exit`.
pub fn run(change: Option<&str>, paths: Vec<String>, json: bool) -> Result<i32> {
    let project_root = std::env::current_dir()?;
    run_with_root(&project_root, change, paths, json)
}

/// Run the change-scoped lint gate and refuse a non-zero exit.
///
/// Used by `givn check review --change`, where a duplicate title, an invalid
/// use-case document, or an interaction mismatch is a blocking finding.
pub(crate) fn run_scoped(project_root: &std::path::Path, change: &str) -> Result<()> {
    let code = run_with_root(project_root, Some(change), Vec::new(), false)?;
    if code != 0 {
        anyhow::bail!(
            "lint gate failed for change '{change}' (exit code {code}); fix the findings above"
        );
    }
    Ok(())
}

fn run_with_root(
    project_root: &std::path::Path,
    change: Option<&str>,
    paths: Vec<String>,
    json: bool,
) -> Result<i32> {
    // Advisory hygiene checks must not change the exit-code contract, so they
    // are rendered and counted separately from the blocking flags.
    let default_scope = paths.is_empty() && change.is_none();
    let change_scope = paths.is_empty() && change.is_some();
    let mut findings: Vec<Finding> = if default_scope {
        check_seed_status(project_root)
    } else {
        Vec::new()
    };
    if default_scope || change_scope {
        findings.extend(check_persona_references(project_root, change));
    }
    let advisory_count = findings.len();

    // The declaration is read before the empty-feature shortcut, so a malformed
    // or absent declaration is refused on a project with no `.feature` file. A
    // refused declaration is the only finding: the advisory pass would read a
    // partial role set. `ise` off reads nothing, whatever sits on disk.
    if default_scope {
        match specification_health(project_root)? {
            SpecificationHealth::Blocking(finding) => {
                if json {
                    println!("{}", findings_json(std::slice::from_ref(&finding))?);
                } else {
                    render_prose(&finding);
                }
                return Ok(1);
            }
            SpecificationHealth::Advisories(declaration_findings) => {
                findings.extend(declaration_findings);
            }
        }
    }

    let feature_files = if !paths.is_empty() {
        paths.iter().map(std::path::PathBuf::from).collect()
    } else if let Some(id) = change {
        // Scope to givn/changes/<id>/specs/
        let change_dir = crate::state::resolve_change_dir(project_root, id)?;
        if !change_dir.exists() {
            anyhow::bail!("change '{id}' not found");
        }
        collect_feature_files(&change_dir.join("specs"))
    } else {
        // Default: check all .feature files under givn/ in the project,
        // excluding the archive tree (historical, no longer executable).
        let archive_root = project_root.join("givn/archive");
        collect_feature_files(&project_root.join("givn"))
            .into_iter()
            .filter(|p| !p.starts_with(&archive_root))
            .collect::<Vec<_>>()
    };

    if feature_files.is_empty() && paths.is_empty() {
        let scope = match change {
            Some(id) => format!("givn/changes/{id}/specs/"),
            None => "givn/".to_string(),
        };
        if change.is_none() {
            if json {
                println!("{}", findings_json(&findings)?);
            } else {
                for finding in &findings {
                    render_prose(finding);
                }
                println!("givn lint: no .feature files found under {scope}");
            }
            return Ok(0);
        }
        // A change with a narrative-only delta still projects. Keep going so
        // the interaction inventory is checked before the empty-feature shortcut.
        if !json {
            println!("givn lint: no .feature files found under {scope}");
        }
    }

    let mut has_parse_error = false;
    let mut has_wip = false;
    let mut has_duplicate = false;
    let mut has_empty_feature = false;
    let mut has_use_case_error = false;
    let mut total_files = 0;
    let mut total_errors = advisory_count;

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

        if let Some(feature) = &result.feature
            && feature.scenarios.is_empty()
            && !crate::gherkin::delta::is_retired_feature(feature)
        {
            has_empty_feature = true;
            total_errors += 1;
            findings.push(Finding {
                kind: "empty_feature".to_string(),
                level: "error".to_string(),
                files: vec![path.display().to_string()],
                titles: vec!["feature has zero scenarios".to_string()],
                contradiction: false,
            });
        }
    }

    // Cross-reference tasks.md for @e2e intent: if tasks.md plans e2e but no
    // @e2e-tagged scenarios exist in the specs, the tags may have been removed.
    if let Some(id) = change
        && !has_parse_error
    {
        let change_dir = crate::state::resolve_change_dir(project_root, id)?;
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
                        eprintln!("[WARN] finding  {}  {}", specs_dir.display(), message);
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

    // Tree-wide overlap scan. Whole-tree scope by default; with --change,
    // run the same scan but keep only findings that involve the change's
    // delta (same delta-prefix filter as run_review_overlap_gate), so the
    // authoring loop surfaces overlap findings before check review does.
    if paths.is_empty() && !has_parse_error {
        let all_findings = crate::overlap::analysis::scan_tree(project_root);
        let scoped_findings: Vec<_> = if let Some(id) = change {
            let change_dir = crate::state::resolve_change_dir(project_root, id)?;
            let delta_prefix = change_dir.join("specs").to_string_lossy().into_owned();
            all_findings
                .into_iter()
                .filter(|f| f.files.iter().any(|file| file.starts_with(&delta_prefix)))
                .collect()
        } else {
            all_findings
        };
        for finding in scoped_findings {
            if finding.kind == "duplicate_title" {
                total_errors += 1;
                has_duplicate = true;
            }
            if matches!(
                finding.kind.as_str(),
                "usecase_invalid"
                    | "fragment_invalid"
                    | "dangling_capability_ref"
                    | "ambiguous_relationship_target"
                    | "dangling_use_case_ref"
                    | "use_case_cycle"
                    | "extend_fragment"
                    | "interaction_mismatch"
                    | "usecase_interaction_invalid"
                    | "undeclared_capability"
            ) {
                total_errors += 1;
                has_use_case_error = true;
            }
            findings.push(finding);
        }

        // Reuse the same in-memory projection for --change. It reports only
        // the projected corpus; the delta scan above owns syntax and schema
        // findings, so skip the projection when one is already blocking.
        if let Some(id) = change
            && !has_use_case_error
        {
            for finding in crate::spec::projection::interaction_findings(project_root, id)? {
                total_errors += 1;
                has_use_case_error = true;
                findings.push(finding);
            }
        }
    }

    if json {
        println!("{}", findings_json(&findings)?);
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

    // Exit code: parse errors (1) outrank duplicate titles and empty
    // features (3), which outrank @wip/PENDING (2).
    let exit_code = if has_parse_error || has_use_case_error {
        1
    } else if has_duplicate || has_empty_feature {
        3
    } else if has_wip {
        2
    } else {
        0
    };
    Ok(exit_code)
}

/// The severity of one lint finding. Descriptive: the exit-code flags still
/// drive the exit code, so an advisory-only run exits 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Advisory,
    Finding,
    Blocking,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Advisory => "advisory",
            Severity::Finding => "finding",
            Severity::Blocking => "blocking",
        }
    }
}

/// The one registry mapping a finding kind to its severity. New kinds
/// register here; rendering never changes.
fn severity_for(kind: &str) -> Severity {
    match kind {
        "specification_declaration"
        | "parse_error"
        | "duplicate_title"
        | "empty_feature"
        | "usecase_invalid"
        | "fragment_invalid"
        | "dangling_capability_ref"
        | "ambiguous_relationship_target"
        | "dangling_use_case_ref"
        | "use_case_cycle"
        | "extend_fragment"
        | "interaction_mismatch"
        | "usecase_interaction_invalid"
        | "undeclared_capability" => Severity::Blocking,
        "specification_path_missing"
        | "specification_role_duplicate"
        | "specification_feature_unretained"
        | "specification_e2e_boundary" => Severity::Advisory,
        "wip" | "e2e_intent" => Severity::Finding,
        _ => Severity::Advisory,
    }
}

/// Render findings as JSON objects with the original fields plus `severity`.
fn findings_json(findings: &[Finding]) -> Result<String> {
    let enriched: Vec<serde_json::Value> = findings
        .iter()
        .map(|finding| {
            let mut value = serde_json::to_value(finding)?;
            value["severity"] = serde_json::json!(severity_for(&finding.kind).as_str());
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(serde_json::to_string_pretty(&enriched)?)
}

/// Render one finding in the human-readable format on stderr.
fn render_prose(finding: &Finding) {
    let severity = severity_for(&finding.kind).as_str();
    let level = match finding.kind.as_str() {
        "duplicate_title" => "DUPL ",
        "specification_declaration" => "DECL ",
        "specification_path_missing"
        | "specification_role_duplicate"
        | "specification_feature_unretained"
        | "specification_e2e_boundary" => "SPEC ",
        "shape_match" => "SHAPE",
        "subset" => "SUBST",
        "long_scenario" => "LONG ",
        "empty_feature" => "EMPTY",
        "seed_status" => "SEED ",
        "persona_ref" => "PERS ",
        "usecase_invalid" | "fragment_invalid" => "UCDOC",
        "dangling_use_case_ref" => "UCREF",
        "use_case_cycle" => "UCCYC",
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
                "[{level}] {severity}  {location}  duplicate scenario title '{}'",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        "shape_match" => {
            let (a, b) = (finding.titles[0].as_str(), finding.titles[1].as_str());
            if finding.contradiction {
                eprintln!(
                    "[{level}] {severity}  {location}  related_scenario: '{a}' and '{b}' share a shape but their facts contradict"
                );
            } else {
                eprintln!(
                    "[{level}] {severity}  {location}  scenarios '{a}' and '{b}' share a shape — consider merging"
                );
            }
        }
        "subset" => {
            eprintln!(
                "[{level}] {severity}  {location}  subset: scenario '{a}' is a subset of '{b}'",
                a = finding.titles[0],
                b = finding.titles[1]
            );
        }
        "long_scenario" => {
            eprintln!(
                "[{level}] {severity}  {location}  long scenario: '{title}' has more than {} steps",
                crate::overlap::analysis::LONG_SCENARIO_STEP_THRESHOLD,
                title = finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        "empty_feature" => {
            eprintln!("[{level}] {severity}  {location}  feature has zero scenarios");
        }
        _ => {
            eprintln!(
                "[{level}] {severity}  {location}  {}",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A temp project with givn/specs/ seeded from the given (path, text)
    /// pairs. Returns the temp dir handle (keep alive) and the project root.
    fn temp_project(files: &[(&str, &str)]) -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        for (rel, text) in files {
            let path = project.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let mut usecases = std::collections::BTreeSet::new();
        for (rel, _) in files {
            let path = std::path::Path::new(rel);
            if path
                .extension()
                .is_some_and(|extension| extension == "feature")
                && let Some(parent) = path.parent()
            {
                usecases.insert(parent.to_path_buf());
            }
        }
        for relative in usecases {
            let usecase = relative.file_name().unwrap().to_string_lossy();
            let feature = relative
                .file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_default();
            let document = format!(
                "# Use case: {usecase}\n\n## Level\n\n!\n\n## Actors\n\n- Operator\n\n## Goal\n\nInspect behavior.\n\n## Trigger\n\nThe operator asks.\n\n## Preconditions\n\n- The project exists.\n\n## Main flow\n\n1. Inspect.\n\n## Extensions\n\n- none\n\n## Rules\n\n- The project remains valid.\n\n## Examples\n\n- A project.\n\n## Minimal guarantee\n\nThe project remains inspectable.\n\n## Success guarantee\n\nThe result is visible.\n\n## Personas\n\n- none\n\n## Capabilities\n\n- {feature}\n\n## Interactions\n\n- none\n\n## Includes\n\n- none\n\n## Extends\n\n- none\n\n## Out of scope\n\nNothing.\n\n## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
            );
            std::fs::write(project.join(relative).join("usecase.md"), document).unwrap();
        }
        (root, project)
    }

    #[test]
    fn empty_feature_is_an_error_and_exits_3() {
        let (_root, project) = temp_project(&[(
            "givn/specs/hollow/hollow.feature",
            "@givn.delta @hollow\nFeature: Hollow\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 3, "empty_feature must exit 3");
    }

    #[test]
    fn retired_empty_feature_does_not_trigger_the_finding() {
        let (_root, project) = temp_project(&[(
            "givn/specs/old-cap/old-cap.feature",
            "@givn.delta @givn.retired @old-cap\nFeature: Old capability\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 0, "retirement markers are legitimately empty");
    }

    #[test]
    fn populated_feature_is_clean() {
        let (_root, project) = temp_project(&[(
            "givn/specs/full/full.feature",
            "Feature: Full\n\n  Scenario: One\n    Given x\n    Then y\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 0);
    }

    #[test]
    fn an_invalid_interaction_structure_is_blocking() {
        assert_eq!(
            severity_for("usecase_interaction_invalid"),
            Severity::Blocking
        );
    }
}
