//! # Type System for Smooth Cubical Type Theory
//!
//! This module implements the complete type system for SCTT, including:
//! - Basic types (Universe, Pi, Sigma, etc.)
//! - Cubical types (Path, Extension, Glue)
//! - Smooth types (Tangent bundles, Differential forms)
//! - Higher Inductive Types
//! - System types and partial elements

use crate::interval::{Interval, Face, PartialSystem};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Debug};
use uuid::Uuid;

/// Universe levels for the type hierarchy
pub type Level = u32;

/// Variable names
pub type Var = String;

/// De Bruijn indices for bound variables
pub type Index = usize;

/// Core type expressions in SCTT
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Universe Type_i
    Universe(Level),
    
    /// Dependent function type (x : A) → B
    Pi {
        /// Parameter name (for display only)
        param: Var,
        /// Domain type
        domain: Box<Type>,
        /// Codomain type (dependent on parameter)
        codomain: Box<Type>,
    },
    
    /// Dependent pair type (x : A) × B
    Sigma {
        /// Parameter name
        param: Var,
        /// First component type
        first: Box<Type>,
        /// Second component type (dependent on first)
        second: Box<Type>,
    },
    
    /// Path type Path A a b
    Path {
        /// Type being connected
        type_family: Box<Type>,
        /// Left endpoint
        left: Box<Term>,
        /// Right endpoint  
        right: Box<Term>,
    },
    
    /// Extension type for partial elements
    Extension {
        /// Base type
        base: Box<Type>,
        /// Partial system of types
        partial: PartialSystem<Type>,
    },
    
    /// Glue type for univalence
    Glue {
        /// Base type
        base: Box<Type>,
        /// Family of equivalent types
        family: PartialSystem<EquivData>,
    },
    
    /// System type [φ ↦ A]
    System {
        /// Face constraint
        face: Face,
        /// Type when face is true
        when_true: Box<Type>,
    },
    
    /// Smooth tangent bundle T(A)
    TangentBundle {
        /// Base type
        base: Box<Type>,
    },
    
    /// Differential form type Ω^k(A)
    DifferentialForm {
        /// Base type
        base: Box<Type>,
        /// Degree of form
        degree: usize,
    },
    
    /// Higher inductive type
    HigherInductive {
        /// HIT name
        name: String,
        /// Type parameters
        params: Vec<Type>,
        /// Constructors
        constructors: Vec<HITConstructor>,
    },
    
    /// Variable reference
    Var(Var),
    
    /// Application type
    App {
        /// Function type
        function: Box<Type>,
        /// Argument
        argument: Box<Term>,
    },
    
    /// Metavariable for unification
    Meta {
        /// Unique identifier
        id: Uuid,
        /// Substitution spine
        spine: Vec<Term>,
    },
}

/// Core term expressions
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Term {
    /// Variable
    Var(Var),
    
    /// De Bruijn index
    Index(Index),
    
    /// Lambda abstraction λx.e
    Lambda {
        /// Parameter name
        param: Var,
        /// Body
        body: Box<Term>,
    },
    
    /// Application f(a)
    App {
        /// Function
        function: Box<Term>,
        /// Argument
        argument: Box<Term>,
    },
    
    /// Pair (a, b)
    Pair {
        /// First component
        first: Box<Term>,
        /// Second component
        second: Box<Term>,
    },
    
    /// First projection π₁(p)
    Fst(Box<Term>),
    
    /// Second projection π₂(p)
    Snd(Box<Term>),
    
    /// Path abstraction λi.e
    PathLam {
        /// Interval parameter
        param: Var,
        /// Body
        body: Box<Term>,
    },
    
    /// Path application p @ i
    PathApp {
        /// Path
        path: Box<Term>,
        /// Interval
        interval: Interval,
    },
    
    /// System term [φ ↦ a]
    System {
        /// Partial system
        partial: PartialSystem<Term>,
    },
    
    /// Composition term
    Comp {
        /// Type family
        type_family: Box<Type>,
        /// Base point
        base: Box<Term>,
        /// Partial system of paths
        partial: PartialSystem<Term>,
    },
    
    /// Homogeneous composition
    HComp {
        /// Type
        typ: Box<Type>,
        /// Base
        base: Box<Term>,
        /// Partial system
        partial: PartialSystem<Term>,
    },
    
    /// Transport/coercion
    Coe {
        /// Path of types
        type_path: Box<Term>,
        /// Starting interval
        from: Interval,
        /// Ending interval  
        to: Interval,
        /// Element to transport
        element: Box<Term>,
    },
    
    /// Glue constructor
    Glue {
        /// Base element
        base: Box<Term>,
        /// Partial equivalences
        partial: PartialSystem<Term>,
    },
    
    /// Unglue operation
    Unglue {
        /// Glued element
        element: Box<Term>,
        /// Type information
        typ: Box<Type>,
    },
    
    /// Smooth tangent vector
    TangentVector {
        /// Base point
        base: Box<Term>,
        /// Tangent direction
        direction: Box<SmoothTerm>,
    },
    
    /// Differential operator d(f)
    Differential {
        /// Function to differentiate
        function: Box<Term>,
    },
    
    /// Integral ∫f
    Integral {
        /// Integrand
        integrand: Box<Term>,
        /// Integration bounds/domain
        domain: Box<Term>,
    },
    
    /// Higher inductive type constructor
    HITConstructor {
        /// Constructor name
        name: String,
        /// Arguments
        args: Vec<Term>,
    },
    
    /// Metavariable
    Meta {
        /// Unique identifier
        id: Uuid,
        /// Substitution spine
        spine: Vec<Term>,
    },
}

