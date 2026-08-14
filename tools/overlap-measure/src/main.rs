//! Throwaway measurement harness: deterministic fingerprint recall on the
//! 18 labeled watn pairs from docs/feature-step-overlap-report.md.
//!
//! Usage: cargo run --release -- <path-to-watn-repo>

use anyhow::Result;

const PAIRS: &[(&str, &str, &str, &str, &str)] = &[
    ("dup", "credential-sources/credential-sources", "A literal saved credential is authoritative over environment fallback",
        "provider-setup/provider-setup", "A literal saved credential is authoritative over environment fallback"),
    ("dup", "model-autosuggest/model-autosuggest", "The newest search result stays visible when an older result arrives later",
        "search-concurrency/search-concurrency", "The newest search result stays visible when an older result arrives later"),
    ("dup", "auto-init-config/auto-init-config", "First non-TTY run does not create a config file",
        "config/config", "Missing config file prints provider setup guidance"),
    ("dup", "interactive-shell-shortcut/interactive-shell-shortcut", "The generated Bash widget runs through Bash without evaluating its result",
        "interactive-shell-shortcut/interactive-shell-shortcut", "The generated Bash widget keeps the request visible and does not evaluate the command"),
    ("dup", "interactive-shell-shortcut/interactive-shell-shortcut", "Failed or empty output preserves the original command line",
        "interactive-shell-shortcut/interactive-shell-shortcut", "Failed or empty generation preserves the original buffer"),
    ("dup", "model-autosuggest/model-autosuggest", "No matching model produces a clear empty state",
        "ratatui-model-picker/ratatui-model-picker", "Empty filter result produces a clear empty state"),
    ("dup", "reasoning-policy/reasoning-policy", "Unknown persisted reasoning remains active and is sent",
        "streamlined-setup/streamlined-setup", "Existing unknown reasoning remains active after rerunning setup"),
    ("dup", "reasoning-policy/reasoning-policy", "Minimal reasoning is persisted and sent",
        "streamlined-setup/streamlined-setup", "Free-form reasoning survives persistence and request construction"),
    ("legit", "ask/ask", "Version flag prints logo and version",
        "release-truth/release-truth", "Version flag reports the package version"),
    ("legit", "credential-sources/credential-sources", "A literal saved credential is authoritative over environment fallback",
        "credential-sources/credential-sources", "Provider-specific environment fallback precedes generic fallback"),
    ("legit", "credential-sources/credential-sources", "Provider-specific environment fallback precedes generic fallback",
        "credential-sources/credential-sources", "A missing saved environment credential fails before discovery"),
    ("legit", "incremental-sse-rendering/incremental-sse-rendering", "A DONE event completes a stream successfully",
        "incremental-sse-rendering/incremental-sse-rendering", "EOF without DONE is a truncated stream"),
    ("legit", "incremental-sse-rendering/incremental-sse-rendering", "Partial network reads are reassembled into complete events",
        "incremental-sse-rendering/incremental-sse-rendering", "Malformed nonessential events do not discard valid content"),
    ("legit", "transport/transport", "Normal debug requests ignore test routing settings",
        "transport/transport", "Test-support requests use isolated routing without changing saved configuration"),
    ("legit", "transport/transport", "Missing or whitespace test overrides fall back to the configured provider",
        "transport/transport", "Provider readiness ignores the test routing setting"),
    ("legit", "model-autosuggest/model-autosuggest", "The newest search result stays visible when an older result arrives later",
        "search-concurrency/search-concurrency", "The terminal model picker displays the newest overlapping search result"),
    ("legit", "reasoning/reasoning", "Thinking tier sends reasoning without printing it",
        "reasoning/reasoning", "Thinking tier with verbose flag prints reasoning to stderr"),
    ("legit", "unified-setup-wizard/unified-setup-wizard", "Setup wizard guides provider and model configuration page by page",
        "streamlined-setup/streamlined-setup", "Coordinated setup completes provider models reasoning and shell choices"),
];

fn main() -> Result<()> {
    let watn = std::env::args().nth(1).unwrap_or_else(|| "../watn".to_string());
    let root = std::path::PathBuf::from(&watn);
    let scenarios = givn::overlap::analysis::collect_tree(&root);
    println!("collected {} scenarios from {}", scenarios.len(), watn);

    let find = |cap: &str, title: &str| -> Option<&givn::overlap::analysis::ScenarioInfo> {
        scenarios.iter().find(|s| {
            s.file
                .to_string_lossy()
                .ends_with(&format!("givn/specs/{cap}.feature"))
                && s.title == title
        })
    };

    let mut dup_hit = 0;
    let mut dup_total = 0;
    let mut legit_fp = 0;
    let mut legit_total = 0;

    println!("{:<6} {:<60} {:>6} {:>8} {:>12}", "label", "pair", "same-title", "shape", "subset");
    for (label, cap_a, title_a, cap_b, title_b) in PAIRS {
        let (Some(a), Some(b)) = (find(cap_a, title_a), find(cap_b, title_b)) else {
            println!("MISSING: {cap_a}/{title_a} or {cap_b}/{title_b}");
            continue;
        };
        let same_title = a.title == b.title;
        let same_shape = a.fingerprint.assertions == b.fingerprint.assertions
            && !a.fingerprint.assertions.is_empty();
        let subset = (a.fingerprint.assertions.len() < b.fingerprint.assertions.len()
            && is_subsequence(&a.fingerprint.assertions, &b.fingerprint.assertions))
            || (b.fingerprint.assertions.len() < a.fingerprint.assertions.len()
                && is_subsequence(&b.fingerprint.assertions, &a.fingerprint.assertions));
        let hit = same_title || same_shape || subset;
        if *label == "dup" {
            dup_total += 1;
            if hit {
                dup_hit += 1;
            }
        } else {
            legit_total += 1;
            if same_title || same_shape {
                legit_fp += 1;
            }
        }
        println!(
            "{:<6} {:<60} {:>6} {:>8} {:>12}",
            label,
            format!("{} / {}", title_a, title_b),
            if same_title { "YES" } else { "-" },
            if same_shape { "YES" } else { "-" },
            if subset { "YES" } else { "-" },
        );
    }

    println!();
    println!("dup pairs found (title|shape|subset): {dup_hit}/{dup_total}");
    println!("legit pairs flagged as duplicate/shape (false positives): {legit_fp}/{legit_total}");
    Ok(())
}

fn is_subsequence(needle: &[String], haystack: &[String]) -> bool {
    let mut it = haystack.iter();
    needle.iter().all(|n| it.any(|h| h == n))
}
