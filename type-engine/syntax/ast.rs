/// Abstract Syntax Tree for Smooth Cubical Type Theory
/// 
/// This module defines the core syntax of SCTT, including:
/// - Terms (expressions)
/// - Types
/// - Dimension expressions
/// - Faces and systems

use std::collections::HashMap;
use std::rc::Rc;

/// Variables are represented by de Bruijn indices for efficiency
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var(pub usize);

/// Names for pretty printing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

/// Dimension variables for cubical structure
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DimVar {
    /// Named dimension variable
    Named(String),
    /// De Bruijn index for dimensions
    Index(usize),
}

/// Dimension expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dim {
    /// Zero endpoint
    D0,
    /// One endpoint  
    D1,
    /// Dimension variable
    Var(DimVar),
    /// Meet (minimum) of dimensions
    Meet(Box<Dim>, Box<Dim>),
    /// Join (maximum) of dimensions
    Join(Box<Dim>, Box<Dim>),
    /// Negation of dimension
    Neg(Box<Dim>),
}

/// Face formulas for boundaries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Face {
    /// Dimension equals endpoint
    Eq(Dim, Dim),
    /// Conjunction of faces
    And(Box<Face>, Box<Face>),
    /// Disjunction of faces
    Or(Box<Face>, Box<Face>),
    /// True (satisfied)
    True,
    /// False (unsatisfied)
    False,
}

/// Systems: partial elements defined on faces
#[derive(Debug, Clone)]
pub struct System<T> {
    /// Map from faces to values
    pub branches: Vec<(Face, T)>,
}

/// Core term syntax
#[derive(Debug, Clone)]
pub enum Term {
    /// Variable
    Var(Var),
    
    /// Type universe
    Type,
    
    /// Smooth type (differentiable structure)
    SmoothType,
    
    /// Pi type (dependent function)
    Pi(Name, Box<Term>, Box<Term>),
    
    /// Lambda abstraction
    Lam(Name, Box<Term>),
    
    /// Application
    App(Box<Term>, Box<Term>),
    
    /// Sigma type (dependent pair)
    Sigma(Name, Box<Term>, Box<Term>),
    
    /// Pair constructor
    Pair(Box<Term>, Box<Term>),
    
    /// First projection
    Fst(Box<Term>),
    
    /// Second projection
    Snd(Box<Term>),
    
    /// Path type
    Path(Box<Term>, Box<Term>, Box<Term>),
    
    /// Path lambda (dimension abstraction)
    PathLam(DimVar, Box<Term>),
    
    /// Path application
    PathApp(Box<Term>, Dim),
    
    /// Composition operation
    Comp {
        /// Starting dimension
        r: Dim,
        /// Ending dimension
        r_prime: Dim,
        /// Type family (may depend on dimension)
        ty: Box<Term>,
        /// Base value
        base: Box<Term>,
        /// System of faces
        faces: System<Term>,
    },
    
    /// Coercion (transport)
    Coe {
        /// Starting dimension
        r: Dim,
        /// Ending dimension  
        r_prime: Dim,
        /// Type family
        ty: Box<Term>,
        /// Term to transport
        tm: Box<Term>,
    },
    
    /// Homogeneous composition
    HCom {
        /// Starting dimension
        r: Dim,
        /// Ending dimension
        r_prime: Dim,
        /// Type (fixed)
        ty: Box<Term>,
        /// Base value
        base: Box<Term>,
        /// System of faces
        faces: System<Term>,
    },
    
    /// Glue type for univalence
    Glue {
        /// Base type
        base: Box<Term>,
        /// Face formula
        face: Face,
        /// Partial type family
        partial_ty: System<Term>,
        /// Equivalences
        equiv: System<Term>,
    },
    
    /// Unglue operation
    Unglue(Box<Term>),
    
    /// Higher inductive type: Circle
    S1,
    
    /// Circle base point
    Base,
    
    /// Circle loop
    Loop(Dim),
    
    /// Circle elimination
    S1Elim {
        /// Motive
        motive: Box<Term>,
        /// Base case
        base_case: Box<Term>,
        /// Loop case
        loop_case: Box<Term>,
        /// Scrutinee
        scrutinee: Box<Term>,
    },
    
