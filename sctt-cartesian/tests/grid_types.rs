//! # ARC Grid Type Tests
//!
//! Verify that ColorType, Color, and GridLit terms evaluate, normalize,
//! convert, and type-check correctly.

use sctt_cartesian::evaluate::evaluate;
use sctt_cartesian::quote::{normalize, quote};
use sctt_cartesian::conv::conv;
use sctt_cartesian::check::TypeChecker;
use sctt_cartesian::syntax::Term;
use sctt_cartesian::value::{Env, Value};

// ─── Evaluation ─────────────────────────────────────────────────────────────

#[test]
fn eval_color_type() {
    let v = evaluate(&Env::new(), &Term::ColorType);
    assert!(matches!(&*v, Value::ColorType));
}

#[test]
fn eval_color_literal() {
    let v = evaluate(&Env::new(), &Term::color(5));
    assert!(matches!(&*v, Value::Color(5)));
}

#[test]
fn eval_grid_lit() {
    let g = Term::grid_lit(2, 3, vec![0, 1, 2, 3, 4, 5]);
    let v = evaluate(&Env::new(), &g);
    match &*v {
        Value::GridLit { rows, cols, data } => {
            assert_eq!(*rows, 2);
            assert_eq!(*cols, 3);
            assert_eq!(data, &vec![0, 1, 2, 3, 4, 5]);
        }
        other => panic!("expected GridLit, got {:?}", other),
    }
}

// ─── Normalization (NbE roundtrip) ──────────────────────────────────────────

#[test]
fn normalize_color_type_idempotent() {
    let t = Term::ColorType;
    let n1 = normalize(&t);
    let n2 = normalize(&n1);
    assert_eq!(n1, n2);
}

#[test]
fn normalize_color_idempotent() {
    let t = Term::color(7);
    let n1 = normalize(&t);
    let n2 = normalize(&n1);
    assert_eq!(n1, n2);
    assert_eq!(n1, Term::Color(7));
}

#[test]
fn normalize_grid_lit_idempotent() {
    let t = Term::grid_lit(2, 2, vec![1, 0, 0, 1]);
    let n1 = normalize(&t);
    let n2 = normalize(&n1);
    assert_eq!(n1, n2);
}

// ─── Conversion ─────────────────────────────────────────────────────────────

#[test]
fn conv_color_type_eq() {
    assert!(conv(0, 0, &Value::ColorType, &Value::ColorType));
}

#[test]
fn conv_color_eq() {
    assert!(conv(0, 0, &Value::Color(3), &Value::Color(3)));
}

#[test]
fn conv_color_neq() {
    assert!(!conv(0, 0, &Value::Color(3), &Value::Color(7)));
}

#[test]
fn conv_color_type_neq_nat() {
    assert!(!conv(0, 0, &Value::ColorType, &Value::Nat));
}

#[test]
fn conv_grid_eq() {
    let g1 = Value::GridLit { rows: 2, cols: 2, data: vec![1, 2, 3, 4] };
    let g2 = Value::GridLit { rows: 2, cols: 2, data: vec![1, 2, 3, 4] };
    assert!(conv(0, 0, &g1, &g2));
}

#[test]
fn conv_grid_neq_data() {
    let g1 = Value::GridLit { rows: 2, cols: 2, data: vec![1, 2, 3, 4] };
    let g2 = Value::GridLit { rows: 2, cols: 2, data: vec![1, 2, 3, 5] };
    assert!(!conv(0, 0, &g1, &g2));
}

#[test]
fn conv_grid_neq_dims() {
    let g1 = Value::GridLit { rows: 2, cols: 2, data: vec![1, 2, 3, 4] };
    let g2 = Value::GridLit { rows: 1, cols: 4, data: vec![1, 2, 3, 4] };
    assert!(!conv(0, 0, &g1, &g2));
}

// ─── Type Checking ──────────────────────────────────────────────────────────

#[test]
fn tc_color_type_is_type0() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::ColorType).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn tc_color_is_color_type() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::color(3)).unwrap();
    assert!(matches!(&*ty, Value::ColorType));
}

#[test]
fn tc_grid_lit_infers() {
    let mut tc = TypeChecker::new();
    let g = Term::grid_lit(2, 2, vec![0, 1, 2, 3]);
    let ty = tc.infer(&g).unwrap();
    assert!(matches!(&*ty, Value::ColorType));
}

#[test]
fn tc_color_check_against_color_type() {
    let mut tc = TypeChecker::new();
    assert!(tc.check(&Term::color(9), &Value::ColorType).is_ok());
}

// ─── Quote Roundtrip ────────────────────────────────────────────────────────

#[test]
fn quote_color_roundtrip() {
    let v = Value::Color(4);
    let t = quote(0, 0, &v);
    assert_eq!(t, Term::Color(4));
}

#[test]
fn quote_grid_roundtrip() {
    let v = Value::GridLit { rows: 3, cols: 3, data: vec![0; 9] };
    let t = quote(0, 0, &v);
    match t {
        Term::GridLit { rows, cols, data } => {
            assert_eq!(rows, 3);
            assert_eq!(cols, 3);
            assert_eq!(data, vec![0; 9]);
        }
        other => panic!("expected GridLit term, got {:?}", other),
    }
}

// ─── Smart Constructor Validation ───────────────────────────────────────────

#[test]
#[should_panic(expected = "ARC color must be 0-9")]
fn color_out_of_range_panics() {
    Term::color(10);
}

#[test]
#[should_panic(expected = "grid data length mismatch")]
fn grid_lit_wrong_size_panics() {
    Term::grid_lit(2, 2, vec![0, 1, 2]); // 3 != 2*2
}

#[test]
#[should_panic(expected = "all grid cells must be colors 0-9")]
fn grid_lit_bad_color_panics() {
    Term::grid_lit(1, 1, vec![15]);
}
