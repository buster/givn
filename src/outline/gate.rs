//! Change-relative outline review gate.
//!
//! The report is advisory; this gate turns its findings into a review
//! requirement. Every touched gate finding at committed `HEAD` needs exactly
//! one unit-level decision: `FIXED` (the finding no longer fires) or
//! `JUSTIFIED` (the finding still fires and a rationale exists). The gate
//! analyzes two revision trees, never the working copy, and writes nothing to
//! the project.

mod base;
mod findings;
mod head;
mod markers;
mod revision;
mod touched;

use crate::outline::classify::Assessment;
use crate::outline::model::{Analysis, Language, SourceScope, UnitId};
use crate::outline::{self, AnalysisRequest};
use crate::scm;
use anyhow::{Context, Result, bail};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub use findings::{FindingReason, GateFinding};
pub use markers::{OutlineVerdict, ReviewMarkers, Verdict};

/// One review run. `roots` defaults to the production `src/` root.
#[derive(Clone, Debug)]
pub struct GateRequest {
    pub project_root: PathBuf,
    pub change_id: String,
    pub change_dir: PathBuf,
    pub roots: Vec<PathBuf>,
    pub scope_error: Option<String>,
}

fn outline_scope(change_dir: &Path) -> (Vec<PathBuf>, Option<String>) {
    let review = change_dir.join("review.md");
    let Ok(text) = std::fs::read_to_string(&review) else {
        return (
            Vec::new(),
            Some(
                "review.md must record `OUTLINE-COMMAND: givn outline report --path <source>`"
                    .to_string(),
            ),
        );
    };
    let mut in_fence = false;
    for raw in text.lines() {
        let trimmed = raw.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let Some(command) = trimmed.strip_prefix("OUTLINE-COMMAND:") else {
            continue;
        };
        let command = command.trim().trim_matches('`');
        let mut roots = Vec::new();
        let mut tokens = command.split_whitespace().peekable();
        while let Some(token) = tokens.next() {
            let token = token.trim_matches('"').trim_matches('\'');
            if token == "--path" {
                if let Some(value) = tokens.next() {
                    roots.push(PathBuf::from(value.trim_matches('"').trim_matches('\'')));
                }
            } else if let Some(value) = token.strip_prefix("--path=") {
                roots.push(PathBuf::from(value.trim_matches('"').trim_matches('\'')));
            }
        }
        if roots.is_empty() {
            return (
                Vec::new(),
                Some("OUTLINE-COMMAND must pass at least one --path".to_string()),
            );
        }
        for root in &roots {
            if root.is_absolute()
                || root
                    .components()
                    .any(|component| matches!(component, std::path::Component::ParentDir))
            {
                return (
                    Vec::new(),
                    Some(format!(
                        "OUTLINE-COMMAND --path must be project-relative: {}",
                        root.display()
                    )),
                );
            }
        }
        return (roots, None);
    }
    (
        Vec::new(),
        Some(
            "review.md must record `OUTLINE-COMMAND: givn outline report --path <source>`"
                .to_string(),
        ),
    )
}

impl GateRequest {
    pub fn new(project_root: &Path, change_id: &str) -> Self {
        let change_dir = project_root.join("givn/changes").join(change_id);
        let (roots, scope_error) = outline_scope(&change_dir);
        Self {
            project_root: project_root.to_path_buf(),
            change_id: change_id.to_string(),
            change_dir,
            roots,
            scope_error,
        }
    }

    /// Project-relative root strings with `/` separators.
    pub fn root_strings(&self) -> Vec<String> {
        self.roots
            .iter()
            .map(|root| {
                root.to_string_lossy()
                    .replace('\\', "/")
                    .trim_end_matches('/')
                    .to_string()
            })
            .collect()
    }
}

/// The first reachable change-directory commit and its first parent. A
/// `parent` of `None` means an empty base tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeBase {
    pub first_touch: String,
    pub parent: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Base,
    Head,
}

impl Side {
    pub fn label(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Head => "head",
        }
    }
}

