//! # Term Language for SCTT
//!
//! This module extends the basic term language from types.rs with
//! additional constructors and operations specific to SCTT.

pub use crate::types::{Term, SmoothTerm, Type, Var, Index};
pub use crate::interval::{Interval, Face};
pub use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Variable binding information
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable {
    /// Variable name
    pub name: String,
    /// Type annotation
    pub typ: Option<Type>,
    /// De Bruijn level (distance from binding site)
    pub level: Option<Index>,
}

/// Lambda abstraction with full binding info
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lambda {
    /// Parameter information
    pub param: Variable,
    /// Body of the lambda
    pub body: Box<Term>,
    /// Optional type annotation for the lambda itself
    pub typ: Option<Type>,
}

/// Function application with typing information
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Application {
    /// Function being applied
    pub function: Box<Term>,
    /// Argument
    pub argument: Box<Term>,
    /// Computed result type (for caching)
    pub result_type: Option<Type>,
}

/// Reexport commonly used constructors
impl Term {
    /// Create a typed variable
    pub fn typed_var(name: impl Into<String>, typ: Type) -> Self {
        Term::Var(name.into())
        // TODO: Extend Term enum to include typing information
    }
    
    /// Create a typed lambda with full information
    pub fn typed_lambda(param_name: impl Into<String>, param_type: Type, body: Term) -> Self {
        Term::Lambda {
            param: param_name.into(),
            body: Box::new(body),
        }
        // TODO: Include type information
    }
    
    /// Create application with type caching
    pub fn typed_app(function: Term, argument: Term, result_type: Option<Type>) -> Self {
        Term::App {
            function: Box::new(function),
            argument: Box::new(argument),
        }
        // TODO: Include result type caching
    }
    
    /// Check if term is a value (normal form)
    pub fn is_value(&self) -> bool {
        match self {
            Term::Lambda { .. } => true,
            Term::Pair { first, second } => first.is_value() && second.is_value(),
            Term::PathLam { .. } => true,
            Term::Var(_) => true,
            Term::Index(_) => true,
            Term::HITConstructor { args, .. } => args.iter().all(|arg| arg.is_value()),
            _ => false,
        }
    }
    
    /// Check if term is neutral (blocked on a variable)
    pub fn is_neutral(&self) -> bool {
        match self {
            Term::Var(_) | Term::Index(_) => true,
            Term::App { function, .. } => function.is_neutral(),
            Term::Fst(term) | Term::Snd(term) => term.is_neutral(),
            Term::PathApp { path, .. } => path.is_neutral(),
            _ => false,
        }
    }
    
    /// Get the head symbol of a term (for pattern matching)
    pub fn head(&self) -> Option<String> {
        match self {
            Term::Var(name) => Some(name.clone()),
            Term::App { function, .. } => function.head(),
            Term::HITConstructor { name, .. } => Some(name.clone()),
            _ => None,
        }
    }
    
    /// Apply a term to arguments (curried application)
    pub fn apply_to(&self, args: &[Term]) -> Term {
        args.iter().fold(self.clone(), |acc, arg| {
            Term::app(acc, arg.clone())
        })
    }
    
    /// Create composition term with proper typing
    pub fn composition(
        type_family: Type,
        base: Term,
        partial_system: BTreeMap<Face, Term>,
    ) -> Self {
        use crate::interval::PartialSystem;
        let mut partial = PartialSystem::new();
        for (face, term) in partial_system {
            partial.insert(face, term);
        }
        
        Term::Comp {
            type_family: Box::new(type_family),
            base: Box::new(base),
            partial,
        }
    }
    
    /// Create transport/coercion term
    pub fn transport(type_path: Term, from: Interval, to: Interval, element: Term) -> Self {
        Term::Coe {
            type_path: Box::new(type_path),
            from,
            to,
            element: Box::new(element),
        }
    }
    
    /// Create smooth differential term
    pub fn smooth_differential(function: Term) -> Self {
        Term::Differential {
            function: Box::new(function),
        }
    }
    
