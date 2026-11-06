use crate::error::GivnError;
use crate::gherkin::delta::{DeltaOp, DeltaScenario, strip_givn_tags};
/// Merge engine: apply delta scenarios to a permanent .feature file.
///
/// Semantics (ADR-011, arc42 ch.8.3):
/// - `@givn.added`: append scenario to permanent spec. Error if title already exists.
/// - `@givn.modified`: replace scenario by title. Error if not found.
/// - `@givn.removed`: delete scenario by title. Error if not found.
/// - All `@givn.*` tags are stripped from the output.
/// - Merge is all-or-nothing: validate all preconditions before any write.
///
/// The merge works at the text level rather than on a parsed AST, to preserve
/// formatting and comments in the permanent spec files.
use anyhow::{Result, bail};
use gherkin::{Feature, Scenario};

/// Apply a set of delta scenarios to the permanent feature text.
/// Returns the new permanent feature text with all @givn.* tags stripped.
///
/// `permanent_text`: the current content of `givn/specs/<cap>/<cap>.feature`.
///                   May be empty/None for a brand-new capability.
/// `deltas`:         the delta scenarios to apply.
/// `delta_feature_name`: the name of the delta feature — used as the baseline
///                   header when `permanent_text` is empty (new capability).
pub fn apply_deltas(
    permanent_text: Option<&str>,
    deltas: &[DeltaScenario<'_>],
    delta_feature_name: &str,
) -> Result<String> {
    apply_deltas_mode(
        permanent_text,
        deltas,
        delta_feature_name,
        MergeMode::Strict,
    )
}

/// Apply deltas for a retried in-progress file. An added scenario whose
/// parsed body already matches is a no-op, and an already-absent removed
/// scenario is a no-op; a different body under the same title still fails.
#[cfg(test)]
pub fn apply_deltas_retry(
    permanent_text: Option<&str>,
    deltas: &[DeltaScenario<'_>],
    delta_feature_name: &str,
) -> Result<String> {
    apply_deltas_mode(permanent_text, deltas, delta_feature_name, MergeMode::Retry)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MergeMode {
    Strict,
    Retry,
}

fn apply_deltas_mode(
    permanent_text: Option<&str>,
    deltas: &[DeltaScenario<'_>],
    delta_feature_name: &str,
    mode: MergeMode,
) -> Result<String> {
    if let Some(text) = permanent_text.filter(|text| !text.trim().is_empty()) {
        crate::gherkin::parse::parse_supported_source(text, std::path::Path::new("<permanent>"))?;
    }
    // Parse the permanent feature (or start from an empty scenario list).
    let mut perm_scenarios: Vec<ScenarioText> = match permanent_text {
        Some(text) if !text.trim().is_empty() => parse_scenarios_from_text(text)?,
        _ => vec![],
    };

    let feature_header = match permanent_text {
        Some(text) if !text.trim().is_empty() => extract_feature_header(text),
        _ => format!("Feature: {}\n", delta_feature_name),
    };

    // Validate and apply in one sequential pass. Each op is checked
    // against the *projected* state, so a delta may legitimately remove a
    // scenario and re-add a replacement with the same title (rewriting a
    // capability is one atomic delta, not two).
    for delta in deltas {
        let title = &delta.scenario.name;
        let exists = perm_scenarios.iter().any(|s| s.title == *title);
        match delta.op {
            DeltaOp::Added => {
                if exists {
                    if mode == MergeMode::Retry
                        && perm_scenarios
                            .iter()
                            .find(|s| s.title == *title)
                            .is_some_and(|s| scenario_matches(&s.text, delta.scenario))
                    {
                        continue;
                    }
                    return Err(GivnError::MergeError(format!(
                        "cannot add scenario '{}': title already exists in the permanent spec",
                        title
                    ))
                    .into());
                }
                perm_scenarios.push(ScenarioText {
                    title: title.clone(),
                    text: render_scenario_stripped(delta.scenario),
                });
            }
            DeltaOp::Modified => {
                if !exists {
                    return Err(GivnError::MergeError(format!(
                        "cannot {:?} scenario '{}': title not found in the permanent spec",
                        delta.op, title
                    ))
                    .into());
                }
                if let Some(s) = perm_scenarios.iter_mut().find(|s| s.title == *title) {
                    s.text = render_scenario_stripped(delta.scenario);
                }
            }
            DeltaOp::Removed => {
                if !exists {
                    if mode == MergeMode::Retry {
                        continue;
                    }
                    return Err(GivnError::MergeError(format!(
                        "cannot {:?} scenario '{}': title not found in the permanent spec",
                        delta.op, title
                    ))
                    .into());
                }
                perm_scenarios.retain(|s| s.title != *title);
            }
        }
    }

    // Reassemble the file.
    let mut output = feature_header;
    for scenario in &perm_scenarios {
        output.push('\n');
        output.push_str(scenario.text.trim_matches('\n'));
        output.push('\n');
    }
    if !output.ends_with('\n') {
        output.push('\n');
    }

    Ok(output)
}

/// Compare the parsed permanent scenario with the delta scenario by rendering
/// both through the same tag-stripping renderer. This normalizes tag lines,
/// blank lines, step formatting, tables, and docstrings without comparing raw
/// source text.
fn scenario_matches(existing_text: &str, delta: &Scenario) -> bool {
    let wrapped = format!("Feature: retry\n{existing_text}\n");
    let Ok(feature) = gherkin::Feature::parse(&wrapped, gherkin::GherkinEnv::default()) else {
        return false;
    };
    let Some(existing) = feature.scenarios.first() else {
        return false;
    };
    render_scenario_stripped(existing).trim_end() == render_scenario_stripped(delta).trim_end()
}

pub fn validate_feature_merge(permanent_text: Option<&str>, delta_feature: &Feature) -> Result<()> {
    apply_feature_mode(permanent_text, delta_feature, MergeMode::Strict).map(|_| ())
}

pub fn apply_feature_deltas(
    permanent_text: Option<&str>,
    delta_feature: &Feature,
) -> Result<String> {
    apply_feature_mode(permanent_text, delta_feature, MergeMode::Strict)
}

pub fn apply_feature_deltas_retry(
    permanent_text: Option<&str>,
    delta_feature: &Feature,
) -> Result<String> {
    apply_feature_mode(permanent_text, delta_feature, MergeMode::Retry)
}

fn apply_feature_mode(
    permanent_text: Option<&str>,
    delta_feature: &Feature,
    mode: MergeMode,
) -> Result<String> {
    crate::gherkin::parse::validate_supported_feature(delta_feature)?;
    let permanent = permanent_text
        .filter(|text| !text.trim().is_empty())
        .map(|text| {
            crate::gherkin::parse::parse_supported_source(text, std::path::Path::new("<permanent>"))
        })
        .transpose()?;
    if let (Some(permanent), Some(background)) = (&permanent, &delta_feature.background)
        && crate::gherkin::content::background_content(permanent.background.as_ref())
            != crate::gherkin::content::background_content(Some(background))
    {
        bail!(
            "conflicting Background '{}' in feature '{}'",
            background.name,
            delta_feature.name
        );
    }
    let mut scenarios = delta_feature.scenarios.clone();
    for scenario in &mut scenarios {
        for tag in strip_givn_tags(&delta_feature.tags) {
            if !scenario.tags.contains(&tag) {
                scenario.tags.push(tag);
            }
        }
    }
    let deltas: Vec<_> = scenarios
        .iter()
        .map(|scenario| DeltaScenario {
            scenario,
            op: crate::gherkin::delta::delta_op(scenario),
        })
        .collect();
    let baseline = if permanent.is_none() {
        let mut header = String::new();
        let tags = strip_givn_tags(&delta_feature.tags);
        if !tags.is_empty() {
            header.push_str(&format!(
                "{}\n",
                tags.iter()
                    .map(|tag| format!("@{tag}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        header.push_str(&format!("Feature: {}\n", delta_feature.name));
        if let Some(description) = &delta_feature.description {
            for line in description.lines() {
                header.push_str(&format!("  {line}\n"));
            }
        }
        if let Some(background) = &delta_feature.background {
            header.push_str(&format!("\n  Background: {}\n", background.name));
            render_steps(&mut header, &background.steps);
        }
        Some(header)
    } else {
        None
    };
    apply_deltas_mode(
        baseline.as_deref().or(permanent_text),
        &deltas,
        &delta_feature.name,
        mode,
    )
}

// ─── Internal helpers ────────────────────────────────────────────────────────

/// A scenario entry in the permanent spec (title + raw text block).
#[derive(Debug, Clone)]
struct ScenarioText {
    title: String,
    text: String,
}

fn scenario_start(lines: &[&str], scenario: &Scenario) -> usize {
    let mut start = scenario.position.line - 1;
    while start > 0 {
        let previous = lines[start - 1].trim();
        if previous.starts_with('@') || previous.is_empty() || previous.starts_with('#') {
            start -= 1;
        } else {
            break;
        }
    }
    start
}

/// Parse the permanent feature text into a list of scenario text blocks.
fn parse_scenarios_from_text(text: &str) -> Result<Vec<ScenarioText>> {
    let feature =
        crate::gherkin::parse::parse_supported_source(text, std::path::Path::new("<permanent>"))?;
    let lines: Vec<_> = text.lines().collect();
    Ok(feature
        .scenarios
        .iter()
        .enumerate()
        .map(|(index, scenario)| {
            let start = scenario_start(&lines, scenario);
            let end = feature
                .scenarios
                .get(index + 1)
                .map_or(lines.len(), |next| scenario_start(&lines, next));
            ScenarioText {
                title: scenario.name.clone(),
                text: lines[start..end].join("\n"),
            }
        })
        .collect())
}

fn extract_feature_header(text: &str) -> String {
    let feature =
        Feature::parse(text, gherkin::GherkinEnv::default()).expect("validated permanent feature");
    let lines: Vec<_> = text.lines().collect();
    let end = feature
        .scenarios
        .first()
        .map_or(lines.len(), |scenario| scenario_start(&lines, scenario));
    format!("{}\n", lines[..end].join("\n").trim_end())
}

fn render_steps(out: &mut String, steps: &[gherkin::Step]) {
    for step in steps {
        out.push_str(&format!("    {} {}\n", step.keyword, step.value));

        // Docstring (the crate stores content already dedented, with the
        // newline that follows the opening delimiter still present).
        if let Some(doc) = step.docstring() {
            let delimiter = if doc.contains("\"\"\"") {
                "```"
            } else {
                "\"\"\""
            };
            out.push_str(&format!("      {delimiter}"));
            let content = if let Some(content) = doc.strip_prefix('\n') {
                out.push('\n');
                content
            } else if let Some((media_type, content)) = doc.split_once('\n') {
                out.push_str(media_type);
                out.push('\n');
                content
            } else {
                out.push_str(doc);
                out.push('\n');
                ""
            };
            if !content.is_empty() {
                for line in content.lines() {
                    if line.is_empty() {
                        out.push('\n');
                    } else {
                        if doc.starts_with('\n') {
                            out.push_str(&format!("      {line}\n"));
                        } else {
                            out.push_str(line);
                            out.push('\n');
                        }
                    }
                }
            }
            out.push_str(&format!("      {delimiter}\n"));
        }

        // Data table.
        if let Some(table) = step.table() {
            for row in &table.rows {
                let cells: Vec<String> = row.iter().map(|cell| escape_cell(cell)).collect();
                out.push_str(&format!("      | {} |\n", cells.join(" | ")));
            }
        }
    }
}

fn escape_cell(cell: &str) -> String {
    cell.replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', "\\n")
}

/// Render a scenario to text, stripping all @givn.* tags.
fn render_scenario_stripped(scenario: &Scenario) -> String {
    let clean_tags = strip_givn_tags(&scenario.tags);
    let mut out = String::new();

    // Tags line (if any remain after stripping).
    if !clean_tags.is_empty() {
        let tags_str: Vec<String> = clean_tags.iter().map(|t| format!("@{t}")).collect();
        out.push_str(&format!("  {}\n", tags_str.join(" ")));
    }

    // Scenario line.
    out.push_str(&format!("  {}: {}\n", scenario.keyword, scenario.name));

    render_steps(&mut out, &scenario.steps);

    for examples in &scenario.examples {
        let clean_tags = strip_givn_tags(&examples.tags);
        if !clean_tags.is_empty() {
            let tags: Vec<String> = clean_tags.iter().map(|tag| format!("@{tag}")).collect();
            out.push_str(&format!("    {}\n", tags.join(" ")));
        }
        out.push_str("    Examples");
        if let Some(name) = examples.name.as_deref().filter(|name| !name.is_empty()) {
            out.push_str(&format!(": {name}"));
        } else {
            out.push(':');
        }
        out.push('\n');
        if let Some(description) = examples.description.as_deref() {
            for line in description.lines() {
                out.push_str(&format!("      {line}\n"));
            }
        }
        if let Some(table) = &examples.table {
            for row in &table.rows {
                let cells: Vec<String> = row.iter().map(|cell| escape_cell(cell)).collect();
                out.push_str(&format!("      | {} |\n", cells.join(" | ")));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gherkin::delta::DeltaOp;
    use crate::gherkin::parse::parse_str;

    #[test]
    fn supported_new_feature_keeps_setup_and_inherited_tags() {
        let feature = parse_str("@givn.delta @catalog @e2e\nFeature: Catalog\n  Background: Session\n    Given signed in\n      \"\"\"json\n      {\"ok\": true}\n      \"\"\"\n  @givn.added @api\n  Scenario: Read\n    Then visible\n").unwrap();
        let merged = parse_str(&apply_feature_deltas(None, &feature).unwrap()).unwrap();
        assert!(merged.background.is_some(), "new feature lost Background");
        assert!(merged.tags.contains(&"e2e".into()));
        assert!(merged.scenarios[0].tags.contains(&"api".into()));
        assert_eq!(
            merged.background.unwrap().steps[0].docstring,
            feature.background.unwrap().steps[0].docstring
        );
    }

    #[test]
    fn existing_background_arguments_and_unaffected_tags_survive() {
        let permanent = "@e2e\nFeature: Catalog\n  Background: Setup\n    Given payload\n      \"\"\"\n      Scenario: setup payload\n      @literal\n      \"\"\"\n    And mapping\n      | a | b |\n  @keep\n  Scenario: Keep\n    Then unchanged\n  Scenario: Read\n    Then visible\n";
        let delta = parse_str("@givn.delta @catalog @api\nFeature: Catalog\n  @givn.modified @fast\n  Scenario: Read\n    Then updated\n").unwrap();
        let result = apply_feature_deltas(Some(permanent), &delta).unwrap();
        let merged = parse_str(&result).unwrap();
        let original = parse_str(permanent).unwrap();
        assert_eq!(
            crate::gherkin::content::background_content(merged.background.as_ref()),
            crate::gherkin::content::background_content(original.background.as_ref())
        );
        assert_eq!(merged.scenarios.len(), 2);
        assert_eq!(merged.scenarios[0].tags, ["keep"]);
        assert!(merged.tags.contains(&"e2e".into()));
        assert!(merged.scenarios[1].tags.contains(&"api".into()));
        assert!(merged.scenarios[1].tags.contains(&"fast".into()));
        let mut same_background = delta.clone();
        same_background.background = original.background;
        assert!(validate_feature_merge(Some(permanent), &same_background).is_ok());
    }

    #[test]
    fn conflicting_background_refuses_without_writing() {
        let permanent = "Feature: Catalog\n  Background: Session\n    Given original session\n  Scenario: Read\n    Then visible\n  Scenario: Keep\n    Then unchanged\n";
        let feature = parse_str("@givn.delta @catalog\nFeature: Catalog\n  Background: Session\n    Given different session\n  @givn.modified\n  Scenario: Read\n    Then updated\n").unwrap();
        let result = apply_feature_deltas(Some(permanent), &feature);
        assert!(result.is_err(), "conflicting Background accepted");
    }

    #[test]
    fn unsupported_publication_refuses_without_creating_parent() {
        for source in [
            "@givn.delta @catalog\nFeature: Catalog\n  Rule: Restricted\n    Scenario: Read\n      Then visible\n",
            "# language: fr\n@givn.delta @catalog\nFonctionnalité: Catalogue\n  Scénario: Lire\n    Alors visible\n",
        ] {
            let feature = parse_str(source).unwrap();
            assert!(
                apply_feature_deltas(None, &feature).is_err(),
                "unsupported feature accepted: {source}"
            );
        }
    }

    #[test]
    fn supported_arguments_round_trip_without_phantom_structure() {
        let source = "@givn.delta @catalog\nFeature: Catalog\n  @givn.added\n  Scenario Outline: Read\n    Given payload <name>\n      \"\"\"json\n      Scenario: payload heading\n      Background: payload heading\n      @literal\n      ```\n      \"\"\"\n    When mapping\n      | key | value |\n      | a\\|b | c\\\\d |\n      | empty | |\n      | multiline | a\\nb |\n    Then visible\n    @api\n    Examples: First\n      | name |\n      | a\\|b |\n    @slow\n    Examples: Second\n      | name |\n      | c |\n";
        let feature = parse_str(source).unwrap();
        let deltas = crate::gherkin::delta::delta_scenarios(&feature);
        let first = apply_deltas(None, &deltas, "Catalog").unwrap();
        let second = apply_deltas(Some(&first), &[], "Catalog").unwrap();
        let merged = parse_str(&second).unwrap();
        assert_eq!(
            merged.scenarios.len(),
            1,
            "payload created phantom scenario"
        );
        let expected = &feature.scenarios[0];
        let actual = &merged.scenarios[0];
        assert_eq!(actual.steps[0].docstring, expected.steps[0].docstring);
        assert_eq!(
            actual.steps[1].table.as_ref().unwrap().rows,
            expected.steps[1].table.as_ref().unwrap().rows
        );
        assert_eq!(actual.examples.len(), 2);
        for (actual, expected) in actual.examples.iter().zip(&expected.examples) {
            assert_eq!(actual.tags, expected.tags);
            assert_eq!(
                actual.table.as_ref().unwrap().rows,
                expected.table.as_ref().unwrap().rows
            );
        }
    }

    const PERMANENT: &str = r#"Feature: User auth

  Scenario: User logs in
    Given a registered user
    When they provide valid credentials
    Then they are authenticated

  Scenario: User sees profile
    Given a logged-in user
    When they visit their profile
    Then they see their details
"#;

    #[allow(dead_code)]
    fn make_delta_scenario<'a>(scenario: &'a gherkin::Scenario, op: DeltaOp) -> DeltaScenario<'a> {
        DeltaScenario { scenario, op }
    }

    #[test]
    fn add_new_scenario() {
        let new_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User resets password
    Given a user who forgot their password
    When they request a reset
    Then they receive a reset email
"#;
        let feature = parse_str(new_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("User resets password"));
        assert!(result.contains("User logs in"));
    }

    #[test]
    fn modify_existing_scenario() {
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials and MFA
    Then they are authenticated with MFA
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("MFA"));
        assert!(result.contains("User logs in"));
    }

    #[test]
    fn modified_first_scenario_drops_stale_target_tags() {
        let permanent = r#"Feature: User auth

  @wip
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials
    Then they are authenticated
"#;
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials and MFA
    Then they are authenticated with MFA
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let result = apply_deltas(Some(permanent), &[delta], "Delta").unwrap();
        assert!(!result.contains("@wip"), "{result}");
        assert!(result.contains("authenticated with MFA"), "{result}");
    }

    #[test]
    fn remove_existing_scenario() {
        let rem_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.removed
  Scenario: User sees profile
    Given placeholder
"#;
        let feature = parse_str(rem_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Removed,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(!result.contains("User sees profile"));
        assert!(result.contains("User logs in"));
    }

    #[test]
    fn add_duplicate_fails() {
        let dup_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User logs in
    Given a user
    When they log in
    Then nothing
"#;
        let feature = parse_str(dup_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let err = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn retry_added_identical_is_noop() {
        let retry_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials
    Then they are authenticated
"#;
        let feature = parse_str(retry_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas_retry(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert_eq!(
            result.matches("Scenario: User logs in").count(),
            1,
            "{result}"
        );
    }

    #[test]
    fn retry_added_different_body_fails() {
        let retry_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User logs in
    Given a different user
    When they provide a different credential
    Then a different thing happens
"#;
        let feature = parse_str(retry_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let err = apply_deltas_retry(Some(PERMANENT), &[delta], "Delta").unwrap_err();
        assert!(err.to_string().contains("already exists"), "{err}");
    }

    #[test]
    fn retry_modified_is_idempotent() {
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials and MFA
    Then they are authenticated with MFA
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let first =
            apply_deltas_retry(Some(PERMANENT), std::slice::from_ref(&delta), "Delta").unwrap();
        let second = apply_deltas_retry(Some(&first), &[delta], "Delta").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn retry_removed_absent_is_noop() {
        let retry_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.removed
  Scenario: User was already removed
    Given placeholder
"#;
        let feature = parse_str(retry_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Removed,
        };
        let result = apply_deltas_retry(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("User logs in"), "{result}");
    }

    #[test]
    fn modify_missing_fails() {
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: Non-existent scenario
    Given something
    When it happens
    Then nothing
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let err = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn remove_missing_fails() {
        let rem_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.removed
  Scenario: Non-existent scenario
    Given placeholder
"#;
        let feature = parse_str(rem_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Removed,
        };
        let err = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn tags_are_stripped_in_output() {
        let new_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added @wip @user-auth
  Scenario: New scenario
    Given something
    When it happens
    Then it works
"#;
        let feature = parse_str(new_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(None, &[delta], "Delta").unwrap();
        // @givn.added should be stripped; @wip and @user-auth should remain.
        assert!(!result.contains("givn.added"));
        assert!(result.contains("@wip") || result.contains("wip"));
        // Empty baseline uses the delta feature name as the header.
        assert!(result.contains("Feature: Delta"));
        assert!(!result.contains("(merged)"));
    }

    #[test]
    fn modified_scenario_keeps_step_docstring() {
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: User logs in
    Given a registered user
    When a payload is provided:
      """
      {
        "purpose": "docstring-survives",
        "count": 3
      }
      """
    Then they are authenticated
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("\"purpose\": \"docstring-survives\""));
        assert!(result.contains("\"count\": 3"));
    }

    #[test]
    fn added_scenario_keeps_step_docstring() {
        let new_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User resets password
    Given a user who forgot their password
    And a request payload:
      """
      payload:docstring-survives-added
      """
    Then they receive a reset email
"#;
        let feature = parse_str(new_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("payload:docstring-survives-added"));
    }

    #[test]
    fn empty_docstring_still_emits_delimiters() {
        let new_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User resets password
    Given a user who forgot their password
    And an empty payload:
      """
      """
    Then they receive a reset email
"#;
        let feature = parse_str(new_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("      \"\"\"\n      \"\"\""), "{result}");
    }

    #[test]
    fn modified_scenario_keeps_step_data_table() {
        let mod_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.modified
  Scenario: User logs in
    Given a registered user
    When the mapping is applied:
      | key | value |
      | id  | 42    |
    Then they are authenticated
"#;
        let feature = parse_str(mod_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Modified,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        assert!(result.contains("| key | value |"), "{result}");
        assert!(result.contains("| id | 42 |"), "{result}");
    }

    #[test]
    fn step_with_docstring_and_table_renders_docstring_first() {
        let new_feature_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario: User resets password
    Given a user
    And a combined step:
      """
      payload:both
      """
      | a | b |
    Then it works
"#;
        let feature = parse_str(new_feature_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(Some(PERMANENT), &[delta], "Delta").unwrap();
        let doc_pos = result.find("payload:both").expect("docstring content");
        let table_pos = result.find("| a | b |").expect("table row");
        assert!(doc_pos < table_pos, "{result}");
    }

    #[test]
    fn empty_baseline_uses_delta_feature_name_as_header() {
        let delta_text = r#"
@givn.delta @new-capability
Feature: Brand new capability

  @givn.added @wip
  Scenario: First scenario
    Given something
    When it happens
    Then it works
"#;
        let feature = parse_str(delta_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(None, &[delta], "Brand new capability").unwrap();
        assert!(result.starts_with("Feature: Brand new capability\n"));
        assert!(!result.contains("(merged)"));
        assert!(result.contains("First scenario"));
    }

    #[test]
    fn added_scenario_keeps_outline_examples() {
        let delta_text = r#"
@givn.delta @user-auth
Feature: Delta

  @givn.added
  Scenario Outline: User resets password
    Given a user named <name>
    When they request a reset
    Then they receive a reset email

    Examples: users
      | name |
      | Ada  |
"#;
        let feature = parse_str(delta_text).unwrap();
        let delta = DeltaScenario {
            scenario: &feature.scenarios[0],
            op: DeltaOp::Added,
        };
        let result = apply_deltas(None, &[delta], "Delta").unwrap();
        assert!(
            result.contains("Scenario Outline: User resets password"),
            "{result}"
        );
        assert!(result.contains("Examples: users"), "{result}");
        assert!(result.contains("| name |"), "{result}");
        assert!(result.contains("| Ada |"), "{result}");
    }
}
