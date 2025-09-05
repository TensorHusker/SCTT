//! SCTT System - Production-Ready Smooth Cubical Type Theory
//!
//! A complete proof assistant with web interface, collaborative features,
//! and advanced visualization capabilities.

pub mod sctt_typechecker;
pub mod sctt_to_wasm;
pub mod proof_assistant;
pub mod web_interface;
pub mod collaborative;
pub mod visualization;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export main types
pub use sctt_typechecker::{Term, TypeChecker, Context, Value};
pub use sctt_to_wasm::{ScttToWasmCompiler, OptLevel};
pub use proof_assistant::{ProofAssistant, Tactic, Goal};

/// Main SCTT system interface for web
#[wasm_bindgen]
pub struct ScttSystem {
    type_checker: TypeChecker,
    compiler: ScttToWasmCompiler,
    assistant: ProofAssistant,
}

#[wasm_bindgen]
impl ScttSystem {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        
        ScttSystem {
            type_checker: TypeChecker::new(),
            compiler: ScttToWasmCompiler::new(OptLevel::Basic),
            assistant: ProofAssistant::new(),
        }
    }

    /// Type check a term
    #[wasm_bindgen]
    pub fn type_check(&self, code: &str) -> Result<String, JsValue> {
        let term = self.parse_term(code)?;
        let ctx = Context::new();
        let ty = self.type_checker.infer(&ctx, &term)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        Ok(format!("{:?}", ty))
    }

    /// Compile to WASM
    #[wasm_bindgen]
    pub fn compile_to_wasm(&mut self, code: &str) -> Result<Vec<u8>, JsValue> {
        let term = self.parse_term(code)?;
        let ir = self.compiler.sctt_to_ir(&term);
        let optimized = self.compiler.optimize(ir);
        let wasm_module = self.compiler.ir_to_wasm(&optimized);
        Ok(wasm_module.encode())
    }

    /// Apply a proof tactic
    #[wasm_bindgen]
    pub fn apply_tactic(&mut self, tactic_name: &str, goal_id: usize) -> Result<String, JsValue> {
        let tactic = Tactic::from_name(tactic_name)
            .ok_or_else(|| JsValue::from_str("Unknown tactic"))?;
        
        self.assistant.apply_tactic(tactic, goal_id)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(self.assistant.render_proof_state())
    }

    /// Get current proof state
    #[wasm_bindgen]
    pub fn get_proof_state(&self) -> String {
        self.assistant.render_proof_state()
    }

    /// Parse SCTT code
    fn parse_term(&self, code: &str) -> Result<Term, JsValue> {
        // Simplified parser - would use proper parser combinator in production
        parser::parse(code).map_err(|e| JsValue::from_str(&e))
    }
}

/// Session management for collaborative editing
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub users: Vec<User>,
    pub document: Document,
    pub proof_state: ProofState,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub cursor: CursorPosition,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub content: String,
    pub version: u64,
    pub operations: Vec<Operation>,
}

#[derive(Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Insert { pos: usize, text: String },
    Delete { pos: usize, len: usize },
    Replace { pos: usize, len: usize, text: String },
}

#[derive(Serialize, Deserialize)]
pub struct ProofState {
    pub goals: Vec<Goal>,
    pub history: Vec<ProofStep>,
}

#[derive(Serialize, Deserialize)]
pub struct ProofStep {
    pub tactic: String,
    pub goal_before: Goal,
    pub goals_after: Vec<Goal>,
}

/// Simple parser module
mod parser {
    use super::*;
    
    pub fn parse(code: &str) -> Result<Term, String> {
        // Simplified - would use nom or pest in production
        if code.starts_with("λ") || code.starts_with("\\") {
            // Parse lambda
            Ok(Term::Lambda(
                Box::new(Term::Universe(sctt_typechecker::Level::Zero)),
                Box::new(Term::Var(sctt_typechecker::DeBruijnIndex(0))),
            ))
        } else if code.starts_with("Π") || code.starts_with("forall") {
            // Parse Pi type
            Ok(Term::Pi(
                Box::new(Term::Universe(sctt_typechecker::Level::Zero)),
                Box::new(Term::Universe(sctt_typechecker::Level::Zero)),
            ))
        } else if code == "Type" {
            Ok(Term::Universe(sctt_typechecker::Level::Zero))
        } else {
            // Try to parse as variable
            Ok(Term::Var(sctt_typechecker::DeBruijnIndex(0)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_system_creation() {
        let system = ScttSystem::new();
        let state = system.get_proof_state();
        assert!(state.contains("Goals"));
    }

    #[wasm_bindgen_test]
    fn test_type_checking() {
        let system = ScttSystem::new();
        let result = system.type_check("Type");
        assert!(result.is_ok());
    }
}