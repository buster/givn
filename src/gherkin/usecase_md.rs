//! Strict parser for the active `usecase.md` and `fragment.md` contracts.
//!
//! A use-case document is intentionally Markdown rather than a second data
//! format. The headings and list/table shapes are nevertheless fixed so that
//! discovery, relationship checks, and archive operations share one contract.

use anyhow::{Result, bail};
use serde::Serialize;

pub const HEADING_LEVEL: &str = "## Level";
pub const HEADING_ACTORS: &str = "## Actors";
pub const HEADING_GOAL: &str = "## Goal";
pub const HEADING_TRIGGER: &str = "## Trigger";
pub const HEADING_PRECONDITIONS: &str = "## Preconditions";
pub const HEADING_MAIN_FLOW: &str = "## Main flow";
pub const HEADING_EXTENSIONS: &str = "## Extensions";
pub const HEADING_RULES: &str = "## Rules";
pub const HEADING_EXAMPLES: &str = "## Examples";
pub const HEADING_MINIMAL_GUARANTEE: &str = "## Minimal guarantee";
pub const HEADING_SUCCESS_GUARANTEE: &str = "## Success guarantee";
pub const HEADING_PERSONAS: &str = "## Personas";
pub const HEADING_CAPABILITIES: &str = "## Capabilities";
pub const HEADING_INTERACTIONS: &str = "## Interactions";
pub const HEADING_INCLUDES: &str = "## Includes";
pub const HEADING_EXTENDS: &str = "## Extends";
pub const HEADING_OUT_OF_SCOPE: &str = "## Out of scope";
pub const HEADING_DIAGRAM: &str = "## Diagram";

const USECASE_HEADINGS: &[&str] = &[
    HEADING_LEVEL,
    HEADING_ACTORS,
    HEADING_GOAL,
    HEADING_TRIGGER,
    HEADING_PRECONDITIONS,
    HEADING_MAIN_FLOW,
    HEADING_EXTENSIONS,
    HEADING_RULES,
    HEADING_EXAMPLES,
    HEADING_MINIMAL_GUARANTEE,
    HEADING_SUCCESS_GUARANTEE,
    HEADING_PERSONAS,
    HEADING_CAPABILITIES,
    HEADING_INTERACTIONS,
    HEADING_INCLUDES,
    HEADING_EXTENDS,
    HEADING_OUT_OF_SCOPE,
    HEADING_DIAGRAM,
];

const FRAGMENT_HEADINGS: &[&str] = &[HEADING_CAPABILITIES, HEADING_INTERACTIONS, HEADING_DIAGRAM];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum GoalLevel {
    Leaf,
    Summary,
}

