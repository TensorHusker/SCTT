//! # Bidirectional Type Checker
//!
//! Implements bidirectional type checking for SCTT:
//! - **Inference** (`infer`): synthesizes a type from a term.
//! - **Checking** (`check`): verifies a term has an expected type.
//!
//! The checker maintains a context of term variable types (indexed by level)
//! and the current number of dimension variables in scope.  Terms are
//! evaluated against an environment of fresh neutrals so that NbE-based
//! conversion checking works correctly.

use std::sync::Arc;
use crate::conv::conv;
use crate::dim::DimLevel;
use crate::evaluate::{
    apply_closure, apply_dim_closure, do_app, do_fst,
    evaluate, evaluate_dim,
};
use crate::error::{Result, ScttError};
use crate::syntax::Term;
use crate::value::{DimVal, Env, Neutral, TermLevel, Value};

// ─── Type Checker State ─────────────────────────────────────────────────────

/// Bidirectional type checker for SCTT.
pub struct TypeChecker {
    /// Types of term variables, indexed by level (0 = outermost).
    ctx: Vec<Arc<Value>>,
    /// Number of dimension variables currently in scope.
    dim_lvl: usize,
}

impl TypeChecker {
    /// Create a new type checker with an empty context.
    pub fn new() -> Self {
        TypeChecker {
            ctx: Vec::new(),
            dim_lvl: 0,
        }
    }

    /// Number of term variables in scope.
    pub fn term_lvl(&self) -> usize {
        self.ctx.len()
    }

    /// Push a type onto the context (introduces a new term variable).
    pub fn extend_ctx(&mut self, ty: Arc<Value>) {
        self.ctx.push(ty);
    }

    /// Pop the most recently introduced term variable.
    pub fn pop_ctx(&mut self) {
        self.ctx.pop();
    }

    /// Look up a variable by de Bruijn index, returning its type.
    pub fn lookup_var(&self, idx: crate::syntax::Index) -> Result<Arc<Value>> {
        let depth = self.ctx.len();
        if idx.0 >= depth {
            return Err(ScttError::UnboundVar(idx.0));
        }
        let level = depth - 1 - idx.0;
        Ok(Arc::clone(&self.ctx[level]))
    }

    // ─── Environment Construction ───────────────────────────────────────

    /// Build an evaluation environment from the current context.
    ///
    /// Each term variable is mapped to a fresh neutral (at its level)
    /// with the type from the context.  Dimension variables are mapped
    /// to fresh dim vars.
    fn make_env(&self) -> Env {
        let mut env = Env::new();
        for (i, ty) in self.ctx.iter().enumerate() {
            env = env.extend_term(Arc::new(Value::Neutral(
                Arc::new(Neutral::Var(TermLevel(i))),
                Arc::clone(ty),
            )));
        }
        for d in 0..self.dim_lvl {
            env = env.extend_dim(DimVal::Var(DimLevel(d)));
        }
        env
    }

    /// Evaluate a term in the current context's environment.
    fn eval_term(&self, term: &Term) -> Arc<Value> {
        let env = self.make_env();
        evaluate(&env, term)
    }

    // ─── Inference ──────────────────────────────────────────────────────

