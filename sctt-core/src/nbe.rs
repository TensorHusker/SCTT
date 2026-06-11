//! # Normalization by Evaluation for SCTT
//!
//! This module implements Normalization by Evaluation (NbE), a technique for
//! efficiently computing normal forms of terms. The key idea is to:
//! 1. Evaluate terms to semantic values in a rich domain
//! 2. Read back values to syntactic normal forms
//!
//! This approach is particularly effective for dependent type theory and
//! essential for conversion checking in SCTT.

use crate::types::{Type, Term, Context, Var};
use crate::interval::{Interval, Face};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Semantic values in the NbE domain
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Universe
    Universe(u32),
    
    /// Pi type value
    Pi {
        domain: Box<Value>,
        closure: Closure,
    },
    
    /// Sigma type value
    Sigma {
        first: Box<Value>,
        closure: Closure,
    },
    
    /// Path type value
    Path {
        type_family: Box<Value>,
        left: Box<Value>,
        right: Box<Value>,
    },
    
    /// Lambda abstraction value
    Lambda {
        closure: Closure,
    },
    
    /// Pair value
    Pair {
        first: Box<Value>,
        second: Box<Value>,
    },
    
    /// Path lambda value
    PathLam {
        closure: IntervalClosure,
    },
    
    /// Neutral value (stuck computation)
    Neutral {
        typ: Box<Value>,
        neutral: Neutral,
    },
    
    /// Glue type value
    Glue {
        base: Box<Value>,
        family: Box<Value>, // Simplified
    },
    
    /// System value (partial element)
    System {
        partial: BTreeMap<Face, Value>,
    },
    
    /// Smooth values
    TangentBundle {
        base: Box<Value>,
    },
    
    DifferentialForm {
        base: Box<Value>,
        degree: usize,
    },
    
    /// HIT constructor value
    HITConstructor {
        name: String,
        args: Vec<Value>,
    },
}

/// Neutral terms (blocked computations)
#[derive(Clone, Debug, PartialEq)]
pub enum Neutral {
    /// Variable
    Var(String),
    
    /// De Bruijn index
    Index(usize),
    
    /// Application to neutral
    App {
        function: Box<Neutral>,
        argument: Box<Value>,
    },
    
    /// First projection
    Fst(Box<Neutral>),
    
    /// Second projection
    Snd(Box<Neutral>),
    
    /// Path application
    PathApp {
        path: Box<Neutral>,
        interval: Interval,
    },
    
    /// Composition (stuck)
    Comp {
        type_family: Box<Value>,
        base: Box<Value>,
        partial: BTreeMap<Face, Value>,
    },
    
    /// Transport (stuck)
    Coe {
        type_path: Box<Value>,
        from: Interval,
        to: Interval,
        element: Box<Value>,
    },
    
    /// Unglue (stuck)
    Unglue {
        element: Box<Neutral>,
        typ: Box<Value>,
    },
    
    /// Meta variable
    Meta {
        id: uuid::Uuid,
        spine: Vec<Value>,
    },
}

/// Closure for lambda abstractions
#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    /// Environment at time of closure
    pub env: Environment,
    /// Parameter name
    pub param: String,
    /// Body term
    pub body: Term,
}

/// Closure for path lambdas (with interval variable)
#[derive(Clone, Debug, PartialEq)]
pub struct IntervalClosure {
    /// Environment
    pub env: Environment,
    /// Interval parameter
    pub param: String,
    /// Body term
    pub body: Term,
}

/// Evaluation environment
#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    /// Variable bindings
    pub bindings: BTreeMap<String, Value>,
    /// Interval variable bindings
    pub intervals: BTreeMap<String, Interval>,
    /// De Bruijn levels
    pub level: usize,
}

