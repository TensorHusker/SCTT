//! # Smooth Cubical Type Theory (SCTT) -- Cartesian variant
//!
//! A standalone implementation of Cartesian cubical type theory (ABCFHL-style)
//! extended with smooth/differentiable structure.

pub mod dim;
pub mod cof;
pub mod syntax;
pub mod value;
pub mod env;
pub mod evaluate;
pub mod quote;
pub mod conv;
pub mod check;
pub mod kan;
pub mod glue;
pub mod smooth;
pub mod error;
pub mod arc;
pub mod arc_puzzle;

// ─── Convenient Re-exports ──────────────────────────────────────────────────

pub use syntax::{Term, Level, Index, Smoothness};
pub use dim::{Dim, DimIndex};
pub use cof::Cof;
pub use value::{Value, Neutral, Env, Closure, DimClosure, DimVal, TermLevel};
pub use check::TypeChecker;
pub use evaluate::evaluate;
pub use quote::{quote, normalize};
pub use error::{ScttError, Result};
