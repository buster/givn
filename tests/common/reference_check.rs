//! Consumer-view reference check for generated guidance.
//!
//! Extracts rooted file references from generated skills, commands, and
//! printed instructions and resolves each one against the project that
//! receives it. Resolution classes, in order: an existing project path, an
//! agent-target skill file, a workflow-created location (the table below),
//! or a project-owned destination the guidance defines. Anything else is an
//! unresolved finding. See the change design for the table rationale.

use std::path::Path;

/// One labelled body of generated guidance.
pub struct GuidanceSource {
    pub label: String,
    pub text: String,
}

/// One reference that resolves nowhere.
#[derive(Debug)]
pub struct Finding {
    pub reference: String,
    pub source: String,
    pub line: usize,
}

/// The top-level directories the check grades.
const CANDIDATE_ROOTS: &[&str] = &[
    "givn",
    "docs",
    "assets",
    ".opencode",
    ".agents",
    ".claude",
    ".omp",
];

/// Locations a named creation step produces during the lifecycle. Each row
/// is a root segment template plus the leaves the step emits under it; `**`
/// matches zero or more remaining segments.
const WORKFLOW_CREATED: &[(&[&str], &[&[&str]])] = &[
    (
        &["givn", "changes", "<change-id>"],
        &[
            &[],
            &["proposal.md"],
            &["design.md"],
            &["design-review.md"],
            &["tasks.md"],
            &["review.md"],
            &["arc42.md"],
            &["specs", "**"],
            &["evidence", "**"],
        ],
    ),
    (
        &["givn", "archive", "<change-id>"],
        &[
            &[],
            &["proposal.md"],
            &["design.md"],
            &["design-review.md"],
            &["tasks.md"],
            &["review.md"],
            &["arc42.md"],
            &["specs", "**"],
            &["evidence", "**"],
        ],
    ),
    (
        &["givn", "specs", "<usecase-id>"],
        &[&[], &["usecase.md"], &["<capability>.feature"]],
    ),
    (&["givn", "specs"], &[&["**"]]),
    (
        &["givn", "ideation", "<topic-id>"],
        &[
            &[],
            &["session.md"],
            &["frame.md"],
            &["eventstorming.md"],
            &["use-case-definitions.md"],
            &["domain-terms.md"],
            &["questions.md"],
            &["use-cases", "**"],
            &["personas", "**"],
            &["changes", "**"],
        ],
    ),
    (&["givn", "personas"], &[&[], &["<id>.md"]]),
    (&["docs", "ideation"], &[&["<topic-id>.md"]]),
];

/// Locations the guidance itself defines and a project creates while
/// following it.
const RULE_OWNED: &[&[&str]] = &[&["docs", "design", "**"]];

/// Every installed guidance file for the project's configured agent targets:
/// `<target>/skills/*/SKILL.md` and `<target>/commands/*.md`.
pub fn installed_guidance(project_root: &Path) -> Vec<GuidanceSource> {
    let targets = givn::config::overlay::load_skills_targets(project_root)
        .expect("load configured skills targets");
    let mut sources = Vec::new();
    for target in targets {
        collect_skill_files(
            &project_root.join(format!(".{target}/skills")),
            project_root,
            &mut sources,
        );
        collect_command_files(
            &project_root.join(format!(".{target}/commands")),
            project_root,
            &mut sources,
        );
    }
    sources.sort_by(|a, b| a.label.cmp(&b.label));
    sources
}

fn collect_skill_files(dir: &Path, project_root: &Path, out: &mut Vec<GuidanceSource>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        push_source(&entry.path().join("SKILL.md"), project_root, out);
    }
}

fn collect_command_files(dir: &Path, project_root: &Path, out: &mut Vec<GuidanceSource>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md") {
            push_source(&path, project_root, out);
        }
    }
}

fn push_source(path: &Path, project_root: &Path, out: &mut Vec<GuidanceSource>) {
    if !path.is_file() {
        return;
    }
    let label = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();
    out.push(GuidanceSource {
        label,
        text: std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    });
}

