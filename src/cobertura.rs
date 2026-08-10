/// Cobertura XML parsing, merging, and report generation.
use anyhow::{Context, Result, bail};
use std::path::Path;

/// Coverage summary extracted from a Cobertura XML report.
#[derive(Debug, Clone, Default)]
pub struct CoberturaReport {
    pub line_rate: f64,
    pub branch_rate: f64,
    pub lines_covered: u64,
    pub lines_valid: u64,
    pub branches_covered: u64,
    pub branches_valid: u64,
}

/// Parse a Cobertura XML file and extract coverage statistics.
pub fn parse(path: &Path) -> Result<CoberturaReport> {
    let xml = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read Cobertura XML: {}", path.display()))?;

    quick_xml::de::from_str::<CoberturaXml>(&xml)
        .map(|r| CoberturaReport {
            line_rate: r.line_rate,
            branch_rate: r.branch_rate,
            lines_covered: r.lines_covered,
            lines_valid: r.lines_valid,
            branches_covered: r.branches_covered,
            branches_valid: r.branches_valid,
        })
        .with_context(|| format!("failed to parse Cobertura XML: {}", path.display()))
}

/// Validate that coverage data is genuine (non-zero files/lines measured).
pub fn validate(report: &CoberturaReport) -> Result<()> {
    if report.lines_valid == 0 {
        bail!(
            "Cobertura XML shows zero lines measured (lines-valid=0). \
             Coverage instrumentation produced no data."
        );
    }
    let expected_line_rate = report.lines_covered as f64 / report.lines_valid as f64;
    if (report.line_rate - expected_line_rate).abs() > 0.01 {
        bail!(
            "Cobertura XML line-rate={} does not match lines-covered/lines-valid={}",
            report.line_rate,
            expected_line_rate
        );
    }
    if report.branches_valid > 0 {
        let expected_branch_rate = report.branches_covered as f64 / report.branches_valid as f64;
        if (report.branch_rate - expected_branch_rate).abs() > 0.01 {
            bail!(
                "Cobertura XML branch-rate={} does not match branches-covered/branches-valid={}",
                report.branch_rate,
                expected_branch_rate
            );
        }
    }
    Ok(())
}

/// Check that two coverage reports are structurally different — i.e. not the
/// same instrumentation run copied to both paths. E2e coverage must come from
/// running the instrumented server, not from duplicating the non-e2e output.
/// Two reports are considered identical when all four counters match exactly
/// (lines_covered, lines_valid, branches_covered, branches_valid).
pub fn assert_distinct(a: &CoberturaReport, b: &CoberturaReport) -> Result<()> {
    if a.lines_covered == b.lines_covered
        && a.lines_valid == b.lines_valid
        && a.branches_covered == b.branches_covered
        && a.branches_valid == b.branches_valid
    {
        bail!(
            "e2e and non-e2e coverage reports are identical (lines_covered={}, \
             lines_valid={}, branches_covered={}, branches_valid={}). \
             E2e coverage must come from an instrumented server run, not from \
             copying the non-e2e coverage output. Configure \
             coverage.measure_command to run both test subsets against instrumented \
             production processes and collect their outputs.",
            a.lines_covered,
            a.lines_valid,
            a.branches_covered,
            a.branches_valid,
        );
    }
    Ok(())
}

/// Preserve the pre-script behavior for projects that are migrating from the
/// two-command coverage schema. New projects use their merge script instead.
pub(crate) fn merge_legacy_reports(a: &CoberturaReport, b: &CoberturaReport) -> CoberturaReport {
    let lines_covered = a.lines_covered + b.lines_covered;
    let lines_valid = a.lines_valid + b.lines_valid;
    let branches_covered = a.branches_covered + b.branches_covered;
    let branches_valid = a.branches_valid + b.branches_valid;

    CoberturaReport {
        line_rate: if lines_valid > 0 {
            lines_covered as f64 / lines_valid as f64
        } else {
            0.0
        },
        branch_rate: if branches_valid > 0 {
            branches_covered as f64 / branches_valid as f64
        } else {
            0.0
        },
        lines_covered,
        lines_valid,
        branches_covered,
        branches_valid,
    }
}