    /// Synthesize the type of `term`.
    pub fn infer(&mut self, term: &Term) -> Result<Arc<Value>> {
        match term {
            // ── Variables ────────────────────────────────────────────────
            Term::Var(idx) => self.lookup_var(*idx),

            // ── Universe ─────────────────────────────────────────────────
            Term::Universe(l) => {
                let next = l.succ().ok_or(ScttError::UniverseOverflow(l.value()))?;
                Ok(Arc::new(Value::Universe(next)))
            }

            // ── Nat ──────────────────────────────────────────────────────
            Term::Nat => Ok(Arc::new(Value::Universe(crate::syntax::Level::zero()))),

            // ── Zero ─────────────────────────────────────────────────────
            Term::Zero => Ok(Arc::new(Value::Nat)),

            // ── Succ ─────────────────────────────────────────────────────
            Term::Succ(n) => {
                self.check(n, &Value::Nat)?;
                Ok(Arc::new(Value::Nat))
            }

            // ── Pi ───────────────────────────────────────────────────────
            Term::Pi { domain, codomain } => {
                let l1 = self.infer_universe(domain)?;
                let dom_val = self.eval_term(domain);
                self.extend_ctx(dom_val);
                let l2 = self.infer_universe(codomain)?;
                self.pop_ctx();
                Ok(Arc::new(Value::Universe(l1.max(l2))))
            }

            // ── App ──────────────────────────────────────────────────────
            Term::App { func, arg } => {
                let func_ty = self.infer(func)?;
                match &*func_ty {
                    Value::Pi(dom, cod_cl) => {
                        self.check(arg, dom)?;
                        let arg_val = self.eval_term(arg);
                        Ok(apply_closure(cod_cl, arg_val))
                    }
                    _ => Err(ScttError::TypeMismatch {
                        expected: "Pi type".to_string(),
                        actual: format!("{:?}", func_ty),
                    }),
                }
            }

            // ── Sigma ────────────────────────────────────────────────────
            Term::Sigma { fst_type, snd_type } => {
                let l1 = self.infer_universe(fst_type)?;
                let fst_val = self.eval_term(fst_type);
                self.extend_ctx(fst_val);
                let l2 = self.infer_universe(snd_type)?;
                self.pop_ctx();
                Ok(Arc::new(Value::Universe(l1.max(l2))))
            }

            // ── Fst ──────────────────────────────────────────────────────
            Term::Fst(p) => {
                let p_ty = self.infer(p)?;
                match &*p_ty {
                    Value::Sigma(fst_ty, _) => Ok(Arc::clone(fst_ty)),
                    _ => Err(ScttError::TypeMismatch {
                        expected: "Sigma type".to_string(),
                        actual: format!("{:?}", p_ty),
                    }),
                }
            }

            // ── Snd ──────────────────────────────────────────────────────
            Term::Snd(p) => {
                let p_ty = self.infer(p)?;
                match &*p_ty {
                    Value::Sigma(_, snd_cl) => {
                        let p_val = self.eval_term(p);
                        let fst_val = do_fst(p_val);
                        Ok(apply_closure(snd_cl, fst_val))
                    }
                    _ => Err(ScttError::TypeMismatch {
                        expected: "Sigma type".to_string(),
                        actual: format!("{:?}", p_ty),
                    }),
                }
            }

            // ── PathType ─────────────────────────────────────────────────
            Term::PathType { line, left, right } => {
                // The line binds a dim variable; check it yields a Universe.
                let l = self.infer_universe_under_dim(line)?;
                // Check endpoints against the line at 0/1.
                let env = self.make_env();
                let env0 = env.extend_dim(DimVal::Zero);
                let line_at_0 = evaluate(&env0, line);
                self.check(left, &line_at_0)?;
                let env1 = env.extend_dim(DimVal::One);
                let line_at_1 = evaluate(&env1, line);
                self.check(right, &line_at_1)?;
                Ok(Arc::new(Value::Universe(l)))
            }

            // ── PathApp ──────────────────────────────────────────────────
            Term::PathApp { path, dim } => {
                let path_ty = self.infer(path)?;
                match &*path_ty {
                    Value::PathType(line_cl, _, _) => {
                        let env = self.make_env();
                        let dim_val = evaluate_dim(&env, dim);
                        Ok(apply_dim_closure(line_cl, dim_val))
                    }
                    _ => Err(ScttError::TypeMismatch {
                        expected: "Path type".to_string(),
                        actual: format!("{:?}", path_ty),
                    }),
                }
            }

            // ── NatElim ──────────────────────────────────────────────────
            Term::NatElim { motive, base, step, scrutinee } => {
                // motive : Nat -> Type
                self.check(scrutinee, &Value::Nat)?;
                let motive_ty = self.infer(motive)?;
                // Check motive has type Nat -> Universe(_)
                match &*motive_ty {
                    Value::Pi(dom, _) => {
                        if !conv(self.term_lvl(), self.dim_lvl, dom, &Value::Nat) {
                            return Err(ScttError::TypeMismatch {
                                expected: "Nat".to_string(),
                                actual: format!("{:?}", dom),
                            });
                        }
                    }
                    _ => {
                        return Err(ScttError::TypeMismatch {
                            expected: "function type for motive".to_string(),
                            actual: format!("{:?}", motive_ty),
                        });
                    }
                }
                let motive_val = self.eval_term(motive);
                // base : motive Zero
                let base_ty = do_app(Arc::clone(&motive_val), Arc::new(Value::Zero));
                self.check(base, &base_ty)?;
                // step : (n : Nat) -> motive n -> motive (S n)
                // Check outer Pi has Nat domain; full dependent codomain
                // check deferred until we have proper elaboration.
                let step_ty = self.infer(step)?;
                match &*step_ty {
                    Value::Pi(dom, _) => {
                        if !conv(self.term_lvl(), self.dim_lvl, dom, &Value::Nat) {
                            return Err(ScttError::TypeMismatch {
                                expected: "Pi(Nat, ...)".to_string(),
                                actual: format!("{:?}", step_ty),
                            });
                        }
                    }
                    _ => {
                        return Err(ScttError::TypeMismatch {
                            expected: "function type for step".to_string(),
                            actual: format!("{:?}", step_ty),
                        });
                    }
                }

                let scrut_val = self.eval_term(scrutinee);
                Ok(do_app(motive_val, scrut_val))
            }

            // ── Introductions require checking mode ──────────────────────
            Term::Lambda { .. } => Err(ScttError::TypeMismatch {
                expected: "cannot infer lambda; use check mode".to_string(),
                actual: "Lambda".to_string(),
            }),
            Term::Pair { .. } => Err(ScttError::TypeMismatch {
                expected: "cannot infer pair; use check mode".to_string(),
                actual: "Pair".to_string(),
            }),
            Term::PathLam { .. } => Err(ScttError::TypeMismatch {
                expected: "cannot infer path lambda; use check mode".to_string(),
                actual: "PathLam".to_string(),
            }),

            // ── Not yet implemented ──────────────────────────────────────
            _ => Err(ScttError::NotImplemented(format!(
                "infer not implemented for {:?}",
                term
            ))),
        }
    }