/// One reason the gate fails. Every variant renders as
/// `FAILURE  <code>  <message>`.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GateFailure {
    NoChangeCommit(String),
    ShallowRepository,
    DirtyProduction(Vec<String>),
    MissingOutlineScope(String),
    NoProductionSources(String),
    AnalysisFailed {
        side: Side,
        path: String,
        detail: String,
    },
    UnanalyzedFile(String),
    UnsupportedUnit {
        path: String,
        unit: String,
    },
    SkippedEntryDependency {
        side: Side,
        path: String,
        kind: String,
    },
    NonUtf8Path(String),
    NewlinePath(String),
    MissingRecord(String),
    UnknownMarker(String),
    MissingVerdict,
    FailMarker(String),
    MissingOutline {
        unit: String,
    },
    ConflictingOutline {
        unit: String,
    },
    OutlineFailed {
        unit: String,
        reason: String,
    },
    EmptyOutlineExplanation {
        unit: String,
    },
    MissingOutlineAuthor,
    MissingOutlineReviewer,
    SelfOutlineReview {
        author: String,
        reviewer: String,
    },
    MissingRecordedReviewContext,
    OutlineAuthorMismatch {
        outline: String,
        recorded: String,
    },
    OutlineReviewerMismatch {
        outline: String,
        recorded: String,
    },
    InvalidReviewContext(String),
    SlugCollision {
        first: String,
        second: String,
    },
    InvalidRationale {
        path: String,
        message: String,
    },
    MissingRationale {
        unit: String,
        path: String,
    },
    JustifiedWithoutFinding {
        unit: String,
    },
    InvalidFixed {
        unit: String,
        classification: String,
        head: u32,
        base: u32,
    },
    ConflictingDecisions {
        unit: String,
    },
    Unaccounted {
        unit: String,
    },
}