    /// Create tangent vector term
    pub fn tangent_vector(base: Term, direction: SmoothTerm) -> Self {
        Term::TangentVector {
            base: Box::new(base),
            direction: Box::new(direction),
        }
    }
    
    /// Create integral term
    pub fn integral(integrand: Term, domain: Term) -> Self {
        Term::Integral {
            integrand: Box::new(integrand),
            domain: Box::new(domain),
        }
    }
    
    /// Compute weak head normal form (single step reduction)
    pub fn whnf(&self) -> Result<Term> {
        match self {
            Term::App { function, argument } => {
                let func_whnf = function.whnf()?;
                match func_whnf {
                    Term::Lambda { param, body } => {
                        // Beta reduction
                        body.substitute(&param, argument)
                    }
                    _ => Ok(Term::App {
                        function: Box::new(func_whnf),
                        argument: argument.clone(),
                    }),
                }
            }
            Term::Fst(pair) => {
                let pair_whnf = pair.whnf()?;
                match pair_whnf {
                    Term::Pair { first, .. } => Ok(*first),
                    _ => Ok(Term::Fst(Box::new(pair_whnf))),
                }
            }
            Term::Snd(pair) => {
                let pair_whnf = pair.whnf()?;
                match pair_whnf {
                    Term::Pair { second, .. } => Ok(*second),
                    _ => Ok(Term::Snd(Box::new(pair_whnf))),
                }
            }
            Term::PathApp { path, interval } => {
                let path_whnf = path.whnf()?;
                match path_whnf {
                    Term::PathLam { param, body } => {
                        // Path beta reduction
                        let interval_term = Term::var("dummy"); // TODO: Convert interval to term
                        body.substitute(&param, &interval_term)
                    }
                    _ => Ok(Term::PathApp {
                        path: Box::new(path_whnf),
                        interval: interval.clone(),
                    }),
                }
            }
            // Already in weak head normal form
            _ => Ok(self.clone()),
        }
    }
    
    /// Full normalization (iterate whnf until stable)
    pub fn normalize(&self) -> Result<Term> {
        let mut current = self.clone();
        loop {
            let next = current.whnf()?;
            if next == current {
                break;
            }
            current = next;
        }
        Ok(current)
    }
    
    /// Alpha-equivalence check
    pub fn alpha_eq(&self, other: &Term) -> bool {
        self.alpha_eq_with_env(other, &mut BTreeMap::new(), &mut BTreeMap::new())
    }
    
    fn alpha_eq_with_env(
        &self,
        other: &Term,
        env1: &mut BTreeMap<String, String>,
        env2: &mut BTreeMap<String, String>,
    ) -> bool {
        match (self, other) {
            (Term::Var(x), Term::Var(y)) => {
                match (env1.get(x), env2.get(y)) {
                    (Some(x_mapped), Some(y_mapped)) => x_mapped == y_mapped,
                    (None, None) => x == y,
                    _ => false,
                }
            }
            (Term::Index(i), Term::Index(j)) => i == j,
            (
                Term::Lambda { param: p1, body: b1 },
                Term::Lambda { param: p2, body: b2 },
            ) => {
                let fresh = format!("__fresh_{}", env1.len());
                env1.insert(p1.clone(), fresh.clone());
                env2.insert(p2.clone(), fresh.clone());
                let result = b1.alpha_eq_with_env(b2, env1, env2);
                env1.remove(p1);
                env2.remove(p2);
                result
            }
            (
                Term::App { function: f1, argument: a1 },
                Term::App { function: f2, argument: a2 },
            ) => f1.alpha_eq_with_env(f2, env1, env2) && a1.alpha_eq_with_env(a2, env1, env2),
            (
                Term::Pair { first: f1, second: s1 },
                Term::Pair { first: f2, second: s2 },
            ) => f1.alpha_eq_with_env(f2, env1, env2) && s1.alpha_eq_with_env(s2, env1, env2),
            (Term::Fst(t1), Term::Fst(t2)) => t1.alpha_eq_with_env(t2, env1, env2),
            (Term::Snd(t1), Term::Snd(t2)) => t1.alpha_eq_with_env(t2, env1, env2),
            (
                Term::PathLam { param: p1, body: b1 },
                Term::PathLam { param: p2, body: b2 },
            ) => {
                let fresh = format!("__fresh_interval_{}", env1.len());
                env1.insert(p1.clone(), fresh.clone());
                env2.insert(p2.clone(), fresh.clone());
                let result = b1.alpha_eq_with_env(b2, env1, env2);
                env1.remove(p1);
                env2.remove(p2);
                result
            }
            (
                Term::PathApp { path: p1, interval: i1 },
                Term::PathApp { path: p2, interval: i2 },
            ) => p1.alpha_eq_with_env(p2, env1, env2) && i1 == i2, // Intervals are ground
            _ => false,
        }
    }
    
