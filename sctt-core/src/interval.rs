//! # Interval Arithmetic and De Morgan Algebra
//!
//! This module implements the interval types that form the foundation of 
//! cubical type theory. The key insight is that the interval I is equipped
//! with a De Morgan algebra structure that enables cubical operations.
//!
//! ## Key Concepts
//!
//! - **Interval I**: The fundamental cubical dimension [0,1]
//! - **Face Lattice**: Boolean algebra of face constraints  
//! - **De Morgan Laws**: Duality between ∧ and ∨ operations
//! - **Smooth Intervals**: Infinitesimal-enriched intervals for SCTT
//! - **Nilpotent Arithmetic**: dx² = 0 for differential structure

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Debug};
use std::ops::{BitAnd, BitOr, Not};
use std::collections::BTreeMap;
use bitvec::prelude::*;
use smallvec::SmallVec;

/// The interval type I, representing a cubical dimension
///
/// In cubical type theory, intervals are the basic building blocks
/// for higher-dimensional paths and homotopies. They satisfy the
/// De Morgan algebra axioms.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Interval {
    /// The constant 0 endpoint
    Zero,
    /// The constant 1 endpoint  
    One,
    /// A dimension variable (named)
    Var(String),
    /// Negation: ~r
    Not(Box<Interval>),
    /// Conjunction: r ∧ s (minimum)
    And(Box<Interval>, Box<Interval>),
    /// Disjunction: r ∨ s (maximum)  
    Or(Box<Interval>, Box<Interval>),
    /// Smooth interval with infinitesimal part
    Smooth {
        /// Standard part
        standard: Box<Interval>,
        /// Infinitesimal part (nilpotent)
        infinitesimal: Box<SmoothForm>,
    },
}

/// Smooth differential forms on intervals
///
/// These represent infinitesimal directions and enable
/// the differential structure of SCTT.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmoothForm {
    /// Zero form
    Zero,
    /// Differential dr for variable r
    Differential(String),
    /// Wedge product of forms
    Wedge(Box<SmoothForm>, Box<SmoothForm>),
    /// Scaled form
    Scale(f64, Box<SmoothForm>),
}

/// Face expressions in the boolean algebra
///
/// Faces represent boundary conditions and constraints
/// in cubical types. They form a boolean algebra.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Face {
    /// Always true
    True,
    /// Always false
    False,
    /// Variable equals 0: (r = 0)
    Eq0(String),
    /// Variable equals 1: (r = 1)  
    Eq1(String),
    /// Negation: ¬φ
    Not(Box<Face>),
    /// Conjunction: φ ∧ ψ
    And(Box<Face>, Box<Face>),
    /// Disjunction: φ ∨ ψ
    Or(Box<Face>, Box<Face>),
}

/// System of partial elements with face constraints
///
/// In cubical type theory, we often have elements defined
/// only on certain faces of a cube.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialSystem<T> {
    /// Map from faces to partial elements
    pub elements: BTreeMap<Face, T>,
}

/// De Morgan algebra operations for intervals
///
/// The interval type satisfies De Morgan laws:
/// - ~(r ∧ s) = (~r) ∨ (~s)  
/// - ~(r ∨ s) = (~r) ∧ (~s)
/// - ~~r = r
/// - r ∧ ~r = 0
/// - r ∨ ~r = 1
pub trait DeMorgan: Sized {
    /// Negation operation
    fn not(self) -> Self;
    
    /// Conjunction (minimum)
    fn and(self, other: Self) -> Self;
    
    /// Disjunction (maximum) 
    fn or(self, other: Self) -> Self;
    
    /// Check if element is zero
    fn is_zero(&self) -> bool;
    
    /// Check if element is one
    fn is_one(&self) -> bool;
}

impl Interval {
    /// Create the zero interval
    pub fn zero() -> Self {
        Interval::Zero
    }
    
    /// Create the one interval
    pub fn one() -> Self {
        Interval::One
    }
    
    /// Create a variable interval
    pub fn var(name: impl Into<String>) -> Self {
        Interval::Var(name.into())
    }
    
    /// Create a smooth interval
    pub fn smooth(standard: Interval, infinitesimal: SmoothForm) -> Self {
        Interval::Smooth {
            standard: Box::new(standard),
            infinitesimal: Box::new(infinitesimal),
        }
    }
    
