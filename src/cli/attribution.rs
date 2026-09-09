//! Commit-attribution command implementation.

use anyhow::{Context, Result};
use std::path::Path;

const TRAILER_PREFIXES: [&str; 5] = [
    "Givn-Version:",
    "LLM:",
    "LLM-Reasoning:",
    "LLM-Harness:",
    "Givn-Attribution-Status:",
];

pub fn stamp_message(path: &Path) -> Result<()> {
    if std::env::var_os("GIVN_ATTRIBUTION_ACTIVE").is_none() {
        return Ok(());
    }

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

    let llm = attribution_value("GIVN_ATTRIBUTION_LLM");
    let reasoning = attribution_value("GIVN_ATTRIBUTION_REASONING");
    let harness = attribution_value("GIVN_ATTRIBUTION_HARNESS");
    let status = if llm != "unavailable" && reasoning != "unavailable" && harness != "unavailable" {
        "complete"
    } else {
        "partial"
    };

    let stamped = format!(
        "{body}\n\nGivn-Version: {}\nLLM: {llm}\nLLM-Reasoning: {reasoning}\nLLM-Harness: {harness}\nGivn-Attribution-Status: {status}\n",
        env!("CARGO_PKG_VERSION")
    );
    std::fs::write(path, stamped)
        .with_context(|| format!("write commit message {}", path.display()))?;
    Ok(())
}

fn attribution_value(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) if !value.trim().is_empty() && !value.contains(['\n', '\r']) => value,
        _ => "unavailable".to_string(),
    }
}

pub fn claude_pre_tool_use() -> Result<()> {
    // The Claude integration is implemented by its scenario. Malformed hook
    // input must remain non-blocking for the Bash command.
    Ok(())
}
