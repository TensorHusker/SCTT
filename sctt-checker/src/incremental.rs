//! # Incremental Type Checking
//!
//! Stub implementation for incremental checking.

use anyhow::Result;

/// Incremental type checker state
pub struct IncrementalChecker;

impl IncrementalChecker {
    /// Create new incremental checker
    pub fn new() -> Self {
        Self
    }
    
    /// Update with file change
    pub fn update_file(&mut self, _path: &str, _content: &str) -> Result<()> {
        Ok(())
    }
}