/// Bidirectional Type Checking for SCTT
/// 
/// Implements Algorithm W-style bidirectional type checking with:
/// - Synthesis mode (infer type from term)
/// - Checking mode (check term against type)
/// - Normalization by evaluation for equality

use crate::syntax::ast::{Term, Context, Var, Name, Dim, Face, System};
use crate::semantics::domain::{Value, Env, eval, do_app, do_path_app};
use crate::nbe::normalize::{readback, equal_values};
use std::result::Result;

/// Type checking errors
#[derive(Debug, Clone)]
pub enum TypeError {
    UnboundVariable(Var),
    TypeMismatch { expected: Value, got: Value },
    NotAFunction(Value),
    NotAPath(Value),
    NotAPair(Value),
    InvalidApplication,
    InvalidProjection,
    CannotInfer(String),
    FaceInconsistent(Face),
    SystemIncomplete,
    UniverseInconsistency,
    SmoothnessMismatch { expected: f64, got: f64 },
}

pub type Result<T> = std::result::Result<T, TypeError>;

/// Type checking state
pub struct Checker {
    /// Current context
    pub ctx: Context,
    /// Evaluation environment
    pub env: Env,
    /// Universe level checking
    pub max_level: usize,
}

impl Checker {
    pub fn new() -> Self {
        Checker {
            ctx: Context::new(),
            env: Env::new(),
            max_level: 0,
        }
    }
    
    /// Infer the type of a term (synthesis mode)
    pub fn infer(&mut self, term: &Term) -> Result<Value> {
        match term {
            Term::Var(var) => {
                self.ctx.lookup(var)
                    .map(|ty| eval(ty, &self.env))
                    .ok_or(TypeError::UnboundVariable(*var))
            }
            
            Term::Type => Ok(Value::VType),
            
            Term::SmoothType => Ok(Value::VType),
            
            Term::Pi(x, a, b) => {
                self.check(a, &Value::VType)?;
                let va = eval(a, &self.env);
                
                let fresh = self.env.fresh_var();
                self.ctx.bind(x.clone(), a.clone());
                self.env = self.env.bind(fresh.clone());
                
                self.check(b, &Value::VType)?;
                
                Ok(Value::VType)
            }
            
            Term::Sigma(x, a, b) => {
                self.check(a, &Value::VType)?;
                let va = eval(a, &self.env);
                
                let fresh = self.env.fresh_var();
                self.ctx.bind(x.clone(), a.clone());
                self.env = self.env.bind(fresh.clone());
                
                self.check(b, &Value::VType)?;
                
                Ok(Value::VType)
            }
            
            Term::App(fun, arg) => {
                let fun_ty = self.infer(fun)?;
                match fun_ty {
                    Value::VPi(a, b_closure) => {
                        self.check(arg, &a)?;
                        let varg = eval(arg, &self.env);
                        let env = b_closure.env.bind(varg);
                        Ok(eval(&b_closure.body, &env))
                    }
                    _ => Err(TypeError::NotAFunction(fun_ty))
                }
            }
            
            Term::Fst(pair) => {
                let pair_ty = self.infer(pair)?;
                match pair_ty {
                    Value::VSigma(a, _) => Ok(*a),
                    _ => Err(TypeError::NotAPair(pair_ty))
                }
            }
            
            Term::Snd(pair) => {
                let pair_ty = self.infer(pair)?;
                match pair_ty {
                    Value::VSigma(_, b_closure) => {
                        let vpair = eval(pair, &self.env);
                        let vfst = crate::semantics::domain::do_fst(vpair);
                        let env = b_closure.env.bind(vfst);
                        Ok(eval(&b_closure.body, &env))
                    }
                    _ => Err(TypeError::NotAPair(pair_ty))
                }
            }
            
            Term::Path(ty, src, dst) => {
                // Check that ty is a type family over dimensions
                let fresh_dim = crate::semantics::domain::DimValue::DVar(0);
                let env_with_dim = self.env.bind_dim(fresh_dim);
                
                let ty_at_0 = eval(&ty.subst_dim(
                    &crate::syntax::ast::DimVar::Index(0),
                    &Dim::D0
                ), &self.env);
                let ty_at_1 = eval(&ty.subst_dim(
                    &crate::syntax::ast::DimVar::Index(0),
                    &Dim::D1
                ), &self.env);
                
                self.check(src, &ty_at_0)?;
                self.check(dst, &ty_at_1)?;
                
                Ok(Value::VType)
            }
            
            Term::PathApp(path, dim) => {
                let path_ty = self.infer(path)?;
                match path_ty {
                    Value::VPath(ty_closure, _, _) => {
                        let vdim = crate::semantics::domain::eval_dim(dim, &self.env);
                        let env = ty_closure.env.bind_dim(vdim);
                        Ok(eval(&ty_closure.body, &env))
                    }
                    _ => Err(TypeError::NotAPath(path_ty))
                }
            }
            
            Term::S1 => Ok(Value::VType),
            
            Term::Base => Ok(Value::VS1),
            
            Term::Loop(dim) => {
                // Loop is a path from base to base
                Ok(Value::VPath(
                    crate::semantics::domain::DimClosure {
                        env: self.env.clone(),
                        body: Term::S1,
                    },
                    Box::new(Value::VBase),
                    Box::new(Value::VBase),
                ))
            }
            
            Term::Smooth(smooth_op) => {
                self.infer_smooth(smooth_op)
            }
            
            Term::Let(x, ty, def, body) => {
                self.check(ty, &Value::VType)?;
                let vty = eval(ty, &self.env);
                self.check(def, &vty)?;
                let vdef = eval(def, &self.env);
                
                self.ctx.bind(x.clone(), ty.clone());
                self.env = self.env.bind(vdef);
                
                self.infer(body)
            }
            
            Term::Hole(name) => {
                Err(TypeError::CannotInfer(
                    name.clone().unwrap_or_else(|| "_".to_string())
                ))
            }
            
            _ => Err(TypeError::CannotInfer("Complex term".to_string()))
        }
    }
    