    // ─── Checking ───────────────────────────────────────────────────────

    /// Check that `term` has type `expected`.
    pub fn check(&mut self, term: &Term, expected: &Value) -> Result<()> {
        match (term, expected) {
            // ── Lambda against Pi ────────────────────────────────────────
            (Term::Lambda { body }, Value::Pi(dom, cod_cl)) => {
                let fresh = Arc::new(Value::Neutral(
                    Arc::new(Neutral::Var(TermLevel(self.term_lvl()))),
                    Arc::clone(dom),
                ));
                let cod_val = apply_closure(cod_cl, fresh);
                self.extend_ctx(Arc::clone(dom));
                let result = self.check(body, &cod_val);
                self.pop_ctx();
                result
            }

            // ── Pair against Sigma ───────────────────────────────────────
            (Term::Pair { fst, snd }, Value::Sigma(fst_ty, snd_cl)) => {
                self.check(fst, fst_ty)?;
                let fst_val = self.eval_term(fst);
                let snd_ty = apply_closure(snd_cl, fst_val);
                self.check(snd, &snd_ty)
            }

            // ── PathLam against PathType ─────────────────────────────────
            (Term::PathLam { body }, Value::PathType(line_cl, left, right)) => {
                // Open under a fresh dimension variable.
                let fresh_d = DimVal::Var(DimLevel(self.dim_lvl));
                let line_val = apply_dim_closure(line_cl, fresh_d);
                self.dim_lvl += 1;
                let result = self.check(body, &line_val);
                self.dim_lvl -= 1;
                result?;

                // Verify endpoint at 0: body[i:=0] must convert with left.
                let env = self.make_env();
                let env0 = env.extend_dim(DimVal::Zero);
                let body_at_0 = evaluate(&env0, body);
                if !conv(self.term_lvl(), self.dim_lvl, &body_at_0, left) {
                    return Err(ScttError::PathEndpointMismatch {
                        endpoint: "left (i=0)",
                        expected: format!("{:?}", left),
                        actual: format!("{:?}", body_at_0),
                    });
                }

                // Verify endpoint at 1: body[i:=1] must convert with right.
                let env1 = env.extend_dim(DimVal::One);
                let body_at_1 = evaluate(&env1, body);
                if !conv(self.term_lvl(), self.dim_lvl, &body_at_1, right) {
                    return Err(ScttError::PathEndpointMismatch {
                        endpoint: "right (i=1)",
                        expected: format!("{:?}", right),
                        actual: format!("{:?}", body_at_1),
                    });
                }

                Ok(())
            }

            // ── Fallback: infer + convert ────────────────────────────────
            _ => {
                let inferred = self.infer(term)?;
                if conv(self.term_lvl(), self.dim_lvl, &inferred, expected) {
                    Ok(())
                } else {
                    Err(ScttError::TypeMismatch {
                        expected: format!("{:?}", expected),
                        actual: format!("{:?}", inferred),
                    })
                }
            }
        }
    }

    // ─── Helpers ────────────────────────────────────────────────────────

    /// Infer the type of `term` and verify it is a Universe, returning the level.
    fn infer_universe(&mut self, term: &Term) -> Result<crate::syntax::Level> {
        let ty = self.infer(term)?;
        match &*ty {
            Value::Universe(l) => Ok(*l),
            _ => Err(ScttError::TypeMismatch {
                expected: "Universe".to_string(),
                actual: format!("{:?}", ty),
            }),
        }
    }

    /// Infer the universe level of a term that binds one dimension variable.
    /// Used for PathType lines.
    fn infer_universe_under_dim(&mut self, term: &Term) -> Result<crate::syntax::Level> {
        self.dim_lvl += 1;
        let result = self.infer_universe(term);
        self.dim_lvl -= 1;
        result
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
