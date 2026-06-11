//! # Higher Inductive Types for SCTT
//!
//! Higher Inductive Types (HITs) are a fundamental feature of cubical type theory
//! that allow us to define types with both point constructors and path constructors.
//! This enables the definition of quotient types, truncations, and many other
//! important mathematical structures.

use crate::types::{Type, Term, HITConstructor, Context};
use crate::interval::{Interval, Face};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Higher Inductive Type definition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HIT {
    /// Name of the HIT
    pub name: String,
    /// Type parameters
    pub parameters: Vec<(String, Type)>,
    /// Universe level
    pub universe_level: u32,
    /// Point constructors
    pub point_constructors: Vec<PointConstructor>,
    /// Path constructors  
    pub path_constructors: Vec<PathConstructor>,
    /// Higher constructors (2-paths, 3-paths, etc.)
    pub higher_constructors: Vec<HigherConstructor>,
}

/// Point constructor: introduces elements of the type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointConstructor {
    /// Constructor name
    pub name: String,
    /// Argument types
    pub args: Vec<Type>,
    /// Result type (usually the HIT itself)
    pub result: Type,
}

/// Path constructor: introduces paths between elements
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PathConstructor {
    /// Constructor name
    pub name: String,
    /// Argument types (for dependent paths)
    pub args: Vec<Type>,
    /// Left endpoint
    pub left: Term,
    /// Right endpoint
    pub right: Term,
    /// Path type
    pub path_type: Type,
}

/// Higher constructor: introduces higher-dimensional paths
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HigherConstructor {
    /// Constructor name
    pub name: String,
    /// Dimension of the constructor
    pub dimension: usize,
    /// Argument types
    pub args: Vec<Type>,
    /// Boundary specification
    pub boundary: BTreeMap<Face, Term>,
    /// Result type
    pub result: Type,
}

/// HIT elimination rule (pattern matching principle)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HITElimination {
    /// Target type for elimination
    pub target: Type,
    /// Point constructor cases
    pub point_cases: BTreeMap<String, Term>,
    /// Path constructor cases (transport/ap)
    pub path_cases: BTreeMap<String, Term>,
    /// Higher constructor cases
    pub higher_cases: BTreeMap<String, Term>,
}

/// Truncation levels for HITs
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TruncationLevel {
    /// (-2)-truncated: contractible types
    MinusTwo,
    /// (-1)-truncated: propositions  
    MinusOne,
    /// 0-truncated: sets
    Zero,
    /// n-truncated for n ≥ 1
    Positive(u32),
    /// ∞-truncated: no truncation
    Infinity,
}

