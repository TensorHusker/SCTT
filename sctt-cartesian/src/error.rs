//! Error types for SCTT type checking and evaluation.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScttError {
    #[error("type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("unbound variable: index {0}")]
    UnboundVar(usize),

    #[error("unbound dimension variable: index {0}")]
    UnboundDimVar(usize),

    #[error("universe overflow: level {0} exceeds maximum")]
    UniverseOverflow(u8),

    #[error("path endpoint mismatch at {endpoint}: expected {expected}, got {actual}")]
    PathEndpointMismatch {
        endpoint: &'static str,
        expected: String,
        actual: String,
    },

    #[error("cofibration unsatisfiable")]
    CofUnsatisfiable,

    #[error("smoothness violation: {0}")]
    SmoothnessViolation(String),

    #[error("not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, ScttError>;
