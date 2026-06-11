//! # Term Syntax for Smooth Cubical Type Theory
//!
//! Three layers compose to form SCTT:
//! 1. **MLTT base**: Pi, Sigma, Nat, Universe
//! 2. **Cubical**: PathType, PathLam, PathApp, Coe, HCom
//! 3. **Smooth**: SmoothPath, Tangent, Diff, smoothness tracking
//!
//! ## Variable Namespaces
//! - `Index`: de Bruijn index for term variables (bound by Lambda, Pi, Sigma)
//! - `DimIndex`: de Bruijn index for dimension variables (bound by PathLam, PathType)

use std::fmt;
use std::sync::Arc;
use crate::dim::{Dim, DimIndex};
use crate::cof::Cof;

pub const MAX_UNIVERSE_LEVEL: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(u8);

impl Level {
    pub fn new(n: u8) -> Option<Self> {
        if n <= MAX_UNIVERSE_LEVEL { Some(Level(n)) } else { None }
    }
    pub fn zero() -> Self { Level(0) }
    pub fn succ(self) -> Option<Self> { Level::new(self.0 + 1) }
    pub fn max(self, other: Self) -> Self { Level(self.0.max(other.0)) }
    pub fn value(self) -> u8 { self.0 }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index(pub usize);

impl Index {
    pub fn new(n: usize) -> Self { Index(n) }
    pub fn shift(self, cutoff: usize, amount: isize) -> Self {
        if self.0 >= cutoff {
            let new_val = self.0 as isize + amount;
            assert!(new_val >= 0, "de Bruijn index underflow");
            Index(new_val as usize)
        } else {
            self
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Smoothness classification for paths and operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Smoothness {
    Continuous,
    C(u32),
    CInfty,
}

impl Smoothness {
    pub fn meet(self, other: Self) -> Self {
        use Smoothness::*;
        match (self, other) {
            (CInfty, x) | (x, CInfty) => x,
            (C(a), C(b)) => C(a.min(b)),
            (Continuous, _) | (_, Continuous) => Continuous,
        }
    }
}

impl fmt::Display for Smoothness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Smoothness::Continuous => write!(f, "C0"),
            Smoothness::C(n) => write!(f, "C^{}", n),
            Smoothness::CInfty => write!(f, "C^inf"),
        }
    }
}

/// A branch in a boundary system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BdryBranch {
    pub cof: Cof,
    pub body: Arc<Term>,
}

/// Core term syntax for Smooth Cubical Type Theory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    // --- Standard MLTT ---
    Var(Index),
    Universe(Level),
    Pi { domain: Arc<Term>, codomain: Arc<Term> },
    Lambda { body: Arc<Term> },
    App { func: Arc<Term>, arg: Arc<Term> },
    Sigma { fst_type: Arc<Term>, snd_type: Arc<Term> },
    Pair { fst: Arc<Term>, snd: Arc<Term> },
    Fst(Arc<Term>),
    Snd(Arc<Term>),
    // --- Cubical ---
    PathType { line: Arc<Term>, left: Arc<Term>, right: Arc<Term> },
    PathLam { body: Arc<Term> },
    PathApp { path: Arc<Term>, dim: Dim },
    Coe { from: Dim, to: Dim, line: Arc<Term>, body: Arc<Term> },
    HCom { from: Dim, to: Dim, ty: Arc<Term>, branches: Vec<BdryBranch>, base: Arc<Term> },
    // --- Glue Types ---
    GlueType { base: Arc<Term>, cof: Cof, fiber_equiv: Arc<Term> },
    GlueElem { base: Arc<Term>, cof: Cof, fiber_elem: Arc<Term> },
    Unglue { body: Arc<Term>, cof: Cof, fiber_equiv: Arc<Term> },
    // --- Smooth ---
    SmoothPath { order: Smoothness, ty: Arc<Term>, start: Arc<Term>, end: Arc<Term> },
    Tangent(Arc<Term>),
    Diff(Arc<Term>),
    // --- Natural Numbers ---
    Nat,
    Zero,
    Succ(Arc<Term>),
    NatElim { motive: Arc<Term>, base: Arc<Term>, step: Arc<Term>, scrutinee: Arc<Term> },
    // --- ARC Grid Types ---
    /// The type of ARC colors (0-9). Semantically Fin(10).
    ColorType,
    /// A color literal (0-9).
    Color(u8),
    /// A concrete grid: rows × cols matrix of colors.
    /// Semantically: Fin(rows) → Fin(cols) → Color
    GridLit { rows: usize, cols: usize, data: Vec<u8> },
}

