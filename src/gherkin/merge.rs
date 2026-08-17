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
use anyhow::Result;
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
    // Parse the permanent feature (or start from an empty scenario list).
    let mut perm_scenarios: Vec<ScenarioText> = match permanent_text {
        Some(text) if !text.trim().is_empty() => parse_scenarios_from_text(text)?,
        _ => vec![],
    };

    let feature_header = match permanent_text {
        Some(text) if !text.trim().is_empty() => extract_feature_header(text),
        _ => format!("Feature: {}\n", delta_feature_name),
    };

    // Validate all preconditions BEFORE making any changes (all-or-nothing).
    for delta in deltas {
        let title = &delta.scenario.name;
        let exists = perm_scenarios.iter().any(|s| s.title == *title);
        match delta.op {
            DeltaOp::Added => {
                if exists {
                    return Err(GivnError::MergeError(format!(
                        "cannot add scenario '{}': title already exists in the permanent spec",
                        title
                    ))
                    .into());
                }
            }
            DeltaOp::Modified | DeltaOp::Removed => {
                if !exists {
                    return Err(GivnError::MergeError(format!(
                        "cannot {:?} scenario '{}': title not found in the permanent spec",
                        delta.op, title
                    ))
                    .into());
                }
            }
        }
    }

    // Apply all changes.
    for delta in deltas {
        let title = &delta.scenario.name;
        match delta.op {
            DeltaOp::Added => {
                perm_scenarios.push(ScenarioText {
                    title: title.clone(),
                    text: render_scenario_stripped(delta.scenario),
                });
            }
            DeltaOp::Modified => {
                if let Some(s) = perm_scenarios.iter_mut().find(|s| s.title == *title) {
                    s.text = render_scenario_stripped(delta.scenario);
                }
            }
            DeltaOp::Removed => {
                perm_scenarios.retain(|s| s.title != *title);
            }
        }
    }

    // Reassemble the file.
    let mut output = feature_header;
    for scenario in &perm_scenarios {
        output.push('\n');
        output.push_str(&scenario.text);
    }
    if !output.ends_with('\n') {
        output.push('\n');
    }

    Ok(output)
}

/// Merge a delta feature file into a permanent spec file on disk.
/// All-or-nothing: validates before writing.
pub fn merge_file(permanent_path: &std::path::Path, delta_feature: &Feature) -> Result<()> {
    use crate::gherkin::delta::{delta_scenarios, is_delta_feature};

    if !is_delta_feature(delta_feature) {
        return Err(GivnError::MergeError(format!(
            "feature '{}' is not tagged @givn.delta — cannot merge",
            delta_feature.name
        ))
        .into());
    }

    let deltas = delta_scenarios(delta_feature);
    let permanent_text = if permanent_path.exists() {
        Some(std::fs::read_to_string(permanent_path)?)
    } else {
        None
    };

    let new_text = apply_deltas(permanent_text.as_deref(), &deltas, &delta_feature.name)?;

    // Ensure parent directory exists.
    if let Some(parent) = permanent_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(permanent_path, new_text)?;
    Ok(())
}

// ─── Internal helpers ────────────────────────────────────────────────────────

/// A scenario entry in the permanent spec (title + raw text block).
#[derive(Debug, Clone)]
struct ScenarioText {
    title: String,
    text: String,
}

/// Parse the permanent feature text into a list of scenario text blocks.
/// This is a simple line-based parser that splits on "Scenario:" / "Scenario Outline:" lines.
fn parse_scenarios_from_text(text: &str) -> Result<Vec<ScenarioText>> {
    let mut scenarios = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Scenario:") || trimmed.starts_with("Scenario Outline:") {
            // Save previous scenario.
            if let Some(title) = current_title.take() {
                scenarios.push(ScenarioText {
                    title,
                    text: current_lines.join("\n"),
                });
                current_lines.clear();
            }
            // Extract title.
            let title = trimmed
                .split_once(':')
                .map(|x| x.1)
                .unwrap_or("")
                .trim()
                .to_string();
            current_title = Some(title);
        }
        if current_title.is_some() {
            current_lines.push(line.to_string());
        }
    }
    // Flush the last scenario.
    if let Some(title) = current_title {
        scenarios.push(ScenarioText {
            title,
            text: current_lines.join("\n"),
        });
    }

    Ok(scenarios)
}

/// Extract everything before the first Scenario: line (the feature header).
fn extract_feature_header(text: &str) -> String {
    let mut header = Vec::new();
    let mut feature_seen = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Scenario:") || trimmed.starts_with("Scenario Outline:") {
            break;
        }
        // A tag after the Feature line belongs to the first scenario. Keep
        // feature-level tags before Feature, but do not carry stale
        // first-scenario tags into a modified merge target.
        if feature_seen && trimmed.starts_with('@') {
            continue;
        }
        header.push(line);
        if trimmed.starts_with("Feature:") {
            feature_seen = true;
        }
    }
    // Trim trailing blank lines from header.
    while header
        .last()
        .map(|l: &&str| l.trim().is_empty())
        .unwrap_or(false)
    {
        header.pop();
    }
    if header.is_empty() {
        String::new()
    } else {
        format!("{}\n", header.join("\n"))
    }
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

    // Steps.
    for step in &scenario.steps {
        out.push_str(&format!("    {} {}\n", step.keyword, step.value));

        // Docstring (the crate stores content already dedented, with the
        // newline that follows the opening delimiter still present).
        if let Some(doc) = step.docstring() {
            out.push_str("      \"\"\"\n");
            let content = doc.strip_prefix('\n').unwrap_or(doc.as_str());
            if !content.is_empty() {
                for line in content.lines() {
                    if line.is_empty() {
                        out.push('\n');
                    } else {
                        out.push_str(&format!("      {line}\n"));
                    }
                }
            }
            out.push_str("      \"\"\"\n");
        }

        // Data table.
        if let Some(table) = step.table() {
            for row in &table.rows {
                let cells: Vec<&str> = row.iter().map(|c| c.as_str()).collect();
                out.push_str(&format!("      | {} |\n", cells.join(" | ")));
            }
        }
    }

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
                let cells: Vec<&str> = row.iter().map(|cell| cell.as_str()).collect();
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
