//! # SCTT Core - Smooth Cubical Type Theory
//!
//! This crate provides the core implementation of Smooth Cubical Type Theory,
//! a revolutionary extension of cubical type theory that incorporates smooth/
//! differentiable structure into the type-theoretic foundation.
//!
//! ## Architecture
//!
//! - **Interval**: De Morgan algebra for cubical coordinates
//! - **Types**: Complete type system with all judgment forms
//! - **Terms**: Core term language with smooth operations
//! - **Kan**: Composition, transport, and glue operations
//! - **Smooth**: Tangent bundles, differential forms, cohomology
//! - **HIT**: Higher Inductive Types with all constructors
//! - **NbE**: Normalization by Evaluation for conversion checking
//! - **Memory**: Optimized data structures and garbage collection
//! - **Proof**: Proof-relevant computation and verification
//! - **Modality**: Shape, flat, sharp modalities for cohesion
//! - **Groupoid**: ∞-groupoid structure and higher operations

#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod interval;
pub mod types;
pub mod terms;
pub mod kan;
pub mod smooth;
pub mod hit;
pub mod nbe;
pub mod memory;
pub mod proof;
pub mod modality;
pub mod groupoid;
pub mod universe;
pub mod error;
pub mod context;
pub mod conversion;
pub mod normalization;
pub mod unification;

pub use error::{Error, Result};

/// Core SCTT version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude module for common imports
pub mod prelude {
    pub use crate::interval::{Interval, Face, DeMorgan};
    pub use crate::types::{Type, TypeJudgment, PathType, ExtensionType};
    pub use crate::terms::{Term, Variable, Lambda, Application};
    pub use crate::kan::{Composition, Transport, Glue};
    pub use crate::smooth::{TangentBundle, DifferentialForm, SmoothEquiv};
    pub use crate::nbe::{Value, Neutral, Environment, normalize};
    pub use crate::error::{Error, Result};
    pub use crate::context::Context;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_core_exports() {
        // Ensure all core types are accessible
        let _interval = Interval::zero();
        let _face = Face::True;
        let _context = Context::new();
    }
}