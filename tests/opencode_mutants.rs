//! Meta-assertions: every committed mutant of the OpenCode attribution adapter
//! must FAIL the scenario that exists to catch it.
//!
//! A mutant is the shipped adapter with one behavior removed. If a scenario
//! stays green against its mutant, the scenario cannot detect that regression.
//! Each case runs the exact scenario in a child `features_runner` process with
//! the mutant substituted for the generated plugin.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;

/// (mutant file under `tests/fixtures/opencode/mutants/`, scenario it must fail)
const CASES: &[(&str, &str)] = &[
    (
        "no-drop-rule.js",
        "A blocked shell request does not label a later identical command from another session",
    ),
    (
        "no-drop-rule.js",
        "A request from an older message of the same session is dropped",
    ),
    (
        "drop-all-same-session.js",
        "Parallel shell requests in one message are all attributed",
    ),
    (
        "no-completion-hook.js",
        "A completed shell request is cleared by its completion event",
    ),
    (
        "no-event-guards.js",
        "Malformed tool events are ignored without blocking the shell",
    ),
    (
        "no-registration-guard.js",
        "A rejected hook registration does not stop shell attribution",
    ),
    (
        "unbounded-default-lookup.js",
        "A hung configured-default lookup does not stall shell creation",
    ),
    (
        "no-handler-guard.js",
        "Absent hook payloads and a missing shell environment never fail attribution",
    ),
];

/// Largest number of differing lines between the shipped adapter and a mutant:
/// a mutant is one small edit, so a larger diff means it has gone stale.
const MAX_CHANGED_LINES: usize = 12;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn regex_escape(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if r"\.+*?()|[]{}^$".contains(c) {
                vec!['\\', c]
            } else {
                vec![c]
            }
        })
        .collect()
}

/// The feature root that defines `title`: the change delta while it exists,
/// the permanent specs after archive.
fn feature_root(title: &str) -> anyhow::Result<PathBuf> {
    let needle = format!("Scenario: {title}");
    for root in [
        "givn/changes/fix-opencode-v2-attribution/specs",
        "givn/specs",
    ] {
        let dir = repo().join(root);
        if !dir.is_dir() {
            continue;
        }
        for entry in walk(&dir)? {
            if std::fs::read_to_string(&entry)
                .with_context(|| format!("read {}", entry.display()))?
                .lines()
                .any(|line| line.trim() == needle)
            {
                return Ok(dir);
            }
        }
    }
    anyhow::bail!("no feature file defines the scenario: {title}")
}

fn walk(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| format!("list {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(walk(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "feature") {
            files.push(path);
        }
    }
    Ok(files)
}

fn changed_lines(left: &str, right: &str) -> usize {
    let left: Vec<&str> = left.lines().collect();
    let right: Vec<&str> = right.lines().collect();
    left.iter().filter(|line| !right.contains(line)).count()
        + right.iter().filter(|line| !left.contains(line)).count()
}

#[test]
fn should_stay_a_small_edit_of_the_shipped_adapter_when_a_mutant_is_committed() {
    let asset = std::fs::read_to_string(
        repo().join("assets/agent-integrations/opencode/givn-attribution.js"),
    )
    .expect("shipped adapter");
    let mut files: Vec<_> = std::fs::read_dir(repo().join("tests/fixtures/opencode/mutants"))
        .expect("mutants directory")
        .map(|entry| entry.expect("mutant entry").path())
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no committed mutants");
    for path in files {
        let mutant = std::fs::read_to_string(&path).expect("mutant source");
        assert_ne!(
            mutant,
            asset,
            "{} equals the shipped adapter",
            path.display()
        );
        let changed = changed_lines(&asset, &mutant);
        assert!(
            changed <= MAX_CHANGED_LINES,
            "{} differs from the shipped adapter in {changed} lines: regenerate it",
            path.display()
        );
    }
}

#[test]
fn should_fail_the_scenario_when_its_mutant_replaces_the_adapter() {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    for (mutant, title) in CASES {
        let asset = repo().join("tests/fixtures/opencode/mutants").join(mutant);
        assert!(asset.is_file(), "missing mutant {}", asset.display());
        let root = feature_root(title).unwrap_or_else(|error| panic!("{error:#}"));
        let output = Command::new(&cargo)
            .args([
                "test",
                "--locked",
                "--test",
                "features_runner",
                "--",
                "--name",
            ])
            .arg(format!("^{}$", regex_escape(title)))
            .env("GIVN_FEATURES", &root)
            .env("GIVN_OPENCODE_CHARACTERIZE_ASSET", &asset)
            .current_dir(repo())
            .output()
            .expect("run the scenario against the mutant");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.status.success() && text.contains("1 scenario (1 failed)"),
            "mutant {mutant} did not fail scenario '{title}' (status {}):\n{text}",
            output.status
        );
    }
}
