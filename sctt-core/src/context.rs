//! # Context Management for SCTT
//!
//! This module provides sophisticated context management for type checking
//! and evaluation in SCTT, including support for dependent types, interval
//! variables, face constraints, and smooth structure.

pub use crate::types::Context;
use crate::types::{Type, Term, Var};
use crate::interval::{Face, Interval};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

impl Context {
    /// Create context with interval variables
    pub fn with_intervals(intervals: Vec<Var>) -> Self {
        Self {
            bindings: Vec::new(),
            intervals,
            faces: Vec::new(),
        }
    }
    
    /// Create context with face constraints
    pub fn with_faces(faces: Vec<Face>) -> Self {
        Self {
            bindings: Vec::new(),
            intervals: Vec::new(),
            faces,
        }
    }
    
    /// Check if context is well-formed
    pub fn is_well_formed(&self) -> Result<bool> {
        // Check for duplicate variable names
        let mut seen_vars = std::collections::HashSet::new();
        for (var, _) in &self.bindings {
            if seen_vars.contains(var) {
                return Err(Error::type_error(format!("Duplicate variable: {}", var)));
            }
            seen_vars.insert(var);
        }
        
        // Check for duplicate interval names
        let mut seen_intervals = std::collections::HashSet::new();
        for interval in &self.intervals {
            if seen_intervals.contains(interval) {
                return Err(Error::type_error(format!("Duplicate interval variable: {}", interval)));
            }
            seen_intervals.insert(interval);
        }
        
        // Validate face constraints
        for face in &self.faces {
            if !self.face_is_valid(face) {
                return Err(Error::type_error("Invalid face constraint".into()));
            }
        }
        
        Ok(true)
    }
    
    /// Check if a face constraint is valid in this context
    fn face_is_valid(&self, face: &Face) -> bool {
        match face {
            Face::True | Face::False => true,
            Face::Eq0(var) | Face::Eq1(var) => self.has_interval(var),
            Face::Not(box inner) => self.face_is_valid(inner),
            Face::And(box left, box right) | Face::Or(box left, box right) => {
                self.face_is_valid(left) && self.face_is_valid(right)
            }
        }
    }
    
