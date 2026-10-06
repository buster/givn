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

// Only `run_with` is exercised here; the rest serves the feature runner.
#[allow(dead_code)]
#[path = "common/opencode_characterize.rs"]
mod characterize;

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

/// A mutant generated from the shipped adapter by replacing `anchor`, which
/// must occur exactly once, with `replacement`. Generation cannot go stale: a
/// shipped-adapter edit that moves the anchor fails the anchor assertion.
struct Generated {
    name: &'static str,
    anchor: &'static str,
    replacement: &'static str,
    title: &'static str,
}

const GENERATED: &[Generated] = &[
    Generated {
        name: "no-defaults-first.js",
        anchor: "          stamp(event.env, undefined)\n",
        replacement: "",
        title: "A cancelled configured-default lookup still leaves attribution unavailable",
    },
    Generated {
        name: "no-message-guard.js",
        anchor: "            typeof event.messageID !== \"string\" ||\n",
        replacement: "",
        title: "A shell request without a message or call identifier cannot keep or wipe other entries",
    },
    Generated {
        name: "no-call-guard-before.js",
        anchor: "            typeof event.id !== \"string\" ||\n",
        replacement: "",
        title: "A shell request without a message or call identifier cannot keep or wipe other entries",
    },
    Generated {
        name: "no-registration-report.js",
        anchor: "    report(kind, error)\n",
        replacement: "",
        title: "A rejected hook registration is reported once",
    },
    Generated {
        name: "report-every-failure.js",
        anchor: "  if (reported.has(kind)) return\n  reported.add(kind)\n",
        replacement: "",
        title: "A rejected hook registration is reported once",
    },
];

fn shipped_adapter() -> String {
    std::fs::read_to_string(repo().join("assets/agent-integrations/opencode/givn-attribution.js"))
        .expect("shipped adapter")
}

/// Write the generated mutant into `dir` and return its path.
fn generate(dir: &Path, mutant: &Generated) -> PathBuf {
    let asset = shipped_adapter();
    let matches = asset.matches(mutant.anchor).count();
    assert_eq!(
        matches, 1,
        "mutant {}: the anchor must match the shipped adapter exactly once, found {matches}",
        mutant.name
    );
    let path = dir.join(mutant.name);
    std::fs::write(&path, asset.replacen(mutant.anchor, mutant.replacement, 1))
        .expect("write the generated mutant");
    path
}

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

