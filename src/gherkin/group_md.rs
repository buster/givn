//! Parser for `givn/specs/<group>/group.md` — the per-group use-case
//! narrative that survives merges (it is never merged into a `.feature`
//! file).
//!
//! Deliberately simple: exact heading matching on the heading strings
//! below, then line-based body capture. The heading strings in
//! [`HEADING_*`] constants are the single source of truth — any generator
//! or instruction text that tells an agent what to write must copy them
//! from here, never retype them (see the disposition-parser heading
//! mismatch that shipped undetected for this exact reason).

use anyhow::{Result, bail};

/// Exact heading that opens the Actor section.
pub const HEADING_ACTOR: &str = "## Actor";
/// Exact heading that opens the Goal section.
pub const HEADING_GOAL: &str = "## Goal";
/// Exact heading that opens the Main flow section.
pub const HEADING_MAIN_FLOW: &str = "## Main flow";
/// Exact heading that opens the Interactions section (the Phase 3 gate
/// reads this one).
pub const HEADING_INTERACTIONS: &str = "## Interactions";
/// Exact heading that opens the Includes section.
pub const HEADING_INCLUDES: &str = "## Includes";
/// Exact heading that opens the Extends section.
pub const HEADING_EXTENDS: &str = "## Extends";
/// Exact heading that opens the Out of scope section.
pub const HEADING_OUT_OF_SCOPE: &str = "## Out of scope";
/// Exact heading that opens the Diagram section.
pub const HEADING_DIAGRAM: &str = "## Diagram";

/// All recognised headings.
const ALL_HEADINGS: &[&str] = &[
    HEADING_ACTOR,
    HEADING_GOAL,
    HEADING_MAIN_FLOW,
    HEADING_INTERACTIONS,
    HEADING_INCLUDES,
    HEADING_EXTENDS,
    HEADING_OUT_OF_SCOPE,
    HEADING_DIAGRAM,
];

/// The parsed content of a `group.md`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroupDoc {
    pub actor: String,
    pub goal: String,
    pub main_flow: String,
    pub interactions: Vec<String>,
    pub includes: Vec<String>,
    pub extends: Vec<String>,
    pub out_of_scope: String,
    pub diagram: String,
}

/// Parse a `group.md` document. Returns an error when a required section
/// heading is missing — "no interactions declared" and "malformed file"
/// must be distinguishable, so a missing `## Interactions` is an error,
/// not a silent empty list.
pub fn parse_group_md(content: &str) -> Result<GroupDoc> {
    let mut bodies: Vec<(&'static str, Vec<String>)> =
        ALL_HEADINGS.iter().map(|h| (*h, Vec::new())).collect();
    let mut current: Option<&'static str> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = exact_heading(trimmed) {
            current = Some(heading);
            continue;
        }
        if let Some(h) = current {
            bodies
                .iter_mut()
                .find(|(name, _)| *name == h)
                .map(|(_, body)| body.push(line.to_string()))
                .expect("heading present in bodies");
        }
    }

    let seen = |heading: &str| content.lines().any(|l| l.trim() == heading);
    for required in [HEADING_ACTOR, HEADING_GOAL, HEADING_INTERACTIONS] {
        if !seen(required) {
            if required == HEADING_INTERACTIONS {
                bail!(
                    "group.md is missing the '{HEADING_INTERACTIONS}' heading — \
                     declare the group's distinct user actions there, one bullet each, \
                     or write '- none' if the group declares no interactions"
                );
            }
            bail!("group.md is missing the '{required}' heading");
        }
    }

    Ok(GroupDoc {
        actor: text_of(body_of(&bodies, HEADING_ACTOR)),
        goal: text_of(body_of(&bodies, HEADING_GOAL)),
        main_flow: text_of(body_of(&bodies, HEADING_MAIN_FLOW)),
        interactions: list_of(body_of(&bodies, HEADING_INTERACTIONS)),
        includes: list_of(body_of(&bodies, HEADING_INCLUDES)),
        extends: list_of(body_of(&bodies, HEADING_EXTENDS)),
        out_of_scope: text_of(body_of(&bodies, HEADING_OUT_OF_SCOPE)),
        diagram: text_of(body_of(&bodies, HEADING_DIAGRAM)),
    })
}