    /// Check that a term has a given type (checking mode)
    pub fn check(&mut self, term: &Term, ty: &Value) -> Result<()> {
        match (term, ty) {
            // Lambda against Pi type
            (Term::Lam(x, body), Value::VPi(a, b_closure)) => {
                let fresh = self.env.fresh_var();
                self.ctx.bind(x.clone(), readback(a, &Value::VType, &self.env));
                self.env = self.env.bind(fresh.clone());
                
                let b = {
                    let env = b_closure.env.bind(fresh);
                    eval(&b_closure.body, &env)
                };
                
                self.check(body, &b)
            }
            
            // Pair against Sigma type
            (Term::Pair(fst, snd), Value::VSigma(a, b_closure)) => {
                self.check(fst, a)?;
                let vfst = eval(fst, &self.env);
                
                let b = {
                    let env = b_closure.env.bind(vfst);
                    eval(&b_closure.body, &env)
                };
                
                self.check(snd, &b)
            }
            
            // Path lambda against Path type
            (Term::PathLam(dim_var, body), Value::VPath(ty_closure, src, dst)) => {
                // Check endpoints
                let body_at_0 = body.subst_dim(dim_var, &Dim::D0);
                let body_at_1 = body.subst_dim(dim_var, &Dim::D1);
                
                let ty_at_0 = {
                    let env = ty_closure.env.bind_dim(crate::semantics::domain::DimValue::D0);
                    eval(&ty_closure.body, &env)
                };
                let ty_at_1 = {
                    let env = ty_closure.env.bind_dim(crate::semantics::domain::DimValue::D1);
                    eval(&ty_closure.body, &env)
                };
                
                // Check that endpoints match
                self.check(&body_at_0, &ty_at_0)?;
                self.check(&body_at_1, &ty_at_1)?;
                
                let v0 = eval(&body_at_0, &self.env);
                let v1 = eval(&body_at_1, &self.env);
                
                if !equal_values(&v0, src, &self.env) {
                    return Err(TypeError::TypeMismatch {
                        expected: src.clone(),
                        got: v0,
                    });
                }
                
                if !equal_values(&v1, dst, &self.env) {
                    return Err(TypeError::TypeMismatch {
                        expected: dst.clone(),
                        got: v1,
                    });
                }
                
                Ok(())
            }
            
            // Fall back to inference
            _ => {
                let inferred = self.infer(term)?;
                if equal_values(&inferred, ty, &self.env) {
                    Ok(())
                } else {
                    Err(TypeError::TypeMismatch {
                        expected: ty.clone(),
                        got: inferred,
                    })
                }
            }
        }
    }
    
    /// Infer type of smooth operations
    fn infer_smooth(&mut self, op: &crate::syntax::ast::SmoothOp) -> Result<Value> {
        use crate::syntax::ast::SmoothOp;
        
        match op {
            SmoothOp::SmoothPath { start, end, smoothness } => {
                let start_ty = self.infer(start)?;
                let end_ty = self.infer(end)?;
                
                if !equal_values(&start_ty, &end_ty, &self.env) {
                    return Err(TypeError::TypeMismatch {
                        expected: start_ty,
                        got: end_ty,
                    });
                }
                
                Ok(Value::VSmooth(crate::semantics::domain::SmoothValue::VSmoothPath {
                    start: Box::new(eval(start, &self.env)),
                    end: Box::new(eval(end, &self.env)),
                    smoothness: *smoothness,
                    path: crate::semantics::domain::DimClosure {
                        env: self.env.clone(),
                        body: Term::Type, // Placeholder
                    },
                }))
            }
            
            SmoothOp::Diff(term) => {
                let ty = self.infer(term)?;
                // Differential has same type as original
                Ok(ty)
            }
            
            _ => Err(TypeError::CannotInfer("Smooth operation".to_string()))
        }
    }
    
    /// Check that a system is well-formed
    pub fn check_system(&mut self, system: &System<Term>, ty: &Value) -> Result<()> {
        for (face, term) in &system.branches {
            // Check face consistency
            if !self.check_face(face) {
                return Err(TypeError::FaceInconsistent(face.clone()));
            }
            
            // Check term under face assumption
            self.ctx.assume(face.clone());
            self.check(term, ty)?;
        }
        
        Ok(())
    }
    
    /// Check if a face formula is consistent
    fn check_face(&self, face: &Face) -> bool {
        match face {
            Face::True => true,
            Face::False => false,
            Face::Eq(d1, d2) => {
                // Check dimension equality is possible
                true // Simplified
            }
            Face::And(f1, f2) => self.check_face(f1) && self.check_face(f2),
            Face::Or(f1, f2) => self.check_face(f1) || self.check_face(f2),
        }
    }
}