impl GateFailure {
    /// The kebab-case failure code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoChangeCommit(_) => "no-change-commit",
            Self::ShallowRepository => "shallow-repository",
            Self::DirtyProduction(_) => "dirty-production",
            Self::MissingOutlineScope(_) => "missing-outline-scope",
            Self::NoProductionSources(_) => "no-production-sources",
            Self::AnalysisFailed { .. } => "analysis-failed",
            Self::UnanalyzedFile(_) => "unanalyzed-file",
            Self::UnsupportedUnit { .. } => "unsupported-unit",
            Self::SkippedEntryDependency { .. } => "skipped-entry-dependency",
            Self::NonUtf8Path(_) => "non-utf8-path",
            Self::NewlinePath(_) => "newline-path",
            Self::MissingRecord(_) => "missing-record",
            Self::UnknownMarker(_) => "unknown-marker",
            Self::MissingVerdict => "missing-verdict",
            Self::FailMarker(_) => "fail-marker",
            Self::MissingOutline { .. } => "missing-outline",
            Self::ConflictingOutline { .. } => "conflicting-outline",
            Self::OutlineFailed { .. } => "outline-failed",
            Self::EmptyOutlineExplanation { .. } => "empty-outline-explanation",
            Self::MissingOutlineAuthor => "missing-outline-author",
            Self::MissingOutlineReviewer => "missing-outline-reviewer",
            Self::SelfOutlineReview { .. } => "self-outline-review",
            Self::MissingRecordedReviewContext => "missing-recorded-review-context",
            Self::OutlineAuthorMismatch { .. } => "outline-author-mismatch",
            Self::OutlineReviewerMismatch { .. } => "outline-reviewer-mismatch",
            Self::InvalidReviewContext(_) => "invalid-review-context",
            Self::SlugCollision { .. } => "slug-collision",
            Self::InvalidRationale { .. } => "invalid-rationale",
            Self::MissingRationale { .. } => "missing-rationale",
            Self::JustifiedWithoutFinding { .. } => "justified-without-finding",
            Self::InvalidFixed { .. } => "invalid-fixed",
            Self::ConflictingDecisions { .. } => "conflicting-decisions",
            Self::Unaccounted { .. } => "unaccounted",
        }
    }

    /// The rendered message after the code.
    pub fn message(&self) -> String {
        match self {
            Self::NoChangeCommit(change_id) => format!(
                "change directory 'givn/changes/{change_id}/' has no reachable commit; \
                 commit the change directory before review"
            ),
            Self::ShallowRepository => {
                "cannot resolve change base: repository is shallow; fetch the full history before review"
                    .to_string()
            }
            Self::DirtyProduction(paths) => format!(
                "production paths are dirty; commit or discard before review: {}",
                paths.join(", ")
            ),
            Self::MissingOutlineScope(reason) => reason.clone(),
            Self::NoProductionSources(paths) => format!(
                "outline scope found no production sources: {paths}; \
                 record an explicit OUTLINE-COMMAND with --path"
            ),
            Self::AnalysisFailed { side, path, detail } => {
                if detail.is_empty() {
                    format!("cannot analyze {path} at {}", side.label())
                } else {
                    format!("cannot analyze {path} at {}: {detail}", side.label())
                }
            }
            Self::UnanalyzedFile(path) => format!(
                "changed production file '{path}' was not analyzed; \
                 the analyzer scope or extension filter is wrong"
            ),
            Self::UnsupportedUnit { path, unit } => {
                format!("cannot analyze {path} at head: wholly unsupported unit {unit}")
            }
            Self::SkippedEntryDependency { side, path, kind } => {
                format!("cannot analyze {path} at {}: {kind}", side.label())
            }
            Self::NonUtf8Path(path) => format!("revision path is not valid UTF-8: {path}"),
            Self::NewlinePath(path) => format!("revision path contains a line break: {path}"),
            Self::MissingRecord(change_id) => format!(
                "outline record is missing; create givn/changes/{change_id}/review.md \
                 and record 'OUTLINE: PASS'"
            ),
            Self::UnknownMarker(line) => format!("unknown outline marker: {line}"),
            Self::MissingVerdict => "missing 'OUTLINE: PASS'".to_string(),
            Self::FailMarker(reason) => reason.clone(),
            Self::MissingOutline { unit } => format!(
                "MISSING outline {unit}; add 'OUTLINE: PASS {unit} | <responsibility; call order>'"
            ),
            Self::ConflictingOutline { unit } => format!("conflicting outline verdicts for {unit}"),
            Self::OutlineFailed { unit, reason } => {
                if reason.is_empty() {
                    format!("outline review failed for {unit}")
                } else {
                    format!("outline review failed for {unit}: {reason}")
                }
            }
            Self::EmptyOutlineExplanation { unit } => format!(
                "outline PASS for {unit} has no responsibility/call-order explanation"
            ),
            Self::MissingOutlineAuthor => {
                "missing 'OUTLINE: AUTHOR <id>'; the outline review must name its author"
                    .to_string()
            }
            Self::MissingOutlineReviewer => {
                "missing 'OUTLINE: REVIEWER <id>'; the outline review needs a fresh-context reviewer"
                    .to_string()
            }
            Self::SelfOutlineReview { author, reviewer } => format!(
                "outline reviewer '{reviewer}' must differ from the author '{author}'"
            ),
            Self::MissingRecordedReviewContext => {
                "review.md records a Review kind but no Author/Reviewer context; record the review context before the outline review"
                    .to_string()
            }
            Self::OutlineAuthorMismatch { outline, recorded } => format!(
                "outline author '{outline}' does not match the recorded Author '{recorded}'"
            ),
            Self::OutlineReviewerMismatch { outline, recorded } => format!(
                "outline reviewer '{outline}' does not match the recorded Reviewer context '{recorded}'"
            ),
            Self::InvalidReviewContext(message) => format!("review context: {message}"),
            Self::SlugCollision { first, second } => {
                format!("rationale slug collision for {first} and {second}")
            }
            Self::InvalidRationale { path, message } => format!("rationale {path}: {message}"),
            Self::MissingRationale { unit, path } => {
                format!("JUSTIFIED marker for {unit} has no rationale at {path}")
            }
            Self::JustifiedWithoutFinding { unit } => format!(
                "JUSTIFIED marker for {unit}, but {unit} is not a finding at head; \
                 use OUTLINE: FIXED {unit}"
            ),
            Self::InvalidFixed {
                unit,
                classification,
                head,
                base,
            } => format!(
                "FIXED marker for {unit}, but {unit} is still a finding: \
                 {classification} {head} code lines (base {base})"
            ),
            Self::ConflictingDecisions { unit } => format!("conflicting decisions for {unit}"),
            Self::Unaccounted { unit } => format!("MISSING {unit}"),
        }
    }

    fn sort_key(&self) -> (u8, u8, String, String, String) {
        let (group, variant, path, unit) = match self {
            Self::NoChangeCommit(id) => (0, 0, String::new(), id.clone()),
            Self::ShallowRepository => (0, 1, String::new(), String::new()),
            Self::DirtyProduction(paths) => (0, 2, paths.join(","), String::new()),
            Self::MissingOutlineScope(reason) => (0, 3, String::new(), reason.clone()),
            Self::NoProductionSources(paths) => (1, 6, paths.clone(), String::new()),
            Self::AnalysisFailed { path, side, .. } => {
                (1, 0, format!("{}:{path}", side.label()), String::new())
            }
            Self::UnanalyzedFile(path) => (1, 1, path.clone(), String::new()),
            Self::UnsupportedUnit { path, unit } => (1, 2, path.clone(), unit.clone()),
            Self::SkippedEntryDependency { path, .. } => (1, 3, path.clone(), String::new()),
            Self::NonUtf8Path(path) => (1, 4, path.clone(), String::new()),
            Self::NewlinePath(path) => (1, 5, path.clone(), String::new()),
            Self::MissingRecord(id) => (2, 0, String::new(), id.clone()),
            Self::UnknownMarker(line) => (2, 1, String::new(), line.clone()),
            Self::MissingVerdict => (2, 2, String::new(), String::new()),
            Self::FailMarker(reason) => (2, 3, String::new(), reason.clone()),
            Self::MissingOutline { unit } => (5, 0, String::new(), unit.clone()),
            Self::ConflictingOutline { unit } => (5, 1, String::new(), unit.clone()),
            Self::OutlineFailed { unit, .. } => (5, 2, String::new(), unit.clone()),
            Self::EmptyOutlineExplanation { unit } => (5, 3, String::new(), unit.clone()),
            Self::MissingOutlineAuthor => (5, 4, String::new(), String::new()),
            Self::MissingOutlineReviewer => (5, 5, String::new(), String::new()),
            Self::SelfOutlineReview { author, reviewer } => {
                (5, 6, String::new(), format!("{author} {reviewer}"))
            }
            Self::MissingRecordedReviewContext => (5, 7, String::new(), String::new()),
            Self::OutlineAuthorMismatch { outline, recorded } => {
                (5, 8, String::new(), format!("{outline} {recorded}"))
            }
            Self::OutlineReviewerMismatch { outline, recorded } => {
                (5, 9, String::new(), format!("{outline} {recorded}"))
            }
            Self::InvalidReviewContext(message) => (5, 10, String::new(), message.clone()),
            Self::SlugCollision { first, second } => {
                (3, 0, String::new(), format!("{first} {second}"))
            }
            Self::InvalidRationale { path, .. } => (3, 1, path.clone(), String::new()),
            Self::MissingRationale { unit, path } => (3, 2, path.clone(), unit.clone()),
            Self::JustifiedWithoutFinding { unit } => (3, 3, String::new(), unit.clone()),
            Self::InvalidFixed { unit, .. } => (4, 0, String::new(), unit.clone()),
            Self::ConflictingDecisions { unit } => (4, 1, String::new(), unit.clone()),
            Self::Unaccounted { unit } => (4, 2, String::new(), unit.clone()),
        };
        (group, variant, path, unit, self.message())
    }
}

