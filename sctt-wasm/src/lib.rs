//! # WebAssembly Bindings for SCTT
//!
//! This crate provides WebAssembly bindings for SCTT, enabling
//! type checking and theorem proving in web browsers.

use wasm_bindgen::prelude::*;
use sctt_core::prelude::*;
use sctt_checker::TypeChecker;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize WASM module
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// SCTT Type Checker for JavaScript
#[wasm_bindgen]
pub struct SCTTChecker {
    checker: TypeChecker,
}

#[wasm_bindgen]
impl SCTTChecker {
    /// Create new type checker
    #[wasm_bindgen(constructor)]
    pub fn new() -> SCTTChecker {
        SCTTChecker {
            checker: TypeChecker::new(),
        }
    }
    
    /// Type check a term (JSON input/output)
    #[wasm_bindgen]
    pub fn check_term(&mut self, term_json: &str, type_json: &str) -> String {
        match self.check_term_internal(term_json, type_json) {
            Ok(result) => serde_json::to_string(&result).unwrap_or_else(|_| "null".to_string()),
            Err(err) => format!(r#"{{"error": "{}"}}"#, err),
        }
    }
    
    fn check_term_internal(&mut self, term_json: &str, type_json: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let term: Term = serde_json::from_str(term_json)?;
        let typ: Type = serde_json::from_str(type_json)?;
        let context = Context::new();
        
        self.checker.check_type(&term, &typ, &context)?;
        Ok(true)
    }
    
    /// Infer type of a term
    #[wasm_bindgen]
    pub fn infer_type(&mut self, term_json: &str) -> String {
        match self.infer_type_internal(term_json) {
            Ok(typ) => serde_json::to_string(&typ).unwrap_or_else(|_| "null".to_string()),
            Err(err) => format!(r#"{{"error": "{}"}}"#, err),
        }
    }
    
    fn infer_type_internal(&mut self, term_json: &str) -> Result<Type, Box<dyn std::error::Error>> {
        let term: Term = serde_json::from_str(term_json)?;
        let context = Context::new();
        
        let typ = self.checker.infer_type(&term, &context)?;
        Ok(typ)
    }
    
    /// Normalize a term
    #[wasm_bindgen]
    pub fn normalize(&self, term_json: &str) -> String {
        match self.normalize_internal(term_json) {
            Ok(normalized) => serde_json::to_string(&normalized).unwrap_or_else(|_| "null".to_string()),
            Err(err) => format!(r#"{{"error": "{}"}}"#, err),
        }
    }
    
    fn normalize_internal(&self, term_json: &str) -> Result<Term, Box<dyn std::error::Error>> {
        let term: Term = serde_json::from_str(term_json)?;
        let normalized = self.checker.normalize_term(&term)?;
        Ok(normalized)
    }
}

/// JavaScript utility functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

/// Logging macro for WASM
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Test function for JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}! Welcome to SCTT.", name);
}

/// Create example terms for testing
#[wasm_bindgen]
pub fn create_example_term() -> String {
    let term = Term::lambda("x", Term::var("x"));
    serde_json::to_string(&term).unwrap_or_else(|_| "null".to_string())
}

/// Create example types for testing
#[wasm_bindgen]
pub fn create_example_type() -> String {
    let typ = Type::pi("x", Type::universe(0), Type::universe(0));
    serde_json::to_string(&typ).unwrap_or_else(|_| "null".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_checker_creation() {
        let _checker = SCTTChecker::new();
    }
    
    #[wasm_bindgen_test]
    fn test_example_creation() {
        let term_json = create_example_term();
        assert!(!term_json.is_empty());
        
        let type_json = create_example_type();
        assert!(!type_json.is_empty());
    }
}