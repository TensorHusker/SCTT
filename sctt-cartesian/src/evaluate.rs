//! # Evaluation: Term → Value
//!
//! The first pass of Normalization by Evaluation (NbE).
//! All β-rules fire eagerly; stuck terms become `Value::Neutral`.
//!
//! ## Circular-dependency note
//! `Closure` and `DimClosure` need to *call* evaluate in their application.
//! Rather than putting `apply` methods on those types (which would require
//! evaluate.rs to be in scope from value.rs), we define free functions here:
//! `apply_closure` and `apply_dim_closure`.  The rest of the codebase should
//! call these functions instead of methods.

use std::sync::Arc;
use crate::syntax::{Term, BdryBranch};
use crate::dim::{Dim, DimIndex, DimLevel};
use crate::cof::Cof;
use crate::value::{
    BdryVal, Closure, CofVal, DimClosure, DimVal, Env, Neutral, TermLevel, Value,
};

// ─── Closure Application ─────────────────────────────────────────────────────

/// Apply a term closure to a value argument (β-rule for Lambda).
pub fn apply_closure(cl: &Closure, arg: Arc<Value>) -> Arc<Value> {
    let env = cl.env.extend_term(arg);
    evaluate(&env, &cl.body)
}

/// Apply a dimension closure to a dimension value.
pub fn apply_dim_closure(cl: &DimClosure, dim: DimVal) -> Arc<Value> {
    let env = cl.env.extend_dim(dim);
    evaluate(&env, &cl.body)
}

// ─── Dimension and Cofibration Evaluation ────────────────────────────────────

/// Evaluate a syntactic `Dim` to a semantic `DimVal`.
pub fn evaluate_dim(env: &Env, dim: &Dim) -> DimVal {
    match dim {
        Dim::Zero => DimVal::Zero,
        Dim::One => DimVal::One,
        Dim::Var(DimIndex(i)) => {
            // de Bruijn index: 0 = innermost binder.
            // env.dims is a stack: dims[depth - 1] = index 0.
            let depth = env.dim_depth();
            let level = depth - 1 - i;
            env.lookup_dim(DimLevel(level))
        }
    }
}

/// Evaluate a syntactic `Cof` to a semantic `CofVal`.
pub fn evaluate_cof(env: &Env, cof: &Cof) -> CofVal {
    match cof {
        Cof::Eq(r, s) => CofVal::eq(evaluate_dim(env, r), evaluate_dim(env, s)),
        Cof::And(phi, psi) => CofVal::and(evaluate_cof(env, phi), evaluate_cof(env, psi)),
        Cof::Or(phi, psi) => CofVal::or(evaluate_cof(env, phi), evaluate_cof(env, psi)),
        Cof::Top => CofVal::Top,
        Cof::Bot => CofVal::Bot,
    }
}

// ─── Eliminators ─────────────────────────────────────────────────────────────

/// β-reduce a function application, or build a neutral.
pub fn do_app(func: Arc<Value>, arg: Arc<Value>) -> Arc<Value> {
    match &*func {
        Value::Lambda(cl) => apply_closure(cl, arg),
        Value::Neutral(ne, ty) => {
            // Type of the application: instantiate the codomain closure
            let result_ty = match &**ty {
                Value::Pi(_, cod_cl) => apply_closure(cod_cl, Arc::clone(&arg)),
                _ => Arc::new(Value::Neutral(
                    Arc::new(Neutral::App(Arc::clone(ne), Arc::clone(&arg), Arc::clone(ty))),
                    Arc::clone(ty),
                )),
            };
            Arc::new(Value::Neutral(
                Arc::new(Neutral::App(Arc::clone(ne), Arc::clone(&arg), Arc::clone(&result_ty))),
                result_ty,
            ))
        }
        _ => panic!("do_app: not a function: {:?}", func),
    }
}

/// β-reduce `fst`, or build a neutral.
pub fn do_fst(val: Arc<Value>) -> Arc<Value> {
    match &*val {
        Value::Pair(a, _b) => Arc::clone(a),
        Value::Neutral(ne, ty) => {
            let fst_ty = match &**ty {
                Value::Sigma(fst_ty, _) => Arc::clone(fst_ty),
                other => Arc::new(other.clone()),
            };
            Arc::new(Value::Neutral(
                Arc::new(Neutral::Fst(Arc::clone(ne), Arc::clone(&fst_ty))),
                fst_ty,
            ))
        }
        _ => panic!("do_fst: not a pair: {:?}", val),
    }
}

/// β-reduce `snd`, or build a neutral.
pub fn do_snd(val: Arc<Value>) -> Arc<Value> {
    match &*val {
        Value::Pair(_a, b) => Arc::clone(b),
        Value::Neutral(ne, ty) => {
            // snd type: apply snd_type closure to fst
            let snd_ty = match &**ty {
                Value::Sigma(_, snd_cl) => {
                    let fst_val = do_fst(Arc::clone(&val));
                    apply_closure(snd_cl, fst_val)
                }
                other => Arc::new(other.clone()),
            };
            Arc::new(Value::Neutral(
                Arc::new(Neutral::Snd(Arc::clone(ne), Arc::clone(&snd_ty))),
                snd_ty,
            ))
        }
        _ => panic!("do_snd: not a pair: {:?}", val),
    }
}

