/// Normalization by Evaluation for SCTT
/// 
/// Converts terms to normal form via:
/// 1. Evaluation to semantic values
/// 2. Readback to syntax

use crate::syntax::ast::{Term, Var, Name};
use crate::semantics::domain::{Value, Neutral, Env, Level, eval};

/// Read back a value to normal form syntax
pub fn readback(value: &Value, ty: &Value, env: &Env) -> Term {
    match (value, ty) {
        // Functions: eta-expand
        (_, Value::VPi(a, b_closure)) => {
            let fresh = env.fresh_var();
            let arg_term = readback(&fresh, a, env);
            
            let app = crate::semantics::domain::do_app(value.clone(), fresh.clone());
            let b = {
                let env = b_closure.env.bind(fresh);
                eval(&b_closure.body, &env)
            };
            
            let body = readback(&app, &b, &env.bind(fresh));
            Term::Lam(Name("x".to_string()), Box::new(body))
        }
        
        // Pairs: eta-expand
        (_, Value::VSigma(a, b_closure)) => {
            let fst = crate::semantics::domain::do_fst(value.clone());
            let snd = crate::semantics::domain::do_snd(value.clone());
            
            let fst_term = readback(&fst, a, env);
            let b = {
                let env = b_closure.env.bind(fst.clone());
                eval(&b_closure.body, &env)
            };
            let snd_term = readback(&snd, &b, env);
            
            Term::Pair(Box::new(fst_term), Box::new(snd_term))
        }
        
        // Paths: eta-expand
        (_, Value::VPath(ty_closure, src, dst)) => {
            let fresh_dim = crate::semantics::domain::DimValue::DVar(env.dims.len());
            let app = crate::semantics::domain::do_path_app(
                value.clone(),
                fresh_dim.clone()
            );
            
            let ty_at_dim = {
                let env = ty_closure.env.bind_dim(fresh_dim.clone());
                eval(&ty_closure.body, &env)
            };
            
            let body = readback(&app, &ty_at_dim, &env.bind_dim(fresh_dim));
            Term::PathLam(
                crate::syntax::ast::DimVar::Index(env.dims.len()),
                Box::new(body)
            )
        }
        
        // Neutral terms
        (Value::Neutral(n), _) => readback_neutral(n, env),
        
        // Type universes
        (Value::VType, Value::VType) => Term::Type,
        (Value::VSmoothType, Value::VType) => Term::SmoothType,
        
        // Type constructors
        (Value::VPi(a, b_closure), Value::VType) => {
            let a_term = readback(a, &Value::VType, env);
            let fresh = env.fresh_var();
            let b = {
                let env = b_closure.env.bind(fresh.clone());
                eval(&b_closure.body, &env)
            };
            let b_term = readback(&b, &Value::VType, &env.bind(fresh));
            Term::Pi(Name("x".to_string()), Box::new(a_term), Box::new(b_term))
        }
        
        (Value::VSigma(a, b_closure), Value::VType) => {
            let a_term = readback(a, &Value::VType, env);
            let fresh = env.fresh_var();
            let b = {
                let env = b_closure.env.bind(fresh.clone());
                eval(&b_closure.body, &env)
            };
            let b_term = readback(&b, &Value::VType, &env.bind(fresh));
            Term::Sigma(Name("x".to_string()), Box::new(a_term), Box::new(b_term))
        }
        
        (Value::VPath(ty_closure, src, dst), Value::VType) => {
            let fresh_dim = crate::semantics::domain::DimValue::DVar(env.dims.len());
            let ty_at_dim = {
                let env = ty_closure.env.bind_dim(fresh_dim.clone());
                eval(&ty_closure.body, &env)
            };
            let ty_term = readback(&ty_at_dim, &Value::VType, &env.bind_dim(fresh_dim));
            let src_term = readback(src, &ty_at_dim, env);
            let dst_term = readback(dst, &ty_at_dim, env);
            Term::Path(Box::new(ty_term), Box::new(src_term), Box::new(dst_term))
        }
        
        // Higher inductive types
        (Value::VS1, Value::VType) => Term::S1,
        (Value::VBase, Value::VS1) => Term::Base,
        
        _ => panic!("Cannot readback value of type {:?}", ty),
    }
}

/// Read back a neutral term
pub fn readback_neutral(neutral: &Neutral, env: &Env) -> Term {
    match neutral {
        Neutral::NVar(Level(l)) => {
            // Convert level to de Bruijn index
            let index = env.level.0 - l - 1;
            Term::Var(Var(index))
        }
        
        Neutral::NApp(fun, arg) => {
            let fun_term = readback_neutral(fun, env);
            let arg_term = readback(arg, &infer_type(arg, env), env);
            Term::App(Box::new(fun_term), Box::new(arg_term))
        }
        
        Neutral::NPathApp(path, dim) => {
            let path_term = readback_neutral(path, env);
            let dim_term = readback_dim(dim);
            Term::PathApp(Box::new(path_term), dim_term)
        }
        
        Neutral::NFst(pair) => {
            let pair_term = readback_neutral(pair, env);
            Term::Fst(Box::new(pair_term))
        }
        
        Neutral::NSnd(pair) => {
            let pair_term = readback_neutral(pair, env);
            Term::Snd(Box::new(pair_term))
        }
        
        _ => todo!("Readback remaining neutral constructors"),
    }
}

