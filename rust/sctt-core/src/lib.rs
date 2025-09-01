//! Core type system for Smooth Cubical Type Theory
//! This module defines the fundamental types and terms

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};

/// Errors that can occur in the type system
#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Variable {0} not found in context")]
    VariableNotFound(String),
    
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },
    
    #[error("Cannot infer type for term")]
    CannotInfer,
    
    #[error("Boundary condition violated")]
    BoundaryViolation,
    
    #[error("Smoothness verification failed")]
    NotSmooth,
}

pub type Result<T> = std::result::Result<T, TypeError>;

/// Universe levels for type hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Level(pub u32);

impl Level {
    pub const ZERO: Level = Level(0);
    
    pub fn succ(self) -> Level {
        Level(self.0 + 1)
    }
    
    pub fn max(self, other: Level) -> Level {
        Level(self.0.max(other.0))
    }
}

/// Core types in SCTT
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Type universe
    Universe(Level),
    
    /// Function type A → B
    Function {
        domain: Box<Type>,
        codomain: Box<Type>,
        is_smooth: bool,
    },
    
    /// Dependent function type Π(x:A).B
    Pi {
        param: String,
        domain: Box<Type>,
        codomain: Box<Type>,
    },
    
    /// Dependent pair type Σ(x:A).B
    Sigma {
        param: String,
        domain: Box<Type>,
        codomain: Box<Type>,
    },
    
    /// Path type Path A a b
    Path {
        space: Box<Type>,
        start: Box<Term>,
        end: Box<Term>,
    },
    
    /// The interval type I = [0,1]
    Interval,
    
    /// Smooth type modifier
    Smooth(Box<Type>),
    
    /// Real numbers (as a primitive smooth type)
    Real,
}

/// Terms (expressions) in SCTT
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Term {
    /// Variable
    Var(String),
    
    /// Lambda abstraction λx.t
    Lambda {
        param: String,
        param_type: Box<Type>,
        body: Box<Term>,
    },
    
    /// Function application f(a)
    App {
        func: Box<Term>,
        arg: Box<Term>,
    },
    
    /// Pair constructor (a, b)
    Pair {
        first: Box<Term>,
        second: Box<Term>,
    },
    
    /// First projection π₁
    Fst(Box<Term>),
    
    /// Second projection π₂
    Snd(Box<Term>),
    
    /// Path abstraction ⟨i⟩ t
    PathLambda {
        param: String,
        body: Box<Term>,
    },
    
    /// Path application p @ r
    PathApp {
        path: Box<Term>,
        point: Box<Term>,
    },
    
    /// Interval endpoints
    IZero,
    IOne,
    
    /// Interval operations
    IMeet(Box<Term>, Box<Term>),  // i ∧ j
    IJoin(Box<Term>, Box<Term>),   // i ∨ j
    INeg(Box<Term>),               // 1 - i
    
    /// Real number literal
    RealLit(f64),
    
    /// Smooth function (with symbolic representation)
    SmoothFunc {
        expr: String,
        var: String,
    },
}

impl Type {
    /// Check if type is smooth
    pub fn is_smooth(&self) -> bool {
        match self {
            Type::Smooth(_) => true,
            Type::Real => true,  // Real is inherently smooth
            Type::Function { is_smooth, .. } => *is_smooth,
            _ => false,
        }
    }
}

