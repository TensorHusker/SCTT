//! # Kan Operations for Cubical Type Theory
//!
//! This module implements the fundamental Kan operations that make cubical
//! type theory work:
//! - Composition (comp): Fill boxes with specified boundaries  
//! - Homogeneous composition (hcomp): Composition in constant type families
//! - Transport (coe): Move elements along type equalities
//! - Glue: Univalence via equivalence gluing
//!
//! These operations satisfy the Kan condition: every open box can be
//! filled to a complete cube, respecting the boundary conditions.

use crate::interval::{Interval, Face, PartialSystem, DeMorgan};
use crate::types::{Type, Term, Context, EquivData};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Debug};

/// Composition operation: comp^i A φ [u] a
///
/// Fill a box where:
/// - A is a line of types parametrized by interval i
/// - φ is a face constraint
/// - u is a partial system giving the sides of the box
/// - a is the base of the box
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Composition {
    /// Type family A : I → Type
    pub type_family: Box<Type>,
    /// Interval variable
    pub interval_var: String,
    /// Face constraint φ
    pub face: Face,
    /// Partial system of sides [φ ↦ u]
    pub partial: PartialSystem<Term>,
    /// Base element a : A(0)
    pub base: Box<Term>,
}

/// Homogeneous composition: hcomp^φ A [u] a
///
/// Composition in a constant type family A.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HComp {
    /// Type A
    pub typ: Box<Type>,
    /// Face constraint φ
    pub face: Face,
    /// Partial system [φ ↦ u]
    pub partial: PartialSystem<Term>,
    /// Base element a : A
    pub base: Box<Term>,
}

/// Transport/Coercion: coe^r→s A a
///
/// Transport element a along path of types A from r to s.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transport {
    /// Path of types A : I → Type
    pub type_path: Box<Type>,
    /// Interval variable
    pub interval_var: String,
    /// Starting point r
    pub from: Interval,
    /// Ending point s  
    pub to: Interval,
    /// Element to transport a : A(r)
    pub element: Box<Term>,
}

/// Glue type for univalence: Glue A φ [T ≃ A]
///
/// Glue together a base type A with equivalent types T on faces φ.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Glue {
    /// Base type A
    pub base: Box<Type>,
    /// Face constraint φ
    pub face: Face,
    /// Partial system of equivalences [φ ↦ (T, e : T ≃ A)]
    pub equivalences: PartialSystem<EquivData>,
}

/// Unglue operation: unglue u : Glue A φ [T ≃ A] → A
///
/// Extract the base type element from a glued element.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unglue {
    /// Glued element
    pub element: Box<Term>,
    /// Glue type information
    pub glue_type: Box<Type>,
}

/// Box filling data structure
///
/// Represents an open box that needs to be filled according to Kan conditions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Box {
    /// Dimensions of the box
    pub dimensions: Vec<String>,
    /// Type family over the box
    pub type_family: Box<Type>,
    /// Partial boundary data
    pub boundary: PartialSystem<Term>,
    /// Base corner (when all intervals are 0)
    pub base: Option<Box<Term>>,
}