/// Smooth/differential terms
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmoothTerm {
    /// Zero vector
    Zero,
    /// Basis vector ∂/∂x
    Basis(Var),
    /// Linear combination
    Linear {
        /// Coefficient
        coeff: f64,
        /// Vector
        vector: Box<SmoothTerm>,
    },
    /// Sum of vectors
    Sum(Vec<SmoothTerm>),
    /// Pushforward of vector along map
    Pushforward {
        /// Map
        map: Box<Term>,
        /// Vector
        vector: Box<SmoothTerm>,
    },
}

/// Equivalence data for Glue types
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EquivData {
    /// Target type
    pub target: Type,
    /// Forward direction
    pub forward: Term,
    /// Backward direction
    pub backward: Term,
    /// Proof that backward ∘ forward = id
    pub section: Term,
    /// Proof that forward ∘ backward = id  
    pub retraction: Term,
}

/// Higher inductive type constructors
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HITConstructor {
    /// Point constructor: c : A
    Point {
        /// Constructor name
        name: String,
        /// Type
        typ: Type,
    },
    
    /// Path constructor: p : Path A a b
    Path {
        /// Constructor name
        name: String,
        /// Path type
        path_type: Type,
        /// Endpoints
        endpoints: (Term, Term),
    },
    
    /// Higher constructor: s : Path (Path A ...) ...
    Higher {
        /// Constructor name
        name: String,
        /// Dimension
        dimension: usize,
        /// Higher path type
        typ: Type,
    },
}

/// Type judgment forms
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeJudgment {
    /// Γ ⊢ A type
    TypeFormation {
        /// Context
        context: Context,
        /// Type
        typ: Type,
    },
    
    /// Γ ⊢ a : A
    TermTyping {
        /// Context
        context: Context,
        /// Term
        term: Term,
        /// Type
        typ: Type,
    },
    
    /// Γ ⊢ A ≡ B
    TypeEquality {
        /// Context
        context: Context,
        /// Left type
        left: Type,
        /// Right type
        right: Type,
    },
    
    /// Γ ⊢ a ≡ b : A
    TermEquality {
        /// Context
        context: Context,
        /// Left term
        left: Term,
        /// Right term
        right: Term,
        /// Type
        typ: Type,
    },
}

/// Typing context
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Context {
    /// Variable bindings
    pub bindings: Vec<(Var, Type)>,
    /// Interval variables
    pub intervals: Vec<Var>,
    /// Face constraints
    pub faces: Vec<Face>,
}

impl Type {
    /// Create universe type
    pub fn universe(level: Level) -> Self {
        Type::Universe(level)
    }
    
