//! `givn readability report`: the opt-in, advisory code-structure report.
//!
//! The command is always registered and gated at runtime so the disabled path
//! can name the reason. Findings never change the exit code.

use crate::readability::{self, AnalysisRequest, SourceScope, model::Language};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

/// Run the readability report. The `readability` addon must be enabled.
pub fn run_report(json: bool, out: Option<PathBuf>, paths: Vec<PathBuf>) -> Result<()> {
    let manifest = crate::config::overlay::load_effective()?;
    if !manifest.addons.readability {
        bail!("readability experiment is disabled; enable it with givn addons enable readability");
    }

    let project_root = std::env::current_dir().context("resolve the project root")?;
    let roots = if paths.is_empty() {
        vec![PathBuf::from("src")]
    } else {
        paths
            .into_iter()
            .map(|path| validate_scope(&path))
            .collect::<Result<Vec<_>>>()?
    };

    let request = AnalysisRequest {
        project_root: project_root.clone(),
        scope: SourceScope {
            language: Language::Rust,
            roots: roots.clone(),
            include_test_sources: false,
        },
    };
    let analyzer = readability::rust_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = readability::analyze(&request, analyzer.as_ref())?;
    let assessment = readability::assess(&analysis, capabilities);

    let document = if json {
        readability::report::render_json(&assessment, &request.scope)
    } else {
        readability::report::render_human(&assessment)
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
