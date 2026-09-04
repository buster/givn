//! Parsing of the disposition tables in review.md.

use std::path::Path;

/// A shape-match disposition row: both scenario titles + a decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispositionRow {
    pub title_a: String,
    pub title_b: String,
    pub decision: String,
}

/// A split-or-keep row: one scenario title + a decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitOrKeepRow {
    pub title: String,
    pub decision: String,
}

/// Parse review.md disposition sections. Returns (rows, split_or_keep).
pub fn parse_review(path: &Path) -> (Vec<DispositionRow>, Vec<SplitOrKeepRow>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (Vec::new(), Vec::new()),
    };
    parse_review_content(&content)
}

pub fn parse_review_content(content: &str) -> (Vec<DispositionRow>, Vec<SplitOrKeepRow>) {
    let mut rows = Vec::new();
    let mut split_rows = Vec::new();
    let mut section = "";
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let heading = trimmed.trim_start_matches('#').trim().to_lowercase();
            if heading.starts_with("overlap disposition") {
                section = "disposition";
            } else if heading.starts_with("split-or-keep") {
                section = "split";
            } else {
                section = "";
            }
            continue;
        }
        if !trimmed.starts_with('|') || trimmed.starts_with("|-") {
            continue;
        }
        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().trim_matches('`').to_string())
            .collect();
        match section {
            "disposition" => {
                if cells.len() >= 3 && cells[0].to_lowercase() != "scenario a" {
                    rows.push(DispositionRow {
                        title_a: cells[0].clone(),
                        title_b: cells[1].clone(),
                        decision: cells[2].clone(),
                    });
                }
            }
            "split" if cells.len() >= 2 && cells[0].to_lowercase() != "scenario" => {
                split_rows.push(SplitOrKeepRow {
                    title: cells[0].clone(),
                    decision: cells[1].clone(),
                });
            }
            _ => {}
        }
    }
    (rows, split_rows)
}

/// The decision words accepted in the Disposition column of an
/// `## Overlap dispositions` row (case-insensitive).
pub const VALID_DECISIONS: &[&str] = &["duplicate", "variant", "boundary"];

/// True if a disposition row validly disposes the pair (a, b) in either
/// order: the titles match AND the decision is a non-empty member of
/// [`VALID_DECISIONS`]. An empty or unrecognised decision word clears
/// nothing — a disposition must actually say what the pair is.
pub fn disposed(rows: &[DispositionRow], title_a: &str, title_b: &str) -> bool {
    rows.iter().any(|r| {
        let title_match = (r.title_a == title_a && r.title_b == title_b)
            || (r.title_a == title_b && r.title_b == title_a);
        title_match
            && !r.decision.trim().is_empty()
            && VALID_DECISIONS.contains(&r.decision.trim().to_lowercase().as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_disposition_rows() {
        let content = "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `One` | `Two` | boundary |\n";
        let (rows, split) = parse_review_content(content);
        assert_eq!(rows.len(), 1);
        assert!(disposed(&rows, "One", "Two"));
        assert!(disposed(&rows, "Two", "One"));
        assert!(split.is_empty());
    }

    #[test]
    fn empty_decision_is_not_disposed() {
        let content = "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `One` | `Two` | |\n";
        let (rows, _) = parse_review_content(content);
        assert!(
            !disposed(&rows, "One", "Two"),
            "an empty decision must not clear the finding"
        );
    }

    #[test]
    fn invalid_decision_word_is_not_disposed() {
        let content = "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `One` | `Two` | x |\n";
        let (rows, _) = parse_review_content(content);
        assert!(
            !disposed(&rows, "One", "Two"),
            "'x' is not a valid decision word"
        );
    }

    #[test]
    fn filled_in_review_template_parses() {
        // The template the archive flow scaffolds must carry headings the
        // parser recognises — this is the test that would have caught the
        // heading-mismatch bug that shipped with the disposition gate.
        let template = crate::config::embedded::Assets::get_str("templates/review.md")
            .expect("embedded review template");
        // Fill the scaffolded tables the way an author would.
        let filled = template
            .replace(
                "| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| | | |",
                "| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `One` | `Two` | boundary |",
            )
            .replace(
                "| Scenario | Decision |\n|---|---|\n| | |",
                "| Scenario | Decision |\n|---|---|\n| `The marathon` | keep |",
            );
        assert!(
            filled.contains("| `One` | `Two` | boundary |"),
            "template table shape changed — update this test"
        );
        let (rows, split) = parse_review_content(&filled);
        assert!(
            disposed(&rows, "One", "Two"),
            "a filled-in disposition row from the real template must clear the finding"
        );
        assert_eq!(split.len(), 1);
        assert_eq!(split[0].title, "The marathon");
        assert_eq!(split[0].decision, "keep");
    }

    #[test]
    fn parses_split_or_keep_rows() {
        let content =
            "## Split-or-keep\n\n| Scenario | Decision |\n|---|---|\n| `The marathon` | keep |\n";
        let (rows, split) = parse_review_content(content);
        assert!(rows.is_empty());
        assert_eq!(split.len(), 1);
        assert_eq!(split[0].title, "The marathon");
    }
}
