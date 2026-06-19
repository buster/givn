use thiserror::Error;

/// Typed errors for the givn library layer.
///
/// Used for structured error propagation in the core engine modules.
/// Application-layer commands use `anyhow` for context wrapping.
#[derive(Debug, Error)]
pub enum GivnError {
    #[error("merge error: {0}")]
    MergeError(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
