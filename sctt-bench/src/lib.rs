//! # SCTT Benchmarks
//!
//! Comprehensive performance benchmarks for all SCTT operations.

use sctt_core::prelude::*;
use sctt_checker::TypeChecker;
use criterion::{black_box, Criterion};

/// Benchmark type checking performance
pub fn bench_type_checking(c: &mut Criterion) {
    let mut checker = TypeChecker::new();
    let context = Context::new();
    
    // Simple terms
    c.bench_function("type_check_simple", |b| {
        let term = Term::var("x");
        let typ = Type::universe(0);
        b.iter(|| {
            black_box(checker.check_type(&term, &typ, &context))
        })
    });
    
    // Complex terms
    c.bench_function("type_check_complex", |b| {
        let term = create_complex_term();
        let typ = create_complex_type();
        b.iter(|| {
            black_box(checker.check_type(&term, &typ, &context))
        })
    });
}

/// Benchmark normalization performance
pub fn bench_normalization(c: &mut Criterion) {
    let checker = TypeChecker::new();
    
    c.bench_function("normalize_beta", |b| {
        let term = Term::app(
            Term::lambda("x", Term::var("x")),
            Term::var("y")
        );
        b.iter(|| {
            black_box(checker.normalize_term(&term))
        })
    });
    
    c.bench_function("normalize_complex", |b| {
        let term = create_complex_term();
        b.iter(|| {
            black_box(checker.normalize_term(&term))
        })
    });
}

/// Create a complex term for benchmarking
fn create_complex_term() -> Term {
    // Church numeral 5
    let five = Term::lambda("f", Term::lambda("x", 
        Term::app(Term::var("f"),
            Term::app(Term::var("f"),
                Term::app(Term::var("f"),
                    Term::app(Term::var("f"),
                        Term::app(Term::var("f"), Term::var("x"))
                    )
                )
            )
        )
    ));
    
    // Apply to identity and zero
    Term::app(
        Term::app(five, Term::lambda("x", Term::var("x"))),
        Term::var("0")
    )
}

/// Create a complex type for benchmarking
fn create_complex_type() -> Type {
    // (A B C : Type) → (A → B → C) → (A → B) → A → C
    Type::pi("A", Type::universe(0),
        Type::pi("B", Type::universe(0),
            Type::pi("C", Type::universe(0),
                Type::pi("f", Type::pi("x", Type::var("A"), 
                              Type::pi("y", Type::var("B"), Type::var("C"))),
                    Type::pi("g", Type::pi("x", Type::var("A"), Type::var("B")),
                        Type::pi("x", Type::var("A"), Type::var("C"))
                    )
                )
            )
        )
    )
}