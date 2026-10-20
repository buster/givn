//! Rationale artifact lookup and the mechanical validation floor.
//!
//! The gate does not grade truth or plausibility. It proves that a concrete
//! alternative, concrete evidence, a named responsibility, and a confirmed
//! owner are present.

use super::findings::GateFinding;
use crate::readability::model::{UnitId, UnitKind};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

pub const BANNED: &[&str] = &[
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RationaleError {
    Missing,
    Invalid(String),
}

impl fmt::Display for RationaleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => formatter.write_str("rationale is missing"),
            Self::Invalid(message) => formatter.write_str(message),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationaleArtifact {
    pub path: String,
    pub unit: UnitId,
    pub classification: String,
    pub finding: String,
    pub evidence: String,
    pub why: String,
    pub alternatives: String,
    pub owner: String,
    pub follow_up: Option<String>,
    pub expiry: Option<String>,
}

/// Lowercase the unit id, replace every run of non-`[a-z0-9]` characters with
/// `-`, and trim leading/trailing `-`.
pub fn slug(unit: &str) -> String {
    let mut slug = String::with_capacity(unit.len());
    let mut separator = false;
    for character in unit.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            separator = false;
        } else if !separator {
            slug.push('-');
            separator = true;
        }
    }
    slug.trim_matches('-').to_string()
}

pub fn relative_path(change_id: &str, unit: &UnitId) -> String {
    format!(
        "givn/changes/{change_id}/readability/rationales/{}.md",
        slug(&unit.to_string())
    )
}

pub fn validate(
    project_root: &Path,
    relative: &str,
    finding: &GateFinding,
) -> Result<RationaleArtifact, RationaleError> {
    let path = project_root.join(relative);
    let text = std::fs::read_to_string(&path).map_err(|_| RationaleError::Missing)?;
    validate_text(project_root, relative, &text, finding).map_err(RationaleError::Invalid)
}

fn validate_text(
    project_root: &Path,
    relative: &str,
    text: &str,
    finding: &GateFinding,
) -> Result<RationaleArtifact, String> {
    let fields = Fields::parse(text);

    let unit = fields.required("Unit", "missing Unit:")?;
    let classification = fields.required("Classification", "missing Classification:")?;
    let finding_reason = fields.required("Finding", "missing Finding:")?;
    let evidence = fields.required("Evidence", "missing Evidence:")?;
    let why = fields.required(
        "Why this shape is right",
        "missing Why this shape is right:",
    )?;
    let alternatives = fields.required(
        "Alternatives considered",
        "missing Alternatives considered:",
    )?;
    let owner = fields.required("Owner", "missing Owner:")?;

    if unit != finding.unit_id.to_string() {
        return Err(format!(
            "Unit: {unit} does not match the finding {}",
            finding.unit_id
        ));
    }
    let head_label = finding
        .head_classification
        .map(|classification| classification.label())
        .unwrap_or("within-limits");
    if classification != head_label {
        return Err(format!(
            "Classification: {classification} does not match {head_label}"
        ));
    }
    if finding_reason != finding.reason.label() {
        return Err(format!(
            "Finding: {finding_reason} does not match {}",
            finding.reason.label()
        ));
    }

    let follow_up_prose = fields.value("Follow-up").unwrap_or_default();
    let expiry_prose = fields.value("Expiry").unwrap_or_default();
    let prose = [
        evidence.as_str(),
        why.as_str(),
        alternatives.as_str(),
        owner.as_str(),
        follow_up_prose.as_str(),
        expiry_prose.as_str(),
    ]
    .join("\n");
    if let Some(phrase) = banned_phrase(&prose) {
        return Err(format!("banned reason '{phrase}'"));
    }
    if !evidence_cites(&evidence, finding) {
        return Err(
            "Evidence: must cite a concrete metric or path from the gate finding".to_string(),
        );
    }
    if !names_responsibility(&why, finding) {
        return Err("Why this shape is right: must name the unit's responsibility".to_string());
    }
    if !alternatives_ok(&alternatives) {
        return Err(
            "Alternatives considered: must name at least one alternative and why it was rejected"
                .to_string(),
        );
    }

    let owners = confirmed_owners(project_root);
    if owner.eq_ignore_ascii_case("none") || !owners.contains(&owner) {
        return Err(format!(
            "Owner: '{owner}' is not a confirmed owner; use givn-maintainer--owner"
        ));
    }

    let follow_up = fields.value("Follow-up");
    let expiry = fields.value("Expiry");
    match (&follow_up, &expiry) {
        (Some(_), None) => return Err("missing Expiry:".to_string()),
        (None, Some(_)) => return Err("missing Follow-up:".to_string()),
        _ => {}
    }

    Ok(RationaleArtifact {
        path: relative.to_string(),
        unit: finding.unit_id.clone(),
        classification,
        finding: finding_reason,
        evidence,
        why,
        alternatives,
        owner,
        follow_up,
        expiry,
    })
}

