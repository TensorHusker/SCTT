//! # Elaboration from Surface Syntax to Core Terms
//!
//! Stub implementation for elaboration module.

use sctt_core::prelude::*;
use anyhow::Result;

/// Elaborate surface term to core term
pub fn elaborate_term(_surface: &str) -> Result<Term> {
    Err(Error::NotImplemented("Elaboration".into()).into())
}

/// Elaborate surface type to core type
pub fn elaborate_type(_surface: &str) -> Result<Type> {
    Err(Error::NotImplemented("Type elaboration".into()).into())
}