impl Environment {
    /// Create empty environment
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
            intervals: BTreeMap::new(),
            level: 0,
        }
    }
    
    /// Extend with variable binding
    pub fn extend(&self, var: String, value: Value) -> Self {
        let mut new_env = self.clone();
        new_env.bindings.insert(var, value);
        new_env.level += 1;
        new_env
    }
    
    /// Extend with interval binding
    pub fn extend_interval(&self, var: String, interval: Interval) -> Self {
        let mut new_env = self.clone();
        new_env.intervals.insert(var, interval);
        new_env
    }
    
    /// Look up variable
    pub fn lookup(&self, var: &str) -> Option<&Value> {
        self.bindings.get(var)
    }
    
    /// Look up interval
    pub fn lookup_interval(&self, var: &str) -> Option<&Interval> {
        self.intervals.get(var)
    }
    
    /// Convert to De Bruijn index
    pub fn var_to_index(&self, var: &str) -> Option<usize> {
        // This is simplified - real implementation would track binding order
        if self.bindings.contains_key(var) {
            Some(0) // Placeholder
        } else {
            None
        }
    }
}

/// Evaluation functions
pub fn evaluate(term: &Term, env: &Environment) -> Result<Value> {
    match term {
        Term::Var(name) => {
            env.lookup(name)
                .cloned()
                .ok_or_else(|| Error::UnboundVariable(name.clone()))
        }
        
        Term::Index(i) => {
            // Convert De Bruijn index to value
            // This needs proper implementation based on environment structure
            Ok(Value::Neutral {
                typ: Box::new(Value::Universe(0)), // Placeholder
                neutral: Neutral::Index(*i),
            })
        }
        
        Term::Lambda { param, body } => {
            Ok(Value::Lambda {
                closure: Closure {
                    env: env.clone(),
                    param: param.clone(),
                    body: body.as_ref().clone(),
                },
            })
        }
        
        Term::App { function, argument } => {
            let func_val = evaluate(function, env)?;
            let arg_val = evaluate(argument, env)?;
            apply_value(&func_val, &arg_val)
        }
        
        Term::Pair { first, second } => {
            Ok(Value::Pair {
                first: Box::new(evaluate(first, env)?),
                second: Box::new(evaluate(second, env)?),
            })
        }
        
        Term::Fst(pair) => {
            let pair_val = evaluate(pair, env)?;
            match pair_val {
                Value::Pair { first, .. } => Ok(*first),
                Value::Neutral { typ, neutral } => {
                    Ok(Value::Neutral {
                        typ: extract_first_type(&typ)?,
                        neutral: Neutral::Fst(Box::new(neutral)),
                    })
                }
                _ => Err(Error::NormalizationError("Expected pair for fst".into())),
            }
        }
        
        Term::Snd(pair) => {
            let pair_val = evaluate(pair, env)?;
            match pair_val {
                Value::Pair { first, second } => Ok(*second),
                Value::Neutral { typ, neutral } => {
                    Ok(Value::Neutral {
                        typ: extract_second_type(&typ, &first)?,
                        neutral: Neutral::Snd(Box::new(neutral)),
                    })
                }
                _ => Err(Error::NormalizationError("Expected pair for snd".into())),
            }
        }
        
        Term::PathLam { param, body } => {
            Ok(Value::PathLam {
                closure: IntervalClosure {
                    env: env.clone(),
                    param: param.clone(),
                    body: body.as_ref().clone(),
                },
            })
        }
        
        Term::PathApp { path, interval } => {
            let path_val = evaluate(path, env)?;
            apply_path_value(&path_val, interval, env)
        }
        
        Term::System { partial } => {
            let mut eval_partial = BTreeMap::new();
            for (face, term) in &partial.elements {
                eval_partial.insert(face.clone(), evaluate(term, env)?);
            }
            Ok(Value::System {
                partial: eval_partial,
            })
        }
        
        Term::Comp { type_family, base, partial } => {
            let type_val = evaluate_type(type_family, env)?;
            let base_val = evaluate(base, env)?;
            let mut partial_val = BTreeMap::new();
            for (face, term) in &partial.elements {
                partial_val.insert(face.clone(), evaluate(term, env)?);
            }
            
            // Try to reduce composition
            reduce_composition(&type_val, &base_val, &partial_val, env)
        }
        
        Term::HComp { typ, base, partial } => {
            let type_val = evaluate_type(typ, env)?;
            let base_val = evaluate(base, env)?;
            let mut partial_val = BTreeMap::new();
            for (face, term) in &partial.elements {
                partial_val.insert(face.clone(), evaluate(term, env)?);
            }
            
            reduce_hcomposition(&type_val, &base_val, &partial_val, env)
        }
        
        Term::Coe { type_path, from, to, element } => {
            let path_val = evaluate_type(type_path, env)?;
            let elem_val = evaluate(element, env)?;
            
            reduce_transport(&path_val, from, to, &elem_val, env)
        }
        
        Term::TangentVector { base, direction } => {
            let base_val = evaluate(base, env)?;
            // Evaluate smooth term (simplified)
            Ok(Value::Neutral {
                typ: Box::new(Value::TangentBundle {
                    base: Box::new(Value::Universe(0)), // Placeholder
                }),
                neutral: Neutral::Var("tangent_vector".to_string()), // Placeholder
            })
        }
        
        Term::HITConstructor { name, args } => {
            let mut arg_vals = Vec::new();
            for arg in args {
                arg_vals.push(evaluate(arg, env)?);
            }
            Ok(Value::HITConstructor {
                name: name.clone(),
                args: arg_vals,
            })
        }
        
        Term::Meta { id, spine } => {
            let mut spine_vals = Vec::new();
            for arg in spine {
                spine_vals.push(evaluate(arg, env)?);
            }
            Ok(Value::Neutral {
                typ: Box::new(Value::Universe(0)), // Placeholder
                neutral: Neutral::Meta {
                    id: *id,
                    spine: spine_vals,
                },
            })
        }
        
        _ => Err(Error::NotImplemented("Evaluation for this term".into())),
    }
}

