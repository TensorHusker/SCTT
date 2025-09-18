//! # Error Types for SCTT
//!
//! Comprehensive error handling for all SCTT operations.

use thiserror::Error;
use std::fmt::Display;

/// Result type alias for SCTT operations
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive error types for SCTT
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    /// Type checking errors
    #[error("Type error: {message}")]
    TypeError { message: String },
    
    /// Unbound variable reference
    #[error("Unbound variable: {0}")]
    UnboundVariable(String),
    
    /// Universe level inconsistency
    #[error("Universe level error: {0}")]
    UniverseLevel(String),
    
    /// Kan operation failure
    #[error("Kan operation failed: {operation} - {reason}")]
    KanError { operation: String, reason: String },
    
    /// Smooth structure violation
    #[error("Smooth structure error: {0}")]
    SmoothError(String),
    
    /// Higher inductive type error
    #[error("HIT error: {0}")]
    HITError(String),
    
    /// Normalization failure
    #[error("Normalization error: {0}")]
    NormalizationError(String),
    
    /// Unification failure
    #[error("Unification failed: {0}")]
    UnificationError(String),
    
    /// Memory/resource errors
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    /// Proof verification error
    #[error("Proof error: {0}")]
    ProofError(String),
    
    /// Modality error
    #[error("Modality error: {0}")]
    ModalityError(String),
    
    /// Groupoid operation error
    #[error("Groupoid error: {0}")]
    GroupoidError(String),
    
    /// Parse/syntax error
    #[error("Parse error at {location}: {message}")]
    ParseError { location: String, message: String },
    
    /// Internal consistency error
    #[error("Internal error: {0}")]
    InternalError(String),
    
    /// Feature not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl Error {
    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        Error::TypeError {
            message: message.into(),
        }
    }
    
    /// Create a Kan operation error
    pub fn kan_error(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::KanError {
            operation: operation.into(),
            reason: reason.into(),
        }
    }
    
    /// Create a parse error with location
    pub fn parse_error(location: impl Into<String>, message: impl Into<String>) -> Self {
        Error::ParseError {
            location: location.into(),
            message: message.into(),
        }
    }
    
    /// Create a smooth error
    pub fn smooth_error(message: impl Into<String>) -> Self {
        Error::SmoothError(message.into())
    }
    
    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Error::InternalError(message.into())
    }
}

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Source location information
    pub location: Option<SourceLocation>,
    /// Stack trace of operations
    pub stack: Vec<String>,
    /// Additional context information
    pub context: std::collections::HashMap<String, String>,
}

/// Source location information
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    /// File path
    pub file: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl ErrorContext {
    /// Create new error context
    pub fn new() -> Self {
        Self {
            location: None,
            stack: Vec::new(),
            context: std::collections::HashMap::new(),
        }
    }
    
    /// Set source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    /// Add operation to stack
    pub fn push_operation(mut self, operation: impl Into<String>) -> Self {
        self.stack.push(operation.into());
        self
    }
    
    /// Add context information
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }
}