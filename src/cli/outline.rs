//! `givn outline report`: the opt-in, advisory code-structure report.
//!
//! The command is always registered and gated at runtime so the disabled path
//! can name the reason. Hints never change the exit code.

use crate::outline::{self, AnalysisRequest, SourceScope, model::Language};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

/// Run the outline report. The `outline` addon must be enabled.
pub fn run_report(json: bool, out: Option<PathBuf>, paths: Vec<PathBuf>) -> Result<()> {
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.addons.outline {
        bail!("outline experiment is disabled; enable it with givn addons enable outline");
    }

    let project_root = std::env::current_dir().context("resolve the project root")?;
    let roots = if paths.is_empty() {
        // `src/` is the conventional root for the supported languages. Other
        // layouts must name their production paths explicitly.
        if project_root.join("src").is_dir() {
            vec![PathBuf::from("src")]
        } else {
            bail!(
                "outline report requires at least one --path; pass the production sources, \
                 for example: givn outline report --path src --path app.py"
            );
        }
    } else {
        paths
            .into_iter()
            .map(|path| validate_scope(&path))
            .collect::<Result<Vec<_>>>()?
    };
    for root in &roots {
        if !project_root.join(root).exists() {
            bail!("--path does not exist: {}", root.display());
        }
    }

    let request = AnalysisRequest {
        project_root: project_root.clone(),
        scope: SourceScope {
            languages: vec![
                Language::Rust,
                Language::Python,
                Language::Java,
                Language::TypeScript,
            ],
            roots: roots.clone(),
            include_test_sources: false,
        },
    };
    let analyzer = outline::outline_analyzer();
    let capabilities = analyzer.capabilities();
    let supported = capabilities
        .languages
        .iter()
        .map(|language| language.label())
        .collect::<Vec<_>>()
        .join(", ");
    let analysis = outline::analyze(&request, analyzer.as_ref())?;
    if analysis.files.is_empty() {
        let scope = roots
            .iter()
            .map(|root| root.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let unsupported = outline::scan::unsupported_production_sources(&project_root, &roots)?;
        if unsupported.is_empty() {
            bail!(
                "outline report: no supported production sources found in: {scope}; \
                 supported source languages: {supported}"
            );
        }
        bail!(
            "outline report: no supported production sources found in: {scope}; \
             supported source languages: {supported}; the scope contains production sources \
             in unsupported languages: {}",
            unsupported
                .iter()
                .map(|path| path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let assessment = outline::assess(&analysis, capabilities);

    let document = if json {
        outline::report::render_json(&assessment, &request.scope)
    } else {
        outline::report::render_human(&assessment)
    };

    match out {
        Some(path) => {
            std::fs::write(&path, document)
                .with_context(|| format!("write the report to {}", path.display()))?;
        }
        None => print!("{document}"),
    }
    Ok(())
}

fn validate_scope(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        bail!("--path must be project-root-relative: {}", path.display());
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        bail!("--path must not contain '..': {}", path.display());
    }
    Ok(path.to_path_buf())
}