/// Render a CoberturaReport as a short markdown summary for the README fence.
pub fn render_markdown(report: &CoberturaReport) -> String {
    let line_pct = percentage(report.line_rate, report.lines_valid);
    let branch_pct = percentage(report.branch_rate, report.branches_valid);

    format!(
        "Line coverage: {line_pct} ({lines_cov}/{lines_val})\n\
         Branch coverage: {branch_pct} ({branches_cov}/{branches_val})",
        line_pct = line_pct,
        lines_cov = report.lines_covered,
        lines_val = report.lines_valid,
        branch_pct = branch_pct,
        branches_cov = report.branches_covered,
        branches_val = report.branches_valid,
    )
}

/// Render the two README badges from the authoritative merged report.
pub fn render_badges(report: &CoberturaReport, report_path: &str) -> String {
    let line_pct = percentage(report.line_rate, report.lines_valid);
    let branch_pct = percentage(report.branch_rate, report.branches_valid);
    let line_badge = badge_value(&line_pct);
    let branch_badge = badge_value(&branch_pct);

    format!(
        "[![Line Coverage: {line_pct}](https://img.shields.io/badge/line%20coverage-{line_badge}-brightgreen)]({report_path})\n\
         [![Branch Coverage: {branch_pct}](https://img.shields.io/badge/branch%20coverage-{branch_badge}-brightgreen)]({report_path})",
        line_pct = line_pct,
        branch_pct = branch_pct,
        line_badge = line_badge,
        branch_badge = branch_badge,
        report_path = report_path,
    )
}

/// Render the managed README coverage section from the merged report and the
/// exact commands that produced it.
pub fn render_readme_summary(
    report: &CoberturaReport,
    report_path: &str,
    measure_command: &str,
    merge_command: &str,
) -> String {
    format!(
        "## Coverage\n\nMerged report: [{report_path}]({report_path})\n\n{summary}\n\n\
         Reproduce the result:\n\n```sh\n{measure_command}\n{merge_command}\n```",
        report_path = report_path,
        summary = render_markdown(report),
        measure_command = measure_command,
        merge_command = merge_command,
    )
}

fn percentage(rate: f64, denominator: u64) -> String {
    if denominator == 0 {
        "n/a".to_string()
    } else {
        format!("{}%", (rate * 100.0).round() as u64)
    }
}

fn badge_value(percentage: &str) -> String {
    let value = percentage.strip_suffix('%').unwrap_or(percentage);
    if value == "n/a" {
        "n--a".to_string()
    } else {
        format!("{}%25", value.replace('/', "%2F"))
    }
}

// ---------------------------------------------------------------------------
// XML deserialization types — mirrors the subset we care about
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "coverage")]
struct CoberturaXml {
    #[serde(rename = "@line-rate")]
    line_rate: f64,
    #[serde(rename = "@branch-rate")]
    branch_rate: f64,
    #[serde(rename = "@lines-covered")]
    lines_covered: u64,
    #[serde(rename = "@lines-valid")]
    lines_valid: u64,
    #[serde(rename = "@branches-covered")]
    branches_covered: u64,
    #[serde(rename = "@branches-valid")]
    branches_valid: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_cobertura(
        path: &Path,
        line_rate: f64,
        branch_rate: f64,
        lc: u64,
        lv: u64,
        bc: u64,
        bv: u64,
    ) {
        let xml = format!(
            r#"<?xml version="1.0"?>
<coverage line-rate="{lr}" branch-rate="{br}" lines-covered="{lc}" lines-valid="{lv}" branches-covered="{bc}" branches-valid="{bv}" timestamp="0" version="1">
  <packages />
</coverage>"#,
            lr = line_rate,
            br = branch_rate,
            lc = lc,
            lv = lv,
            bc = bc,
            bv = bv
        );
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(xml.as_bytes()).unwrap();
    }

    #[test]
    fn parse_valid_cobertura() {
        let dir = std::env::temp_dir().join("givn_cobertura_test1");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("cobertura.xml");
        write_cobertura(&path, 0.85, 0.72, 340, 400, 36, 50);

        let report = parse(&path).unwrap();
        assert!((report.line_rate - 0.85).abs() < 0.001);
        assert!((report.branch_rate - 0.72).abs() < 0.001);
        assert_eq!(report.lines_covered, 340);
        assert_eq!(report.lines_valid, 400);
    }

    #[test]
    fn parse_empty_cobertura_fails_validation() {
        let dir = std::env::temp_dir().join("givn_cobertura_test2");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("cobertura.xml");
        write_cobertura(&path, 0.0, 0.0, 0, 0, 0, 0);

        let report = parse(&path).unwrap();
        assert!(validate(&report).is_err());
    }