/// One touched unit's rendered outline and the verdict the review gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutlineEntry {
    pub unit_id: UnitId,
    pub text: String,
}

/// The complete gate result. The CLI renders it verbatim.
#[derive(Debug)]
pub struct GateOutcome {
    pub base: Option<ChangeBase>,
    pub analyzed: bool,
    pub touched: usize,
    pub hints: Vec<GateFinding>,
    pub outlines: Vec<OutlineEntry>,
    #[allow(dead_code)]
    pub markers: ReviewMarkers,
    pub failures: Vec<GateFailure>,
}

impl GateOutcome {
    fn early(base: Option<ChangeBase>, failure: GateFailure) -> Self {
        Self {
            base,
            analyzed: false,
            touched: 0,
            hints: Vec::new(),
            outlines: Vec::new(),
            markers: ReviewMarkers::default(),
            failures: vec![failure],
        }
    }

    /// A change with no touched production path has no outline obligation.
    fn pass(base: Option<ChangeBase>) -> Self {
        Self {
            base,
            analyzed: false,
            touched: 0,
            hints: Vec::new(),
            outlines: Vec::new(),
            markers: ReviewMarkers::default(),
            failures: Vec::new(),
        }
    }

    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn render(&self, _request: &GateRequest) -> String {
        let mut out = String::from("Outline review gate\n");
        if let Some(base) = &self.base {
            match &base.parent {
                Some(parent) => {
                    let _ = writeln!(out, "  base {parent} (parent of {})", base.first_touch);
                }
                None => out.push_str("  base none (root change commit; empty base tree)\n"),
            }
            out.push_str("  head HEAD\n");
            if self.analyzed {
                let _ = writeln!(
                    out,
                    "  touched {} unit(s), {} outline(s), {} metric hint(s)",
                    self.touched,
                    self.outlines.len(),
                    self.hints.len()
                );
            }
        }

        for outline in &self.outlines {
            let _ = writeln!(out, "  OUTLINE   {}", outline.unit_id);
            out.push_str(&outline.text);
            let check = match self.markers.outlines.get(&outline.unit_id) {
                Some(verdicts) if verdicts.len() > 1 => "CONFLICT".to_string(),
                Some(verdicts) => match verdicts.first() {
                    Some(OutlineVerdict::Pass(explanation)) if explanation.trim().is_empty() => {
                        "MISSING explanation".to_string()
                    }
                    Some(OutlineVerdict::Pass(explanation)) => {
                        format!("PASS  {explanation}")
                    }
                    Some(OutlineVerdict::Fail(reason)) if reason.is_empty() => "FAIL".to_string(),
                    Some(OutlineVerdict::Fail(reason)) => format!("FAIL  {reason}"),
                    None => "MISSING".to_string(),
                },
                None => format!(
                    "MISSING  add 'OUTLINE: PASS {} | <responsibility; call order>'",
                    outline.unit_id
                ),
            };
            let _ = writeln!(out, "  CHECK     {check}");
        }

        for finding in &self.hints {
            let _ = writeln!(
                out,
                "  HINT      {}  {}",
                finding.unit_id,
                finding.message()
            );
        }

        for failure in &self.failures {
            let _ = writeln!(out, "  FAILURE  {}  {}", failure.code(), failure.message());
        }
        out.push_str(&self.final_line());
        out
    }

