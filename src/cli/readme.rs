use anyhow::{Context, Result};

/// Deterministic completeness findings for a project README.
///
/// The checker reads the README text and reports machine-recognizable states:
/// open placeholders (`givn:todo` markers), remnants of the scaffolding,
/// a damaged development managed block, an unmanaged README (no givn markers
/// at all), or a missing README. It never judges prose quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadmeFindingKind {
    OpenPlaceholder,
    Remnant,
    DamagedBlock,
    Unmanaged,
    MissingReadme,
}

#[derive(Debug, Clone)]
pub struct ReadmeFinding {
    pub kind: ReadmeFindingKind,
    /// Nearest preceding `## ` heading, or the document root.
    pub message: String,
}

/// Exact placeholder sentences of the previous scaffold template. Substring
/// matches; `git clone <repo>` is deliberately absent — real quick-starts
/// contain it, and the gate is blocking.
const LEGACY_TEXTS: &[&str] = &[
    "One-paragraph description of what this project does and who it is for.",
    "List any required tools, languages, or runtimes.",
    "Brief example or `--help` output to show how to use the project.",
    "PR guidelines, commit conventions (Conventional Commits), code style,",
];

/// The opening marker of the development managed block.
pub const DEV_BLOCK_BEGIN: &str = "<!-- givn:begin:development -->";
/// The closing marker of the development managed block.
pub const DEV_BLOCK_END: &str = "<!-- givn:end:development -->";

/// The opening marker of the run managed block.
pub const RUN_BLOCK_BEGIN: &str = "<!-- givn:begin:run -->";
/// The closing marker of the run managed block.
pub const RUN_BLOCK_END: &str = "<!-- givn:end:run -->";

/// Check the project README and return every deterministic finding.
pub fn check_readme(project_root: &std::path::Path) -> Vec<ReadmeFinding> {
    let path = project_root.join("README.md");
    if !path.exists() {
        return vec![ReadmeFinding {
            kind: ReadmeFindingKind::MissingReadme,
            message: "README.md is missing — run `givn init` or `givn addons enable readme` to scaffold it."
                .to_string(),
        }];
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return vec![ReadmeFinding {
            kind: ReadmeFindingKind::MissingReadme,
            message: "README.md is unreadable — restore or scaffold it.".to_string(),
        }];
    };

    let mut findings: Vec<ReadmeFinding> = Vec::new();

    // A README is givn-managed when it carries at least one management marker:
    // any managed-block begin marker, any open placeholder, or a legacy
    // scaffold text.
    let has_begin_marker = content.contains("<!-- givn:begin:");
    let has_todo_marker = content.contains("<!-- givn:todo:");
    let legacy_hit_count = LEGACY_TEXTS
        .iter()
        .filter(|text| content.contains(**text))
        .count();
    if !has_begin_marker && !has_todo_marker && legacy_hit_count == 0 {
        findings.push(ReadmeFinding {
            kind: ReadmeFindingKind::Unmanaged,
            message:
                "README.md carries no givn markers — adapt the README or disable the readme addon."
                    .to_string(),
        });
    }

    let mut current_section = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("## ") {
            current_section = heading.to_string();
            continue;
        }
        if let Some(rest) = line
            .strip_prefix("<!-- givn:todo:")
            .and_then(|r| r.strip_suffix("-->"))
        {
            findings.push(ReadmeFinding {
                kind: ReadmeFindingKind::OpenPlaceholder,
                message: format!(
                    "open placeholder in {section} — {expectation}",
                    section = section_label(&current_section),
                    expectation = rest.trim()
                ),
            });
            continue;
        }
        for text in LEGACY_TEXTS {
            if trimmed.contains(text) {
                findings.push(ReadmeFinding {
                    kind: ReadmeFindingKind::OpenPlaceholder,
                    message: format!(
                        "legacy scaffold placeholder in {section} — run the README migration \
                         (rewrite with the current template, preserving real content)",
                        section = section_label(&current_section)
                    ),
                });
                break;
            }
        }
        if trimmed == "TBD" {
            findings.push(ReadmeFinding {
                kind: ReadmeFindingKind::Remnant,
                message:
                    "the license line says \"TBD\" — replace it with the project's actual license."
                        .to_string(),
            });
        }
        if trimmed.contains("{{PROJECT_NAME}}") {
            findings.push(ReadmeFinding {
                kind: ReadmeFindingKind::Remnant,
                message:
                    "the README still contains {{PROJECT_NAME}} — replace it with the project name."
                        .to_string(),
            });
        }
    }

    for (name, begin, end) in [
        ("development", DEV_BLOCK_BEGIN, DEV_BLOCK_END),
        ("run", RUN_BLOCK_BEGIN, RUN_BLOCK_END),
    ] {
        if let Err(error) = managed_block_range(&content, name, begin, end) {
            findings.push(ReadmeFinding {
                kind: ReadmeFindingKind::DamagedBlock,
                message: error.to_string(),
            });
        }
    }

    findings
}

