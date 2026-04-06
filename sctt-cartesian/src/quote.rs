//! # Quote / Readback: Value → Term
//!
//! The second pass of NbE.  `quote` eta-expands canonical values and
//! converts de Bruijn *levels* back into de Bruijn *indices*.
//!
//! Invariant: all values produced by `evaluate` are closed w.r.t. levels
//! 0..term_lvl and dim levels 0..dim_lvl.

use std::sync::Arc;
use crate::syntax::{Term, Index};
use crate::dim::{DimLevel};
use crate::value::{BdryVal, CofVal, DimVal, Neutral, TermLevel, Value};
use crate::evaluate::{apply_closure, apply_dim_closure};

// ─── Fresh Variable Generators ───────────────────────────────────────────────

/// Create a fresh neutral term variable at the given level.
/// We use `Value::Nat` as a placeholder type — quote does not need the real type.
fn fresh_term_var(lvl: usize) -> Arc<Value> {
    Arc::new(Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(lvl))),
        Arc::new(Value::Nat),
    ))
}

/// Create a fresh dimension value at the given level.
fn fresh_dim_var(lvl: usize) -> DimVal {
    DimVal::Var(DimLevel(lvl))
}

// ─── Level → Index Conversion ────────────────────────────────────────────────

/// Convert a term level to a de Bruijn index given the current depth.
fn level_to_index(term_lvl: usize, l: TermLevel) -> Index {
    Index(term_lvl - 1 - l.0)
}

// ─── DimVal Quoting ──────────────────────────────────────────────────────────

fn quote_dim(dim_lvl: usize, dv: &DimVal) -> crate::dim::Dim {
    use crate::dim::{Dim, DimIndex};
    match dv {
        DimVal::Zero => Dim::Zero,
        DimVal::One => Dim::One,
        DimVal::Var(DimLevel(l)) => Dim::Var(DimIndex(dim_lvl - 1 - l)),
    }
}

// ─── CofVal Quoting ──────────────────────────────────────────────────────────

fn quote_cof(dim_lvl: usize, cv: &CofVal) -> crate::cof::Cof {
    use crate::cof::Cof;
    match cv {
        CofVal::Top => Cof::Top,
        CofVal::Bot => Cof::Bot,
        CofVal::Eq(r, s) => {
            Cof::eq_dim(quote_dim(dim_lvl, r), quote_dim(dim_lvl, s))
        }
        CofVal::And(phi, psi) => {
            Cof::and(quote_cof(dim_lvl, phi), quote_cof(dim_lvl, psi))
        }
        CofVal::Or(phi, psi) => {
            Cof::or(quote_cof(dim_lvl, phi), quote_cof(dim_lvl, psi))
        }
    }
}

// ─── Main Quote ──────────────────────────────────────────────────────────────

/// Quote a value back to a term.
///
/// - `term_lvl`: number of term variables currently in scope (depth).
/// - `dim_lvl`: number of dimension variables currently in scope.
pub fn quote(term_lvl: usize, dim_lvl: usize, val: &Value) -> Term {
    match val {
        // ── MLTT ──────────────────────────────────────────────────────────────
        Value::Universe(l) => Term::Universe(*l),
        Value::Nat => Term::Nat,
        Value::Zero => Term::Zero,
        Value::Succ(n) => Term::succ(quote(term_lvl, dim_lvl, n)),

        // ── Pi: eta-expand as lambda ──────────────────────────────────────────
        Value::Pi(dom, cod_cl) => {
            let dom_t = quote(term_lvl, dim_lvl, dom);
            let fresh = fresh_term_var(term_lvl);
            let cod_v = apply_closure(cod_cl, fresh);
            let cod_t = quote(term_lvl + 1, dim_lvl, &cod_v);
            Term::pi(dom_t, cod_t)
        }

        // ── Lambda ───────────────────────────────────────────────────────────
        Value::Lambda(cl) => {
            let fresh = fresh_term_var(term_lvl);
            let body_v = apply_closure(cl, fresh);
            let body_t = quote(term_lvl + 1, dim_lvl, &body_v);
            Term::lambda(body_t)
        }

        // ── Sigma ─────────────────────────────────────────────────────────────
        Value::Sigma(fst_ty, snd_cl) => {
            let fst_t = quote(term_lvl, dim_lvl, fst_ty);
            let fresh = fresh_term_var(term_lvl);
            let snd_v = apply_closure(snd_cl, fresh);
            let snd_t = quote(term_lvl + 1, dim_lvl, &snd_v);
            Term::sigma(fst_t, snd_t)
        }

        // ── Pair ──────────────────────────────────────────────────────────────
        Value::Pair(a, b) => {
            Term::pair(quote(term_lvl, dim_lvl, a), quote(term_lvl, dim_lvl, b))
        }

        // ── Path Type ─────────────────────────────────────────────────────────
        Value::PathType(line_cl, lv, rv) => {
            let fresh_d = fresh_dim_var(dim_lvl);
            let line_v = apply_dim_closure(line_cl, fresh_d);
            let line_t = quote(term_lvl, dim_lvl + 1, &line_v);
            let lv_t = quote(term_lvl, dim_lvl, lv);
            let rv_t = quote(term_lvl, dim_lvl, rv);
            Term::path_type(line_t, lv_t, rv_t)
        }

        // ── Path Lambda ───────────────────────────────────────────────────────
        Value::PathLam(cl) => {
            let fresh_d = fresh_dim_var(dim_lvl);
            let body_v = apply_dim_closure(cl, fresh_d);
            let body_t = quote(term_lvl, dim_lvl + 1, &body_v);
            Term::path_lam(body_t)
        }

        // ── Stuck Coe ────────────────────────────────────────────────────────
        Value::Coe(from, to, line_cl, body) => {
            let fresh_d = fresh_dim_var(dim_lvl);
            let line_v = apply_dim_closure(line_cl, fresh_d);
            let line_t = quote(term_lvl, dim_lvl + 1, &line_v);
            Term::coe(
                quote_dim(dim_lvl, from),
                quote_dim(dim_lvl, to),
                line_t,
                quote(term_lvl, dim_lvl, body),
            )
        }

        // ── Stuck HCom ───────────────────────────────────────────────────────
        Value::HCom(from, to, ty, bdry, base) => {
            let from_d = quote_dim(dim_lvl, from);
            let to_d = quote_dim(dim_lvl, to);
            let ty_t = quote(term_lvl, dim_lvl, ty);
            let base_t = quote(term_lvl, dim_lvl, base);
            let branches = quote_bdry(term_lvl, dim_lvl, bdry);
            Term::hcom(from_d, to_d, ty_t, branches, base_t)
        }

        // ── Glue ─────────────────────────────────────────────────────────────
        Value::GlueType(base, cof, fiber) => {
            Term::GlueType {
                base: Arc::new(quote(term_lvl, dim_lvl, base)),
                cof: quote_cof(dim_lvl, cof),
                fiber_equiv: Arc::new(quote(term_lvl, dim_lvl, fiber)),
            }
        }
        Value::GlueElem(base, cof, elem) => {
            Term::GlueElem {
                base: Arc::new(quote(term_lvl, dim_lvl, base)),
                cof: quote_cof(dim_lvl, cof),
                fiber_elem: Arc::new(quote(term_lvl, dim_lvl, elem)),
            }
        }

        // ── Smooth ───────────────────────────────────────────────────────────
        Value::SmoothPathType(order, ty, start, end) => {
            Term::SmoothPath {
                order: *order,
                ty: Arc::new(quote(term_lvl, dim_lvl, ty)),
                start: Arc::new(quote(term_lvl, dim_lvl, start)),
                end: Arc::new(quote(term_lvl, dim_lvl, end)),
            }
        }
        Value::Tangent(t) => Term::tangent(quote(term_lvl, dim_lvl, t)),

        // ── Neutral ──────────────────────────────────────────────────────────
        Value::Neutral(ne, _ty) => quote_neutral(term_lvl, dim_lvl, ne),
    }
}

