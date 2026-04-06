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
