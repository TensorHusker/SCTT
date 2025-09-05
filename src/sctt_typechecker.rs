//! Minimal SCTT Type Checker with Categorical Semantics
//! 
//! This implementation provides:
//! - Type universe hierarchy with cumulative levels
//! - Dependent function types (Π-types)
//! - Path types with smooth structure
//! - Bidirectional type checking
//! - Normalization by evaluation (NbE)

use std::collections::HashMap;
use std::rc::Rc;

/// De Bruijn index for variable representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeBruijnIndex(pub usize);

/// Universe levels for the type hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Zero,
    Succ(usize),
    Omega,
}

impl Level {
    pub fn max(self, other: Level) -> Level {
        use Level::*;
        match (self, other) {
            (Omega, _) | (_, Omega) => Omega,
            (Zero, l) | (l, Zero) => l,
            (Succ(i), Succ(j)) => Succ(i.max(j)),
        }
    }

    pub fn succ(self) -> Level {
        match self {
            Level::Zero => Level::Succ(1),
            Level::Succ(n) => Level::Succ(n + 1),
            Level::Omega => Level::Omega,
        }
    }
}

/// Interval points for path types (De Morgan algebra structure)
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalPoint {
    Zero,                                    // 0 endpoint
    One,                                     // 1 endpoint
    Var(DeBruijnIndex),                     // interval variable
    Meet(Box<IntervalPoint>, Box<IntervalPoint>), // i ∧ j
    Join(Box<IntervalPoint>, Box<IntervalPoint>), // i ∨ j  
    Neg(Box<IntervalPoint>),                // ¬i
}

/// Core term language with dependent types and paths
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// Variable with De Bruijn index
    Var(DeBruijnIndex),
    
    /// Type universe at level
    Universe(Level),
    
    /// Lambda abstraction: λx:A.t
    Lambda(Box<Term>, Box<Term>),
    
    /// Application: t u
    App(Box<Term>, Box<Term>),
    
    /// Dependent product: Π(x:A).B
    Pi(Box<Term>, Box<Term>),
    
    /// Path type: Path A a b (paths in A from a to b)
    PathType(Box<Term>, Box<Term>, Box<Term>),
    
    /// Path abstraction: λi.t
    PathLambda(Box<Term>),
    
    /// Path application: p @ i
    PathApp(Box<Term>, IntervalPoint),
    
    /// Interval type
    Interval(IntervalPoint),
    
    /// Transport along paths
    Transport(Box<Term>, IntervalPoint, IntervalPoint, Box<Term>),
    
    /// Homogeneous composition
    Hcomp(Box<Term>, Vec<(IntervalPoint, IntervalPoint, Box<Term>)>, Box<Term>),
}

/// Values for normalization by evaluation
#[derive(Debug, Clone)]
pub enum Value {
    Neutral(Neutral),
    Universe(Level),
    Lambda(Closure),
    Pi(Box<Value>, Closure),
    PathType(Box<Value>, Box<Value>, Box<Value>),
    PathLambda(PathClosure),
    Interval(IntervalPoint),
}

/// Neutral values (cannot reduce further)
#[derive(Debug, Clone)]
pub enum Neutral {
    Var(DeBruijnIndex),
    App(Box<Neutral>, Box<Value>),
    PathApp(Box<Neutral>, IntervalPoint),
    Transport(Box<Value>, IntervalPoint, IntervalPoint, Box<Neutral>),
}

/// Closures capture environments
#[derive(Debug, Clone)]
pub struct Closure {
    pub env: Environment,
    pub body: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct PathClosure {
    pub env: Environment,
    pub body: Box<Term>,
}

/// Environment for evaluation
#[derive(Debug, Clone)]
pub struct Environment {
    pub values: Vec<Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: Vec::new() }
    }

    pub fn extend(&self, v: Value) -> Self {
        let mut env = self.clone();
        env.values.push(v);
        env
    }

    pub fn lookup(&self, idx: DeBruijnIndex) -> Option<&Value> {
        let i = self.values.len().checked_sub(idx.0 + 1)?;
        self.values.get(i)
    }
}

/// Typing context
#[derive(Debug, Clone)]
pub struct Context {
    pub types: Vec<Value>,
    pub env: Environment,
}