    fn final_line(&self) -> String {
        if self.failures.is_empty() {
            return "outline review: PASS".to_string();
        }
        let fail_reasons: Vec<String> = self
            .failures
            .iter()
            .filter_map(|failure| match failure {
                GateFailure::FailMarker(reason) => Some(reason.clone()),
                GateFailure::OutlineFailed { reason, .. } if !reason.is_empty() => {
                    Some(reason.clone())
                }
                _ => None,
            })
            .collect();
        if !fail_reasons.is_empty() {
            return format!("outline review: FAIL ({})", fail_reasons.join("; "));
        }
        if self
            .failures
            .iter()
            .all(|failure| matches!(failure, GateFailure::MissingOutline { .. }))
        {
            let count = self.failures.len();
            let noun = if count == 1 { "outline" } else { "outlines" };
            return format!("outline review: FAIL ({count} {noun} without a verdict)");
        }
        if self.failures.len() == 1 {
            return format!("outline review: FAIL ({})", self.failures[0].message());
        }
        format!(
            "outline review: FAIL ({} failures; first: {})",
            self.failures.len(),
            self.failures[0].message()
        )
    }
}

/// Evaluate the gate. Returns the pass flag and the complete report text.
/// Gate failures are data, not `Err`; only infrastructure errors propagate.
pub fn evaluate(project_root: &Path, change_id: &str) -> Result<(bool, String)> {
    let request = GateRequest::new(project_root, change_id);
    let outcome = review(&request)?;
    let text = outcome.render(&request);
    Ok((outcome.passed(), text))
}

/// Run the gate for `givn check review`: print the complete report and bail
/// with its final line when the gate fails.
pub fn enforce(project_root: &Path, change_id: &str) -> Result<()> {
    let (passed, text) = evaluate(project_root, change_id)?;
    println!("{text}");
    if !passed {
        let last = text.lines().last().unwrap_or("outline review: FAIL");
        bail!("{last}");
    }
    Ok(())
}

