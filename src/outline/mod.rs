//! Opt-in, advisory code-structure analysis.

pub mod analyzer;
pub mod classify;
pub mod gate;
pub mod metrics;
pub mod model;
pub mod report;
pub mod scan;

mod python_adapter;
mod resolve;
mod syn_adapter;

use anyhow::Result;

pub use analyzer::{SourceAnalyzer, outline_analyzer};
pub use classify::assess;
pub use model::{Analysis, AnalysisRequest, SourceScope};

/// Run the neutral analysis pipeline through the supplied analyzer.
pub fn analyze(request: &AnalysisRequest, analyzer: &dyn SourceAnalyzer) -> Result<Analysis> {
    let mut analysis = analyzer.analyze(request)?;
    resolve::resolve_calls(&mut analysis);
    Ok(analysis)
}
