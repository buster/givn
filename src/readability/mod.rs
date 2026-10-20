//! Opt-in, advisory code-structure analysis.

pub mod analyzer;
pub mod classify;
pub mod gate;
pub mod metrics;
pub mod model;
pub mod report;
pub mod scan;

mod syn_adapter;

use anyhow::Result;

pub use analyzer::{SourceAnalyzer, rust_analyzer};
pub use classify::assess;
pub use model::{Analysis, AnalysisRequest, SourceScope};

/// Run the neutral analysis pipeline through the supplied analyzer.
pub fn analyze(request: &AnalysisRequest, analyzer: &dyn SourceAnalyzer) -> Result<Analysis> {
    analyzer.analyze(request)
}