fn review(request: &GateRequest) -> Result<GateOutcome> {
    if !request.change_dir.is_dir() {
        bail!(
            "change directory 'givn/changes/{}/' does not exist",
            request.change_id
        );
    }
    let backend = scm::select(&request.project_root)?;
    let base = match base::resolve(&backend, &request.change_id)? {
        base::Resolution::Resolved(base) => base,
        base::Resolution::NoCommit => {
            return Ok(GateOutcome::early(
                None,
                GateFailure::NoChangeCommit(request.change_id.clone()),
            ));
        }
        base::Resolution::Shallow => {
            return Ok(GateOutcome::early(None, GateFailure::ShallowRepository));
        }
    };

    let dirty = head::dirty_production(&backend, request)?;
    if !dirty.is_empty() {
        return Ok(GateOutcome::early(
            Some(base),
            GateFailure::DirtyProduction(dirty),
        ));
    }

    if let Some(reason) = &request.scope_error {
        // A change with no touched production path has no outline obligation.
        // Probe the conventional `src` root before requiring the command, so
        // enabling the outline addon by default does not force an
        // OUTLINE-COMMAND on archive-only or documentation-only changes.
        let probe_roots = vec!["src".to_string()];
        let probe_changed =
            head::changed_production_paths(&backend, base.parent.as_deref(), &probe_roots);
        if matches!(probe_changed, Ok(changed) if changed.is_empty()) {
            return Ok(GateOutcome::pass(Some(base)));
        }
        return Ok(GateOutcome::early(
            None,
            GateFailure::MissingOutlineScope(reason.clone()),
        ));
    }
    if request.roots.is_empty() {
        return Ok(GateOutcome::early(
            None,
            GateFailure::MissingOutlineScope(
                "review.md must record `OUTLINE-COMMAND: givn outline report --path <source>`"
                    .to_string(),
            ),
        ));
    }

    let roots = request.root_strings();
    let mut failures = Vec::new();

    let base_tree = match &base.parent {
        Some(parent) => revision::materialize(&backend, parent, Side::Base, &roots)?,
        None => revision::empty(Side::Base)?,
    };
    let head_tree = revision::materialize(&backend, "HEAD", Side::Head, &roots)?;
    failures.extend(base_tree.failures.iter().cloned());
    failures.extend(head_tree.failures.iter().cloned());
    if !failures.is_empty() {
        let mut outcome = GateOutcome::early(Some(base), failures.remove(0));
        outcome.failures.extend(failures);
        sort_failures(&mut outcome.failures);
        return Ok(outcome);
    }

    let (base_analysis, base_assessment) = analyze(&base_tree, request)?;
    let (head_analysis, head_assessment) = analyze(&head_tree, request)?;
    if head_analysis.files.is_empty() {
        return Ok(GateOutcome::early(
            Some(base),
            GateFailure::NoProductionSources(roots.join(", ")),
        ));
    }

    let changed = match head::changed_production_paths(&backend, base.parent.as_deref(), &roots) {
        Ok(changed) => changed,
        Err(error) => {
            return Ok(GateOutcome::early(
                Some(base),
                base::path_failure(&error, Side::Head),
            ));
        }
    };
    failures.extend(findings::guard(
        &changed,
        &base_tree,
        &head_tree,
        &base_analysis,
        &head_analysis,
    ));
    if !failures.is_empty() {
        let mut outcome = GateOutcome::early(Some(base), failures.remove(0));
        outcome.failures.extend(failures);
        sort_failures(&mut outcome.failures);
        return Ok(outcome);
    }

    let touched = touched::select(&head_analysis, &base_assessment, &changed);
    let hints = findings::collect(&touched.units, &base_assessment, &head_assessment);
    let index = crate::outline::report::MethodIndex::from_assessment(&head_assessment);
    let outlines: Vec<OutlineEntry> = touched
        .units
        .iter()
        .filter_map(|id| {
            head_assessment
                .units
                .iter()
                .find(|unit| &unit.unit_id == id)
        })
        .filter(|unit| !unit.method_names.is_empty())
        .map(|unit| OutlineEntry {
            unit_id: unit.unit_id.clone(),
            text: crate::outline::report::render_outline(unit, &index),
        })
        .collect();
    outline_outcome(
        request,
        base,
        touched.units.len(),
        hints,
        outlines,
        failures,
    )
}

