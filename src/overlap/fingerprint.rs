//! Deterministic scenario fingerprint: shape (assertion sequence) + facts.

use std::collections::BTreeMap;

use gherkin::{Background, Scenario, Step, StepType};

/// Facts extracted best-effort from a scenario. Missing keys never
/// contradict (conservative rule).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Facts {
    pub exit_code: Option<String>,
    /// Assertion position -> polarity ("positive" | "negative").
    pub contains_polarity: BTreeMap<usize, String>,
    /// Data-table facts: column header (or `col_<n>`) -> masked cell value.
    pub tables: BTreeMap<String, String>,
    /// `[DONE]` marker in the scenario title.
    pub done: Option<bool>,
    /// `TTY` mention in any step.
    pub tty: Option<bool>,
}

impl Facts {
    /// True if a key is extracted on both sides with different values.
    pub fn contradicts(&self, other: &Facts) -> bool {
        if let (Some(a), Some(b)) = (&self.exit_code, &other.exit_code)
            && a != b
        {
            return true;
        }
        for (pos, polarity) in &self.contains_polarity {
            if let Some(other_polarity) = other.contains_polarity.get(pos)
                && other_polarity != polarity
            {
                return true;
            }
        }
        for (key, value) in &self.tables {
            if let Some(other_value) = other.tables.get(key)
                && other_value != value
            {
                return true;
            }
        }
        if let (Some(a), Some(b)) = (self.done, other.done)
            && a != b
        {
            return true;
        }
        if let (Some(a), Some(b)) = (self.tty, other.tty)
            && a != b
        {
            return true;
        }
        false
    }

    fn extract(assertion_steps: &[&Step], scenario_name: &str, all_steps: &[&Step]) -> Facts {
        let mut facts = Facts {
            done: if scenario_name.contains("[DONE]") {
                Some(true)
            } else {
                None
            },
            ..Default::default()
        };
        for step in all_steps {
            let lower = step.value.to_lowercase();
            if lower
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| word == "tty")
            {
                facts.tty = Some(true);
            }
        }
        for (pos, step) in assertion_steps.iter().enumerate() {
            let value = &step.value;
            if let Some(code) = extract_exit_code(value) {
                facts.exit_code = Some(code);
            }
            let lower = value.to_lowercase();
            if lower.contains("contains") {
                let negative = lower.contains("not contain")
                    || lower.contains("not-contain")
                    || lower.contains("does not contain")
                    || lower.contains("doesn't contain");
                facts.contains_polarity.insert(
                    pos,
                    if negative { "negative" } else { "positive" }.to_string(),
                );
            }
            if let Some(table) = &step.table {
                extract_table_facts(table, &mut facts.tables);
            }
        }
        facts
    }
}

fn extract_table_facts(table: &gherkin::Table, out: &mut BTreeMap<String, String>) {
    let rows: Vec<Vec<String>> = table.rows.to_vec();
    let (header, body) = match rows.split_first() {
        Some((h, b)) => (h.clone(), b),
        None => return,
    };
    for row in body {
        for (i, cell) in row.iter().enumerate() {
            let key = header
                .get(i)
                .cloned()
                .filter(|h| !h.is_empty())
                .unwrap_or_else(|| format!("col_{i}"));
            out.insert(key, mask(cell));
        }
    }
}

fn extract_exit_code(value: &str) -> Option<String> {
    let lower = value.to_lowercase();
    if lower.contains("exits non-zero") || lower.contains("exit code non-zero") {
        return Some("non-zero".to_string());
    }
    let needle = if lower.contains("exits ") {
        "exits "
    } else if lower.contains("exit code ") {
        "exit code "
    } else {
        return None;
    };
    let idx = lower.find(needle)? + needle.len();
    let rest = &lower[idx..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

/// Mask quoted strings and numbers in a step value.
pub fn mask(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                out.push('"');
                for n in chars.by_ref() {
                    if n == '"' {
                        break;
                    }
                }
                out.push_str("*\"");
            }
            // A bare ' is never a quote delimiter in this corpus — it is an
            // apostrophe (e.g. "the user's output"). Treating it as an
            // opening quote consumed to end-of-line, discarding the actual
            // assertion content and creating false shape matches.
            c if c.is_ascii_digit() => {
                while chars.peek().is_some_and(|n| n.is_ascii_digit()) {
                    chars.next();
                }
                out.push('*');
            }
            c => out.push(c),
        }
    }
    out
}