impl Term {
    /// Substitute a term for a variable
    pub fn subst(&self, var: &str, replacement: &Term) -> Term {
        match self {
            Term::Var(name) => {
                if name == var {
                    replacement.clone()
                } else {
                    self.clone()
                }
            }
            Term::Lambda { param, param_type, body } => {
                if param == var {
                    // Variable is bound, no substitution in body
                    self.clone()
                } else {
                    Term::Lambda {
                        param: param.clone(),
                        param_type: param_type.clone(),
                        body: Box::new(body.subst(var, replacement)),
                    }
                }
            }
            Term::App { func, arg } => Term::App {
                func: Box::new(func.subst(var, replacement)),
                arg: Box::new(arg.subst(var, replacement)),
            },
            Term::PathLambda { param, body } => {
                if param == var {
                    self.clone()
                } else {
                    Term::PathLambda {
                        param: param.clone(),
                        body: Box::new(body.subst(var, replacement)),
                    }
                }
            }
            // ... handle other cases
            _ => self.clone(), // For now, clone for unhandled cases
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Universe(level) => write!(f, "Type_{}", level.0),
            Type::Function { domain, codomain, is_smooth } => {
                if *is_smooth {
                    write!(f, "C^∞({} → {})", domain, codomain)
                } else {
                    write!(f, "{} → {}", domain, codomain)
                }
            }
            Type::Pi { param, domain, codomain } => {
                write!(f, "Π({}: {}).{}", param, domain, codomain)
            }
            Type::Sigma { param, domain, codomain } => {
                write!(f, "Σ({}: {}).{}", param, domain, codomain)
            }
            Type::Path { space, start, end } => {
                write!(f, "Path({}, {:?}, {:?})", space, start, end)
            }
            Type::Interval => write!(f, "I"),
            Type::Smooth(ty) => write!(f, "Smooth({})", ty),
            Type::Real => write!(f, "ℝ"),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(name) => write!(f, "{}", name),
            Term::Lambda { param, .. } => write!(f, "λ{}.(...)", param),
            Term::App { func, arg } => write!(f, "({} {})", func, arg),
            Term::IZero => write!(f, "0"),
            Term::IOne => write!(f, "1"),
            Term::RealLit(x) => write!(f, "{}", x),
            Term::SmoothFunc { expr, var } => write!(f, "λ{}.{}", var, expr),
            _ => write!(f, "<term>"),
        }
    }
}

// WASM bindings
#[wasm_bindgen]
pub struct WasmTypeChecker {
    context: Vec<(String, Type)>,
}

#[wasm_bindgen]
impl WasmTypeChecker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTypeChecker {
        WasmTypeChecker {
            context: Vec::new(),
        }
    }

    pub fn add_binding(&mut self, name: String, type_str: String) -> Result<()> {
        // Parse type string into Type
        let ty = self.parse_type(&type_str)?;
        self.context.push((name, ty));
        Ok(())
    }

    pub fn check(&self, expr: &str) -> JsValue {
        // Simple type checking for demonstration
        let result = if expr.contains("sin") || expr.contains("cos") {
            Ok("C∞(ℝ, ℝ)".to_string())
        } else if expr.contains("Path") {
            Ok("Type".to_string())
        } else {
            Err(TypeError::CannotInfer)
        };
        
        match result {
            Ok(ty) => to_value(&serde_json::json!({
                "ok": true,
                "type": ty
            })).unwrap(),
            Err(e) => to_value(&serde_json::json!({
                "ok": false,
                "error": e.to_string()
            })).unwrap()
        }
    }

    fn parse_type(&self, s: &str) -> Result<Type> {
        // Simple type parser
        match s {
            "Real" | "ℝ" => Ok(Type::Real),
            "I" | "Interval" => Ok(Type::Interval),
            s if s.starts_with("C∞") => Ok(Type::Smooth(Box::new(Type::Real))),
            _ => Ok(Type::Universe(Level::ZERO)),
        }
    }
}

// WASM memory management
#[wasm_bindgen]
pub fn wasm_malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[wasm_bindgen]
pub fn wasm_free(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

// Initialize WASM module
#[wasm_bindgen(start)]
pub fn init_wasm() {
    // Set up panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Version info
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_smoothness() {
        let smooth_real = Type::Smooth(Box::new(Type::Real));
        assert!(smooth_real.is_smooth());
        
        let regular_func = Type::Function {
            domain: Box::new(Type::Real),
            codomain: Box::new(Type::Real),
            is_smooth: false,
        };
        assert!(!regular_func.is_smooth());
    }
    
    #[test]
    fn test_substitution() {
        let var = Term::Var("x".to_string());
        let replacement = Term::RealLit(3.14);
        let result = var.subst("x", &replacement);
        assert_eq!(result, Term::RealLit(3.14));
    }
}