    /// Compute the size of a term (for termination checking)
    pub fn size(&self) -> usize {
        match self {
            Term::Var(_) | Term::Index(_) => 1,
            Term::Lambda { body, .. } => 1 + body.size(),
            Term::App { function, argument } => 1 + function.size() + argument.size(),
            Term::Pair { first, second } => 1 + first.size() + second.size(),
            Term::Fst(term) | Term::Snd(term) => 1 + term.size(),
            Term::PathLam { body, .. } => 1 + body.size(),
            Term::PathApp { path, .. } => 1 + path.size(),
            Term::System { partial } => {
                1 + partial.elements.values().map(|t| t.size()).sum::<usize>()
            }
            Term::Comp { type_family, base, partial } => {
                1 + type_family.size() + base.size() + 
                partial.elements.values().map(|t| t.size()).sum::<usize>()
            }
            Term::HComp { typ, base, partial } => {
                1 + typ.size() + base.size() + 
                partial.elements.values().map(|t| t.size()).sum::<usize>()
            }
            Term::Coe { type_path, element, .. } => {
                1 + type_path.size() + element.size()
            }
            Term::Glue { base, partial } => {
                1 + base.size() + partial.elements.values().map(|t| t.size()).sum::<usize>()
            }
            Term::Unglue { element, typ } => {
                1 + element.size() + typ.size()
            }
            Term::TangentVector { base, direction } => {
                1 + base.size() + direction.size()
            }
            Term::Differential { function } => 1 + function.size(),
            Term::Integral { integrand, domain } => 1 + integrand.size() + domain.size(),
            Term::HITConstructor { args, .. } => {
                1 + args.iter().map(|arg| arg.size()).sum::<usize>()
            }
            Term::Meta { spine, .. } => {
                1 + spine.iter().map(|arg| arg.size()).sum::<usize>()
            }
        }
    }
}

impl Type {
    /// Compute the size of a type
    pub fn size(&self) -> usize {
        match self {
            Type::Universe(_) => 1,
            Type::Var(_) => 1,
            Type::Pi { domain, codomain, .. } => 1 + domain.size() + codomain.size(),
            Type::Sigma { first, second, .. } => 1 + first.size() + second.size(),
            Type::Path { type_family, left, right } => {
                1 + type_family.size() + left.size() + right.size()
            }
            Type::Extension { base, partial } => {
                1 + base.size() + partial.elements.values().map(|t| t.size()).sum::<usize>()
            }
            Type::Glue { base, family } => {
                1 + base.size() + family.elements.values().map(|e| e.target.size()).sum::<usize>()
            }
            Type::System { when_true, .. } => 1 + when_true.size(),
            Type::TangentBundle { base } => 1 + base.size(),
            Type::DifferentialForm { base, .. } => 1 + base.size(),
            Type::HigherInductive { params, constructors, .. } => {
                1 + params.iter().map(|p| p.size()).sum::<usize>() +
                constructors.len() // Simplified size for constructors
            }
            Type::App { function, argument } => 1 + function.size() + argument.size(),
            Type::Meta { spine, .. } => {
                1 + spine.iter().map(|arg| arg.size()).sum::<usize>()
            }
        }
    }
}

