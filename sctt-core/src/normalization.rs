//! # Normalization for SCTT
//!
//! Stub implementation for normalization module.

use crate::error::{Error, Result};

/// Normalize a term
pub fn normalize(_term: &crate::types::Term) -> Result<crate::types::Term> {
    Err(Error::NotImplemented("Normalization".into()))
}