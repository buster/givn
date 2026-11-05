//! Commit-attribution command implementation.

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Read;
use std::path::Path;

const UNAVAILABLE: &str = "unavailable";
const CLAUDE_HARNESS: &str = "claude-code";

/// The facts one attribution context contributes. A struct keeps the three
/// same-typed strings from being swapped at a call site, and gives the commit
/// trailer and the shell prefix one source.
struct Attribution {
    llm: String,
    reasoning: String,
    harness: String,
}

impl Attribution {
    /// The active context from the hook environment, or `None` when the hook
    /// runs for a human commit.
    fn from_env() -> Option<Self> {
        std::env::var_os("GIVN_ATTRIBUTION_ACTIVE")?;
        Some(Self {
            llm: attribution_value("GIVN_ATTRIBUTION_LLM"),
            reasoning: attribution_value("GIVN_ATTRIBUTION_REASONING"),
            harness: attribution_value("GIVN_ATTRIBUTION_HARNESS"),
        })
    }

    fn trailer(&self, version: &str) -> String {
        format!(
            "Co-Authored-By: {}/{reasoning}@{harness} with Givn {version}",
            self.llm,
            reasoning = self.reasoning,
            harness = self.harness
        )
    }

    /// The shell environment the generated integrations export. Kept beside
    /// the trailer so a protocol change touches one Rust place; the Lua and
    /// JavaScript assets mirror this shape.
    fn export_prefix(&self) -> String {
        let mut values = vec![
            ("GIVN_ATTRIBUTION_ACTIVE", "1".to_string()),
            ("GIVN_ATTRIBUTION_LLM", self.llm.clone()),
            ("GIVN_ATTRIBUTION_REASONING", self.reasoning.clone()),
            ("GIVN_ATTRIBUTION_HARNESS", self.harness.clone()),
        ];
        if self.llm != UNAVAILABLE {
            values.push((
                "GIT_COMMITTER_NAME",
                format!("LLM: {} via {}", self.llm, self.harness),
            ));
        }
        values
            .into_iter()
            .map(|(name, value)| format!("{name}={}", shell_quote(&value)))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn stamp_message(path: &Path) -> Result<()> {
    let Some(attribution) = Attribution::from_env() else {
        return Ok(());
    };
    stamp_message_values(path, env!("CARGO_PKG_VERSION"), &attribution)
}

fn stamp_message_values(path: &Path, version: &str, attribution: &Attribution) -> Result<()> {
    let original = std::fs::read_to_string(path)
        .with_context(|| format!("read commit message {}", path.display()))?;
    let stamped = stamp_text(&original, version, attribution);
    std::fs::write(path, stamped)
        .with_context(|| format!("write commit message {}", path.display()))?;
    Ok(())
}

/// `message` with the managed attribution trailer, when attribution context
/// is active; the message unchanged otherwise. Existing managed trailers are
/// replaced, so stamping is idempotent.
pub fn stamped_message(message: &str) -> String {
    match Attribution::from_env() {
        Some(attribution) => stamp_text(message, env!("CARGO_PKG_VERSION"), &attribution),
        None => message.to_string(),
    }
}

fn stamp_text(message: &str, version: &str, attribution: &Attribution) -> String {
    let body = message
        .lines()
        .filter(|line| !is_managed_trailer(line))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string();

    format!("{body}\n\n{}\n", attribution.trailer(version))
}

fn is_managed_trailer(line: &str) -> bool {
    line.starts_with("Co-Authored-By:") && line.contains(" with Givn ")
}

fn attribution_value(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) if !value.trim().is_empty() && !value.contains(['\n', '\r']) => value,
        _ => UNAVAILABLE.to_string(),
    }
}

pub fn claude_pre_tool_use() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let Ok(payload) = serde_json::from_str::<Value>(&input) else {
        return Ok(());
    };
    let Some(updated) = claude_updated_command(&payload) else {
        return Ok(());
    };

    println!(
        "{}",
        serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "updatedInput": {"command": updated}
            }
        })
    );
    Ok(())
}

fn claude_updated_command(payload: &Value) -> Option<String> {
    let command = payload
        .get("tool_input")
        .and_then(|tool_input| tool_input.get("command"))
        .and_then(Value::as_str)?;
    if !names_revision_verb(command) {
        return None;
    }
    let attribution = Attribution {
        llm: claude_model(payload),
        reasoning: claude_reasoning(payload),
        harness: CLAUDE_HARNESS.to_string(),
    };
    Some(format!("export {}; {command}", attribution.export_prefix()))
}