impl GoalLevel {
    fn parse(value: &str) -> Result<Self> {
        match value.trim() {
            "!" => Ok(Self::Leaf),
            "+" => Ok(Self::Summary),
            other => bail!("use-case Level must be '!' or '+', got '{other}'"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RelationshipTarget {
    Usecase(String),
    Fragment(String),
}

impl RelationshipTarget {
    pub fn id(&self) -> &str {
        match self {
            Self::Usecase(id) | Self::Fragment(id) => id,
        }
    }

    pub fn is_fragment(&self) -> bool {
        matches!(self, Self::Fragment(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Interaction {
    pub capability: String,
    pub consumer_action: String,
    pub e2e_scenario: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UsecaseDoc {
    pub id: String,
    pub level: GoalLevel,
    pub actors: Vec<String>,
    pub goal: String,
    pub trigger: String,
    pub preconditions: Vec<String>,
    pub main_flow: String,
    pub extensions: Vec<String>,
    pub rules: Vec<String>,
    pub examples: Vec<String>,
    pub minimal_guarantee: String,
    pub success_guarantee: String,
    pub personas: Vec<String>,
    pub capabilities: Vec<String>,
    pub interactions: Vec<Interaction>,
    pub includes: Vec<RelationshipTarget>,
    pub extends: Vec<RelationshipTarget>,
    pub out_of_scope: String,
    pub diagram: String,
}

impl UsecaseDoc {
    #[allow(dead_code)]
    pub fn child_usecases(&self) -> impl Iterator<Item = &str> {
        self.includes.iter().filter_map(|target| match target {
            RelationshipTarget::Usecase(id) => Some(id.as_str()),
            RelationshipTarget::Fragment(_) => None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FragmentDoc {
    pub id: String,
    pub capabilities: Vec<String>,
    pub interactions: Vec<Interaction>,
    pub diagram: String,
}

/// Parse a strict use-case narrative.
pub fn parse_usecase_md(content: &str) -> Result<UsecaseDoc> {
    let (id, sections) = parse_document(content, "Use case", USECASE_HEADINGS)?;
    let level_text = required_text(&sections, HEADING_LEVEL)?;
    let level = GoalLevel::parse(&level_text)?;
    let capabilities = list_section(&sections, HEADING_CAPABILITIES, "capability")?;
    let interactions = interaction_section(&sections, &capabilities)?;

    if level == GoalLevel::Leaf && capabilities.is_empty() {
        bail!("leaf use case '{id}' must declare at least one capability");
    }

    Ok(UsecaseDoc {
        id,
        level,
        actors: list_section(&sections, HEADING_ACTORS, "actor")?,
        goal: required_text(&sections, HEADING_GOAL)?,
        trigger: required_text(&sections, HEADING_TRIGGER)?,
        preconditions: list_section(&sections, HEADING_PRECONDITIONS, "precondition")?,
        main_flow: required_text(&sections, HEADING_MAIN_FLOW)?,
        extensions: list_section(&sections, HEADING_EXTENSIONS, "extension")?,
        rules: list_section(&sections, HEADING_RULES, "rule")?,
        examples: list_section(&sections, HEADING_EXAMPLES, "example")?,
        minimal_guarantee: required_text(&sections, HEADING_MINIMAL_GUARANTEE)?,
        success_guarantee: required_text(&sections, HEADING_SUCCESS_GUARANTEE)?,
        personas: list_section(&sections, HEADING_PERSONAS, "persona")?,
        capabilities,
        interactions,
        includes: relationship_section(&sections, HEADING_INCLUDES)?,
        extends: relationship_section(&sections, HEADING_EXTENDS)?,
        out_of_scope: required_text(&sections, HEADING_OUT_OF_SCOPE)?,
        diagram: required_text(&sections, HEADING_DIAGRAM)?,
    })
}

/// Parse a reusable fragment narrative. Fragments deliberately have no actor,
/// goal, or Persona sections because they are not independent user goals.
pub fn parse_fragment_md(content: &str) -> Result<FragmentDoc> {
    let (id, sections) = parse_document(content, "Fragment", FRAGMENT_HEADINGS)?;
    let capabilities = list_section(&sections, HEADING_CAPABILITIES, "capability")?;
    if capabilities.is_empty() {
        bail!("fragment '{id}' must declare at least one capability");
    }
    Ok(FragmentDoc {
        id,
        capabilities: capabilities.clone(),
        interactions: interaction_section(&sections, &capabilities)?,
        diagram: required_text(&sections, HEADING_DIAGRAM)?,
    })
}

fn parse_document(
    content: &str,
    kind: &str,
    headings: &[&'static str],
) -> Result<(
    String,
    std::collections::BTreeMap<&'static str, Vec<String>>,
)> {
    let expected_title = format!("# {kind}: ");
    let mut title: Option<String> = None;
    let mut current: Option<&'static str> = None;
    let mut seen_headings = std::collections::BTreeSet::new();
    let mut in_fence = false;
    let mut sections: std::collections::BTreeMap<&'static str, Vec<String>> = headings
        .iter()
        .map(|heading| (*heading, Vec::new()))
        .collect();

    for (line_number, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            if let Some(heading) = current {
                sections
                    .get_mut(heading)
                    .expect("heading exists")
                    .push(line.to_string());
            }
            continue;
        }
        if in_fence {
            if let Some(heading) = current {
                sections
                    .get_mut(heading)
                    .expect("heading exists")
                    .push(line.to_string());
            }
            continue;
        }

        if trimmed.starts_with("# ") {
            if title.is_some() {
                bail!("line {}: duplicate document title", line_number + 1);
            }
            if !trimmed.starts_with(&expected_title) {
                bail!(
                    "line {}: expected '{}<kebab-case-id>', got '{}',",
                    line_number + 1,
                    expected_title,
                    trimmed
                );
            }
            let id = trimmed[expected_title.len()..].trim();
            validate_id(id, kind)?;
            title = Some(id.to_string());
            current = None;
            continue;
        }
        if trimmed.starts_with("## ") {
            let Some(heading) = headings
                .iter()
                .copied()
                .find(|candidate| *candidate == trimmed)
            else {
                bail!("line {}: unknown heading '{trimmed}'", line_number + 1);
            };
            if !seen_headings.insert(heading) {
                bail!("line {}: duplicate heading '{heading}'", line_number + 1);
            }
            current = Some(heading);
            continue;
        }
        if let Some(heading) = current {
            sections
                .get_mut(heading)
                .expect("heading exists")
                .push(line.to_string());
        } else if !trimmed.is_empty() && title.is_some() {
            bail!(
                "line {}: content is outside a recognised section",
                line_number + 1
            );
        }
    }

    let id = title.ok_or_else(|| anyhow::anyhow!("document is missing '# {kind}: <id>'"))?;
    for heading in headings {
        if !seen_headings.contains(heading) {
            bail!("document schema does not recognise '{heading}'");
        }
    }
    Ok((id, sections))
}

fn validate_id(id: &str, kind: &str) -> Result<()> {
    if id.is_empty()
        || id.starts_with('-')
        || id.ends_with('-')
        || id.contains("--")
        || !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        bail!("{kind} ID '{id}' must be a stable kebab-case identifier");
    }
    Ok(())
}

fn required_text(
    sections: &std::collections::BTreeMap<&'static str, Vec<String>>,
    heading: &'static str,
) -> Result<String> {
    let text = sections
        .get(heading)
        .map(|lines| lines.join("\n").trim().to_string())
        .unwrap_or_default();
    if text.is_empty() {
        bail!("section '{heading}' must not be empty");
    }
    Ok(text)
}

fn list_section(
    sections: &std::collections::BTreeMap<&'static str, Vec<String>>,
    heading: &'static str,
    noun: &str,
) -> Result<Vec<String>> {
    let mut values = Vec::new();
    for line in sections.get(heading).expect("known heading") {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(value) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        else {
            bail!("section '{heading}' must contain one {noun} per bullet");
        };
        let value = value.trim();
        if !value.is_empty() && !value.eq_ignore_ascii_case("none") {
            values.push(value.to_string());
        }
    }
    Ok(values)
}

fn relationship_section(
    sections: &std::collections::BTreeMap<&'static str, Vec<String>>,
    heading: &'static str,
) -> Result<Vec<RelationshipTarget>> {
    let mut values = Vec::new();
    for line in sections.get(heading).expect("known heading") {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("- none") {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("- ") else {
            bail!("section '{heading}' must contain typed relationship bullets");
        };
        let Some((kind, id)) = value.split_once(':') else {
            bail!(
                "section '{heading}' relationship '{value}' must use 'usecase: id' or 'fragment: id'"
            );
        };
        let id = id.trim();
        validate_id(id, "relationship target")?;
        let target = match kind.trim() {
            "usecase" => RelationshipTarget::Usecase(id.to_string()),
            "fragment" => RelationshipTarget::Fragment(id.to_string()),
            other => bail!("section '{heading}' has unknown relationship target kind '{other}'"),
        };
        if values.contains(&target) {
            bail!(
                "section '{heading}' contains duplicate relationship '{}'",
                target.id()
            );
        }
        values.push(target);
    }
    Ok(values)
}

fn interaction_section(
    sections: &std::collections::BTreeMap<&'static str, Vec<String>>,
    capabilities: &[String],
) -> Result<Vec<Interaction>> {
    let lines = sections.get(HEADING_INTERACTIONS).expect("known heading");
    let rows: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|line| !line.trim().is_empty())
        .collect();
    if rows.len() == 1 && rows[0].trim().eq_ignore_ascii_case("- none") {
        return Ok(Vec::new());
    }
    if rows.len() < 2 {
        bail!("section '{HEADING_INTERACTIONS}' must contain a Markdown table");
    }
    let header = split_table_row(rows[0])?;
    if header != ["Capability", "Consumer action", "E2E scenario"] {
        bail!(
            "section '{HEADING_INTERACTIONS}' must use columns Capability, Consumer action, E2E scenario"
        );
    }
    if split_table_row(rows[1])?.iter().any(|cell| {
        !cell
            .chars()
            .all(|c| c == '-' || c == ':' || c.is_whitespace())
    }) {
        bail!("section '{HEADING_INTERACTIONS}' is missing its Markdown separator row");
    }
    let mut interactions = Vec::new();
    for row in &rows[2..] {
        let cells = split_table_row(row)?;
        if cells.len() != 3 || cells.iter().any(String::is_empty) {
            bail!("section '{HEADING_INTERACTIONS}' contains a malformed table row");
        }
        if !capabilities
            .iter()
            .any(|capability| capability == &cells[0])
        {
            bail!(
                "interaction references undeclared capability '{}'",
                cells[0]
            );
        }
        interactions.push(Interaction {
            capability: cells[0].clone(),
            consumer_action: cells[1].clone(),
            e2e_scenario: cells[2].clone(),
        });
    }
    Ok(interactions)
}

fn split_table_row(line: &str) -> Result<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        bail!("interaction table row must start and end with '|'");
    }
    Ok(trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect())
}

#[allow(dead_code)]
pub fn usecase_md_template() -> String {
    r#"# Use case: <usecase-id>

## Level

!

## Actors

- <actor role>

## Goal

<the domain goal>

## Trigger

<what starts this use case>

## Preconditions

- <precondition>

## Main flow

1. <first step>

## Extensions

- none

## Rules

- <domain rule>

## Examples

- <concrete example>

## Minimal guarantee

<what remains true when the goal cannot be completed>

## Success guarantee

<what is true when the goal succeeds>

## Personas

- none

## Capabilities

- <capability-id>

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| <capability-id> | <consumer action> | <scenario title> |

## Includes

- none

## Extends

- none

## Out of scope

<what this use case deliberately does not cover>

## Diagram

```mermaid
flowchart LR
  Actor((Actor)) --> UC1[[Use case]]
```
"#
    .to_string()
}

#[allow(dead_code)]
pub fn fragment_md_template() -> String {
    r#"# Fragment: <fragment-id>

## Capabilities

- <capability-id>

## Interactions

- none

## Diagram

```mermaid
flowchart LR
  Consumer((Consumer)) --> Fragment[[Fragment]]
```
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_LEAF: &str = r#"# Use case: start-project

## Level

!

## Actors

- Project operator

## Goal

Start a project run.

## Trigger

The operator requests a run.

## Preconditions

- The project is configured.

## Main flow

1. The operator starts the run.

## Extensions

- none

## Rules

- A run has one authority.

## Examples

- A clean project starts successfully.

## Minimal guarantee

The project remains inspectable.

## Success guarantee

The run is handed to its authority.

## Personas

- none

## Capabilities

- run-project

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| run-project | start a project run | Consumer starts a project run |

## Includes

- fragment: transport

## Extends

- usecase: archive-project

## Out of scope

Scheduling future runs.

## Diagram

```mermaid
flowchart LR
  Actor((Operator)) --> UC1[[Start project]]
```
"#;

    #[test]
    fn parses_leaf_use_case_and_typed_relationships() {
        let doc = parse_usecase_md(VALID_LEAF).unwrap();
        assert_eq!(doc.id, "start-project");
        assert_eq!(doc.level, GoalLevel::Leaf);
        assert_eq!(doc.capabilities, vec!["run-project"]);
        assert_eq!(doc.interactions[0].capability, "run-project");
        assert_eq!(
            doc.includes,
            vec![RelationshipTarget::Fragment("transport".into())]
        );
        assert_eq!(
            doc.extends,
            vec![RelationshipTarget::Usecase("archive-project".into())]
        );
    }

    #[test]
    fn parses_summary_use_case_without_direct_capabilities() {
        let text = VALID_LEAF
            .replace("# Use case: start-project", "# Use case: project-lifecycle")
            .replace("\n!\n", "\n+\n")
            .replace("- run-project\n", "- none\n")
            .replace(
                "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| run-project | start a project run | Consumer starts a project run |",
                "- none",
            )
            .replace("- fragment: transport", "- usecase: start-project");
        let doc = parse_usecase_md(&text).unwrap();
        assert_eq!(doc.level, GoalLevel::Summary);
        assert!(doc.capabilities.is_empty());
        assert_eq!(
            doc.child_usecases().collect::<Vec<_>>(),
            vec!["start-project"]
        );
    }

    #[test]
    fn rejects_unknown_heading_without_bleeding_content() {
        let text = VALID_LEAF.replace("## Goal", "## Unknown\n\nwrong\n\n## Goal");
        let error = parse_usecase_md(&text).unwrap_err();
        assert!(error.to_string().contains("unknown heading '## Unknown'"));
    }

    #[test]
    fn rejects_untyped_relationships_and_invalid_identity() {
        let text = VALID_LEAF
            .replace("# Use case: start-project", "# Use case: Start Project")
            .replace("- fragment: transport", "- transport");
        let error = parse_usecase_md(&text).unwrap_err();
        assert!(error.to_string().contains("stable kebab-case"));
    }

    #[test]
    fn fragment_schema_is_distinct() {
        let text = r#"# Fragment: transport

## Capabilities

- send-message

## Interactions

- none

## Diagram

```mermaid
flowchart LR
  A --> B
```
"#;
        let fragment = parse_fragment_md(text).unwrap();
        assert_eq!(fragment.id, "transport");
        assert_eq!(fragment.capabilities, vec!["send-message"]);
        assert!(parse_usecase_md(text).is_err());
    }

    #[test]
    fn templates_parse_with_their_own_headings() {
        let template = usecase_md_template()
            .replace("<usecase-id>", "example-usecase")
            .replace("<capability-id>", "example-capability")
            .replace("<scenario title>", "Consumer uses capability");
        assert!(parse_usecase_md(&template).is_ok());
        let fragment = fragment_md_template()
            .replace("<fragment-id>", "example-fragment")
            .replace("<capability-id>", "example-capability");
        assert!(parse_fragment_md(&fragment).is_ok());
    }
}