/// β-reduce a path application, or build a neutral.
pub fn do_path_app(path: Arc<Value>, dim: DimVal) -> Arc<Value> {
    match &*path {
        Value::PathLam(cl) => apply_dim_closure(cl, dim),
        Value::Neutral(ne, ty) => {
            let result_ty = match &**ty {
                Value::PathType(line_cl, _, _) => apply_dim_closure(line_cl, dim.clone()),
                other => Arc::new(other.clone()),
            };
            Arc::new(Value::Neutral(
                Arc::new(Neutral::PathApp(Arc::clone(ne), dim, Arc::clone(&result_ty))),
                result_ty,
            ))
        }
        _ => panic!("do_path_app: not a path: {:?}", path),
    }
}

/// Evaluate `NatElim P z s n` recursively.
pub fn do_nat_elim(
    motive: Arc<Value>,
    base: Arc<Value>,
    step: Arc<Value>,
    scrutinee: Arc<Value>,
) -> Arc<Value> {
    match &*scrutinee {
        Value::Zero => Arc::clone(&base),
        Value::Succ(n) => {
            let rec = do_nat_elim(Arc::clone(&motive), Arc::clone(&base), Arc::clone(&step), Arc::clone(n));
            let after_n = do_app(Arc::clone(&step), Arc::clone(n));
            do_app(after_n, rec)
        }
        Value::Neutral(ne, _) => {
            // Compute the type: motive applied to scrutinee
            let result_ty = do_app(Arc::clone(&motive), Arc::clone(&scrutinee));
            Arc::new(Value::Neutral(
                Arc::new(Neutral::NatElim(
                    Arc::clone(&motive),
                    Arc::clone(&base),
                    Arc::clone(&step),
                    Arc::clone(ne),
                )),
                result_ty,
            ))
        }
        _ => panic!("do_nat_elim: not a nat: {:?}", scrutinee),
    }
}

/// Coerce along a type family: if `from == to`, return body unchanged; else stuck.
pub fn do_coe(from: DimVal, to: DimVal, line_cl: DimClosure, body: Arc<Value>) -> Arc<Value> {
    if from == to {
        body
    } else {
        Arc::new(Value::Coe(from, to, line_cl, body))
    }
}

/// Homogeneous composition: if `from == to`, return base; else stuck.
pub fn do_hcom(
    from: DimVal,
    to: DimVal,
    ty: Arc<Value>,
    bdry: BdryVal,
    base: Arc<Value>,
) -> Arc<Value> {
    if from == to {
        base
    } else {
        Arc::new(Value::HCom(from, to, ty, bdry, base))
    }
}

// ─── Main Evaluation ─────────────────────────────────────────────────────────

