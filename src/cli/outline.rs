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
    let explicit_scope = !paths.is_empty();
    let roots = if paths.is_empty() {
        vec![PathBuf::from("src")]
    } else {
        paths
            .into_iter()
            .map(|path| validate_scope(&path))
            .collect::<Result<Vec<_>>>()?
    };
    if explicit_scope {
        for root in &roots {
            if !project_root.join(root).exists() {
                bail!("--path does not exist: {}", root.display());
            }
        }
    }

    let request = AnalysisRequest {
        project_root: project_root.clone(),
        scope: SourceScope {
            languages: vec![Language::Rust, Language::Python],
            roots: roots.clone(),
            include_test_sources: false,
        },
    };
    let analyzer = outline::outline_analyzer();
    let capabilities = analyzer.capabilities();
    let analysis = outline::analyze(&request, analyzer.as_ref())?;
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
