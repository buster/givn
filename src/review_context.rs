//! Review-context record for a change.
//!
//! The proposal records the requested review kind; the design review and the
//! review record the performed kind, the author, and the reviewer context.
//! The gate reads those records and never infers independence from other
//! markers. A change with no recorded kind keeps its previous gate result.

use anyhow::{Result, bail};
use std::path::Path;

/// How a change's review was performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviewKind {
    SelfReview,
    Independent,
}

impl ReviewKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::SelfReview => "self-review",
            Self::Independent => "independent",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "self-review" | "self review" => Some(Self::SelfReview),
            "independent" => Some(Self::Independent),
            _ => None,
        }
    }
}

/// One artifact's review record. Every field is optional so a legacy record
/// and a partially written record are distinguishable.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReviewRecord {
    pub kind: Option<ReviewKind>,
    pub author: Option<String>,
    pub reviewer: Option<String>,
}

/// Remove fenced examples so a template scaffold never satisfies the gate.
fn strip_fenced_blocks(text: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn marker_value(line: &str, marker: &str) -> Option<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    let trimmed = trimmed.trim_matches('*').trim();
    let value = trimmed.strip_prefix(marker)?;
    Some(
        value
            .trim_matches(|c: char| c == '*' || c == '`' || c.is_whitespace())
            .to_string(),
    )
}

fn parse_kind(value: &str, path: &Path) -> Result<ReviewKind> {
    ReviewKind::parse(value).ok_or_else(|| {
        anyhow::anyhow!(
            "{}: 'Review kind: {value}' is not valid — use 'Review kind: self-review' or 'Review kind: independent'",
            path.display()
        )
    })
}

/// Parse one artifact's review record. A missing file yields the default
/// empty record; a malformed value is an error.
pub fn parse(path: &Path) -> Result<ReviewRecord> {
    let Ok(raw) = std::fs::read_to_string(path) else {
        return Ok(ReviewRecord::default());
    };
    let text = strip_fenced_blocks(&raw);
    let mut record = ReviewRecord::default();
    for line in text.lines() {
        if let Some(value) = marker_value(line, "Review kind:") {
            record.kind = Some(parse_kind(&value, path)?);
        } else if let Some(value) = marker_value(line, "Author:") {
            record.author = non_empty(value);
        } else if let Some(value) = marker_value(line, "Reviewer context:") {
            record.reviewer = non_empty(value);
        }
    }
    Ok(record)
}

fn non_empty(value: String) -> Option<String> {
    let value = value.trim().to_string();
    (!value.is_empty()).then_some(value)
}

fn required(value: Option<&str>, field: &str, path: &Path) -> Result<String> {
    value.map(str::to_string).ok_or_else(|| {
        anyhow::anyhow!(
            "{} is missing '{}' — record the review context",
            path.display(),
            field
        )
    })
}