/// A scenario fingerprint: masked assertion sequence + extracted facts.
#[derive(Debug, Clone)]
pub struct Fingerprint {
    pub assertions: Vec<String>,
    pub facts: Facts,
}

impl Fingerprint {
    pub fn of_scenario(scenario: &Scenario, background: Option<&Background>) -> Fingerprint {
        // Background steps are prepended before the comparison (they belong
        // to the scenario's step sequence) but do not count toward length.
        let background_steps: Vec<&Step> = background
            .map(|b| b.steps.iter().collect())
            .unwrap_or_default();
        let own_steps: Vec<&Step> = scenario.steps.iter().collect();
        let all_steps: Vec<&Step> = background_steps
            .iter()
            .copied()
            .chain(own_steps.iter().copied())
            .collect();

        let last_when = all_steps.iter().rposition(|s| s.ty == StepType::When);
        let assertion_steps: Vec<&Step> = all_steps
            .iter()
            .enumerate()
            .filter(|(i, s)| s.ty == StepType::Then && last_when.is_none_or(|w| *i > w))
            .map(|(_, s)| *s)
            .collect();

        let facts = Facts::extract(&assertion_steps, &scenario.name, &all_steps);
        let assertions = assertion_steps.iter().map(|s| mask(&s.value)).collect();
        Fingerprint { assertions, facts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_exit_codes() {
        assert_eq!(extract_exit_code("the command exits 0"), Some("0".into()));
        assert_eq!(extract_exit_code("the command exits 12"), Some("12".into()));
        assert_eq!(
            extract_exit_code("the command exits non-zero"),
            Some("non-zero".into())
        );
        assert_eq!(extract_exit_code("the command finishes"), None);
    }

    #[test]
    fn exit_code_contradiction() {
        let a = Facts {
            exit_code: Some("0".into()),
            ..Default::default()
        };
        let b = Facts {
            exit_code: Some("2".into()),
            ..Default::default()
        };
        assert!(a.contradicts(&b));
    }

    #[test]
    fn missing_fact_never_contradicts() {
        let a = Facts {
            exit_code: Some("0".into()),
            ..Default::default()
        };
        let b = Facts::default();
        assert!(!a.contradicts(&b));
    }
}

#[cfg(test)]
mod phase5_tests {
    use super::*;
    use crate::gherkin::parse::parse_str;

    fn steps_of(scenario_text: &str) -> Vec<Step> {
        let feature = parse_str(scenario_text).unwrap();
        feature
            .scenarios
            .first()
            .expect("scenario present")
            .steps
            .clone()
    }

    #[test]
    fn pretty_does_not_trigger_tty() {
        let steps = steps_of(
            "Feature: T\n\n  Scenario: S\n    Given x\n    Then the output is pretty-printed\n",
        );
        let refs: Vec<&Step> = steps.iter().collect();
        let facts = Facts::extract(&refs, "scenario", &refs);
        assert_eq!(facts.tty, None, "'pretty-printed' must not set tty");
    }

    #[test]
    fn standalone_tty_word_still_triggers() {
        let steps = steps_of(
            "Feature: T\n\n  Scenario: S\n    Given x\n    Then the output is printed to a TTY\n",
        );
        let refs: Vec<&Step> = steps.iter().collect();
        let facts = Facts::extract(&refs, "scenario", &refs);
        assert_eq!(facts.tty, Some(true));
    }

    #[test]
    fn missing_fact_never_contradicts_done_or_tty() {
        let a = Facts::default();
        let b = Facts {
            tty: Some(true),
            ..Default::default()
        };
        assert!(!a.contradicts(&b));
        assert!(!b.contradicts(&a));
    }
}

#[cfg(test)]
mod mask_tests {
    use super::mask;

    #[test]
    fn apostrophe_does_not_eat_rest_of_line() {
        assert_eq!(mask("the user's output is red"), "the user's output is red");
    }

    #[test]
    fn double_quoted_values_are_still_masked() {
        assert_eq!(
            mask(r#"stdout contains "Created change 'x'""#),
            r#"stdout contains "*""#
        );
    }
}
