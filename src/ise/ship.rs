//! The `Rebuilt code: ship` declaration and the component tier it requires.
//!
//! An Ise rebuilding change records one `Ise rebuild decision` and, when it
//! rebuilds, a plain `Rebuilt code` line. `ship` adopts the replacement into
//! the change, so the retained component corpus must still execute. `discard`
//! leaves the result outside the project and is covered by the trial gate.

use std::path::Path;

use anyhow::Result;

/// Whether the change's design records `Rebuilt code: ship`.
pub(crate) fn records_rebuilt_ship(change_dir: &Path) -> bool {
    let Ok(design) = std::fs::read_to_string(change_dir.join("design.md")) else {
        return false;
    };
    rebuilt_code(&design).is_some_and(|value| value == "ship")
}

/// The declared component corpus this change must execute, when it is an Ise
/// ship with at least one component execution unit.
///
/// The change lock is the authority for `ise` effectiveness. A design that
/// records `discard`, a change with `ise` off, and a corpus with no component
/// scenario all return `None`.
pub(crate) fn component_tier_required(
    project_root: &Path,
    change_dir: &Path,
) -> Result<Option<usize>> {
    if !crate::config::lock::ise_effective_for_change(change_dir)? {
        return Ok(None);
    }
    if !records_rebuilt_ship(change_dir) {
        return Ok(None);
    }
    let declared = super::contract::retained_component_units(project_root)?;
    Ok((declared > 0).then_some(declared))
}

/// The value of a plain `Rebuilt code:` declaration line.
///
/// The guidance asks for a plain line, so prose mentions do not declare a
/// rebuild. `state::actual_markdown` removes fenced examples and HTML comments
/// first; a list marker is accepted, and markdown emphasis around the line or
/// the value is ignored. A wrapped inline mention inside a `Decides` paragraph
/// is prose, not a declaration.
fn rebuilt_code(design: &str) -> Option<String> {
    let actual = crate::state::actual_markdown(design);
    actual.lines().find_map(|line| {
        let line = line.trim();
        let line = line
            .strip_prefix("- ")
            .or_else(|| line.strip_prefix("* "))
            .unwrap_or(line);
        let line = line.trim_matches(['*', '_', '`']).trim();
        let value = line.strip_prefix("Rebuilt code:")?;
        let value = value.trim_matches(['`', '*', '_', '.', ' ']);
        (!value.is_empty()).then(|| value.to_ascii_lowercase())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_ship_line_is_read() {
        assert_eq!(
            rebuilt_code("# Design\n\nRebuilt code: ship\n"),
            Some("ship".to_string())
        );
    }

    #[test]
    fn a_list_item_declaration_is_read() {
        assert_eq!(
            rebuilt_code("- **Rebuilt code:** `ship`\n"),
            Some("ship".to_string())
        );
    }

    #[test]
    fn a_prose_mention_is_not_the_declaration() {
        let design = "**Decides D1: the Ise rebuild decision is `Whole-system rebuild` with `Rebuilt\ncode: ship`.**\n";
        assert_eq!(rebuilt_code(design), None);
    }

    #[test]
    fn a_design_that_quotes_the_rule_does_not_declare_a_ship() {
        let design = "# Design\n\nThe component tier applies to every `Rebuilt code: ship` change.\n\n## Decisions\n\n- **Decides D7**: Ise rebuild decision: `No rebuild`.\n";
        assert_eq!(rebuilt_code(design), None);
    }

    #[test]
    fn a_fenced_example_is_not_the_declaration() {
        let design = "# Design\n\n```text\nRebuilt code: ship\n```\n";
        assert_eq!(rebuilt_code(design), None);
    }

    #[test]
    fn discard_is_not_a_ship() {
        assert_eq!(
            rebuilt_code("Rebuilt code: discard\n"),
            Some("discard".to_string())
        );
    }
}