fn managed_block_range(
    content: &str,
    name: &str,
    begin: &str,
    end: &str,
) -> Result<Option<std::ops::Range<usize>>> {
    let mut begins = content.match_indices(begin);
    let mut ends = content.match_indices(end);
    let first_begin = begins.next();
    let first_end = ends.next();
    match (first_begin, first_end) {
        (None, None) => Ok(None),
        (Some((start, _)), Some((finish, _)))
            if start < finish && begins.next().is_none() && ends.next().is_none() =>
        {
            Ok(Some(start..finish + end.len()))
        }
        _ => anyhow::bail!(
            "the {name} managed block markers are damaged — require exactly one ordered begin/end marker pair; restore the marker pair."
        ),
    }
}

fn readme_block_ranges(content: &str) -> Result<[Option<std::ops::Range<usize>>; 2]> {
    Ok([
        managed_block_range(content, "development", DEV_BLOCK_BEGIN, DEV_BLOCK_END)?,
        managed_block_range(content, "run", RUN_BLOCK_BEGIN, RUN_BLOCK_END)?,
    ])
}

/// The `## ` heading text, or the document root when the finding sits above
/// the first heading.
fn section_label(section: &str) -> &str {
    if section.is_empty() {
        "the README top"
    } else {
        section
    }
}

/// `givn readme check` — print the report; exit 1 while findings remain.
///
/// Returns the exit code; the caller maps non-zero to `std::process::exit`.
pub fn run_check() -> Result<i32> {
    let project_root = std::env::current_dir()?;
    let findings = check_readme(&project_root);
    if findings.is_empty() {
        println!("README check: OK");
        return Ok(0);
    }
    for finding in &findings {
        println!("[{}] {}", finding_label(finding.kind), finding.message);
    }
    println!("README check: {} finding(s)", findings.len());
    Ok(1)
}

pub(crate) fn finding_label(kind: ReadmeFindingKind) -> &'static str {
    match kind {
        ReadmeFindingKind::OpenPlaceholder => "OPEN ",
        ReadmeFindingKind::Remnant => "REMN ",
        ReadmeFindingKind::DamagedBlock => "DMG  ",
        ReadmeFindingKind::Unmanaged => "UNMG ",
        ReadmeFindingKind::MissingReadme => "MISS ",
    }
}

/// Scaffold an initial README.md from the embedded template.
///
/// Idempotent — does nothing if README.md already exists.
/// Returns the number of files written (0 or 1).
pub fn scaffold_readme() -> Result<usize> {
    let project_root = std::env::current_dir()?;
    let dest = project_root.join("README.md");
    if dest.exists() {
        return Ok(0);
    }

    let template = crate::config::cascade::resolve_template("readme", "README.md.tmpl")
        .context("failed to resolve readme template from embedded assets")?;

    let project_name = detect_project_name(&project_root);
    let development_block = render_development_block();
    let run_block = render_run_block();
    let rendered = template
        .replace("{{PROJECT_NAME}}", &project_name)
        .replace("{{DEVELOPMENT_BLOCK}}", &development_block)
        .replace("{{RUN_BLOCK}}", &run_block);

    std::fs::write(&dest, rendered)?;
    Ok(1)
}

/// Render the development managed block from the effective command file.
///
/// `givn init` writes `givn/commands.yaml` before addon scaffolding runs, so
/// init renders the real file. Standalone `givn addons enable readme` in a
/// project without a command file falls back to the embedded defaults.
pub(crate) fn render_development_block() -> String {
    const COVERAGE_SENTINEL: &str = "givn missing-coverage";
    let body = match crate::config::overlay::load_effective() {
        Ok(manifest) => {
            // Coverage commands are rendered when configured, independent of
            // the addon flag: during `givn init` the readme scaffold runs
            // before the coverage addon is enabled, but the command file
            // already lists the project's scripts.
            let measure = &manifest.coverage.measure_command;
            let merge = &manifest.coverage.merge_command;
            let coverage_configured = measure != COVERAGE_SENTINEL && merge != COVERAGE_SENTINEL;
            development_block_body(
                &manifest.verify.command,
                &manifest.verify.e2e_command,
                coverage_configured,
                measure,
                merge,
            )
        }
        Err(_) => development_block_body("givn missing-testrunner", "", false, "", ""),
    };
    format!("<!-- givn:begin:development -->\n{body}<!-- givn:end:development -->\n")
}

