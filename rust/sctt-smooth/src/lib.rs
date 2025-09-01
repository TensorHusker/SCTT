//! Smooth mathematics module for SCTT
//! Handles derivatives, integrals, and smooth functions

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use num_traits::{Float, Zero, One};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmoothFunction {
    expression: String,
    variable: String,
}

impl SmoothFunction {
    pub fn new(expression: String, variable: String) -> Self {
        SmoothFunction { expression, variable }
    }

    /// Evaluate the function at a point (simplified)
    pub fn evaluate(&self, x: f64) -> f64 {
        // Simplified evaluation for demo
        if self.expression.contains("sin") {
            x.sin()
        } else if self.expression.contains("cos") {
            x.cos()
        } else if self.expression.contains("x²") || self.expression.contains("x^2") {
            x * x
        } else if self.expression.contains("exp") {
            x.exp()
        } else {
            x
        }
    }

    /// Compute symbolic derivative (simplified)
    pub fn derivative(&self) -> SmoothFunction {
        let deriv_expr = if self.expression.contains("sin") {
            format!("cos({})", self.variable)
        } else if self.expression.contains("cos") {
            format!("-sin({})", self.variable)
        } else if self.expression.contains("x²") {
            format!("2*{}", self.variable)
        } else if self.expression.contains("exp") {
            self.expression.clone()
        } else {
            "1".to_string()
        };

        SmoothFunction::new(deriv_expr, self.variable.clone())
    }

    /// Compute nth derivative
    pub fn nth_derivative(&self, n: usize) -> SmoothFunction {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.derivative();
        }
        result
    }
}

// WASM bindings
#[wasm_bindgen]
pub struct WasmSmoothFunction {
    inner: SmoothFunction,
}

#[wasm_bindgen]
impl WasmSmoothFunction {
    #[wasm_bindgen(constructor)]
    pub fn new(expression: String, variable: String) -> WasmSmoothFunction {
        WasmSmoothFunction {
            inner: SmoothFunction::new(expression, variable),
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.inner.evaluate(x)
    }

    pub fn derivative(&self) -> WasmSmoothFunction {
        WasmSmoothFunction {
            inner: self.inner.derivative(),
        }
    }

    pub fn nth_derivative(&self, n: usize) -> WasmSmoothFunction {
        WasmSmoothFunction {
            inner: self.inner.nth_derivative(n),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> String {
        self.inner.expression.clone()
    }
}

// Taylor series expansion
#[wasm_bindgen]
pub struct TaylorSeries {
    center: f64,
    coefficients: Vec<f64>,
}

#[wasm_bindgen]
impl TaylorSeries {
    #[wasm_bindgen(constructor)]
    pub fn new(center: f64) -> TaylorSeries {
        TaylorSeries {
            center,
            coefficients: Vec::new(),
        }
    }

    pub fn add_term(&mut self, coefficient: f64) {
        self.coefficients.push(coefficient);
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let dx = x - self.center;
        let mut result = 0.0;
        let mut power = 1.0;

        for coeff in &self.coefficients {
            result += coeff * power;
            power *= dx;
        }

        result
    }

    pub fn truncate(&mut self, order: usize) {
        self.coefficients.truncate(order + 1);
    }
}

// Global exported functions for direct WASM use
#[wasm_bindgen]
pub fn evaluate(expr: &str, x: f64) -> f64 {
    let func = SmoothFunction::new(expr.to_string(), "x".to_string());
    func.evaluate(x)
}

#[wasm_bindgen]
pub fn derivative(expr: &str, x: f64, order: usize) -> f64 {
    let func = SmoothFunction::new(expr.to_string(), "x".to_string());
    let deriv = func.nth_derivative(order);
    deriv.evaluate(x)
}

// Chain rule demonstration
#[wasm_bindgen]
pub fn chain_rule(f_expr: &str, g_expr: &str, x: f64) -> JsValue {
    let f = SmoothFunction::new(f_expr.to_string(), "x".to_string());
    let g = SmoothFunction::new(g_expr.to_string(), "x".to_string());
    
    let gx = g.evaluate(x);
    let f_prime_gx = f.derivative().evaluate(gx);
    let g_prime_x = g.derivative().evaluate(x);
    let result = f_prime_gx * g_prime_x;

    serde_wasm_bindgen::to_value(&serde_json::json!({
        "f": f_expr,
        "g": g_expr,
        "x": x,
        "g(x)": gx,
        "f'(g(x))": f_prime_gx,
        "g'(x)": g_prime_x,
        "result": result,
        "formula": format!("(f∘g)'({}) = f'(g({})) · g'({}) = {} · {} = {}", 
                          x, x, x, f_prime_gx, g_prime_x, result)
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
    fn test_smooth_evaluation() {
        let f = SmoothFunction::new("sin(x)".to_string(), "x".to_string());
        let val = f.evaluate(0.0);
        assert!((val - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_derivative() {
        let f = SmoothFunction::new("x²".to_string(), "x".to_string());
        let df = f.derivative();
        assert_eq!(df.expression, "2*x");
    }
}