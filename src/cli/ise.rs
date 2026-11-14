//! `givn ise` — format the clean-room preparation and verification outcome.
//!
//! The library entry points return the process exit code; only the binary
//! wrapper in `main.rs` converts it with `std::process::exit`. All domain
//! decisions live in `crate::ise`; this module owns the `ise replace:` and
//! `ise verify:` prefixes and the exit mapping.

use std::path::Path;

use crate::ise::{ReplaceRequest, Report};

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

/// The fixed report lines: backend, revision, and the five placement counts.
fn render(report: &Report) -> String {
    format!(
        "Backend: {}\n\
         Revision: {}\n\
         Retained: {}\n\
         Kept source: {}\n\
         Replaced: {}\n\
         Outside the declaration: {}\n\
         Not replaced: {}\n",
        report.backend,
        report.revision,
        report.counts.retained,
        report.counts.kept_source,
        report.counts.replaced,
        report.counts.outside_declaration,
        report.counts.not_replaced,
    )
}
