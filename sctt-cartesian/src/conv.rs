//! # Conversion Checking (Definitional Equality)
//!
//! Two values are convertible when they denote the same element up to
//! beta-eta equivalence.  We compare values in their evaluated (semantic)
//! form, so beta-rules have already fired; the main work here is:
//!
//! - **Eta for functions**: `f ≡ λx. f(x)`
//! - **Eta for pairs**: `p ≡ (fst p, snd p)`
//! - **Eta for paths**: `<i> p @ i ≡ p`
//! - **Structural comparison** of canonical forms and neutral spines.
//!
//! The `term_lvl` / `dim_lvl` counters track the number of fresh variables
//! generated so far, ensuring each closure is opened with a globally unique
//! variable.

use std::sync::Arc;
use crate::dim::DimLevel;
use crate::value::{BdryVal, CofVal, Closure, DimClosure, DimVal, Neutral, TermLevel, Value};
use crate::evaluate::{apply_closure, apply_dim_closure, do_app, do_path_app};

// ─── Fresh Variable Helpers ─────────────────────────────────────────────────

/// Create a fresh neutral term variable at the given level.
/// Nat is used as a dummy type — conversion does not inspect types.
fn fresh_term_var(lvl: usize) -> Arc<Value> {
    Arc::new(Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(lvl))),
        Arc::new(Value::Nat),
    ))
}

/// Create a fresh dimension variable at the given level.
fn fresh_dim_var(lvl: usize) -> DimVal {
    DimVal::Var(DimLevel(lvl))
}

// ─── Main Entry Point ───────────────────────────────────────────────────────

/// Check whether two values are definitionally equal (convertible).
///
/// `term_lvl`: number of term variables in scope.
/// `dim_lvl`: number of dimension variables in scope.
pub fn conv(term_lvl: usize, dim_lvl: usize, a: &Value, b: &Value) -> bool {
    match (a, b) {
        // ── Universes ────────────────────────────────────────────────────
        (Value::Universe(l1), Value::Universe(l2)) => l1 == l2,

        // ── Pi ───────────────────────────────────────────────────────────
        (Value::Pi(a1, b1), Value::Pi(a2, b2)) => {
            conv(term_lvl, dim_lvl, a1, a2)
                && conv_closure(term_lvl, dim_lvl, b1, b2)
        }

        // ── Sigma ────────────────────────────────────────────────────────
        (Value::Sigma(a1, b1), Value::Sigma(a2, b2)) => {
            conv(term_lvl, dim_lvl, a1, a2)
                && conv_closure(term_lvl, dim_lvl, b1, b2)
        }

        // ── Lambda-Lambda ────────────────────────────────────────────────
        (Value::Lambda(cl1), Value::Lambda(cl2)) => {
            conv_closure(term_lvl, dim_lvl, cl1, cl2)
        }

        // ── Lambda-Other (eta for functions) ─────────────────────────────
        (Value::Lambda(_), _) | (_, Value::Lambda(_)) => {
            let fresh = fresh_term_var(term_lvl);
            let lhs = do_app(Arc::new(a.clone()), Arc::clone(&fresh));
            let rhs = do_app(Arc::new(b.clone()), fresh);
            conv(term_lvl + 1, dim_lvl, &lhs, &rhs)
        }

        // ── Pair-Pair ──────────────────────────────────────────────────
        (Value::Pair(a1, b1), Value::Pair(a2, b2)) => {
            conv(term_lvl, dim_lvl, a1, a2)
                && conv(term_lvl, dim_lvl, b1, b2)
        }

        // ── Pair-Other (eta for pairs: p ≡ (fst p, snd p)) ───────────
        (Value::Pair(_, _), _) | (_, Value::Pair(_, _)) => {
            let a_arc = Arc::new(a.clone());
            let b_arc = Arc::new(b.clone());
            let fst_a = crate::evaluate::do_fst(Arc::clone(&a_arc));
            let fst_b = crate::evaluate::do_fst(Arc::clone(&b_arc));
            let snd_a = crate::evaluate::do_snd(a_arc);
            let snd_b = crate::evaluate::do_snd(b_arc);
            conv(term_lvl, dim_lvl, &fst_a, &fst_b)
                && conv(term_lvl, dim_lvl, &snd_a, &snd_b)
        }

        // ── PathType ─────────────────────────────────────────────────────
        (Value::PathType(l1, lv1, rv1), Value::PathType(l2, lv2, rv2)) => {
            conv_dim_closure(term_lvl, dim_lvl, l1, l2)
                && conv(term_lvl, dim_lvl, lv1, lv2)
                && conv(term_lvl, dim_lvl, rv1, rv2)
        }

        // ── PathLam-PathLam ──────────────────────────────────────────────
        (Value::PathLam(cl1), Value::PathLam(cl2)) => {
            conv_dim_closure(term_lvl, dim_lvl, cl1, cl2)
        }

        // ── PathLam-Other (eta for paths) ────────────────────────────────
        (Value::PathLam(_), _) | (_, Value::PathLam(_)) => {
            let fresh_d = fresh_dim_var(dim_lvl);
            let lhs = do_path_app(Arc::new(a.clone()), fresh_d.clone());
            let rhs = do_path_app(Arc::new(b.clone()), fresh_d);
            conv(term_lvl, dim_lvl + 1, &lhs, &rhs)
        }

        // ── Nat ──────────────────────────────────────────────────────────
        (Value::Nat, Value::Nat) => true,
        (Value::Zero, Value::Zero) => true,
        (Value::Succ(n1), Value::Succ(n2)) => conv(term_lvl, dim_lvl, n1, n2),

        // ── Neutral ──────────────────────────────────────────────────────
        (Value::Neutral(ne1, _), Value::Neutral(ne2, _)) => {
            conv_neutral(term_lvl, dim_lvl, ne1, ne2)
        }

        // ── Coe (stuck) ──────────────────────────────────────────────────
        (Value::Coe(f1, t1, l1, b1), Value::Coe(f2, t2, l2, b2)) => {
            conv_dim_val(f1, f2)
                && conv_dim_val(t1, t2)
                && conv_dim_closure(term_lvl, dim_lvl, l1, l2)
                && conv(term_lvl, dim_lvl, b1, b2)
        }

        // ── HCom (stuck) ─────────────────────────────────────────────────
        (Value::HCom(f1, t1, ty1, bdry1, b1), Value::HCom(f2, t2, ty2, bdry2, b2)) => {
            conv_dim_val(f1, f2)
                && conv_dim_val(t1, t2)
                && conv(term_lvl, dim_lvl, ty1, ty2)
                && conv(term_lvl, dim_lvl, b1, b2)
                && conv_bdry(term_lvl, dim_lvl, bdry1, bdry2)
        }

        // ── Glue ─────────────────────────────────────────────────────────
        (Value::GlueType(b1, c1, f1), Value::GlueType(b2, c2, f2)) => {
            conv(term_lvl, dim_lvl, b1, b2)
                && c1 == c2
                && conv(term_lvl, dim_lvl, f1, f2)
        }
        (Value::GlueElem(b1, c1, f1), Value::GlueElem(b2, c2, f2)) => {
            conv(term_lvl, dim_lvl, b1, b2)
                && c1 == c2
                && conv(term_lvl, dim_lvl, f1, f2)
        }

        // ── Smooth ───────────────────────────────────────────────────────
        (Value::SmoothPathType(o1, ty1, s1, e1), Value::SmoothPathType(o2, ty2, s2, e2)) => {
            o1 == o2
                && conv(term_lvl, dim_lvl, ty1, ty2)
                && conv(term_lvl, dim_lvl, s1, s2)
                && conv(term_lvl, dim_lvl, e1, e2)
        }
        (Value::Tangent(t1), Value::Tangent(t2)) => {
            conv(term_lvl, dim_lvl, t1, t2)
        }

        // ── Mismatch ─────────────────────────────────────────────────────
        _ => false,
    }
}

