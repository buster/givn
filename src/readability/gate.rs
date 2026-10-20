//! Change-relative readability review gate.
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
mod rationale;
mod revision;
mod touched;

use crate::readability::classify::Assessment;
use crate::readability::model::{Analysis, Language, SourceScope};
use crate::readability::{self, AnalysisRequest};
use crate::scm;
use anyhow::{Context, Result, bail};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub use findings::{Decision, FindingReason, GateFinding};
pub use markers::{ReviewMarkers, Verdict};

/// One review run. `roots` defaults to the production `src/` root.
#[derive(Clone, Debug)]
pub struct GateRequest {
    pub project_root: PathBuf,
    pub change_id: String,
    pub change_dir: PathBuf,
    pub roots: Vec<PathBuf>,
}

impl GateRequest {
    pub fn new(project_root: &Path, change_id: &str) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
            change_id: change_id.to_string(),
            change_dir: project_root.join("givn/changes").join(change_id),
            roots: vec![PathBuf::from("src")],
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
                "readability record is missing; create givn/changes/{change_id}/review.md \
                 and record 'READABILITY: PASS'"
            ),
            Self::UnknownMarker(line) => format!("unknown readability marker: {line}"),
            Self::MissingVerdict => "missing 'READABILITY: PASS'".to_string(),
            Self::FailMarker(reason) => reason.clone(),
            Self::SlugCollision { first, second } => {
                format!("rationale slug collision for {first} and {second}")
            }
            Self::InvalidRationale { path, message } => format!("rationale {path}: {message}"),
            Self::MissingRationale { unit, path } => {
                format!("JUSTIFIED marker for {unit} has no rationale at {path}")
            }
            Self::JustifiedWithoutFinding { unit } => format!(
                "JUSTIFIED marker for {unit}, but {unit} is not a finding at head; \
                 use READABILITY: FIXED {unit}"
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

/// The complete gate result. The CLI renders it verbatim.
#[derive(Debug)]
pub struct GateOutcome {
    pub base: Option<ChangeBase>,
    pub analyzed: bool,
    pub touched: usize,
    pub findings: Vec<GateFinding>,
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
            findings: Vec::new(),
            markers: ReviewMarkers::default(),
            failures: vec![failure],
        }
    }

    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn render(&self, request: &GateRequest) -> String {
        let mut out = String::from("Readability review gate\n");
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
                    "  touched {} unit(s), {} finding(s)",
                    self.touched,
                    self.findings.len()
                );
            }
        }

        for finding in &self.findings {
            let _ = writeln!(
                out,
                "  FINDING   {}  {}",
                finding.unit_id,
                finding.message()
            );
            let rationale = rationale::relative_path(&request.change_id, &finding.unit_id);
            let decision = match &finding.decision {
                Decision::Missing => format!(
                    "  DECISION  {:<11}{}  {}",
                    "MISSING",
                    finding.unit_id,
                    missing_suggestion(finding, &rationale)
                ),
                Decision::Fixed { .. } => format!(
                    "  DECISION  {:<11}{}  marker READABILITY: FIXED {}",
                    "FIXED", finding.unit_id, finding.unit_id
                ),
                Decision::Justified { rationale } => format!(
                    "  DECISION  {:<11}{}  rationale {}",
                    "JUSTIFIED",
                    finding.unit_id,
                    display_path(rationale)
                ),
            };
            out.push_str(&decision);
            out.push('\n');
        }

        for failure in &self.failures {
            let _ = writeln!(out, "  FAILURE  {}  {}", failure.code(), failure.message());
        }
        out.push_str(&self.final_line());
        out
    }

    fn final_line(&self) -> String {
        if self.failures.is_empty() {
            return "readability review: PASS".to_string();
        }
        let fail_reasons: Vec<String> = self
            .failures
            .iter()
            .filter_map(|failure| match failure {
                GateFailure::FailMarker(reason) => Some(reason.clone()),
                _ => None,
            })
            .collect();
        if !fail_reasons.is_empty() {
            return format!("readability review: FAIL ({})", fail_reasons.join("; "));
        }
        if self
            .failures
            .iter()
            .all(|failure| matches!(failure, GateFailure::Unaccounted { .. }))
        {
            let count = self.failures.len();
            let noun = if count == 1 { "finding" } else { "findings" };
            return format!("readability review: FAIL ({count} {noun} without a decision)");
        }
        if self.failures.len() == 1 {
            return format!("readability review: FAIL ({})", self.failures[0].message());
        }
        format!(
            "readability review: FAIL ({} failures; first: {})",
            self.failures.len(),
            self.failures[0].message()
        )
    }
}