/// Evaluate type expressions
pub fn evaluate_type(typ: &Type, env: &Environment) -> Result<Value> {
    match typ {
        Type::Universe(level) => Ok(Value::Universe(*level)),
        
        Type::Pi { param, domain, codomain } => {
            let domain_val = evaluate_type(domain, env)?;
            let closure = Closure {
                env: env.clone(),
                param: param.clone(),
                body: Term::var("dummy"), // TODO: Convert type to term
            };
            Ok(Value::Pi {
                domain: Box::new(domain_val),
                closure,
            })
        }
        
        Type::Sigma { param, first, second } => {
            let first_val = evaluate_type(first, env)?;
            let closure = Closure {
                env: env.clone(),
                param: param.clone(),
                body: Term::var("dummy"), // TODO: Convert type to term
            };
            Ok(Value::Sigma {
                first: Box::new(first_val),
                closure,
            })
        }
        
        Type::Path { type_family, left, right } => {
            Ok(Value::Path {
                type_family: Box::new(evaluate_type(type_family, env)?),
                left: Box::new(evaluate(left, env)?),
                right: Box::new(evaluate(right, env)?),
            })
        }
        
        Type::TangentBundle { base } => {
            Ok(Value::TangentBundle {
                base: Box::new(evaluate_type(base, env)?),
            })
        }
        
        Type::DifferentialForm { base, degree } => {
            Ok(Value::DifferentialForm {
                base: Box::new(evaluate_type(base, env)?),
                degree: *degree,
            })
        }
        
        _ => Err(Error::NotImplemented("Type evaluation for this type".into())),
    }
}

/// Apply a value to an argument
pub fn apply_value(function: &Value, argument: &Value) -> Result<Value> {
    match function {
        Value::Lambda { closure } => {
            let new_env = closure.env.extend(closure.param.clone(), argument.clone());
            evaluate(&closure.body, &new_env)
        }
        
        Value::Neutral { typ, neutral } => {
            // Compute result type by applying type function
            let result_type = apply_pi_type(typ, argument)?;
            Ok(Value::Neutral {
                typ: Box::new(result_type),
                neutral: Neutral::App {
                    function: Box::new(neutral.clone()),
                    argument: Box::new(argument.clone()),
                },
            })
        }
        
        _ => Err(Error::NormalizationError("Cannot apply non-function".into())),
    }
}

