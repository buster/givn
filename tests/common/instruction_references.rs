//! Instruction-id reference check over served guidance.
//!
//! Extracts every `givn instructions <id>` direction from served instruction
//! text and every effective addon instruction, then resolves each id the way
//! the CLI does: a policy id with no change context, an artifact id in a
//! change context. A direction to an id the project does not serve, or a
//! policy direction carrying a change context the CLI ignores, is a finding.
//! See the change design, "Instruction Reference Check".
//!
//! Resolution runs the built binary in the project; nothing is inferred from
//! the asset set, so a project override, an addon instruction, and an embedded
//! asset all count as serving an id. The check is read-only.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Output;

/// One body of served instruction text and the label it is reported under.
#[derive(Debug, Clone)]
pub struct InstructionSource {
    pub label: String,
    pub text: String,
}

/// One unresolved id or one id named in the wrong context.
#[derive(Debug, Clone)]
pub struct IdFinding {
    pub id: String,
    pub source: String,
    pub line: usize,
    pub line_text: String,
    pub expected: String,
}

/// Addon instruction directories whose addon flag is not the directory name
/// with `-` replaced by `_`.
const ADDON_DIR_FLAGS: &[(&str, &str)] = &[("arc42-docs", "arc42")];

/// Tokens a direction writes instead of a change id.
const CHANGE_PLACEHOLDERS: [&str; 3] = ["<id>", "<change-id>", "<CHANGE_ID>"];

/// Every instruction direction in `text`, as its 1-based line, that line's
/// text, and the id.
///
/// A direction is a `givn instructions <id>` occurrence inside a code span, or
/// one followed by `--change`/`--json`, so prose such as "givn instructions
/// are resolved" is not a direction.
pub fn named_instruction_ids(text: &str) -> Vec<(usize, String, String)> {
    const NEEDLE: &str = "givn instructions ";
    let mut directions = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut start = 0;
        while let Some(position) = line[start..].find(NEEDLE) {
            let at = start + position;
            let after = &line[at + NEEDLE.len()..];
            let token: String = after
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                .collect();
            if token.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                && (inside_code_span(line, at) || carries_flag(&after[token.len()..]))
            {
                directions.push((index + 1, line.trim().to_string(), token));
            }
            start = at + NEEDLE.len();
        }
    }
    directions
}

/// The `--change` token a direction line names, when it names one.
///
/// A placeholder is returned with both of its angle brackets (`<id>`,
/// `<CHANGE>`), so the caller can substitute it instead of passing a truncated
/// token to the CLI.
pub fn named_change(line: &str) -> Option<String> {
    let position = line.find("--change ")?;
    let rest = &line[position + "--change ".len()..];
    if rest.starts_with('<') {
        let end = rest.find('>')?;
        return Some(rest[..=end].to_string());
    }
    let token: String = rest
        .chars()
        .take_while(|c| !c.is_whitespace() && !matches!(c, '`' | ')'))
        .collect();
    if token.is_empty() { None } else { Some(token) }
}

/// Every body of served instruction text the check grades: the project's
/// manifest artifact instructions (in a change context), every policy
/// instruction this checkout ships, and every effective addon instruction.
///
/// A policy id whose addon is not effective is not a source: the CLI refuses it
/// by addon name, and its directions are not served to anyone.
pub fn served_instruction_sources(
    project_root: &Path,
    artifact_ids: &BTreeSet<String>,
    change: &str,
) -> Vec<InstructionSource> {
    let manifest = givn::config::overlay::load_effective_at(project_root)
        .expect("load the fixture's effective manifest");
    let mut sources = Vec::new();
    for id in artifact_ids {
        sources.push(InstructionSource {
            label: id.clone(),
            text: served_text(project_root, id, Some(change)),
        });
    }
    for id in shipped_instruction_ids() {
        if artifact_ids.contains(&id) {
            continue;
        }
        if let Some(text) = served_text_if_available(project_root, &id) {
            sources.push(InstructionSource { label: id, text });
        }
    }
    for (directory, flag) in addon_instruction_directories() {
        if !givn::skills::render::manifest_addon_enabled(&manifest, &flag) {
            continue;
        }
        let context = artifact_ids.contains(&directory).then_some(change);
        sources.push(InstructionSource {
            label: format!("addons/{directory}"),
            text: served_text(project_root, &directory, context),
        });
    }
    sources
}

