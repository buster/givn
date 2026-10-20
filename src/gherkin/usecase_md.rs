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

/// One parser diagnostic. `line` is `None` for a document-level problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsecaseDiagnostic {
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}

impl UsecaseDiagnostic {
    fn at(line: usize, message: impl Into<String>) -> Self {
        Self {
            line: Some(line),
            message: message.into(),
            suggestion: None,
        }
    }

    fn document(message: impl Into<String>) -> Self {
        Self {
            line: None,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn rendered(&self) -> String {
        let mut text = match self.line {
            Some(line) => format!("line {line}: {}", self.message),
            None => self.message.clone(),
        };
        if let Some(suggestion) = &self.suggestion {
            text.push_str(&format!(" — did you mean '{suggestion}'?"));
        }
        text
    }
}

/// The complete diagnostic set for one document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UsecaseDiagnostics(pub Vec<UsecaseDiagnostic>);

impl UsecaseDiagnostics {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push(&mut self, diagnostic: UsecaseDiagnostic) {
        if !self.0.contains(&diagnostic) {
            self.0.push(diagnostic);
        }
    }

    fn sort(&mut self) {
        self.0.sort_by(|a, b| {
            a.line
                .cmp(&b.line)
                .then_with(|| a.message.cmp(&b.message))
                .then_with(|| a.suggestion.cmp(&b.suggestion))
        });
    }
}

impl std::fmt::Display for UsecaseDiagnostics {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut diagnostics = self.clone();
        diagnostics.sort();
        let rendered = diagnostics
            .0
            .iter()
            .map(UsecaseDiagnostic::rendered)
            .collect::<Vec<_>>()
            .join("\n");
        formatter.write_str(&rendered)
    }
}

type SectionLines = std::collections::BTreeMap<&'static str, Vec<(usize, String)>>;

/// Parse a strict use-case narrative, returning every diagnostic.
pub fn parse_usecase_md_diagnostics(content: &str) -> (Option<UsecaseDoc>, UsecaseDiagnostics) {
    let (title, sections, seen, mut diagnostics) =
        parse_document(content, "Use case", USECASE_HEADINGS);

    let id = title.clone().unwrap_or_default();
    let level_text = required_text(&sections, HEADING_LEVEL, &mut diagnostics);
    let level = match GoalLevel::parse(&level_text) {
        Ok(level) => level,
        Err(error) => {
            diagnostics.push(UsecaseDiagnostic::document(error.to_string()));
            GoalLevel::Leaf
        }
    };
    let capabilities = list_section(
        &sections,
        HEADING_CAPABILITIES,
        "capability",
        &mut diagnostics,
    );
    let interactions = interaction_section(&sections, &capabilities, &mut diagnostics);

    if level == GoalLevel::Leaf && capabilities.is_empty() {
        diagnostics.push(UsecaseDiagnostic::document(format!(
            "leaf use case '{id}' must declare at least one capability"
        )));
    }

    let doc = if title.is_some()
        && seen.contains(HEADING_CAPABILITIES)
        && seen.contains(HEADING_INTERACTIONS)
    {
        Some(UsecaseDoc {
            id,
            level,
            actors: list_section(&sections, HEADING_ACTORS, "actor", &mut diagnostics),
            goal: required_text(&sections, HEADING_GOAL, &mut diagnostics),
            trigger: required_text(&sections, HEADING_TRIGGER, &mut diagnostics),
            preconditions: list_section(
                &sections,
                HEADING_PRECONDITIONS,
                "precondition",
                &mut diagnostics,
            ),
            main_flow: required_text(&sections, HEADING_MAIN_FLOW, &mut diagnostics),
            extensions: list_section(&sections, HEADING_EXTENSIONS, "extension", &mut diagnostics),
            rules: list_section(&sections, HEADING_RULES, "rule", &mut diagnostics),
            examples: list_section(&sections, HEADING_EXAMPLES, "example", &mut diagnostics),
            minimal_guarantee: required_text(
                &sections,
                HEADING_MINIMAL_GUARANTEE,
                &mut diagnostics,
            ),
            success_guarantee: required_text(
                &sections,
                HEADING_SUCCESS_GUARANTEE,
                &mut diagnostics,
            ),
            personas: list_section(&sections, HEADING_PERSONAS, "persona", &mut diagnostics),
            capabilities,
            interactions,
            includes: relationship_section(&sections, HEADING_INCLUDES, &mut diagnostics),
            extends: relationship_section(&sections, HEADING_EXTENDS, &mut diagnostics),
            out_of_scope: required_text(&sections, HEADING_OUT_OF_SCOPE, &mut diagnostics),
            diagram: required_text(&sections, HEADING_DIAGRAM, &mut diagnostics),
        })
    } else {
        None
    };

    diagnostics.sort();
    (doc, diagnostics)
}

/// Parse a strict use-case narrative, formatting every diagnostic on failure.
pub fn parse_usecase_md(content: &str) -> Result<UsecaseDoc> {
    let (doc, diagnostics) = parse_usecase_md_diagnostics(content);
    if !diagnostics.is_empty() {
        bail!("{diagnostics}");
    }
    doc.ok_or_else(|| anyhow::anyhow!("use-case document could not be parsed"))
}

/// Parse a fragment narrative, returning every diagnostic.
pub fn parse_fragment_md_diagnostics(content: &str) -> (Option<FragmentDoc>, UsecaseDiagnostics) {
    let (title, sections, seen, mut diagnostics) =
        parse_document(content, "Fragment", FRAGMENT_HEADINGS);

    let id = title.clone().unwrap_or_default();
    let capabilities = list_section(
        &sections,
        HEADING_CAPABILITIES,
        "capability",
        &mut diagnostics,
    );
    if capabilities.is_empty() {
        diagnostics.push(UsecaseDiagnostic::document(format!(
            "fragment '{id}' must declare at least one capability"
        )));
    }
    let interactions = interaction_section(&sections, &capabilities, &mut diagnostics);

    let doc = if title.is_some()
        && seen.contains(HEADING_CAPABILITIES)
        && seen.contains(HEADING_INTERACTIONS)
    {
        Some(FragmentDoc {
            id,
            capabilities: capabilities.clone(),
            interactions,
            diagram: required_text(&sections, HEADING_DIAGRAM, &mut diagnostics),
        })
    } else {
        None
    };

    diagnostics.sort();
    (doc, diagnostics)
}

/// Parse a fragment narrative, formatting every diagnostic on failure.
pub fn parse_fragment_md(content: &str) -> Result<FragmentDoc> {
    let (doc, diagnostics) = parse_fragment_md_diagnostics(content);
    if !diagnostics.is_empty() {
        bail!("{diagnostics}");
    }
    doc.ok_or_else(|| anyhow::anyhow!("fragment document could not be parsed"))
}

fn parse_document(
    content: &str,
    kind: &str,
    headings: &[&'static str],
) -> (
    Option<String>,
    SectionLines,
    std::collections::BTreeSet<&'static str>,
    UsecaseDiagnostics,
) {
    let expected_title = format!("# {kind}: ");
    let mut title: Option<String> = None;
    let mut current: Option<&'static str> = None;
    let mut seen_headings = std::collections::BTreeSet::new();
    let mut sections: SectionLines = headings
        .iter()
        .map(|heading| (*heading, Vec::new()))
        .collect();
    let mut diagnostics = UsecaseDiagnostics::default();
    let mut in_fence = false;
    let mut ignoring_section = false;
    let mut outside_block_reported = false;

    for (line_number, line) in content.lines().enumerate() {
        let number = line_number + 1;
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            if !ignoring_section
                && let Some(heading) = current
                && let Some(lines) = sections.get_mut(heading)
            {
                lines.push((number, line.to_string()));
            }
            continue;
        }
        if in_fence {
            if !ignoring_section
                && let Some(heading) = current
                && let Some(lines) = sections.get_mut(heading)
            {
                lines.push((number, line.to_string()));
            }
            continue;
        }

        if trimmed.starts_with("# ") {
            current = None;
            ignoring_section = false;
            outside_block_reported = false;
            if title.is_some() {
                diagnostics.push(UsecaseDiagnostic::at(number, "duplicate document title"));
                continue;
            }
            if trimmed.starts_with(&expected_title) {
                let id = trimmed[expected_title.len()..].trim().to_string();
                if let Err(error) = validate_id(&id, kind) {
                    diagnostics.push(UsecaseDiagnostic::at(number, error.to_string()));
                }
                title = Some(id);
            } else {
                let suggestion = recover_title(&expected_title, trimmed);
                let message = format!(
                    "expected '{}<kebab-case-id>', got '{}'; headings are case-sensitive",
                    expected_title, trimmed
                );
                let mut diagnostic = UsecaseDiagnostic::at(number, message);
                if let Some(recovered) = suggestion.clone() {
                    diagnostic.suggestion = Some(recovered);
                    title = Some(trimmed[expected_title.len()..].trim().to_string());
                }
                diagnostics.push(diagnostic);
            }
            continue;
        }

        if trimmed.starts_with("## ") {
            current = None;
            outside_block_reported = false;
            if let Some(heading) = headings
                .iter()
                .copied()
                .find(|candidate| *candidate == trimmed)
            {
                if seen_headings.insert(heading) {
                    current = Some(heading);
                    ignoring_section = false;
                } else {
                    diagnostics.push(UsecaseDiagnostic::at(
                        number,
                        format!("duplicate heading '{heading}'"),
                    ));
                    ignoring_section = true;
                }
            } else {
                let suggestion = suggest_heading(trimmed, headings);
                let mut message = format!("unknown heading '{trimmed}'");
                if suggestion.is_none() {
                    message.push_str("; expected one of ");
                    message.push_str(
                        &headings
                            .iter()
                            .map(|heading| format!("'{heading}'"))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                } else if trimmed.eq_ignore_ascii_case("## Guarantees") {
                    message.push_str(
                        "; expected one of '## Minimal guarantee', '## Success guarantee'",
                    );
                }
                let mut diagnostic = UsecaseDiagnostic::at(number, message);
                diagnostic.suggestion = suggestion;
                diagnostics.push(diagnostic);
                ignoring_section = true;
            }
            continue;
        }

        if ignoring_section {
            continue;
        }
        if let Some(heading) = current {
            if let Some(lines) = sections.get_mut(heading) {
                lines.push((number, line.to_string()));
            }
        } else if !trimmed.is_empty() && title.is_some() && !outside_block_reported {
            diagnostics.push(UsecaseDiagnostic::at(
                number,
                "content is outside a recognised section",
            ));
            outside_block_reported = true;
        }
    }

    if title.is_none() {
        diagnostics.push(UsecaseDiagnostic::document(format!(
            "document is missing '# {kind}: <id>'"
        )));
    }
    for heading in headings {
        if !seen_headings.contains(heading) {
            diagnostics.push(UsecaseDiagnostic::document(format!(
                "document schema does not recognise '{heading}'"
            )));
        }
    }

    (title, sections, seen_headings, diagnostics)
}

fn recover_title(expected_title: &str, line: &str) -> Option<String> {
    if line.len() >= expected_title.len()
        && line[..expected_title.len()].eq_ignore_ascii_case(expected_title)
    {
        let id = line[expected_title.len()..].trim();
        Some(format!("{expected_title}{id}"))
    } else {
        None
    }
}

fn suggest_heading(unknown: &str, headings: &[&'static str]) -> Option<String> {
    if unknown.eq_ignore_ascii_case("## Guarantees") {
        return Some("## Minimal guarantee' or '## Success guarantee".to_string());
    }
    headings
        .iter()
        .copied()
        .find(|heading| heading.eq_ignore_ascii_case(unknown))
        .map(str::to_string)
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
    sections: &SectionLines,
    heading: &'static str,
    diagnostics: &mut UsecaseDiagnostics,
) -> String {
    let text = sections
        .get(heading)
        .map(|lines| {
            lines
                .iter()
                .map(|(_, line)| line.as_str())
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string()
        })
        .unwrap_or_default();
    if text.is_empty() {
        diagnostics.push(UsecaseDiagnostic::document(format!(
            "section '{heading}' must not be empty"
        )));
    }
    text
}

fn list_section(
    sections: &SectionLines,
    heading: &'static str,
    noun: &str,
    diagnostics: &mut UsecaseDiagnostics,
) -> Vec<String> {
    let mut values = Vec::new();
    for (line_number, line) in sections.get(heading).expect("known heading") {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(value) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        else {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!("section '{heading}' must contain one {noun} per bullet: '{trimmed}'"),
            ));
            continue;
        };
        let value = value.trim();
        if !value.is_empty() && !value.eq_ignore_ascii_case("none") {
            values.push(value.to_string());
        }
    }
    values
}

fn relationship_section(
    sections: &SectionLines,
    heading: &'static str,
    diagnostics: &mut UsecaseDiagnostics,
) -> Vec<RelationshipTarget> {
    let mut values = Vec::new();
    for (line_number, line) in sections.get(heading).expect("known heading") {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("- none") {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("- ") else {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!("section '{heading}' must contain typed relationship bullets"),
            ));
            continue;
        };
        let Some((kind, id)) = value.split_once(':') else {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!(
                    "section '{heading}' relationship '{value}' must use 'usecase: id' or 'fragment: id'"
                ),
            ));
            continue;
        };
        let id = id.trim();
        if let Err(error) = validate_id(id, "relationship target") {
            diagnostics.push(UsecaseDiagnostic::at(*line_number, error.to_string()));
            continue;
        }
        let target = match kind.trim() {
            "usecase" => RelationshipTarget::Usecase(id.to_string()),
            "fragment" => RelationshipTarget::Fragment(id.to_string()),
            other => {
                diagnostics.push(UsecaseDiagnostic::at(
                    *line_number,
                    format!("section '{heading}' has unknown relationship target kind '{other}'"),
                ));
                continue;
            }
        };
        if values.contains(&target) {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!(
                    "section '{heading}' contains duplicate relationship '{}'",
                    target.id()
                ),
            ));
            continue;
        }
        values.push(target);
    }
    values
}