/// Evaluate a term in an environment, producing a semantic value.
pub fn evaluate(env: &Env, term: &Term) -> Arc<Value> {
    match term {
        // ── Variables ────────────────────────────────────────────────────────
        Term::Var(idx) => {
            // de Bruijn index → level: level = depth - 1 - index
            let depth = env.term_depth();
            let level = depth - 1 - idx.0;
            env.lookup_term(TermLevel(level))
        }

        // ── Universes ────────────────────────────────────────────────────────
        Term::Universe(l) => Arc::new(Value::Universe(*l)),

        // ── Pi ───────────────────────────────────────────────────────────────
        Term::Pi { domain, codomain } => {
            let dom_val = evaluate(env, domain);
            let cod_cl = Closure { env: env.clone(), body: Arc::clone(codomain) };
            Arc::new(Value::Pi(dom_val, cod_cl))
        }

        // ── Lambda ───────────────────────────────────────────────────────────
        Term::Lambda { body } => {
            let cl = Closure { env: env.clone(), body: Arc::clone(body) };
            Arc::new(Value::Lambda(cl))
        }

        // ── Application ──────────────────────────────────────────────────────
        Term::App { func, arg } => {
            let func_val = evaluate(env, func);
            let arg_val = evaluate(env, arg);
            do_app(func_val, arg_val)
        }

        // ── Sigma ────────────────────────────────────────────────────────────
        Term::Sigma { fst_type, snd_type } => {
            let fst_val = evaluate(env, fst_type);
            let snd_cl = Closure { env: env.clone(), body: Arc::clone(snd_type) };
            Arc::new(Value::Sigma(fst_val, snd_cl))
        }

        // ── Pair ─────────────────────────────────────────────────────────────
        Term::Pair { fst, snd } => {
            let a = evaluate(env, fst);
            let b = evaluate(env, snd);
            Arc::new(Value::Pair(a, b))
        }

        // ── Projections ──────────────────────────────────────────────────────
        Term::Fst(p) => {
            let pv = evaluate(env, p);
            do_fst(pv)
        }
        Term::Snd(p) => {
            let pv = evaluate(env, p);
            do_snd(pv)
        }

        // ── Path Types ───────────────────────────────────────────────────────
        Term::PathType { line, left, right } => {
            let line_cl = DimClosure { env: env.clone(), body: Arc::clone(line) };
            let lv = evaluate(env, left);
            let rv = evaluate(env, right);
            Arc::new(Value::PathType(line_cl, lv, rv))
        }

        // ── Path Lambda ───────────────────────────────────────────────────────
        Term::PathLam { body } => {
            let cl = DimClosure { env: env.clone(), body: Arc::clone(body) };
            Arc::new(Value::PathLam(cl))
        }

        // ── Path Application ──────────────────────────────────────────────────
        Term::PathApp { path, dim } => {
            let pv = evaluate(env, path);
            let dv = evaluate_dim(env, dim);
            do_path_app(pv, dv)
        }

        // ── Coercion ─────────────────────────────────────────────────────────
        Term::Coe { from, to, line, body } => {
            let from_v = evaluate_dim(env, from);
            let to_v = evaluate_dim(env, to);
            let line_cl = DimClosure { env: env.clone(), body: Arc::clone(line) };
            let body_v = evaluate(env, body);
            do_coe(from_v, to_v, line_cl, body_v)
        }

        // ── HCom ─────────────────────────────────────────────────────────────
        Term::HCom { from, to, ty, branches, base } => {
            let from_v = evaluate_dim(env, from);
            let to_v = evaluate_dim(env, to);
            let ty_v = evaluate(env, ty);
            let bdry = evaluate_bdry(env, branches);
            let base_v = evaluate(env, base);
            do_hcom(from_v, to_v, ty_v, bdry, base_v)
        }

        // ── Glue ─────────────────────────────────────────────────────────────
        Term::GlueType { base, cof, fiber_equiv } => {
            let bv = evaluate(env, base);
            let cv = evaluate_cof(env, cof);
            let fv = evaluate(env, fiber_equiv);
            Arc::new(Value::GlueType(bv, cv, fv))
        }
        Term::GlueElem { base, cof, fiber_elem } => {
            let bv = evaluate(env, base);
            let cv = evaluate_cof(env, cof);
            let fv = evaluate(env, fiber_elem);
            Arc::new(Value::GlueElem(bv, cv, fv))
        }
        Term::Unglue { body, cof, fiber_equiv } => {
            let bv = evaluate(env, body);
            let cv = evaluate_cof(env, cof);
            let fv = evaluate(env, fiber_equiv);
            match &*bv {
                Value::GlueElem(inner, _, _) => Arc::clone(inner),
                Value::Neutral(ne, _) => {
                    Arc::new(Value::Neutral(
                        Arc::new(Neutral::Unglue(Arc::clone(ne), cv, fv)),
                        // type is the base type — we just use the glue base
                        Arc::new(Value::Nat), // placeholder: proper typing needs the checker
                    ))
                }
                _ => Arc::clone(&bv),
            }
        }

        // ── Smooth ───────────────────────────────────────────────────────────
        Term::SmoothPath { order, ty, start, end } => {
            let tv = evaluate(env, ty);
            let sv = evaluate(env, start);
            let ev = evaluate(env, end);
            Arc::new(Value::SmoothPathType(*order, tv, sv, ev))
        }
        Term::Tangent(t) => {
            let tv = evaluate(env, t);
            Arc::new(Value::Tangent(tv))
        }
        Term::Diff(f) => {
            let fv = evaluate(env, f);
            match &*fv {
                Value::Neutral(ne, ty) => Arc::new(Value::Neutral(
                    Arc::new(Neutral::Diff(Arc::clone(ne), Arc::clone(ty))),
                    Arc::clone(ty),
                )),
                _ => fv, // Diff of a value: leave as-is (smooth ops stubbed)
            }
        }

        // ── Natural Numbers ───────────────────────────────────────────────────
        Term::Nat => Arc::new(Value::Nat),
        Term::Zero => Arc::new(Value::Zero),
        Term::Succ(n) => {
            let nv = evaluate(env, n);
            Arc::new(Value::Succ(nv))
        }
        Term::NatElim { motive, base, step, scrutinee } => {
            let mv = evaluate(env, motive);
            let bv = evaluate(env, base);
            let sv = evaluate(env, step);
            let scrv = evaluate(env, scrutinee);
            do_nat_elim(mv, bv, sv, scrv)
        }
    }
}

// ─── Boundary Helper ─────────────────────────────────────────────────────────

fn evaluate_bdry(env: &Env, branches: &[BdryBranch]) -> BdryVal {
    let branches = branches
        .iter()
        .map(|br| {
            let cv = evaluate_cof(env, &br.cof);
            let cl = DimClosure { env: env.clone(), body: Arc::clone(&br.body) };
            (cv, cl)
        })
        .collect();
    BdryVal { branches }
}