    /// Smooth structure operations
    Smooth(SmoothOp),
    
    /// Let binding (for readability)
    Let(Name, Box<Term>, Box<Term>, Box<Term>),
    
    /// Hole (for interactive development)
    Hole(Option<String>),
}

/// Smooth operations specific to SCTT
#[derive(Debug, Clone)]
pub enum SmoothOp {
    /// Smooth path between terms
    SmoothPath {
        start: Box<Term>,
        end: Box<Term>,
        smoothness: f64,
    },
    
    /// Smooth composition with C^n continuity
    SmoothComp {
        order: usize,
        path1: Box<Term>,
        path2: Box<Term>,
    },
    
    /// Differential of a term
    Diff(Box<Term>),
    
    /// Integral along a path
    Integral {
        path: Box<Term>,
        integrand: Box<Term>,
    },
    
    /// Taylor expansion
    Taylor {
        center: Box<Term>,
        order: usize,
        term: Box<Term>,
    },
    
    /// Smooth homotopy
    Homotopy {
        path1: Box<Term>,
        path2: Box<Term>,
        level: usize,
    },
}

/// Type checking context
#[derive(Debug, Clone)]
pub struct Context {
    /// Variable bindings
    pub bindings: Vec<(Name, Term)>,
    /// Dimension variables
    pub dims: Vec<DimVar>,
    /// Face constraints
    pub constraints: Vec<Face>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            bindings: Vec::new(),
            dims: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    pub fn bind(&mut self, name: Name, ty: Term) {
        self.bindings.push((name, ty));
    }
    
    pub fn bind_dim(&mut self, dim: DimVar) {
        self.dims.push(dim);
    }
    
    pub fn assume(&mut self, face: Face) {
        self.constraints.push(face);
    }
    
    pub fn lookup(&self, var: &Var) -> Option<&Term> {
        self.bindings.get(self.bindings.len() - var.0 - 1)
            .map(|(_, ty)| ty)
    }
}

/// Substitution for terms
impl Term {
    /// Substitute term for variable
    pub fn subst(&self, var: Var, replacement: &Term) -> Term {
        // Implementation would go here
        todo!("Implement substitution")
    }
    
    /// Substitute dimension for dimension variable
    pub fn subst_dim(&self, dim_var: &DimVar, dim: &Dim) -> Term {
        // Implementation would go here
        todo!("Implement dimension substitution")
    }
    
    /// Check if term is a value (canonical form)
    pub fn is_value(&self) -> bool {
        match self {
            Term::Lam(_, _) | Term::Pair(_, _) | Term::PathLam(_, _) |
            Term::Type | Term::SmoothType | Term::Base => true,
            _ => false,
        }
    }
}

/// Pretty printing for debugging
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(Var(i)) => write!(f, "x{}", i),
            Term::Type => write!(f, "Type"),
            Term::SmoothType => write!(f, "Smooth"),
            Term::Pi(x, a, b) => write!(f, "Π({} : {}) → {}", x.0, a, b),
            Term::Lam(x, body) => write!(f, "λ{} → {}", x.0, body),
            Term::App(fun, arg) => write!(f, "({} {})", fun, arg),
            Term::Path(ty, a, b) => write!(f, "Path {} {} {}", ty, a, b),
            Term::PathLam(i, body) => write!(f, "<{:?}> {}", i, body),
            _ => write!(f, "..."), // Simplified for brevity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_operations() {
        let mut ctx = Context::new();
        ctx.bind(Name("x".to_string()), Term::Type);
        ctx.bind(Name("y".to_string()), Term::Var(Var(0)));
        
        assert_eq!(ctx.bindings.len(), 2);
        assert!(ctx.lookup(&Var(0)).is_some());
    }
    
    #[test]
    fn test_term_is_value() {
        let lam = Term::Lam(Name("x".to_string()), Box::new(Term::Var(Var(0))));
        assert!(lam.is_value());
        
        let app = Term::App(Box::new(lam.clone()), Box::new(Term::Type));
        assert!(!app.is_value());
    }
}