fn interaction_section(
    sections: &SectionLines,
    capabilities: &[String],
    diagnostics: &mut UsecaseDiagnostics,
) -> Vec<Interaction> {
    let lines = sections.get(HEADING_INTERACTIONS).expect("known heading");
    let rows: Vec<(usize, &str)> = lines
        .iter()
        .map(|(number, line)| (*number, line.as_str()))
        .filter(|(_, line)| !line.trim().is_empty())
        .collect();
    if rows.len() == 1 && rows[0].1.trim().eq_ignore_ascii_case("- none") {
        return Vec::new();
    }
    if rows.len() < 2 {
        diagnostics.push(UsecaseDiagnostic::document(format!(
            "section '{HEADING_INTERACTIONS}' must contain a Markdown table"
        )));
        return Vec::new();
    }
    let (header_line, header_text) = rows[0];
    let header = match split_table_row(header_text) {
        Ok(header) => header,
        Err(error) => {
            diagnostics.push(UsecaseDiagnostic::at(header_line, error.to_string()));
            return Vec::new();
        }
    };
    if header != ["Capability", "Consumer action", "E2E scenario"] {
        diagnostics.push(UsecaseDiagnostic::at(
            header_line,
            format!(
                "section '{HEADING_INTERACTIONS}' must use columns Capability, Consumer action, E2E scenario"
            ),
        ));
        return Vec::new();
    }
    let (separator_line, separator_text) = rows[1];
    match split_table_row(separator_text) {
        Ok(cells) if cells.iter().all(|cell| is_separator_cell(cell)) && cells.len() == 3 => {}
        Ok(_) => {
            diagnostics.push(UsecaseDiagnostic::at(
                separator_line,
                format!("section '{HEADING_INTERACTIONS}' has a malformed separator row"),
            ));
            return Vec::new();
        }
        Err(error) => {
            diagnostics.push(UsecaseDiagnostic::at(separator_line, error.to_string()));
            return Vec::new();
        }
    }

    let mut interactions = Vec::new();
    for (line_number, line) in &rows[2..] {
        let cells = match split_table_row(line) {
            Ok(cells) => cells,
            Err(error) => {
                diagnostics.push(UsecaseDiagnostic::at(*line_number, error.to_string()));
                continue;
            }
        };
        if cells.len() != 3 || cells.iter().any(String::is_empty) {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!("section '{HEADING_INTERACTIONS}' contains a malformed table row"),
            ));
            continue;
        }
        if !capabilities
            .iter()
            .any(|capability| capability == &cells[0])
        {
            diagnostics.push(UsecaseDiagnostic::at(
                *line_number,
                format!(
                    "interaction references undeclared capability '{}'",
                    cells[0]
                ),
            ));
            continue;
        }
        interactions.push(Interaction {
            capability: cells[0].clone(),
            consumer_action: cells[1].clone(),
            e2e_scenario: cells[2].clone(),
        });
    }
    interactions
}