/// Read back dimension value to syntax
pub fn readback_dim(dim: &crate::semantics::domain::DimValue) -> crate::syntax::ast::Dim {
    use crate::semantics::domain::DimValue;
    use crate::syntax::ast::Dim;
    
    match dim {
        DimValue::D0 => Dim::D0,
        DimValue::D1 => Dim::D1,
        DimValue::DVar(i) => Dim::Var(crate::syntax::ast::DimVar::Index(*i)),
        DimValue::DMeet(d1, d2) => {
            Dim::Meet(Box::new(readback_dim(d1)), Box::new(readback_dim(d2)))
        }
        DimValue::DJoin(d1, d2) => {
            Dim::Join(Box::new(readback_dim(d1)), Box::new(readback_dim(d2)))
        }
        DimValue::DNeg(d) => {
            Dim::Neg(Box::new(readback_dim(d)))
        }
    }
}

/// Check equality of values
pub fn equal_values(v1: &Value, v2: &Value, env: &Env) -> bool {
    match (v1, v2) {
        (Value::VType, Value::VType) => true,
        (Value::VSmoothType, Value::VSmoothType) => true,
        
        (Value::VPi(a1, b1), Value::VPi(a2, b2)) => {
            if !equal_values(a1, a2, env) {
                return false;
            }
            
            let fresh = env.fresh_var();
            let b1_val = {
                let env = b1.env.bind(fresh.clone());
                eval(&b1.body, &env)
            };
            let b2_val = {
                let env = b2.env.bind(fresh.clone());
                eval(&b2.body, &env)
            };
            
            equal_values(&b1_val, &b2_val, &env.bind(fresh))
        }
        
        (Value::VSigma(a1, b1), Value::VSigma(a2, b2)) => {
            if !equal_values(a1, a2, env) {
                return false;
            }
            
            let fresh = env.fresh_var();
            let b1_val = {
                let env = b1.env.bind(fresh.clone());
                eval(&b1.body, &env)
            };
            let b2_val = {
                let env = b2.env.bind(fresh.clone());
                eval(&b2.body, &env)
            };
            
            equal_values(&b1_val, &b2_val, &env.bind(fresh))
        }
        
        (Value::VPath(ty1, src1, dst1), Value::VPath(ty2, src2, dst2)) => {
            // Check path types are equal
            let fresh_dim = crate::semantics::domain::DimValue::DVar(env.dims.len());
            let ty1_at_dim = {
                let env = ty1.env.bind_dim(fresh_dim.clone());
                eval(&ty1.body, &env)
            };
            let ty2_at_dim = {
                let env = ty2.env.bind_dim(fresh_dim.clone());
                eval(&ty2.body, &env)
            };
            
            equal_values(&ty1_at_dim, &ty2_at_dim, &env.bind_dim(fresh_dim)) &&
            equal_values(src1, src2, env) &&
            equal_values(dst1, dst2, env)
        }
        
        (Value::Neutral(n1), Value::Neutral(n2)) => equal_neutrals(n1, n2, env),
        
        (Value::VS1, Value::VS1) => true,
        (Value::VBase, Value::VBase) => true,
        
        _ => false,
    }
}

/// Check equality of neutral terms
pub fn equal_neutrals(n1: &Neutral, n2: &Neutral, env: &Env) -> bool {
    match (n1, n2) {
        (Neutral::NVar(l1), Neutral::NVar(l2)) => l1 == l2,
        
        (Neutral::NApp(f1, a1), Neutral::NApp(f2, a2)) => {
            equal_neutrals(f1, f2, env) && equal_values(a1, a2, env)
        }
        
        (Neutral::NFst(p1), Neutral::NFst(p2)) => equal_neutrals(p1, p2, env),
        (Neutral::NSnd(p1), Neutral::NSnd(p2)) => equal_neutrals(p1, p2, env),
        
        _ => false,
    }
}

/// Normalize a term by evaluating and reading back
pub fn normalize(term: &Term, ty: &Value, env: &Env) -> Term {
    let value = eval(term, env);
    readback(&value, ty, env)
}

/// Infer type of a value (for readback)
fn infer_type(value: &Value, env: &Env) -> Value {
    // Simplified type inference
    // In practice, we'd track types through evaluation
    Value::VType
}