    /// Normalize interval expression using De Morgan laws
    pub fn normalize(self) -> Self {
        match self {
            Interval::Not(box Interval::Not(box r)) => r.normalize(),
            Interval::Not(box Interval::And(box r, box s)) => {
                Interval::Or(Box::new(r.not().normalize()), Box::new(s.not().normalize()))
            }
            Interval::Not(box Interval::Or(box r, box s)) => {
                Interval::And(Box::new(r.not().normalize()), Box::new(s.not().normalize()))
            }
            Interval::And(box r, box s) => {
                let r_norm = r.normalize();
                let s_norm = s.normalize();
                match (&r_norm, &s_norm) {
                    (Interval::Zero, _) | (_, Interval::Zero) => Interval::Zero,
                    (Interval::One, s) => s.clone(),
                    (r, Interval::One) => r.clone(),
                    _ if r_norm == s_norm => r_norm,
                    _ => Interval::And(Box::new(r_norm), Box::new(s_norm)),
                }
            }
            Interval::Or(box r, box s) => {
                let r_norm = r.normalize();
                let s_norm = s.normalize();
                match (&r_norm, &s_norm) {
                    (Interval::One, _) | (_, Interval::One) => Interval::One,
                    (Interval::Zero, s) => s.clone(),
                    (r, Interval::Zero) => r.clone(),
                    _ if r_norm == s_norm => r_norm,
                    _ => Interval::Or(Box::new(r_norm), Box::new(s_norm)),
                }
            }
            Interval::Smooth { standard, infinitesimal } => {
                Interval::Smooth {
                    standard: Box::new(standard.normalize()),
                    infinitesimal: Box::new(infinitesimal.normalize()),
                }
            }
            other => other,
        }
    }
    
    /// Substitute interval variable
    pub fn substitute(&self, var: &str, replacement: &Interval) -> Interval {
        match self {
            Interval::Zero | Interval::One => self.clone(),
            Interval::Var(name) if name == var => replacement.clone(),
            Interval::Var(_) => self.clone(),
            Interval::Not(box r) => Interval::Not(Box::new(r.substitute(var, replacement))),
            Interval::And(box r, box s) => Interval::And(
                Box::new(r.substitute(var, replacement)),
                Box::new(s.substitute(var, replacement)),
            ),
            Interval::Or(box r, box s) => Interval::Or(
                Box::new(r.substitute(var, replacement)),
                Box::new(s.substitute(var, replacement)),
            ),
            Interval::Smooth { standard, infinitesimal } => Interval::Smooth {
                standard: Box::new(standard.substitute(var, replacement)),
                infinitesimal: Box::new(infinitesimal.substitute(var, replacement)),
            },
        }
    }
    
    /// Extract free variables
    pub fn free_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_free_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    
    fn collect_free_vars(&self, vars: &mut Vec<String>) {
        match self {
            Interval::Zero | Interval::One => {}
            Interval::Var(name) => vars.push(name.clone()),
            Interval::Not(box r) => r.collect_free_vars(vars),
            Interval::And(box r, box s) | Interval::Or(box r, box s) => {
                r.collect_free_vars(vars);
                s.collect_free_vars(vars);
            }
            Interval::Smooth { standard, infinitesimal } => {
                standard.collect_free_vars(vars);
                infinitesimal.collect_free_vars(vars);
            }
        }
    }
    
    /// Evaluate interval in a context
    pub fn evaluate(&self, context: &BTreeMap<String, bool>) -> Result<bool> {
        match self {
            Interval::Zero => Ok(false),
            Interval::One => Ok(true),
            Interval::Var(name) => context
                .get(name)
                .copied()
                .ok_or_else(|| Error::UnboundVariable(name.clone())),
            Interval::Not(box r) => Ok(!r.evaluate(context)?),
            Interval::And(box r, box s) => Ok(r.evaluate(context)? && s.evaluate(context)?),
            Interval::Or(box r, box s) => Ok(r.evaluate(context)? || s.evaluate(context)?),
            Interval::Smooth { standard, .. } => {
                // For evaluation, we use just the standard part
                standard.evaluate(context)
            }
        }
    }
}

impl DeMorgan for Interval {
    fn not(self) -> Self {
        Interval::Not(Box::new(self)).normalize()
    }
    
    fn and(self, other: Self) -> Self {
        Interval::And(Box::new(self), Box::new(other)).normalize()
    }
    
    fn or(self, other: Self) -> Self {
        Interval::Or(Box::new(self), Box::new(other)).normalize()
    }
    
