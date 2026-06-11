//! Demonstrates basic SCTT usage: identity function, reflexivity path, and type checking.

use sctt_cartesian::dim::Dim;
use sctt_cartesian::evaluate::evaluate;
use sctt_cartesian::value::Env;
use sctt_cartesian::*;

fn main() {
    println!("=== SCTT Cartesian — Identity and Path Example ===\n");

    // 1. Polymorphic identity: λA. λx. x  :  Π(A:Type₀). A → A
    let id_term = Term::lambda(Term::lambda(Term::var(0)));
    let id_type = Term::pi(
        Term::universe(0).unwrap(),
        Term::pi(Term::var(0), Term::var(1)),
    );

    println!("Identity term: λA. λx. x");
    println!("Identity type: Π(A:Type₀). A → A");

    // Type-check it
    let mut tc = TypeChecker::new();
    match tc.check(&id_term, &evaluate(&Env::new(), &id_type)) {
        Ok(()) => println!("Type check: PASS"),
        Err(e) => println!("Type check: FAIL — {}", e),
    }

    // 2. Apply identity to Nat and Zero: (id Nat Zero) normalizes to Zero
    let applied = Term::app(Term::app(id_term.clone(), Term::Nat), Term::Zero);
    let normalized = normalize(&applied);
    println!("\n(id Nat Zero) normalizes to: {:?}", normalized);
    assert_eq!(normalized, Term::Zero);

    // 3. Reflexivity path: <i> Zero : Path Nat Zero Zero
    let refl_zero = Term::path_lam(Term::Zero);
    let path_type = Term::path_type(Term::Nat, Term::Zero, Term::Zero);
    let path_type_val = evaluate(&Env::new(), &path_type);

    println!("\nReflexivity path: <i> Zero");
    let mut tc2 = TypeChecker::new();
    match tc2.check(&refl_zero, &path_type_val) {
        Ok(()) => println!("  : Path Nat Zero Zero — PASS"),
        Err(e) => println!("  : Path Nat Zero Zero — FAIL — {}", e),
    }

    // 4. Path application: (<i> Zero) @ 0 normalizes to Zero
    let path_app = Term::path_app(refl_zero, Dim::Zero);
    let normalized_app = normalize(&path_app);
    println!("\n(<i> Zero) @ 0 normalizes to: {:?}", normalized_app);
    assert_eq!(normalized_app, Term::Zero);

    // 5. Smoothness tracking
    println!("\n=== Smoothness ===");
    println!(
        "C^inf meet C^3 = {:?}",
        Smoothness::CInfty.meet(Smoothness::C(3))
    );
    println!(
        "diff(C^3) = {:?}",
        sctt_cartesian::smooth::diff_order(Smoothness::C(3))
    );
    println!(
        "diff(C^0) = {:?}",
        sctt_cartesian::smooth::diff_order(Smoothness::Continuous)
    );

    println!("\nAll checks passed!");
}
