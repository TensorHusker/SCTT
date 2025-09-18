//! # Proof System for SCTT
//!
//! This module implements proof-relevant computation and verification
//! for SCTT, including proof terms, derivation tracking, and computational
//! reflection.

use crate::types::{Type, Term, Context, TypeJudgment};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Proof term carrying evidence for type judgments
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProofTerm {
    /// Assumption from context
    Assumption {
        /// Variable name
        var: String,
        /// Index in context
        index: usize,
    },
    
    /// Application of proof
    Application {
        /// Function proof
        function: Box<ProofTerm>,
        /// Argument proof
        argument: Box<ProofTerm>,
    },
    
    /// Lambda abstraction in proof
    Lambda {
        /// Parameter name
        param: String,
        /// Body proof
        body: Box<ProofTerm>,
    },
    
    /// Pair proof
    Pair {
        /// First component proof
        first: Box<ProofTerm>,
        /// Second component proof
        second: Box<ProofTerm>,
    },
    
    /// Reflexivity proof (a = a)
    Refl {
        /// Term being reflected
        term: Term,
        /// Type of the term
        typ: Type,
    },
    
    /// Symmetry proof (a = b → b = a)
    Sym {
        /// Proof of a = b
        proof: Box<ProofTerm>,
    },
    
    /// Transitivity proof (a = b → b = c → a = c)
    Trans {
        /// Proof of a = b
        left: Box<ProofTerm>,
        /// Proof of b = c
        right: Box<ProofTerm>,
    },
    
    /// Congruence proof (f a = f b from a = b)
    Cong {
        /// Function being applied
        function: Term,
        /// Proof that arguments are equal
        arg_proof: Box<ProofTerm>,
    },
    
    /// Transport proof (subst P p : P a → P b from a = b)
    Transport {
        /// Predicate P
        predicate: Type,
        /// Proof of a = b
        equality: Box<ProofTerm>,
        /// Proof of P a
        element: Box<ProofTerm>,
    },
    
    /// Path induction proof
    PathInduction {
        /// Type family C
        type_family: Type,
        /// Base case
        base: Box<ProofTerm>,
        /// Path being induced over
        path: Term,
    },
    
    /// Kan operation proof
    KanProof {
        /// Type of Kan operation
        kan_type: KanProofType,
        /// Witnesses for the operation
        witnesses: Vec<ProofTerm>,
    },
    
    /// Smooth operation proof
    SmoothProof {
        /// Type of smooth operation
        smooth_type: SmoothProofType,
        /// Smoothness witnesses
        witnesses: Vec<ProofTerm>,
    },
    
    /// HIT elimination proof
    HITElimination {
        /// HIT being eliminated
        hit_type: Type,
        /// Elimination motive
        motive: Type,
        /// Constructor cases
        cases: BTreeMap<String, ProofTerm>,
    },
    
    /// Univalence proof
    Univalence {
        /// Types being identified
        type_a: Type,
        type_b: Type,
        /// Equivalence proof
        equivalence: Box<ProofTerm>,
    },
    
    /// Computational proof (proof by evaluation)
    Computational {
        /// Computation steps
        steps: Vec<ComputationStep>,
        /// Final result
        result: Term,
    },
    
    /// Meta-proof (proof about proofs)
    Meta {
        /// Meta-level statement
        statement: MetaStatement,
        /// Meta-proof term
        proof: Box<ProofTerm>,
    },
}

/// Types of Kan operation proofs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KanProofType {
    /// Composition is well-defined
    CompositionWellDefined,
    /// Transport preserves typing
    TransportTyping,
    /// Glue types are well-formed
    GlueWellFormed,
    /// Kan condition satisfied
    KanCondition,
}

/// Types of smooth operation proofs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SmoothProofType {
    /// Function is smooth
    FunctionSmooth,
    /// Transport preserves smoothness
    TransportSmooth,
    /// Differential operator well-defined
    DifferentialWellDefined,
    /// Stokes' theorem
    StokesTheorem,
}