fn validate_outline_identity(
    request: &GateRequest,
    outline_author: &str,
    outline_reviewer: &str,
    failures: &mut Vec<GateFailure>,
) {
    let review_path = request.change_dir.join("review.md");
    let record = match crate::review_context::parse(&review_path) {
        Ok(record) => record,
        Err(error) => {
            failures.push(GateFailure::InvalidReviewContext(error.to_string()));
            return;
        }
    };
    match record.kind {
        None => {
            if outline_author == outline_reviewer {
                failures.push(GateFailure::SelfOutlineReview {
                    author: outline_author.to_string(),
                    reviewer: outline_reviewer.to_string(),
                });
            }
        }
        Some(crate::review_context::ReviewKind::SelfReview) => {
            let (Some(recorded_author), Some(recorded_reviewer)) =
                (&record.author, &record.reviewer)
            else {
                failures.push(GateFailure::MissingRecordedReviewContext);
                return;
            };
            if outline_author != recorded_author {
                failures.push(GateFailure::OutlineAuthorMismatch {
                    outline: outline_author.to_string(),
                    recorded: recorded_author.clone(),
                });
            }
            if outline_reviewer != recorded_reviewer {
                failures.push(GateFailure::OutlineReviewerMismatch {
                    outline: outline_reviewer.to_string(),
                    recorded: recorded_reviewer.clone(),
                });
            }
        }
        Some(crate::review_context::ReviewKind::Independent) => {
            let (Some(recorded_author), Some(recorded_reviewer)) =
                (&record.author, &record.reviewer)
            else {
                failures.push(GateFailure::MissingRecordedReviewContext);
                return;
            };
            if outline_author != recorded_author {
                failures.push(GateFailure::OutlineAuthorMismatch {
                    outline: outline_author.to_string(),
                    recorded: recorded_author.clone(),
                });
            }
            if outline_reviewer != recorded_reviewer {
                failures.push(GateFailure::OutlineReviewerMismatch {
                    outline: outline_reviewer.to_string(),
                    recorded: recorded_reviewer.clone(),
                });
            }
            if outline_author == outline_reviewer {
                failures.push(GateFailure::SelfOutlineReview {
                    author: outline_author.to_string(),
                    reviewer: outline_reviewer.to_string(),
                });
            }
        }
    }
}

fn outline_outcome(
    request: &GateRequest,
    base: ChangeBase,
    touched: usize,
    hints: Vec<GateFinding>,
    outlines: Vec<OutlineEntry>,
    mut failures: Vec<GateFailure>,
) -> Result<GateOutcome> {
    let markers = markers::parse(&request.change_dir);
    if !markers.present {
        failures.push(GateFailure::MissingRecord(request.change_id.clone()));
    } else {
        if !markers.outlines.is_empty() {
            match (&markers.author, &markers.reviewer) {
                (None, _) => failures.push(GateFailure::MissingOutlineAuthor),
                (_, None) => failures.push(GateFailure::MissingOutlineReviewer),
                (Some(author), Some(reviewer)) => {
                    validate_outline_identity(request, author, reviewer, &mut failures);
                }
            }
        }
        for outline in &outlines {
            let unit = outline.unit_id.to_string();
            match markers.outlines.get(&outline.unit_id) {
                None => failures.push(GateFailure::MissingOutline { unit }),
                Some(verdicts) if verdicts.len() > 1 => {
                    failures.push(GateFailure::ConflictingOutline { unit })
                }
                Some(verdicts) => match &verdicts[0] {
                    OutlineVerdict::Pass(explanation) if explanation.trim().is_empty() => {
                        failures.push(GateFailure::EmptyOutlineExplanation { unit })
                    }
                    OutlineVerdict::Pass(_) => {}
                    OutlineVerdict::Fail(reason) => failures.push(GateFailure::OutlineFailed {
                        unit,
                        reason: reason.clone(),
                    }),
                },
            }
        }
    }
    failures.extend(
        markers
            .unknown
            .iter()
            .cloned()
            .map(GateFailure::UnknownMarker),
    );
    failures.extend(markers.fail.iter().cloned().map(GateFailure::FailMarker));
    let missing_verdict = markers.present
        && !outlines.is_empty()
        && !matches!(markers.verdict, Some(Verdict::Pass))
        && markers.fail.is_empty()
        && markers.unknown.is_empty()
        && !failures.iter().any(|failure| {
            matches!(
                failure,
                GateFailure::MissingOutline { .. }
                    | GateFailure::ConflictingOutline { .. }
                    | GateFailure::OutlineFailed { .. }
                    | GateFailure::EmptyOutlineExplanation { .. }
                    | GateFailure::MissingOutlineAuthor
                    | GateFailure::MissingOutlineReviewer
                    | GateFailure::SelfOutlineReview { .. }
            )
        });
    if missing_verdict {
        failures.push(GateFailure::MissingVerdict);
    }

    sort_failures(&mut failures);
    Ok(GateOutcome {
        base: Some(base),
        analyzed: true,
        touched,
        hints,
        outlines,
        markers,
        failures,
    })
}