fn is_separator_cell(cell: &str) -> bool {
    !cell.is_empty()
        && cell
            .chars()
            .all(|c| c == '-' || c == ':' || c.is_whitespace())
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

- <actor role> (the first actor is the primary actor; mark additional actors `Supporting:`)

## Goal

<the domain goal>

A leaf whose flow names three or more distinct actor goals, or whose success
guarantee is a conjunction, must be promoted to `+` and split.

## Trigger

<what starts this use case; an actor action or event, never a state or desire>

## Preconditions

- <precondition that holds before the trigger>

## Main flow

1. <actor intention>; System responsibility: <what the system does>
2. <next actor intention>; System responsibility: <what the system does>

## Extensions

- <step>: <condition> → <handling>; protects <guarantee>

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
    fn case_mismatched_title_recovers_and_suggests_the_exact_spelling() {
        let text = VALID_LEAF.replace("# Use case: start-project", "# Use Case: start-project");
        let (doc, diagnostics) = parse_usecase_md_diagnostics(&text);
        let doc = doc.expect("a case-mismatched title is recoverable");
        assert_eq!(doc.id, "start-project");
        let rendered = diagnostics.to_string();
        assert!(
            rendered.contains("headings are case-sensitive"),
            "{rendered}"
        );
        assert!(
            rendered.contains("did you mean '# Use case: start-project'?"),
            "{rendered}"
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