    /// Create Pi type
    pub fn pi(param: impl Into<String>, domain: Type, codomain: Type) -> Self {
        Type::Pi {
            param: param.into(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }
    
    /// Create Sigma type
    pub fn sigma(param: impl Into<String>, first: Type, second: Type) -> Self {
        Type::Sigma {
            param: param.into(),
            first: Box::new(first),
            second: Box::new(second),
        }
    }
    
    /// Create Path type
    pub fn path(type_family: Type, left: Term, right: Term) -> Self {
        Type::Path {
            type_family: Box::new(type_family),
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    /// Create tangent bundle type
    pub fn tangent_bundle(base: Type) -> Self {
        Type::TangentBundle {
            base: Box::new(base),
        }
    }
    
    /// Create differential form type
    pub fn differential_form(base: Type, degree: usize) -> Self {
        Type::DifferentialForm { 
            base: Box::new(base),
            degree,
        }
    }
    
    /// Get the universe level of a type
    pub fn universe_level(&self) -> Result<Level> {
        match self {
            Type::Universe(level) => Ok(*level),
            Type::Pi { domain, codomain, .. } => {
                let dom_level = domain.universe_level()?;
                let cod_level = codomain.universe_level()?;
                Ok(dom_level.max(cod_level))
            }
            Type::Sigma { first, second, .. } => {
                let first_level = first.universe_level()?;
                let second_level = second.universe_level()?;
                Ok(first_level.max(second_level))
            }
            Type::Path { type_family, .. } => type_family.universe_level(),
            Type::TangentBundle { base } => base.universe_level(),
            Type::DifferentialForm { base, .. } => base.universe_level(),
            _ => Err(Error::UniverseLevel("Cannot determine universe level".into())),
        }
    }
    
    /// Substitute a variable in the type
    pub fn substitute(&self, var: &str, replacement: &Term) -> Result<Type> {
        match self {
            Type::Universe(level) => Ok(Type::Universe(*level)),
            Type::Pi { param, domain, codomain } => {
                let new_domain = domain.substitute(var, replacement)?;
                let new_codomain = if param == var {
                    // Variable is bound, don't substitute in codomain
                    codomain.as_ref().clone()
                } else {
                    codomain.substitute(var, replacement)?
                };
                Ok(Type::Pi {
                    param: param.clone(),
                    domain: Box::new(new_domain),
                    codomain: Box::new(new_codomain),
                })
            }
            Type::Sigma { param, first, second } => {
                let new_first = first.substitute(var, replacement)?;
                let new_second = if param == var {
                    second.as_ref().clone()
                } else {
                    second.substitute(var, replacement)?
                };
                Ok(Type::Sigma {
                    param: param.clone(),
                    first: Box::new(new_first),
                    second: Box::new(new_second),
                })
            }
            Type::Path { type_family, left, right } => {
                Ok(Type::Path {
                    type_family: Box::new(type_family.substitute(var, replacement)?),
                    left: Box::new(left.substitute(var, replacement)?),
                    right: Box::new(right.substitute(var, replacement)?),
                })
            }
            Type::TangentBundle { base } => {
                Ok(Type::TangentBundle {
                    base: Box::new(base.substitute(var, replacement)?),
                })
            }
            Type::DifferentialForm { base, degree } => {
                Ok(Type::DifferentialForm {
                    base: Box::new(base.substitute(var, replacement)?),
                    degree: *degree,
                })
            }
            Type::Var(name) if name == var => {
                // This is a type-level variable substitution
                // For now, we assume the replacement is a type
                Err(Error::NotImplemented("Type-level variable substitution".into()))
            }
            Type::Var(_) => Ok(self.clone()),
            Type::App { function, argument } => {
                Ok(Type::App {
                    function: Box::new(function.substitute(var, replacement)?),
                    argument: Box::new(argument.substitute(var, replacement)?),
                })
            }
            _ => Err(Error::NotImplemented("Substitution for this type".into())),
        }
    }
    
    /// Collect free variables in the type
    pub fn free_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_free_vars(&mut vars, &mut Vec::new());
        vars.sort();
        vars.dedup();
        vars
    }
    
    fn collect_free_vars(&self, vars: &mut Vec<String>, bound: &mut Vec<String>) {
        match self {
            Type::Universe(_) => {}
            Type::Pi { param, domain, codomain } => {
                domain.collect_free_vars(vars, bound);
                bound.push(param.clone());
                codomain.collect_free_vars(vars, bound);
                bound.pop();
            }
            Type::Sigma { param, first, second } => {
                first.collect_free_vars(vars, bound);
                bound.push(param.clone());
                second.collect_free_vars(vars, bound);
                bound.pop();
            }
            Type::Path { type_family, left, right } => {
                type_family.collect_free_vars(vars, bound);
                left.collect_free_vars(vars, bound);
                right.collect_free_vars(vars, bound);
            }
            Type::TangentBundle { base } => {
                base.collect_free_vars(vars, bound);
            }
            Type::DifferentialForm { base, .. } => {
                base.collect_free_vars(vars, bound);
            }
            Type::Var(name) => {
                if !bound.contains(name) {
                    vars.push(name.clone());
                }
            }
            Type::App { function, argument } => {
                function.collect_free_vars(vars, bound);
                argument.collect_free_vars(vars, bound);
            }
            _ => {} // TODO: Handle other cases
        }
    }
}

impl Term {
    /// Create variable term
    pub fn var(name: impl Into<String>) -> Self {
        Term::Var(name.into())
    }
    
    /// Create lambda abstraction
    pub fn lambda(param: impl Into<String>, body: Term) -> Self {
        Term::Lambda {
            param: param.into(),
            body: Box::new(body),
        }
    }
    
    /// Create application
    pub fn app(function: Term, argument: Term) -> Self {
        Term::App {
            function: Box::new(function),
            argument: Box::new(argument),
        }
    }
    
    /// Create pair
    pub fn pair(first: Term, second: Term) -> Self {
        Term::Pair {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
    
    /// Create path lambda
    pub fn path_lambda(param: impl Into<String>, body: Term) -> Self {
        Term::PathLam {
            param: param.into(),
            body: Box::new(body),
        }
    }
    
    /// Apply path to interval
    pub fn path_app(path: Term, interval: Interval) -> Self {
        Term::PathApp {
            path: Box::new(path),
            interval,
        }
    }
    
    /// Substitute variable in term
    pub fn substitute(&self, var: &str, replacement: &Term) -> Result<Term> {
        match self {
            Term::Var(name) if name == var => Ok(replacement.clone()),
            Term::Var(_) => Ok(self.clone()),
            Term::Index(_) => Ok(self.clone()),
            Term::Lambda { param, body } => {
                if param == var {
                    // Variable is bound
                    Ok(self.clone())
                } else {
                    Ok(Term::Lambda {
                        param: param.clone(),
                        body: Box::new(body.substitute(var, replacement)?),
                    })
                }
            }
            Term::App { function, argument } => {
                Ok(Term::App {
                    function: Box::new(function.substitute(var, replacement)?),
                    argument: Box::new(argument.substitute(var, replacement)?),
                })
            }
            Term::Pair { first, second } => {
                Ok(Term::Pair {
                    first: Box::new(first.substitute(var, replacement)?),
                    second: Box::new(second.substitute(var, replacement)?),
                })
            }
            Term::Fst(term) => {
                Ok(Term::Fst(Box::new(term.substitute(var, replacement)?)))
            }
            Term::Snd(term) => {
                Ok(Term::Snd(Box::new(term.substitute(var, replacement)?)))
            }
            Term::PathLam { param, body } => {
                if param == var {
                    Ok(self.clone())
                } else {
                    Ok(Term::PathLam {
                        param: param.clone(),
                        body: Box::new(body.substitute(var, replacement)?),
                    })
                }
            }
            Term::PathApp { path, interval } => {
                Ok(Term::PathApp {
                    path: Box::new(path.substitute(var, replacement)?),
                    interval: interval.substitute(var, &Term::var("dummy")), // TODO: Handle interval substitution
                })
            }
            _ => Err(Error::NotImplemented("Substitution for this term".into())),
        }
    }
    
    /// Collect free variables in term
    pub fn collect_free_vars(&self, vars: &mut Vec<String>, bound: &mut Vec<String>) {
        match self {
            Term::Var(name) => {
                if !bound.contains(name) {
                    vars.push(name.clone());
                }
            }
            Term::Index(_) => {}
            Term::Lambda { param, body } => {
                bound.push(param.clone());
                body.collect_free_vars(vars, bound);
                bound.pop();
            }
            Term::App { function, argument } => {
                function.collect_free_vars(vars, bound);
                argument.collect_free_vars(vars, bound);
            }
            Term::Pair { first, second } => {
                first.collect_free_vars(vars, bound);
                second.collect_free_vars(vars, bound);
            }
            Term::Fst(term) | Term::Snd(term) => {
                term.collect_free_vars(vars, bound);
            }
            Term::PathLam { param, body } => {
                bound.push(param.clone());
                body.collect_free_vars(vars, bound);
                bound.pop();
            }
            Term::PathApp { path, .. } => {
                path.collect_free_vars(vars, bound);
                // TODO: Handle interval variables
            }
            _ => {} // TODO: Handle other cases
        }
    }
}

impl Context {
    /// Create empty context
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            intervals: Vec::new(),
            faces: Vec::new(),
        }
    }
    
    /// Add variable binding
    pub fn bind(&mut self, var: Var, typ: Type) {
        self.bindings.push((var, typ));
    }
    
    /// Add interval variable
    pub fn add_interval(&mut self, var: Var) {
        self.intervals.push(var);
    }
    
    /// Add face constraint
    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }
    
    /// Look up variable type
    pub fn lookup(&self, var: &str) -> Option<&Type> {
        self.bindings
            .iter()
            .rev()
            .find(|(name, _)| name == var)
            .map(|(_, typ)| typ)
    }
    
    /// Check if interval variable is bound
    pub fn has_interval(&self, var: &str) -> bool {
        self.intervals.contains(&var.to_string())
    }
    
    /// Extend context with new binding
    pub fn extend(&self, var: Var, typ: Type) -> Self {
        let mut new_ctx = self.clone();
        new_ctx.bind(var, typ);
        new_ctx
    }
}

// Display implementations
impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Universe(level) => write!(f, "Type_{}", level),
            Type::Pi { param, domain, codomain } => {
                write!(f, "({} : {}) → {}", param, domain, codomain)
            }
            Type::Sigma { param, first, second } => {
                write!(f, "({} : {}) × {}", param, first, second)
            }
            Type::Path { type_family, left, right } => {
                write!(f, "Path {} {} {}", type_family, left, right)
            }
            Type::TangentBundle { base } => write!(f, "T({})", base),
            Type::DifferentialForm { base, degree } => write!(f, "Ω^{}({})", degree, base),
            Type::Var(name) => write!(f, "{}", name),
            Type::App { function, argument } => write!(f, "{} {}", function, argument),
            _ => write!(f, "{{complex type}}"),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(name) => write!(f, "{}", name),
            Term::Index(i) => write!(f, "${}", i),
            Term::Lambda { param, body } => write!(f, "λ{}.{}", param, body),
            Term::App { function, argument } => write!(f, "({} {})", function, argument),
            Term::Pair { first, second } => write!(f, "({}, {})", first, second),
            Term::Fst(term) => write!(f, "π₁({})", term),
            Term::Snd(term) => write!(f, "π₂({})", term),
            Term::PathLam { param, body } => write!(f, "λ{}.{}", param, body),
            Term::PathApp { path, interval } => write!(f, "{} @ {}", path, interval),
            _ => write!(f, "{{complex term}}"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Debug for SmoothTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmoothTerm::Zero => write!(f, "0"),
            SmoothTerm::Basis(var) => write!(f, "∂/∂{}", var),
            SmoothTerm::Linear { coeff, vector } => write!(f, "{} * {}", coeff, vector),
            SmoothTerm::Sum(vectors) => {
                write!(f, "(")?;
                for (i, v) in vectors.iter().enumerate() {
                    if i > 0 { write!(f, " + ")?; }
                    write!(f, "{:?}", v)?;
                }
                write!(f, ")")
            }
            SmoothTerm::Pushforward { map, vector } => {
                write!(f, "d{}({:?})", map, vector)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_construction() {
        let nat = Type::var("ℕ");
        let bool_type = Type::var("Bool");
        
        // (x : ℕ) → Bool
        let pred_type = Type::pi("x", nat.clone(), bool_type);
        
        // Path ℕ 0 1
        let path_type = Type::path(nat, Term::var("0"), Term::var("1"));
        
        assert_eq!(pred_type.free_vars(), vec!["Bool".to_string(), "ℕ".to_string()]);
    }
    
    #[test]
    fn test_term_construction() {
        let id_term = Term::lambda("x", Term::var("x"));
        let app_term = Term::app(id_term, Term::var("y"));
        
        assert_eq!(app_term.to_string(), "((λx.x) y)");
    }
    
    #[test]
    fn test_substitution() {
        let term = Term::lambda("x", Term::app(Term::var("f"), Term::var("x")));
        let replacement = Term::var("g");
        
        let result = term.substitute("f", &replacement).unwrap();
        // Should be λx.(g x)
        
        match result {
            Term::Lambda { body, .. } => {
                match body.as_ref() {
                    Term::App { function, .. } => {
                        assert_eq!(**function, Term::var("g"));
                    }
                    _ => panic!("Expected application"),
                }
            }
            _ => panic!("Expected lambda"),
        }
    }
    
    #[test]
    fn test_context_operations() {
        let mut ctx = Context::new();
        ctx.bind("x".to_string(), Type::var("ℕ"));
        ctx.bind("y".to_string(), Type::var("Bool"));
        
        assert_eq!(ctx.lookup("x"), Some(&Type::var("ℕ")));
        assert_eq!(ctx.lookup("y"), Some(&Type::var("Bool")));
        assert_eq!(ctx.lookup("z"), None);
    }
}