    /// Get all variables in scope (both term and interval variables)
    pub fn all_variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        vars.extend(self.bindings.iter().map(|(name, _)| name.clone()));
        vars.extend(self.intervals.iter().cloned());
        vars
    }
    
    /// Restrict context to a face
    pub fn restrict_to_face(&self, face: &Face) -> Context {
        let mut new_ctx = self.clone();
        new_ctx.faces.push(face.clone());
        new_ctx
    }
    
    /// Check if face constraints are satisfiable
    pub fn faces_satisfiable(&self) -> bool {
        // Simple check: look for contradictions like (i = 0) ∧ (i = 1)
        for i in 0..self.faces.len() {
            for j in i + 1..self.faces.len() {
                if self.faces_contradict(&self.faces[i], &self.faces[j]) {
                    return false;
                }
            }
        }
        true
    }
    
    fn faces_contradict(&self, face1: &Face, face2: &Face) -> bool {
        match (face1, face2) {
            (Face::Eq0(var1), Face::Eq1(var2)) if var1 == var2 => true,
            (Face::Eq1(var1), Face::Eq0(var2)) if var1 == var2 => true,
            _ => false, // More sophisticated contradiction checking would go here
        }
    }
    
    /// Substitute interval variable in context
    pub fn substitute_interval(&self, var: &str, replacement: &Interval) -> Context {
        let mut new_ctx = self.clone();
        
        // Substitute in face constraints
        new_ctx.faces = new_ctx.faces.into_iter()
            .map(|face| self.substitute_interval_in_face(&face, var, replacement))
            .collect();
        
        // Remove the substituted interval variable
        new_ctx.intervals.retain(|v| v != var);
        
        new_ctx
    }
    
    fn substitute_interval_in_face(&self, face: &Face, var: &str, replacement: &Interval) -> Face {
        match face {
            Face::True | Face::False => face.clone(),
            Face::Eq0(interval_var) if interval_var == var => {
                // (var = 0) becomes (replacement = 0)
                // This is complex because replacement might not be a variable
                match replacement {
                    Interval::Var(new_var) => Face::Eq0(new_var.clone()),
                    Interval::Zero => Face::True,
                    Interval::One => Face::False,
                    _ => face.clone(), // TODO: Handle complex intervals
                }
            }
            Face::Eq1(interval_var) if interval_var == var => {
                match replacement {
                    Interval::Var(new_var) => Face::Eq1(new_var.clone()),
                    Interval::Zero => Face::False,
                    Interval::One => Face::True,
                    _ => face.clone(),
                }
            }
            Face::Eq0(_) | Face::Eq1(_) => face.clone(),
            Face::Not(box inner) => Face::Not(Box::new(
                self.substitute_interval_in_face(inner, var, replacement)
            )),
            Face::And(box left, box right) => Face::And(
                Box::new(self.substitute_interval_in_face(left, var, replacement)),
                Box::new(self.substitute_interval_in_face(right, var, replacement)),
            ),
            Face::Or(box left, box right) => Face::Or(
                Box::new(self.substitute_interval_in_face(left, var, replacement)),
                Box::new(self.substitute_interval_in_face(right, var, replacement)),
            ),
        }
    }
    
    /// Add multiple bindings at once
    pub fn extend_with_bindings(&self, bindings: Vec<(Var, Type)>) -> Context {
        let mut new_ctx = self.clone();
        new_ctx.bindings.extend(bindings);
        new_ctx
    }
    
    /// Look up type with De Bruijn index
    pub fn lookup_index(&self, index: usize) -> Option<&Type> {
        // De Bruijn indices count from the most recent binding
        if index < self.bindings.len() {
            Some(&self.bindings[self.bindings.len() - 1 - index].1)
        } else {
            None
        }
    }
    
    /// Convert variable name to De Bruijn index
    pub fn var_to_index(&self, var: &str) -> Option<usize> {
        self.bindings.iter().rev().position(|(name, _)| name == var)
    }
    
    /// Convert De Bruijn index to variable name
    pub fn index_to_var(&self, index: usize) -> Option<&str> {
        if index < self.bindings.len() {
            Some(&self.bindings[self.bindings.len() - 1 - index].0)
        } else {
            None
        }
    }
    
    /// Get the length of the context (number of bindings)
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    
    /// Check if context is empty
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty() && self.intervals.is_empty() && self.faces.is_empty()
    }
    
    /// Get a fresh variable name not in use
    pub fn fresh_var(&self, base: &str) -> String {
        let existing = self.all_variables();
        let mut candidate = base.to_string();
        let mut counter = 0;
        
        while existing.contains(&candidate) {
            counter += 1;
            candidate = format!("{}_{}", base, counter);
        }
        
        candidate
    }
    
    /// Get a fresh interval variable name
    pub fn fresh_interval(&self, base: &str) -> String {
        let mut candidate = base.to_string();
        let mut counter = 0;
        
        while self.intervals.contains(&candidate) {
            counter += 1;
            candidate = format!("{}_{}", base, counter);
        }
        
        candidate
    }
    
    /// Weakening: add a variable without removing any existing bindings
    pub fn weaken(&self, var: Var, typ: Type) -> Context {
        let mut new_ctx = self.clone();
        new_ctx.bindings.insert(0, (var, typ)); // Add at beginning
        new_ctx
    }
    
    /// Strengthen: remove the most recent binding
    pub fn strengthen(&self) -> Option<(Context, (Var, Type))> {
        if self.bindings.is_empty() {
            None
        } else {
            let mut new_ctx = self.clone();
            let removed = new_ctx.bindings.pop().unwrap();
            Some((new_ctx, removed))
        }
    }
    
    /// Apply a face constraint evaluation to the context
    pub fn evaluate_faces(&self, interval_values: &BTreeMap<String, bool>) -> Result<Context> {
        let mut new_faces = Vec::new();
        
        for face in &self.faces {
            match face.evaluate(interval_values) {
                Ok(true) => {}, // Face is satisfied, keep it implicit
                Ok(false) => return Err(Error::type_error("Face constraint violated")),
                Err(_) => new_faces.push(face.clone()), // Keep unevaluated faces
            }
        }
        
        Ok(Context {
            bindings: self.bindings.clone(),
            intervals: self.intervals.iter()
                .filter(|var| !interval_values.contains_key(*var))
                .cloned()
                .collect(),
            faces: new_faces,
        })
    }
    
    /// Check if a term is well-typed in this context
    pub fn well_typed(&self, term: &Term, expected_type: &Type) -> Result<bool> {
        // This would typically call into the type checker
        // For now, just basic checks
        
        match term {
            Term::Var(name) => {
                match self.lookup(name) {
                    Some(actual_type) => Ok(actual_type == expected_type),
                    None => Err(Error::UnboundVariable(name.clone())),
                }
            }
            Term::Index(i) => {
                match self.lookup_index(*i) {
                    Some(actual_type) => Ok(actual_type == expected_type),
                    None => Err(Error::type_error(format!("Index {} out of bounds", i))),
                }
            }
            _ => Ok(true), // TODO: Implement full type checking
        }
    }
}

/// Context zipper for efficient navigation
#[derive(Clone, Debug, PartialEq)]
pub struct ContextZipper {
    /// Bindings before the focus
    pub before: Vec<(Var, Type)>,
    /// Binding at the focus (if any)
    pub focus: Option<(Var, Type)>,
    /// Bindings after the focus
    pub after: Vec<(Var, Type)>,
    /// Interval variables
    pub intervals: Vec<Var>,
    /// Face constraints
    pub faces: Vec<Face>,
}

impl ContextZipper {
    /// Create zipper from context
    pub fn from_context(ctx: &Context) -> Self {
        Self {
            before: Vec::new(),
            focus: None,
            after: ctx.bindings.clone(),
            intervals: ctx.intervals.clone(),
            faces: ctx.faces.clone(),
        }
    }
    