fn missing_suggestion(finding: &GateFinding, rationale: &str) -> String {
    let unit = &finding.unit_id;
    if finding.is_head_finding() {
        format!(
            "add 'READABILITY: FIXED {unit}' or 'READABILITY: JUSTIFIED {unit}' and create {rationale}"
        )
    } else {
        format!("add 'READABILITY: FIXED {unit}'")
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
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
        let last = text.lines().last().unwrap_or("readability review: FAIL");
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

    let touched = touched::select(&head_analysis, &base_assessment, &changed);
    let gate_findings = findings::collect(&touched.units, &base_assessment, &head_assessment);
    decide_outcome(request, base, touched.units.len(), gate_findings, failures)
}

fn decide_outcome(
    request: &GateRequest,
    base: ChangeBase,
    touched: usize,
    mut gate_findings: Vec<GateFinding>,
    mut failures: Vec<GateFailure>,
) -> Result<GateOutcome> {
    let markers = markers::parse(&request.change_dir);
    let record_missing = !markers.present && !gate_findings.is_empty();
    failures.extend(findings::decide(
        &mut gate_findings,
        &markers,
        request,
        record_missing,
    ));
    if record_missing {
        failures.push(GateFailure::MissingRecord(request.change_id.clone()));
    }
    let unknown = markers.unknown.clone();
    failures.extend(unknown.into_iter().map(GateFailure::UnknownMarker));
    failures.extend(markers.fail.iter().cloned().map(GateFailure::FailMarker));
    // A missing verdict is the actionable failure only when every finding was
    // otherwise decided. The complete MISSING block in the design ends with
    // the missing-decision count, not a second "missing PASS" failure.
    let missing_verdict = !gate_findings.is_empty()
        && !matches!(markers.verdict, Some(Verdict::Pass))
        && markers.fail.is_empty()
        && !record_missing
        && markers.unknown.is_empty()
        && !failures.iter().any(|failure| {
            matches!(
                failure,
                GateFailure::Unaccounted { .. }
                    | GateFailure::InvalidFixed { .. }
                    | GateFailure::ConflictingDecisions { .. }
                    | GateFailure::MissingRationale { .. }
                    | GateFailure::InvalidRationale { .. }
                    | GateFailure::JustifiedWithoutFinding { .. }
                    | GateFailure::SlugCollision { .. }
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
        findings: gate_findings,
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
            language: Language::Rust,
            roots: request.roots.clone(),
            include_test_sources: false,
        },
    };
    let analyzer = readability::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis =
        readability::analyze(&analysis_request, analyzer.as_ref()).with_context(|| {
            format!(
                "analyze the revision tree at {}",
                tree.root.path().display()
            )
        })?;
    let assessment = readability::assess(&analysis, capabilities);
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
            findings: Vec::new(),
            markers: ReviewMarkers::default(),
            failures,
        }
    }

    #[test]
    fn the_final_line_counts_missing_decisions() {
        let one = outcome(vec![GateFailure::Unaccounted {
            unit: "type:src/engine.rs::Engine".to_string(),
        }]);
        assert_eq!(
            one.final_line(),
            "readability review: FAIL (1 finding without a decision)"
        );
        let two = outcome(vec![
            GateFailure::Unaccounted {
                unit: "module:src/engine.rs".to_string(),
            },
            GateFailure::Unaccounted {
                unit: "type:src/engine.rs::Engine".to_string(),
            },
        ]);
        assert_eq!(
            two.final_line(),
            "readability review: FAIL (2 findings without a decision)"
        );
    }

    #[test]
    fn the_final_line_keeps_a_single_failure_message() {
        let unknown = outcome(vec![GateFailure::UnknownMarker(
            "READABILITY: DEFERRED type:src/engine.rs::Engine".to_string(),
        )]);
        assert_eq!(
            unknown.final_line(),
            "readability review: FAIL (unknown readability marker: READABILITY: DEFERRED type:src/engine.rs::Engine)"
        );
        let verdict = outcome(vec![GateFailure::MissingVerdict]);
        assert_eq!(
            verdict.final_line(),
            "readability review: FAIL (missing 'READABILITY: PASS')"
        );
    }

    #[test]
    fn the_final_line_aggregates_several_failures_and_fail_wins() {
        let invalid = GateFailure::InvalidFixed {
            unit: "type:src/engine.rs::Engine".to_string(),
            classification: "god-type".to_string(),
            head: 700,
            base: 620,
        };
        let unaccounted = GateFailure::Unaccounted {
            unit: "type:src/cohesive.rs::Cohesive".to_string(),
        };
        let aggregate = outcome(vec![unaccounted.clone(), invalid.clone()]);
        assert_eq!(
            aggregate.final_line(),
            "readability review: FAIL (2 failures; first: FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 700 code lines (base 620))"
        );
        let failed = outcome(vec![
            unaccounted,
            GateFailure::FailMarker("the type has no single responsibility".to_string()),
        ]);
        assert_eq!(
            failed.final_line(),
            "readability review: FAIL (the type has no single responsibility)"
        );
    }

    #[test]
    fn a_clean_outcome_ends_with_the_exact_pass_line() {
        let clean = outcome(Vec::new());
        assert_eq!(clean.final_line(), "readability review: PASS");
    }
}
