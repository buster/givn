//! Commit-attribution command implementation.

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Read;
use std::path::Path;

const TRAILER_PREFIXES: [&str; 5] = [
    "Givn-Version:",
    "LLM:",
    "LLM-Reasoning:",
    "LLM-Harness:",
    "Givn-Attribution-Status:",
];
const UNAVAILABLE: &str = "unavailable";

pub fn stamp_message(path: &Path) -> Result<()> {
    if std::env::var_os("GIVN_ATTRIBUTION_ACTIVE").is_none() {
        return Ok(());
    }

    let llm = attribution_value("GIVN_ATTRIBUTION_LLM");
    let reasoning = attribution_value("GIVN_ATTRIBUTION_REASONING");
    let harness = attribution_value("GIVN_ATTRIBUTION_HARNESS");
    stamp_message_values(path, env!("CARGO_PKG_VERSION"), &llm, &reasoning, &harness)
}

fn stamp_message_values(
    path: &Path,
    version: &str,
    llm: &str,
    reasoning: &str,
    harness: &str,
) -> Result<()> {
    let original = std::fs::read_to_string(path)
        .with_context(|| format!("read commit message {}", path.display()))?;
    let body = original
        .lines()
        .filter(|line| {
            !TRAILER_PREFIXES
                .iter()
                .any(|prefix| line.starts_with(prefix))
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string();

    let status = if llm != UNAVAILABLE && reasoning != UNAVAILABLE && harness != UNAVAILABLE {
        "complete"
    } else {
        "partial"
    };

    let stamped = format!(
        "{body}\n\nGivn-Version: {}\nLLM: {llm}\nLLM-Reasoning: {reasoning}\nLLM-Harness: {harness}\nGivn-Attribution-Status: {status}\n",
        version
    );
    std::fs::write(path, stamped)
        .with_context(|| format!("write commit message {}", path.display()))?;
    Ok(())
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
    if !command.contains("git commit") {
        return None;
    }

    let llm = std::env::var("CLAUDE_MODEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "claude-code".to_string());
    let reasoning = std::env::var("CLAUDE_CODE_EFFORT_LEVEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            payload
                .get("effort")
                .and_then(|effort| effort.get("level"))
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| "unavailable".to_string());
    let prefix = [
        ("GIVN_ATTRIBUTION_ACTIVE", "1".to_string()),
        ("GIVN_ATTRIBUTION_LLM", llm.clone()),
        ("GIVN_ATTRIBUTION_REASONING", reasoning),
        ("GIVN_ATTRIBUTION_HARNESS", "claude-code".to_string()),
        ("GIT_COMMITTER_NAME", format!("LLM: {llm} via claude-code")),
    ]
    .into_iter()
    .map(|(name, value)| format!("{name}={}", shell_quote(&value)))
    .collect::<Vec<_>>()
    .join(" ");

    Some(format!("env {prefix} {command}"))
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_replaces_managed_trailers_and_marks_partial_context() {
        let directory = tempfile::tempdir().expect("temp directory");
        let path = directory.path().join("message");
        std::fs::write(
            &path,
            "subject\n\nGivn-Version: old\nLLM: old\nUser-Trailer: keep\n",
        )
        .expect("write message");

        stamp_message_values(&path, "test-version", "unavailable", "high", "opencode")
            .expect("stamp message");

        let message = std::fs::read_to_string(path).expect("read message");
        assert!(message.contains("Givn-Version: test-version"));
        assert!(message.contains("LLM: unavailable"));
        assert!(message.contains("LLM-Reasoning: high"));
        assert!(message.contains("LLM-Harness: opencode"));
        assert!(message.contains("User-Trailer: keep"));
        assert!(message.ends_with("Givn-Attribution-Status: partial\n"));
        assert!(!message.contains("Givn-Version: old"));
    }

    #[test]
    fn claude_payload_updates_only_git_commit_commands() {
        let commit = serde_json::json!({
            "tool_input": {"command": "git commit -m 'subject'"},
            "effort": {"level": "high"}
        });
        let updated = claude_updated_command(&commit).expect("updated commit command");
        assert!(updated.contains("GIVN_ATTRIBUTION_LLM='claude-code'"));
        assert!(updated.contains("GIVN_ATTRIBUTION_REASONING='high'"));
        assert!(updated.ends_with("git commit -m 'subject'"));

        let read = serde_json::json!({"tool_input": {"command": "git status"}});
        assert!(claude_updated_command(&read).is_none());
    }
}