fn claude_reasoning(payload: &Value) -> String {
    std::env::var("CLAUDE_CODE_EFFORT_LEVEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            payload
                .get("effort")
                .and_then(|effort| effort.get("level"))
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| UNAVAILABLE.to_string())
}

fn claude_model(payload: &Value) -> String {
    claude_transcript_model(payload)
        .or_else(|| {
            std::env::var("CLAUDE_MODEL")
                .ok()
                .filter(|value| !value.trim().is_empty() && !value.contains(['\n', '\r']))
        })
        .unwrap_or_else(|| CLAUDE_HARNESS.to_string())
}

/// Whether the command names a standalone `commit` word. The generated Lua
/// and JavaScript layers apply the same check, so a command that creates no
/// revision stays untouched for every harness.
fn names_revision_verb(command: &str) -> bool {
    command
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(|word| word == "commit")
}

fn claude_transcript_model(payload: &Value) -> Option<String> {
    let path = payload.get("transcript_path").and_then(Value::as_str)?;
    let transcript = std::fs::read_to_string(path).ok()?;

    transcript.lines().rev().find_map(|line| {
        let entry = serde_json::from_str::<Value>(line).ok()?;
        let is_assistant = entry.get("type").and_then(Value::as_str) == Some("assistant")
            || entry
                .get("message")
                .and_then(|message| message.get("role"))
                .and_then(Value::as_str)
                == Some("assistant");
        if !is_assistant {
            return None;
        }

        let model = entry
            .get("message")
            .and_then(|message| message.get("model"))
            .and_then(Value::as_str)
            .or_else(|| entry.get("model").and_then(Value::as_str))?;
        (!model.trim().is_empty() && !model.contains(['\n', '\r'])).then(|| model.to_string())
    })
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_replaces_managed_trailers_with_only_the_coauthor_line() {
        let directory = tempfile::tempdir().expect("temp directory");
        let path = directory.path().join("message");
        std::fs::write(
            &path,
            "subject\n\nCo-Authored-By: Human Developer <human@example.test>\nCo-Authored-By: old/high@old-harness with Givn old\nUser-Trailer: keep\n",
        )
        .expect("write message");

        let attribution = Attribution {
            llm: "unavailable".to_string(),
            reasoning: "high".to_string(),
            harness: "opencode".to_string(),
        };
        stamp_message_values(&path, "test-version", &attribution).expect("stamp message");

        let message = std::fs::read_to_string(path).expect("read message");
        assert!(
            message.contains("Co-Authored-By: unavailable/high@opencode with Givn test-version")
        );
        assert!(message.contains("User-Trailer: keep"));
        assert!(message.contains("Co-Authored-By: Human Developer <human@example.test>"));
        assert!(!message.contains("Co-Authored-By: old/high@old-harness with Givn old"));
    }

    #[test]
    fn claude_exports_context_for_compound_revision_commands_only() {
        let commit = serde_json::json!({
            "tool_input": {"command": "echo before && git commit -m 'subject'"},
            "effort": {"level": "high"}
        });
        let updated = claude_updated_command(&commit).expect("updated commit command");
        assert!(updated.starts_with("export GIVN_ATTRIBUTION_ACTIVE='1'"));
        assert!(updated.contains("GIVN_ATTRIBUTION_LLM='claude-code'"));
        assert!(updated.contains("GIVN_ATTRIBUTION_REASONING='high'"));
        assert!(updated.ends_with("; echo before && git commit -m 'subject'"));

        for command in ["git status", "make test", "echo committed"] {
            let unchanged = serde_json::json!({"tool_input": {"command": command}});
            assert_eq!(claude_updated_command(&unchanged), None, "{command}");
        }
    }

    #[test]
    fn claude_payload_prefers_the_latest_assistant_model_from_transcript() {
        let directory = tempfile::tempdir().expect("temp directory");
        let transcript = directory.path().join("transcript.jsonl");
        std::fs::write(
            &transcript,
            "{\"type\":\"assistant\",\"message\":{\"role\":\"assistant\",\"model\":\"claude-opus-5\"}}\n",
        )
        .expect("write transcript");
        let payload = serde_json::json!({
            "transcript_path": transcript,
            "tool_input": {"command": "git commit -m 'subject'"}
        });

        let updated = claude_updated_command(&payload).expect("updated commit command");
        assert!(updated.contains("GIVN_ATTRIBUTION_LLM='claude-opus-5'"));
        assert!(updated.contains("GIT_COMMITTER_NAME='LLM: claude-opus-5 via claude-code'"));
    }
}