/// Fail unless `mutant_name` (a file at `path`) fails exactly the scenario `title`.
fn assert_mutant_fails_scenario(mutant: &str, asset: &Path, title: &str) {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    {
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
            .env("GIVN_OPENCODE_CHARACTERIZE_ASSET", asset)
            .env("GIVN_OPENCODE_NEGATIVE_CONTROL", "1")
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

#[test]
fn should_fail_the_scenario_when_its_mutant_replaces_the_adapter() {
    for (mutant, title) in CASES {
        let asset = repo().join("tests/fixtures/opencode/mutants").join(mutant);
        assert!(asset.is_file(), "missing mutant {}", asset.display());
        assert_mutant_fails_scenario(mutant, &asset, title);
    }
}

#[test]
fn should_fail_the_scenario_when_its_generated_mutant_replaces_the_adapter() {
    let directory = tempfile::tempdir().expect("temporary directory");
    for mutant in GENERATED {
        let asset = generate(directory.path(), mutant);
        assert_mutant_fails_scenario(mutant.name, &asset, mutant.title);
    }
}

#[test]
fn should_parse_every_mutant_as_a_module_when_checked_with_node() {
    let node = std::env::var_os("GIVN_OPENCODE_NODE")
        .expect("GIVN_OPENCODE_NODE is not exported by the development shell");
    let directory = tempfile::tempdir().expect("temporary directory");
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo().join("tests/fixtures/opencode/mutants"))
        .expect("mutants directory")
        .map(|entry| entry.expect("mutant entry").path())
        .collect();
    files.extend(
        GENERATED
            .iter()
            .map(|mutant| generate(directory.path(), mutant)),
    );
    for file in files {
        // `node --check` needs a module extension to parse `export`; copy as .mjs.
        let copy = directory.path().join(format!(
            "check-{}.mjs",
            file.file_name().unwrap().to_string_lossy()
        ));
        std::fs::copy(&file, &copy).expect("copy the mutant");
        let output = Command::new(&node)
            .arg("--check")
            .arg(&copy)
            .output()
            .expect("run node --check");
        assert!(
            output.status.success(),
            "{} is not valid JavaScript: {}",
            file.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Run one scenario with `variable` pointing at a mutant and no negative-control flag.
fn run_with_override_only(variable: &str, title: &str) -> String {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let root = feature_root(title).unwrap_or_else(|error| panic!("{error:#}"));
    let mutant = repo().join("tests/fixtures/opencode/mutants/no-drop-rule.js");
    let output = Command::new(cargo)
        .args([
            "test",
            "--locked",
            "--test",
            "features_runner",
            "--",
            "--name",
        ])
        .arg(format!("^{}$", regex_escape(title)))
        .env("GIVN_FEATURES", root)
        .env(variable, mutant)
        .env_remove("GIVN_OPENCODE_NEGATIVE_CONTROL")
        .current_dir(repo())
        .output()
        .expect("run the scenario with an exported override");
    assert!(
        !output.status.success(),
        "{variable} alone must fail the suite, but the run succeeded"
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn should_refuse_a_substitute_adapter_when_negative_control_is_not_set() {
    let text = run_with_override_only(
        "GIVN_OPENCODE_CHARACTERIZE_ASSET",
        "A request from an older message of the same session is dropped",
    );
    assert!(
        text.contains("GIVN_OPENCODE_NEGATIVE_CONTROL=1"),
        "the refusal must name the required flag:\n{text}"
    );
}

#[test]
fn should_refuse_a_substitute_plugin_when_negative_control_is_not_set() {
    let text = run_with_override_only(
        "GIVN_OPENCODE_CONTROL_PLUGIN",
        "OpenCode V2 delivers the shell request to the attribution adapter before shell creation",
    );
    assert!(
        text.contains("GIVN_OPENCODE_NEGATIVE_CONTROL=1"),
        "the refusal must name the required flag:\n{text}"
    );
}

#[test]
fn should_fail_with_a_deadline_error_when_the_harness_never_finishes() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let harness = directory.path().join("hang.sh");
    std::fs::write(&harness, "sleep 30\n").expect("write the hanging harness");
    let started = std::time::Instant::now();
    let error = characterize::run_with(
        std::ffi::OsStr::new("/bin/sh"),
        &harness,
        std::time::Duration::from_millis(300),
        Path::new("/unused"),
        "V2",
        &serde_json::Map::new(),
        &[],
    )
    .expect_err("a harness that never finishes must fail");
    assert!(
        format!("{error:#}").contains("deadline"),
        "the failure must name the deadline: {error:#}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(10),
        "the harness was not stopped at its deadline"
    );
}

/// The permanent real-runtime scenario must read the completed shell result: with
/// a plugin that loads but never attributes, it has to fail.
#[test]
fn should_fail_the_permanent_selected_model_scenario_when_the_plugin_does_not_attribute() {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let title = "OpenCode prepares attribution using its selected model";
    let output = Command::new(cargo)
        .args([
            "test",
            "--locked",
            "--test",
            "features_runner",
            "--",
            "--name",
        ])
        .arg(format!("^{}$", regex_escape(title)))
        .env("GIVN_FEATURES", repo().join("givn/specs"))
        .env(
            "GIVN_OPENCODE_CONTROL_PLUGIN",
            repo().join("tests/fixtures/opencode/controls/inert-plugin.js"),
        )
        .env("GIVN_OPENCODE_NEGATIVE_CONTROL", "1")
        .current_dir(repo())
        .output()
        .expect("run the permanent scenario against the inert plugin");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.status.success()
            && text.contains("1 scenario (1 failed)")
            && text.contains("Then the shell environment identifies LLM"),
        "the inert plugin did not fail the permanent scenario at its assertion (status {}):\n{text}",
        output.status
    );
}
