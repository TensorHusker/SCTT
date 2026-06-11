//! # Semantic Domain for NbE
//!
//! Values are the semantic counterpart of syntactic terms. The evaluation
//! phase maps `Term → Value` (firing β-rules), and the quote/readback phase
//! maps `Value → Term` (eta-expanding neutrals).
//!
//! ## Level vs Index
//! - **Term variables in syntax** use de Bruijn *indices* (count binders inward).
//! - **Term variables in values** use de Bruijn *levels* (count from the bottom).
//!   A level `l` in an environment of depth `d` corresponds to index `d - 1 - l`.
//! - The same convention applies to dimension variables.

use std::sync::Arc;
use crate::syntax::{Term, Level, Smoothness};
use crate::dim::DimLevel;

// ─── Level Newtype ────────────────────────────────────────────────────────────

/// A de Bruijn *level* for a term variable (counts from the bottom of the stack).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermLevel(pub usize);

// ─── Dimension Values ─────────────────────────────────────────────────────────

/// A semantic dimension: either a variable (by level) or a concrete endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimVal {
    Var(DimLevel),
    Zero,
    One,
}

// ─── Cofibration Values ───────────────────────────────────────────────────────

/// A semantic cofibration, mirroring `Cof` but with `DimVal` atoms.
#[derive(Debug, Clone, PartialEq)]
pub enum CofVal {
    Eq(DimVal, DimVal),
    And(Box<CofVal>, Box<CofVal>),
    Or(Box<CofVal>, Box<CofVal>),
    Top,
    Bot,
}

impl CofVal {
    /// Smart constructor: immediately simplify concrete equalities.
    pub fn eq(r: DimVal, s: DimVal) -> Self {
        match (&r, &s) {
            (DimVal::Zero, DimVal::Zero) | (DimVal::One, DimVal::One) => CofVal::Top,
            (DimVal::Zero, DimVal::One) | (DimVal::One, DimVal::Zero) => CofVal::Bot,
            _ => CofVal::Eq(r, s),
        }
    }

    pub fn and(phi: CofVal, psi: CofVal) -> Self {
        match (&phi, &psi) {
            (CofVal::Top, _) => psi,
            (_, CofVal::Top) => phi,
            (CofVal::Bot, _) | (_, CofVal::Bot) => CofVal::Bot,
            _ => CofVal::And(Box::new(phi), Box::new(psi)),
        }
    }

    pub fn or(phi: CofVal, psi: CofVal) -> Self {
        match (&phi, &psi) {
            (CofVal::Top, _) | (_, CofVal::Top) => CofVal::Top,
            (CofVal::Bot, _) => psi,
            (_, CofVal::Bot) => phi,
            _ => CofVal::Or(Box::new(phi), Box::new(psi)),
        }
    }

    pub fn is_true(&self) -> bool { matches!(self, CofVal::Top) }
    pub fn is_false(&self) -> bool { matches!(self, CofVal::Bot) }
}

// ─── Environment ─────────────────────────────────────────────────────────────

/// An evaluation environment holding term bindings (by level) and dim bindings (by level).
#[derive(Clone, Debug)]
pub struct Env {
    /// `terms[l]` is the value of the term variable at level `l`.
    pub terms: Vec<Arc<Value>>,
    /// `dims[l]` is the value of the dimension variable at level `l`.
    pub dims: Vec<DimVal>,
}

impl Env {
    pub fn new() -> Self {
        Env { terms: Vec::new(), dims: Vec::new() }
    }

    /// Extend with a new term variable binding (highest level = `terms.len()` before push).
    pub fn extend_term(&self, val: Arc<Value>) -> Self {
        let mut e = self.clone();
        e.terms.push(val);
        e
    }

    /// Extend with a new dimension variable binding.
    pub fn extend_dim(&self, dim: DimVal) -> Self {
        let mut e = self.clone();
        e.dims.push(dim);
        e
    }

    /// Look up a term *level* and return its value.
    ///
    /// Levels count from the bottom (0 = outermost), so
    /// `TermLevel(l)` maps to `terms[l]`.
    pub fn lookup_term(&self, lvl: TermLevel) -> Arc<Value> {
        Arc::clone(&self.terms[lvl.0])
    }

    /// Look up a dimension *level*.
    pub fn lookup_dim(&self, lvl: DimLevel) -> DimVal {
        self.dims[lvl.0].clone()
    }