/// Every candidate reference in `sources` that resolves nowhere.
pub fn unresolved_references(project_root: &Path, sources: &[GuidanceSource]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for source in sources {
        for (line, reference) in candidates(&source.text) {
            if !resolves(project_root, &reference) {
                findings.push(Finding {
                    reference,
                    source: source.label.clone(),
                    line,
                });
            }
        }
    }
    findings
}

/// A printable report for assertion messages.
pub fn describe(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|finding| {
            format!(
                "{}:{} — {}",
                finding.source, finding.line, finding.reference
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn candidates(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(start) = rest.find('`') {
            let after = &rest[start + 1..];
            let Some(end) = after.find('`') else {
                break;
            };
            let span = &after[..end];
            if is_candidate(span) {
                out.push((index + 1, span.to_string()));
            }
            rest = &after[end + 1..];
        }
    }
    out
}

fn is_candidate(span: &str) -> bool {
    if !span.contains('/')
        || span.contains(char::is_whitespace)
        || span.contains("://")
        || span.contains('=')
        || span.starts_with('/')
    {
        return false;
    }
    let first = span.split('/').next().unwrap_or_default();
    CANDIDATE_ROOTS.contains(&first)
}

fn resolves(project_root: &Path, raw: &str) -> bool {
    let trimmed = raw.trim_end_matches('/');
    let candidate = glob_free(trimmed);
    if candidate.is_empty() {
        return true;
    }
    if project_root.join(&candidate).exists() {
        return true;
    }
    matches_workflow_created(&candidate) || matches_rule_owned(&candidate)
}

/// A candidate that begins with a glob name is resolved through its longest
/// glob-free prefix; that prefix must be a real path or a table match.
fn glob_free(candidate: &str) -> String {
    candidate
        .split('/')
        .take_while(|segment| !segment.contains('*'))
        .collect::<Vec<_>>()
        .join("/")
}

fn matches_workflow_created(candidate: &str) -> bool {
    let segments: Vec<&str> = candidate.split('/').collect();
    WORKFLOW_CREATED.iter().any(|(root, leaves)| {
        root_segments_match(root, &segments)
            .is_some_and(|matched| leaves.iter().any(|leaf| leaf_matches(leaf, &segments[matched..])))
    })
}

fn matches_rule_owned(candidate: &str) -> bool {
    let segments: Vec<&str> = candidate.split('/').collect();
    RULE_OWNED
        .iter()
        .any(|pattern| leaf_matches(pattern, &segments))
}

fn root_segments_match(root: &[&str], segments: &[&str]) -> Option<usize> {
    if segments.len() < root.len() {
        return None;
    }
    root.iter()
        .zip(segments)
        .all(|(template, segment)| segment_template_matches(template, segment))
        .then_some(root.len())
}

fn leaf_matches(leaf: &[&str], rest: &[&str]) -> bool {
    if let Some(fixed) = leaf.strip_suffix(&["**"]) {
        rest.len() >= fixed.len()
            && fixed
                .iter()
                .zip(rest)
                .all(|(template, segment)| segment_template_matches(template, segment))
    } else {
        leaf.len() == rest.len()
            && leaf
                .iter()
                .zip(rest)
                .all(|(template, segment)| segment_template_matches(template, segment))
    }
}

/// A segment template with one `<...>` placeholder matches any segment with
/// the template's prefix and suffix; without a placeholder it matches
/// literally.
fn segment_template_matches(template: &str, segment: &str) -> bool {
    match template.find('<').zip(template.find('>')) {
        Some((open, close)) if close > open => {
            let prefix = &template[..open];
            let suffix = &template[close + 1..];
            segment.len() >= prefix.len() + suffix.len()
                && segment.starts_with(prefix)
                && segment.ends_with(suffix)
        }
        _ => template == segment,
    }
}
