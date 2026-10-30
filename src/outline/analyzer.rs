//! The neutral analyzer port.

use super::model::{Analysis, AnalysisRequest, AnalyzerCapabilities, Language};
use anyhow::Result;

pub trait SourceAnalyzer: Send + Sync {
    fn capabilities(&self) -> AnalyzerCapabilities;
    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis>;
}

/// The Rust-only analyzer used by the composite analyzer and integration tests.
pub fn rust_analyzer() -> Box<dyn SourceAnalyzer> {
    Box::new(super::syn_adapter::SynAnalyzer)
}

/// The addon's analyzer: Rust first, Python only when `.py` files exist.
pub fn outline_analyzer() -> Box<dyn SourceAnalyzer> {
    Box::new(CompositeAnalyzer)
}

struct CompositeAnalyzer;

impl SourceAnalyzer for CompositeAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        let rust = rust_analyzer().capabilities();
        let python = super::python_adapter::PythonAnalyzer.capabilities();
        let java = super::tree_sitter_adapter::JavaAnalyzer.capabilities();
        let typescript = super::tree_sitter_adapter::TypeScriptAnalyzer.capabilities();
        AnalyzerCapabilities {
            languages: vec![
                Language::Rust,
                Language::Python,
                Language::Java,
                Language::TypeScript,
            ],
            supported_constructs: unique(
                unique(rust.supported_constructs, python.supported_constructs),
                unique(java.supported_constructs, typescript.supported_constructs),
            ),
            limitations: unique(
                unique(rust.limitations, python.limitations),
                unique(java.limitations, typescript.limitations),
            ),
            resolution_labels: unique(
                unique(rust.resolution_labels, python.resolution_labels),
                unique(java.resolution_labels, typescript.resolution_labels),
            ),
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        let rust = rust_analyzer().analyze(request)?;
        let python = super::python_adapter::PythonAnalyzer.analyze(request)?;
        let java = super::tree_sitter_adapter::JavaAnalyzer.analyze(request)?;
        let typescript = super::tree_sitter_adapter::TypeScriptAnalyzer.analyze(request)?;
        let mut merged = Analysis::default();
        merged.files.extend(rust.files);
        merged.files.extend(python.files);
        merged.files.extend(java.files);
        merged.files.extend(typescript.files);
        merged.units.extend(rust.units);
        merged.units.extend(python.units);
        merged.units.extend(java.units);
        merged.units.extend(typescript.units);
        merged.diagnostics.extend(rust.diagnostics);
        merged.diagnostics.extend(python.diagnostics);
        merged.diagnostics.extend(java.diagnostics);
        merged.diagnostics.extend(typescript.diagnostics);
        merged
            .files
            .sort_by(|left, right| left.path.cmp(&right.path));
        merged.units.sort_by(|left, right| left.id.cmp(&right.id));
        merged.diagnostics.sort_by(|left, right| {
            (left.path.clone(), left.line).cmp(&(right.path.clone(), right.line))
        });
        Ok(merged)
    }
}

fn unique<T: PartialEq>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut values = left;
    for value in right {
        if !values.contains(&value) {
            values.push(value);
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_disclose_structural_analysis_limits() {
        let capabilities = outline_analyzer().capabilities();
        let limitations = capabilities
            .limitations
            .iter()
            .map(|limitation| limitation.label())
            .collect::<Vec<_>>()
            .join("; ")
            .to_lowercase();
        assert!(
            limitations.contains("structural call order is not a complete runtime trace"),
            "runtime-trace limitation missing: {limitations}"
        );
        assert!(
            limitations.contains("uncertain or omitted call edges limit conclusions"),
            "uncertain-edge limitation missing: {limitations}"
        );
    }
}