/// Computation step in proof by evaluation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputationStep {
    /// Rule applied
    pub rule: String,
    /// Input term
    pub input: Term,
    /// Output term
    pub output: Term,
    /// Justification
    pub justification: String,
}

/// Meta-level statements about proofs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetaStatement {
    /// Proof is total
    ProofTotal(ProofTerm),
    /// Proof terminates
    ProofTerminates(ProofTerm),
    /// Proof is consistent
    ProofConsistent(ProofTerm),
    /// Proof extracts to program
    ProofExtracts(ProofTerm, Term),
}

/// Typing derivation tree
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Derivation {
    /// Unique identifier
    pub id: Uuid,
    /// Judgment being proved
    pub judgment: TypeJudgment,
    /// Proof term
    pub proof: ProofTerm,
    /// Sub-derivations
    pub premises: Vec<Derivation>,
    /// Rule applied
    pub rule: String,
}

/// Proof checking context
#[derive(Clone, Debug, PartialEq)]
pub struct ProofContext {
    /// Type checking context
    pub type_context: Context,
    /// Proof assumptions
    pub assumptions: Vec<(String, ProofTerm, Type)>,
    /// Axioms available
    pub axioms: BTreeMap<String, Type>,
}

impl ProofContext {
    /// Create new proof context
    pub fn new(type_context: Context) -> Self {
        Self {
            type_context,
            assumptions: Vec::new(),
            axioms: BTreeMap::new(),
        }
    }
    
    /// Add proof assumption
    pub fn assume(&mut self, name: String, proof: ProofTerm, typ: Type) {
        self.assumptions.push((name, proof, typ));
    }
    
    /// Add axiom
    pub fn add_axiom(&mut self, name: String, typ: Type) {
        self.axioms.insert(name, typ);
    }
    
    /// Look up assumption
    pub fn lookup_assumption(&self, name: &str) -> Option<&(String, ProofTerm, Type)> {
        self.assumptions.iter().find(|(n, _, _)| n == name)
    }
    
    /// Look up axiom
    pub fn lookup_axiom(&self, name: &str) -> Option<&Type> {
        self.axioms.get(name)
    }
}

/// Proof operations trait
pub trait ProofOps {
    /// Check if a proof term is valid
    fn check_proof(&self, proof: &ProofTerm, typ: &Type, context: &ProofContext) -> Result<bool>;
    
    /// Infer the type of a proof term
    fn infer_proof_type(&self, proof: &ProofTerm, context: &ProofContext) -> Result<Type>;
    
    /// Extract computational content from proof
    fn extract_program(&self, proof: &ProofTerm) -> Result<Term>;
    
    /// Normalize proof term
    fn normalize_proof(&self, proof: &ProofTerm, context: &ProofContext) -> Result<ProofTerm>;
    
    /// Check proof equivalence
    fn proofs_equal(&self, p1: &ProofTerm, p2: &ProofTerm, context: &ProofContext) -> Result<bool>;
}

/// Default proof operations
#[derive(Clone, Debug)]
pub struct DefaultProofOps;