/// Apply path value to interval
pub fn apply_path_value(path: &Value, interval: &Interval, env: &Environment) -> Result<Value> {
    match path {
        Value::PathLam { closure } => {
            let new_env = closure.env.extend_interval(closure.param.clone(), interval.clone());
            evaluate(&closure.body, &new_env)
        }
        
        Value::Neutral { typ, neutral } => {
            Ok(Value::Neutral {
                typ: extract_path_type(typ, interval)?,
                neutral: Neutral::PathApp {
                    path: Box::new(neutral.clone()),
                    interval: interval.clone(),
                },
            })
        }
        
        _ => Err(Error::NormalizationError("Cannot apply non-path".into())),
    }
}

/// Readback values to normal terms
pub fn readback_value(value: &Value, level: usize) -> Result<Term> {
    match value {
        Value::Universe(n) => Ok(Term::var(&format!("Type_{}", n))),
        
        Value::Lambda { closure } => {
            let var_name = format!("x_{}", level);
            let var_value = Value::Neutral {
                typ: Box::new(Value::Universe(0)), // Placeholder
                neutral: Neutral::Var(var_name.clone()),
            };
            let new_env = closure.env.extend(closure.param.clone(), var_value);
            let body_val = evaluate(&closure.body, &new_env)?;
            let body_normal = readback_value(&body_val, level + 1)?;
            Ok(Term::lambda(var_name, body_normal))
        }
        
        Value::Pair { first, second } => {
            Ok(Term::pair(
                readback_value(first, level)?,
                readback_value(second, level)?,
            ))
        }
        
        Value::PathLam { closure } => {
            let var_name = format!("i_{}", level);
            let interval = Interval::var(&var_name);
            let new_env = closure.env.extend_interval(closure.param.clone(), interval);
            let body_val = evaluate(&closure.body, &new_env)?;
            let body_normal = readback_value(&body_val, level + 1)?;
            Ok(Term::path_lambda(var_name, body_normal))
        }
        
        Value::Neutral { neutral, .. } => {
            readback_neutral(neutral, level)
        }
        
        Value::HITConstructor { name, args } => {
            let mut normal_args = Vec::new();
            for arg in args {
                normal_args.push(readback_value(arg, level)?);
            }
            Ok(Term::HITConstructor {
                name: name.clone(),
                args: normal_args,
            })
        }
        
        _ => Err(Error::NotImplemented("Readback for this value".into())),
    }
}

/// Readback neutral terms
pub fn readback_neutral(neutral: &Neutral, level: usize) -> Result<Term> {
    match neutral {
        Neutral::Var(name) => Ok(Term::var(name)),
        Neutral::Index(i) => Ok(Term::Index(*i)),
        
        Neutral::App { function, argument } => {
            let func_term = readback_neutral(function, level)?;
            let arg_term = readback_value(argument, level)?;
            Ok(Term::app(func_term, arg_term))
        }
        
        Neutral::Fst(pair) => {
            let pair_term = readback_neutral(pair, level)?;
            Ok(Term::Fst(Box::new(pair_term)))
        }
        
        Neutral::Snd(pair) => {
            let pair_term = readback_neutral(pair, level)?;
            Ok(Term::Snd(Box::new(pair_term)))
        }
        
        Neutral::PathApp { path, interval } => {
            let path_term = readback_neutral(path, level)?;
            Ok(Term::path_app(path_term, interval.clone()))
        }
        
        Neutral::Meta { id, spine } => {
            let mut spine_terms = Vec::new();
            for arg in spine {
                spine_terms.push(readback_value(arg, level)?);
            }
            Ok(Term::Meta {
                id: *id,
                spine: spine_terms,
            })
        }
        
        _ => Err(Error::NotImplemented("Readback for this neutral".into())),
    }
}

/// Normalize a term
pub fn normalize(term: &Term, env: &Environment) -> Result<Term> {
    let value = evaluate(term, env)?;
    readback_value(&value, env.level)
}