/// Quote a neutral term back to syntax.
fn quote_neutral(term_lvl: usize, dim_lvl: usize, ne: &Neutral) -> Term {
    match ne {
        Neutral::Var(TermLevel(l)) => Term::Var(level_to_index(term_lvl, TermLevel(*l))),

        Neutral::App(ne_inner, arg, _result_ty) => {
            let func_t = quote_neutral(term_lvl, dim_lvl, ne_inner);
            let arg_t = quote(term_lvl, dim_lvl, arg);
            Term::app(func_t, arg_t)
        }

        Neutral::Fst(ne_inner, _ty) => {
            Term::fst(quote_neutral(term_lvl, dim_lvl, ne_inner))
        }

        Neutral::Snd(ne_inner, _ty) => {
            Term::snd(quote_neutral(term_lvl, dim_lvl, ne_inner))
        }

        Neutral::PathApp(ne_inner, dv, _ty) => {
            Term::PathApp {
                path: Arc::new(quote_neutral(term_lvl, dim_lvl, ne_inner)),
                dim: quote_dim(dim_lvl, dv),
            }
        }

        Neutral::NatElim(motive, base, step, ne_inner) => {
            Term::nat_elim(
                quote(term_lvl, dim_lvl, motive),
                quote(term_lvl, dim_lvl, base),
                quote(term_lvl, dim_lvl, step),
                quote_neutral(term_lvl, dim_lvl, ne_inner),
            )
        }

        Neutral::Unglue(ne_inner, cof, fiber_equiv) => {
            Term::Unglue {
                body: Arc::new(quote_neutral(term_lvl, dim_lvl, ne_inner)),
                cof: quote_cof(dim_lvl, cof),
                fiber_equiv: Arc::new(quote(term_lvl, dim_lvl, fiber_equiv)),
            }
        }

        Neutral::Diff(ne_inner, _ty) => {
            Term::diff(quote_neutral(term_lvl, dim_lvl, ne_inner))
        }
    }
}

// ─── Boundary Quoting ────────────────────────────────────────────────────────

fn quote_bdry(term_lvl: usize, dim_lvl: usize, bdry: &BdryVal) -> Vec<crate::syntax::BdryBranch> {
    use crate::syntax::BdryBranch;
    bdry.branches
        .iter()
        .map(|(cv, cl)| {
            let cof = quote_cof(dim_lvl, cv);
            let fresh_d = fresh_dim_var(dim_lvl);
            let body_v = apply_dim_closure(cl, fresh_d);
            let body_t = quote(term_lvl, dim_lvl + 1, &body_v);
            BdryBranch { cof, body: Arc::new(body_t) }
        })
        .collect()
}

// ─── Normalize ───────────────────────────────────────────────────────────────

/// Normalize a closed term: evaluate then quote.
pub fn normalize(term: &Term) -> Term {
    use crate::value::Env;
    use crate::evaluate::evaluate;
    let val = evaluate(&Env::new(), term);
    quote(0, 0, &val)
}