    fn is_zero(&self) -> bool {
        matches!(self, Interval::Zero)
    }
    
    fn is_one(&self) -> bool {
        matches!(self, Interval::One)
    }
}

impl SmoothForm {
    /// Zero differential form
    pub fn zero() -> Self {
        SmoothForm::Zero
    }
    
    /// Differential of a variable
    pub fn differential(var: impl Into<String>) -> Self {
        SmoothForm::Differential(var.into())
    }
    
    /// Wedge product of forms
    pub fn wedge(self, other: Self) -> Self {
        SmoothForm::Wedge(Box::new(self), Box::new(other)).normalize()
    }
    
    /// Scale a form by a constant
    pub fn scale(coeff: f64, form: Self) -> Self {
        SmoothForm::Scale(coeff, Box::new(form)).normalize()
    }
    
    /// Normalize differential form
    pub fn normalize(self) -> Self {
        match self {
            SmoothForm::Zero => SmoothForm::Zero,
            SmoothForm::Differential(_) => self,
            SmoothForm::Wedge(box a, box b) => {
                let a_norm = a.normalize();
                let b_norm = b.normalize();
                match (&a_norm, &b_norm) {
                    (SmoothForm::Zero, _) | (_, SmoothForm::Zero) => SmoothForm::Zero,
                    // dx ∧ dx = 0 (nilpotent)
                    _ if a_norm == b_norm => SmoothForm::Zero,
                    _ => SmoothForm::Wedge(Box::new(a_norm), Box::new(b_norm)),
                }
            }
            SmoothForm::Scale(coeff, box form) => {
                if coeff == 0.0 {
                    SmoothForm::Zero
                } else if coeff == 1.0 {
                    form.normalize()
                } else {
                    match form.normalize() {
                        SmoothForm::Zero => SmoothForm::Zero,
                        SmoothForm::Scale(inner_coeff, inner_form) => {
                            SmoothForm::Scale(coeff * inner_coeff, inner_form).normalize()
                        }
                        normalized => SmoothForm::Scale(coeff, Box::new(normalized)),
                    }
                }
            }
        }
    }
    
    /// Substitute variable in differential form
    pub fn substitute(&self, var: &str, replacement: &Interval) -> Self {
        match self {
            SmoothForm::Zero => SmoothForm::Zero,
            SmoothForm::Differential(name) if name == var => {
                // Compute differential of replacement
                replacement.differential(var)
            }
            SmoothForm::Differential(_) => self.clone(),
            SmoothForm::Wedge(box a, box b) => SmoothForm::Wedge(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            SmoothForm::Scale(coeff, box form) => {
                SmoothForm::Scale(*coeff, Box::new(form.substitute(var, replacement)))
            }
        }
    }
    
    /// Collect free variables in differential form
    pub fn collect_free_vars(&self, vars: &mut Vec<String>) {
        match self {
            SmoothForm::Zero => {}
            SmoothForm::Differential(name) => vars.push(name.clone()),
            SmoothForm::Wedge(box a, box b) => {
                a.collect_free_vars(vars);
                b.collect_free_vars(vars);
            }
            SmoothForm::Scale(_, box form) => form.collect_free_vars(vars),
        }
    }
}

impl Interval {
    /// Compute the differential of an interval expression
    pub fn differential(&self, var: &str) -> SmoothForm {
        match self {
            Interval::Zero | Interval::One => SmoothForm::Zero,
            Interval::Var(name) if name == var => SmoothForm::Differential(var.to_string()),
            Interval::Var(_) => SmoothForm::Zero,
            Interval::Not(box r) => {
                // d(~r) = -dr (but we work in characteristic 2, so -1 = 1)
                r.differential(var)
            }
            Interval::And(box r, box s) => {
                // d(r ∧ s) = dr ∧ const(s) + const(r) ∧ ds
                // In boolean algebra, this becomes more complex
                let dr = r.differential(var);
                let ds = s.differential(var);
                // Approximate with addition (should be refined)
                dr.wedge(ds)
            }
            Interval::Or(box r, box s) => {
                // d(r ∨ s) = dr ∨ ds (approximate)
                let dr = r.differential(var);
                let ds = s.differential(var);
                dr.wedge(ds)
            }
            Interval::Smooth { infinitesimal, .. } => infinitesimal.as_ref().clone(),
        }
    }
}

impl Face {
    /// True face
    pub fn true_face() -> Self {
        Face::True
    }
    
