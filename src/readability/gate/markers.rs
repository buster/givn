//! `review.md` marker parsing.
//!
//! The vocabulary is exactly `READABILITY: FIXED <unit>`,
//! `READABILITY: JUSTIFIED <unit>`, `READABILITY: PASS`, and `FAIL <reason>`.
//! Fenced examples are ignored; an unknown `READABILITY:` line is a failure.

use crate::readability::model::UnitId;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReviewMarkers {
    pub present: bool,
    pub verdict: Option<Verdict>,
    pub fixed: BTreeMap<UnitId, usize>,
    pub justified: BTreeMap<UnitId, usize>,
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

        if let Some(rest) = line.strip_prefix("READABILITY: ") {
            parse_readability(rest.trim(), line, &mut markers);
        } else if line.starts_with("READABILITY:") {
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

fn parse_readability(rest: &str, original: &str, markers: &mut ReviewMarkers) {
    if rest == "PASS" {
        if markers.verdict != Some(Verdict::Fail) {
            markers.verdict = Some(Verdict::Pass);
        }
        return;
    }
    if let Some(unit) = rest.strip_prefix("FIXED ") {
        if let Some(unit) = clean_unit(unit) {
            *markers.fixed.entry(unit).or_insert(0) += 1;
        } else {
            markers.unknown.push(original.to_string());
        }
        return;
    }
    if let Some(unit) = rest.strip_prefix("JUSTIFIED ") {
        if let Some(unit) = clean_unit(unit) {
            *markers.justified.entry(unit).or_insert(0) += 1;
        } else {
            markers.unknown.push(original.to_string());
        }
        return;
    }
    markers.unknown.push(original.to_string());
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
            "READABILITY: FIXED type:src/engine.rs::Engine\n\
             - READABILITY: JUSTIFIED `type:src/cohesive.rs::Cohesive`\n\
             READABILITY: PASS\n\
             FAIL the type has no single responsibility\n",
        );
        assert!(markers.present);
        assert_eq!(markers.verdict, Some(Verdict::Fail));
        assert_eq!(
            markers
                .fixed
                .get(&UnitId("type:src/engine.rs::Engine".to_string())),
            Some(&1)
        );
        assert_eq!(
            markers
                .justified
                .get(&UnitId("type:src/cohesive.rs::Cohesive".to_string())),
            Some(&1)
        );
        assert_eq!(markers.fail, vec!["the type has no single responsibility"]);
        assert!(markers.unknown.is_empty());
    }

    #[test]
    fn fenced_examples_and_inline_prose_are_not_records() {
        let markers = parse_text(
            "Example:\n```text\nREADABILITY: PASS\nFAILED\n```\n\
             The prose says READABILITY: PASS but this is not a marker.\n",
        );
        assert_eq!(markers.verdict, None);
        assert!(markers.unknown.is_empty());
    }

    #[test]
    fn unknown_markers_and_failed_words_fail_closed() {
        let markers = parse_text(
            "READABILITY: DEFERRED type:src/engine.rs::Engine\nFAILED the build\nFAILURE no\nFAIL\n",
        );
        assert_eq!(
            markers.unknown,
            vec![
                "READABILITY: DEFERRED type:src/engine.rs::Engine",
                "FAILED the build",
                "FAILURE no",
                "FAIL",
            ]
        );
    }

    #[test]
    fn duplicate_decisions_count_once_per_marker() {
        let markers = parse_text(
            "READABILITY: FIXED type:src/engine.rs::Engine\n\
             READABILITY: FIXED type:src/engine.rs::Engine\n\
             READABILITY: PASS\nREADABILITY: PASS\n",
        );
        assert_eq!(
            markers
                .fixed
                .get(&UnitId("type:src/engine.rs::Engine".to_string())),
            Some(&2)
        );
        assert_eq!(markers.verdict, Some(Verdict::Pass));
    }

    #[test]
    fn a_missing_record_is_reported_separately() {
        let dir = tempfile::tempdir().unwrap();
        let markers = parse(dir.path());
        assert!(!markers.present);
        assert!(markers.fixed.is_empty());
    }
}
