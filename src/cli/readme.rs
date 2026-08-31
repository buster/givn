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
    pub section: String,
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

/// Check the project README and return every deterministic finding.
pub fn check_readme(project_root: &std::path::Path) -> Vec<ReadmeFinding> {
    let path = project_root.join("README.md");
    if !path.exists() {
        return vec![ReadmeFinding {
            kind: ReadmeFindingKind::MissingReadme,
            section: String::new(),
            message: "README.md is missing — run `givn init` or `givn addons enable readme` to scaffold it."
                .to_string(),
        }];
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return vec![ReadmeFinding {
            kind: ReadmeFindingKind::MissingReadme,
            section: String::new(),
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
            section: String::new(),
            message: "README.md carries no givn markers — adapt the README or disable the readme addon."
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
                section: current_section.clone(),
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
                    section: current_section.clone(),
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
                section: current_section.clone(),
                message: "the license line says \"TBD\" — replace it with the project's actual license."
                    .to_string(),
            });
        }
        if trimmed.contains("{{PROJECT_NAME}}") {
            findings.push(ReadmeFinding {
                kind: ReadmeFindingKind::Remnant,
                section: current_section.clone(),
                message: "the README still contains {{PROJECT_NAME}} — replace it with the project name."
                    .to_string(),
            });
        }
    }

    // Development managed block damage: exactly one of the two markers present.
    let begins = content.matches(DEV_BLOCK_BEGIN).count();
    let ends = content.matches(DEV_BLOCK_END).count();
    if (begins == 0) != (ends == 0) {
        findings.push(ReadmeFinding {
            kind: ReadmeFindingKind::DamagedBlock,
            section: "Development".to_string(),
            message: "the development managed block is damaged — exactly one of the \
                      begin/end markers is present; restore the marker pair."
                .to_string(),
        });
    }

    findings
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

fn finding_label(kind: ReadmeFindingKind) -> &'static str {
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
    let rendered = template.replace("{{PROJECT_NAME}}", &project_name);

    std::fs::write(&dest, rendered)?;
    Ok(1)
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