/// Normalize a type
pub fn normalize_type(typ: &Type, env: &Environment) -> Result<Type> {
    let value = evaluate_type(typ, env)?;
    let term = readback_value(&value, env.level)?;
    // Convert term back to type (simplified)
    match term {
        Term::Var(name) => Ok(Type::Var(name)),
        _ => Err(Error::NotImplemented("Type readback".into())),
    }
}

/// Check conversion (definitional equality) using NbE
pub fn conversion_check(term1: &Term, term2: &Term, env: &Environment) -> Result<bool> {
    let val1 = evaluate(term1, env)?;
    let val2 = evaluate(term2, env)?;
    values_equal(&val1, &val2, env.level)
}

/// Check if two values are equal
pub fn values_equal(val1: &Value, val2: &Value, level: usize) -> Result<bool> {
    match (val1, val2) {
        (Value::Universe(n1), Value::Universe(n2)) => Ok(n1 == n2),
        
        (Value::Lambda { closure: c1 }, Value::Lambda { closure: c2 }) => {
            // Eta-equivalence: compare by applying to fresh variable
            let var_name = format!("x_{}", level);
            let var_value = Value::Neutral {
                typ: Box::new(Value::Universe(0)), // Placeholder
                neutral: Neutral::Var(var_name.clone()),
            };
            
            let env1 = c1.env.extend(c1.param.clone(), var_value.clone());
            let env2 = c2.env.extend(c2.param.clone(), var_value);
            
            let body1 = evaluate(&c1.body, &env1)?;
            let body2 = evaluate(&c2.body, &env2)?;
            
            values_equal(&body1, &body2, level + 1)
        }
        
        (Value::Pair { first: f1, second: s1 }, Value::Pair { first: f2, second: s2 }) => {
            Ok(values_equal(f1, f2, level)? && values_equal(s1, s2, level)?)
        }
        
        (Value::Neutral { neutral: n1, .. }, Value::Neutral { neutral: n2, .. }) => {
            neutrals_equal(n1, n2, level)
        }
        
        _ => Ok(false), // Different constructors
    }
}

/// Check if two neutral terms are equal
pub fn neutrals_equal(neut1: &Neutral, neut2: &Neutral, level: usize) -> Result<bool> {
    match (neut1, neut2) {
        (Neutral::Var(x), Neutral::Var(y)) => Ok(x == y),
        (Neutral::Index(i), Neutral::Index(j)) => Ok(i == j),
        
        (
            Neutral::App { function: f1, argument: a1 },
            Neutral::App { function: f2, argument: a2 },
        ) => {
            Ok(neutrals_equal(f1, f2, level)? && values_equal(a1, a2, level)?)
        }
        
        (Neutral::Fst(p1), Neutral::Fst(p2)) => neutrals_equal(p1, p2, level),
        (Neutral::Snd(p1), Neutral::Snd(p2)) => neutrals_equal(p1, p2, level),
        
        _ => Ok(false),
    }
}

// Helper functions for type computations
fn apply_pi_type(pi_type: &Value, argument: &Value) -> Result<Value> {
    match pi_type {
        Value::Pi { closure, .. } => {
            let new_env = closure.env.extend(closure.param.clone(), argument.clone());
            evaluate(&closure.body, &new_env)
        }
        _ => Err(Error::NormalizationError("Expected Pi type".into())),
    }
}

fn extract_first_type(sigma_type: &Value) -> Result<Value> {
    match sigma_type {
        Value::Sigma { first, .. } => Ok(*first.clone()),
        _ => Err(Error::NormalizationError("Expected Sigma type".into())),
    }
}

fn extract_second_type(sigma_type: &Value, first_val: &Value) -> Result<Value> {
    match sigma_type {
        Value::Sigma { closure, .. } => {
            let new_env = closure.env.extend(closure.param.clone(), first_val.clone());
            evaluate(&closure.body, &new_env)
        }
        _ => Err(Error::NormalizationError("Expected Sigma type".into())),
    }
}