struct Fields {
    values: BTreeMap<&'static str, String>,
}

impl Fields {
    fn parse(text: &str) -> Self {
        const LABELS: &[&str] = &[
            "Unit",
            "Classification",
            "Finding",
            "Evidence",
            "Why this shape is right",
            "Alternatives considered",
            "Owner",
            "Follow-up",
            "Expiry",
        ];
        let mut values: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
        let mut current: Option<&'static str> = None;
        for line in text.lines() {
            let trimmed = line.trim();
            let label = LABELS.iter().find(|label| {
                trimmed
                    .strip_prefix(**label)
                    .is_some_and(|rest| rest.starts_with(':'))
            });
            if let Some(label) = label {
                let label = *label;
                let rest = trimmed[label.len() + 1..].trim();
                values.entry(label).or_default().push(rest.to_string());
                current = Some(label);
            } else if let Some(label) = current
                && !trimmed.is_empty()
            {
                values.entry(label).or_default().push(trimmed.to_string());
            }
        }
        Self {
            values: values
                .into_iter()
                .map(|(label, parts)| (label, parts.join(" ").trim().to_string()))
                .collect(),
        }
    }

    fn value(&self, label: &str) -> Option<String> {
        self.values
            .get(label)
            .filter(|value| !value.is_empty())
            .cloned()
    }

    fn required(&self, label: &str, message: &str) -> Result<String, String> {
        self.value(label).ok_or_else(|| message.to_string())
    }
}

fn banned_phrase(text: &str) -> Option<&'static str> {
    let words: Vec<String> = text
        .to_lowercase()
        .split(|character: char| !character.is_alphanumeric() && character != '_')
        .filter(|word| !word.is_empty())
        .map(ToString::to_string)
        .collect();
    for phrase in BANNED {
        let needle: Vec<&str> = phrase.split_whitespace().collect();
        if words
            .windows(needle.len())
            .any(|window| window.iter().map(String::as_str).eq(needle.iter().copied()))
        {
            return Some(phrase);
        }
    }
    None
}

fn evidence_cites(evidence: &str, finding: &GateFinding) -> bool {
    let lower = evidence.to_lowercase();
    if [
        "code lines",
        "public methods",
        "lcom4",
        "tcc",
        "complexity",
        "methods",
        "fields",
    ]
    .iter()
    .any(|metric| lower.contains(metric))
    {
        return true;
    }
    if std::iter::once(finding.path.as_str())
        .chain(finding.files.iter().map(|file| file.as_str()))
        .any(|path| evidence.contains(path))
    {
        return true;
    }
    let numbers = [
        finding.base_code_lines,
        finding.head_code_lines,
        Some(finding.public_methods),
    ];
    numbers.iter().flatten().any(|number| {
        evidence
            .split(|character: char| !character.is_ascii_digit())
            .filter(|token| !token.is_empty())
            .any(|token| token.parse::<u32>().ok() == Some(*number))
    })
}

