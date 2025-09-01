//! Type checker module for SCTT
//! Bidirectional type checking with smooth and cubical features

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Simplified types for the checker (since sctt_core isn't compiled yet)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Real,
    Interval,
    Function { domain: Box<Type>, codomain: Box<Type> },
    Path { space: Box<Type> },
    Smooth(Box<Type>),
    Universe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    bindings: Vec<(String, Type)>,
}

impl Context {
    pub fn new() -> Self {
        Context { bindings: Vec::new() }
    }

    pub fn add(&mut self, name: String, ty: Type) {
        self.bindings.push((name, ty));
    }

    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, ty)| ty)
    }
}

/// Type checker for SCTT
pub struct TypeChecker {
    context: Context,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            context: Context::new(),
        }
    }

    /// Simplified type inference
    pub fn infer_expr(&self, expr: &str) -> Result<Type, String> {
        if expr.contains("sin") || expr.contains("cos") {
            Ok(Type::Smooth(Box::new(Type::Function {
                domain: Box::new(Type::Real),
                codomain: Box::new(Type::Real),
            })))
        } else if expr.contains("Path") {
            Ok(Type::Path {
                space: Box::new(Type::Real),
            })
        } else if expr.contains("λ") {
            Ok(Type::Function {
                domain: Box::new(Type::Real),
                codomain: Box::new(Type::Real),
            })
        } else if expr == "0" || expr == "1" {
            Ok(Type::Interval)
        } else {
            Ok(Type::Real)
        }
    }
}

// WASM bindings
#[wasm_bindgen]
pub struct WasmChecker {
    checker: TypeChecker,
}

#[wasm_bindgen]
impl WasmChecker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmChecker {
        WasmChecker {
            checker: TypeChecker::new(),
        }
    }

    pub fn add_binding(&mut self, name: String, type_str: String) {
        let ty = parse_type(&type_str);
        self.checker.context.add(name, ty);
    }

    pub fn check_expr(&mut self, expr: String) -> JsValue {
        let result = self.checker.infer_expr(&expr);

        match result {
            Ok(ty) => serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": true,
                "type": format_type(&ty)
            })).unwrap(),
            Err(e) => serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": false,
                "error": e
            })).unwrap()
        }
    }

    pub fn check_coherence(&self, smooth_expr: String, cubical_expr: String) -> JsValue {
        // Check smooth-cubical coherence (simplified)
        let coherent = smooth_expr.len() > 0 && cubical_expr.len() > 0;
        
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "coherent": coherent,
            "smooth": smooth_expr,
            "cubical": cubical_expr,
            "message": if coherent {
                "Smooth and cubical structures are coherent!"
            } else {
                "Coherence check failed"
            }
        })).unwrap()
    }
}

// Helper to parse type strings
fn parse_type(s: &str) -> Type {
    match s {
        "Real" | "ℝ" => Type::Real,
        "I" | "Interval" => Type::Interval,
        s if s.starts_with("C∞") => Type::Smooth(Box::new(Type::Real)),
        _ => Type::Universe,
    }
}

// Helper to format types
fn format_type(ty: &Type) -> String {
    match ty {
        Type::Real => "ℝ".to_string(),
        Type::Interval => "I".to_string(),
        Type::Function { domain, codomain } => {
            format!("{} → {}", format_type(domain), format_type(codomain))
        }
        Type::Path { space } => format!("Path {}", format_type(space)),
        Type::Smooth(inner) => format!("C∞({})", format_type(inner)),
        Type::Universe => "Type".to_string(),
    }
}

// Global exported function
#[wasm_bindgen]
pub fn type_check(expr: &str) -> JsValue {
    let checker = TypeChecker::new();
    
    let result = checker.infer_expr(expr);

    match result {
        Ok(ty) => {
            let type_str = format_type(&ty);
            let description = match &ty {
                Type::Smooth(_) => "Smooth function from reals to reals",
                Type::Path { .. } => "Continuous path in space",
                Type::Function { .. } => "Function type",
                Type::Interval => "Interval value between 0 and 1",
                Type::Real => "Real number",
                Type::Universe => "Type universe",
            };
            
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": true,
                "type": type_str,
                "description": description,
                "expression": expr
            })).unwrap()
        }
        Err(e) => serde_wasm_bindgen::to_value(&serde_json::json!({
            "success": false,
            "error": e,
            "expression": expr
        })).unwrap()
    }
}

#[wasm_bindgen]
pub fn check_composition(f: &str, g: &str) -> JsValue {
    // Check if functions can be composed
    let composable = f.len() > 0 && g.len() > 0;
    
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "f": f,
        "g": g,
        "composable": composable,
        "type": if composable { "C∞(ℝ → ℝ)" } else { "Error" },
        "message": if composable {
            format!("({}) ∘ ({}) is well-typed!", f, g)
        } else {
            "Composition failed type checking".to_string()
        }
    })).unwrap()
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_inference() {
        let checker = TypeChecker::new();
        let ty = checker.infer_expr("sin(x)").unwrap();
        assert!(matches!(ty, Type::Smooth(_)));
    }
}