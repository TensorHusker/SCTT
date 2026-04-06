//! # Cofibrations (Face Formulas)
//!
//! A cofibration is a formula built from dimension equalities,
//! conjunction, and disjunction. It describes which faces of a
//! cube are "specified" (have known values).
//!
//! In Cartesian cubical TT, cofibrations form a distributive lattice:
//! - Atoms: `r = s` where r, s are dimension expressions
//! - Meet: phi /\ psi
//! - Join: phi \/ psi
//! - Top (always true, whole cube)
//! - Bot (never true, empty)

use crate::dim::{Dim, DimIndex};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cof {
    Eq(Dim, Dim),
    And(Box<Cof>, Box<Cof>),
    Or(Box<Cof>, Box<Cof>),
    Top,
    Bot,
}

impl Cof {
    pub fn eq_dim(r: Dim, s: Dim) -> Self {
        match (&r, &s) {
            (Dim::Zero, Dim::Zero) | (Dim::One, Dim::One) => Cof::Top,
            (Dim::Zero, Dim::One) | (Dim::One, Dim::Zero) => Cof::Bot,
            _ => Cof::Eq(r, s),
        }
    }

    pub fn and(phi: Cof, psi: Cof) -> Self {
        match (&phi, &psi) {
            (Cof::Top, _) => psi,
            (_, Cof::Top) => phi,
            (Cof::Bot, _) | (_, Cof::Bot) => Cof::Bot,
            _ => Cof::And(Box::new(phi), Box::new(psi)),
        }
    }

    pub fn or(phi: Cof, psi: Cof) -> Self {
        match (&phi, &psi) {
            (Cof::Top, _) | (_, Cof::Top) => Cof::Top,
            (Cof::Bot, _) => psi,
            (_, Cof::Bot) => phi,
            _ => Cof::Or(Box::new(phi), Box::new(psi)),
        }
    }

    pub fn is_true(&self) -> bool { matches!(self, Cof::Top) }
    pub fn is_false(&self) -> bool { matches!(self, Cof::Bot) }

    pub fn subst_dim(&self, target: DimIndex, replacement: &Dim) -> Cof {
        match self {
            Cof::Eq(r, s) => {
                let r2 = r.subst_dim(target, replacement);
                let s2 = s.subst_dim(target, replacement);
                Cof::eq_dim(r2, s2)
            }
            Cof::And(phi, psi) => {
                Cof::and(
                    phi.subst_dim(target, replacement),
                    psi.subst_dim(target, replacement),
                )
            }
            Cof::Or(phi, psi) => {
                Cof::or(
                    phi.subst_dim(target, replacement),
                    psi.subst_dim(target, replacement),
                )
            }
            Cof::Top => Cof::Top,
            Cof::Bot => Cof::Bot,
        }
    }

    pub fn shift_dim(&self, cutoff: usize, amount: isize) -> Cof {
        match self {
            Cof::Eq(r, s) => Cof::Eq(
                r.shift_dim(cutoff, amount),
                s.shift_dim(cutoff, amount),
            ),
            Cof::And(phi, psi) => Cof::And(
                Box::new(phi.shift_dim(cutoff, amount)),
                Box::new(psi.shift_dim(cutoff, amount)),
            ),
            Cof::Or(phi, psi) => Cof::Or(
                Box::new(phi.shift_dim(cutoff, amount)),
                Box::new(psi.shift_dim(cutoff, amount)),
            ),
            Cof::Top => Cof::Top,
            Cof::Bot => Cof::Bot,
        }
    }
}

impl fmt::Display for Cof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cof::Eq(r, s) => write!(f, "({} = {})", r, s),
            Cof::And(phi, psi) => write!(f, "({} /\\ {})", phi, psi),
            Cof::Or(phi, psi) => write!(f, "({} \\/ {})", phi, psi),
            Cof::Top => write!(f, "T"),
            Cof::Bot => write!(f, "F"),
        }
    }
}
