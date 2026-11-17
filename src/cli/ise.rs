//! `givn ise` — format the clean-room rebuild outcome.
//!
//! The library entry points return the process exit code; only the binary
//! wrapper in `main.rs` converts it with `std::process::exit`. All domain
//! decisions live in `crate::ise`; this module owns the `ise rebuild-all:`
//! and `ise rebuild-component:` prefixes, the report lines, and the exit
//! mapping.

use std::path::Path;

use crate::ise::{RebuildRequest, Report, abbreviate};

/// Run `ise rebuild-all` and return its process exit code.
pub fn rebuild_all(project_root: &Path, request: &RebuildRequest) -> i32 {
    rebuild(project_root, request, "ise rebuild-all")
}

/// Run `ise rebuild-component` and return its process exit code.
pub fn rebuild_component(project_root: &Path, request: &RebuildRequest) -> i32 {
    rebuild(project_root, request, "ise rebuild-component")
}

/// Run one rebuild command with the prefix of the verb that refused.
fn rebuild(project_root: &Path, request: &RebuildRequest, verb: &str) -> i32 {
    match crate::ise::rebuild(project_root, request) {
        Ok(report) => {
            print!("{}", render(&report));
            0
        }
        Err(error) => {
            eprintln!("{verb}: {error:#}");
            1
        }
    }
}

/// The report: the fixed lines with the conditional frozen-package line
/// between `E2E digest` and `Written path`, the dry run's `Would write:`
/// block, and the record when it went to stdout.
fn render(report: &Report) -> String {
    let mut text = format!(
        "Backend: {}\n\
         Revision: {}\n\
         Specification files: {}\n\
         Source code kept: {}\n\
         Source code removed: {}\n\
         Outside the spec copy: {}\n\
         Files beside removed paths: {}\n\
         Specification checksum: {}\n\
         End-to-end checksum: {}\n",
        report.backend,
        report.revision,
        report.counts.retained,
        report.counts.kept_source,
        report.counts.replaced,
        report.counts.outside_declaration,
        report.counts.not_replaced,
        abbreviate(&report.retained_digest),
        abbreviate(&report.e2e_digest),
    );
    if !report.excluded_roles.is_empty() {
        let noun = if report.frozen_files == 1 {
            "file"
        } else {
            "files"
        };
        text.push_str(&format!(
            "Frozen package: {}, not copied ({} {})\n",
            report.excluded_roles.join(", "),
            report.frozen_files,
            noun
        ));
    }
    text.push_str(&format!(
        "Written path: {}\n",
        report.written_path.display()
    ));
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
