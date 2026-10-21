//! The neutral analyzer port.

use super::model::{Analysis, AnalysisRequest, AnalyzerCapabilities};
use anyhow::Result;

pub trait SourceAnalyzer: Send + Sync {
    fn capabilities(&self) -> AnalyzerCapabilities;
    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis>;
}

/// The only public analyzer factory.
pub fn rust_analyzer() -> Box<dyn SourceAnalyzer> {
    Box::new(super::syn_adapter::SynAnalyzer)
}
