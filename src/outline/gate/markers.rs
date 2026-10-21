//! `review.md` marker parsing.
//!
//! The vocabulary is `OUTLINE: AUTHOR <id>`, `OUTLINE: REVIEWER <id>`,
//! `OUTLINE: PASS <unit> | <explanation>`, `OUTLINE: FAIL <unit> | <reason>`,
//! `OUTLINE: PASS`, and the general `FAIL <reason>`. Fenced examples are
//! ignored; an unknown `OUTLINE:` line is a failure.

use crate::outline::model::UnitId;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutlineVerdict {
    Pass(String),
    Fail(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReviewMarkers {
    pub present: bool,
    pub verdict: Option<Verdict>,
    pub outlines: BTreeMap<UnitId, Vec<OutlineVerdict>>,
    pub author: Option<String>,
    pub reviewer: Option<String>,
    pub fail: Vec<String>,
    pub unknown: Vec<String>,
}

pub fn parse(change_dir: &Path) -> ReviewMarkers {
    let path = change_dir.join("review.md");
    let present = path.is_file();
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let mut markers = ReviewMarkers {
        present,
        ..ReviewMarkers::default()
    };
    if !present {
        return markers;
    }

    let mut in_fence = false;
    for raw in text.lines() {
        let trimmed = raw.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let line = trimmed
            .strip_prefix("- ")
            .map(str::trim_start)
            .unwrap_or(trimmed);
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("OUTLINE: ") {
            parse_outline_marker(rest.trim(), line, &mut markers);
        } else if line.starts_with("OUTLINE:") {
            markers.unknown.push(line.to_string());
        } else if let Some(reason) = line.strip_prefix("FAIL ") {
            let reason = reason.trim();
            if reason.is_empty() {
                markers.unknown.push(line.to_string());
            } else {
                markers.fail.push(reason.to_string());
                markers.verdict = Some(Verdict::Fail);
            }
        } else if line == "FAIL" || line.starts_with("FAILED") || line.starts_with("FAILURE") {
            markers.unknown.push(line.to_string());
        }
    }
    markers
}

fn parse_outline_marker(rest: &str, original: &str, markers: &mut ReviewMarkers) {
    if rest == "PASS" {
        if markers.verdict != Some(Verdict::Fail) {
            markers.verdict = Some(Verdict::Pass);
        }
        return;
    }
    if rest.starts_with("PASS ") || rest.starts_with("FAIL ") {
        parse_outline_verdict(rest, original, markers);
        return;
    }
    if let Some(value) = rest.strip_prefix("AUTHOR ") {
        let value = value.trim();
        if value.is_empty() {
            markers.unknown.push(original.to_string());
        } else {
            markers.author = Some(value.to_string());
        }
        return;
    }
    if let Some(value) = rest.strip_prefix("REVIEWER ") {
        let value = value.trim();
        if value.is_empty() {
            markers.unknown.push(original.to_string());
        } else {
            markers.reviewer = Some(value.to_string());
        }
        return;
    }
    markers.unknown.push(original.to_string());
}

fn parse_outline_verdict(rest: &str, original: &str, markers: &mut ReviewMarkers) {
    let (pass, rest) = if let Some(rest) = rest.strip_prefix("PASS ") {
        (true, rest)
    } else if let Some(rest) = rest.strip_prefix("FAIL ") {
        (false, rest)
    } else {
        markers.unknown.push(original.to_string());
        return;
    };
    let Some((unit, explanation)) = rest.split_once('|') else {
        markers.unknown.push(original.to_string());
        return;
    };
    let Some(unit) = clean_unit(unit) else {
        markers.unknown.push(original.to_string());
        return;
    };
    let explanation = explanation.trim();
    let verdict = if pass {
        OutlineVerdict::Pass(explanation.to_string())
    } else {
        OutlineVerdict::Fail(explanation.to_string())
    };
    markers.outlines.entry(unit).or_default().push(verdict);
}

fn clean_unit(value: &str) -> Option<UnitId> {
    let mut unit = value.trim();
    if unit.len() >= 2 && unit.starts_with('`') && unit.ends_with('`') {
        unit = &unit[1..unit.len() - 1];
    }
    let unit = unit.trim();
    if unit.is_empty() {
        None
    } else {
        Some(UnitId(unit.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_text(text: &str) -> ReviewMarkers {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("review.md"), text).unwrap();
        parse(dir.path())
    }

    #[test]
    fn parses_the_complete_vocabulary() {
        let markers = parse_text(
            "OUTLINE: AUTHOR agent\n\
             OUTLINE: REVIEWER fresh-context-reviewer\n\
             OUTLINE: PASS type:src/engine.rs::Engine | builds an Engine; validate then compile\n\
             - OUTLINE: FAIL `module:src/history.rs` | the call order is not explainable\n\
             OUTLINE: PASS\n\
             FAIL the type has no single responsibility\n",
        );
        assert!(markers.present);
        assert_eq!(markers.verdict, Some(Verdict::Fail));
        assert_eq!(markers.author.as_deref(), Some("agent"));
        assert_eq!(markers.reviewer.as_deref(), Some("fresh-context-reviewer"));
        assert_eq!(
            markers
                .outlines
                .get(&UnitId("type:src/engine.rs::Engine".to_string())),
            Some(&vec![OutlineVerdict::Pass(
                "builds an Engine; validate then compile".to_string()
            )])
        );
        assert_eq!(
            markers
                .outlines
                .get(&UnitId("module:src/history.rs".to_string())),
            Some(&vec![OutlineVerdict::Fail(
                "the call order is not explainable".to_string()
            )])
        );
        assert_eq!(markers.fail, vec!["the type has no single responsibility"]);
        assert!(markers.unknown.is_empty());
    }

    #[test]
    fn fenced_examples_and_inline_prose_are_not_records() {
        let markers = parse_text(
            "Example:\n```text\nOUTLINE: PASS\nFAILED\n```\n\
             The prose says OUTLINE: PASS but this is not a marker.\n",
        );
        assert_eq!(markers.verdict, None);
        assert!(markers.unknown.is_empty());
    }

    #[test]
    fn unknown_markers_and_failed_words_fail_closed() {
        let markers = parse_text(
            "OUTLINE: DEFERRED type:src/engine.rs::Engine\nFAILED the build\nFAILURE no\nFAIL\n",
        );
        assert_eq!(
            markers.unknown,
            vec![
                "OUTLINE: DEFERRED type:src/engine.rs::Engine",
                "FAILED the build",
                "FAILURE no",
                "FAIL",
            ]
        );
    }

    #[test]
    fn duplicate_outline_verdicts_are_kept_for_the_gate() {
        let markers = parse_text(
            "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility\n\
             OUTLINE: PASS type:src/engine.rs::Engine | another responsibility\n\
             OUTLINE: PASS\n",
        );
        assert_eq!(
            markers
                .outlines
                .get(&UnitId("type:src/engine.rs::Engine".to_string()))
                .map(Vec::len),
            Some(2)
        );
        assert_eq!(markers.verdict, Some(Verdict::Pass));
    }

    #[test]
    fn a_missing_record_is_reported_separately() {
        let dir = tempfile::tempdir().unwrap();
        let markers = parse(dir.path());
        assert!(!markers.present);
        assert!(markers.outlines.is_empty());
    }

    #[test]
    fn outline_lines_without_a_unit_are_unknown() {
        let markers = parse_text(
            "OUTLINE: PASS type:src/engine.rs::Engine\n\
             OUTLINE: PASS type:src/engine.rs::Engine | \n\
             OUTLINE: MAYBE type:src/engine.rs::Engine | unclear\n",
        );
        assert_eq!(
            markers
                .outlines
                .get(&UnitId("type:src/engine.rs::Engine".to_string())),
            Some(&vec![OutlineVerdict::Pass(String::new())])
        );
        assert_eq!(markers.unknown.len(), 2);
    }

    #[test]
    fn a_general_fail_wins_over_an_outline_pass() {
        let markers = parse_text("OUTLINE: PASS\nFAIL do not ship this\nOUTLINE: PASS\n");
        assert_eq!(markers.verdict, Some(Verdict::Fail));
        assert_eq!(markers.fail, vec!["do not ship this"]);
    }
}
