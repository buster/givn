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
             coverage.e2e_command to run your tests against an instrumented \
             application server and collect coverage from the server's output.",
            a.lines_covered,
            a.lines_valid,
            a.branches_covered,
            a.branches_valid,
        );
    }
    Ok(())
}

/// Merge two Cobertura reports by summing counters and recomputing rates.
pub fn merge(a: &CoberturaReport, b: &CoberturaReport) -> CoberturaReport {
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
    let line_pct = (report.line_rate * 100.0).round();
    let branch_pct = (report.branch_rate * 100.0).round();

    format!(
        "Line coverage: {line_pct}% ({lines_cov}/{lines_val})\n\
         Branch coverage: {branch_pct}% ({branches_cov}/{branches_val})",
        line_pct = line_pct,
        lines_cov = report.lines_covered,
        lines_val = report.lines_valid,
        branch_pct = branch_pct,
        branches_cov = report.branches_covered,
        branches_val = report.branches_valid,
    )
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
    fn merge_two_reports() {
        let a = CoberturaReport {
            line_rate: 0.8,
            branch_rate: 0.7,
            lines_covered: 80,
            lines_valid: 100,
            branches_covered: 14,
            branches_valid: 20,
        };
        let b = CoberturaReport {
            line_rate: 0.9,
            branch_rate: 0.8,
            lines_covered: 90,
            lines_valid: 100,
            branches_covered: 16,
            branches_valid: 20,
        };

        let merged = merge(&a, &b);
        assert_eq!(merged.lines_covered, 170);
        assert_eq!(merged.lines_valid, 200);
        assert_eq!(merged.branches_covered, 30);
        assert_eq!(merged.branches_valid, 40);
        assert!((merged.line_rate - 0.85).abs() < 0.001);
        assert!((merged.branch_rate - 0.75).abs() < 0.001);
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
}
