//! Change-relative readability review gate.
//!
//! The report is advisory; this gate turns its findings into a review
//! requirement. Every touched finding at committed `HEAD` must be `FIXED`
//! (under threshold) or `JUSTIFIED` (a rationale artifact), at unit level.
//! The gate stores no baseline and writes nothing to the project.

use crate::readability::model::{Language, SourceScope};
use crate::readability::{self, AnalysisRequest};
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const BANNED: &[&str] = &[
    "not my code",
    "not my work",
    "not this change",
    "predates my work",
    "no capacity",
    "out of scope",
    "legacy",
    "too risky",
    "no time",
    "later",
];

pub fn enforce(project_root: &Path, change_id: &str) -> Result<()> {
    let change_dir = project_root.join("givn/changes").join(change_id);
    if !change_dir.exists() {
        bail!("change directory 'givn/changes/{change_id}/' does not exist");
    }

    let dirty = git(project_root, &["status", "--porcelain", "--", "src"])?;
    if !dirty.trim().is_empty() {
        let paths = dirty
            .lines()
            .filter_map(|line| line.get(3..))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("production paths are dirty; commit or discard before review: {paths}");
    }

    let change_path = format!("givn/changes/{change_id}/");
    let revisions = git(
        project_root,
        &[
            "rev-list",
            "--reverse",
            "--full-history",
            "HEAD",
            "--",
            &change_path,
        ],
    )?;
    let first = revisions
        .lines()
        .next()
        .with_context(|| format!(
            "change directory 'givn/changes/{change_id}/' has no reachable commit; commit the change directory before review"
        ))?;
    let parent = parent_revision(project_root, first).ok();

    let base_tree = tempfile::tempdir().context("create base-tree temp directory")?;
    if let Some(parent) = &parent {
        let archive = format!(
            "git archive {} | tar -x -C {}",
            parent,
            base_tree.path().display()
        );
        let status = Command::new("sh")
            .arg("-c")
            .arg(&archive)
            .current_dir(project_root)
            .status()
            .context("materialize the change base tree")?;
        if !status.success() {
            bail!("cannot materialize base revision {parent}");
        }
    }

    let base = analyze(base_tree.path())?;
    let head = analyze(project_root)?;
    let changed = changed_paths(project_root, parent.as_deref())?;
    let touched: Vec<_> = head
        .units
        .iter()
        .filter(|unit| {
            changed.contains(unit.path.as_str())
                || unit
                    .files
                    .iter()
                    .any(|file| changed.contains(file.as_str()))
        })
        .collect();

    let markers = markers(&change_dir);
    let findings: Vec<_> = touched
        .iter()
        .copied()
        .filter(|unit| unit.classification != readability::classify::Classification::WithinLimits)
        .collect();

    let mut failures: Vec<String> = Vec::new();
    if let Some(reason) = &markers.fail {
        failures.push(format!("FAIL {reason}"));
    }
    for unit in &findings {
        let id = unit.unit_id.to_string();
        if markers.fixed.contains(&id) {
            failures.push(format!(
                "FIXED marker for {id} but the unit is still a finding at head"
            ));
        } else if markers.justified.contains(&id) {
            if let Err(error) = validate_rationale(&change_dir, unit) {
                failures.push(format!("{id}: {error}"));
            }
        } else {
            failures.push(format!(
                "missing decision for {id}; add 'READABILITY: FIXED {id}' or 'READABILITY: JUSTIFIED {id}'"
            ));
        }
    }
    if !findings.is_empty() && !markers.pass {
        failures.push("missing 'READABILITY: PASS' for a review with findings".to_string());
    }

    println!("Readability review gate");
    println!(
        "  base {}",
        parent
            .as_deref()
            .unwrap_or("none (root change commit; empty base tree)")
    );
    println!("  head HEAD");
    println!(
        "  touched {} unit(s), {} finding(s)",
        touched.len(),
        findings.len()
    );
    for unit in &findings {
        let base_lines = base
            .units
            .iter()
            .find(|candidate| candidate.unit_id == unit.unit_id)
            .map(|candidate| candidate.metrics.code_lines)
            .map(|lines| lines.to_string())
            .unwrap_or_else(|| "none".to_string());
        println!(
            "  FINDING   {}  code lines {} -> {}; classification {}",
            unit.unit_id,
            base_lines,
            unit.metrics.code_lines,
            unit.classification.label()
        );
    }
    for id in &markers.justified {
        if findings.iter().any(|unit| &unit.unit_id.to_string() == id) {
            println!("  DECISION  JUSTIFIED  {id}");
        }
    }
    for id in &markers.fixed {
        if findings.iter().any(|unit| &unit.unit_id.to_string() == id) {
            continue;
        }
        println!("  DECISION  FIXED      {id}");
    }

    if failures.is_empty() {
        println!("readability review: PASS");
        Ok(())
    } else {
        for failure in &failures {
            println!("  FAILURE   {failure}");
        }
        let line = format!(
            "readability review: FAIL ({} failure(s): {})",
            failures.len(),
            failures.join("; ")
        );
        println!("{line}");
        bail!("{line}")
    }
}