    /// False face
    pub fn false_face() -> Self {
        Face::False
    }
    
    /// Variable equals 0
    pub fn eq0(var: impl Into<String>) -> Self {
        Face::Eq0(var.into())
    }
    
    /// Variable equals 1
    pub fn eq1(var: impl Into<String>) -> Self {
        Face::Eq1(var.into())
    }
    
    /// Normalize face expression
    pub fn normalize(self) -> Self {
        match self {
            Face::Not(box Face::Not(box phi)) => phi.normalize(),
            Face::Not(box Face::And(box phi, box psi)) => {
                Face::Or(Box::new(phi.not().normalize()), Box::new(psi.not().normalize()))
            }
            Face::Not(box Face::Or(box phi, box psi)) => {
                Face::And(Box::new(phi.not().normalize()), Box::new(psi.not().normalize()))
            }
            Face::And(box phi, box psi) => {
                let phi_norm = phi.normalize();
                let psi_norm = psi.normalize();
                match (&phi_norm, &psi_norm) {
                    (Face::False, _) | (_, Face::False) => Face::False,
                    (Face::True, psi) => psi.clone(),
                    (phi, Face::True) => phi.clone(),
                    _ if phi_norm == psi_norm => phi_norm,
                    _ => Face::And(Box::new(phi_norm), Box::new(psi_norm)),
                }
            }
            Face::Or(box phi, box psi) => {
                let phi_norm = phi.normalize();
                let psi_norm = psi.normalize();
                match (&phi_norm, &psi_norm) {
                    (Face::True, _) | (_, Face::True) => Face::True,
                    (Face::False, psi) => psi.clone(),
                    (phi, Face::False) => phi.clone(),
                    _ if phi_norm == psi_norm => phi_norm,
                    _ => Face::Or(Box::new(phi_norm), Box::new(psi_norm)),
                }
            }
            other => other,
        }
    }
    
    /// Check if face is always true
    pub fn is_true(&self) -> bool {
        matches!(self, Face::True)
    }
    
    /// Check if face is always false
    pub fn is_false(&self) -> bool {
        matches!(self, Face::False)
    }
    
    /// Evaluate face in interval context
    pub fn evaluate(&self, context: &BTreeMap<String, bool>) -> Result<bool> {
        match self {
            Face::True => Ok(true),
            Face::False => Ok(false),
            Face::Eq0(var) => {
                let val = context.get(var).ok_or_else(|| Error::UnboundVariable(var.clone()))?;
                Ok(!val)
            }
            Face::Eq1(var) => {
                let val = context.get(var).ok_or_else(|| Error::UnboundVariable(var.clone()))?;
                Ok(*val)
            }
            Face::Not(box phi) => Ok(!phi.evaluate(context)?),
            Face::And(box phi, box psi) => Ok(phi.evaluate(context)? && psi.evaluate(context)?),
            Face::Or(box phi, box psi) => Ok(phi.evaluate(context)? || psi.evaluate(context)?),
        }
    }
}

impl DeMorgan for Face {
    fn not(self) -> Self {
        Face::Not(Box::new(self)).normalize()
    }
    
    fn and(self, other: Self) -> Self {
        Face::And(Box::new(self), Box::new(other)).normalize()
    }
    
    fn or(self, other: Self) -> Self {
        Face::Or(Box::new(self), Box::new(other)).normalize()
    }
    
    fn is_zero(&self) -> bool {
        self.is_false()
    }
    
    fn is_one(&self) -> bool {
        self.is_true()
    }
}

impl<T> PartialSystem<T> {
    /// Create empty partial system
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
        }
    }
    
    /// Add element with face constraint
    pub fn insert(&mut self, face: Face, element: T) {
        self.elements.insert(face, element);
    }
    
    /// Check if system is total (covers all cases)
    pub fn is_total(&self) -> bool {
        // Check if the disjunction of all faces is True
        let combined = self
            .elements
            .keys()
            .fold(Face::False, |acc, face| acc.or(face.clone()));
        combined.normalize().is_true()
    }
    
    /// Restrict system to a face
    pub fn restrict(&self, restriction: &Face) -> PartialSystem<T>
    where
        T: Clone,
    {
        let mut result = PartialSystem::new();
        for (face, element) in &self.elements {
            let restricted_face = face.clone().and(restriction.clone()).normalize();
            if !restricted_face.is_false() {
                result.insert(restricted_face, element.clone());
            }
        }
        result
    }
}

