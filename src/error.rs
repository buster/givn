use thiserror::Error;

/// Typed errors for the givn library layer.
#[derive(Debug, Error)]
pub enum GivnError {
    #[error("change '{0}' not found under givn/changes/")]
    ChangeNotFound(String),

    #[error("artifact '{0}' not found in the effective manifest")]
    ArtifactNotFound(String),

    #[error("manifest is invalid: {0}")]
    InvalidManifest(String),

    #[error("dependency graph error: {0}")]
    GraphError(String),

    #[error("merge error: {0}")]
    MergeError(String),

    #[error("hook failed: {0}")]
    HookFailed(String),

    #[error("archive gate failed: {0}")]
    ArchiveGate(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