/// Every finding for the directions in `sources`.
///
/// `artifact_ids` is the project's manifest artifact set: an id inside it takes
/// a change context, every other id is a policy id. An artifact direction
/// without a change resolves against the fixture's single active change, the
/// way the CLI auto-selects one; a policy direction carrying `--change` is a
/// finding because the CLI prints a non-artifact id before it reads the flag.
pub fn unresolved_instruction_ids(
    project_root: &Path,
    sources: &[InstructionSource],
    artifact_ids: &BTreeSet<String>,
    change: &str,
) -> Vec<IdFinding> {
    let mut resolutions: BTreeMap<(String, Option<String>), Result<(), String>> = BTreeMap::new();
    let mut findings = Vec::new();
    for source in sources {
        for (line, line_text, id) in named_instruction_ids(&source.text) {
            let artifact = artifact_ids.contains(&id);
            let named = named_change(&line_text).map(|token| with_placeholder(&token, change));
            if let (false, Some(token)) = (artifact, named.clone()) {
                findings.push(IdFinding {
                    id,
                    source: source.label.clone(),
                    line,
                    line_text,
                    expected: format!(
                        "is a policy instruction and takes no change context; the direction names `--change {token}`, which the CLI ignores"
                    ),
                });
                continue;
            }
            let context = artifact.then(|| named.clone().unwrap_or_else(|| change.to_string()));
            let outcome = resolutions
                .entry((id.clone(), context.clone()))
                .or_insert_with(|| resolve(project_root, &id, context.as_deref()));
            match outcome {
                Ok(()) => {}
                Err(refusal) if addon_gated(refusal, &id) => {}
                Err(refusal) => findings.push(IdFinding {
                    id,
                    source: source.label.clone(),
                    line,
                    line_text,
                    expected: format!(
                        "is served {}, but the request failed: {refusal}",
                        match context {
                            Some(change) => format!("in change `{change}`"),
                            None => "with no change context".to_string(),
                        }
                    ),
                }),
            }
        }
    }
    findings
}

/// A printable report for assertion messages.
pub fn describe(findings: &[IdFinding]) -> String {
    findings
        .iter()
        .map(|finding| {
            format!(
                "{}:{} — `givn instructions {}`: {} ({})",
                finding.source, finding.line, finding.id, finding.expected, finding.line_text
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether `text` carries a direction's id inside a code span at `at`.
fn inside_code_span(line: &str, at: usize) -> bool {
    line[..at].matches('`').count() % 2 == 1
}

/// Whether the text after an id names the flags that mark a direction.
fn carries_flag(rest: &str) -> bool {
    let rest = rest.trim_start();
    rest.starts_with("--change") || rest.starts_with("--json")
}

/// The change id a placeholder token stands for.
fn with_placeholder(token: &str, change: &str) -> String {
    if CHANGE_PLACEHOLDERS.contains(&token) {
        change.to_string()
    } else {
        token.to_string()
    }
}

/// Serve `id` and return its text, panicking with the refusal otherwise.
fn served_text(project_root: &Path, id: &str, change: Option<&str>) -> String {
    let output = request(project_root, id, change);
    assert!(
        output.status.success(),
        "`givn instructions {id}` failed in {}: {}",
        project_root.display(),
        first_line(&output)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Serve a policy id, returning `None` when its addon is not effective.
fn served_text_if_available(project_root: &Path, id: &str) -> Option<String> {
    let output = request(project_root, id, None);
    if output.status.success() {
        return Some(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    let refusal = first_line(&output);
    assert!(
        addon_gated(&refusal, id),
        "`givn instructions {id}` failed in {}: {refusal}",
        project_root.display()
    );
    None
}

/// Resolve one id in the context the direction needs.
fn resolve(project_root: &Path, id: &str, change: Option<&str>) -> Result<(), String> {
    let output = request(project_root, id, change);
    if output.status.success() {
        Ok(())
    } else {
        Err(first_line(&output))
    }
}

fn request(project_root: &Path, id: &str, change: Option<&str>) -> Output {
    let mut args = vec!["instructions", id];
    if let Some(change) = change {
        args.push("--change");
        args.push(change);
    }
    super::run_givn_at(project_root, &args)
}

/// Whether the refusal is the addon gate: the id exists but its addon is off.
/// Such an id is not dangling, so the check skips it instead of reporting a
/// false finding.
fn addon_gated(refusal: &str, id: &str) -> bool {
    refusal.contains(&format!("instruction '{id}' requires the"))
}

fn first_line(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .next()
        .unwrap_or_default()
        .to_string()
}

/// The instruction ids this checkout ships as assets.
fn shipped_instruction_ids() -> Vec<String> {
    let directory = super::repo_path("assets/instructions");
    let mut ids: Vec<String> = std::fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension()? == "md").then(|| {
                path.file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
            })?
        })
        .collect();
    ids.sort();
    ids
}

/// The addon instruction directories this checkout ships, with the addon flag
/// that makes each effective.
fn addon_instruction_directories() -> Vec<(String, String)> {
    let directory = super::repo_path("assets/addons");
    let mut addons: Vec<(String, String)> = std::fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if !path.join("instructions.md").is_file() {
                return None;
            }
            let id = path.file_name()?.to_string_lossy().into_owned();
            let flag = ADDON_DIR_FLAGS
                .iter()
                .find_map(|(dir, flag)| (*dir == id).then(|| flag.to_string()))
                .unwrap_or_else(|| id.replace('-', "_"));
            Some((id, flag))
        })
        .collect();
    addons.sort();
    addons
}