fn body_of<'a>(bodies: &'a [(&'static str, Vec<String>)], heading: &str) -> &'a [String] {
    &bodies
        .iter()
        .find(|(name, _)| *name == heading)
        .expect("heading present in bodies")
        .1
}

fn text_of(lines: &[String]) -> String {
    lines.join("\n").trim().to_string()
}

fn list_of(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|l| {
            let t = l.trim();
            t.strip_prefix("- ")
                .or_else(|| t.strip_prefix("* "))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        // The literal "none" is the documented "nothing here" sentinel,
        // not an item — drop it so an empty list and "- none" agree.
        .filter(|s| !s.eq_ignore_ascii_case("none"))
        .collect()
}

/// The exact heading a line must be (after trimming) to be recognised.
/// Prefix-matching headings (like `starts_with`) is what let the shipped
/// instruction/template heading names drift apart in disposition.rs —
/// here the strings must match byte-for-byte after trimming.
fn exact_heading(line: &str) -> Option<&'static str> {
    ALL_HEADINGS.iter().copied().find(|h| line == *h)
}

/// The canonical `group.md` scaffold. Generators (task 2.5's scaffolding)
/// must emit this template verbatim so the parser and the generator can
/// never drift apart.
pub fn group_md_template() -> String {
    r#"# Group: <group-name>

## Actor

<one line: who performs this use case>

## Goal

<one or two sentences>

## Main flow

<short prose or numbered list>

## Interactions

<one bullet per distinct real user action this group's capabilities cover end-to-end>
- <interaction 1>

## Includes

- none

## Extends

- none

## Out of scope

<what this group deliberately does not cover>

## Diagram

```mermaid
flowchart LR
  subgraph System
    UC1[[Use case name]]
  end
  Actor((Actor)) --> UC1
```
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const WELL_FORMED: &str = r#"# Group: spec authoring

## Actor

A spec author working inside a givn change.

## Goal

Write executable Gherkin specs that survive the archive merge.

## Main flow

1. Inventory the interactions.
2. Write one @e2e scenario per distinct action.

## Interactions

- author inventories user actions
- author writes e2e scenarios

## Includes

- none

## Extends

- review-disposition-gates

## Out of scope

Step-definition implementation.

## Diagram

```mermaid
flowchart LR
  Actor((Author)) --> UC1[[Write specs]]
```
"#;

    #[test]
    fn well_formed_group_md_parses_to_expected_struct() {
        let doc = parse_group_md(WELL_FORMED).unwrap();
        assert_eq!(doc.actor, "A spec author working inside a givn change.");
        assert_eq!(
            doc.goal,
            "Write executable Gherkin specs that survive the archive merge."
        );
        assert_eq!(doc.main_flow.lines().count(), 2);
        assert_eq!(doc.out_of_scope, "Step-definition implementation.");
        assert!(doc.diagram.contains("flowchart LR"));
    }

    #[test]
    fn interactions_list_is_parsed() {
        let doc = parse_group_md(WELL_FORMED).unwrap();
        assert_eq!(
            doc.interactions,
            vec![
                "author inventories user actions",
                "author writes e2e scenarios"
            ]
        );
        assert_eq!(doc.extends, vec!["review-disposition-gates"]);
        assert!(doc.includes.is_empty(), "'- none' must parse to empty");
    }

    #[test]
    fn missing_interactions_heading_is_an_error_not_an_empty_list() {
        let text = WELL_FORMED.replace("## Interactions", "## Interaktion");
        let err = parse_group_md(&text).unwrap_err();
        assert!(
            err.to_string().contains("## Interactions"),
            "the error must name the missing heading: {err}"
        );
    }

    #[test]
    fn missing_actor_heading_is_an_error() {
        let text = WELL_FORMED.replace("## Actor", "## Wer");
        let err = parse_group_md(&text).unwrap_err();
        assert!(err.to_string().contains("## Actor"), "{err}");
    }

    #[test]
    fn no_interactions_declared_is_an_empty_list_not_an_error() {
        let text = WELL_FORMED.replace(
            "- author inventories user actions\n- author writes e2e scenarios",
            "- none",
        );
        let doc = parse_group_md(&text).unwrap();
        assert!(doc.interactions.is_empty());
    }

    #[test]
    fn unmatched_heading_content_is_not_captured_elsewhere() {
        // "## Out of scope (extra words)" is NOT the exact heading, so its
        // body must not be captured into any other section.
        let text = WELL_FORMED.replace("## Out of scope", "## Out of scope (extra)");
        let doc = parse_group_md(&text).unwrap();
        assert!(doc.out_of_scope.is_empty());
        assert_eq!(doc.diagram.contains("flowchart"), true);
    }

    #[test]
    fn parser_and_generator_agree_on_headings() {
        // The template the generator emits must parse — this is the test
        // that would have caught the disposition heading mismatch.
        let template = group_md_template();
        let doc = parse_group_md(&template).unwrap();
        assert!(doc.actor.contains("<"), "placeholder body parses as-is");
        assert_eq!(doc.interactions, vec!["<interaction 1>"]);
        assert!(doc.includes.is_empty(), "'- none' must parse to empty");
        assert!(doc.extends.is_empty(), "'- none' must parse to empty");
        assert!(doc.diagram.contains("flowchart LR"));
    }
}