    /// Number of term variables in scope.
    pub fn term_depth(&self) -> usize { self.terms.len() }

    /// Number of dimension variables in scope.
    pub fn dim_depth(&self) -> usize { self.dims.len() }
}

impl Default for Env {
    fn default() -> Self { Env::new() }
}

// ─── Closures ─────────────────────────────────────────────────────────────────

/// A term closure: an environment paired with a body that binds one more term variable.
#[derive(Clone, Debug)]
pub struct Closure {
    pub env: Env,
    pub body: Arc<Term>,
}

/// A dimension closure: an environment paired with a body that binds one more dim variable.
#[derive(Clone, Debug)]
pub struct DimClosure {
    pub env: Env,
    pub body: Arc<Term>,
}

// Note: `apply` methods live in `evaluate.rs` to avoid a circular-dependency cycle.
// Use `apply_closure` / `apply_dim_closure` from that module instead.

// ─── Boundary Values ─────────────────────────────────────────────────────────

/// A semantic boundary system: a list of (cofibration, dim-closure) branches.
#[derive(Clone, Debug)]
pub struct BdryVal {
    pub branches: Vec<(CofVal, DimClosure)>,
}

// ─── Values ───────────────────────────────────────────────────────────────────

/// The semantic domain. Every `Value` is either a canonical form or a neutral.
#[derive(Clone, Debug)]
pub enum Value {
    /// A stuck computation together with its type.
    Neutral(Arc<Neutral>, Arc<Value>),

    // ── MLTT ──────────────────────────────────────────────────────────────────
    Universe(Level),
    Pi(Arc<Value>, Closure),        // domain type, codomain closure
    Lambda(Closure),
    Sigma(Arc<Value>, Closure),     // fst type, snd type closure
    Pair(Arc<Value>, Arc<Value>),

    // ── Cubical ───────────────────────────────────────────────────────────────
    PathType(DimClosure, Arc<Value>, Arc<Value>),  // line, left endpoint, right endpoint
    PathLam(DimClosure),
    /// Stuck coercion (fires only when `from == to`; otherwise stays stuck).
    Coe(DimVal, DimVal, DimClosure, Arc<Value>),
    /// Stuck homogeneous composition.
    HCom(DimVal, DimVal, Arc<Value>, BdryVal, Arc<Value>),

    // ── Glue ──────────────────────────────────────────────────────────────────
    GlueType(Arc<Value>, CofVal, Arc<Value>),
    GlueElem(Arc<Value>, CofVal, Arc<Value>),

    // ── Smooth ────────────────────────────────────────────────────────────────
    SmoothPathType(Smoothness, Arc<Value>, Arc<Value>, Arc<Value>),
    Tangent(Arc<Value>),

    // ── Natural Numbers ───────────────────────────────────────────────────────
    Nat,
    Zero,
    Succ(Arc<Value>),

    // ── ARC Grid Types ─────────────────────────────────────────────────────
    /// The type of colors (Fin(10)).
    ColorType,
    /// A color value (0-9).
    Color(u8),
    /// A concrete grid value: rows × cols matrix of colors.
    GridLit { rows: usize, cols: usize, data: Vec<u8> },
}

// ─── Neutrals ─────────────────────────────────────────────────────────────────

/// Stuck (neutral) computations — headed by a free variable.
#[derive(Clone, Debug)]
pub enum Neutral {
    /// Free term variable (by level).
    Var(TermLevel),
    /// Stuck function application: `ne arg`.
    App(Arc<Neutral>, Arc<Value>, Arc<Value>),    // ne, arg, result type
    /// Stuck first projection.
    Fst(Arc<Neutral>, Arc<Value>),
    /// Stuck second projection.
    Snd(Arc<Neutral>, Arc<Value>),
    /// Stuck path application at a dimension.
    PathApp(Arc<Neutral>, DimVal, Arc<Value>),
    /// Stuck natural-number eliminator.
    NatElim(Arc<Value>, Arc<Value>, Arc<Value>, Arc<Neutral>), // motive, base, step, ne
    /// Stuck unglue.
    Unglue(Arc<Neutral>, CofVal, Arc<Value>),
    /// Stuck differentiation.
    Diff(Arc<Neutral>, Arc<Value>),
}
