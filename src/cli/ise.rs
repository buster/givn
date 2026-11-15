//! `givn ise` — format the clean-room preparation and verification outcome.
//!
//! The library entry points return the process exit code; only the binary
//! wrapper in `main.rs` converts it with `std::process::exit`. All domain
//! decisions live in `crate::ise`; this module owns the `ise replace:` and
//! `ise verify:` prefixes, the report lines, and the exit mapping.

use std::path::Path;

use crate::ise::{ReplaceRequest, Report, abbreviate};

/// Run `ise replace` and return its process exit code.
pub fn replace(project_root: &Path, request: &ReplaceRequest) -> i32 {
    match crate::ise::replace(project_root, request) {
        Ok(report) => {
            print!("{}", render(&report));
            0
        }
        Err(error) => {
            eprintln!("ise replace: {error:#}");
            1
        }
    }
}

/// Run `ise verify` and return its process exit code.
pub fn verify(project_root: &Path, trial: &Path, at: &Path) -> i32 {
    match crate::ise::verify(project_root, trial, at) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("ise verify: {error:#}");
            1
        }
    }
}

/// The report: ten fixed lines, the receipt path only when one was given, the
/// dry run's `Would write:` block, and the record when it went to stdout.
fn render(report: &Report) -> String {
    let mut text = format!(
        "Backend: {}\n\
         Revision: {}\n\
         Retained: {}\n\
         Kept source: {}\n\
         Replaced: {}\n\
         Outside the declaration: {}\n\
         Not replaced: {}\n\
         Retained digest: {}\n\
         E2E digest: {}\n\
         Written path: {}\n",
        report.backend,
        report.revision,
        report.counts.retained,
        report.counts.kept_source,
        report.counts.replaced,
        report.counts.outside_declaration,
        report.counts.not_replaced,
        abbreviate(&report.retained_digest),
        abbreviate(&report.e2e_digest),
        report.written_path.display(),
    );
    if let Some(receipt) = &report.receipt {
        text.push_str(&format!("Receipt path: {}\n", receipt.display()));
    }
    if let Some(paths) = &report.would_write {
        text.push_str("Would write:\n");
        for path in paths {
            text.push_str(&format!("  {path}\n"));
        }
    }
    if let Some(record) = &report.record {
        text.push_str(record);
        if !record.ends_with('\n') {
            text.push('\n');
        }
    }
    text
}
