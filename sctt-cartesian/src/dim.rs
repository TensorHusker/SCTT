//! # Interval Type and Dimension Expressions
//!
//! The abstract interval I has two endpoints (0 and 1) and dimension
//! variables. In Cartesian cubical TT, there are NO connections
//! (min/max) or reversal (1-i) -- only substitution of endpoints.
//!
//! Dimension variables use their own de Bruijn namespace, separate
//! from term variables.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DimLevel(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dim {
    Var(DimIndex),
    Zero,
    One,
}

impl Dim {
    pub fn is_const(&self) -> bool {
        matches!(self, Dim::Zero | Dim::One)
    }

    pub fn subst_dim(&self, target: DimIndex, replacement: &Dim) -> Dim {
        match self {
            Dim::Var(i) if *i == target => replacement.clone(),
            other => other.clone(),
        }
    }

    pub fn shift_dim(&self, cutoff: usize, amount: isize) -> Dim {
        match self {
            Dim::Var(DimIndex(i)) if *i >= cutoff => {
                let new = (*i as isize + amount) as usize;
                Dim::Var(DimIndex(new))
            }
            other => other.clone(),
        }
    }
}

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dim::Var(DimIndex(i)) => write!(f, "i{}", i),
            Dim::Zero => write!(f, "0"),
            Dim::One => write!(f, "1"),
        }
    }
}
