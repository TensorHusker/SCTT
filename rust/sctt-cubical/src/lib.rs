//! Cubical operations module for SCTT
//! Handles paths, intervals, and composition

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

/// Interval values in cubical type theory
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IntervalValue {
    Zero,
    One,
    Var(usize),  // Variable index
}

/// De Morgan algebra operations on intervals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Interval {
    Value(IntervalValue),
    Meet(Box<Interval>, Box<Interval>),  // ∧
    Join(Box<Interval>, Box<Interval>),   // ∨
    Neg(Box<Interval>),                   // 1 - i
}

impl Interval {
    pub fn zero() -> Self {
        Interval::Value(IntervalValue::Zero)
    }

    pub fn one() -> Self {
        Interval::Value(IntervalValue::One)
    }

    pub fn var(index: usize) -> Self {
        Interval::Value(IntervalValue::Var(index))
    }

    /// Evaluate interval expression at a point
    pub fn eval(&self, point: f64) -> f64 {
        match self {
            Interval::Value(IntervalValue::Zero) => 0.0,
            Interval::Value(IntervalValue::One) => 1.0,
            Interval::Value(IntervalValue::Var(_)) => point.clamp(0.0, 1.0),
            Interval::Meet(i, j) => i.eval(point).min(j.eval(point)),
            Interval::Join(i, j) => i.eval(point).max(j.eval(point)),
            Interval::Neg(i) => 1.0 - i.eval(point),
        }
    }
}

/// Path in cubical type theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    expression: String,
    parameter: String,
}

impl Path {
    pub fn new(expression: String, parameter: String) -> Self {
        Path { expression, parameter }
    }

    /// Evaluate path at a point t ∈ [0,1]
    pub fn evaluate(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        
        // Simplified evaluation for demo
        if self.expression.contains("π * t * (3 - 2*t)") {
            std::f64::consts::PI * t * (3.0 - 2.0 * t)
        } else if self.expression.contains("sin") {
            (std::f64::consts::PI * t).sin()
        } else if self.expression.contains("t²") || self.expression.contains("t^2") {
            t * t
        } else {
            t
        }
    }

    /// Check if path satisfies boundary conditions
    pub fn check_boundaries(&self, start: f64, end: f64) -> bool {
        let epsilon = 1e-10;
        (self.evaluate(0.0) - start).abs() < epsilon &&
        (self.evaluate(1.0) - end).abs() < epsilon
    }
}

// WASM bindings
#[wasm_bindgen]
pub struct WasmInterval {
    inner: Interval,
}

#[wasm_bindgen]
impl WasmInterval {
    #[wasm_bindgen(constructor)]
    pub fn new(value: f64) -> WasmInterval {
        let inner = if value <= 0.0 {
            Interval::zero()
        } else if value >= 1.0 {
            Interval::one()
        } else {
            Interval::var(0)
        };
        WasmInterval { inner }
    }

    pub fn zero() -> WasmInterval {
        WasmInterval { inner: Interval::zero() }
    }

    pub fn one() -> WasmInterval {
        WasmInterval { inner: Interval::one() }
    }

    pub fn meet(&self, other: &WasmInterval) -> WasmInterval {
        WasmInterval {
            inner: Interval::Meet(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone())
            )
        }
    }

    pub fn join(&self, other: &WasmInterval) -> WasmInterval {
        WasmInterval {
            inner: Interval::Join(
                Box::new(self.inner.clone()),
                Box::new(other.inner.clone())
            )
        }
    }

    pub fn neg(&self) -> WasmInterval {
        WasmInterval {
            inner: Interval::Neg(Box::new(self.inner.clone()))
        }
    }

    pub fn eval(&self, point: f64) -> f64 {
        self.inner.eval(point)
    }
}

#[wasm_bindgen]
pub struct WasmPath {
    inner: Path,
}

#[wasm_bindgen]
impl WasmPath {
    #[wasm_bindgen(constructor)]
    pub fn new(expression: String, parameter: String) -> WasmPath {
        WasmPath {
            inner: Path::new(expression, parameter)
        }
    }

    pub fn evaluate(&self, t: f64) -> f64 {
        self.inner.evaluate(t)
    }

    pub fn check_boundaries(&self, start: f64, end: f64) -> bool {
        self.inner.check_boundaries(start, end)
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> String {
        self.inner.expression.clone()
    }
}

// Global exported functions
#[wasm_bindgen]
pub fn path_eval(expr: &str, t: f64) -> f64 {
    let path = Path::new(expr.to_string(), "t".to_string());
    path.evaluate(t)
}

#[wasm_bindgen]
pub fn interval_meet(i: f64, j: f64) -> f64 {
    i.min(j).clamp(0.0, 1.0)
}

#[wasm_bindgen]
pub fn interval_join(i: f64, j: f64) -> f64 {
    i.max(j).clamp(0.0, 1.0)
}

#[wasm_bindgen]
pub fn interval_neg(i: f64) -> f64 {
    (1.0 - i).clamp(0.0, 1.0)
}

// Composition operation demo
#[wasm_bindgen]
pub fn composition(path1_expr: &str, path2_expr: &str, t: f64) -> JsValue {
    let path1 = Path::new(path1_expr.to_string(), "t".to_string());
    let path2 = Path::new(path2_expr.to_string(), "t".to_string());
    
    // Evaluate paths
    let p1_val = path1.evaluate(t);
    let p2_val = path2.evaluate(t);
    
    // Check composition at boundaries
    let p1_end = path1.evaluate(1.0);
    let p2_start = path2.evaluate(0.0);
    let composable = (p1_end - p2_start).abs() < 1e-10;
    
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "path1": path1_expr,
        "path2": path2_expr,
        "t": t,
        "path1_value": p1_val,
        "path2_value": p2_val,
        "path1_endpoint": p1_end,
        "path2_startpoint": p2_start,
        "composable": composable,
        "message": if composable { 
            "Paths can be composed!" 
        } else { 
            "Paths cannot be composed - boundary mismatch" 
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
    fn test_interval_operations() {
        let i = Interval::var(0);
        assert_eq!(i.eval(0.5), 0.5);
        
        let meet = Interval::Meet(
            Box::new(Interval::var(0)),
            Box::new(Interval::Value(IntervalValue::One))
        );
        assert_eq!(meet.eval(0.5), 0.5);
    }

    #[test]
    fn test_path_evaluation() {
        let path = Path::new("t²".to_string(), "t".to_string());
        assert!((path.evaluate(0.5) - 0.25).abs() < 1e-10);
    }
}