/// Validate the review-context records for one change.
pub fn check(change_dir: &Path) -> Result<()> {
    let proposal_path = change_dir.join("proposal.md");
    let design_review_path = change_dir.join("design-review.md");
    let review_path = change_dir.join("review.md");

    let review = match std::fs::read_to_string(&review_path) {
        Ok(_) => parse(&review_path)?,
        Err(_) => return Ok(()),
    };
    let proposal = parse(&proposal_path)?;
    let design_review = if design_review_path.exists() {
        Some(parse(&design_review_path)?)
    } else {
        None
    };

    // Legacy records: neither the proposal nor the review recorded a kind.
    if proposal.kind.is_none() && review.kind.is_none() {
        return Ok(());
    }

    let Some(review_kind) = review.kind else {
        if proposal.kind == Some(ReviewKind::Independent) {
            bail!(
                "independent review is required: the proposal records Review kind: independent, but review.md does not record a Review kind"
            );
        }
        // A proposal that records the default self-review, or a legacy
        // proposal with no kind, accepts a review record that omits the kind.
        // The gate never infers independence from that absence.
        return Ok(());
    };

    if let Some(requested) = proposal.kind
        && requested != review_kind
    {
        if requested == ReviewKind::Independent {
            bail!(
                "independent review is required: the proposal records Review kind: independent, but review.md records Review kind: {}",
                review_kind.label()
            );
        }
        bail!(
            "review.md records Review kind: {}, but the proposal records Review kind: {} — record the requested kind in the proposal",
            review_kind.label(),
            requested.label()
        );
    }

    let author = required(review.author.as_deref(), "Author:", &review_path)?;
    let reviewer = required(
        review.reviewer.as_deref(),
        "Reviewer context:",
        &review_path,
    )?;

    if review_kind == ReviewKind::Independent {
        if reviewer == author {
            bail!(
                "independent review: Reviewer context '{reviewer}' must differ from the author '{author}'"
            );
        }
        if let Some(design) = &design_review {
            match design.kind {
                Some(ReviewKind::Independent) => {}
                Some(other) => bail!(
                    "design-review.md and review.md must record one review context for an independent review: design-review.md records Review kind: {}, review.md records Review kind: independent",
                    other.label()
                ),
                None => bail!(
                    "design-review.md must record 'Review kind: independent' and 'Reviewer context:' — design review and final review share one review context"
                ),
            }
            let design_reviewer = required(
                design.reviewer.as_deref(),
                "Reviewer context:",
                &design_review_path,
            )?;
            if design_reviewer != reviewer {
                bail!(
                    "design-review.md and review.md must record one review context for an independent review: '{design_reviewer}' vs '{reviewer}'"
                );
            }
        }
    } else if let Some(design) = &design_review
        && design.kind == Some(ReviewKind::Independent)
    {
        bail!(
            "design-review.md records Review kind: independent, but review.md records Review kind: self-review — one review context must cover both"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_change(review: &str, proposal: &str, design_review: &str) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("proposal.md"), proposal).unwrap();
        std::fs::write(dir.path().join("design-review.md"), design_review).unwrap();
        std::fs::write(dir.path().join("review.md"), review).unwrap();
        dir
    }

    fn record(kind: &str, author: &str, reviewer: &str) -> String {
        format!("Review kind: {kind}\nAuthor: {author}\nReviewer context: {reviewer}\n")
    }

    #[test]
    fn missing_review_file_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn legacy_record_without_kind_passes() {
        let dir = temp_change("REVIEW: PASS\n", "# Proposal\n", "DESIGN-REVIEW: PASS\n");
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn self_review_proposal_accepts_a_review_without_a_kind() {
        let dir = temp_change(
            "REVIEW: PASS\n",
            "Review kind: self-review\n",
            "DESIGN-REVIEW: PASS\n",
        );
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn independent_proposal_rejects_a_review_without_a_kind() {
        let dir = temp_change(
            "REVIEW: PASS\n",
            "Review kind: independent\n",
            "DESIGN-REVIEW: PASS\n",
        );
        let error = check(dir.path()).unwrap_err().to_string();
        assert!(error.contains("independent review is required"));
    }

    #[test]
    fn self_review_passes() {
        let dir = temp_change(
            &record("self-review", "agent", "agent"),
            "Review kind: self-review\n",
            &format!(
                "{}\nDESIGN-REVIEW: PASS\n",
                record("self-review", "agent", "agent")
            ),
        );
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn independent_review_with_one_context_passes() {
        let dir = temp_change(
            &record("independent", "agent", "reviewer-one"),
            "Review kind: independent\n",
            &format!(
                "{}\nDESIGN-REVIEW: PASS\n",
                record("independent", "agent", "reviewer-one")
            ),
        );
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn independent_review_rejects_the_author() {
        let dir = temp_change(
            &record("independent", "agent", "agent"),
            "Review kind: independent\n",
            &format!(
                "{}\nDESIGN-REVIEW: PASS\n",
                record("independent", "agent", "agent")
            ),
        );
        let error = check(dir.path()).unwrap_err().to_string();
        assert!(error.contains("must differ from the author"));
    }

    #[test]
    fn independent_proposal_rejects_self_review() {
        let dir = temp_change(
            &record("self-review", "agent", "agent"),
            "Review kind: independent\n",
            &format!(
                "{}\nDESIGN-REVIEW: PASS\n",
                record("self-review", "agent", "agent")
            ),
        );
        let error = check(dir.path()).unwrap_err().to_string();
        assert!(error.contains("independent review is required"));
    }

    #[test]
    fn independent_review_needs_one_design_review_context() {
        let dir = temp_change(
            &record("independent", "agent", "reviewer-two"),
            "Review kind: independent\n",
            &format!(
                "{}\nDESIGN-REVIEW: PASS\n",
                record("independent", "agent", "reviewer-one")
            ),
        );
        let error = check(dir.path()).unwrap_err().to_string();
        assert!(error.contains("one review context"));
        assert!(error.contains("reviewer-one"));
        assert!(error.contains("reviewer-two"));
    }

    #[test]
    fn fenced_examples_are_ignored() {
        let dir = temp_change(
            "```\nReview kind: independent\nAuthor: a\nReviewer context: a\n```\nREVIEW: PASS\n",
            "```\nReview kind: independent\n```\n",
            "DESIGN-REVIEW: PASS\n",
        );
        assert!(check(dir.path()).is_ok());
    }

    #[test]
    fn invalid_kind_is_named() {
        let dir = temp_change(
            "Review kind: partly-independent\nREVIEW: PASS\n",
            "",
            "DESIGN-REVIEW: PASS\n",
        );
        let error = check(dir.path()).unwrap_err().to_string();
        assert!(error.contains("partly-independent"));
        assert!(error.contains("self-review"));
    }

    #[test]
    fn parse_strips_marker_decoration() {
        let dir = tempfile::tempdir().unwrap();
        let path: PathBuf = dir.path().join("review.md");
        std::fs::write(
            &path,
            "**Review kind:** `independent`\n**Author:** `agent`\n**Reviewer context:** `reviewer`\n",
        )
        .unwrap();
        let record = parse(&path).unwrap();
        assert_eq!(record.kind, Some(ReviewKind::Independent));
        assert_eq!(record.author.as_deref(), Some("agent"));
        assert_eq!(record.reviewer.as_deref(), Some("reviewer"));
    }
}
