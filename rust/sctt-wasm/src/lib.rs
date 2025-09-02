use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct SCTTEngine {
    checker: sctt_checker::Checker,
}

#[wasm_bindgen]
impl SCTTEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            checker: sctt_checker::Checker::new(),
        }
    }

    #[wasm_bindgen]
    pub fn typecheck(&self, code: &str) -> String {
        let result = self.checker.check(code);
        serde_json::to_string(&result).unwrap_or_else(|_| "error".to_string())
    }

    #[wasm_bindgen]
    pub fn evaluate(&self, expr: &str, value: f64) -> f64 {
        // Simple evaluation for demo
        if expr.contains("sin") {
            value.sin()
        } else if expr.contains("cos") {
            value.cos()
        } else {
            value
        }
    }
}

// WASI entry point for Wasmer
#[cfg(target_os = "wasi")]
fn main() {
    println!("SCTT Engine running on WASI");
}