// Smart constructors
impl Term {
    pub fn var(n: usize) -> Self { Term::Var(Index::new(n)) }
    pub fn universe(level: u8) -> Option<Self> { Level::new(level).map(Term::Universe) }
    pub fn pi(domain: Term, codomain: Term) -> Self {
        Term::Pi { domain: Arc::new(domain), codomain: Arc::new(codomain) }
    }
    pub fn lambda(body: Term) -> Self { Term::Lambda { body: Arc::new(body) } }
    pub fn app(func: Term, arg: Term) -> Self {
        Term::App { func: Arc::new(func), arg: Arc::new(arg) }
    }
    pub fn sigma(fst_type: Term, snd_type: Term) -> Self {
        Term::Sigma { fst_type: Arc::new(fst_type), snd_type: Arc::new(snd_type) }
    }
    pub fn pair(fst: Term, snd: Term) -> Self {
        Term::Pair { fst: Arc::new(fst), snd: Arc::new(snd) }
    }
    pub fn fst(p: Term) -> Self { Term::Fst(Arc::new(p)) }
    pub fn snd(p: Term) -> Self { Term::Snd(Arc::new(p)) }
    pub fn succ(n: Term) -> Self { Term::Succ(Arc::new(n)) }
    pub fn nat_elim(motive: Term, base: Term, step: Term, scrutinee: Term) -> Self {
        Term::NatElim {
            motive: Arc::new(motive), base: Arc::new(base),
            step: Arc::new(step), scrutinee: Arc::new(scrutinee),
        }
    }
    pub fn path_type(line: Term, left: Term, right: Term) -> Self {
        Term::PathType { line: Arc::new(line), left: Arc::new(left), right: Arc::new(right) }
    }
    pub fn path(ty: Term, left: Term, right: Term) -> Self {
        Term::path_type(ty, left, right)
    }
    pub fn path_lam(body: Term) -> Self { Term::PathLam { body: Arc::new(body) } }
    pub fn path_app(path: Term, dim: Dim) -> Self {
        Term::PathApp { path: Arc::new(path), dim }
    }
    pub fn coe(from: Dim, to: Dim, line: Term, body: Term) -> Self {
        Term::Coe { from, to, line: Arc::new(line), body: Arc::new(body) }
    }
    pub fn hcom(from: Dim, to: Dim, ty: Term, branches: Vec<BdryBranch>, base: Term) -> Self {
        Term::HCom { from, to, ty: Arc::new(ty), branches, base: Arc::new(base) }
    }
    pub fn smooth_path(order: Smoothness, ty: Term, start: Term, end: Term) -> Self {
        Term::SmoothPath { order, ty: Arc::new(ty), start: Arc::new(start), end: Arc::new(end) }
    }
    pub fn tangent(ty: Term) -> Self { Term::Tangent(Arc::new(ty)) }
    pub fn diff(f: Term) -> Self { Term::Diff(Arc::new(f)) }
    pub fn glue_type(base: Term, cof: Cof, fiber_equiv: Term) -> Self {
        Term::GlueType { base: Arc::new(base), cof, fiber_equiv: Arc::new(fiber_equiv) }
    }
    pub fn nat_lit(n: u64) -> Self {
        let mut t = Term::Zero;
        for _ in 0..n { t = Term::succ(t); }
        t
    }
    pub fn color(c: u8) -> Self {
        assert!(c < 10, "ARC color must be 0-9, got {}", c);
        Term::Color(c)
    }
    pub fn grid_lit(rows: usize, cols: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), rows * cols, "grid data length mismatch");
        assert!(data.iter().all(|&c| c < 10), "all grid cells must be colors 0-9");
        Term::GridLit { rows, cols, data }
    }
}

// ─── Substitution and Shifting ───────────────────────────────────────────────