    #[test]
    fn validate_rejects_rates_that_disagree_with_counters() {
        let report = CoberturaReport {
            line_rate: 0.1,
            branch_rate: 0.5,
            lines_covered: 9,
            lines_valid: 10,
            branches_covered: 1,
            branches_valid: 2,
        };
        assert!(validate(&report).is_err());
    }

    #[test]
    fn assert_distinct_rejects_identical_reports() {
        let a = CoberturaReport {
            line_rate: 0.85,
            branch_rate: 0.72,
            lines_covered: 340,
            lines_valid: 400,
            branches_covered: 36,
            branches_valid: 50,
        };
        let b = CoberturaReport {
            line_rate: 0.85,
            branch_rate: 0.72,
            lines_covered: 340,
            lines_valid: 400,
            branches_covered: 36,
            branches_valid: 50,
        };
        assert!(assert_distinct(&a, &b).is_err());
    }

    #[test]
    fn assert_distinct_passes_when_counters_differ() {
        let a = CoberturaReport {
            line_rate: 0.85,
            branch_rate: 0.72,
            lines_covered: 340,
            lines_valid: 400,
            branches_covered: 36,
            branches_valid: 50,
        };
        let b = CoberturaReport {
            line_rate: 0.9,
            branch_rate: 0.8,
            lines_covered: 360,
            lines_valid: 400,
            branches_covered: 40,
            branches_valid: 50,
        };
        assert!(assert_distinct(&a, &b).is_ok());
    }

    #[test]
    fn assert_distinct_passes_when_only_lines_differ() {
        let a = CoberturaReport {
            line_rate: 0.85,
            branch_rate: 0.72,
            lines_covered: 340,
            lines_valid: 400,
            branches_covered: 36,
            branches_valid: 50,
        };
        let b = CoberturaReport {
            line_rate: 0.0,
            branch_rate: 0.0,
            lines_covered: 0,
            lines_valid: 0,
            branches_covered: 36,
            branches_valid: 50,
        };
        assert!(assert_distinct(&a, &b).is_ok());
    }

    #[test]
    fn render_markdown_produces_summary() {
        let report = CoberturaReport {
            line_rate: 0.852,
            branch_rate: 0.723,
            lines_covered: 340,
            lines_valid: 400,
            branches_covered: 36,
            branches_valid: 50,
        };
        let md = render_markdown(&report);
        assert!(md.contains("85%"));
        assert!(md.contains("72%"));
        assert!(md.contains("340/400"));
        assert!(md.contains("36/50"));
    }

    #[test]
    fn render_markdown_does_not_claim_zero_percent_for_unmeasured_branches() {
        let report = CoberturaReport {
            line_rate: 0.5,
            branch_rate: 0.0,
            lines_covered: 1,
            lines_valid: 2,
            branches_covered: 0,
            branches_valid: 0,
        };
        let md = render_markdown(&report);
        assert!(md.contains("Branch coverage: n/a (0/0)"));
    }

    #[test]
    fn render_badges_use_both_metrics_and_the_merged_report() {
        let report = CoberturaReport {
            line_rate: 0.95,
            branch_rate: 0.9,
            lines_covered: 95,
            lines_valid: 100,
            branches_covered: 9,
            branches_valid: 10,
        };
        let badges = render_badges(&report, "coverage/cobertura-coverage.xml");
        assert!(badges.contains("Line Coverage: 95%"));
        assert!(badges.contains("Branch Coverage: 90%"));
        assert_eq!(badges.matches("coverage/cobertura-coverage.xml").count(), 2);
    }

    #[test]
    fn render_readme_summary_contains_merged_report_and_scripts() {
        let report = CoberturaReport {
            line_rate: 0.95,
            branch_rate: 0.9,
            lines_covered: 95,
            lines_valid: 100,
            branches_covered: 9,
            branches_valid: 10,
        };
        let summary = render_readme_summary(
            &report,
            "coverage/cobertura-coverage.xml",
            "./measure-coverage.sh",
            "./merge-coverages.sh",
        );
        assert!(summary.contains("Line coverage: 95% (95/100)"));
        assert!(summary.contains("Branch coverage: 90% (9/10)"));
        assert!(summary.contains("./measure-coverage.sh"));
        assert!(summary.contains("./merge-coverages.sh"));
    }
}