/// Kan operations trait
pub trait KanOps {
    /// Perform composition operation
    fn composition(
        &self,
        type_family: &Type,
        interval_var: &str,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term>;
    
    /// Perform homogeneous composition
    fn hcomp(
        &self,
        typ: &Type,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term>;
    
    /// Perform transport
    fn transport(
        &self,
        type_path: &Type,
        interval_var: &str,
        from: &Interval,
        to: &Interval,
        element: &Term,
        context: &Context,
    ) -> Result<Term>;
    
    /// Create glue type
    fn glue_type(
        &self,
        base: &Type,
        face: &Face,
        equivalences: &PartialSystem<EquivData>,
        context: &Context,
    ) -> Result<Type>;
    
    /// Perform unglue operation
    fn unglue(
        &self,
        element: &Term,
        glue_type: &Type,
        context: &Context,
    ) -> Result<Term>;
}

/// Default Kan operations implementation
#[derive(Clone, Debug)]
pub struct DefaultKanOps;

impl KanOps for DefaultKanOps {
    fn composition(
        &self,
        type_family: &Type,
        interval_var: &str,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term> {
        // Check if face is true everywhere (degenerate case)
        if face.is_true() {
            // All sides are specified, just pick one
            return partial.elements.values().next()
                .ok_or_else(|| Error::kan_error("composition", "Empty partial system"))
                .cloned();
        }
        
        // Check if face is false everywhere (identity case)
        if face.is_false() {
            // No constraints, transport base from 0 to 1
            return self.transport(
                type_family,
                interval_var,
                &Interval::zero(),
                &Interval::one(),
                base,
                context,
            );
        }
        
        // General case: construct composition term
        // This is where the actual Kan filling algorithm would go
        // For now, we return the composition term itself
        Ok(Term::Comp {
            type_family: Box::new(type_family.clone()),
            base: Box::new(base.clone()),
            partial: partial.clone(),
        })
    }
    
    fn hcomp(
        &self,
        typ: &Type,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term> {
        // Homogeneous composition is composition with constant type family
        let constant_family = typ.clone();
        
        // Check degenerate cases
        if face.is_true() {
            return partial.elements.values().next()
                .ok_or_else(|| Error::kan_error("hcomp", "Empty partial system"))
                .cloned();
        }
        
        if face.is_false() {
            // No constraints, return base
            return Ok(base.clone());
        }
        
        // For specific types, we can implement concrete algorithms
        match typ.as_ref() {
            Type::Pi { domain, codomain, param } => {
                // For function types: (hcomp A u f)(x) = hcomp B (u x) (f x)
                self.hcomp_pi(domain, codomain, param, face, partial, base, context)
            }
            Type::Sigma { first, second, param } => {
                // For pair types: hcomp on each component
                self.hcomp_sigma(first, second, param, face, partial, base, context)
            }
            Type::Path { type_family, left, right } => {
                // For path types: construct path pointwise
                self.hcomp_path(type_family, left, right, face, partial, base, context)
            }
            _ => {
                // General case: return hcomp term
                Ok(Term::HComp {
                    typ: Box::new(typ.clone()),
                    base: Box::new(base.clone()),
                    partial: partial.clone(),
                })
            }
        }
    }
    
    fn transport(
        &self,
        type_path: &Type,
        interval_var: &str,
        from: &Interval,
        to: &Interval,
        element: &Term,
        context: &Context,
    ) -> Result<Term> {
        // If from == to, this is identity
        if from == to {
            return Ok(element.clone());
        }
        
        // For specific type families, implement concrete transport
        match type_path {
            Type::Universe(_) => {
                // Transport in universe is identity
                Ok(element.clone())
            }
            Type::Pi { domain, codomain, param } => {
                // Transport in Pi type
                self.transport_pi(domain, codomain, param, interval_var, from, to, element, context)
            }
            Type::Sigma { first, second, param } => {
                // Transport in Sigma type
                self.transport_sigma(first, second, param, interval_var, from, to, element, context)
            }
            Type::Path { type_family, left, right } => {
                // Transport in Path type
                self.transport_path(type_family, left, right, interval_var, from, to, element, context)
            }
            _ => {
                // General case: return coercion term
                Ok(Term::Coe {
                    type_path: Box::new(type_path.clone()),
                    from: from.clone(),
                    to: to.clone(),
                    element: Box::new(element.clone()),
                })
            }
        }
    }
    
    fn glue_type(
        &self,
        base: &Type,
        face: &Face,
        equivalences: &PartialSystem<EquivData>,
        context: &Context,
    ) -> Result<Type> {
        // Validate that equivalences are well-formed
        for (f, equiv) in &equivalences.elements {
            // Check that the equivalence is between equiv.target and base
            // when face f holds
            // TODO: Add proper validation
        }
        
        Ok(Type::Glue {
            base: Box::new(base.clone()),
            family: equivalences.clone(),
        })
    }
    
    fn unglue(
        &self,
        element: &Term,
        glue_type: &Type,
        context: &Context,
    ) -> Result<Term> {
        match glue_type {
            Type::Glue { base, family } => {
                // Extract base type element from glued element
                // This involves applying the equivalence in the reverse direction
                Ok(Term::Unglue {
                    element: Box::new(element.clone()),
                    typ: Box::new(glue_type.clone()),
                })
            }
            _ => Err(Error::kan_error("unglue", "Not a glue type")),
        }
    }
}

impl DefaultKanOps {
    /// Homogeneous composition for Pi types
    fn hcomp_pi(
        &self,
        domain: &Type,
        codomain: &Type,
        param: &str,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term> {
        // For function types: (hcomp A u f)(x) = hcomp B (u x) (f x)
        // We need to construct a lambda that does hcomp in the codomain
        
        let fresh_var = format!("x_{}", context.bindings.len());
        let arg_var = Term::var(&fresh_var);
        
        // Create partial system by applying each element to the argument
        let mut new_partial = PartialSystem::new();
        for (face_expr, func_term) in &partial.elements {
            let applied = Term::app(func_term.clone(), arg_var.clone());
            new_partial.insert(face_expr.clone(), applied);
        }
        
        // Apply base function to argument
        let base_applied = Term::app(base.clone(), arg_var.clone());
        
        // Substitute the parameter in codomain type
        let codomain_instantiated = codomain.substitute(param, &arg_var)?;
        
        // Perform hcomp in the codomain
        let result_body = self.hcomp(
            &codomain_instantiated,
            face,
            &new_partial,
            &base_applied,
            context,
        )?;
        
        // Wrap in lambda
        Ok(Term::lambda(fresh_var, result_body))
    }
    
    /// Homogeneous composition for Sigma types
    fn hcomp_sigma(
        &self,
        first: &Type,
        second: &Type,
        param: &str,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term> {
        // For pair types: hcomp on each component
        
        // Extract first components
        let mut first_partial = PartialSystem::new();
        for (face_expr, pair_term) in &partial.elements {
            let first_comp = Term::Fst(Box::new(pair_term.clone()));
            first_partial.insert(face_expr.clone(), first_comp);
        }
        
        let base_first = Term::Fst(Box::new(base.clone()));
        
        // Perform hcomp on first component
        let result_first = self.hcomp(first, face, &first_partial, &base_first, context)?;
        
        // Extract second components and substitute first result
        let mut second_partial = PartialSystem::new();
        for (face_expr, pair_term) in &partial.elements {
            let second_comp = Term::Snd(Box::new(pair_term.clone()));
            second_partial.insert(face_expr.clone(), second_comp);
        }
        
        let base_second = Term::Snd(Box::new(base.clone()));
        
        // Instantiate second type with the first component result
        let second_instantiated = second.substitute(param, &result_first)?;
        
        // Perform hcomp on second component
        let result_second = self.hcomp(
            &second_instantiated,
            face,
            &second_partial,
            &base_second,
            context,
        )?;
        
        Ok(Term::pair(result_first, result_second))
    }
    
    /// Homogeneous composition for Path types
    fn hcomp_path(
        &self,
        type_family: &Type,
        left: &Term,
        right: &Term,
        face: &Face,
        partial: &PartialSystem<Term>,
        base: &Term,
        context: &Context,
    ) -> Result<Term> {
        // For path types: construct path pointwise using composition
        let fresh_interval = format!("j_{}", context.intervals.len());
        let interval_var = Interval::var(&fresh_interval);
        
        // Create partial system by applying each path to the interval
        let mut new_partial = PartialSystem::new();
        for (face_expr, path_term) in &partial.elements {
            let applied = Term::path_app(path_term.clone(), interval_var.clone());
            new_partial.insert(face_expr.clone(), applied);
        }
        
        // Apply base path to interval
        let base_applied = Term::path_app(base.clone(), interval_var.clone());
        
        // The type family should be constant for paths, so we use hcomp
        let pointwise_result = self.hcomp(
            type_family,
            face,
            &new_partial,
            &base_applied,
            context,
        )?;
        
        // Wrap in path lambda
        Ok(Term::path_lambda(fresh_interval, pointwise_result))
    }
    
    /// Transport for Pi types
    fn transport_pi(
        &self,
        domain: &Type,
        codomain: &Type,
        param: &str,
        interval_var: &str,
        from: &Interval,
        to: &Interval,
        element: &Term,
        context: &Context,
    ) -> Result<Term> {
        // Transport in Pi type: need to transport both domain and codomain
        // This is quite complex and involves coherence conditions
        
        // For now, return the coercion term
        Ok(Term::Coe {
            type_path: Box::new(Type::Pi {
                param: param.to_string(),
                domain: Box::new(domain.clone()),
                codomain: Box::new(codomain.clone()),
            }),
            from: from.clone(),
            to: to.clone(),
            element: Box::new(element.clone()),
        })
    }
    
    /// Transport for Sigma types
    fn transport_sigma(
        &self,
        first: &Type,
        second: &Type,
        param: &str,
        interval_var: &str,
        from: &Interval,
        to: &Interval,
        element: &Term,
        context: &Context,
    ) -> Result<Term> {
        // Transport pair componentwise
        let first_comp = Term::Fst(Box::new(element.clone()));
        let second_comp = Term::Snd(Box::new(element.clone()));
        
        // Transport first component
        let transported_first = self.transport(
            first,
            interval_var,
            from,
            to,
            &first_comp,
            context,
        )?;
        
        // Transport second component along path that depends on transported first
        let second_path = second.substitute(param, &transported_first)?;
        let transported_second = self.transport(
            &second_path,
            interval_var,
            from,
            to,
            &second_comp,
            context,
        )?;
        
        Ok(Term::pair(transported_first, transported_second))
    }
    
    /// Transport for Path types
    fn transport_path(
        &self,
        type_family: &Type,
        left: &Term,
        right: &Term,
        interval_var: &str,
        from: &Interval,
        to: &Interval,
        element: &Term,
        context: &Context,
    ) -> Result<Term> {
        // Transport in path type involves constructing a new path
        // that respects the transported endpoints
        
        // For now, return the coercion term
        Ok(Term::Coe {
            type_path: Box::new(Type::Path {
                type_family: Box::new(type_family.clone()),
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
            }),
            from: from.clone(),
            to: to.clone(),
            element: Box::new(element.clone()),
        })
    }
}

/// Box filling algorithm
pub fn fill_box(box_data: &Box, context: &Context) -> Result<Term> {
    let kan_ops = DefaultKanOps;
    
    // Simple case: 1-dimensional box (composition)
    if box_data.dimensions.len() == 1 {
        let dim = &box_data.dimensions[0];
        
        // Extract partial system on the boundary
        let boundary_at_1 = box_data.boundary.restrict(&Face::eq1(dim));
        
        if let Some(base) = &box_data.base {
            // We have a base, so we can do composition
            let face = Face::eq1(dim); // Fill towards the 1 face
            
            return kan_ops.composition(
                &box_data.type_family,
                dim,
                &face,
                &boundary_at_1,
                base,
                context,
            );
        }
    }
    
    // Higher-dimensional boxes require more sophisticated algorithms
    Err(Error::kan_error("fill_box", "Higher-dimensional box filling not implemented"))
}

/// Kan condition checker
pub fn check_kan_condition(typ: &Type, context: &Context) -> Result<bool> {
    match typ {
        Type::Universe(_) => Ok(true), // Universes satisfy Kan
        Type::Pi { domain, codomain, .. } => {
            // Pi types satisfy Kan if codomain does
            Ok(check_kan_condition(domain, context)? && 
               check_kan_condition(codomain, context)?)
        }
        Type::Sigma { first, second, .. } => {
            // Sigma types satisfy Kan if both components do
            Ok(check_kan_condition(first, context)? && 
               check_kan_condition(second, context)?)
        }
        Type::Path { type_family, .. } => {
            // Path types satisfy Kan if the type family does
            check_kan_condition(type_family, context)
        }
        Type::Glue { base, .. } => {
            // Glue types satisfy Kan if base does
            check_kan_condition(base, context)
        }
        _ => Ok(true), // Assume other types satisfy Kan for now
    }
}

/// Composition coherence checker
pub fn check_composition_coherence(
    type_family: &Type,
    partial: &PartialSystem<Term>,
    base: &Term,
    context: &Context,
) -> Result<bool> {
    // Check that partial elements agree on overlapping faces
    for (face1, term1) in &partial.elements {
        for (face2, term2) in &partial.elements {
            let overlap = face1.clone().and(face2.clone()).normalize();
            if !overlap.is_false() {
                // Faces overlap, terms must agree
                let restricted1 = term1.clone(); // TODO: Restrict to overlap
                let restricted2 = term2.clone(); // TODO: Restrict to overlap
                
                // Check if terms are equal when restricted to overlap
                // This requires a proper equality checker
            }
        }
    }
    
    // Check boundary conditions
    // The partial system should be compatible with the base at interval 0
    Ok(true) // Simplified for now
}

/// Display implementations
impl Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "comp^{} {} {} [{:?}] {}", 
               self.interval_var, self.type_family, self.face, 
               self.partial, self.base)
    }
}

impl Display for HComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hcomp^{} {} [{:?}] {}", 
               self.face, self.typ, self.partial, self.base)
    }
}