fn extract_path_type(path_type: &Value, interval: &Interval) -> Result<Value> {
    match path_type {
        Value::Path { type_family, .. } => {
            // Apply type family to interval
            Ok(*type_family.clone()) // Simplified
        }
        _ => Err(Error::NormalizationError("Expected Path type".into())),
    }
}

// Kan operation reductions
fn reduce_composition(
    type_family: &Value,
    base: &Value,
    partial: &BTreeMap<Face, Value>,
    env: &Environment,
) -> Result<Value> {
    // Check for degenerate cases
    if partial.is_empty() {
        // No constraints: this is transport
        return Ok(base.clone()); // Simplified
    }
    
    // General composition reduction would go here
    Ok(Value::Neutral {
        typ: Box::new(type_family.clone()),
        neutral: Neutral::Comp {
            type_family: Box::new(type_family.clone()),
            base: Box::new(base.clone()),
            partial: partial.clone(),
        },
    })
}

fn reduce_hcomposition(
    typ: &Value,
    base: &Value,
    partial: &BTreeMap<Face, Value>,
    env: &Environment,
) -> Result<Value> {
    // Homogeneous composition reduction
    Ok(Value::Neutral {
        typ: Box::new(typ.clone()),
        neutral: Neutral::Comp {
            type_family: Box::new(typ.clone()),
            base: Box::new(base.clone()),
            partial: partial.clone(),
        },
    })
}

fn reduce_transport(
    type_path: &Value,
    from: &Interval,
    to: &Interval,
    element: &Value,
    env: &Environment,
) -> Result<Value> {
    // Check if from == to
    if from == to {
        return Ok(element.clone());
    }
    
    // General transport reduction
    Ok(Value::Neutral {
        typ: Box::new(type_path.clone()), // Simplified
        neutral: Neutral::Coe {
            type_path: Box::new(type_path.clone()),
            from: from.clone(),
            to: to.clone(),
            element: Box::new(element.clone()),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lambda_evaluation() {
        let env = Environment::new();
        let term = Term::lambda("x", Term::var("x"));
        let value = evaluate(&term, &env).unwrap();
        
        match value {
            Value::Lambda { .. } => {},
            _ => panic!("Expected lambda value"),
        }
    }
    
    #[test]
    fn test_application_evaluation() {
        let env = Environment::new();
        let id = Term::lambda("x", Term::var("x"));
        let app = Term::app(id, Term::var("y"));
        
        // This should be stuck because y is unbound
        let value = evaluate(&app, &env).unwrap();
        match value {
            Value::Neutral { .. } => {},
            _ => panic!("Expected neutral value for unbound variable"),
        }
    }
    
    #[test]
    fn test_pair_projections() {
        let env = Environment::new();
        let pair = Term::pair(Term::var("a"), Term::var("b"));
        let fst = Term::Fst(Box::new(pair.clone()));
        
        // Should be stuck on unbound variables
        let value = evaluate(&fst, &env).unwrap();
        match value {
            Value::Neutral { .. } => {},
            _ => panic!("Expected neutral value"),
        }
    }
    
    #[test]
    fn test_readback() {
        let lambda_val = Value::Lambda {
            closure: Closure {
                env: Environment::new(),
                param: "x".to_string(),
                body: Term::var("x"),
            },
        };
        
        let term = readback_value(&lambda_val, 0).unwrap();
        match term {
            Term::Lambda { .. } => {},
            _ => panic!("Expected lambda term"),
        }
    }
    
    #[test]
    fn test_normalization() {
        let env = Environment::new();
        let term = Term::app(
            Term::lambda("x", Term::var("x")),
            Term::var("y")
        );
        
        let normal = normalize(&term, &env).unwrap();
        // Should normalize to y (if y were bound) or stay as application
        assert!(normal.to_string().contains("y"));
    }
    
    #[test]
    fn test_conversion() {
        let env = Environment::new();
        let term1 = Term::lambda("x", Term::var("x"));
        let term2 = Term::lambda("y", Term::var("y"));
        
        // These should be convertible (alpha-equivalent)
        let equal = conversion_check(&term1, &term2, &env).unwrap();
        assert!(equal);
    }
}