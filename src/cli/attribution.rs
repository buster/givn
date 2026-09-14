//! Commit-attribution command implementation.

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::Read;
use std::path::Path;

const LEGACY_TRAILER_PREFIXES: [&str; 5] = [
    "Givn-Version:",
    "LLM:",
    "LLM-Reasoning:",
    "LLM-Harness:",
    "Givn-Attribution-Status:",
];
const UNAVAILABLE: &str = "unavailable";
const CLAUDE_HARNESS: &str = "claude-code";

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
    let stamped = stamp_text(&original, version, llm, reasoning, harness);
    std::fs::write(path, stamped)
        .with_context(|| format!("write commit message {}", path.display()))?;
    Ok(())
}

/// `message` with the managed attribution trailer, when attribution context
/// is active; the message unchanged otherwise. Existing managed trailers are
/// replaced, so stamping is idempotent.
pub fn stamped_message(message: &str) -> String {
    if std::env::var_os("GIVN_ATTRIBUTION_ACTIVE").is_none() {
        return message.to_string();
    }
    let llm = attribution_value("GIVN_ATTRIBUTION_LLM");
    let reasoning = attribution_value("GIVN_ATTRIBUTION_REASONING");
    let harness = attribution_value("GIVN_ATTRIBUTION_HARNESS");
    stamp_text(
        message,
        env!("CARGO_PKG_VERSION"),
        &llm,
        &reasoning,
        &harness,
    )
}

fn stamp_text(message: &str, version: &str, llm: &str, reasoning: &str, harness: &str) -> String {
    let body = message
        .lines()
        .filter(|line| !is_managed_trailer(line))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string();

    format!("{body}\n\nCo-Authored-By: {llm}/{reasoning}@{harness} with Givn {version}\n",)
}

fn is_managed_trailer(line: &str) -> bool {
    line.starts_with("Co-Authored-By:") && line.contains(" with Givn ")
        || LEGACY_TRAILER_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(prefix))
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
    let llm = claude_model(payload);
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
        ("GIVN_ATTRIBUTION_HARNESS", CLAUDE_HARNESS.to_string()),
        (
            "GIT_COMMITTER_NAME",
            format!("LLM: {llm} via {CLAUDE_HARNESS}"),
        ),
    ]
    .into_iter()
    .map(|(name, value)| format!("{name}={}", shell_quote(&value)))
    .collect::<Vec<_>>()
    .join(" ");

    Some(format!("export {prefix}; {command}"))
}

fn claude_model(payload: &Value) -> String {
    claude_transcript_model(payload)
        .or_else(|| {
            std::env::var("CLAUDE_MODEL")
                .ok()
                .filter(|value| !value.trim().is_empty() && !value.contains(['\n', '\r']))
        })
        .unwrap_or_else(|| "claude-code".to_string())
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
            "subject\n\nCo-Authored-By: Human Developer <human@example.test>\nCo-Authored-By: old/high@old-harness with Givn old\nGivn-Version: old\nLLM: old\nUser-Trailer: keep\n",
        )
        .expect("write message");

        stamp_message_values(&path, "test-version", "unavailable", "high", "opencode")
            .expect("stamp message");

        let message = std::fs::read_to_string(path).expect("read message");
        assert!(
            message.contains("Co-Authored-By: unavailable/high@opencode with Givn test-version")
        );
        assert!(message.contains("User-Trailer: keep"));
        assert!(message.contains("Co-Authored-By: Human Developer <human@example.test>"));
        assert!(!message.contains("Givn-Version:"));
        assert!(!message.contains("LLM:"));
        assert!(!message.contains("LLM-Reasoning:"));
        assert!(!message.contains("LLM-Harness:"));
        assert!(!message.contains("Givn-Attribution-Status:"));
        assert!(!message.contains("Givn-Version: old"));
        assert!(!message.contains("Co-Authored-By: old/high@old-harness with Givn old"));
    }

    #[test]
    fn claude_payload_exports_context_for_compound_bash_commands() {
        let commit = serde_json::json!({
            "tool_input": {"command": "echo before && git commit -m 'subject'"},
            "effort": {"level": "high"}
        });
        let updated = claude_updated_command(&commit).expect("updated commit command");
        assert!(updated.starts_with("export GIVN_ATTRIBUTION_ACTIVE='1'"));
        assert!(updated.contains("GIVN_ATTRIBUTION_LLM='claude-code'"));
        assert!(updated.contains("GIVN_ATTRIBUTION_REASONING='high'"));
        assert!(updated.ends_with("; echo before && git commit -m 'subject'"));

        let read = serde_json::json!({"tool_input": {"command": "git status"}});
        let updated = claude_updated_command(&read).expect("updated Bash command");
        assert!(updated.ends_with("; git status"));
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