// ─── Closure Comparison ─────────────────────────────────────────────────────

/// Compare two term closures by opening them with the same fresh variable.
fn conv_closure(term_lvl: usize, dim_lvl: usize, cl1: &Closure, cl2: &Closure) -> bool {
    let fresh = fresh_term_var(term_lvl);
    let v1 = apply_closure(cl1, Arc::clone(&fresh));
    let v2 = apply_closure(cl2, fresh);
    conv(term_lvl + 1, dim_lvl, &v1, &v2)
}

/// Compare two dimension closures by opening them with the same fresh dim variable.
fn conv_dim_closure(term_lvl: usize, dim_lvl: usize, cl1: &DimClosure, cl2: &DimClosure) -> bool {
    let fresh_d = fresh_dim_var(dim_lvl);
    let v1 = apply_dim_closure(cl1, fresh_d.clone());
    let v2 = apply_dim_closure(cl2, fresh_d);
    conv(term_lvl, dim_lvl + 1, &v1, &v2)
}

// ─── Dimension Value Comparison ─────────────────────────────────────────────

/// Structural equality of dimension values.
fn conv_dim_val(a: &DimVal, b: &DimVal) -> bool {
    a == b
}

// ─── Neutral Comparison ─────────────────────────────────────────────────────

/// Compare two neutral terms by matching their head variable and spine.
fn conv_neutral(term_lvl: usize, dim_lvl: usize, ne1: &Neutral, ne2: &Neutral) -> bool {
    match (ne1, ne2) {
        (Neutral::Var(l1), Neutral::Var(l2)) => l1 == l2,

        (Neutral::App(n1, a1, _), Neutral::App(n2, a2, _)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
                && conv(term_lvl, dim_lvl, a1, a2)
        }

        (Neutral::Fst(n1, _), Neutral::Fst(n2, _)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
        }

        (Neutral::Snd(n1, _), Neutral::Snd(n2, _)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
        }

        (Neutral::PathApp(n1, d1, _), Neutral::PathApp(n2, d2, _)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
                && conv_dim_val(d1, d2)
        }

        (Neutral::NatElim(m1, b1, s1, n1), Neutral::NatElim(m2, b2, s2, n2)) => {
            conv(term_lvl, dim_lvl, m1, m2)
                && conv(term_lvl, dim_lvl, b1, b2)
                && conv(term_lvl, dim_lvl, s1, s2)
                && conv_neutral(term_lvl, dim_lvl, n1, n2)
        }

        (Neutral::Unglue(n1, c1, f1), Neutral::Unglue(n2, c2, f2)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
                && c1 == c2
                && conv(term_lvl, dim_lvl, f1, f2)
        }

        (Neutral::Diff(n1, _), Neutral::Diff(n2, _)) => {
            conv_neutral(term_lvl, dim_lvl, n1, n2)
        }

        _ => false,
    }
}

// ─── Boundary Comparison ────────────────────────────────────────────────────

/// Compare two boundary systems branch-by-branch.
fn conv_bdry(term_lvl: usize, dim_lvl: usize, b1: &BdryVal, b2: &BdryVal) -> bool {
    if b1.branches.len() != b2.branches.len() {
        return false;
    }
    b1.branches.iter().zip(b2.branches.iter()).all(|((c1, cl1), (c2, cl2))| {
        conv_cof_val(c1, c2) && conv_dim_closure(term_lvl, dim_lvl, cl1, cl2)
    })
}

/// Structural equality of cofibration values.
fn conv_cof_val(a: &CofVal, b: &CofVal) -> bool {
    a == b
}