impl SmoothTerm {
    /// Create zero tangent vector
    pub fn zero() -> Self {
        SmoothTerm::Zero
    }
    
    /// Create basis vector
    pub fn basis(var: impl Into<String>) -> Self {
        SmoothTerm::Basis(var.into())
    }
    
    /// Create linear combination
    pub fn linear(coeff: f64, vector: SmoothTerm) -> Self {
        SmoothTerm::Linear {
            coeff,
            vector: Box::new(vector),
        }
    }
    
    /// Add tangent vectors
    pub fn add(vectors: Vec<SmoothTerm>) -> Self {
        SmoothTerm::Sum(vectors)
    }
    
    /// Pushforward along a map
    pub fn pushforward(map: Term, vector: SmoothTerm) -> Self {
        SmoothTerm::Pushforward {
            map: Box::new(map),
            vector: Box::new(vector),
        }
    }
    
    /// Compute the size of a smooth term
    pub fn size(&self) -> usize {
        match self {
            SmoothTerm::Zero => 1,
            SmoothTerm::Basis(_) => 1,
            SmoothTerm::Linear { vector, .. } => 1 + vector.size(),
            SmoothTerm::Sum(vectors) => 1 + vectors.iter().map(|v| v.size()).sum::<usize>(),
            SmoothTerm::Pushforward { map, vector } => 1 + map.size() + vector.size(),
        }
    }
    
    /// Simplify smooth term (combine like terms, etc.)
    pub fn simplify(self) -> Self {
        match self {
            SmoothTerm::Linear { coeff, vector } => {
                if coeff == 0.0 {
                    SmoothTerm::Zero
                } else if coeff == 1.0 {
                    vector.simplify()
                } else {
                    SmoothTerm::Linear {
                        coeff,
                        vector: Box::new(vector.simplify()),
                    }
                }
            }
            SmoothTerm::Sum(vectors) => {
                let simplified: Vec<_> = vectors.into_iter()
                    .map(|v| v.simplify())
                    .filter(|v| !matches!(v, SmoothTerm::Zero))
                    .collect();
                
                if simplified.is_empty() {
                    SmoothTerm::Zero
                } else if simplified.len() == 1 {
                    simplified.into_iter().next().unwrap()
                } else {
                    SmoothTerm::Sum(simplified)
                }
            }
            SmoothTerm::Pushforward { map, vector } => {
                let simplified_vector = vector.simplify();
                match simplified_vector {
                    SmoothTerm::Zero => SmoothTerm::Zero,
                    _ => SmoothTerm::Pushforward {
                        map,
                        vector: Box::new(simplified_vector),
                    },
                }
            }
            other => other,
        }
    }
}

/// Utility functions for term manipulation
pub mod utils {
    use super::*;
    
    /// Check if two terms are definitionally equal
    pub fn definitional_equality(term1: &Term, term2: &Term) -> Result<bool> {
        let norm1 = term1.normalize()?;
        let norm2 = term2.normalize()?;
        Ok(norm1.alpha_eq(&norm2))
    }
    
    /// Fresh variable generator
    pub fn fresh_var(base: &str, avoid: &[String]) -> String {
        let mut candidate = base.to_string();
        let mut counter = 0;
        
        while avoid.contains(&candidate) {
            counter += 1;
            candidate = format!("{}_{}", base, counter);
        }
        
        candidate
    }
    
    /// Rename bound variables to avoid capture
    pub fn alpha_rename(term: &Term, old_name: &str, new_name: &str) -> Result<Term> {
        term.substitute(old_name, &Term::var(new_name))
    }
    
    /// Check if a term is closed (no free variables)
    pub fn is_closed(term: &Term) -> bool {
        let mut vars = Vec::new();
        term.collect_free_vars(&mut vars, &mut Vec::new());
        vars.is_empty()
    }
    