fn analyze(root: &Path) -> Result<readability::classify::Assessment> {
    let request = AnalysisRequest {
        project_root: root.to_path_buf(),
        scope: SourceScope {
            language: Language::Rust,
            roots: vec![PathBuf::from("src")],
            include_test_sources: false,
        },
    };
    let analyzer = readability::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = readability::analyze(&request, analyzer.as_ref())?;
    Ok(readability::assess(&analysis, capabilities))
}

fn parent_revision(root: &Path, revision: &str) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "--verify", &format!("{revision}^1")])
        .output()
        .context("resolve the change base parent")?;
    if !output.status.success() {
        bail!("root commit");
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn changed_paths(root: &Path, parent: Option<&str>) -> Result<HashSet<String>> {
    let mut paths = HashSet::new();
    let rev_args = parent
        .map(|parent| format!("{parent}..HEAD"))
        .unwrap_or_else(|| "HEAD".to_string());
    let diff = git(root, &["diff", "--name-only", &rev_args, "--", "src"])?;
    for line in diff.lines().filter(|line| line.ends_with(".rs")) {
        paths.insert(line.to_string());
    }
    let untracked = git(
        root,
        &["ls-files", "--others", "--exclude-standard", "--", "src"],
    )?;
    for line in untracked.lines().filter(|line| line.ends_with(".rs")) {
        paths.insert(line.to_string());
    }
    Ok(paths)
}

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[derive(Default)]
struct Markers {
    fixed: HashSet<String>,
    justified: HashSet<String>,
    pass: bool,
    fail: Option<String>,
}

fn markers(change_dir: &Path) -> Markers {
    let mut markers = Markers::default();
    let review = std::fs::read_to_string(change_dir.join("review.md")).unwrap_or_default();
    for line in review.lines() {
        let line = line.trim();
        if line == "READABILITY: PASS" {
            markers.pass = true;
        } else if let Some(unit) = line.strip_prefix("READABILITY: FIXED ") {
            markers.fixed.insert(unit.trim().to_string());
        } else if let Some(unit) = line.strip_prefix("READABILITY: JUSTIFIED ") {
            markers.justified.insert(unit.trim().to_string());
        } else if let Some(reason) = line.strip_prefix("FAIL ") {
            markers.fail = Some(reason.trim().to_string());
        }
    }
    markers
}

fn validate_rationale(
    change_dir: &Path,
    unit: &crate::readability::classify::ClassifiedUnit,
) -> Result<()> {
    let slug = slug(&unit.unit_id.to_string());
    let path = change_dir
        .join("readability/rationales")
        .join(format!("{slug}.md"));
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("missing rationale {}", path.display()))?;
    let field = |name: &str| {
        text.lines()
            .find_map(|line| line.strip_prefix(name))
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    };
    let require = |name: &str| -> Result<String> {
        field(name).with_context(|| format!("rationale missing {name}"))
    };
    if require("Unit:")? != unit.unit_id.to_string() {
        bail!("rationale Unit does not match {}", unit.unit_id);
    }
    if require("Classification:")? != unit.classification.label() {
        bail!(
            "rationale Classification does not match {}",
            unit.classification.label()
        );
    }
    let evidence = require("Evidence:")?;
    if !evidence.chars().any(|c| c.is_ascii_digit()) && !evidence.contains("src/") {
        bail!("rationale Evidence must cite a metric or path");
    }
    let why = require("Why this shape is right:")?;
    if why.len() < 20 {
        bail!("rationale Why this shape is right is too short");
    }
    let alternatives = require("Alternatives considered:")?;
    if alternatives.eq_ignore_ascii_case("none") || alternatives.eq_ignore_ascii_case("n/a") {
        bail!("rationale Alternatives considered must name an alternative");
    }
    if !alternatives.to_lowercase().contains("reject") {
        bail!("rationale Alternatives considered must state why the alternative was rejected");
    }
    let owner = require("Owner:")?;
    if owner.len() < 3 || owner.eq_ignore_ascii_case("x") {
        bail!("rationale Owner is not a confirmed owner");
    }
    let lower = text.to_lowercase();
    for banned in BANNED {
        if lower.contains(banned) {
            bail!("rationale uses a banned reason: {banned}");
        }
    }
    Ok(())
}

fn slug(unit: &str) -> String {
    unit.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}
