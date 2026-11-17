//! `givn ise` — format the clean-room rebuild outcome.
//!
//! The library entry points return the process exit code; only the binary
//! wrapper in `main.rs` converts it with `std::process::exit`. All domain
//! decisions live in `crate::ise`; this module owns the `ise rebuild-all:`
//! and `ise rebuild-component:` prefixes, the report lines, and the exit
//! mapping.

use std::path::Path;

use crate::ise::{
    CompareReport, CompareRequest, Difference, DifferenceKind, RebuildRequest, Report, abbreviate,
};

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

/// Run `ise compare` and return its process exit code.
pub fn compare(project_root: &Path, request: &CompareRequest) -> i32 {
    match crate::ise::compare(project_root, request) {
        Ok(report) => {
            print!("{}", render_comparison(&report));
            if report.is_clean() { 0 } else { 1 }
        }
        Err(error) => {
            eprintln!("ise compare: {error:#}");
            1
        }
    }
}

/// The two subject lines and their difference lines. A subject with no
/// difference prints `match` alone; the other prints its non-zero counts in
/// the fixed order changed, missing, added.
fn render_comparison(report: &CompareReport) -> String {
    let mut text = String::new();
    for (subject, differences) in [
        ("Specification files", &report.specification),
        ("End-to-end tests", &report.end_to_end),
    ] {
        text.push_str(&format!("{subject}: {}\n", comparison_counts(differences)));
        for difference in differences {
            let marker = if difference.kind == DifferenceKind::Added {
                '+'
            } else {
                '-'
            };
            text.push_str(&format!("{marker} {}\n", difference.path));
            if let Some(note) = difference.note {
                text.push_str(note);
                text.push('\n');
            }
        }
    }
    text
}

/// The subject's count summary, or `match` when it holds no difference.
fn comparison_counts(differences: &[Difference]) -> String {
    let counts: Vec<String> = [
        (DifferenceKind::Changed, "changed"),
        (DifferenceKind::Missing, "missing"),
        (DifferenceKind::Added, "added"),
    ]
    .into_iter()
    .filter_map(|(kind, label)| {
        let count = differences
            .iter()
            .filter(|difference| difference.kind == kind)
            .count();
        (count > 0).then(|| format!("{count} {label}"))
    })
    .collect();
    if counts.is_empty() {
        "match".to_string()
    } else {
        counts.join(", ")
    }
}

/// The report: eleven fixed lines, and `Manifest` only when `--manifest` was
/// given. A dry run replaces the two write lines with
/// `Copy directory (would write):` and `Manifest (not written):`.
fn render(report: &Report) -> String {
    let noun = if report.end_to_end_files == 1 {
        "file"
    } else {
        "files"
    };
    let mut text = format!(
        "Backend: {}\n\
         Revision: {}\n\
         Specification files: {}\n\
         Source code kept: {}\n\
         Source code removed: {}\n\
         Outside the spec copy: {}\n\
         Files beside removed paths: {}\n\
         Specification checksum: {}\n\
         End-to-end checksum: {}\n\
         End-to-end tests: not copied ({} {})\n",
        report.backend,
        report.revision,
        report.counts.retained,
        report.counts.kept_source,
        report.counts.replaced,
        report.counts.outside_declaration,
        report.counts.not_replaced,
        abbreviate(&report.specification_checksum),
        abbreviate(&report.end_to_end_checksum),
        report.end_to_end_files,
        noun,
    );
    if report.dry_run {
        text.push_str(&format!(
            "Copy directory (would write): {}\n",
            report.copy_directory.display()
        ));
        if let Some(path) = &report.manifest {
            text.push_str(&format!("Manifest (not written): {}\n", path.display()));
        }
        return text;
    }
    text.push_str(&format!(
        "Copy directory: {}\n",
        report.copy_directory.display()
    ));
    if let Some(path) = &report.manifest {
        text.push_str(&format!("Manifest: {}\n", path.display()));
    }
    text
}