fn names_responsibility(why: &str, finding: &GateFinding) -> bool {
    let short = match finding.kind {
        UnitKind::Type(_) => finding.name.clone(),
        UnitKind::Module => Path::new(finding.path.as_str())
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_default(),
    };
    if short.is_empty() || !why.to_lowercase().contains(&short.to_lowercase()) {
        return false;
    }
    why.split(['.', '!', '?']).any(|sentence| {
        sentence
            .split_whitespace()
            .filter(|word| word.chars().any(char::is_alphanumeric))
            .count()
            >= 8
    })
}

fn alternatives_ok(alternatives: &str) -> bool {
    let lower = alternatives.trim().to_lowercase();
    if lower.is_empty() || matches!(lower.as_str(), "none" | "n/a" | "n.a." | "-") {
        return false;
    }
    let Some(index) = lower.find("rejected:") else {
        return false;
    };
    let before = lower[..index].trim();
    if before.is_empty() {
        return false;
    }
    before
        .split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .any(|word| {
            word.chars()
                .filter(|character| character.is_alphanumeric())
                .count()
                >= 3
        })
}

/// `givn-maintainer--owner` is the durable owner of the givn project itself
/// and is accepted in every project; the persona collection can add a
/// project-confirmed owner. A consumer project without a persona collection
/// therefore still records the maintainer's decision with the exact owner id.
fn confirmed_owners(project_root: &Path) -> BTreeSet<String> {
    let mut owners = BTreeSet::from(["givn-maintainer--owner".to_string()]);
    let Ok(entries) = std::fs::read_dir(project_root.join("givn/personas")) else {
        return owners;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path
            .extension()
            .is_none_or(|extension| extension != std::ffi::OsStr::new("md"))
        {
            continue;
        }
        let Some(stem) = path.file_stem() else {
            continue;
        };
        let id = stem.to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        if id.ends_with("--owner") || role_owns(&text) {
            owners.insert(id);
        }
    }
    owners
}

