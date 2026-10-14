//! Project-owned instruction extensions (`extensions:` in `givn/config.yaml`).
//!
//! The extension is validated when guidance is resolved: an unknown
//! instruction id or a malformed entry warns and is skipped, the request
//! serves the built-in rules alone, and nothing in the project is mutated.
//! The map is read on every request (no cache, nothing written).

use std::path::Path;

use serde_yaml_ng::Value;

/// Load and validate the project's extension map.
///
/// Returns `(instruction id, rules)` pairs in config document order; an
/// absent key or an absent config file yields an empty list. Every rejection
/// is deterministic — one warning line per rejected entry — and never fails
/// the request.
pub fn load(project_root: &Path) -> Vec<(String, Vec<String>)> {
    let config_path = project_root.join("givn/config.yaml");
    let Ok(yaml) = std::fs::read_to_string(&config_path) else {
        return Vec::new();
    };
    let Ok(doc) = serde_yaml_ng::from_str::<crate::config::overlay::OverlayDoc>(&yaml) else {
        // The manifest load path reports config parse failures itself; the
        // extension read never fails a command.
        return Vec::new();
    };
    let Some(entries) = doc.extensions else {
        return Vec::new();
    };

    let mut extensions: Vec<(String, Vec<String>)> = Vec::new();
    for (id, value) in entries.entries {
        let mut rules: Vec<String> = Vec::new();
        match value {
            Value::Sequence(elements) => {
                for element in elements {
                    match element {
                        Value::String(text) if !text.trim().is_empty() => {
                            rules.push(text.trim().to_string());
                        }
                        _ => {
                            eprintln!(
                                "warning: ignoring malformed extension for '{id}': \
                                 a rule must be a non-empty string"
                            );
                        }
                    }
                }
            }
            _ => {
                eprintln!(
                    "warning: ignoring malformed extension for '{id}': \
                     expected a list of rule strings"
                );
                continue;
            }
        }
        if crate::config::cascade::resolve_instruction(&id).is_err() {
            eprintln!("warning: ignoring extension for unknown instruction '{id}'");
            continue;
        }
        extensions.push((id, rules));
    }
    extensions
}

/// Append the project's rules to `content` under the marked extension
/// section. An empty rule list adds nothing.
pub fn append(content: &str, rules: &[String]) -> String {
    if rules.is_empty() {
        return content.to_string();
    }
    let mut extended = String::with_capacity(content.len() + 256);
    extended.push_str(content);
    if !extended.ends_with('\n') {
        extended.push('\n');
    }
    extended.push_str(
        "\n---\n\n## Project extension\n\n\
         This project extends the built-in rules above with the following \
         rules.\n\
         They add to the built-in rules and never override, suppress, or \
         reorder\nthem.\n",
    );
    for rule in rules {
        extended.push_str("\n- ");
        extended.push_str(rule);
        extended.push('\n');
    }
    extended
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_adds_the_marked_section_with_rule_order() {
        let content = "# Instruction: demo\n";
        let extended = append(
            content,
            &["First rule.".to_string(), "Second rule.".to_string()],
        );
        assert!(extended.starts_with(content));
        let section = extended
            .split_once("## Project extension")
            .expect("marked section")
            .1;
        let first = section.find("First rule.").expect("first rule");
        let second = section.find("Second rule.").expect("second rule");
        assert!(first < second, "rule order follows the config order");
    }

    #[test]
    fn append_without_rules_returns_the_content_unchanged() {
        let content = "# Instruction: demo\n";
        assert_eq!(append(content, &[]), content);
    }
}
