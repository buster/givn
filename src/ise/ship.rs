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

/// The first value after a `Rebuilt code:` marker.
///
/// `state::actual_markdown` removes fenced examples and HTML comments. The
/// remaining text is collapsed to one line so a bold, wrapped `Decides` entry
/// (`Rebuilt\ncode: ship`) matches the plain line the guidance asks for, and
/// markdown emphasis around the value is skipped.
fn rebuilt_code(design: &str) -> Option<String> {
    let actual = crate::state::actual_markdown(design);
    let collapsed = actual.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut rest = collapsed.as_str();
    while let Some(found) = rest.find("Rebuilt code:") {
        let after = &rest[found + "Rebuilt code:".len()..];
        let value: String = after
            .chars()
            .skip_while(|character| !character.is_ascii_alphanumeric())
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '-')
            .collect();
        if !value.is_empty() {
            return Some(value.to_ascii_lowercase());
        }
        rest = after;
    }
    None
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
    fn a_wrapped_bold_ship_value_is_read() {
        let design = "**Decides D1: the Ise rebuild decision is `Whole-system rebuild` with `Rebuilt\ncode: ship`.**\n";
        assert_eq!(rebuilt_code(design), Some("ship".to_string()));
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