fn role_owns(text: &str) -> bool {
    let mut role = String::new();
    let mut in_role = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("- Role:") {
            in_role = true;
            role.push_str(rest);
            continue;
        }
        if in_role {
            if trimmed.starts_with("- ") {
                break;
            }
            role.push(' ');
            role.push_str(trimmed);
        }
    }
    let role = role.to_lowercase();
    role.contains("ownership") || role.contains("owner") || role.contains("owns")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_match_the_design_examples_and_collapse_runs() {
        assert_eq!(
            slug("type:src/engine.rs::Engine"),
            "type-src-engine-rs-engine"
        );
        assert_eq!(slug("module:src/history.rs"), "module-src-history-rs");
        assert_eq!(slug("type:src/a_b.rs::C"), "type-src-a-b-rs-c");
    }

    #[test]
    fn banned_phrases_match_whole_words_only() {
        assert_eq!(banned_phrase("This is not my code."), Some("not my code"));
        assert_eq!(banned_phrase("Translator and lateral work."), None);
        assert_eq!(banned_phrase("We will do this later."), Some("later"));
        assert_eq!(banned_phrase("It is legacy."), Some("legacy"));
    }

    #[test]
    fn alternatives_require_a_named_rejection() {
        assert!(!alternatives_ok("none"));
        assert!(!alternatives_ok("n/a"));
        assert!(!alternatives_ok("Extract the renderer."));
        assert!(alternatives_ok(
            "Extract the renderer. Rejected: rendering already delegates to the report module."
        ));
    }

    fn finding() -> GateFinding {
        GateFinding {
            unit_id: UnitId("type:src/engine.rs::Engine".to_string()),
            path: crate::readability::model::RepoPath::new("src/engine.rs"),
            files: vec![crate::readability::model::RepoPath::new("src/engine.rs")],
            kind: UnitKind::Type(crate::readability::model::TypeKind::Struct),
            name: "Engine".to_string(),
            head_classification: Some(crate::readability::classify::Classification::GodType),
            base_code_lines: Some(620),
            head_code_lines: Some(700),
            public_methods: 8,
            method_findings: Vec::new(),
            reason: crate::readability::gate::findings::FindingReason::ExistingGrown,
            decision: crate::readability::gate::findings::Decision::Missing,
        }
    }

    fn project_with_owner() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        let personas = root.path().join("givn/personas");
        std::fs::create_dir_all(&personas).unwrap();
        std::fs::write(
            personas.join("givn-maintainer--owner.md"),
            "# Persona\n\n- Role: The maintainer who owns the architecture record.\n",
        )
        .unwrap();
        root
    }

    const VALID: &str = "Unit: type:src/engine.rs::Engine\n\
Classification: god-type\n\
Finding: existing-grown\n\
Evidence: 620 -> 700 code lines; every call site is in src/engine.rs.\n\
Why this shape is right: Engine owns the engine state machine and its phase order, so splitting it would move the transition rules into two modules.\n\
Alternatives considered: Extract the renderer. Rejected: rendering delegates already and the extraction adds indirection.\n\
Owner: givn-maintainer--owner\n";

    #[test]
    fn a_complete_rationale_passes_and_each_floor_fails() {
        let root = project_with_owner();
        let finding = finding();
        let path = "givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md";
        assert!(
            validate_text(root.path(), path, VALID, &finding).is_ok(),
            "{:?}",
            validate_text(root.path(), path, VALID, &finding)
        );

        let cases = [
            ("Owner: someone-else", "not a confirmed owner"),
            (
                "Alternatives considered: none",
                "must name at least one alternative",
            ),
            (
                "Evidence: it is fine",
                "must cite a concrete metric or path",
            ),
            (
                "Why this shape is right: It works.",
                "must name the unit's responsibility",
            ),
            (
                "Owner: givn-maintainer--owner\nExpiry: 2026-12-01",
                "missing Follow-up:",
            ),
        ];
        for (replacement, expected) in cases {
            let mut text = VALID.to_string();
            let label = replacement.split(':').next().unwrap();
            let start = text.find(&format!("{label}:")).expect("field");
            let end = text[start..]
                .find('\n')
                .map(|offset| start + offset + 1)
                .unwrap_or(text.len());
            text.replace_range(start..end, &format!("{replacement}\n"));
            let error = validate_text(root.path(), path, &text, &finding).unwrap_err();
            assert!(error.contains(expected), "{replacement}: {error}");
        }

        let banned = VALID.replace(
            "Engine owns the engine state machine",
            "This is not my code; Engine owns the engine state machine",
        );
        let error = validate_text(root.path(), path, &banned, &finding).unwrap_err();
        assert_eq!(error, "banned reason 'not my code'");
    }

    #[test]
    fn owner_resolution_accepts_the_maintainer_and_project_owners() {
        let root = tempfile::tempdir().unwrap();
        let owners = confirmed_owners(root.path());
        assert!(owners.contains("givn-maintainer--owner"));
        assert!(!owners.contains("someone-else"));

        let personas = root.path().join("givn/personas");
        std::fs::create_dir_all(&personas).unwrap();
        std::fs::write(
            personas.join("project--owner.md"),
            "# Persona\n\n- Role: The maintainer who owns the architecture record.\n",
        )
        .unwrap();
        std::fs::write(
            personas.join("agent--technical-role.md"),
            "# Persona\n\n- Role: The automation caller.\n",
        )
        .unwrap();
        let owners = confirmed_owners(root.path());
        assert!(owners.contains("givn-maintainer--owner"));
        assert!(owners.contains("project--owner"));
        assert!(!owners.contains("agent--technical-role"));
    }
}