    /// Convert back to context
    pub fn to_context(&self) -> Context {
        let mut bindings = self.before.clone();
        if let Some(focus) = &self.focus {
            bindings.push(focus.clone());
        }
        bindings.extend(self.after.clone());
        
        Context {
            bindings,
            intervals: self.intervals.clone(),
            faces: self.faces.clone(),
        }
    }
    
    /// Move focus to the right
    pub fn move_right(&mut self) -> bool {
        if let Some(focus) = self.focus.take() {
            self.before.push(focus);
        }
        
        if let Some(next) = self.after.first() {
            self.focus = Some(next.clone());
            self.after.remove(0);
            true
        } else {
            false
        }
    }
    
    /// Move focus to the left  
    pub fn move_left(&mut self) -> bool {
        if let Some(focus) = self.focus.take() {
            self.after.insert(0, focus);
        }
        
        if let Some(prev) = self.before.pop() {
            self.focus = Some(prev);
            true
        } else {
            false
        }
    }
    
    /// Insert binding at current position
    pub fn insert(&mut self, var: Var, typ: Type) {
        if let Some(focus) = self.focus.take() {
            self.after.insert(0, focus);
        }
        self.focus = Some((var, typ));
    }
    
    /// Delete current focus
    pub fn delete(&mut self) -> Option<(Var, Type)> {
        let deleted = self.focus.take();
        
        if let Some(next) = self.after.first() {
            self.focus = Some(next.clone());
            self.after.remove(0);
        } else if let Some(prev) = self.before.pop() {
            self.focus = Some(prev);
        }
        
        deleted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Type;
    
    #[test]
    fn test_context_creation() {
        let ctx = Context::new();
        assert!(ctx.is_empty());
        assert!(ctx.is_well_formed().unwrap());
    }
    
    #[test]
    fn test_context_binding() {
        let mut ctx = Context::new();
        ctx.bind("x".to_string(), Type::var("A"));
        ctx.bind("y".to_string(), Type::var("B"));
        
        assert_eq!(ctx.len(), 2);
        assert_eq!(ctx.lookup("x"), Some(&Type::var("A")));
        assert_eq!(ctx.lookup("y"), Some(&Type::var("B")));
    }
    
    #[test]
    fn test_de_bruijn_conversion() {
        let mut ctx = Context::new();
        ctx.bind("x".to_string(), Type::var("A"));
        ctx.bind("y".to_string(), Type::var("B"));
        
        assert_eq!(ctx.var_to_index("y"), Some(0)); // Most recent
        assert_eq!(ctx.var_to_index("x"), Some(1));
        assert_eq!(ctx.index_to_var(0), Some("y"));
        assert_eq!(ctx.index_to_var(1), Some("x"));
    }
    
    #[test]
    fn test_interval_variables() {
        let mut ctx = Context::new();
        ctx.add_interval("i".to_string());
        ctx.add_interval("j".to_string());
        
        assert!(ctx.has_interval("i"));
        assert!(ctx.has_interval("j"));
        assert!(!ctx.has_interval("k"));
    }
    
    #[test]
    fn test_face_constraints() {
        let mut ctx = Context::new();
        ctx.add_interval("i".to_string());
        ctx.add_face(Face::eq0("i"));
        
        assert!(ctx.is_well_formed().unwrap());
        assert!(ctx.faces_satisfiable());
        
        // Add contradictory constraint
        ctx.add_face(Face::eq1("i"));
        assert!(!ctx.faces_satisfiable());
    }
    
    #[test]
    fn test_context_zipper() {
        let mut ctx = Context::new();
        ctx.bind("x".to_string(), Type::var("A"));
        ctx.bind("y".to_string(), Type::var("B"));
        
        let mut zipper = ContextZipper::from_context(&ctx);
        
        assert!(zipper.move_right());
        assert_eq!(zipper.focus, Some(("x".to_string(), Type::var("A"))));
        
        assert!(zipper.move_right());
        assert_eq!(zipper.focus, Some(("y".to_string(), Type::var("B"))));
        
        assert!(!zipper.move_right()); // At end
        
        let reconstructed = zipper.to_context();
        assert_eq!(reconstructed.bindings, ctx.bindings);
    }
    
    #[test]
    fn test_fresh_variables() {
        let mut ctx = Context::new();
        ctx.bind("x".to_string(), Type::var("A"));
        ctx.add_interval("i".to_string());
        
        let fresh_var = ctx.fresh_var("x");
        assert_eq!(fresh_var, "x_1");
        
        let fresh_interval = ctx.fresh_interval("i");
        assert_eq!(fresh_interval, "i_1");
    }
    
    #[test]
    fn test_interval_substitution() {
        let mut ctx = Context::new();
        ctx.add_interval("i".to_string());
        ctx.add_face(Face::eq0("i"));
        
        let new_ctx = ctx.substitute_interval("i", &Interval::zero());
        
        // Face (i = 0) should become True when i is substituted with 0
        assert_eq!(new_ctx.faces.len(), 1);
        assert_eq!(new_ctx.faces[0], Face::True);
        assert!(!new_ctx.has_interval("i"));
    }
}