impl Term {
    /// Shift all term de Bruijn indices >= `cutoff` by `amount`.
    ///
    /// Term binders (Lambda body, Pi codomain, Sigma snd_type) increment `cutoff`.
    /// Dim binders (PathLam body) do NOT affect the term cutoff.
    pub fn shift_term(&self, cutoff: usize, amount: isize) -> Term {
        match self {
            Term::Var(idx) => Term::Var(idx.shift(cutoff, amount)),
            Term::Universe(l) => Term::Universe(*l),
            Term::Pi { domain, codomain } => Term::Pi {
                domain: Arc::new(domain.shift_term(cutoff, amount)),
                // Pi binds a term var in the codomain
                codomain: Arc::new(codomain.shift_term(cutoff + 1, amount)),
            },
            Term::Lambda { body } => Term::Lambda {
                // Lambda binds a term var
                body: Arc::new(body.shift_term(cutoff + 1, amount)),
            },
            Term::App { func, arg } => Term::App {
                func: Arc::new(func.shift_term(cutoff, amount)),
                arg: Arc::new(arg.shift_term(cutoff, amount)),
            },
            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Arc::new(fst_type.shift_term(cutoff, amount)),
                // Sigma binds a term var in the snd_type
                snd_type: Arc::new(snd_type.shift_term(cutoff + 1, amount)),
            },
            Term::Pair { fst, snd } => Term::Pair {
                fst: Arc::new(fst.shift_term(cutoff, amount)),
                snd: Arc::new(snd.shift_term(cutoff, amount)),
            },
            Term::Fst(p) => Term::Fst(Arc::new(p.shift_term(cutoff, amount))),
            Term::Snd(p) => Term::Snd(Arc::new(p.shift_term(cutoff, amount))),
            // Cubical: PathLam binds a dim var, NOT a term var — cutoff unchanged
            Term::PathType { line, left, right } => Term::PathType {
                // PathType line binds a dim var — no change to term cutoff
                line: Arc::new(line.shift_term(cutoff, amount)),
                left: Arc::new(left.shift_term(cutoff, amount)),
                right: Arc::new(right.shift_term(cutoff, amount)),
            },
            Term::PathLam { body } => Term::PathLam {
                // PathLam binds a dim var — no change to term cutoff
                body: Arc::new(body.shift_term(cutoff, amount)),
            },
            Term::PathApp { path, dim } => Term::PathApp {
                path: Arc::new(path.shift_term(cutoff, amount)),
                dim: dim.clone(),
            },
            Term::Coe { from, to, line, body } => Term::Coe {
                from: from.clone(),
                to: to.clone(),
                // Coe line binds a dim var — no change to term cutoff
                line: Arc::new(line.shift_term(cutoff, amount)),
                body: Arc::new(body.shift_term(cutoff, amount)),
            },
            Term::HCom { from, to, ty, branches, base } => Term::HCom {
                from: from.clone(),
                to: to.clone(),
                ty: Arc::new(ty.shift_term(cutoff, amount)),
                branches: branches.iter().map(|br| BdryBranch {
                    cof: br.cof.clone(),
                    // HCom branches bind a dim var — no change to term cutoff
                    body: Arc::new(br.body.shift_term(cutoff, amount)),
                }).collect(),
                base: Arc::new(base.shift_term(cutoff, amount)),
            },
            Term::GlueType { base, cof, fiber_equiv } => Term::GlueType {
                base: Arc::new(base.shift_term(cutoff, amount)),
                cof: cof.clone(),
                fiber_equiv: Arc::new(fiber_equiv.shift_term(cutoff, amount)),
            },
            Term::GlueElem { base, cof, fiber_elem } => Term::GlueElem {
                base: Arc::new(base.shift_term(cutoff, amount)),
                cof: cof.clone(),
                fiber_elem: Arc::new(fiber_elem.shift_term(cutoff, amount)),
            },
            Term::Unglue { body, cof, fiber_equiv } => Term::Unglue {
                body: Arc::new(body.shift_term(cutoff, amount)),
                cof: cof.clone(),
                fiber_equiv: Arc::new(fiber_equiv.shift_term(cutoff, amount)),
            },
            Term::SmoothPath { order, ty, start, end } => Term::SmoothPath {
                order: *order,
                ty: Arc::new(ty.shift_term(cutoff, amount)),
                start: Arc::new(start.shift_term(cutoff, amount)),
                end: Arc::new(end.shift_term(cutoff, amount)),
            },
            Term::Tangent(t) => Term::Tangent(Arc::new(t.shift_term(cutoff, amount))),
            Term::Diff(t) => Term::Diff(Arc::new(t.shift_term(cutoff, amount))),
            Term::Nat => Term::Nat,
            Term::Zero => Term::Zero,
            Term::Succ(n) => Term::Succ(Arc::new(n.shift_term(cutoff, amount))),
            Term::NatElim { motive, base, step, scrutinee } => Term::NatElim {
                motive: Arc::new(motive.shift_term(cutoff, amount)),
                base: Arc::new(base.shift_term(cutoff, amount)),
                step: Arc::new(step.shift_term(cutoff, amount)),
                scrutinee: Arc::new(scrutinee.shift_term(cutoff, amount)),
            },
            // ARC grid types: no term subterms — pass through
            Term::ColorType => Term::ColorType,
            Term::Color(c) => Term::Color(*c),
            Term::GridLit { rows, cols, data } => Term::GridLit {
                rows: *rows, cols: *cols, data: data.clone(),
            },
        }
    }

    /// Substitute term variable `target` with `replacement` throughout `self`.
    ///
    /// Under term binders: target+1 (the binder introduces a new var at 0),
    /// and the replacement must be shifted up by 1 to keep it valid.
    /// Under dim binders (PathLam, PathType line, Coe line, HCom branches):
    /// no change to target or replacement.
    pub fn subst_term(&self, target: usize, replacement: &Term) -> Term {
        match self {
            Term::Var(Index(i)) => {
                if *i == target {
                    replacement.clone()
                } else {
                    Term::Var(Index(*i))
                }
            }
            Term::Universe(l) => Term::Universe(*l),
            Term::Pi { domain, codomain } => Term::Pi {
                domain: Arc::new(domain.subst_term(target, replacement)),
                codomain: Arc::new(codomain.subst_term(
                    target + 1,
                    &replacement.shift_term(0, 1),
                )),
            },
            Term::Lambda { body } => Term::Lambda {
                body: Arc::new(body.subst_term(
                    target + 1,
                    &replacement.shift_term(0, 1),
                )),
            },
            Term::App { func, arg } => Term::App {
                func: Arc::new(func.subst_term(target, replacement)),
                arg: Arc::new(arg.subst_term(target, replacement)),
            },
            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Arc::new(fst_type.subst_term(target, replacement)),
                snd_type: Arc::new(snd_type.subst_term(
                    target + 1,
                    &replacement.shift_term(0, 1),
                )),
            },
            Term::Pair { fst, snd } => Term::Pair {
                fst: Arc::new(fst.subst_term(target, replacement)),
                snd: Arc::new(snd.subst_term(target, replacement)),
            },
            Term::Fst(p) => Term::Fst(Arc::new(p.subst_term(target, replacement))),
            Term::Snd(p) => Term::Snd(Arc::new(p.subst_term(target, replacement))),
            // Dim binders: PathLam, PathType line — no change to term target
            Term::PathType { line, left, right } => Term::PathType {
                line: Arc::new(line.subst_term(target, replacement)),
                left: Arc::new(left.subst_term(target, replacement)),
                right: Arc::new(right.subst_term(target, replacement)),
            },
            Term::PathLam { body } => Term::PathLam {
                // PathLam binds a dim var, not a term var
                body: Arc::new(body.subst_term(target, replacement)),
            },
            Term::PathApp { path, dim } => Term::PathApp {
                path: Arc::new(path.subst_term(target, replacement)),
                dim: dim.clone(),
            },
            Term::Coe { from, to, line, body } => Term::Coe {
                from: from.clone(),
                to: to.clone(),
                // Coe line binds a dim var
                line: Arc::new(line.subst_term(target, replacement)),
                body: Arc::new(body.subst_term(target, replacement)),
            },
            Term::HCom { from, to, ty, branches, base } => Term::HCom {
                from: from.clone(),
                to: to.clone(),
                ty: Arc::new(ty.subst_term(target, replacement)),
                branches: branches.iter().map(|br| BdryBranch {
                    cof: br.cof.clone(),
                    // HCom branches bind a dim var
                    body: Arc::new(br.body.subst_term(target, replacement)),
                }).collect(),
                base: Arc::new(base.subst_term(target, replacement)),
            },
            Term::GlueType { base, cof, fiber_equiv } => Term::GlueType {
                base: Arc::new(base.subst_term(target, replacement)),
                cof: cof.clone(),
                fiber_equiv: Arc::new(fiber_equiv.subst_term(target, replacement)),
            },
            Term::GlueElem { base, cof, fiber_elem } => Term::GlueElem {
                base: Arc::new(base.subst_term(target, replacement)),
                cof: cof.clone(),
                fiber_elem: Arc::new(fiber_elem.subst_term(target, replacement)),
            },
            Term::Unglue { body, cof, fiber_equiv } => Term::Unglue {
                body: Arc::new(body.subst_term(target, replacement)),
                cof: cof.clone(),
                fiber_equiv: Arc::new(fiber_equiv.subst_term(target, replacement)),
            },
            Term::SmoothPath { order, ty, start, end } => Term::SmoothPath {
                order: *order,
                ty: Arc::new(ty.subst_term(target, replacement)),
                start: Arc::new(start.subst_term(target, replacement)),
                end: Arc::new(end.subst_term(target, replacement)),
            },
            Term::Tangent(t) => Term::Tangent(Arc::new(t.subst_term(target, replacement))),
            Term::Diff(t) => Term::Diff(Arc::new(t.subst_term(target, replacement))),
            Term::Nat => Term::Nat,
            Term::Zero => Term::Zero,
            Term::Succ(n) => Term::Succ(Arc::new(n.subst_term(target, replacement))),
            Term::NatElim { motive, base, step, scrutinee } => Term::NatElim {
                motive: Arc::new(motive.subst_term(target, replacement)),
                base: Arc::new(base.subst_term(target, replacement)),
                step: Arc::new(step.subst_term(target, replacement)),
                scrutinee: Arc::new(scrutinee.subst_term(target, replacement)),
            },
            // ARC grid types: no term variables — pass through
            Term::ColorType => Term::ColorType,
            Term::Color(c) => Term::Color(*c),
            Term::GridLit { rows, cols, data } => Term::GridLit {
                rows: *rows, cols: *cols, data: data.clone(),
            },
        }
    }

    /// Substitute dimension variable `target` with `replacement` throughout `self`.
    ///
    /// Under dim binders (PathLam body, PathType line, Coe line, HCom branches):
    /// target+1 and replacement shifted by +1 in the dim namespace.
    /// Under term binders (Lambda, Pi codomain, Sigma snd_type): no change to dim target.
    pub fn subst_dim(&self, target: DimIndex, replacement: &Dim) -> Term {
        match self {
            Term::Var(idx) => Term::Var(*idx),
            Term::Universe(l) => Term::Universe(*l),
            Term::Pi { domain, codomain } => Term::Pi {
                domain: Arc::new(domain.subst_dim(target, replacement)),
                // Pi binds a term var — dim target unchanged
                codomain: Arc::new(codomain.subst_dim(target, replacement)),
            },
            Term::Lambda { body } => Term::Lambda {
                // Lambda binds a term var — dim target unchanged
                body: Arc::new(body.subst_dim(target, replacement)),
            },
            Term::App { func, arg } => Term::App {
                func: Arc::new(func.subst_dim(target, replacement)),
                arg: Arc::new(arg.subst_dim(target, replacement)),
            },
            Term::Sigma { fst_type, snd_type } => Term::Sigma {
                fst_type: Arc::new(fst_type.subst_dim(target, replacement)),
                // Sigma binds a term var — dim target unchanged
                snd_type: Arc::new(snd_type.subst_dim(target, replacement)),
            },
            Term::Pair { fst, snd } => Term::Pair {
                fst: Arc::new(fst.subst_dim(target, replacement)),
                snd: Arc::new(snd.subst_dim(target, replacement)),
            },
            Term::Fst(p) => Term::Fst(Arc::new(p.subst_dim(target, replacement))),
            Term::Snd(p) => Term::Snd(Arc::new(p.subst_dim(target, replacement))),
            // PathType: the `line` binds a dim var (it's a family over I)
            Term::PathType { line, left, right } => {
                let shifted_repl = replacement.shift_dim(0, 1);
                Term::PathType {
                    line: Arc::new(line.subst_dim(DimIndex(target.0 + 1), &shifted_repl)),
                    left: Arc::new(left.subst_dim(target, replacement)),
                    right: Arc::new(right.subst_dim(target, replacement)),
                }
            }
            // PathLam binds a dim var
            Term::PathLam { body } => {
                let shifted_repl = replacement.shift_dim(0, 1);
                Term::PathLam {
                    body: Arc::new(body.subst_dim(DimIndex(target.0 + 1), &shifted_repl)),
                }
            }
            Term::PathApp { path, dim } => Term::PathApp {
                path: Arc::new(path.subst_dim(target, replacement)),
                dim: dim.subst_dim(target, replacement),
            },
            // Coe line binds a dim var
            Term::Coe { from, to, line, body } => {
                let shifted_repl = replacement.shift_dim(0, 1);
                Term::Coe {
                    from: from.subst_dim(target, replacement),
                    to: to.subst_dim(target, replacement),
                    line: Arc::new(line.subst_dim(DimIndex(target.0 + 1), &shifted_repl)),
                    body: Arc::new(body.subst_dim(target, replacement)),
                }
            }
            // HCom branches each bind a dim var
            Term::HCom { from, to, ty, branches, base } => {
                let shifted_repl = replacement.shift_dim(0, 1);
                Term::HCom {
                    from: from.subst_dim(target, replacement),
                    to: to.subst_dim(target, replacement),
                    ty: Arc::new(ty.subst_dim(target, replacement)),
                    branches: branches.iter().map(|br| BdryBranch {
                        cof: br.cof.subst_dim(target, replacement),
                        body: Arc::new(br.body.subst_dim(
                            DimIndex(target.0 + 1),
                            &shifted_repl,
                        )),
                    }).collect(),
                    base: Arc::new(base.subst_dim(target, replacement)),
                }
            }
            Term::GlueType { base, cof, fiber_equiv } => Term::GlueType {
                base: Arc::new(base.subst_dim(target, replacement)),
                cof: cof.subst_dim(target, replacement),
                fiber_equiv: Arc::new(fiber_equiv.subst_dim(target, replacement)),
            },
            Term::GlueElem { base, cof, fiber_elem } => Term::GlueElem {
                base: Arc::new(base.subst_dim(target, replacement)),
                cof: cof.subst_dim(target, replacement),
                fiber_elem: Arc::new(fiber_elem.subst_dim(target, replacement)),
            },
            Term::Unglue { body, cof, fiber_equiv } => Term::Unglue {
                body: Arc::new(body.subst_dim(target, replacement)),
                cof: cof.subst_dim(target, replacement),
                fiber_equiv: Arc::new(fiber_equiv.subst_dim(target, replacement)),
            },
            Term::SmoothPath { order, ty, start, end } => Term::SmoothPath {
                order: *order,
                ty: Arc::new(ty.subst_dim(target, replacement)),
                start: Arc::new(start.subst_dim(target, replacement)),
                end: Arc::new(end.subst_dim(target, replacement)),
            },
            Term::Tangent(t) => Term::Tangent(Arc::new(t.subst_dim(target, replacement))),
            Term::Diff(t) => Term::Diff(Arc::new(t.subst_dim(target, replacement))),
            Term::Nat => Term::Nat,
            Term::Zero => Term::Zero,
            Term::Succ(n) => Term::Succ(Arc::new(n.subst_dim(target, replacement))),
            Term::NatElim { motive, base, step, scrutinee } => Term::NatElim {
                motive: Arc::new(motive.subst_dim(target, replacement)),
                base: Arc::new(base.subst_dim(target, replacement)),
                step: Arc::new(step.subst_dim(target, replacement)),
                scrutinee: Arc::new(scrutinee.subst_dim(target, replacement)),
            },
            // ARC grid types: no dimension variables — pass through
            Term::ColorType => Term::ColorType,
            Term::Color(c) => Term::Color(*c),
            Term::GridLit { rows, cols, data } => Term::GridLit {
                rows: *rows, cols: *cols, data: data.clone(),
            },
        }
    }
}