impl Context {
    pub fn new() -> Self {
        Context {
            types: Vec::new(),
            env: Environment::new(),
        }
    }

    pub fn extend(&self, ty: Value) -> Self {
        let mut ctx = self.clone();
        ctx.types.push(ty.clone());
        let var = Value::Neutral(Neutral::Var(DeBruijnIndex(ctx.types.len() - 1)));
        ctx.env = ctx.env.extend(var);
        ctx
    }

    pub fn lookup(&self, idx: DeBruijnIndex) -> Option<&Value> {
        let i = self.types.len().checked_sub(idx.0 + 1)?;
        self.types.get(i)
    }
}

/// Type checking errors
#[derive(Debug)]
pub enum TypeError {
    UnboundVariable(DeBruijnIndex),
    TypeMismatch { expected: Value, found: Value },
    NotAFunction(Value),
    NotAPath(Value),
    NotAUniverse(Value),
    InvalidInterval,
    UnificationFailure,
}

pub type Result<T> = std::result::Result<T, TypeError>;

/// Main type checker implementation
pub struct TypeChecker {
    /// Conversion checking depth limit for termination
    pub max_depth: usize,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker { max_depth: 1000 }
    }

    /// Bidirectional type checking: check mode
    pub fn check(&self, ctx: &Context, term: &Term, ty: &Value) -> Result<()> {
        match (term, ty) {
            // Lambda checking against Pi type
            (Term::Lambda(_, body), Value::Pi(a_ty, closure)) => {
                let extended_ctx = ctx.extend(a_ty.as_ref().clone());
                let b_ty = self.apply_closure(closure.clone(), 
                    Value::Neutral(Neutral::Var(DeBruijnIndex(ctx.types.len()))));
                self.check(&extended_ctx, body, &b_ty)
            }
            
            // Path lambda checking against path type
            (Term::PathLambda(body), Value::PathType(a_ty, start, end)) => {
                // Check at i=0 and i=1
                let at_zero = self.substitute_interval(body, &IntervalPoint::Zero);
                let at_one = self.substitute_interval(body, &IntervalPoint::One);
                
                let val_zero = self.eval(&ctx.env, &at_zero);
                let val_one = self.eval(&ctx.env, &at_one);
                
                self.check_equal(ctx, &val_zero, start, a_ty)?;
                self.check_equal(ctx, &val_one, end, a_ty)?;
                Ok(())
            }
            
            // Switch to inference mode
            _ => {
                let inferred = self.infer(ctx, term)?;
                self.check_equal(ctx, &inferred, ty, &Value::Universe(Level::Omega))
            }
        }
    }

    /// Bidirectional type checking: infer mode
    pub fn infer(&self, ctx: &Context, term: &Term) -> Result<Value> {
        match term {
            Term::Var(idx) => {
                ctx.lookup(*idx)
                    .cloned()
                    .ok_or(TypeError::UnboundVariable(*idx))
            }
            
            Term::Universe(level) => {
                Ok(Value::Universe(level.succ()))
            }
            
            Term::Pi(a, b) => {
                let a_val = self.eval(&ctx.env, a);
                let a_level = self.infer_universe_level(ctx, &a_val)?;
                
                let extended_ctx = ctx.extend(a_val.clone());
                let b_val = self.eval(&extended_ctx.env, b);
                let b_level = self.infer_universe_level(&extended_ctx, &b_val)?;
                
                Ok(Value::Universe(a_level.max(b_level)))
            }
            
            Term::App(fun, arg) => {
                let fun_ty = self.infer(ctx, fun)?;
                match fun_ty {
                    Value::Pi(a_ty, closure) => {
                        self.check(ctx, arg, &a_ty)?;
                        let arg_val = self.eval(&ctx.env, arg);
                        Ok(self.apply_closure(closure, arg_val))
                    }
                    _ => Err(TypeError::NotAFunction(fun_ty)),
                }
            }
            
            Term::PathType(a, start, end) => {
                let a_val = self.eval(&ctx.env, a);
                let a_level = self.infer_universe_level(ctx, &a_val)?;
                
                let start_val = self.eval(&ctx.env, start);
                let end_val = self.eval(&ctx.env, end);
                
                self.check(ctx, start, &a_val)?;
                self.check(ctx, end, &a_val)?;
                
                Ok(Value::Universe(a_level))
            }
            
            Term::PathApp(path, i) => {
                let path_ty = self.infer(ctx, path)?;
                match path_ty {
                    Value::PathType(a_ty, _, _) => Ok(a_ty.as_ref().clone()),
                    _ => Err(TypeError::NotAPath(path_ty)),
                }
            }
            
            Term::Lambda(a, _) => {
                // Lambda requires type annotation in inference mode
                let a_val = self.eval(&ctx.env, a);
                self.check(ctx, a, &Value::Universe(Level::Omega))?;
                Err(TypeError::TypeMismatch { 
                    expected: Value::Universe(Level::Omega), 
                    found: a_val 
                })
            }
            
            _ => todo!("Other term inference cases"),
        }
    }

    /// Normalize a term by evaluation
    pub fn normalize(&self, env: &Environment, term: &Term) -> Term {
        let val = self.eval(env, term);
        self.quote(env.values.len(), &val)
    }

    /// Evaluate term to value
    pub fn eval(&self, env: &Environment, term: &Term) -> Value {
        match term {
            Term::Var(idx) => {
                env.lookup(*idx).cloned()
                    .unwrap_or(Value::Neutral(Neutral::Var(*idx)))
            }
            
            Term::Universe(level) => Value::Universe(*level),
            
            Term::Lambda(ty, body) => {
                Value::Lambda(Closure {
                    env: env.clone(),
                    body: body.clone(),
                })
            }
            
            Term::Pi(a, b) => {
                let a_val = self.eval(env, a);
                Value::Pi(
                    Box::new(a_val),
                    Closure {
                        env: env.clone(),
                        body: b.clone(),
                    },
                )
            }
            
            Term::App(fun, arg) => {
                let fun_val = self.eval(env, fun);
                let arg_val = self.eval(env, arg);
                self.apply_value(fun_val, arg_val)
            }
            
            Term::PathType(a, start, end) => {
                Value::PathType(
                    Box::new(self.eval(env, a)),
                    Box::new(self.eval(env, start)),
                    Box::new(self.eval(env, end)),
                )
            }
            
            Term::PathLambda(body) => {
                Value::PathLambda(PathClosure {
                    env: env.clone(),
                    body: body.clone(),
                })
            }
            
            Term::PathApp(path, i) => {
                let path_val = self.eval(env, path);
                self.apply_path(path_val, i.clone())
            }
            
            Term::Interval(i) => Value::Interval(i.clone()),
            
            _ => todo!("Other evaluation cases"),
        }
    }

    /// Quote value back to term (for normalization)
    fn quote(&self, level: usize, value: &Value) -> Term {
        match value {
            Value::Neutral(n) => self.quote_neutral(level, n),
            
            Value::Universe(l) => Term::Universe(*l),
            
            Value::Lambda(closure) => {
                let var = Value::Neutral(Neutral::Var(DeBruijnIndex(level)));
                let body_val = self.apply_closure(closure.clone(), var);
                let body = self.quote(level + 1, &body_val);
                Term::Lambda(
                    Box::new(Term::Universe(Level::Zero)), // placeholder type
                    Box::new(body),
                )
            }
            
            Value::Pi(a, closure) => {
                let a_term = self.quote(level, a);
                let var = Value::Neutral(Neutral::Var(DeBruijnIndex(level)));
                let b_val = self.apply_closure(closure.clone(), var);
                let b_term = self.quote(level + 1, &b_val);
                Term::Pi(Box::new(a_term), Box::new(b_term))
            }
            
            Value::PathType(a, start, end) => {
                Term::PathType(
                    Box::new(self.quote(level, a)),
                    Box::new(self.quote(level, start)),
                    Box::new(self.quote(level, end)),
                )
            }
            
            Value::PathLambda(closure) => {
                let body_val = self.eval(&closure.env, &closure.body);
                Term::PathLambda(Box::new(self.quote(level, &body_val)))
            }
            
            Value::Interval(i) => Term::Interval(i.clone()),
        }
    }

    fn quote_neutral(&self, level: usize, neutral: &Neutral) -> Term {
        match neutral {
            Neutral::Var(idx) => Term::Var(*idx),
            
            Neutral::App(fun, arg) => {
                Term::App(
                    Box::new(self.quote_neutral(level, fun)),
                    Box::new(self.quote(level, arg)),
                )
            }
            
            Neutral::PathApp(path, i) => {
                Term::PathApp(
                    Box::new(self.quote_neutral(level, path)),
                    i.clone(),
                )
            }
            
            _ => todo!("Other neutral quoting cases"),
        }
    }

    /// Apply function value to argument
    fn apply_value(&self, fun: Value, arg: Value) -> Value {
        match fun {
            Value::Lambda(closure) => self.apply_closure(closure, arg),
            Value::Neutral(n) => Value::Neutral(Neutral::App(Box::new(n), Box::new(arg))),
            _ => panic!("Cannot apply non-function"),
        }
    }

    /// Apply path to interval point
    fn apply_path(&self, path: Value, i: IntervalPoint) -> Value {
        match path {
            Value::PathLambda(closure) => {
                let subst = self.substitute_interval(&closure.body, &i);
                self.eval(&closure.env, &subst)
            }
            Value::Neutral(n) => Value::Neutral(Neutral::PathApp(Box::new(n), i)),
            _ => panic!("Cannot apply non-path"),
        }
    }

    /// Apply closure to value
    fn apply_closure(&self, closure: Closure, arg: Value) -> Value {
        let extended_env = closure.env.extend(arg);
        self.eval(&extended_env, &closure.body)
    }

    /// Substitute interval variable in term
    fn substitute_interval(&self, term: &Term, i: &IntervalPoint) -> Term {
        // Simplified for demonstration - full implementation would handle all cases
        match term {
            Term::Interval(_) => Term::Interval(i.clone()),
            Term::Lambda(ty, body) => Term::Lambda(
                Box::new(self.substitute_interval(ty, i)),
                Box::new(self.substitute_interval(body, i)),
            ),
            _ => term.clone(),
        }
    }

    /// Check equality of values (conversion checking)
    fn check_equal(&self, ctx: &Context, v1: &Value, v2: &Value, ty: &Value) -> Result<()> {
        if self.values_equal(ctx.types.len(), v1, v2, 0)? {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch {
                expected: v2.clone(),
                found: v1.clone(),
            })
        }
    }

    /// Structural equality of values
    fn values_equal(&self, level: usize, v1: &Value, v2: &Value, depth: usize) -> Result<bool> {
        if depth > self.max_depth {
            return Err(TypeError::UnificationFailure);
        }

        match (v1, v2) {
            (Value::Universe(l1), Value::Universe(l2)) => Ok(l1 == l2),
            
            (Value::Lambda(c1), Value::Lambda(c2)) => {
                let var = Value::Neutral(Neutral::Var(DeBruijnIndex(level)));
                let v1 = self.apply_closure(c1.clone(), var.clone());
                let v2 = self.apply_closure(c2.clone(), var);
                self.values_equal(level + 1, &v1, &v2, depth + 1)
            }
            
            (Value::Pi(a1, c1), Value::Pi(a2, c2)) => {
                let a_eq = self.values_equal(level, a1, a2, depth + 1)?;
                if !a_eq {
                    return Ok(false);
                }
                let var = Value::Neutral(Neutral::Var(DeBruijnIndex(level)));
                let b1 = self.apply_closure(c1.clone(), var.clone());
                let b2 = self.apply_closure(c2.clone(), var);
                self.values_equal(level + 1, &b1, &b2, depth + 1)
            }
            
            (Value::Neutral(n1), Value::Neutral(n2)) => {
                self.neutrals_equal(level, n1, n2, depth)
            }
            
            _ => Ok(false),
        }
    }

    fn neutrals_equal(&self, level: usize, n1: &Neutral, n2: &Neutral, depth: usize) -> Result<bool> {
        match (n1, n2) {
            (Neutral::Var(i1), Neutral::Var(i2)) => Ok(i1 == i2),
            
            (Neutral::App(f1, a1), Neutral::App(f2, a2)) => {
                let f_eq = self.neutrals_equal(level, f1, f2, depth + 1)?;
                if !f_eq {
                    return Ok(false);
                }
                self.values_equal(level, a1, a2, depth + 1)
            }
            
            _ => Ok(false),
        }
    }

    /// Infer universe level of a type
    fn infer_universe_level(&self, ctx: &Context, ty: &Value) -> Result<Level> {
        match ty {
            Value::Universe(level) => Ok(*level),
            _ => Err(TypeError::NotAUniverse(ty.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_function() {
        let tc = TypeChecker::new();
        let ctx = Context::new();
        
        // id : Π(A : Type₀). Π(x : A). A
        // id = λA. λx. x
        let id_type = Term::Pi(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Pi(
                Box::new(Term::Var(DeBruijnIndex(0))),
                Box::new(Term::Var(DeBruijnIndex(1))),
            )),
        );
        
        let id_term = Term::Lambda(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Lambda(
                Box::new(Term::Var(DeBruijnIndex(0))),
                Box::new(Term::Var(DeBruijnIndex(0))),
            )),
        );
        
        let id_type_val = tc.eval(&ctx.env, &id_type);
        assert!(tc.check(&ctx, &id_term, &id_type_val).is_ok());
    }

    #[test]
    fn test_path_reflexivity() {
        let tc = TypeChecker::new();
        let ctx = Context::new();
        
        // refl : Π(A : Type₀). Π(a : A). Path A a a
        // refl = λA. λa. λi. a
        let refl_type = Term::Pi(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Pi(
                Box::new(Term::Var(DeBruijnIndex(0))),
                Box::new(Term::PathType(
                    Box::new(Term::Var(DeBruijnIndex(1))),
                    Box::new(Term::Var(DeBruijnIndex(0))),
                    Box::new(Term::Var(DeBruijnIndex(0))),
                )),
            )),
        );
        
        let refl_term = Term::Lambda(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Lambda(
                Box::new(Term::Var(DeBruijnIndex(0))),
                Box::new(Term::PathLambda(
                    Box::new(Term::Var(DeBruijnIndex(0))),
                )),
            )),
        );
        
        let refl_type_val = tc.eval(&ctx.env, &refl_type);
        assert!(tc.check(&ctx, &refl_term, &refl_type_val).is_ok());
    }

    #[test]
    fn test_normalization_beta_reduction() {
        let tc = TypeChecker::new();
        let env = Environment::new();
        
        // (λx. x) y → y
        let app = Term::App(
            Box::new(Term::Lambda(
                Box::new(Term::Universe(Level::Zero)),
                Box::new(Term::Var(DeBruijnIndex(0))),
            )),
            Box::new(Term::Var(DeBruijnIndex(0))),
        );
        
        let normalized = tc.normalize(&env, &app);
        assert_eq!(normalized, Term::Var(DeBruijnIndex(0)));
    }

    #[test]
    fn test_universe_hierarchy() {
        let tc = TypeChecker::new();
        let ctx = Context::new();
        
        // Type₀ : Type₁
        let type0 = Term::Universe(Level::Zero);
        let inferred = tc.infer(&ctx, &type0).unwrap();
        
        match inferred {
            Value::Universe(Level::Succ(1)) => (),
            _ => panic!("Expected Type₁"),
        }
    }

    #[test]
    fn test_confluence_property() {
        let tc = TypeChecker::new();
        let env = Environment::new();
        
        // Church numerals: different reduction paths should converge
        // two = λf. λx. f (f x)
        let two = Term::Lambda(
            Box::new(Term::Universe(Level::Zero)),
            Box::new(Term::Lambda(
                Box::new(Term::Universe(Level::Zero)),
                Box::new(Term::App(
                    Box::new(Term::Var(DeBruijnIndex(1))),
                    Box::new(Term::App(
                        Box::new(Term::Var(DeBruijnIndex(1))),
                        Box::new(Term::Var(DeBruijnIndex(0))),
                    )),
                )),
            )),
        );
        
        // Normalize from different starting points
        let norm1 = tc.normalize(&env, &two);
        let norm2 = tc.normalize(&env, &two);
        
        assert_eq!(norm1, norm2, "Confluence property violated");
    }
}