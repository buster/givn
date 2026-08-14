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
    pub done: bool,
    /// `TTY` mention in any step.
    pub tty: bool,
}

impl Facts {
    /// True if a key is extracted on both sides with different values.
    pub fn contradicts(&self, other: &Facts) -> bool {
        if let (Some(a), Some(b)) = (&self.exit_code, &other.exit_code) {
            if a != b {
                return true;
            }
        }
        for (pos, polarity) in &self.contains_polarity {
            if let Some(other_polarity) = other.contains_polarity.get(pos) {
                if other_polarity != polarity {
                    return true;
                }
            }
        }
        for (key, value) in &self.tables {
            if let Some(other_value) = other.tables.get(key) {
                if other_value != value {
                    return true;
                }
            }
        }
        if self.done != other.done || self.tty != other.tty {
            return true;
        }
        false
    }

    fn extract(assertion_steps: &[&Step], scenario_name: &str, all_steps: &[&Step]) -> Facts {
        let mut facts = Facts {
            done: scenario_name.contains("[DONE]"),
            ..Default::default()
        };
        for step in all_steps {
            if step.value.contains("TTY") || step.value.contains("tty") {
                facts.tty = true;
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
                facts
                    .contains_polarity
                    .insert(pos, if negative { "negative" } else { "positive" }.to_string());
            }
        }
        facts
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
            '\'' => {
                out.push('\'');
                for n in chars.by_ref() {
                    if n == '\'' {
                        break;
                    }
                }
                out.push_str("*'");
            }
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
        let last_when = scenario.steps.iter().rposition(|s| s.ty == StepType::When);
        let assertion_steps: Vec<&Step> = scenario
            .steps
            .iter()
            .enumerate()
            .filter(|(i, s)| s.ty == StepType::Then && last_when.is_none_or(|w| *i > w))
            .map(|(_, s)| s)
            .collect();

        let facts = Facts::extract(&assertion_steps, &scenario.name, &scenario.steps.iter().collect::<Vec<_>>());
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
        assert_eq!(extract_exit_code("the command exits non-zero"), Some("non-zero".into()));
        assert_eq!(extract_exit_code("the command finishes"), None);
    }

    #[test]
    fn exit_code_contradiction() {
        let mut a = Facts::default();
        a.exit_code = Some("0".into());
        let mut b = Facts::default();
        b.exit_code = Some("2".into());
        assert!(a.contradicts(&b));
    }

    #[test]
    fn missing_fact_never_contradicts() {
        let mut a = Facts::default();
        a.exit_code = Some("0".into());
        let b = Facts::default();
        assert!(!a.contradicts(&b));
    }
}