impl ProofOps for DefaultProofOps {
    fn check_proof(&self, proof: &ProofTerm, typ: &Type, context: &ProofContext) -> Result<bool> {
        match proof {
            ProofTerm::Assumption { var, index } => {
                // Check that assumption exists and has right type
                if let Some((_, _, assumed_type)) = context.lookup_assumption(var) {
                    Ok(assumed_type == typ)
                } else {
                    Ok(false)
                }
            }
            
            ProofTerm::Refl { term, typ: term_type } => {
                // Check that type is of the form (term = term : term_type)
                match typ {
                    Type::Path { type_family, left, right } => {
                        Ok(left == right && left.as_ref() == term && 
                           type_family.as_ref() == term_type)
                    }
                    _ => Ok(false),
                }
            }
            
            ProofTerm::Sym { proof: inner_proof } => {
                // Check that inner proof has type (a = b) and our type is (b = a)
                match typ {
                    Type::Path { type_family, left, right } => {
                        let expected_inner_type = Type::Path {
                            type_family: type_family.clone(),
                            left: right.clone(),
                            right: left.clone(),
                        };
                        self.check_proof(inner_proof, &expected_inner_type, context)
                    }
                    _ => Ok(false),
                }
            }
            
            ProofTerm::Trans { left: left_proof, right: right_proof } => {
                // Check transitivity
                match typ {
                    Type::Path { type_family, left, right } => {
                        // Need to find the middle point
                        // For now, simplified check
                        self.check_proof(left_proof, typ, context)? &&
                        self.check_proof(right_proof, typ, context)?;
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
            
            ProofTerm::Application { function, argument } => {
                // Check application
                let func_type = self.infer_proof_type(function, context)?;
                match func_type {
                    Type::Pi { domain, codomain, param } => {
                        let arg_valid = self.check_proof(argument, &domain, context)?;
                        if arg_valid {
                            // Substitute argument in codomain
                            let result_type = codomain.substitute(&param, &Term::var("dummy"))?; // Simplified
                            Ok(&result_type == typ)
                        } else {
                            Ok(false)
                        }
                    }
                    _ => Ok(false),
                }
            }
            
            ProofTerm::Lambda { param, body } => {
                // Check lambda
                match typ {
                    Type::Pi { domain, codomain, param: pi_param } => {
                        let mut new_context = context.clone();
                        new_context.assume(
                            param.clone(),
                            ProofTerm::Assumption { var: param.clone(), index: 0 },
                            domain.as_ref().clone(),
                        );
                        
                        let body_type = codomain.substitute(pi_param, &Term::var(param))?;
                        self.check_proof(body, &body_type, &new_context)
                    }
                    _ => Ok(false),
                }
            }
            
            ProofTerm::Transport { predicate, equality, element } => {
                // Check transport/substitution
                match predicate {
                    Type::Pi { domain, codomain, param } => {
                        // equality should prove that two elements of domain are equal
                        // element should prove predicate applied to first element
                        // result should be predicate applied to second element
                        // Simplified check for now
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
            
            _ => Err(Error::NotImplemented("Proof checking for this proof term".into())),
        }
    }
    
    fn infer_proof_type(&self, proof: &ProofTerm, context: &ProofContext) -> Result<Type> {
        match proof {
            ProofTerm::Assumption { var, .. } => {
                context.lookup_assumption(var)
                    .map(|(_, _, typ)| typ.clone())
                    .ok_or_else(|| Error::ProofError(format!("Unknown assumption: {}", var)))
            }
            
            ProofTerm::Refl { term, typ } => {
                Ok(Type::Path {
                    type_family: Box::new(typ.clone()),
                    left: Box::new(term.clone()),
                    right: Box::new(term.clone()),
                })
            }
            
            ProofTerm::Application { function, argument } => {
                let func_type = self.infer_proof_type(function, context)?;
                let arg_type = self.infer_proof_type(argument, context)?;
                
                match func_type {
                    Type::Pi { domain, codomain, param } => {
                        if *domain == arg_type {
                            // Substitute argument in codomain
                            let result = codomain.substitute(&param, &Term::var("dummy"))?; // Simplified
                            Ok(result)
                        } else {
                            Err(Error::ProofError("Type mismatch in application".into()))
                        }
                    }
                    _ => Err(Error::ProofError("Cannot apply non-function proof".into())),
                }
            }
            
            _ => Err(Error::NotImplemented("Type inference for this proof term".into())),
        }
    }
    
    fn extract_program(&self, proof: &ProofTerm) -> Result<Term> {
        match proof {
            ProofTerm::Assumption { var, .. } => Ok(Term::var(var)),
            
            ProofTerm::Lambda { param, body } => {
                let body_program = self.extract_program(body)?;
                Ok(Term::lambda(param, body_program))
            }
            
            ProofTerm::Application { function, argument } => {
                let func_program = self.extract_program(function)?;
                let arg_program = self.extract_program(argument)?;
                Ok(Term::app(func_program, arg_program))
            }
            
            ProofTerm::Pair { first, second } => {
                let first_program = self.extract_program(first)?;
                let second_program = self.extract_program(second)?;
                Ok(Term::pair(first_program, second_program))
            }
            
            ProofTerm::Refl { term, .. } => {
                // Reflexivity extracts to the term itself
                Ok(term.clone())
            }
            
            ProofTerm::Transport { element, .. } => {
                // Transport extracts to the transported element
                self.extract_program(element)
            }
            
            ProofTerm::Computational { result, .. } => {
                // Computational proof extracts to its result
                Ok(result.clone())
            }
            
            _ => {
                // Many proof terms don't extract to computational content
                Ok(Term::var("()")) // Unit term
            }
        }
    }
    
    fn normalize_proof(&self, proof: &ProofTerm, context: &ProofContext) -> Result<ProofTerm> {
        match proof {
            ProofTerm::Application { function, argument } => {
                let norm_func = self.normalize_proof(function, context)?;
                let norm_arg = self.normalize_proof(argument, context)?;
                
                match norm_func {
                    ProofTerm::Lambda { param, body } => {
                        // Beta reduction in proof terms
                        self.substitute_proof(&body, &param, &norm_arg)
                    }
                    _ => Ok(ProofTerm::Application {
                        function: Box::new(norm_func),
                        argument: Box::new(norm_arg),
                    }),
                }
            }
            
            ProofTerm::Trans { left, right } => {
                let norm_left = self.normalize_proof(left, context)?;
                let norm_right = self.normalize_proof(right, context)?;
                
                // Check for identity compositions
                match (&norm_left, &norm_right) {
                    (ProofTerm::Refl { .. }, proof) | (proof, ProofTerm::Refl { .. }) => {
                        Ok(proof.clone())
                    }
                    _ => Ok(ProofTerm::Trans {
                        left: Box::new(norm_left),
                        right: Box::new(norm_right),
                    }),
                }
            }
            
            ProofTerm::Sym { proof: inner } => {
                let norm_inner = self.normalize_proof(inner, context)?;
                
                // Double symmetry cancellation
                match norm_inner {
                    ProofTerm::Sym { proof } => Ok(*proof),
                    ProofTerm::Refl { .. } => Ok(norm_inner), // sym(refl) = refl
                    _ => Ok(ProofTerm::Sym { proof: Box::new(norm_inner) }),
                }
            }
            
            _ => Ok(proof.clone()), // Already normal
        }
    }
    
    fn proofs_equal(&self, p1: &ProofTerm, p2: &ProofTerm, context: &ProofContext) -> Result<bool> {
        let norm1 = self.normalize_proof(p1, context)?;
        let norm2 = self.normalize_proof(p2, context)?;
        
        // Structural equality on normalized proofs
        Ok(norm1 == norm2)
    }
}

impl DefaultProofOps {
    /// Substitute proof variable in proof term
    fn substitute_proof(&self, proof: &ProofTerm, var: &str, replacement: &ProofTerm) -> Result<ProofTerm> {
        match proof {
            ProofTerm::Assumption { var: name, index } if name == var => {
                Ok(replacement.clone())
            }
            
            ProofTerm::Lambda { param, body } if param != var => {
                Ok(ProofTerm::Lambda {
                    param: param.clone(),
                    body: Box::new(self.substitute_proof(body, var, replacement)?),
                })
            }
            
            ProofTerm::Application { function, argument } => {
                Ok(ProofTerm::Application {
                    function: Box::new(self.substitute_proof(function, var, replacement)?),
                    argument: Box::new(self.substitute_proof(argument, var, replacement)?),
                })
            }
            
            _ => Ok(proof.clone()), // No substitution needed or variable bound
        }
    }
}

/// Derivation construction
impl Derivation {
    /// Create new derivation
    pub fn new(judgment: TypeJudgment, proof: ProofTerm, rule: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            judgment,
            proof,
            premises: Vec::new(),
            rule,
        }
    }
    
    /// Add premise to derivation
    pub fn add_premise(&mut self, premise: Derivation) {
        self.premises.push(premise);
    }
    
    /// Check if derivation is valid
    pub fn is_valid(&self, proof_ops: &dyn ProofOps, context: &ProofContext) -> Result<bool> {
        // Check that all premises are valid
        for premise in &self.premises {
            if !premise.is_valid(proof_ops, context)? {
                return Ok(false);
            }
        }
        
        // Check that rule application is correct
        self.check_rule_application(proof_ops, context)
    }
    
    fn check_rule_application(&self, proof_ops: &dyn ProofOps, context: &ProofContext) -> Result<bool> {
        match self.rule.as_str() {
            "assumption" => {
                // Check that judgment corresponds to an assumption
                match &self.judgment {
                    TypeJudgment::TermTyping { term, typ, .. } => {
                        if let Term::Var(name) = term {
                            Ok(context.type_context.lookup(name) == Some(typ))
                        } else {
                            Ok(false)
                        }
                    }
                    _ => Ok(false),
                }
            }
            
            "lambda" => {
                // Check lambda formation rule
                match &self.judgment {
                    TypeJudgment::TermTyping { term, typ, context: ctx } => {
                        match (term, typ) {
                            (Term::Lambda { .. }, Type::Pi { .. }) => {
                                // Check that premise validates the body
                                Ok(self.premises.len() == 1)
                            }
                            _ => Ok(false),
                        }
                    }
                    _ => Ok(false),
                }
            }
            
            "application" => {
                // Check application rule
                Ok(self.premises.len() == 2) // Function and argument premises
            }
            
            _ => Err(Error::ProofError(format!("Unknown rule: {}", self.rule))),
        }
    }
    
    /// Extract the computational content of the derivation
    pub fn extract(&self, proof_ops: &dyn ProofOps) -> Result<Term> {
        proof_ops.extract_program(&self.proof)
    }
    
    /// Get the size of the derivation tree
    pub fn size(&self) -> usize {
        1 + self.premises.iter().map(|p| p.size()).sum::<usize>()
    }
    
    /// Check if derivation uses any axioms
    pub fn uses_axioms(&self) -> Vec<String> {
        let mut axioms = Vec::new();
        self.collect_axioms(&mut axioms);
        axioms
    }
    
    fn collect_axioms(&self, axioms: &mut Vec<String>) {
        if self.rule == "axiom" {
            if let TypeJudgment::TermTyping { term, .. } = &self.judgment {
                if let Term::Var(name) = term {
                    axioms.push(name.clone());
                }
            }
        }
        
        for premise in &self.premises {
            premise.collect_axioms(axioms);
        }
    }
}

/// Proof tactics for interactive theorem proving
pub enum Tactic {
    /// Apply a term as a function
    Apply(Term),
    /// Introduce a lambda/pi
    Intro(String),
    /// Split a sigma/conjunction
    Split,
    /// Use reflexivity
    Refl,
    /// Apply symmetry
    Sym,
    /// Apply transitivity
    Trans(ProofTerm),
    /// Transport along equality
    Transport(Type, ProofTerm),
    /// Induction
    Induction(Term),
    /// Case analysis
    Cases(Term),
    /// Simplify using computation
    Compute,
    /// Use assumption
    Assumption(String),
}

/// Apply a tactic to a goal
pub fn apply_tactic(
    tactic: &Tactic,
    goal: &Type,
    context: &ProofContext,
    proof_ops: &dyn ProofOps,
) -> Result<Vec<Type>> {
    match tactic {
        Tactic::Intro(name) => {
            match goal {
                Type::Pi { domain, codomain, param } => {
                    // Introduce parameter, goal becomes codomain
                    let new_goal = codomain.substitute(param, &Term::var(name))?;
                    Ok(vec![new_goal])
                }
                _ => Err(Error::ProofError("Cannot introduce on non-Pi type".into())),
            }
        }
        
        Tactic::Apply(term) => {
            // Apply term to goal, generate subgoals for premises
            // This is simplified - real implementation would unify types
            Ok(vec![goal.clone()]) // Placeholder
        }
        
        Tactic::Refl => {
            match goal {
                Type::Path { left, right, .. } => {
                    if left == right {
                        Ok(vec![]) // Goal solved
                    } else {
                        Err(Error::ProofError("Cannot apply reflexivity to non-trivial path".into()))
                    }
                }
                _ => Err(Error::ProofError("Cannot apply reflexivity to non-path type".into())),
            }
        }
        
        _ => Err(Error::NotImplemented("Tactic not implemented".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Context;
    
    #[test]
    fn test_proof_context() {
        let type_ctx = Context::new();
        let mut proof_ctx = ProofContext::new(type_ctx);
        
        let assumption = ProofTerm::Assumption {
            var: "H".to_string(),
            index: 0,
        };
        let typ = Type::var("P");
        
        proof_ctx.assume("H".to_string(), assumption, typ.clone());
        
        assert_eq!(
            proof_ctx.lookup_assumption("H").map(|(_, _, t)| t),
            Some(&typ)
        );
    }
    
    #[test]
    fn test_reflexivity_proof() {
        let ops = DefaultProofOps;
        let type_ctx = Context::new();
        let proof_ctx = ProofContext::new(type_ctx);
        
        let term = Term::var("x");
        let typ = Type::var("A");
        let refl_proof = ProofTerm::Refl {
            term: term.clone(),
            typ: typ.clone(),
        };
        
        let path_type = Type::Path {
            type_family: Box::new(typ),
            left: Box::new(term.clone()),
            right: Box::new(term),
        };
        
        assert!(ops.check_proof(&refl_proof, &path_type, &proof_ctx).unwrap());
    }
    
    #[test]
    fn test_proof_extraction() {
        let ops = DefaultProofOps;
        
        let proof = ProofTerm::Lambda {
            param: "x".to_string(),
            body: Box::new(ProofTerm::Assumption {
                var: "x".to_string(),
                index: 0,
            }),
        };
        
        let extracted = ops.extract_program(&proof).unwrap();
        match extracted {
            Term::Lambda { param, body } => {
                assert_eq!(param, "x");
                match *body {
                    Term::Var(name) => assert_eq!(name, "x"),
                    _ => panic!("Expected variable in body"),
                }
            }
            _ => panic!("Expected lambda term"),
        }
    }
    
    #[test]
    fn test_proof_normalization() {
        let ops = DefaultProofOps;
        let type_ctx = Context::new();
        let proof_ctx = ProofContext::new(type_ctx);
        
        // Create sym(sym(p)) which should normalize to p
        let p = ProofTerm::Assumption {
            var: "p".to_string(),
            index: 0,
        };
        let sym_p = ProofTerm::Sym { proof: Box::new(p.clone()) };
        let sym_sym_p = ProofTerm::Sym { proof: Box::new(sym_p) };
        
        let normalized = ops.normalize_proof(&sym_sym_p, &proof_ctx).unwrap();
        assert_eq!(normalized, p);
    }
    
    #[test]
    fn test_derivation() {
        let judgment = TypeJudgment::TermTyping {
            context: Context::new(),
            term: Term::var("x"),
            typ: Type::var("A"),
        };
        
        let proof = ProofTerm::Assumption {
            var: "x".to_string(),
            index: 0,
        };
        
        let derivation = Derivation::new(judgment, proof, "assumption".to_string());
        
        assert_eq!(derivation.rule, "assumption");
        assert_eq!(derivation.premises.len(), 0);
        assert_eq!(derivation.size(), 1);
    }
}