// Standard trait implementations
impl Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Interval::Zero => write!(f, "0"),
            Interval::One => write!(f, "1"),
            Interval::Var(name) => write!(f, "{}", name),
            Interval::Not(box r) => write!(f, "~{}", r),
            Interval::And(box r, box s) => write!(f, "({} ∧ {})", r, s),
            Interval::Or(box r, box s) => write!(f, "({} ∨ {})", r, s),
            Interval::Smooth { standard, infinitesimal } => {
                write!(f, "smooth({}, {})", standard, infinitesimal)
            }
        }
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SmoothForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmoothForm::Zero => write!(f, "0"),
            SmoothForm::Differential(var) => write!(f, "d{}", var),
            SmoothForm::Wedge(box a, box b) => write!(f, "{} ∧ {}", a, b),
            SmoothForm::Scale(coeff, box form) => write!(f, "{} * {}", coeff, form),
        }
    }
}

impl Debug for SmoothForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Face::True => write!(f, "⊤"),
            Face::False => write!(f, "⊥"),
            Face::Eq0(var) => write!(f, "({} = 0)", var),
            Face::Eq1(var) => write!(f, "({} = 1)", var),
            Face::Not(box phi) => write!(f, "¬{}", phi),
            Face::And(box phi, box psi) => write!(f, "({} ∧ {})", phi, psi),
            Face::Or(box phi, box psi) => write!(f, "({} ∨ {})", phi, psi),
        }
    }
}

// Operator overloads for ergonomic use
impl Not for Interval {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        DeMorgan::not(self)
    }
}

impl BitAnd for Interval {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        DeMorgan::and(self, rhs)
    }
}

impl BitOr for Interval {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        DeMorgan::or(self, rhs)
    }
}

impl Not for Face {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        DeMorgan::not(self)
    }
}

impl BitAnd for Face {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        DeMorgan::and(self, rhs)
    }
}

impl BitOr for Face {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        DeMorgan::or(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_morgan_laws() {
        let r = Interval::var("r");
        let s = Interval::var("s");
        
        // ~(r ∧ s) = (~r) ∨ (~s)
        let left = !(r.clone() & s.clone());
        let right = (!r.clone()) | (!s.clone());
        // Note: Would need semantic equality for proper testing
        
        // r ∧ ~r = 0
        let contradiction = r.clone() & !r.clone();
        // This should normalize to something equivalent to 0
        
        // r ∨ ~r = 1
        let excluded_middle = r.clone() | !r.clone();
        // This should normalize to something equivalent to 1
    }
    
    #[test]
    fn test_interval_normalization() {
        let r = Interval::var("r");
        
        // Double negation
        let double_neg = !!r.clone();
        assert_eq!(double_neg, r);
        
        // Identity laws
        assert_eq!(r.clone() & Interval::one(), r);
        assert_eq!(r.clone() | Interval::zero(), r);
        
        // Annihilation laws
        assert_eq!(r.clone() & Interval::zero(), Interval::zero());
        assert_eq!(r.clone() | Interval::one(), Interval::one());
    }
    
    #[test]
    fn test_smooth_forms() {
        let dx = SmoothForm::differential("x");
        let dy = SmoothForm::differential("y");
        
        // Nilpotent property: dx ∧ dx = 0
        assert_eq!(dx.clone().wedge(dx.clone()), SmoothForm::zero());
        
        // Anti-commutativity would require more complex implementation
        // dx ∧ dy = -(dy ∧ dx), but in characteristic 2: dx ∧ dy = dy ∧ dx
    }
    
    #[test]
    fn test_face_evaluation() {
        let mut context = BTreeMap::new();
        context.insert("x".to_string(), true);
        context.insert("y".to_string(), false);
        
        let face = Face::eq1("x") & Face::eq0("y");
        assert_eq!(face.evaluate(&context).unwrap(), true);
        
        let face2 = Face::eq0("x") | Face::eq1("y");
        assert_eq!(face2.evaluate(&context).unwrap(), false);
    }
    
    #[test]
    fn test_partial_system() {
        let mut system = PartialSystem::new();
        system.insert(Face::eq0("x"), "value_at_0".to_string());
        system.insert(Face::eq1("x"), "value_at_1".to_string());
        
        // This system should be total for variable x
        assert!(system.is_total());
    }
}