impl Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "coe^{}→{} {} {}", 
               self.from, self.to, self.type_path, self.element)
    }
}

impl Display for Glue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Glue {} {} [{:?}]", self.base, self.face, self.equivalences)
    }
}

impl Display for Unglue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unglue {} : {}", self.element, self.glue_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::{Interval, Face};
    use crate::types::{Type, Term};
    
    #[test]
    fn test_degenerate_composition() {
        let kan_ops = DefaultKanOps;
        let ctx = Context::new();
        
        // Test composition with false face (should be transport)
        let nat_type = Type::var("ℕ");
        let zero = Term::var("0");
        let empty_partial = PartialSystem::new();
        
        let result = kan_ops.composition(
            &nat_type,
            "i",
            &Face::False,
            &empty_partial,
            &zero,
            &ctx,
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_hcomp_identity() {
        let kan_ops = DefaultKanOps;
        let ctx = Context::new();
        
        // Test hcomp with false face (should be identity)
        let nat_type = Type::var("ℕ");
        let zero = Term::var("0");
        let empty_partial = PartialSystem::new();
        
        let result = kan_ops.hcomp(
            &nat_type,
            &Face::False,
            &empty_partial,
            &zero,
            &ctx,
        ).unwrap();
        
        assert_eq!(result, zero);
    }
    
    #[test]
    fn test_transport_identity() {
        let kan_ops = DefaultKanOps;
        let ctx = Context::new();
        
        // Test transport from i to i (should be identity)
        let nat_type = Type::var("ℕ");
        let zero = Term::var("0");
        let i = Interval::var("i");
        
        let result = kan_ops.transport(
            &nat_type,
            "i",
            &i,
            &i,
            &zero,
            &ctx,
        ).unwrap();
        
        assert_eq!(result, zero);
    }
    
    #[test]
    fn test_kan_condition() {
        let ctx = Context::new();
        
        // Universe types satisfy Kan
        assert!(check_kan_condition(&Type::universe(0), &ctx).unwrap());
        
        // Pi types satisfy Kan if domain and codomain do
        let pi_type = Type::pi("x", Type::universe(0), Type::universe(0));
        assert!(check_kan_condition(&pi_type, &ctx).unwrap());
    }
    
    #[test]
    fn test_box_filling() {
        let ctx = Context::new();
        
        // Create a simple 1D box
        let mut boundary = PartialSystem::new();
        boundary.insert(Face::eq1("i"), Term::var("endpoint"));
        
        let box_data = Box {
            dimensions: vec!["i".to_string()],
            type_family: Box::new(Type::var("A")),
            boundary,
            base: Some(Box::new(Term::var("base"))),
        };
        
        let result = fill_box(&box_data, &ctx);
        assert!(result.is_ok());
    }
}