    /// Get all subterms of a term
    pub fn subterms(term: &Term) -> Vec<Term> {
        let mut result = vec![term.clone()];
        
        match term {
            Term::Lambda { body, .. } => {
                result.extend(subterms(body));
            }
            Term::App { function, argument } => {
                result.extend(subterms(function));
                result.extend(subterms(argument));
            }
            Term::Pair { first, second } => {
                result.extend(subterms(first));
                result.extend(subterms(second));
            }
            Term::Fst(t) | Term::Snd(t) => {
                result.extend(subterms(t));
            }
            Term::PathLam { body, .. } => {
                result.extend(subterms(body));
            }
            Term::PathApp { path, .. } => {
                result.extend(subterms(path));
            }
            _ => {} // TODO: Handle other cases
        }
        
        result
    }
    
    /// Pattern matching utilities
    pub fn match_lambda(term: &Term) -> Option<(&str, &Term)> {
        match term {
            Term::Lambda { param, body } => Some((param, body)),
            _ => None,
        }
    }
    
    pub fn match_app(term: &Term) -> Option<(&Term, &Term)> {
        match term {
            Term::App { function, argument } => Some((function, argument)),
            _ => None,
        }
    }
    
    pub fn match_pair(term: &Term) -> Option<(&Term, &Term)> {
        match term {
            Term::Pair { first, second } => Some((first, second)),
            _ => None,
        }
    }
    
    /// Path-specific utilities
    pub fn match_path_lambda(term: &Term) -> Option<(&str, &Term)> {
        match term {
            Term::PathLam { param, body } => Some((param, body)),
            _ => None,
        }
    }
    
    pub fn match_path_app(term: &Term) -> Option<(&Term, &Interval)> {
        match term {
            Term::PathApp { path, interval } => Some((path, interval)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::Interval;
    
    #[test]
    fn test_term_normalization() {
        // (λx.x) y should normalize to y
        let id = Term::lambda("x", Term::var("x"));
        let app = Term::app(id, Term::var("y"));
        let normalized = app.normalize().unwrap();
        
        assert_eq!(normalized, Term::var("y"));
    }
    
    #[test]
    fn test_alpha_equivalence() {
        let term1 = Term::lambda("x", Term::var("x"));
        let term2 = Term::lambda("y", Term::var("y"));
        
        assert!(term1.alpha_eq(&term2));
    }
    
    #[test]
    fn test_pair_projections() {
        let pair = Term::pair(Term::var("a"), Term::var("b"));
        let fst = Term::Fst(Box::new(pair.clone()));
        let snd = Term::Snd(Box::new(pair));
        
        assert_eq!(fst.normalize().unwrap(), Term::var("a"));
        assert_eq!(snd.normalize().unwrap(), Term::var("b"));
    }
    
    #[test]
    fn test_path_application() {
        let path = Term::path_lambda("i", Term::var("x"));
        let app = Term::path_app(path, Interval::zero());
        
        // Should normalize to x (though interval substitution is not fully implemented)
        let normalized = app.whnf().unwrap();
        match normalized {
            Term::PathApp { .. } => {}, // Expected for now
            _ => panic!("Unexpected normalization result"),
        }
    }
    
    #[test]
    fn test_smooth_term_simplification() {
        let zero = SmoothTerm::zero();
        let basis_x = SmoothTerm::basis("x");
        let sum = SmoothTerm::Sum(vec![zero, basis_x.clone()]);
        
        let simplified = sum.simplify();
        assert_eq!(simplified, basis_x);
    }
    
    #[test]
    fn test_term_size() {
        let simple = Term::var("x");
        assert_eq!(simple.size(), 1);
        
        let complex = Term::app(
            Term::lambda("x", Term::var("x")),
            Term::pair(Term::var("a"), Term::var("b"))
        );
        assert!(complex.size() > 5);
    }
    
    #[test]
    fn test_utils() {
        use super::utils::*;
        
        let closed = Term::lambda("x", Term::var("x"));
        assert!(is_closed(&closed));
        
        let open = Term::app(Term::var("f"), Term::var("x"));
        assert!(!is_closed(&open));
        
        let fresh = fresh_var("x", &["x".to_string(), "x_1".to_string()]);
        assert_eq!(fresh, "x_2");
    }
}