fn sort_failures(failures: &mut Vec<GateFailure>) {
    failures.sort_by_key(GateFailure::sort_key);
    failures.dedup();
}

fn analyze(tree: &revision::RevisionTree, request: &GateRequest) -> Result<(Analysis, Assessment)> {
    let analysis_request = AnalysisRequest {
        project_root: tree.root.path().to_path_buf(),
        scope: SourceScope {
            languages: vec![Language::Rust, Language::Python],
            roots: request.roots.clone(),
            include_test_sources: false,
        },
    };
    let analyzer = outline::outline_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = outline::analyze(&analysis_request, analyzer.as_ref()).with_context(|| {
        format!(
            "analyze the revision tree at {}",
            tree.root.path().display()
        )
    })?;
    let assessment = outline::assess(&analysis, capabilities);
    Ok((analysis, assessment))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome(mut failures: Vec<GateFailure>) -> GateOutcome {
        sort_failures(&mut failures);
        GateOutcome {
            base: None,
            analyzed: false,
            touched: 0,
            hints: Vec::new(),
            outlines: Vec::new(),
            markers: ReviewMarkers::default(),
            failures,
        }
    }

    #[test]
    fn the_final_line_counts_missing_outlines() {
        let one = outcome(vec![GateFailure::MissingOutline {
            unit: "type:src/engine.rs::Engine".to_string(),
        }]);
        assert_eq!(
            one.final_line(),
            "outline review: FAIL (1 outline without a verdict)"
        );
        let two = outcome(vec![
            GateFailure::MissingOutline {
                unit: "module:src/engine.rs".to_string(),
            },
            GateFailure::MissingOutline {
                unit: "type:src/engine.rs::Engine".to_string(),
            },
        ]);
        assert_eq!(
            two.final_line(),
            "outline review: FAIL (2 outlines without a verdict)"
        );
    }

    #[test]
    fn the_final_line_keeps_a_single_failure_message() {
        let unknown = outcome(vec![GateFailure::UnknownMarker(
            "OUTLINE: DEFERRED type:src/engine.rs::Engine".to_string(),
        )]);
        assert_eq!(
            unknown.final_line(),
            "outline review: FAIL (unknown outline marker: OUTLINE: DEFERRED type:src/engine.rs::Engine)"
        );
        let verdict = outcome(vec![GateFailure::MissingVerdict]);
        assert_eq!(
            verdict.final_line(),
            "outline review: FAIL (missing 'OUTLINE: PASS')"
        );
    }

    #[test]
    fn the_final_line_aggregates_several_failures_and_fail_wins() {
        let failed = GateFailure::OutlineFailed {
            unit: "type:src/engine.rs::Engine".to_string(),
            reason: "the call order is not explainable".to_string(),
        };
        let empty = GateFailure::EmptyOutlineExplanation {
            unit: "type:src/cohesive.rs::Cohesive".to_string(),
        };
        let aggregate = outcome(vec![empty, failed]);
        assert_eq!(
            aggregate.final_line(),
            "outline review: FAIL (the call order is not explainable)"
        );
        let marker = outcome(vec![
            GateFailure::MissingOutline {
                unit: "type:src/cohesive.rs::Cohesive".to_string(),
            },
            GateFailure::FailMarker("the type has no single responsibility".to_string()),
        ]);
        assert_eq!(
            marker.final_line(),
            "outline review: FAIL (the type has no single responsibility)"
        );
    }

    #[test]
    fn a_clean_outcome_ends_with_the_exact_pass_line() {
        let clean = outcome(Vec::new());
        assert_eq!(clean.final_line(), "outline review: PASS");
    }
}