fn development_block_body(
    verify_command: &str,
    e2e_command: &str,
    coverage_enabled: bool,
    measure_command: &str,
    merge_command: &str,
) -> String {
    let mut out = String::from("## Development\n\n");
    out.push_str("Run the project-owned scripts configured in `givn/commands.yaml`:\n\n");
    out.push_str("```sh\n");
    out.push_str(verify_command);
    out.push('\n');
    if !e2e_command.is_empty() && e2e_command != verify_command {
        out.push_str(e2e_command);
        out.push('\n');
    }
    out.push_str("```\n\n");
    if coverage_enabled && !measure_command.is_empty() && !merge_command.is_empty() {
        out.push_str(&format!(
            "`{measure_command}` writes the non-E2E and E2E Cobertura reports.\n\
             `{merge_command}` combines them into the merged report linked by the\n\
             badges above.\n\n"
        ));
    }
    out.push_str("Add lint, build, and other project commands alongside these scripts.\n\n");
    out
}

/// Replace the development managed block content in an existing README.
///
pub(crate) fn refresh_development_block(project_root: &std::path::Path) -> Result<bool> {
    let path = project_root.join("README.md");
    if !path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(&path)?;
    let ranges = readme_block_ranges(&content)?;
    let Some(range) = ranges[0].clone() else {
        return Ok(false);
    };
    let new_block = render_development_block();
    let updated = format!(
        "{}{}{}",
        &content[..range.start],
        new_block.trim_end_matches('\n'),
        &content[range.end..]
    );
    std::fs::write(&path, updated)?;
    Ok(true)
}

/// Render the run managed block from the effective command file.
///
/// One render predicate governs the body: while the run declaration has
/// validation findings (unconfigured or half-configured), the block shows
/// exactly one open todo line and never a partial render; once the
/// declaration is complete, the real start instructions are rendered.
pub fn render_run_block() -> String {
    const RUN_TODO: &str =
        "<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->\n";
    match crate::config::overlay::load_effective() {
        Ok(manifest) => render_run_block_for(&manifest),
        Err(_) => format!("{RUN_BLOCK_BEGIN}\n{RUN_TODO}{RUN_BLOCK_END}\n"),
    }
}

/// Render the run managed block for a concrete manifest (the seam the
/// unit scenarios and `render_run_block` both use).
pub fn render_run_block_for(manifest: &crate::config::manifest::Manifest) -> String {
    const RUN_TODO: &str =
        "<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->\n";
    let body = {
        let findings = crate::cli::run_block::check_run(manifest);
        if findings.is_empty() {
            run_block_body(&manifest.run)
        } else {
            RUN_TODO.to_string()
        }
    };
    format!("{RUN_BLOCK_BEGIN}\n{body}{RUN_BLOCK_END}\n")
}

fn run_block_body(run: &crate::config::manifest::Run) -> String {
    use crate::config::manifest::ProductType;

    let mut out = String::from("## Run\n\n");
    match run.product_type {
        Some(ProductType::Server) => {
            out.push_str("Start the project:\n\n```sh\n");
            out.push_str(&run.command);
            out.push_str("\n```\n\n");
            let port_source = match &run.port_env {
                Some(name) => format!("${name}"),
                None => format!("port {}", run.port.unwrap_or_default()),
            };
            let ready_path = run.ready_path.as_deref().unwrap_or("/healthz");
            out.push_str(&format!(
                "The server listens on {port_source} and answers readiness at \
                 {ready_path}; givn waits up to {}s for readiness.\n\n",
                run.ready_timeout_s
            ));
        }
        Some(ProductType::Cli) => {
            out.push_str("Run the tool:\n\n```sh\n");
            out.push_str(&run.command);
            out.push_str("\n```\n\n");
        }
        Some(ProductType::Library) => {
            out.push_str(crate::cli::run::LIBRARY_LINE);
            out.push('\n');
            out.push('\n');
            if run.command != crate::config::manifest::Run::SENTINEL {
                out.push_str("The declared start command:\n\n```sh\n");
                out.push_str(&run.command);
                out.push_str("\n```\n\n");
            }
        }
        // Unconfigured or half-configured declarations never reach this
        // function: the render predicate routes findings to the todo line.
        None => {}
    }
    out
}