impl HIT {
    /// Create a new HIT definition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Vec::new(),
            universe_level: 0,
            point_constructors: Vec::new(),
            path_constructors: Vec::new(),
            higher_constructors: Vec::new(),
        }
    }
    
    /// Add a point constructor
    pub fn add_point_constructor(&mut self, name: impl Into<String>, args: Vec<Type>, result: Type) {
        self.point_constructors.push(PointConstructor {
            name: name.into(),
            args,
            result,
        });
    }
    
    /// Add a path constructor
    pub fn add_path_constructor(
        &mut self,
        name: impl Into<String>,
        args: Vec<Type>,
        left: Term,
        right: Term,
        path_type: Type,
    ) {
        self.path_constructors.push(PathConstructor {
            name: name.into(),
            args,
            left,
            right,
            path_type,
        });
    }
    
    /// Add a higher constructor
    pub fn add_higher_constructor(
        &mut self,
        name: impl Into<String>,
        dimension: usize,
        args: Vec<Type>,
        boundary: BTreeMap<Face, Term>,
        result: Type,
    ) {
        self.higher_constructors.push(HigherConstructor {
            name: name.into(),
            dimension,
            args,
            boundary,
            result,
        });
    }
    
    /// Check if HIT is well-formed
    pub fn is_well_formed(&self, context: &Context) -> Result<bool> {
        // Check that all constructors are well-typed
        for constructor in &self.point_constructors {
            if !self.point_constructor_well_typed(constructor, context)? {
                return Ok(false);
            }
        }
        
        for constructor in &self.path_constructors {
            if !self.path_constructor_well_typed(constructor, context)? {
                return Ok(false);
            }
        }
        
        for constructor in &self.higher_constructors {
            if !self.higher_constructor_well_typed(constructor, context)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    fn point_constructor_well_typed(&self, constructor: &PointConstructor, context: &Context) -> Result<bool> {
        // Check that arguments are well-typed and result matches HIT
        // TODO: Implement proper type checking
        Ok(true)
    }
    
    fn path_constructor_well_typed(&self, constructor: &PathConstructor, context: &Context) -> Result<bool> {
        // Check that endpoints have the right type and path type is correct
        // TODO: Implement proper type checking
        Ok(true)
    }
    
    fn higher_constructor_well_typed(&self, constructor: &HigherConstructor, context: &Context) -> Result<bool> {
        // Check that boundary conditions are satisfied
        // TODO: Implement proper type checking
        Ok(true)
    }
    
    /// Generate elimination principle for this HIT
    pub fn elimination_principle(&self) -> HITElimination {
        let target = Type::var("P"); // Placeholder
        
        HITElimination {
            target,
            point_cases: BTreeMap::new(), // TODO: Generate from constructors
            path_cases: BTreeMap::new(),
            higher_cases: BTreeMap::new(),
        }
    }
}

/// Standard HITs
impl HIT {
    /// Circle S¹
    pub fn circle() -> Self {
        let mut circle = HIT::new("S¹");
        
        // base : S¹
        circle.add_point_constructor("base", vec![], Type::var("S¹"));
        
        // loop : base = base
        circle.add_path_constructor(
            "loop",
            vec![],
            Term::var("base"),
            Term::var("base"),
            Type::path(Type::var("S¹"), Term::var("base"), Term::var("base")),
        );
        
        circle
    }
    
    /// Sphere S²
    pub fn sphere() -> Self {
        let mut sphere = HIT::new("S²");
        
        // base : S²
        sphere.add_point_constructor("base", vec![], Type::var("S²"));
        
        // surf : refl base = refl base (2-dimensional)
        let mut boundary = BTreeMap::new();
        boundary.insert(Face::eq0("i"), Term::var("refl_base"));
        boundary.insert(Face::eq1("i"), Term::var("refl_base"));
        boundary.insert(Face::eq0("j"), Term::var("refl_base"));
        boundary.insert(Face::eq1("j"), Term::var("refl_base"));
        
        sphere.add_higher_constructor(
            "surf",
            2,
            vec![],
            boundary,
            Type::var("S²_2path"), // Placeholder for 2-path type
        );
        
        sphere
    }
    
    /// Suspension Susp A of a type A
    pub fn suspension(a: Type) -> Self {
        let mut susp = HIT::new("Susp");
        susp.parameters.push(("A".to_string(), a.clone()));
        
        // north : Susp A
        susp.add_point_constructor("north", vec![], Type::var("Susp_A"));
        
        // south : Susp A  
        susp.add_point_constructor("south", vec![], Type::var("Susp_A"));
        
        // merid : (a : A) → north = south
        susp.add_path_constructor(
            "merid",
            vec![a],
            Term::var("north"),
            Term::var("south"),
            Type::path(Type::var("Susp_A"), Term::var("north"), Term::var("south")),
        );
        
        susp
    }
    
    /// Pushout of a span A ← C → B
    pub fn pushout(a: Type, b: Type, c: Type, f: Term, g: Term) -> Self {
        let mut pushout = HIT::new("Pushout");
        pushout.parameters.extend(vec![
            ("A".to_string(), a.clone()),
            ("B".to_string(), b.clone()),
            ("C".to_string(), c.clone()),
        ]);
        
        // inl : A → Pushout
        pushout.add_point_constructor("inl", vec![a], Type::var("Pushout"));
        
        // inr : B → Pushout
        pushout.add_point_constructor("inr", vec![b], Type::var("Pushout"));
        
        // glue : (c : C) → inl (f c) = inr (g c)
        pushout.add_path_constructor(
            "glue",
            vec![c],
            Term::app(Term::var("inl"), Term::app(f, Term::var("c"))),
            Term::app(Term::var("inr"), Term::app(g, Term::var("c"))),
            Type::path(
                Type::var("Pushout"),
                Term::app(Term::var("inl"), Term::app(f, Term::var("c"))),
                Term::app(Term::var("inr"), Term::app(g, Term::var("c"))),
            ),
        );
        
        pushout
    }
    
    /// Quotient type A / R for an equivalence relation R
    pub fn quotient(a: Type, r: Term) -> Self {
        let mut quotient = HIT::new("Quotient");
        quotient.parameters.extend(vec![
            ("A".to_string(), a.clone()),
            ("R".to_string(), Type::pi("x", a.clone(), Type::pi("y", a, Type::universe(0)))),
        ]);
        
        // class : A → A/R
        quotient.add_point_constructor("class", vec![a.clone()], Type::var("Quotient"));
        
        // relate : (x y : A) → R x y → class x = class y
        quotient.add_path_constructor(
            "relate",
            vec![a.clone(), a, Type::var("R_x_y")],
            Term::app(Term::var("class"), Term::var("x")),
            Term::app(Term::var("class"), Term::var("y")),
            Type::path(
                Type::var("Quotient"),
                Term::app(Term::var("class"), Term::var("x")),
                Term::app(Term::var("class"), Term::var("y")),
            ),
        );
        
        quotient
    }
    
    /// n-truncation of a type A
    pub fn truncation(a: Type, level: TruncationLevel) -> Self {
        let mut trunc = HIT::new("Trunc");
        trunc.parameters.push(("A".to_string(), a.clone()));
        
        // inc : A → ∥A∥_n
        trunc.add_point_constructor("inc", vec![a], Type::var("Trunc_A"));
        
        // Add truncation conditions based on level
        match level {
            TruncationLevel::MinusTwo => {
                // All elements are equal
                trunc.add_path_constructor(
                    "contr",
                    vec![Type::var("Trunc_A"), Type::var("Trunc_A")],
                    Term::var("x"),
                    Term::var("y"),
                    Type::path(Type::var("Trunc_A"), Term::var("x"), Term::var("y")),
                );
            }
            TruncationLevel::MinusOne => {
                // All paths are equal (propositions)
                let path_type = Type::path(Type::var("Trunc_A"), Term::var("x"), Term::var("y"));
                trunc.add_path_constructor(
                    "prop",
                    vec![path_type.clone(), path_type],
                    Term::var("p"),
                    Term::var("q"),
                    Type::path(path_type, Term::var("p"), Term::var("q")),
                );
            }
            TruncationLevel::Zero => {
                // All 2-paths are equal (sets)
                let mut boundary = BTreeMap::new();
                // Complex boundary conditions for set truncation
                trunc.add_higher_constructor("set", 2, vec![], boundary, Type::var("set_2path"));
            }
            _ => {} // Higher truncations require more complex higher constructors
        }
        
        trunc
    }
}

/// HIT operations
pub trait HITOps {
    /// Construct a HIT element using a constructor
    fn construct(&self, hit: &HIT, constructor: &str, args: &[Term], context: &Context) -> Result<Term>;
    
    /// Eliminate from a HIT using the elimination principle
    fn eliminate(&self, hit: &HIT, elimination: &HITElimination, element: &Term, context: &Context) -> Result<Term>;
    
    /// Check if a HIT has a given truncation level
    fn has_truncation_level(&self, hit: &HIT, level: &TruncationLevel, context: &Context) -> Result<bool>;
    
    /// Compute the fundamental group of a HIT
    fn fundamental_group(&self, hit: &HIT, basepoint: &Term, context: &Context) -> Result<Type>;
}

/// Default HIT operations
#[derive(Clone, Debug)]
pub struct DefaultHITOps;

impl HITOps for DefaultHITOps {
    fn construct(&self, hit: &HIT, constructor: &str, args: &[Term], context: &Context) -> Result<Term> {
        // Find the constructor
        for point_cons in &hit.point_constructors {
            if point_cons.name == constructor {
                if args.len() != point_cons.args.len() {
                    return Err(Error::HITError(format!(
                        "Constructor {} expects {} arguments, got {}",
                        constructor, point_cons.args.len(), args.len()
                    )));
                }
                
                return Ok(Term::HITConstructor {
                    name: constructor.to_string(),
                    args: args.to_vec(),
                });
            }
        }
        
        // Check path constructors
        for path_cons in &hit.path_constructors {
            if path_cons.name == constructor {
                if args.len() != path_cons.args.len() {
                    return Err(Error::HITError(format!(
                        "Path constructor {} expects {} arguments, got {}",
                        constructor, path_cons.args.len(), args.len()
                    )));
                }
                
                return Ok(Term::HITConstructor {
                    name: constructor.to_string(),
                    args: args.to_vec(),
                });
            }
        }
        
        Err(Error::HITError(format!("Constructor {} not found in HIT {}", constructor, hit.name)))
    }
    
    fn eliminate(&self, hit: &HIT, elimination: &HITElimination, element: &Term, context: &Context) -> Result<Term> {
        match element {
            Term::HITConstructor { name, args } => {
                // Apply the appropriate elimination case
                if let Some(case) = elimination.point_cases.get(name) {
                    // Apply case to arguments
                    Ok(args.iter().fold(case.clone(), |acc, arg| {
                        Term::app(acc, arg.clone())
                    }))
                } else if let Some(case) = elimination.path_cases.get(name) {
                    // Path elimination involves transport
                    Ok(args.iter().fold(case.clone(), |acc, arg| {
                        Term::app(acc, arg.clone())
                    }))
                } else {
                    Err(Error::HITError(format!("No elimination case for constructor {}", name)))
                }
            }
            _ => Err(Error::HITError("Can only eliminate HIT constructors".into())),
        }
    }
    
    fn has_truncation_level(&self, hit: &HIT, level: &TruncationLevel, context: &Context) -> Result<bool> {
        // Check if HIT satisfies the truncation conditions
        match level {
            TruncationLevel::MinusTwo => {
                // Check if all elements are equal
                self.check_contractible(hit, context)
            }
            TruncationLevel::MinusOne => {
                // Check if all paths are equal
                self.check_proposition(hit, context)
            }
            TruncationLevel::Zero => {
                // Check if all 2-paths are equal
                self.check_set(hit, context)
            }
            _ => Ok(false), // Higher truncations not implemented
        }
    }
    
    fn fundamental_group(&self, hit: &HIT, basepoint: &Term, context: &Context) -> Result<Type> {
        // Compute π₁(HIT, basepoint)
        // This is the type of loops at the basepoint up to homotopy
        
        let loop_space = Type::path(
            Type::HigherInductive {
                name: hit.name.clone(),
                params: hit.parameters.iter().map(|(_, t)| t.clone()).collect(),
                constructors: vec![], // Simplified
            },
            basepoint.clone(),
            basepoint.clone(),
        );
        
        // The fundamental group is the 0-truncation of the loop space
        Ok(Type::HigherInductive {
            name: "π₁".to_string(),
            params: vec![loop_space],
            constructors: vec![], // Would need truncation constructors
        })
    }
}

impl DefaultHITOps {
    fn check_contractible(&self, hit: &HIT, context: &Context) -> Result<bool> {
        // Check if there's a constructor making all elements equal
        for path_cons in &hit.path_constructors {
            // Look for paths that connect arbitrary elements
            // This is a simplified check
        }
        Ok(false) // Conservative
    }
    
    fn check_proposition(&self, hit: &HIT, context: &Context) -> Result<bool> {
        // Check if all paths between any two elements are equal
        for higher_cons in &hit.higher_constructors {
            if higher_cons.dimension == 2 {
                // Check if this provides path equality
                // Simplified check
            }
        }
        Ok(false) // Conservative
    }
    
    fn check_set(&self, hit: &HIT, context: &Context) -> Result<bool> {
        // Check if all 2-paths are equal
        for higher_cons in &hit.higher_constructors {
            if higher_cons.dimension == 3 {
                // Check if this provides 2-path equality
            }
        }
        Ok(false) // Conservative
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circle_hit() {
        let circle = HIT::circle();
        assert_eq!(circle.name, "S¹");
        assert_eq!(circle.point_constructors.len(), 1);
        assert_eq!(circle.path_constructors.len(), 1);
        assert_eq!(circle.point_constructors[0].name, "base");
        assert_eq!(circle.path_constructors[0].name, "loop");
    }
    
    #[test]
    fn test_sphere_hit() {
        let sphere = HIT::sphere();
        assert_eq!(sphere.name, "S²");
        assert_eq!(sphere.point_constructors.len(), 1);
        assert_eq!(sphere.higher_constructors.len(), 1);
        assert_eq!(sphere.higher_constructors[0].dimension, 2);
    }
    
    #[test]
    fn test_suspension_hit() {
        let a = Type::var("A");
        let susp = HIT::suspension(a);
        assert_eq!(susp.name, "Susp");
        assert_eq!(susp.point_constructors.len(), 2);
        assert_eq!(susp.path_constructors.len(), 1);
        assert_eq!(susp.point_constructors[0].name, "north");
        assert_eq!(susp.point_constructors[1].name, "south");
    }
    
    #[test]
    fn test_quotient_hit() {
        let a = Type::var("A");
        let r = Term::var("R");
        let quotient = HIT::quotient(a, r);
        assert_eq!(quotient.name, "Quotient");
        assert_eq!(quotient.point_constructors.len(), 1);
        assert_eq!(quotient.path_constructors.len(), 1);
        assert_eq!(quotient.point_constructors[0].name, "class");
        assert_eq!(quotient.path_constructors[0].name, "relate");
    }
    
    #[test]
    fn test_hit_construction() {
        let ops = DefaultHITOps;
        let circle = HIT::circle();
        let ctx = Context::new();
        
        // Construct base point
        let base = ops.construct(&circle, "base", &[], &ctx).unwrap();
        match base {
            Term::HITConstructor { name, args } => {
                assert_eq!(name, "base");
                assert!(args.is_empty());
            }
            _ => panic!("Expected HIT constructor"),
        }
    }
    
    #[test]
    fn test_truncation_levels() {
        assert!(TruncationLevel::MinusTwo < TruncationLevel::MinusOne);
        assert!(TruncationLevel::MinusOne < TruncationLevel::Zero);
        assert!(TruncationLevel::Zero < TruncationLevel::Positive(1));
        assert!(TruncationLevel::Positive(1) < TruncationLevel::Infinity);
    }
}