/// Replace the run managed block content in an existing README.
///
/// Runs only after the run gate has passed, so a refresh never
/// re-introduces a placeholder.
pub(crate) fn refresh_run_block(project_root: &std::path::Path) -> Result<bool> {
    let path = project_root.join("README.md");
    if !path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(&path)?;
    let ranges = readme_block_ranges(&content)?;
    let Some(range) = ranges[1].clone() else {
        return Ok(false);
    };
    let new_block = render_run_block();
    let updated = format!(
        "{}{}{}",
        &content[..range.start],
        new_block.trim_end_matches('\n'),
        &content[range.end..]
    );
    std::fs::write(&path, updated)?;
    Ok(true)
}

/// Detect the project name from package metadata or fall back to the
/// directory name.
fn detect_project_name(project_root: &std::path::Path) -> String {
    // 1. Cargo.toml → package.name
    if let Ok(content) = std::fs::read_to_string(project_root.join("Cargo.toml")) {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = trimmed
                .strip_prefix("name = \"")
                .and_then(|s| s.strip_suffix('"'))
            {
                return name.to_string();
            }
        }
    }
    // 2. package.json → "name"
    if let Ok(content) = std::fs::read_to_string(project_root.join("package.json"))
        && let Ok(json) = serde_json::from_str::<serde_json::Value>(&content)
        && let Some(name) = json.get("name").and_then(|v| v.as_str())
    {
        return name.to_string();
    }
    // 3. Fallback: directory name
    project_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed-project")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn damaged_blocks(begin: &str, end: &str) -> Vec<String> {
        vec![
            begin.to_owned(),
            end.to_owned(),
            format!("{end}\nprose\n{begin}"),
            format!("{begin}\nprose\n{begin}\n{end}"),
            format!("{begin}\n{end}\nprose\n{end}"),
            format!("{begin}\n{end}\nprose\n{begin}\n{end}"),
        ]
    }

    #[test]
    fn readme_rejects_ambiguous_markers_before_either_refresh() {
        for (begin, end, name) in [
            (DEV_BLOCK_BEGIN, DEV_BLOCK_END, "development"),
            (RUN_BLOCK_BEGIN, RUN_BLOCK_END, "run"),
        ] {
            for damaged in damaged_blocks(begin, end) {
                let root = tempfile::tempdir().unwrap();
                let content = format!("prefix\r\n{damaged}\nsuffix\r\n");
                let path = root.path().join("README.md");
                std::fs::write(&path, &content).unwrap();
                let findings = check_readme(root.path());
                assert!(
                    findings
                        .iter()
                        .any(|f| f.kind == ReadmeFindingKind::DamagedBlock
                            && f.message.contains(name)),
                    "missing damaged {name} finding for {damaged:?}"
                );
                assert!(refresh_development_block(root.path()).is_err());
                assert_eq!(std::fs::read(&path).unwrap(), content.as_bytes());
                assert!(refresh_run_block(root.path()).is_err());
                assert_eq!(std::fs::read(&path).unwrap(), content.as_bytes());
            }
        }
    }

    #[test]
    fn readme_refresh_preserves_surrounding_bytes() {
        for (begin, end, refresh) in [
            (
                DEV_BLOCK_BEGIN,
                DEV_BLOCK_END,
                refresh_development_block as fn(&std::path::Path) -> Result<bool>,
            ),
            (
                RUN_BLOCK_BEGIN,
                RUN_BLOCK_END,
                refresh_run_block as fn(&std::path::Path) -> Result<bool>,
            ),
        ] {
            let root = tempfile::tempdir().unwrap();
            let path = root.path().join("README.md");
            let prefix = "# Résumé\r\n\r\nHuman prose  \t\r\n";
            let suffix = "\r\n\r\nUser suffix\t  ";
            std::fs::write(&path, format!("{prefix}{begin}\nstale\n{end}{suffix}")).unwrap();
            assert!(refresh(root.path()).unwrap());
            let updated = std::fs::read_to_string(&path).unwrap();
            assert!(updated.starts_with(prefix));
            assert!(updated.ends_with(&format!("{end}{suffix}")), "{updated:?}");
            assert!(!updated.contains("stale"));
        }
    }

    #[test]
    fn readme_refresh_leaves_wholly_missing_blocks_alone() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("README.md");
        let content = "# Human README\r\nNo managed blocks.\r\n";
        std::fs::write(&path, content).unwrap();
        assert!(!refresh_development_block(root.path()).unwrap());
        assert!(!refresh_run_block(root.path()).unwrap());
        assert_eq!(std::fs::read(&path).unwrap(), content.as_bytes());
    }
}
